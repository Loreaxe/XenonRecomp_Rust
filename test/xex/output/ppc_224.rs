pub fn sub_832850D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832850D0 size=64
    let mut pc: u32 = 0x832850D0;
    'dispatch: loop {
        match pc {
            0x832850D0 => {
    //   block [0x832850D0..0x83285110)
	// 832850D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832850D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832850D8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832850DC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 832850E0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832850E4: 388BB0E0  addi r4, r11, -0x4f20
	ctx.r[4].s64 = ctx.r[11].s64 + -20256;
	// 832850E8: 386AE5BC  addi r3, r10, -0x1a44
	ctx.r[3].s64 = ctx.r[10].s64 + -6724;
	// 832850EC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832850F0: 4AFA7DE1  bl 0x8222ced0
	ctx.lr = 0x832850F4;
	sub_8222CED0(ctx, base);
	// 832850F4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 832850F8: 38691940  addi r3, r9, 0x1940
	ctx.r[3].s64 = ctx.r[9].s64 + 6464;
	// 832850FC: 4BA24E25  bl 0x82ca9f20
	ctx.lr = 0x83285100;
	sub_82CA9F20(ctx, base);
	// 83285100: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83285104: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83285108: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8328510C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83285110(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83285110 size=64
    let mut pc: u32 = 0x83285110;
    'dispatch: loop {
        match pc {
            0x83285110 => {
    //   block [0x83285110..0x83285150)
	// 83285110: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83285114: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83285118: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8328511C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83285120: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83285124: 388BB104  addi r4, r11, -0x4efc
	ctx.r[4].s64 = ctx.r[11].s64 + -20220;
	// 83285128: 386AE5C0  addi r3, r10, -0x1a40
	ctx.r[3].s64 = ctx.r[10].s64 + -6720;
	// 8328512C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83285130: 4AFA7DA1  bl 0x8222ced0
	ctx.lr = 0x83285134;
	sub_8222CED0(ctx, base);
	// 83285134: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83285138: 38691950  addi r3, r9, 0x1950
	ctx.r[3].s64 = ctx.r[9].s64 + 6480;
	// 8328513C: 4BA24DE5  bl 0x82ca9f20
	ctx.lr = 0x83285140;
	sub_82CA9F20(ctx, base);
	// 83285140: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83285144: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83285148: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8328514C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83285150(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83285150 size=20
    let mut pc: u32 = 0x83285150;
    'dispatch: loop {
        match pc {
            0x83285150 => {
    //   block [0x83285150..0x83285164)
	// 83285150: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 83285154: 1000038C  vspltisw v0, 0
	for i in 0..4 {
		ctx.v[0].u32[i] = 0;
	}
	// 83285158: 394BE5D0  addi r10, r11, -0x1a30
	ctx.r[10].s64 = ctx.r[11].s64 + -6704;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83285168(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83285168 size=20
    let mut pc: u32 = 0x83285168;
    'dispatch: loop {
        match pc {
            0x83285168 => {
    //   block [0x83285168..0x8328517C)
	// 83285168: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 8328516C: 1000038C  vspltisw v0, 0
	for i in 0..4 {
		ctx.v[0].u32[i] = 0;
	}
	// 83285170: 394BE5E0  addi r10, r11, -0x1a20
	ctx.r[10].s64 = ctx.r[11].s64 + -6688;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83285180(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83285180 size=12
    let mut pc: u32 = 0x83285180;
    'dispatch: loop {
        match pc {
            0x83285180 => {
    //   block [0x83285180..0x8328518C)
	// 83285180: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 83285184: 386BE5F0  addi r3, r11, -0x1a10
	ctx.r[3].s64 = ctx.r[11].s64 + -6672;
	// 83285188: 4AFB6C28  b 0x8223bdb0
	sub_8223BDB0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83285190(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83285190 size=64
    let mut pc: u32 = 0x83285190;
    'dispatch: loop {
        match pc {
            0x83285190 => {
    //   block [0x83285190..0x832851D0)
	// 83285190: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83285194: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83285198: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8328519C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 832851A0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832851A4: 388BB074  addi r4, r11, -0x4f8c
	ctx.r[4].s64 = ctx.r[11].s64 + -20364;
	// 832851A8: 386AE630  addi r3, r10, -0x19d0
	ctx.r[3].s64 = ctx.r[10].s64 + -6608;
	// 832851AC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832851B0: 4AFA7D21  bl 0x8222ced0
	ctx.lr = 0x832851B4;
	sub_8222CED0(ctx, base);
	// 832851B4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 832851B8: 38691AC0  addi r3, r9, 0x1ac0
	ctx.r[3].s64 = ctx.r[9].s64 + 6848;
	// 832851BC: 4BA24D65  bl 0x82ca9f20
	ctx.lr = 0x832851C0;
	sub_82CA9F20(ctx, base);
	// 832851C0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832851C4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832851C8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832851CC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832851D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832851D0 size=64
    let mut pc: u32 = 0x832851D0;
    'dispatch: loop {
        match pc {
            0x832851D0 => {
    //   block [0x832851D0..0x83285210)
	// 832851D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832851D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832851D8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832851DC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 832851E0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832851E4: 388BB088  addi r4, r11, -0x4f78
	ctx.r[4].s64 = ctx.r[11].s64 + -20344;
	// 832851E8: 386AE634  addi r3, r10, -0x19cc
	ctx.r[3].s64 = ctx.r[10].s64 + -6604;
	// 832851EC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832851F0: 4AFA7CE1  bl 0x8222ced0
	ctx.lr = 0x832851F4;
	sub_8222CED0(ctx, base);
	// 832851F4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 832851F8: 38691AD0  addi r3, r9, 0x1ad0
	ctx.r[3].s64 = ctx.r[9].s64 + 6864;
	// 832851FC: 4BA24D25  bl 0x82ca9f20
	ctx.lr = 0x83285200;
	sub_82CA9F20(ctx, base);
	// 83285200: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83285204: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83285208: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8328520C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83285210(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83285210 size=64
    let mut pc: u32 = 0x83285210;
    'dispatch: loop {
        match pc {
            0x83285210 => {
    //   block [0x83285210..0x83285250)
	// 83285210: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83285214: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83285218: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8328521C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83285220: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83285224: 388BB098  addi r4, r11, -0x4f68
	ctx.r[4].s64 = ctx.r[11].s64 + -20328;
	// 83285228: 386AE638  addi r3, r10, -0x19c8
	ctx.r[3].s64 = ctx.r[10].s64 + -6600;
	// 8328522C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83285230: 4AFA7CA1  bl 0x8222ced0
	ctx.lr = 0x83285234;
	sub_8222CED0(ctx, base);
	// 83285234: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83285238: 38691AE0  addi r3, r9, 0x1ae0
	ctx.r[3].s64 = ctx.r[9].s64 + 6880;
	// 8328523C: 4BA24CE5  bl 0x82ca9f20
	ctx.lr = 0x83285240;
	sub_82CA9F20(ctx, base);
	// 83285240: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83285244: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83285248: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8328524C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83285250(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83285250 size=52
    let mut pc: u32 = 0x83285250;
    'dispatch: loop {
        match pc {
            0x83285250 => {
    //   block [0x83285250..0x83285284)
	// 83285250: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83285254: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83285258: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8328525C: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 83285260: 386BE63C  addi r3, r11, -0x19c4
	ctx.r[3].s64 = ctx.r[11].s64 + -6596;
	// 83285264: 4BFBBD95  bl 0x83240ff8
	ctx.lr = 0x83285268;
	sub_83240FF8(ctx, base);
	// 83285268: 3D40832B  lis r10, -0x7cd5
	ctx.r[10].s64 = -2094333952;
	// 8328526C: 386A1AF0  addi r3, r10, 0x1af0
	ctx.r[3].s64 = ctx.r[10].s64 + 6896;
	// 83285270: 4BA24CB1  bl 0x82ca9f20
	ctx.lr = 0x83285274;
	sub_82CA9F20(ctx, base);
	// 83285274: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83285278: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8328527C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83285280: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83285288(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83285288 size=64
    let mut pc: u32 = 0x83285288;
    'dispatch: loop {
        match pc {
            0x83285288 => {
    //   block [0x83285288..0x832852C8)
	// 83285288: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8328528C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83285290: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83285294: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83285298: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8328529C: 388BB038  addi r4, r11, -0x4fc8
	ctx.r[4].s64 = ctx.r[11].s64 + -20424;
	// 832852A0: 386AE64C  addi r3, r10, -0x19b4
	ctx.r[3].s64 = ctx.r[10].s64 + -6580;
	// 832852A4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832852A8: 4AFA7C29  bl 0x8222ced0
	ctx.lr = 0x832852AC;
	sub_8222CED0(ctx, base);
	// 832852AC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 832852B0: 38691B38  addi r3, r9, 0x1b38
	ctx.r[3].s64 = ctx.r[9].s64 + 6968;
	// 832852B4: 4BA24C6D  bl 0x82ca9f20
	ctx.lr = 0x832852B8;
	sub_82CA9F20(ctx, base);
	// 832852B8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832852BC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832852C0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832852C4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832852C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832852C8 size=64
    let mut pc: u32 = 0x832852C8;
    'dispatch: loop {
        match pc {
            0x832852C8 => {
    //   block [0x832852C8..0x83285308)
	// 832852C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832852CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832852D0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832852D4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 832852D8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832852DC: 388BB054  addi r4, r11, -0x4fac
	ctx.r[4].s64 = ctx.r[11].s64 + -20396;
	// 832852E0: 386AE650  addi r3, r10, -0x19b0
	ctx.r[3].s64 = ctx.r[10].s64 + -6576;
	// 832852E4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832852E8: 4AFA7BE9  bl 0x8222ced0
	ctx.lr = 0x832852EC;
	sub_8222CED0(ctx, base);
	// 832852EC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 832852F0: 38691B48  addi r3, r9, 0x1b48
	ctx.r[3].s64 = ctx.r[9].s64 + 6984;
	// 832852F4: 4BA24C2D  bl 0x82ca9f20
	ctx.lr = 0x832852F8;
	sub_82CA9F20(ctx, base);
	// 832852F8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832852FC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83285300: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83285304: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83285308(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83285308 size=64
    let mut pc: u32 = 0x83285308;
    'dispatch: loop {
        match pc {
            0x83285308 => {
    //   block [0x83285308..0x83285348)
	// 83285308: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8328530C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83285310: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83285314: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83285318: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8328531C: 388BB06C  addi r4, r11, -0x4f94
	ctx.r[4].s64 = ctx.r[11].s64 + -20372;
	// 83285320: 386AE654  addi r3, r10, -0x19ac
	ctx.r[3].s64 = ctx.r[10].s64 + -6572;
	// 83285324: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83285328: 4AFA7BA9  bl 0x8222ced0
	ctx.lr = 0x8328532C;
	sub_8222CED0(ctx, base);
	// 8328532C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83285330: 38691B58  addi r3, r9, 0x1b58
	ctx.r[3].s64 = ctx.r[9].s64 + 7000;
	// 83285334: 4BA24BED  bl 0x82ca9f20
	ctx.lr = 0x83285338;
	sub_82CA9F20(ctx, base);
	// 83285338: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8328533C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83285340: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83285344: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83285348(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83285348 size=52
    let mut pc: u32 = 0x83285348;
    'dispatch: loop {
        match pc {
            0x83285348 => {
    //   block [0x83285348..0x8328537C)
	// 83285348: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8328534C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83285350: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83285354: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 83285358: 386BE658  addi r3, r11, -0x19a8
	ctx.r[3].s64 = ctx.r[11].s64 + -6568;
	// 8328535C: 4BFBBC9D  bl 0x83240ff8
	ctx.lr = 0x83285360;
	sub_83240FF8(ctx, base);
	// 83285360: 3D40832B  lis r10, -0x7cd5
	ctx.r[10].s64 = -2094333952;
	// 83285364: 386A1B68  addi r3, r10, 0x1b68
	ctx.r[3].s64 = ctx.r[10].s64 + 7016;
	// 83285368: 4BA24BB9  bl 0x82ca9f20
	ctx.lr = 0x8328536C;
	sub_82CA9F20(ctx, base);
	// 8328536C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83285370: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83285374: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83285378: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83285380(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83285380 size=64
    let mut pc: u32 = 0x83285380;
    'dispatch: loop {
        match pc {
            0x83285380 => {
    //   block [0x83285380..0x832853C0)
	// 83285380: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83285384: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83285388: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8328538C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83285390: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83285394: 388BAFAC  addi r4, r11, -0x5054
	ctx.r[4].s64 = ctx.r[11].s64 + -20564;
	// 83285398: 386AE668  addi r3, r10, -0x1998
	ctx.r[3].s64 = ctx.r[10].s64 + -6552;
	// 8328539C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832853A0: 4AFA7B31  bl 0x8222ced0
	ctx.lr = 0x832853A4;
	sub_8222CED0(ctx, base);
	// 832853A4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 832853A8: 38691BB0  addi r3, r9, 0x1bb0
	ctx.r[3].s64 = ctx.r[9].s64 + 7088;
	// 832853AC: 4BA24B75  bl 0x82ca9f20
	ctx.lr = 0x832853B0;
	sub_82CA9F20(ctx, base);
	// 832853B0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832853B4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832853B8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832853BC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832853C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832853C0 size=64
    let mut pc: u32 = 0x832853C0;
    'dispatch: loop {
        match pc {
            0x832853C0 => {
    //   block [0x832853C0..0x83285400)
	// 832853C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832853C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832853C8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832853CC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 832853D0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832853D4: 388BAFB4  addi r4, r11, -0x504c
	ctx.r[4].s64 = ctx.r[11].s64 + -20556;
	// 832853D8: 386AE66C  addi r3, r10, -0x1994
	ctx.r[3].s64 = ctx.r[10].s64 + -6548;
	// 832853DC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832853E0: 4AFA7AF1  bl 0x8222ced0
	ctx.lr = 0x832853E4;
	sub_8222CED0(ctx, base);
	// 832853E4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 832853E8: 38691BC0  addi r3, r9, 0x1bc0
	ctx.r[3].s64 = ctx.r[9].s64 + 7104;
	// 832853EC: 4BA24B35  bl 0x82ca9f20
	ctx.lr = 0x832853F0;
	sub_82CA9F20(ctx, base);
	// 832853F0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832853F4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832853F8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832853FC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83285400(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83285400 size=64
    let mut pc: u32 = 0x83285400;
    'dispatch: loop {
        match pc {
            0x83285400 => {
    //   block [0x83285400..0x83285440)
	// 83285400: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83285404: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83285408: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8328540C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83285410: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83285414: 388BAFC8  addi r4, r11, -0x5038
	ctx.r[4].s64 = ctx.r[11].s64 + -20536;
	// 83285418: 386AE670  addi r3, r10, -0x1990
	ctx.r[3].s64 = ctx.r[10].s64 + -6544;
	// 8328541C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83285420: 4AFA7AB1  bl 0x8222ced0
	ctx.lr = 0x83285424;
	sub_8222CED0(ctx, base);
	// 83285424: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83285428: 38691BD0  addi r3, r9, 0x1bd0
	ctx.r[3].s64 = ctx.r[9].s64 + 7120;
	// 8328542C: 4BA24AF5  bl 0x82ca9f20
	ctx.lr = 0x83285430;
	sub_82CA9F20(ctx, base);
	// 83285430: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83285434: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83285438: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8328543C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83285440(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83285440 size=64
    let mut pc: u32 = 0x83285440;
    'dispatch: loop {
        match pc {
            0x83285440 => {
    //   block [0x83285440..0x83285480)
	// 83285440: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83285444: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83285448: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8328544C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83285450: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83285454: 388BAFDC  addi r4, r11, -0x5024
	ctx.r[4].s64 = ctx.r[11].s64 + -20516;
	// 83285458: 386AE674  addi r3, r10, -0x198c
	ctx.r[3].s64 = ctx.r[10].s64 + -6540;
	// 8328545C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83285460: 4AFA7A71  bl 0x8222ced0
	ctx.lr = 0x83285464;
	sub_8222CED0(ctx, base);
	// 83285464: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83285468: 38691BE0  addi r3, r9, 0x1be0
	ctx.r[3].s64 = ctx.r[9].s64 + 7136;
	// 8328546C: 4BA24AB5  bl 0x82ca9f20
	ctx.lr = 0x83285470;
	sub_82CA9F20(ctx, base);
	// 83285470: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83285474: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83285478: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8328547C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83285480(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83285480 size=128
    let mut pc: u32 = 0x83285480;
    'dispatch: loop {
        match pc {
            0x83285480 => {
    //   block [0x83285480..0x832854C0)
	// 83285480: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83285484: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83285488: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8328548C: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 83285490: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 83285494: 388BE674  addi r4, r11, -0x198c
	ctx.r[4].s64 = ctx.r[11].s64 + -6540;
	// 83285498: 4AF696A1  bl 0x821eeb38
	ctx.lr = 0x8328549C;
	sub_821EEB38(ctx, base);
	// 8328549C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 832854A0: 4B97E351  bl 0x82c037f0
	ctx.lr = 0x832854A4;
	sub_82C037F0(ctx, base);
	// 832854A4: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832854A8: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 832854AC: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 832854B0: 916AE678  stw r11, -0x1988(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-6536 as u32), ctx.r[11].u32 ) };
	// 832854B4: 4AF412B5  bl 0x821c6768
	ctx.lr = 0x832854B8;
	sub_821C6768(ctx, base);
	// 832854B8: 3D208349  lis r9, -0x7cb7
	ctx.r[9].s64 = -2092367872;
	// 832854BC: 38C97088  addi r6, r9, 0x7088
	ctx.r[6].s64 = ctx.r[9].s64 + 28808;
	pc = 0x832854C0; continue 'dispatch;
            }
            0x832854C0 => {
    //   block [0x832854C0..0x83285500)
	// 832854C0: 7CE000A6  mfmsr r7
	ctx.r[7].u64 = ctx.msr;
	// 832854C4: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 832854C8: 7D003028  lwarx r8, 0, r6
	// lwarx
	let ea = ctx.r[6].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[8].u64 = ctx.reserved.u32 as u64;
	// 832854CC: 3908FFFF  addi r8, r8, -1
	ctx.r[8].s64 = ctx.r[8].s64 + -1;
	// 832854D0: 7D00312D  stwcx. r8, 0, r6
	// stwcx.
	let addr = ctx.r[6].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[8].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 832854D4: 7CE10164  mtmsrd r7, 1
	ctx.msr = (ctx.r[7].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 832854D8: 4082FFE8  bne 0x832854c0
	if !ctx.cr[0].eq {
	pc = 0x832854C0; continue 'dispatch;
	}
	// 832854DC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 832854E0: 3C80832B  lis r4, -0x7cd5
	ctx.r[4].s64 = -2094333952;
	// 832854E4: 90A10050  stw r5, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[5].u32 ) };
	// 832854E8: 38641BF0  addi r3, r4, 0x1bf0
	ctx.r[3].s64 = ctx.r[4].s64 + 7152;
	// 832854EC: 4BA24A35  bl 0x82ca9f20
	ctx.lr = 0x832854F0;
	sub_82CA9F20(ctx, base);
	// 832854F0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832854F4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832854F8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832854FC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83285500(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83285500 size=64
    let mut pc: u32 = 0x83285500;
    'dispatch: loop {
        match pc {
            0x83285500 => {
    //   block [0x83285500..0x83285540)
	// 83285500: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83285504: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83285508: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8328550C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83285510: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83285514: 388BAFF4  addi r4, r11, -0x500c
	ctx.r[4].s64 = ctx.r[11].s64 + -20492;
	// 83285518: 386AE67C  addi r3, r10, -0x1984
	ctx.r[3].s64 = ctx.r[10].s64 + -6532;
	// 8328551C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83285520: 4AFA79B1  bl 0x8222ced0
	ctx.lr = 0x83285524;
	sub_8222CED0(ctx, base);
	// 83285524: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83285528: 38691BF8  addi r3, r9, 0x1bf8
	ctx.r[3].s64 = ctx.r[9].s64 + 7160;
	// 8328552C: 4BA249F5  bl 0x82ca9f20
	ctx.lr = 0x83285530;
	sub_82CA9F20(ctx, base);
	// 83285530: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83285534: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83285538: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8328553C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83285540(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83285540 size=64
    let mut pc: u32 = 0x83285540;
    'dispatch: loop {
        match pc {
            0x83285540 => {
    //   block [0x83285540..0x83285580)
	// 83285540: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83285544: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83285548: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8328554C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83285550: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83285554: 388BB018  addi r4, r11, -0x4fe8
	ctx.r[4].s64 = ctx.r[11].s64 + -20456;
	// 83285558: 386AE680  addi r3, r10, -0x1980
	ctx.r[3].s64 = ctx.r[10].s64 + -6528;
	// 8328555C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83285560: 4AFA7971  bl 0x8222ced0
	ctx.lr = 0x83285564;
	sub_8222CED0(ctx, base);
	// 83285564: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83285568: 38691C08  addi r3, r9, 0x1c08
	ctx.r[3].s64 = ctx.r[9].s64 + 7176;
	// 8328556C: 4BA249B5  bl 0x82ca9f20
	ctx.lr = 0x83285570;
	sub_82CA9F20(ctx, base);
	// 83285570: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83285574: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83285578: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8328557C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83285580(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83285580 size=64
    let mut pc: u32 = 0x83285580;
    'dispatch: loop {
        match pc {
            0x83285580 => {
    //   block [0x83285580..0x832855C0)
	// 83285580: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83285584: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83285588: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8328558C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83285590: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83285594: 388BB030  addi r4, r11, -0x4fd0
	ctx.r[4].s64 = ctx.r[11].s64 + -20432;
	// 83285598: 386AE684  addi r3, r10, -0x197c
	ctx.r[3].s64 = ctx.r[10].s64 + -6524;
	// 8328559C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832855A0: 4AFA7931  bl 0x8222ced0
	ctx.lr = 0x832855A4;
	sub_8222CED0(ctx, base);
	// 832855A4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 832855A8: 38691C18  addi r3, r9, 0x1c18
	ctx.r[3].s64 = ctx.r[9].s64 + 7192;
	// 832855AC: 4BA24975  bl 0x82ca9f20
	ctx.lr = 0x832855B0;
	sub_82CA9F20(ctx, base);
	// 832855B0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832855B4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832855B8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832855BC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832855C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832855C0 size=52
    let mut pc: u32 = 0x832855C0;
    'dispatch: loop {
        match pc {
            0x832855C0 => {
    //   block [0x832855C0..0x832855F4)
	// 832855C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832855C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832855C8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832855CC: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832855D0: 386BE688  addi r3, r11, -0x1978
	ctx.r[3].s64 = ctx.r[11].s64 + -6520;
	// 832855D4: 4BFBBA25  bl 0x83240ff8
	ctx.lr = 0x832855D8;
	sub_83240FF8(ctx, base);
	// 832855D8: 3D40832B  lis r10, -0x7cd5
	ctx.r[10].s64 = -2094333952;
	// 832855DC: 386A1C28  addi r3, r10, 0x1c28
	ctx.r[3].s64 = ctx.r[10].s64 + 7208;
	// 832855E0: 4BA24941  bl 0x82ca9f20
	ctx.lr = 0x832855E4;
	sub_82CA9F20(ctx, base);
	// 832855E4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832855E8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832855EC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832855F0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832855F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832855F8 size=64
    let mut pc: u32 = 0x832855F8;
    'dispatch: loop {
        match pc {
            0x832855F8 => {
    //   block [0x832855F8..0x83285638)
	// 832855F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832855FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83285600: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83285604: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83285608: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8328560C: 388BAE48  addi r4, r11, -0x51b8
	ctx.r[4].s64 = ctx.r[11].s64 + -20920;
	// 83285610: 386AE698  addi r3, r10, -0x1968
	ctx.r[3].s64 = ctx.r[10].s64 + -6504;
	// 83285614: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83285618: 4AFA78B9  bl 0x8222ced0
	ctx.lr = 0x8328561C;
	sub_8222CED0(ctx, base);
	// 8328561C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83285620: 38691C70  addi r3, r9, 0x1c70
	ctx.r[3].s64 = ctx.r[9].s64 + 7280;
	// 83285624: 4BA248FD  bl 0x82ca9f20
	ctx.lr = 0x83285628;
	sub_82CA9F20(ctx, base);
	// 83285628: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8328562C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83285630: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83285634: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83285638(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83285638 size=64
    let mut pc: u32 = 0x83285638;
    'dispatch: loop {
        match pc {
            0x83285638 => {
    //   block [0x83285638..0x83285678)
	// 83285638: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8328563C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83285640: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83285644: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83285648: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8328564C: 388BAE50  addi r4, r11, -0x51b0
	ctx.r[4].s64 = ctx.r[11].s64 + -20912;
	// 83285650: 386AE69C  addi r3, r10, -0x1964
	ctx.r[3].s64 = ctx.r[10].s64 + -6500;
	// 83285654: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83285658: 4AFA7879  bl 0x8222ced0
	ctx.lr = 0x8328565C;
	sub_8222CED0(ctx, base);
	// 8328565C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83285660: 38691C80  addi r3, r9, 0x1c80
	ctx.r[3].s64 = ctx.r[9].s64 + 7296;
	// 83285664: 4BA248BD  bl 0x82ca9f20
	ctx.lr = 0x83285668;
	sub_82CA9F20(ctx, base);
	// 83285668: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8328566C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83285670: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83285674: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83285678(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83285678 size=64
    let mut pc: u32 = 0x83285678;
    'dispatch: loop {
        match pc {
            0x83285678 => {
    //   block [0x83285678..0x832856B8)
	// 83285678: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8328567C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83285680: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83285684: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83285688: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8328568C: 388BAE64  addi r4, r11, -0x519c
	ctx.r[4].s64 = ctx.r[11].s64 + -20892;
	// 83285690: 386AE6A0  addi r3, r10, -0x1960
	ctx.r[3].s64 = ctx.r[10].s64 + -6496;
	// 83285694: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83285698: 4AFA7839  bl 0x8222ced0
	ctx.lr = 0x8328569C;
	sub_8222CED0(ctx, base);
	// 8328569C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 832856A0: 38691C90  addi r3, r9, 0x1c90
	ctx.r[3].s64 = ctx.r[9].s64 + 7312;
	// 832856A4: 4BA2487D  bl 0x82ca9f20
	ctx.lr = 0x832856A8;
	sub_82CA9F20(ctx, base);
	// 832856A8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832856AC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832856B0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832856B4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832856B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832856B8 size=64
    let mut pc: u32 = 0x832856B8;
    'dispatch: loop {
        match pc {
            0x832856B8 => {
    //   block [0x832856B8..0x832856F8)
	// 832856B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832856BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832856C0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832856C4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 832856C8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832856CC: 388BAE78  addi r4, r11, -0x5188
	ctx.r[4].s64 = ctx.r[11].s64 + -20872;
	// 832856D0: 386AE6A4  addi r3, r10, -0x195c
	ctx.r[3].s64 = ctx.r[10].s64 + -6492;
	// 832856D4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832856D8: 4AFA77F9  bl 0x8222ced0
	ctx.lr = 0x832856DC;
	sub_8222CED0(ctx, base);
	// 832856DC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 832856E0: 38691CA0  addi r3, r9, 0x1ca0
	ctx.r[3].s64 = ctx.r[9].s64 + 7328;
	// 832856E4: 4BA2483D  bl 0x82ca9f20
	ctx.lr = 0x832856E8;
	sub_82CA9F20(ctx, base);
	// 832856E8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832856EC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832856F0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832856F4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832856F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832856F8 size=64
    let mut pc: u32 = 0x832856F8;
    'dispatch: loop {
        match pc {
            0x832856F8 => {
    //   block [0x832856F8..0x83285738)
	// 832856F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832856FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83285700: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83285704: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83285708: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8328570C: 388BAE9C  addi r4, r11, -0x5164
	ctx.r[4].s64 = ctx.r[11].s64 + -20836;
	// 83285710: 386AE6A8  addi r3, r10, -0x1958
	ctx.r[3].s64 = ctx.r[10].s64 + -6488;
	// 83285714: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83285718: 4AFA77B9  bl 0x8222ced0
	ctx.lr = 0x8328571C;
	sub_8222CED0(ctx, base);
	// 8328571C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83285720: 38691CB0  addi r3, r9, 0x1cb0
	ctx.r[3].s64 = ctx.r[9].s64 + 7344;
	// 83285724: 4BA247FD  bl 0x82ca9f20
	ctx.lr = 0x83285728;
	sub_82CA9F20(ctx, base);
	// 83285728: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8328572C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83285730: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83285734: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83285738(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83285738 size=64
    let mut pc: u32 = 0x83285738;
    'dispatch: loop {
        match pc {
            0x83285738 => {
    //   block [0x83285738..0x83285778)
	// 83285738: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8328573C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83285740: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83285744: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83285748: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8328574C: 388BAEA4  addi r4, r11, -0x515c
	ctx.r[4].s64 = ctx.r[11].s64 + -20828;
	// 83285750: 386AE6AC  addi r3, r10, -0x1954
	ctx.r[3].s64 = ctx.r[10].s64 + -6484;
	// 83285754: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83285758: 4AFA7779  bl 0x8222ced0
	ctx.lr = 0x8328575C;
	sub_8222CED0(ctx, base);
	// 8328575C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83285760: 38691CC0  addi r3, r9, 0x1cc0
	ctx.r[3].s64 = ctx.r[9].s64 + 7360;
	// 83285764: 4BA247BD  bl 0x82ca9f20
	ctx.lr = 0x83285768;
	sub_82CA9F20(ctx, base);
	// 83285768: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8328576C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83285770: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83285774: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83285778(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83285778 size=64
    let mut pc: u32 = 0x83285778;
    'dispatch: loop {
        match pc {
            0x83285778 => {
    //   block [0x83285778..0x832857B8)
	// 83285778: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8328577C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83285780: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83285784: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83285788: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8328578C: 388BAEC8  addi r4, r11, -0x5138
	ctx.r[4].s64 = ctx.r[11].s64 + -20792;
	// 83285790: 386AE6B0  addi r3, r10, -0x1950
	ctx.r[3].s64 = ctx.r[10].s64 + -6480;
	// 83285794: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83285798: 4AFA7739  bl 0x8222ced0
	ctx.lr = 0x8328579C;
	sub_8222CED0(ctx, base);
	// 8328579C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 832857A0: 38691CD0  addi r3, r9, 0x1cd0
	ctx.r[3].s64 = ctx.r[9].s64 + 7376;
	// 832857A4: 4BA2477D  bl 0x82ca9f20
	ctx.lr = 0x832857A8;
	sub_82CA9F20(ctx, base);
	// 832857A8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832857AC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832857B0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832857B4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832857B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832857B8 size=64
    let mut pc: u32 = 0x832857B8;
    'dispatch: loop {
        match pc {
            0x832857B8 => {
    //   block [0x832857B8..0x832857F8)
	// 832857B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832857BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832857C0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832857C4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 832857C8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832857CC: 388BAEEC  addi r4, r11, -0x5114
	ctx.r[4].s64 = ctx.r[11].s64 + -20756;
	// 832857D0: 386AE6B4  addi r3, r10, -0x194c
	ctx.r[3].s64 = ctx.r[10].s64 + -6476;
	// 832857D4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832857D8: 4AFA76F9  bl 0x8222ced0
	ctx.lr = 0x832857DC;
	sub_8222CED0(ctx, base);
	// 832857DC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 832857E0: 38691CE0  addi r3, r9, 0x1ce0
	ctx.r[3].s64 = ctx.r[9].s64 + 7392;
	// 832857E4: 4BA2473D  bl 0x82ca9f20
	ctx.lr = 0x832857E8;
	sub_82CA9F20(ctx, base);
	// 832857E8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832857EC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832857F0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832857F4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832857F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832857F8 size=64
    let mut pc: u32 = 0x832857F8;
    'dispatch: loop {
        match pc {
            0x832857F8 => {
    //   block [0x832857F8..0x83285838)
	// 832857F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832857FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83285800: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83285804: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83285808: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8328580C: 388BAEF4  addi r4, r11, -0x510c
	ctx.r[4].s64 = ctx.r[11].s64 + -20748;
	// 83285810: 386AE6B8  addi r3, r10, -0x1948
	ctx.r[3].s64 = ctx.r[10].s64 + -6472;
	// 83285814: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83285818: 4AFA76B9  bl 0x8222ced0
	ctx.lr = 0x8328581C;
	sub_8222CED0(ctx, base);
	// 8328581C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83285820: 38691CF0  addi r3, r9, 0x1cf0
	ctx.r[3].s64 = ctx.r[9].s64 + 7408;
	// 83285824: 4BA246FD  bl 0x82ca9f20
	ctx.lr = 0x83285828;
	sub_82CA9F20(ctx, base);
	// 83285828: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8328582C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83285830: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83285834: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83285838(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83285838 size=64
    let mut pc: u32 = 0x83285838;
    'dispatch: loop {
        match pc {
            0x83285838 => {
    //   block [0x83285838..0x83285878)
	// 83285838: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8328583C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83285840: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83285844: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83285848: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8328584C: 388BAF14  addi r4, r11, -0x50ec
	ctx.r[4].s64 = ctx.r[11].s64 + -20716;
	// 83285850: 386AE6BC  addi r3, r10, -0x1944
	ctx.r[3].s64 = ctx.r[10].s64 + -6468;
	// 83285854: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83285858: 4AFA7679  bl 0x8222ced0
	ctx.lr = 0x8328585C;
	sub_8222CED0(ctx, base);
	// 8328585C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83285860: 38691D00  addi r3, r9, 0x1d00
	ctx.r[3].s64 = ctx.r[9].s64 + 7424;
	// 83285864: 4BA246BD  bl 0x82ca9f20
	ctx.lr = 0x83285868;
	sub_82CA9F20(ctx, base);
	// 83285868: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8328586C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83285870: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83285874: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83285878(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83285878 size=64
    let mut pc: u32 = 0x83285878;
    'dispatch: loop {
        match pc {
            0x83285878 => {
    //   block [0x83285878..0x832858B8)
	// 83285878: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8328587C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83285880: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83285884: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83285888: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8328588C: 388BAF24  addi r4, r11, -0x50dc
	ctx.r[4].s64 = ctx.r[11].s64 + -20700;
	// 83285890: 386AE6C0  addi r3, r10, -0x1940
	ctx.r[3].s64 = ctx.r[10].s64 + -6464;
	// 83285894: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83285898: 4AFA7639  bl 0x8222ced0
	ctx.lr = 0x8328589C;
	sub_8222CED0(ctx, base);
	// 8328589C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 832858A0: 38691D10  addi r3, r9, 0x1d10
	ctx.r[3].s64 = ctx.r[9].s64 + 7440;
	// 832858A4: 4BA2467D  bl 0x82ca9f20
	ctx.lr = 0x832858A8;
	sub_82CA9F20(ctx, base);
	// 832858A8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832858AC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832858B0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832858B4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832858B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832858B8 size=64
    let mut pc: u32 = 0x832858B8;
    'dispatch: loop {
        match pc {
            0x832858B8 => {
    //   block [0x832858B8..0x832858F8)
	// 832858B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832858BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832858C0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832858C4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 832858C8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832858CC: 388BAF30  addi r4, r11, -0x50d0
	ctx.r[4].s64 = ctx.r[11].s64 + -20688;
	// 832858D0: 386AE6C4  addi r3, r10, -0x193c
	ctx.r[3].s64 = ctx.r[10].s64 + -6460;
	// 832858D4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832858D8: 4AFA75F9  bl 0x8222ced0
	ctx.lr = 0x832858DC;
	sub_8222CED0(ctx, base);
	// 832858DC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 832858E0: 38691D20  addi r3, r9, 0x1d20
	ctx.r[3].s64 = ctx.r[9].s64 + 7456;
	// 832858E4: 4BA2463D  bl 0x82ca9f20
	ctx.lr = 0x832858E8;
	sub_82CA9F20(ctx, base);
	// 832858E8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832858EC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832858F0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832858F4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832858F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832858F8 size=64
    let mut pc: u32 = 0x832858F8;
    'dispatch: loop {
        match pc {
            0x832858F8 => {
    //   block [0x832858F8..0x83285938)
	// 832858F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832858FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83285900: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83285904: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83285908: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8328590C: 388BAF3C  addi r4, r11, -0x50c4
	ctx.r[4].s64 = ctx.r[11].s64 + -20676;
	// 83285910: 386AE6C8  addi r3, r10, -0x1938
	ctx.r[3].s64 = ctx.r[10].s64 + -6456;
	// 83285914: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83285918: 4AFA75B9  bl 0x8222ced0
	ctx.lr = 0x8328591C;
	sub_8222CED0(ctx, base);
	// 8328591C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83285920: 38691D30  addi r3, r9, 0x1d30
	ctx.r[3].s64 = ctx.r[9].s64 + 7472;
	// 83285924: 4BA245FD  bl 0x82ca9f20
	ctx.lr = 0x83285928;
	sub_82CA9F20(ctx, base);
	// 83285928: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8328592C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83285930: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83285934: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83285938(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83285938 size=64
    let mut pc: u32 = 0x83285938;
    'dispatch: loop {
        match pc {
            0x83285938 => {
    //   block [0x83285938..0x83285978)
	// 83285938: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8328593C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83285940: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83285944: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83285948: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8328594C: 388BAF44  addi r4, r11, -0x50bc
	ctx.r[4].s64 = ctx.r[11].s64 + -20668;
	// 83285950: 386AE6CC  addi r3, r10, -0x1934
	ctx.r[3].s64 = ctx.r[10].s64 + -6452;
	// 83285954: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83285958: 4AFA7579  bl 0x8222ced0
	ctx.lr = 0x8328595C;
	sub_8222CED0(ctx, base);
	// 8328595C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83285960: 38691D40  addi r3, r9, 0x1d40
	ctx.r[3].s64 = ctx.r[9].s64 + 7488;
	// 83285964: 4BA245BD  bl 0x82ca9f20
	ctx.lr = 0x83285968;
	sub_82CA9F20(ctx, base);
	// 83285968: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8328596C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83285970: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83285974: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83285978(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83285978 size=64
    let mut pc: u32 = 0x83285978;
    'dispatch: loop {
        match pc {
            0x83285978 => {
    //   block [0x83285978..0x832859B8)
	// 83285978: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8328597C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83285980: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83285984: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83285988: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8328598C: 388BAF54  addi r4, r11, -0x50ac
	ctx.r[4].s64 = ctx.r[11].s64 + -20652;
	// 83285990: 386AE6D0  addi r3, r10, -0x1930
	ctx.r[3].s64 = ctx.r[10].s64 + -6448;
	// 83285994: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83285998: 4AFA7539  bl 0x8222ced0
	ctx.lr = 0x8328599C;
	sub_8222CED0(ctx, base);
	// 8328599C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 832859A0: 38691D50  addi r3, r9, 0x1d50
	ctx.r[3].s64 = ctx.r[9].s64 + 7504;
	// 832859A4: 4BA2457D  bl 0x82ca9f20
	ctx.lr = 0x832859A8;
	sub_82CA9F20(ctx, base);
	// 832859A8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832859AC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832859B0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832859B4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832859B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832859B8 size=64
    let mut pc: u32 = 0x832859B8;
    'dispatch: loop {
        match pc {
            0x832859B8 => {
    //   block [0x832859B8..0x832859F8)
	// 832859B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832859BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832859C0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832859C4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 832859C8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832859CC: 388BAF60  addi r4, r11, -0x50a0
	ctx.r[4].s64 = ctx.r[11].s64 + -20640;
	// 832859D0: 386AE6D4  addi r3, r10, -0x192c
	ctx.r[3].s64 = ctx.r[10].s64 + -6444;
	// 832859D4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832859D8: 4AFA74F9  bl 0x8222ced0
	ctx.lr = 0x832859DC;
	sub_8222CED0(ctx, base);
	// 832859DC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 832859E0: 38691D60  addi r3, r9, 0x1d60
	ctx.r[3].s64 = ctx.r[9].s64 + 7520;
	// 832859E4: 4BA2453D  bl 0x82ca9f20
	ctx.lr = 0x832859E8;
	sub_82CA9F20(ctx, base);
	// 832859E8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832859EC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832859F0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832859F4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832859F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832859F8 size=64
    let mut pc: u32 = 0x832859F8;
    'dispatch: loop {
        match pc {
            0x832859F8 => {
    //   block [0x832859F8..0x83285A38)
	// 832859F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832859FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83285A00: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83285A04: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83285A08: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83285A0C: 388BAF68  addi r4, r11, -0x5098
	ctx.r[4].s64 = ctx.r[11].s64 + -20632;
	// 83285A10: 386AE6D8  addi r3, r10, -0x1928
	ctx.r[3].s64 = ctx.r[10].s64 + -6440;
	// 83285A14: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83285A18: 4AFA74B9  bl 0x8222ced0
	ctx.lr = 0x83285A1C;
	sub_8222CED0(ctx, base);
	// 83285A1C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83285A20: 38691D70  addi r3, r9, 0x1d70
	ctx.r[3].s64 = ctx.r[9].s64 + 7536;
	// 83285A24: 4BA244FD  bl 0x82ca9f20
	ctx.lr = 0x83285A28;
	sub_82CA9F20(ctx, base);
	// 83285A28: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83285A2C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83285A30: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83285A34: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83285A38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83285A38 size=64
    let mut pc: u32 = 0x83285A38;
    'dispatch: loop {
        match pc {
            0x83285A38 => {
    //   block [0x83285A38..0x83285A78)
	// 83285A38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83285A3C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83285A40: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83285A44: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83285A48: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83285A4C: 388BAF74  addi r4, r11, -0x508c
	ctx.r[4].s64 = ctx.r[11].s64 + -20620;
	// 83285A50: 386AE6DC  addi r3, r10, -0x1924
	ctx.r[3].s64 = ctx.r[10].s64 + -6436;
	// 83285A54: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83285A58: 4AFA7479  bl 0x8222ced0
	ctx.lr = 0x83285A5C;
	sub_8222CED0(ctx, base);
	// 83285A5C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83285A60: 38691D80  addi r3, r9, 0x1d80
	ctx.r[3].s64 = ctx.r[9].s64 + 7552;
	// 83285A64: 4BA244BD  bl 0x82ca9f20
	ctx.lr = 0x83285A68;
	sub_82CA9F20(ctx, base);
	// 83285A68: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83285A6C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83285A70: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83285A74: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83285A78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83285A78 size=64
    let mut pc: u32 = 0x83285A78;
    'dispatch: loop {
        match pc {
            0x83285A78 => {
    //   block [0x83285A78..0x83285AB8)
	// 83285A78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83285A7C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83285A80: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83285A84: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83285A88: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83285A8C: 388BAF80  addi r4, r11, -0x5080
	ctx.r[4].s64 = ctx.r[11].s64 + -20608;
	// 83285A90: 386AE6E0  addi r3, r10, -0x1920
	ctx.r[3].s64 = ctx.r[10].s64 + -6432;
	// 83285A94: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83285A98: 4AFA7439  bl 0x8222ced0
	ctx.lr = 0x83285A9C;
	sub_8222CED0(ctx, base);
	// 83285A9C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83285AA0: 38691D90  addi r3, r9, 0x1d90
	ctx.r[3].s64 = ctx.r[9].s64 + 7568;
	// 83285AA4: 4BA2447D  bl 0x82ca9f20
	ctx.lr = 0x83285AA8;
	sub_82CA9F20(ctx, base);
	// 83285AA8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83285AAC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83285AB0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83285AB4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83285AB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83285AB8 size=64
    let mut pc: u32 = 0x83285AB8;
    'dispatch: loop {
        match pc {
            0x83285AB8 => {
    //   block [0x83285AB8..0x83285AF8)
	// 83285AB8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83285ABC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83285AC0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83285AC4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83285AC8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83285ACC: 388BADFC  addi r4, r11, -0x5204
	ctx.r[4].s64 = ctx.r[11].s64 + -20996;
	// 83285AD0: 386AE6E4  addi r3, r10, -0x191c
	ctx.r[3].s64 = ctx.r[10].s64 + -6428;
	// 83285AD4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83285AD8: 4AFA73F9  bl 0x8222ced0
	ctx.lr = 0x83285ADC;
	sub_8222CED0(ctx, base);
	// 83285ADC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83285AE0: 38691DA0  addi r3, r9, 0x1da0
	ctx.r[3].s64 = ctx.r[9].s64 + 7584;
	// 83285AE4: 4BA2443D  bl 0x82ca9f20
	ctx.lr = 0x83285AE8;
	sub_82CA9F20(ctx, base);
	// 83285AE8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83285AEC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83285AF0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83285AF4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83285AF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83285AF8 size=64
    let mut pc: u32 = 0x83285AF8;
    'dispatch: loop {
        match pc {
            0x83285AF8 => {
    //   block [0x83285AF8..0x83285B38)
	// 83285AF8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83285AFC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83285B00: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83285B04: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83285B08: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83285B0C: 388BAE04  addi r4, r11, -0x51fc
	ctx.r[4].s64 = ctx.r[11].s64 + -20988;
	// 83285B10: 386AE6E8  addi r3, r10, -0x1918
	ctx.r[3].s64 = ctx.r[10].s64 + -6424;
	// 83285B14: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83285B18: 4AFA73B9  bl 0x8222ced0
	ctx.lr = 0x83285B1C;
	sub_8222CED0(ctx, base);
	// 83285B1C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83285B20: 38691DB0  addi r3, r9, 0x1db0
	ctx.r[3].s64 = ctx.r[9].s64 + 7600;
	// 83285B24: 4BA243FD  bl 0x82ca9f20
	ctx.lr = 0x83285B28;
	sub_82CA9F20(ctx, base);
	// 83285B28: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83285B2C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83285B30: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83285B34: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83285B38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83285B38 size=64
    let mut pc: u32 = 0x83285B38;
    'dispatch: loop {
        match pc {
            0x83285B38 => {
    //   block [0x83285B38..0x83285B78)
	// 83285B38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83285B3C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83285B40: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83285B44: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83285B48: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83285B4C: 388BAE18  addi r4, r11, -0x51e8
	ctx.r[4].s64 = ctx.r[11].s64 + -20968;
	// 83285B50: 386AE6EC  addi r3, r10, -0x1914
	ctx.r[3].s64 = ctx.r[10].s64 + -6420;
	// 83285B54: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83285B58: 4AFA7379  bl 0x8222ced0
	ctx.lr = 0x83285B5C;
	sub_8222CED0(ctx, base);
	// 83285B5C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83285B60: 38691DC0  addi r3, r9, 0x1dc0
	ctx.r[3].s64 = ctx.r[9].s64 + 7616;
	// 83285B64: 4BA243BD  bl 0x82ca9f20
	ctx.lr = 0x83285B68;
	sub_82CA9F20(ctx, base);
	// 83285B68: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83285B6C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83285B70: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83285B74: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83285B78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83285B78 size=64
    let mut pc: u32 = 0x83285B78;
    'dispatch: loop {
        match pc {
            0x83285B78 => {
    //   block [0x83285B78..0x83285BB8)
	// 83285B78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83285B7C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83285B80: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83285B84: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83285B88: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83285B8C: 388BAE2C  addi r4, r11, -0x51d4
	ctx.r[4].s64 = ctx.r[11].s64 + -20948;
	// 83285B90: 386AE6F0  addi r3, r10, -0x1910
	ctx.r[3].s64 = ctx.r[10].s64 + -6416;
	// 83285B94: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83285B98: 4AFA7339  bl 0x8222ced0
	ctx.lr = 0x83285B9C;
	sub_8222CED0(ctx, base);
	// 83285B9C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83285BA0: 38691DD0  addi r3, r9, 0x1dd0
	ctx.r[3].s64 = ctx.r[9].s64 + 7632;
	// 83285BA4: 4BA2437D  bl 0x82ca9f20
	ctx.lr = 0x83285BA8;
	sub_82CA9F20(ctx, base);
	// 83285BA8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83285BAC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83285BB0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83285BB4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83285BB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83285BB8 size=64
    let mut pc: u32 = 0x83285BB8;
    'dispatch: loop {
        match pc {
            0x83285BB8 => {
    //   block [0x83285BB8..0x83285BF8)
	// 83285BB8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83285BBC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83285BC0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83285BC4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83285BC8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83285BCC: 388BADB4  addi r4, r11, -0x524c
	ctx.r[4].s64 = ctx.r[11].s64 + -21068;
	// 83285BD0: 386AE6F4  addi r3, r10, -0x190c
	ctx.r[3].s64 = ctx.r[10].s64 + -6412;
	// 83285BD4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83285BD8: 4AFA72F9  bl 0x8222ced0
	ctx.lr = 0x83285BDC;
	sub_8222CED0(ctx, base);
	// 83285BDC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83285BE0: 38691DE0  addi r3, r9, 0x1de0
	ctx.r[3].s64 = ctx.r[9].s64 + 7648;
	// 83285BE4: 4BA2433D  bl 0x82ca9f20
	ctx.lr = 0x83285BE8;
	sub_82CA9F20(ctx, base);
	// 83285BE8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83285BEC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83285BF0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83285BF4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83285BF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83285BF8 size=64
    let mut pc: u32 = 0x83285BF8;
    'dispatch: loop {
        match pc {
            0x83285BF8 => {
    //   block [0x83285BF8..0x83285C38)
	// 83285BF8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83285BFC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83285C00: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83285C04: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83285C08: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83285C0C: 388BADBC  addi r4, r11, -0x5244
	ctx.r[4].s64 = ctx.r[11].s64 + -21060;
	// 83285C10: 386AE6F8  addi r3, r10, -0x1908
	ctx.r[3].s64 = ctx.r[10].s64 + -6408;
	// 83285C14: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83285C18: 4AFA72B9  bl 0x8222ced0
	ctx.lr = 0x83285C1C;
	sub_8222CED0(ctx, base);
	// 83285C1C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83285C20: 38691DF0  addi r3, r9, 0x1df0
	ctx.r[3].s64 = ctx.r[9].s64 + 7664;
	// 83285C24: 4BA242FD  bl 0x82ca9f20
	ctx.lr = 0x83285C28;
	sub_82CA9F20(ctx, base);
	// 83285C28: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83285C2C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83285C30: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83285C34: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83285C38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83285C38 size=64
    let mut pc: u32 = 0x83285C38;
    'dispatch: loop {
        match pc {
            0x83285C38 => {
    //   block [0x83285C38..0x83285C78)
	// 83285C38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83285C3C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83285C40: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83285C44: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83285C48: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83285C4C: 388BADD0  addi r4, r11, -0x5230
	ctx.r[4].s64 = ctx.r[11].s64 + -21040;
	// 83285C50: 386AE6FC  addi r3, r10, -0x1904
	ctx.r[3].s64 = ctx.r[10].s64 + -6404;
	// 83285C54: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83285C58: 4AFA7279  bl 0x8222ced0
	ctx.lr = 0x83285C5C;
	sub_8222CED0(ctx, base);
	// 83285C5C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83285C60: 38691E00  addi r3, r9, 0x1e00
	ctx.r[3].s64 = ctx.r[9].s64 + 7680;
	// 83285C64: 4BA242BD  bl 0x82ca9f20
	ctx.lr = 0x83285C68;
	sub_82CA9F20(ctx, base);
	// 83285C68: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83285C6C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83285C70: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83285C74: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83285C78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83285C78 size=64
    let mut pc: u32 = 0x83285C78;
    'dispatch: loop {
        match pc {
            0x83285C78 => {
    //   block [0x83285C78..0x83285CB8)
	// 83285C78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83285C7C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83285C80: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83285C84: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83285C88: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83285C8C: 388BADE4  addi r4, r11, -0x521c
	ctx.r[4].s64 = ctx.r[11].s64 + -21020;
	// 83285C90: 386AE700  addi r3, r10, -0x1900
	ctx.r[3].s64 = ctx.r[10].s64 + -6400;
	// 83285C94: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83285C98: 4AFA7239  bl 0x8222ced0
	ctx.lr = 0x83285C9C;
	sub_8222CED0(ctx, base);
	// 83285C9C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83285CA0: 38691E10  addi r3, r9, 0x1e10
	ctx.r[3].s64 = ctx.r[9].s64 + 7696;
	// 83285CA4: 4BA2427D  bl 0x82ca9f20
	ctx.lr = 0x83285CA8;
	sub_82CA9F20(ctx, base);
	// 83285CA8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83285CAC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83285CB0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83285CB4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83285CB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83285CB8 size=64
    let mut pc: u32 = 0x83285CB8;
    'dispatch: loop {
        match pc {
            0x83285CB8 => {
    //   block [0x83285CB8..0x83285CF8)
	// 83285CB8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83285CBC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83285CC0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83285CC4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83285CC8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83285CCC: 388BAD60  addi r4, r11, -0x52a0
	ctx.r[4].s64 = ctx.r[11].s64 + -21152;
	// 83285CD0: 386AE704  addi r3, r10, -0x18fc
	ctx.r[3].s64 = ctx.r[10].s64 + -6396;
	// 83285CD4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83285CD8: 4AFA71F9  bl 0x8222ced0
	ctx.lr = 0x83285CDC;
	sub_8222CED0(ctx, base);
	// 83285CDC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83285CE0: 38691E20  addi r3, r9, 0x1e20
	ctx.r[3].s64 = ctx.r[9].s64 + 7712;
	// 83285CE4: 4BA2423D  bl 0x82ca9f20
	ctx.lr = 0x83285CE8;
	sub_82CA9F20(ctx, base);
	// 83285CE8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83285CEC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83285CF0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83285CF4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83285CF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83285CF8 size=64
    let mut pc: u32 = 0x83285CF8;
    'dispatch: loop {
        match pc {
            0x83285CF8 => {
    //   block [0x83285CF8..0x83285D38)
	// 83285CF8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83285CFC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83285D00: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83285D04: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83285D08: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83285D0C: 388BAD68  addi r4, r11, -0x5298
	ctx.r[4].s64 = ctx.r[11].s64 + -21144;
	// 83285D10: 386AE708  addi r3, r10, -0x18f8
	ctx.r[3].s64 = ctx.r[10].s64 + -6392;
	// 83285D14: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83285D18: 4AFA71B9  bl 0x8222ced0
	ctx.lr = 0x83285D1C;
	sub_8222CED0(ctx, base);
	// 83285D1C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83285D20: 38691E30  addi r3, r9, 0x1e30
	ctx.r[3].s64 = ctx.r[9].s64 + 7728;
	// 83285D24: 4BA241FD  bl 0x82ca9f20
	ctx.lr = 0x83285D28;
	sub_82CA9F20(ctx, base);
	// 83285D28: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83285D2C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83285D30: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83285D34: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83285D38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83285D38 size=64
    let mut pc: u32 = 0x83285D38;
    'dispatch: loop {
        match pc {
            0x83285D38 => {
    //   block [0x83285D38..0x83285D78)
	// 83285D38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83285D3C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83285D40: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83285D44: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83285D48: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83285D4C: 388BAD7C  addi r4, r11, -0x5284
	ctx.r[4].s64 = ctx.r[11].s64 + -21124;
	// 83285D50: 386AE70C  addi r3, r10, -0x18f4
	ctx.r[3].s64 = ctx.r[10].s64 + -6388;
	// 83285D54: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83285D58: 4AFA7179  bl 0x8222ced0
	ctx.lr = 0x83285D5C;
	sub_8222CED0(ctx, base);
	// 83285D5C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83285D60: 38691E40  addi r3, r9, 0x1e40
	ctx.r[3].s64 = ctx.r[9].s64 + 7744;
	// 83285D64: 4BA241BD  bl 0x82ca9f20
	ctx.lr = 0x83285D68;
	sub_82CA9F20(ctx, base);
	// 83285D68: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83285D6C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83285D70: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83285D74: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83285D78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83285D78 size=64
    let mut pc: u32 = 0x83285D78;
    'dispatch: loop {
        match pc {
            0x83285D78 => {
    //   block [0x83285D78..0x83285DB8)
	// 83285D78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83285D7C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83285D80: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83285D84: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83285D88: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83285D8C: 388BAD90  addi r4, r11, -0x5270
	ctx.r[4].s64 = ctx.r[11].s64 + -21104;
	// 83285D90: 386AE710  addi r3, r10, -0x18f0
	ctx.r[3].s64 = ctx.r[10].s64 + -6384;
	// 83285D94: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83285D98: 4AFA7139  bl 0x8222ced0
	ctx.lr = 0x83285D9C;
	sub_8222CED0(ctx, base);
	// 83285D9C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83285DA0: 38691E50  addi r3, r9, 0x1e50
	ctx.r[3].s64 = ctx.r[9].s64 + 7760;
	// 83285DA4: 4BA2417D  bl 0x82ca9f20
	ctx.lr = 0x83285DA8;
	sub_82CA9F20(ctx, base);
	// 83285DA8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83285DAC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83285DB0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83285DB4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83285DB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83285DB8 size=64
    let mut pc: u32 = 0x83285DB8;
    'dispatch: loop {
        match pc {
            0x83285DB8 => {
    //   block [0x83285DB8..0x83285DF8)
	// 83285DB8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83285DBC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83285DC0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83285DC4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83285DC8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83285DCC: 388BAD0C  addi r4, r11, -0x52f4
	ctx.r[4].s64 = ctx.r[11].s64 + -21236;
	// 83285DD0: 386AE714  addi r3, r10, -0x18ec
	ctx.r[3].s64 = ctx.r[10].s64 + -6380;
	// 83285DD4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83285DD8: 4AFA70F9  bl 0x8222ced0
	ctx.lr = 0x83285DDC;
	sub_8222CED0(ctx, base);
	// 83285DDC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83285DE0: 38691E60  addi r3, r9, 0x1e60
	ctx.r[3].s64 = ctx.r[9].s64 + 7776;
	// 83285DE4: 4BA2413D  bl 0x82ca9f20
	ctx.lr = 0x83285DE8;
	sub_82CA9F20(ctx, base);
	// 83285DE8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83285DEC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83285DF0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83285DF4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83285DF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83285DF8 size=64
    let mut pc: u32 = 0x83285DF8;
    'dispatch: loop {
        match pc {
            0x83285DF8 => {
    //   block [0x83285DF8..0x83285E38)
	// 83285DF8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83285DFC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83285E00: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83285E04: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83285E08: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83285E0C: 388BAD14  addi r4, r11, -0x52ec
	ctx.r[4].s64 = ctx.r[11].s64 + -21228;
	// 83285E10: 386AE718  addi r3, r10, -0x18e8
	ctx.r[3].s64 = ctx.r[10].s64 + -6376;
	// 83285E14: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83285E18: 4AFA70B9  bl 0x8222ced0
	ctx.lr = 0x83285E1C;
	sub_8222CED0(ctx, base);
	// 83285E1C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83285E20: 38691E70  addi r3, r9, 0x1e70
	ctx.r[3].s64 = ctx.r[9].s64 + 7792;
	// 83285E24: 4BA240FD  bl 0x82ca9f20
	ctx.lr = 0x83285E28;
	sub_82CA9F20(ctx, base);
	// 83285E28: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83285E2C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83285E30: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83285E34: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83285E38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83285E38 size=64
    let mut pc: u32 = 0x83285E38;
    'dispatch: loop {
        match pc {
            0x83285E38 => {
    //   block [0x83285E38..0x83285E78)
	// 83285E38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83285E3C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83285E40: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83285E44: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83285E48: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83285E4C: 388BAD28  addi r4, r11, -0x52d8
	ctx.r[4].s64 = ctx.r[11].s64 + -21208;
	// 83285E50: 386AE71C  addi r3, r10, -0x18e4
	ctx.r[3].s64 = ctx.r[10].s64 + -6372;
	// 83285E54: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83285E58: 4AFA7079  bl 0x8222ced0
	ctx.lr = 0x83285E5C;
	sub_8222CED0(ctx, base);
	// 83285E5C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83285E60: 38691E80  addi r3, r9, 0x1e80
	ctx.r[3].s64 = ctx.r[9].s64 + 7808;
	// 83285E64: 4BA240BD  bl 0x82ca9f20
	ctx.lr = 0x83285E68;
	sub_82CA9F20(ctx, base);
	// 83285E68: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83285E6C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83285E70: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83285E74: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83285E78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83285E78 size=64
    let mut pc: u32 = 0x83285E78;
    'dispatch: loop {
        match pc {
            0x83285E78 => {
    //   block [0x83285E78..0x83285EB8)
	// 83285E78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83285E7C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83285E80: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83285E84: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83285E88: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83285E8C: 388BAD3C  addi r4, r11, -0x52c4
	ctx.r[4].s64 = ctx.r[11].s64 + -21188;
	// 83285E90: 386AE720  addi r3, r10, -0x18e0
	ctx.r[3].s64 = ctx.r[10].s64 + -6368;
	// 83285E94: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83285E98: 4AFA7039  bl 0x8222ced0
	ctx.lr = 0x83285E9C;
	sub_8222CED0(ctx, base);
	// 83285E9C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83285EA0: 38691E90  addi r3, r9, 0x1e90
	ctx.r[3].s64 = ctx.r[9].s64 + 7824;
	// 83285EA4: 4BA2407D  bl 0x82ca9f20
	ctx.lr = 0x83285EA8;
	sub_82CA9F20(ctx, base);
	// 83285EA8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83285EAC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83285EB0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83285EB4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83285EB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83285EB8 size=64
    let mut pc: u32 = 0x83285EB8;
    'dispatch: loop {
        match pc {
            0x83285EB8 => {
    //   block [0x83285EB8..0x83285EF8)
	// 83285EB8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83285EBC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83285EC0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83285EC4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83285EC8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83285ECC: 388BACA0  addi r4, r11, -0x5360
	ctx.r[4].s64 = ctx.r[11].s64 + -21344;
	// 83285ED0: 386AE724  addi r3, r10, -0x18dc
	ctx.r[3].s64 = ctx.r[10].s64 + -6364;
	// 83285ED4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83285ED8: 4AFA6FF9  bl 0x8222ced0
	ctx.lr = 0x83285EDC;
	sub_8222CED0(ctx, base);
	// 83285EDC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83285EE0: 38691EA0  addi r3, r9, 0x1ea0
	ctx.r[3].s64 = ctx.r[9].s64 + 7840;
	// 83285EE4: 4BA2403D  bl 0x82ca9f20
	ctx.lr = 0x83285EE8;
	sub_82CA9F20(ctx, base);
	// 83285EE8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83285EEC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83285EF0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83285EF4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83285EF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83285EF8 size=64
    let mut pc: u32 = 0x83285EF8;
    'dispatch: loop {
        match pc {
            0x83285EF8 => {
    //   block [0x83285EF8..0x83285F38)
	// 83285EF8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83285EFC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83285F00: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83285F04: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83285F08: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83285F0C: 388BACB8  addi r4, r11, -0x5348
	ctx.r[4].s64 = ctx.r[11].s64 + -21320;
	// 83285F10: 386AE728  addi r3, r10, -0x18d8
	ctx.r[3].s64 = ctx.r[10].s64 + -6360;
	// 83285F14: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83285F18: 4AFA6FB9  bl 0x8222ced0
	ctx.lr = 0x83285F1C;
	sub_8222CED0(ctx, base);
	// 83285F1C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83285F20: 38691EB0  addi r3, r9, 0x1eb0
	ctx.r[3].s64 = ctx.r[9].s64 + 7856;
	// 83285F24: 4BA23FFD  bl 0x82ca9f20
	ctx.lr = 0x83285F28;
	sub_82CA9F20(ctx, base);
	// 83285F28: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83285F2C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83285F30: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83285F34: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83285F38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83285F38 size=64
    let mut pc: u32 = 0x83285F38;
    'dispatch: loop {
        match pc {
            0x83285F38 => {
    //   block [0x83285F38..0x83285F78)
	// 83285F38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83285F3C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83285F40: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83285F44: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83285F48: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83285F4C: 388BACC8  addi r4, r11, -0x5338
	ctx.r[4].s64 = ctx.r[11].s64 + -21304;
	// 83285F50: 386AE72C  addi r3, r10, -0x18d4
	ctx.r[3].s64 = ctx.r[10].s64 + -6356;
	// 83285F54: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83285F58: 4AFA6F79  bl 0x8222ced0
	ctx.lr = 0x83285F5C;
	sub_8222CED0(ctx, base);
	// 83285F5C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83285F60: 38691EC0  addi r3, r9, 0x1ec0
	ctx.r[3].s64 = ctx.r[9].s64 + 7872;
	// 83285F64: 4BA23FBD  bl 0x82ca9f20
	ctx.lr = 0x83285F68;
	sub_82CA9F20(ctx, base);
	// 83285F68: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83285F6C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83285F70: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83285F74: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83285F78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83285F78 size=64
    let mut pc: u32 = 0x83285F78;
    'dispatch: loop {
        match pc {
            0x83285F78 => {
    //   block [0x83285F78..0x83285FB8)
	// 83285F78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83285F7C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83285F80: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83285F84: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83285F88: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83285F8C: 388BACCC  addi r4, r11, -0x5334
	ctx.r[4].s64 = ctx.r[11].s64 + -21300;
	// 83285F90: 386AE730  addi r3, r10, -0x18d0
	ctx.r[3].s64 = ctx.r[10].s64 + -6352;
	// 83285F94: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83285F98: 4AFA6F39  bl 0x8222ced0
	ctx.lr = 0x83285F9C;
	sub_8222CED0(ctx, base);
	// 83285F9C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83285FA0: 38691ED0  addi r3, r9, 0x1ed0
	ctx.r[3].s64 = ctx.r[9].s64 + 7888;
	// 83285FA4: 4BA23F7D  bl 0x82ca9f20
	ctx.lr = 0x83285FA8;
	sub_82CA9F20(ctx, base);
	// 83285FA8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83285FAC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83285FB0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83285FB4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83285FB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83285FB8 size=64
    let mut pc: u32 = 0x83285FB8;
    'dispatch: loop {
        match pc {
            0x83285FB8 => {
    //   block [0x83285FB8..0x83285FF8)
	// 83285FB8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83285FBC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83285FC0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83285FC4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83285FC8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83285FCC: 388BACE0  addi r4, r11, -0x5320
	ctx.r[4].s64 = ctx.r[11].s64 + -21280;
	// 83285FD0: 386AE734  addi r3, r10, -0x18cc
	ctx.r[3].s64 = ctx.r[10].s64 + -6348;
	// 83285FD4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83285FD8: 4AFA6EF9  bl 0x8222ced0
	ctx.lr = 0x83285FDC;
	sub_8222CED0(ctx, base);
	// 83285FDC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83285FE0: 38691EE0  addi r3, r9, 0x1ee0
	ctx.r[3].s64 = ctx.r[9].s64 + 7904;
	// 83285FE4: 4BA23F3D  bl 0x82ca9f20
	ctx.lr = 0x83285FE8;
	sub_82CA9F20(ctx, base);
	// 83285FE8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83285FEC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83285FF0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83285FF4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83285FF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83285FF8 size=64
    let mut pc: u32 = 0x83285FF8;
    'dispatch: loop {
        match pc {
            0x83285FF8 => {
    //   block [0x83285FF8..0x83286038)
	// 83285FF8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83285FFC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83286000: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83286004: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83286008: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8328600C: 388BACF0  addi r4, r11, -0x5310
	ctx.r[4].s64 = ctx.r[11].s64 + -21264;
	// 83286010: 386AE738  addi r3, r10, -0x18c8
	ctx.r[3].s64 = ctx.r[10].s64 + -6344;
	// 83286014: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83286018: 4AFA6EB9  bl 0x8222ced0
	ctx.lr = 0x8328601C;
	sub_8222CED0(ctx, base);
	// 8328601C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83286020: 38691EF0  addi r3, r9, 0x1ef0
	ctx.r[3].s64 = ctx.r[9].s64 + 7920;
	// 83286024: 4BA23EFD  bl 0x82ca9f20
	ctx.lr = 0x83286028;
	sub_82CA9F20(ctx, base);
	// 83286028: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8328602C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83286030: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83286034: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83286038(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83286038 size=64
    let mut pc: u32 = 0x83286038;
    'dispatch: loop {
        match pc {
            0x83286038 => {
    //   block [0x83286038..0x83286078)
	// 83286038: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8328603C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83286040: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83286044: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83286048: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8328604C: 388BAAAC  addi r4, r11, -0x5554
	ctx.r[4].s64 = ctx.r[11].s64 + -21844;
	// 83286050: 386AE73C  addi r3, r10, -0x18c4
	ctx.r[3].s64 = ctx.r[10].s64 + -6340;
	// 83286054: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83286058: 4AFA6E79  bl 0x8222ced0
	ctx.lr = 0x8328605C;
	sub_8222CED0(ctx, base);
	// 8328605C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83286060: 38691F00  addi r3, r9, 0x1f00
	ctx.r[3].s64 = ctx.r[9].s64 + 7936;
	// 83286064: 4BA23EBD  bl 0x82ca9f20
	ctx.lr = 0x83286068;
	sub_82CA9F20(ctx, base);
	// 83286068: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8328606C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83286070: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83286074: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83286078(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83286078 size=64
    let mut pc: u32 = 0x83286078;
    'dispatch: loop {
        match pc {
            0x83286078 => {
    //   block [0x83286078..0x832860B8)
	// 83286078: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8328607C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83286080: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83286084: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83286088: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8328608C: 388BAAB4  addi r4, r11, -0x554c
	ctx.r[4].s64 = ctx.r[11].s64 + -21836;
	// 83286090: 386AE740  addi r3, r10, -0x18c0
	ctx.r[3].s64 = ctx.r[10].s64 + -6336;
	// 83286094: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83286098: 4AFA6E39  bl 0x8222ced0
	ctx.lr = 0x8328609C;
	sub_8222CED0(ctx, base);
	// 8328609C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 832860A0: 38691F10  addi r3, r9, 0x1f10
	ctx.r[3].s64 = ctx.r[9].s64 + 7952;
	// 832860A4: 4BA23E7D  bl 0x82ca9f20
	ctx.lr = 0x832860A8;
	sub_82CA9F20(ctx, base);
	// 832860A8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832860AC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832860B0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832860B4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832860B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832860B8 size=64
    let mut pc: u32 = 0x832860B8;
    'dispatch: loop {
        match pc {
            0x832860B8 => {
    //   block [0x832860B8..0x832860F8)
	// 832860B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832860BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832860C0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832860C4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 832860C8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832860CC: 388BAAC8  addi r4, r11, -0x5538
	ctx.r[4].s64 = ctx.r[11].s64 + -21816;
	// 832860D0: 386AE744  addi r3, r10, -0x18bc
	ctx.r[3].s64 = ctx.r[10].s64 + -6332;
	// 832860D4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832860D8: 4AFA6DF9  bl 0x8222ced0
	ctx.lr = 0x832860DC;
	sub_8222CED0(ctx, base);
	// 832860DC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 832860E0: 38691F20  addi r3, r9, 0x1f20
	ctx.r[3].s64 = ctx.r[9].s64 + 7968;
	// 832860E4: 4BA23E3D  bl 0x82ca9f20
	ctx.lr = 0x832860E8;
	sub_82CA9F20(ctx, base);
	// 832860E8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832860EC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832860F0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832860F4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832860F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832860F8 size=64
    let mut pc: u32 = 0x832860F8;
    'dispatch: loop {
        match pc {
            0x832860F8 => {
    //   block [0x832860F8..0x83286138)
	// 832860F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832860FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83286100: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83286104: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83286108: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8328610C: 388BAADC  addi r4, r11, -0x5524
	ctx.r[4].s64 = ctx.r[11].s64 + -21796;
	// 83286110: 386AE748  addi r3, r10, -0x18b8
	ctx.r[3].s64 = ctx.r[10].s64 + -6328;
	// 83286114: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83286118: 4AFA6DB9  bl 0x8222ced0
	ctx.lr = 0x8328611C;
	sub_8222CED0(ctx, base);
	// 8328611C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83286120: 38691F30  addi r3, r9, 0x1f30
	ctx.r[3].s64 = ctx.r[9].s64 + 7984;
	// 83286124: 4BA23DFD  bl 0x82ca9f20
	ctx.lr = 0x83286128;
	sub_82CA9F20(ctx, base);
	// 83286128: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8328612C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83286130: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83286134: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83286138(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83286138 size=64
    let mut pc: u32 = 0x83286138;
    'dispatch: loop {
        match pc {
            0x83286138 => {
    //   block [0x83286138..0x83286178)
	// 83286138: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8328613C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83286140: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83286144: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83286148: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8328614C: 388BAB00  addi r4, r11, -0x5500
	ctx.r[4].s64 = ctx.r[11].s64 + -21760;
	// 83286150: 386AE74C  addi r3, r10, -0x18b4
	ctx.r[3].s64 = ctx.r[10].s64 + -6324;
	// 83286154: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83286158: 4AFA6D79  bl 0x8222ced0
	ctx.lr = 0x8328615C;
	sub_8222CED0(ctx, base);
	// 8328615C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83286160: 38691F40  addi r3, r9, 0x1f40
	ctx.r[3].s64 = ctx.r[9].s64 + 8000;
	// 83286164: 4BA23DBD  bl 0x82ca9f20
	ctx.lr = 0x83286168;
	sub_82CA9F20(ctx, base);
	// 83286168: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8328616C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83286170: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83286174: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83286178(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83286178 size=64
    let mut pc: u32 = 0x83286178;
    'dispatch: loop {
        match pc {
            0x83286178 => {
    //   block [0x83286178..0x832861B8)
	// 83286178: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8328617C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83286180: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83286184: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83286188: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8328618C: 388BAB08  addi r4, r11, -0x54f8
	ctx.r[4].s64 = ctx.r[11].s64 + -21752;
	// 83286190: 386AE750  addi r3, r10, -0x18b0
	ctx.r[3].s64 = ctx.r[10].s64 + -6320;
	// 83286194: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83286198: 4AFA6D39  bl 0x8222ced0
	ctx.lr = 0x8328619C;
	sub_8222CED0(ctx, base);
	// 8328619C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 832861A0: 38691F50  addi r3, r9, 0x1f50
	ctx.r[3].s64 = ctx.r[9].s64 + 8016;
	// 832861A4: 4BA23D7D  bl 0x82ca9f20
	ctx.lr = 0x832861A8;
	sub_82CA9F20(ctx, base);
	// 832861A8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832861AC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832861B0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832861B4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832861B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832861B8 size=64
    let mut pc: u32 = 0x832861B8;
    'dispatch: loop {
        match pc {
            0x832861B8 => {
    //   block [0x832861B8..0x832861F8)
	// 832861B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832861BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832861C0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832861C4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 832861C8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832861CC: 388BAB28  addi r4, r11, -0x54d8
	ctx.r[4].s64 = ctx.r[11].s64 + -21720;
	// 832861D0: 386AE754  addi r3, r10, -0x18ac
	ctx.r[3].s64 = ctx.r[10].s64 + -6316;
	// 832861D4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832861D8: 4AFA6CF9  bl 0x8222ced0
	ctx.lr = 0x832861DC;
	sub_8222CED0(ctx, base);
	// 832861DC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 832861E0: 38691F60  addi r3, r9, 0x1f60
	ctx.r[3].s64 = ctx.r[9].s64 + 8032;
	// 832861E4: 4BA23D3D  bl 0x82ca9f20
	ctx.lr = 0x832861E8;
	sub_82CA9F20(ctx, base);
	// 832861E8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832861EC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832861F0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832861F4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832861F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832861F8 size=64
    let mut pc: u32 = 0x832861F8;
    'dispatch: loop {
        match pc {
            0x832861F8 => {
    //   block [0x832861F8..0x83286238)
	// 832861F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832861FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83286200: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83286204: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83286208: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8328620C: 388BAB48  addi r4, r11, -0x54b8
	ctx.r[4].s64 = ctx.r[11].s64 + -21688;
	// 83286210: 386AE758  addi r3, r10, -0x18a8
	ctx.r[3].s64 = ctx.r[10].s64 + -6312;
	// 83286214: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83286218: 4AFA6CB9  bl 0x8222ced0
	ctx.lr = 0x8328621C;
	sub_8222CED0(ctx, base);
	// 8328621C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83286220: 38691F70  addi r3, r9, 0x1f70
	ctx.r[3].s64 = ctx.r[9].s64 + 8048;
	// 83286224: 4BA23CFD  bl 0x82ca9f20
	ctx.lr = 0x83286228;
	sub_82CA9F20(ctx, base);
	// 83286228: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8328622C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83286230: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83286234: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83286238(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83286238 size=64
    let mut pc: u32 = 0x83286238;
    'dispatch: loop {
        match pc {
            0x83286238 => {
    //   block [0x83286238..0x83286278)
	// 83286238: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8328623C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83286240: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83286244: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83286248: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8328624C: 388BAB50  addi r4, r11, -0x54b0
	ctx.r[4].s64 = ctx.r[11].s64 + -21680;
	// 83286250: 386AE75C  addi r3, r10, -0x18a4
	ctx.r[3].s64 = ctx.r[10].s64 + -6308;
	// 83286254: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83286258: 4AFA6C79  bl 0x8222ced0
	ctx.lr = 0x8328625C;
	sub_8222CED0(ctx, base);
	// 8328625C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83286260: 38691F80  addi r3, r9, 0x1f80
	ctx.r[3].s64 = ctx.r[9].s64 + 8064;
	// 83286264: 4BA23CBD  bl 0x82ca9f20
	ctx.lr = 0x83286268;
	sub_82CA9F20(ctx, base);
	// 83286268: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8328626C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83286270: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83286274: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83286278(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83286278 size=64
    let mut pc: u32 = 0x83286278;
    'dispatch: loop {
        match pc {
            0x83286278 => {
    //   block [0x83286278..0x832862B8)
	// 83286278: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8328627C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83286280: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83286284: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83286288: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8328628C: 388BAB58  addi r4, r11, -0x54a8
	ctx.r[4].s64 = ctx.r[11].s64 + -21672;
	// 83286290: 386AE760  addi r3, r10, -0x18a0
	ctx.r[3].s64 = ctx.r[10].s64 + -6304;
	// 83286294: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83286298: 4AFA6C39  bl 0x8222ced0
	ctx.lr = 0x8328629C;
	sub_8222CED0(ctx, base);
	// 8328629C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 832862A0: 38691F90  addi r3, r9, 0x1f90
	ctx.r[3].s64 = ctx.r[9].s64 + 8080;
	// 832862A4: 4BA23C7D  bl 0x82ca9f20
	ctx.lr = 0x832862A8;
	sub_82CA9F20(ctx, base);
	// 832862A8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832862AC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832862B0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832862B4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832862B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832862B8 size=64
    let mut pc: u32 = 0x832862B8;
    'dispatch: loop {
        match pc {
            0x832862B8 => {
    //   block [0x832862B8..0x832862F8)
	// 832862B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832862BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832862C0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832862C4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 832862C8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832862CC: 388BAB68  addi r4, r11, -0x5498
	ctx.r[4].s64 = ctx.r[11].s64 + -21656;
	// 832862D0: 386AE764  addi r3, r10, -0x189c
	ctx.r[3].s64 = ctx.r[10].s64 + -6300;
	// 832862D4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832862D8: 4AFA6BF9  bl 0x8222ced0
	ctx.lr = 0x832862DC;
	sub_8222CED0(ctx, base);
	// 832862DC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 832862E0: 38691FA0  addi r3, r9, 0x1fa0
	ctx.r[3].s64 = ctx.r[9].s64 + 8096;
	// 832862E4: 4BA23C3D  bl 0x82ca9f20
	ctx.lr = 0x832862E8;
	sub_82CA9F20(ctx, base);
	// 832862E8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832862EC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832862F0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832862F4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832862F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832862F8 size=64
    let mut pc: u32 = 0x832862F8;
    'dispatch: loop {
        match pc {
            0x832862F8 => {
    //   block [0x832862F8..0x83286338)
	// 832862F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832862FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83286300: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83286304: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83286308: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8328630C: 388BAB84  addi r4, r11, -0x547c
	ctx.r[4].s64 = ctx.r[11].s64 + -21628;
	// 83286310: 386AE768  addi r3, r10, -0x1898
	ctx.r[3].s64 = ctx.r[10].s64 + -6296;
	// 83286314: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83286318: 4AFA6BB9  bl 0x8222ced0
	ctx.lr = 0x8328631C;
	sub_8222CED0(ctx, base);
	// 8328631C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83286320: 38691FB0  addi r3, r9, 0x1fb0
	ctx.r[3].s64 = ctx.r[9].s64 + 8112;
	// 83286324: 4BA23BFD  bl 0x82ca9f20
	ctx.lr = 0x83286328;
	sub_82CA9F20(ctx, base);
	// 83286328: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8328632C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83286330: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83286334: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83286338(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83286338 size=64
    let mut pc: u32 = 0x83286338;
    'dispatch: loop {
        match pc {
            0x83286338 => {
    //   block [0x83286338..0x83286378)
	// 83286338: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8328633C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83286340: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83286344: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83286348: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8328634C: 388BAB90  addi r4, r11, -0x5470
	ctx.r[4].s64 = ctx.r[11].s64 + -21616;
	// 83286350: 386AE76C  addi r3, r10, -0x1894
	ctx.r[3].s64 = ctx.r[10].s64 + -6292;
	// 83286354: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83286358: 4AFA6B79  bl 0x8222ced0
	ctx.lr = 0x8328635C;
	sub_8222CED0(ctx, base);
	// 8328635C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83286360: 38691FC0  addi r3, r9, 0x1fc0
	ctx.r[3].s64 = ctx.r[9].s64 + 8128;
	// 83286364: 4BA23BBD  bl 0x82ca9f20
	ctx.lr = 0x83286368;
	sub_82CA9F20(ctx, base);
	// 83286368: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8328636C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83286370: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83286374: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83286378(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83286378 size=64
    let mut pc: u32 = 0x83286378;
    'dispatch: loop {
        match pc {
            0x83286378 => {
    //   block [0x83286378..0x832863B8)
	// 83286378: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8328637C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83286380: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83286384: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83286388: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8328638C: 388BAB9C  addi r4, r11, -0x5464
	ctx.r[4].s64 = ctx.r[11].s64 + -21604;
	// 83286390: 386AE770  addi r3, r10, -0x1890
	ctx.r[3].s64 = ctx.r[10].s64 + -6288;
	// 83286394: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83286398: 4AFA6B39  bl 0x8222ced0
	ctx.lr = 0x8328639C;
	sub_8222CED0(ctx, base);
	// 8328639C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 832863A0: 38691FD0  addi r3, r9, 0x1fd0
	ctx.r[3].s64 = ctx.r[9].s64 + 8144;
	// 832863A4: 4BA23B7D  bl 0x82ca9f20
	ctx.lr = 0x832863A8;
	sub_82CA9F20(ctx, base);
	// 832863A8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832863AC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832863B0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832863B4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832863B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832863B8 size=64
    let mut pc: u32 = 0x832863B8;
    'dispatch: loop {
        match pc {
            0x832863B8 => {
    //   block [0x832863B8..0x832863F8)
	// 832863B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832863BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832863C0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832863C4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 832863C8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832863CC: 388BABA4  addi r4, r11, -0x545c
	ctx.r[4].s64 = ctx.r[11].s64 + -21596;
	// 832863D0: 386AE774  addi r3, r10, -0x188c
	ctx.r[3].s64 = ctx.r[10].s64 + -6284;
	// 832863D4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832863D8: 4AFA6AF9  bl 0x8222ced0
	ctx.lr = 0x832863DC;
	sub_8222CED0(ctx, base);
	// 832863DC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 832863E0: 38691FE0  addi r3, r9, 0x1fe0
	ctx.r[3].s64 = ctx.r[9].s64 + 8160;
	// 832863E4: 4BA23B3D  bl 0x82ca9f20
	ctx.lr = 0x832863E8;
	sub_82CA9F20(ctx, base);
	// 832863E8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832863EC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832863F0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832863F4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832863F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832863F8 size=64
    let mut pc: u32 = 0x832863F8;
    'dispatch: loop {
        match pc {
            0x832863F8 => {
    //   block [0x832863F8..0x83286438)
	// 832863F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832863FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83286400: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83286404: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83286408: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8328640C: 388BABB0  addi r4, r11, -0x5450
	ctx.r[4].s64 = ctx.r[11].s64 + -21584;
	// 83286410: 386AE778  addi r3, r10, -0x1888
	ctx.r[3].s64 = ctx.r[10].s64 + -6280;
	// 83286414: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83286418: 4AFA6AB9  bl 0x8222ced0
	ctx.lr = 0x8328641C;
	sub_8222CED0(ctx, base);
	// 8328641C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83286420: 38691FF0  addi r3, r9, 0x1ff0
	ctx.r[3].s64 = ctx.r[9].s64 + 8176;
	// 83286424: 4BA23AFD  bl 0x82ca9f20
	ctx.lr = 0x83286428;
	sub_82CA9F20(ctx, base);
	// 83286428: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8328642C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83286430: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83286434: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83286438(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83286438 size=64
    let mut pc: u32 = 0x83286438;
    'dispatch: loop {
        match pc {
            0x83286438 => {
    //   block [0x83286438..0x83286478)
	// 83286438: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8328643C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83286440: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83286444: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83286448: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8328644C: 388BABC0  addi r4, r11, -0x5440
	ctx.r[4].s64 = ctx.r[11].s64 + -21568;
	// 83286450: 386AE77C  addi r3, r10, -0x1884
	ctx.r[3].s64 = ctx.r[10].s64 + -6276;
	// 83286454: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83286458: 4AFA6A79  bl 0x8222ced0
	ctx.lr = 0x8328645C;
	sub_8222CED0(ctx, base);
	// 8328645C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83286460: 38692000  addi r3, r9, 0x2000
	ctx.r[3].s64 = ctx.r[9].s64 + 8192;
	// 83286464: 4BA23ABD  bl 0x82ca9f20
	ctx.lr = 0x83286468;
	sub_82CA9F20(ctx, base);
	// 83286468: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8328646C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83286470: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83286474: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83286478(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x83286478 size=312
    let mut pc: u32 = 0x83286478;
    'dispatch: loop {
        match pc {
            0x83286478 => {
    //   block [0x83286478..0x832865B0)
	// 83286478: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8328647C: 4BA22F89  bl 0x82ca9404
	ctx.lr = 0x83286480;
	sub_82CA93D0(ctx, base);
	// 83286480: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83286484: 38E1FFA8  addi r7, r1, -0x58
	ctx.r[7].s64 = ctx.r[1].s64 + -88;
	// 83286488: 392B9490  addi r9, r11, -0x6b70
	ctx.r[9].s64 = ctx.r[11].s64 + -27504;
	// 8328648C: 38C1FFAC  addi r6, r1, -0x54
	ctx.r[6].s64 = ctx.r[1].s64 + -84;
	// 83286490: 3941FFA0  addi r10, r1, -0x60
	ctx.r[10].s64 = ctx.r[1].s64 + -96;
	// 83286494: 3901FFA4  addi r8, r1, -0x5c
	ctx.r[8].s64 = ctx.r[1].s64 + -92;
	// 83286498: C1AB9490  lfs f13, -0x6b70(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-27504 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 8328649C: 3BC1FFC0  addi r30, r1, -0x40
	ctx.r[30].s64 = ctx.r[1].s64 + -64;
	// 832864A0: D1A1FFA0  stfs f13, -0x60(r1)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-96 as u32), tmp.u32 ) };
	// 832864A4: C009FFF4  lfs f0, -0xc(r9)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(-12 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 832864A8: 3BA1FFC4  addi r29, r1, -0x3c
	ctx.r[29].s64 = ctx.r[1].s64 + -60;
	// 832864AC: D001FFA8  stfs f0, -0x58(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-88 as u32), tmp.u32 ) };
	// 832864B0: 38A1FFB0  addi r5, r1, -0x50
	ctx.r[5].s64 = ctx.r[1].s64 + -80;
	// 832864B4: D001FFAC  stfs f0, -0x54(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-84 as u32), tmp.u32 ) };
	// 832864B8: 3921FFB8  addi r9, r1, -0x48
	ctx.r[9].s64 = ctx.r[1].s64 + -72;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832865B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832865B0 size=64
    let mut pc: u32 = 0x832865B0;
    'dispatch: loop {
        match pc {
            0x832865B0 => {
    //   block [0x832865B0..0x832865F0)
	// 832865B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832865B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832865B8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832865BC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 832865C0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832865C4: 388BAA44  addi r4, r11, -0x55bc
	ctx.r[4].s64 = ctx.r[11].s64 + -21948;
	// 832865C8: 386AE7C0  addi r3, r10, -0x1840
	ctx.r[3].s64 = ctx.r[10].s64 + -6208;
	// 832865CC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832865D0: 4AFA6901  bl 0x8222ced0
	ctx.lr = 0x832865D4;
	sub_8222CED0(ctx, base);
	// 832865D4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 832865D8: 38692118  addi r3, r9, 0x2118
	ctx.r[3].s64 = ctx.r[9].s64 + 8472;
	// 832865DC: 4BA23945  bl 0x82ca9f20
	ctx.lr = 0x832865E0;
	sub_82CA9F20(ctx, base);
	// 832865E0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832865E4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832865E8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832865EC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832865F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832865F0 size=64
    let mut pc: u32 = 0x832865F0;
    'dispatch: loop {
        match pc {
            0x832865F0 => {
    //   block [0x832865F0..0x83286630)
	// 832865F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832865F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832865F8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832865FC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83286600: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83286604: 388BAA4C  addi r4, r11, -0x55b4
	ctx.r[4].s64 = ctx.r[11].s64 + -21940;
	// 83286608: 386AE7C4  addi r3, r10, -0x183c
	ctx.r[3].s64 = ctx.r[10].s64 + -6204;
	// 8328660C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83286610: 4AFA68C1  bl 0x8222ced0
	ctx.lr = 0x83286614;
	sub_8222CED0(ctx, base);
	// 83286614: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83286618: 38692128  addi r3, r9, 0x2128
	ctx.r[3].s64 = ctx.r[9].s64 + 8488;
	// 8328661C: 4BA23905  bl 0x82ca9f20
	ctx.lr = 0x83286620;
	sub_82CA9F20(ctx, base);
	// 83286620: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83286624: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83286628: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8328662C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83286630(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83286630 size=64
    let mut pc: u32 = 0x83286630;
    'dispatch: loop {
        match pc {
            0x83286630 => {
    //   block [0x83286630..0x83286670)
	// 83286630: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83286634: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83286638: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8328663C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83286640: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83286644: 388BAA60  addi r4, r11, -0x55a0
	ctx.r[4].s64 = ctx.r[11].s64 + -21920;
	// 83286648: 386AE7C8  addi r3, r10, -0x1838
	ctx.r[3].s64 = ctx.r[10].s64 + -6200;
	// 8328664C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83286650: 4AFA6881  bl 0x8222ced0
	ctx.lr = 0x83286654;
	sub_8222CED0(ctx, base);
	// 83286654: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83286658: 38692138  addi r3, r9, 0x2138
	ctx.r[3].s64 = ctx.r[9].s64 + 8504;
	// 8328665C: 4BA238C5  bl 0x82ca9f20
	ctx.lr = 0x83286660;
	sub_82CA9F20(ctx, base);
	// 83286660: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83286664: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83286668: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8328666C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83286670(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83286670 size=64
    let mut pc: u32 = 0x83286670;
    'dispatch: loop {
        match pc {
            0x83286670 => {
    //   block [0x83286670..0x832866B0)
	// 83286670: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83286674: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83286678: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8328667C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83286680: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83286684: 388BAA74  addi r4, r11, -0x558c
	ctx.r[4].s64 = ctx.r[11].s64 + -21900;
	// 83286688: 386AE7CC  addi r3, r10, -0x1834
	ctx.r[3].s64 = ctx.r[10].s64 + -6196;
	// 8328668C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83286690: 4AFA6841  bl 0x8222ced0
	ctx.lr = 0x83286694;
	sub_8222CED0(ctx, base);
	// 83286694: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83286698: 38692148  addi r3, r9, 0x2148
	ctx.r[3].s64 = ctx.r[9].s64 + 8520;
	// 8328669C: 4BA23885  bl 0x82ca9f20
	ctx.lr = 0x832866A0;
	sub_82CA9F20(ctx, base);
	// 832866A0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832866A4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832866A8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832866AC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832866B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832866B0 size=64
    let mut pc: u32 = 0x832866B0;
    'dispatch: loop {
        match pc {
            0x832866B0 => {
    //   block [0x832866B0..0x832866F0)
	// 832866B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832866B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832866B8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832866BC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 832866C0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832866C4: 388BAA88  addi r4, r11, -0x5578
	ctx.r[4].s64 = ctx.r[11].s64 + -21880;
	// 832866C8: 386AE7D0  addi r3, r10, -0x1830
	ctx.r[3].s64 = ctx.r[10].s64 + -6192;
	// 832866CC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832866D0: 4AFA6801  bl 0x8222ced0
	ctx.lr = 0x832866D4;
	sub_8222CED0(ctx, base);
	// 832866D4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 832866D8: 38692158  addi r3, r9, 0x2158
	ctx.r[3].s64 = ctx.r[9].s64 + 8536;
	// 832866DC: 4BA23845  bl 0x82ca9f20
	ctx.lr = 0x832866E0;
	sub_82CA9F20(ctx, base);
	// 832866E0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832866E4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832866E8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832866EC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832866F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832866F0 size=64
    let mut pc: u32 = 0x832866F0;
    'dispatch: loop {
        match pc {
            0x832866F0 => {
    //   block [0x832866F0..0x83286730)
	// 832866F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832866F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832866F8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832866FC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83286700: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83286704: 388BAA98  addi r4, r11, -0x5568
	ctx.r[4].s64 = ctx.r[11].s64 + -21864;
	// 83286708: 386AE7D4  addi r3, r10, -0x182c
	ctx.r[3].s64 = ctx.r[10].s64 + -6188;
	// 8328670C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83286710: 4AFA67C1  bl 0x8222ced0
	ctx.lr = 0x83286714;
	sub_8222CED0(ctx, base);
	// 83286714: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83286718: 38692168  addi r3, r9, 0x2168
	ctx.r[3].s64 = ctx.r[9].s64 + 8552;
	// 8328671C: 4BA23805  bl 0x82ca9f20
	ctx.lr = 0x83286720;
	sub_82CA9F20(ctx, base);
	// 83286720: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83286724: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83286728: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8328672C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83286730(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83286730 size=64
    let mut pc: u32 = 0x83286730;
    'dispatch: loop {
        match pc {
            0x83286730 => {
    //   block [0x83286730..0x83286770)
	// 83286730: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83286734: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83286738: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8328673C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83286740: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83286744: 388BA9E0  addi r4, r11, -0x5620
	ctx.r[4].s64 = ctx.r[11].s64 + -22048;
	// 83286748: 386AE7D8  addi r3, r10, -0x1828
	ctx.r[3].s64 = ctx.r[10].s64 + -6184;
	// 8328674C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83286750: 4AFA6781  bl 0x8222ced0
	ctx.lr = 0x83286754;
	sub_8222CED0(ctx, base);
	// 83286754: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83286758: 38692178  addi r3, r9, 0x2178
	ctx.r[3].s64 = ctx.r[9].s64 + 8568;
	// 8328675C: 4BA237C5  bl 0x82ca9f20
	ctx.lr = 0x83286760;
	sub_82CA9F20(ctx, base);
	// 83286760: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83286764: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83286768: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8328676C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83286770(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83286770 size=64
    let mut pc: u32 = 0x83286770;
    'dispatch: loop {
        match pc {
            0x83286770 => {
    //   block [0x83286770..0x832867B0)
	// 83286770: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83286774: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83286778: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8328677C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83286780: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83286784: 388BA9E8  addi r4, r11, -0x5618
	ctx.r[4].s64 = ctx.r[11].s64 + -22040;
	// 83286788: 386AE7DC  addi r3, r10, -0x1824
	ctx.r[3].s64 = ctx.r[10].s64 + -6180;
	// 8328678C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83286790: 4AFA6741  bl 0x8222ced0
	ctx.lr = 0x83286794;
	sub_8222CED0(ctx, base);
	// 83286794: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83286798: 38692188  addi r3, r9, 0x2188
	ctx.r[3].s64 = ctx.r[9].s64 + 8584;
	// 8328679C: 4BA23785  bl 0x82ca9f20
	ctx.lr = 0x832867A0;
	sub_82CA9F20(ctx, base);
	// 832867A0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832867A4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832867A8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832867AC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832867B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832867B0 size=64
    let mut pc: u32 = 0x832867B0;
    'dispatch: loop {
        match pc {
            0x832867B0 => {
    //   block [0x832867B0..0x832867F0)
	// 832867B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832867B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832867B8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832867BC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 832867C0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832867C4: 388BA9FC  addi r4, r11, -0x5604
	ctx.r[4].s64 = ctx.r[11].s64 + -22020;
	// 832867C8: 386AE7E0  addi r3, r10, -0x1820
	ctx.r[3].s64 = ctx.r[10].s64 + -6176;
	// 832867CC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832867D0: 4AFA6701  bl 0x8222ced0
	ctx.lr = 0x832867D4;
	sub_8222CED0(ctx, base);
	// 832867D4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 832867D8: 38692198  addi r3, r9, 0x2198
	ctx.r[3].s64 = ctx.r[9].s64 + 8600;
	// 832867DC: 4BA23745  bl 0x82ca9f20
	ctx.lr = 0x832867E0;
	sub_82CA9F20(ctx, base);
	// 832867E0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832867E4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832867E8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832867EC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832867F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832867F0 size=64
    let mut pc: u32 = 0x832867F0;
    'dispatch: loop {
        match pc {
            0x832867F0 => {
    //   block [0x832867F0..0x83286830)
	// 832867F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832867F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832867F8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832867FC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83286800: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83286804: 388BAA10  addi r4, r11, -0x55f0
	ctx.r[4].s64 = ctx.r[11].s64 + -22000;
	// 83286808: 386AE7E4  addi r3, r10, -0x181c
	ctx.r[3].s64 = ctx.r[10].s64 + -6172;
	// 8328680C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83286810: 4AFA66C1  bl 0x8222ced0
	ctx.lr = 0x83286814;
	sub_8222CED0(ctx, base);
	// 83286814: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83286818: 386921A8  addi r3, r9, 0x21a8
	ctx.r[3].s64 = ctx.r[9].s64 + 8616;
	// 8328681C: 4BA23705  bl 0x82ca9f20
	ctx.lr = 0x83286820;
	sub_82CA9F20(ctx, base);
	// 83286820: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83286824: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83286828: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8328682C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83286830(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83286830 size=64
    let mut pc: u32 = 0x83286830;
    'dispatch: loop {
        match pc {
            0x83286830 => {
    //   block [0x83286830..0x83286870)
	// 83286830: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83286834: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83286838: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8328683C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83286840: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83286844: 388BAA24  addi r4, r11, -0x55dc
	ctx.r[4].s64 = ctx.r[11].s64 + -21980;
	// 83286848: 386AE7E8  addi r3, r10, -0x1818
	ctx.r[3].s64 = ctx.r[10].s64 + -6168;
	// 8328684C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83286850: 4AFA6681  bl 0x8222ced0
	ctx.lr = 0x83286854;
	sub_8222CED0(ctx, base);
	// 83286854: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83286858: 386921B8  addi r3, r9, 0x21b8
	ctx.r[3].s64 = ctx.r[9].s64 + 8632;
	// 8328685C: 4BA236C5  bl 0x82ca9f20
	ctx.lr = 0x83286860;
	sub_82CA9F20(ctx, base);
	// 83286860: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83286864: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83286868: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8328686C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83286870(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83286870 size=64
    let mut pc: u32 = 0x83286870;
    'dispatch: loop {
        match pc {
            0x83286870 => {
    //   block [0x83286870..0x832868B0)
	// 83286870: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83286874: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83286878: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8328687C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83286880: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83286884: 388BAA30  addi r4, r11, -0x55d0
	ctx.r[4].s64 = ctx.r[11].s64 + -21968;
	// 83286888: 386AE7EC  addi r3, r10, -0x1814
	ctx.r[3].s64 = ctx.r[10].s64 + -6164;
	// 8328688C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83286890: 4AFA6641  bl 0x8222ced0
	ctx.lr = 0x83286894;
	sub_8222CED0(ctx, base);
	// 83286894: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83286898: 386921C8  addi r3, r9, 0x21c8
	ctx.r[3].s64 = ctx.r[9].s64 + 8648;
	// 8328689C: 4BA23685  bl 0x82ca9f20
	ctx.lr = 0x832868A0;
	sub_82CA9F20(ctx, base);
	// 832868A0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832868A4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832868A8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832868AC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832868B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832868B0 size=64
    let mut pc: u32 = 0x832868B0;
    'dispatch: loop {
        match pc {
            0x832868B0 => {
    //   block [0x832868B0..0x832868F0)
	// 832868B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832868B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832868B8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832868BC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 832868C0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832868C4: 388BA96C  addi r4, r11, -0x5694
	ctx.r[4].s64 = ctx.r[11].s64 + -22164;
	// 832868C8: 386AE7F0  addi r3, r10, -0x1810
	ctx.r[3].s64 = ctx.r[10].s64 + -6160;
	// 832868CC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832868D0: 4AFA6601  bl 0x8222ced0
	ctx.lr = 0x832868D4;
	sub_8222CED0(ctx, base);
	// 832868D4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 832868D8: 386921D8  addi r3, r9, 0x21d8
	ctx.r[3].s64 = ctx.r[9].s64 + 8664;
	// 832868DC: 4BA23645  bl 0x82ca9f20
	ctx.lr = 0x832868E0;
	sub_82CA9F20(ctx, base);
	// 832868E0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832868E4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832868E8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832868EC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832868F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832868F0 size=64
    let mut pc: u32 = 0x832868F0;
    'dispatch: loop {
        match pc {
            0x832868F0 => {
    //   block [0x832868F0..0x83286930)
	// 832868F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832868F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832868F8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832868FC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83286900: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83286904: 388BA974  addi r4, r11, -0x568c
	ctx.r[4].s64 = ctx.r[11].s64 + -22156;
	// 83286908: 386AE7F4  addi r3, r10, -0x180c
	ctx.r[3].s64 = ctx.r[10].s64 + -6156;
	// 8328690C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83286910: 4AFA65C1  bl 0x8222ced0
	ctx.lr = 0x83286914;
	sub_8222CED0(ctx, base);
	// 83286914: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83286918: 386921E8  addi r3, r9, 0x21e8
	ctx.r[3].s64 = ctx.r[9].s64 + 8680;
	// 8328691C: 4BA23605  bl 0x82ca9f20
	ctx.lr = 0x83286920;
	sub_82CA9F20(ctx, base);
	// 83286920: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83286924: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83286928: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8328692C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83286930(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83286930 size=64
    let mut pc: u32 = 0x83286930;
    'dispatch: loop {
        match pc {
            0x83286930 => {
    //   block [0x83286930..0x83286970)
	// 83286930: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83286934: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83286938: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8328693C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83286940: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83286944: 388BA988  addi r4, r11, -0x5678
	ctx.r[4].s64 = ctx.r[11].s64 + -22136;
	// 83286948: 386AE7F8  addi r3, r10, -0x1808
	ctx.r[3].s64 = ctx.r[10].s64 + -6152;
	// 8328694C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83286950: 4AFA6581  bl 0x8222ced0
	ctx.lr = 0x83286954;
	sub_8222CED0(ctx, base);
	// 83286954: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83286958: 386921F8  addi r3, r9, 0x21f8
	ctx.r[3].s64 = ctx.r[9].s64 + 8696;
	// 8328695C: 4BA235C5  bl 0x82ca9f20
	ctx.lr = 0x83286960;
	sub_82CA9F20(ctx, base);
	// 83286960: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83286964: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83286968: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8328696C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83286970(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83286970 size=64
    let mut pc: u32 = 0x83286970;
    'dispatch: loop {
        match pc {
            0x83286970 => {
    //   block [0x83286970..0x832869B0)
	// 83286970: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83286974: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83286978: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8328697C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83286980: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83286984: 388BA99C  addi r4, r11, -0x5664
	ctx.r[4].s64 = ctx.r[11].s64 + -22116;
	// 83286988: 386AE7FC  addi r3, r10, -0x1804
	ctx.r[3].s64 = ctx.r[10].s64 + -6148;
	// 8328698C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83286990: 4AFA6541  bl 0x8222ced0
	ctx.lr = 0x83286994;
	sub_8222CED0(ctx, base);
	// 83286994: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83286998: 38692208  addi r3, r9, 0x2208
	ctx.r[3].s64 = ctx.r[9].s64 + 8712;
	// 8328699C: 4BA23585  bl 0x82ca9f20
	ctx.lr = 0x832869A0;
	sub_82CA9F20(ctx, base);
	// 832869A0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832869A4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832869A8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832869AC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832869B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832869B0 size=64
    let mut pc: u32 = 0x832869B0;
    'dispatch: loop {
        match pc {
            0x832869B0 => {
    //   block [0x832869B0..0x832869F0)
	// 832869B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832869B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832869B8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832869BC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 832869C0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832869C4: 388BA9B0  addi r4, r11, -0x5650
	ctx.r[4].s64 = ctx.r[11].s64 + -22096;
	// 832869C8: 386AE800  addi r3, r10, -0x1800
	ctx.r[3].s64 = ctx.r[10].s64 + -6144;
	// 832869CC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832869D0: 4AFA6501  bl 0x8222ced0
	ctx.lr = 0x832869D4;
	sub_8222CED0(ctx, base);
	// 832869D4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 832869D8: 38692218  addi r3, r9, 0x2218
	ctx.r[3].s64 = ctx.r[9].s64 + 8728;
	// 832869DC: 4BA23545  bl 0x82ca9f20
	ctx.lr = 0x832869E0;
	sub_82CA9F20(ctx, base);
	// 832869E0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832869E4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832869E8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832869EC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832869F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832869F0 size=64
    let mut pc: u32 = 0x832869F0;
    'dispatch: loop {
        match pc {
            0x832869F0 => {
    //   block [0x832869F0..0x83286A30)
	// 832869F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832869F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832869F8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832869FC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83286A00: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83286A04: 388BA9BC  addi r4, r11, -0x5644
	ctx.r[4].s64 = ctx.r[11].s64 + -22084;
	// 83286A08: 386AE804  addi r3, r10, -0x17fc
	ctx.r[3].s64 = ctx.r[10].s64 + -6140;
	// 83286A0C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83286A10: 4AFA64C1  bl 0x8222ced0
	ctx.lr = 0x83286A14;
	sub_8222CED0(ctx, base);
	// 83286A14: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83286A18: 38692228  addi r3, r9, 0x2228
	ctx.r[3].s64 = ctx.r[9].s64 + 8744;
	// 83286A1C: 4BA23505  bl 0x82ca9f20
	ctx.lr = 0x83286A20;
	sub_82CA9F20(ctx, base);
	// 83286A20: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83286A24: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83286A28: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83286A2C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83286A30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83286A30 size=64
    let mut pc: u32 = 0x83286A30;
    'dispatch: loop {
        match pc {
            0x83286A30 => {
    //   block [0x83286A30..0x83286A70)
	// 83286A30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83286A34: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83286A38: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83286A3C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83286A40: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83286A44: 388BA9CC  addi r4, r11, -0x5634
	ctx.r[4].s64 = ctx.r[11].s64 + -22068;
	// 83286A48: 386AE808  addi r3, r10, -0x17f8
	ctx.r[3].s64 = ctx.r[10].s64 + -6136;
	// 83286A4C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83286A50: 4AFA6481  bl 0x8222ced0
	ctx.lr = 0x83286A54;
	sub_8222CED0(ctx, base);
	// 83286A54: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83286A58: 38692238  addi r3, r9, 0x2238
	ctx.r[3].s64 = ctx.r[9].s64 + 8760;
	// 83286A5C: 4BA234C5  bl 0x82ca9f20
	ctx.lr = 0x83286A60;
	sub_82CA9F20(ctx, base);
	// 83286A60: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83286A64: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83286A68: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83286A6C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83286A70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83286A70 size=64
    let mut pc: u32 = 0x83286A70;
    'dispatch: loop {
        match pc {
            0x83286A70 => {
    //   block [0x83286A70..0x83286AB0)
	// 83286A70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83286A74: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83286A78: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83286A7C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83286A80: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83286A84: 388BA93C  addi r4, r11, -0x56c4
	ctx.r[4].s64 = ctx.r[11].s64 + -22212;
	// 83286A88: 386AE80C  addi r3, r10, -0x17f4
	ctx.r[3].s64 = ctx.r[10].s64 + -6132;
	// 83286A8C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83286A90: 4AFA6441  bl 0x8222ced0
	ctx.lr = 0x83286A94;
	sub_8222CED0(ctx, base);
	// 83286A94: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83286A98: 38692248  addi r3, r9, 0x2248
	ctx.r[3].s64 = ctx.r[9].s64 + 8776;
	// 83286A9C: 4BA23485  bl 0x82ca9f20
	ctx.lr = 0x83286AA0;
	sub_82CA9F20(ctx, base);
	// 83286AA0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83286AA4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83286AA8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83286AAC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83286AB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83286AB0 size=64
    let mut pc: u32 = 0x83286AB0;
    'dispatch: loop {
        match pc {
            0x83286AB0 => {
    //   block [0x83286AB0..0x83286AF0)
	// 83286AB0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83286AB4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83286AB8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83286ABC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83286AC0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83286AC4: 388BA944  addi r4, r11, -0x56bc
	ctx.r[4].s64 = ctx.r[11].s64 + -22204;
	// 83286AC8: 386AE810  addi r3, r10, -0x17f0
	ctx.r[3].s64 = ctx.r[10].s64 + -6128;
	// 83286ACC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83286AD0: 4AFA6401  bl 0x8222ced0
	ctx.lr = 0x83286AD4;
	sub_8222CED0(ctx, base);
	// 83286AD4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83286AD8: 38692258  addi r3, r9, 0x2258
	ctx.r[3].s64 = ctx.r[9].s64 + 8792;
	// 83286ADC: 4BA23445  bl 0x82ca9f20
	ctx.lr = 0x83286AE0;
	sub_82CA9F20(ctx, base);
	// 83286AE0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83286AE4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83286AE8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83286AEC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83286AF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83286AF0 size=64
    let mut pc: u32 = 0x83286AF0;
    'dispatch: loop {
        match pc {
            0x83286AF0 => {
    //   block [0x83286AF0..0x83286B30)
	// 83286AF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83286AF4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83286AF8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83286AFC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83286B00: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83286B04: 388BA958  addi r4, r11, -0x56a8
	ctx.r[4].s64 = ctx.r[11].s64 + -22184;
	// 83286B08: 386AE814  addi r3, r10, -0x17ec
	ctx.r[3].s64 = ctx.r[10].s64 + -6124;
	// 83286B0C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83286B10: 4AFA63C1  bl 0x8222ced0
	ctx.lr = 0x83286B14;
	sub_8222CED0(ctx, base);
	// 83286B14: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83286B18: 38692268  addi r3, r9, 0x2268
	ctx.r[3].s64 = ctx.r[9].s64 + 8808;
	// 83286B1C: 4BA23405  bl 0x82ca9f20
	ctx.lr = 0x83286B20;
	sub_82CA9F20(ctx, base);
	// 83286B20: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83286B24: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83286B28: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83286B2C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83286B30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83286B30 size=64
    let mut pc: u32 = 0x83286B30;
    'dispatch: loop {
        match pc {
            0x83286B30 => {
    //   block [0x83286B30..0x83286B70)
	// 83286B30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83286B34: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83286B38: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83286B3C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83286B40: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83286B44: 388BA854  addi r4, r11, -0x57ac
	ctx.r[4].s64 = ctx.r[11].s64 + -22444;
	// 83286B48: 386AE818  addi r3, r10, -0x17e8
	ctx.r[3].s64 = ctx.r[10].s64 + -6120;
	// 83286B4C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83286B50: 4AFA6381  bl 0x8222ced0
	ctx.lr = 0x83286B54;
	sub_8222CED0(ctx, base);
	// 83286B54: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83286B58: 38692278  addi r3, r9, 0x2278
	ctx.r[3].s64 = ctx.r[9].s64 + 8824;
	// 83286B5C: 4BA233C5  bl 0x82ca9f20
	ctx.lr = 0x83286B60;
	sub_82CA9F20(ctx, base);
	// 83286B60: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83286B64: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83286B68: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83286B6C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83286B70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83286B70 size=64
    let mut pc: u32 = 0x83286B70;
    'dispatch: loop {
        match pc {
            0x83286B70 => {
    //   block [0x83286B70..0x83286BB0)
	// 83286B70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83286B74: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83286B78: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83286B7C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83286B80: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83286B84: 388BA85C  addi r4, r11, -0x57a4
	ctx.r[4].s64 = ctx.r[11].s64 + -22436;
	// 83286B88: 386AE81C  addi r3, r10, -0x17e4
	ctx.r[3].s64 = ctx.r[10].s64 + -6116;
	// 83286B8C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83286B90: 4AFA6341  bl 0x8222ced0
	ctx.lr = 0x83286B94;
	sub_8222CED0(ctx, base);
	// 83286B94: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83286B98: 38692288  addi r3, r9, 0x2288
	ctx.r[3].s64 = ctx.r[9].s64 + 8840;
	// 83286B9C: 4BA23385  bl 0x82ca9f20
	ctx.lr = 0x83286BA0;
	sub_82CA9F20(ctx, base);
	// 83286BA0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83286BA4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83286BA8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83286BAC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83286BB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83286BB0 size=64
    let mut pc: u32 = 0x83286BB0;
    'dispatch: loop {
        match pc {
            0x83286BB0 => {
    //   block [0x83286BB0..0x83286BF0)
	// 83286BB0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83286BB4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83286BB8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83286BBC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83286BC0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83286BC4: 388BA870  addi r4, r11, -0x5790
	ctx.r[4].s64 = ctx.r[11].s64 + -22416;
	// 83286BC8: 386AE820  addi r3, r10, -0x17e0
	ctx.r[3].s64 = ctx.r[10].s64 + -6112;
	// 83286BCC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83286BD0: 4AFA6301  bl 0x8222ced0
	ctx.lr = 0x83286BD4;
	sub_8222CED0(ctx, base);
	// 83286BD4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83286BD8: 38692298  addi r3, r9, 0x2298
	ctx.r[3].s64 = ctx.r[9].s64 + 8856;
	// 83286BDC: 4BA23345  bl 0x82ca9f20
	ctx.lr = 0x83286BE0;
	sub_82CA9F20(ctx, base);
	// 83286BE0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83286BE4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83286BE8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83286BEC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83286BF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83286BF0 size=64
    let mut pc: u32 = 0x83286BF0;
    'dispatch: loop {
        match pc {
            0x83286BF0 => {
    //   block [0x83286BF0..0x83286C30)
	// 83286BF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83286BF4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83286BF8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83286BFC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83286C00: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83286C04: 388BA884  addi r4, r11, -0x577c
	ctx.r[4].s64 = ctx.r[11].s64 + -22396;
	// 83286C08: 386AE824  addi r3, r10, -0x17dc
	ctx.r[3].s64 = ctx.r[10].s64 + -6108;
	// 83286C0C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83286C10: 4AFA62C1  bl 0x8222ced0
	ctx.lr = 0x83286C14;
	sub_8222CED0(ctx, base);
	// 83286C14: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83286C18: 386922A8  addi r3, r9, 0x22a8
	ctx.r[3].s64 = ctx.r[9].s64 + 8872;
	// 83286C1C: 4BA23305  bl 0x82ca9f20
	ctx.lr = 0x83286C20;
	sub_82CA9F20(ctx, base);
	// 83286C20: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83286C24: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83286C28: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83286C2C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83286C30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83286C30 size=64
    let mut pc: u32 = 0x83286C30;
    'dispatch: loop {
        match pc {
            0x83286C30 => {
    //   block [0x83286C30..0x83286C70)
	// 83286C30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83286C34: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83286C38: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83286C3C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83286C40: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83286C44: 388BA8A0  addi r4, r11, -0x5760
	ctx.r[4].s64 = ctx.r[11].s64 + -22368;
	// 83286C48: 386AE828  addi r3, r10, -0x17d8
	ctx.r[3].s64 = ctx.r[10].s64 + -6104;
	// 83286C4C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83286C50: 4AFA6281  bl 0x8222ced0
	ctx.lr = 0x83286C54;
	sub_8222CED0(ctx, base);
	// 83286C54: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83286C58: 386922B8  addi r3, r9, 0x22b8
	ctx.r[3].s64 = ctx.r[9].s64 + 8888;
	// 83286C5C: 4BA232C5  bl 0x82ca9f20
	ctx.lr = 0x83286C60;
	sub_82CA9F20(ctx, base);
	// 83286C60: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83286C64: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83286C68: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83286C6C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83286C70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83286C70 size=64
    let mut pc: u32 = 0x83286C70;
    'dispatch: loop {
        match pc {
            0x83286C70 => {
    //   block [0x83286C70..0x83286CB0)
	// 83286C70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83286C74: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83286C78: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83286C7C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83286C80: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83286C84: 388BA8B0  addi r4, r11, -0x5750
	ctx.r[4].s64 = ctx.r[11].s64 + -22352;
	// 83286C88: 386AE82C  addi r3, r10, -0x17d4
	ctx.r[3].s64 = ctx.r[10].s64 + -6100;
	// 83286C8C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83286C90: 4AFA6241  bl 0x8222ced0
	ctx.lr = 0x83286C94;
	sub_8222CED0(ctx, base);
	// 83286C94: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83286C98: 386922C8  addi r3, r9, 0x22c8
	ctx.r[3].s64 = ctx.r[9].s64 + 8904;
	// 83286C9C: 4BA23285  bl 0x82ca9f20
	ctx.lr = 0x83286CA0;
	sub_82CA9F20(ctx, base);
	// 83286CA0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83286CA4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83286CA8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83286CAC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83286CB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83286CB0 size=64
    let mut pc: u32 = 0x83286CB0;
    'dispatch: loop {
        match pc {
            0x83286CB0 => {
    //   block [0x83286CB0..0x83286CF0)
	// 83286CB0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83286CB4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83286CB8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83286CBC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83286CC0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83286CC4: 388BA8C4  addi r4, r11, -0x573c
	ctx.r[4].s64 = ctx.r[11].s64 + -22332;
	// 83286CC8: 386AE830  addi r3, r10, -0x17d0
	ctx.r[3].s64 = ctx.r[10].s64 + -6096;
	// 83286CCC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83286CD0: 4AFA6201  bl 0x8222ced0
	ctx.lr = 0x83286CD4;
	sub_8222CED0(ctx, base);
	// 83286CD4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83286CD8: 386922D8  addi r3, r9, 0x22d8
	ctx.r[3].s64 = ctx.r[9].s64 + 8920;
	// 83286CDC: 4BA23245  bl 0x82ca9f20
	ctx.lr = 0x83286CE0;
	sub_82CA9F20(ctx, base);
	// 83286CE0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83286CE4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83286CE8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83286CEC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83286CF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83286CF0 size=64
    let mut pc: u32 = 0x83286CF0;
    'dispatch: loop {
        match pc {
            0x83286CF0 => {
    //   block [0x83286CF0..0x83286D30)
	// 83286CF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83286CF4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83286CF8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83286CFC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83286D00: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83286D04: 388BA8DC  addi r4, r11, -0x5724
	ctx.r[4].s64 = ctx.r[11].s64 + -22308;
	// 83286D08: 386AE834  addi r3, r10, -0x17cc
	ctx.r[3].s64 = ctx.r[10].s64 + -6092;
	// 83286D0C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83286D10: 4AFA61C1  bl 0x8222ced0
	ctx.lr = 0x83286D14;
	sub_8222CED0(ctx, base);
	// 83286D14: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83286D18: 386922E8  addi r3, r9, 0x22e8
	ctx.r[3].s64 = ctx.r[9].s64 + 8936;
	// 83286D1C: 4BA23205  bl 0x82ca9f20
	ctx.lr = 0x83286D20;
	sub_82CA9F20(ctx, base);
	// 83286D20: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83286D24: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83286D28: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83286D2C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83286D30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83286D30 size=64
    let mut pc: u32 = 0x83286D30;
    'dispatch: loop {
        match pc {
            0x83286D30 => {
    //   block [0x83286D30..0x83286D70)
	// 83286D30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83286D34: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83286D38: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83286D3C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83286D40: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83286D44: 388BA8F0  addi r4, r11, -0x5710
	ctx.r[4].s64 = ctx.r[11].s64 + -22288;
	// 83286D48: 386AE838  addi r3, r10, -0x17c8
	ctx.r[3].s64 = ctx.r[10].s64 + -6088;
	// 83286D4C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83286D50: 4AFA6181  bl 0x8222ced0
	ctx.lr = 0x83286D54;
	sub_8222CED0(ctx, base);
	// 83286D54: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83286D58: 386922F8  addi r3, r9, 0x22f8
	ctx.r[3].s64 = ctx.r[9].s64 + 8952;
	// 83286D5C: 4BA231C5  bl 0x82ca9f20
	ctx.lr = 0x83286D60;
	sub_82CA9F20(ctx, base);
	// 83286D60: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83286D64: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83286D68: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83286D6C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83286D70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83286D70 size=64
    let mut pc: u32 = 0x83286D70;
    'dispatch: loop {
        match pc {
            0x83286D70 => {
    //   block [0x83286D70..0x83286DB0)
	// 83286D70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83286D74: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83286D78: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83286D7C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83286D80: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83286D84: 388BA90C  addi r4, r11, -0x56f4
	ctx.r[4].s64 = ctx.r[11].s64 + -22260;
	// 83286D88: 386AE83C  addi r3, r10, -0x17c4
	ctx.r[3].s64 = ctx.r[10].s64 + -6084;
	// 83286D8C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83286D90: 4AFA6141  bl 0x8222ced0
	ctx.lr = 0x83286D94;
	sub_8222CED0(ctx, base);
	// 83286D94: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83286D98: 38692308  addi r3, r9, 0x2308
	ctx.r[3].s64 = ctx.r[9].s64 + 8968;
	// 83286D9C: 4BA23185  bl 0x82ca9f20
	ctx.lr = 0x83286DA0;
	sub_82CA9F20(ctx, base);
	// 83286DA0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83286DA4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83286DA8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83286DAC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83286DB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83286DB0 size=64
    let mut pc: u32 = 0x83286DB0;
    'dispatch: loop {
        match pc {
            0x83286DB0 => {
    //   block [0x83286DB0..0x83286DF0)
	// 83286DB0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83286DB4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83286DB8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83286DBC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83286DC0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83286DC4: 388BA924  addi r4, r11, -0x56dc
	ctx.r[4].s64 = ctx.r[11].s64 + -22236;
	// 83286DC8: 386AE840  addi r3, r10, -0x17c0
	ctx.r[3].s64 = ctx.r[10].s64 + -6080;
	// 83286DCC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83286DD0: 4AFA6101  bl 0x8222ced0
	ctx.lr = 0x83286DD4;
	sub_8222CED0(ctx, base);
	// 83286DD4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83286DD8: 38692318  addi r3, r9, 0x2318
	ctx.r[3].s64 = ctx.r[9].s64 + 8984;
	// 83286DDC: 4BA23145  bl 0x82ca9f20
	ctx.lr = 0x83286DE0;
	sub_82CA9F20(ctx, base);
	// 83286DE0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83286DE4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83286DE8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83286DEC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83286DF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83286DF0 size=64
    let mut pc: u32 = 0x83286DF0;
    'dispatch: loop {
        match pc {
            0x83286DF0 => {
    //   block [0x83286DF0..0x83286E30)
	// 83286DF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83286DF4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83286DF8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83286DFC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83286E00: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83286E04: 388BA72C  addi r4, r11, -0x58d4
	ctx.r[4].s64 = ctx.r[11].s64 + -22740;
	// 83286E08: 386AE844  addi r3, r10, -0x17bc
	ctx.r[3].s64 = ctx.r[10].s64 + -6076;
	// 83286E0C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83286E10: 4AFA60C1  bl 0x8222ced0
	ctx.lr = 0x83286E14;
	sub_8222CED0(ctx, base);
	// 83286E14: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83286E18: 38692328  addi r3, r9, 0x2328
	ctx.r[3].s64 = ctx.r[9].s64 + 9000;
	// 83286E1C: 4BA23105  bl 0x82ca9f20
	ctx.lr = 0x83286E20;
	sub_82CA9F20(ctx, base);
	// 83286E20: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83286E24: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83286E28: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83286E2C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83286E30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83286E30 size=64
    let mut pc: u32 = 0x83286E30;
    'dispatch: loop {
        match pc {
            0x83286E30 => {
    //   block [0x83286E30..0x83286E70)
	// 83286E30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83286E34: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83286E38: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83286E3C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83286E40: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83286E44: 388BA734  addi r4, r11, -0x58cc
	ctx.r[4].s64 = ctx.r[11].s64 + -22732;
	// 83286E48: 386AE848  addi r3, r10, -0x17b8
	ctx.r[3].s64 = ctx.r[10].s64 + -6072;
	// 83286E4C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83286E50: 4AFA6081  bl 0x8222ced0
	ctx.lr = 0x83286E54;
	sub_8222CED0(ctx, base);
	// 83286E54: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83286E58: 38692338  addi r3, r9, 0x2338
	ctx.r[3].s64 = ctx.r[9].s64 + 9016;
	// 83286E5C: 4BA230C5  bl 0x82ca9f20
	ctx.lr = 0x83286E60;
	sub_82CA9F20(ctx, base);
	// 83286E60: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83286E64: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83286E68: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83286E6C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83286E70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83286E70 size=64
    let mut pc: u32 = 0x83286E70;
    'dispatch: loop {
        match pc {
            0x83286E70 => {
    //   block [0x83286E70..0x83286EB0)
	// 83286E70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83286E74: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83286E78: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83286E7C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83286E80: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83286E84: 388BA748  addi r4, r11, -0x58b8
	ctx.r[4].s64 = ctx.r[11].s64 + -22712;
	// 83286E88: 386AE84C  addi r3, r10, -0x17b4
	ctx.r[3].s64 = ctx.r[10].s64 + -6068;
	// 83286E8C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83286E90: 4AFA6041  bl 0x8222ced0
	ctx.lr = 0x83286E94;
	sub_8222CED0(ctx, base);
	// 83286E94: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83286E98: 38692348  addi r3, r9, 0x2348
	ctx.r[3].s64 = ctx.r[9].s64 + 9032;
	// 83286E9C: 4BA23085  bl 0x82ca9f20
	ctx.lr = 0x83286EA0;
	sub_82CA9F20(ctx, base);
	// 83286EA0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83286EA4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83286EA8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83286EAC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83286EB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83286EB0 size=64
    let mut pc: u32 = 0x83286EB0;
    'dispatch: loop {
        match pc {
            0x83286EB0 => {
    //   block [0x83286EB0..0x83286EF0)
	// 83286EB0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83286EB4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83286EB8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83286EBC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83286EC0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83286EC4: 388BA75C  addi r4, r11, -0x58a4
	ctx.r[4].s64 = ctx.r[11].s64 + -22692;
	// 83286EC8: 386AE850  addi r3, r10, -0x17b0
	ctx.r[3].s64 = ctx.r[10].s64 + -6064;
	// 83286ECC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83286ED0: 4AFA6001  bl 0x8222ced0
	ctx.lr = 0x83286ED4;
	sub_8222CED0(ctx, base);
	// 83286ED4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83286ED8: 38692358  addi r3, r9, 0x2358
	ctx.r[3].s64 = ctx.r[9].s64 + 9048;
	// 83286EDC: 4BA23045  bl 0x82ca9f20
	ctx.lr = 0x83286EE0;
	sub_82CA9F20(ctx, base);
	// 83286EE0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83286EE4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83286EE8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83286EEC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83286EF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83286EF0 size=64
    let mut pc: u32 = 0x83286EF0;
    'dispatch: loop {
        match pc {
            0x83286EF0 => {
    //   block [0x83286EF0..0x83286F30)
	// 83286EF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83286EF4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83286EF8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83286EFC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83286F00: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83286F04: 388BA790  addi r4, r11, -0x5870
	ctx.r[4].s64 = ctx.r[11].s64 + -22640;
	// 83286F08: 386AE854  addi r3, r10, -0x17ac
	ctx.r[3].s64 = ctx.r[10].s64 + -6060;
	// 83286F0C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83286F10: 4AFA5FC1  bl 0x8222ced0
	ctx.lr = 0x83286F14;
	sub_8222CED0(ctx, base);
	// 83286F14: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83286F18: 38692368  addi r3, r9, 0x2368
	ctx.r[3].s64 = ctx.r[9].s64 + 9064;
	// 83286F1C: 4BA23005  bl 0x82ca9f20
	ctx.lr = 0x83286F20;
	sub_82CA9F20(ctx, base);
	// 83286F20: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83286F24: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83286F28: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83286F2C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83286F30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83286F30 size=64
    let mut pc: u32 = 0x83286F30;
    'dispatch: loop {
        match pc {
            0x83286F30 => {
    //   block [0x83286F30..0x83286F70)
	// 83286F30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83286F34: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83286F38: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83286F3C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83286F40: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83286F44: 388BA7C4  addi r4, r11, -0x583c
	ctx.r[4].s64 = ctx.r[11].s64 + -22588;
	// 83286F48: 386AE858  addi r3, r10, -0x17a8
	ctx.r[3].s64 = ctx.r[10].s64 + -6056;
	// 83286F4C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83286F50: 4AFA5F81  bl 0x8222ced0
	ctx.lr = 0x83286F54;
	sub_8222CED0(ctx, base);
	// 83286F54: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83286F58: 38692378  addi r3, r9, 0x2378
	ctx.r[3].s64 = ctx.r[9].s64 + 9080;
	// 83286F5C: 4BA22FC5  bl 0x82ca9f20
	ctx.lr = 0x83286F60;
	sub_82CA9F20(ctx, base);
	// 83286F60: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83286F64: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83286F68: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83286F6C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83286F70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83286F70 size=64
    let mut pc: u32 = 0x83286F70;
    'dispatch: loop {
        match pc {
            0x83286F70 => {
    //   block [0x83286F70..0x83286FB0)
	// 83286F70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83286F74: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83286F78: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83286F7C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83286F80: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83286F84: 388BA7C8  addi r4, r11, -0x5838
	ctx.r[4].s64 = ctx.r[11].s64 + -22584;
	// 83286F88: 386AE85C  addi r3, r10, -0x17a4
	ctx.r[3].s64 = ctx.r[10].s64 + -6052;
	// 83286F8C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83286F90: 4AFA5F41  bl 0x8222ced0
	ctx.lr = 0x83286F94;
	sub_8222CED0(ctx, base);
	// 83286F94: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83286F98: 38692388  addi r3, r9, 0x2388
	ctx.r[3].s64 = ctx.r[9].s64 + 9096;
	// 83286F9C: 4BA22F85  bl 0x82ca9f20
	ctx.lr = 0x83286FA0;
	sub_82CA9F20(ctx, base);
	// 83286FA0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83286FA4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83286FA8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83286FAC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83286FB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83286FB0 size=64
    let mut pc: u32 = 0x83286FB0;
    'dispatch: loop {
        match pc {
            0x83286FB0 => {
    //   block [0x83286FB0..0x83286FF0)
	// 83286FB0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83286FB4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83286FB8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83286FBC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83286FC0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83286FC4: 388BA7F8  addi r4, r11, -0x5808
	ctx.r[4].s64 = ctx.r[11].s64 + -22536;
	// 83286FC8: 386AE860  addi r3, r10, -0x17a0
	ctx.r[3].s64 = ctx.r[10].s64 + -6048;
	// 83286FCC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83286FD0: 4AFA5F01  bl 0x8222ced0
	ctx.lr = 0x83286FD4;
	sub_8222CED0(ctx, base);
	// 83286FD4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83286FD8: 38692398  addi r3, r9, 0x2398
	ctx.r[3].s64 = ctx.r[9].s64 + 9112;
	// 83286FDC: 4BA22F45  bl 0x82ca9f20
	ctx.lr = 0x83286FE0;
	sub_82CA9F20(ctx, base);
	// 83286FE0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83286FE4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83286FE8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83286FEC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83286FF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83286FF0 size=64
    let mut pc: u32 = 0x83286FF0;
    'dispatch: loop {
        match pc {
            0x83286FF0 => {
    //   block [0x83286FF0..0x83287030)
	// 83286FF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83286FF4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83286FF8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83286FFC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83287000: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83287004: 388BA808  addi r4, r11, -0x57f8
	ctx.r[4].s64 = ctx.r[11].s64 + -22520;
	// 83287008: 386AE864  addi r3, r10, -0x179c
	ctx.r[3].s64 = ctx.r[10].s64 + -6044;
	// 8328700C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83287010: 4AFA5EC1  bl 0x8222ced0
	ctx.lr = 0x83287014;
	sub_8222CED0(ctx, base);
	// 83287014: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83287018: 386923A8  addi r3, r9, 0x23a8
	ctx.r[3].s64 = ctx.r[9].s64 + 9128;
	// 8328701C: 4BA22F05  bl 0x82ca9f20
	ctx.lr = 0x83287020;
	sub_82CA9F20(ctx, base);
	// 83287020: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83287024: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83287028: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8328702C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83287030(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83287030 size=64
    let mut pc: u32 = 0x83287030;
    'dispatch: loop {
        match pc {
            0x83287030 => {
    //   block [0x83287030..0x83287070)
	// 83287030: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83287034: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83287038: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8328703C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83287040: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83287044: 388BA814  addi r4, r11, -0x57ec
	ctx.r[4].s64 = ctx.r[11].s64 + -22508;
	// 83287048: 386AE868  addi r3, r10, -0x1798
	ctx.r[3].s64 = ctx.r[10].s64 + -6040;
	// 8328704C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83287050: 4AFA5E81  bl 0x8222ced0
	ctx.lr = 0x83287054;
	sub_8222CED0(ctx, base);
	// 83287054: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83287058: 386923B8  addi r3, r9, 0x23b8
	ctx.r[3].s64 = ctx.r[9].s64 + 9144;
	// 8328705C: 4BA22EC5  bl 0x82ca9f20
	ctx.lr = 0x83287060;
	sub_82CA9F20(ctx, base);
	// 83287060: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83287064: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83287068: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8328706C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83287070(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83287070 size=64
    let mut pc: u32 = 0x83287070;
    'dispatch: loop {
        match pc {
            0x83287070 => {
    //   block [0x83287070..0x832870B0)
	// 83287070: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83287074: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83287078: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8328707C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83287080: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83287084: 388BA828  addi r4, r11, -0x57d8
	ctx.r[4].s64 = ctx.r[11].s64 + -22488;
	// 83287088: 386AE86C  addi r3, r10, -0x1794
	ctx.r[3].s64 = ctx.r[10].s64 + -6036;
	// 8328708C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83287090: 4AFA5E41  bl 0x8222ced0
	ctx.lr = 0x83287094;
	sub_8222CED0(ctx, base);
	// 83287094: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83287098: 386923C8  addi r3, r9, 0x23c8
	ctx.r[3].s64 = ctx.r[9].s64 + 9160;
	// 8328709C: 4BA22E85  bl 0x82ca9f20
	ctx.lr = 0x832870A0;
	sub_82CA9F20(ctx, base);
	// 832870A0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832870A4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832870A8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832870AC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832870B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832870B0 size=64
    let mut pc: u32 = 0x832870B0;
    'dispatch: loop {
        match pc {
            0x832870B0 => {
    //   block [0x832870B0..0x832870F0)
	// 832870B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832870B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832870B8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832870BC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 832870C0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832870C4: 388BA834  addi r4, r11, -0x57cc
	ctx.r[4].s64 = ctx.r[11].s64 + -22476;
	// 832870C8: 386AE870  addi r3, r10, -0x1790
	ctx.r[3].s64 = ctx.r[10].s64 + -6032;
	// 832870CC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832870D0: 4AFA5E01  bl 0x8222ced0
	ctx.lr = 0x832870D4;
	sub_8222CED0(ctx, base);
	// 832870D4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 832870D8: 386923D8  addi r3, r9, 0x23d8
	ctx.r[3].s64 = ctx.r[9].s64 + 9176;
	// 832870DC: 4BA22E45  bl 0x82ca9f20
	ctx.lr = 0x832870E0;
	sub_82CA9F20(ctx, base);
	// 832870E0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832870E4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832870E8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832870EC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832870F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832870F0 size=64
    let mut pc: u32 = 0x832870F0;
    'dispatch: loop {
        match pc {
            0x832870F0 => {
    //   block [0x832870F0..0x83287130)
	// 832870F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832870F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832870F8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832870FC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83287100: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83287104: 388BA844  addi r4, r11, -0x57bc
	ctx.r[4].s64 = ctx.r[11].s64 + -22460;
	// 83287108: 386AE874  addi r3, r10, -0x178c
	ctx.r[3].s64 = ctx.r[10].s64 + -6028;
	// 8328710C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83287110: 4AFA5DC1  bl 0x8222ced0
	ctx.lr = 0x83287114;
	sub_8222CED0(ctx, base);
	// 83287114: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83287118: 386923E8  addi r3, r9, 0x23e8
	ctx.r[3].s64 = ctx.r[9].s64 + 9192;
	// 8328711C: 4BA22E05  bl 0x82ca9f20
	ctx.lr = 0x83287120;
	sub_82CA9F20(ctx, base);
	// 83287120: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83287124: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83287128: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8328712C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83287130(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83287130 size=64
    let mut pc: u32 = 0x83287130;
    'dispatch: loop {
        match pc {
            0x83287130 => {
    //   block [0x83287130..0x83287170)
	// 83287130: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83287134: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83287138: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8328713C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83287140: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83287144: 388BA594  addi r4, r11, -0x5a6c
	ctx.r[4].s64 = ctx.r[11].s64 + -23148;
	// 83287148: 386AE878  addi r3, r10, -0x1788
	ctx.r[3].s64 = ctx.r[10].s64 + -6024;
	// 8328714C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83287150: 4AFA5D81  bl 0x8222ced0
	ctx.lr = 0x83287154;
	sub_8222CED0(ctx, base);
	// 83287154: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83287158: 386923F8  addi r3, r9, 0x23f8
	ctx.r[3].s64 = ctx.r[9].s64 + 9208;
	// 8328715C: 4BA22DC5  bl 0x82ca9f20
	ctx.lr = 0x83287160;
	sub_82CA9F20(ctx, base);
	// 83287160: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83287164: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83287168: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8328716C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83287170(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83287170 size=64
    let mut pc: u32 = 0x83287170;
    'dispatch: loop {
        match pc {
            0x83287170 => {
    //   block [0x83287170..0x832871B0)
	// 83287170: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83287174: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83287178: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8328717C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83287180: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83287184: 388BA59C  addi r4, r11, -0x5a64
	ctx.r[4].s64 = ctx.r[11].s64 + -23140;
	// 83287188: 386AE87C  addi r3, r10, -0x1784
	ctx.r[3].s64 = ctx.r[10].s64 + -6020;
	// 8328718C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83287190: 4AFA5D41  bl 0x8222ced0
	ctx.lr = 0x83287194;
	sub_8222CED0(ctx, base);
	// 83287194: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83287198: 38692408  addi r3, r9, 0x2408
	ctx.r[3].s64 = ctx.r[9].s64 + 9224;
	// 8328719C: 4BA22D85  bl 0x82ca9f20
	ctx.lr = 0x832871A0;
	sub_82CA9F20(ctx, base);
	// 832871A0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832871A4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832871A8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832871AC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832871B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832871B0 size=64
    let mut pc: u32 = 0x832871B0;
    'dispatch: loop {
        match pc {
            0x832871B0 => {
    //   block [0x832871B0..0x832871F0)
	// 832871B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832871B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832871B8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832871BC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 832871C0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832871C4: 388BA5B0  addi r4, r11, -0x5a50
	ctx.r[4].s64 = ctx.r[11].s64 + -23120;
	// 832871C8: 386AE880  addi r3, r10, -0x1780
	ctx.r[3].s64 = ctx.r[10].s64 + -6016;
	// 832871CC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832871D0: 4AFA5D01  bl 0x8222ced0
	ctx.lr = 0x832871D4;
	sub_8222CED0(ctx, base);
	// 832871D4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 832871D8: 38692418  addi r3, r9, 0x2418
	ctx.r[3].s64 = ctx.r[9].s64 + 9240;
	// 832871DC: 4BA22D45  bl 0x82ca9f20
	ctx.lr = 0x832871E0;
	sub_82CA9F20(ctx, base);
	// 832871E0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832871E4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832871E8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832871EC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832871F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832871F0 size=64
    let mut pc: u32 = 0x832871F0;
    'dispatch: loop {
        match pc {
            0x832871F0 => {
    //   block [0x832871F0..0x83287230)
	// 832871F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832871F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832871F8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832871FC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83287200: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83287204: 388BA5C4  addi r4, r11, -0x5a3c
	ctx.r[4].s64 = ctx.r[11].s64 + -23100;
	// 83287208: 386AE884  addi r3, r10, -0x177c
	ctx.r[3].s64 = ctx.r[10].s64 + -6012;
	// 8328720C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83287210: 4AFA5CC1  bl 0x8222ced0
	ctx.lr = 0x83287214;
	sub_8222CED0(ctx, base);
	// 83287214: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83287218: 38692428  addi r3, r9, 0x2428
	ctx.r[3].s64 = ctx.r[9].s64 + 9256;
	// 8328721C: 4BA22D05  bl 0x82ca9f20
	ctx.lr = 0x83287220;
	sub_82CA9F20(ctx, base);
	// 83287220: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83287224: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83287228: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8328722C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83287230(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83287230 size=64
    let mut pc: u32 = 0x83287230;
    'dispatch: loop {
        match pc {
            0x83287230 => {
    //   block [0x83287230..0x83287270)
	// 83287230: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83287234: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83287238: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8328723C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83287240: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83287244: 388BA5DC  addi r4, r11, -0x5a24
	ctx.r[4].s64 = ctx.r[11].s64 + -23076;
	// 83287248: 386AE888  addi r3, r10, -0x1778
	ctx.r[3].s64 = ctx.r[10].s64 + -6008;
	// 8328724C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83287250: 4AFA5C81  bl 0x8222ced0
	ctx.lr = 0x83287254;
	sub_8222CED0(ctx, base);
	// 83287254: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83287258: 38692438  addi r3, r9, 0x2438
	ctx.r[3].s64 = ctx.r[9].s64 + 9272;
	// 8328725C: 4BA22CC5  bl 0x82ca9f20
	ctx.lr = 0x83287260;
	sub_82CA9F20(ctx, base);
	// 83287260: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83287264: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83287268: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8328726C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83287270(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83287270 size=64
    let mut pc: u32 = 0x83287270;
    'dispatch: loop {
        match pc {
            0x83287270 => {
    //   block [0x83287270..0x832872B0)
	// 83287270: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83287274: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83287278: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8328727C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83287280: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83287284: 388BA5EC  addi r4, r11, -0x5a14
	ctx.r[4].s64 = ctx.r[11].s64 + -23060;
	// 83287288: 386AE88C  addi r3, r10, -0x1774
	ctx.r[3].s64 = ctx.r[10].s64 + -6004;
	// 8328728C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83287290: 4AFA5C41  bl 0x8222ced0
	ctx.lr = 0x83287294;
	sub_8222CED0(ctx, base);
	// 83287294: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83287298: 38692448  addi r3, r9, 0x2448
	ctx.r[3].s64 = ctx.r[9].s64 + 9288;
	// 8328729C: 4BA22C85  bl 0x82ca9f20
	ctx.lr = 0x832872A0;
	sub_82CA9F20(ctx, base);
	// 832872A0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832872A4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832872A8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832872AC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832872B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832872B0 size=64
    let mut pc: u32 = 0x832872B0;
    'dispatch: loop {
        match pc {
            0x832872B0 => {
    //   block [0x832872B0..0x832872F0)
	// 832872B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832872B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832872B8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832872BC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 832872C0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832872C4: 388BA604  addi r4, r11, -0x59fc
	ctx.r[4].s64 = ctx.r[11].s64 + -23036;
	// 832872C8: 386AE890  addi r3, r10, -0x1770
	ctx.r[3].s64 = ctx.r[10].s64 + -6000;
	// 832872CC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832872D0: 4AFA5C01  bl 0x8222ced0
	ctx.lr = 0x832872D4;
	sub_8222CED0(ctx, base);
	// 832872D4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 832872D8: 38692458  addi r3, r9, 0x2458
	ctx.r[3].s64 = ctx.r[9].s64 + 9304;
	// 832872DC: 4BA22C45  bl 0x82ca9f20
	ctx.lr = 0x832872E0;
	sub_82CA9F20(ctx, base);
	// 832872E0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832872E4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832872E8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832872EC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832872F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832872F0 size=64
    let mut pc: u32 = 0x832872F0;
    'dispatch: loop {
        match pc {
            0x832872F0 => {
    //   block [0x832872F0..0x83287330)
	// 832872F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832872F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832872F8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832872FC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83287300: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83287304: 388BA618  addi r4, r11, -0x59e8
	ctx.r[4].s64 = ctx.r[11].s64 + -23016;
	// 83287308: 386AE894  addi r3, r10, -0x176c
	ctx.r[3].s64 = ctx.r[10].s64 + -5996;
	// 8328730C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83287310: 4AFA5BC1  bl 0x8222ced0
	ctx.lr = 0x83287314;
	sub_8222CED0(ctx, base);
	// 83287314: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83287318: 38692468  addi r3, r9, 0x2468
	ctx.r[3].s64 = ctx.r[9].s64 + 9320;
	// 8328731C: 4BA22C05  bl 0x82ca9f20
	ctx.lr = 0x83287320;
	sub_82CA9F20(ctx, base);
	// 83287320: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83287324: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83287328: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8328732C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83287330(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83287330 size=64
    let mut pc: u32 = 0x83287330;
    'dispatch: loop {
        match pc {
            0x83287330 => {
    //   block [0x83287330..0x83287370)
	// 83287330: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83287334: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83287338: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8328733C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83287340: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83287344: 388BA62C  addi r4, r11, -0x59d4
	ctx.r[4].s64 = ctx.r[11].s64 + -22996;
	// 83287348: 386AE898  addi r3, r10, -0x1768
	ctx.r[3].s64 = ctx.r[10].s64 + -5992;
	// 8328734C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83287350: 4AFA5B81  bl 0x8222ced0
	ctx.lr = 0x83287354;
	sub_8222CED0(ctx, base);
	// 83287354: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83287358: 38692478  addi r3, r9, 0x2478
	ctx.r[3].s64 = ctx.r[9].s64 + 9336;
	// 8328735C: 4BA22BC5  bl 0x82ca9f20
	ctx.lr = 0x83287360;
	sub_82CA9F20(ctx, base);
	// 83287360: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83287364: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83287368: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8328736C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83287370(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83287370 size=64
    let mut pc: u32 = 0x83287370;
    'dispatch: loop {
        match pc {
            0x83287370 => {
    //   block [0x83287370..0x832873B0)
	// 83287370: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83287374: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83287378: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8328737C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83287380: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83287384: 388BA63C  addi r4, r11, -0x59c4
	ctx.r[4].s64 = ctx.r[11].s64 + -22980;
	// 83287388: 386AE89C  addi r3, r10, -0x1764
	ctx.r[3].s64 = ctx.r[10].s64 + -5988;
	// 8328738C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83287390: 4AFA5B41  bl 0x8222ced0
	ctx.lr = 0x83287394;
	sub_8222CED0(ctx, base);
	// 83287394: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83287398: 38692488  addi r3, r9, 0x2488
	ctx.r[3].s64 = ctx.r[9].s64 + 9352;
	// 8328739C: 4BA22B85  bl 0x82ca9f20
	ctx.lr = 0x832873A0;
	sub_82CA9F20(ctx, base);
	// 832873A0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832873A4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832873A8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832873AC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832873B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832873B0 size=64
    let mut pc: u32 = 0x832873B0;
    'dispatch: loop {
        match pc {
            0x832873B0 => {
    //   block [0x832873B0..0x832873F0)
	// 832873B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832873B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832873B8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832873BC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 832873C0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832873C4: 388BA650  addi r4, r11, -0x59b0
	ctx.r[4].s64 = ctx.r[11].s64 + -22960;
	// 832873C8: 386AE8A0  addi r3, r10, -0x1760
	ctx.r[3].s64 = ctx.r[10].s64 + -5984;
	// 832873CC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832873D0: 4AFA5B01  bl 0x8222ced0
	ctx.lr = 0x832873D4;
	sub_8222CED0(ctx, base);
	// 832873D4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 832873D8: 38692498  addi r3, r9, 0x2498
	ctx.r[3].s64 = ctx.r[9].s64 + 9368;
	// 832873DC: 4BA22B45  bl 0x82ca9f20
	ctx.lr = 0x832873E0;
	sub_82CA9F20(ctx, base);
	// 832873E0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832873E4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832873E8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832873EC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832873F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832873F0 size=64
    let mut pc: u32 = 0x832873F0;
    'dispatch: loop {
        match pc {
            0x832873F0 => {
    //   block [0x832873F0..0x83287430)
	// 832873F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832873F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832873F8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832873FC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83287400: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83287404: 388BA664  addi r4, r11, -0x599c
	ctx.r[4].s64 = ctx.r[11].s64 + -22940;
	// 83287408: 386AE8A4  addi r3, r10, -0x175c
	ctx.r[3].s64 = ctx.r[10].s64 + -5980;
	// 8328740C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83287410: 4AFA5AC1  bl 0x8222ced0
	ctx.lr = 0x83287414;
	sub_8222CED0(ctx, base);
	// 83287414: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83287418: 386924A8  addi r3, r9, 0x24a8
	ctx.r[3].s64 = ctx.r[9].s64 + 9384;
	// 8328741C: 4BA22B05  bl 0x82ca9f20
	ctx.lr = 0x83287420;
	sub_82CA9F20(ctx, base);
	// 83287420: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83287424: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83287428: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8328742C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83287430(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83287430 size=64
    let mut pc: u32 = 0x83287430;
    'dispatch: loop {
        match pc {
            0x83287430 => {
    //   block [0x83287430..0x83287470)
	// 83287430: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83287434: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83287438: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8328743C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83287440: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83287444: 388BA67C  addi r4, r11, -0x5984
	ctx.r[4].s64 = ctx.r[11].s64 + -22916;
	// 83287448: 386AE8A8  addi r3, r10, -0x1758
	ctx.r[3].s64 = ctx.r[10].s64 + -5976;
	// 8328744C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83287450: 4AFA5A81  bl 0x8222ced0
	ctx.lr = 0x83287454;
	sub_8222CED0(ctx, base);
	// 83287454: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83287458: 386924B8  addi r3, r9, 0x24b8
	ctx.r[3].s64 = ctx.r[9].s64 + 9400;
	// 8328745C: 4BA22AC5  bl 0x82ca9f20
	ctx.lr = 0x83287460;
	sub_82CA9F20(ctx, base);
	// 83287460: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83287464: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83287468: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8328746C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83287470(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83287470 size=64
    let mut pc: u32 = 0x83287470;
    'dispatch: loop {
        match pc {
            0x83287470 => {
    //   block [0x83287470..0x832874B0)
	// 83287470: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83287474: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83287478: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8328747C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83287480: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83287484: 388BA690  addi r4, r11, -0x5970
	ctx.r[4].s64 = ctx.r[11].s64 + -22896;
	// 83287488: 386AE8AC  addi r3, r10, -0x1754
	ctx.r[3].s64 = ctx.r[10].s64 + -5972;
	// 8328748C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83287490: 4AFA5A41  bl 0x8222ced0
	ctx.lr = 0x83287494;
	sub_8222CED0(ctx, base);
	// 83287494: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83287498: 386924C8  addi r3, r9, 0x24c8
	ctx.r[3].s64 = ctx.r[9].s64 + 9416;
	// 8328749C: 4BA22A85  bl 0x82ca9f20
	ctx.lr = 0x832874A0;
	sub_82CA9F20(ctx, base);
	// 832874A0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832874A4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832874A8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832874AC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832874B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832874B0 size=64
    let mut pc: u32 = 0x832874B0;
    'dispatch: loop {
        match pc {
            0x832874B0 => {
    //   block [0x832874B0..0x832874F0)
	// 832874B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832874B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832874B8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832874BC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 832874C0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832874C4: 388BA6A0  addi r4, r11, -0x5960
	ctx.r[4].s64 = ctx.r[11].s64 + -22880;
	// 832874C8: 386AE8B0  addi r3, r10, -0x1750
	ctx.r[3].s64 = ctx.r[10].s64 + -5968;
	// 832874CC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832874D0: 4AFA5A01  bl 0x8222ced0
	ctx.lr = 0x832874D4;
	sub_8222CED0(ctx, base);
	// 832874D4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 832874D8: 386924D8  addi r3, r9, 0x24d8
	ctx.r[3].s64 = ctx.r[9].s64 + 9432;
	// 832874DC: 4BA22A45  bl 0x82ca9f20
	ctx.lr = 0x832874E0;
	sub_82CA9F20(ctx, base);
	// 832874E0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832874E4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832874E8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832874EC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832874F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832874F0 size=64
    let mut pc: u32 = 0x832874F0;
    'dispatch: loop {
        match pc {
            0x832874F0 => {
    //   block [0x832874F0..0x83287530)
	// 832874F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832874F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832874F8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832874FC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83287500: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83287504: 388BA54C  addi r4, r11, -0x5ab4
	ctx.r[4].s64 = ctx.r[11].s64 + -23220;
	// 83287508: 386AE8B4  addi r3, r10, -0x174c
	ctx.r[3].s64 = ctx.r[10].s64 + -5964;
	// 8328750C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83287510: 4AFA59C1  bl 0x8222ced0
	ctx.lr = 0x83287514;
	sub_8222CED0(ctx, base);
	// 83287514: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83287518: 386924E8  addi r3, r9, 0x24e8
	ctx.r[3].s64 = ctx.r[9].s64 + 9448;
	// 8328751C: 4BA22A05  bl 0x82ca9f20
	ctx.lr = 0x83287520;
	sub_82CA9F20(ctx, base);
	// 83287520: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83287524: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83287528: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8328752C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83287530(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83287530 size=64
    let mut pc: u32 = 0x83287530;
    'dispatch: loop {
        match pc {
            0x83287530 => {
    //   block [0x83287530..0x83287570)
	// 83287530: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83287534: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83287538: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8328753C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83287540: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83287544: 388BA554  addi r4, r11, -0x5aac
	ctx.r[4].s64 = ctx.r[11].s64 + -23212;
	// 83287548: 386AE8B8  addi r3, r10, -0x1748
	ctx.r[3].s64 = ctx.r[10].s64 + -5960;
	// 8328754C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83287550: 4AFA5981  bl 0x8222ced0
	ctx.lr = 0x83287554;
	sub_8222CED0(ctx, base);
	// 83287554: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83287558: 386924F8  addi r3, r9, 0x24f8
	ctx.r[3].s64 = ctx.r[9].s64 + 9464;
	// 8328755C: 4BA229C5  bl 0x82ca9f20
	ctx.lr = 0x83287560;
	sub_82CA9F20(ctx, base);
	// 83287560: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83287564: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83287568: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8328756C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83287570(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83287570 size=64
    let mut pc: u32 = 0x83287570;
    'dispatch: loop {
        match pc {
            0x83287570 => {
    //   block [0x83287570..0x832875B0)
	// 83287570: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83287574: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83287578: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8328757C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83287580: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83287584: 388BA568  addi r4, r11, -0x5a98
	ctx.r[4].s64 = ctx.r[11].s64 + -23192;
	// 83287588: 386AE8BC  addi r3, r10, -0x1744
	ctx.r[3].s64 = ctx.r[10].s64 + -5956;
	// 8328758C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83287590: 4AFA5941  bl 0x8222ced0
	ctx.lr = 0x83287594;
	sub_8222CED0(ctx, base);
	// 83287594: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83287598: 38692508  addi r3, r9, 0x2508
	ctx.r[3].s64 = ctx.r[9].s64 + 9480;
	// 8328759C: 4BA22985  bl 0x82ca9f20
	ctx.lr = 0x832875A0;
	sub_82CA9F20(ctx, base);
	// 832875A0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832875A4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832875A8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832875AC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832875B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832875B0 size=64
    let mut pc: u32 = 0x832875B0;
    'dispatch: loop {
        match pc {
            0x832875B0 => {
    //   block [0x832875B0..0x832875F0)
	// 832875B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832875B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832875B8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832875BC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 832875C0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832875C4: 388BA57C  addi r4, r11, -0x5a84
	ctx.r[4].s64 = ctx.r[11].s64 + -23172;
	// 832875C8: 386AE8C0  addi r3, r10, -0x1740
	ctx.r[3].s64 = ctx.r[10].s64 + -5952;
	// 832875CC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832875D0: 4AFA5901  bl 0x8222ced0
	ctx.lr = 0x832875D4;
	sub_8222CED0(ctx, base);
	// 832875D4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 832875D8: 38692518  addi r3, r9, 0x2518
	ctx.r[3].s64 = ctx.r[9].s64 + 9496;
	// 832875DC: 4BA22945  bl 0x82ca9f20
	ctx.lr = 0x832875E0;
	sub_82CA9F20(ctx, base);
	// 832875E0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832875E4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832875E8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832875EC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832875F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832875F0 size=64
    let mut pc: u32 = 0x832875F0;
    'dispatch: loop {
        match pc {
            0x832875F0 => {
    //   block [0x832875F0..0x83287630)
	// 832875F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832875F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832875F8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832875FC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83287600: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83287604: 388BA4E4  addi r4, r11, -0x5b1c
	ctx.r[4].s64 = ctx.r[11].s64 + -23324;
	// 83287608: 386AE8C4  addi r3, r10, -0x173c
	ctx.r[3].s64 = ctx.r[10].s64 + -5948;
	// 8328760C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83287610: 4AFA58C1  bl 0x8222ced0
	ctx.lr = 0x83287614;
	sub_8222CED0(ctx, base);
	// 83287614: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83287618: 38692528  addi r3, r9, 0x2528
	ctx.r[3].s64 = ctx.r[9].s64 + 9512;
	// 8328761C: 4BA22905  bl 0x82ca9f20
	ctx.lr = 0x83287620;
	sub_82CA9F20(ctx, base);
	// 83287620: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83287624: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83287628: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8328762C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83287630(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83287630 size=64
    let mut pc: u32 = 0x83287630;
    'dispatch: loop {
        match pc {
            0x83287630 => {
    //   block [0x83287630..0x83287670)
	// 83287630: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83287634: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83287638: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8328763C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83287640: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83287644: 388BA4EC  addi r4, r11, -0x5b14
	ctx.r[4].s64 = ctx.r[11].s64 + -23316;
	// 83287648: 386AE8C8  addi r3, r10, -0x1738
	ctx.r[3].s64 = ctx.r[10].s64 + -5944;
	// 8328764C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83287650: 4AFA5881  bl 0x8222ced0
	ctx.lr = 0x83287654;
	sub_8222CED0(ctx, base);
	// 83287654: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83287658: 38692538  addi r3, r9, 0x2538
	ctx.r[3].s64 = ctx.r[9].s64 + 9528;
	// 8328765C: 4BA228C5  bl 0x82ca9f20
	ctx.lr = 0x83287660;
	sub_82CA9F20(ctx, base);
	// 83287660: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83287664: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83287668: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8328766C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83287670(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83287670 size=64
    let mut pc: u32 = 0x83287670;
    'dispatch: loop {
        match pc {
            0x83287670 => {
    //   block [0x83287670..0x832876B0)
	// 83287670: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83287674: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83287678: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8328767C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83287680: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83287684: 388BA500  addi r4, r11, -0x5b00
	ctx.r[4].s64 = ctx.r[11].s64 + -23296;
	// 83287688: 386AE8CC  addi r3, r10, -0x1734
	ctx.r[3].s64 = ctx.r[10].s64 + -5940;
	// 8328768C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83287690: 4AFA5841  bl 0x8222ced0
	ctx.lr = 0x83287694;
	sub_8222CED0(ctx, base);
	// 83287694: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83287698: 38692548  addi r3, r9, 0x2548
	ctx.r[3].s64 = ctx.r[9].s64 + 9544;
	// 8328769C: 4BA22885  bl 0x82ca9f20
	ctx.lr = 0x832876A0;
	sub_82CA9F20(ctx, base);
	// 832876A0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832876A4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832876A8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832876AC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832876B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832876B0 size=64
    let mut pc: u32 = 0x832876B0;
    'dispatch: loop {
        match pc {
            0x832876B0 => {
    //   block [0x832876B0..0x832876F0)
	// 832876B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832876B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832876B8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832876BC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 832876C0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832876C4: 388BA514  addi r4, r11, -0x5aec
	ctx.r[4].s64 = ctx.r[11].s64 + -23276;
	// 832876C8: 386AE8D0  addi r3, r10, -0x1730
	ctx.r[3].s64 = ctx.r[10].s64 + -5936;
	// 832876CC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832876D0: 4AFA5801  bl 0x8222ced0
	ctx.lr = 0x832876D4;
	sub_8222CED0(ctx, base);
	// 832876D4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 832876D8: 38692558  addi r3, r9, 0x2558
	ctx.r[3].s64 = ctx.r[9].s64 + 9560;
	// 832876DC: 4BA22845  bl 0x82ca9f20
	ctx.lr = 0x832876E0;
	sub_82CA9F20(ctx, base);
	// 832876E0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832876E4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832876E8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832876EC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832876F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832876F0 size=64
    let mut pc: u32 = 0x832876F0;
    'dispatch: loop {
        match pc {
            0x832876F0 => {
    //   block [0x832876F0..0x83287730)
	// 832876F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832876F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832876F8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832876FC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83287700: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83287704: 388BA528  addi r4, r11, -0x5ad8
	ctx.r[4].s64 = ctx.r[11].s64 + -23256;
	// 83287708: 386AE8D4  addi r3, r10, -0x172c
	ctx.r[3].s64 = ctx.r[10].s64 + -5932;
	// 8328770C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83287710: 4AFA57C1  bl 0x8222ced0
	ctx.lr = 0x83287714;
	sub_8222CED0(ctx, base);
	// 83287714: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83287718: 38692568  addi r3, r9, 0x2568
	ctx.r[3].s64 = ctx.r[9].s64 + 9576;
	// 8328771C: 4BA22805  bl 0x82ca9f20
	ctx.lr = 0x83287720;
	sub_82CA9F20(ctx, base);
	// 83287720: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83287724: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83287728: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8328772C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83287730(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83287730 size=64
    let mut pc: u32 = 0x83287730;
    'dispatch: loop {
        match pc {
            0x83287730 => {
    //   block [0x83287730..0x83287770)
	// 83287730: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83287734: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83287738: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8328773C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83287740: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83287744: 388BA404  addi r4, r11, -0x5bfc
	ctx.r[4].s64 = ctx.r[11].s64 + -23548;
	// 83287748: 386AE8D8  addi r3, r10, -0x1728
	ctx.r[3].s64 = ctx.r[10].s64 + -5928;
	// 8328774C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83287750: 4AFA5781  bl 0x8222ced0
	ctx.lr = 0x83287754;
	sub_8222CED0(ctx, base);
	// 83287754: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83287758: 38692578  addi r3, r9, 0x2578
	ctx.r[3].s64 = ctx.r[9].s64 + 9592;
	// 8328775C: 4BA227C5  bl 0x82ca9f20
	ctx.lr = 0x83287760;
	sub_82CA9F20(ctx, base);
	// 83287760: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83287764: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83287768: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8328776C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83287770(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83287770 size=64
    let mut pc: u32 = 0x83287770;
    'dispatch: loop {
        match pc {
            0x83287770 => {
    //   block [0x83287770..0x832877B0)
	// 83287770: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83287774: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83287778: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8328777C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83287780: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83287784: 388BA40C  addi r4, r11, -0x5bf4
	ctx.r[4].s64 = ctx.r[11].s64 + -23540;
	// 83287788: 386AE8DC  addi r3, r10, -0x1724
	ctx.r[3].s64 = ctx.r[10].s64 + -5924;
	// 8328778C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83287790: 4AFA5741  bl 0x8222ced0
	ctx.lr = 0x83287794;
	sub_8222CED0(ctx, base);
	// 83287794: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83287798: 38692588  addi r3, r9, 0x2588
	ctx.r[3].s64 = ctx.r[9].s64 + 9608;
	// 8328779C: 4BA22785  bl 0x82ca9f20
	ctx.lr = 0x832877A0;
	sub_82CA9F20(ctx, base);
	// 832877A0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832877A4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832877A8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832877AC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832877B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832877B0 size=64
    let mut pc: u32 = 0x832877B0;
    'dispatch: loop {
        match pc {
            0x832877B0 => {
    //   block [0x832877B0..0x832877F0)
	// 832877B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832877B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832877B8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832877BC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 832877C0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832877C4: 388BA420  addi r4, r11, -0x5be0
	ctx.r[4].s64 = ctx.r[11].s64 + -23520;
	// 832877C8: 386AE8E0  addi r3, r10, -0x1720
	ctx.r[3].s64 = ctx.r[10].s64 + -5920;
	// 832877CC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832877D0: 4AFA5701  bl 0x8222ced0
	ctx.lr = 0x832877D4;
	sub_8222CED0(ctx, base);
	// 832877D4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 832877D8: 38692598  addi r3, r9, 0x2598
	ctx.r[3].s64 = ctx.r[9].s64 + 9624;
	// 832877DC: 4BA22745  bl 0x82ca9f20
	ctx.lr = 0x832877E0;
	sub_82CA9F20(ctx, base);
	// 832877E0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832877E4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832877E8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832877EC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832877F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x832877F0 size=316
    let mut pc: u32 = 0x832877F0;
    'dispatch: loop {
        match pc {
            0x832877F0 => {
    //   block [0x832877F0..0x8328792C)
	// 832877F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832877F4: 4BA21C11  bl 0x82ca9404
	ctx.lr = 0x832877F8;
	sub_82CA93D0(ctx, base);
	// 832877F8: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 832877FC: 3901FFA4  addi r8, r1, -0x5c
	ctx.r[8].s64 = ctx.r[1].s64 + -92;
	// 83287800: 392BD5C8  addi r9, r11, -0x2a38
	ctx.r[9].s64 = ctx.r[11].s64 + -10808;
	// 83287804: 38E1FFA8  addi r7, r1, -0x58
	ctx.r[7].s64 = ctx.r[1].s64 + -88;
	// 83287808: 3941FFA0  addi r10, r1, -0x60
	ctx.r[10].s64 = ctx.r[1].s64 + -96;
	// 8328780C: 3BA1FFC0  addi r29, r1, -0x40
	ctx.r[29].s64 = ctx.r[1].s64 + -64;
	// 83287810: C18BD5C8  lfs f12, -0x2a38(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-10808 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 83287814: 3B81FFC4  addi r28, r1, -0x3c
	ctx.r[28].s64 = ctx.r[1].s64 + -60;
	// 83287818: D181FFA0  stfs f12, -0x60(r1)
	tmp.f32 = (ctx.f[12].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-96 as u32), tmp.u32 ) };
	// 8328781C: C009BEBC  lfs f0, -0x4144(r9)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(-16708 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 83287820: 38A1FFB0  addi r5, r1, -0x50
	ctx.r[5].s64 = ctx.r[1].s64 + -80;
	// 83287824: D001FFA4  stfs f0, -0x5c(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-92 as u32), tmp.u32 ) };
	// 83287828: 38C1FFAC  addi r6, r1, -0x54
	ctx.r[6].s64 = ctx.r[1].s64 + -84;
	// 8328782C: D001FFA8  stfs f0, -0x58(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-88 as u32), tmp.u32 ) };
	// 83287830: 3BE1FFB8  addi r31, r1, -0x48
	ctx.r[31].s64 = ctx.r[1].s64 + -72;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83287930(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83287930 size=64
    let mut pc: u32 = 0x83287930;
    'dispatch: loop {
        match pc {
            0x83287930 => {
    //   block [0x83287930..0x83287970)
	// 83287930: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83287934: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83287938: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8328793C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83287940: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83287944: 388BA434  addi r4, r11, -0x5bcc
	ctx.r[4].s64 = ctx.r[11].s64 + -23500;
	// 83287948: 386AE8E4  addi r3, r10, -0x171c
	ctx.r[3].s64 = ctx.r[10].s64 + -5916;
	// 8328794C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83287950: 4AFA5581  bl 0x8222ced0
	ctx.lr = 0x83287954;
	sub_8222CED0(ctx, base);
	// 83287954: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83287958: 386925A8  addi r3, r9, 0x25a8
	ctx.r[3].s64 = ctx.r[9].s64 + 9640;
	// 8328795C: 4BA225C5  bl 0x82ca9f20
	ctx.lr = 0x83287960;
	sub_82CA9F20(ctx, base);
	// 83287960: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83287964: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83287968: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8328796C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83287970(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83287970 size=64
    let mut pc: u32 = 0x83287970;
    'dispatch: loop {
        match pc {
            0x83287970 => {
    //   block [0x83287970..0x832879B0)
	// 83287970: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83287974: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83287978: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8328797C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83287980: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83287984: 388BA454  addi r4, r11, -0x5bac
	ctx.r[4].s64 = ctx.r[11].s64 + -23468;
	// 83287988: 386AE8E8  addi r3, r10, -0x1718
	ctx.r[3].s64 = ctx.r[10].s64 + -5912;
	// 8328798C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83287990: 4AFA5541  bl 0x8222ced0
	ctx.lr = 0x83287994;
	sub_8222CED0(ctx, base);
	// 83287994: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83287998: 386925B8  addi r3, r9, 0x25b8
	ctx.r[3].s64 = ctx.r[9].s64 + 9656;
	// 8328799C: 4BA22585  bl 0x82ca9f20
	ctx.lr = 0x832879A0;
	sub_82CA9F20(ctx, base);
	// 832879A0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832879A4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832879A8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832879AC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832879B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832879B0 size=64
    let mut pc: u32 = 0x832879B0;
    'dispatch: loop {
        match pc {
            0x832879B0 => {
    //   block [0x832879B0..0x832879F0)
	// 832879B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832879B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832879B8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832879BC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 832879C0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832879C4: 388BA464  addi r4, r11, -0x5b9c
	ctx.r[4].s64 = ctx.r[11].s64 + -23452;
	// 832879C8: 386AE8EC  addi r3, r10, -0x1714
	ctx.r[3].s64 = ctx.r[10].s64 + -5908;
	// 832879CC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832879D0: 4AFA5501  bl 0x8222ced0
	ctx.lr = 0x832879D4;
	sub_8222CED0(ctx, base);
	// 832879D4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 832879D8: 386925C8  addi r3, r9, 0x25c8
	ctx.r[3].s64 = ctx.r[9].s64 + 9672;
	// 832879DC: 4BA22545  bl 0x82ca9f20
	ctx.lr = 0x832879E0;
	sub_82CA9F20(ctx, base);
	// 832879E0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832879E4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832879E8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832879EC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832879F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832879F0 size=64
    let mut pc: u32 = 0x832879F0;
    'dispatch: loop {
        match pc {
            0x832879F0 => {
    //   block [0x832879F0..0x83287A30)
	// 832879F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832879F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832879F8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832879FC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83287A00: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83287A04: 388BA484  addi r4, r11, -0x5b7c
	ctx.r[4].s64 = ctx.r[11].s64 + -23420;
	// 83287A08: 386AE930  addi r3, r10, -0x16d0
	ctx.r[3].s64 = ctx.r[10].s64 + -5840;
	// 83287A0C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83287A10: 4AFA54C1  bl 0x8222ced0
	ctx.lr = 0x83287A14;
	sub_8222CED0(ctx, base);
	// 83287A14: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83287A18: 386925D8  addi r3, r9, 0x25d8
	ctx.r[3].s64 = ctx.r[9].s64 + 9688;
	// 83287A1C: 4BA22505  bl 0x82ca9f20
	ctx.lr = 0x83287A20;
	sub_82CA9F20(ctx, base);
	// 83287A20: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83287A24: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83287A28: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83287A2C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83287A30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83287A30 size=64
    let mut pc: u32 = 0x83287A30;
    'dispatch: loop {
        match pc {
            0x83287A30 => {
    //   block [0x83287A30..0x83287A70)
	// 83287A30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83287A34: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83287A38: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83287A3C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83287A40: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83287A44: 388BA4A0  addi r4, r11, -0x5b60
	ctx.r[4].s64 = ctx.r[11].s64 + -23392;
	// 83287A48: 386AE934  addi r3, r10, -0x16cc
	ctx.r[3].s64 = ctx.r[10].s64 + -5836;
	// 83287A4C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83287A50: 4AFA5481  bl 0x8222ced0
	ctx.lr = 0x83287A54;
	sub_8222CED0(ctx, base);
	// 83287A54: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83287A58: 386925E8  addi r3, r9, 0x25e8
	ctx.r[3].s64 = ctx.r[9].s64 + 9704;
	// 83287A5C: 4BA224C5  bl 0x82ca9f20
	ctx.lr = 0x83287A60;
	sub_82CA9F20(ctx, base);
	// 83287A60: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83287A64: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83287A68: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83287A6C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83287A70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83287A70 size=64
    let mut pc: u32 = 0x83287A70;
    'dispatch: loop {
        match pc {
            0x83287A70 => {
    //   block [0x83287A70..0x83287AB0)
	// 83287A70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83287A74: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83287A78: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83287A7C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83287A80: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83287A84: 388BA4B0  addi r4, r11, -0x5b50
	ctx.r[4].s64 = ctx.r[11].s64 + -23376;
	// 83287A88: 386AE938  addi r3, r10, -0x16c8
	ctx.r[3].s64 = ctx.r[10].s64 + -5832;
	// 83287A8C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83287A90: 4AFA5441  bl 0x8222ced0
	ctx.lr = 0x83287A94;
	sub_8222CED0(ctx, base);
	// 83287A94: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83287A98: 386925F8  addi r3, r9, 0x25f8
	ctx.r[3].s64 = ctx.r[9].s64 + 9720;
	// 83287A9C: 4BA22485  bl 0x82ca9f20
	ctx.lr = 0x83287AA0;
	sub_82CA9F20(ctx, base);
	// 83287AA0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83287AA4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83287AA8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83287AAC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83287AB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83287AB0 size=64
    let mut pc: u32 = 0x83287AB0;
    'dispatch: loop {
        match pc {
            0x83287AB0 => {
    //   block [0x83287AB0..0x83287AF0)
	// 83287AB0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83287AB4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83287AB8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83287ABC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83287AC0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83287AC4: 388BA4C0  addi r4, r11, -0x5b40
	ctx.r[4].s64 = ctx.r[11].s64 + -23360;
	// 83287AC8: 386AE93C  addi r3, r10, -0x16c4
	ctx.r[3].s64 = ctx.r[10].s64 + -5828;
	// 83287ACC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83287AD0: 4AFA5401  bl 0x8222ced0
	ctx.lr = 0x83287AD4;
	sub_8222CED0(ctx, base);
	// 83287AD4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83287AD8: 38692608  addi r3, r9, 0x2608
	ctx.r[3].s64 = ctx.r[9].s64 + 9736;
	// 83287ADC: 4BA22445  bl 0x82ca9f20
	ctx.lr = 0x83287AE0;
	sub_82CA9F20(ctx, base);
	// 83287AE0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83287AE4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83287AE8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83287AEC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83287AF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83287AF0 size=64
    let mut pc: u32 = 0x83287AF0;
    'dispatch: loop {
        match pc {
            0x83287AF0 => {
    //   block [0x83287AF0..0x83287B30)
	// 83287AF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83287AF4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83287AF8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83287AFC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83287B00: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83287B04: 388BA3B8  addi r4, r11, -0x5c48
	ctx.r[4].s64 = ctx.r[11].s64 + -23624;
	// 83287B08: 386AE940  addi r3, r10, -0x16c0
	ctx.r[3].s64 = ctx.r[10].s64 + -5824;
	// 83287B0C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83287B10: 4AFA53C1  bl 0x8222ced0
	ctx.lr = 0x83287B14;
	sub_8222CED0(ctx, base);
	// 83287B14: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83287B18: 38692618  addi r3, r9, 0x2618
	ctx.r[3].s64 = ctx.r[9].s64 + 9752;
	// 83287B1C: 4BA22405  bl 0x82ca9f20
	ctx.lr = 0x83287B20;
	sub_82CA9F20(ctx, base);
	// 83287B20: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83287B24: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83287B28: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83287B2C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83287B30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83287B30 size=64
    let mut pc: u32 = 0x83287B30;
    'dispatch: loop {
        match pc {
            0x83287B30 => {
    //   block [0x83287B30..0x83287B70)
	// 83287B30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83287B34: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83287B38: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83287B3C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83287B40: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83287B44: 388BA3C8  addi r4, r11, -0x5c38
	ctx.r[4].s64 = ctx.r[11].s64 + -23608;
	// 83287B48: 386AE944  addi r3, r10, -0x16bc
	ctx.r[3].s64 = ctx.r[10].s64 + -5820;
	// 83287B4C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83287B50: 4AFA5381  bl 0x8222ced0
	ctx.lr = 0x83287B54;
	sub_8222CED0(ctx, base);
	// 83287B54: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83287B58: 38692628  addi r3, r9, 0x2628
	ctx.r[3].s64 = ctx.r[9].s64 + 9768;
	// 83287B5C: 4BA223C5  bl 0x82ca9f20
	ctx.lr = 0x83287B60;
	sub_82CA9F20(ctx, base);
	// 83287B60: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83287B64: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83287B68: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83287B6C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83287B70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83287B70 size=64
    let mut pc: u32 = 0x83287B70;
    'dispatch: loop {
        match pc {
            0x83287B70 => {
    //   block [0x83287B70..0x83287BB0)
	// 83287B70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83287B74: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83287B78: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83287B7C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83287B80: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83287B84: 388BA3D8  addi r4, r11, -0x5c28
	ctx.r[4].s64 = ctx.r[11].s64 + -23592;
	// 83287B88: 386AE948  addi r3, r10, -0x16b8
	ctx.r[3].s64 = ctx.r[10].s64 + -5816;
	// 83287B8C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83287B90: 4AFA5341  bl 0x8222ced0
	ctx.lr = 0x83287B94;
	sub_8222CED0(ctx, base);
	// 83287B94: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83287B98: 38692638  addi r3, r9, 0x2638
	ctx.r[3].s64 = ctx.r[9].s64 + 9784;
	// 83287B9C: 4BA22385  bl 0x82ca9f20
	ctx.lr = 0x83287BA0;
	sub_82CA9F20(ctx, base);
	// 83287BA0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83287BA4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83287BA8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83287BAC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83287BB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83287BB0 size=64
    let mut pc: u32 = 0x83287BB0;
    'dispatch: loop {
        match pc {
            0x83287BB0 => {
    //   block [0x83287BB0..0x83287BF0)
	// 83287BB0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83287BB4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83287BB8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83287BBC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83287BC0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83287BC4: 388BA3DC  addi r4, r11, -0x5c24
	ctx.r[4].s64 = ctx.r[11].s64 + -23588;
	// 83287BC8: 386AE94C  addi r3, r10, -0x16b4
	ctx.r[3].s64 = ctx.r[10].s64 + -5812;
	// 83287BCC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83287BD0: 4AFA5301  bl 0x8222ced0
	ctx.lr = 0x83287BD4;
	sub_8222CED0(ctx, base);
	// 83287BD4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83287BD8: 38692648  addi r3, r9, 0x2648
	ctx.r[3].s64 = ctx.r[9].s64 + 9800;
	// 83287BDC: 4BA22345  bl 0x82ca9f20
	ctx.lr = 0x83287BE0;
	sub_82CA9F20(ctx, base);
	// 83287BE0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83287BE4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83287BE8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83287BEC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83287BF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83287BF0 size=64
    let mut pc: u32 = 0x83287BF0;
    'dispatch: loop {
        match pc {
            0x83287BF0 => {
    //   block [0x83287BF0..0x83287C30)
	// 83287BF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83287BF4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83287BF8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83287BFC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83287C00: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83287C04: 388BA3E8  addi r4, r11, -0x5c18
	ctx.r[4].s64 = ctx.r[11].s64 + -23576;
	// 83287C08: 386AE950  addi r3, r10, -0x16b0
	ctx.r[3].s64 = ctx.r[10].s64 + -5808;
	// 83287C0C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83287C10: 4AFA52C1  bl 0x8222ced0
	ctx.lr = 0x83287C14;
	sub_8222CED0(ctx, base);
	// 83287C14: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83287C18: 38692658  addi r3, r9, 0x2658
	ctx.r[3].s64 = ctx.r[9].s64 + 9816;
	// 83287C1C: 4BA22305  bl 0x82ca9f20
	ctx.lr = 0x83287C20;
	sub_82CA9F20(ctx, base);
	// 83287C20: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83287C24: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83287C28: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83287C2C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83287C30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83287C30 size=64
    let mut pc: u32 = 0x83287C30;
    'dispatch: loop {
        match pc {
            0x83287C30 => {
    //   block [0x83287C30..0x83287C70)
	// 83287C30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83287C34: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83287C38: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83287C3C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83287C40: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83287C44: 388BA3F4  addi r4, r11, -0x5c0c
	ctx.r[4].s64 = ctx.r[11].s64 + -23564;
	// 83287C48: 386AE954  addi r3, r10, -0x16ac
	ctx.r[3].s64 = ctx.r[10].s64 + -5804;
	// 83287C4C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83287C50: 4AFA5281  bl 0x8222ced0
	ctx.lr = 0x83287C54;
	sub_8222CED0(ctx, base);
	// 83287C54: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83287C58: 38692668  addi r3, r9, 0x2668
	ctx.r[3].s64 = ctx.r[9].s64 + 9832;
	// 83287C5C: 4BA222C5  bl 0x82ca9f20
	ctx.lr = 0x83287C60;
	sub_82CA9F20(ctx, base);
	// 83287C60: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83287C64: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83287C68: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83287C6C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83287C70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83287C70 size=64
    let mut pc: u32 = 0x83287C70;
    'dispatch: loop {
        match pc {
            0x83287C70 => {
    //   block [0x83287C70..0x83287CB0)
	// 83287C70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83287C74: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83287C78: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83287C7C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83287C80: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83287C84: 388BA3FC  addi r4, r11, -0x5c04
	ctx.r[4].s64 = ctx.r[11].s64 + -23556;
	// 83287C88: 386AE958  addi r3, r10, -0x16a8
	ctx.r[3].s64 = ctx.r[10].s64 + -5800;
	// 83287C8C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83287C90: 4AFA5241  bl 0x8222ced0
	ctx.lr = 0x83287C94;
	sub_8222CED0(ctx, base);
	// 83287C94: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83287C98: 38692678  addi r3, r9, 0x2678
	ctx.r[3].s64 = ctx.r[9].s64 + 9848;
	// 83287C9C: 4BA22285  bl 0x82ca9f20
	ctx.lr = 0x83287CA0;
	sub_82CA9F20(ctx, base);
	// 83287CA0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83287CA4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83287CA8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83287CAC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83287CB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83287CB0 size=64
    let mut pc: u32 = 0x83287CB0;
    'dispatch: loop {
        match pc {
            0x83287CB0 => {
    //   block [0x83287CB0..0x83287CF0)
	// 83287CB0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83287CB4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83287CB8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83287CBC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83287CC0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83287CC4: 388BA304  addi r4, r11, -0x5cfc
	ctx.r[4].s64 = ctx.r[11].s64 + -23804;
	// 83287CC8: 386AE95C  addi r3, r10, -0x16a4
	ctx.r[3].s64 = ctx.r[10].s64 + -5796;
	// 83287CCC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83287CD0: 4AFA5201  bl 0x8222ced0
	ctx.lr = 0x83287CD4;
	sub_8222CED0(ctx, base);
	// 83287CD4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83287CD8: 38692688  addi r3, r9, 0x2688
	ctx.r[3].s64 = ctx.r[9].s64 + 9864;
	// 83287CDC: 4BA22245  bl 0x82ca9f20
	ctx.lr = 0x83287CE0;
	sub_82CA9F20(ctx, base);
	// 83287CE0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83287CE4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83287CE8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83287CEC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83287CF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83287CF0 size=64
    let mut pc: u32 = 0x83287CF0;
    'dispatch: loop {
        match pc {
            0x83287CF0 => {
    //   block [0x83287CF0..0x83287D30)
	// 83287CF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83287CF4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83287CF8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83287CFC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83287D00: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83287D04: 388BA30C  addi r4, r11, -0x5cf4
	ctx.r[4].s64 = ctx.r[11].s64 + -23796;
	// 83287D08: 386AE960  addi r3, r10, -0x16a0
	ctx.r[3].s64 = ctx.r[10].s64 + -5792;
	// 83287D0C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83287D10: 4AFA51C1  bl 0x8222ced0
	ctx.lr = 0x83287D14;
	sub_8222CED0(ctx, base);
	// 83287D14: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83287D18: 38692698  addi r3, r9, 0x2698
	ctx.r[3].s64 = ctx.r[9].s64 + 9880;
	// 83287D1C: 4BA22205  bl 0x82ca9f20
	ctx.lr = 0x83287D20;
	sub_82CA9F20(ctx, base);
	// 83287D20: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83287D24: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83287D28: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83287D2C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83287D30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83287D30 size=64
    let mut pc: u32 = 0x83287D30;
    'dispatch: loop {
        match pc {
            0x83287D30 => {
    //   block [0x83287D30..0x83287D70)
	// 83287D30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83287D34: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83287D38: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83287D3C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83287D40: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83287D44: 388BA320  addi r4, r11, -0x5ce0
	ctx.r[4].s64 = ctx.r[11].s64 + -23776;
	// 83287D48: 386AE964  addi r3, r10, -0x169c
	ctx.r[3].s64 = ctx.r[10].s64 + -5788;
	// 83287D4C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83287D50: 4AFA5181  bl 0x8222ced0
	ctx.lr = 0x83287D54;
	sub_8222CED0(ctx, base);
	// 83287D54: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83287D58: 386926A8  addi r3, r9, 0x26a8
	ctx.r[3].s64 = ctx.r[9].s64 + 9896;
	// 83287D5C: 4BA221C5  bl 0x82ca9f20
	ctx.lr = 0x83287D60;
	sub_82CA9F20(ctx, base);
	// 83287D60: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83287D64: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83287D68: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83287D6C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83287D70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83287D70 size=64
    let mut pc: u32 = 0x83287D70;
    'dispatch: loop {
        match pc {
            0x83287D70 => {
    //   block [0x83287D70..0x83287DB0)
	// 83287D70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83287D74: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83287D78: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83287D7C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83287D80: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83287D84: 388BA334  addi r4, r11, -0x5ccc
	ctx.r[4].s64 = ctx.r[11].s64 + -23756;
	// 83287D88: 386AE968  addi r3, r10, -0x1698
	ctx.r[3].s64 = ctx.r[10].s64 + -5784;
	// 83287D8C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83287D90: 4AFA5141  bl 0x8222ced0
	ctx.lr = 0x83287D94;
	sub_8222CED0(ctx, base);
	// 83287D94: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83287D98: 386926B8  addi r3, r9, 0x26b8
	ctx.r[3].s64 = ctx.r[9].s64 + 9912;
	// 83287D9C: 4BA22185  bl 0x82ca9f20
	ctx.lr = 0x83287DA0;
	sub_82CA9F20(ctx, base);
	// 83287DA0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83287DA4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83287DA8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83287DAC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83287DB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83287DB0 size=64
    let mut pc: u32 = 0x83287DB0;
    'dispatch: loop {
        match pc {
            0x83287DB0 => {
    //   block [0x83287DB0..0x83287DF0)
	// 83287DB0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83287DB4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83287DB8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83287DBC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83287DC0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83287DC4: 388BA34C  addi r4, r11, -0x5cb4
	ctx.r[4].s64 = ctx.r[11].s64 + -23732;
	// 83287DC8: 386AE96C  addi r3, r10, -0x1694
	ctx.r[3].s64 = ctx.r[10].s64 + -5780;
	// 83287DCC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83287DD0: 4AFA5101  bl 0x8222ced0
	ctx.lr = 0x83287DD4;
	sub_8222CED0(ctx, base);
	// 83287DD4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83287DD8: 386926C8  addi r3, r9, 0x26c8
	ctx.r[3].s64 = ctx.r[9].s64 + 9928;
	// 83287DDC: 4BA22145  bl 0x82ca9f20
	ctx.lr = 0x83287DE0;
	sub_82CA9F20(ctx, base);
	// 83287DE0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83287DE4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83287DE8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83287DEC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83287DF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83287DF0 size=64
    let mut pc: u32 = 0x83287DF0;
    'dispatch: loop {
        match pc {
            0x83287DF0 => {
    //   block [0x83287DF0..0x83287E30)
	// 83287DF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83287DF4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83287DF8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83287DFC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83287E00: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83287E04: 388BA364  addi r4, r11, -0x5c9c
	ctx.r[4].s64 = ctx.r[11].s64 + -23708;
	// 83287E08: 386AE970  addi r3, r10, -0x1690
	ctx.r[3].s64 = ctx.r[10].s64 + -5776;
	// 83287E0C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83287E10: 4AFA50C1  bl 0x8222ced0
	ctx.lr = 0x83287E14;
	sub_8222CED0(ctx, base);
	// 83287E14: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83287E18: 386926D8  addi r3, r9, 0x26d8
	ctx.r[3].s64 = ctx.r[9].s64 + 9944;
	// 83287E1C: 4BA22105  bl 0x82ca9f20
	ctx.lr = 0x83287E20;
	sub_82CA9F20(ctx, base);
	// 83287E20: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83287E24: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83287E28: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83287E2C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83287E30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83287E30 size=64
    let mut pc: u32 = 0x83287E30;
    'dispatch: loop {
        match pc {
            0x83287E30 => {
    //   block [0x83287E30..0x83287E70)
	// 83287E30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83287E34: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83287E38: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83287E3C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83287E40: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83287E44: 388BA268  addi r4, r11, -0x5d98
	ctx.r[4].s64 = ctx.r[11].s64 + -23960;
	// 83287E48: 386AE974  addi r3, r10, -0x168c
	ctx.r[3].s64 = ctx.r[10].s64 + -5772;
	// 83287E4C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83287E50: 4AFA5081  bl 0x8222ced0
	ctx.lr = 0x83287E54;
	sub_8222CED0(ctx, base);
	// 83287E54: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83287E58: 386926E8  addi r3, r9, 0x26e8
	ctx.r[3].s64 = ctx.r[9].s64 + 9960;
	// 83287E5C: 4BA220C5  bl 0x82ca9f20
	ctx.lr = 0x83287E60;
	sub_82CA9F20(ctx, base);
	// 83287E60: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83287E64: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83287E68: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83287E6C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83287E70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83287E70 size=64
    let mut pc: u32 = 0x83287E70;
    'dispatch: loop {
        match pc {
            0x83287E70 => {
    //   block [0x83287E70..0x83287EB0)
	// 83287E70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83287E74: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83287E78: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83287E7C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83287E80: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83287E84: 388BA270  addi r4, r11, -0x5d90
	ctx.r[4].s64 = ctx.r[11].s64 + -23952;
	// 83287E88: 386AE978  addi r3, r10, -0x1688
	ctx.r[3].s64 = ctx.r[10].s64 + -5768;
	// 83287E8C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83287E90: 4AFA5041  bl 0x8222ced0
	ctx.lr = 0x83287E94;
	sub_8222CED0(ctx, base);
	// 83287E94: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83287E98: 386926F8  addi r3, r9, 0x26f8
	ctx.r[3].s64 = ctx.r[9].s64 + 9976;
	// 83287E9C: 4BA22085  bl 0x82ca9f20
	ctx.lr = 0x83287EA0;
	sub_82CA9F20(ctx, base);
	// 83287EA0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83287EA4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83287EA8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83287EAC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83287EB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83287EB0 size=64
    let mut pc: u32 = 0x83287EB0;
    'dispatch: loop {
        match pc {
            0x83287EB0 => {
    //   block [0x83287EB0..0x83287EF0)
	// 83287EB0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83287EB4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83287EB8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83287EBC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83287EC0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83287EC4: 388BA284  addi r4, r11, -0x5d7c
	ctx.r[4].s64 = ctx.r[11].s64 + -23932;
	// 83287EC8: 386AE97C  addi r3, r10, -0x1684
	ctx.r[3].s64 = ctx.r[10].s64 + -5764;
	// 83287ECC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83287ED0: 4AFA5001  bl 0x8222ced0
	ctx.lr = 0x83287ED4;
	sub_8222CED0(ctx, base);
	// 83287ED4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83287ED8: 38692708  addi r3, r9, 0x2708
	ctx.r[3].s64 = ctx.r[9].s64 + 9992;
	// 83287EDC: 4BA22045  bl 0x82ca9f20
	ctx.lr = 0x83287EE0;
	sub_82CA9F20(ctx, base);
	// 83287EE0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83287EE4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83287EE8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83287EEC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83287EF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83287EF0 size=64
    let mut pc: u32 = 0x83287EF0;
    'dispatch: loop {
        match pc {
            0x83287EF0 => {
    //   block [0x83287EF0..0x83287F30)
	// 83287EF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83287EF4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83287EF8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83287EFC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83287F00: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83287F04: 388BA298  addi r4, r11, -0x5d68
	ctx.r[4].s64 = ctx.r[11].s64 + -23912;
	// 83287F08: 386AE980  addi r3, r10, -0x1680
	ctx.r[3].s64 = ctx.r[10].s64 + -5760;
	// 83287F0C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83287F10: 4AFA4FC1  bl 0x8222ced0
	ctx.lr = 0x83287F14;
	sub_8222CED0(ctx, base);
	// 83287F14: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83287F18: 38692718  addi r3, r9, 0x2718
	ctx.r[3].s64 = ctx.r[9].s64 + 10008;
	// 83287F1C: 4BA22005  bl 0x82ca9f20
	ctx.lr = 0x83287F20;
	sub_82CA9F20(ctx, base);
	// 83287F20: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83287F24: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83287F28: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83287F2C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83287F30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83287F30 size=64
    let mut pc: u32 = 0x83287F30;
    'dispatch: loop {
        match pc {
            0x83287F30 => {
    //   block [0x83287F30..0x83287F70)
	// 83287F30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83287F34: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83287F38: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83287F3C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83287F40: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83287F44: 388BA2AC  addi r4, r11, -0x5d54
	ctx.r[4].s64 = ctx.r[11].s64 + -23892;
	// 83287F48: 386AE984  addi r3, r10, -0x167c
	ctx.r[3].s64 = ctx.r[10].s64 + -5756;
	// 83287F4C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83287F50: 4AFA4F81  bl 0x8222ced0
	ctx.lr = 0x83287F54;
	sub_8222CED0(ctx, base);
	// 83287F54: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83287F58: 38692728  addi r3, r9, 0x2728
	ctx.r[3].s64 = ctx.r[9].s64 + 10024;
	// 83287F5C: 4BA21FC5  bl 0x82ca9f20
	ctx.lr = 0x83287F60;
	sub_82CA9F20(ctx, base);
	// 83287F60: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83287F64: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83287F68: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83287F6C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83287F70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83287F70 size=64
    let mut pc: u32 = 0x83287F70;
    'dispatch: loop {
        match pc {
            0x83287F70 => {
    //   block [0x83287F70..0x83287FB0)
	// 83287F70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83287F74: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83287F78: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83287F7C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83287F80: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83287F84: 388BA2C4  addi r4, r11, -0x5d3c
	ctx.r[4].s64 = ctx.r[11].s64 + -23868;
	// 83287F88: 386AE988  addi r3, r10, -0x1678
	ctx.r[3].s64 = ctx.r[10].s64 + -5752;
	// 83287F8C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83287F90: 4AFA4F41  bl 0x8222ced0
	ctx.lr = 0x83287F94;
	sub_8222CED0(ctx, base);
	// 83287F94: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83287F98: 38692738  addi r3, r9, 0x2738
	ctx.r[3].s64 = ctx.r[9].s64 + 10040;
	// 83287F9C: 4BA21F85  bl 0x82ca9f20
	ctx.lr = 0x83287FA0;
	sub_82CA9F20(ctx, base);
	// 83287FA0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83287FA4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83287FA8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83287FAC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83287FB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83287FB0 size=64
    let mut pc: u32 = 0x83287FB0;
    'dispatch: loop {
        match pc {
            0x83287FB0 => {
    //   block [0x83287FB0..0x83287FF0)
	// 83287FB0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83287FB4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83287FB8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83287FBC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83287FC0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83287FC4: 388BA2D4  addi r4, r11, -0x5d2c
	ctx.r[4].s64 = ctx.r[11].s64 + -23852;
	// 83287FC8: 386AE98C  addi r3, r10, -0x1674
	ctx.r[3].s64 = ctx.r[10].s64 + -5748;
	// 83287FCC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83287FD0: 4AFA4F01  bl 0x8222ced0
	ctx.lr = 0x83287FD4;
	sub_8222CED0(ctx, base);
	// 83287FD4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83287FD8: 38692748  addi r3, r9, 0x2748
	ctx.r[3].s64 = ctx.r[9].s64 + 10056;
	// 83287FDC: 4BA21F45  bl 0x82ca9f20
	ctx.lr = 0x83287FE0;
	sub_82CA9F20(ctx, base);
	// 83287FE0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83287FE4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83287FE8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83287FEC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83287FF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83287FF0 size=64
    let mut pc: u32 = 0x83287FF0;
    'dispatch: loop {
        match pc {
            0x83287FF0 => {
    //   block [0x83287FF0..0x83288030)
	// 83287FF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83287FF4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83287FF8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83287FFC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83288000: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83288004: 388BA1EC  addi r4, r11, -0x5e14
	ctx.r[4].s64 = ctx.r[11].s64 + -24084;
	// 83288008: 386AE990  addi r3, r10, -0x1670
	ctx.r[3].s64 = ctx.r[10].s64 + -5744;
	// 8328800C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83288010: 4AFA4EC1  bl 0x8222ced0
	ctx.lr = 0x83288014;
	sub_8222CED0(ctx, base);
	// 83288014: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83288018: 38692758  addi r3, r9, 0x2758
	ctx.r[3].s64 = ctx.r[9].s64 + 10072;
	// 8328801C: 4BA21F05  bl 0x82ca9f20
	ctx.lr = 0x83288020;
	sub_82CA9F20(ctx, base);
	// 83288020: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83288024: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83288028: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8328802C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83288030(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83288030 size=64
    let mut pc: u32 = 0x83288030;
    'dispatch: loop {
        match pc {
            0x83288030 => {
    //   block [0x83288030..0x83288070)
	// 83288030: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83288034: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83288038: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8328803C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83288040: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83288044: 388BA1F4  addi r4, r11, -0x5e0c
	ctx.r[4].s64 = ctx.r[11].s64 + -24076;
	// 83288048: 386AE994  addi r3, r10, -0x166c
	ctx.r[3].s64 = ctx.r[10].s64 + -5740;
	// 8328804C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83288050: 4AFA4E81  bl 0x8222ced0
	ctx.lr = 0x83288054;
	sub_8222CED0(ctx, base);
	// 83288054: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83288058: 38692768  addi r3, r9, 0x2768
	ctx.r[3].s64 = ctx.r[9].s64 + 10088;
	// 8328805C: 4BA21EC5  bl 0x82ca9f20
	ctx.lr = 0x83288060;
	sub_82CA9F20(ctx, base);
	// 83288060: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83288064: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83288068: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8328806C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83288070(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83288070 size=64
    let mut pc: u32 = 0x83288070;
    'dispatch: loop {
        match pc {
            0x83288070 => {
    //   block [0x83288070..0x832880B0)
	// 83288070: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83288074: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83288078: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8328807C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83288080: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83288084: 388BA208  addi r4, r11, -0x5df8
	ctx.r[4].s64 = ctx.r[11].s64 + -24056;
	// 83288088: 386AE998  addi r3, r10, -0x1668
	ctx.r[3].s64 = ctx.r[10].s64 + -5736;
	// 8328808C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83288090: 4AFA4E41  bl 0x8222ced0
	ctx.lr = 0x83288094;
	sub_8222CED0(ctx, base);
	// 83288094: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83288098: 38692778  addi r3, r9, 0x2778
	ctx.r[3].s64 = ctx.r[9].s64 + 10104;
	// 8328809C: 4BA21E85  bl 0x82ca9f20
	ctx.lr = 0x832880A0;
	sub_82CA9F20(ctx, base);
	// 832880A0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832880A4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832880A8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832880AC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832880B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832880B0 size=64
    let mut pc: u32 = 0x832880B0;
    'dispatch: loop {
        match pc {
            0x832880B0 => {
    //   block [0x832880B0..0x832880F0)
	// 832880B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832880B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832880B8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832880BC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 832880C0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832880C4: 388BA21C  addi r4, r11, -0x5de4
	ctx.r[4].s64 = ctx.r[11].s64 + -24036;
	// 832880C8: 386AE99C  addi r3, r10, -0x1664
	ctx.r[3].s64 = ctx.r[10].s64 + -5732;
	// 832880CC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832880D0: 4AFA4E01  bl 0x8222ced0
	ctx.lr = 0x832880D4;
	sub_8222CED0(ctx, base);
	// 832880D4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 832880D8: 38692788  addi r3, r9, 0x2788
	ctx.r[3].s64 = ctx.r[9].s64 + 10120;
	// 832880DC: 4BA21E45  bl 0x82ca9f20
	ctx.lr = 0x832880E0;
	sub_82CA9F20(ctx, base);
	// 832880E0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832880E4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832880E8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832880EC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832880F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832880F0 size=64
    let mut pc: u32 = 0x832880F0;
    'dispatch: loop {
        match pc {
            0x832880F0 => {
    //   block [0x832880F0..0x83288130)
	// 832880F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832880F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832880F8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832880FC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83288100: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83288104: 388BA234  addi r4, r11, -0x5dcc
	ctx.r[4].s64 = ctx.r[11].s64 + -24012;
	// 83288108: 386AE9A0  addi r3, r10, -0x1660
	ctx.r[3].s64 = ctx.r[10].s64 + -5728;
	// 8328810C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83288110: 4AFA4DC1  bl 0x8222ced0
	ctx.lr = 0x83288114;
	sub_8222CED0(ctx, base);
	// 83288114: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83288118: 38692798  addi r3, r9, 0x2798
	ctx.r[3].s64 = ctx.r[9].s64 + 10136;
	// 8328811C: 4BA21E05  bl 0x82ca9f20
	ctx.lr = 0x83288120;
	sub_82CA9F20(ctx, base);
	// 83288120: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83288124: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83288128: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8328812C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83288130(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83288130 size=64
    let mut pc: u32 = 0x83288130;
    'dispatch: loop {
        match pc {
            0x83288130 => {
    //   block [0x83288130..0x83288170)
	// 83288130: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83288134: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83288138: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8328813C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83288140: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83288144: 388BA24C  addi r4, r11, -0x5db4
	ctx.r[4].s64 = ctx.r[11].s64 + -23988;
	// 83288148: 386AE9A4  addi r3, r10, -0x165c
	ctx.r[3].s64 = ctx.r[10].s64 + -5724;
	// 8328814C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83288150: 4AFA4D81  bl 0x8222ced0
	ctx.lr = 0x83288154;
	sub_8222CED0(ctx, base);
	// 83288154: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83288158: 386927A8  addi r3, r9, 0x27a8
	ctx.r[3].s64 = ctx.r[9].s64 + 10152;
	// 8328815C: 4BA21DC5  bl 0x82ca9f20
	ctx.lr = 0x83288160;
	sub_82CA9F20(ctx, base);
	// 83288160: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83288164: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83288168: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8328816C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83288170(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83288170 size=64
    let mut pc: u32 = 0x83288170;
    'dispatch: loop {
        match pc {
            0x83288170 => {
    //   block [0x83288170..0x832881B0)
	// 83288170: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83288174: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83288178: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8328817C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83288180: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83288184: 388BA140  addi r4, r11, -0x5ec0
	ctx.r[4].s64 = ctx.r[11].s64 + -24256;
	// 83288188: 386AE9A8  addi r3, r10, -0x1658
	ctx.r[3].s64 = ctx.r[10].s64 + -5720;
	// 8328818C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83288190: 4AFA4D41  bl 0x8222ced0
	ctx.lr = 0x83288194;
	sub_8222CED0(ctx, base);
	// 83288194: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83288198: 386927B8  addi r3, r9, 0x27b8
	ctx.r[3].s64 = ctx.r[9].s64 + 10168;
	// 8328819C: 4BA21D85  bl 0x82ca9f20
	ctx.lr = 0x832881A0;
	sub_82CA9F20(ctx, base);
	// 832881A0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832881A4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832881A8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832881AC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832881B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832881B0 size=64
    let mut pc: u32 = 0x832881B0;
    'dispatch: loop {
        match pc {
            0x832881B0 => {
    //   block [0x832881B0..0x832881F0)
	// 832881B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832881B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832881B8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832881BC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 832881C0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832881C4: 388BA148  addi r4, r11, -0x5eb8
	ctx.r[4].s64 = ctx.r[11].s64 + -24248;
	// 832881C8: 386AE9AC  addi r3, r10, -0x1654
	ctx.r[3].s64 = ctx.r[10].s64 + -5716;
	// 832881CC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832881D0: 4AFA4D01  bl 0x8222ced0
	ctx.lr = 0x832881D4;
	sub_8222CED0(ctx, base);
	// 832881D4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 832881D8: 386927C8  addi r3, r9, 0x27c8
	ctx.r[3].s64 = ctx.r[9].s64 + 10184;
	// 832881DC: 4BA21D45  bl 0x82ca9f20
	ctx.lr = 0x832881E0;
	sub_82CA9F20(ctx, base);
	// 832881E0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832881E4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832881E8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832881EC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832881F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832881F0 size=64
    let mut pc: u32 = 0x832881F0;
    'dispatch: loop {
        match pc {
            0x832881F0 => {
    //   block [0x832881F0..0x83288230)
	// 832881F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832881F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832881F8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832881FC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83288200: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83288204: 388BA15C  addi r4, r11, -0x5ea4
	ctx.r[4].s64 = ctx.r[11].s64 + -24228;
	// 83288208: 386AE9B0  addi r3, r10, -0x1650
	ctx.r[3].s64 = ctx.r[10].s64 + -5712;
	// 8328820C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83288210: 4AFA4CC1  bl 0x8222ced0
	ctx.lr = 0x83288214;
	sub_8222CED0(ctx, base);
	// 83288214: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83288218: 386927D8  addi r3, r9, 0x27d8
	ctx.r[3].s64 = ctx.r[9].s64 + 10200;
	// 8328821C: 4BA21D05  bl 0x82ca9f20
	ctx.lr = 0x83288220;
	sub_82CA9F20(ctx, base);
	// 83288220: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83288224: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83288228: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8328822C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83288230(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83288230 size=64
    let mut pc: u32 = 0x83288230;
    'dispatch: loop {
        match pc {
            0x83288230 => {
    //   block [0x83288230..0x83288270)
	// 83288230: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83288234: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83288238: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8328823C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83288240: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83288244: 388BA170  addi r4, r11, -0x5e90
	ctx.r[4].s64 = ctx.r[11].s64 + -24208;
	// 83288248: 386AE9B4  addi r3, r10, -0x164c
	ctx.r[3].s64 = ctx.r[10].s64 + -5708;
	// 8328824C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83288250: 4AFA4C81  bl 0x8222ced0
	ctx.lr = 0x83288254;
	sub_8222CED0(ctx, base);
	// 83288254: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83288258: 386927E8  addi r3, r9, 0x27e8
	ctx.r[3].s64 = ctx.r[9].s64 + 10216;
	// 8328825C: 4BA21CC5  bl 0x82ca9f20
	ctx.lr = 0x83288260;
	sub_82CA9F20(ctx, base);
	// 83288260: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83288264: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83288268: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8328826C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83288270(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83288270 size=64
    let mut pc: u32 = 0x83288270;
    'dispatch: loop {
        match pc {
            0x83288270 => {
    //   block [0x83288270..0x832882B0)
	// 83288270: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83288274: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83288278: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8328827C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83288280: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83288284: 388BA188  addi r4, r11, -0x5e78
	ctx.r[4].s64 = ctx.r[11].s64 + -24184;
	// 83288288: 386AE9B8  addi r3, r10, -0x1648
	ctx.r[3].s64 = ctx.r[10].s64 + -5704;
	// 8328828C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83288290: 4AFA4C41  bl 0x8222ced0
	ctx.lr = 0x83288294;
	sub_8222CED0(ctx, base);
	// 83288294: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83288298: 386927F8  addi r3, r9, 0x27f8
	ctx.r[3].s64 = ctx.r[9].s64 + 10232;
	// 8328829C: 4BA21C85  bl 0x82ca9f20
	ctx.lr = 0x832882A0;
	sub_82CA9F20(ctx, base);
	// 832882A0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832882A4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832882A8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832882AC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832882B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832882B0 size=64
    let mut pc: u32 = 0x832882B0;
    'dispatch: loop {
        match pc {
            0x832882B0 => {
    //   block [0x832882B0..0x832882F0)
	// 832882B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832882B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832882B8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832882BC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 832882C0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832882C4: 388BA1A0  addi r4, r11, -0x5e60
	ctx.r[4].s64 = ctx.r[11].s64 + -24160;
	// 832882C8: 386AE9BC  addi r3, r10, -0x1644
	ctx.r[3].s64 = ctx.r[10].s64 + -5700;
	// 832882CC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832882D0: 4AFA4C01  bl 0x8222ced0
	ctx.lr = 0x832882D4;
	sub_8222CED0(ctx, base);
	// 832882D4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 832882D8: 38692808  addi r3, r9, 0x2808
	ctx.r[3].s64 = ctx.r[9].s64 + 10248;
	// 832882DC: 4BA21C45  bl 0x82ca9f20
	ctx.lr = 0x832882E0;
	sub_82CA9F20(ctx, base);
	// 832882E0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832882E4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832882E8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832882EC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832882F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832882F0 size=64
    let mut pc: u32 = 0x832882F0;
    'dispatch: loop {
        match pc {
            0x832882F0 => {
    //   block [0x832882F0..0x83288330)
	// 832882F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832882F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832882F8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832882FC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83288300: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83288304: 388BA1B8  addi r4, r11, -0x5e48
	ctx.r[4].s64 = ctx.r[11].s64 + -24136;
	// 83288308: 386AE9C0  addi r3, r10, -0x1640
	ctx.r[3].s64 = ctx.r[10].s64 + -5696;
	// 8328830C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83288310: 4AFA4BC1  bl 0x8222ced0
	ctx.lr = 0x83288314;
	sub_8222CED0(ctx, base);
	// 83288314: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83288318: 38692818  addi r3, r9, 0x2818
	ctx.r[3].s64 = ctx.r[9].s64 + 10264;
	// 8328831C: 4BA21C05  bl 0x82ca9f20
	ctx.lr = 0x83288320;
	sub_82CA9F20(ctx, base);
	// 83288320: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83288324: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83288328: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8328832C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83288330(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83288330 size=64
    let mut pc: u32 = 0x83288330;
    'dispatch: loop {
        match pc {
            0x83288330 => {
    //   block [0x83288330..0x83288370)
	// 83288330: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83288334: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83288338: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8328833C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83288340: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83288344: 388BA1C8  addi r4, r11, -0x5e38
	ctx.r[4].s64 = ctx.r[11].s64 + -24120;
	// 83288348: 386AE9C4  addi r3, r10, -0x163c
	ctx.r[3].s64 = ctx.r[10].s64 + -5692;
	// 8328834C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83288350: 4AFA4B81  bl 0x8222ced0
	ctx.lr = 0x83288354;
	sub_8222CED0(ctx, base);
	// 83288354: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83288358: 38692828  addi r3, r9, 0x2828
	ctx.r[3].s64 = ctx.r[9].s64 + 10280;
	// 8328835C: 4BA21BC5  bl 0x82ca9f20
	ctx.lr = 0x83288360;
	sub_82CA9F20(ctx, base);
	// 83288360: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83288364: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83288368: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8328836C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83288370(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83288370 size=64
    let mut pc: u32 = 0x83288370;
    'dispatch: loop {
        match pc {
            0x83288370 => {
    //   block [0x83288370..0x832883B0)
	// 83288370: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83288374: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83288378: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8328837C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83288380: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83288384: 388BA0BC  addi r4, r11, -0x5f44
	ctx.r[4].s64 = ctx.r[11].s64 + -24388;
	// 83288388: 386AE9C8  addi r3, r10, -0x1638
	ctx.r[3].s64 = ctx.r[10].s64 + -5688;
	// 8328838C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83288390: 4AFA4B41  bl 0x8222ced0
	ctx.lr = 0x83288394;
	sub_8222CED0(ctx, base);
	// 83288394: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83288398: 38692878  addi r3, r9, 0x2878
	ctx.r[3].s64 = ctx.r[9].s64 + 10360;
	// 8328839C: 4BA21B85  bl 0x82ca9f20
	ctx.lr = 0x832883A0;
	sub_82CA9F20(ctx, base);
	// 832883A0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832883A4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832883A8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832883AC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832883B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832883B0 size=64
    let mut pc: u32 = 0x832883B0;
    'dispatch: loop {
        match pc {
            0x832883B0 => {
    //   block [0x832883B0..0x832883F0)
	// 832883B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832883B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832883B8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832883BC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 832883C0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832883C4: 388BA0CC  addi r4, r11, -0x5f34
	ctx.r[4].s64 = ctx.r[11].s64 + -24372;
	// 832883C8: 386AE9CC  addi r3, r10, -0x1634
	ctx.r[3].s64 = ctx.r[10].s64 + -5684;
	// 832883CC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832883D0: 4AFA4B01  bl 0x8222ced0
	ctx.lr = 0x832883D4;
	sub_8222CED0(ctx, base);
	// 832883D4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 832883D8: 38692888  addi r3, r9, 0x2888
	ctx.r[3].s64 = ctx.r[9].s64 + 10376;
	// 832883DC: 4BA21B45  bl 0x82ca9f20
	ctx.lr = 0x832883E0;
	sub_82CA9F20(ctx, base);
	// 832883E0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832883E4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832883E8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832883EC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832883F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832883F0 size=64
    let mut pc: u32 = 0x832883F0;
    'dispatch: loop {
        match pc {
            0x832883F0 => {
    //   block [0x832883F0..0x83288430)
	// 832883F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832883F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832883F8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832883FC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83288400: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83288404: 388BA0DC  addi r4, r11, -0x5f24
	ctx.r[4].s64 = ctx.r[11].s64 + -24356;
	// 83288408: 386AE9D0  addi r3, r10, -0x1630
	ctx.r[3].s64 = ctx.r[10].s64 + -5680;
	// 8328840C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83288410: 4AFA4AC1  bl 0x8222ced0
	ctx.lr = 0x83288414;
	sub_8222CED0(ctx, base);
	// 83288414: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83288418: 38692898  addi r3, r9, 0x2898
	ctx.r[3].s64 = ctx.r[9].s64 + 10392;
	// 8328841C: 4BA21B05  bl 0x82ca9f20
	ctx.lr = 0x83288420;
	sub_82CA9F20(ctx, base);
	// 83288420: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83288424: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83288428: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8328842C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83288430(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83288430 size=64
    let mut pc: u32 = 0x83288430;
    'dispatch: loop {
        match pc {
            0x83288430 => {
    //   block [0x83288430..0x83288470)
	// 83288430: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83288434: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83288438: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8328843C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83288440: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83288444: 388BA0F0  addi r4, r11, -0x5f10
	ctx.r[4].s64 = ctx.r[11].s64 + -24336;
	// 83288448: 386AE9D4  addi r3, r10, -0x162c
	ctx.r[3].s64 = ctx.r[10].s64 + -5676;
	// 8328844C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83288450: 4AFA4A81  bl 0x8222ced0
	ctx.lr = 0x83288454;
	sub_8222CED0(ctx, base);
	// 83288454: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83288458: 386928A8  addi r3, r9, 0x28a8
	ctx.r[3].s64 = ctx.r[9].s64 + 10408;
	// 8328845C: 4BA21AC5  bl 0x82ca9f20
	ctx.lr = 0x83288460;
	sub_82CA9F20(ctx, base);
	// 83288460: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83288464: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83288468: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8328846C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83288470(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83288470 size=64
    let mut pc: u32 = 0x83288470;
    'dispatch: loop {
        match pc {
            0x83288470 => {
    //   block [0x83288470..0x832884B0)
	// 83288470: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83288474: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83288478: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8328847C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83288480: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83288484: 388B9FEC  addi r4, r11, -0x6014
	ctx.r[4].s64 = ctx.r[11].s64 + -24596;
	// 83288488: 386AE9D8  addi r3, r10, -0x1628
	ctx.r[3].s64 = ctx.r[10].s64 + -5672;
	// 8328848C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83288490: 4AFA4A41  bl 0x8222ced0
	ctx.lr = 0x83288494;
	sub_8222CED0(ctx, base);
	// 83288494: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83288498: 386928B8  addi r3, r9, 0x28b8
	ctx.r[3].s64 = ctx.r[9].s64 + 10424;
	// 8328849C: 4BA21A85  bl 0x82ca9f20
	ctx.lr = 0x832884A0;
	sub_82CA9F20(ctx, base);
	// 832884A0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832884A4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832884A8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832884AC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832884B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832884B0 size=64
    let mut pc: u32 = 0x832884B0;
    'dispatch: loop {
        match pc {
            0x832884B0 => {
    //   block [0x832884B0..0x832884F0)
	// 832884B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832884B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832884B8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832884BC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 832884C0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832884C4: 388B9F9C  addi r4, r11, -0x6064
	ctx.r[4].s64 = ctx.r[11].s64 + -24676;
	// 832884C8: 386AE9DC  addi r3, r10, -0x1624
	ctx.r[3].s64 = ctx.r[10].s64 + -5668;
	// 832884CC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832884D0: 4AFA4A01  bl 0x8222ced0
	ctx.lr = 0x832884D4;
	sub_8222CED0(ctx, base);
	// 832884D4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 832884D8: 386928C8  addi r3, r9, 0x28c8
	ctx.r[3].s64 = ctx.r[9].s64 + 10440;
	// 832884DC: 4BA21A45  bl 0x82ca9f20
	ctx.lr = 0x832884E0;
	sub_82CA9F20(ctx, base);
	// 832884E0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832884E4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832884E8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832884EC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832884F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832884F0 size=64
    let mut pc: u32 = 0x832884F0;
    'dispatch: loop {
        match pc {
            0x832884F0 => {
    //   block [0x832884F0..0x83288530)
	// 832884F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832884F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832884F8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832884FC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83288500: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83288504: 388B9FA4  addi r4, r11, -0x605c
	ctx.r[4].s64 = ctx.r[11].s64 + -24668;
	// 83288508: 386AE9E0  addi r3, r10, -0x1620
	ctx.r[3].s64 = ctx.r[10].s64 + -5664;
	// 8328850C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83288510: 4AFA49C1  bl 0x8222ced0
	ctx.lr = 0x83288514;
	sub_8222CED0(ctx, base);
	// 83288514: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83288518: 386928D8  addi r3, r9, 0x28d8
	ctx.r[3].s64 = ctx.r[9].s64 + 10456;
	// 8328851C: 4BA21A05  bl 0x82ca9f20
	ctx.lr = 0x83288520;
	sub_82CA9F20(ctx, base);
	// 83288520: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83288524: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83288528: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8328852C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83288530(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83288530 size=64
    let mut pc: u32 = 0x83288530;
    'dispatch: loop {
        match pc {
            0x83288530 => {
    //   block [0x83288530..0x83288570)
	// 83288530: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83288534: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83288538: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8328853C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83288540: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83288544: 388B9FB8  addi r4, r11, -0x6048
	ctx.r[4].s64 = ctx.r[11].s64 + -24648;
	// 83288548: 386AE9E4  addi r3, r10, -0x161c
	ctx.r[3].s64 = ctx.r[10].s64 + -5660;
	// 8328854C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83288550: 4AFA4981  bl 0x8222ced0
	ctx.lr = 0x83288554;
	sub_8222CED0(ctx, base);
	// 83288554: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83288558: 386928E8  addi r3, r9, 0x28e8
	ctx.r[3].s64 = ctx.r[9].s64 + 10472;
	// 8328855C: 4BA219C5  bl 0x82ca9f20
	ctx.lr = 0x83288560;
	sub_82CA9F20(ctx, base);
	// 83288560: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83288564: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83288568: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8328856C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83288570(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83288570 size=64
    let mut pc: u32 = 0x83288570;
    'dispatch: loop {
        match pc {
            0x83288570 => {
    //   block [0x83288570..0x832885B0)
	// 83288570: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83288574: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83288578: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8328857C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83288580: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83288584: 388B9FCC  addi r4, r11, -0x6034
	ctx.r[4].s64 = ctx.r[11].s64 + -24628;
	// 83288588: 386AE9E8  addi r3, r10, -0x1618
	ctx.r[3].s64 = ctx.r[10].s64 + -5656;
	// 8328858C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83288590: 4AFA4941  bl 0x8222ced0
	ctx.lr = 0x83288594;
	sub_8222CED0(ctx, base);
	// 83288594: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83288598: 386928F8  addi r3, r9, 0x28f8
	ctx.r[3].s64 = ctx.r[9].s64 + 10488;
	// 8328859C: 4BA21985  bl 0x82ca9f20
	ctx.lr = 0x832885A0;
	sub_82CA9F20(ctx, base);
	// 832885A0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832885A4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832885A8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832885AC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832885B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832885B0 size=12
    let mut pc: u32 = 0x832885B0;
    'dispatch: loop {
        match pc {
            0x832885B0 => {
    //   block [0x832885B0..0x832885BC)
	// 832885B0: 3D60832B  lis r11, -0x7cd5
	ctx.r[11].s64 = -2094333952;
	// 832885B4: 386B2908  addi r3, r11, 0x2908
	ctx.r[3].s64 = ctx.r[11].s64 + 10504;
	// 832885B8: 4BA21968  b 0x82ca9f20
	sub_82CA9F20(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832885C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832885C0 size=96
    let mut pc: u32 = 0x832885C0;
    'dispatch: loop {
        match pc {
            0x832885C0 => {
    //   block [0x832885C0..0x832885E4)
	// 832885C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832885C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832885C8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832885CC: 3860000C  li r3, 0xc
	ctx.r[3].s64 = 12;
	// 832885D0: 4AF96C89  bl 0x8221f258
	ctx.lr = 0x832885D4;
	sub_8221F258(ctx, base);
	// 832885D4: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 832885D8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 832885DC: 419A0008  beq cr6, 0x832885e4
	if ctx.cr[6].eq {
	pc = 0x832885E4; continue 'dispatch;
	}
	// 832885E0: 916B0000  stw r11, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	pc = 0x832885E4; continue 'dispatch;
            }
            0x832885E4 => {
    //   block [0x832885E4..0x832885F0)
	// 832885E4: 354B0004  addic. r10, r11, 4
	ctx.xer.ca = (ctx.r[11].u32 > (!(4 as u32)));
	ctx.r[10].s64 = ctx.r[11].s64 + 4;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 832885E8: 41820008  beq 0x832885f0
	if ctx.cr[0].eq {
	pc = 0x832885F0; continue 'dispatch;
	}
	// 832885EC: 916A0000  stw r11, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	pc = 0x832885F0; continue 'dispatch;
            }
            0x832885F0 => {
    //   block [0x832885F0..0x83288620)
	// 832885F0: 3D20834A  lis r9, -0x7cb6
	ctx.r[9].s64 = -2092302336;
	// 832885F4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 832885F8: 3909E9F4  addi r8, r9, -0x160c
	ctx.r[8].s64 = ctx.r[9].s64 + -5644;
	// 832885FC: 3CE0832B  lis r7, -0x7cd5
	ctx.r[7].s64 = -2094333952;
	// 83288600: 38672918  addi r3, r7, 0x2918
	ctx.r[3].s64 = ctx.r[7].s64 + 10520;
	// 83288604: 91680004  stw r11, 4(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 83288608: 91480008  stw r10, 8(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 8328860C: 4BA21915  bl 0x82ca9f20
	ctx.lr = 0x83288610;
	sub_82CA9F20(ctx, base);
	// 83288610: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83288614: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83288618: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8328861C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83288620(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83288620 size=12
    let mut pc: u32 = 0x83288620;
    'dispatch: loop {
        match pc {
            0x83288620 => {
    //   block [0x83288620..0x8328862C)
	// 83288620: 3D60832B  lis r11, -0x7cd5
	ctx.r[11].s64 = -2094333952;
	// 83288624: 386B29A0  addi r3, r11, 0x29a0
	ctx.r[3].s64 = ctx.r[11].s64 + 10656;
	// 83288628: 4BA218F8  b 0x82ca9f20
	sub_82CA9F20(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83288630(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83288630 size=64
    let mut pc: u32 = 0x83288630;
    'dispatch: loop {
        match pc {
            0x83288630 => {
    //   block [0x83288630..0x83288670)
	// 83288630: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83288634: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83288638: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8328863C: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 83288640: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83288644: 388B0CA0  addi r4, r11, 0xca0
	ctx.r[4].s64 = ctx.r[11].s64 + 3232;
	// 83288648: 386AEA10  addi r3, r10, -0x15f0
	ctx.r[3].s64 = ctx.r[10].s64 + -5616;
	// 8328864C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83288650: 4AFA4881  bl 0x8222ced0
	ctx.lr = 0x83288654;
	sub_8222CED0(ctx, base);
	// 83288654: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83288658: 386929F8  addi r3, r9, 0x29f8
	ctx.r[3].s64 = ctx.r[9].s64 + 10744;
	// 8328865C: 4BA218C5  bl 0x82ca9f20
	ctx.lr = 0x83288660;
	sub_82CA9F20(ctx, base);
	// 83288660: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83288664: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83288668: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8328866C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83288670(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83288670 size=64
    let mut pc: u32 = 0x83288670;
    'dispatch: loop {
        match pc {
            0x83288670 => {
    //   block [0x83288670..0x832886B0)
	// 83288670: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83288674: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83288678: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8328867C: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 83288680: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83288684: 388B0CA0  addi r4, r11, 0xca0
	ctx.r[4].s64 = ctx.r[11].s64 + 3232;
	// 83288688: 386AEA14  addi r3, r10, -0x15ec
	ctx.r[3].s64 = ctx.r[10].s64 + -5612;
	// 8328868C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83288690: 4AFA4841  bl 0x8222ced0
	ctx.lr = 0x83288694;
	sub_8222CED0(ctx, base);
	// 83288694: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83288698: 38692A08  addi r3, r9, 0x2a08
	ctx.r[3].s64 = ctx.r[9].s64 + 10760;
	// 8328869C: 4BA21885  bl 0x82ca9f20
	ctx.lr = 0x832886A0;
	sub_82CA9F20(ctx, base);
	// 832886A0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832886A4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832886A8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832886AC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832886B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832886B0 size=12
    let mut pc: u32 = 0x832886B0;
    'dispatch: loop {
        match pc {
            0x832886B0 => {
    //   block [0x832886B0..0x832886BC)
	// 832886B0: 3D60832B  lis r11, -0x7cd5
	ctx.r[11].s64 = -2094333952;
	// 832886B4: 386B2A18  addi r3, r11, 0x2a18
	ctx.r[3].s64 = ctx.r[11].s64 + 10776;
	// 832886B8: 4BA21868  b 0x82ca9f20
	sub_82CA9F20(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832886C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832886C0 size=52
    let mut pc: u32 = 0x832886C0;
    'dispatch: loop {
        match pc {
            0x832886C0 => {
    //   block [0x832886C0..0x832886F4)
	// 832886C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832886C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832886C8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832886CC: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832886D0: 386BEA1C  addi r3, r11, -0x15e4
	ctx.r[3].s64 = ctx.r[11].s64 + -5604;
	// 832886D4: 4B0B843D  bl 0x82340b10
	ctx.lr = 0x832886D8;
	sub_82340B10(ctx, base);
	// 832886D8: 3D40832B  lis r10, -0x7cd5
	ctx.r[10].s64 = -2094333952;
	// 832886DC: 386A2A50  addi r3, r10, 0x2a50
	ctx.r[3].s64 = ctx.r[10].s64 + 10832;
	// 832886E0: 4BA21841  bl 0x82ca9f20
	ctx.lr = 0x832886E4;
	sub_82CA9F20(ctx, base);
	// 832886E4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832886E8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832886EC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832886F0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832886F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832886F8 size=52
    let mut pc: u32 = 0x832886F8;
    'dispatch: loop {
        match pc {
            0x832886F8 => {
    //   block [0x832886F8..0x8328872C)
	// 832886F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832886FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83288700: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83288704: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 83288708: 386BEA28  addi r3, r11, -0x15d8
	ctx.r[3].s64 = ctx.r[11].s64 + -5592;
	// 8328870C: 4B0B8405  bl 0x82340b10
	ctx.lr = 0x83288710;
	sub_82340B10(ctx, base);
	// 83288710: 3D40832B  lis r10, -0x7cd5
	ctx.r[10].s64 = -2094333952;
	// 83288714: 386A2A60  addi r3, r10, 0x2a60
	ctx.r[3].s64 = ctx.r[10].s64 + 10848;
	// 83288718: 4BA21809  bl 0x82ca9f20
	ctx.lr = 0x8328871C;
	sub_82CA9F20(ctx, base);
	// 8328871C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83288720: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83288724: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83288728: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83288730(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83288730 size=64
    let mut pc: u32 = 0x83288730;
    'dispatch: loop {
        match pc {
            0x83288730 => {
    //   block [0x83288730..0x83288770)
	// 83288730: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83288734: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83288738: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8328873C: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 83288740: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83288744: 388B0CA0  addi r4, r11, 0xca0
	ctx.r[4].s64 = ctx.r[11].s64 + 3232;
	// 83288748: 386AEA34  addi r3, r10, -0x15cc
	ctx.r[3].s64 = ctx.r[10].s64 + -5580;
	// 8328874C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83288750: 4AFA4781  bl 0x8222ced0
	ctx.lr = 0x83288754;
	sub_8222CED0(ctx, base);
	// 83288754: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83288758: 38692A70  addi r3, r9, 0x2a70
	ctx.r[3].s64 = ctx.r[9].s64 + 10864;
	// 8328875C: 4BA217C5  bl 0x82ca9f20
	ctx.lr = 0x83288760;
	sub_82CA9F20(ctx, base);
	// 83288760: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83288764: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83288768: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8328876C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83288770(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83288770 size=64
    let mut pc: u32 = 0x83288770;
    'dispatch: loop {
        match pc {
            0x83288770 => {
    //   block [0x83288770..0x832887B0)
	// 83288770: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83288774: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83288778: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8328877C: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 83288780: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83288784: 388B0CA0  addi r4, r11, 0xca0
	ctx.r[4].s64 = ctx.r[11].s64 + 3232;
	// 83288788: 386AEA38  addi r3, r10, -0x15c8
	ctx.r[3].s64 = ctx.r[10].s64 + -5576;
	// 8328878C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83288790: 4AFA4741  bl 0x8222ced0
	ctx.lr = 0x83288794;
	sub_8222CED0(ctx, base);
	// 83288794: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83288798: 38692A80  addi r3, r9, 0x2a80
	ctx.r[3].s64 = ctx.r[9].s64 + 10880;
	// 8328879C: 4BA21785  bl 0x82ca9f20
	ctx.lr = 0x832887A0;
	sub_82CA9F20(ctx, base);
	// 832887A0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832887A4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832887A8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832887AC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832887B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x832887B0 size=96
    let mut pc: u32 = 0x832887B0;
    'dispatch: loop {
        match pc {
            0x832887B0 => {
    //   block [0x832887B0..0x83288810)
	// 832887B0: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 832887B4: 3901FFF0  addi r8, r1, -0x10
	ctx.r[8].s64 = ctx.r[1].s64 + -16;
	// 832887B8: 392B92CC  addi r9, r11, -0x6d34
	ctx.r[9].s64 = ctx.r[11].s64 + -27956;
	// 832887BC: 3941FFF4  addi r10, r1, -0xc
	ctx.r[10].s64 = ctx.r[1].s64 + -12;
	// 832887C0: 38E1FFF4  addi r7, r1, -0xc
	ctx.r[7].s64 = ctx.r[1].s64 + -12;
	// 832887C4: C00B92CC  lfs f0, -0x6d34(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-27956 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 832887C8: 38C1FFF0  addi r6, r1, -0x10
	ctx.r[6].s64 = ctx.r[1].s64 + -16;
	// 832887CC: D001FFF0  stfs f0, -0x10(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), tmp.u32 ) };
	// 832887D0: 3CA0834A  lis r5, -0x7cb6
	ctx.r[5].s64 = -2092302336;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83288810(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83288810 size=64
    let mut pc: u32 = 0x83288810;
    'dispatch: loop {
        match pc {
            0x83288810 => {
    //   block [0x83288810..0x83288850)
	// 83288810: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83288814: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83288818: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8328881C: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 83288820: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83288824: 388B0CA0  addi r4, r11, 0xca0
	ctx.r[4].s64 = ctx.r[11].s64 + 3232;
	// 83288828: 386AEA50  addi r3, r10, -0x15b0
	ctx.r[3].s64 = ctx.r[10].s64 + -5552;
	// 8328882C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83288830: 4AFA46A1  bl 0x8222ced0
	ctx.lr = 0x83288834;
	sub_8222CED0(ctx, base);
	// 83288834: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83288838: 38692A90  addi r3, r9, 0x2a90
	ctx.r[3].s64 = ctx.r[9].s64 + 10896;
	// 8328883C: 4BA216E5  bl 0x82ca9f20
	ctx.lr = 0x83288840;
	sub_82CA9F20(ctx, base);
	// 83288840: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83288844: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83288848: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8328884C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83288850(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83288850 size=64
    let mut pc: u32 = 0x83288850;
    'dispatch: loop {
        match pc {
            0x83288850 => {
    //   block [0x83288850..0x83288890)
	// 83288850: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83288854: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83288858: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8328885C: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 83288860: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83288864: 388B0CA0  addi r4, r11, 0xca0
	ctx.r[4].s64 = ctx.r[11].s64 + 3232;
	// 83288868: 386AEA54  addi r3, r10, -0x15ac
	ctx.r[3].s64 = ctx.r[10].s64 + -5548;
	// 8328886C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83288870: 4AFA4661  bl 0x8222ced0
	ctx.lr = 0x83288874;
	sub_8222CED0(ctx, base);
	// 83288874: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83288878: 38692AA0  addi r3, r9, 0x2aa0
	ctx.r[3].s64 = ctx.r[9].s64 + 10912;
	// 8328887C: 4BA216A5  bl 0x82ca9f20
	ctx.lr = 0x83288880;
	sub_82CA9F20(ctx, base);
	// 83288880: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83288884: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83288888: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8328888C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83288890(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83288890 size=1036
    let mut pc: u32 = 0x83288890;
    'dispatch: loop {
        match pc {
            0x83288890 => {
    //   block [0x83288890..0x83288C9C)
	// 83288890: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83288894: 4BA20B79  bl 0x82ca940c
	ctx.lr = 0x83288898;
	sub_82CA93D0(ctx, base);
	// 83288898: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8328889C: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832888A0: 3D40820F  lis r10, -0x7df1
	ctx.r[10].s64 = -2112946176;
	// 832888A4: 3BEBEA58  addi r31, r11, -0x15a8
	ctx.r[31].s64 = ctx.r[11].s64 + -5544;
	// 832888A8: 388A49A8  addi r4, r10, 0x49a8
	ctx.r[4].s64 = ctx.r[10].s64 + 18856;
	// 832888AC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 832888B0: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832888B4: 4AFA461D  bl 0x8222ced0
	ctx.lr = 0x832888B8;
	sub_8222CED0(ctx, base);
	// 832888B8: 39600004  li r11, 4
	ctx.r[11].s64 = 4;
	// 832888BC: 3D20820F  lis r9, -0x7df1
	ctx.r[9].s64 = -2112946176;
	// 832888C0: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 832888C4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832888C8: 38894990  addi r4, r9, 0x4990
	ctx.r[4].s64 = ctx.r[9].s64 + 18832;
	// 832888CC: 387F0008  addi r3, r31, 8
	ctx.r[3].s64 = ctx.r[31].s64 + 8;
	// 832888D0: 4AFA4601  bl 0x8222ced0
	ctx.lr = 0x832888D4;
	sub_8222CED0(ctx, base);
	// 832888D4: 3D00820F  lis r8, -0x7df1
	ctx.r[8].s64 = -2112946176;
	// 832888D8: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832888DC: 38884980  addi r4, r8, 0x4980
	ctx.r[4].s64 = ctx.r[8].s64 + 18816;
	// 832888E0: 387F000C  addi r3, r31, 0xc
	ctx.r[3].s64 = ctx.r[31].s64 + 12;
	// 832888E4: 4AFA45ED  bl 0x8222ced0
	ctx.lr = 0x832888E8;
	sub_8222CED0(ctx, base);
	// 832888E8: 3CE0820F  lis r7, -0x7df1
	ctx.r[7].s64 = -2112946176;
	// 832888EC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832888F0: 38874974  addi r4, r7, 0x4974
	ctx.r[4].s64 = ctx.r[7].s64 + 18804;
	// 832888F4: 387F0010  addi r3, r31, 0x10
	ctx.r[3].s64 = ctx.r[31].s64 + 16;
	// 832888F8: 4AFA45D9  bl 0x8222ced0
	ctx.lr = 0x832888FC;
	sub_8222CED0(ctx, base);
	// 832888FC: 39600004  li r11, 4
	ctx.r[11].s64 = 4;
	// 83288900: 3CC0820F  lis r6, -0x7df1
	ctx.r[6].s64 = -2112946176;
	// 83288904: 917F0014  stw r11, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[11].u32 ) };
	// 83288908: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8328890C: 3BC64970  addi r30, r6, 0x4970
	ctx.r[30].s64 = ctx.r[6].s64 + 18800;
	// 83288910: 387F0018  addi r3, r31, 0x18
	ctx.r[3].s64 = ctx.r[31].s64 + 24;
	// 83288914: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 83288918: 4AFA45B9  bl 0x8222ced0
	ctx.lr = 0x8328891C;
	sub_8222CED0(ctx, base);
	// 8328891C: 3C80820F  lis r4, -0x7df1
	ctx.r[4].s64 = -2112946176;
	// 83288920: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83288924: 3884495C  addi r4, r4, 0x495c
	ctx.r[4].s64 = ctx.r[4].s64 + 18780;
	// 83288928: 387F001C  addi r3, r31, 0x1c
	ctx.r[3].s64 = ctx.r[31].s64 + 28;
	// 8328892C: 4AFA45A5  bl 0x8222ced0
	ctx.lr = 0x83288930;
	sub_8222CED0(ctx, base);
	// 83288930: 3C60820F  lis r3, -0x7df1
	ctx.r[3].s64 = -2112946176;
	// 83288934: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83288938: 3883494C  addi r4, r3, 0x494c
	ctx.r[4].s64 = ctx.r[3].s64 + 18764;
	// 8328893C: 387F0020  addi r3, r31, 0x20
	ctx.r[3].s64 = ctx.r[31].s64 + 32;
	// 83288940: 4AFA4591  bl 0x8222ced0
	ctx.lr = 0x83288944;
	sub_8222CED0(ctx, base);
	// 83288944: 39600004  li r11, 4
	ctx.r[11].s64 = 4;
	// 83288948: 3D40820F  lis r10, -0x7df1
	ctx.r[10].s64 = -2112946176;
	// 8328894C: 917F0024  stw r11, 0x24(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(36 as u32), ctx.r[11].u32 ) };
	// 83288950: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83288954: 3BAA4948  addi r29, r10, 0x4948
	ctx.r[29].s64 = ctx.r[10].s64 + 18760;
	// 83288958: 387F0028  addi r3, r31, 0x28
	ctx.r[3].s64 = ctx.r[31].s64 + 40;
	// 8328895C: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 83288960: 4AFA4571  bl 0x8222ced0
	ctx.lr = 0x83288964;
	sub_8222CED0(ctx, base);
	// 83288964: 3D20820F  lis r9, -0x7df1
	ctx.r[9].s64 = -2112946176;
	// 83288968: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8328896C: 38894938  addi r4, r9, 0x4938
	ctx.r[4].s64 = ctx.r[9].s64 + 18744;
	// 83288970: 387F002C  addi r3, r31, 0x2c
	ctx.r[3].s64 = ctx.r[31].s64 + 44;
	// 83288974: 4AFA455D  bl 0x8222ced0
	ctx.lr = 0x83288978;
	sub_8222CED0(ctx, base);
	// 83288978: 3D00820F  lis r8, -0x7df1
	ctx.r[8].s64 = -2112946176;
	// 8328897C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83288980: 3888492C  addi r4, r8, 0x492c
	ctx.r[4].s64 = ctx.r[8].s64 + 18732;
	// 83288984: 387F0030  addi r3, r31, 0x30
	ctx.r[3].s64 = ctx.r[31].s64 + 48;
	// 83288988: 4AFA4549  bl 0x8222ced0
	ctx.lr = 0x8328898C;
	sub_8222CED0(ctx, base);
	// 8328898C: 39600004  li r11, 4
	ctx.r[11].s64 = 4;
	// 83288990: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 83288994: 917F0034  stw r11, 0x34(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(52 as u32), ctx.r[11].u32 ) };
	// 83288998: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8328899C: 387F0038  addi r3, r31, 0x38
	ctx.r[3].s64 = ctx.r[31].s64 + 56;
	// 832889A0: 4AFA4531  bl 0x8222ced0
	ctx.lr = 0x832889A4;
	sub_8222CED0(ctx, base);
	// 832889A4: 3CE0820F  lis r7, -0x7df1
	ctx.r[7].s64 = -2112946176;
	// 832889A8: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832889AC: 38874920  addi r4, r7, 0x4920
	ctx.r[4].s64 = ctx.r[7].s64 + 18720;
	// 832889B0: 387F003C  addi r3, r31, 0x3c
	ctx.r[3].s64 = ctx.r[31].s64 + 60;
	// 832889B4: 4AFA451D  bl 0x8222ced0
	ctx.lr = 0x832889B8;
	sub_8222CED0(ctx, base);
	// 832889B8: 3CC0820F  lis r6, -0x7df1
	ctx.r[6].s64 = -2112946176;
	// 832889BC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832889C0: 3886490C  addi r4, r6, 0x490c
	ctx.r[4].s64 = ctx.r[6].s64 + 18700;
	// 832889C4: 387F0040  addi r3, r31, 0x40
	ctx.r[3].s64 = ctx.r[31].s64 + 64;
	// 832889C8: 4AFA4509  bl 0x8222ced0
	ctx.lr = 0x832889CC;
	sub_8222CED0(ctx, base);
	// 832889CC: 39600004  li r11, 4
	ctx.r[11].s64 = 4;
	// 832889D0: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 832889D4: 917F0044  stw r11, 0x44(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(68 as u32), ctx.r[11].u32 ) };
	// 832889D8: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832889DC: 387F0048  addi r3, r31, 0x48
	ctx.r[3].s64 = ctx.r[31].s64 + 72;
	// 832889E0: 4AFA44F1  bl 0x8222ced0
	ctx.lr = 0x832889E4;
	sub_8222CED0(ctx, base);
	// 832889E4: 3C80820F  lis r4, -0x7df1
	ctx.r[4].s64 = -2112946176;
	// 832889E8: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832889EC: 388448FC  addi r4, r4, 0x48fc
	ctx.r[4].s64 = ctx.r[4].s64 + 18684;
	// 832889F0: 387F004C  addi r3, r31, 0x4c
	ctx.r[3].s64 = ctx.r[31].s64 + 76;
	// 832889F4: 4AFA44DD  bl 0x8222ced0
	ctx.lr = 0x832889F8;
	sub_8222CED0(ctx, base);
	// 832889F8: 3C60820F  lis r3, -0x7df1
	ctx.r[3].s64 = -2112946176;
	// 832889FC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83288A00: 388348E8  addi r4, r3, 0x48e8
	ctx.r[4].s64 = ctx.r[3].s64 + 18664;
	// 83288A04: 387F0050  addi r3, r31, 0x50
	ctx.r[3].s64 = ctx.r[31].s64 + 80;
	// 83288A08: 4AFA44C9  bl 0x8222ced0
	ctx.lr = 0x83288A0C;
	sub_8222CED0(ctx, base);
	// 83288A0C: 39600004  li r11, 4
	ctx.r[11].s64 = 4;
	// 83288A10: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 83288A14: 917F0054  stw r11, 0x54(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 83288A18: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83288A1C: 387F0058  addi r3, r31, 0x58
	ctx.r[3].s64 = ctx.r[31].s64 + 88;
	// 83288A20: 4AFA44B1  bl 0x8222ced0
	ctx.lr = 0x83288A24;
	sub_8222CED0(ctx, base);
	// 83288A24: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 83288A28: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83288A2C: 388B48D8  addi r4, r11, 0x48d8
	ctx.r[4].s64 = ctx.r[11].s64 + 18648;
	// 83288A30: 387F005C  addi r3, r31, 0x5c
	ctx.r[3].s64 = ctx.r[31].s64 + 92;
	// 83288A34: 4AFA449D  bl 0x8222ced0
	ctx.lr = 0x83288A38;
	sub_8222CED0(ctx, base);
	// 83288A38: 3D40820F  lis r10, -0x7df1
	ctx.r[10].s64 = -2112946176;
	// 83288A3C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83288A40: 388A48C8  addi r4, r10, 0x48c8
	ctx.r[4].s64 = ctx.r[10].s64 + 18632;
	// 83288A44: 387F0060  addi r3, r31, 0x60
	ctx.r[3].s64 = ctx.r[31].s64 + 96;
	// 83288A48: 4AFA4489  bl 0x8222ced0
	ctx.lr = 0x83288A4C;
	sub_8222CED0(ctx, base);
	// 83288A4C: 39600004  li r11, 4
	ctx.r[11].s64 = 4;
	// 83288A50: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 83288A54: 917F0064  stw r11, 0x64(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 83288A58: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83288A5C: 387F0068  addi r3, r31, 0x68
	ctx.r[3].s64 = ctx.r[31].s64 + 104;
	// 83288A60: 4AFA4471  bl 0x8222ced0
	ctx.lr = 0x83288A64;
	sub_8222CED0(ctx, base);
	// 83288A64: 3D20820F  lis r9, -0x7df1
	ctx.r[9].s64 = -2112946176;
	// 83288A68: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83288A6C: 388948BC  addi r4, r9, 0x48bc
	ctx.r[4].s64 = ctx.r[9].s64 + 18620;
	// 83288A70: 387F006C  addi r3, r31, 0x6c
	ctx.r[3].s64 = ctx.r[31].s64 + 108;
	// 83288A74: 4AFA445D  bl 0x8222ced0
	ctx.lr = 0x83288A78;
	sub_8222CED0(ctx, base);
	// 83288A78: 3D00820F  lis r8, -0x7df1
	ctx.r[8].s64 = -2112946176;
	// 83288A7C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83288A80: 388848A4  addi r4, r8, 0x48a4
	ctx.r[4].s64 = ctx.r[8].s64 + 18596;
	// 83288A84: 387F0070  addi r3, r31, 0x70
	ctx.r[3].s64 = ctx.r[31].s64 + 112;
	// 83288A88: 4AFA4449  bl 0x8222ced0
	ctx.lr = 0x83288A8C;
	sub_8222CED0(ctx, base);
	// 83288A8C: 39600004  li r11, 4
	ctx.r[11].s64 = 4;
	// 83288A90: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83288A94: 917F0074  stw r11, 0x74(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 83288A98: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 83288A9C: 387F0078  addi r3, r31, 0x78
	ctx.r[3].s64 = ctx.r[31].s64 + 120;
	// 83288AA0: 4AFA4431  bl 0x8222ced0
	ctx.lr = 0x83288AA4;
	sub_8222CED0(ctx, base);
	// 83288AA4: 3CE0820F  lis r7, -0x7df1
	ctx.r[7].s64 = -2112946176;
	// 83288AA8: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83288AAC: 38874890  addi r4, r7, 0x4890
	ctx.r[4].s64 = ctx.r[7].s64 + 18576;
	// 83288AB0: 387F007C  addi r3, r31, 0x7c
	ctx.r[3].s64 = ctx.r[31].s64 + 124;
	// 83288AB4: 4AFA441D  bl 0x8222ced0
	ctx.lr = 0x83288AB8;
	sub_8222CED0(ctx, base);
	// 83288AB8: 3CC0820F  lis r6, -0x7df1
	ctx.r[6].s64 = -2112946176;
	// 83288ABC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83288AC0: 38864874  addi r4, r6, 0x4874
	ctx.r[4].s64 = ctx.r[6].s64 + 18548;
	// 83288AC4: 387F0080  addi r3, r31, 0x80
	ctx.r[3].s64 = ctx.r[31].s64 + 128;
	// 83288AC8: 4AFA4409  bl 0x8222ced0
	ctx.lr = 0x83288ACC;
	sub_8222CED0(ctx, base);
	// 83288ACC: 39600004  li r11, 4
	ctx.r[11].s64 = 4;
	// 83288AD0: 3C80820F  lis r4, -0x7df1
	ctx.r[4].s64 = -2112946176;
	// 83288AD4: 917F0084  stw r11, 0x84(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(132 as u32), ctx.r[11].u32 ) };
	// 83288AD8: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83288ADC: 38844864  addi r4, r4, 0x4864
	ctx.r[4].s64 = ctx.r[4].s64 + 18532;
	// 83288AE0: 387F0088  addi r3, r31, 0x88
	ctx.r[3].s64 = ctx.r[31].s64 + 136;
	// 83288AE4: 4AFA43ED  bl 0x8222ced0
	ctx.lr = 0x83288AE8;
	sub_8222CED0(ctx, base);
	// 83288AE8: 3C60820F  lis r3, -0x7df1
	ctx.r[3].s64 = -2112946176;
	// 83288AEC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83288AF0: 3BC34858  addi r30, r3, 0x4858
	ctx.r[30].s64 = ctx.r[3].s64 + 18520;
	// 83288AF4: 387F008C  addi r3, r31, 0x8c
	ctx.r[3].s64 = ctx.r[31].s64 + 140;
	// 83288AF8: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 83288AFC: 4AFA43D5  bl 0x8222ced0
	ctx.lr = 0x83288B00;
	sub_8222CED0(ctx, base);
	// 83288B00: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 83288B04: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83288B08: 388B483C  addi r4, r11, 0x483c
	ctx.r[4].s64 = ctx.r[11].s64 + 18492;
	// 83288B0C: 387F0090  addi r3, r31, 0x90
	ctx.r[3].s64 = ctx.r[31].s64 + 144;
	// 83288B10: 4AFA43C1  bl 0x8222ced0
	ctx.lr = 0x83288B14;
	sub_8222CED0(ctx, base);
	// 83288B14: 39600004  li r11, 4
	ctx.r[11].s64 = 4;
	// 83288B18: 3D40820F  lis r10, -0x7df1
	ctx.r[10].s64 = -2112946176;
	// 83288B1C: 917F0094  stw r11, 0x94(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(148 as u32), ctx.r[11].u32 ) };
	// 83288B20: 388A482C  addi r4, r10, 0x482c
	ctx.r[4].s64 = ctx.r[10].s64 + 18476;
	// 83288B24: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83288B28: 387F0098  addi r3, r31, 0x98
	ctx.r[3].s64 = ctx.r[31].s64 + 152;
	// 83288B2C: 4AFA43A5  bl 0x8222ced0
	ctx.lr = 0x83288B30;
	sub_8222CED0(ctx, base);
	// 83288B30: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 83288B34: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83288B38: 387F009C  addi r3, r31, 0x9c
	ctx.r[3].s64 = ctx.r[31].s64 + 156;
	// 83288B3C: 4AFA4395  bl 0x8222ced0
	ctx.lr = 0x83288B40;
	sub_8222CED0(ctx, base);
	// 83288B40: 3D20820F  lis r9, -0x7df1
	ctx.r[9].s64 = -2112946176;
	// 83288B44: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83288B48: 38894810  addi r4, r9, 0x4810
	ctx.r[4].s64 = ctx.r[9].s64 + 18448;
	// 83288B4C: 387F00A0  addi r3, r31, 0xa0
	ctx.r[3].s64 = ctx.r[31].s64 + 160;
	// 83288B50: 4AFA4381  bl 0x8222ced0
	ctx.lr = 0x83288B54;
	sub_8222CED0(ctx, base);
	// 83288B54: 39600004  li r11, 4
	ctx.r[11].s64 = 4;
	// 83288B58: 3D00820F  lis r8, -0x7df1
	ctx.r[8].s64 = -2112946176;
	// 83288B5C: 917F00A4  stw r11, 0xa4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(164 as u32), ctx.r[11].u32 ) };
	// 83288B60: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83288B64: 38884800  addi r4, r8, 0x4800
	ctx.r[4].s64 = ctx.r[8].s64 + 18432;
	// 83288B68: 387F00A8  addi r3, r31, 0xa8
	ctx.r[3].s64 = ctx.r[31].s64 + 168;
	// 83288B6C: 4AFA4365  bl 0x8222ced0
	ctx.lr = 0x83288B70;
	sub_8222CED0(ctx, base);
	// 83288B70: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 83288B74: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83288B78: 387F00AC  addi r3, r31, 0xac
	ctx.r[3].s64 = ctx.r[31].s64 + 172;
	// 83288B7C: 4AFA4355  bl 0x8222ced0
	ctx.lr = 0x83288B80;
	sub_8222CED0(ctx, base);
	// 83288B80: 3CE0820F  lis r7, -0x7df1
	ctx.r[7].s64 = -2112946176;
	// 83288B84: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83288B88: 388747E4  addi r4, r7, 0x47e4
	ctx.r[4].s64 = ctx.r[7].s64 + 18404;
	// 83288B8C: 387F00B0  addi r3, r31, 0xb0
	ctx.r[3].s64 = ctx.r[31].s64 + 176;
	// 83288B90: 4AFA4341  bl 0x8222ced0
	ctx.lr = 0x83288B94;
	sub_8222CED0(ctx, base);
	// 83288B94: 39600004  li r11, 4
	ctx.r[11].s64 = 4;
	// 83288B98: 3CC0820F  lis r6, -0x7df1
	ctx.r[6].s64 = -2112946176;
	// 83288B9C: 917F00B4  stw r11, 0xb4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(180 as u32), ctx.r[11].u32 ) };
	// 83288BA0: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83288BA4: 388647D4  addi r4, r6, 0x47d4
	ctx.r[4].s64 = ctx.r[6].s64 + 18388;
	// 83288BA8: 387F00B8  addi r3, r31, 0xb8
	ctx.r[3].s64 = ctx.r[31].s64 + 184;
	// 83288BAC: 4AFA4325  bl 0x8222ced0
	ctx.lr = 0x83288BB0;
	sub_8222CED0(ctx, base);
	// 83288BB0: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 83288BB4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83288BB8: 387F00BC  addi r3, r31, 0xbc
	ctx.r[3].s64 = ctx.r[31].s64 + 188;
	// 83288BBC: 4AFA4315  bl 0x8222ced0
	ctx.lr = 0x83288BC0;
	sub_8222CED0(ctx, base);
	// 83288BC0: 3C80820F  lis r4, -0x7df1
	ctx.r[4].s64 = -2112946176;
	// 83288BC4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83288BC8: 388447C0  addi r4, r4, 0x47c0
	ctx.r[4].s64 = ctx.r[4].s64 + 18368;
	// 83288BCC: 387F00C0  addi r3, r31, 0xc0
	ctx.r[3].s64 = ctx.r[31].s64 + 192;
	// 83288BD0: 4AFA4301  bl 0x8222ced0
	ctx.lr = 0x83288BD4;
	sub_8222CED0(ctx, base);
	// 83288BD4: 39600004  li r11, 4
	ctx.r[11].s64 = 4;
	// 83288BD8: 3C60820F  lis r3, -0x7df1
	ctx.r[3].s64 = -2112946176;
	// 83288BDC: 917F00C4  stw r11, 0xc4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(196 as u32), ctx.r[11].u32 ) };
	// 83288BE0: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83288BE4: 3BC347B8  addi r30, r3, 0x47b8
	ctx.r[30].s64 = ctx.r[3].s64 + 18360;
	// 83288BE8: 387F00C8  addi r3, r31, 0xc8
	ctx.r[3].s64 = ctx.r[31].s64 + 200;
	// 83288BEC: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 83288BF0: 4AFA42E1  bl 0x8222ced0
	ctx.lr = 0x83288BF4;
	sub_8222CED0(ctx, base);
	// 83288BF4: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 83288BF8: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83288BFC: 388B47A8  addi r4, r11, 0x47a8
	ctx.r[4].s64 = ctx.r[11].s64 + 18344;
	// 83288C00: 387F00CC  addi r3, r31, 0xcc
	ctx.r[3].s64 = ctx.r[31].s64 + 204;
	// 83288C04: 4AFA42CD  bl 0x8222ced0
	ctx.lr = 0x83288C08;
	sub_8222CED0(ctx, base);
	// 83288C08: 3D40820F  lis r10, -0x7df1
	ctx.r[10].s64 = -2112946176;
	// 83288C0C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83288C10: 388A4794  addi r4, r10, 0x4794
	ctx.r[4].s64 = ctx.r[10].s64 + 18324;
	// 83288C14: 387F00D0  addi r3, r31, 0xd0
	ctx.r[3].s64 = ctx.r[31].s64 + 208;
	// 83288C18: 4AFA42B9  bl 0x8222ced0
	ctx.lr = 0x83288C1C;
	sub_8222CED0(ctx, base);
	// 83288C1C: 39600004  li r11, 4
	ctx.r[11].s64 = 4;
	// 83288C20: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 83288C24: 917F00D4  stw r11, 0xd4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(212 as u32), ctx.r[11].u32 ) };
	// 83288C28: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83288C2C: 387F00D8  addi r3, r31, 0xd8
	ctx.r[3].s64 = ctx.r[31].s64 + 216;
	// 83288C30: 4AFA42A1  bl 0x8222ced0
	ctx.lr = 0x83288C34;
	sub_8222CED0(ctx, base);
	// 83288C34: 3D20820F  lis r9, -0x7df1
	ctx.r[9].s64 = -2112946176;
	// 83288C38: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83288C3C: 38894784  addi r4, r9, 0x4784
	ctx.r[4].s64 = ctx.r[9].s64 + 18308;
	// 83288C40: 387F00DC  addi r3, r31, 0xdc
	ctx.r[3].s64 = ctx.r[31].s64 + 220;
	// 83288C44: 4AFA428D  bl 0x8222ced0
	ctx.lr = 0x83288C48;
	sub_8222CED0(ctx, base);
	// 83288C48: 3D00820F  lis r8, -0x7df1
	ctx.r[8].s64 = -2112946176;
	// 83288C4C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83288C50: 38884770  addi r4, r8, 0x4770
	ctx.r[4].s64 = ctx.r[8].s64 + 18288;
	// 83288C54: 387F00E0  addi r3, r31, 0xe0
	ctx.r[3].s64 = ctx.r[31].s64 + 224;
	// 83288C58: 4AFA4279  bl 0x8222ced0
	ctx.lr = 0x83288C5C;
	sub_8222CED0(ctx, base);
	// 83288C5C: 39600004  li r11, 4
	ctx.r[11].s64 = 4;
	// 83288C60: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 83288C64: 917F00E4  stw r11, 0xe4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(228 as u32), ctx.r[11].u32 ) };
	// 83288C68: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83288C6C: 387F00E8  addi r3, r31, 0xe8
	ctx.r[3].s64 = ctx.r[31].s64 + 232;
	// 83288C70: 4AFA4261  bl 0x8222ced0
	ctx.lr = 0x83288C74;
	sub_8222CED0(ctx, base);
	// 83288C74: 3CE0820F  lis r7, -0x7df1
	ctx.r[7].s64 = -2112946176;
	// 83288C78: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83288C7C: 38874760  addi r4, r7, 0x4760
	ctx.r[4].s64 = ctx.r[7].s64 + 18272;
	// 83288C80: 387F00EC  addi r3, r31, 0xec
	ctx.r[3].s64 = ctx.r[31].s64 + 236;
	// 83288C84: 4AFA424D  bl 0x8222ced0
	ctx.lr = 0x83288C88;
	sub_8222CED0(ctx, base);
	// 83288C88: 3CC0832B  lis r6, -0x7cd5
	ctx.r[6].s64 = -2094333952;
	// 83288C8C: 38662AB0  addi r3, r6, 0x2ab0
	ctx.r[3].s64 = ctx.r[6].s64 + 10928;
	// 83288C90: 4BA21291  bl 0x82ca9f20
	ctx.lr = 0x83288C94;
	sub_82CA9F20(ctx, base);
	// 83288C94: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83288C98: 4BA207C4  b 0x82ca945c
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83288CA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83288CA0 size=12480
    let mut pc: u32 = 0x83288CA0;
    'dispatch: loop {
        match pc {
            0x83288CA0 => {
    //   block [0x83288CA0..0x8328BD60)
	// 83288CA0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83288CA4: 4BA2072D  bl 0x82ca93d0
	ctx.lr = 0x83288CA8;
	sub_82CA93D0(ctx, base);
	// 83288CA8: 9421FF10  stwu r1, -0xf0(r1)
	ea = ctx.r[1].u32.wrapping_add(-240 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83288CAC: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 83288CB0: 3D40820F  lis r10, -0x7df1
	ctx.r[10].s64 = -2112946176;
	// 83288CB4: 3BEBEB48  addi r31, r11, -0x14b8
	ctx.r[31].s64 = ctx.r[11].s64 + -5304;
	// 83288CB8: 388A6914  addi r4, r10, 0x6914
	ctx.r[4].s64 = ctx.r[10].s64 + 26900;
	// 83288CBC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83288CC0: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83288CC4: 4AFA420D  bl 0x8222ced0
	ctx.lr = 0x83288CC8;
	sub_8222CED0(ctx, base);
	// 83288CC8: 39600002  li r11, 2
	ctx.r[11].s64 = 2;
	// 83288CCC: 3D20820F  lis r9, -0x7df1
	ctx.r[9].s64 = -2112946176;
	// 83288CD0: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 83288CD4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83288CD8: 3BC96908  addi r30, r9, 0x6908
	ctx.r[30].s64 = ctx.r[9].s64 + 26888;
	// 83288CDC: 387F0008  addi r3, r31, 8
	ctx.r[3].s64 = ctx.r[31].s64 + 8;
	// 83288CE0: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 83288CE4: 4AFA41ED  bl 0x8222ced0
	ctx.lr = 0x83288CE8;
	sub_8222CED0(ctx, base);
	// 83288CE8: 3D00820F  lis r8, -0x7df1
	ctx.r[8].s64 = -2112946176;
	// 83288CEC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83288CF0: 388868F8  addi r4, r8, 0x68f8
	ctx.r[4].s64 = ctx.r[8].s64 + 26872;
	// 83288CF4: 387F000C  addi r3, r31, 0xc
	ctx.r[3].s64 = ctx.r[31].s64 + 12;
	// 83288CF8: 4AFA41D9  bl 0x8222ced0
	ctx.lr = 0x83288CFC;
	sub_8222CED0(ctx, base);
	// 83288CFC: 3CE0820F  lis r7, -0x7df1
	ctx.r[7].s64 = -2112946176;
	// 83288D00: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83288D04: 388768E4  addi r4, r7, 0x68e4
	ctx.r[4].s64 = ctx.r[7].s64 + 26852;
	// 83288D08: 387F0010  addi r3, r31, 0x10
	ctx.r[3].s64 = ctx.r[31].s64 + 16;
	// 83288D0C: 4AFA41C5  bl 0x8222ced0
	ctx.lr = 0x83288D10;
	sub_8222CED0(ctx, base);
	// 83288D10: 39600003  li r11, 3
	ctx.r[11].s64 = 3;
	// 83288D14: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 83288D18: 917F0014  stw r11, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[11].u32 ) };
	// 83288D1C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83288D20: 387F0018  addi r3, r31, 0x18
	ctx.r[3].s64 = ctx.r[31].s64 + 24;
	// 83288D24: 4AFA41AD  bl 0x8222ced0
	ctx.lr = 0x83288D28;
	sub_8222CED0(ctx, base);
	// 83288D28: 3CC0820F  lis r6, -0x7df1
	ctx.r[6].s64 = -2112946176;
	// 83288D2C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83288D30: 388668D0  addi r4, r6, 0x68d0
	ctx.r[4].s64 = ctx.r[6].s64 + 26832;
	// 83288D34: 387F001C  addi r3, r31, 0x1c
	ctx.r[3].s64 = ctx.r[31].s64 + 28;
	// 83288D38: 4AFA4199  bl 0x8222ced0
	ctx.lr = 0x83288D3C;
	sub_8222CED0(ctx, base);
	// 83288D3C: 3C80820F  lis r4, -0x7df1
	ctx.r[4].s64 = -2112946176;
	// 83288D40: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83288D44: 388468C0  addi r4, r4, 0x68c0
	ctx.r[4].s64 = ctx.r[4].s64 + 26816;
	// 83288D48: 387F0020  addi r3, r31, 0x20
	ctx.r[3].s64 = ctx.r[31].s64 + 32;
	// 83288D4C: 4AFA4185  bl 0x8222ced0
	ctx.lr = 0x83288D50;
	sub_8222CED0(ctx, base);
	// 83288D50: 39600002  li r11, 2
	ctx.r[11].s64 = 2;
	// 83288D54: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 83288D58: 917F0024  stw r11, 0x24(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(36 as u32), ctx.r[11].u32 ) };
	// 83288D5C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83288D60: 387F0028  addi r3, r31, 0x28
	ctx.r[3].s64 = ctx.r[31].s64 + 40;
	// 83288D64: 4AFA416D  bl 0x8222ced0
	ctx.lr = 0x83288D68;
	sub_8222CED0(ctx, base);
	// 83288D68: 3C60820F  lis r3, -0x7df1
	ctx.r[3].s64 = -2112946176;
	// 83288D6C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83288D70: 388368B0  addi r4, r3, 0x68b0
	ctx.r[4].s64 = ctx.r[3].s64 + 26800;
	// 83288D74: 387F002C  addi r3, r31, 0x2c
	ctx.r[3].s64 = ctx.r[31].s64 + 44;
	// 83288D78: 4AFA4159  bl 0x8222ced0
	ctx.lr = 0x83288D7C;
	sub_8222CED0(ctx, base);
	// 83288D7C: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 83288D80: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83288D84: 388B68A0  addi r4, r11, 0x68a0
	ctx.r[4].s64 = ctx.r[11].s64 + 26784;
	// 83288D88: 387F0030  addi r3, r31, 0x30
	ctx.r[3].s64 = ctx.r[31].s64 + 48;
	// 83288D8C: 4AFA4145  bl 0x8222ced0
	ctx.lr = 0x83288D90;
	sub_8222CED0(ctx, base);
	// 83288D90: 39600002  li r11, 2
	ctx.r[11].s64 = 2;
	// 83288D94: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 83288D98: 917F0034  stw r11, 0x34(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(52 as u32), ctx.r[11].u32 ) };
	// 83288D9C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83288DA0: 387F0038  addi r3, r31, 0x38
	ctx.r[3].s64 = ctx.r[31].s64 + 56;
	// 83288DA4: 4AFA412D  bl 0x8222ced0
	ctx.lr = 0x83288DA8;
	sub_8222CED0(ctx, base);
	// 83288DA8: 3D40820F  lis r10, -0x7df1
	ctx.r[10].s64 = -2112946176;
	// 83288DAC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83288DB0: 388A6890  addi r4, r10, 0x6890
	ctx.r[4].s64 = ctx.r[10].s64 + 26768;
	// 83288DB4: 387F003C  addi r3, r31, 0x3c
	ctx.r[3].s64 = ctx.r[31].s64 + 60;
	// 83288DB8: 4AFA4119  bl 0x8222ced0
	ctx.lr = 0x83288DBC;
	sub_8222CED0(ctx, base);
	// 83288DBC: 3D20820F  lis r9, -0x7df1
	ctx.r[9].s64 = -2112946176;
	// 83288DC0: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83288DC4: 3889686C  addi r4, r9, 0x686c
	ctx.r[4].s64 = ctx.r[9].s64 + 26732;
	// 83288DC8: 387F0040  addi r3, r31, 0x40
	ctx.r[3].s64 = ctx.r[31].s64 + 64;
	// 83288DCC: 4AFA4105  bl 0x8222ced0
	ctx.lr = 0x83288DD0;
	sub_8222CED0(ctx, base);
	// 83288DD0: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 83288DD4: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 83288DD8: 917F0044  stw r11, 0x44(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(68 as u32), ctx.r[11].u32 ) };
	// 83288DDC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83288DE0: 387F0048  addi r3, r31, 0x48
	ctx.r[3].s64 = ctx.r[31].s64 + 72;
	// 83288DE4: 4AFA40ED  bl 0x8222ced0
	ctx.lr = 0x83288DE8;
	sub_8222CED0(ctx, base);
	// 83288DE8: 3D00820F  lis r8, -0x7df1
	ctx.r[8].s64 = -2112946176;
	// 83288DEC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83288DF0: 3888684C  addi r4, r8, 0x684c
	ctx.r[4].s64 = ctx.r[8].s64 + 26700;
	// 83288DF4: 387F004C  addi r3, r31, 0x4c
	ctx.r[3].s64 = ctx.r[31].s64 + 76;
	// 83288DF8: 4AFA40D9  bl 0x8222ced0
	ctx.lr = 0x83288DFC;
	sub_8222CED0(ctx, base);
	// 83288DFC: 3CE0820F  lis r7, -0x7df1
	ctx.r[7].s64 = -2112946176;
	// 83288E00: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83288E04: 38876834  addi r4, r7, 0x6834
	ctx.r[4].s64 = ctx.r[7].s64 + 26676;
	// 83288E08: 387F0050  addi r3, r31, 0x50
	ctx.r[3].s64 = ctx.r[31].s64 + 80;
	// 83288E0C: 4AFA40C5  bl 0x8222ced0
	ctx.lr = 0x83288E10;
	sub_8222CED0(ctx, base);
	// 83288E10: 39600003  li r11, 3
	ctx.r[11].s64 = 3;
	// 83288E14: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 83288E18: 917F0054  stw r11, 0x54(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 83288E1C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83288E20: 387F0058  addi r3, r31, 0x58
	ctx.r[3].s64 = ctx.r[31].s64 + 88;
	// 83288E24: 4AFA40AD  bl 0x8222ced0
	ctx.lr = 0x83288E28;
	sub_8222CED0(ctx, base);
	// 83288E28: 3CC0820F  lis r6, -0x7df1
	ctx.r[6].s64 = -2112946176;
	// 83288E2C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83288E30: 38866820  addi r4, r6, 0x6820
	ctx.r[4].s64 = ctx.r[6].s64 + 26656;
	// 83288E34: 387F005C  addi r3, r31, 0x5c
	ctx.r[3].s64 = ctx.r[31].s64 + 92;
	// 83288E38: 4AFA4099  bl 0x8222ced0
	ctx.lr = 0x83288E3C;
	sub_8222CED0(ctx, base);
	// 83288E3C: 3C80820F  lis r4, -0x7df1
	ctx.r[4].s64 = -2112946176;
	// 83288E40: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83288E44: 388467FC  addi r4, r4, 0x67fc
	ctx.r[4].s64 = ctx.r[4].s64 + 26620;
	// 83288E48: 387F0060  addi r3, r31, 0x60
	ctx.r[3].s64 = ctx.r[31].s64 + 96;
	// 83288E4C: 4AFA4085  bl 0x8222ced0
	ctx.lr = 0x83288E50;
	sub_8222CED0(ctx, base);
	// 83288E50: 39600003  li r11, 3
	ctx.r[11].s64 = 3;
	// 83288E54: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 83288E58: 917F0064  stw r11, 0x64(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 83288E5C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83288E60: 387F0068  addi r3, r31, 0x68
	ctx.r[3].s64 = ctx.r[31].s64 + 104;
	// 83288E64: 4AFA406D  bl 0x8222ced0
	ctx.lr = 0x83288E68;
	sub_8222CED0(ctx, base);
	// 83288E68: 3C60820C  lis r3, -0x7df4
	ctx.r[3].s64 = -2113142784;
	// 83288E6C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83288E70: 38833414  addi r4, r3, 0x3414
	ctx.r[4].s64 = ctx.r[3].s64 + 13332;
	// 83288E74: 387F006C  addi r3, r31, 0x6c
	ctx.r[3].s64 = ctx.r[31].s64 + 108;
	// 83288E78: 4AFA4059  bl 0x8222ced0
	ctx.lr = 0x83288E7C;
	sub_8222CED0(ctx, base);
	// 83288E7C: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 83288E80: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83288E84: 388B67EC  addi r4, r11, 0x67ec
	ctx.r[4].s64 = ctx.r[11].s64 + 26604;
	// 83288E88: 387F0070  addi r3, r31, 0x70
	ctx.r[3].s64 = ctx.r[31].s64 + 112;
	// 83288E8C: 4AFA4045  bl 0x8222ced0
	ctx.lr = 0x83288E90;
	sub_8222CED0(ctx, base);
	// 83288E90: 39600003  li r11, 3
	ctx.r[11].s64 = 3;
	// 83288E94: 3D40820F  lis r10, -0x7df1
	ctx.r[10].s64 = -2112946176;
	// 83288E98: 917F0074  stw r11, 0x74(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 83288E9C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83288EA0: 3BAA4970  addi r29, r10, 0x4970
	ctx.r[29].s64 = ctx.r[10].s64 + 18800;
	// 83288EA4: 387F0078  addi r3, r31, 0x78
	ctx.r[3].s64 = ctx.r[31].s64 + 120;
	// 83288EA8: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 83288EAC: 4AFA4025  bl 0x8222ced0
	ctx.lr = 0x83288EB0;
	sub_8222CED0(ctx, base);
	// 83288EB0: 3D20820F  lis r9, -0x7df1
	ctx.r[9].s64 = -2112946176;
	// 83288EB4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83288EB8: 388967DC  addi r4, r9, 0x67dc
	ctx.r[4].s64 = ctx.r[9].s64 + 26588;
	// 83288EBC: 387F007C  addi r3, r31, 0x7c
	ctx.r[3].s64 = ctx.r[31].s64 + 124;
	// 83288EC0: 4AFA4011  bl 0x8222ced0
	ctx.lr = 0x83288EC4;
	sub_8222CED0(ctx, base);
	// 83288EC4: 3D00820F  lis r8, -0x7df1
	ctx.r[8].s64 = -2112946176;
	// 83288EC8: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83288ECC: 388867BC  addi r4, r8, 0x67bc
	ctx.r[4].s64 = ctx.r[8].s64 + 26556;
	// 83288ED0: 387F0080  addi r3, r31, 0x80
	ctx.r[3].s64 = ctx.r[31].s64 + 128;
	// 83288ED4: 4AFA3FFD  bl 0x8222ced0
	ctx.lr = 0x83288ED8;
	sub_8222CED0(ctx, base);
	// 83288ED8: 39600003  li r11, 3
	ctx.r[11].s64 = 3;
	// 83288EDC: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 83288EE0: 917F0084  stw r11, 0x84(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(132 as u32), ctx.r[11].u32 ) };
	// 83288EE4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83288EE8: 387F0088  addi r3, r31, 0x88
	ctx.r[3].s64 = ctx.r[31].s64 + 136;
	// 83288EEC: 4AFA3FE5  bl 0x8222ced0
	ctx.lr = 0x83288EF0;
	sub_8222CED0(ctx, base);
	// 83288EF0: 3CE0820F  lis r7, -0x7df1
	ctx.r[7].s64 = -2112946176;
	// 83288EF4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83288EF8: 388767A0  addi r4, r7, 0x67a0
	ctx.r[4].s64 = ctx.r[7].s64 + 26528;
	// 83288EFC: 387F008C  addi r3, r31, 0x8c
	ctx.r[3].s64 = ctx.r[31].s64 + 140;
	// 83288F00: 4AFA3FD1  bl 0x8222ced0
	ctx.lr = 0x83288F04;
	sub_8222CED0(ctx, base);
	// 83288F04: 3CC0820F  lis r6, -0x7df1
	ctx.r[6].s64 = -2112946176;
	// 83288F08: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83288F0C: 38866784  addi r4, r6, 0x6784
	ctx.r[4].s64 = ctx.r[6].s64 + 26500;
	// 83288F10: 387F0090  addi r3, r31, 0x90
	ctx.r[3].s64 = ctx.r[31].s64 + 144;
	// 83288F14: 4AFA3FBD  bl 0x8222ced0
	ctx.lr = 0x83288F18;
	sub_8222CED0(ctx, base);
	// 83288F18: 39600003  li r11, 3
	ctx.r[11].s64 = 3;
	// 83288F1C: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 83288F20: 917F0094  stw r11, 0x94(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(148 as u32), ctx.r[11].u32 ) };
	// 83288F24: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83288F28: 387F0098  addi r3, r31, 0x98
	ctx.r[3].s64 = ctx.r[31].s64 + 152;
	// 83288F2C: 4AFA3FA5  bl 0x8222ced0
	ctx.lr = 0x83288F30;
	sub_8222CED0(ctx, base);
	// 83288F30: 3C80820F  lis r4, -0x7df1
	ctx.r[4].s64 = -2112946176;
	// 83288F34: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83288F38: 3884676C  addi r4, r4, 0x676c
	ctx.r[4].s64 = ctx.r[4].s64 + 26476;
	// 83288F3C: 387F009C  addi r3, r31, 0x9c
	ctx.r[3].s64 = ctx.r[31].s64 + 156;
	// 83288F40: 4AFA3F91  bl 0x8222ced0
	ctx.lr = 0x83288F44;
	sub_8222CED0(ctx, base);
	// 83288F44: 3C60820F  lis r3, -0x7df1
	ctx.r[3].s64 = -2112946176;
	// 83288F48: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83288F4C: 38836760  addi r4, r3, 0x6760
	ctx.r[4].s64 = ctx.r[3].s64 + 26464;
	// 83288F50: 387F00A0  addi r3, r31, 0xa0
	ctx.r[3].s64 = ctx.r[31].s64 + 160;
	// 83288F54: 4AFA3F7D  bl 0x8222ced0
	ctx.lr = 0x83288F58;
	sub_8222CED0(ctx, base);
	// 83288F58: 39600002  li r11, 2
	ctx.r[11].s64 = 2;
	// 83288F5C: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 83288F60: 917F00A4  stw r11, 0xa4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(164 as u32), ctx.r[11].u32 ) };
	// 83288F64: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83288F68: 387F00A8  addi r3, r31, 0xa8
	ctx.r[3].s64 = ctx.r[31].s64 + 168;
	// 83288F6C: 4AFA3F65  bl 0x8222ced0
	ctx.lr = 0x83288F70;
	sub_8222CED0(ctx, base);
	// 83288F70: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 83288F74: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83288F78: 388B6754  addi r4, r11, 0x6754
	ctx.r[4].s64 = ctx.r[11].s64 + 26452;
	// 83288F7C: 387F00AC  addi r3, r31, 0xac
	ctx.r[3].s64 = ctx.r[31].s64 + 172;
	// 83288F80: 4AFA3F51  bl 0x8222ced0
	ctx.lr = 0x83288F84;
	sub_8222CED0(ctx, base);
	// 83288F84: 3D40820F  lis r10, -0x7df1
	ctx.r[10].s64 = -2112946176;
	// 83288F88: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83288F8C: 388A6738  addi r4, r10, 0x6738
	ctx.r[4].s64 = ctx.r[10].s64 + 26424;
	// 83288F90: 387F00B0  addi r3, r31, 0xb0
	ctx.r[3].s64 = ctx.r[31].s64 + 176;
	// 83288F94: 4AFA3F3D  bl 0x8222ced0
	ctx.lr = 0x83288F98;
	sub_8222CED0(ctx, base);
	// 83288F98: 39600002  li r11, 2
	ctx.r[11].s64 = 2;
	// 83288F9C: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 83288FA0: 917F00B4  stw r11, 0xb4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(180 as u32), ctx.r[11].u32 ) };
	// 83288FA4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83288FA8: 387F00B8  addi r3, r31, 0xb8
	ctx.r[3].s64 = ctx.r[31].s64 + 184;
	// 83288FAC: 4AFA3F25  bl 0x8222ced0
	ctx.lr = 0x83288FB0;
	sub_8222CED0(ctx, base);
	// 83288FB0: 3D20820F  lis r9, -0x7df1
	ctx.r[9].s64 = -2112946176;
	// 83288FB4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83288FB8: 38896720  addi r4, r9, 0x6720
	ctx.r[4].s64 = ctx.r[9].s64 + 26400;
	// 83288FBC: 387F00BC  addi r3, r31, 0xbc
	ctx.r[3].s64 = ctx.r[31].s64 + 188;
	// 83288FC0: 4AFA3F11  bl 0x8222ced0
	ctx.lr = 0x83288FC4;
	sub_8222CED0(ctx, base);
	// 83288FC4: 3D00820F  lis r8, -0x7df1
	ctx.r[8].s64 = -2112946176;
	// 83288FC8: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83288FCC: 38886700  addi r4, r8, 0x6700
	ctx.r[4].s64 = ctx.r[8].s64 + 26368;
	// 83288FD0: 387F00C0  addi r3, r31, 0xc0
	ctx.r[3].s64 = ctx.r[31].s64 + 192;
	// 83288FD4: 4AFA3EFD  bl 0x8222ced0
	ctx.lr = 0x83288FD8;
	sub_8222CED0(ctx, base);
	// 83288FD8: 39600003  li r11, 3
	ctx.r[11].s64 = 3;
	// 83288FDC: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 83288FE0: 917F00C4  stw r11, 0xc4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(196 as u32), ctx.r[11].u32 ) };
	// 83288FE4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83288FE8: 387F00C8  addi r3, r31, 0xc8
	ctx.r[3].s64 = ctx.r[31].s64 + 200;
	// 83288FEC: 4AFA3EE5  bl 0x8222ced0
	ctx.lr = 0x83288FF0;
	sub_8222CED0(ctx, base);
	// 83288FF0: 3CE0820F  lis r7, -0x7df1
	ctx.r[7].s64 = -2112946176;
	// 83288FF4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83288FF8: 388766E4  addi r4, r7, 0x66e4
	ctx.r[4].s64 = ctx.r[7].s64 + 26340;
	// 83288FFC: 387F00CC  addi r3, r31, 0xcc
	ctx.r[3].s64 = ctx.r[31].s64 + 204;
	// 83289000: 4AFA3ED1  bl 0x8222ced0
	ctx.lr = 0x83289004;
	sub_8222CED0(ctx, base);
	// 83289004: 3CC0820F  lis r6, -0x7df1
	ctx.r[6].s64 = -2112946176;
	// 83289008: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8328900C: 388666D0  addi r4, r6, 0x66d0
	ctx.r[4].s64 = ctx.r[6].s64 + 26320;
	// 83289010: 387F00D0  addi r3, r31, 0xd0
	ctx.r[3].s64 = ctx.r[31].s64 + 208;
	// 83289014: 4AFA3EBD  bl 0x8222ced0
	ctx.lr = 0x83289018;
	sub_8222CED0(ctx, base);
	// 83289018: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 8328901C: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 83289020: 917F00D4  stw r11, 0xd4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(212 as u32), ctx.r[11].u32 ) };
	// 83289024: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83289028: 387F00D8  addi r3, r31, 0xd8
	ctx.r[3].s64 = ctx.r[31].s64 + 216;
	// 8328902C: 4AFA3EA5  bl 0x8222ced0
	ctx.lr = 0x83289030;
	sub_8222CED0(ctx, base);
	// 83289030: 3C80820F  lis r4, -0x7df1
	ctx.r[4].s64 = -2112946176;
	// 83289034: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83289038: 388466C0  addi r4, r4, 0x66c0
	ctx.r[4].s64 = ctx.r[4].s64 + 26304;
	// 8328903C: 387F00DC  addi r3, r31, 0xdc
	ctx.r[3].s64 = ctx.r[31].s64 + 220;
	// 83289040: 4AFA3E91  bl 0x8222ced0
	ctx.lr = 0x83289044;
	sub_8222CED0(ctx, base);
	// 83289044: 3C60820F  lis r3, -0x7df1
	ctx.r[3].s64 = -2112946176;
	// 83289048: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8328904C: 388366AC  addi r4, r3, 0x66ac
	ctx.r[4].s64 = ctx.r[3].s64 + 26284;
	// 83289050: 387F00E0  addi r3, r31, 0xe0
	ctx.r[3].s64 = ctx.r[31].s64 + 224;
	// 83289054: 4AFA3E7D  bl 0x8222ced0
	ctx.lr = 0x83289058;
	sub_8222CED0(ctx, base);
	// 83289058: 39600003  li r11, 3
	ctx.r[11].s64 = 3;
	// 8328905C: 3D40820F  lis r10, -0x7df1
	ctx.r[10].s64 = -2112946176;
	// 83289060: 917F00E4  stw r11, 0xe4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(228 as u32), ctx.r[11].u32 ) };
	// 83289064: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83289068: 3B8A66A4  addi r28, r10, 0x66a4
	ctx.r[28].s64 = ctx.r[10].s64 + 26276;
	// 8328906C: 387F00E8  addi r3, r31, 0xe8
	ctx.r[3].s64 = ctx.r[31].s64 + 232;
	// 83289070: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 83289074: 4AFA3E5D  bl 0x8222ced0
	ctx.lr = 0x83289078;
	sub_8222CED0(ctx, base);
	// 83289078: 3D20820F  lis r9, -0x7df1
	ctx.r[9].s64 = -2112946176;
	// 8328907C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83289080: 38896694  addi r4, r9, 0x6694
	ctx.r[4].s64 = ctx.r[9].s64 + 26260;
	// 83289084: 387F00EC  addi r3, r31, 0xec
	ctx.r[3].s64 = ctx.r[31].s64 + 236;
	// 83289088: 4AFA3E49  bl 0x8222ced0
	ctx.lr = 0x8328908C;
	sub_8222CED0(ctx, base);
	// 8328908C: 3D00820F  lis r8, -0x7df1
	ctx.r[8].s64 = -2112946176;
	// 83289090: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83289094: 38886678  addi r4, r8, 0x6678
	ctx.r[4].s64 = ctx.r[8].s64 + 26232;
	// 83289098: 387F00F0  addi r3, r31, 0xf0
	ctx.r[3].s64 = ctx.r[31].s64 + 240;
	// 8328909C: 4AFA3E35  bl 0x8222ced0
	ctx.lr = 0x832890A0;
	sub_8222CED0(ctx, base);
	// 832890A0: 39600003  li r11, 3
	ctx.r[11].s64 = 3;
	// 832890A4: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 832890A8: 917F00F4  stw r11, 0xf4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(244 as u32), ctx.r[11].u32 ) };
	// 832890AC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832890B0: 387F00F8  addi r3, r31, 0xf8
	ctx.r[3].s64 = ctx.r[31].s64 + 248;
	// 832890B4: 4AFA3E1D  bl 0x8222ced0
	ctx.lr = 0x832890B8;
	sub_8222CED0(ctx, base);
	// 832890B8: 3CE0820F  lis r7, -0x7df1
	ctx.r[7].s64 = -2112946176;
	// 832890BC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832890C0: 38876668  addi r4, r7, 0x6668
	ctx.r[4].s64 = ctx.r[7].s64 + 26216;
	// 832890C4: 387F00FC  addi r3, r31, 0xfc
	ctx.r[3].s64 = ctx.r[31].s64 + 252;
	// 832890C8: 4AFA3E09  bl 0x8222ced0
	ctx.lr = 0x832890CC;
	sub_8222CED0(ctx, base);
	// 832890CC: 3CC0820F  lis r6, -0x7df1
	ctx.r[6].s64 = -2112946176;
	// 832890D0: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832890D4: 3886664C  addi r4, r6, 0x664c
	ctx.r[4].s64 = ctx.r[6].s64 + 26188;
	// 832890D8: 387F0100  addi r3, r31, 0x100
	ctx.r[3].s64 = ctx.r[31].s64 + 256;
	// 832890DC: 4AFA3DF5  bl 0x8222ced0
	ctx.lr = 0x832890E0;
	sub_8222CED0(ctx, base);
	// 832890E0: 39600003  li r11, 3
	ctx.r[11].s64 = 3;
	// 832890E4: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 832890E8: 917F0104  stw r11, 0x104(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(260 as u32), ctx.r[11].u32 ) };
	// 832890EC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832890F0: 387F0108  addi r3, r31, 0x108
	ctx.r[3].s64 = ctx.r[31].s64 + 264;
	// 832890F4: 4AFA3DDD  bl 0x8222ced0
	ctx.lr = 0x832890F8;
	sub_8222CED0(ctx, base);
	// 832890F8: 3C80820F  lis r4, -0x7df1
	ctx.r[4].s64 = -2112946176;
	// 832890FC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83289100: 38846640  addi r4, r4, 0x6640
	ctx.r[4].s64 = ctx.r[4].s64 + 26176;
	// 83289104: 387F010C  addi r3, r31, 0x10c
	ctx.r[3].s64 = ctx.r[31].s64 + 268;
	// 83289108: 4AFA3DC9  bl 0x8222ced0
	ctx.lr = 0x8328910C;
	sub_8222CED0(ctx, base);
	// 8328910C: 3C60820F  lis r3, -0x7df1
	ctx.r[3].s64 = -2112946176;
	// 83289110: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83289114: 38836624  addi r4, r3, 0x6624
	ctx.r[4].s64 = ctx.r[3].s64 + 26148;
	// 83289118: 387F0110  addi r3, r31, 0x110
	ctx.r[3].s64 = ctx.r[31].s64 + 272;
	// 8328911C: 4AFA3DB5  bl 0x8222ced0
	ctx.lr = 0x83289120;
	sub_8222CED0(ctx, base);
	// 83289120: 39600003  li r11, 3
	ctx.r[11].s64 = 3;
	// 83289124: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 83289128: 917F0114  stw r11, 0x114(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(276 as u32), ctx.r[11].u32 ) };
	// 8328912C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83289130: 387F0118  addi r3, r31, 0x118
	ctx.r[3].s64 = ctx.r[31].s64 + 280;
	// 83289134: 4AFA3D9D  bl 0x8222ced0
	ctx.lr = 0x83289138;
	sub_8222CED0(ctx, base);
	// 83289138: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 8328913C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83289140: 388B6618  addi r4, r11, 0x6618
	ctx.r[4].s64 = ctx.r[11].s64 + 26136;
	// 83289144: 387F011C  addi r3, r31, 0x11c
	ctx.r[3].s64 = ctx.r[31].s64 + 284;
	// 83289148: 4AFA3D89  bl 0x8222ced0
	ctx.lr = 0x8328914C;
	sub_8222CED0(ctx, base);
	// 8328914C: 3D40820F  lis r10, -0x7df1
	ctx.r[10].s64 = -2112946176;
	// 83289150: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83289154: 388A6600  addi r4, r10, 0x6600
	ctx.r[4].s64 = ctx.r[10].s64 + 26112;
	// 83289158: 387F0120  addi r3, r31, 0x120
	ctx.r[3].s64 = ctx.r[31].s64 + 288;
	// 8328915C: 4AFA3D75  bl 0x8222ced0
	ctx.lr = 0x83289160;
	sub_8222CED0(ctx, base);
	// 83289160: 39600003  li r11, 3
	ctx.r[11].s64 = 3;
	// 83289164: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 83289168: 917F0124  stw r11, 0x124(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(292 as u32), ctx.r[11].u32 ) };
	// 8328916C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83289170: 387F0128  addi r3, r31, 0x128
	ctx.r[3].s64 = ctx.r[31].s64 + 296;
	// 83289174: 4AFA3D5D  bl 0x8222ced0
	ctx.lr = 0x83289178;
	sub_8222CED0(ctx, base);
	// 83289178: 3D20820F  lis r9, -0x7df1
	ctx.r[9].s64 = -2112946176;
	// 8328917C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83289180: 388965F4  addi r4, r9, 0x65f4
	ctx.r[4].s64 = ctx.r[9].s64 + 26100;
	// 83289184: 387F012C  addi r3, r31, 0x12c
	ctx.r[3].s64 = ctx.r[31].s64 + 300;
	// 83289188: 4AFA3D49  bl 0x8222ced0
	ctx.lr = 0x8328918C;
	sub_8222CED0(ctx, base);
	// 8328918C: 3D00820F  lis r8, -0x7df1
	ctx.r[8].s64 = -2112946176;
	// 83289190: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83289194: 388865D4  addi r4, r8, 0x65d4
	ctx.r[4].s64 = ctx.r[8].s64 + 26068;
	// 83289198: 387F0130  addi r3, r31, 0x130
	ctx.r[3].s64 = ctx.r[31].s64 + 304;
	// 8328919C: 4AFA3D35  bl 0x8222ced0
	ctx.lr = 0x832891A0;
	sub_8222CED0(ctx, base);
	// 832891A0: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 832891A4: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 832891A8: 917F0134  stw r11, 0x134(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(308 as u32), ctx.r[11].u32 ) };
	// 832891AC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832891B0: 387F0138  addi r3, r31, 0x138
	ctx.r[3].s64 = ctx.r[31].s64 + 312;
	// 832891B4: 4AFA3D1D  bl 0x8222ced0
	ctx.lr = 0x832891B8;
	sub_8222CED0(ctx, base);
	// 832891B8: 3CE0820F  lis r7, -0x7df1
	ctx.r[7].s64 = -2112946176;
	// 832891BC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832891C0: 388765C4  addi r4, r7, 0x65c4
	ctx.r[4].s64 = ctx.r[7].s64 + 26052;
	// 832891C4: 387F013C  addi r3, r31, 0x13c
	ctx.r[3].s64 = ctx.r[31].s64 + 316;
	// 832891C8: 4AFA3D09  bl 0x8222ced0
	ctx.lr = 0x832891CC;
	sub_8222CED0(ctx, base);
	// 832891CC: 3CC0820F  lis r6, -0x7df1
	ctx.r[6].s64 = -2112946176;
	// 832891D0: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832891D4: 388665A0  addi r4, r6, 0x65a0
	ctx.r[4].s64 = ctx.r[6].s64 + 26016;
	// 832891D8: 387F0140  addi r3, r31, 0x140
	ctx.r[3].s64 = ctx.r[31].s64 + 320;
	// 832891DC: 4AFA3CF5  bl 0x8222ced0
	ctx.lr = 0x832891E0;
	sub_8222CED0(ctx, base);
	// 832891E0: 39600003  li r11, 3
	ctx.r[11].s64 = 3;
	// 832891E4: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 832891E8: 917F0144  stw r11, 0x144(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(324 as u32), ctx.r[11].u32 ) };
	// 832891EC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832891F0: 387F0148  addi r3, r31, 0x148
	ctx.r[3].s64 = ctx.r[31].s64 + 328;
	// 832891F4: 4AFA3CDD  bl 0x8222ced0
	ctx.lr = 0x832891F8;
	sub_8222CED0(ctx, base);
	// 832891F8: 3C80820F  lis r4, -0x7df1
	ctx.r[4].s64 = -2112946176;
	// 832891FC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83289200: 3884658C  addi r4, r4, 0x658c
	ctx.r[4].s64 = ctx.r[4].s64 + 25996;
	// 83289204: 387F014C  addi r3, r31, 0x14c
	ctx.r[3].s64 = ctx.r[31].s64 + 332;
	// 83289208: 4AFA3CC9  bl 0x8222ced0
	ctx.lr = 0x8328920C;
	sub_8222CED0(ctx, base);
	// 8328920C: 3C60820F  lis r3, -0x7df1
	ctx.r[3].s64 = -2112946176;
	// 83289210: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83289214: 38836570  addi r4, r3, 0x6570
	ctx.r[4].s64 = ctx.r[3].s64 + 25968;
	// 83289218: 387F0150  addi r3, r31, 0x150
	ctx.r[3].s64 = ctx.r[31].s64 + 336;
	// 8328921C: 4AFA3CB5  bl 0x8222ced0
	ctx.lr = 0x83289220;
	sub_8222CED0(ctx, base);
	// 83289220: 39600003  li r11, 3
	ctx.r[11].s64 = 3;
	// 83289224: 3D40820F  lis r10, -0x7df1
	ctx.r[10].s64 = -2112946176;
	// 83289228: 917F0154  stw r11, 0x154(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(340 as u32), ctx.r[11].u32 ) };
	// 8328922C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83289230: 3B8A4990  addi r28, r10, 0x4990
	ctx.r[28].s64 = ctx.r[10].s64 + 18832;
	// 83289234: 387F0158  addi r3, r31, 0x158
	ctx.r[3].s64 = ctx.r[31].s64 + 344;
	// 83289238: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 8328923C: 4AFA3C95  bl 0x8222ced0
	ctx.lr = 0x83289240;
	sub_8222CED0(ctx, base);
	// 83289240: 3D20820F  lis r9, -0x7df1
	ctx.r[9].s64 = -2112946176;
	// 83289244: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83289248: 38896560  addi r4, r9, 0x6560
	ctx.r[4].s64 = ctx.r[9].s64 + 25952;
	// 8328924C: 387F015C  addi r3, r31, 0x15c
	ctx.r[3].s64 = ctx.r[31].s64 + 348;
	// 83289250: 4AFA3C81  bl 0x8222ced0
	ctx.lr = 0x83289254;
	sub_8222CED0(ctx, base);
	// 83289254: 3D00820F  lis r8, -0x7df1
	ctx.r[8].s64 = -2112946176;
	// 83289258: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8328925C: 38886544  addi r4, r8, 0x6544
	ctx.r[4].s64 = ctx.r[8].s64 + 25924;
	// 83289260: 387F0160  addi r3, r31, 0x160
	ctx.r[3].s64 = ctx.r[31].s64 + 352;
	// 83289264: 4AFA3C6D  bl 0x8222ced0
	ctx.lr = 0x83289268;
	sub_8222CED0(ctx, base);
	// 83289268: 39600003  li r11, 3
	ctx.r[11].s64 = 3;
	// 8328926C: 3CE0820F  lis r7, -0x7df1
	ctx.r[7].s64 = -2112946176;
	// 83289270: 917F0164  stw r11, 0x164(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(356 as u32), ctx.r[11].u32 ) };
	// 83289274: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83289278: 3B676538  addi r27, r7, 0x6538
	ctx.r[27].s64 = ctx.r[7].s64 + 25912;
	// 8328927C: 387F0168  addi r3, r31, 0x168
	ctx.r[3].s64 = ctx.r[31].s64 + 360;
	// 83289280: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 83289284: 4AFA3C4D  bl 0x8222ced0
	ctx.lr = 0x83289288;
	sub_8222CED0(ctx, base);
	// 83289288: 3CC0820F  lis r6, -0x7df1
	ctx.r[6].s64 = -2112946176;
	// 8328928C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83289290: 3886652C  addi r4, r6, 0x652c
	ctx.r[4].s64 = ctx.r[6].s64 + 25900;
	// 83289294: 387F016C  addi r3, r31, 0x16c
	ctx.r[3].s64 = ctx.r[31].s64 + 364;
	// 83289298: 4AFA3C39  bl 0x8222ced0
	ctx.lr = 0x8328929C;
	sub_8222CED0(ctx, base);
	// 8328929C: 3C80820F  lis r4, -0x7df1
	ctx.r[4].s64 = -2112946176;
	// 832892A0: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832892A4: 38846514  addi r4, r4, 0x6514
	ctx.r[4].s64 = ctx.r[4].s64 + 25876;
	// 832892A8: 387F0170  addi r3, r31, 0x170
	ctx.r[3].s64 = ctx.r[31].s64 + 368;
	// 832892AC: 4AFA3C25  bl 0x8222ced0
	ctx.lr = 0x832892B0;
	sub_8222CED0(ctx, base);
	// 832892B0: 39600003  li r11, 3
	ctx.r[11].s64 = 3;
	// 832892B4: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 832892B8: 917F0174  stw r11, 0x174(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(372 as u32), ctx.r[11].u32 ) };
	// 832892BC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832892C0: 387F0178  addi r3, r31, 0x178
	ctx.r[3].s64 = ctx.r[31].s64 + 376;
	// 832892C4: 4AFA3C0D  bl 0x8222ced0
	ctx.lr = 0x832892C8;
	sub_8222CED0(ctx, base);
	// 832892C8: 3C60820F  lis r3, -0x7df1
	ctx.r[3].s64 = -2112946176;
	// 832892CC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832892D0: 38836508  addi r4, r3, 0x6508
	ctx.r[4].s64 = ctx.r[3].s64 + 25864;
	// 832892D4: 387F017C  addi r3, r31, 0x17c
	ctx.r[3].s64 = ctx.r[31].s64 + 380;
	// 832892D8: 4AFA3BF9  bl 0x8222ced0
	ctx.lr = 0x832892DC;
	sub_8222CED0(ctx, base);
	// 832892DC: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 832892E0: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832892E4: 388B64E0  addi r4, r11, 0x64e0
	ctx.r[4].s64 = ctx.r[11].s64 + 25824;
	// 832892E8: 387F0180  addi r3, r31, 0x180
	ctx.r[3].s64 = ctx.r[31].s64 + 384;
	// 832892EC: 4AFA3BE5  bl 0x8222ced0
	ctx.lr = 0x832892F0;
	sub_8222CED0(ctx, base);
	// 832892F0: 39600003  li r11, 3
	ctx.r[11].s64 = 3;
	// 832892F4: 3D40820F  lis r10, -0x7df1
	ctx.r[10].s64 = -2112946176;
	// 832892F8: 917F0184  stw r11, 0x184(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(388 as u32), ctx.r[11].u32 ) };
	// 832892FC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83289300: 3B4A64C0  addi r26, r10, 0x64c0
	ctx.r[26].s64 = ctx.r[10].s64 + 25792;
	// 83289304: 387F0188  addi r3, r31, 0x188
	ctx.r[3].s64 = ctx.r[31].s64 + 392;
	// 83289308: 7F44D378  mr r4, r26
	ctx.r[4].u64 = ctx.r[26].u64;
	// 8328930C: 4AFA3BC5  bl 0x8222ced0
	ctx.lr = 0x83289310;
	sub_8222CED0(ctx, base);
	// 83289310: 3D20820F  lis r9, -0x7df1
	ctx.r[9].s64 = -2112946176;
	// 83289314: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83289318: 388964A8  addi r4, r9, 0x64a8
	ctx.r[4].s64 = ctx.r[9].s64 + 25768;
	// 8328931C: 387F018C  addi r3, r31, 0x18c
	ctx.r[3].s64 = ctx.r[31].s64 + 396;
	// 83289320: 4AFA3BB1  bl 0x8222ced0
	ctx.lr = 0x83289324;
	sub_8222CED0(ctx, base);
	// 83289324: 3D00820F  lis r8, -0x7df1
	ctx.r[8].s64 = -2112946176;
	// 83289328: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8328932C: 38886480  addi r4, r8, 0x6480
	ctx.r[4].s64 = ctx.r[8].s64 + 25728;
	// 83289330: 387F0190  addi r3, r31, 0x190
	ctx.r[3].s64 = ctx.r[31].s64 + 400;
	// 83289334: 4AFA3B9D  bl 0x8222ced0
	ctx.lr = 0x83289338;
	sub_8222CED0(ctx, base);
	// 83289338: 39600003  li r11, 3
	ctx.r[11].s64 = 3;
	// 8328933C: 7F44D378  mr r4, r26
	ctx.r[4].u64 = ctx.r[26].u64;
	// 83289340: 917F0194  stw r11, 0x194(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(404 as u32), ctx.r[11].u32 ) };
	// 83289344: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83289348: 387F0198  addi r3, r31, 0x198
	ctx.r[3].s64 = ctx.r[31].s64 + 408;
	// 8328934C: 4AFA3B85  bl 0x8222ced0
	ctx.lr = 0x83289350;
	sub_8222CED0(ctx, base);
	// 83289350: 3CE0820F  lis r7, -0x7df1
	ctx.r[7].s64 = -2112946176;
	// 83289354: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83289358: 38876468  addi r4, r7, 0x6468
	ctx.r[4].s64 = ctx.r[7].s64 + 25704;
	// 8328935C: 387F019C  addi r3, r31, 0x19c
	ctx.r[3].s64 = ctx.r[31].s64 + 412;
	// 83289360: 4AFA3B71  bl 0x8222ced0
	ctx.lr = 0x83289364;
	sub_8222CED0(ctx, base);
	// 83289364: 3CC0820F  lis r6, -0x7df1
	ctx.r[6].s64 = -2112946176;
	// 83289368: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8328936C: 38866440  addi r4, r6, 0x6440
	ctx.r[4].s64 = ctx.r[6].s64 + 25664;
	// 83289370: 387F01A0  addi r3, r31, 0x1a0
	ctx.r[3].s64 = ctx.r[31].s64 + 416;
	// 83289374: 4AFA3B5D  bl 0x8222ced0
	ctx.lr = 0x83289378;
	sub_8222CED0(ctx, base);
	// 83289378: 39600003  li r11, 3
	ctx.r[11].s64 = 3;
	// 8328937C: 7F44D378  mr r4, r26
	ctx.r[4].u64 = ctx.r[26].u64;
	// 83289380: 917F01A4  stw r11, 0x1a4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(420 as u32), ctx.r[11].u32 ) };
	// 83289384: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83289388: 387F01A8  addi r3, r31, 0x1a8
	ctx.r[3].s64 = ctx.r[31].s64 + 424;
	// 8328938C: 4AFA3B45  bl 0x8222ced0
	ctx.lr = 0x83289390;
	sub_8222CED0(ctx, base);
	// 83289390: 3C80820F  lis r4, -0x7df1
	ctx.r[4].s64 = -2112946176;
	// 83289394: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83289398: 38846428  addi r4, r4, 0x6428
	ctx.r[4].s64 = ctx.r[4].s64 + 25640;
	// 8328939C: 387F01AC  addi r3, r31, 0x1ac
	ctx.r[3].s64 = ctx.r[31].s64 + 428;
	// 832893A0: 4AFA3B31  bl 0x8222ced0
	ctx.lr = 0x832893A4;
	sub_8222CED0(ctx, base);
	// 832893A4: 3C60820F  lis r3, -0x7df1
	ctx.r[3].s64 = -2112946176;
	// 832893A8: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832893AC: 38836404  addi r4, r3, 0x6404
	ctx.r[4].s64 = ctx.r[3].s64 + 25604;
	// 832893B0: 387F01B0  addi r3, r31, 0x1b0
	ctx.r[3].s64 = ctx.r[31].s64 + 432;
	// 832893B4: 4AFA3B1D  bl 0x8222ced0
	ctx.lr = 0x832893B8;
	sub_8222CED0(ctx, base);
	// 832893B8: 39600003  li r11, 3
	ctx.r[11].s64 = 3;
	// 832893BC: 7F44D378  mr r4, r26
	ctx.r[4].u64 = ctx.r[26].u64;
	// 832893C0: 917F01B4  stw r11, 0x1b4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(436 as u32), ctx.r[11].u32 ) };
	// 832893C4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832893C8: 387F01B8  addi r3, r31, 0x1b8
	ctx.r[3].s64 = ctx.r[31].s64 + 440;
	// 832893CC: 4AFA3B05  bl 0x8222ced0
	ctx.lr = 0x832893D0;
	sub_8222CED0(ctx, base);
	// 832893D0: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 832893D4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832893D8: 388B63F0  addi r4, r11, 0x63f0
	ctx.r[4].s64 = ctx.r[11].s64 + 25584;
	// 832893DC: 387F01BC  addi r3, r31, 0x1bc
	ctx.r[3].s64 = ctx.r[31].s64 + 444;
	// 832893E0: 4AFA3AF1  bl 0x8222ced0
	ctx.lr = 0x832893E4;
	sub_8222CED0(ctx, base);
	// 832893E4: 3D40820F  lis r10, -0x7df1
	ctx.r[10].s64 = -2112946176;
	// 832893E8: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832893EC: 388A63C4  addi r4, r10, 0x63c4
	ctx.r[4].s64 = ctx.r[10].s64 + 25540;
	// 832893F0: 387F01C0  addi r3, r31, 0x1c0
	ctx.r[3].s64 = ctx.r[31].s64 + 448;
	// 832893F4: 4AFA3ADD  bl 0x8222ced0
	ctx.lr = 0x832893F8;
	sub_8222CED0(ctx, base);
	// 832893F8: 39600003  li r11, 3
	ctx.r[11].s64 = 3;
	// 832893FC: 7F44D378  mr r4, r26
	ctx.r[4].u64 = ctx.r[26].u64;
	// 83289400: 917F01C4  stw r11, 0x1c4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(452 as u32), ctx.r[11].u32 ) };
	// 83289404: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83289408: 387F01C8  addi r3, r31, 0x1c8
	ctx.r[3].s64 = ctx.r[31].s64 + 456;
	// 8328940C: 4AFA3AC5  bl 0x8222ced0
	ctx.lr = 0x83289410;
	sub_8222CED0(ctx, base);
	// 83289410: 3D20820F  lis r9, -0x7df1
	ctx.r[9].s64 = -2112946176;
	// 83289414: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83289418: 388963A8  addi r4, r9, 0x63a8
	ctx.r[4].s64 = ctx.r[9].s64 + 25512;
	// 8328941C: 387F01CC  addi r3, r31, 0x1cc
	ctx.r[3].s64 = ctx.r[31].s64 + 460;
	// 83289420: 4AFA3AB1  bl 0x8222ced0
	ctx.lr = 0x83289424;
	sub_8222CED0(ctx, base);
	// 83289424: 3D00820F  lis r8, -0x7df1
	ctx.r[8].s64 = -2112946176;
	// 83289428: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8328942C: 38886384  addi r4, r8, 0x6384
	ctx.r[4].s64 = ctx.r[8].s64 + 25476;
	// 83289430: 387F01D0  addi r3, r31, 0x1d0
	ctx.r[3].s64 = ctx.r[31].s64 + 464;
	// 83289434: 4AFA3A9D  bl 0x8222ced0
	ctx.lr = 0x83289438;
	sub_8222CED0(ctx, base);
	// 83289438: 39600003  li r11, 3
	ctx.r[11].s64 = 3;
	// 8328943C: 7F44D378  mr r4, r26
	ctx.r[4].u64 = ctx.r[26].u64;
	// 83289440: 917F01D4  stw r11, 0x1d4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(468 as u32), ctx.r[11].u32 ) };
	// 83289444: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83289448: 387F01D8  addi r3, r31, 0x1d8
	ctx.r[3].s64 = ctx.r[31].s64 + 472;
	// 8328944C: 4AFA3A85  bl 0x8222ced0
	ctx.lr = 0x83289450;
	sub_8222CED0(ctx, base);
	// 83289450: 3CE0820F  lis r7, -0x7df1
	ctx.r[7].s64 = -2112946176;
	// 83289454: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83289458: 38876364  addi r4, r7, 0x6364
	ctx.r[4].s64 = ctx.r[7].s64 + 25444;
	// 8328945C: 387F01DC  addi r3, r31, 0x1dc
	ctx.r[3].s64 = ctx.r[31].s64 + 476;
	// 83289460: 4AFA3A71  bl 0x8222ced0
	ctx.lr = 0x83289464;
	sub_8222CED0(ctx, base);
	// 83289464: 3CC0820F  lis r6, -0x7df1
	ctx.r[6].s64 = -2112946176;
	// 83289468: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8328946C: 38866344  addi r4, r6, 0x6344
	ctx.r[4].s64 = ctx.r[6].s64 + 25412;
	// 83289470: 387F01E0  addi r3, r31, 0x1e0
	ctx.r[3].s64 = ctx.r[31].s64 + 480;
	// 83289474: 4AFA3A5D  bl 0x8222ced0
	ctx.lr = 0x83289478;
	sub_8222CED0(ctx, base);
	// 83289478: 39600003  li r11, 3
	ctx.r[11].s64 = 3;
	// 8328947C: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 83289480: 917F01E4  stw r11, 0x1e4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(484 as u32), ctx.r[11].u32 ) };
	// 83289484: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83289488: 387F01E8  addi r3, r31, 0x1e8
	ctx.r[3].s64 = ctx.r[31].s64 + 488;
	// 8328948C: 4AFA3A45  bl 0x8222ced0
	ctx.lr = 0x83289490;
	sub_8222CED0(ctx, base);
	// 83289490: 3C80820F  lis r4, -0x7df1
	ctx.r[4].s64 = -2112946176;
	// 83289494: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83289498: 38846330  addi r4, r4, 0x6330
	ctx.r[4].s64 = ctx.r[4].s64 + 25392;
	// 8328949C: 387F01EC  addi r3, r31, 0x1ec
	ctx.r[3].s64 = ctx.r[31].s64 + 492;
	// 832894A0: 4AFA3A31  bl 0x8222ced0
	ctx.lr = 0x832894A4;
	sub_8222CED0(ctx, base);
	// 832894A4: 3C60820F  lis r3, -0x7df1
	ctx.r[3].s64 = -2112946176;
	// 832894A8: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832894AC: 38836308  addi r4, r3, 0x6308
	ctx.r[4].s64 = ctx.r[3].s64 + 25352;
	// 832894B0: 387F01F0  addi r3, r31, 0x1f0
	ctx.r[3].s64 = ctx.r[31].s64 + 496;
	// 832894B4: 4AFA3A1D  bl 0x8222ced0
	ctx.lr = 0x832894B8;
	sub_8222CED0(ctx, base);
	// 832894B8: 39600003  li r11, 3
	ctx.r[11].s64 = 3;
	// 832894BC: 3D40820F  lis r10, -0x7df1
	ctx.r[10].s64 = -2112946176;
	// 832894C0: 917F01F4  stw r11, 0x1f4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(500 as u32), ctx.r[11].u32 ) };
	// 832894C4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832894C8: 3B4A62E0  addi r26, r10, 0x62e0
	ctx.r[26].s64 = ctx.r[10].s64 + 25312;
	// 832894CC: 387F01F8  addi r3, r31, 0x1f8
	ctx.r[3].s64 = ctx.r[31].s64 + 504;
	// 832894D0: 7F44D378  mr r4, r26
	ctx.r[4].u64 = ctx.r[26].u64;
	// 832894D4: 4AFA39FD  bl 0x8222ced0
	ctx.lr = 0x832894D8;
	sub_8222CED0(ctx, base);
	// 832894D8: 3D20820F  lis r9, -0x7df1
	ctx.r[9].s64 = -2112946176;
	// 832894DC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832894E0: 388962C8  addi r4, r9, 0x62c8
	ctx.r[4].s64 = ctx.r[9].s64 + 25288;
	// 832894E4: 387F01FC  addi r3, r31, 0x1fc
	ctx.r[3].s64 = ctx.r[31].s64 + 508;
	// 832894E8: 4AFA39E9  bl 0x8222ced0
	ctx.lr = 0x832894EC;
	sub_8222CED0(ctx, base);
	// 832894EC: 3D00820F  lis r8, -0x7df1
	ctx.r[8].s64 = -2112946176;
	// 832894F0: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832894F4: 388862A0  addi r4, r8, 0x62a0
	ctx.r[4].s64 = ctx.r[8].s64 + 25248;
	// 832894F8: 387F0200  addi r3, r31, 0x200
	ctx.r[3].s64 = ctx.r[31].s64 + 512;
	// 832894FC: 4AFA39D5  bl 0x8222ced0
	ctx.lr = 0x83289500;
	sub_8222CED0(ctx, base);
	// 83289500: 39600003  li r11, 3
	ctx.r[11].s64 = 3;
	// 83289504: 7F44D378  mr r4, r26
	ctx.r[4].u64 = ctx.r[26].u64;
	// 83289508: 917F0204  stw r11, 0x204(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(516 as u32), ctx.r[11].u32 ) };
	// 8328950C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83289510: 387F0208  addi r3, r31, 0x208
	ctx.r[3].s64 = ctx.r[31].s64 + 520;
	// 83289514: 4AFA39BD  bl 0x8222ced0
	ctx.lr = 0x83289518;
	sub_8222CED0(ctx, base);
	// 83289518: 3CE0820F  lis r7, -0x7df1
	ctx.r[7].s64 = -2112946176;
	// 8328951C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83289520: 38876288  addi r4, r7, 0x6288
	ctx.r[4].s64 = ctx.r[7].s64 + 25224;
	// 83289524: 387F020C  addi r3, r31, 0x20c
	ctx.r[3].s64 = ctx.r[31].s64 + 524;
	// 83289528: 4AFA39A9  bl 0x8222ced0
	ctx.lr = 0x8328952C;
	sub_8222CED0(ctx, base);
	// 8328952C: 3CC0820F  lis r6, -0x7df1
	ctx.r[6].s64 = -2112946176;
	// 83289530: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83289534: 38866260  addi r4, r6, 0x6260
	ctx.r[4].s64 = ctx.r[6].s64 + 25184;
	// 83289538: 387F0210  addi r3, r31, 0x210
	ctx.r[3].s64 = ctx.r[31].s64 + 528;
	// 8328953C: 4AFA3995  bl 0x8222ced0
	ctx.lr = 0x83289540;
	sub_8222CED0(ctx, base);
	// 83289540: 39600003  li r11, 3
	ctx.r[11].s64 = 3;
	// 83289544: 7F44D378  mr r4, r26
	ctx.r[4].u64 = ctx.r[26].u64;
	// 83289548: 917F0214  stw r11, 0x214(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(532 as u32), ctx.r[11].u32 ) };
	// 8328954C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83289550: 387F0218  addi r3, r31, 0x218
	ctx.r[3].s64 = ctx.r[31].s64 + 536;
	// 83289554: 4AFA397D  bl 0x8222ced0
	ctx.lr = 0x83289558;
	sub_8222CED0(ctx, base);
	// 83289558: 3C80820F  lis r4, -0x7df1
	ctx.r[4].s64 = -2112946176;
	// 8328955C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83289560: 38846248  addi r4, r4, 0x6248
	ctx.r[4].s64 = ctx.r[4].s64 + 25160;
	// 83289564: 387F021C  addi r3, r31, 0x21c
	ctx.r[3].s64 = ctx.r[31].s64 + 540;
	// 83289568: 4AFA3969  bl 0x8222ced0
	ctx.lr = 0x8328956C;
	sub_8222CED0(ctx, base);
	// 8328956C: 3C60820F  lis r3, -0x7df1
	ctx.r[3].s64 = -2112946176;
	// 83289570: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83289574: 38836220  addi r4, r3, 0x6220
	ctx.r[4].s64 = ctx.r[3].s64 + 25120;
	// 83289578: 387F0220  addi r3, r31, 0x220
	ctx.r[3].s64 = ctx.r[31].s64 + 544;
	// 8328957C: 4AFA3955  bl 0x8222ced0
	ctx.lr = 0x83289580;
	sub_8222CED0(ctx, base);
	// 83289580: 39600003  li r11, 3
	ctx.r[11].s64 = 3;
	// 83289584: 3D40820F  lis r10, -0x7df1
	ctx.r[10].s64 = -2112946176;
	// 83289588: 917F0224  stw r11, 0x224(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(548 as u32), ctx.r[11].u32 ) };
	// 8328958C: 3B4A620C  addi r26, r10, 0x620c
	ctx.r[26].s64 = ctx.r[10].s64 + 25100;
	// 83289590: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83289594: 7F44D378  mr r4, r26
	ctx.r[4].u64 = ctx.r[26].u64;
	// 83289598: 387F0228  addi r3, r31, 0x228
	ctx.r[3].s64 = ctx.r[31].s64 + 552;
	// 8328959C: 4AFA3935  bl 0x8222ced0
	ctx.lr = 0x832895A0;
	sub_8222CED0(ctx, base);
	// 832895A0: 3D20820F  lis r9, -0x7df1
	ctx.r[9].s64 = -2112946176;
	// 832895A4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832895A8: 388961FC  addi r4, r9, 0x61fc
	ctx.r[4].s64 = ctx.r[9].s64 + 25084;
	// 832895AC: 387F022C  addi r3, r31, 0x22c
	ctx.r[3].s64 = ctx.r[31].s64 + 556;
	// 832895B0: 4AFA3921  bl 0x8222ced0
	ctx.lr = 0x832895B4;
	sub_8222CED0(ctx, base);
	// 832895B4: 3D00820F  lis r8, -0x7df1
	ctx.r[8].s64 = -2112946176;
	// 832895B8: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832895BC: 388861E0  addi r4, r8, 0x61e0
	ctx.r[4].s64 = ctx.r[8].s64 + 25056;
	// 832895C0: 387F0230  addi r3, r31, 0x230
	ctx.r[3].s64 = ctx.r[31].s64 + 560;
	// 832895C4: 4AFA390D  bl 0x8222ced0
	ctx.lr = 0x832895C8;
	sub_8222CED0(ctx, base);
	// 832895C8: 39600003  li r11, 3
	ctx.r[11].s64 = 3;
	// 832895CC: 7F44D378  mr r4, r26
	ctx.r[4].u64 = ctx.r[26].u64;
	// 832895D0: 917F0234  stw r11, 0x234(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(564 as u32), ctx.r[11].u32 ) };
	// 832895D4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832895D8: 387F0238  addi r3, r31, 0x238
	ctx.r[3].s64 = ctx.r[31].s64 + 568;
	// 832895DC: 4AFA38F5  bl 0x8222ced0
	ctx.lr = 0x832895E0;
	sub_8222CED0(ctx, base);
	// 832895E0: 3CE0820F  lis r7, -0x7df1
	ctx.r[7].s64 = -2112946176;
	// 832895E4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832895E8: 388761D0  addi r4, r7, 0x61d0
	ctx.r[4].s64 = ctx.r[7].s64 + 25040;
	// 832895EC: 387F023C  addi r3, r31, 0x23c
	ctx.r[3].s64 = ctx.r[31].s64 + 572;
	// 832895F0: 4AFA38E1  bl 0x8222ced0
	ctx.lr = 0x832895F4;
	sub_8222CED0(ctx, base);
	// 832895F4: 3CC0820F  lis r6, -0x7df1
	ctx.r[6].s64 = -2112946176;
	// 832895F8: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832895FC: 388661B0  addi r4, r6, 0x61b0
	ctx.r[4].s64 = ctx.r[6].s64 + 25008;
	// 83289600: 387F0240  addi r3, r31, 0x240
	ctx.r[3].s64 = ctx.r[31].s64 + 576;
	// 83289604: 4AFA38CD  bl 0x8222ced0
	ctx.lr = 0x83289608;
	sub_8222CED0(ctx, base);
	// 83289608: 39600003  li r11, 3
	ctx.r[11].s64 = 3;
	// 8328960C: 7F44D378  mr r4, r26
	ctx.r[4].u64 = ctx.r[26].u64;
	// 83289610: 917F0244  stw r11, 0x244(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(580 as u32), ctx.r[11].u32 ) };
	// 83289614: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83289618: 387F0248  addi r3, r31, 0x248
	ctx.r[3].s64 = ctx.r[31].s64 + 584;
	// 8328961C: 4AFA38B5  bl 0x8222ced0
	ctx.lr = 0x83289620;
	sub_8222CED0(ctx, base);
	// 83289620: 3C80820F  lis r4, -0x7df1
	ctx.r[4].s64 = -2112946176;
	// 83289624: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83289628: 388461A0  addi r4, r4, 0x61a0
	ctx.r[4].s64 = ctx.r[4].s64 + 24992;
	// 8328962C: 387F024C  addi r3, r31, 0x24c
	ctx.r[3].s64 = ctx.r[31].s64 + 588;
	// 83289630: 4AFA38A1  bl 0x8222ced0
	ctx.lr = 0x83289634;
	sub_8222CED0(ctx, base);
	// 83289634: 3C60820F  lis r3, -0x7df1
	ctx.r[3].s64 = -2112946176;
	// 83289638: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8328963C: 38836178  addi r4, r3, 0x6178
	ctx.r[4].s64 = ctx.r[3].s64 + 24952;
	// 83289640: 387F0250  addi r3, r31, 0x250
	ctx.r[3].s64 = ctx.r[31].s64 + 592;
	// 83289644: 4AFA388D  bl 0x8222ced0
	ctx.lr = 0x83289648;
	sub_8222CED0(ctx, base);
	// 83289648: 39600003  li r11, 3
	ctx.r[11].s64 = 3;
	// 8328964C: 7F44D378  mr r4, r26
	ctx.r[4].u64 = ctx.r[26].u64;
	// 83289650: 917F0254  stw r11, 0x254(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(596 as u32), ctx.r[11].u32 ) };
	// 83289654: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83289658: 387F0258  addi r3, r31, 0x258
	ctx.r[3].s64 = ctx.r[31].s64 + 600;
	// 8328965C: 4AFA3875  bl 0x8222ced0
	ctx.lr = 0x83289660;
	sub_8222CED0(ctx, base);
	// 83289660: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 83289664: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83289668: 388B6160  addi r4, r11, 0x6160
	ctx.r[4].s64 = ctx.r[11].s64 + 24928;
	// 8328966C: 387F025C  addi r3, r31, 0x25c
	ctx.r[3].s64 = ctx.r[31].s64 + 604;
	// 83289670: 4AFA3861  bl 0x8222ced0
	ctx.lr = 0x83289674;
	sub_8222CED0(ctx, base);
	// 83289674: 3D40820F  lis r10, -0x7df1
	ctx.r[10].s64 = -2112946176;
	// 83289678: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8328967C: 388A6134  addi r4, r10, 0x6134
	ctx.r[4].s64 = ctx.r[10].s64 + 24884;
	// 83289680: 387F0260  addi r3, r31, 0x260
	ctx.r[3].s64 = ctx.r[31].s64 + 608;
	// 83289684: 4AFA384D  bl 0x8222ced0
	ctx.lr = 0x83289688;
	sub_8222CED0(ctx, base);
	// 83289688: 39600003  li r11, 3
	ctx.r[11].s64 = 3;
	// 8328968C: 7F44D378  mr r4, r26
	ctx.r[4].u64 = ctx.r[26].u64;
	// 83289690: 917F0264  stw r11, 0x264(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(612 as u32), ctx.r[11].u32 ) };
	// 83289694: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83289698: 387F0268  addi r3, r31, 0x268
	ctx.r[3].s64 = ctx.r[31].s64 + 616;
	// 8328969C: 4AFA3835  bl 0x8222ced0
	ctx.lr = 0x832896A0;
	sub_8222CED0(ctx, base);
	// 832896A0: 3D20820F  lis r9, -0x7df1
	ctx.r[9].s64 = -2112946176;
	// 832896A4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832896A8: 38896118  addi r4, r9, 0x6118
	ctx.r[4].s64 = ctx.r[9].s64 + 24856;
	// 832896AC: 387F026C  addi r3, r31, 0x26c
	ctx.r[3].s64 = ctx.r[31].s64 + 620;
	// 832896B0: 4AFA3821  bl 0x8222ced0
	ctx.lr = 0x832896B4;
	sub_8222CED0(ctx, base);
	// 832896B4: 3D00820F  lis r8, -0x7df1
	ctx.r[8].s64 = -2112946176;
	// 832896B8: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832896BC: 388860F4  addi r4, r8, 0x60f4
	ctx.r[4].s64 = ctx.r[8].s64 + 24820;
	// 832896C0: 387F0270  addi r3, r31, 0x270
	ctx.r[3].s64 = ctx.r[31].s64 + 624;
	// 832896C4: 4AFA380D  bl 0x8222ced0
	ctx.lr = 0x832896C8;
	sub_8222CED0(ctx, base);
	// 832896C8: 39600003  li r11, 3
	ctx.r[11].s64 = 3;
	// 832896CC: 917F0274  stw r11, 0x274(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(628 as u32), ctx.r[11].u32 ) };
	// 832896D0: 7F44D378  mr r4, r26
	ctx.r[4].u64 = ctx.r[26].u64;
	// 832896D4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832896D8: 387F0278  addi r3, r31, 0x278
	ctx.r[3].s64 = ctx.r[31].s64 + 632;
	// 832896DC: 4AFA37F5  bl 0x8222ced0
	ctx.lr = 0x832896E0;
	sub_8222CED0(ctx, base);
	// 832896E0: 3CE0820F  lis r7, -0x7df1
	ctx.r[7].s64 = -2112946176;
	// 832896E4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832896E8: 388760DC  addi r4, r7, 0x60dc
	ctx.r[4].s64 = ctx.r[7].s64 + 24796;
	// 832896EC: 387F027C  addi r3, r31, 0x27c
	ctx.r[3].s64 = ctx.r[31].s64 + 636;
	// 832896F0: 4AFA37E1  bl 0x8222ced0
	ctx.lr = 0x832896F4;
	sub_8222CED0(ctx, base);
	// 832896F4: 3CC0820F  lis r6, -0x7df1
	ctx.r[6].s64 = -2112946176;
	// 832896F8: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832896FC: 388660B0  addi r4, r6, 0x60b0
	ctx.r[4].s64 = ctx.r[6].s64 + 24752;
	// 83289700: 387F0280  addi r3, r31, 0x280
	ctx.r[3].s64 = ctx.r[31].s64 + 640;
	// 83289704: 4AFA37CD  bl 0x8222ced0
	ctx.lr = 0x83289708;
	sub_8222CED0(ctx, base);
	// 83289708: 39600003  li r11, 3
	ctx.r[11].s64 = 3;
	// 8328970C: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 83289710: 917F0284  stw r11, 0x284(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(644 as u32), ctx.r[11].u32 ) };
	// 83289714: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83289718: 387F0288  addi r3, r31, 0x288
	ctx.r[3].s64 = ctx.r[31].s64 + 648;
	// 8328971C: 4AFA37B5  bl 0x8222ced0
	ctx.lr = 0x83289720;
	sub_8222CED0(ctx, base);
	// 83289720: 3C80820F  lis r4, -0x7df1
	ctx.r[4].s64 = -2112946176;
	// 83289724: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83289728: 38846094  addi r4, r4, 0x6094
	ctx.r[4].s64 = ctx.r[4].s64 + 24724;
	// 8328972C: 387F028C  addi r3, r31, 0x28c
	ctx.r[3].s64 = ctx.r[31].s64 + 652;
	// 83289730: 4AFA37A1  bl 0x8222ced0
	ctx.lr = 0x83289734;
	sub_8222CED0(ctx, base);
	// 83289734: 3C60820F  lis r3, -0x7df1
	ctx.r[3].s64 = -2112946176;
	// 83289738: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8328973C: 3883606C  addi r4, r3, 0x606c
	ctx.r[4].s64 = ctx.r[3].s64 + 24684;
	// 83289740: 387F0290  addi r3, r31, 0x290
	ctx.r[3].s64 = ctx.r[31].s64 + 656;
	// 83289744: 4AFA378D  bl 0x8222ced0
	ctx.lr = 0x83289748;
	sub_8222CED0(ctx, base);
	// 83289748: 39600003  li r11, 3
	ctx.r[11].s64 = 3;
	// 8328974C: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 83289750: 917F0294  stw r11, 0x294(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(660 as u32), ctx.r[11].u32 ) };
	// 83289754: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83289758: 387F0298  addi r3, r31, 0x298
	ctx.r[3].s64 = ctx.r[31].s64 + 664;
	// 8328975C: 4AFA3775  bl 0x8222ced0
	ctx.lr = 0x83289760;
	sub_8222CED0(ctx, base);
	// 83289760: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 83289764: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83289768: 388B6054  addi r4, r11, 0x6054
	ctx.r[4].s64 = ctx.r[11].s64 + 24660;
	// 8328976C: 387F029C  addi r3, r31, 0x29c
	ctx.r[3].s64 = ctx.r[31].s64 + 668;
	// 83289770: 4AFA3761  bl 0x8222ced0
	ctx.lr = 0x83289774;
	sub_8222CED0(ctx, base);
	// 83289774: 3D40820F  lis r10, -0x7df1
	ctx.r[10].s64 = -2112946176;
	// 83289778: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8328977C: 388A602C  addi r4, r10, 0x602c
	ctx.r[4].s64 = ctx.r[10].s64 + 24620;
	// 83289780: 387F02A0  addi r3, r31, 0x2a0
	ctx.r[3].s64 = ctx.r[31].s64 + 672;
	// 83289784: 4AFA374D  bl 0x8222ced0
	ctx.lr = 0x83289788;
	sub_8222CED0(ctx, base);
	// 83289788: 39600003  li r11, 3
	ctx.r[11].s64 = 3;
	// 8328978C: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 83289790: 917F02A4  stw r11, 0x2a4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(676 as u32), ctx.r[11].u32 ) };
	// 83289794: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83289798: 387F02A8  addi r3, r31, 0x2a8
	ctx.r[3].s64 = ctx.r[31].s64 + 680;
	// 8328979C: 4AFA3735  bl 0x8222ced0
	ctx.lr = 0x832897A0;
	sub_8222CED0(ctx, base);
	// 832897A0: 3D20820F  lis r9, -0x7df1
	ctx.r[9].s64 = -2112946176;
	// 832897A4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832897A8: 38896014  addi r4, r9, 0x6014
	ctx.r[4].s64 = ctx.r[9].s64 + 24596;
	// 832897AC: 387F02AC  addi r3, r31, 0x2ac
	ctx.r[3].s64 = ctx.r[31].s64 + 684;
	// 832897B0: 4AFA3721  bl 0x8222ced0
	ctx.lr = 0x832897B4;
	sub_8222CED0(ctx, base);
	// 832897B4: 3D00820F  lis r8, -0x7df1
	ctx.r[8].s64 = -2112946176;
	// 832897B8: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832897BC: 38885FEC  addi r4, r8, 0x5fec
	ctx.r[4].s64 = ctx.r[8].s64 + 24556;
	// 832897C0: 387F02B0  addi r3, r31, 0x2b0
	ctx.r[3].s64 = ctx.r[31].s64 + 688;
	// 832897C4: 4AFA370D  bl 0x8222ced0
	ctx.lr = 0x832897C8;
	sub_8222CED0(ctx, base);
	// 832897C8: 39600003  li r11, 3
	ctx.r[11].s64 = 3;
	// 832897CC: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 832897D0: 917F02B4  stw r11, 0x2b4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(692 as u32), ctx.r[11].u32 ) };
	// 832897D4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832897D8: 387F02B8  addi r3, r31, 0x2b8
	ctx.r[3].s64 = ctx.r[31].s64 + 696;
	// 832897DC: 4AFA36F5  bl 0x8222ced0
	ctx.lr = 0x832897E0;
	sub_8222CED0(ctx, base);
	// 832897E0: 3CE0820F  lis r7, -0x7df1
	ctx.r[7].s64 = -2112946176;
	// 832897E4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832897E8: 38875FDC  addi r4, r7, 0x5fdc
	ctx.r[4].s64 = ctx.r[7].s64 + 24540;
	// 832897EC: 387F02BC  addi r3, r31, 0x2bc
	ctx.r[3].s64 = ctx.r[31].s64 + 700;
	// 832897F0: 4AFA36E1  bl 0x8222ced0
	ctx.lr = 0x832897F4;
	sub_8222CED0(ctx, base);
	// 832897F4: 3CC0820F  lis r6, -0x7df1
	ctx.r[6].s64 = -2112946176;
	// 832897F8: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832897FC: 38865FC4  addi r4, r6, 0x5fc4
	ctx.r[4].s64 = ctx.r[6].s64 + 24516;
	// 83289800: 387F02C0  addi r3, r31, 0x2c0
	ctx.r[3].s64 = ctx.r[31].s64 + 704;
	// 83289804: 4AFA36CD  bl 0x8222ced0
	ctx.lr = 0x83289808;
	sub_8222CED0(ctx, base);
	// 83289808: 39600003  li r11, 3
	ctx.r[11].s64 = 3;
	// 8328980C: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 83289810: 917F02C4  stw r11, 0x2c4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(708 as u32), ctx.r[11].u32 ) };
	// 83289814: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83289818: 387F02C8  addi r3, r31, 0x2c8
	ctx.r[3].s64 = ctx.r[31].s64 + 712;
	// 8328981C: 4AFA36B5  bl 0x8222ced0
	ctx.lr = 0x83289820;
	sub_8222CED0(ctx, base);
	// 83289820: 3C80820F  lis r4, -0x7df1
	ctx.r[4].s64 = -2112946176;
	// 83289824: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83289828: 38845FB8  addi r4, r4, 0x5fb8
	ctx.r[4].s64 = ctx.r[4].s64 + 24504;
	// 8328982C: 387F02CC  addi r3, r31, 0x2cc
	ctx.r[3].s64 = ctx.r[31].s64 + 716;
	// 83289830: 4AFA36A1  bl 0x8222ced0
	ctx.lr = 0x83289834;
	sub_8222CED0(ctx, base);
	// 83289834: 3C60820F  lis r3, -0x7df1
	ctx.r[3].s64 = -2112946176;
	// 83289838: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8328983C: 38835F90  addi r4, r3, 0x5f90
	ctx.r[4].s64 = ctx.r[3].s64 + 24464;
	// 83289840: 387F02D0  addi r3, r31, 0x2d0
	ctx.r[3].s64 = ctx.r[31].s64 + 720;
	// 83289844: 4AFA368D  bl 0x8222ced0
	ctx.lr = 0x83289848;
	sub_8222CED0(ctx, base);
	// 83289848: 39600003  li r11, 3
	ctx.r[11].s64 = 3;
	// 8328984C: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 83289850: 917F02D4  stw r11, 0x2d4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(724 as u32), ctx.r[11].u32 ) };
	// 83289854: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83289858: 387F02D8  addi r3, r31, 0x2d8
	ctx.r[3].s64 = ctx.r[31].s64 + 728;
	// 8328985C: 4AFA3675  bl 0x8222ced0
	ctx.lr = 0x83289860;
	sub_8222CED0(ctx, base);
	// 83289860: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 83289864: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83289868: 388B5F7C  addi r4, r11, 0x5f7c
	ctx.r[4].s64 = ctx.r[11].s64 + 24444;
	// 8328986C: 387F02DC  addi r3, r31, 0x2dc
	ctx.r[3].s64 = ctx.r[31].s64 + 732;
	// 83289870: 4AFA3661  bl 0x8222ced0
	ctx.lr = 0x83289874;
	sub_8222CED0(ctx, base);
	// 83289874: 3D40820F  lis r10, -0x7df1
	ctx.r[10].s64 = -2112946176;
	// 83289878: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8328987C: 388A5F64  addi r4, r10, 0x5f64
	ctx.r[4].s64 = ctx.r[10].s64 + 24420;
	// 83289880: 387F02E0  addi r3, r31, 0x2e0
	ctx.r[3].s64 = ctx.r[31].s64 + 736;
	// 83289884: 4AFA364D  bl 0x8222ced0
	ctx.lr = 0x83289888;
	sub_8222CED0(ctx, base);
	// 83289888: 39600003  li r11, 3
	ctx.r[11].s64 = 3;
	// 8328988C: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 83289890: 917F02E4  stw r11, 0x2e4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(740 as u32), ctx.r[11].u32 ) };
	// 83289894: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83289898: 387F02E8  addi r3, r31, 0x2e8
	ctx.r[3].s64 = ctx.r[31].s64 + 744;
	// 8328989C: 4AFA3635  bl 0x8222ced0
	ctx.lr = 0x832898A0;
	sub_8222CED0(ctx, base);
	// 832898A0: 3D20820B  lis r9, -0x7df5
	ctx.r[9].s64 = -2113208320;
	// 832898A4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832898A8: 3B89ABE0  addi r28, r9, -0x5420
	ctx.r[28].s64 = ctx.r[9].s64 + -21536;
	// 832898AC: 387F02EC  addi r3, r31, 0x2ec
	ctx.r[3].s64 = ctx.r[31].s64 + 748;
	// 832898B0: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 832898B4: 4AFA361D  bl 0x8222ced0
	ctx.lr = 0x832898B8;
	sub_8222CED0(ctx, base);
	// 832898B8: 3D00820F  lis r8, -0x7df1
	ctx.r[8].s64 = -2112946176;
	// 832898BC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832898C0: 38885F3C  addi r4, r8, 0x5f3c
	ctx.r[4].s64 = ctx.r[8].s64 + 24380;
	// 832898C4: 387F02F0  addi r3, r31, 0x2f0
	ctx.r[3].s64 = ctx.r[31].s64 + 752;
	// 832898C8: 4AFA3609  bl 0x8222ced0
	ctx.lr = 0x832898CC;
	sub_8222CED0(ctx, base);
	// 832898CC: 39600003  li r11, 3
	ctx.r[11].s64 = 3;
	// 832898D0: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 832898D4: 917F02F4  stw r11, 0x2f4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(756 as u32), ctx.r[11].u32 ) };
	// 832898D8: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832898DC: 387F02F8  addi r3, r31, 0x2f8
	ctx.r[3].s64 = ctx.r[31].s64 + 760;
	// 832898E0: 4AFA35F1  bl 0x8222ced0
	ctx.lr = 0x832898E4;
	sub_8222CED0(ctx, base);
	// 832898E4: 3CE0820F  lis r7, -0x7df1
	ctx.r[7].s64 = -2112946176;
	// 832898E8: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832898EC: 38875F28  addi r4, r7, 0x5f28
	ctx.r[4].s64 = ctx.r[7].s64 + 24360;
	// 832898F0: 387F02FC  addi r3, r31, 0x2fc
	ctx.r[3].s64 = ctx.r[31].s64 + 764;
	// 832898F4: 4AFA35DD  bl 0x8222ced0
	ctx.lr = 0x832898F8;
	sub_8222CED0(ctx, base);
	// 832898F8: 3CC0820F  lis r6, -0x7df1
	ctx.r[6].s64 = -2112946176;
	// 832898FC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83289900: 38865F08  addi r4, r6, 0x5f08
	ctx.r[4].s64 = ctx.r[6].s64 + 24328;
	// 83289904: 387F0300  addi r3, r31, 0x300
	ctx.r[3].s64 = ctx.r[31].s64 + 768;
	// 83289908: 4AFA35C9  bl 0x8222ced0
	ctx.lr = 0x8328990C;
	sub_8222CED0(ctx, base);
	// 8328990C: 39600003  li r11, 3
	ctx.r[11].s64 = 3;
	// 83289910: 3C80820F  lis r4, -0x7df1
	ctx.r[4].s64 = -2112946176;
	// 83289914: 917F0304  stw r11, 0x304(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(772 as u32), ctx.r[11].u32 ) };
	// 83289918: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8328991C: 3B645EF0  addi r27, r4, 0x5ef0
	ctx.r[27].s64 = ctx.r[4].s64 + 24304;
	// 83289920: 387F0308  addi r3, r31, 0x308
	ctx.r[3].s64 = ctx.r[31].s64 + 776;
	// 83289924: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 83289928: 4AFA35A9  bl 0x8222ced0
	ctx.lr = 0x8328992C;
	sub_8222CED0(ctx, base);
	// 8328992C: 3C60820F  lis r3, -0x7df1
	ctx.r[3].s64 = -2112946176;
	// 83289930: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83289934: 38835EDC  addi r4, r3, 0x5edc
	ctx.r[4].s64 = ctx.r[3].s64 + 24284;
	// 83289938: 387F030C  addi r3, r31, 0x30c
	ctx.r[3].s64 = ctx.r[31].s64 + 780;
	// 8328993C: 4AFA3595  bl 0x8222ced0
	ctx.lr = 0x83289940;
	sub_8222CED0(ctx, base);
	// 83289940: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 83289944: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83289948: 388B5EB0  addi r4, r11, 0x5eb0
	ctx.r[4].s64 = ctx.r[11].s64 + 24240;
	// 8328994C: 387F0310  addi r3, r31, 0x310
	ctx.r[3].s64 = ctx.r[31].s64 + 784;
	// 83289950: 4AFA3581  bl 0x8222ced0
	ctx.lr = 0x83289954;
	sub_8222CED0(ctx, base);
	// 83289954: 39600003  li r11, 3
	ctx.r[11].s64 = 3;
	// 83289958: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 8328995C: 917F0314  stw r11, 0x314(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(788 as u32), ctx.r[11].u32 ) };
	// 83289960: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83289964: 387F0318  addi r3, r31, 0x318
	ctx.r[3].s64 = ctx.r[31].s64 + 792;
	// 83289968: 4AFA3569  bl 0x8222ced0
	ctx.lr = 0x8328996C;
	sub_8222CED0(ctx, base);
	// 8328996C: 3D40820F  lis r10, -0x7df1
	ctx.r[10].s64 = -2112946176;
	// 83289970: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83289974: 388A5E94  addi r4, r10, 0x5e94
	ctx.r[4].s64 = ctx.r[10].s64 + 24212;
	// 83289978: 387F031C  addi r3, r31, 0x31c
	ctx.r[3].s64 = ctx.r[31].s64 + 796;
	// 8328997C: 4AFA3555  bl 0x8222ced0
	ctx.lr = 0x83289980;
	sub_8222CED0(ctx, base);
	// 83289980: 3D20820F  lis r9, -0x7df1
	ctx.r[9].s64 = -2112946176;
	// 83289984: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83289988: 38895E64  addi r4, r9, 0x5e64
	ctx.r[4].s64 = ctx.r[9].s64 + 24164;
	// 8328998C: 387F0320  addi r3, r31, 0x320
	ctx.r[3].s64 = ctx.r[31].s64 + 800;
	// 83289990: 4AFA3541  bl 0x8222ced0
	ctx.lr = 0x83289994;
	sub_8222CED0(ctx, base);
	// 83289994: 39600003  li r11, 3
	ctx.r[11].s64 = 3;
	// 83289998: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 8328999C: 917F0324  stw r11, 0x324(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(804 as u32), ctx.r[11].u32 ) };
	// 832899A0: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832899A4: 387F0328  addi r3, r31, 0x328
	ctx.r[3].s64 = ctx.r[31].s64 + 808;
	// 832899A8: 4AFA3529  bl 0x8222ced0
	ctx.lr = 0x832899AC;
	sub_8222CED0(ctx, base);
	// 832899AC: 3D00820F  lis r8, -0x7df1
	ctx.r[8].s64 = -2112946176;
	// 832899B0: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832899B4: 38885E44  addi r4, r8, 0x5e44
	ctx.r[4].s64 = ctx.r[8].s64 + 24132;
	// 832899B8: 387F032C  addi r3, r31, 0x32c
	ctx.r[3].s64 = ctx.r[31].s64 + 812;
	// 832899BC: 4AFA3515  bl 0x8222ced0
	ctx.lr = 0x832899C0;
	sub_8222CED0(ctx, base);
	// 832899C0: 3CE0820F  lis r7, -0x7df1
	ctx.r[7].s64 = -2112946176;
	// 832899C4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832899C8: 38875E20  addi r4, r7, 0x5e20
	ctx.r[4].s64 = ctx.r[7].s64 + 24096;
	// 832899CC: 387F0330  addi r3, r31, 0x330
	ctx.r[3].s64 = ctx.r[31].s64 + 816;
	// 832899D0: 4AFA3501  bl 0x8222ced0
	ctx.lr = 0x832899D4;
	sub_8222CED0(ctx, base);
	// 832899D4: 39600003  li r11, 3
	ctx.r[11].s64 = 3;
	// 832899D8: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 832899DC: 917F0334  stw r11, 0x334(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(820 as u32), ctx.r[11].u32 ) };
	// 832899E0: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832899E4: 387F0338  addi r3, r31, 0x338
	ctx.r[3].s64 = ctx.r[31].s64 + 824;
	// 832899E8: 4AFA34E9  bl 0x8222ced0
	ctx.lr = 0x832899EC;
	sub_8222CED0(ctx, base);
	// 832899EC: 3CC0820F  lis r6, -0x7df1
	ctx.r[6].s64 = -2112946176;
	// 832899F0: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832899F4: 38865E0C  addi r4, r6, 0x5e0c
	ctx.r[4].s64 = ctx.r[6].s64 + 24076;
	// 832899F8: 387F033C  addi r3, r31, 0x33c
	ctx.r[3].s64 = ctx.r[31].s64 + 828;
	// 832899FC: 4AFA34D5  bl 0x8222ced0
	ctx.lr = 0x83289A00;
	sub_8222CED0(ctx, base);
	// 83289A00: 3C80820F  lis r4, -0x7df1
	ctx.r[4].s64 = -2112946176;
	// 83289A04: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83289A08: 38845DE4  addi r4, r4, 0x5de4
	ctx.r[4].s64 = ctx.r[4].s64 + 24036;
	// 83289A0C: 387F0340  addi r3, r31, 0x340
	ctx.r[3].s64 = ctx.r[31].s64 + 832;
	// 83289A10: 4AFA34C1  bl 0x8222ced0
	ctx.lr = 0x83289A14;
	sub_8222CED0(ctx, base);
	// 83289A14: 39600003  li r11, 3
	ctx.r[11].s64 = 3;
	// 83289A18: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 83289A1C: 917F0344  stw r11, 0x344(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(836 as u32), ctx.r[11].u32 ) };
	// 83289A20: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83289A24: 387F0348  addi r3, r31, 0x348
	ctx.r[3].s64 = ctx.r[31].s64 + 840;
	// 83289A28: 4AFA34A9  bl 0x8222ced0
	ctx.lr = 0x83289A2C;
	sub_8222CED0(ctx, base);
	// 83289A2C: 3C60820F  lis r3, -0x7df1
	ctx.r[3].s64 = -2112946176;
	// 83289A30: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83289A34: 38835DCC  addi r4, r3, 0x5dcc
	ctx.r[4].s64 = ctx.r[3].s64 + 24012;
	// 83289A38: 387F034C  addi r3, r31, 0x34c
	ctx.r[3].s64 = ctx.r[31].s64 + 844;
	// 83289A3C: 4AFA3495  bl 0x8222ced0
	ctx.lr = 0x83289A40;
	sub_8222CED0(ctx, base);
	// 83289A40: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 83289A44: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83289A48: 388B5DB4  addi r4, r11, 0x5db4
	ctx.r[4].s64 = ctx.r[11].s64 + 23988;
	// 83289A4C: 387F0350  addi r3, r31, 0x350
	ctx.r[3].s64 = ctx.r[31].s64 + 848;
	// 83289A50: 4AFA3481  bl 0x8222ced0
	ctx.lr = 0x83289A54;
	sub_8222CED0(ctx, base);
	// 83289A54: 39600003  li r11, 3
	ctx.r[11].s64 = 3;
	// 83289A58: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 83289A5C: 917F0354  stw r11, 0x354(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(852 as u32), ctx.r[11].u32 ) };
	// 83289A60: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83289A64: 387F0358  addi r3, r31, 0x358
	ctx.r[3].s64 = ctx.r[31].s64 + 856;
	// 83289A68: 4AFA3469  bl 0x8222ced0
	ctx.lr = 0x83289A6C;
	sub_8222CED0(ctx, base);
	// 83289A6C: 3D40820F  lis r10, -0x7df1
	ctx.r[10].s64 = -2112946176;
	// 83289A70: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83289A74: 388A5DA0  addi r4, r10, 0x5da0
	ctx.r[4].s64 = ctx.r[10].s64 + 23968;
	// 83289A78: 387F035C  addi r3, r31, 0x35c
	ctx.r[3].s64 = ctx.r[31].s64 + 860;
	// 83289A7C: 4AFA3455  bl 0x8222ced0
	ctx.lr = 0x83289A80;
	sub_8222CED0(ctx, base);
	// 83289A80: 3D20820F  lis r9, -0x7df1
	ctx.r[9].s64 = -2112946176;
	// 83289A84: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83289A88: 38895D88  addi r4, r9, 0x5d88
	ctx.r[4].s64 = ctx.r[9].s64 + 23944;
	// 83289A8C: 387F0360  addi r3, r31, 0x360
	ctx.r[3].s64 = ctx.r[31].s64 + 864;
	// 83289A90: 4AFA3441  bl 0x8222ced0
	ctx.lr = 0x83289A94;
	sub_8222CED0(ctx, base);
	// 83289A94: 39600003  li r11, 3
	ctx.r[11].s64 = 3;
	// 83289A98: 917F0364  stw r11, 0x364(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(868 as u32), ctx.r[11].u32 ) };
	// 83289A9C: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 83289AA0: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83289AA4: 387F0368  addi r3, r31, 0x368
	ctx.r[3].s64 = ctx.r[31].s64 + 872;
	// 83289AA8: 4AFA3429  bl 0x8222ced0
	ctx.lr = 0x83289AAC;
	sub_8222CED0(ctx, base);
	// 83289AAC: 3D00820F  lis r8, -0x7df1
	ctx.r[8].s64 = -2112946176;
	// 83289AB0: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83289AB4: 38885D74  addi r4, r8, 0x5d74
	ctx.r[4].s64 = ctx.r[8].s64 + 23924;
	// 83289AB8: 387F036C  addi r3, r31, 0x36c
	ctx.r[3].s64 = ctx.r[31].s64 + 876;
	// 83289ABC: 4AFA3415  bl 0x8222ced0
	ctx.lr = 0x83289AC0;
	sub_8222CED0(ctx, base);
	// 83289AC0: 3CE0820F  lis r7, -0x7df1
	ctx.r[7].s64 = -2112946176;
	// 83289AC4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83289AC8: 38875D5C  addi r4, r7, 0x5d5c
	ctx.r[4].s64 = ctx.r[7].s64 + 23900;
	// 83289ACC: 387F0370  addi r3, r31, 0x370
	ctx.r[3].s64 = ctx.r[31].s64 + 880;
	// 83289AD0: 4AFA3401  bl 0x8222ced0
	ctx.lr = 0x83289AD4;
	sub_8222CED0(ctx, base);
	// 83289AD4: 39600003  li r11, 3
	ctx.r[11].s64 = 3;
	// 83289AD8: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 83289ADC: 917F0374  stw r11, 0x374(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(884 as u32), ctx.r[11].u32 ) };
	// 83289AE0: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83289AE4: 387F0378  addi r3, r31, 0x378
	ctx.r[3].s64 = ctx.r[31].s64 + 888;
	// 83289AE8: 4AFA33E9  bl 0x8222ced0
	ctx.lr = 0x83289AEC;
	sub_8222CED0(ctx, base);
	// 83289AEC: 3CC0820F  lis r6, -0x7df1
	ctx.r[6].s64 = -2112946176;
	// 83289AF0: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83289AF4: 38865D44  addi r4, r6, 0x5d44
	ctx.r[4].s64 = ctx.r[6].s64 + 23876;
	// 83289AF8: 387F037C  addi r3, r31, 0x37c
	ctx.r[3].s64 = ctx.r[31].s64 + 892;
	// 83289AFC: 4AFA33D5  bl 0x8222ced0
	ctx.lr = 0x83289B00;
	sub_8222CED0(ctx, base);
	// 83289B00: 3C80820F  lis r4, -0x7df1
	ctx.r[4].s64 = -2112946176;
	// 83289B04: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83289B08: 38845D2C  addi r4, r4, 0x5d2c
	ctx.r[4].s64 = ctx.r[4].s64 + 23852;
	// 83289B0C: 387F0380  addi r3, r31, 0x380
	ctx.r[3].s64 = ctx.r[31].s64 + 896;
	// 83289B10: 4AFA33C1  bl 0x8222ced0
	ctx.lr = 0x83289B14;
	sub_8222CED0(ctx, base);
	// 83289B14: 39600003  li r11, 3
	ctx.r[11].s64 = 3;
	// 83289B18: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 83289B1C: 917F0384  stw r11, 0x384(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(900 as u32), ctx.r[11].u32 ) };
	// 83289B20: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83289B24: 387F0388  addi r3, r31, 0x388
	ctx.r[3].s64 = ctx.r[31].s64 + 904;
	// 83289B28: 4AFA33A9  bl 0x8222ced0
	ctx.lr = 0x83289B2C;
	sub_8222CED0(ctx, base);
	// 83289B2C: 3C60820F  lis r3, -0x7df1
	ctx.r[3].s64 = -2112946176;
	// 83289B30: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83289B34: 38835D14  addi r4, r3, 0x5d14
	ctx.r[4].s64 = ctx.r[3].s64 + 23828;
	// 83289B38: 387F038C  addi r3, r31, 0x38c
	ctx.r[3].s64 = ctx.r[31].s64 + 908;
	// 83289B3C: 4AFA3395  bl 0x8222ced0
	ctx.lr = 0x83289B40;
	sub_8222CED0(ctx, base);
	// 83289B40: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 83289B44: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83289B48: 388B5CFC  addi r4, r11, 0x5cfc
	ctx.r[4].s64 = ctx.r[11].s64 + 23804;
	// 83289B4C: 387F0390  addi r3, r31, 0x390
	ctx.r[3].s64 = ctx.r[31].s64 + 912;
	// 83289B50: 4AFA3381  bl 0x8222ced0
	ctx.lr = 0x83289B54;
	sub_8222CED0(ctx, base);
	// 83289B54: 39600003  li r11, 3
	ctx.r[11].s64 = 3;
	// 83289B58: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 83289B5C: 917F0394  stw r11, 0x394(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(916 as u32), ctx.r[11].u32 ) };
	// 83289B60: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83289B64: 387F0398  addi r3, r31, 0x398
	ctx.r[3].s64 = ctx.r[31].s64 + 920;
	// 83289B68: 4AFA3369  bl 0x8222ced0
	ctx.lr = 0x83289B6C;
	sub_8222CED0(ctx, base);
	// 83289B6C: 3D40820F  lis r10, -0x7df1
	ctx.r[10].s64 = -2112946176;
	// 83289B70: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83289B74: 388A5CE8  addi r4, r10, 0x5ce8
	ctx.r[4].s64 = ctx.r[10].s64 + 23784;
	// 83289B78: 387F039C  addi r3, r31, 0x39c
	ctx.r[3].s64 = ctx.r[31].s64 + 924;
	// 83289B7C: 4AFA3355  bl 0x8222ced0
	ctx.lr = 0x83289B80;
	sub_8222CED0(ctx, base);
	// 83289B80: 3D20820F  lis r9, -0x7df1
	ctx.r[9].s64 = -2112946176;
	// 83289B84: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83289B88: 38895CD0  addi r4, r9, 0x5cd0
	ctx.r[4].s64 = ctx.r[9].s64 + 23760;
	// 83289B8C: 387F03A0  addi r3, r31, 0x3a0
	ctx.r[3].s64 = ctx.r[31].s64 + 928;
	// 83289B90: 4AFA3341  bl 0x8222ced0
	ctx.lr = 0x83289B94;
	sub_8222CED0(ctx, base);
	// 83289B94: 39600003  li r11, 3
	ctx.r[11].s64 = 3;
	// 83289B98: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 83289B9C: 917F03A4  stw r11, 0x3a4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(932 as u32), ctx.r[11].u32 ) };
	// 83289BA0: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83289BA4: 387F03A8  addi r3, r31, 0x3a8
	ctx.r[3].s64 = ctx.r[31].s64 + 936;
	// 83289BA8: 4AFA3329  bl 0x8222ced0
	ctx.lr = 0x83289BAC;
	sub_8222CED0(ctx, base);
	// 83289BAC: 3D00820E  lis r8, -0x7df2
	ctx.r[8].s64 = -2113011712;
	// 83289BB0: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83289BB4: 38889DD0  addi r4, r8, -0x6230
	ctx.r[4].s64 = ctx.r[8].s64 + -25136;
	// 83289BB8: 387F03AC  addi r3, r31, 0x3ac
	ctx.r[3].s64 = ctx.r[31].s64 + 940;
	// 83289BBC: 4AFA3315  bl 0x8222ced0
	ctx.lr = 0x83289BC0;
	sub_8222CED0(ctx, base);
	// 83289BC0: 3CE0820F  lis r7, -0x7df1
	ctx.r[7].s64 = -2112946176;
	// 83289BC4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83289BC8: 38875CBC  addi r4, r7, 0x5cbc
	ctx.r[4].s64 = ctx.r[7].s64 + 23740;
	// 83289BCC: 387F03B0  addi r3, r31, 0x3b0
	ctx.r[3].s64 = ctx.r[31].s64 + 944;
	// 83289BD0: 4AFA3301  bl 0x8222ced0
	ctx.lr = 0x83289BD4;
	sub_8222CED0(ctx, base);
	// 83289BD4: 39600003  li r11, 3
	ctx.r[11].s64 = 3;
	// 83289BD8: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 83289BDC: 917F03B4  stw r11, 0x3b4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(948 as u32), ctx.r[11].u32 ) };
	// 83289BE0: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83289BE4: 387F03B8  addi r3, r31, 0x3b8
	ctx.r[3].s64 = ctx.r[31].s64 + 952;
	// 83289BE8: 4AFA32E9  bl 0x8222ced0
	ctx.lr = 0x83289BEC;
	sub_8222CED0(ctx, base);
	// 83289BEC: 3CC0820F  lis r6, -0x7df1
	ctx.r[6].s64 = -2112946176;
	// 83289BF0: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83289BF4: 38865CA8  addi r4, r6, 0x5ca8
	ctx.r[4].s64 = ctx.r[6].s64 + 23720;
	// 83289BF8: 387F03BC  addi r3, r31, 0x3bc
	ctx.r[3].s64 = ctx.r[31].s64 + 956;
	// 83289BFC: 4AFA32D5  bl 0x8222ced0
	ctx.lr = 0x83289C00;
	sub_8222CED0(ctx, base);
	// 83289C00: 3C80820F  lis r4, -0x7df1
	ctx.r[4].s64 = -2112946176;
	// 83289C04: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83289C08: 38845C94  addi r4, r4, 0x5c94
	ctx.r[4].s64 = ctx.r[4].s64 + 23700;
	// 83289C0C: 387F03C0  addi r3, r31, 0x3c0
	ctx.r[3].s64 = ctx.r[31].s64 + 960;
	// 83289C10: 4AFA32C1  bl 0x8222ced0
	ctx.lr = 0x83289C14;
	sub_8222CED0(ctx, base);
	// 83289C14: 39600003  li r11, 3
	ctx.r[11].s64 = 3;
	// 83289C18: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 83289C1C: 917F03C4  stw r11, 0x3c4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(964 as u32), ctx.r[11].u32 ) };
	// 83289C20: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83289C24: 387F03C8  addi r3, r31, 0x3c8
	ctx.r[3].s64 = ctx.r[31].s64 + 968;
	// 83289C28: 4AFA32A9  bl 0x8222ced0
	ctx.lr = 0x83289C2C;
	sub_8222CED0(ctx, base);
	// 83289C2C: 3C60820F  lis r3, -0x7df1
	ctx.r[3].s64 = -2112946176;
	// 83289C30: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83289C34: 38835C84  addi r4, r3, 0x5c84
	ctx.r[4].s64 = ctx.r[3].s64 + 23684;
	// 83289C38: 387F03CC  addi r3, r31, 0x3cc
	ctx.r[3].s64 = ctx.r[31].s64 + 972;
	// 83289C3C: 4AFA3295  bl 0x8222ced0
	ctx.lr = 0x83289C40;
	sub_8222CED0(ctx, base);
	// 83289C40: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 83289C44: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83289C48: 388B5C6C  addi r4, r11, 0x5c6c
	ctx.r[4].s64 = ctx.r[11].s64 + 23660;
	// 83289C4C: 387F03D0  addi r3, r31, 0x3d0
	ctx.r[3].s64 = ctx.r[31].s64 + 976;
	// 83289C50: 4AFA3281  bl 0x8222ced0
	ctx.lr = 0x83289C54;
	sub_8222CED0(ctx, base);
	// 83289C54: 39600003  li r11, 3
	ctx.r[11].s64 = 3;
	// 83289C58: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 83289C5C: 917F03D4  stw r11, 0x3d4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(980 as u32), ctx.r[11].u32 ) };
	// 83289C60: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83289C64: 387F03D8  addi r3, r31, 0x3d8
	ctx.r[3].s64 = ctx.r[31].s64 + 984;
	// 83289C68: 4AFA3269  bl 0x8222ced0
	ctx.lr = 0x83289C6C;
	sub_8222CED0(ctx, base);
	// 83289C6C: 3D40820F  lis r10, -0x7df1
	ctx.r[10].s64 = -2112946176;
	// 83289C70: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83289C74: 388A5C54  addi r4, r10, 0x5c54
	ctx.r[4].s64 = ctx.r[10].s64 + 23636;
	// 83289C78: 387F03DC  addi r3, r31, 0x3dc
	ctx.r[3].s64 = ctx.r[31].s64 + 988;
	// 83289C7C: 4AFA3255  bl 0x8222ced0
	ctx.lr = 0x83289C80;
	sub_8222CED0(ctx, base);
	// 83289C80: 3D20820F  lis r9, -0x7df1
	ctx.r[9].s64 = -2112946176;
	// 83289C84: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83289C88: 38895C40  addi r4, r9, 0x5c40
	ctx.r[4].s64 = ctx.r[9].s64 + 23616;
	// 83289C8C: 387F03E0  addi r3, r31, 0x3e0
	ctx.r[3].s64 = ctx.r[31].s64 + 992;
	// 83289C90: 4AFA3241  bl 0x8222ced0
	ctx.lr = 0x83289C94;
	sub_8222CED0(ctx, base);
	// 83289C94: 39600003  li r11, 3
	ctx.r[11].s64 = 3;
	// 83289C98: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 83289C9C: 917F03E4  stw r11, 0x3e4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(996 as u32), ctx.r[11].u32 ) };
	// 83289CA0: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83289CA4: 387F03E8  addi r3, r31, 0x3e8
	ctx.r[3].s64 = ctx.r[31].s64 + 1000;
	// 83289CA8: 4AFA3229  bl 0x8222ced0
	ctx.lr = 0x83289CAC;
	sub_8222CED0(ctx, base);
	// 83289CAC: 3D00820F  lis r8, -0x7df1
	ctx.r[8].s64 = -2112946176;
	// 83289CB0: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83289CB4: 38885C2C  addi r4, r8, 0x5c2c
	ctx.r[4].s64 = ctx.r[8].s64 + 23596;
	// 83289CB8: 387F03EC  addi r3, r31, 0x3ec
	ctx.r[3].s64 = ctx.r[31].s64 + 1004;
	// 83289CBC: 4AFA3215  bl 0x8222ced0
	ctx.lr = 0x83289CC0;
	sub_8222CED0(ctx, base);
	// 83289CC0: 3CE0820F  lis r7, -0x7df1
	ctx.r[7].s64 = -2112946176;
	// 83289CC4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83289CC8: 38875C18  addi r4, r7, 0x5c18
	ctx.r[4].s64 = ctx.r[7].s64 + 23576;
	// 83289CCC: 387F03F0  addi r3, r31, 0x3f0
	ctx.r[3].s64 = ctx.r[31].s64 + 1008;
	// 83289CD0: 4AFA3201  bl 0x8222ced0
	ctx.lr = 0x83289CD4;
	sub_8222CED0(ctx, base);
	// 83289CD4: 39600003  li r11, 3
	ctx.r[11].s64 = 3;
	// 83289CD8: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 83289CDC: 917F03F4  stw r11, 0x3f4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(1012 as u32), ctx.r[11].u32 ) };
	// 83289CE0: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83289CE4: 387F03F8  addi r3, r31, 0x3f8
	ctx.r[3].s64 = ctx.r[31].s64 + 1016;
	// 83289CE8: 4AFA31E9  bl 0x8222ced0
	ctx.lr = 0x83289CEC;
	sub_8222CED0(ctx, base);
	// 83289CEC: 3CC0820F  lis r6, -0x7df1
	ctx.r[6].s64 = -2112946176;
	// 83289CF0: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83289CF4: 38865C08  addi r4, r6, 0x5c08
	ctx.r[4].s64 = ctx.r[6].s64 + 23560;
	// 83289CF8: 387F03FC  addi r3, r31, 0x3fc
	ctx.r[3].s64 = ctx.r[31].s64 + 1020;
	// 83289CFC: 4AFA31D5  bl 0x8222ced0
	ctx.lr = 0x83289D00;
	sub_8222CED0(ctx, base);
	// 83289D00: 3C80820F  lis r4, -0x7df1
	ctx.r[4].s64 = -2112946176;
	// 83289D04: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83289D08: 38845BF0  addi r4, r4, 0x5bf0
	ctx.r[4].s64 = ctx.r[4].s64 + 23536;
	// 83289D0C: 387F0400  addi r3, r31, 0x400
	ctx.r[3].s64 = ctx.r[31].s64 + 1024;
	// 83289D10: 4AFA31C1  bl 0x8222ced0
	ctx.lr = 0x83289D14;
	sub_8222CED0(ctx, base);
	// 83289D14: 39600003  li r11, 3
	ctx.r[11].s64 = 3;
	// 83289D18: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 83289D1C: 917F0404  stw r11, 0x404(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(1028 as u32), ctx.r[11].u32 ) };
	// 83289D20: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83289D24: 387F0408  addi r3, r31, 0x408
	ctx.r[3].s64 = ctx.r[31].s64 + 1032;
	// 83289D28: 4AFA31A9  bl 0x8222ced0
	ctx.lr = 0x83289D2C;
	sub_8222CED0(ctx, base);
	// 83289D2C: 3C60820F  lis r3, -0x7df1
	ctx.r[3].s64 = -2112946176;
	// 83289D30: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83289D34: 38835BD4  addi r4, r3, 0x5bd4
	ctx.r[4].s64 = ctx.r[3].s64 + 23508;
	// 83289D38: 387F040C  addi r3, r31, 0x40c
	ctx.r[3].s64 = ctx.r[31].s64 + 1036;
	// 83289D3C: 4AFA3195  bl 0x8222ced0
	ctx.lr = 0x83289D40;
	sub_8222CED0(ctx, base);
	// 83289D40: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 83289D44: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83289D48: 388B5BC4  addi r4, r11, 0x5bc4
	ctx.r[4].s64 = ctx.r[11].s64 + 23492;
	// 83289D4C: 387F0410  addi r3, r31, 0x410
	ctx.r[3].s64 = ctx.r[31].s64 + 1040;
	// 83289D50: 4AFA3181  bl 0x8222ced0
	ctx.lr = 0x83289D54;
	sub_8222CED0(ctx, base);
	// 83289D54: 39600003  li r11, 3
	ctx.r[11].s64 = 3;
	// 83289D58: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 83289D5C: 917F0414  stw r11, 0x414(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(1044 as u32), ctx.r[11].u32 ) };
	// 83289D60: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83289D64: 387F0418  addi r3, r31, 0x418
	ctx.r[3].s64 = ctx.r[31].s64 + 1048;
	// 83289D68: 4AFA3169  bl 0x8222ced0
	ctx.lr = 0x83289D6C;
	sub_8222CED0(ctx, base);
	// 83289D6C: 3D40820F  lis r10, -0x7df1
	ctx.r[10].s64 = -2112946176;
	// 83289D70: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83289D74: 388A5BB4  addi r4, r10, 0x5bb4
	ctx.r[4].s64 = ctx.r[10].s64 + 23476;
	// 83289D78: 387F041C  addi r3, r31, 0x41c
	ctx.r[3].s64 = ctx.r[31].s64 + 1052;
	// 83289D7C: 4AFA3155  bl 0x8222ced0
	ctx.lr = 0x83289D80;
	sub_8222CED0(ctx, base);
	// 83289D80: 3D20820F  lis r9, -0x7df1
	ctx.r[9].s64 = -2112946176;
	// 83289D84: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83289D88: 38895BA8  addi r4, r9, 0x5ba8
	ctx.r[4].s64 = ctx.r[9].s64 + 23464;
	// 83289D8C: 387F0420  addi r3, r31, 0x420
	ctx.r[3].s64 = ctx.r[31].s64 + 1056;
	// 83289D90: 4AFA3141  bl 0x8222ced0
	ctx.lr = 0x83289D94;
	sub_8222CED0(ctx, base);
	// 83289D94: 39600003  li r11, 3
	ctx.r[11].s64 = 3;
	// 83289D98: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 83289D9C: 917F0424  stw r11, 0x424(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(1060 as u32), ctx.r[11].u32 ) };
	// 83289DA0: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83289DA4: 387F0428  addi r3, r31, 0x428
	ctx.r[3].s64 = ctx.r[31].s64 + 1064;
	// 83289DA8: 4AFA3129  bl 0x8222ced0
	ctx.lr = 0x83289DAC;
	sub_8222CED0(ctx, base);
	// 83289DAC: 3D00820F  lis r8, -0x7df1
	ctx.r[8].s64 = -2112946176;
	// 83289DB0: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83289DB4: 38885B9C  addi r4, r8, 0x5b9c
	ctx.r[4].s64 = ctx.r[8].s64 + 23452;
	// 83289DB8: 387F042C  addi r3, r31, 0x42c
	ctx.r[3].s64 = ctx.r[31].s64 + 1068;
	// 83289DBC: 4AFA3115  bl 0x8222ced0
	ctx.lr = 0x83289DC0;
	sub_8222CED0(ctx, base);
	// 83289DC0: 3CE0820F  lis r7, -0x7df1
	ctx.r[7].s64 = -2112946176;
	// 83289DC4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83289DC8: 38875B84  addi r4, r7, 0x5b84
	ctx.r[4].s64 = ctx.r[7].s64 + 23428;
	// 83289DCC: 387F0430  addi r3, r31, 0x430
	ctx.r[3].s64 = ctx.r[31].s64 + 1072;
	// 83289DD0: 4AFA3101  bl 0x8222ced0
	ctx.lr = 0x83289DD4;
	sub_8222CED0(ctx, base);
	// 83289DD4: 39600003  li r11, 3
	ctx.r[11].s64 = 3;
	// 83289DD8: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 83289DDC: 917F0434  stw r11, 0x434(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(1076 as u32), ctx.r[11].u32 ) };
	// 83289DE0: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83289DE4: 387F0438  addi r3, r31, 0x438
	ctx.r[3].s64 = ctx.r[31].s64 + 1080;
	// 83289DE8: 4AFA30E9  bl 0x8222ced0
	ctx.lr = 0x83289DEC;
	sub_8222CED0(ctx, base);
	// 83289DEC: 3CC0820F  lis r6, -0x7df1
	ctx.r[6].s64 = -2112946176;
	// 83289DF0: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83289DF4: 38865B70  addi r4, r6, 0x5b70
	ctx.r[4].s64 = ctx.r[6].s64 + 23408;
	// 83289DF8: 387F043C  addi r3, r31, 0x43c
	ctx.r[3].s64 = ctx.r[31].s64 + 1084;
	// 83289DFC: 4AFA30D5  bl 0x8222ced0
	ctx.lr = 0x83289E00;
	sub_8222CED0(ctx, base);
	// 83289E00: 3C80820F  lis r4, -0x7df1
	ctx.r[4].s64 = -2112946176;
	// 83289E04: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83289E08: 38845B60  addi r4, r4, 0x5b60
	ctx.r[4].s64 = ctx.r[4].s64 + 23392;
	// 83289E0C: 387F0440  addi r3, r31, 0x440
	ctx.r[3].s64 = ctx.r[31].s64 + 1088;
	// 83289E10: 4AFA30C1  bl 0x8222ced0
	ctx.lr = 0x83289E14;
	sub_8222CED0(ctx, base);
	// 83289E14: 39600003  li r11, 3
	ctx.r[11].s64 = 3;
	// 83289E18: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 83289E1C: 917F0444  stw r11, 0x444(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(1092 as u32), ctx.r[11].u32 ) };
	// 83289E20: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83289E24: 387F0448  addi r3, r31, 0x448
	ctx.r[3].s64 = ctx.r[31].s64 + 1096;
	// 83289E28: 4AFA30A9  bl 0x8222ced0
	ctx.lr = 0x83289E2C;
	sub_8222CED0(ctx, base);
	// 83289E2C: 3C60820F  lis r3, -0x7df1
	ctx.r[3].s64 = -2112946176;
	// 83289E30: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83289E34: 38835B50  addi r4, r3, 0x5b50
	ctx.r[4].s64 = ctx.r[3].s64 + 23376;
	// 83289E38: 387F044C  addi r3, r31, 0x44c
	ctx.r[3].s64 = ctx.r[31].s64 + 1100;
	// 83289E3C: 4AFA3095  bl 0x8222ced0
	ctx.lr = 0x83289E40;
	sub_8222CED0(ctx, base);
	// 83289E40: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 83289E44: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83289E48: 388B5B3C  addi r4, r11, 0x5b3c
	ctx.r[4].s64 = ctx.r[11].s64 + 23356;
	// 83289E4C: 387F0450  addi r3, r31, 0x450
	ctx.r[3].s64 = ctx.r[31].s64 + 1104;
	// 83289E50: 4AFA3081  bl 0x8222ced0
	ctx.lr = 0x83289E54;
	sub_8222CED0(ctx, base);
	// 83289E54: 39600003  li r11, 3
	ctx.r[11].s64 = 3;
	// 83289E58: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 83289E5C: 917F0454  stw r11, 0x454(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(1108 as u32), ctx.r[11].u32 ) };
	// 83289E60: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83289E64: 387F0458  addi r3, r31, 0x458
	ctx.r[3].s64 = ctx.r[31].s64 + 1112;
	// 83289E68: 4AFA3069  bl 0x8222ced0
	ctx.lr = 0x83289E6C;
	sub_8222CED0(ctx, base);
	// 83289E6C: 3D40820F  lis r10, -0x7df1
	ctx.r[10].s64 = -2112946176;
	// 83289E70: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83289E74: 388A5B28  addi r4, r10, 0x5b28
	ctx.r[4].s64 = ctx.r[10].s64 + 23336;
	// 83289E78: 387F045C  addi r3, r31, 0x45c
	ctx.r[3].s64 = ctx.r[31].s64 + 1116;
	// 83289E7C: 4AFA3055  bl 0x8222ced0
	ctx.lr = 0x83289E80;
	sub_8222CED0(ctx, base);
	// 83289E80: 3D20820F  lis r9, -0x7df1
	ctx.r[9].s64 = -2112946176;
	// 83289E84: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83289E88: 38895B14  addi r4, r9, 0x5b14
	ctx.r[4].s64 = ctx.r[9].s64 + 23316;
	// 83289E8C: 387F0460  addi r3, r31, 0x460
	ctx.r[3].s64 = ctx.r[31].s64 + 1120;
	// 83289E90: 4AFA3041  bl 0x8222ced0
	ctx.lr = 0x83289E94;
	sub_8222CED0(ctx, base);
	// 83289E94: 39600003  li r11, 3
	ctx.r[11].s64 = 3;
	// 83289E98: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 83289E9C: 917F0464  stw r11, 0x464(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(1124 as u32), ctx.r[11].u32 ) };
	// 83289EA0: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83289EA4: 387F0468  addi r3, r31, 0x468
	ctx.r[3].s64 = ctx.r[31].s64 + 1128;
	// 83289EA8: 4AFA3029  bl 0x8222ced0
	ctx.lr = 0x83289EAC;
	sub_8222CED0(ctx, base);
	// 83289EAC: 3D00820F  lis r8, -0x7df1
	ctx.r[8].s64 = -2112946176;
	// 83289EB0: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83289EB4: 38885B04  addi r4, r8, 0x5b04
	ctx.r[4].s64 = ctx.r[8].s64 + 23300;
	// 83289EB8: 387F046C  addi r3, r31, 0x46c
	ctx.r[3].s64 = ctx.r[31].s64 + 1132;
	// 83289EBC: 4AFA3015  bl 0x8222ced0
	ctx.lr = 0x83289EC0;
	sub_8222CED0(ctx, base);
	// 83289EC0: 3CE0820F  lis r7, -0x7df1
	ctx.r[7].s64 = -2112946176;
	// 83289EC4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83289EC8: 38875AF8  addi r4, r7, 0x5af8
	ctx.r[4].s64 = ctx.r[7].s64 + 23288;
	// 83289ECC: 387F0470  addi r3, r31, 0x470
	ctx.r[3].s64 = ctx.r[31].s64 + 1136;
	// 83289ED0: 4AFA3001  bl 0x8222ced0
	ctx.lr = 0x83289ED4;
	sub_8222CED0(ctx, base);
	// 83289ED4: 39600003  li r11, 3
	ctx.r[11].s64 = 3;
	// 83289ED8: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 83289EDC: 917F0474  stw r11, 0x474(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(1140 as u32), ctx.r[11].u32 ) };
	// 83289EE0: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83289EE4: 387F0478  addi r3, r31, 0x478
	ctx.r[3].s64 = ctx.r[31].s64 + 1144;
	// 83289EE8: 4AFA2FE9  bl 0x8222ced0
	ctx.lr = 0x83289EEC;
	sub_8222CED0(ctx, base);
	// 83289EEC: 3CC0820F  lis r6, -0x7df1
	ctx.r[6].s64 = -2112946176;
	// 83289EF0: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83289EF4: 38865AEC  addi r4, r6, 0x5aec
	ctx.r[4].s64 = ctx.r[6].s64 + 23276;
	// 83289EF8: 387F047C  addi r3, r31, 0x47c
	ctx.r[3].s64 = ctx.r[31].s64 + 1148;
	// 83289EFC: 4AFA2FD5  bl 0x8222ced0
	ctx.lr = 0x83289F00;
	sub_8222CED0(ctx, base);
	// 83289F00: 3C80820F  lis r4, -0x7df1
	ctx.r[4].s64 = -2112946176;
	// 83289F04: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83289F08: 38845ADC  addi r4, r4, 0x5adc
	ctx.r[4].s64 = ctx.r[4].s64 + 23260;
	// 83289F0C: 387F0480  addi r3, r31, 0x480
	ctx.r[3].s64 = ctx.r[31].s64 + 1152;
	// 83289F10: 4AFA2FC1  bl 0x8222ced0
	ctx.lr = 0x83289F14;
	sub_8222CED0(ctx, base);
	// 83289F14: 39600003  li r11, 3
	ctx.r[11].s64 = 3;
	// 83289F18: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 83289F1C: 917F0484  stw r11, 0x484(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(1156 as u32), ctx.r[11].u32 ) };
	// 83289F20: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83289F24: 387F0488  addi r3, r31, 0x488
	ctx.r[3].s64 = ctx.r[31].s64 + 1160;
	// 83289F28: 4AFA2FA9  bl 0x8222ced0
	ctx.lr = 0x83289F2C;
	sub_8222CED0(ctx, base);
	// 83289F2C: 3C60820F  lis r3, -0x7df1
	ctx.r[3].s64 = -2112946176;
	// 83289F30: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83289F34: 38835ACC  addi r4, r3, 0x5acc
	ctx.r[4].s64 = ctx.r[3].s64 + 23244;
	// 83289F38: 387F048C  addi r3, r31, 0x48c
	ctx.r[3].s64 = ctx.r[31].s64 + 1164;
	// 83289F3C: 4AFA2F95  bl 0x8222ced0
	ctx.lr = 0x83289F40;
	sub_8222CED0(ctx, base);
	// 83289F40: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 83289F44: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83289F48: 388B5AC0  addi r4, r11, 0x5ac0
	ctx.r[4].s64 = ctx.r[11].s64 + 23232;
	// 83289F4C: 387F0490  addi r3, r31, 0x490
	ctx.r[3].s64 = ctx.r[31].s64 + 1168;
	// 83289F50: 4AFA2F81  bl 0x8222ced0
	ctx.lr = 0x83289F54;
	sub_8222CED0(ctx, base);
	// 83289F54: 39600003  li r11, 3
	ctx.r[11].s64 = 3;
	// 83289F58: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 83289F5C: 917F0494  stw r11, 0x494(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(1172 as u32), ctx.r[11].u32 ) };
	// 83289F60: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83289F64: 387F0498  addi r3, r31, 0x498
	ctx.r[3].s64 = ctx.r[31].s64 + 1176;
	// 83289F68: 4AFA2F69  bl 0x8222ced0
	ctx.lr = 0x83289F6C;
	sub_8222CED0(ctx, base);
	// 83289F6C: 3D40820F  lis r10, -0x7df1
	ctx.r[10].s64 = -2112946176;
	// 83289F70: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83289F74: 388A5AB4  addi r4, r10, 0x5ab4
	ctx.r[4].s64 = ctx.r[10].s64 + 23220;
	// 83289F78: 387F049C  addi r3, r31, 0x49c
	ctx.r[3].s64 = ctx.r[31].s64 + 1180;
	// 83289F7C: 4AFA2F55  bl 0x8222ced0
	ctx.lr = 0x83289F80;
	sub_8222CED0(ctx, base);
	// 83289F80: 3D20820B  lis r9, -0x7df5
	ctx.r[9].s64 = -2113208320;
	// 83289F84: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83289F88: 388903D0  addi r4, r9, 0x3d0
	ctx.r[4].s64 = ctx.r[9].s64 + 976;
	// 83289F8C: 387F04A0  addi r3, r31, 0x4a0
	ctx.r[3].s64 = ctx.r[31].s64 + 1184;
	// 83289F90: 4AFA2F41  bl 0x8222ced0
	ctx.lr = 0x83289F94;
	sub_8222CED0(ctx, base);
	// 83289F94: 39600003  li r11, 3
	ctx.r[11].s64 = 3;
	// 83289F98: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 83289F9C: 917F04A4  stw r11, 0x4a4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(1188 as u32), ctx.r[11].u32 ) };
	// 83289FA0: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83289FA4: 387F04A8  addi r3, r31, 0x4a8
	ctx.r[3].s64 = ctx.r[31].s64 + 1192;
	// 83289FA8: 4AFA2F29  bl 0x8222ced0
	ctx.lr = 0x83289FAC;
	sub_8222CED0(ctx, base);
	// 83289FAC: 3D00820F  lis r8, -0x7df1
	ctx.r[8].s64 = -2112946176;
	// 83289FB0: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83289FB4: 38885AA8  addi r4, r8, 0x5aa8
	ctx.r[4].s64 = ctx.r[8].s64 + 23208;
	// 83289FB8: 387F04AC  addi r3, r31, 0x4ac
	ctx.r[3].s64 = ctx.r[31].s64 + 1196;
	// 83289FBC: 4AFA2F15  bl 0x8222ced0
	ctx.lr = 0x83289FC0;
	sub_8222CED0(ctx, base);
	// 83289FC0: 3CE0820F  lis r7, -0x7df1
	ctx.r[7].s64 = -2112946176;
	// 83289FC4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83289FC8: 38875A90  addi r4, r7, 0x5a90
	ctx.r[4].s64 = ctx.r[7].s64 + 23184;
	// 83289FCC: 387F04B0  addi r3, r31, 0x4b0
	ctx.r[3].s64 = ctx.r[31].s64 + 1200;
	// 83289FD0: 4AFA2F01  bl 0x8222ced0
	ctx.lr = 0x83289FD4;
	sub_8222CED0(ctx, base);
	// 83289FD4: 39600003  li r11, 3
	ctx.r[11].s64 = 3;
	// 83289FD8: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 83289FDC: 917F04B4  stw r11, 0x4b4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(1204 as u32), ctx.r[11].u32 ) };
	// 83289FE0: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83289FE4: 387F04B8  addi r3, r31, 0x4b8
	ctx.r[3].s64 = ctx.r[31].s64 + 1208;
	// 83289FE8: 4AFA2EE9  bl 0x8222ced0
	ctx.lr = 0x83289FEC;
	sub_8222CED0(ctx, base);
	// 83289FEC: 3CC0820F  lis r6, -0x7df1
	ctx.r[6].s64 = -2112946176;
	// 83289FF0: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83289FF4: 38865A7C  addi r4, r6, 0x5a7c
	ctx.r[4].s64 = ctx.r[6].s64 + 23164;
	// 83289FF8: 387F04BC  addi r3, r31, 0x4bc
	ctx.r[3].s64 = ctx.r[31].s64 + 1212;
	// 83289FFC: 4AFA2ED5  bl 0x8222ced0
	ctx.lr = 0x8328A000;
	sub_8222CED0(ctx, base);
	// 8328A000: 3C80820F  lis r4, -0x7df1
	ctx.r[4].s64 = -2112946176;
	// 8328A004: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8328A008: 38845A68  addi r4, r4, 0x5a68
	ctx.r[4].s64 = ctx.r[4].s64 + 23144;
	// 8328A00C: 387F04C0  addi r3, r31, 0x4c0
	ctx.r[3].s64 = ctx.r[31].s64 + 1216;
	// 8328A010: 4AFA2EC1  bl 0x8222ced0
	ctx.lr = 0x8328A014;
	sub_8222CED0(ctx, base);
	// 8328A014: 39600003  li r11, 3
	ctx.r[11].s64 = 3;
	// 8328A018: 3C60820F  lis r3, -0x7df1
	ctx.r[3].s64 = -2112946176;
	// 8328A01C: 917F04C4  stw r11, 0x4c4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(1220 as u32), ctx.r[11].u32 ) };
	// 8328A020: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8328A024: 3BC35A60  addi r30, r3, 0x5a60
	ctx.r[30].s64 = ctx.r[3].s64 + 23136;
	// 8328A028: 387F04C8  addi r3, r31, 0x4c8
	ctx.r[3].s64 = ctx.r[31].s64 + 1224;
	// 8328A02C: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8328A030: 4AFA2EA1  bl 0x8222ced0
	ctx.lr = 0x8328A034;
	sub_8222CED0(ctx, base);
	// 8328A034: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 8328A038: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8328A03C: 388B5A50  addi r4, r11, 0x5a50
	ctx.r[4].s64 = ctx.r[11].s64 + 23120;
	// 8328A040: 387F04CC  addi r3, r31, 0x4cc
	ctx.r[3].s64 = ctx.r[31].s64 + 1228;
	// 8328A044: 4AFA2E8D  bl 0x8222ced0
	ctx.lr = 0x8328A048;
	sub_8222CED0(ctx, base);
	// 8328A048: 3D40820F  lis r10, -0x7df1
	ctx.r[10].s64 = -2112946176;
	// 8328A04C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8328A050: 388A5A3C  addi r4, r10, 0x5a3c
	ctx.r[4].s64 = ctx.r[10].s64 + 23100;
	// 8328A054: 387F04D0  addi r3, r31, 0x4d0
	ctx.r[3].s64 = ctx.r[31].s64 + 1232;
	// 8328A058: 4AFA2E79  bl 0x8222ced0
	ctx.lr = 0x8328A05C;
	sub_8222CED0(ctx, base);
	// 8328A05C: 39600003  li r11, 3
	ctx.r[11].s64 = 3;
	// 8328A060: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8328A064: 917F04D4  stw r11, 0x4d4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(1236 as u32), ctx.r[11].u32 ) };
	// 8328A068: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8328A06C: 387F04D8  addi r3, r31, 0x4d8
	ctx.r[3].s64 = ctx.r[31].s64 + 1240;
	// 8328A070: 4AFA2E61  bl 0x8222ced0
	ctx.lr = 0x8328A074;
	sub_8222CED0(ctx, base);
	// 8328A074: 3D20820F  lis r9, -0x7df1
	ctx.r[9].s64 = -2112946176;
	// 8328A078: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8328A07C: 38895A2C  addi r4, r9, 0x5a2c
	ctx.r[4].s64 = ctx.r[9].s64 + 23084;
	// 8328A080: 387F04DC  addi r3, r31, 0x4dc
	ctx.r[3].s64 = ctx.r[31].s64 + 1244;
	// 8328A084: 4AFA2E4D  bl 0x8222ced0
	ctx.lr = 0x8328A088;
	sub_8222CED0(ctx, base);
	// 8328A088: 3D00820F  lis r8, -0x7df1
	ctx.r[8].s64 = -2112946176;
	// 8328A08C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8328A090: 38885A14  addi r4, r8, 0x5a14
	ctx.r[4].s64 = ctx.r[8].s64 + 23060;
	// 8328A094: 387F04E0  addi r3, r31, 0x4e0
	ctx.r[3].s64 = ctx.r[31].s64 + 1248;
	// 8328A098: 4AFA2E39  bl 0x8222ced0
	ctx.lr = 0x8328A09C;
	sub_8222CED0(ctx, base);
	// 8328A09C: 39600003  li r11, 3
	ctx.r[11].s64 = 3;
	// 8328A0A0: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8328A0A4: 917F04E4  stw r11, 0x4e4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(1252 as u32), ctx.r[11].u32 ) };
	// 8328A0A8: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8328A0AC: 387F04E8  addi r3, r31, 0x4e8
	ctx.r[3].s64 = ctx.r[31].s64 + 1256;
	// 8328A0B0: 4AFA2E21  bl 0x8222ced0
	ctx.lr = 0x8328A0B4;
	sub_8222CED0(ctx, base);
	// 8328A0B4: 3CE0820F  lis r7, -0x7df1
	ctx.r[7].s64 = -2112946176;
	// 8328A0B8: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8328A0BC: 388759FC  addi r4, r7, 0x59fc
	ctx.r[4].s64 = ctx.r[7].s64 + 23036;
	// 8328A0C0: 387F04EC  addi r3, r31, 0x4ec
	ctx.r[3].s64 = ctx.r[31].s64 + 1260;
	// 8328A0C4: 4AFA2E0D  bl 0x8222ced0
	ctx.lr = 0x8328A0C8;
	sub_8222CED0(ctx, base);
	// 8328A0C8: 3CC0820F  lis r6, -0x7df1
	ctx.r[6].s64 = -2112946176;
	// 8328A0CC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8328A0D0: 388659E4  addi r4, r6, 0x59e4
	ctx.r[4].s64 = ctx.r[6].s64 + 23012;
	// 8328A0D4: 387F04F0  addi r3, r31, 0x4f0
	ctx.r[3].s64 = ctx.r[31].s64 + 1264;
	// 8328A0D8: 4AFA2DF9  bl 0x8222ced0
	ctx.lr = 0x8328A0DC;
	sub_8222CED0(ctx, base);
	// 8328A0DC: 39600003  li r11, 3
	ctx.r[11].s64 = 3;
	// 8328A0E0: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8328A0E4: 917F04F4  stw r11, 0x4f4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(1268 as u32), ctx.r[11].u32 ) };
	// 8328A0E8: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8328A0EC: 387F04F8  addi r3, r31, 0x4f8
	ctx.r[3].s64 = ctx.r[31].s64 + 1272;
	// 8328A0F0: 4AFA2DE1  bl 0x8222ced0
	ctx.lr = 0x8328A0F4;
	sub_8222CED0(ctx, base);
	// 8328A0F4: 3C80820F  lis r4, -0x7df1
	ctx.r[4].s64 = -2112946176;
	// 8328A0F8: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8328A0FC: 388459D0  addi r4, r4, 0x59d0
	ctx.r[4].s64 = ctx.r[4].s64 + 22992;
	// 8328A100: 387F04FC  addi r3, r31, 0x4fc
	ctx.r[3].s64 = ctx.r[31].s64 + 1276;
	// 8328A104: 4AFA2DCD  bl 0x8222ced0
	ctx.lr = 0x8328A108;
	sub_8222CED0(ctx, base);
	// 8328A108: 3C60820F  lis r3, -0x7df1
	ctx.r[3].s64 = -2112946176;
	// 8328A10C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8328A110: 388359BC  addi r4, r3, 0x59bc
	ctx.r[4].s64 = ctx.r[3].s64 + 22972;
	// 8328A114: 387F0500  addi r3, r31, 0x500
	ctx.r[3].s64 = ctx.r[31].s64 + 1280;
	// 8328A118: 4AFA2DB9  bl 0x8222ced0
	ctx.lr = 0x8328A11C;
	sub_8222CED0(ctx, base);
	// 8328A11C: 39600003  li r11, 3
	ctx.r[11].s64 = 3;
	// 8328A120: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8328A124: 917F0504  stw r11, 0x504(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(1284 as u32), ctx.r[11].u32 ) };
	// 8328A128: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8328A12C: 387F0508  addi r3, r31, 0x508
	ctx.r[3].s64 = ctx.r[31].s64 + 1288;
	// 8328A130: 4AFA2DA1  bl 0x8222ced0
	ctx.lr = 0x8328A134;
	sub_8222CED0(ctx, base);
	// 8328A134: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 8328A138: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8328A13C: 388B59A8  addi r4, r11, 0x59a8
	ctx.r[4].s64 = ctx.r[11].s64 + 22952;
	// 8328A140: 387F050C  addi r3, r31, 0x50c
	ctx.r[3].s64 = ctx.r[31].s64 + 1292;
	// 8328A144: 4AFA2D8D  bl 0x8222ced0
	ctx.lr = 0x8328A148;
	sub_8222CED0(ctx, base);
	// 8328A148: 3D40820F  lis r10, -0x7df1
	ctx.r[10].s64 = -2112946176;
	// 8328A14C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8328A150: 388A5990  addi r4, r10, 0x5990
	ctx.r[4].s64 = ctx.r[10].s64 + 22928;
	// 8328A154: 387F0510  addi r3, r31, 0x510
	ctx.r[3].s64 = ctx.r[31].s64 + 1296;
	// 8328A158: 4AFA2D79  bl 0x8222ced0
	ctx.lr = 0x8328A15C;
	sub_8222CED0(ctx, base);
	// 8328A15C: 39600003  li r11, 3
	ctx.r[11].s64 = 3;
	// 8328A160: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8328A164: 917F0514  stw r11, 0x514(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(1300 as u32), ctx.r[11].u32 ) };
	// 8328A168: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8328A16C: 387F0518  addi r3, r31, 0x518
	ctx.r[3].s64 = ctx.r[31].s64 + 1304;
	// 8328A170: 4AFA2D61  bl 0x8222ced0
	ctx.lr = 0x8328A174;
	sub_8222CED0(ctx, base);
	// 8328A174: 3D20820F  lis r9, -0x7df1
	ctx.r[9].s64 = -2112946176;
	// 8328A178: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8328A17C: 3889597C  addi r4, r9, 0x597c
	ctx.r[4].s64 = ctx.r[9].s64 + 22908;
	// 8328A180: 387F051C  addi r3, r31, 0x51c
	ctx.r[3].s64 = ctx.r[31].s64 + 1308;
	// 8328A184: 4AFA2D4D  bl 0x8222ced0
	ctx.lr = 0x8328A188;
	sub_8222CED0(ctx, base);
	// 8328A188: 3D00820F  lis r8, -0x7df1
	ctx.r[8].s64 = -2112946176;
	// 8328A18C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8328A190: 38885968  addi r4, r8, 0x5968
	ctx.r[4].s64 = ctx.r[8].s64 + 22888;
	// 8328A194: 387F0520  addi r3, r31, 0x520
	ctx.r[3].s64 = ctx.r[31].s64 + 1312;
	// 8328A198: 4AFA2D39  bl 0x8222ced0
	ctx.lr = 0x8328A19C;
	sub_8222CED0(ctx, base);
	// 8328A19C: 39600003  li r11, 3
	ctx.r[11].s64 = 3;
	// 8328A1A0: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8328A1A4: 917F0524  stw r11, 0x524(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(1316 as u32), ctx.r[11].u32 ) };
	// 8328A1A8: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8328A1AC: 387F0528  addi r3, r31, 0x528
	ctx.r[3].s64 = ctx.r[31].s64 + 1320;
	// 8328A1B0: 4AFA2D21  bl 0x8222ced0
	ctx.lr = 0x8328A1B4;
	sub_8222CED0(ctx, base);
	// 8328A1B4: 3CE0820F  lis r7, -0x7df1
	ctx.r[7].s64 = -2112946176;
	// 8328A1B8: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8328A1BC: 38875954  addi r4, r7, 0x5954
	ctx.r[4].s64 = ctx.r[7].s64 + 22868;
	// 8328A1C0: 387F052C  addi r3, r31, 0x52c
	ctx.r[3].s64 = ctx.r[31].s64 + 1324;
	// 8328A1C4: 4AFA2D0D  bl 0x8222ced0
	ctx.lr = 0x8328A1C8;
	sub_8222CED0(ctx, base);
	// 8328A1C8: 3CC0820F  lis r6, -0x7df1
	ctx.r[6].s64 = -2112946176;
	// 8328A1CC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8328A1D0: 38865938  addi r4, r6, 0x5938
	ctx.r[4].s64 = ctx.r[6].s64 + 22840;
	// 8328A1D4: 387F0530  addi r3, r31, 0x530
	ctx.r[3].s64 = ctx.r[31].s64 + 1328;
	// 8328A1D8: 4AFA2CF9  bl 0x8222ced0
	ctx.lr = 0x8328A1DC;
	sub_8222CED0(ctx, base);
	// 8328A1DC: 39600003  li r11, 3
	ctx.r[11].s64 = 3;
	// 8328A1E0: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8328A1E4: 917F0534  stw r11, 0x534(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(1332 as u32), ctx.r[11].u32 ) };
	// 8328A1E8: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8328A1EC: 387F0538  addi r3, r31, 0x538
	ctx.r[3].s64 = ctx.r[31].s64 + 1336;
	// 8328A1F0: 4AFA2CE1  bl 0x8222ced0
	ctx.lr = 0x8328A1F4;
	sub_8222CED0(ctx, base);
	// 8328A1F4: 3C80820D  lis r4, -0x7df3
	ctx.r[4].s64 = -2113077248;
	// 8328A1F8: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8328A1FC: 38848234  addi r4, r4, -0x7dcc
	ctx.r[4].s64 = ctx.r[4].s64 + -32204;
	// 8328A200: 387F053C  addi r3, r31, 0x53c
	ctx.r[3].s64 = ctx.r[31].s64 + 1340;
	// 8328A204: 4AFA2CCD  bl 0x8222ced0
	ctx.lr = 0x8328A208;
	sub_8222CED0(ctx, base);
	// 8328A208: 3C60820F  lis r3, -0x7df1
	ctx.r[3].s64 = -2112946176;
	// 8328A20C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8328A210: 38835920  addi r4, r3, 0x5920
	ctx.r[4].s64 = ctx.r[3].s64 + 22816;
	// 8328A214: 387F0540  addi r3, r31, 0x540
	ctx.r[3].s64 = ctx.r[31].s64 + 1344;
	// 8328A218: 4AFA2CB9  bl 0x8222ced0
	ctx.lr = 0x8328A21C;
	sub_8222CED0(ctx, base);
	// 8328A21C: 39600003  li r11, 3
	ctx.r[11].s64 = 3;
	// 8328A220: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8328A224: 917F0544  stw r11, 0x544(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(1348 as u32), ctx.r[11].u32 ) };
	// 8328A228: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8328A22C: 387F0548  addi r3, r31, 0x548
	ctx.r[3].s64 = ctx.r[31].s64 + 1352;
	// 8328A230: 4AFA2CA1  bl 0x8222ced0
	ctx.lr = 0x8328A234;
	sub_8222CED0(ctx, base);
	// 8328A234: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 8328A238: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8328A23C: 388B590C  addi r4, r11, 0x590c
	ctx.r[4].s64 = ctx.r[11].s64 + 22796;
	// 8328A240: 387F054C  addi r3, r31, 0x54c
	ctx.r[3].s64 = ctx.r[31].s64 + 1356;
	// 8328A244: 4AFA2C8D  bl 0x8222ced0
	ctx.lr = 0x8328A248;
	sub_8222CED0(ctx, base);
	// 8328A248: 3D40820F  lis r10, -0x7df1
	ctx.r[10].s64 = -2112946176;
	// 8328A24C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8328A250: 388A58F4  addi r4, r10, 0x58f4
	ctx.r[4].s64 = ctx.r[10].s64 + 22772;
	// 8328A254: 387F0550  addi r3, r31, 0x550
	ctx.r[3].s64 = ctx.r[31].s64 + 1360;
	// 8328A258: 4AFA2C79  bl 0x8222ced0
	ctx.lr = 0x8328A25C;
	sub_8222CED0(ctx, base);
	// 8328A25C: 39600003  li r11, 3
	ctx.r[11].s64 = 3;
	// 8328A260: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8328A264: 917F0554  stw r11, 0x554(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(1364 as u32), ctx.r[11].u32 ) };
	// 8328A268: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8328A26C: 387F0558  addi r3, r31, 0x558
	ctx.r[3].s64 = ctx.r[31].s64 + 1368;
	// 8328A270: 4AFA2C61  bl 0x8222ced0
	ctx.lr = 0x8328A274;
	sub_8222CED0(ctx, base);
	// 8328A274: 3D20820F  lis r9, -0x7df1
	ctx.r[9].s64 = -2112946176;
	// 8328A278: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8328A27C: 388958E0  addi r4, r9, 0x58e0
	ctx.r[4].s64 = ctx.r[9].s64 + 22752;
	// 8328A280: 387F055C  addi r3, r31, 0x55c
	ctx.r[3].s64 = ctx.r[31].s64 + 1372;
	// 8328A284: 4AFA2C4D  bl 0x8222ced0
	ctx.lr = 0x8328A288;
	sub_8222CED0(ctx, base);
	// 8328A288: 3D00820F  lis r8, -0x7df1
	ctx.r[8].s64 = -2112946176;
	// 8328A28C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8328A290: 388858D0  addi r4, r8, 0x58d0
	ctx.r[4].s64 = ctx.r[8].s64 + 22736;
	// 8328A294: 387F0560  addi r3, r31, 0x560
	ctx.r[3].s64 = ctx.r[31].s64 + 1376;
	// 8328A298: 4AFA2C39  bl 0x8222ced0
	ctx.lr = 0x8328A29C;
	sub_8222CED0(ctx, base);
	// 8328A29C: 39600003  li r11, 3
	ctx.r[11].s64 = 3;
	// 8328A2A0: 3CE0820F  lis r7, -0x7df1
	ctx.r[7].s64 = -2112946176;
	// 8328A2A4: 917F0564  stw r11, 0x564(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(1380 as u32), ctx.r[11].u32 ) };
	// 8328A2A8: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8328A2AC: 3BC758C0  addi r30, r7, 0x58c0
	ctx.r[30].s64 = ctx.r[7].s64 + 22720;
	// 8328A2B0: 387F0568  addi r3, r31, 0x568
	ctx.r[3].s64 = ctx.r[31].s64 + 1384;
	// 8328A2B4: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8328A2B8: 4AFA2C19  bl 0x8222ced0
	ctx.lr = 0x8328A2BC;
	sub_8222CED0(ctx, base);
	// 8328A2BC: 3CC0820F  lis r6, -0x7df1
	ctx.r[6].s64 = -2112946176;
	// 8328A2C0: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8328A2C4: 388658A8  addi r4, r6, 0x58a8
	ctx.r[4].s64 = ctx.r[6].s64 + 22696;
	// 8328A2C8: 387F056C  addi r3, r31, 0x56c
	ctx.r[3].s64 = ctx.r[31].s64 + 1388;
	// 8328A2CC: 4AFA2C05  bl 0x8222ced0
	ctx.lr = 0x8328A2D0;
	sub_8222CED0(ctx, base);
	// 8328A2D0: 3C80820F  lis r4, -0x7df1
	ctx.r[4].s64 = -2112946176;
	// 8328A2D4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8328A2D8: 38845890  addi r4, r4, 0x5890
	ctx.r[4].s64 = ctx.r[4].s64 + 22672;
	// 8328A2DC: 387F0570  addi r3, r31, 0x570
	ctx.r[3].s64 = ctx.r[31].s64 + 1392;
	// 8328A2E0: 4AFA2BF1  bl 0x8222ced0
	ctx.lr = 0x8328A2E4;
	sub_8222CED0(ctx, base);
	// 8328A2E4: 39600003  li r11, 3
	ctx.r[11].s64 = 3;
	// 8328A2E8: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8328A2EC: 917F0574  stw r11, 0x574(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(1396 as u32), ctx.r[11].u32 ) };
	// 8328A2F0: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8328A2F4: 387F0578  addi r3, r31, 0x578
	ctx.r[3].s64 = ctx.r[31].s64 + 1400;
	// 8328A2F8: 4AFA2BD9  bl 0x8222ced0
	ctx.lr = 0x8328A2FC;
	sub_8222CED0(ctx, base);
	// 8328A2FC: 3C60820F  lis r3, -0x7df1
	ctx.r[3].s64 = -2112946176;
	// 8328A300: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8328A304: 38835874  addi r4, r3, 0x5874
	ctx.r[4].s64 = ctx.r[3].s64 + 22644;
	// 8328A308: 387F057C  addi r3, r31, 0x57c
	ctx.r[3].s64 = ctx.r[31].s64 + 1404;
	// 8328A30C: 4AFA2BC5  bl 0x8222ced0
	ctx.lr = 0x8328A310;
	sub_8222CED0(ctx, base);
	// 8328A310: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 8328A314: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8328A318: 388B5860  addi r4, r11, 0x5860
	ctx.r[4].s64 = ctx.r[11].s64 + 22624;
	// 8328A31C: 387F0580  addi r3, r31, 0x580
	ctx.r[3].s64 = ctx.r[31].s64 + 1408;
	// 8328A320: 4AFA2BB1  bl 0x8222ced0
	ctx.lr = 0x8328A324;
	sub_8222CED0(ctx, base);
	// 8328A324: 39600003  li r11, 3
	ctx.r[11].s64 = 3;
	// 8328A328: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8328A32C: 917F0584  stw r11, 0x584(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(1412 as u32), ctx.r[11].u32 ) };
	// 8328A330: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8328A334: 387F0588  addi r3, r31, 0x588
	ctx.r[3].s64 = ctx.r[31].s64 + 1416;
	// 8328A338: 4AFA2B99  bl 0x8222ced0
	ctx.lr = 0x8328A33C;
	sub_8222CED0(ctx, base);
	// 8328A33C: 3D40820F  lis r10, -0x7df1
	ctx.r[10].s64 = -2112946176;
	// 8328A340: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8328A344: 388A5844  addi r4, r10, 0x5844
	ctx.r[4].s64 = ctx.r[10].s64 + 22596;
	// 8328A348: 387F058C  addi r3, r31, 0x58c
	ctx.r[3].s64 = ctx.r[31].s64 + 1420;
	// 8328A34C: 4AFA2B85  bl 0x8222ced0
	ctx.lr = 0x8328A350;
	sub_8222CED0(ctx, base);
	// 8328A350: 3D20820F  lis r9, -0x7df1
	ctx.r[9].s64 = -2112946176;
	// 8328A354: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8328A358: 38895834  addi r4, r9, 0x5834
	ctx.r[4].s64 = ctx.r[9].s64 + 22580;
	// 8328A35C: 387F0590  addi r3, r31, 0x590
	ctx.r[3].s64 = ctx.r[31].s64 + 1424;
	// 8328A360: 4AFA2B71  bl 0x8222ced0
	ctx.lr = 0x8328A364;
	sub_8222CED0(ctx, base);
	// 8328A364: 39600003  li r11, 3
	ctx.r[11].s64 = 3;
	// 8328A368: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8328A36C: 917F0594  stw r11, 0x594(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(1428 as u32), ctx.r[11].u32 ) };
	// 8328A370: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8328A374: 387F0598  addi r3, r31, 0x598
	ctx.r[3].s64 = ctx.r[31].s64 + 1432;
	// 8328A378: 4AFA2B59  bl 0x8222ced0
	ctx.lr = 0x8328A37C;
	sub_8222CED0(ctx, base);
	// 8328A37C: 3D00820F  lis r8, -0x7df1
	ctx.r[8].s64 = -2112946176;
	// 8328A380: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8328A384: 3888581C  addi r4, r8, 0x581c
	ctx.r[4].s64 = ctx.r[8].s64 + 22556;
	// 8328A388: 387F059C  addi r3, r31, 0x59c
	ctx.r[3].s64 = ctx.r[31].s64 + 1436;
	// 8328A38C: 4AFA2B45  bl 0x8222ced0
	ctx.lr = 0x8328A390;
	sub_8222CED0(ctx, base);
	// 8328A390: 3CE0820F  lis r7, -0x7df1
	ctx.r[7].s64 = -2112946176;
	// 8328A394: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8328A398: 38875804  addi r4, r7, 0x5804
	ctx.r[4].s64 = ctx.r[7].s64 + 22532;
	// 8328A39C: 387F05A0  addi r3, r31, 0x5a0
	ctx.r[3].s64 = ctx.r[31].s64 + 1440;
	// 8328A3A0: 4AFA2B31  bl 0x8222ced0
	ctx.lr = 0x8328A3A4;
	sub_8222CED0(ctx, base);
	// 8328A3A4: 39600003  li r11, 3
	ctx.r[11].s64 = 3;
	// 8328A3A8: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8328A3AC: 917F05A4  stw r11, 0x5a4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(1444 as u32), ctx.r[11].u32 ) };
	// 8328A3B0: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8328A3B4: 387F05A8  addi r3, r31, 0x5a8
	ctx.r[3].s64 = ctx.r[31].s64 + 1448;
	// 8328A3B8: 4AFA2B19  bl 0x8222ced0
	ctx.lr = 0x8328A3BC;
	sub_8222CED0(ctx, base);
	// 8328A3BC: 3CC0820F  lis r6, -0x7df1
	ctx.r[6].s64 = -2112946176;
	// 8328A3C0: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8328A3C4: 388657E8  addi r4, r6, 0x57e8
	ctx.r[4].s64 = ctx.r[6].s64 + 22504;
	// 8328A3C8: 387F05AC  addi r3, r31, 0x5ac
	ctx.r[3].s64 = ctx.r[31].s64 + 1452;
	// 8328A3CC: 4AFA2B05  bl 0x8222ced0
	ctx.lr = 0x8328A3D0;
	sub_8222CED0(ctx, base);
	// 8328A3D0: 3C80820F  lis r4, -0x7df1
	ctx.r[4].s64 = -2112946176;
	// 8328A3D4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8328A3D8: 388457D4  addi r4, r4, 0x57d4
	ctx.r[4].s64 = ctx.r[4].s64 + 22484;
	// 8328A3DC: 387F05B0  addi r3, r31, 0x5b0
	ctx.r[3].s64 = ctx.r[31].s64 + 1456;
	// 8328A3E0: 4AFA2AF1  bl 0x8222ced0
	ctx.lr = 0x8328A3E4;
	sub_8222CED0(ctx, base);
	// 8328A3E4: 39600003  li r11, 3
	ctx.r[11].s64 = 3;
	// 8328A3E8: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8328A3EC: 917F05B4  stw r11, 0x5b4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(1460 as u32), ctx.r[11].u32 ) };
	// 8328A3F0: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8328A3F4: 387F05B8  addi r3, r31, 0x5b8
	ctx.r[3].s64 = ctx.r[31].s64 + 1464;
	// 8328A3F8: 4AFA2AD9  bl 0x8222ced0
	ctx.lr = 0x8328A3FC;
	sub_8222CED0(ctx, base);
	// 8328A3FC: 3C60820F  lis r3, -0x7df1
	ctx.r[3].s64 = -2112946176;
	// 8328A400: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8328A404: 388357B8  addi r4, r3, 0x57b8
	ctx.r[4].s64 = ctx.r[3].s64 + 22456;
	// 8328A408: 387F05BC  addi r3, r31, 0x5bc
	ctx.r[3].s64 = ctx.r[31].s64 + 1468;
	// 8328A40C: 4AFA2AC5  bl 0x8222ced0
	ctx.lr = 0x8328A410;
	sub_8222CED0(ctx, base);
	// 8328A410: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 8328A414: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8328A418: 388B57A8  addi r4, r11, 0x57a8
	ctx.r[4].s64 = ctx.r[11].s64 + 22440;
	// 8328A41C: 387F05C0  addi r3, r31, 0x5c0
	ctx.r[3].s64 = ctx.r[31].s64 + 1472;
	// 8328A420: 4AFA2AB1  bl 0x8222ced0
	ctx.lr = 0x8328A424;
	sub_8222CED0(ctx, base);
	// 8328A424: 39600003  li r11, 3
	ctx.r[11].s64 = 3;
	// 8328A428: 3D40820F  lis r10, -0x7df1
	ctx.r[10].s64 = -2112946176;
	// 8328A42C: 917F05C4  stw r11, 0x5c4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(1476 as u32), ctx.r[11].u32 ) };
	// 8328A430: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8328A434: 3BCA4948  addi r30, r10, 0x4948
	ctx.r[30].s64 = ctx.r[10].s64 + 18760;
	// 8328A438: 387F05C8  addi r3, r31, 0x5c8
	ctx.r[3].s64 = ctx.r[31].s64 + 1480;
	// 8328A43C: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8328A440: 4AFA2A91  bl 0x8222ced0
	ctx.lr = 0x8328A444;
	sub_8222CED0(ctx, base);
	// 8328A444: 3D20820F  lis r9, -0x7df1
	ctx.r[9].s64 = -2112946176;
	// 8328A448: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8328A44C: 3889579C  addi r4, r9, 0x579c
	ctx.r[4].s64 = ctx.r[9].s64 + 22428;
	// 8328A450: 387F05CC  addi r3, r31, 0x5cc
	ctx.r[3].s64 = ctx.r[31].s64 + 1484;
	// 8328A454: 4AFA2A7D  bl 0x8222ced0
	ctx.lr = 0x8328A458;
	sub_8222CED0(ctx, base);
	// 8328A458: 3D00820F  lis r8, -0x7df1
	ctx.r[8].s64 = -2112946176;
	// 8328A45C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8328A460: 3888578C  addi r4, r8, 0x578c
	ctx.r[4].s64 = ctx.r[8].s64 + 22412;
	// 8328A464: 387F05D0  addi r3, r31, 0x5d0
	ctx.r[3].s64 = ctx.r[31].s64 + 1488;
	// 8328A468: 4AFA2A69  bl 0x8222ced0
	ctx.lr = 0x8328A46C;
	sub_8222CED0(ctx, base);
	// 8328A46C: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 8328A470: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8328A474: 917F05D4  stw r11, 0x5d4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(1492 as u32), ctx.r[11].u32 ) };
	// 8328A478: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8328A47C: 387F05D8  addi r3, r31, 0x5d8
	ctx.r[3].s64 = ctx.r[31].s64 + 1496;
	// 8328A480: 4AFA2A51  bl 0x8222ced0
	ctx.lr = 0x8328A484;
	sub_8222CED0(ctx, base);
	// 8328A484: 3CE0820F  lis r7, -0x7df1
	ctx.r[7].s64 = -2112946176;
	// 8328A488: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8328A48C: 38875780  addi r4, r7, 0x5780
	ctx.r[4].s64 = ctx.r[7].s64 + 22400;
	// 8328A490: 387F05DC  addi r3, r31, 0x5dc
	ctx.r[3].s64 = ctx.r[31].s64 + 1500;
	// 8328A494: 4AFA2A3D  bl 0x8222ced0
	ctx.lr = 0x8328A498;
	sub_8222CED0(ctx, base);
	// 8328A498: 3CC0820F  lis r6, -0x7df1
	ctx.r[6].s64 = -2112946176;
	// 8328A49C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8328A4A0: 3886576C  addi r4, r6, 0x576c
	ctx.r[4].s64 = ctx.r[6].s64 + 22380;
	// 8328A4A4: 387F05E0  addi r3, r31, 0x5e0
	ctx.r[3].s64 = ctx.r[31].s64 + 1504;
	// 8328A4A8: 4AFA2A29  bl 0x8222ced0
	ctx.lr = 0x8328A4AC;
	sub_8222CED0(ctx, base);
	// 8328A4AC: 39600003  li r11, 3
	ctx.r[11].s64 = 3;
	// 8328A4B0: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8328A4B4: 917F05E4  stw r11, 0x5e4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(1508 as u32), ctx.r[11].u32 ) };
	// 8328A4B8: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8328A4BC: 387F05E8  addi r3, r31, 0x5e8
	ctx.r[3].s64 = ctx.r[31].s64 + 1512;
	// 8328A4C0: 4AFA2A11  bl 0x8222ced0
	ctx.lr = 0x8328A4C4;
	sub_8222CED0(ctx, base);
	// 8328A4C4: 3C80820F  lis r4, -0x7df1
	ctx.r[4].s64 = -2112946176;
	// 8328A4C8: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8328A4CC: 3884575C  addi r4, r4, 0x575c
	ctx.r[4].s64 = ctx.r[4].s64 + 22364;
	// 8328A4D0: 387F05EC  addi r3, r31, 0x5ec
	ctx.r[3].s64 = ctx.r[31].s64 + 1516;
	// 8328A4D4: 4AFA29FD  bl 0x8222ced0
	ctx.lr = 0x8328A4D8;
	sub_8222CED0(ctx, base);
	// 8328A4D8: 3C60820F  lis r3, -0x7df1
	ctx.r[3].s64 = -2112946176;
	// 8328A4DC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8328A4E0: 3883574C  addi r4, r3, 0x574c
	ctx.r[4].s64 = ctx.r[3].s64 + 22348;
	// 8328A4E4: 387F05F0  addi r3, r31, 0x5f0
	ctx.r[3].s64 = ctx.r[31].s64 + 1520;
	// 8328A4E8: 4AFA29E9  bl 0x8222ced0
	ctx.lr = 0x8328A4EC;
	sub_8222CED0(ctx, base);
	// 8328A4EC: 39600003  li r11, 3
	ctx.r[11].s64 = 3;
	// 8328A4F0: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8328A4F4: 917F05F4  stw r11, 0x5f4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(1524 as u32), ctx.r[11].u32 ) };
	// 8328A4F8: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8328A4FC: 387F05F8  addi r3, r31, 0x5f8
	ctx.r[3].s64 = ctx.r[31].s64 + 1528;
	// 8328A500: 4AFA29D1  bl 0x8222ced0
	ctx.lr = 0x8328A504;
	sub_8222CED0(ctx, base);
	// 8328A504: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 8328A508: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8328A50C: 388B573C  addi r4, r11, 0x573c
	ctx.r[4].s64 = ctx.r[11].s64 + 22332;
	// 8328A510: 387F05FC  addi r3, r31, 0x5fc
	ctx.r[3].s64 = ctx.r[31].s64 + 1532;
	// 8328A514: 4AFA29BD  bl 0x8222ced0
	ctx.lr = 0x8328A518;
	sub_8222CED0(ctx, base);
	// 8328A518: 3D40820F  lis r10, -0x7df1
	ctx.r[10].s64 = -2112946176;
	// 8328A51C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8328A520: 388A5728  addi r4, r10, 0x5728
	ctx.r[4].s64 = ctx.r[10].s64 + 22312;
	// 8328A524: 387F0600  addi r3, r31, 0x600
	ctx.r[3].s64 = ctx.r[31].s64 + 1536;
	// 8328A528: 4AFA29A9  bl 0x8222ced0
	ctx.lr = 0x8328A52C;
	sub_8222CED0(ctx, base);
	// 8328A52C: 39600003  li r11, 3
	ctx.r[11].s64 = 3;
	// 8328A530: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8328A534: 917F0604  stw r11, 0x604(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(1540 as u32), ctx.r[11].u32 ) };
	// 8328A538: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8328A53C: 387F0608  addi r3, r31, 0x608
	ctx.r[3].s64 = ctx.r[31].s64 + 1544;
	// 8328A540: 4AFA2991  bl 0x8222ced0
	ctx.lr = 0x8328A544;
	sub_8222CED0(ctx, base);
	// 8328A544: 3D20820F  lis r9, -0x7df1
	ctx.r[9].s64 = -2112946176;
	// 8328A548: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8328A54C: 38895718  addi r4, r9, 0x5718
	ctx.r[4].s64 = ctx.r[9].s64 + 22296;
	// 8328A550: 387F060C  addi r3, r31, 0x60c
	ctx.r[3].s64 = ctx.r[31].s64 + 1548;
	// 8328A554: 4AFA297D  bl 0x8222ced0
	ctx.lr = 0x8328A558;
	sub_8222CED0(ctx, base);
	// 8328A558: 3D00820F  lis r8, -0x7df1
	ctx.r[8].s64 = -2112946176;
	// 8328A55C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8328A560: 38885704  addi r4, r8, 0x5704
	ctx.r[4].s64 = ctx.r[8].s64 + 22276;
	// 8328A564: 387F0610  addi r3, r31, 0x610
	ctx.r[3].s64 = ctx.r[31].s64 + 1552;
	// 8328A568: 4AFA2969  bl 0x8222ced0
	ctx.lr = 0x8328A56C;
	sub_8222CED0(ctx, base);
	// 8328A56C: 39600003  li r11, 3
	ctx.r[11].s64 = 3;
	// 8328A570: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8328A574: 917F0614  stw r11, 0x614(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(1556 as u32), ctx.r[11].u32 ) };
	// 8328A578: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8328A57C: 387F0618  addi r3, r31, 0x618
	ctx.r[3].s64 = ctx.r[31].s64 + 1560;
	// 8328A580: 4AFA2951  bl 0x8222ced0
	ctx.lr = 0x8328A584;
	sub_8222CED0(ctx, base);
	// 8328A584: 3CE0820F  lis r7, -0x7df1
	ctx.r[7].s64 = -2112946176;
	// 8328A588: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8328A58C: 388756F0  addi r4, r7, 0x56f0
	ctx.r[4].s64 = ctx.r[7].s64 + 22256;
	// 8328A590: 387F061C  addi r3, r31, 0x61c
	ctx.r[3].s64 = ctx.r[31].s64 + 1564;
	// 8328A594: 4AFA293D  bl 0x8222ced0
	ctx.lr = 0x8328A598;
	sub_8222CED0(ctx, base);
	// 8328A598: 3CC0820F  lis r6, -0x7df1
	ctx.r[6].s64 = -2112946176;
	// 8328A59C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8328A5A0: 388656D8  addi r4, r6, 0x56d8
	ctx.r[4].s64 = ctx.r[6].s64 + 22232;
	// 8328A5A4: 387F0620  addi r3, r31, 0x620
	ctx.r[3].s64 = ctx.r[31].s64 + 1568;
	// 8328A5A8: 4AFA2929  bl 0x8222ced0
	ctx.lr = 0x8328A5AC;
	sub_8222CED0(ctx, base);
	// 8328A5AC: 39600003  li r11, 3
	ctx.r[11].s64 = 3;
	// 8328A5B0: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8328A5B4: 917F0624  stw r11, 0x624(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(1572 as u32), ctx.r[11].u32 ) };
	// 8328A5B8: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8328A5BC: 387F0628  addi r3, r31, 0x628
	ctx.r[3].s64 = ctx.r[31].s64 + 1576;
	// 8328A5C0: 4AFA2911  bl 0x8222ced0
	ctx.lr = 0x8328A5C4;
	sub_8222CED0(ctx, base);
	// 8328A5C4: 3C80820F  lis r4, -0x7df1
	ctx.r[4].s64 = -2112946176;
	// 8328A5C8: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8328A5CC: 388456C4  addi r4, r4, 0x56c4
	ctx.r[4].s64 = ctx.r[4].s64 + 22212;
	// 8328A5D0: 387F062C  addi r3, r31, 0x62c
	ctx.r[3].s64 = ctx.r[31].s64 + 1580;
	// 8328A5D4: 4AFA28FD  bl 0x8222ced0
	ctx.lr = 0x8328A5D8;
	sub_8222CED0(ctx, base);
	// 8328A5D8: 3C60820F  lis r3, -0x7df1
	ctx.r[3].s64 = -2112946176;
	// 8328A5DC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8328A5E0: 388356B0  addi r4, r3, 0x56b0
	ctx.r[4].s64 = ctx.r[3].s64 + 22192;
	// 8328A5E4: 387F0630  addi r3, r31, 0x630
	ctx.r[3].s64 = ctx.r[31].s64 + 1584;
	// 8328A5E8: 4AFA28E9  bl 0x8222ced0
	ctx.lr = 0x8328A5EC;
	sub_8222CED0(ctx, base);
	// 8328A5EC: 39600003  li r11, 3
	ctx.r[11].s64 = 3;
	// 8328A5F0: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8328A5F4: 917F0634  stw r11, 0x634(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(1588 as u32), ctx.r[11].u32 ) };
	// 8328A5F8: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8328A5FC: 387F0638  addi r3, r31, 0x638
	ctx.r[3].s64 = ctx.r[31].s64 + 1592;
	// 8328A600: 4AFA28D1  bl 0x8222ced0
	ctx.lr = 0x8328A604;
	sub_8222CED0(ctx, base);
	// 8328A604: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 8328A608: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8328A60C: 388B56A4  addi r4, r11, 0x56a4
	ctx.r[4].s64 = ctx.r[11].s64 + 22180;
	// 8328A610: 387F063C  addi r3, r31, 0x63c
	ctx.r[3].s64 = ctx.r[31].s64 + 1596;
	// 8328A614: 4AFA28BD  bl 0x8222ced0
	ctx.lr = 0x8328A618;
	sub_8222CED0(ctx, base);
	// 8328A618: 3D40820F  lis r10, -0x7df1
	ctx.r[10].s64 = -2112946176;
	// 8328A61C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8328A620: 388A5690  addi r4, r10, 0x5690
	ctx.r[4].s64 = ctx.r[10].s64 + 22160;
	// 8328A624: 387F0640  addi r3, r31, 0x640
	ctx.r[3].s64 = ctx.r[31].s64 + 1600;
	// 8328A628: 4AFA28A9  bl 0x8222ced0
	ctx.lr = 0x8328A62C;
	sub_8222CED0(ctx, base);
	// 8328A62C: 39600003  li r11, 3
	ctx.r[11].s64 = 3;
	// 8328A630: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8328A634: 917F0644  stw r11, 0x644(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(1604 as u32), ctx.r[11].u32 ) };
	// 8328A638: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8328A63C: 387F0648  addi r3, r31, 0x648
	ctx.r[3].s64 = ctx.r[31].s64 + 1608;
	// 8328A640: 4AFA2891  bl 0x8222ced0
	ctx.lr = 0x8328A644;
	sub_8222CED0(ctx, base);
	// 8328A644: 3D20820F  lis r9, -0x7df1
	ctx.r[9].s64 = -2112946176;
	// 8328A648: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8328A64C: 38895680  addi r4, r9, 0x5680
	ctx.r[4].s64 = ctx.r[9].s64 + 22144;
	// 8328A650: 387F064C  addi r3, r31, 0x64c
	ctx.r[3].s64 = ctx.r[31].s64 + 1612;
	// 8328A654: 4AFA287D  bl 0x8222ced0
	ctx.lr = 0x8328A658;
	sub_8222CED0(ctx, base);
	// 8328A658: 3D00820F  lis r8, -0x7df1
	ctx.r[8].s64 = -2112946176;
	// 8328A65C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8328A660: 38885670  addi r4, r8, 0x5670
	ctx.r[4].s64 = ctx.r[8].s64 + 22128;
	// 8328A664: 387F0650  addi r3, r31, 0x650
	ctx.r[3].s64 = ctx.r[31].s64 + 1616;
	// 8328A668: 4AFA2869  bl 0x8222ced0
	ctx.lr = 0x8328A66C;
	sub_8222CED0(ctx, base);
	// 8328A66C: 39600003  li r11, 3
	ctx.r[11].s64 = 3;
	// 8328A670: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8328A674: 917F0654  stw r11, 0x654(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(1620 as u32), ctx.r[11].u32 ) };
	// 8328A678: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8328A67C: 387F0658  addi r3, r31, 0x658
	ctx.r[3].s64 = ctx.r[31].s64 + 1624;
	// 8328A680: 4AFA2851  bl 0x8222ced0
	ctx.lr = 0x8328A684;
	sub_8222CED0(ctx, base);
	// 8328A684: 3CE0820F  lis r7, -0x7df1
	ctx.r[7].s64 = -2112946176;
	// 8328A688: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8328A68C: 38875664  addi r4, r7, 0x5664
	ctx.r[4].s64 = ctx.r[7].s64 + 22116;
	// 8328A690: 387F065C  addi r3, r31, 0x65c
	ctx.r[3].s64 = ctx.r[31].s64 + 1628;
	// 8328A694: 4AFA283D  bl 0x8222ced0
	ctx.lr = 0x8328A698;
	sub_8222CED0(ctx, base);
	// 8328A698: 3CC0820F  lis r6, -0x7df1
	ctx.r[6].s64 = -2112946176;
	// 8328A69C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8328A6A0: 3886564C  addi r4, r6, 0x564c
	ctx.r[4].s64 = ctx.r[6].s64 + 22092;
	// 8328A6A4: 387F0660  addi r3, r31, 0x660
	ctx.r[3].s64 = ctx.r[31].s64 + 1632;
	// 8328A6A8: 4AFA2829  bl 0x8222ced0
	ctx.lr = 0x8328A6AC;
	sub_8222CED0(ctx, base);
	// 8328A6AC: 39600003  li r11, 3
	ctx.r[11].s64 = 3;
	// 8328A6B0: 3C80820F  lis r4, -0x7df1
	ctx.r[4].s64 = -2112946176;
	// 8328A6B4: 917F0664  stw r11, 0x664(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(1636 as u32), ctx.r[11].u32 ) };
	// 8328A6B8: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8328A6BC: 3BC447B8  addi r30, r4, 0x47b8
	ctx.r[30].s64 = ctx.r[4].s64 + 18360;
	// 8328A6C0: 387F0668  addi r3, r31, 0x668
	ctx.r[3].s64 = ctx.r[31].s64 + 1640;
	// 8328A6C4: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8328A6C8: 4AFA2809  bl 0x8222ced0
	ctx.lr = 0x8328A6CC;
	sub_8222CED0(ctx, base);
	// 8328A6CC: 3C60820F  lis r3, -0x7df1
	ctx.r[3].s64 = -2112946176;
	// 8328A6D0: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8328A6D4: 38835638  addi r4, r3, 0x5638
	ctx.r[4].s64 = ctx.r[3].s64 + 22072;
	// 8328A6D8: 387F066C  addi r3, r31, 0x66c
	ctx.r[3].s64 = ctx.r[31].s64 + 1644;
	// 8328A6DC: 4AFA27F5  bl 0x8222ced0
	ctx.lr = 0x8328A6E0;
	sub_8222CED0(ctx, base);
	// 8328A6E0: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 8328A6E4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8328A6E8: 388B561C  addi r4, r11, 0x561c
	ctx.r[4].s64 = ctx.r[11].s64 + 22044;
	// 8328A6EC: 387F0670  addi r3, r31, 0x670
	ctx.r[3].s64 = ctx.r[31].s64 + 1648;
	// 8328A6F0: 4AFA27E1  bl 0x8222ced0
	ctx.lr = 0x8328A6F4;
	sub_8222CED0(ctx, base);
	// 8328A6F4: 39600003  li r11, 3
	ctx.r[11].s64 = 3;
	// 8328A6F8: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8328A6FC: 917F0674  stw r11, 0x674(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(1652 as u32), ctx.r[11].u32 ) };
	// 8328A700: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8328A704: 387F0678  addi r3, r31, 0x678
	ctx.r[3].s64 = ctx.r[31].s64 + 1656;
	// 8328A708: 4AFA27C9  bl 0x8222ced0
	ctx.lr = 0x8328A70C;
	sub_8222CED0(ctx, base);
	// 8328A70C: 3D40820F  lis r10, -0x7df1
	ctx.r[10].s64 = -2112946176;
	// 8328A710: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8328A714: 388A5608  addi r4, r10, 0x5608
	ctx.r[4].s64 = ctx.r[10].s64 + 22024;
	// 8328A718: 387F067C  addi r3, r31, 0x67c
	ctx.r[3].s64 = ctx.r[31].s64 + 1660;
	// 8328A71C: 4AFA27B5  bl 0x8222ced0
	ctx.lr = 0x8328A720;
	sub_8222CED0(ctx, base);
	// 8328A720: 3D20820F  lis r9, -0x7df1
	ctx.r[9].s64 = -2112946176;
	// 8328A724: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8328A728: 388955EC  addi r4, r9, 0x55ec
	ctx.r[4].s64 = ctx.r[9].s64 + 21996;
	// 8328A72C: 387F0680  addi r3, r31, 0x680
	ctx.r[3].s64 = ctx.r[31].s64 + 1664;
	// 8328A730: 4AFA27A1  bl 0x8222ced0
	ctx.lr = 0x8328A734;
	sub_8222CED0(ctx, base);
	// 8328A734: 39600003  li r11, 3
	ctx.r[11].s64 = 3;
	// 8328A738: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8328A73C: 917F0684  stw r11, 0x684(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(1668 as u32), ctx.r[11].u32 ) };
	// 8328A740: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8328A744: 387F0688  addi r3, r31, 0x688
	ctx.r[3].s64 = ctx.r[31].s64 + 1672;
	// 8328A748: 4AFA2789  bl 0x8222ced0
	ctx.lr = 0x8328A74C;
	sub_8222CED0(ctx, base);
	// 8328A74C: 3D00820F  lis r8, -0x7df1
	ctx.r[8].s64 = -2112946176;
	// 8328A750: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8328A754: 388855D8  addi r4, r8, 0x55d8
	ctx.r[4].s64 = ctx.r[8].s64 + 21976;
	// 8328A758: 387F068C  addi r3, r31, 0x68c
	ctx.r[3].s64 = ctx.r[31].s64 + 1676;
	// 8328A75C: 4AFA2775  bl 0x8222ced0
	ctx.lr = 0x8328A760;
	sub_8222CED0(ctx, base);
	// 8328A760: 3CE0820F  lis r7, -0x7df1
	ctx.r[7].s64 = -2112946176;
	// 8328A764: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8328A768: 388755BC  addi r4, r7, 0x55bc
	ctx.r[4].s64 = ctx.r[7].s64 + 21948;
	// 8328A76C: 387F0690  addi r3, r31, 0x690
	ctx.r[3].s64 = ctx.r[31].s64 + 1680;
	// 8328A770: 4AFA2761  bl 0x8222ced0
	ctx.lr = 0x8328A774;
	sub_8222CED0(ctx, base);
	// 8328A774: 39600003  li r11, 3
	ctx.r[11].s64 = 3;
	// 8328A778: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8328A77C: 917F0694  stw r11, 0x694(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(1684 as u32), ctx.r[11].u32 ) };
	// 8328A780: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8328A784: 387F0698  addi r3, r31, 0x698
	ctx.r[3].s64 = ctx.r[31].s64 + 1688;
	// 8328A788: 4AFA2749  bl 0x8222ced0
	ctx.lr = 0x8328A78C;
	sub_8222CED0(ctx, base);
	// 8328A78C: 3CC0820F  lis r6, -0x7df1
	ctx.r[6].s64 = -2112946176;
	// 8328A790: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8328A794: 388655A8  addi r4, r6, 0x55a8
	ctx.r[4].s64 = ctx.r[6].s64 + 21928;
	// 8328A798: 387F069C  addi r3, r31, 0x69c
	ctx.r[3].s64 = ctx.r[31].s64 + 1692;
	// 8328A79C: 4AFA2735  bl 0x8222ced0
	ctx.lr = 0x8328A7A0;
	sub_8222CED0(ctx, base);
	// 8328A7A0: 3C80820F  lis r4, -0x7df1
	ctx.r[4].s64 = -2112946176;
	// 8328A7A4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8328A7A8: 3884558C  addi r4, r4, 0x558c
	ctx.r[4].s64 = ctx.r[4].s64 + 21900;
	// 8328A7AC: 387F06A0  addi r3, r31, 0x6a0
	ctx.r[3].s64 = ctx.r[31].s64 + 1696;
	// 8328A7B0: 4AFA2721  bl 0x8222ced0
	ctx.lr = 0x8328A7B4;
	sub_8222CED0(ctx, base);
	// 8328A7B4: 39600003  li r11, 3
	ctx.r[11].s64 = 3;
	// 8328A7B8: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8328A7BC: 917F06A4  stw r11, 0x6a4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(1700 as u32), ctx.r[11].u32 ) };
	// 8328A7C0: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8328A7C4: 387F06A8  addi r3, r31, 0x6a8
	ctx.r[3].s64 = ctx.r[31].s64 + 1704;
	// 8328A7C8: 4AFA2709  bl 0x8222ced0
	ctx.lr = 0x8328A7CC;
	sub_8222CED0(ctx, base);
	// 8328A7CC: 3C60820F  lis r3, -0x7df1
	ctx.r[3].s64 = -2112946176;
	// 8328A7D0: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8328A7D4: 38835578  addi r4, r3, 0x5578
	ctx.r[4].s64 = ctx.r[3].s64 + 21880;
	// 8328A7D8: 387F06AC  addi r3, r31, 0x6ac
	ctx.r[3].s64 = ctx.r[31].s64 + 1708;
	// 8328A7DC: 4AFA26F5  bl 0x8222ced0
	ctx.lr = 0x8328A7E0;
	sub_8222CED0(ctx, base);
	// 8328A7E0: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 8328A7E4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8328A7E8: 388B5560  addi r4, r11, 0x5560
	ctx.r[4].s64 = ctx.r[11].s64 + 21856;
	// 8328A7EC: 387F06B0  addi r3, r31, 0x6b0
	ctx.r[3].s64 = ctx.r[31].s64 + 1712;
	// 8328A7F0: 4AFA26E1  bl 0x8222ced0
	ctx.lr = 0x8328A7F4;
	sub_8222CED0(ctx, base);
	// 8328A7F4: 39600003  li r11, 3
	ctx.r[11].s64 = 3;
	// 8328A7F8: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8328A7FC: 917F06B4  stw r11, 0x6b4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(1716 as u32), ctx.r[11].u32 ) };
	// 8328A800: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8328A804: 387F06B8  addi r3, r31, 0x6b8
	ctx.r[3].s64 = ctx.r[31].s64 + 1720;
	// 8328A808: 4AFA26C9  bl 0x8222ced0
	ctx.lr = 0x8328A80C;
	sub_8222CED0(ctx, base);
	// 8328A80C: 3D40820F  lis r10, -0x7df1
	ctx.r[10].s64 = -2112946176;
	// 8328A810: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8328A814: 388A5550  addi r4, r10, 0x5550
	ctx.r[4].s64 = ctx.r[10].s64 + 21840;
	// 8328A818: 387F06BC  addi r3, r31, 0x6bc
	ctx.r[3].s64 = ctx.r[31].s64 + 1724;
	// 8328A81C: 4AFA26B5  bl 0x8222ced0
	ctx.lr = 0x8328A820;
	sub_8222CED0(ctx, base);
	// 8328A820: 3D20820F  lis r9, -0x7df1
	ctx.r[9].s64 = -2112946176;
	// 8328A824: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8328A828: 38895538  addi r4, r9, 0x5538
	ctx.r[4].s64 = ctx.r[9].s64 + 21816;
	// 8328A82C: 387F06C0  addi r3, r31, 0x6c0
	ctx.r[3].s64 = ctx.r[31].s64 + 1728;
	// 8328A830: 4AFA26A1  bl 0x8222ced0
	ctx.lr = 0x8328A834;
	sub_8222CED0(ctx, base);
	// 8328A834: 39600003  li r11, 3
	ctx.r[11].s64 = 3;
	// 8328A838: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8328A83C: 917F06C4  stw r11, 0x6c4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(1732 as u32), ctx.r[11].u32 ) };
	// 8328A840: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8328A844: 387F06C8  addi r3, r31, 0x6c8
	ctx.r[3].s64 = ctx.r[31].s64 + 1736;
	// 8328A848: 4AFA2689  bl 0x8222ced0
	ctx.lr = 0x8328A84C;
	sub_8222CED0(ctx, base);
	// 8328A84C: 3D00820F  lis r8, -0x7df1
	ctx.r[8].s64 = -2112946176;
	// 8328A850: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8328A854: 38885528  addi r4, r8, 0x5528
	ctx.r[4].s64 = ctx.r[8].s64 + 21800;
	// 8328A858: 387F06CC  addi r3, r31, 0x6cc
	ctx.r[3].s64 = ctx.r[31].s64 + 1740;
	// 8328A85C: 4AFA2675  bl 0x8222ced0
	ctx.lr = 0x8328A860;
	sub_8222CED0(ctx, base);
	// 8328A860: 3CE0820F  lis r7, -0x7df1
	ctx.r[7].s64 = -2112946176;
	// 8328A864: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8328A868: 38875510  addi r4, r7, 0x5510
	ctx.r[4].s64 = ctx.r[7].s64 + 21776;
	// 8328A86C: 387F06D0  addi r3, r31, 0x6d0
	ctx.r[3].s64 = ctx.r[31].s64 + 1744;
	// 8328A870: 4AFA2661  bl 0x8222ced0
	ctx.lr = 0x8328A874;
	sub_8222CED0(ctx, base);
	// 8328A874: 39600003  li r11, 3
	ctx.r[11].s64 = 3;
	// 8328A878: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8328A87C: 917F06D4  stw r11, 0x6d4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(1748 as u32), ctx.r[11].u32 ) };
	// 8328A880: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8328A884: 387F06D8  addi r3, r31, 0x6d8
	ctx.r[3].s64 = ctx.r[31].s64 + 1752;
	// 8328A888: 4AFA2649  bl 0x8222ced0
	ctx.lr = 0x8328A88C;
	sub_8222CED0(ctx, base);
	// 8328A88C: 3CC0820F  lis r6, -0x7df1
	ctx.r[6].s64 = -2112946176;
	// 8328A890: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8328A894: 38865500  addi r4, r6, 0x5500
	ctx.r[4].s64 = ctx.r[6].s64 + 21760;
	// 8328A898: 387F06DC  addi r3, r31, 0x6dc
	ctx.r[3].s64 = ctx.r[31].s64 + 1756;
	// 8328A89C: 4AFA2635  bl 0x8222ced0
	ctx.lr = 0x8328A8A0;
	sub_8222CED0(ctx, base);
	// 8328A8A0: 3C80820F  lis r4, -0x7df1
	ctx.r[4].s64 = -2112946176;
	// 8328A8A4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8328A8A8: 388454E8  addi r4, r4, 0x54e8
	ctx.r[4].s64 = ctx.r[4].s64 + 21736;
	// 8328A8AC: 387F06E0  addi r3, r31, 0x6e0
	ctx.r[3].s64 = ctx.r[31].s64 + 1760;
	// 8328A8B0: 4AFA2621  bl 0x8222ced0
	ctx.lr = 0x8328A8B4;
	sub_8222CED0(ctx, base);
	// 8328A8B4: 39600003  li r11, 3
	ctx.r[11].s64 = 3;
	// 8328A8B8: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8328A8BC: 917F06E4  stw r11, 0x6e4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(1764 as u32), ctx.r[11].u32 ) };
	// 8328A8C0: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8328A8C4: 387F06E8  addi r3, r31, 0x6e8
	ctx.r[3].s64 = ctx.r[31].s64 + 1768;
	// 8328A8C8: 4AFA2609  bl 0x8222ced0
	ctx.lr = 0x8328A8CC;
	sub_8222CED0(ctx, base);
	// 8328A8CC: 3C60820F  lis r3, -0x7df1
	ctx.r[3].s64 = -2112946176;
	// 8328A8D0: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8328A8D4: 388354D8  addi r4, r3, 0x54d8
	ctx.r[4].s64 = ctx.r[3].s64 + 21720;
	// 8328A8D8: 387F06EC  addi r3, r31, 0x6ec
	ctx.r[3].s64 = ctx.r[31].s64 + 1772;
	// 8328A8DC: 4AFA25F5  bl 0x8222ced0
	ctx.lr = 0x8328A8E0;
	sub_8222CED0(ctx, base);
	// 8328A8E0: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 8328A8E4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8328A8E8: 388B54C0  addi r4, r11, 0x54c0
	ctx.r[4].s64 = ctx.r[11].s64 + 21696;
	// 8328A8EC: 387F06F0  addi r3, r31, 0x6f0
	ctx.r[3].s64 = ctx.r[31].s64 + 1776;
	// 8328A8F0: 4AFA25E1  bl 0x8222ced0
	ctx.lr = 0x8328A8F4;
	sub_8222CED0(ctx, base);
	// 8328A8F4: 39600003  li r11, 3
	ctx.r[11].s64 = 3;
	// 8328A8F8: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8328A8FC: 917F06F4  stw r11, 0x6f4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(1780 as u32), ctx.r[11].u32 ) };
	// 8328A900: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8328A904: 387F06F8  addi r3, r31, 0x6f8
	ctx.r[3].s64 = ctx.r[31].s64 + 1784;
	// 8328A908: 4AFA25C9  bl 0x8222ced0
	ctx.lr = 0x8328A90C;
	sub_8222CED0(ctx, base);
	// 8328A90C: 3D40820F  lis r10, -0x7df1
	ctx.r[10].s64 = -2112946176;
	// 8328A910: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8328A914: 388A54B0  addi r4, r10, 0x54b0
	ctx.r[4].s64 = ctx.r[10].s64 + 21680;
	// 8328A918: 387F06FC  addi r3, r31, 0x6fc
	ctx.r[3].s64 = ctx.r[31].s64 + 1788;
	// 8328A91C: 4AFA25B5  bl 0x8222ced0
	ctx.lr = 0x8328A920;
	sub_8222CED0(ctx, base);
	// 8328A920: 3D20820F  lis r9, -0x7df1
	ctx.r[9].s64 = -2112946176;
	// 8328A924: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8328A928: 38895498  addi r4, r9, 0x5498
	ctx.r[4].s64 = ctx.r[9].s64 + 21656;
	// 8328A92C: 387F0700  addi r3, r31, 0x700
	ctx.r[3].s64 = ctx.r[31].s64 + 1792;
	// 8328A930: 4AFA25A1  bl 0x8222ced0
	ctx.lr = 0x8328A934;
	sub_8222CED0(ctx, base);
	// 8328A934: 39600003  li r11, 3
	ctx.r[11].s64 = 3;
	// 8328A938: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8328A93C: 917F0704  stw r11, 0x704(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(1796 as u32), ctx.r[11].u32 ) };
	// 8328A940: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8328A944: 387F0708  addi r3, r31, 0x708
	ctx.r[3].s64 = ctx.r[31].s64 + 1800;
	// 8328A948: 4AFA2589  bl 0x8222ced0
	ctx.lr = 0x8328A94C;
	sub_8222CED0(ctx, base);
	// 8328A94C: 3D00820F  lis r8, -0x7df1
	ctx.r[8].s64 = -2112946176;
	// 8328A950: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8328A954: 38885488  addi r4, r8, 0x5488
	ctx.r[4].s64 = ctx.r[8].s64 + 21640;
	// 8328A958: 387F070C  addi r3, r31, 0x70c
	ctx.r[3].s64 = ctx.r[31].s64 + 1804;
	// 8328A95C: 4AFA2575  bl 0x8222ced0
	ctx.lr = 0x8328A960;
	sub_8222CED0(ctx, base);
	// 8328A960: 3CE0820F  lis r7, -0x7df1
	ctx.r[7].s64 = -2112946176;
	// 8328A964: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8328A968: 38875470  addi r4, r7, 0x5470
	ctx.r[4].s64 = ctx.r[7].s64 + 21616;
	// 8328A96C: 387F0710  addi r3, r31, 0x710
	ctx.r[3].s64 = ctx.r[31].s64 + 1808;
	// 8328A970: 4AFA2561  bl 0x8222ced0
	ctx.lr = 0x8328A974;
	sub_8222CED0(ctx, base);
	// 8328A974: 39600003  li r11, 3
	ctx.r[11].s64 = 3;
	// 8328A978: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8328A97C: 917F0714  stw r11, 0x714(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(1812 as u32), ctx.r[11].u32 ) };
	// 8328A980: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8328A984: 387F0718  addi r3, r31, 0x718
	ctx.r[3].s64 = ctx.r[31].s64 + 1816;
	// 8328A988: 4AFA2549  bl 0x8222ced0
	ctx.lr = 0x8328A98C;
	sub_8222CED0(ctx, base);
	// 8328A98C: 3CC0820F  lis r6, -0x7df1
	ctx.r[6].s64 = -2112946176;
	// 8328A990: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8328A994: 38865460  addi r4, r6, 0x5460
	ctx.r[4].s64 = ctx.r[6].s64 + 21600;
	// 8328A998: 387F071C  addi r3, r31, 0x71c
	ctx.r[3].s64 = ctx.r[31].s64 + 1820;
	// 8328A99C: 4AFA2535  bl 0x8222ced0
	ctx.lr = 0x8328A9A0;
	sub_8222CED0(ctx, base);
	// 8328A9A0: 3C80820F  lis r4, -0x7df1
	ctx.r[4].s64 = -2112946176;
	// 8328A9A4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8328A9A8: 38845448  addi r4, r4, 0x5448
	ctx.r[4].s64 = ctx.r[4].s64 + 21576;
	// 8328A9AC: 387F0720  addi r3, r31, 0x720
	ctx.r[3].s64 = ctx.r[31].s64 + 1824;
	// 8328A9B0: 4AFA2521  bl 0x8222ced0
	ctx.lr = 0x8328A9B4;
	sub_8222CED0(ctx, base);
	// 8328A9B4: 39600003  li r11, 3
	ctx.r[11].s64 = 3;
	// 8328A9B8: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8328A9BC: 917F0724  stw r11, 0x724(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(1828 as u32), ctx.r[11].u32 ) };
	// 8328A9C0: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8328A9C4: 387F0728  addi r3, r31, 0x728
	ctx.r[3].s64 = ctx.r[31].s64 + 1832;
	// 8328A9C8: 4AFA2509  bl 0x8222ced0
	ctx.lr = 0x8328A9CC;
	sub_8222CED0(ctx, base);
	// 8328A9CC: 3C60820F  lis r3, -0x7df1
	ctx.r[3].s64 = -2112946176;
	// 8328A9D0: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8328A9D4: 38835438  addi r4, r3, 0x5438
	ctx.r[4].s64 = ctx.r[3].s64 + 21560;
	// 8328A9D8: 387F072C  addi r3, r31, 0x72c
	ctx.r[3].s64 = ctx.r[31].s64 + 1836;
	// 8328A9DC: 4AFA24F5  bl 0x8222ced0
	ctx.lr = 0x8328A9E0;
	sub_8222CED0(ctx, base);
	// 8328A9E0: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 8328A9E4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8328A9E8: 388B5420  addi r4, r11, 0x5420
	ctx.r[4].s64 = ctx.r[11].s64 + 21536;
	// 8328A9EC: 387F0730  addi r3, r31, 0x730
	ctx.r[3].s64 = ctx.r[31].s64 + 1840;
	// 8328A9F0: 4AFA24E1  bl 0x8222ced0
	ctx.lr = 0x8328A9F4;
	sub_8222CED0(ctx, base);
	// 8328A9F4: 39600003  li r11, 3
	ctx.r[11].s64 = 3;
	// 8328A9F8: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8328A9FC: 917F0734  stw r11, 0x734(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(1844 as u32), ctx.r[11].u32 ) };
	// 8328AA00: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8328AA04: 387F0738  addi r3, r31, 0x738
	ctx.r[3].s64 = ctx.r[31].s64 + 1848;
	// 8328AA08: 4AFA24C9  bl 0x8222ced0
	ctx.lr = 0x8328AA0C;
	sub_8222CED0(ctx, base);
	// 8328AA0C: 3D40820F  lis r10, -0x7df1
	ctx.r[10].s64 = -2112946176;
	// 8328AA10: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8328AA14: 388A5410  addi r4, r10, 0x5410
	ctx.r[4].s64 = ctx.r[10].s64 + 21520;
	// 8328AA18: 387F073C  addi r3, r31, 0x73c
	ctx.r[3].s64 = ctx.r[31].s64 + 1852;
	// 8328AA1C: 4AFA24B5  bl 0x8222ced0
	ctx.lr = 0x8328AA20;
	sub_8222CED0(ctx, base);
	// 8328AA20: 3D20820F  lis r9, -0x7df1
	ctx.r[9].s64 = -2112946176;
	// 8328AA24: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8328AA28: 388953F8  addi r4, r9, 0x53f8
	ctx.r[4].s64 = ctx.r[9].s64 + 21496;
	// 8328AA2C: 387F0740  addi r3, r31, 0x740
	ctx.r[3].s64 = ctx.r[31].s64 + 1856;
	// 8328AA30: 4AFA24A1  bl 0x8222ced0
	ctx.lr = 0x8328AA34;
	sub_8222CED0(ctx, base);
	// 8328AA34: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 8328AA38: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8328AA3C: 917F0744  stw r11, 0x744(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(1860 as u32), ctx.r[11].u32 ) };
	// 8328AA40: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8328AA44: 387F0748  addi r3, r31, 0x748
	ctx.r[3].s64 = ctx.r[31].s64 + 1864;
	// 8328AA48: 4AFA2489  bl 0x8222ced0
	ctx.lr = 0x8328AA4C;
	sub_8222CED0(ctx, base);
	// 8328AA4C: 3D00820F  lis r8, -0x7df1
	ctx.r[8].s64 = -2112946176;
	// 8328AA50: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8328AA54: 388853E4  addi r4, r8, 0x53e4
	ctx.r[4].s64 = ctx.r[8].s64 + 21476;
	// 8328AA58: 387F074C  addi r3, r31, 0x74c
	ctx.r[3].s64 = ctx.r[31].s64 + 1868;
	// 8328AA5C: 4AFA2475  bl 0x8222ced0
	ctx.lr = 0x8328AA60;
	sub_8222CED0(ctx, base);
	// 8328AA60: 3CE0820F  lis r7, -0x7df1
	ctx.r[7].s64 = -2112946176;
	// 8328AA64: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8328AA68: 388753D0  addi r4, r7, 0x53d0
	ctx.r[4].s64 = ctx.r[7].s64 + 21456;
	// 8328AA6C: 387F0750  addi r3, r31, 0x750
	ctx.r[3].s64 = ctx.r[31].s64 + 1872;
	// 8328AA70: 4AFA2461  bl 0x8222ced0
	ctx.lr = 0x8328AA74;
	sub_8222CED0(ctx, base);
	// 8328AA74: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 8328AA78: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8328AA7C: 917F0754  stw r11, 0x754(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(1876 as u32), ctx.r[11].u32 ) };
	// 8328AA80: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8328AA84: 387F0758  addi r3, r31, 0x758
	ctx.r[3].s64 = ctx.r[31].s64 + 1880;
	// 8328AA88: 4AFA2449  bl 0x8222ced0
	ctx.lr = 0x8328AA8C;
	sub_8222CED0(ctx, base);
	// 8328AA8C: 3CC0820F  lis r6, -0x7df1
	ctx.r[6].s64 = -2112946176;
	// 8328AA90: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8328AA94: 388653C0  addi r4, r6, 0x53c0
	ctx.r[4].s64 = ctx.r[6].s64 + 21440;
	// 8328AA98: 387F075C  addi r3, r31, 0x75c
	ctx.r[3].s64 = ctx.r[31].s64 + 1884;
	// 8328AA9C: 4AFA2435  bl 0x8222ced0
	ctx.lr = 0x8328AAA0;
	sub_8222CED0(ctx, base);
	// 8328AAA0: 3C80820F  lis r4, -0x7df1
	ctx.r[4].s64 = -2112946176;
	// 8328AAA4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8328AAA8: 388453AC  addi r4, r4, 0x53ac
	ctx.r[4].s64 = ctx.r[4].s64 + 21420;
	// 8328AAAC: 387F0760  addi r3, r31, 0x760
	ctx.r[3].s64 = ctx.r[31].s64 + 1888;
	// 8328AAB0: 4AFA2421  bl 0x8222ced0
	ctx.lr = 0x8328AAB4;
	sub_8222CED0(ctx, base);
	// 8328AAB4: 39600003  li r11, 3
	ctx.r[11].s64 = 3;
	// 8328AAB8: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8328AABC: 917F0764  stw r11, 0x764(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(1892 as u32), ctx.r[11].u32 ) };
	// 8328AAC0: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8328AAC4: 387F0768  addi r3, r31, 0x768
	ctx.r[3].s64 = ctx.r[31].s64 + 1896;
	// 8328AAC8: 4AFA2409  bl 0x8222ced0
	ctx.lr = 0x8328AACC;
	sub_8222CED0(ctx, base);
	// 8328AACC: 3C60820F  lis r3, -0x7df1
	ctx.r[3].s64 = -2112946176;
	// 8328AAD0: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8328AAD4: 388353A0  addi r4, r3, 0x53a0
	ctx.r[4].s64 = ctx.r[3].s64 + 21408;
	// 8328AAD8: 387F076C  addi r3, r31, 0x76c
	ctx.r[3].s64 = ctx.r[31].s64 + 1900;
	// 8328AADC: 4AFA23F5  bl 0x8222ced0
	ctx.lr = 0x8328AAE0;
	sub_8222CED0(ctx, base);
	// 8328AAE0: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 8328AAE4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8328AAE8: 388B538C  addi r4, r11, 0x538c
	ctx.r[4].s64 = ctx.r[11].s64 + 21388;
	// 8328AAEC: 387F0770  addi r3, r31, 0x770
	ctx.r[3].s64 = ctx.r[31].s64 + 1904;
	// 8328AAF0: 4AFA23E1  bl 0x8222ced0
	ctx.lr = 0x8328AAF4;
	sub_8222CED0(ctx, base);
	// 8328AAF4: 39600003  li r11, 3
	ctx.r[11].s64 = 3;
	// 8328AAF8: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8328AAFC: 917F0774  stw r11, 0x774(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(1908 as u32), ctx.r[11].u32 ) };
	// 8328AB00: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8328AB04: 387F0778  addi r3, r31, 0x778
	ctx.r[3].s64 = ctx.r[31].s64 + 1912;
	// 8328AB08: 4AFA23C9  bl 0x8222ced0
	ctx.lr = 0x8328AB0C;
	sub_8222CED0(ctx, base);
	// 8328AB0C: 3D40820F  lis r10, -0x7df1
	ctx.r[10].s64 = -2112946176;
	// 8328AB10: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8328AB14: 388A5380  addi r4, r10, 0x5380
	ctx.r[4].s64 = ctx.r[10].s64 + 21376;
	// 8328AB18: 387F077C  addi r3, r31, 0x77c
	ctx.r[3].s64 = ctx.r[31].s64 + 1916;
	// 8328AB1C: 4AFA23B5  bl 0x8222ced0
	ctx.lr = 0x8328AB20;
	sub_8222CED0(ctx, base);
	// 8328AB20: 3D20820F  lis r9, -0x7df1
	ctx.r[9].s64 = -2112946176;
	// 8328AB24: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8328AB28: 38895368  addi r4, r9, 0x5368
	ctx.r[4].s64 = ctx.r[9].s64 + 21352;
	// 8328AB2C: 387F0780  addi r3, r31, 0x780
	ctx.r[3].s64 = ctx.r[31].s64 + 1920;
	// 8328AB30: 4AFA23A1  bl 0x8222ced0
	ctx.lr = 0x8328AB34;
	sub_8222CED0(ctx, base);
	// 8328AB34: 39600003  li r11, 3
	ctx.r[11].s64 = 3;
	// 8328AB38: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8328AB3C: 917F0784  stw r11, 0x784(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(1924 as u32), ctx.r[11].u32 ) };
	// 8328AB40: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8328AB44: 387F0788  addi r3, r31, 0x788
	ctx.r[3].s64 = ctx.r[31].s64 + 1928;
	// 8328AB48: 4AFA2389  bl 0x8222ced0
	ctx.lr = 0x8328AB4C;
	sub_8222CED0(ctx, base);
	// 8328AB4C: 3D00820F  lis r8, -0x7df1
	ctx.r[8].s64 = -2112946176;
	// 8328AB50: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8328AB54: 38885358  addi r4, r8, 0x5358
	ctx.r[4].s64 = ctx.r[8].s64 + 21336;
	// 8328AB58: 387F078C  addi r3, r31, 0x78c
	ctx.r[3].s64 = ctx.r[31].s64 + 1932;
	// 8328AB5C: 4AFA2375  bl 0x8222ced0
	ctx.lr = 0x8328AB60;
	sub_8222CED0(ctx, base);
	// 8328AB60: 3CE0820F  lis r7, -0x7df1
	ctx.r[7].s64 = -2112946176;
	// 8328AB64: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8328AB68: 38875340  addi r4, r7, 0x5340
	ctx.r[4].s64 = ctx.r[7].s64 + 21312;
	// 8328AB6C: 387F0790  addi r3, r31, 0x790
	ctx.r[3].s64 = ctx.r[31].s64 + 1936;
	// 8328AB70: 4AFA2361  bl 0x8222ced0
	ctx.lr = 0x8328AB74;
	sub_8222CED0(ctx, base);
	// 8328AB74: 39600003  li r11, 3
	ctx.r[11].s64 = 3;
	// 8328AB78: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8328AB7C: 917F0794  stw r11, 0x794(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(1940 as u32), ctx.r[11].u32 ) };
	// 8328AB80: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8328AB84: 387F0798  addi r3, r31, 0x798
	ctx.r[3].s64 = ctx.r[31].s64 + 1944;
	// 8328AB88: 4AFA2349  bl 0x8222ced0
	ctx.lr = 0x8328AB8C;
	sub_8222CED0(ctx, base);
	// 8328AB8C: 3CC0820F  lis r6, -0x7df1
	ctx.r[6].s64 = -2112946176;
	// 8328AB90: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8328AB94: 38865330  addi r4, r6, 0x5330
	ctx.r[4].s64 = ctx.r[6].s64 + 21296;
	// 8328AB98: 387F079C  addi r3, r31, 0x79c
	ctx.r[3].s64 = ctx.r[31].s64 + 1948;
	// 8328AB9C: 4AFA2335  bl 0x8222ced0
	ctx.lr = 0x8328ABA0;
	sub_8222CED0(ctx, base);
	// 8328ABA0: 3C80820F  lis r4, -0x7df1
	ctx.r[4].s64 = -2112946176;
	// 8328ABA4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8328ABA8: 38845318  addi r4, r4, 0x5318
	ctx.r[4].s64 = ctx.r[4].s64 + 21272;
	// 8328ABAC: 387F07A0  addi r3, r31, 0x7a0
	ctx.r[3].s64 = ctx.r[31].s64 + 1952;
	// 8328ABB0: 4AFA2321  bl 0x8222ced0
	ctx.lr = 0x8328ABB4;
	sub_8222CED0(ctx, base);
	// 8328ABB4: 39600003  li r11, 3
	ctx.r[11].s64 = 3;
	// 8328ABB8: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8328ABBC: 917F07A4  stw r11, 0x7a4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(1956 as u32), ctx.r[11].u32 ) };
	// 8328ABC0: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8328ABC4: 387F07A8  addi r3, r31, 0x7a8
	ctx.r[3].s64 = ctx.r[31].s64 + 1960;
	// 8328ABC8: 4AFA2309  bl 0x8222ced0
	ctx.lr = 0x8328ABCC;
	sub_8222CED0(ctx, base);
	// 8328ABCC: 3C60820F  lis r3, -0x7df1
	ctx.r[3].s64 = -2112946176;
	// 8328ABD0: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8328ABD4: 38835308  addi r4, r3, 0x5308
	ctx.r[4].s64 = ctx.r[3].s64 + 21256;
	// 8328ABD8: 387F07AC  addi r3, r31, 0x7ac
	ctx.r[3].s64 = ctx.r[31].s64 + 1964;
	// 8328ABDC: 4AFA22F5  bl 0x8222ced0
	ctx.lr = 0x8328ABE0;
	sub_8222CED0(ctx, base);
	// 8328ABE0: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 8328ABE4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8328ABE8: 388B52EC  addi r4, r11, 0x52ec
	ctx.r[4].s64 = ctx.r[11].s64 + 21228;
	// 8328ABEC: 387F07B0  addi r3, r31, 0x7b0
	ctx.r[3].s64 = ctx.r[31].s64 + 1968;
	// 8328ABF0: 4AFA22E1  bl 0x8222ced0
	ctx.lr = 0x8328ABF4;
	sub_8222CED0(ctx, base);
	// 8328ABF4: 39600003  li r11, 3
	ctx.r[11].s64 = 3;
	// 8328ABF8: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8328ABFC: 917F07B4  stw r11, 0x7b4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(1972 as u32), ctx.r[11].u32 ) };
	// 8328AC00: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8328AC04: 387F07B8  addi r3, r31, 0x7b8
	ctx.r[3].s64 = ctx.r[31].s64 + 1976;
	// 8328AC08: 4AFA22C9  bl 0x8222ced0
	ctx.lr = 0x8328AC0C;
	sub_8222CED0(ctx, base);
	// 8328AC0C: 3D40820F  lis r10, -0x7df1
	ctx.r[10].s64 = -2112946176;
	// 8328AC10: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8328AC14: 388A52D8  addi r4, r10, 0x52d8
	ctx.r[4].s64 = ctx.r[10].s64 + 21208;
	// 8328AC18: 387F07BC  addi r3, r31, 0x7bc
	ctx.r[3].s64 = ctx.r[31].s64 + 1980;
	// 8328AC1C: 4AFA22B5  bl 0x8222ced0
	ctx.lr = 0x8328AC20;
	sub_8222CED0(ctx, base);
	// 8328AC20: 3D20820F  lis r9, -0x7df1
	ctx.r[9].s64 = -2112946176;
	// 8328AC24: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8328AC28: 388952B4  addi r4, r9, 0x52b4
	ctx.r[4].s64 = ctx.r[9].s64 + 21172;
	// 8328AC2C: 387F07C0  addi r3, r31, 0x7c0
	ctx.r[3].s64 = ctx.r[31].s64 + 1984;
	// 8328AC30: 4AFA22A1  bl 0x8222ced0
	ctx.lr = 0x8328AC34;
	sub_8222CED0(ctx, base);
	// 8328AC34: 39600003  li r11, 3
	ctx.r[11].s64 = 3;
	// 8328AC38: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8328AC3C: 917F07C4  stw r11, 0x7c4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(1988 as u32), ctx.r[11].u32 ) };
	// 8328AC40: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8328AC44: 387F07C8  addi r3, r31, 0x7c8
	ctx.r[3].s64 = ctx.r[31].s64 + 1992;
	// 8328AC48: 4AFA2289  bl 0x8222ced0
	ctx.lr = 0x8328AC4C;
	sub_8222CED0(ctx, base);
	// 8328AC4C: 3D00820F  lis r8, -0x7df1
	ctx.r[8].s64 = -2112946176;
	// 8328AC50: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8328AC54: 38885298  addi r4, r8, 0x5298
	ctx.r[4].s64 = ctx.r[8].s64 + 21144;
	// 8328AC58: 387F07CC  addi r3, r31, 0x7cc
	ctx.r[3].s64 = ctx.r[31].s64 + 1996;
	// 8328AC5C: 4AFA2275  bl 0x8222ced0
	ctx.lr = 0x8328AC60;
	sub_8222CED0(ctx, base);
	// 8328AC60: 3CE0820F  lis r7, -0x7df1
	ctx.r[7].s64 = -2112946176;
	// 8328AC64: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8328AC68: 3887527C  addi r4, r7, 0x527c
	ctx.r[4].s64 = ctx.r[7].s64 + 21116;
	// 8328AC6C: 387F07D0  addi r3, r31, 0x7d0
	ctx.r[3].s64 = ctx.r[31].s64 + 2000;
	// 8328AC70: 4AFA2261  bl 0x8222ced0
	ctx.lr = 0x8328AC74;
	sub_8222CED0(ctx, base);
	// 8328AC74: 39600003  li r11, 3
	ctx.r[11].s64 = 3;
	// 8328AC78: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8328AC7C: 917F07D4  stw r11, 0x7d4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(2004 as u32), ctx.r[11].u32 ) };
	// 8328AC80: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8328AC84: 387F07D8  addi r3, r31, 0x7d8
	ctx.r[3].s64 = ctx.r[31].s64 + 2008;
	// 8328AC88: 4AFA2249  bl 0x8222ced0
	ctx.lr = 0x8328AC8C;
	sub_8222CED0(ctx, base);
	// 8328AC8C: 3CC0820F  lis r6, -0x7df1
	ctx.r[6].s64 = -2112946176;
	// 8328AC90: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8328AC94: 38865268  addi r4, r6, 0x5268
	ctx.r[4].s64 = ctx.r[6].s64 + 21096;
	// 8328AC98: 387F07DC  addi r3, r31, 0x7dc
	ctx.r[3].s64 = ctx.r[31].s64 + 2012;
	// 8328AC9C: 4AFA2235  bl 0x8222ced0
	ctx.lr = 0x8328ACA0;
	sub_8222CED0(ctx, base);
	// 8328ACA0: 3C80820F  lis r4, -0x7df1
	ctx.r[4].s64 = -2112946176;
	// 8328ACA4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8328ACA8: 38845250  addi r4, r4, 0x5250
	ctx.r[4].s64 = ctx.r[4].s64 + 21072;
	// 8328ACAC: 387F07E0  addi r3, r31, 0x7e0
	ctx.r[3].s64 = ctx.r[31].s64 + 2016;
	// 8328ACB0: 4AFA2221  bl 0x8222ced0
	ctx.lr = 0x8328ACB4;
	sub_8222CED0(ctx, base);
	// 8328ACB4: 39600003  li r11, 3
	ctx.r[11].s64 = 3;
	// 8328ACB8: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8328ACBC: 917F07E4  stw r11, 0x7e4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(2020 as u32), ctx.r[11].u32 ) };
	// 8328ACC0: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8328ACC4: 387F07E8  addi r3, r31, 0x7e8
	ctx.r[3].s64 = ctx.r[31].s64 + 2024;
	// 8328ACC8: 4AFA2209  bl 0x8222ced0
	ctx.lr = 0x8328ACCC;
	sub_8222CED0(ctx, base);
	// 8328ACCC: 3C60820F  lis r3, -0x7df1
	ctx.r[3].s64 = -2112946176;
	// 8328ACD0: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8328ACD4: 38835240  addi r4, r3, 0x5240
	ctx.r[4].s64 = ctx.r[3].s64 + 21056;
	// 8328ACD8: 387F07EC  addi r3, r31, 0x7ec
	ctx.r[3].s64 = ctx.r[31].s64 + 2028;
	// 8328ACDC: 4AFA21F5  bl 0x8222ced0
	ctx.lr = 0x8328ACE0;
	sub_8222CED0(ctx, base);
	// 8328ACE0: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 8328ACE4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8328ACE8: 388B5228  addi r4, r11, 0x5228
	ctx.r[4].s64 = ctx.r[11].s64 + 21032;
	// 8328ACEC: 387F07F0  addi r3, r31, 0x7f0
	ctx.r[3].s64 = ctx.r[31].s64 + 2032;
	// 8328ACF0: 4AFA21E1  bl 0x8222ced0
	ctx.lr = 0x8328ACF4;
	sub_8222CED0(ctx, base);
	// 8328ACF4: 39600003  li r11, 3
	ctx.r[11].s64 = 3;
	// 8328ACF8: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8328ACFC: 917F07F4  stw r11, 0x7f4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(2036 as u32), ctx.r[11].u32 ) };
	// 8328AD00: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8328AD04: 387F07F8  addi r3, r31, 0x7f8
	ctx.r[3].s64 = ctx.r[31].s64 + 2040;
	// 8328AD08: 4AFA21C9  bl 0x8222ced0
	ctx.lr = 0x8328AD0C;
	sub_8222CED0(ctx, base);
	// 8328AD0C: 3D40820F  lis r10, -0x7df1
	ctx.r[10].s64 = -2112946176;
	// 8328AD10: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8328AD14: 388A5218  addi r4, r10, 0x5218
	ctx.r[4].s64 = ctx.r[10].s64 + 21016;
	// 8328AD18: 387F07FC  addi r3, r31, 0x7fc
	ctx.r[3].s64 = ctx.r[31].s64 + 2044;
	// 8328AD1C: 4AFA21B5  bl 0x8222ced0
	ctx.lr = 0x8328AD20;
	sub_8222CED0(ctx, base);
	// 8328AD20: 3D20820F  lis r9, -0x7df1
	ctx.r[9].s64 = -2112946176;
	// 8328AD24: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8328AD28: 388951F8  addi r4, r9, 0x51f8
	ctx.r[4].s64 = ctx.r[9].s64 + 20984;
	// 8328AD2C: 387F0800  addi r3, r31, 0x800
	ctx.r[3].s64 = ctx.r[31].s64 + 2048;
	// 8328AD30: 4AFA21A1  bl 0x8222ced0
	ctx.lr = 0x8328AD34;
	sub_8222CED0(ctx, base);
	// 8328AD34: 39600003  li r11, 3
	ctx.r[11].s64 = 3;
	// 8328AD38: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8328AD3C: 917F0804  stw r11, 0x804(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(2052 as u32), ctx.r[11].u32 ) };
	// 8328AD40: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8328AD44: 387F0808  addi r3, r31, 0x808
	ctx.r[3].s64 = ctx.r[31].s64 + 2056;
	// 8328AD48: 4AFA2189  bl 0x8222ced0
	ctx.lr = 0x8328AD4C;
	sub_8222CED0(ctx, base);
	// 8328AD4C: 3D00820F  lis r8, -0x7df1
	ctx.r[8].s64 = -2112946176;
	// 8328AD50: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8328AD54: 388851E0  addi r4, r8, 0x51e0
	ctx.r[4].s64 = ctx.r[8].s64 + 20960;
	// 8328AD58: 387F080C  addi r3, r31, 0x80c
	ctx.r[3].s64 = ctx.r[31].s64 + 2060;
	// 8328AD5C: 4AFA2175  bl 0x8222ced0
	ctx.lr = 0x8328AD60;
	sub_8222CED0(ctx, base);
	// 8328AD60: 3CE0820F  lis r7, -0x7df1
	ctx.r[7].s64 = -2112946176;
	// 8328AD64: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8328AD68: 388751C8  addi r4, r7, 0x51c8
	ctx.r[4].s64 = ctx.r[7].s64 + 20936;
	// 8328AD6C: 387F0810  addi r3, r31, 0x810
	ctx.r[3].s64 = ctx.r[31].s64 + 2064;
	// 8328AD70: 4AFA2161  bl 0x8222ced0
	ctx.lr = 0x8328AD74;
	sub_8222CED0(ctx, base);
	// 8328AD74: 39600003  li r11, 3
	ctx.r[11].s64 = 3;
	// 8328AD78: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8328AD7C: 917F0814  stw r11, 0x814(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(2068 as u32), ctx.r[11].u32 ) };
	// 8328AD80: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8328AD84: 387F0818  addi r3, r31, 0x818
	ctx.r[3].s64 = ctx.r[31].s64 + 2072;
	// 8328AD88: 4AFA2149  bl 0x8222ced0
	ctx.lr = 0x8328AD8C;
	sub_8222CED0(ctx, base);
	// 8328AD8C: 3CC0820F  lis r6, -0x7df1
	ctx.r[6].s64 = -2112946176;
	// 8328AD90: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8328AD94: 388651B8  addi r4, r6, 0x51b8
	ctx.r[4].s64 = ctx.r[6].s64 + 20920;
	// 8328AD98: 387F081C  addi r3, r31, 0x81c
	ctx.r[3].s64 = ctx.r[31].s64 + 2076;
	// 8328AD9C: 4AFA2135  bl 0x8222ced0
	ctx.lr = 0x8328ADA0;
	sub_8222CED0(ctx, base);
	// 8328ADA0: 3C80820F  lis r4, -0x7df1
	ctx.r[4].s64 = -2112946176;
	// 8328ADA4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8328ADA8: 388451A0  addi r4, r4, 0x51a0
	ctx.r[4].s64 = ctx.r[4].s64 + 20896;
	// 8328ADAC: 387F0820  addi r3, r31, 0x820
	ctx.r[3].s64 = ctx.r[31].s64 + 2080;
	// 8328ADB0: 4AFA2121  bl 0x8222ced0
	ctx.lr = 0x8328ADB4;
	sub_8222CED0(ctx, base);
	// 8328ADB4: 39600003  li r11, 3
	ctx.r[11].s64 = 3;
	// 8328ADB8: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8328ADBC: 917F0824  stw r11, 0x824(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(2084 as u32), ctx.r[11].u32 ) };
	// 8328ADC0: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8328ADC4: 387F0828  addi r3, r31, 0x828
	ctx.r[3].s64 = ctx.r[31].s64 + 2088;
	// 8328ADC8: 4AFA2109  bl 0x8222ced0
	ctx.lr = 0x8328ADCC;
	sub_8222CED0(ctx, base);
	// 8328ADCC: 3C60820F  lis r3, -0x7df1
	ctx.r[3].s64 = -2112946176;
	// 8328ADD0: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8328ADD4: 38835190  addi r4, r3, 0x5190
	ctx.r[4].s64 = ctx.r[3].s64 + 20880;
	// 8328ADD8: 387F082C  addi r3, r31, 0x82c
	ctx.r[3].s64 = ctx.r[31].s64 + 2092;
	// 8328ADDC: 4AFA20F5  bl 0x8222ced0
	ctx.lr = 0x8328ADE0;
	sub_8222CED0(ctx, base);
	// 8328ADE0: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 8328ADE4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8328ADE8: 388B5178  addi r4, r11, 0x5178
	ctx.r[4].s64 = ctx.r[11].s64 + 20856;
	// 8328ADEC: 387F0830  addi r3, r31, 0x830
	ctx.r[3].s64 = ctx.r[31].s64 + 2096;
	// 8328ADF0: 4AFA20E1  bl 0x8222ced0
	ctx.lr = 0x8328ADF4;
	sub_8222CED0(ctx, base);
	// 8328ADF4: 39600003  li r11, 3
	ctx.r[11].s64 = 3;
	// 8328ADF8: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8328ADFC: 917F0834  stw r11, 0x834(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(2100 as u32), ctx.r[11].u32 ) };
	// 8328AE00: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8328AE04: 387F0838  addi r3, r31, 0x838
	ctx.r[3].s64 = ctx.r[31].s64 + 2104;
	// 8328AE08: 4AFA20C9  bl 0x8222ced0
	ctx.lr = 0x8328AE0C;
	sub_8222CED0(ctx, base);
	// 8328AE0C: 3D40820F  lis r10, -0x7df1
	ctx.r[10].s64 = -2112946176;
	// 8328AE10: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8328AE14: 388A5168  addi r4, r10, 0x5168
	ctx.r[4].s64 = ctx.r[10].s64 + 20840;
	// 8328AE18: 387F083C  addi r3, r31, 0x83c
	ctx.r[3].s64 = ctx.r[31].s64 + 2108;
	// 8328AE1C: 4AFA20B5  bl 0x8222ced0
	ctx.lr = 0x8328AE20;
	sub_8222CED0(ctx, base);
	// 8328AE20: 3D20820F  lis r9, -0x7df1
	ctx.r[9].s64 = -2112946176;
	// 8328AE24: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8328AE28: 38895148  addi r4, r9, 0x5148
	ctx.r[4].s64 = ctx.r[9].s64 + 20808;
	// 8328AE2C: 387F0840  addi r3, r31, 0x840
	ctx.r[3].s64 = ctx.r[31].s64 + 2112;
	// 8328AE30: 4AFA20A1  bl 0x8222ced0
	ctx.lr = 0x8328AE34;
	sub_8222CED0(ctx, base);
	// 8328AE34: 39600003  li r11, 3
	ctx.r[11].s64 = 3;
	// 8328AE38: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8328AE3C: 917F0844  stw r11, 0x844(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(2116 as u32), ctx.r[11].u32 ) };
	// 8328AE40: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8328AE44: 387F0848  addi r3, r31, 0x848
	ctx.r[3].s64 = ctx.r[31].s64 + 2120;
	// 8328AE48: 4AFA2089  bl 0x8222ced0
	ctx.lr = 0x8328AE4C;
	sub_8222CED0(ctx, base);
	// 8328AE4C: 3D00820F  lis r8, -0x7df1
	ctx.r[8].s64 = -2112946176;
	// 8328AE50: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8328AE54: 38885130  addi r4, r8, 0x5130
	ctx.r[4].s64 = ctx.r[8].s64 + 20784;
	// 8328AE58: 387F084C  addi r3, r31, 0x84c
	ctx.r[3].s64 = ctx.r[31].s64 + 2124;
	// 8328AE5C: 4AFA2075  bl 0x8222ced0
	ctx.lr = 0x8328AE60;
	sub_8222CED0(ctx, base);
	// 8328AE60: 3CE0820F  lis r7, -0x7df1
	ctx.r[7].s64 = -2112946176;
	// 8328AE64: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8328AE68: 38875114  addi r4, r7, 0x5114
	ctx.r[4].s64 = ctx.r[7].s64 + 20756;
	// 8328AE6C: 387F0850  addi r3, r31, 0x850
	ctx.r[3].s64 = ctx.r[31].s64 + 2128;
	// 8328AE70: 4AFA2061  bl 0x8222ced0
	ctx.lr = 0x8328AE74;
	sub_8222CED0(ctx, base);
	// 8328AE74: 39600003  li r11, 3
	ctx.r[11].s64 = 3;
	// 8328AE78: 3CC0820F  lis r6, -0x7df1
	ctx.r[6].s64 = -2112946176;
	// 8328AE7C: 917F0854  stw r11, 0x854(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(2132 as u32), ctx.r[11].u32 ) };
	// 8328AE80: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8328AE84: 3BC64864  addi r30, r6, 0x4864
	ctx.r[30].s64 = ctx.r[6].s64 + 18532;
	// 8328AE88: 387F0858  addi r3, r31, 0x858
	ctx.r[3].s64 = ctx.r[31].s64 + 2136;
	// 8328AE8C: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8328AE90: 4AFA2041  bl 0x8222ced0
	ctx.lr = 0x8328AE94;
	sub_8222CED0(ctx, base);
	// 8328AE94: 3C80820F  lis r4, -0x7df1
	ctx.r[4].s64 = -2112946176;
	// 8328AE98: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8328AE9C: 3BA45104  addi r29, r4, 0x5104
	ctx.r[29].s64 = ctx.r[4].s64 + 20740;
	// 8328AEA0: 387F085C  addi r3, r31, 0x85c
	ctx.r[3].s64 = ctx.r[31].s64 + 2140;
	// 8328AEA4: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 8328AEA8: 4AFA2029  bl 0x8222ced0
	ctx.lr = 0x8328AEAC;
	sub_8222CED0(ctx, base);
	// 8328AEAC: 3C60820F  lis r3, -0x7df1
	ctx.r[3].s64 = -2112946176;
	// 8328AEB0: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8328AEB4: 388350EC  addi r4, r3, 0x50ec
	ctx.r[4].s64 = ctx.r[3].s64 + 20716;
	// 8328AEB8: 387F0860  addi r3, r31, 0x860
	ctx.r[3].s64 = ctx.r[31].s64 + 2144;
	// 8328AEBC: 4AFA2015  bl 0x8222ced0
	ctx.lr = 0x8328AEC0;
	sub_8222CED0(ctx, base);
	// 8328AEC0: 39600003  li r11, 3
	ctx.r[11].s64 = 3;
	// 8328AEC4: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8328AEC8: 917F0864  stw r11, 0x864(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(2148 as u32), ctx.r[11].u32 ) };
	// 8328AECC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8328AED0: 387F0868  addi r3, r31, 0x868
	ctx.r[3].s64 = ctx.r[31].s64 + 2152;
	// 8328AED4: 4AFA1FFD  bl 0x8222ced0
	ctx.lr = 0x8328AED8;
	sub_8222CED0(ctx, base);
	// 8328AED8: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8328AEDC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8328AEE0: 3B6B86D8  addi r27, r11, -0x7928
	ctx.r[27].s64 = ctx.r[11].s64 + -31016;
	// 8328AEE4: 387F086C  addi r3, r31, 0x86c
	ctx.r[3].s64 = ctx.r[31].s64 + 2156;
	// 8328AEE8: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 8328AEEC: 4AFA1FE5  bl 0x8222ced0
	ctx.lr = 0x8328AEF0;
	sub_8222CED0(ctx, base);
	// 8328AEF0: 3D40820F  lis r10, -0x7df1
	ctx.r[10].s64 = -2112946176;
	// 8328AEF4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8328AEF8: 388A50CC  addi r4, r10, 0x50cc
	ctx.r[4].s64 = ctx.r[10].s64 + 20684;
	// 8328AEFC: 387F0870  addi r3, r31, 0x870
	ctx.r[3].s64 = ctx.r[31].s64 + 2160;
	// 8328AF00: 4AFA1FD1  bl 0x8222ced0
	ctx.lr = 0x8328AF04;
	sub_8222CED0(ctx, base);
	// 8328AF04: 39600003  li r11, 3
	ctx.r[11].s64 = 3;
	// 8328AF08: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8328AF0C: 917F0874  stw r11, 0x874(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(2164 as u32), ctx.r[11].u32 ) };
	// 8328AF10: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8328AF14: 387F0878  addi r3, r31, 0x878
	ctx.r[3].s64 = ctx.r[31].s64 + 2168;
	// 8328AF18: 4AFA1FB9  bl 0x8222ced0
	ctx.lr = 0x8328AF1C;
	sub_8222CED0(ctx, base);
	// 8328AF1C: 3D20820F  lis r9, -0x7df1
	ctx.r[9].s64 = -2112946176;
	// 8328AF20: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8328AF24: 3B4950BC  addi r26, r9, 0x50bc
	ctx.r[26].s64 = ctx.r[9].s64 + 20668;
	// 8328AF28: 387F087C  addi r3, r31, 0x87c
	ctx.r[3].s64 = ctx.r[31].s64 + 2172;
	// 8328AF2C: 7F44D378  mr r4, r26
	ctx.r[4].u64 = ctx.r[26].u64;
	// 8328AF30: 4AFA1FA1  bl 0x8222ced0
	ctx.lr = 0x8328AF34;
	sub_8222CED0(ctx, base);
	// 8328AF34: 3D00820F  lis r8, -0x7df1
	ctx.r[8].s64 = -2112946176;
	// 8328AF38: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8328AF3C: 38885098  addi r4, r8, 0x5098
	ctx.r[4].s64 = ctx.r[8].s64 + 20632;
	// 8328AF40: 387F0880  addi r3, r31, 0x880
	ctx.r[3].s64 = ctx.r[31].s64 + 2176;
	// 8328AF44: 4AFA1F8D  bl 0x8222ced0
	ctx.lr = 0x8328AF48;
	sub_8222CED0(ctx, base);
	// 8328AF48: 39600003  li r11, 3
	ctx.r[11].s64 = 3;
	// 8328AF4C: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8328AF50: 917F0884  stw r11, 0x884(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(2180 as u32), ctx.r[11].u32 ) };
	// 8328AF54: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8328AF58: 387F0888  addi r3, r31, 0x888
	ctx.r[3].s64 = ctx.r[31].s64 + 2184;
	// 8328AF5C: 4AFA1F75  bl 0x8222ced0
	ctx.lr = 0x8328AF60;
	sub_8222CED0(ctx, base);
	// 8328AF60: 3CE0820F  lis r7, -0x7df1
	ctx.r[7].s64 = -2112946176;
	// 8328AF64: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8328AF68: 3B275080  addi r25, r7, 0x5080
	ctx.r[25].s64 = ctx.r[7].s64 + 20608;
	// 8328AF6C: 387F088C  addi r3, r31, 0x88c
	ctx.r[3].s64 = ctx.r[31].s64 + 2188;
	// 8328AF70: 7F24CB78  mr r4, r25
	ctx.r[4].u64 = ctx.r[25].u64;
	// 8328AF74: 4AFA1F5D  bl 0x8222ced0
	ctx.lr = 0x8328AF78;
	sub_8222CED0(ctx, base);
	// 8328AF78: 3CC0820F  lis r6, -0x7df1
	ctx.r[6].s64 = -2112946176;
	// 8328AF7C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8328AF80: 3B065064  addi r24, r6, 0x5064
	ctx.r[24].s64 = ctx.r[6].s64 + 20580;
	// 8328AF84: 387F0890  addi r3, r31, 0x890
	ctx.r[3].s64 = ctx.r[31].s64 + 2192;
	// 8328AF88: 7F04C378  mr r4, r24
	ctx.r[4].u64 = ctx.r[24].u64;
	// 8328AF8C: 4AFA1F45  bl 0x8222ced0
	ctx.lr = 0x8328AF90;
	sub_8222CED0(ctx, base);
	// 8328AF90: 39600003  li r11, 3
	ctx.r[11].s64 = 3;
	// 8328AF94: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8328AF98: 917F0894  stw r11, 0x894(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(2196 as u32), ctx.r[11].u32 ) };
	// 8328AF9C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8328AFA0: 387F0898  addi r3, r31, 0x898
	ctx.r[3].s64 = ctx.r[31].s64 + 2200;
	// 8328AFA4: 4AFA1F2D  bl 0x8222ced0
	ctx.lr = 0x8328AFA8;
	sub_8222CED0(ctx, base);
	// 8328AFA8: 3C80820F  lis r4, -0x7df1
	ctx.r[4].s64 = -2112946176;
	// 8328AFAC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8328AFB0: 3AE45054  addi r23, r4, 0x5054
	ctx.r[23].s64 = ctx.r[4].s64 + 20564;
	// 8328AFB4: 387F089C  addi r3, r31, 0x89c
	ctx.r[3].s64 = ctx.r[31].s64 + 2204;
	// 8328AFB8: 7EE4BB78  mr r4, r23
	ctx.r[4].u64 = ctx.r[23].u64;
	// 8328AFBC: 4AFA1F15  bl 0x8222ced0
	ctx.lr = 0x8328AFC0;
	sub_8222CED0(ctx, base);
	// 8328AFC0: 3C60820F  lis r3, -0x7df1
	ctx.r[3].s64 = -2112946176;
	// 8328AFC4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8328AFC8: 3AC35038  addi r22, r3, 0x5038
	ctx.r[22].s64 = ctx.r[3].s64 + 20536;
	// 8328AFCC: 387F08A0  addi r3, r31, 0x8a0
	ctx.r[3].s64 = ctx.r[31].s64 + 2208;
	// 8328AFD0: 7EC4B378  mr r4, r22
	ctx.r[4].u64 = ctx.r[22].u64;
	// 8328AFD4: 4AFA1EFD  bl 0x8222ced0
	ctx.lr = 0x8328AFD8;
	sub_8222CED0(ctx, base);
	// 8328AFD8: 39600003  li r11, 3
	ctx.r[11].s64 = 3;
	// 8328AFDC: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8328AFE0: 917F08A4  stw r11, 0x8a4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(2212 as u32), ctx.r[11].u32 ) };
	// 8328AFE4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8328AFE8: 387F08A8  addi r3, r31, 0x8a8
	ctx.r[3].s64 = ctx.r[31].s64 + 2216;
	// 8328AFEC: 4AFA1EE5  bl 0x8222ced0
	ctx.lr = 0x8328AFF0;
	sub_8222CED0(ctx, base);
	// 8328AFF0: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 8328AFF4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8328AFF8: 387F08AC  addi r3, r31, 0x8ac
	ctx.r[3].s64 = ctx.r[31].s64 + 2220;
	// 8328AFFC: 4AFA1ED5  bl 0x8222ced0
	ctx.lr = 0x8328B000;
	sub_8222CED0(ctx, base);
	// 8328B000: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 8328B004: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8328B008: 388B5020  addi r4, r11, 0x5020
	ctx.r[4].s64 = ctx.r[11].s64 + 20512;
	// 8328B00C: 387F08B0  addi r3, r31, 0x8b0
	ctx.r[3].s64 = ctx.r[31].s64 + 2224;
	// 8328B010: 4AFA1EC1  bl 0x8222ced0
	ctx.lr = 0x8328B014;
	sub_8222CED0(ctx, base);
	// 8328B014: 39600003  li r11, 3
	ctx.r[11].s64 = 3;
	// 8328B018: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8328B01C: 917F08B4  stw r11, 0x8b4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(2228 as u32), ctx.r[11].u32 ) };
	// 8328B020: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8328B024: 387F08B8  addi r3, r31, 0x8b8
	ctx.r[3].s64 = ctx.r[31].s64 + 2232;
	// 8328B028: 4AFA1EA9  bl 0x8222ced0
	ctx.lr = 0x8328B02C;
	sub_8222CED0(ctx, base);
	// 8328B02C: 3D40820F  lis r10, -0x7df1
	ctx.r[10].s64 = -2112946176;
	// 8328B030: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8328B034: 3AAA5014  addi r21, r10, 0x5014
	ctx.r[21].s64 = ctx.r[10].s64 + 20500;
	// 8328B038: 387F08BC  addi r3, r31, 0x8bc
	ctx.r[3].s64 = ctx.r[31].s64 + 2236;
	// 8328B03C: 7EA4AB78  mr r4, r21
	ctx.r[4].u64 = ctx.r[21].u64;
	// 8328B040: 4AFA1E91  bl 0x8222ced0
	ctx.lr = 0x8328B044;
	sub_8222CED0(ctx, base);
	// 8328B044: 3D20820F  lis r9, -0x7df1
	ctx.r[9].s64 = -2112946176;
	// 8328B048: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8328B04C: 38894FFC  addi r4, r9, 0x4ffc
	ctx.r[4].s64 = ctx.r[9].s64 + 20476;
	// 8328B050: 387F08C0  addi r3, r31, 0x8c0
	ctx.r[3].s64 = ctx.r[31].s64 + 2240;
	// 8328B054: 4AFA1E7D  bl 0x8222ced0
	ctx.lr = 0x8328B058;
	sub_8222CED0(ctx, base);
	// 8328B058: 39600003  li r11, 3
	ctx.r[11].s64 = 3;
	// 8328B05C: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8328B060: 917F08C4  stw r11, 0x8c4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(2244 as u32), ctx.r[11].u32 ) };
	// 8328B064: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8328B068: 387F08C8  addi r3, r31, 0x8c8
	ctx.r[3].s64 = ctx.r[31].s64 + 2248;
	// 8328B06C: 4AFA1E65  bl 0x8222ced0
	ctx.lr = 0x8328B070;
	sub_8222CED0(ctx, base);
	// 8328B070: 3D00820F  lis r8, -0x7df1
	ctx.r[8].s64 = -2112946176;
	// 8328B074: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8328B078: 3A884FF0  addi r20, r8, 0x4ff0
	ctx.r[20].s64 = ctx.r[8].s64 + 20464;
	// 8328B07C: 387F08CC  addi r3, r31, 0x8cc
	ctx.r[3].s64 = ctx.r[31].s64 + 2252;
	// 8328B080: 7E84A378  mr r4, r20
	ctx.r[4].u64 = ctx.r[20].u64;
	// 8328B084: 4AFA1E4D  bl 0x8222ced0
	ctx.lr = 0x8328B088;
	sub_8222CED0(ctx, base);
	// 8328B088: 3CE0820F  lis r7, -0x7df1
	ctx.r[7].s64 = -2112946176;
	// 8328B08C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8328B090: 38874FDC  addi r4, r7, 0x4fdc
	ctx.r[4].s64 = ctx.r[7].s64 + 20444;
	// 8328B094: 387F08D0  addi r3, r31, 0x8d0
	ctx.r[3].s64 = ctx.r[31].s64 + 2256;
	// 8328B098: 4AFA1E39  bl 0x8222ced0
	ctx.lr = 0x8328B09C;
	sub_8222CED0(ctx, base);
	// 8328B09C: 39600003  li r11, 3
	ctx.r[11].s64 = 3;
	// 8328B0A0: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8328B0A4: 917F08D4  stw r11, 0x8d4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(2260 as u32), ctx.r[11].u32 ) };
	// 8328B0A8: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8328B0AC: 387F08D8  addi r3, r31, 0x8d8
	ctx.r[3].s64 = ctx.r[31].s64 + 2264;
	// 8328B0B0: 4AFA1E21  bl 0x8222ced0
	ctx.lr = 0x8328B0B4;
	sub_8222CED0(ctx, base);
	// 8328B0B4: 3CC0820F  lis r6, -0x7df1
	ctx.r[6].s64 = -2112946176;
	// 8328B0B8: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8328B0BC: 3A664FD4  addi r19, r6, 0x4fd4
	ctx.r[19].s64 = ctx.r[6].s64 + 20436;
	// 8328B0C0: 387F08DC  addi r3, r31, 0x8dc
	ctx.r[3].s64 = ctx.r[31].s64 + 2268;
	// 8328B0C4: 7E649B78  mr r4, r19
	ctx.r[4].u64 = ctx.r[19].u64;
	// 8328B0C8: 4AFA1E09  bl 0x8222ced0
	ctx.lr = 0x8328B0CC;
	sub_8222CED0(ctx, base);
	// 8328B0CC: 3C80820F  lis r4, -0x7df1
	ctx.r[4].s64 = -2112946176;
	// 8328B0D0: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8328B0D4: 38844FC0  addi r4, r4, 0x4fc0
	ctx.r[4].s64 = ctx.r[4].s64 + 20416;
	// 8328B0D8: 387F08E0  addi r3, r31, 0x8e0
	ctx.r[3].s64 = ctx.r[31].s64 + 2272;
	// 8328B0DC: 4AFA1DF5  bl 0x8222ced0
	ctx.lr = 0x8328B0E0;
	sub_8222CED0(ctx, base);
	// 8328B0E0: 39600003  li r11, 3
	ctx.r[11].s64 = 3;
	// 8328B0E4: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8328B0E8: 917F08E4  stw r11, 0x8e4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(2276 as u32), ctx.r[11].u32 ) };
	// 8328B0EC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8328B0F0: 387F08E8  addi r3, r31, 0x8e8
	ctx.r[3].s64 = ctx.r[31].s64 + 2280;
	// 8328B0F4: 4AFA1DDD  bl 0x8222ced0
	ctx.lr = 0x8328B0F8;
	sub_8222CED0(ctx, base);
	// 8328B0F8: 3C60820F  lis r3, -0x7df1
	ctx.r[3].s64 = -2112946176;
	// 8328B0FC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8328B100: 3A434FB8  addi r18, r3, 0x4fb8
	ctx.r[18].s64 = ctx.r[3].s64 + 20408;
	// 8328B104: 387F08EC  addi r3, r31, 0x8ec
	ctx.r[3].s64 = ctx.r[31].s64 + 2284;
	// 8328B108: 7E449378  mr r4, r18
	ctx.r[4].u64 = ctx.r[18].u64;
	// 8328B10C: 4AFA1DC5  bl 0x8222ced0
	ctx.lr = 0x8328B110;
	sub_8222CED0(ctx, base);
	// 8328B110: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 8328B114: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8328B118: 388B4F98  addi r4, r11, 0x4f98
	ctx.r[4].s64 = ctx.r[11].s64 + 20376;
	// 8328B11C: 387F08F0  addi r3, r31, 0x8f0
	ctx.r[3].s64 = ctx.r[31].s64 + 2288;
	// 8328B120: 4AFA1DB1  bl 0x8222ced0
	ctx.lr = 0x8328B124;
	sub_8222CED0(ctx, base);
	// 8328B124: 39600003  li r11, 3
	ctx.r[11].s64 = 3;
	// 8328B128: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8328B12C: 917F08F4  stw r11, 0x8f4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(2292 as u32), ctx.r[11].u32 ) };
	// 8328B130: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8328B134: 387F08F8  addi r3, r31, 0x8f8
	ctx.r[3].s64 = ctx.r[31].s64 + 2296;
	// 8328B138: 4AFA1D99  bl 0x8222ced0
	ctx.lr = 0x8328B13C;
	sub_8222CED0(ctx, base);
	// 8328B13C: 3D40820F  lis r10, -0x7df1
	ctx.r[10].s64 = -2112946176;
	// 8328B140: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8328B144: 3A2A4F88  addi r17, r10, 0x4f88
	ctx.r[17].s64 = ctx.r[10].s64 + 20360;
	// 8328B148: 387F08FC  addi r3, r31, 0x8fc
	ctx.r[3].s64 = ctx.r[31].s64 + 2300;
	// 8328B14C: 7E248B78  mr r4, r17
	ctx.r[4].u64 = ctx.r[17].u64;
	// 8328B150: 4AFA1D81  bl 0x8222ced0
	ctx.lr = 0x8328B154;
	sub_8222CED0(ctx, base);
	// 8328B154: 3D20820F  lis r9, -0x7df1
	ctx.r[9].s64 = -2112946176;
	// 8328B158: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8328B15C: 38894F68  addi r4, r9, 0x4f68
	ctx.r[4].s64 = ctx.r[9].s64 + 20328;
	// 8328B160: 387F0900  addi r3, r31, 0x900
	ctx.r[3].s64 = ctx.r[31].s64 + 2304;
	// 8328B164: 4AFA1D6D  bl 0x8222ced0
	ctx.lr = 0x8328B168;
	sub_8222CED0(ctx, base);
	// 8328B168: 39600003  li r11, 3
	ctx.r[11].s64 = 3;
	// 8328B16C: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8328B170: 917F0904  stw r11, 0x904(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(2308 as u32), ctx.r[11].u32 ) };
	// 8328B174: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8328B178: 387F0908  addi r3, r31, 0x908
	ctx.r[3].s64 = ctx.r[31].s64 + 2312;
	// 8328B17C: 4AFA1D55  bl 0x8222ced0
	ctx.lr = 0x8328B180;
	sub_8222CED0(ctx, base);
	// 8328B180: 3D00820F  lis r8, -0x7df1
	ctx.r[8].s64 = -2112946176;
	// 8328B184: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8328B188: 3A084F58  addi r16, r8, 0x4f58
	ctx.r[16].s64 = ctx.r[8].s64 + 20312;
	// 8328B18C: 387F090C  addi r3, r31, 0x90c
	ctx.r[3].s64 = ctx.r[31].s64 + 2316;
	// 8328B190: 7E048378  mr r4, r16
	ctx.r[4].u64 = ctx.r[16].u64;
	// 8328B194: 4AFA1D3D  bl 0x8222ced0
	ctx.lr = 0x8328B198;
	sub_8222CED0(ctx, base);
	// 8328B198: 3CE0820F  lis r7, -0x7df1
	ctx.r[7].s64 = -2112946176;
	// 8328B19C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8328B1A0: 38874F40  addi r4, r7, 0x4f40
	ctx.r[4].s64 = ctx.r[7].s64 + 20288;
	// 8328B1A4: 387F0910  addi r3, r31, 0x910
	ctx.r[3].s64 = ctx.r[31].s64 + 2320;
	// 8328B1A8: 4AFA1D29  bl 0x8222ced0
	ctx.lr = 0x8328B1AC;
	sub_8222CED0(ctx, base);
	// 8328B1AC: 39600003  li r11, 3
	ctx.r[11].s64 = 3;
	// 8328B1B0: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8328B1B4: 917F0914  stw r11, 0x914(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(2324 as u32), ctx.r[11].u32 ) };
	// 8328B1B8: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8328B1BC: 387F0918  addi r3, r31, 0x918
	ctx.r[3].s64 = ctx.r[31].s64 + 2328;
	// 8328B1C0: 4AFA1D11  bl 0x8222ced0
	ctx.lr = 0x8328B1C4;
	sub_8222CED0(ctx, base);
	// 8328B1C4: 3CC0820F  lis r6, -0x7df1
	ctx.r[6].s64 = -2112946176;
	// 8328B1C8: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8328B1CC: 39E64F34  addi r15, r6, 0x4f34
	ctx.r[15].s64 = ctx.r[6].s64 + 20276;
	// 8328B1D0: 387F091C  addi r3, r31, 0x91c
	ctx.r[3].s64 = ctx.r[31].s64 + 2332;
	// 8328B1D4: 7DE47B78  mr r4, r15
	ctx.r[4].u64 = ctx.r[15].u64;
	// 8328B1D8: 4AFA1CF9  bl 0x8222ced0
	ctx.lr = 0x8328B1DC;
	sub_8222CED0(ctx, base);
	// 8328B1DC: 3C80820F  lis r4, -0x7df1
	ctx.r[4].s64 = -2112946176;
	// 8328B1E0: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8328B1E4: 38844F1C  addi r4, r4, 0x4f1c
	ctx.r[4].s64 = ctx.r[4].s64 + 20252;
	// 8328B1E8: 387F0920  addi r3, r31, 0x920
	ctx.r[3].s64 = ctx.r[31].s64 + 2336;
	// 8328B1EC: 4AFA1CE5  bl 0x8222ced0
	ctx.lr = 0x8328B1F0;
	sub_8222CED0(ctx, base);
	// 8328B1F0: 39600003  li r11, 3
	ctx.r[11].s64 = 3;
	// 8328B1F4: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8328B1F8: 917F0924  stw r11, 0x924(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(2340 as u32), ctx.r[11].u32 ) };
	// 8328B1FC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8328B200: 387F0928  addi r3, r31, 0x928
	ctx.r[3].s64 = ctx.r[31].s64 + 2344;
	// 8328B204: 4AFA1CCD  bl 0x8222ced0
	ctx.lr = 0x8328B208;
	sub_8222CED0(ctx, base);
	// 8328B208: 3C60820F  lis r3, -0x7df1
	ctx.r[3].s64 = -2112946176;
	// 8328B20C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8328B210: 3BC34F10  addi r30, r3, 0x4f10
	ctx.r[30].s64 = ctx.r[3].s64 + 20240;
	// 8328B214: 387F092C  addi r3, r31, 0x92c
	ctx.r[3].s64 = ctx.r[31].s64 + 2348;
	// 8328B218: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8328B21C: 4AFA1CB5  bl 0x8222ced0
	ctx.lr = 0x8328B220;
	sub_8222CED0(ctx, base);
	// 8328B220: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 8328B224: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8328B228: 388B4EF4  addi r4, r11, 0x4ef4
	ctx.r[4].s64 = ctx.r[11].s64 + 20212;
	// 8328B22C: 387F0930  addi r3, r31, 0x930
	ctx.r[3].s64 = ctx.r[31].s64 + 2352;
	// 8328B230: 4AFA1CA1  bl 0x8222ced0
	ctx.lr = 0x8328B234;
	sub_8222CED0(ctx, base);
	// 8328B234: 39600003  li r11, 3
	ctx.r[11].s64 = 3;
	// 8328B238: 3D40820F  lis r10, -0x7df1
	ctx.r[10].s64 = -2112946176;
	// 8328B23C: 917F0934  stw r11, 0x934(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(2356 as u32), ctx.r[11].u32 ) };
	// 8328B240: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8328B244: 39CA482C  addi r14, r10, 0x482c
	ctx.r[14].s64 = ctx.r[10].s64 + 18476;
	// 8328B248: 387F0938  addi r3, r31, 0x938
	ctx.r[3].s64 = ctx.r[31].s64 + 2360;
	// 8328B24C: 7DC47378  mr r4, r14
	ctx.r[4].u64 = ctx.r[14].u64;
	// 8328B250: 4AFA1C81  bl 0x8222ced0
	ctx.lr = 0x8328B254;
	sub_8222CED0(ctx, base);
	// 8328B254: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 8328B258: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8328B25C: 387F093C  addi r3, r31, 0x93c
	ctx.r[3].s64 = ctx.r[31].s64 + 2364;
	// 8328B260: 4AFA1C71  bl 0x8222ced0
	ctx.lr = 0x8328B264;
	sub_8222CED0(ctx, base);
	// 8328B264: 3D20820F  lis r9, -0x7df1
	ctx.r[9].s64 = -2112946176;
	// 8328B268: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8328B26C: 38894EDC  addi r4, r9, 0x4edc
	ctx.r[4].s64 = ctx.r[9].s64 + 20188;
	// 8328B270: 387F0940  addi r3, r31, 0x940
	ctx.r[3].s64 = ctx.r[31].s64 + 2368;
	// 8328B274: 4AFA1C5D  bl 0x8222ced0
	ctx.lr = 0x8328B278;
	sub_8222CED0(ctx, base);
	// 8328B278: 39600003  li r11, 3
	ctx.r[11].s64 = 3;
	// 8328B27C: 7DC47378  mr r4, r14
	ctx.r[4].u64 = ctx.r[14].u64;
	// 8328B280: 917F0944  stw r11, 0x944(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(2372 as u32), ctx.r[11].u32 ) };
	// 8328B284: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8328B288: 387F0948  addi r3, r31, 0x948
	ctx.r[3].s64 = ctx.r[31].s64 + 2376;
	// 8328B28C: 4AFA1C45  bl 0x8222ced0
	ctx.lr = 0x8328B290;
	sub_8222CED0(ctx, base);
	// 8328B290: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 8328B294: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8328B298: 387F094C  addi r3, r31, 0x94c
	ctx.r[3].s64 = ctx.r[31].s64 + 2380;
	// 8328B29C: 4AFA1C35  bl 0x8222ced0
	ctx.lr = 0x8328B2A0;
	sub_8222CED0(ctx, base);
	// 8328B2A0: 3D00820F  lis r8, -0x7df1
	ctx.r[8].s64 = -2112946176;
	// 8328B2A4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8328B2A8: 38884EBC  addi r4, r8, 0x4ebc
	ctx.r[4].s64 = ctx.r[8].s64 + 20156;
	// 8328B2AC: 387F0950  addi r3, r31, 0x950
	ctx.r[3].s64 = ctx.r[31].s64 + 2384;
	// 8328B2B0: 4AFA1C21  bl 0x8222ced0
	ctx.lr = 0x8328B2B4;
	sub_8222CED0(ctx, base);
	// 8328B2B4: 39600003  li r11, 3
	ctx.r[11].s64 = 3;
	// 8328B2B8: 7DC47378  mr r4, r14
	ctx.r[4].u64 = ctx.r[14].u64;
	// 8328B2BC: 917F0954  stw r11, 0x954(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(2388 as u32), ctx.r[11].u32 ) };
	// 8328B2C0: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8328B2C4: 387F0958  addi r3, r31, 0x958
	ctx.r[3].s64 = ctx.r[31].s64 + 2392;
	// 8328B2C8: 4AFA1C09  bl 0x8222ced0
	ctx.lr = 0x8328B2CC;
	sub_8222CED0(ctx, base);
	// 8328B2CC: 7F44D378  mr r4, r26
	ctx.r[4].u64 = ctx.r[26].u64;
	// 8328B2D0: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8328B2D4: 387F095C  addi r3, r31, 0x95c
	ctx.r[3].s64 = ctx.r[31].s64 + 2396;
	// 8328B2D8: 4AFA1BF9  bl 0x8222ced0
	ctx.lr = 0x8328B2DC;
	sub_8222CED0(ctx, base);
	// 8328B2DC: 3CE0820F  lis r7, -0x7df1
	ctx.r[7].s64 = -2112946176;
	// 8328B2E0: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8328B2E4: 38874E98  addi r4, r7, 0x4e98
	ctx.r[4].s64 = ctx.r[7].s64 + 20120;
	// 8328B2E8: 387F0960  addi r3, r31, 0x960
	ctx.r[3].s64 = ctx.r[31].s64 + 2400;
	// 8328B2EC: 4AFA1BE5  bl 0x8222ced0
	ctx.lr = 0x8328B2F0;
	sub_8222CED0(ctx, base);
	// 8328B2F0: 39600003  li r11, 3
	ctx.r[11].s64 = 3;
	// 8328B2F4: 7DC47378  mr r4, r14
	ctx.r[4].u64 = ctx.r[14].u64;
	// 8328B2F8: 917F0964  stw r11, 0x964(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(2404 as u32), ctx.r[11].u32 ) };
	// 8328B2FC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8328B300: 387F0968  addi r3, r31, 0x968
	ctx.r[3].s64 = ctx.r[31].s64 + 2408;
	// 8328B304: 4AFA1BCD  bl 0x8222ced0
	ctx.lr = 0x8328B308;
	sub_8222CED0(ctx, base);
	// 8328B308: 7F24CB78  mr r4, r25
	ctx.r[4].u64 = ctx.r[25].u64;
	// 8328B30C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8328B310: 387F096C  addi r3, r31, 0x96c
	ctx.r[3].s64 = ctx.r[31].s64 + 2412;
	// 8328B314: 4AFA1BBD  bl 0x8222ced0
	ctx.lr = 0x8328B318;
	sub_8222CED0(ctx, base);
	// 8328B318: 7F04C378  mr r4, r24
	ctx.r[4].u64 = ctx.r[24].u64;
	// 8328B31C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8328B320: 387F0970  addi r3, r31, 0x970
	ctx.r[3].s64 = ctx.r[31].s64 + 2416;
	// 8328B324: 4AFA1BAD  bl 0x8222ced0
	ctx.lr = 0x8328B328;
	sub_8222CED0(ctx, base);
	// 8328B328: 39600003  li r11, 3
	ctx.r[11].s64 = 3;
	// 8328B32C: 7DC47378  mr r4, r14
	ctx.r[4].u64 = ctx.r[14].u64;
	// 8328B330: 917F0974  stw r11, 0x974(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(2420 as u32), ctx.r[11].u32 ) };
	// 8328B334: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8328B338: 387F0978  addi r3, r31, 0x978
	ctx.r[3].s64 = ctx.r[31].s64 + 2424;
	// 8328B33C: 4AFA1B95  bl 0x8222ced0
	ctx.lr = 0x8328B340;
	sub_8222CED0(ctx, base);
	// 8328B340: 7EE4BB78  mr r4, r23
	ctx.r[4].u64 = ctx.r[23].u64;
	// 8328B344: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8328B348: 387F097C  addi r3, r31, 0x97c
	ctx.r[3].s64 = ctx.r[31].s64 + 2428;
	// 8328B34C: 4AFA1B85  bl 0x8222ced0
	ctx.lr = 0x8328B350;
	sub_8222CED0(ctx, base);
	// 8328B350: 7EC4B378  mr r4, r22
	ctx.r[4].u64 = ctx.r[22].u64;
	// 8328B354: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8328B358: 387F0980  addi r3, r31, 0x980
	ctx.r[3].s64 = ctx.r[31].s64 + 2432;
	// 8328B35C: 4AFA1B75  bl 0x8222ced0
	ctx.lr = 0x8328B360;
	sub_8222CED0(ctx, base);
	// 8328B360: 39600003  li r11, 3
	ctx.r[11].s64 = 3;
	// 8328B364: 7DC47378  mr r4, r14
	ctx.r[4].u64 = ctx.r[14].u64;
	// 8328B368: 917F0984  stw r11, 0x984(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(2436 as u32), ctx.r[11].u32 ) };
	// 8328B36C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8328B370: 387F0988  addi r3, r31, 0x988
	ctx.r[3].s64 = ctx.r[31].s64 + 2440;
	// 8328B374: 4AFA1B5D  bl 0x8222ced0
	ctx.lr = 0x8328B378;
	sub_8222CED0(ctx, base);
	// 8328B378: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 8328B37C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8328B380: 387F098C  addi r3, r31, 0x98c
	ctx.r[3].s64 = ctx.r[31].s64 + 2444;
	// 8328B384: 4AFA1B4D  bl 0x8222ced0
	ctx.lr = 0x8328B388;
	sub_8222CED0(ctx, base);
	// 8328B388: 3CC0820F  lis r6, -0x7df1
	ctx.r[6].s64 = -2112946176;
	// 8328B38C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8328B390: 38864E80  addi r4, r6, 0x4e80
	ctx.r[4].s64 = ctx.r[6].s64 + 20096;
	// 8328B394: 387F0990  addi r3, r31, 0x990
	ctx.r[3].s64 = ctx.r[31].s64 + 2448;
	// 8328B398: 4AFA1B39  bl 0x8222ced0
	ctx.lr = 0x8328B39C;
	sub_8222CED0(ctx, base);
	// 8328B39C: 39600003  li r11, 3
	ctx.r[11].s64 = 3;
	// 8328B3A0: 7DC47378  mr r4, r14
	ctx.r[4].u64 = ctx.r[14].u64;
	// 8328B3A4: 917F0994  stw r11, 0x994(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(2452 as u32), ctx.r[11].u32 ) };
	// 8328B3A8: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8328B3AC: 387F0998  addi r3, r31, 0x998
	ctx.r[3].s64 = ctx.r[31].s64 + 2456;
	// 8328B3B0: 4AFA1B21  bl 0x8222ced0
	ctx.lr = 0x8328B3B4;
	sub_8222CED0(ctx, base);
	// 8328B3B4: 7EA4AB78  mr r4, r21
	ctx.r[4].u64 = ctx.r[21].u64;
	// 8328B3B8: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8328B3BC: 387F099C  addi r3, r31, 0x99c
	ctx.r[3].s64 = ctx.r[31].s64 + 2460;
	// 8328B3C0: 4AFA1B11  bl 0x8222ced0
	ctx.lr = 0x8328B3C4;
	sub_8222CED0(ctx, base);
	// 8328B3C4: 3C80820F  lis r4, -0x7df1
	ctx.r[4].s64 = -2112946176;
	// 8328B3C8: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8328B3CC: 38844E68  addi r4, r4, 0x4e68
	ctx.r[4].s64 = ctx.r[4].s64 + 20072;
	// 8328B3D0: 387F09A0  addi r3, r31, 0x9a0
	ctx.r[3].s64 = ctx.r[31].s64 + 2464;
	// 8328B3D4: 4AFA1AFD  bl 0x8222ced0
	ctx.lr = 0x8328B3D8;
	sub_8222CED0(ctx, base);
	// 8328B3D8: 39600003  li r11, 3
	ctx.r[11].s64 = 3;
	// 8328B3DC: 7DC47378  mr r4, r14
	ctx.r[4].u64 = ctx.r[14].u64;
	// 8328B3E0: 917F09A4  stw r11, 0x9a4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(2468 as u32), ctx.r[11].u32 ) };
	// 8328B3E4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8328B3E8: 387F09A8  addi r3, r31, 0x9a8
	ctx.r[3].s64 = ctx.r[31].s64 + 2472;
	// 8328B3EC: 4AFA1AE5  bl 0x8222ced0
	ctx.lr = 0x8328B3F0;
	sub_8222CED0(ctx, base);
	// 8328B3F0: 7E84A378  mr r4, r20
	ctx.r[4].u64 = ctx.r[20].u64;
	// 8328B3F4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8328B3F8: 387F09AC  addi r3, r31, 0x9ac
	ctx.r[3].s64 = ctx.r[31].s64 + 2476;
	// 8328B3FC: 4AFA1AD5  bl 0x8222ced0
	ctx.lr = 0x8328B400;
	sub_8222CED0(ctx, base);
	// 8328B400: 3C60820F  lis r3, -0x7df1
	ctx.r[3].s64 = -2112946176;
	// 8328B404: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8328B408: 38834E54  addi r4, r3, 0x4e54
	ctx.r[4].s64 = ctx.r[3].s64 + 20052;
	// 8328B40C: 387F09B0  addi r3, r31, 0x9b0
	ctx.r[3].s64 = ctx.r[31].s64 + 2480;
	// 8328B410: 4AFA1AC1  bl 0x8222ced0
	ctx.lr = 0x8328B414;
	sub_8222CED0(ctx, base);
	// 8328B414: 39600003  li r11, 3
	ctx.r[11].s64 = 3;
	// 8328B418: 7DC47378  mr r4, r14
	ctx.r[4].u64 = ctx.r[14].u64;
	// 8328B41C: 917F09B4  stw r11, 0x9b4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(2484 as u32), ctx.r[11].u32 ) };
	// 8328B420: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8328B424: 387F09B8  addi r3, r31, 0x9b8
	ctx.r[3].s64 = ctx.r[31].s64 + 2488;
	// 8328B428: 4AFA1AA9  bl 0x8222ced0
	ctx.lr = 0x8328B42C;
	sub_8222CED0(ctx, base);
	// 8328B42C: 7E649B78  mr r4, r19
	ctx.r[4].u64 = ctx.r[19].u64;
	// 8328B430: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8328B434: 387F09BC  addi r3, r31, 0x9bc
	ctx.r[3].s64 = ctx.r[31].s64 + 2492;
	// 8328B438: 4AFA1A99  bl 0x8222ced0
	ctx.lr = 0x8328B43C;
	sub_8222CED0(ctx, base);
	// 8328B43C: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 8328B440: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8328B444: 388B4E40  addi r4, r11, 0x4e40
	ctx.r[4].s64 = ctx.r[11].s64 + 20032;
	// 8328B448: 387F09C0  addi r3, r31, 0x9c0
	ctx.r[3].s64 = ctx.r[31].s64 + 2496;
	// 8328B44C: 4AFA1A85  bl 0x8222ced0
	ctx.lr = 0x8328B450;
	sub_8222CED0(ctx, base);
	// 8328B450: 39600003  li r11, 3
	ctx.r[11].s64 = 3;
	// 8328B454: 7DC47378  mr r4, r14
	ctx.r[4].u64 = ctx.r[14].u64;
	// 8328B458: 917F09C4  stw r11, 0x9c4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(2500 as u32), ctx.r[11].u32 ) };
	// 8328B45C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8328B460: 387F09C8  addi r3, r31, 0x9c8
	ctx.r[3].s64 = ctx.r[31].s64 + 2504;
	// 8328B464: 4AFA1A6D  bl 0x8222ced0
	ctx.lr = 0x8328B468;
	sub_8222CED0(ctx, base);
	// 8328B468: 7E449378  mr r4, r18
	ctx.r[4].u64 = ctx.r[18].u64;
	// 8328B46C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8328B470: 387F09CC  addi r3, r31, 0x9cc
	ctx.r[3].s64 = ctx.r[31].s64 + 2508;
	// 8328B474: 4AFA1A5D  bl 0x8222ced0
	ctx.lr = 0x8328B478;
	sub_8222CED0(ctx, base);
	// 8328B478: 3D40820F  lis r10, -0x7df1
	ctx.r[10].s64 = -2112946176;
	// 8328B47C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8328B480: 388A4E20  addi r4, r10, 0x4e20
	ctx.r[4].s64 = ctx.r[10].s64 + 20000;
	// 8328B484: 387F09D0  addi r3, r31, 0x9d0
	ctx.r[3].s64 = ctx.r[31].s64 + 2512;
	// 8328B488: 4AFA1A49  bl 0x8222ced0
	ctx.lr = 0x8328B48C;
	sub_8222CED0(ctx, base);
	// 8328B48C: 39600003  li r11, 3
	ctx.r[11].s64 = 3;
	// 8328B490: 7DC47378  mr r4, r14
	ctx.r[4].u64 = ctx.r[14].u64;
	// 8328B494: 917F09D4  stw r11, 0x9d4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(2516 as u32), ctx.r[11].u32 ) };
	// 8328B498: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8328B49C: 387F09D8  addi r3, r31, 0x9d8
	ctx.r[3].s64 = ctx.r[31].s64 + 2520;
	// 8328B4A0: 4AFA1A31  bl 0x8222ced0
	ctx.lr = 0x8328B4A4;
	sub_8222CED0(ctx, base);
	// 8328B4A4: 7E248B78  mr r4, r17
	ctx.r[4].u64 = ctx.r[17].u64;
	// 8328B4A8: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8328B4AC: 387F09DC  addi r3, r31, 0x9dc
	ctx.r[3].s64 = ctx.r[31].s64 + 2524;
	// 8328B4B0: 4AFA1A21  bl 0x8222ced0
	ctx.lr = 0x8328B4B4;
	sub_8222CED0(ctx, base);
	// 8328B4B4: 3D20820F  lis r9, -0x7df1
	ctx.r[9].s64 = -2112946176;
	// 8328B4B8: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8328B4BC: 38894E00  addi r4, r9, 0x4e00
	ctx.r[4].s64 = ctx.r[9].s64 + 19968;
	// 8328B4C0: 387F09E0  addi r3, r31, 0x9e0
	ctx.r[3].s64 = ctx.r[31].s64 + 2528;
	// 8328B4C4: 4AFA1A0D  bl 0x8222ced0
	ctx.lr = 0x8328B4C8;
	sub_8222CED0(ctx, base);
	// 8328B4C8: 39600003  li r11, 3
	ctx.r[11].s64 = 3;
	// 8328B4CC: 7DC47378  mr r4, r14
	ctx.r[4].u64 = ctx.r[14].u64;
	// 8328B4D0: 917F09E4  stw r11, 0x9e4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(2532 as u32), ctx.r[11].u32 ) };
	// 8328B4D4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8328B4D8: 387F09E8  addi r3, r31, 0x9e8
	ctx.r[3].s64 = ctx.r[31].s64 + 2536;
	// 8328B4DC: 4AFA19F5  bl 0x8222ced0
	ctx.lr = 0x8328B4E0;
	sub_8222CED0(ctx, base);
	// 8328B4E0: 7E048378  mr r4, r16
	ctx.r[4].u64 = ctx.r[16].u64;
	// 8328B4E4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8328B4E8: 387F09EC  addi r3, r31, 0x9ec
	ctx.r[3].s64 = ctx.r[31].s64 + 2540;
	// 8328B4EC: 4AFA19E5  bl 0x8222ced0
	ctx.lr = 0x8328B4F0;
	sub_8222CED0(ctx, base);
	// 8328B4F0: 3D00820F  lis r8, -0x7df1
	ctx.r[8].s64 = -2112946176;
	// 8328B4F4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8328B4F8: 38884DE8  addi r4, r8, 0x4de8
	ctx.r[4].s64 = ctx.r[8].s64 + 19944;
	// 8328B4FC: 387F09F0  addi r3, r31, 0x9f0
	ctx.r[3].s64 = ctx.r[31].s64 + 2544;
	// 8328B500: 4AFA19D1  bl 0x8222ced0
	ctx.lr = 0x8328B504;
	sub_8222CED0(ctx, base);
	// 8328B504: 39600003  li r11, 3
	ctx.r[11].s64 = 3;
	// 8328B508: 7DC47378  mr r4, r14
	ctx.r[4].u64 = ctx.r[14].u64;
	// 8328B50C: 917F09F4  stw r11, 0x9f4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(2548 as u32), ctx.r[11].u32 ) };
	// 8328B510: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8328B514: 387F09F8  addi r3, r31, 0x9f8
	ctx.r[3].s64 = ctx.r[31].s64 + 2552;
	// 8328B518: 4AFA19B9  bl 0x8222ced0
	ctx.lr = 0x8328B51C;
	sub_8222CED0(ctx, base);
	// 8328B51C: 7DE47B78  mr r4, r15
	ctx.r[4].u64 = ctx.r[15].u64;
	// 8328B520: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8328B524: 387F09FC  addi r3, r31, 0x9fc
	ctx.r[3].s64 = ctx.r[31].s64 + 2556;
	// 8328B528: 4AFA19A9  bl 0x8222ced0
	ctx.lr = 0x8328B52C;
	sub_8222CED0(ctx, base);
	// 8328B52C: 3CE0820F  lis r7, -0x7df1
	ctx.r[7].s64 = -2112946176;
	// 8328B530: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8328B534: 38874DD0  addi r4, r7, 0x4dd0
	ctx.r[4].s64 = ctx.r[7].s64 + 19920;
	// 8328B538: 387F0A00  addi r3, r31, 0xa00
	ctx.r[3].s64 = ctx.r[31].s64 + 2560;
	// 8328B53C: 4AFA1995  bl 0x8222ced0
	ctx.lr = 0x8328B540;
	sub_8222CED0(ctx, base);
	// 8328B540: 39600003  li r11, 3
	ctx.r[11].s64 = 3;
	// 8328B544: 7DC47378  mr r4, r14
	ctx.r[4].u64 = ctx.r[14].u64;
	// 8328B548: 917F0A04  stw r11, 0xa04(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(2564 as u32), ctx.r[11].u32 ) };
	// 8328B54C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8328B550: 387F0A08  addi r3, r31, 0xa08
	ctx.r[3].s64 = ctx.r[31].s64 + 2568;
	// 8328B554: 4AFA197D  bl 0x8222ced0
	ctx.lr = 0x8328B558;
	sub_8222CED0(ctx, base);
	// 8328B558: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8328B55C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8328B560: 387F0A0C  addi r3, r31, 0xa0c
	ctx.r[3].s64 = ctx.r[31].s64 + 2572;
	// 8328B564: 4AFA196D  bl 0x8222ced0
	ctx.lr = 0x8328B568;
	sub_8222CED0(ctx, base);
	// 8328B568: 3CC0820F  lis r6, -0x7df1
	ctx.r[6].s64 = -2112946176;
	// 8328B56C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8328B570: 38864DB4  addi r4, r6, 0x4db4
	ctx.r[4].s64 = ctx.r[6].s64 + 19892;
	// 8328B574: 387F0A10  addi r3, r31, 0xa10
	ctx.r[3].s64 = ctx.r[31].s64 + 2576;
	// 8328B578: 4AFA1959  bl 0x8222ced0
	ctx.lr = 0x8328B57C;
	sub_8222CED0(ctx, base);
	// 8328B57C: 39600003  li r11, 3
	ctx.r[11].s64 = 3;
	// 8328B580: 3C80820F  lis r4, -0x7df1
	ctx.r[4].s64 = -2112946176;
	// 8328B584: 917F0A14  stw r11, 0xa14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(2580 as u32), ctx.r[11].u32 ) };
	// 8328B588: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8328B58C: 39C44800  addi r14, r4, 0x4800
	ctx.r[14].s64 = ctx.r[4].s64 + 18432;
	// 8328B590: 387F0A18  addi r3, r31, 0xa18
	ctx.r[3].s64 = ctx.r[31].s64 + 2584;
	// 8328B594: 7DC47378  mr r4, r14
	ctx.r[4].u64 = ctx.r[14].u64;
	// 8328B598: 4AFA1939  bl 0x8222ced0
	ctx.lr = 0x8328B59C;
	sub_8222CED0(ctx, base);
	// 8328B59C: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 8328B5A0: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8328B5A4: 387F0A1C  addi r3, r31, 0xa1c
	ctx.r[3].s64 = ctx.r[31].s64 + 2588;
	// 8328B5A8: 4AFA1929  bl 0x8222ced0
	ctx.lr = 0x8328B5AC;
	sub_8222CED0(ctx, base);
	// 8328B5AC: 3C60820F  lis r3, -0x7df1
	ctx.r[3].s64 = -2112946176;
	// 8328B5B0: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8328B5B4: 38834D9C  addi r4, r3, 0x4d9c
	ctx.r[4].s64 = ctx.r[3].s64 + 19868;
	// 8328B5B8: 387F0A20  addi r3, r31, 0xa20
	ctx.r[3].s64 = ctx.r[31].s64 + 2592;
	// 8328B5BC: 4AFA1915  bl 0x8222ced0
	ctx.lr = 0x8328B5C0;
	sub_8222CED0(ctx, base);
	// 8328B5C0: 39600003  li r11, 3
	ctx.r[11].s64 = 3;
	// 8328B5C4: 7DC47378  mr r4, r14
	ctx.r[4].u64 = ctx.r[14].u64;
	// 8328B5C8: 917F0A24  stw r11, 0xa24(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(2596 as u32), ctx.r[11].u32 ) };
	// 8328B5CC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8328B5D0: 387F0A28  addi r3, r31, 0xa28
	ctx.r[3].s64 = ctx.r[31].s64 + 2600;
	// 8328B5D4: 4AFA18FD  bl 0x8222ced0
	ctx.lr = 0x8328B5D8;
	sub_8222CED0(ctx, base);
	// 8328B5D8: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 8328B5DC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8328B5E0: 387F0A2C  addi r3, r31, 0xa2c
	ctx.r[3].s64 = ctx.r[31].s64 + 2604;
	// 8328B5E4: 4AFA18ED  bl 0x8222ced0
	ctx.lr = 0x8328B5E8;
	sub_8222CED0(ctx, base);
	// 8328B5E8: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 8328B5EC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8328B5F0: 388B4D7C  addi r4, r11, 0x4d7c
	ctx.r[4].s64 = ctx.r[11].s64 + 19836;
	// 8328B5F4: 387F0A30  addi r3, r31, 0xa30
	ctx.r[3].s64 = ctx.r[31].s64 + 2608;
	// 8328B5F8: 4AFA18D9  bl 0x8222ced0
	ctx.lr = 0x8328B5FC;
	sub_8222CED0(ctx, base);
	// 8328B5FC: 39600003  li r11, 3
	ctx.r[11].s64 = 3;
	// 8328B600: 7DC47378  mr r4, r14
	ctx.r[4].u64 = ctx.r[14].u64;
	// 8328B604: 917F0A34  stw r11, 0xa34(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(2612 as u32), ctx.r[11].u32 ) };
	// 8328B608: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8328B60C: 387F0A38  addi r3, r31, 0xa38
	ctx.r[3].s64 = ctx.r[31].s64 + 2616;
	// 8328B610: 4AFA18C1  bl 0x8222ced0
	ctx.lr = 0x8328B614;
	sub_8222CED0(ctx, base);
	// 8328B614: 7F44D378  mr r4, r26
	ctx.r[4].u64 = ctx.r[26].u64;
	// 8328B618: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8328B61C: 387F0A3C  addi r3, r31, 0xa3c
	ctx.r[3].s64 = ctx.r[31].s64 + 2620;
	// 8328B620: 4AFA18B1  bl 0x8222ced0
	ctx.lr = 0x8328B624;
	sub_8222CED0(ctx, base);
	// 8328B624: 3D40820F  lis r10, -0x7df1
	ctx.r[10].s64 = -2112946176;
	// 8328B628: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8328B62C: 388A4D58  addi r4, r10, 0x4d58
	ctx.r[4].s64 = ctx.r[10].s64 + 19800;
	// 8328B630: 387F0A40  addi r3, r31, 0xa40
	ctx.r[3].s64 = ctx.r[31].s64 + 2624;
	// 8328B634: 4AFA189D  bl 0x8222ced0
	ctx.lr = 0x8328B638;
	sub_8222CED0(ctx, base);
	// 8328B638: 39600003  li r11, 3
	ctx.r[11].s64 = 3;
	// 8328B63C: 7DC47378  mr r4, r14
	ctx.r[4].u64 = ctx.r[14].u64;
	// 8328B640: 917F0A44  stw r11, 0xa44(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(2628 as u32), ctx.r[11].u32 ) };
	// 8328B644: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8328B648: 387F0A48  addi r3, r31, 0xa48
	ctx.r[3].s64 = ctx.r[31].s64 + 2632;
	// 8328B64C: 4AFA1885  bl 0x8222ced0
	ctx.lr = 0x8328B650;
	sub_8222CED0(ctx, base);
	// 8328B650: 7F24CB78  mr r4, r25
	ctx.r[4].u64 = ctx.r[25].u64;
	// 8328B654: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8328B658: 387F0A4C  addi r3, r31, 0xa4c
	ctx.r[3].s64 = ctx.r[31].s64 + 2636;
	// 8328B65C: 4AFA1875  bl 0x8222ced0
	ctx.lr = 0x8328B660;
	sub_8222CED0(ctx, base);
	// 8328B660: 7F04C378  mr r4, r24
	ctx.r[4].u64 = ctx.r[24].u64;
	// 8328B664: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8328B668: 387F0A50  addi r3, r31, 0xa50
	ctx.r[3].s64 = ctx.r[31].s64 + 2640;
	// 8328B66C: 4AFA1865  bl 0x8222ced0
	ctx.lr = 0x8328B670;
	sub_8222CED0(ctx, base);
	// 8328B670: 39600003  li r11, 3
	ctx.r[11].s64 = 3;
	// 8328B674: 7DC47378  mr r4, r14
	ctx.r[4].u64 = ctx.r[14].u64;
	// 8328B678: 917F0A54  stw r11, 0xa54(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(2644 as u32), ctx.r[11].u32 ) };
	// 8328B67C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8328B680: 387F0A58  addi r3, r31, 0xa58
	ctx.r[3].s64 = ctx.r[31].s64 + 2648;
	// 8328B684: 4AFA184D  bl 0x8222ced0
	ctx.lr = 0x8328B688;
	sub_8222CED0(ctx, base);
	// 8328B688: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8328B68C: 7EE4BB78  mr r4, r23
	ctx.r[4].u64 = ctx.r[23].u64;
	// 8328B690: 387F0A5C  addi r3, r31, 0xa5c
	ctx.r[3].s64 = ctx.r[31].s64 + 2652;
	// 8328B694: 4AFA183D  bl 0x8222ced0
	ctx.lr = 0x8328B698;
	sub_8222CED0(ctx, base);
	// 8328B698: 7EC4B378  mr r4, r22
	ctx.r[4].u64 = ctx.r[22].u64;
	// 8328B69C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8328B6A0: 387F0A60  addi r3, r31, 0xa60
	ctx.r[3].s64 = ctx.r[31].s64 + 2656;
	// 8328B6A4: 4AFA182D  bl 0x8222ced0
	ctx.lr = 0x8328B6A8;
	sub_8222CED0(ctx, base);
	// 8328B6A8: 39600003  li r11, 3
	ctx.r[11].s64 = 3;
	// 8328B6AC: 7DC47378  mr r4, r14
	ctx.r[4].u64 = ctx.r[14].u64;
	// 8328B6B0: 917F0A64  stw r11, 0xa64(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(2660 as u32), ctx.r[11].u32 ) };
	// 8328B6B4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8328B6B8: 387F0A68  addi r3, r31, 0xa68
	ctx.r[3].s64 = ctx.r[31].s64 + 2664;
	// 8328B6BC: 4AFA1815  bl 0x8222ced0
	ctx.lr = 0x8328B6C0;
	sub_8222CED0(ctx, base);
	// 8328B6C0: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 8328B6C4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8328B6C8: 387F0A6C  addi r3, r31, 0xa6c
	ctx.r[3].s64 = ctx.r[31].s64 + 2668;
	// 8328B6CC: 4AFA1805  bl 0x8222ced0
	ctx.lr = 0x8328B6D0;
	sub_8222CED0(ctx, base);
	// 8328B6D0: 3D20820F  lis r9, -0x7df1
	ctx.r[9].s64 = -2112946176;
	// 8328B6D4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8328B6D8: 38894D40  addi r4, r9, 0x4d40
	ctx.r[4].s64 = ctx.r[9].s64 + 19776;
	// 8328B6DC: 387F0A70  addi r3, r31, 0xa70
	ctx.r[3].s64 = ctx.r[31].s64 + 2672;
	// 8328B6E0: 4AFA17F1  bl 0x8222ced0
	ctx.lr = 0x8328B6E4;
	sub_8222CED0(ctx, base);
	// 8328B6E4: 39600003  li r11, 3
	ctx.r[11].s64 = 3;
	// 8328B6E8: 7DC47378  mr r4, r14
	ctx.r[4].u64 = ctx.r[14].u64;
	// 8328B6EC: 917F0A74  stw r11, 0xa74(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(2676 as u32), ctx.r[11].u32 ) };
	// 8328B6F0: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8328B6F4: 387F0A78  addi r3, r31, 0xa78
	ctx.r[3].s64 = ctx.r[31].s64 + 2680;
	// 8328B6F8: 4AFA17D9  bl 0x8222ced0
	ctx.lr = 0x8328B6FC;
	sub_8222CED0(ctx, base);
	// 8328B6FC: 7EA4AB78  mr r4, r21
	ctx.r[4].u64 = ctx.r[21].u64;
	// 8328B700: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8328B704: 387F0A7C  addi r3, r31, 0xa7c
	ctx.r[3].s64 = ctx.r[31].s64 + 2684;
	// 8328B708: 4AFA17C9  bl 0x8222ced0
	ctx.lr = 0x8328B70C;
	sub_8222CED0(ctx, base);
	// 8328B70C: 3D00820F  lis r8, -0x7df1
	ctx.r[8].s64 = -2112946176;
	// 8328B710: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8328B714: 38884D28  addi r4, r8, 0x4d28
	ctx.r[4].s64 = ctx.r[8].s64 + 19752;
	// 8328B718: 387F0A80  addi r3, r31, 0xa80
	ctx.r[3].s64 = ctx.r[31].s64 + 2688;
	// 8328B71C: 4AFA17B5  bl 0x8222ced0
	ctx.lr = 0x8328B720;
	sub_8222CED0(ctx, base);
	// 8328B720: 39600003  li r11, 3
	ctx.r[11].s64 = 3;
	// 8328B724: 7DC47378  mr r4, r14
	ctx.r[4].u64 = ctx.r[14].u64;
	// 8328B728: 917F0A84  stw r11, 0xa84(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(2692 as u32), ctx.r[11].u32 ) };
	// 8328B72C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8328B730: 387F0A88  addi r3, r31, 0xa88
	ctx.r[3].s64 = ctx.r[31].s64 + 2696;
	// 8328B734: 4AFA179D  bl 0x8222ced0
	ctx.lr = 0x8328B738;
	sub_8222CED0(ctx, base);
	// 8328B738: 7E84A378  mr r4, r20
	ctx.r[4].u64 = ctx.r[20].u64;
	// 8328B73C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8328B740: 387F0A8C  addi r3, r31, 0xa8c
	ctx.r[3].s64 = ctx.r[31].s64 + 2700;
	// 8328B744: 4AFA178D  bl 0x8222ced0
	ctx.lr = 0x8328B748;
	sub_8222CED0(ctx, base);
	// 8328B748: 3CE0820F  lis r7, -0x7df1
	ctx.r[7].s64 = -2112946176;
	// 8328B74C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8328B750: 38874D14  addi r4, r7, 0x4d14
	ctx.r[4].s64 = ctx.r[7].s64 + 19732;
	// 8328B754: 387F0A90  addi r3, r31, 0xa90
	ctx.r[3].s64 = ctx.r[31].s64 + 2704;
	// 8328B758: 4AFA1779  bl 0x8222ced0
	ctx.lr = 0x8328B75C;
	sub_8222CED0(ctx, base);
	// 8328B75C: 39600003  li r11, 3
	ctx.r[11].s64 = 3;
	// 8328B760: 7DC47378  mr r4, r14
	ctx.r[4].u64 = ctx.r[14].u64;
	// 8328B764: 917F0A94  stw r11, 0xa94(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(2708 as u32), ctx.r[11].u32 ) };
	// 8328B768: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8328B76C: 387F0A98  addi r3, r31, 0xa98
	ctx.r[3].s64 = ctx.r[31].s64 + 2712;
	// 8328B770: 4AFA1761  bl 0x8222ced0
	ctx.lr = 0x8328B774;
	sub_8222CED0(ctx, base);
	// 8328B774: 7E649B78  mr r4, r19
	ctx.r[4].u64 = ctx.r[19].u64;
	// 8328B778: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8328B77C: 387F0A9C  addi r3, r31, 0xa9c
	ctx.r[3].s64 = ctx.r[31].s64 + 2716;
	// 8328B780: 4AFA1751  bl 0x8222ced0
	ctx.lr = 0x8328B784;
	sub_8222CED0(ctx, base);
	// 8328B784: 3CC0820F  lis r6, -0x7df1
	ctx.r[6].s64 = -2112946176;
	// 8328B788: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8328B78C: 38864D00  addi r4, r6, 0x4d00
	ctx.r[4].s64 = ctx.r[6].s64 + 19712;
	// 8328B790: 387F0AA0  addi r3, r31, 0xaa0
	ctx.r[3].s64 = ctx.r[31].s64 + 2720;
	// 8328B794: 4AFA173D  bl 0x8222ced0
	ctx.lr = 0x8328B798;
	sub_8222CED0(ctx, base);
	// 8328B798: 39600003  li r11, 3
	ctx.r[11].s64 = 3;
	// 8328B79C: 7DC47378  mr r4, r14
	ctx.r[4].u64 = ctx.r[14].u64;
	// 8328B7A0: 917F0AA4  stw r11, 0xaa4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(2724 as u32), ctx.r[11].u32 ) };
	// 8328B7A4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8328B7A8: 387F0AA8  addi r3, r31, 0xaa8
	ctx.r[3].s64 = ctx.r[31].s64 + 2728;
	// 8328B7AC: 4AFA1725  bl 0x8222ced0
	ctx.lr = 0x8328B7B0;
	sub_8222CED0(ctx, base);
	// 8328B7B0: 7E449378  mr r4, r18
	ctx.r[4].u64 = ctx.r[18].u64;
	// 8328B7B4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8328B7B8: 387F0AAC  addi r3, r31, 0xaac
	ctx.r[3].s64 = ctx.r[31].s64 + 2732;
	// 8328B7BC: 4AFA1715  bl 0x8222ced0
	ctx.lr = 0x8328B7C0;
	sub_8222CED0(ctx, base);
	// 8328B7C0: 3C80820F  lis r4, -0x7df1
	ctx.r[4].s64 = -2112946176;
	// 8328B7C4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8328B7C8: 38844CE0  addi r4, r4, 0x4ce0
	ctx.r[4].s64 = ctx.r[4].s64 + 19680;
	// 8328B7CC: 387F0AB0  addi r3, r31, 0xab0
	ctx.r[3].s64 = ctx.r[31].s64 + 2736;
	// 8328B7D0: 4AFA1701  bl 0x8222ced0
	ctx.lr = 0x8328B7D4;
	sub_8222CED0(ctx, base);
	// 8328B7D4: 39600003  li r11, 3
	ctx.r[11].s64 = 3;
	// 8328B7D8: 7DC47378  mr r4, r14
	ctx.r[4].u64 = ctx.r[14].u64;
	// 8328B7DC: 917F0AB4  stw r11, 0xab4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(2740 as u32), ctx.r[11].u32 ) };
	// 8328B7E0: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8328B7E4: 387F0AB8  addi r3, r31, 0xab8
	ctx.r[3].s64 = ctx.r[31].s64 + 2744;
	// 8328B7E8: 4AFA16E9  bl 0x8222ced0
	ctx.lr = 0x8328B7EC;
	sub_8222CED0(ctx, base);
	// 8328B7EC: 7E248B78  mr r4, r17
	ctx.r[4].u64 = ctx.r[17].u64;
	// 8328B7F0: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8328B7F4: 387F0ABC  addi r3, r31, 0xabc
	ctx.r[3].s64 = ctx.r[31].s64 + 2748;
	// 8328B7F8: 4AFA16D9  bl 0x8222ced0
	ctx.lr = 0x8328B7FC;
	sub_8222CED0(ctx, base);
	// 8328B7FC: 3C60820F  lis r3, -0x7df1
	ctx.r[3].s64 = -2112946176;
	// 8328B800: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8328B804: 38834CC0  addi r4, r3, 0x4cc0
	ctx.r[4].s64 = ctx.r[3].s64 + 19648;
	// 8328B808: 387F0AC0  addi r3, r31, 0xac0
	ctx.r[3].s64 = ctx.r[31].s64 + 2752;
	// 8328B80C: 4AFA16C5  bl 0x8222ced0
	ctx.lr = 0x8328B810;
	sub_8222CED0(ctx, base);
	// 8328B810: 39600003  li r11, 3
	ctx.r[11].s64 = 3;
	// 8328B814: 7DC47378  mr r4, r14
	ctx.r[4].u64 = ctx.r[14].u64;
	// 8328B818: 917F0AC4  stw r11, 0xac4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(2756 as u32), ctx.r[11].u32 ) };
	// 8328B81C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8328B820: 387F0AC8  addi r3, r31, 0xac8
	ctx.r[3].s64 = ctx.r[31].s64 + 2760;
	// 8328B824: 4AFA16AD  bl 0x8222ced0
	ctx.lr = 0x8328B828;
	sub_8222CED0(ctx, base);
	// 8328B828: 7E048378  mr r4, r16
	ctx.r[4].u64 = ctx.r[16].u64;
	// 8328B82C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8328B830: 387F0ACC  addi r3, r31, 0xacc
	ctx.r[3].s64 = ctx.r[31].s64 + 2764;
	// 8328B834: 4AFA169D  bl 0x8222ced0
	ctx.lr = 0x8328B838;
	sub_8222CED0(ctx, base);
	// 8328B838: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 8328B83C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8328B840: 388B4CA8  addi r4, r11, 0x4ca8
	ctx.r[4].s64 = ctx.r[11].s64 + 19624;
	// 8328B844: 387F0AD0  addi r3, r31, 0xad0
	ctx.r[3].s64 = ctx.r[31].s64 + 2768;
	// 8328B848: 4AFA1689  bl 0x8222ced0
	ctx.lr = 0x8328B84C;
	sub_8222CED0(ctx, base);
	// 8328B84C: 39600003  li r11, 3
	ctx.r[11].s64 = 3;
	// 8328B850: 7DC47378  mr r4, r14
	ctx.r[4].u64 = ctx.r[14].u64;
	// 8328B854: 917F0AD4  stw r11, 0xad4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(2772 as u32), ctx.r[11].u32 ) };
	// 8328B858: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8328B85C: 387F0AD8  addi r3, r31, 0xad8
	ctx.r[3].s64 = ctx.r[31].s64 + 2776;
	// 8328B860: 4AFA1671  bl 0x8222ced0
	ctx.lr = 0x8328B864;
	sub_8222CED0(ctx, base);
	// 8328B864: 7DE47B78  mr r4, r15
	ctx.r[4].u64 = ctx.r[15].u64;
	// 8328B868: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8328B86C: 387F0ADC  addi r3, r31, 0xadc
	ctx.r[3].s64 = ctx.r[31].s64 + 2780;
	// 8328B870: 4AFA1661  bl 0x8222ced0
	ctx.lr = 0x8328B874;
	sub_8222CED0(ctx, base);
	// 8328B874: 3D40820F  lis r10, -0x7df1
	ctx.r[10].s64 = -2112946176;
	// 8328B878: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8328B87C: 388A4C90  addi r4, r10, 0x4c90
	ctx.r[4].s64 = ctx.r[10].s64 + 19600;
	// 8328B880: 387F0AE0  addi r3, r31, 0xae0
	ctx.r[3].s64 = ctx.r[31].s64 + 2784;
	// 8328B884: 4AFA164D  bl 0x8222ced0
	ctx.lr = 0x8328B888;
	sub_8222CED0(ctx, base);
	// 8328B888: 39600003  li r11, 3
	ctx.r[11].s64 = 3;
	// 8328B88C: 7DC47378  mr r4, r14
	ctx.r[4].u64 = ctx.r[14].u64;
	// 8328B890: 917F0AE4  stw r11, 0xae4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(2788 as u32), ctx.r[11].u32 ) };
	// 8328B894: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8328B898: 387F0AE8  addi r3, r31, 0xae8
	ctx.r[3].s64 = ctx.r[31].s64 + 2792;
	// 8328B89C: 4AFA1635  bl 0x8222ced0
	ctx.lr = 0x8328B8A0;
	sub_8222CED0(ctx, base);
	// 8328B8A0: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8328B8A4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8328B8A8: 387F0AEC  addi r3, r31, 0xaec
	ctx.r[3].s64 = ctx.r[31].s64 + 2796;
	// 8328B8AC: 4AFA1625  bl 0x8222ced0
	ctx.lr = 0x8328B8B0;
	sub_8222CED0(ctx, base);
	// 8328B8B0: 3D20820F  lis r9, -0x7df1
	ctx.r[9].s64 = -2112946176;
	// 8328B8B4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8328B8B8: 38894C74  addi r4, r9, 0x4c74
	ctx.r[4].s64 = ctx.r[9].s64 + 19572;
	// 8328B8BC: 387F0AF0  addi r3, r31, 0xaf0
	ctx.r[3].s64 = ctx.r[31].s64 + 2800;
	// 8328B8C0: 4AFA1611  bl 0x8222ced0
	ctx.lr = 0x8328B8C4;
	sub_8222CED0(ctx, base);
	// 8328B8C4: 39600003  li r11, 3
	ctx.r[11].s64 = 3;
	// 8328B8C8: 3D00820F  lis r8, -0x7df1
	ctx.r[8].s64 = -2112946176;
	// 8328B8CC: 917F0AF4  stw r11, 0xaf4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(2804 as u32), ctx.r[11].u32 ) };
	// 8328B8D0: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8328B8D4: 3B0847D4  addi r24, r8, 0x47d4
	ctx.r[24].s64 = ctx.r[8].s64 + 18388;
	// 8328B8D8: 387F0AF8  addi r3, r31, 0xaf8
	ctx.r[3].s64 = ctx.r[31].s64 + 2808;
	// 8328B8DC: 7F04C378  mr r4, r24
	ctx.r[4].u64 = ctx.r[24].u64;
	// 8328B8E0: 4AFA15F1  bl 0x8222ced0
	ctx.lr = 0x8328B8E4;
	sub_8222CED0(ctx, base);
	// 8328B8E4: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 8328B8E8: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8328B8EC: 387F0AFC  addi r3, r31, 0xafc
	ctx.r[3].s64 = ctx.r[31].s64 + 2812;
	// 8328B8F0: 4AFA15E1  bl 0x8222ced0
	ctx.lr = 0x8328B8F4;
	sub_8222CED0(ctx, base);
	// 8328B8F4: 3CE0820F  lis r7, -0x7df1
	ctx.r[7].s64 = -2112946176;
	// 8328B8F8: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8328B8FC: 38874C5C  addi r4, r7, 0x4c5c
	ctx.r[4].s64 = ctx.r[7].s64 + 19548;
	// 8328B900: 387F0B00  addi r3, r31, 0xb00
	ctx.r[3].s64 = ctx.r[31].s64 + 2816;
	// 8328B904: 4AFA15CD  bl 0x8222ced0
	ctx.lr = 0x8328B908;
	sub_8222CED0(ctx, base);
	// 8328B908: 39600003  li r11, 3
	ctx.r[11].s64 = 3;
	// 8328B90C: 7F04C378  mr r4, r24
	ctx.r[4].u64 = ctx.r[24].u64;
	// 8328B910: 917F0B04  stw r11, 0xb04(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(2820 as u32), ctx.r[11].u32 ) };
	// 8328B914: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8328B918: 387F0B08  addi r3, r31, 0xb08
	ctx.r[3].s64 = ctx.r[31].s64 + 2824;
	// 8328B91C: 4AFA15B5  bl 0x8222ced0
	ctx.lr = 0x8328B920;
	sub_8222CED0(ctx, base);
	// 8328B920: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8328B924: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 8328B928: 387F0B0C  addi r3, r31, 0xb0c
	ctx.r[3].s64 = ctx.r[31].s64 + 2828;
	// 8328B92C: 4AFA15A5  bl 0x8222ced0
	ctx.lr = 0x8328B930;
	sub_8222CED0(ctx, base);
	// 8328B930: 3CC0820F  lis r6, -0x7df1
	ctx.r[6].s64 = -2112946176;
	// 8328B934: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8328B938: 38864C3C  addi r4, r6, 0x4c3c
	ctx.r[4].s64 = ctx.r[6].s64 + 19516;
	// 8328B93C: 387F0B10  addi r3, r31, 0xb10
	ctx.r[3].s64 = ctx.r[31].s64 + 2832;
	// 8328B940: 4AFA1591  bl 0x8222ced0
	ctx.lr = 0x8328B944;
	sub_8222CED0(ctx, base);
	// 8328B944: 39600003  li r11, 3
	ctx.r[11].s64 = 3;
	// 8328B948: 7F04C378  mr r4, r24
	ctx.r[4].u64 = ctx.r[24].u64;
	// 8328B94C: 917F0B14  stw r11, 0xb14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(2836 as u32), ctx.r[11].u32 ) };
	// 8328B950: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8328B954: 387F0B18  addi r3, r31, 0xb18
	ctx.r[3].s64 = ctx.r[31].s64 + 2840;
	// 8328B958: 4AFA1579  bl 0x8222ced0
	ctx.lr = 0x8328B95C;
	sub_8222CED0(ctx, base);
	// 8328B95C: 7F44D378  mr r4, r26
	ctx.r[4].u64 = ctx.r[26].u64;
	// 8328B960: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8328B964: 387F0B1C  addi r3, r31, 0xb1c
	ctx.r[3].s64 = ctx.r[31].s64 + 2844;
	// 8328B968: 4AFA1569  bl 0x8222ced0
	ctx.lr = 0x8328B96C;
	sub_8222CED0(ctx, base);
	// 8328B96C: 3C80820F  lis r4, -0x7df1
	ctx.r[4].s64 = -2112946176;
	// 8328B970: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8328B974: 38844C18  addi r4, r4, 0x4c18
	ctx.r[4].s64 = ctx.r[4].s64 + 19480;
	// 8328B978: 387F0B20  addi r3, r31, 0xb20
	ctx.r[3].s64 = ctx.r[31].s64 + 2848;
	// 8328B97C: 4AFA1555  bl 0x8222ced0
	ctx.lr = 0x8328B980;
	sub_8222CED0(ctx, base);
	// 8328B980: 39600003  li r11, 3
	ctx.r[11].s64 = 3;
	// 8328B984: 7F04C378  mr r4, r24
	ctx.r[4].u64 = ctx.r[24].u64;
	// 8328B988: 917F0B24  stw r11, 0xb24(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(2852 as u32), ctx.r[11].u32 ) };
	// 8328B98C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8328B990: 387F0B28  addi r3, r31, 0xb28
	ctx.r[3].s64 = ctx.r[31].s64 + 2856;
	// 8328B994: 4AFA153D  bl 0x8222ced0
	ctx.lr = 0x8328B998;
	sub_8222CED0(ctx, base);
	// 8328B998: 7F24CB78  mr r4, r25
	ctx.r[4].u64 = ctx.r[25].u64;
	// 8328B99C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8328B9A0: 387F0B2C  addi r3, r31, 0xb2c
	ctx.r[3].s64 = ctx.r[31].s64 + 2860;
	// 8328B9A4: 4AFA152D  bl 0x8222ced0
	ctx.lr = 0x8328B9A8;
	sub_8222CED0(ctx, base);
	// 8328B9A8: 3C60820F  lis r3, -0x7df1
	ctx.r[3].s64 = -2112946176;
	// 8328B9AC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8328B9B0: 38834BFC  addi r4, r3, 0x4bfc
	ctx.r[4].s64 = ctx.r[3].s64 + 19452;
	// 8328B9B4: 387F0B30  addi r3, r31, 0xb30
	ctx.r[3].s64 = ctx.r[31].s64 + 2864;
	// 8328B9B8: 4AFA1519  bl 0x8222ced0
	ctx.lr = 0x8328B9BC;
	sub_8222CED0(ctx, base);
	// 8328B9BC: 39600003  li r11, 3
	ctx.r[11].s64 = 3;
	// 8328B9C0: 7F04C378  mr r4, r24
	ctx.r[4].u64 = ctx.r[24].u64;
	// 8328B9C4: 917F0B34  stw r11, 0xb34(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(2868 as u32), ctx.r[11].u32 ) };
	// 8328B9C8: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8328B9CC: 387F0B38  addi r3, r31, 0xb38
	ctx.r[3].s64 = ctx.r[31].s64 + 2872;
	// 8328B9D0: 4AFA1501  bl 0x8222ced0
	ctx.lr = 0x8328B9D4;
	sub_8222CED0(ctx, base);
	// 8328B9D4: 7EE4BB78  mr r4, r23
	ctx.r[4].u64 = ctx.r[23].u64;
	// 8328B9D8: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8328B9DC: 387F0B3C  addi r3, r31, 0xb3c
	ctx.r[3].s64 = ctx.r[31].s64 + 2876;
	// 8328B9E0: 4AFA14F1  bl 0x8222ced0
	ctx.lr = 0x8328B9E4;
	sub_8222CED0(ctx, base);
	// 8328B9E4: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 8328B9E8: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8328B9EC: 388B4BE0  addi r4, r11, 0x4be0
	ctx.r[4].s64 = ctx.r[11].s64 + 19424;
	// 8328B9F0: 387F0B40  addi r3, r31, 0xb40
	ctx.r[3].s64 = ctx.r[31].s64 + 2880;
	// 8328B9F4: 4AFA14DD  bl 0x8222ced0
	ctx.lr = 0x8328B9F8;
	sub_8222CED0(ctx, base);
	// 8328B9F8: 39600003  li r11, 3
	ctx.r[11].s64 = 3;
	// 8328B9FC: 7F04C378  mr r4, r24
	ctx.r[4].u64 = ctx.r[24].u64;
	// 8328BA00: 917F0B44  stw r11, 0xb44(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(2884 as u32), ctx.r[11].u32 ) };
	// 8328BA04: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8328BA08: 387F0B48  addi r3, r31, 0xb48
	ctx.r[3].s64 = ctx.r[31].s64 + 2888;
	// 8328BA0C: 4AFA14C5  bl 0x8222ced0
	ctx.lr = 0x8328BA10;
	sub_8222CED0(ctx, base);
	// 8328BA10: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 8328BA14: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8328BA18: 387F0B4C  addi r3, r31, 0xb4c
	ctx.r[3].s64 = ctx.r[31].s64 + 2892;
	// 8328BA1C: 4AFA14B5  bl 0x8222ced0
	ctx.lr = 0x8328BA20;
	sub_8222CED0(ctx, base);
	// 8328BA20: 3D40820F  lis r10, -0x7df1
	ctx.r[10].s64 = -2112946176;
	// 8328BA24: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8328BA28: 388A4BC8  addi r4, r10, 0x4bc8
	ctx.r[4].s64 = ctx.r[10].s64 + 19400;
	// 8328BA2C: 387F0B50  addi r3, r31, 0xb50
	ctx.r[3].s64 = ctx.r[31].s64 + 2896;
	// 8328BA30: 4AFA14A1  bl 0x8222ced0
	ctx.lr = 0x8328BA34;
	sub_8222CED0(ctx, base);
	// 8328BA34: 39600003  li r11, 3
	ctx.r[11].s64 = 3;
	// 8328BA38: 7F04C378  mr r4, r24
	ctx.r[4].u64 = ctx.r[24].u64;
	// 8328BA3C: 917F0B54  stw r11, 0xb54(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(2900 as u32), ctx.r[11].u32 ) };
	// 8328BA40: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8328BA44: 387F0B58  addi r3, r31, 0xb58
	ctx.r[3].s64 = ctx.r[31].s64 + 2904;
	// 8328BA48: 4AFA1489  bl 0x8222ced0
	ctx.lr = 0x8328BA4C;
	sub_8222CED0(ctx, base);
	// 8328BA4C: 7EA4AB78  mr r4, r21
	ctx.r[4].u64 = ctx.r[21].u64;
	// 8328BA50: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8328BA54: 387F0B5C  addi r3, r31, 0xb5c
	ctx.r[3].s64 = ctx.r[31].s64 + 2908;
	// 8328BA58: 4AFA1479  bl 0x8222ced0
	ctx.lr = 0x8328BA5C;
	sub_8222CED0(ctx, base);
	// 8328BA5C: 3D20820F  lis r9, -0x7df1
	ctx.r[9].s64 = -2112946176;
	// 8328BA60: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8328BA64: 38894BB0  addi r4, r9, 0x4bb0
	ctx.r[4].s64 = ctx.r[9].s64 + 19376;
	// 8328BA68: 387F0B60  addi r3, r31, 0xb60
	ctx.r[3].s64 = ctx.r[31].s64 + 2912;
	// 8328BA6C: 4AFA1465  bl 0x8222ced0
	ctx.lr = 0x8328BA70;
	sub_8222CED0(ctx, base);
	// 8328BA70: 39600003  li r11, 3
	ctx.r[11].s64 = 3;
	// 8328BA74: 7F04C378  mr r4, r24
	ctx.r[4].u64 = ctx.r[24].u64;
	// 8328BA78: 917F0B64  stw r11, 0xb64(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(2916 as u32), ctx.r[11].u32 ) };
	// 8328BA7C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8328BA80: 387F0B68  addi r3, r31, 0xb68
	ctx.r[3].s64 = ctx.r[31].s64 + 2920;
	// 8328BA84: 4AFA144D  bl 0x8222ced0
	ctx.lr = 0x8328BA88;
	sub_8222CED0(ctx, base);
	// 8328BA88: 7E84A378  mr r4, r20
	ctx.r[4].u64 = ctx.r[20].u64;
	// 8328BA8C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8328BA90: 387F0B6C  addi r3, r31, 0xb6c
	ctx.r[3].s64 = ctx.r[31].s64 + 2924;
	// 8328BA94: 4AFA143D  bl 0x8222ced0
	ctx.lr = 0x8328BA98;
	sub_8222CED0(ctx, base);
	// 8328BA98: 3D00820F  lis r8, -0x7df1
	ctx.r[8].s64 = -2112946176;
	// 8328BA9C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8328BAA0: 38884B9C  addi r4, r8, 0x4b9c
	ctx.r[4].s64 = ctx.r[8].s64 + 19356;
	// 8328BAA4: 387F0B70  addi r3, r31, 0xb70
	ctx.r[3].s64 = ctx.r[31].s64 + 2928;
	// 8328BAA8: 4AFA1429  bl 0x8222ced0
	ctx.lr = 0x8328BAAC;
	sub_8222CED0(ctx, base);
	// 8328BAAC: 39600003  li r11, 3
	ctx.r[11].s64 = 3;
	// 8328BAB0: 7F04C378  mr r4, r24
	ctx.r[4].u64 = ctx.r[24].u64;
	// 8328BAB4: 917F0B74  stw r11, 0xb74(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(2932 as u32), ctx.r[11].u32 ) };
	// 8328BAB8: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8328BABC: 387F0B78  addi r3, r31, 0xb78
	ctx.r[3].s64 = ctx.r[31].s64 + 2936;
	// 8328BAC0: 4AFA1411  bl 0x8222ced0
	ctx.lr = 0x8328BAC4;
	sub_8222CED0(ctx, base);
	// 8328BAC4: 7E649B78  mr r4, r19
	ctx.r[4].u64 = ctx.r[19].u64;
	// 8328BAC8: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8328BACC: 387F0B7C  addi r3, r31, 0xb7c
	ctx.r[3].s64 = ctx.r[31].s64 + 2940;
	// 8328BAD0: 4AFA1401  bl 0x8222ced0
	ctx.lr = 0x8328BAD4;
	sub_8222CED0(ctx, base);
	// 8328BAD4: 3CE0820F  lis r7, -0x7df1
	ctx.r[7].s64 = -2112946176;
	// 8328BAD8: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8328BADC: 38874B88  addi r4, r7, 0x4b88
	ctx.r[4].s64 = ctx.r[7].s64 + 19336;
	// 8328BAE0: 387F0B80  addi r3, r31, 0xb80
	ctx.r[3].s64 = ctx.r[31].s64 + 2944;
	// 8328BAE4: 4AFA13ED  bl 0x8222ced0
	ctx.lr = 0x8328BAE8;
	sub_8222CED0(ctx, base);
	// 8328BAE8: 39600003  li r11, 3
	ctx.r[11].s64 = 3;
	// 8328BAEC: 7F04C378  mr r4, r24
	ctx.r[4].u64 = ctx.r[24].u64;
	// 8328BAF0: 917F0B84  stw r11, 0xb84(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(2948 as u32), ctx.r[11].u32 ) };
	// 8328BAF4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8328BAF8: 387F0B88  addi r3, r31, 0xb88
	ctx.r[3].s64 = ctx.r[31].s64 + 2952;
	// 8328BAFC: 4AFA13D5  bl 0x8222ced0
	ctx.lr = 0x8328BB00;
	sub_8222CED0(ctx, base);
	// 8328BB00: 7E449378  mr r4, r18
	ctx.r[4].u64 = ctx.r[18].u64;
	// 8328BB04: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8328BB08: 387F0B8C  addi r3, r31, 0xb8c
	ctx.r[3].s64 = ctx.r[31].s64 + 2956;
	// 8328BB0C: 4AFA13C5  bl 0x8222ced0
	ctx.lr = 0x8328BB10;
	sub_8222CED0(ctx, base);
	// 8328BB10: 3CC0820F  lis r6, -0x7df1
	ctx.r[6].s64 = -2112946176;
	// 8328BB14: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8328BB18: 38864B68  addi r4, r6, 0x4b68
	ctx.r[4].s64 = ctx.r[6].s64 + 19304;
	// 8328BB1C: 387F0B90  addi r3, r31, 0xb90
	ctx.r[3].s64 = ctx.r[31].s64 + 2960;
	// 8328BB20: 4AFA13B1  bl 0x8222ced0
	ctx.lr = 0x8328BB24;
	sub_8222CED0(ctx, base);
	// 8328BB24: 39600003  li r11, 3
	ctx.r[11].s64 = 3;
	// 8328BB28: 7F04C378  mr r4, r24
	ctx.r[4].u64 = ctx.r[24].u64;
	// 8328BB2C: 917F0B94  stw r11, 0xb94(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(2964 as u32), ctx.r[11].u32 ) };
	// 8328BB30: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8328BB34: 387F0B98  addi r3, r31, 0xb98
	ctx.r[3].s64 = ctx.r[31].s64 + 2968;
	// 8328BB38: 4AFA1399  bl 0x8222ced0
	ctx.lr = 0x8328BB3C;
	sub_8222CED0(ctx, base);
	// 8328BB3C: 7E248B78  mr r4, r17
	ctx.r[4].u64 = ctx.r[17].u64;
	// 8328BB40: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8328BB44: 387F0B9C  addi r3, r31, 0xb9c
	ctx.r[3].s64 = ctx.r[31].s64 + 2972;
	// 8328BB48: 4AFA1389  bl 0x8222ced0
	ctx.lr = 0x8328BB4C;
	sub_8222CED0(ctx, base);
	// 8328BB4C: 3C80820F  lis r4, -0x7df1
	ctx.r[4].s64 = -2112946176;
	// 8328BB50: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8328BB54: 38844B48  addi r4, r4, 0x4b48
	ctx.r[4].s64 = ctx.r[4].s64 + 19272;
	// 8328BB58: 387F0BA0  addi r3, r31, 0xba0
	ctx.r[3].s64 = ctx.r[31].s64 + 2976;
	// 8328BB5C: 4AFA1375  bl 0x8222ced0
	ctx.lr = 0x8328BB60;
	sub_8222CED0(ctx, base);
	// 8328BB60: 39600003  li r11, 3
	ctx.r[11].s64 = 3;
	// 8328BB64: 7F04C378  mr r4, r24
	ctx.r[4].u64 = ctx.r[24].u64;
	// 8328BB68: 917F0BA4  stw r11, 0xba4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(2980 as u32), ctx.r[11].u32 ) };
	// 8328BB6C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8328BB70: 387F0BA8  addi r3, r31, 0xba8
	ctx.r[3].s64 = ctx.r[31].s64 + 2984;
	// 8328BB74: 4AFA135D  bl 0x8222ced0
	ctx.lr = 0x8328BB78;
	sub_8222CED0(ctx, base);
	// 8328BB78: 7E048378  mr r4, r16
	ctx.r[4].u64 = ctx.r[16].u64;
	// 8328BB7C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8328BB80: 387F0BAC  addi r3, r31, 0xbac
	ctx.r[3].s64 = ctx.r[31].s64 + 2988;
	// 8328BB84: 4AFA134D  bl 0x8222ced0
	ctx.lr = 0x8328BB88;
	sub_8222CED0(ctx, base);
	// 8328BB88: 3C60820F  lis r3, -0x7df1
	ctx.r[3].s64 = -2112946176;
	// 8328BB8C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8328BB90: 38834B30  addi r4, r3, 0x4b30
	ctx.r[4].s64 = ctx.r[3].s64 + 19248;
	// 8328BB94: 387F0BB0  addi r3, r31, 0xbb0
	ctx.r[3].s64 = ctx.r[31].s64 + 2992;
	// 8328BB98: 4AFA1339  bl 0x8222ced0
	ctx.lr = 0x8328BB9C;
	sub_8222CED0(ctx, base);
	// 8328BB9C: 39600003  li r11, 3
	ctx.r[11].s64 = 3;
	// 8328BBA0: 7F04C378  mr r4, r24
	ctx.r[4].u64 = ctx.r[24].u64;
	// 8328BBA4: 917F0BB4  stw r11, 0xbb4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(2996 as u32), ctx.r[11].u32 ) };
	// 8328BBA8: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8328BBAC: 387F0BB8  addi r3, r31, 0xbb8
	ctx.r[3].s64 = ctx.r[31].s64 + 3000;
	// 8328BBB0: 4AFA1321  bl 0x8222ced0
	ctx.lr = 0x8328BBB4;
	sub_8222CED0(ctx, base);
	// 8328BBB4: 7DE47B78  mr r4, r15
	ctx.r[4].u64 = ctx.r[15].u64;
	// 8328BBB8: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8328BBBC: 387F0BBC  addi r3, r31, 0xbbc
	ctx.r[3].s64 = ctx.r[31].s64 + 3004;
	// 8328BBC0: 4AFA1311  bl 0x8222ced0
	ctx.lr = 0x8328BBC4;
	sub_8222CED0(ctx, base);
	// 8328BBC4: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 8328BBC8: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8328BBCC: 388B4B18  addi r4, r11, 0x4b18
	ctx.r[4].s64 = ctx.r[11].s64 + 19224;
	// 8328BBD0: 387F0BC0  addi r3, r31, 0xbc0
	ctx.r[3].s64 = ctx.r[31].s64 + 3008;
	// 8328BBD4: 4AFA12FD  bl 0x8222ced0
	ctx.lr = 0x8328BBD8;
	sub_8222CED0(ctx, base);
	// 8328BBD8: 39600003  li r11, 3
	ctx.r[11].s64 = 3;
	// 8328BBDC: 7F04C378  mr r4, r24
	ctx.r[4].u64 = ctx.r[24].u64;
	// 8328BBE0: 917F0BC4  stw r11, 0xbc4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(3012 as u32), ctx.r[11].u32 ) };
	// 8328BBE4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8328BBE8: 387F0BC8  addi r3, r31, 0xbc8
	ctx.r[3].s64 = ctx.r[31].s64 + 3016;
	// 8328BBEC: 4AFA12E5  bl 0x8222ced0
	ctx.lr = 0x8328BBF0;
	sub_8222CED0(ctx, base);
	// 8328BBF0: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8328BBF4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8328BBF8: 387F0BCC  addi r3, r31, 0xbcc
	ctx.r[3].s64 = ctx.r[31].s64 + 3020;
	// 8328BBFC: 4AFA12D5  bl 0x8222ced0
	ctx.lr = 0x8328BC00;
	sub_8222CED0(ctx, base);
	// 8328BC00: 3D40820F  lis r10, -0x7df1
	ctx.r[10].s64 = -2112946176;
	// 8328BC04: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8328BC08: 388A4B00  addi r4, r10, 0x4b00
	ctx.r[4].s64 = ctx.r[10].s64 + 19200;
	// 8328BC0C: 387F0BD0  addi r3, r31, 0xbd0
	ctx.r[3].s64 = ctx.r[31].s64 + 3024;
	// 8328BC10: 4AFA12C1  bl 0x8222ced0
	ctx.lr = 0x8328BC14;
	sub_8222CED0(ctx, base);
	// 8328BC14: 39600003  li r11, 3
	ctx.r[11].s64 = 3;
	// 8328BC18: 3D20820F  lis r9, -0x7df1
	ctx.r[9].s64 = -2112946176;
	// 8328BC1C: 917F0BD4  stw r11, 0xbd4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(3028 as u32), ctx.r[11].u32 ) };
	// 8328BC20: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8328BC24: 38894AF4  addi r4, r9, 0x4af4
	ctx.r[4].s64 = ctx.r[9].s64 + 19188;
	// 8328BC28: 387F0BD8  addi r3, r31, 0xbd8
	ctx.r[3].s64 = ctx.r[31].s64 + 3032;
	// 8328BC2C: 4AFA12A5  bl 0x8222ced0
	ctx.lr = 0x8328BC30;
	sub_8222CED0(ctx, base);
	// 8328BC30: 3D00820F  lis r8, -0x7df1
	ctx.r[8].s64 = -2112946176;
	// 8328BC34: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8328BC38: 3888C17C  addi r4, r8, -0x3e84
	ctx.r[4].s64 = ctx.r[8].s64 + -16004;
	// 8328BC3C: 387F0BDC  addi r3, r31, 0xbdc
	ctx.r[3].s64 = ctx.r[31].s64 + 3036;
	// 8328BC40: 4AFA1291  bl 0x8222ced0
	ctx.lr = 0x8328BC44;
	sub_8222CED0(ctx, base);
	// 8328BC44: 3CE0820F  lis r7, -0x7df1
	ctx.r[7].s64 = -2112946176;
	// 8328BC48: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8328BC4C: 38874AD0  addi r4, r7, 0x4ad0
	ctx.r[4].s64 = ctx.r[7].s64 + 19152;
	// 8328BC50: 387F0BE0  addi r3, r31, 0xbe0
	ctx.r[3].s64 = ctx.r[31].s64 + 3040;
	// 8328BC54: 4AFA127D  bl 0x8222ced0
	ctx.lr = 0x8328BC58;
	sub_8222CED0(ctx, base);
	// 8328BC58: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 8328BC5C: 3CC0820F  lis r6, -0x7df1
	ctx.r[6].s64 = -2112946176;
	// 8328BC60: 917F0BE4  stw r11, 0xbe4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(3044 as u32), ctx.r[11].u32 ) };
	// 8328BC64: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8328BC68: 3BC64AB8  addi r30, r6, 0x4ab8
	ctx.r[30].s64 = ctx.r[6].s64 + 19128;
	// 8328BC6C: 387F0BE8  addi r3, r31, 0xbe8
	ctx.r[3].s64 = ctx.r[31].s64 + 3048;
	// 8328BC70: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8328BC74: 4AFA125D  bl 0x8222ced0
	ctx.lr = 0x8328BC78;
	sub_8222CED0(ctx, base);
	// 8328BC78: 3C80820F  lis r4, -0x7df1
	ctx.r[4].s64 = -2112946176;
	// 8328BC7C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8328BC80: 38844AA8  addi r4, r4, 0x4aa8
	ctx.r[4].s64 = ctx.r[4].s64 + 19112;
	// 8328BC84: 387F0BEC  addi r3, r31, 0xbec
	ctx.r[3].s64 = ctx.r[31].s64 + 3052;
	// 8328BC88: 4AFA1249  bl 0x8222ced0
	ctx.lr = 0x8328BC8C;
	sub_8222CED0(ctx, base);
	// 8328BC8C: 3C60820F  lis r3, -0x7df1
	ctx.r[3].s64 = -2112946176;
	// 8328BC90: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8328BC94: 38834A80  addi r4, r3, 0x4a80
	ctx.r[4].s64 = ctx.r[3].s64 + 19072;
	// 8328BC98: 387F0BF0  addi r3, r31, 0xbf0
	ctx.r[3].s64 = ctx.r[31].s64 + 3056;
	// 8328BC9C: 4AFA1235  bl 0x8222ced0
	ctx.lr = 0x8328BCA0;
	sub_8222CED0(ctx, base);
	// 8328BCA0: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 8328BCA4: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8328BCA8: 917F0BF4  stw r11, 0xbf4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(3060 as u32), ctx.r[11].u32 ) };
	// 8328BCAC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8328BCB0: 387F0BF8  addi r3, r31, 0xbf8
	ctx.r[3].s64 = ctx.r[31].s64 + 3064;
	// 8328BCB4: 4AFA121D  bl 0x8222ced0
	ctx.lr = 0x8328BCB8;
	sub_8222CED0(ctx, base);
	// 8328BCB8: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 8328BCBC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8328BCC0: 388B4A70  addi r4, r11, 0x4a70
	ctx.r[4].s64 = ctx.r[11].s64 + 19056;
	// 8328BCC4: 387F0BFC  addi r3, r31, 0xbfc
	ctx.r[3].s64 = ctx.r[31].s64 + 3068;
	// 8328BCC8: 4AFA1209  bl 0x8222ced0
	ctx.lr = 0x8328BCCC;
	sub_8222CED0(ctx, base);
	// 8328BCCC: 3D40820F  lis r10, -0x7df1
	ctx.r[10].s64 = -2112946176;
	// 8328BCD0: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8328BCD4: 388A4A34  addi r4, r10, 0x4a34
	ctx.r[4].s64 = ctx.r[10].s64 + 18996;
	// 8328BCD8: 387F0C00  addi r3, r31, 0xc00
	ctx.r[3].s64 = ctx.r[31].s64 + 3072;
	// 8328BCDC: 4AFA11F5  bl 0x8222ced0
	ctx.lr = 0x8328BCE0;
	sub_8222CED0(ctx, base);
	// 8328BCE0: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 8328BCE4: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8328BCE8: 917F0C04  stw r11, 0xc04(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(3076 as u32), ctx.r[11].u32 ) };
	// 8328BCEC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8328BCF0: 387F0C08  addi r3, r31, 0xc08
	ctx.r[3].s64 = ctx.r[31].s64 + 3080;
	// 8328BCF4: 4AFA11DD  bl 0x8222ced0
	ctx.lr = 0x8328BCF8;
	sub_8222CED0(ctx, base);
	// 8328BCF8: 3D20820F  lis r9, -0x7df1
	ctx.r[9].s64 = -2112946176;
	// 8328BCFC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8328BD00: 38894A1C  addi r4, r9, 0x4a1c
	ctx.r[4].s64 = ctx.r[9].s64 + 18972;
	// 8328BD04: 387F0C0C  addi r3, r31, 0xc0c
	ctx.r[3].s64 = ctx.r[31].s64 + 3084;
	// 8328BD08: 4AFA11C9  bl 0x8222ced0
	ctx.lr = 0x8328BD0C;
	sub_8222CED0(ctx, base);
	// 8328BD0C: 3D00820F  lis r8, -0x7df1
	ctx.r[8].s64 = -2112946176;
	// 8328BD10: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8328BD14: 388849DC  addi r4, r8, 0x49dc
	ctx.r[4].s64 = ctx.r[8].s64 + 18908;
	// 8328BD18: 387F0C10  addi r3, r31, 0xc10
	ctx.r[3].s64 = ctx.r[31].s64 + 3088;
	// 8328BD1C: 4AFA11B5  bl 0x8222ced0
	ctx.lr = 0x8328BD20;
	sub_8222CED0(ctx, base);
	// 8328BD20: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 8328BD24: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8328BD28: 917F0C14  stw r11, 0xc14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(3092 as u32), ctx.r[11].u32 ) };
	// 8328BD2C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8328BD30: 387F0C18  addi r3, r31, 0xc18
	ctx.r[3].s64 = ctx.r[31].s64 + 3096;
	// 8328BD34: 4AFA119D  bl 0x8222ced0
	ctx.lr = 0x8328BD38;
	sub_8222CED0(ctx, base);
	// 8328BD38: 3CE0820F  lis r7, -0x7df1
	ctx.r[7].s64 = -2112946176;
	// 8328BD3C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8328BD40: 388749C0  addi r4, r7, 0x49c0
	ctx.r[4].s64 = ctx.r[7].s64 + 18880;
	// 8328BD44: 387F0C1C  addi r3, r31, 0xc1c
	ctx.r[3].s64 = ctx.r[31].s64 + 3100;
	// 8328BD48: 4AFA1189  bl 0x8222ced0
	ctx.lr = 0x8328BD4C;
	sub_8222CED0(ctx, base);
	// 8328BD4C: 3CC0832B  lis r6, -0x7cd5
	ctx.r[6].s64 = -2094333952;
	// 8328BD50: 38662B70  addi r3, r6, 0x2b70
	ctx.r[3].s64 = ctx.r[6].s64 + 11120;
	// 8328BD54: 4BA1E1CD  bl 0x82ca9f20
	ctx.lr = 0x8328BD58;
	sub_82CA9F20(ctx, base);
	// 8328BD58: 382100F0  addi r1, r1, 0xf0
	ctx.r[1].s64 = ctx.r[1].s64 + 240;
	// 8328BD5C: 4BA1D6C4  b 0x82ca9420
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8328BD60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8328BD60 size=64
    let mut pc: u32 = 0x8328BD60;
    'dispatch: loop {
        match pc {
            0x8328BD60 => {
    //   block [0x8328BD60..0x8328BDA0)
	// 8328BD60: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8328BD64: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8328BD68: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8328BD6C: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 8328BD70: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8328BD74: 388B0CA0  addi r4, r11, 0xca0
	ctx.r[4].s64 = ctx.r[11].s64 + 3232;
	// 8328BD78: 386AF768  addi r3, r10, -0x898
	ctx.r[3].s64 = ctx.r[10].s64 + -2200;
	// 8328BD7C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8328BD80: 4AFA1151  bl 0x8222ced0
	ctx.lr = 0x8328BD84;
	sub_8222CED0(ctx, base);
	// 8328BD84: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8328BD88: 38692C30  addi r3, r9, 0x2c30
	ctx.r[3].s64 = ctx.r[9].s64 + 11312;
	// 8328BD8C: 4BA1E195  bl 0x82ca9f20
	ctx.lr = 0x8328BD90;
	sub_82CA9F20(ctx, base);
	// 8328BD90: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8328BD94: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8328BD98: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8328BD9C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8328BDA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8328BDA0 size=64
    let mut pc: u32 = 0x8328BDA0;
    'dispatch: loop {
        match pc {
            0x8328BDA0 => {
    //   block [0x8328BDA0..0x8328BDE0)
	// 8328BDA0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8328BDA4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8328BDA8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8328BDAC: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 8328BDB0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8328BDB4: 388B0CA0  addi r4, r11, 0xca0
	ctx.r[4].s64 = ctx.r[11].s64 + 3232;
	// 8328BDB8: 386AF76C  addi r3, r10, -0x894
	ctx.r[3].s64 = ctx.r[10].s64 + -2196;
	// 8328BDBC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8328BDC0: 4AFA1111  bl 0x8222ced0
	ctx.lr = 0x8328BDC4;
	sub_8222CED0(ctx, base);
	// 8328BDC4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8328BDC8: 38692C40  addi r3, r9, 0x2c40
	ctx.r[3].s64 = ctx.r[9].s64 + 11328;
	// 8328BDCC: 4BA1E155  bl 0x82ca9f20
	ctx.lr = 0x8328BDD0;
	sub_82CA9F20(ctx, base);
	// 8328BDD0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8328BDD4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8328BDD8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8328BDDC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8328BDE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8328BDE0 size=192
    let mut pc: u32 = 0x8328BDE0;
    'dispatch: loop {
        match pc {
            0x8328BDE0 => {
    //   block [0x8328BDE0..0x8328BE38)
	// 8328BDE0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8328BDE4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8328BDE8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8328BDEC: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8328BDF0: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 8328BDF4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8328BDF8: 388B0CA0  addi r4, r11, 0xca0
	ctx.r[4].s64 = ctx.r[11].s64 + 3232;
	// 8328BDFC: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 8328BE00: 4AFA10D1  bl 0x8222ced0
	ctx.lr = 0x8328BE04;
	sub_8222CED0(ctx, base);
	// 8328BE04: 38810054  addi r4, r1, 0x54
	ctx.r[4].s64 = ctx.r[1].s64 + 84;
	// 8328BE08: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8328BE0C: 4AF62D2D  bl 0x821eeb38
	ctx.lr = 0x8328BE10;
	sub_821EEB38(ctx, base);
	// 8328BE10: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8328BE14: 4B9779DD  bl 0x82c037f0
	ctx.lr = 0x8328BE18;
	sub_82C037F0(ctx, base);
	// 8328BE18: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8328BE1C: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 8328BE20: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8328BE24: 916AF770  stw r11, -0x890(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-2192 as u32), ctx.r[11].u32 ) };
	// 8328BE28: 4AF3A941  bl 0x821c6768
	ctx.lr = 0x8328BE2C;
	sub_821C6768(ctx, base);
	// 8328BE2C: 3D208349  lis r9, -0x7cb7
	ctx.r[9].s64 = -2092367872;
	// 8328BE30: 3BE97088  addi r31, r9, 0x7088
	ctx.r[31].s64 = ctx.r[9].s64 + 28808;
	// 8328BE34: 7FE6FB78  mr r6, r31
	ctx.r[6].u64 = ctx.r[31].u64;
	pc = 0x8328BE38; continue 'dispatch;
            }
            0x8328BE38 => {
    //   block [0x8328BE38..0x8328BE64)
	// 8328BE38: 7CE000A6  mfmsr r7
	ctx.r[7].u64 = ctx.msr;
	// 8328BE3C: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8328BE40: 7D003028  lwarx r8, 0, r6
	// lwarx
	let ea = ctx.r[6].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[8].u64 = ctx.reserved.u32 as u64;
	// 8328BE44: 3908FFFF  addi r8, r8, -1
	ctx.r[8].s64 = ctx.r[8].s64 + -1;
	// 8328BE48: 7D00312D  stwcx. r8, 0, r6
	// stwcx.
	let addr = ctx.r[6].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[8].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 8328BE4C: 7CE10164  mtmsrd r7, 1
	ctx.msr = (ctx.r[7].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8328BE50: 4082FFE8  bne 0x8328be38
	if !ctx.cr[0].eq {
	pc = 0x8328BE38; continue 'dispatch;
	}
	// 8328BE54: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8328BE58: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 8328BE5C: 90A10050  stw r5, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[5].u32 ) };
	// 8328BE60: 4AF3A909  bl 0x821c6768
	ctx.lr = 0x8328BE64;
	sub_821C6768(ctx, base);
	pc = 0x8328BE64; continue 'dispatch;
            }
            0x8328BE64 => {
    //   block [0x8328BE64..0x8328BEA0)
	// 8328BE64: 7C6000A6  mfmsr r3
	ctx.r[3].u64 = ctx.msr;
	// 8328BE68: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8328BE6C: 7C80F828  lwarx r4, 0, r31
	// lwarx
	let ea = ctx.r[31].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[4].u64 = ctx.reserved.u32 as u64;
	// 8328BE70: 3884FFFF  addi r4, r4, -1
	ctx.r[4].s64 = ctx.r[4].s64 + -1;
	// 8328BE74: 7C80F92D  stwcx. r4, 0, r31
	// stwcx.
	let addr = ctx.r[31].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[4].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 8328BE78: 7C610164  mtmsrd r3, 1
	ctx.msr = (ctx.r[3].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8328BE7C: 4082FFE8  bne 0x8328be64
	if !ctx.cr[0].eq {
	pc = 0x8328BE64; continue 'dispatch;
	}
	// 8328BE80: 3D60832B  lis r11, -0x7cd5
	ctx.r[11].s64 = -2094333952;
	// 8328BE84: 386B2C50  addi r3, r11, 0x2c50
	ctx.r[3].s64 = ctx.r[11].s64 + 11344;
	// 8328BE88: 4BA1E099  bl 0x82ca9f20
	ctx.lr = 0x8328BE8C;
	sub_82CA9F20(ctx, base);
	// 8328BE8C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8328BE90: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8328BE94: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8328BE98: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8328BE9C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8328BEA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8328BEA0 size=192
    let mut pc: u32 = 0x8328BEA0;
    'dispatch: loop {
        match pc {
            0x8328BEA0 => {
    //   block [0x8328BEA0..0x8328BEF8)
	// 8328BEA0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8328BEA4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8328BEA8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8328BEAC: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8328BEB0: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 8328BEB4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8328BEB8: 388B69EC  addi r4, r11, 0x69ec
	ctx.r[4].s64 = ctx.r[11].s64 + 27116;
	// 8328BEBC: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 8328BEC0: 4AFA1011  bl 0x8222ced0
	ctx.lr = 0x8328BEC4;
	sub_8222CED0(ctx, base);
	// 8328BEC4: 38810054  addi r4, r1, 0x54
	ctx.r[4].s64 = ctx.r[1].s64 + 84;
	// 8328BEC8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8328BECC: 4AF62C6D  bl 0x821eeb38
	ctx.lr = 0x8328BED0;
	sub_821EEB38(ctx, base);
	// 8328BED0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8328BED4: 4B97791D  bl 0x82c037f0
	ctx.lr = 0x8328BED8;
	sub_82C037F0(ctx, base);
	// 8328BED8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8328BEDC: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 8328BEE0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8328BEE4: 916AF774  stw r11, -0x88c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-2188 as u32), ctx.r[11].u32 ) };
	// 8328BEE8: 4AF3A881  bl 0x821c6768
	ctx.lr = 0x8328BEEC;
	sub_821C6768(ctx, base);
	// 8328BEEC: 3D208349  lis r9, -0x7cb7
	ctx.r[9].s64 = -2092367872;
	// 8328BEF0: 3BE97088  addi r31, r9, 0x7088
	ctx.r[31].s64 = ctx.r[9].s64 + 28808;
	// 8328BEF4: 7FE6FB78  mr r6, r31
	ctx.r[6].u64 = ctx.r[31].u64;
	pc = 0x8328BEF8; continue 'dispatch;
            }
            0x8328BEF8 => {
    //   block [0x8328BEF8..0x8328BF24)
	// 8328BEF8: 7CE000A6  mfmsr r7
	ctx.r[7].u64 = ctx.msr;
	// 8328BEFC: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8328BF00: 7D003028  lwarx r8, 0, r6
	// lwarx
	let ea = ctx.r[6].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[8].u64 = ctx.reserved.u32 as u64;
	// 8328BF04: 3908FFFF  addi r8, r8, -1
	ctx.r[8].s64 = ctx.r[8].s64 + -1;
	// 8328BF08: 7D00312D  stwcx. r8, 0, r6
	// stwcx.
	let addr = ctx.r[6].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[8].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 8328BF0C: 7CE10164  mtmsrd r7, 1
	ctx.msr = (ctx.r[7].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8328BF10: 4082FFE8  bne 0x8328bef8
	if !ctx.cr[0].eq {
	pc = 0x8328BEF8; continue 'dispatch;
	}
	// 8328BF14: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8328BF18: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 8328BF1C: 90A10050  stw r5, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[5].u32 ) };
	// 8328BF20: 4AF3A849  bl 0x821c6768
	ctx.lr = 0x8328BF24;
	sub_821C6768(ctx, base);
	pc = 0x8328BF24; continue 'dispatch;
            }
            0x8328BF24 => {
    //   block [0x8328BF24..0x8328BF60)
	// 8328BF24: 7C6000A6  mfmsr r3
	ctx.r[3].u64 = ctx.msr;
	// 8328BF28: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8328BF2C: 7C80F828  lwarx r4, 0, r31
	// lwarx
	let ea = ctx.r[31].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[4].u64 = ctx.reserved.u32 as u64;
	// 8328BF30: 3884FFFF  addi r4, r4, -1
	ctx.r[4].s64 = ctx.r[4].s64 + -1;
	// 8328BF34: 7C80F92D  stwcx. r4, 0, r31
	// stwcx.
	let addr = ctx.r[31].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[4].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 8328BF38: 7C610164  mtmsrd r3, 1
	ctx.msr = (ctx.r[3].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8328BF3C: 4082FFE8  bne 0x8328bf24
	if !ctx.cr[0].eq {
	pc = 0x8328BF24; continue 'dispatch;
	}
	// 8328BF40: 3D60832B  lis r11, -0x7cd5
	ctx.r[11].s64 = -2094333952;
	// 8328BF44: 386B2C58  addi r3, r11, 0x2c58
	ctx.r[3].s64 = ctx.r[11].s64 + 11352;
	// 8328BF48: 4BA1DFD9  bl 0x82ca9f20
	ctx.lr = 0x8328BF4C;
	sub_82CA9F20(ctx, base);
	// 8328BF4C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8328BF50: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8328BF54: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8328BF58: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8328BF5C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8328BF60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8328BF60 size=192
    let mut pc: u32 = 0x8328BF60;
    'dispatch: loop {
        match pc {
            0x8328BF60 => {
    //   block [0x8328BF60..0x8328BFB8)
	// 8328BF60: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8328BF64: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8328BF68: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8328BF6C: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8328BF70: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 8328BF74: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8328BF78: 388B6A14  addi r4, r11, 0x6a14
	ctx.r[4].s64 = ctx.r[11].s64 + 27156;
	// 8328BF7C: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 8328BF80: 4AFA0F51  bl 0x8222ced0
	ctx.lr = 0x8328BF84;
	sub_8222CED0(ctx, base);
	// 8328BF84: 38810054  addi r4, r1, 0x54
	ctx.r[4].s64 = ctx.r[1].s64 + 84;
	// 8328BF88: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8328BF8C: 4AF62BAD  bl 0x821eeb38
	ctx.lr = 0x8328BF90;
	sub_821EEB38(ctx, base);
	// 8328BF90: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8328BF94: 4B97785D  bl 0x82c037f0
	ctx.lr = 0x8328BF98;
	sub_82C037F0(ctx, base);
	// 8328BF98: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8328BF9C: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 8328BFA0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8328BFA4: 916AF778  stw r11, -0x888(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-2184 as u32), ctx.r[11].u32 ) };
	// 8328BFA8: 4AF3A7C1  bl 0x821c6768
	ctx.lr = 0x8328BFAC;
	sub_821C6768(ctx, base);
	// 8328BFAC: 3D208349  lis r9, -0x7cb7
	ctx.r[9].s64 = -2092367872;
	// 8328BFB0: 3BE97088  addi r31, r9, 0x7088
	ctx.r[31].s64 = ctx.r[9].s64 + 28808;
	// 8328BFB4: 7FE6FB78  mr r6, r31
	ctx.r[6].u64 = ctx.r[31].u64;
	pc = 0x8328BFB8; continue 'dispatch;
            }
            0x8328BFB8 => {
    //   block [0x8328BFB8..0x8328BFE4)
	// 8328BFB8: 7CE000A6  mfmsr r7
	ctx.r[7].u64 = ctx.msr;
	// 8328BFBC: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8328BFC0: 7D003028  lwarx r8, 0, r6
	// lwarx
	let ea = ctx.r[6].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[8].u64 = ctx.reserved.u32 as u64;
	// 8328BFC4: 3908FFFF  addi r8, r8, -1
	ctx.r[8].s64 = ctx.r[8].s64 + -1;
	// 8328BFC8: 7D00312D  stwcx. r8, 0, r6
	// stwcx.
	let addr = ctx.r[6].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[8].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 8328BFCC: 7CE10164  mtmsrd r7, 1
	ctx.msr = (ctx.r[7].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8328BFD0: 4082FFE8  bne 0x8328bfb8
	if !ctx.cr[0].eq {
	pc = 0x8328BFB8; continue 'dispatch;
	}
	// 8328BFD4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8328BFD8: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 8328BFDC: 90A10050  stw r5, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[5].u32 ) };
	// 8328BFE0: 4AF3A789  bl 0x821c6768
	ctx.lr = 0x8328BFE4;
	sub_821C6768(ctx, base);
	pc = 0x8328BFE4; continue 'dispatch;
            }
            0x8328BFE4 => {
    //   block [0x8328BFE4..0x8328C020)
	// 8328BFE4: 7C6000A6  mfmsr r3
	ctx.r[3].u64 = ctx.msr;
	// 8328BFE8: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8328BFEC: 7C80F828  lwarx r4, 0, r31
	// lwarx
	let ea = ctx.r[31].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[4].u64 = ctx.reserved.u32 as u64;
	// 8328BFF0: 3884FFFF  addi r4, r4, -1
	ctx.r[4].s64 = ctx.r[4].s64 + -1;
	// 8328BFF4: 7C80F92D  stwcx. r4, 0, r31
	// stwcx.
	let addr = ctx.r[31].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[4].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 8328BFF8: 7C610164  mtmsrd r3, 1
	ctx.msr = (ctx.r[3].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8328BFFC: 4082FFE8  bne 0x8328bfe4
	if !ctx.cr[0].eq {
	pc = 0x8328BFE4; continue 'dispatch;
	}
	// 8328C000: 3D60832B  lis r11, -0x7cd5
	ctx.r[11].s64 = -2094333952;
	// 8328C004: 386B2C60  addi r3, r11, 0x2c60
	ctx.r[3].s64 = ctx.r[11].s64 + 11360;
	// 8328C008: 4BA1DF19  bl 0x82ca9f20
	ctx.lr = 0x8328C00C;
	sub_82CA9F20(ctx, base);
	// 8328C00C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8328C010: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8328C014: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8328C018: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8328C01C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8328C020(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8328C020 size=192
    let mut pc: u32 = 0x8328C020;
    'dispatch: loop {
        match pc {
            0x8328C020 => {
    //   block [0x8328C020..0x8328C078)
	// 8328C020: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8328C024: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8328C028: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8328C02C: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8328C030: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 8328C034: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8328C038: 388B6A40  addi r4, r11, 0x6a40
	ctx.r[4].s64 = ctx.r[11].s64 + 27200;
	// 8328C03C: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 8328C040: 4AFA0E91  bl 0x8222ced0
	ctx.lr = 0x8328C044;
	sub_8222CED0(ctx, base);
	// 8328C044: 38810054  addi r4, r1, 0x54
	ctx.r[4].s64 = ctx.r[1].s64 + 84;
	// 8328C048: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8328C04C: 4AF62AED  bl 0x821eeb38
	ctx.lr = 0x8328C050;
	sub_821EEB38(ctx, base);
	// 8328C050: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8328C054: 4B97779D  bl 0x82c037f0
	ctx.lr = 0x8328C058;
	sub_82C037F0(ctx, base);
	// 8328C058: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8328C05C: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 8328C060: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8328C064: 916AF77C  stw r11, -0x884(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-2180 as u32), ctx.r[11].u32 ) };
	// 8328C068: 4AF3A701  bl 0x821c6768
	ctx.lr = 0x8328C06C;
	sub_821C6768(ctx, base);
	// 8328C06C: 3D208349  lis r9, -0x7cb7
	ctx.r[9].s64 = -2092367872;
	// 8328C070: 3BE97088  addi r31, r9, 0x7088
	ctx.r[31].s64 = ctx.r[9].s64 + 28808;
	// 8328C074: 7FE6FB78  mr r6, r31
	ctx.r[6].u64 = ctx.r[31].u64;
	pc = 0x8328C078; continue 'dispatch;
            }
            0x8328C078 => {
    //   block [0x8328C078..0x8328C0A4)
	// 8328C078: 7CE000A6  mfmsr r7
	ctx.r[7].u64 = ctx.msr;
	// 8328C07C: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8328C080: 7D003028  lwarx r8, 0, r6
	// lwarx
	let ea = ctx.r[6].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[8].u64 = ctx.reserved.u32 as u64;
	// 8328C084: 3908FFFF  addi r8, r8, -1
	ctx.r[8].s64 = ctx.r[8].s64 + -1;
	// 8328C088: 7D00312D  stwcx. r8, 0, r6
	// stwcx.
	let addr = ctx.r[6].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[8].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 8328C08C: 7CE10164  mtmsrd r7, 1
	ctx.msr = (ctx.r[7].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8328C090: 4082FFE8  bne 0x8328c078
	if !ctx.cr[0].eq {
	pc = 0x8328C078; continue 'dispatch;
	}
	// 8328C094: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8328C098: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 8328C09C: 90A10050  stw r5, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[5].u32 ) };
	// 8328C0A0: 4AF3A6C9  bl 0x821c6768
	ctx.lr = 0x8328C0A4;
	sub_821C6768(ctx, base);
	pc = 0x8328C0A4; continue 'dispatch;
            }
            0x8328C0A4 => {
    //   block [0x8328C0A4..0x8328C0E0)
	// 8328C0A4: 7C6000A6  mfmsr r3
	ctx.r[3].u64 = ctx.msr;
	// 8328C0A8: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8328C0AC: 7C80F828  lwarx r4, 0, r31
	// lwarx
	let ea = ctx.r[31].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[4].u64 = ctx.reserved.u32 as u64;
	// 8328C0B0: 3884FFFF  addi r4, r4, -1
	ctx.r[4].s64 = ctx.r[4].s64 + -1;
	// 8328C0B4: 7C80F92D  stwcx. r4, 0, r31
	// stwcx.
	let addr = ctx.r[31].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[4].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 8328C0B8: 7C610164  mtmsrd r3, 1
	ctx.msr = (ctx.r[3].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8328C0BC: 4082FFE8  bne 0x8328c0a4
	if !ctx.cr[0].eq {
	pc = 0x8328C0A4; continue 'dispatch;
	}
	// 8328C0C0: 3D60832B  lis r11, -0x7cd5
	ctx.r[11].s64 = -2094333952;
	// 8328C0C4: 386B2C68  addi r3, r11, 0x2c68
	ctx.r[3].s64 = ctx.r[11].s64 + 11368;
	// 8328C0C8: 4BA1DE59  bl 0x82ca9f20
	ctx.lr = 0x8328C0CC;
	sub_82CA9F20(ctx, base);
	// 8328C0CC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8328C0D0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8328C0D4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8328C0D8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8328C0DC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8328C0E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8328C0E0 size=64
    let mut pc: u32 = 0x8328C0E0;
    'dispatch: loop {
        match pc {
            0x8328C0E0 => {
    //   block [0x8328C0E0..0x8328C120)
	// 8328C0E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8328C0E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8328C0E8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8328C0EC: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 8328C0F0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8328C0F4: 388B0CA0  addi r4, r11, 0xca0
	ctx.r[4].s64 = ctx.r[11].s64 + 3232;
	// 8328C0F8: 386AF780  addi r3, r10, -0x880
	ctx.r[3].s64 = ctx.r[10].s64 + -2176;
	// 8328C0FC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8328C100: 4AFA0DD1  bl 0x8222ced0
	ctx.lr = 0x8328C104;
	sub_8222CED0(ctx, base);
	// 8328C104: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8328C108: 38692C70  addi r3, r9, 0x2c70
	ctx.r[3].s64 = ctx.r[9].s64 + 11376;
	// 8328C10C: 4BA1DE15  bl 0x82ca9f20
	ctx.lr = 0x8328C110;
	sub_82CA9F20(ctx, base);
	// 8328C110: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8328C114: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8328C118: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8328C11C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8328C120(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8328C120 size=64
    let mut pc: u32 = 0x8328C120;
    'dispatch: loop {
        match pc {
            0x8328C120 => {
    //   block [0x8328C120..0x8328C160)
	// 8328C120: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8328C124: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8328C128: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8328C12C: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 8328C130: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8328C134: 388B0CA0  addi r4, r11, 0xca0
	ctx.r[4].s64 = ctx.r[11].s64 + 3232;
	// 8328C138: 386AF784  addi r3, r10, -0x87c
	ctx.r[3].s64 = ctx.r[10].s64 + -2172;
	// 8328C13C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8328C140: 4AFA0D91  bl 0x8222ced0
	ctx.lr = 0x8328C144;
	sub_8222CED0(ctx, base);
	// 8328C144: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8328C148: 38692C80  addi r3, r9, 0x2c80
	ctx.r[3].s64 = ctx.r[9].s64 + 11392;
	// 8328C14C: 4BA1DDD5  bl 0x82ca9f20
	ctx.lr = 0x8328C150;
	sub_82CA9F20(ctx, base);
	// 8328C150: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8328C154: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8328C158: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8328C15C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8328C160(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8328C160 size=64
    let mut pc: u32 = 0x8328C160;
    'dispatch: loop {
        match pc {
            0x8328C160 => {
    //   block [0x8328C160..0x8328C1A0)
	// 8328C160: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8328C164: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8328C168: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8328C16C: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 8328C170: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8328C174: 388B0CA0  addi r4, r11, 0xca0
	ctx.r[4].s64 = ctx.r[11].s64 + 3232;
	// 8328C178: 386AF788  addi r3, r10, -0x878
	ctx.r[3].s64 = ctx.r[10].s64 + -2168;
	// 8328C17C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8328C180: 4AFA0D51  bl 0x8222ced0
	ctx.lr = 0x8328C184;
	sub_8222CED0(ctx, base);
	// 8328C184: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8328C188: 38692C90  addi r3, r9, 0x2c90
	ctx.r[3].s64 = ctx.r[9].s64 + 11408;
	// 8328C18C: 4BA1DD95  bl 0x82ca9f20
	ctx.lr = 0x8328C190;
	sub_82CA9F20(ctx, base);
	// 8328C190: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8328C194: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8328C198: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8328C19C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8328C1A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8328C1A0 size=64
    let mut pc: u32 = 0x8328C1A0;
    'dispatch: loop {
        match pc {
            0x8328C1A0 => {
    //   block [0x8328C1A0..0x8328C1E0)
	// 8328C1A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8328C1A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8328C1A8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8328C1AC: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 8328C1B0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8328C1B4: 388B0CA0  addi r4, r11, 0xca0
	ctx.r[4].s64 = ctx.r[11].s64 + 3232;
	// 8328C1B8: 386AF78C  addi r3, r10, -0x874
	ctx.r[3].s64 = ctx.r[10].s64 + -2164;
	// 8328C1BC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8328C1C0: 4AFA0D11  bl 0x8222ced0
	ctx.lr = 0x8328C1C4;
	sub_8222CED0(ctx, base);
	// 8328C1C4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8328C1C8: 38692CA0  addi r3, r9, 0x2ca0
	ctx.r[3].s64 = ctx.r[9].s64 + 11424;
	// 8328C1CC: 4BA1DD55  bl 0x82ca9f20
	ctx.lr = 0x8328C1D0;
	sub_82CA9F20(ctx, base);
	// 8328C1D0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8328C1D4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8328C1D8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8328C1DC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8328C1E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8328C1E0 size=64
    let mut pc: u32 = 0x8328C1E0;
    'dispatch: loop {
        match pc {
            0x8328C1E0 => {
    //   block [0x8328C1E0..0x8328C220)
	// 8328C1E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8328C1E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8328C1E8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8328C1EC: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 8328C1F0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8328C1F4: 388B0CA0  addi r4, r11, 0xca0
	ctx.r[4].s64 = ctx.r[11].s64 + 3232;
	// 8328C1F8: 386AF790  addi r3, r10, -0x870
	ctx.r[3].s64 = ctx.r[10].s64 + -2160;
	// 8328C1FC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8328C200: 4AFA0CD1  bl 0x8222ced0
	ctx.lr = 0x8328C204;
	sub_8222CED0(ctx, base);
	// 8328C204: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8328C208: 38692CB0  addi r3, r9, 0x2cb0
	ctx.r[3].s64 = ctx.r[9].s64 + 11440;
	// 8328C20C: 4BA1DD15  bl 0x82ca9f20
	ctx.lr = 0x8328C210;
	sub_82CA9F20(ctx, base);
	// 8328C210: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8328C214: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8328C218: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8328C21C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8328C220(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8328C220 size=64
    let mut pc: u32 = 0x8328C220;
    'dispatch: loop {
        match pc {
            0x8328C220 => {
    //   block [0x8328C220..0x8328C260)
	// 8328C220: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8328C224: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8328C228: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8328C22C: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 8328C230: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8328C234: 388B0CA0  addi r4, r11, 0xca0
	ctx.r[4].s64 = ctx.r[11].s64 + 3232;
	// 8328C238: 386AF794  addi r3, r10, -0x86c
	ctx.r[3].s64 = ctx.r[10].s64 + -2156;
	// 8328C23C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8328C240: 4AFA0C91  bl 0x8222ced0
	ctx.lr = 0x8328C244;
	sub_8222CED0(ctx, base);
	// 8328C244: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8328C248: 38692CC0  addi r3, r9, 0x2cc0
	ctx.r[3].s64 = ctx.r[9].s64 + 11456;
	// 8328C24C: 4BA1DCD5  bl 0x82ca9f20
	ctx.lr = 0x8328C250;
	sub_82CA9F20(ctx, base);
	// 8328C250: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8328C254: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8328C258: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8328C25C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8328C260(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8328C260 size=12
    let mut pc: u32 = 0x8328C260;
    'dispatch: loop {
        match pc {
            0x8328C260 => {
    //   block [0x8328C260..0x8328C26C)
	// 8328C260: 3D60832B  lis r11, -0x7cd5
	ctx.r[11].s64 = -2094333952;
	// 8328C264: 386B2CD0  addi r3, r11, 0x2cd0
	ctx.r[3].s64 = ctx.r[11].s64 + 11472;
	// 8328C268: 4BA1DCB8  b 0x82ca9f20
	sub_82CA9F20(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8328C270(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8328C270 size=12
    let mut pc: u32 = 0x8328C270;
    'dispatch: loop {
        match pc {
            0x8328C270 => {
    //   block [0x8328C270..0x8328C27C)
	// 8328C270: 3D60832B  lis r11, -0x7cd5
	ctx.r[11].s64 = -2094333952;
	// 8328C274: 386B2CE0  addi r3, r11, 0x2ce0
	ctx.r[3].s64 = ctx.r[11].s64 + 11488;
	// 8328C278: 4BA1DCA8  b 0x82ca9f20
	sub_82CA9F20(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8328C280(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8328C280 size=12
    let mut pc: u32 = 0x8328C280;
    'dispatch: loop {
        match pc {
            0x8328C280 => {
    //   block [0x8328C280..0x8328C28C)
	// 8328C280: 3D60832B  lis r11, -0x7cd5
	ctx.r[11].s64 = -2094333952;
	// 8328C284: 386B2D38  addi r3, r11, 0x2d38
	ctx.r[3].s64 = ctx.r[11].s64 + 11576;
	// 8328C288: 4BA1DC98  b 0x82ca9f20
	sub_82CA9F20(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8328C290(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8328C290 size=176
    let mut pc: u32 = 0x8328C290;
    'dispatch: loop {
        match pc {
            0x8328C290 => {
    //   block [0x8328C290..0x8328C29C)
	// 8328C290: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 8328C294: 390B7088  addi r8, r11, 0x7088
	ctx.r[8].s64 = ctx.r[11].s64 + 28808;
	// 8328C298: 7D074378  mr r7, r8
	ctx.r[7].u64 = ctx.r[8].u64;
	pc = 0x8328C29C; continue 'dispatch;
            }
            0x8328C29C => {
    //   block [0x8328C29C..0x8328C2CC)
	// 8328C29C: 7D2000A6  mfmsr r9
	ctx.r[9].u64 = ctx.msr;
	// 8328C2A0: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8328C2A4: 7D403828  lwarx r10, 0, r7
	// lwarx
	let ea = ctx.r[7].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 8328C2A8: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 8328C2AC: 7D40392D  stwcx. r10, 0, r7
	// stwcx.
	let addr = ctx.r[7].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[10].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 8328C2B0: 7D210164  mtmsrd r9, 1
	ctx.msr = (ctx.r[9].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8328C2B4: 4082FFE8  bne 0x8328c29c
	if !ctx.cr[0].eq {
	pc = 0x8328C29C; continue 'dispatch;
	}
	// 8328C2B8: 3CA0834A  lis r5, -0x7cb6
	ctx.r[5].s64 = -2092302336;
	// 8328C2BC: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8328C2C0: 3865F7C8  addi r3, r5, -0x838
	ctx.r[3].s64 = ctx.r[5].s64 + -2104;
	// 8328C2C4: 7D0A4378  mr r10, r8
	ctx.r[10].u64 = ctx.r[8].u64;
	// 8328C2C8: 91630004  stw r11, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	pc = 0x8328C2CC; continue 'dispatch;
            }
            0x8328C2CC => {
    //   block [0x8328C2CC..0x8328C2F4)
	// 8328C2CC: 7C8000A6  mfmsr r4
	ctx.r[4].u64 = ctx.msr;
	// 8328C2D0: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8328C2D4: 7CC05028  lwarx r6, 0, r10
	// lwarx
	let ea = ctx.r[10].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[6].u64 = ctx.reserved.u32 as u64;
	// 8328C2D8: 38C60001  addi r6, r6, 1
	ctx.r[6].s64 = ctx.r[6].s64 + 1;
	// 8328C2DC: 7CC0512D  stwcx. r6, 0, r10
	// stwcx.
	let addr = ctx.r[10].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[6].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 8328C2E0: 7C810164  mtmsrd r4, 1
	ctx.msr = (ctx.r[4].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8328C2E4: 4082FFE8  bne 0x8328c2cc
	if !ctx.cr[0].eq {
	pc = 0x8328C2CC; continue 'dispatch;
	}
	// 8328C2E8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8328C2EC: 7D064378  mr r6, r8
	ctx.r[6].u64 = ctx.r[8].u64;
	// 8328C2F0: 91630008  stw r11, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	pc = 0x8328C2F4; continue 'dispatch;
            }
            0x8328C2F4 => {
    //   block [0x8328C2F4..0x8328C318)
	// 8328C2F4: 7CE000A6  mfmsr r7
	ctx.r[7].u64 = ctx.msr;
	// 8328C2F8: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8328C2FC: 7D203028  lwarx r9, 0, r6
	// lwarx
	let ea = ctx.r[6].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[9].u64 = ctx.reserved.u32 as u64;
	// 8328C300: 39290001  addi r9, r9, 1
	ctx.r[9].s64 = ctx.r[9].s64 + 1;
	// 8328C304: 7D20312D  stwcx. r9, 0, r6
	// stwcx.
	let addr = ctx.r[6].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[9].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 8328C308: 7CE10164  mtmsrd r7, 1
	ctx.msr = (ctx.r[7].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8328C30C: 4082FFE8  bne 0x8328c2f4
	if !ctx.cr[0].eq {
	pc = 0x8328C2F4; continue 'dispatch;
	}
	// 8328C310: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8328C314: 9163000C  stw r11, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	pc = 0x8328C318; continue 'dispatch;
            }
            0x8328C318 => {
    //   block [0x8328C318..0x8328C340)
	// 8328C318: 7C8000A6  mfmsr r4
	ctx.r[4].u64 = ctx.msr;
	// 8328C31C: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8328C320: 7CA04028  lwarx r5, 0, r8
	// lwarx
	let ea = ctx.r[8].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[5].u64 = ctx.reserved.u32 as u64;
	// 8328C324: 38A50001  addi r5, r5, 1
	ctx.r[5].s64 = ctx.r[5].s64 + 1;
	// 8328C328: 7CA0412D  stwcx. r5, 0, r8
	// stwcx.
	let addr = ctx.r[8].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[5].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 8328C32C: 7C810164  mtmsrd r4, 1
	ctx.msr = (ctx.r[4].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8328C330: 4082FFE8  bne 0x8328c318
	if !ctx.cr[0].eq {
	pc = 0x8328C318; continue 'dispatch;
	}
	// 8328C334: 3C60832B  lis r3, -0x7cd5
	ctx.r[3].s64 = -2094333952;
	// 8328C338: 38632D48  addi r3, r3, 0x2d48
	ctx.r[3].s64 = ctx.r[3].s64 + 11592;
	// 8328C33C: 4BA1DBE4  b 0x82ca9f20
	sub_82CA9F20(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8328C340(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8328C340 size=12
    let mut pc: u32 = 0x8328C340;
    'dispatch: loop {
        match pc {
            0x8328C340 => {
    //   block [0x8328C340..0x8328C34C)
	// 8328C340: 3D60832B  lis r11, -0x7cd5
	ctx.r[11].s64 = -2094333952;
	// 8328C344: 386B2D98  addi r3, r11, 0x2d98
	ctx.r[3].s64 = ctx.r[11].s64 + 11672;
	// 8328C348: 4BA1DBD8  b 0x82ca9f20
	sub_82CA9F20(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8328C350(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8328C350 size=12
    let mut pc: u32 = 0x8328C350;
    'dispatch: loop {
        match pc {
            0x8328C350 => {
    //   block [0x8328C350..0x8328C35C)
	// 8328C350: 3D60832B  lis r11, -0x7cd5
	ctx.r[11].s64 = -2094333952;
	// 8328C354: 386B2DC0  addi r3, r11, 0x2dc0
	ctx.r[3].s64 = ctx.r[11].s64 + 11712;
	// 8328C358: 4BA1DBC8  b 0x82ca9f20
	sub_82CA9F20(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8328C360(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8328C360 size=12
    let mut pc: u32 = 0x8328C360;
    'dispatch: loop {
        match pc {
            0x8328C360 => {
    //   block [0x8328C360..0x8328C36C)
	// 8328C360: 3D60832B  lis r11, -0x7cd5
	ctx.r[11].s64 = -2094333952;
	// 8328C364: 386B2E18  addi r3, r11, 0x2e18
	ctx.r[3].s64 = ctx.r[11].s64 + 11800;
	// 8328C368: 4BA1DBB8  b 0x82ca9f20
	sub_82CA9F20(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8328C370(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8328C370 size=12
    let mut pc: u32 = 0x8328C370;
    'dispatch: loop {
        match pc {
            0x8328C370 => {
    //   block [0x8328C370..0x8328C37C)
	// 8328C370: 3D60832B  lis r11, -0x7cd5
	ctx.r[11].s64 = -2094333952;
	// 8328C374: 386B2E70  addi r3, r11, 0x2e70
	ctx.r[3].s64 = ctx.r[11].s64 + 11888;
	// 8328C378: 4BA1DBA8  b 0x82ca9f20
	sub_82CA9F20(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8328C380(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8328C380 size=12
    let mut pc: u32 = 0x8328C380;
    'dispatch: loop {
        match pc {
            0x8328C380 => {
    //   block [0x8328C380..0x8328C38C)
	// 8328C380: 3D60832B  lis r11, -0x7cd5
	ctx.r[11].s64 = -2094333952;
	// 8328C384: 386B2EC8  addi r3, r11, 0x2ec8
	ctx.r[3].s64 = ctx.r[11].s64 + 11976;
	// 8328C388: 4BA1DB98  b 0x82ca9f20
	sub_82CA9F20(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8328C390(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8328C390 size=12
    let mut pc: u32 = 0x8328C390;
    'dispatch: loop {
        match pc {
            0x8328C390 => {
    //   block [0x8328C390..0x8328C39C)
	// 8328C390: 3D60832B  lis r11, -0x7cd5
	ctx.r[11].s64 = -2094333952;
	// 8328C394: 386B2F20  addi r3, r11, 0x2f20
	ctx.r[3].s64 = ctx.r[11].s64 + 12064;
	// 8328C398: 4BA1DB88  b 0x82ca9f20
	sub_82CA9F20(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8328C3A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8328C3A0 size=12
    let mut pc: u32 = 0x8328C3A0;
    'dispatch: loop {
        match pc {
            0x8328C3A0 => {
    //   block [0x8328C3A0..0x8328C3AC)
	// 8328C3A0: 3D60832B  lis r11, -0x7cd5
	ctx.r[11].s64 = -2094333952;
	// 8328C3A4: 386B2F78  addi r3, r11, 0x2f78
	ctx.r[3].s64 = ctx.r[11].s64 + 12152;
	// 8328C3A8: 4BA1DB78  b 0x82ca9f20
	sub_82CA9F20(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8328C3B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8328C3B0 size=12
    let mut pc: u32 = 0x8328C3B0;
    'dispatch: loop {
        match pc {
            0x8328C3B0 => {
    //   block [0x8328C3B0..0x8328C3BC)
	// 8328C3B0: 3D60832B  lis r11, -0x7cd5
	ctx.r[11].s64 = -2094333952;
	// 8328C3B4: 386B2FD0  addi r3, r11, 0x2fd0
	ctx.r[3].s64 = ctx.r[11].s64 + 12240;
	// 8328C3B8: 4BA1DB68  b 0x82ca9f20
	sub_82CA9F20(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8328C3C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8328C3C0 size=12
    let mut pc: u32 = 0x8328C3C0;
    'dispatch: loop {
        match pc {
            0x8328C3C0 => {
    //   block [0x8328C3C0..0x8328C3CC)
	// 8328C3C0: 3D60832B  lis r11, -0x7cd5
	ctx.r[11].s64 = -2094333952;
	// 8328C3C4: 386B3028  addi r3, r11, 0x3028
	ctx.r[3].s64 = ctx.r[11].s64 + 12328;
	// 8328C3C8: 4BA1DB58  b 0x82ca9f20
	sub_82CA9F20(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8328C3D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8328C3D0 size=52
    let mut pc: u32 = 0x8328C3D0;
    'dispatch: loop {
        match pc {
            0x8328C3D0 => {
    //   block [0x8328C3D0..0x8328C404)
	// 8328C3D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8328C3D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8328C3D8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8328C3DC: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 8328C3E0: 386BF7FC  addi r3, r11, -0x804
	ctx.r[3].s64 = ctx.r[11].s64 + -2052;
	// 8328C3E4: 4802D8A1  bl 0x832b9c84
	ctx.lr = 0x8328C3E8;
	// extern call 0x832B9C84 → crate::xboxkrnl::RtlInitializeCriticalSection
	crate::xboxkrnl::RtlInitializeCriticalSection(ctx, base);
	// 8328C3E8: 3D40832B  lis r10, -0x7cd5
	ctx.r[10].s64 = -2094333952;
	// 8328C3EC: 386A30A0  addi r3, r10, 0x30a0
	ctx.r[3].s64 = ctx.r[10].s64 + 12448;
	// 8328C3F0: 4BA1DB31  bl 0x82ca9f20
	ctx.lr = 0x8328C3F4;
	sub_82CA9F20(ctx, base);
	// 8328C3F4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8328C3F8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8328C3FC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8328C400: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8328C408(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8328C408 size=12
    let mut pc: u32 = 0x8328C408;
    'dispatch: loop {
        match pc {
            0x8328C408 => {
    //   block [0x8328C408..0x8328C414)
	// 8328C408: 3D60832B  lis r11, -0x7cd5
	ctx.r[11].s64 = -2094333952;
	// 8328C40C: 386B30A8  addi r3, r11, 0x30a8
	ctx.r[3].s64 = ctx.r[11].s64 + 12456;
	// 8328C410: 4BA1DB10  b 0x82ca9f20
	sub_82CA9F20(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8328C418(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8328C418 size=12
    let mut pc: u32 = 0x8328C418;
    'dispatch: loop {
        match pc {
            0x8328C418 => {
    //   block [0x8328C418..0x8328C424)
	// 8328C418: 3D60832B  lis r11, -0x7cd5
	ctx.r[11].s64 = -2094333952;
	// 8328C41C: 386B30B8  addi r3, r11, 0x30b8
	ctx.r[3].s64 = ctx.r[11].s64 + 12472;
	// 8328C420: 4BA1DB00  b 0x82ca9f20
	sub_82CA9F20(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8328C428(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8328C428 size=12
    let mut pc: u32 = 0x8328C428;
    'dispatch: loop {
        match pc {
            0x8328C428 => {
    //   block [0x8328C428..0x8328C434)
	// 8328C428: 3D60832B  lis r11, -0x7cd5
	ctx.r[11].s64 = -2094333952;
	// 8328C42C: 386B30C8  addi r3, r11, 0x30c8
	ctx.r[3].s64 = ctx.r[11].s64 + 12488;
	// 8328C430: 4BA1DAF0  b 0x82ca9f20
	sub_82CA9F20(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8328C438(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8328C438 size=12
    let mut pc: u32 = 0x8328C438;
    'dispatch: loop {
        match pc {
            0x8328C438 => {
    //   block [0x8328C438..0x8328C444)
	// 8328C438: 3D60832B  lis r11, -0x7cd5
	ctx.r[11].s64 = -2094333952;
	// 8328C43C: 386B30D8  addi r3, r11, 0x30d8
	ctx.r[3].s64 = ctx.r[11].s64 + 12504;
	// 8328C440: 4BA1DAE0  b 0x82ca9f20
	sub_82CA9F20(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


