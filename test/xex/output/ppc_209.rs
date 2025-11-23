pub fn sub_83248FC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83248FC0 size=64
    let mut pc: u32 = 0x83248FC0;
    'dispatch: loop {
        match pc {
            0x83248FC0 => {
    //   block [0x83248FC0..0x83249000)
	// 83248FC0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83248FC4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83248FC8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83248FCC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83248FD0: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 83248FD4: 388B77EC  addi r4, r11, 0x77ec
	ctx.r[4].s64 = ctx.r[11].s64 + 30700;
	// 83248FD8: 386A7630  addi r3, r10, 0x7630
	ctx.r[3].s64 = ctx.r[10].s64 + 30256;
	// 83248FDC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83248FE0: 4AFE3EF1  bl 0x8222ced0
	ctx.lr = 0x83248FE4;
	sub_8222CED0(ctx, base);
	// 83248FE4: 3D20832A  lis r9, -0x7cd6
	ctx.r[9].s64 = -2094399488;
	// 83248FE8: 386979F0  addi r3, r9, 0x79f0
	ctx.r[3].s64 = ctx.r[9].s64 + 31216;
	// 83248FEC: 4BA60F35  bl 0x82ca9f20
	ctx.lr = 0x83248FF0;
	sub_82CA9F20(ctx, base);
	// 83248FF0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83248FF4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83248FF8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83248FFC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83249000(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83249000 size=64
    let mut pc: u32 = 0x83249000;
    'dispatch: loop {
        match pc {
            0x83249000 => {
    //   block [0x83249000..0x83249040)
	// 83249000: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83249004: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83249008: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324900C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83249010: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 83249014: 388B7F98  addi r4, r11, 0x7f98
	ctx.r[4].s64 = ctx.r[11].s64 + 32664;
	// 83249018: 386A7634  addi r3, r10, 0x7634
	ctx.r[3].s64 = ctx.r[10].s64 + 30260;
	// 8324901C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83249020: 4AFE3EB1  bl 0x8222ced0
	ctx.lr = 0x83249024;
	sub_8222CED0(ctx, base);
	// 83249024: 3D20832A  lis r9, -0x7cd6
	ctx.r[9].s64 = -2094399488;
	// 83249028: 38697A00  addi r3, r9, 0x7a00
	ctx.r[3].s64 = ctx.r[9].s64 + 31232;
	// 8324902C: 4BA60EF5  bl 0x82ca9f20
	ctx.lr = 0x83249030;
	sub_82CA9F20(ctx, base);
	// 83249030: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83249034: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83249038: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324903C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83249040(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83249040 size=64
    let mut pc: u32 = 0x83249040;
    'dispatch: loop {
        match pc {
            0x83249040 => {
    //   block [0x83249040..0x83249080)
	// 83249040: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83249044: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83249048: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324904C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83249050: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 83249054: 388B04E8  addi r4, r11, 0x4e8
	ctx.r[4].s64 = ctx.r[11].s64 + 1256;
	// 83249058: 386A7638  addi r3, r10, 0x7638
	ctx.r[3].s64 = ctx.r[10].s64 + 30264;
	// 8324905C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83249060: 4AFE3E71  bl 0x8222ced0
	ctx.lr = 0x83249064;
	sub_8222CED0(ctx, base);
	// 83249064: 3D20832A  lis r9, -0x7cd6
	ctx.r[9].s64 = -2094399488;
	// 83249068: 38697A10  addi r3, r9, 0x7a10
	ctx.r[3].s64 = ctx.r[9].s64 + 31248;
	// 8324906C: 4BA60EB5  bl 0x82ca9f20
	ctx.lr = 0x83249070;
	sub_82CA9F20(ctx, base);
	// 83249070: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83249074: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83249078: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324907C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83249080(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83249080 size=64
    let mut pc: u32 = 0x83249080;
    'dispatch: loop {
        match pc {
            0x83249080 => {
    //   block [0x83249080..0x832490C0)
	// 83249080: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83249084: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83249088: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324908C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83249090: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 83249094: 388B7FB8  addi r4, r11, 0x7fb8
	ctx.r[4].s64 = ctx.r[11].s64 + 32696;
	// 83249098: 386A763C  addi r3, r10, 0x763c
	ctx.r[3].s64 = ctx.r[10].s64 + 30268;
	// 8324909C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832490A0: 4AFE3E31  bl 0x8222ced0
	ctx.lr = 0x832490A4;
	sub_8222CED0(ctx, base);
	// 832490A4: 3D20832A  lis r9, -0x7cd6
	ctx.r[9].s64 = -2094399488;
	// 832490A8: 38697A20  addi r3, r9, 0x7a20
	ctx.r[3].s64 = ctx.r[9].s64 + 31264;
	// 832490AC: 4BA60E75  bl 0x82ca9f20
	ctx.lr = 0x832490B0;
	sub_82CA9F20(ctx, base);
	// 832490B0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832490B4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832490B8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832490BC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832490C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832490C0 size=64
    let mut pc: u32 = 0x832490C0;
    'dispatch: loop {
        match pc {
            0x832490C0 => {
    //   block [0x832490C0..0x83249100)
	// 832490C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832490C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832490C8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832490CC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 832490D0: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 832490D4: 388B7FE4  addi r4, r11, 0x7fe4
	ctx.r[4].s64 = ctx.r[11].s64 + 32740;
	// 832490D8: 386A7640  addi r3, r10, 0x7640
	ctx.r[3].s64 = ctx.r[10].s64 + 30272;
	// 832490DC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832490E0: 4AFE3DF1  bl 0x8222ced0
	ctx.lr = 0x832490E4;
	sub_8222CED0(ctx, base);
	// 832490E4: 3D20832A  lis r9, -0x7cd6
	ctx.r[9].s64 = -2094399488;
	// 832490E8: 38697A30  addi r3, r9, 0x7a30
	ctx.r[3].s64 = ctx.r[9].s64 + 31280;
	// 832490EC: 4BA60E35  bl 0x82ca9f20
	ctx.lr = 0x832490F0;
	sub_82CA9F20(ctx, base);
	// 832490F0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832490F4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832490F8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832490FC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83249100(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83249100 size=64
    let mut pc: u32 = 0x83249100;
    'dispatch: loop {
        match pc {
            0x83249100 => {
    //   block [0x83249100..0x83249140)
	// 83249100: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83249104: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83249108: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324910C: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 83249110: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 83249114: 388B800C  addi r4, r11, -0x7ff4
	ctx.r[4].s64 = ctx.r[11].s64 + -32756;
	// 83249118: 386A7644  addi r3, r10, 0x7644
	ctx.r[3].s64 = ctx.r[10].s64 + 30276;
	// 8324911C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83249120: 4AFE3DB1  bl 0x8222ced0
	ctx.lr = 0x83249124;
	sub_8222CED0(ctx, base);
	// 83249124: 3D20832A  lis r9, -0x7cd6
	ctx.r[9].s64 = -2094399488;
	// 83249128: 38697A40  addi r3, r9, 0x7a40
	ctx.r[3].s64 = ctx.r[9].s64 + 31296;
	// 8324912C: 4BA60DF5  bl 0x82ca9f20
	ctx.lr = 0x83249130;
	sub_82CA9F20(ctx, base);
	// 83249130: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83249134: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83249138: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324913C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83249140(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83249140 size=64
    let mut pc: u32 = 0x83249140;
    'dispatch: loop {
        match pc {
            0x83249140 => {
    //   block [0x83249140..0x83249180)
	// 83249140: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83249144: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83249148: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324914C: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 83249150: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 83249154: 388B8034  addi r4, r11, -0x7fcc
	ctx.r[4].s64 = ctx.r[11].s64 + -32716;
	// 83249158: 386A7648  addi r3, r10, 0x7648
	ctx.r[3].s64 = ctx.r[10].s64 + 30280;
	// 8324915C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83249160: 4AFE3D71  bl 0x8222ced0
	ctx.lr = 0x83249164;
	sub_8222CED0(ctx, base);
	// 83249164: 3D20832A  lis r9, -0x7cd6
	ctx.r[9].s64 = -2094399488;
	// 83249168: 38697A50  addi r3, r9, 0x7a50
	ctx.r[3].s64 = ctx.r[9].s64 + 31312;
	// 8324916C: 4BA60DB5  bl 0x82ca9f20
	ctx.lr = 0x83249170;
	sub_82CA9F20(ctx, base);
	// 83249170: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83249174: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83249178: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324917C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83249180(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83249180 size=64
    let mut pc: u32 = 0x83249180;
    'dispatch: loop {
        match pc {
            0x83249180 => {
    //   block [0x83249180..0x832491C0)
	// 83249180: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83249184: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83249188: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324918C: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 83249190: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 83249194: 388B8064  addi r4, r11, -0x7f9c
	ctx.r[4].s64 = ctx.r[11].s64 + -32668;
	// 83249198: 386A764C  addi r3, r10, 0x764c
	ctx.r[3].s64 = ctx.r[10].s64 + 30284;
	// 8324919C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832491A0: 4AFE3D31  bl 0x8222ced0
	ctx.lr = 0x832491A4;
	sub_8222CED0(ctx, base);
	// 832491A4: 3D20832A  lis r9, -0x7cd6
	ctx.r[9].s64 = -2094399488;
	// 832491A8: 38697A60  addi r3, r9, 0x7a60
	ctx.r[3].s64 = ctx.r[9].s64 + 31328;
	// 832491AC: 4BA60D75  bl 0x82ca9f20
	ctx.lr = 0x832491B0;
	sub_82CA9F20(ctx, base);
	// 832491B0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832491B4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832491B8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832491BC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832491C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832491C0 size=64
    let mut pc: u32 = 0x832491C0;
    'dispatch: loop {
        match pc {
            0x832491C0 => {
    //   block [0x832491C0..0x83249200)
	// 832491C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832491C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832491C8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832491CC: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 832491D0: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 832491D4: 388B8090  addi r4, r11, -0x7f70
	ctx.r[4].s64 = ctx.r[11].s64 + -32624;
	// 832491D8: 386A7650  addi r3, r10, 0x7650
	ctx.r[3].s64 = ctx.r[10].s64 + 30288;
	// 832491DC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832491E0: 4AFE3CF1  bl 0x8222ced0
	ctx.lr = 0x832491E4;
	sub_8222CED0(ctx, base);
	// 832491E4: 3D20832A  lis r9, -0x7cd6
	ctx.r[9].s64 = -2094399488;
	// 832491E8: 38697A70  addi r3, r9, 0x7a70
	ctx.r[3].s64 = ctx.r[9].s64 + 31344;
	// 832491EC: 4BA60D35  bl 0x82ca9f20
	ctx.lr = 0x832491F0;
	sub_82CA9F20(ctx, base);
	// 832491F0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832491F4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832491F8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832491FC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83249200(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83249200 size=64
    let mut pc: u32 = 0x83249200;
    'dispatch: loop {
        match pc {
            0x83249200 => {
    //   block [0x83249200..0x83249240)
	// 83249200: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83249204: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83249208: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324920C: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 83249210: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 83249214: 388B80BC  addi r4, r11, -0x7f44
	ctx.r[4].s64 = ctx.r[11].s64 + -32580;
	// 83249218: 386A7654  addi r3, r10, 0x7654
	ctx.r[3].s64 = ctx.r[10].s64 + 30292;
	// 8324921C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83249220: 4AFE3CB1  bl 0x8222ced0
	ctx.lr = 0x83249224;
	sub_8222CED0(ctx, base);
	// 83249224: 3D20832A  lis r9, -0x7cd6
	ctx.r[9].s64 = -2094399488;
	// 83249228: 38697A80  addi r3, r9, 0x7a80
	ctx.r[3].s64 = ctx.r[9].s64 + 31360;
	// 8324922C: 4BA60CF5  bl 0x82ca9f20
	ctx.lr = 0x83249230;
	sub_82CA9F20(ctx, base);
	// 83249230: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83249234: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83249238: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324923C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83249240(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83249240 size=64
    let mut pc: u32 = 0x83249240;
    'dispatch: loop {
        match pc {
            0x83249240 => {
    //   block [0x83249240..0x83249280)
	// 83249240: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83249244: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83249248: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324924C: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 83249250: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 83249254: 388B80E8  addi r4, r11, -0x7f18
	ctx.r[4].s64 = ctx.r[11].s64 + -32536;
	// 83249258: 386A7658  addi r3, r10, 0x7658
	ctx.r[3].s64 = ctx.r[10].s64 + 30296;
	// 8324925C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83249260: 4AFE3C71  bl 0x8222ced0
	ctx.lr = 0x83249264;
	sub_8222CED0(ctx, base);
	// 83249264: 3D20832A  lis r9, -0x7cd6
	ctx.r[9].s64 = -2094399488;
	// 83249268: 38697A90  addi r3, r9, 0x7a90
	ctx.r[3].s64 = ctx.r[9].s64 + 31376;
	// 8324926C: 4BA60CB5  bl 0x82ca9f20
	ctx.lr = 0x83249270;
	sub_82CA9F20(ctx, base);
	// 83249270: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83249274: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83249278: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324927C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83249280(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83249280 size=64
    let mut pc: u32 = 0x83249280;
    'dispatch: loop {
        match pc {
            0x83249280 => {
    //   block [0x83249280..0x832492C0)
	// 83249280: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83249284: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83249288: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324928C: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 83249290: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 83249294: 388B8114  addi r4, r11, -0x7eec
	ctx.r[4].s64 = ctx.r[11].s64 + -32492;
	// 83249298: 386A765C  addi r3, r10, 0x765c
	ctx.r[3].s64 = ctx.r[10].s64 + 30300;
	// 8324929C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832492A0: 4AFE3C31  bl 0x8222ced0
	ctx.lr = 0x832492A4;
	sub_8222CED0(ctx, base);
	// 832492A4: 3D20832A  lis r9, -0x7cd6
	ctx.r[9].s64 = -2094399488;
	// 832492A8: 38697AA0  addi r3, r9, 0x7aa0
	ctx.r[3].s64 = ctx.r[9].s64 + 31392;
	// 832492AC: 4BA60C75  bl 0x82ca9f20
	ctx.lr = 0x832492B0;
	sub_82CA9F20(ctx, base);
	// 832492B0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832492B4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832492B8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832492BC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832492C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832492C0 size=64
    let mut pc: u32 = 0x832492C0;
    'dispatch: loop {
        match pc {
            0x832492C0 => {
    //   block [0x832492C0..0x83249300)
	// 832492C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832492C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832492C8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832492CC: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 832492D0: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 832492D4: 388B8140  addi r4, r11, -0x7ec0
	ctx.r[4].s64 = ctx.r[11].s64 + -32448;
	// 832492D8: 386A7660  addi r3, r10, 0x7660
	ctx.r[3].s64 = ctx.r[10].s64 + 30304;
	// 832492DC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832492E0: 4AFE3BF1  bl 0x8222ced0
	ctx.lr = 0x832492E4;
	sub_8222CED0(ctx, base);
	// 832492E4: 3D20832A  lis r9, -0x7cd6
	ctx.r[9].s64 = -2094399488;
	// 832492E8: 38697AB0  addi r3, r9, 0x7ab0
	ctx.r[3].s64 = ctx.r[9].s64 + 31408;
	// 832492EC: 4BA60C35  bl 0x82ca9f20
	ctx.lr = 0x832492F0;
	sub_82CA9F20(ctx, base);
	// 832492F0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832492F4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832492F8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832492FC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83249300(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83249300 size=64
    let mut pc: u32 = 0x83249300;
    'dispatch: loop {
        match pc {
            0x83249300 => {
    //   block [0x83249300..0x83249340)
	// 83249300: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83249304: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83249308: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324930C: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 83249310: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 83249314: 388B816C  addi r4, r11, -0x7e94
	ctx.r[4].s64 = ctx.r[11].s64 + -32404;
	// 83249318: 386A7664  addi r3, r10, 0x7664
	ctx.r[3].s64 = ctx.r[10].s64 + 30308;
	// 8324931C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83249320: 4AFE3BB1  bl 0x8222ced0
	ctx.lr = 0x83249324;
	sub_8222CED0(ctx, base);
	// 83249324: 3D20832A  lis r9, -0x7cd6
	ctx.r[9].s64 = -2094399488;
	// 83249328: 38697AC0  addi r3, r9, 0x7ac0
	ctx.r[3].s64 = ctx.r[9].s64 + 31424;
	// 8324932C: 4BA60BF5  bl 0x82ca9f20
	ctx.lr = 0x83249330;
	sub_82CA9F20(ctx, base);
	// 83249330: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83249334: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83249338: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324933C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83249340(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x83249340 size=88
    let mut pc: u32 = 0x83249340;
    'dispatch: loop {
        match pc {
            0x83249340 => {
    //   block [0x83249340..0x83249398)
	// 83249340: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83249344: 3941FFF0  addi r10, r1, -0x10
	ctx.r[10].s64 = ctx.r[1].s64 + -16;
	// 83249348: 3921FFF4  addi r9, r1, -0xc
	ctx.r[9].s64 = ctx.r[1].s64 + -12;
	// 8324934C: 3901FFF4  addi r8, r1, -0xc
	ctx.r[8].s64 = ctx.r[1].s64 + -12;
	// 83249350: 38E1FFF0  addi r7, r1, -0x10
	ctx.r[7].s64 = ctx.r[1].s64 + -16;
	// 83249354: C00B9490  lfs f0, -0x6b70(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-27504 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 83249358: 3CC08349  lis r6, -0x7cb7
	ctx.r[6].s64 = -2092367872;
	// 8324935C: D001FFF0  stfs f0, -0x10(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), tmp.u32 ) };
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83249398(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x83249398 size=96
    let mut pc: u32 = 0x83249398;
    'dispatch: loop {
        match pc {
            0x83249398 => {
    //   block [0x83249398..0x832493F8)
	// 83249398: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8324939C: 3901FFF0  addi r8, r1, -0x10
	ctx.r[8].s64 = ctx.r[1].s64 + -16;
	// 832493A0: 392B9484  addi r9, r11, -0x6b7c
	ctx.r[9].s64 = ctx.r[11].s64 + -27516;
	// 832493A4: 3941FFF4  addi r10, r1, -0xc
	ctx.r[10].s64 = ctx.r[1].s64 + -12;
	// 832493A8: 38E1FFF4  addi r7, r1, -0xc
	ctx.r[7].s64 = ctx.r[1].s64 + -12;
	// 832493AC: C00B9484  lfs f0, -0x6b7c(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-27516 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 832493B0: 38C1FFF0  addi r6, r1, -0x10
	ctx.r[6].s64 = ctx.r[1].s64 + -16;
	// 832493B4: D001FFF0  stfs f0, -0x10(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), tmp.u32 ) };
	// 832493B8: 3CA08349  lis r5, -0x7cb7
	ctx.r[5].s64 = -2092367872;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832493F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832493F8 size=64
    let mut pc: u32 = 0x832493F8;
    'dispatch: loop {
        match pc {
            0x832493F8 => {
    //   block [0x832493F8..0x83249438)
	// 832493F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832493FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83249400: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83249404: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 83249408: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324940C: 388B819C  addi r4, r11, -0x7e64
	ctx.r[4].s64 = ctx.r[11].s64 + -32356;
	// 83249410: 386A7668  addi r3, r10, 0x7668
	ctx.r[3].s64 = ctx.r[10].s64 + 30312;
	// 83249414: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83249418: 4AFE3AB9  bl 0x8222ced0
	ctx.lr = 0x8324941C;
	sub_8222CED0(ctx, base);
	// 8324941C: 3D20832A  lis r9, -0x7cd6
	ctx.r[9].s64 = -2094399488;
	// 83249420: 38697AD0  addi r3, r9, 0x7ad0
	ctx.r[3].s64 = ctx.r[9].s64 + 31440;
	// 83249424: 4BA60AFD  bl 0x82ca9f20
	ctx.lr = 0x83249428;
	sub_82CA9F20(ctx, base);
	// 83249428: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324942C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83249430: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83249434: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83249438(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83249438 size=64
    let mut pc: u32 = 0x83249438;
    'dispatch: loop {
        match pc {
            0x83249438 => {
    //   block [0x83249438..0x83249478)
	// 83249438: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324943C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83249440: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83249444: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 83249448: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324944C: 388B81CC  addi r4, r11, -0x7e34
	ctx.r[4].s64 = ctx.r[11].s64 + -32308;
	// 83249450: 386A766C  addi r3, r10, 0x766c
	ctx.r[3].s64 = ctx.r[10].s64 + 30316;
	// 83249454: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83249458: 4AFE3A79  bl 0x8222ced0
	ctx.lr = 0x8324945C;
	sub_8222CED0(ctx, base);
	// 8324945C: 3D20832A  lis r9, -0x7cd6
	ctx.r[9].s64 = -2094399488;
	// 83249460: 38697AE0  addi r3, r9, 0x7ae0
	ctx.r[3].s64 = ctx.r[9].s64 + 31456;
	// 83249464: 4BA60ABD  bl 0x82ca9f20
	ctx.lr = 0x83249468;
	sub_82CA9F20(ctx, base);
	// 83249468: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324946C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83249470: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83249474: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83249478(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83249478 size=192
    let mut pc: u32 = 0x83249478;
    'dispatch: loop {
        match pc {
            0x83249478 => {
    //   block [0x83249478..0x832494D0)
	// 83249478: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324947C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83249480: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83249484: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83249488: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 8324948C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83249490: 388B81F0  addi r4, r11, -0x7e10
	ctx.r[4].s64 = ctx.r[11].s64 + -32272;
	// 83249494: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 83249498: 4AFE3A39  bl 0x8222ced0
	ctx.lr = 0x8324949C;
	sub_8222CED0(ctx, base);
	// 8324949C: 38810054  addi r4, r1, 0x54
	ctx.r[4].s64 = ctx.r[1].s64 + 84;
	// 832494A0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 832494A4: 4AFA5695  bl 0x821eeb38
	ctx.lr = 0x832494A8;
	sub_821EEB38(ctx, base);
	// 832494A8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 832494AC: 4B9BA345  bl 0x82c037f0
	ctx.lr = 0x832494B0;
	sub_82C037F0(ctx, base);
	// 832494B0: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 832494B4: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 832494B8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 832494BC: 916A7690  stw r11, 0x7690(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(30352 as u32), ctx.r[11].u32 ) };
	// 832494C0: 4AF7D2A9  bl 0x821c6768
	ctx.lr = 0x832494C4;
	sub_821C6768(ctx, base);
	// 832494C4: 3D208349  lis r9, -0x7cb7
	ctx.r[9].s64 = -2092367872;
	// 832494C8: 3BE97088  addi r31, r9, 0x7088
	ctx.r[31].s64 = ctx.r[9].s64 + 28808;
	// 832494CC: 7FE6FB78  mr r6, r31
	ctx.r[6].u64 = ctx.r[31].u64;
	pc = 0x832494D0; continue 'dispatch;
            }
            0x832494D0 => {
    //   block [0x832494D0..0x832494FC)
	// 832494D0: 7CE000A6  mfmsr r7
	ctx.r[7].u64 = ctx.msr;
	// 832494D4: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 832494D8: 7D003028  lwarx r8, 0, r6
	// lwarx
	let ea = ctx.r[6].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[8].u64 = ctx.reserved.u32 as u64;
	// 832494DC: 3908FFFF  addi r8, r8, -1
	ctx.r[8].s64 = ctx.r[8].s64 + -1;
	// 832494E0: 7D00312D  stwcx. r8, 0, r6
	// stwcx.
	let addr = ctx.r[6].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[8].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 832494E4: 7CE10164  mtmsrd r7, 1
	ctx.msr = (ctx.r[7].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 832494E8: 4082FFE8  bne 0x832494d0
	if !ctx.cr[0].eq {
	pc = 0x832494D0; continue 'dispatch;
	}
	// 832494EC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 832494F0: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 832494F4: 90A10050  stw r5, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[5].u32 ) };
	// 832494F8: 4AF7D271  bl 0x821c6768
	ctx.lr = 0x832494FC;
	sub_821C6768(ctx, base);
	pc = 0x832494FC; continue 'dispatch;
            }
            0x832494FC => {
    //   block [0x832494FC..0x83249538)
	// 832494FC: 7C6000A6  mfmsr r3
	ctx.r[3].u64 = ctx.msr;
	// 83249500: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 83249504: 7C80F828  lwarx r4, 0, r31
	// lwarx
	let ea = ctx.r[31].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[4].u64 = ctx.reserved.u32 as u64;
	// 83249508: 3884FFFF  addi r4, r4, -1
	ctx.r[4].s64 = ctx.r[4].s64 + -1;
	// 8324950C: 7C80F92D  stwcx. r4, 0, r31
	// stwcx.
	let addr = ctx.r[31].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[4].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 83249510: 7C610164  mtmsrd r3, 1
	ctx.msr = (ctx.r[3].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 83249514: 4082FFE8  bne 0x832494fc
	if !ctx.cr[0].eq {
	pc = 0x832494FC; continue 'dispatch;
	}
	// 83249518: 3D60832A  lis r11, -0x7cd6
	ctx.r[11].s64 = -2094399488;
	// 8324951C: 386B7AF0  addi r3, r11, 0x7af0
	ctx.r[3].s64 = ctx.r[11].s64 + 31472;
	// 83249520: 4BA60A01  bl 0x82ca9f20
	ctx.lr = 0x83249524;
	sub_82CA9F20(ctx, base);
	// 83249524: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83249528: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324952C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83249530: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83249534: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83249538(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83249538 size=64
    let mut pc: u32 = 0x83249538;
    'dispatch: loop {
        match pc {
            0x83249538 => {
    //   block [0x83249538..0x83249578)
	// 83249538: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324953C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83249540: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83249544: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83249548: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324954C: 388B4888  addi r4, r11, 0x4888
	ctx.r[4].s64 = ctx.r[11].s64 + 18568;
	// 83249550: 386A7694  addi r3, r10, 0x7694
	ctx.r[3].s64 = ctx.r[10].s64 + 30356;
	// 83249554: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83249558: 4AFE3979  bl 0x8222ced0
	ctx.lr = 0x8324955C;
	sub_8222CED0(ctx, base);
	// 8324955C: 3D20832A  lis r9, -0x7cd6
	ctx.r[9].s64 = -2094399488;
	// 83249560: 38697AF8  addi r3, r9, 0x7af8
	ctx.r[3].s64 = ctx.r[9].s64 + 31480;
	// 83249564: 4BA609BD  bl 0x82ca9f20
	ctx.lr = 0x83249568;
	sub_82CA9F20(ctx, base);
	// 83249568: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324956C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83249570: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83249574: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83249578(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83249578 size=64
    let mut pc: u32 = 0x83249578;
    'dispatch: loop {
        match pc {
            0x83249578 => {
    //   block [0x83249578..0x832495B8)
	// 83249578: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324957C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83249580: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83249584: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 83249588: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324958C: 388B824C  addi r4, r11, -0x7db4
	ctx.r[4].s64 = ctx.r[11].s64 + -32180;
	// 83249590: 386A7698  addi r3, r10, 0x7698
	ctx.r[3].s64 = ctx.r[10].s64 + 30360;
	// 83249594: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83249598: 4AFE3939  bl 0x8222ced0
	ctx.lr = 0x8324959C;
	sub_8222CED0(ctx, base);
	// 8324959C: 3D20832A  lis r9, -0x7cd6
	ctx.r[9].s64 = -2094399488;
	// 832495A0: 38697B08  addi r3, r9, 0x7b08
	ctx.r[3].s64 = ctx.r[9].s64 + 31496;
	// 832495A4: 4BA6097D  bl 0x82ca9f20
	ctx.lr = 0x832495A8;
	sub_82CA9F20(ctx, base);
	// 832495A8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832495AC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832495B0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832495B4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832495B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832495B8 size=204
    let mut pc: u32 = 0x832495B8;
    'dispatch: loop {
        match pc {
            0x832495B8 => {
    //   block [0x832495B8..0x83249614)
	// 832495B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832495BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832495C0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 832495C4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 832495C8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832495CC: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 832495D0: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832495D4: 388B827C  addi r4, r11, -0x7d84
	ctx.r[4].s64 = ctx.r[11].s64 + -32132;
	// 832495D8: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 832495DC: 4AFE38F5  bl 0x8222ced0
	ctx.lr = 0x832495E0;
	sub_8222CED0(ctx, base);
	// 832495E0: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 832495E4: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 832495E8: 4AFA5551  bl 0x821eeb38
	ctx.lr = 0x832495EC;
	sub_821EEB38(ctx, base);
	// 832495EC: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 832495F0: 4B9BA201  bl 0x82c037f0
	ctx.lr = 0x832495F4;
	sub_82C037F0(ctx, base);
	// 832495F4: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 832495F8: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 832495FC: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 83249600: 916A769C  stw r11, 0x769c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(30364 as u32), ctx.r[11].u32 ) };
	// 83249604: 4AF7D165  bl 0x821c6768
	ctx.lr = 0x83249608;
	sub_821C6768(ctx, base);
	// 83249608: 3D208349  lis r9, -0x7cb7
	ctx.r[9].s64 = -2092367872;
	// 8324960C: 3BC97088  addi r30, r9, 0x7088
	ctx.r[30].s64 = ctx.r[9].s64 + 28808;
	// 83249610: 7FC6F378  mr r6, r30
	ctx.r[6].u64 = ctx.r[30].u64;
	pc = 0x83249614; continue 'dispatch;
            }
            0x83249614 => {
    //   block [0x83249614..0x83249640)
	// 83249614: 7CE000A6  mfmsr r7
	ctx.r[7].u64 = ctx.msr;
	// 83249618: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8324961C: 7D003028  lwarx r8, 0, r6
	// lwarx
	let ea = ctx.r[6].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[8].u64 = ctx.reserved.u32 as u64;
	// 83249620: 3908FFFF  addi r8, r8, -1
	ctx.r[8].s64 = ctx.r[8].s64 + -1;
	// 83249624: 7D00312D  stwcx. r8, 0, r6
	// stwcx.
	let addr = ctx.r[6].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[8].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 83249628: 7CE10164  mtmsrd r7, 1
	ctx.msr = (ctx.r[7].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8324962C: 4082FFE8  bne 0x83249614
	if !ctx.cr[0].eq {
	pc = 0x83249614; continue 'dispatch;
	}
	// 83249630: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 83249634: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 83249638: 93E10050  stw r31, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[31].u32 ) };
	// 8324963C: 4AF7D12D  bl 0x821c6768
	ctx.lr = 0x83249640;
	sub_821C6768(ctx, base);
	pc = 0x83249640; continue 'dispatch;
            }
            0x83249640 => {
    //   block [0x83249640..0x83249684)
	// 83249640: 7C8000A6  mfmsr r4
	ctx.r[4].u64 = ctx.msr;
	// 83249644: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 83249648: 7CA0F028  lwarx r5, 0, r30
	// lwarx
	let ea = ctx.r[30].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[5].u64 = ctx.reserved.u32 as u64;
	// 8324964C: 38A5FFFF  addi r5, r5, -1
	ctx.r[5].s64 = ctx.r[5].s64 + -1;
	// 83249650: 7CA0F12D  stwcx. r5, 0, r30
	// stwcx.
	let addr = ctx.r[30].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[5].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 83249654: 7C810164  mtmsrd r4, 1
	ctx.msr = (ctx.r[4].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 83249658: 4082FFE8  bne 0x83249640
	if !ctx.cr[0].eq {
	pc = 0x83249640; continue 'dispatch;
	}
	// 8324965C: 93E10054  stw r31, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[31].u32 ) };
	// 83249660: 3C60832A  lis r3, -0x7cd6
	ctx.r[3].s64 = -2094399488;
	// 83249664: 38637B18  addi r3, r3, 0x7b18
	ctx.r[3].s64 = ctx.r[3].s64 + 31512;
	// 83249668: 4BA608B9  bl 0x82ca9f20
	ctx.lr = 0x8324966C;
	sub_82CA9F20(ctx, base);
	// 8324966C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83249670: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83249674: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83249678: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8324967C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83249680: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83249688(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83249688 size=64
    let mut pc: u32 = 0x83249688;
    'dispatch: loop {
        match pc {
            0x83249688 => {
    //   block [0x83249688..0x832496C8)
	// 83249688: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324968C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83249690: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83249694: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83249698: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324969C: 388B1AD4  addi r4, r11, 0x1ad4
	ctx.r[4].s64 = ctx.r[11].s64 + 6868;
	// 832496A0: 386A76A0  addi r3, r10, 0x76a0
	ctx.r[3].s64 = ctx.r[10].s64 + 30368;
	// 832496A4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832496A8: 4AFE3829  bl 0x8222ced0
	ctx.lr = 0x832496AC;
	sub_8222CED0(ctx, base);
	// 832496AC: 3D20832A  lis r9, -0x7cd6
	ctx.r[9].s64 = -2094399488;
	// 832496B0: 38697B20  addi r3, r9, 0x7b20
	ctx.r[3].s64 = ctx.r[9].s64 + 31520;
	// 832496B4: 4BA6086D  bl 0x82ca9f20
	ctx.lr = 0x832496B8;
	sub_82CA9F20(ctx, base);
	// 832496B8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832496BC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832496C0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832496C4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832496C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832496C8 size=64
    let mut pc: u32 = 0x832496C8;
    'dispatch: loop {
        match pc {
            0x832496C8 => {
    //   block [0x832496C8..0x83249708)
	// 832496C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832496CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832496D0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832496D4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 832496D8: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 832496DC: 388B1ADC  addi r4, r11, 0x1adc
	ctx.r[4].s64 = ctx.r[11].s64 + 6876;
	// 832496E0: 386A76A4  addi r3, r10, 0x76a4
	ctx.r[3].s64 = ctx.r[10].s64 + 30372;
	// 832496E4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832496E8: 4AFE37E9  bl 0x8222ced0
	ctx.lr = 0x832496EC;
	sub_8222CED0(ctx, base);
	// 832496EC: 3D20832A  lis r9, -0x7cd6
	ctx.r[9].s64 = -2094399488;
	// 832496F0: 38697B30  addi r3, r9, 0x7b30
	ctx.r[3].s64 = ctx.r[9].s64 + 31536;
	// 832496F4: 4BA6082D  bl 0x82ca9f20
	ctx.lr = 0x832496F8;
	sub_82CA9F20(ctx, base);
	// 832496F8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832496FC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83249700: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83249704: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83249708(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83249708 size=64
    let mut pc: u32 = 0x83249708;
    'dispatch: loop {
        match pc {
            0x83249708 => {
    //   block [0x83249708..0x83249748)
	// 83249708: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324970C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83249710: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83249714: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83249718: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324971C: 388B1AF0  addi r4, r11, 0x1af0
	ctx.r[4].s64 = ctx.r[11].s64 + 6896;
	// 83249720: 386A76A8  addi r3, r10, 0x76a8
	ctx.r[3].s64 = ctx.r[10].s64 + 30376;
	// 83249724: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83249728: 4AFE37A9  bl 0x8222ced0
	ctx.lr = 0x8324972C;
	sub_8222CED0(ctx, base);
	// 8324972C: 3D20832A  lis r9, -0x7cd6
	ctx.r[9].s64 = -2094399488;
	// 83249730: 38697B40  addi r3, r9, 0x7b40
	ctx.r[3].s64 = ctx.r[9].s64 + 31552;
	// 83249734: 4BA607ED  bl 0x82ca9f20
	ctx.lr = 0x83249738;
	sub_82CA9F20(ctx, base);
	// 83249738: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324973C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83249740: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83249744: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83249748(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83249748 size=12
    let mut pc: u32 = 0x83249748;
    'dispatch: loop {
        match pc {
            0x83249748 => {
    //   block [0x83249748..0x83249754)
	// 83249748: 3D60832A  lis r11, -0x7cd6
	ctx.r[11].s64 = -2094399488;
	// 8324974C: 386B7B50  addi r3, r11, 0x7b50
	ctx.r[3].s64 = ctx.r[11].s64 + 31568;
	// 83249750: 4BA607D0  b 0x82ca9f20
	sub_82CA9F20(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83249758(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83249758 size=64
    let mut pc: u32 = 0x83249758;
    'dispatch: loop {
        match pc {
            0x83249758 => {
    //   block [0x83249758..0x83249798)
	// 83249758: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324975C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83249760: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83249764: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83249768: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324976C: 388B052C  addi r4, r11, 0x52c
	ctx.r[4].s64 = ctx.r[11].s64 + 1324;
	// 83249770: 386A76BC  addi r3, r10, 0x76bc
	ctx.r[3].s64 = ctx.r[10].s64 + 30396;
	// 83249774: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83249778: 4AFE3759  bl 0x8222ced0
	ctx.lr = 0x8324977C;
	sub_8222CED0(ctx, base);
	// 8324977C: 3D20832A  lis r9, -0x7cd6
	ctx.r[9].s64 = -2094399488;
	// 83249780: 38697BB8  addi r3, r9, 0x7bb8
	ctx.r[3].s64 = ctx.r[9].s64 + 31672;
	// 83249784: 4BA6079D  bl 0x82ca9f20
	ctx.lr = 0x83249788;
	sub_82CA9F20(ctx, base);
	// 83249788: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324978C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83249790: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83249794: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83249798(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83249798 size=376
    let mut pc: u32 = 0x83249798;
    'dispatch: loop {
        match pc {
            0x83249798 => {
    //   block [0x83249798..0x83249910)
	// 83249798: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324979C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832497A0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 832497A4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832497A8: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 832497AC: 3D40820B  lis r10, -0x7df5
	ctx.r[10].s64 = -2113208320;
	// 832497B0: 3BEB76C0  addi r31, r11, 0x76c0
	ctx.r[31].s64 = ctx.r[11].s64 + 30400;
	// 832497B4: 388A8A50  addi r4, r10, -0x75b0
	ctx.r[4].s64 = ctx.r[10].s64 + -30128;
	// 832497B8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 832497BC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832497C0: 4AFE3711  bl 0x8222ced0
	ctx.lr = 0x832497C4;
	sub_8222CED0(ctx, base);
	// 832497C4: 3D20820B  lis r9, -0x7df5
	ctx.r[9].s64 = -2113208320;
	// 832497C8: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832497CC: 38898A30  addi r4, r9, -0x75d0
	ctx.r[4].s64 = ctx.r[9].s64 + -30160;
	// 832497D0: 387F0004  addi r3, r31, 4
	ctx.r[3].s64 = ctx.r[31].s64 + 4;
	// 832497D4: 4AFE36FD  bl 0x8222ced0
	ctx.lr = 0x832497D8;
	sub_8222CED0(ctx, base);
	// 832497D8: 3D00820B  lis r8, -0x7df5
	ctx.r[8].s64 = -2113208320;
	// 832497DC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832497E0: 38888A10  addi r4, r8, -0x75f0
	ctx.r[4].s64 = ctx.r[8].s64 + -30192;
	// 832497E4: 387F0008  addi r3, r31, 8
	ctx.r[3].s64 = ctx.r[31].s64 + 8;
	// 832497E8: 4AFE36E9  bl 0x8222ced0
	ctx.lr = 0x832497EC;
	sub_8222CED0(ctx, base);
	// 832497EC: 3CE0820B  lis r7, -0x7df5
	ctx.r[7].s64 = -2113208320;
	// 832497F0: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832497F4: 388789F0  addi r4, r7, -0x7610
	ctx.r[4].s64 = ctx.r[7].s64 + -30224;
	// 832497F8: 387F000C  addi r3, r31, 0xc
	ctx.r[3].s64 = ctx.r[31].s64 + 12;
	// 832497FC: 4AFE36D5  bl 0x8222ced0
	ctx.lr = 0x83249800;
	sub_8222CED0(ctx, base);
	// 83249800: 3CC0820B  lis r6, -0x7df5
	ctx.r[6].s64 = -2113208320;
	// 83249804: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83249808: 388689D0  addi r4, r6, -0x7630
	ctx.r[4].s64 = ctx.r[6].s64 + -30256;
	// 8324980C: 387F0010  addi r3, r31, 0x10
	ctx.r[3].s64 = ctx.r[31].s64 + 16;
	// 83249810: 4AFE36C1  bl 0x8222ced0
	ctx.lr = 0x83249814;
	sub_8222CED0(ctx, base);
	// 83249814: 3C80820B  lis r4, -0x7df5
	ctx.r[4].s64 = -2113208320;
	// 83249818: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324981C: 388489B0  addi r4, r4, -0x7650
	ctx.r[4].s64 = ctx.r[4].s64 + -30288;
	// 83249820: 387F0014  addi r3, r31, 0x14
	ctx.r[3].s64 = ctx.r[31].s64 + 20;
	// 83249824: 4AFE36AD  bl 0x8222ced0
	ctx.lr = 0x83249828;
	sub_8222CED0(ctx, base);
	// 83249828: 3C60820B  lis r3, -0x7df5
	ctx.r[3].s64 = -2113208320;
	// 8324982C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83249830: 38838990  addi r4, r3, -0x7670
	ctx.r[4].s64 = ctx.r[3].s64 + -30320;
	// 83249834: 387F0018  addi r3, r31, 0x18
	ctx.r[3].s64 = ctx.r[31].s64 + 24;
	// 83249838: 4AFE3699  bl 0x8222ced0
	ctx.lr = 0x8324983C;
	sub_8222CED0(ctx, base);
	// 8324983C: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 83249840: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83249844: 388B8970  addi r4, r11, -0x7690
	ctx.r[4].s64 = ctx.r[11].s64 + -30352;
	// 83249848: 387F001C  addi r3, r31, 0x1c
	ctx.r[3].s64 = ctx.r[31].s64 + 28;
	// 8324984C: 4AFE3685  bl 0x8222ced0
	ctx.lr = 0x83249850;
	sub_8222CED0(ctx, base);
	// 83249850: 3D40820B  lis r10, -0x7df5
	ctx.r[10].s64 = -2113208320;
	// 83249854: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83249858: 388A8950  addi r4, r10, -0x76b0
	ctx.r[4].s64 = ctx.r[10].s64 + -30384;
	// 8324985C: 387F0020  addi r3, r31, 0x20
	ctx.r[3].s64 = ctx.r[31].s64 + 32;
	// 83249860: 4AFE3671  bl 0x8222ced0
	ctx.lr = 0x83249864;
	sub_8222CED0(ctx, base);
	// 83249864: 3D20820B  lis r9, -0x7df5
	ctx.r[9].s64 = -2113208320;
	// 83249868: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324986C: 38898930  addi r4, r9, -0x76d0
	ctx.r[4].s64 = ctx.r[9].s64 + -30416;
	// 83249870: 387F0024  addi r3, r31, 0x24
	ctx.r[3].s64 = ctx.r[31].s64 + 36;
	// 83249874: 4AFE365D  bl 0x8222ced0
	ctx.lr = 0x83249878;
	sub_8222CED0(ctx, base);
	// 83249878: 3D00820B  lis r8, -0x7df5
	ctx.r[8].s64 = -2113208320;
	// 8324987C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83249880: 38888914  addi r4, r8, -0x76ec
	ctx.r[4].s64 = ctx.r[8].s64 + -30444;
	// 83249884: 387F0028  addi r3, r31, 0x28
	ctx.r[3].s64 = ctx.r[31].s64 + 40;
	// 83249888: 4AFE3649  bl 0x8222ced0
	ctx.lr = 0x8324988C;
	sub_8222CED0(ctx, base);
	// 8324988C: 3CE0820B  lis r7, -0x7df5
	ctx.r[7].s64 = -2113208320;
	// 83249890: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83249894: 388788F0  addi r4, r7, -0x7710
	ctx.r[4].s64 = ctx.r[7].s64 + -30480;
	// 83249898: 387F002C  addi r3, r31, 0x2c
	ctx.r[3].s64 = ctx.r[31].s64 + 44;
	// 8324989C: 4AFE3635  bl 0x8222ced0
	ctx.lr = 0x832498A0;
	sub_8222CED0(ctx, base);
	// 832498A0: 3CC0820B  lis r6, -0x7df5
	ctx.r[6].s64 = -2113208320;
	// 832498A4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832498A8: 388688CC  addi r4, r6, -0x7734
	ctx.r[4].s64 = ctx.r[6].s64 + -30516;
	// 832498AC: 387F0030  addi r3, r31, 0x30
	ctx.r[3].s64 = ctx.r[31].s64 + 48;
	// 832498B0: 4AFE3621  bl 0x8222ced0
	ctx.lr = 0x832498B4;
	sub_8222CED0(ctx, base);
	// 832498B4: 3C80820B  lis r4, -0x7df5
	ctx.r[4].s64 = -2113208320;
	// 832498B8: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832498BC: 388488AC  addi r4, r4, -0x7754
	ctx.r[4].s64 = ctx.r[4].s64 + -30548;
	// 832498C0: 387F0034  addi r3, r31, 0x34
	ctx.r[3].s64 = ctx.r[31].s64 + 52;
	// 832498C4: 4AFE360D  bl 0x8222ced0
	ctx.lr = 0x832498C8;
	sub_8222CED0(ctx, base);
	// 832498C8: 3C60820B  lis r3, -0x7df5
	ctx.r[3].s64 = -2113208320;
	// 832498CC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832498D0: 3883888C  addi r4, r3, -0x7774
	ctx.r[4].s64 = ctx.r[3].s64 + -30580;
	// 832498D4: 387F0038  addi r3, r31, 0x38
	ctx.r[3].s64 = ctx.r[31].s64 + 56;
	// 832498D8: 4AFE35F9  bl 0x8222ced0
	ctx.lr = 0x832498DC;
	sub_8222CED0(ctx, base);
	// 832498DC: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 832498E0: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832498E4: 388B8868  addi r4, r11, -0x7798
	ctx.r[4].s64 = ctx.r[11].s64 + -30616;
	// 832498E8: 387F003C  addi r3, r31, 0x3c
	ctx.r[3].s64 = ctx.r[31].s64 + 60;
	// 832498EC: 4AFE35E5  bl 0x8222ced0
	ctx.lr = 0x832498F0;
	sub_8222CED0(ctx, base);
	// 832498F0: 3D40832A  lis r10, -0x7cd6
	ctx.r[10].s64 = -2094399488;
	// 832498F4: 386A7BC8  addi r3, r10, 0x7bc8
	ctx.r[3].s64 = ctx.r[10].s64 + 31688;
	// 832498F8: 4BA60629  bl 0x82ca9f20
	ctx.lr = 0x832498FC;
	sub_82CA9F20(ctx, base);
	// 832498FC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83249900: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83249904: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83249908: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8324990C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83249910(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83249910 size=376
    let mut pc: u32 = 0x83249910;
    'dispatch: loop {
        match pc {
            0x83249910 => {
    //   block [0x83249910..0x83249A88)
	// 83249910: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83249914: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83249918: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8324991C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83249920: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 83249924: 3D40820B  lis r10, -0x7df5
	ctx.r[10].s64 = -2113208320;
	// 83249928: 3BEB7700  addi r31, r11, 0x7700
	ctx.r[31].s64 = ctx.r[11].s64 + 30464;
	// 8324992C: 388A8C58  addi r4, r10, -0x73a8
	ctx.r[4].s64 = ctx.r[10].s64 + -29608;
	// 83249930: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83249934: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83249938: 4AFE3599  bl 0x8222ced0
	ctx.lr = 0x8324993C;
	sub_8222CED0(ctx, base);
	// 8324993C: 3D20820B  lis r9, -0x7df5
	ctx.r[9].s64 = -2113208320;
	// 83249940: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83249944: 38898C38  addi r4, r9, -0x73c8
	ctx.r[4].s64 = ctx.r[9].s64 + -29640;
	// 83249948: 387F0004  addi r3, r31, 4
	ctx.r[3].s64 = ctx.r[31].s64 + 4;
	// 8324994C: 4AFE3585  bl 0x8222ced0
	ctx.lr = 0x83249950;
	sub_8222CED0(ctx, base);
	// 83249950: 3D00820B  lis r8, -0x7df5
	ctx.r[8].s64 = -2113208320;
	// 83249954: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83249958: 38888C18  addi r4, r8, -0x73e8
	ctx.r[4].s64 = ctx.r[8].s64 + -29672;
	// 8324995C: 387F0008  addi r3, r31, 8
	ctx.r[3].s64 = ctx.r[31].s64 + 8;
	// 83249960: 4AFE3571  bl 0x8222ced0
	ctx.lr = 0x83249964;
	sub_8222CED0(ctx, base);
	// 83249964: 3CE0820B  lis r7, -0x7df5
	ctx.r[7].s64 = -2113208320;
	// 83249968: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324996C: 38878BF8  addi r4, r7, -0x7408
	ctx.r[4].s64 = ctx.r[7].s64 + -29704;
	// 83249970: 387F000C  addi r3, r31, 0xc
	ctx.r[3].s64 = ctx.r[31].s64 + 12;
	// 83249974: 4AFE355D  bl 0x8222ced0
	ctx.lr = 0x83249978;
	sub_8222CED0(ctx, base);
	// 83249978: 3CC0820B  lis r6, -0x7df5
	ctx.r[6].s64 = -2113208320;
	// 8324997C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83249980: 38868BD8  addi r4, r6, -0x7428
	ctx.r[4].s64 = ctx.r[6].s64 + -29736;
	// 83249984: 387F0010  addi r3, r31, 0x10
	ctx.r[3].s64 = ctx.r[31].s64 + 16;
	// 83249988: 4AFE3549  bl 0x8222ced0
	ctx.lr = 0x8324998C;
	sub_8222CED0(ctx, base);
	// 8324998C: 3C80820B  lis r4, -0x7df5
	ctx.r[4].s64 = -2113208320;
	// 83249990: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83249994: 38848BB8  addi r4, r4, -0x7448
	ctx.r[4].s64 = ctx.r[4].s64 + -29768;
	// 83249998: 387F0014  addi r3, r31, 0x14
	ctx.r[3].s64 = ctx.r[31].s64 + 20;
	// 8324999C: 4AFE3535  bl 0x8222ced0
	ctx.lr = 0x832499A0;
	sub_8222CED0(ctx, base);
	// 832499A0: 3C60820B  lis r3, -0x7df5
	ctx.r[3].s64 = -2113208320;
	// 832499A4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832499A8: 38838B98  addi r4, r3, -0x7468
	ctx.r[4].s64 = ctx.r[3].s64 + -29800;
	// 832499AC: 387F0018  addi r3, r31, 0x18
	ctx.r[3].s64 = ctx.r[31].s64 + 24;
	// 832499B0: 4AFE3521  bl 0x8222ced0
	ctx.lr = 0x832499B4;
	sub_8222CED0(ctx, base);
	// 832499B4: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 832499B8: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832499BC: 388B8B78  addi r4, r11, -0x7488
	ctx.r[4].s64 = ctx.r[11].s64 + -29832;
	// 832499C0: 387F001C  addi r3, r31, 0x1c
	ctx.r[3].s64 = ctx.r[31].s64 + 28;
	// 832499C4: 4AFE350D  bl 0x8222ced0
	ctx.lr = 0x832499C8;
	sub_8222CED0(ctx, base);
	// 832499C8: 3D40820B  lis r10, -0x7df5
	ctx.r[10].s64 = -2113208320;
	// 832499CC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832499D0: 388A8B58  addi r4, r10, -0x74a8
	ctx.r[4].s64 = ctx.r[10].s64 + -29864;
	// 832499D4: 387F0020  addi r3, r31, 0x20
	ctx.r[3].s64 = ctx.r[31].s64 + 32;
	// 832499D8: 4AFE34F9  bl 0x8222ced0
	ctx.lr = 0x832499DC;
	sub_8222CED0(ctx, base);
	// 832499DC: 3D20820B  lis r9, -0x7df5
	ctx.r[9].s64 = -2113208320;
	// 832499E0: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832499E4: 38898B38  addi r4, r9, -0x74c8
	ctx.r[4].s64 = ctx.r[9].s64 + -29896;
	// 832499E8: 387F0024  addi r3, r31, 0x24
	ctx.r[3].s64 = ctx.r[31].s64 + 36;
	// 832499EC: 4AFE34E5  bl 0x8222ced0
	ctx.lr = 0x832499F0;
	sub_8222CED0(ctx, base);
	// 832499F0: 3D00820B  lis r8, -0x7df5
	ctx.r[8].s64 = -2113208320;
	// 832499F4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832499F8: 38888B1C  addi r4, r8, -0x74e4
	ctx.r[4].s64 = ctx.r[8].s64 + -29924;
	// 832499FC: 387F0028  addi r3, r31, 0x28
	ctx.r[3].s64 = ctx.r[31].s64 + 40;
	// 83249A00: 4AFE34D1  bl 0x8222ced0
	ctx.lr = 0x83249A04;
	sub_8222CED0(ctx, base);
	// 83249A04: 3CE0820B  lis r7, -0x7df5
	ctx.r[7].s64 = -2113208320;
	// 83249A08: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83249A0C: 38878AF8  addi r4, r7, -0x7508
	ctx.r[4].s64 = ctx.r[7].s64 + -29960;
	// 83249A10: 387F002C  addi r3, r31, 0x2c
	ctx.r[3].s64 = ctx.r[31].s64 + 44;
	// 83249A14: 4AFE34BD  bl 0x8222ced0
	ctx.lr = 0x83249A18;
	sub_8222CED0(ctx, base);
	// 83249A18: 3CC0820B  lis r6, -0x7df5
	ctx.r[6].s64 = -2113208320;
	// 83249A1C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83249A20: 38868AD4  addi r4, r6, -0x752c
	ctx.r[4].s64 = ctx.r[6].s64 + -29996;
	// 83249A24: 387F0030  addi r3, r31, 0x30
	ctx.r[3].s64 = ctx.r[31].s64 + 48;
	// 83249A28: 4AFE34A9  bl 0x8222ced0
	ctx.lr = 0x83249A2C;
	sub_8222CED0(ctx, base);
	// 83249A2C: 3C80820B  lis r4, -0x7df5
	ctx.r[4].s64 = -2113208320;
	// 83249A30: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83249A34: 38848AB4  addi r4, r4, -0x754c
	ctx.r[4].s64 = ctx.r[4].s64 + -30028;
	// 83249A38: 387F0034  addi r3, r31, 0x34
	ctx.r[3].s64 = ctx.r[31].s64 + 52;
	// 83249A3C: 4AFE3495  bl 0x8222ced0
	ctx.lr = 0x83249A40;
	sub_8222CED0(ctx, base);
	// 83249A40: 3C60820B  lis r3, -0x7df5
	ctx.r[3].s64 = -2113208320;
	// 83249A44: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83249A48: 38838A94  addi r4, r3, -0x756c
	ctx.r[4].s64 = ctx.r[3].s64 + -30060;
	// 83249A4C: 387F0038  addi r3, r31, 0x38
	ctx.r[3].s64 = ctx.r[31].s64 + 56;
	// 83249A50: 4AFE3481  bl 0x8222ced0
	ctx.lr = 0x83249A54;
	sub_8222CED0(ctx, base);
	// 83249A54: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 83249A58: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83249A5C: 388B8A70  addi r4, r11, -0x7590
	ctx.r[4].s64 = ctx.r[11].s64 + -30096;
	// 83249A60: 387F003C  addi r3, r31, 0x3c
	ctx.r[3].s64 = ctx.r[31].s64 + 60;
	// 83249A64: 4AFE346D  bl 0x8222ced0
	ctx.lr = 0x83249A68;
	sub_8222CED0(ctx, base);
	// 83249A68: 3D40832A  lis r10, -0x7cd6
	ctx.r[10].s64 = -2094399488;
	// 83249A6C: 386A7C30  addi r3, r10, 0x7c30
	ctx.r[3].s64 = ctx.r[10].s64 + 31792;
	// 83249A70: 4BA604B1  bl 0x82ca9f20
	ctx.lr = 0x83249A74;
	sub_82CA9F20(ctx, base);
	// 83249A74: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83249A78: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83249A7C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83249A80: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83249A84: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83249A88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83249A88 size=56
    let mut pc: u32 = 0x83249A88;
    'dispatch: loop {
        match pc {
            0x83249A88 => {
    //   block [0x83249A88..0x83249AC0)
	// 83249A88: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83249A8C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83249A90: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83249A94: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83249A98: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83249A9C: 386B42C8  addi r3, r11, 0x42c8
	ctx.r[3].s64 = ctx.r[11].s64 + 17096;
	// 83249AA0: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83249AA4: 4AFAA2B5  bl 0x821f3d58
	ctx.lr = 0x83249AA8;
	sub_821F3D58(ctx, base);
	// 83249AA8: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 83249AAC: 906A7740  stw r3, 0x7740(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(30528 as u32), ctx.r[3].u32 ) };
	// 83249AB0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83249AB4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83249AB8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83249ABC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83249AC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83249AC0 size=56
    let mut pc: u32 = 0x83249AC0;
    'dispatch: loop {
        match pc {
            0x83249AC0 => {
    //   block [0x83249AC0..0x83249AF8)
	// 83249AC0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83249AC4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83249AC8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83249ACC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83249AD0: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83249AD4: 386B42DC  addi r3, r11, 0x42dc
	ctx.r[3].s64 = ctx.r[11].s64 + 17116;
	// 83249AD8: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83249ADC: 4AFAA27D  bl 0x821f3d58
	ctx.lr = 0x83249AE0;
	sub_821F3D58(ctx, base);
	// 83249AE0: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 83249AE4: 906A7744  stw r3, 0x7744(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(30532 as u32), ctx.r[3].u32 ) };
	// 83249AE8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83249AEC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83249AF0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83249AF4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83249AF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83249AF8 size=56
    let mut pc: u32 = 0x83249AF8;
    'dispatch: loop {
        match pc {
            0x83249AF8 => {
    //   block [0x83249AF8..0x83249B30)
	// 83249AF8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83249AFC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83249B00: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83249B04: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83249B08: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83249B0C: 386B42E8  addi r3, r11, 0x42e8
	ctx.r[3].s64 = ctx.r[11].s64 + 17128;
	// 83249B10: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83249B14: 4AFAA245  bl 0x821f3d58
	ctx.lr = 0x83249B18;
	sub_821F3D58(ctx, base);
	// 83249B18: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 83249B1C: 906A7748  stw r3, 0x7748(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(30536 as u32), ctx.r[3].u32 ) };
	// 83249B20: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83249B24: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83249B28: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83249B2C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83249B30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83249B30 size=56
    let mut pc: u32 = 0x83249B30;
    'dispatch: loop {
        match pc {
            0x83249B30 => {
    //   block [0x83249B30..0x83249B68)
	// 83249B30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83249B34: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83249B38: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83249B3C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83249B40: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83249B44: 386B4300  addi r3, r11, 0x4300
	ctx.r[3].s64 = ctx.r[11].s64 + 17152;
	// 83249B48: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83249B4C: 4AFAA20D  bl 0x821f3d58
	ctx.lr = 0x83249B50;
	sub_821F3D58(ctx, base);
	// 83249B50: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 83249B54: 906A774C  stw r3, 0x774c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(30540 as u32), ctx.r[3].u32 ) };
	// 83249B58: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83249B5C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83249B60: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83249B64: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83249B68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83249B68 size=56
    let mut pc: u32 = 0x83249B68;
    'dispatch: loop {
        match pc {
            0x83249B68 => {
    //   block [0x83249B68..0x83249BA0)
	// 83249B68: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83249B6C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83249B70: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83249B74: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83249B78: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83249B7C: 386B4310  addi r3, r11, 0x4310
	ctx.r[3].s64 = ctx.r[11].s64 + 17168;
	// 83249B80: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83249B84: 4AFAA1D5  bl 0x821f3d58
	ctx.lr = 0x83249B88;
	sub_821F3D58(ctx, base);
	// 83249B88: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 83249B8C: 906A7750  stw r3, 0x7750(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(30544 as u32), ctx.r[3].u32 ) };
	// 83249B90: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83249B94: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83249B98: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83249B9C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83249BA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83249BA0 size=56
    let mut pc: u32 = 0x83249BA0;
    'dispatch: loop {
        match pc {
            0x83249BA0 => {
    //   block [0x83249BA0..0x83249BD8)
	// 83249BA0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83249BA4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83249BA8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83249BAC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83249BB0: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83249BB4: 386B4328  addi r3, r11, 0x4328
	ctx.r[3].s64 = ctx.r[11].s64 + 17192;
	// 83249BB8: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83249BBC: 4AFAA19D  bl 0x821f3d58
	ctx.lr = 0x83249BC0;
	sub_821F3D58(ctx, base);
	// 83249BC0: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 83249BC4: 906A7754  stw r3, 0x7754(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(30548 as u32), ctx.r[3].u32 ) };
	// 83249BC8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83249BCC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83249BD0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83249BD4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83249BD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83249BD8 size=56
    let mut pc: u32 = 0x83249BD8;
    'dispatch: loop {
        match pc {
            0x83249BD8 => {
    //   block [0x83249BD8..0x83249C10)
	// 83249BD8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83249BDC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83249BE0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83249BE4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83249BE8: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83249BEC: 386B4338  addi r3, r11, 0x4338
	ctx.r[3].s64 = ctx.r[11].s64 + 17208;
	// 83249BF0: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83249BF4: 4AFAA165  bl 0x821f3d58
	ctx.lr = 0x83249BF8;
	sub_821F3D58(ctx, base);
	// 83249BF8: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 83249BFC: 906A7758  stw r3, 0x7758(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(30552 as u32), ctx.r[3].u32 ) };
	// 83249C00: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83249C04: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83249C08: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83249C0C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83249C10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83249C10 size=56
    let mut pc: u32 = 0x83249C10;
    'dispatch: loop {
        match pc {
            0x83249C10 => {
    //   block [0x83249C10..0x83249C48)
	// 83249C10: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83249C14: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83249C18: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83249C1C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83249C20: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83249C24: 386B4350  addi r3, r11, 0x4350
	ctx.r[3].s64 = ctx.r[11].s64 + 17232;
	// 83249C28: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83249C2C: 4AFAA12D  bl 0x821f3d58
	ctx.lr = 0x83249C30;
	sub_821F3D58(ctx, base);
	// 83249C30: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 83249C34: 906A775C  stw r3, 0x775c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(30556 as u32), ctx.r[3].u32 ) };
	// 83249C38: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83249C3C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83249C40: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83249C44: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83249C48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83249C48 size=56
    let mut pc: u32 = 0x83249C48;
    'dispatch: loop {
        match pc {
            0x83249C48 => {
    //   block [0x83249C48..0x83249C80)
	// 83249C48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83249C4C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83249C50: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83249C54: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83249C58: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83249C5C: 386B4368  addi r3, r11, 0x4368
	ctx.r[3].s64 = ctx.r[11].s64 + 17256;
	// 83249C60: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83249C64: 4AFAA0F5  bl 0x821f3d58
	ctx.lr = 0x83249C68;
	sub_821F3D58(ctx, base);
	// 83249C68: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 83249C6C: 906A7760  stw r3, 0x7760(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(30560 as u32), ctx.r[3].u32 ) };
	// 83249C70: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83249C74: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83249C78: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83249C7C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83249C80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83249C80 size=56
    let mut pc: u32 = 0x83249C80;
    'dispatch: loop {
        match pc {
            0x83249C80 => {
    //   block [0x83249C80..0x83249CB8)
	// 83249C80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83249C84: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83249C88: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83249C8C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83249C90: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83249C94: 386B4380  addi r3, r11, 0x4380
	ctx.r[3].s64 = ctx.r[11].s64 + 17280;
	// 83249C98: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83249C9C: 4AFAA0BD  bl 0x821f3d58
	ctx.lr = 0x83249CA0;
	sub_821F3D58(ctx, base);
	// 83249CA0: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 83249CA4: 906A7764  stw r3, 0x7764(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(30564 as u32), ctx.r[3].u32 ) };
	// 83249CA8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83249CAC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83249CB0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83249CB4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83249CB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83249CB8 size=56
    let mut pc: u32 = 0x83249CB8;
    'dispatch: loop {
        match pc {
            0x83249CB8 => {
    //   block [0x83249CB8..0x83249CF0)
	// 83249CB8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83249CBC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83249CC0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83249CC4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83249CC8: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83249CCC: 386B4394  addi r3, r11, 0x4394
	ctx.r[3].s64 = ctx.r[11].s64 + 17300;
	// 83249CD0: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83249CD4: 4AFAA085  bl 0x821f3d58
	ctx.lr = 0x83249CD8;
	sub_821F3D58(ctx, base);
	// 83249CD8: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 83249CDC: 906A7768  stw r3, 0x7768(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(30568 as u32), ctx.r[3].u32 ) };
	// 83249CE0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83249CE4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83249CE8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83249CEC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83249CF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83249CF0 size=56
    let mut pc: u32 = 0x83249CF0;
    'dispatch: loop {
        match pc {
            0x83249CF0 => {
    //   block [0x83249CF0..0x83249D28)
	// 83249CF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83249CF4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83249CF8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83249CFC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83249D00: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83249D04: 386B43B4  addi r3, r11, 0x43b4
	ctx.r[3].s64 = ctx.r[11].s64 + 17332;
	// 83249D08: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83249D0C: 4AFAA04D  bl 0x821f3d58
	ctx.lr = 0x83249D10;
	sub_821F3D58(ctx, base);
	// 83249D10: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 83249D14: 906A776C  stw r3, 0x776c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(30572 as u32), ctx.r[3].u32 ) };
	// 83249D18: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83249D1C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83249D20: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83249D24: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83249D28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83249D28 size=56
    let mut pc: u32 = 0x83249D28;
    'dispatch: loop {
        match pc {
            0x83249D28 => {
    //   block [0x83249D28..0x83249D60)
	// 83249D28: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83249D2C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83249D30: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83249D34: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83249D38: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83249D3C: 386B43C8  addi r3, r11, 0x43c8
	ctx.r[3].s64 = ctx.r[11].s64 + 17352;
	// 83249D40: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83249D44: 4AFAA015  bl 0x821f3d58
	ctx.lr = 0x83249D48;
	sub_821F3D58(ctx, base);
	// 83249D48: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 83249D4C: 906A7770  stw r3, 0x7770(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(30576 as u32), ctx.r[3].u32 ) };
	// 83249D50: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83249D54: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83249D58: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83249D5C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83249D60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83249D60 size=56
    let mut pc: u32 = 0x83249D60;
    'dispatch: loop {
        match pc {
            0x83249D60 => {
    //   block [0x83249D60..0x83249D98)
	// 83249D60: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83249D64: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83249D68: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83249D6C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83249D70: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83249D74: 386B43D4  addi r3, r11, 0x43d4
	ctx.r[3].s64 = ctx.r[11].s64 + 17364;
	// 83249D78: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83249D7C: 4AFA9FDD  bl 0x821f3d58
	ctx.lr = 0x83249D80;
	sub_821F3D58(ctx, base);
	// 83249D80: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 83249D84: 906A7774  stw r3, 0x7774(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(30580 as u32), ctx.r[3].u32 ) };
	// 83249D88: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83249D8C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83249D90: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83249D94: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83249D98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83249D98 size=56
    let mut pc: u32 = 0x83249D98;
    'dispatch: loop {
        match pc {
            0x83249D98 => {
    //   block [0x83249D98..0x83249DD0)
	// 83249D98: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83249D9C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83249DA0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83249DA4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83249DA8: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83249DAC: 386B43E0  addi r3, r11, 0x43e0
	ctx.r[3].s64 = ctx.r[11].s64 + 17376;
	// 83249DB0: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83249DB4: 4AFA9FA5  bl 0x821f3d58
	ctx.lr = 0x83249DB8;
	sub_821F3D58(ctx, base);
	// 83249DB8: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 83249DBC: 906A7778  stw r3, 0x7778(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(30584 as u32), ctx.r[3].u32 ) };
	// 83249DC0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83249DC4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83249DC8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83249DCC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83249DD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83249DD0 size=56
    let mut pc: u32 = 0x83249DD0;
    'dispatch: loop {
        match pc {
            0x83249DD0 => {
    //   block [0x83249DD0..0x83249E08)
	// 83249DD0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83249DD4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83249DD8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83249DDC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83249DE0: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83249DE4: 386B43F0  addi r3, r11, 0x43f0
	ctx.r[3].s64 = ctx.r[11].s64 + 17392;
	// 83249DE8: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83249DEC: 4AFA9F6D  bl 0x821f3d58
	ctx.lr = 0x83249DF0;
	sub_821F3D58(ctx, base);
	// 83249DF0: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 83249DF4: 906A777C  stw r3, 0x777c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(30588 as u32), ctx.r[3].u32 ) };
	// 83249DF8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83249DFC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83249E00: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83249E04: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83249E08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83249E08 size=56
    let mut pc: u32 = 0x83249E08;
    'dispatch: loop {
        match pc {
            0x83249E08 => {
    //   block [0x83249E08..0x83249E40)
	// 83249E08: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83249E0C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83249E10: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83249E14: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83249E18: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83249E1C: 386B4400  addi r3, r11, 0x4400
	ctx.r[3].s64 = ctx.r[11].s64 + 17408;
	// 83249E20: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83249E24: 4AFA9F35  bl 0x821f3d58
	ctx.lr = 0x83249E28;
	sub_821F3D58(ctx, base);
	// 83249E28: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 83249E2C: 906A7780  stw r3, 0x7780(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(30592 as u32), ctx.r[3].u32 ) };
	// 83249E30: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83249E34: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83249E38: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83249E3C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83249E40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83249E40 size=56
    let mut pc: u32 = 0x83249E40;
    'dispatch: loop {
        match pc {
            0x83249E40 => {
    //   block [0x83249E40..0x83249E78)
	// 83249E40: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83249E44: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83249E48: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83249E4C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83249E50: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83249E54: 386B4418  addi r3, r11, 0x4418
	ctx.r[3].s64 = ctx.r[11].s64 + 17432;
	// 83249E58: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83249E5C: 4AFA9EFD  bl 0x821f3d58
	ctx.lr = 0x83249E60;
	sub_821F3D58(ctx, base);
	// 83249E60: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 83249E64: 906A7784  stw r3, 0x7784(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(30596 as u32), ctx.r[3].u32 ) };
	// 83249E68: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83249E6C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83249E70: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83249E74: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83249E78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83249E78 size=56
    let mut pc: u32 = 0x83249E78;
    'dispatch: loop {
        match pc {
            0x83249E78 => {
    //   block [0x83249E78..0x83249EB0)
	// 83249E78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83249E7C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83249E80: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83249E84: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83249E88: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83249E8C: 386B4424  addi r3, r11, 0x4424
	ctx.r[3].s64 = ctx.r[11].s64 + 17444;
	// 83249E90: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83249E94: 4AFA9EC5  bl 0x821f3d58
	ctx.lr = 0x83249E98;
	sub_821F3D58(ctx, base);
	// 83249E98: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 83249E9C: 906A7788  stw r3, 0x7788(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(30600 as u32), ctx.r[3].u32 ) };
	// 83249EA0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83249EA4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83249EA8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83249EAC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83249EB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83249EB0 size=56
    let mut pc: u32 = 0x83249EB0;
    'dispatch: loop {
        match pc {
            0x83249EB0 => {
    //   block [0x83249EB0..0x83249EE8)
	// 83249EB0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83249EB4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83249EB8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83249EBC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83249EC0: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83249EC4: 386B4430  addi r3, r11, 0x4430
	ctx.r[3].s64 = ctx.r[11].s64 + 17456;
	// 83249EC8: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83249ECC: 4AFA9E8D  bl 0x821f3d58
	ctx.lr = 0x83249ED0;
	sub_821F3D58(ctx, base);
	// 83249ED0: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 83249ED4: 906A778C  stw r3, 0x778c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(30604 as u32), ctx.r[3].u32 ) };
	// 83249ED8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83249EDC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83249EE0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83249EE4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83249EE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83249EE8 size=56
    let mut pc: u32 = 0x83249EE8;
    'dispatch: loop {
        match pc {
            0x83249EE8 => {
    //   block [0x83249EE8..0x83249F20)
	// 83249EE8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83249EEC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83249EF0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83249EF4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83249EF8: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83249EFC: 386B4440  addi r3, r11, 0x4440
	ctx.r[3].s64 = ctx.r[11].s64 + 17472;
	// 83249F00: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83249F04: 4AFA9E55  bl 0x821f3d58
	ctx.lr = 0x83249F08;
	sub_821F3D58(ctx, base);
	// 83249F08: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 83249F0C: 906A7790  stw r3, 0x7790(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(30608 as u32), ctx.r[3].u32 ) };
	// 83249F10: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83249F14: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83249F18: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83249F1C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83249F20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83249F20 size=64
    let mut pc: u32 = 0x83249F20;
    'dispatch: loop {
        match pc {
            0x83249F20 => {
    //   block [0x83249F20..0x83249F60)
	// 83249F20: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83249F24: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83249F28: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83249F2C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83249F30: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 83249F34: 388B1AD4  addi r4, r11, 0x1ad4
	ctx.r[4].s64 = ctx.r[11].s64 + 6868;
	// 83249F38: 386A7794  addi r3, r10, 0x7794
	ctx.r[3].s64 = ctx.r[10].s64 + 30612;
	// 83249F3C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83249F40: 4AFE2F91  bl 0x8222ced0
	ctx.lr = 0x83249F44;
	sub_8222CED0(ctx, base);
	// 83249F44: 3D20832A  lis r9, -0x7cd6
	ctx.r[9].s64 = -2094399488;
	// 83249F48: 38697C98  addi r3, r9, 0x7c98
	ctx.r[3].s64 = ctx.r[9].s64 + 31896;
	// 83249F4C: 4BA5FFD5  bl 0x82ca9f20
	ctx.lr = 0x83249F50;
	sub_82CA9F20(ctx, base);
	// 83249F50: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83249F54: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83249F58: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83249F5C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83249F60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83249F60 size=64
    let mut pc: u32 = 0x83249F60;
    'dispatch: loop {
        match pc {
            0x83249F60 => {
    //   block [0x83249F60..0x83249FA0)
	// 83249F60: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83249F64: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83249F68: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83249F6C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83249F70: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 83249F74: 388B1ADC  addi r4, r11, 0x1adc
	ctx.r[4].s64 = ctx.r[11].s64 + 6876;
	// 83249F78: 386A7798  addi r3, r10, 0x7798
	ctx.r[3].s64 = ctx.r[10].s64 + 30616;
	// 83249F7C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83249F80: 4AFE2F51  bl 0x8222ced0
	ctx.lr = 0x83249F84;
	sub_8222CED0(ctx, base);
	// 83249F84: 3D20832A  lis r9, -0x7cd6
	ctx.r[9].s64 = -2094399488;
	// 83249F88: 38697CA8  addi r3, r9, 0x7ca8
	ctx.r[3].s64 = ctx.r[9].s64 + 31912;
	// 83249F8C: 4BA5FF95  bl 0x82ca9f20
	ctx.lr = 0x83249F90;
	sub_82CA9F20(ctx, base);
	// 83249F90: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83249F94: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83249F98: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83249F9C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83249FA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83249FA0 size=64
    let mut pc: u32 = 0x83249FA0;
    'dispatch: loop {
        match pc {
            0x83249FA0 => {
    //   block [0x83249FA0..0x83249FE0)
	// 83249FA0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83249FA4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83249FA8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83249FAC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83249FB0: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 83249FB4: 388B1AF0  addi r4, r11, 0x1af0
	ctx.r[4].s64 = ctx.r[11].s64 + 6896;
	// 83249FB8: 386A779C  addi r3, r10, 0x779c
	ctx.r[3].s64 = ctx.r[10].s64 + 30620;
	// 83249FBC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83249FC0: 4AFE2F11  bl 0x8222ced0
	ctx.lr = 0x83249FC4;
	sub_8222CED0(ctx, base);
	// 83249FC4: 3D20832A  lis r9, -0x7cd6
	ctx.r[9].s64 = -2094399488;
	// 83249FC8: 38697CB8  addi r3, r9, 0x7cb8
	ctx.r[3].s64 = ctx.r[9].s64 + 31928;
	// 83249FCC: 4BA5FF55  bl 0x82ca9f20
	ctx.lr = 0x83249FD0;
	sub_82CA9F20(ctx, base);
	// 83249FD0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83249FD4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83249FD8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83249FDC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83249FE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83249FE0 size=64
    let mut pc: u32 = 0x83249FE0;
    'dispatch: loop {
        match pc {
            0x83249FE0 => {
    //   block [0x83249FE0..0x8324A020)
	// 83249FE0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83249FE4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83249FE8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83249FEC: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 83249FF0: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 83249FF4: 388B8C78  addi r4, r11, -0x7388
	ctx.r[4].s64 = ctx.r[11].s64 + -29576;
	// 83249FF8: 386A77A0  addi r3, r10, 0x77a0
	ctx.r[3].s64 = ctx.r[10].s64 + 30624;
	// 83249FFC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324A000: 4AFE2ED1  bl 0x8222ced0
	ctx.lr = 0x8324A004;
	sub_8222CED0(ctx, base);
	// 8324A004: 3D20832A  lis r9, -0x7cd6
	ctx.r[9].s64 = -2094399488;
	// 8324A008: 38697CC8  addi r3, r9, 0x7cc8
	ctx.r[3].s64 = ctx.r[9].s64 + 31944;
	// 8324A00C: 4BA5FF15  bl 0x82ca9f20
	ctx.lr = 0x8324A010;
	sub_82CA9F20(ctx, base);
	// 8324A010: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324A014: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324A018: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324A01C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324A020(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324A020 size=64
    let mut pc: u32 = 0x8324A020;
    'dispatch: loop {
        match pc {
            0x8324A020 => {
    //   block [0x8324A020..0x8324A060)
	// 8324A020: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324A024: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324A028: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324A02C: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 8324A030: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324A034: 388B8C88  addi r4, r11, -0x7378
	ctx.r[4].s64 = ctx.r[11].s64 + -29560;
	// 8324A038: 386A77A4  addi r3, r10, 0x77a4
	ctx.r[3].s64 = ctx.r[10].s64 + 30628;
	// 8324A03C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324A040: 4AFE2E91  bl 0x8222ced0
	ctx.lr = 0x8324A044;
	sub_8222CED0(ctx, base);
	// 8324A044: 3D20832A  lis r9, -0x7cd6
	ctx.r[9].s64 = -2094399488;
	// 8324A048: 38697CD8  addi r3, r9, 0x7cd8
	ctx.r[3].s64 = ctx.r[9].s64 + 31960;
	// 8324A04C: 4BA5FED5  bl 0x82ca9f20
	ctx.lr = 0x8324A050;
	sub_82CA9F20(ctx, base);
	// 8324A050: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324A054: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324A058: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324A05C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324A060(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324A060 size=64
    let mut pc: u32 = 0x8324A060;
    'dispatch: loop {
        match pc {
            0x8324A060 => {
    //   block [0x8324A060..0x8324A0A0)
	// 8324A060: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324A064: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324A068: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324A06C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8324A070: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324A074: 388B052C  addi r4, r11, 0x52c
	ctx.r[4].s64 = ctx.r[11].s64 + 1324;
	// 8324A078: 386A77A8  addi r3, r10, 0x77a8
	ctx.r[3].s64 = ctx.r[10].s64 + 30632;
	// 8324A07C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324A080: 4AFE2E51  bl 0x8222ced0
	ctx.lr = 0x8324A084;
	sub_8222CED0(ctx, base);
	// 8324A084: 3D20832A  lis r9, -0x7cd6
	ctx.r[9].s64 = -2094399488;
	// 8324A088: 38697CE8  addi r3, r9, 0x7ce8
	ctx.r[3].s64 = ctx.r[9].s64 + 31976;
	// 8324A08C: 4BA5FE95  bl 0x82ca9f20
	ctx.lr = 0x8324A090;
	sub_82CA9F20(ctx, base);
	// 8324A090: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324A094: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324A098: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324A09C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324A0A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324A0A0 size=64
    let mut pc: u32 = 0x8324A0A0;
    'dispatch: loop {
        match pc {
            0x8324A0A0 => {
    //   block [0x8324A0A0..0x8324A0E0)
	// 8324A0A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324A0A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324A0A8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324A0AC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8324A0B0: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324A0B4: 388B1AD4  addi r4, r11, 0x1ad4
	ctx.r[4].s64 = ctx.r[11].s64 + 6868;
	// 8324A0B8: 386A77AC  addi r3, r10, 0x77ac
	ctx.r[3].s64 = ctx.r[10].s64 + 30636;
	// 8324A0BC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324A0C0: 4AFE2E11  bl 0x8222ced0
	ctx.lr = 0x8324A0C4;
	sub_8222CED0(ctx, base);
	// 8324A0C4: 3D20832A  lis r9, -0x7cd6
	ctx.r[9].s64 = -2094399488;
	// 8324A0C8: 38697CF8  addi r3, r9, 0x7cf8
	ctx.r[3].s64 = ctx.r[9].s64 + 31992;
	// 8324A0CC: 4BA5FE55  bl 0x82ca9f20
	ctx.lr = 0x8324A0D0;
	sub_82CA9F20(ctx, base);
	// 8324A0D0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324A0D4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324A0D8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324A0DC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324A0E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324A0E0 size=64
    let mut pc: u32 = 0x8324A0E0;
    'dispatch: loop {
        match pc {
            0x8324A0E0 => {
    //   block [0x8324A0E0..0x8324A120)
	// 8324A0E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324A0E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324A0E8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324A0EC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8324A0F0: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324A0F4: 388B1ADC  addi r4, r11, 0x1adc
	ctx.r[4].s64 = ctx.r[11].s64 + 6876;
	// 8324A0F8: 386A77B0  addi r3, r10, 0x77b0
	ctx.r[3].s64 = ctx.r[10].s64 + 30640;
	// 8324A0FC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324A100: 4AFE2DD1  bl 0x8222ced0
	ctx.lr = 0x8324A104;
	sub_8222CED0(ctx, base);
	// 8324A104: 3D20832A  lis r9, -0x7cd6
	ctx.r[9].s64 = -2094399488;
	// 8324A108: 38697D08  addi r3, r9, 0x7d08
	ctx.r[3].s64 = ctx.r[9].s64 + 32008;
	// 8324A10C: 4BA5FE15  bl 0x82ca9f20
	ctx.lr = 0x8324A110;
	sub_82CA9F20(ctx, base);
	// 8324A110: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324A114: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324A118: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324A11C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324A120(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324A120 size=64
    let mut pc: u32 = 0x8324A120;
    'dispatch: loop {
        match pc {
            0x8324A120 => {
    //   block [0x8324A120..0x8324A160)
	// 8324A120: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324A124: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324A128: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324A12C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8324A130: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324A134: 388B1AF0  addi r4, r11, 0x1af0
	ctx.r[4].s64 = ctx.r[11].s64 + 6896;
	// 8324A138: 386A77B4  addi r3, r10, 0x77b4
	ctx.r[3].s64 = ctx.r[10].s64 + 30644;
	// 8324A13C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324A140: 4AFE2D91  bl 0x8222ced0
	ctx.lr = 0x8324A144;
	sub_8222CED0(ctx, base);
	// 8324A144: 3D20832A  lis r9, -0x7cd6
	ctx.r[9].s64 = -2094399488;
	// 8324A148: 38697D18  addi r3, r9, 0x7d18
	ctx.r[3].s64 = ctx.r[9].s64 + 32024;
	// 8324A14C: 4BA5FDD5  bl 0x82ca9f20
	ctx.lr = 0x8324A150;
	sub_82CA9F20(ctx, base);
	// 8324A150: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324A154: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324A158: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324A15C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324A160(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324A160 size=64
    let mut pc: u32 = 0x8324A160;
    'dispatch: loop {
        match pc {
            0x8324A160 => {
    //   block [0x8324A160..0x8324A1A0)
	// 8324A160: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324A164: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324A168: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324A16C: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 8324A170: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324A174: 388B8E84  addi r4, r11, -0x717c
	ctx.r[4].s64 = ctx.r[11].s64 + -29052;
	// 8324A178: 386A77B8  addi r3, r10, 0x77b8
	ctx.r[3].s64 = ctx.r[10].s64 + 30648;
	// 8324A17C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324A180: 4AFE2D51  bl 0x8222ced0
	ctx.lr = 0x8324A184;
	sub_8222CED0(ctx, base);
	// 8324A184: 3D20832A  lis r9, -0x7cd6
	ctx.r[9].s64 = -2094399488;
	// 8324A188: 38697D28  addi r3, r9, 0x7d28
	ctx.r[3].s64 = ctx.r[9].s64 + 32040;
	// 8324A18C: 4BA5FD95  bl 0x82ca9f20
	ctx.lr = 0x8324A190;
	sub_82CA9F20(ctx, base);
	// 8324A190: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324A194: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324A198: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324A19C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324A1A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324A1A0 size=56
    let mut pc: u32 = 0x8324A1A0;
    'dispatch: loop {
        match pc {
            0x8324A1A0 => {
    //   block [0x8324A1A0..0x8324A1D8)
	// 8324A1A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324A1A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324A1A8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324A1AC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8324A1B0: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8324A1B4: 386B4CEC  addi r3, r11, 0x4cec
	ctx.r[3].s64 = ctx.r[11].s64 + 19692;
	// 8324A1B8: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8324A1BC: 4AFA9B9D  bl 0x821f3d58
	ctx.lr = 0x8324A1C0;
	sub_821F3D58(ctx, base);
	// 8324A1C0: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324A1C4: 906A77BC  stw r3, 0x77bc(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(30652 as u32), ctx.r[3].u32 ) };
	// 8324A1C8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324A1CC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324A1D0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324A1D4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324A1D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324A1D8 size=64
    let mut pc: u32 = 0x8324A1D8;
    'dispatch: loop {
        match pc {
            0x8324A1D8 => {
    //   block [0x8324A1D8..0x8324A218)
	// 8324A1D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324A1DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324A1E0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324A1E4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8324A1E8: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324A1EC: 388B1AD4  addi r4, r11, 0x1ad4
	ctx.r[4].s64 = ctx.r[11].s64 + 6868;
	// 8324A1F0: 386A77C0  addi r3, r10, 0x77c0
	ctx.r[3].s64 = ctx.r[10].s64 + 30656;
	// 8324A1F4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324A1F8: 4AFE2CD9  bl 0x8222ced0
	ctx.lr = 0x8324A1FC;
	sub_8222CED0(ctx, base);
	// 8324A1FC: 3D20832A  lis r9, -0x7cd6
	ctx.r[9].s64 = -2094399488;
	// 8324A200: 38697D90  addi r3, r9, 0x7d90
	ctx.r[3].s64 = ctx.r[9].s64 + 32144;
	// 8324A204: 4BA5FD1D  bl 0x82ca9f20
	ctx.lr = 0x8324A208;
	sub_82CA9F20(ctx, base);
	// 8324A208: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324A20C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324A210: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324A214: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324A218(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324A218 size=64
    let mut pc: u32 = 0x8324A218;
    'dispatch: loop {
        match pc {
            0x8324A218 => {
    //   block [0x8324A218..0x8324A258)
	// 8324A218: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324A21C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324A220: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324A224: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8324A228: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324A22C: 388B1ADC  addi r4, r11, 0x1adc
	ctx.r[4].s64 = ctx.r[11].s64 + 6876;
	// 8324A230: 386A77C4  addi r3, r10, 0x77c4
	ctx.r[3].s64 = ctx.r[10].s64 + 30660;
	// 8324A234: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324A238: 4AFE2C99  bl 0x8222ced0
	ctx.lr = 0x8324A23C;
	sub_8222CED0(ctx, base);
	// 8324A23C: 3D20832A  lis r9, -0x7cd6
	ctx.r[9].s64 = -2094399488;
	// 8324A240: 38697DA0  addi r3, r9, 0x7da0
	ctx.r[3].s64 = ctx.r[9].s64 + 32160;
	// 8324A244: 4BA5FCDD  bl 0x82ca9f20
	ctx.lr = 0x8324A248;
	sub_82CA9F20(ctx, base);
	// 8324A248: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324A24C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324A250: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324A254: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324A258(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324A258 size=64
    let mut pc: u32 = 0x8324A258;
    'dispatch: loop {
        match pc {
            0x8324A258 => {
    //   block [0x8324A258..0x8324A298)
	// 8324A258: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324A25C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324A260: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324A264: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8324A268: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324A26C: 388B1AF0  addi r4, r11, 0x1af0
	ctx.r[4].s64 = ctx.r[11].s64 + 6896;
	// 8324A270: 386A77C8  addi r3, r10, 0x77c8
	ctx.r[3].s64 = ctx.r[10].s64 + 30664;
	// 8324A274: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324A278: 4AFE2C59  bl 0x8222ced0
	ctx.lr = 0x8324A27C;
	sub_8222CED0(ctx, base);
	// 8324A27C: 3D20832A  lis r9, -0x7cd6
	ctx.r[9].s64 = -2094399488;
	// 8324A280: 38697DB0  addi r3, r9, 0x7db0
	ctx.r[3].s64 = ctx.r[9].s64 + 32176;
	// 8324A284: 4BA5FC9D  bl 0x82ca9f20
	ctx.lr = 0x8324A288;
	sub_82CA9F20(ctx, base);
	// 8324A288: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324A28C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324A290: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324A294: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324A298(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324A298 size=64
    let mut pc: u32 = 0x8324A298;
    'dispatch: loop {
        match pc {
            0x8324A298 => {
    //   block [0x8324A298..0x8324A2D8)
	// 8324A298: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324A29C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324A2A0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324A2A4: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 8324A2A8: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324A2AC: 388B8FE0  addi r4, r11, -0x7020
	ctx.r[4].s64 = ctx.r[11].s64 + -28704;
	// 8324A2B0: 386A77CC  addi r3, r10, 0x77cc
	ctx.r[3].s64 = ctx.r[10].s64 + 30668;
	// 8324A2B4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324A2B8: 4AFE2C19  bl 0x8222ced0
	ctx.lr = 0x8324A2BC;
	sub_8222CED0(ctx, base);
	// 8324A2BC: 3D20832A  lis r9, -0x7cd6
	ctx.r[9].s64 = -2094399488;
	// 8324A2C0: 38697DC0  addi r3, r9, 0x7dc0
	ctx.r[3].s64 = ctx.r[9].s64 + 32192;
	// 8324A2C4: 4BA5FC5D  bl 0x82ca9f20
	ctx.lr = 0x8324A2C8;
	sub_82CA9F20(ctx, base);
	// 8324A2C8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324A2CC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324A2D0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324A2D4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324A2D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324A2D8 size=64
    let mut pc: u32 = 0x8324A2D8;
    'dispatch: loop {
        match pc {
            0x8324A2D8 => {
    //   block [0x8324A2D8..0x8324A318)
	// 8324A2D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324A2DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324A2E0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324A2E4: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 8324A2E8: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324A2EC: 388B8FFC  addi r4, r11, -0x7004
	ctx.r[4].s64 = ctx.r[11].s64 + -28676;
	// 8324A2F0: 386A77D0  addi r3, r10, 0x77d0
	ctx.r[3].s64 = ctx.r[10].s64 + 30672;
	// 8324A2F4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324A2F8: 4AFE2BD9  bl 0x8222ced0
	ctx.lr = 0x8324A2FC;
	sub_8222CED0(ctx, base);
	// 8324A2FC: 3D20832A  lis r9, -0x7cd6
	ctx.r[9].s64 = -2094399488;
	// 8324A300: 38697DD0  addi r3, r9, 0x7dd0
	ctx.r[3].s64 = ctx.r[9].s64 + 32208;
	// 8324A304: 4BA5FC1D  bl 0x82ca9f20
	ctx.lr = 0x8324A308;
	sub_82CA9F20(ctx, base);
	// 8324A308: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324A30C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324A310: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324A314: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324A318(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324A318 size=64
    let mut pc: u32 = 0x8324A318;
    'dispatch: loop {
        match pc {
            0x8324A318 => {
    //   block [0x8324A318..0x8324A358)
	// 8324A318: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324A31C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324A320: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324A324: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 8324A328: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324A32C: 388B8FE0  addi r4, r11, -0x7020
	ctx.r[4].s64 = ctx.r[11].s64 + -28704;
	// 8324A330: 386A77D4  addi r3, r10, 0x77d4
	ctx.r[3].s64 = ctx.r[10].s64 + 30676;
	// 8324A334: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324A338: 4AFE2B99  bl 0x8222ced0
	ctx.lr = 0x8324A33C;
	sub_8222CED0(ctx, base);
	// 8324A33C: 3D20832A  lis r9, -0x7cd6
	ctx.r[9].s64 = -2094399488;
	// 8324A340: 38697DE0  addi r3, r9, 0x7de0
	ctx.r[3].s64 = ctx.r[9].s64 + 32224;
	// 8324A344: 4BA5FBDD  bl 0x82ca9f20
	ctx.lr = 0x8324A348;
	sub_82CA9F20(ctx, base);
	// 8324A348: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324A34C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324A350: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324A354: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324A358(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324A358 size=64
    let mut pc: u32 = 0x8324A358;
    'dispatch: loop {
        match pc {
            0x8324A358 => {
    //   block [0x8324A358..0x8324A398)
	// 8324A358: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324A35C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324A360: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324A364: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 8324A368: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324A36C: 388B9014  addi r4, r11, -0x6fec
	ctx.r[4].s64 = ctx.r[11].s64 + -28652;
	// 8324A370: 386A77D8  addi r3, r10, 0x77d8
	ctx.r[3].s64 = ctx.r[10].s64 + 30680;
	// 8324A374: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324A378: 4AFE2B59  bl 0x8222ced0
	ctx.lr = 0x8324A37C;
	sub_8222CED0(ctx, base);
	// 8324A37C: 3D20832A  lis r9, -0x7cd6
	ctx.r[9].s64 = -2094399488;
	// 8324A380: 38697DF0  addi r3, r9, 0x7df0
	ctx.r[3].s64 = ctx.r[9].s64 + 32240;
	// 8324A384: 4BA5FB9D  bl 0x82ca9f20
	ctx.lr = 0x8324A388;
	sub_82CA9F20(ctx, base);
	// 8324A388: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324A38C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324A390: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324A394: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324A398(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324A398 size=64
    let mut pc: u32 = 0x8324A398;
    'dispatch: loop {
        match pc {
            0x8324A398 => {
    //   block [0x8324A398..0x8324A3D8)
	// 8324A398: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324A39C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324A3A0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324A3A4: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 8324A3A8: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324A3AC: 388B902C  addi r4, r11, -0x6fd4
	ctx.r[4].s64 = ctx.r[11].s64 + -28628;
	// 8324A3B0: 386A77DC  addi r3, r10, 0x77dc
	ctx.r[3].s64 = ctx.r[10].s64 + 30684;
	// 8324A3B4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324A3B8: 4AFE2B19  bl 0x8222ced0
	ctx.lr = 0x8324A3BC;
	sub_8222CED0(ctx, base);
	// 8324A3BC: 3D20832A  lis r9, -0x7cd6
	ctx.r[9].s64 = -2094399488;
	// 8324A3C0: 38697E00  addi r3, r9, 0x7e00
	ctx.r[3].s64 = ctx.r[9].s64 + 32256;
	// 8324A3C4: 4BA5FB5D  bl 0x82ca9f20
	ctx.lr = 0x8324A3C8;
	sub_82CA9F20(ctx, base);
	// 8324A3C8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324A3CC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324A3D0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324A3D4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324A3D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324A3D8 size=1368
    let mut pc: u32 = 0x8324A3D8;
    'dispatch: loop {
        match pc {
            0x8324A3D8 => {
    //   block [0x8324A3D8..0x8324A430)
	// 8324A3D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324A3DC: 4BA5F031  bl 0x82ca940c
	ctx.lr = 0x8324A3E0;
	sub_82CA93D0(ctx, base);
	// 8324A3E0: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324A3E4: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 8324A3E8: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324A3EC: 388B9174  addi r4, r11, -0x6e8c
	ctx.r[4].s64 = ctx.r[11].s64 + -28300;
	// 8324A3F0: 38610078  addi r3, r1, 0x78
	ctx.r[3].s64 = ctx.r[1].s64 + 120;
	// 8324A3F4: 4AFE2ADD  bl 0x8222ced0
	ctx.lr = 0x8324A3F8;
	sub_8222CED0(ctx, base);
	// 8324A3F8: 38810078  addi r4, r1, 0x78
	ctx.r[4].s64 = ctx.r[1].s64 + 120;
	// 8324A3FC: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8324A400: 4AFA4739  bl 0x821eeb38
	ctx.lr = 0x8324A404;
	sub_821EEB38(ctx, base);
	// 8324A404: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8324A408: 4B9B93E9  bl 0x82c037f0
	ctx.lr = 0x8324A40C;
	sub_82C037F0(ctx, base);
	// 8324A40C: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324A410: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 8324A414: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8324A418: 3BCA77E0  addi r30, r10, 0x77e0
	ctx.r[30].s64 = ctx.r[10].s64 + 30688;
	// 8324A41C: 916A77E0  stw r11, 0x77e0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(30688 as u32), ctx.r[11].u32 ) };
	// 8324A420: 4AF7C349  bl 0x821c6768
	ctx.lr = 0x8324A424;
	sub_821C6768(ctx, base);
	// 8324A424: 3D208349  lis r9, -0x7cb7
	ctx.r[9].s64 = -2092367872;
	// 8324A428: 3BA97088  addi r29, r9, 0x7088
	ctx.r[29].s64 = ctx.r[9].s64 + 28808;
	// 8324A42C: 7FA6EB78  mr r6, r29
	ctx.r[6].u64 = ctx.r[29].u64;
	pc = 0x8324A430; continue 'dispatch;
            }
            0x8324A430 => {
    //   block [0x8324A430..0x8324A460)
	// 8324A430: 7CE000A6  mfmsr r7
	ctx.r[7].u64 = ctx.msr;
	// 8324A434: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8324A438: 7D003028  lwarx r8, 0, r6
	// lwarx
	let ea = ctx.r[6].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[8].u64 = ctx.reserved.u32 as u64;
	// 8324A43C: 3908FFFF  addi r8, r8, -1
	ctx.r[8].s64 = ctx.r[8].s64 + -1;
	// 8324A440: 7D00312D  stwcx. r8, 0, r6
	// stwcx.
	let addr = ctx.r[6].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[8].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 8324A444: 7CE10164  mtmsrd r7, 1
	ctx.msr = (ctx.r[7].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8324A448: 4082FFE8  bne 0x8324a430
	if !ctx.cr[0].eq {
	pc = 0x8324A430; continue 'dispatch;
	}
	// 8324A44C: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 8324A450: 38610078  addi r3, r1, 0x78
	ctx.r[3].s64 = ctx.r[1].s64 + 120;
	// 8324A454: 93E10050  stw r31, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[31].u32 ) };
	// 8324A458: 4AF7C311  bl 0x821c6768
	ctx.lr = 0x8324A45C;
	sub_821C6768(ctx, base);
	// 8324A45C: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	pc = 0x8324A460; continue 'dispatch;
            }
            0x8324A460 => {
    //   block [0x8324A460..0x8324A4B8)
	// 8324A460: 7C8000A6  mfmsr r4
	ctx.r[4].u64 = ctx.msr;
	// 8324A464: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8324A468: 7CA01828  lwarx r5, 0, r3
	// lwarx
	let ea = ctx.r[3].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[5].u64 = ctx.reserved.u32 as u64;
	// 8324A46C: 38A5FFFF  addi r5, r5, -1
	ctx.r[5].s64 = ctx.r[5].s64 + -1;
	// 8324A470: 7CA0192D  stwcx. r5, 0, r3
	// stwcx.
	let addr = ctx.r[3].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[5].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 8324A474: 7C810164  mtmsrd r4, 1
	ctx.msr = (ctx.r[4].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8324A478: 4082FFE8  bne 0x8324a460
	if !ctx.cr[0].eq {
	pc = 0x8324A460; continue 'dispatch;
	}
	// 8324A47C: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 8324A480: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324A484: 388B9148  addi r4, r11, -0x6eb8
	ctx.r[4].s64 = ctx.r[11].s64 + -28344;
	// 8324A488: 38610078  addi r3, r1, 0x78
	ctx.r[3].s64 = ctx.r[1].s64 + 120;
	// 8324A48C: 4AFE2A45  bl 0x8222ced0
	ctx.lr = 0x8324A490;
	sub_8222CED0(ctx, base);
	// 8324A490: 38810078  addi r4, r1, 0x78
	ctx.r[4].s64 = ctx.r[1].s64 + 120;
	// 8324A494: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 8324A498: 4AFA46A1  bl 0x821eeb38
	ctx.lr = 0x8324A49C;
	sub_821EEB38(ctx, base);
	// 8324A49C: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 8324A4A0: 4B9B9351  bl 0x82c037f0
	ctx.lr = 0x8324A4A4;
	sub_82C037F0(ctx, base);
	// 8324A4A4: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 8324A4A8: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 8324A4AC: 917E0004  stw r11, 4(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 8324A4B0: 4AF7C2B9  bl 0x821c6768
	ctx.lr = 0x8324A4B4;
	sub_821C6768(ctx, base);
	// 8324A4B4: 7FA8EB78  mr r8, r29
	ctx.r[8].u64 = ctx.r[29].u64;
	pc = 0x8324A4B8; continue 'dispatch;
            }
            0x8324A4B8 => {
    //   block [0x8324A4B8..0x8324A4E4)
	// 8324A4B8: 7D2000A6  mfmsr r9
	ctx.r[9].u64 = ctx.msr;
	// 8324A4BC: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8324A4C0: 7D404028  lwarx r10, 0, r8
	// lwarx
	let ea = ctx.r[8].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 8324A4C4: 394AFFFF  addi r10, r10, -1
	ctx.r[10].s64 = ctx.r[10].s64 + -1;
	// 8324A4C8: 7D40412D  stwcx. r10, 0, r8
	// stwcx.
	let addr = ctx.r[8].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[10].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 8324A4CC: 7D210164  mtmsrd r9, 1
	ctx.msr = (ctx.r[9].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8324A4D0: 4082FFE8  bne 0x8324a4b8
	if !ctx.cr[0].eq {
	pc = 0x8324A4B8; continue 'dispatch;
	}
	// 8324A4D4: 93E10054  stw r31, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[31].u32 ) };
	// 8324A4D8: 38610078  addi r3, r1, 0x78
	ctx.r[3].s64 = ctx.r[1].s64 + 120;
	// 8324A4DC: 4AF7C28D  bl 0x821c6768
	ctx.lr = 0x8324A4E0;
	sub_821C6768(ctx, base);
	// 8324A4E0: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	pc = 0x8324A4E4; continue 'dispatch;
            }
            0x8324A4E4 => {
    //   block [0x8324A4E4..0x8324A53C)
	// 8324A4E4: 7CC000A6  mfmsr r6
	ctx.r[6].u64 = ctx.msr;
	// 8324A4E8: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8324A4EC: 7CE02828  lwarx r7, 0, r5
	// lwarx
	let ea = ctx.r[5].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[7].u64 = ctx.reserved.u32 as u64;
	// 8324A4F0: 38E7FFFF  addi r7, r7, -1
	ctx.r[7].s64 = ctx.r[7].s64 + -1;
	// 8324A4F4: 7CE0292D  stwcx. r7, 0, r5
	// stwcx.
	let addr = ctx.r[5].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[7].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 8324A4F8: 7CC10164  mtmsrd r6, 1
	ctx.msr = (ctx.r[6].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8324A4FC: 4082FFE8  bne 0x8324a4e4
	if !ctx.cr[0].eq {
	pc = 0x8324A4E4; continue 'dispatch;
	}
	// 8324A500: 3C80820B  lis r4, -0x7df5
	ctx.r[4].s64 = -2113208320;
	// 8324A504: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324A508: 3884911C  addi r4, r4, -0x6ee4
	ctx.r[4].s64 = ctx.r[4].s64 + -28388;
	// 8324A50C: 38610078  addi r3, r1, 0x78
	ctx.r[3].s64 = ctx.r[1].s64 + 120;
	// 8324A510: 4AFE29C1  bl 0x8222ced0
	ctx.lr = 0x8324A514;
	sub_8222CED0(ctx, base);
	// 8324A514: 38810078  addi r4, r1, 0x78
	ctx.r[4].s64 = ctx.r[1].s64 + 120;
	// 8324A518: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 8324A51C: 4AFA461D  bl 0x821eeb38
	ctx.lr = 0x8324A520;
	sub_821EEB38(ctx, base);
	// 8324A520: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 8324A524: 4B9B92CD  bl 0x82c037f0
	ctx.lr = 0x8324A528;
	sub_82C037F0(ctx, base);
	// 8324A528: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 8324A52C: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 8324A530: 917E0008  stw r11, 8(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 8324A534: 4AF7C235  bl 0x821c6768
	ctx.lr = 0x8324A538;
	sub_821C6768(ctx, base);
	// 8324A538: 7FAAEB78  mr r10, r29
	ctx.r[10].u64 = ctx.r[29].u64;
	pc = 0x8324A53C; continue 'dispatch;
            }
            0x8324A53C => {
    //   block [0x8324A53C..0x8324A568)
	// 8324A53C: 7D6000A6  mfmsr r11
	ctx.r[11].u64 = ctx.msr;
	// 8324A540: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8324A544: 7C605028  lwarx r3, 0, r10
	// lwarx
	let ea = ctx.r[10].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[3].u64 = ctx.reserved.u32 as u64;
	// 8324A548: 3863FFFF  addi r3, r3, -1
	ctx.r[3].s64 = ctx.r[3].s64 + -1;
	// 8324A54C: 7C60512D  stwcx. r3, 0, r10
	// stwcx.
	let addr = ctx.r[10].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[3].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 8324A550: 7D610164  mtmsrd r11, 1
	ctx.msr = (ctx.r[11].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8324A554: 4082FFE8  bne 0x8324a53c
	if !ctx.cr[0].eq {
	pc = 0x8324A53C; continue 'dispatch;
	}
	// 8324A558: 93E10058  stw r31, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[31].u32 ) };
	// 8324A55C: 38610078  addi r3, r1, 0x78
	ctx.r[3].s64 = ctx.r[1].s64 + 120;
	// 8324A560: 4AF7C209  bl 0x821c6768
	ctx.lr = 0x8324A564;
	sub_821C6768(ctx, base);
	// 8324A564: 7FA7EB78  mr r7, r29
	ctx.r[7].u64 = ctx.r[29].u64;
	pc = 0x8324A568; continue 'dispatch;
            }
            0x8324A568 => {
    //   block [0x8324A568..0x8324A5C0)
	// 8324A568: 7D0000A6  mfmsr r8
	ctx.r[8].u64 = ctx.msr;
	// 8324A56C: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8324A570: 7D203828  lwarx r9, 0, r7
	// lwarx
	let ea = ctx.r[7].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[9].u64 = ctx.reserved.u32 as u64;
	// 8324A574: 3929FFFF  addi r9, r9, -1
	ctx.r[9].s64 = ctx.r[9].s64 + -1;
	// 8324A578: 7D20392D  stwcx. r9, 0, r7
	// stwcx.
	let addr = ctx.r[7].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[9].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 8324A57C: 7D010164  mtmsrd r8, 1
	ctx.msr = (ctx.r[8].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8324A580: 4082FFE8  bne 0x8324a568
	if !ctx.cr[0].eq {
	pc = 0x8324A568; continue 'dispatch;
	}
	// 8324A584: 3CC0820B  lis r6, -0x7df5
	ctx.r[6].s64 = -2113208320;
	// 8324A588: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324A58C: 388690F0  addi r4, r6, -0x6f10
	ctx.r[4].s64 = ctx.r[6].s64 + -28432;
	// 8324A590: 38610078  addi r3, r1, 0x78
	ctx.r[3].s64 = ctx.r[1].s64 + 120;
	// 8324A594: 4AFE293D  bl 0x8222ced0
	ctx.lr = 0x8324A598;
	sub_8222CED0(ctx, base);
	// 8324A598: 38810078  addi r4, r1, 0x78
	ctx.r[4].s64 = ctx.r[1].s64 + 120;
	// 8324A59C: 3861005C  addi r3, r1, 0x5c
	ctx.r[3].s64 = ctx.r[1].s64 + 92;
	// 8324A5A0: 4AFA4599  bl 0x821eeb38
	ctx.lr = 0x8324A5A4;
	sub_821EEB38(ctx, base);
	// 8324A5A4: 3861005C  addi r3, r1, 0x5c
	ctx.r[3].s64 = ctx.r[1].s64 + 92;
	// 8324A5A8: 4B9B9249  bl 0x82c037f0
	ctx.lr = 0x8324A5AC;
	sub_82C037F0(ctx, base);
	// 8324A5AC: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 8324A5B0: 3861005C  addi r3, r1, 0x5c
	ctx.r[3].s64 = ctx.r[1].s64 + 92;
	// 8324A5B4: 917E000C  stw r11, 0xc(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 8324A5B8: 4AF7C1B1  bl 0x821c6768
	ctx.lr = 0x8324A5BC;
	sub_821C6768(ctx, base);
	// 8324A5BC: 7FABEB78  mr r11, r29
	ctx.r[11].u64 = ctx.r[29].u64;
	pc = 0x8324A5C0; continue 'dispatch;
            }
            0x8324A5C0 => {
    //   block [0x8324A5C0..0x8324A5EC)
	// 8324A5C0: 7C8000A6  mfmsr r4
	ctx.r[4].u64 = ctx.msr;
	// 8324A5C4: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8324A5C8: 7CA05828  lwarx r5, 0, r11
	// lwarx
	let ea = ctx.r[11].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[5].u64 = ctx.reserved.u32 as u64;
	// 8324A5CC: 38A5FFFF  addi r5, r5, -1
	ctx.r[5].s64 = ctx.r[5].s64 + -1;
	// 8324A5D0: 7CA0592D  stwcx. r5, 0, r11
	// stwcx.
	let addr = ctx.r[11].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[5].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 8324A5D4: 7C810164  mtmsrd r4, 1
	ctx.msr = (ctx.r[4].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8324A5D8: 4082FFE8  bne 0x8324a5c0
	if !ctx.cr[0].eq {
	pc = 0x8324A5C0; continue 'dispatch;
	}
	// 8324A5DC: 93E1005C  stw r31, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[31].u32 ) };
	// 8324A5E0: 38610078  addi r3, r1, 0x78
	ctx.r[3].s64 = ctx.r[1].s64 + 120;
	// 8324A5E4: 4AF7C185  bl 0x821c6768
	ctx.lr = 0x8324A5E8;
	sub_821C6768(ctx, base);
	// 8324A5E8: 7FA8EB78  mr r8, r29
	ctx.r[8].u64 = ctx.r[29].u64;
	pc = 0x8324A5EC; continue 'dispatch;
            }
            0x8324A5EC => {
    //   block [0x8324A5EC..0x8324A644)
	// 8324A5EC: 7D2000A6  mfmsr r9
	ctx.r[9].u64 = ctx.msr;
	// 8324A5F0: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8324A5F4: 7D404028  lwarx r10, 0, r8
	// lwarx
	let ea = ctx.r[8].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 8324A5F8: 394AFFFF  addi r10, r10, -1
	ctx.r[10].s64 = ctx.r[10].s64 + -1;
	// 8324A5FC: 7D40412D  stwcx. r10, 0, r8
	// stwcx.
	let addr = ctx.r[8].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[10].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 8324A600: 7D210164  mtmsrd r9, 1
	ctx.msr = (ctx.r[9].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8324A604: 4082FFE8  bne 0x8324a5ec
	if !ctx.cr[0].eq {
	pc = 0x8324A5EC; continue 'dispatch;
	}
	// 8324A608: 3CE0820B  lis r7, -0x7df5
	ctx.r[7].s64 = -2113208320;
	// 8324A60C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324A610: 388790C0  addi r4, r7, -0x6f40
	ctx.r[4].s64 = ctx.r[7].s64 + -28480;
	// 8324A614: 38610078  addi r3, r1, 0x78
	ctx.r[3].s64 = ctx.r[1].s64 + 120;
	// 8324A618: 4AFE28B9  bl 0x8222ced0
	ctx.lr = 0x8324A61C;
	sub_8222CED0(ctx, base);
	// 8324A61C: 38810078  addi r4, r1, 0x78
	ctx.r[4].s64 = ctx.r[1].s64 + 120;
	// 8324A620: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 8324A624: 4AFA4515  bl 0x821eeb38
	ctx.lr = 0x8324A628;
	sub_821EEB38(ctx, base);
	// 8324A628: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 8324A62C: 4B9B91C5  bl 0x82c037f0
	ctx.lr = 0x8324A630;
	sub_82C037F0(ctx, base);
	// 8324A630: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 8324A634: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 8324A638: 917E0010  stw r11, 0x10(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 8324A63C: 4AF7C12D  bl 0x821c6768
	ctx.lr = 0x8324A640;
	sub_821C6768(ctx, base);
	// 8324A640: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	pc = 0x8324A644; continue 'dispatch;
            }
            0x8324A644 => {
    //   block [0x8324A644..0x8324A670)
	// 8324A644: 7CA000A6  mfmsr r5
	ctx.r[5].u64 = ctx.msr;
	// 8324A648: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8324A64C: 7CC02028  lwarx r6, 0, r4
	// lwarx
	let ea = ctx.r[4].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[6].u64 = ctx.reserved.u32 as u64;
	// 8324A650: 38C6FFFF  addi r6, r6, -1
	ctx.r[6].s64 = ctx.r[6].s64 + -1;
	// 8324A654: 7CC0212D  stwcx. r6, 0, r4
	// stwcx.
	let addr = ctx.r[4].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[6].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 8324A658: 7CA10164  mtmsrd r5, 1
	ctx.msr = (ctx.r[5].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8324A65C: 4082FFE8  bne 0x8324a644
	if !ctx.cr[0].eq {
	pc = 0x8324A644; continue 'dispatch;
	}
	// 8324A660: 93E10060  stw r31, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[31].u32 ) };
	// 8324A664: 38610078  addi r3, r1, 0x78
	ctx.r[3].s64 = ctx.r[1].s64 + 120;
	// 8324A668: 4AF7C101  bl 0x821c6768
	ctx.lr = 0x8324A66C;
	sub_821C6768(ctx, base);
	// 8324A66C: 7FAAEB78  mr r10, r29
	ctx.r[10].u64 = ctx.r[29].u64;
	pc = 0x8324A670; continue 'dispatch;
            }
            0x8324A670 => {
    //   block [0x8324A670..0x8324A6C8)
	// 8324A670: 7D6000A6  mfmsr r11
	ctx.r[11].u64 = ctx.msr;
	// 8324A674: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8324A678: 7C605028  lwarx r3, 0, r10
	// lwarx
	let ea = ctx.r[10].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[3].u64 = ctx.reserved.u32 as u64;
	// 8324A67C: 3863FFFF  addi r3, r3, -1
	ctx.r[3].s64 = ctx.r[3].s64 + -1;
	// 8324A680: 7C60512D  stwcx. r3, 0, r10
	// stwcx.
	let addr = ctx.r[10].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[3].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 8324A684: 7D610164  mtmsrd r11, 1
	ctx.msr = (ctx.r[11].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8324A688: 4082FFE8  bne 0x8324a670
	if !ctx.cr[0].eq {
	pc = 0x8324A670; continue 'dispatch;
	}
	// 8324A68C: 3D20820B  lis r9, -0x7df5
	ctx.r[9].s64 = -2113208320;
	// 8324A690: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324A694: 38899094  addi r4, r9, -0x6f6c
	ctx.r[4].s64 = ctx.r[9].s64 + -28524;
	// 8324A698: 38610078  addi r3, r1, 0x78
	ctx.r[3].s64 = ctx.r[1].s64 + 120;
	// 8324A69C: 4AFE2835  bl 0x8222ced0
	ctx.lr = 0x8324A6A0;
	sub_8222CED0(ctx, base);
	// 8324A6A0: 38810078  addi r4, r1, 0x78
	ctx.r[4].s64 = ctx.r[1].s64 + 120;
	// 8324A6A4: 38610064  addi r3, r1, 0x64
	ctx.r[3].s64 = ctx.r[1].s64 + 100;
	// 8324A6A8: 4AFA4491  bl 0x821eeb38
	ctx.lr = 0x8324A6AC;
	sub_821EEB38(ctx, base);
	// 8324A6AC: 38610064  addi r3, r1, 0x64
	ctx.r[3].s64 = ctx.r[1].s64 + 100;
	// 8324A6B0: 4B9B9141  bl 0x82c037f0
	ctx.lr = 0x8324A6B4;
	sub_82C037F0(ctx, base);
	// 8324A6B4: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 8324A6B8: 38610064  addi r3, r1, 0x64
	ctx.r[3].s64 = ctx.r[1].s64 + 100;
	// 8324A6BC: 917E0014  stw r11, 0x14(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(20 as u32), ctx.r[11].u32 ) };
	// 8324A6C0: 4AF7C0A9  bl 0x821c6768
	ctx.lr = 0x8324A6C4;
	sub_821C6768(ctx, base);
	// 8324A6C4: 7FA6EB78  mr r6, r29
	ctx.r[6].u64 = ctx.r[29].u64;
	pc = 0x8324A6C8; continue 'dispatch;
            }
            0x8324A6C8 => {
    //   block [0x8324A6C8..0x8324A6F4)
	// 8324A6C8: 7CE000A6  mfmsr r7
	ctx.r[7].u64 = ctx.msr;
	// 8324A6CC: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8324A6D0: 7D003028  lwarx r8, 0, r6
	// lwarx
	let ea = ctx.r[6].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[8].u64 = ctx.reserved.u32 as u64;
	// 8324A6D4: 3908FFFF  addi r8, r8, -1
	ctx.r[8].s64 = ctx.r[8].s64 + -1;
	// 8324A6D8: 7D00312D  stwcx. r8, 0, r6
	// stwcx.
	let addr = ctx.r[6].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[8].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 8324A6DC: 7CE10164  mtmsrd r7, 1
	ctx.msr = (ctx.r[7].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8324A6E0: 4082FFE8  bne 0x8324a6c8
	if !ctx.cr[0].eq {
	pc = 0x8324A6C8; continue 'dispatch;
	}
	// 8324A6E4: 93E10064  stw r31, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[31].u32 ) };
	// 8324A6E8: 38610078  addi r3, r1, 0x78
	ctx.r[3].s64 = ctx.r[1].s64 + 120;
	// 8324A6EC: 4AF7C07D  bl 0x821c6768
	ctx.lr = 0x8324A6F0;
	sub_821C6768(ctx, base);
	// 8324A6F0: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	pc = 0x8324A6F4; continue 'dispatch;
            }
            0x8324A6F4 => {
    //   block [0x8324A6F4..0x8324A74C)
	// 8324A6F4: 7C8000A6  mfmsr r4
	ctx.r[4].u64 = ctx.msr;
	// 8324A6F8: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8324A6FC: 7CA01828  lwarx r5, 0, r3
	// lwarx
	let ea = ctx.r[3].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[5].u64 = ctx.reserved.u32 as u64;
	// 8324A700: 38A5FFFF  addi r5, r5, -1
	ctx.r[5].s64 = ctx.r[5].s64 + -1;
	// 8324A704: 7CA0192D  stwcx. r5, 0, r3
	// stwcx.
	let addr = ctx.r[3].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[5].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 8324A708: 7C810164  mtmsrd r4, 1
	ctx.msr = (ctx.r[4].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8324A70C: 4082FFE8  bne 0x8324a6f4
	if !ctx.cr[0].eq {
	pc = 0x8324A6F4; continue 'dispatch;
	}
	// 8324A710: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 8324A714: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324A718: 388B9068  addi r4, r11, -0x6f98
	ctx.r[4].s64 = ctx.r[11].s64 + -28568;
	// 8324A71C: 38610078  addi r3, r1, 0x78
	ctx.r[3].s64 = ctx.r[1].s64 + 120;
	// 8324A720: 4AFE27B1  bl 0x8222ced0
	ctx.lr = 0x8324A724;
	sub_8222CED0(ctx, base);
	// 8324A724: 38810078  addi r4, r1, 0x78
	ctx.r[4].s64 = ctx.r[1].s64 + 120;
	// 8324A728: 38610068  addi r3, r1, 0x68
	ctx.r[3].s64 = ctx.r[1].s64 + 104;
	// 8324A72C: 4AFA440D  bl 0x821eeb38
	ctx.lr = 0x8324A730;
	sub_821EEB38(ctx, base);
	// 8324A730: 38610068  addi r3, r1, 0x68
	ctx.r[3].s64 = ctx.r[1].s64 + 104;
	// 8324A734: 4B9B90BD  bl 0x82c037f0
	ctx.lr = 0x8324A738;
	sub_82C037F0(ctx, base);
	// 8324A738: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 8324A73C: 38610068  addi r3, r1, 0x68
	ctx.r[3].s64 = ctx.r[1].s64 + 104;
	// 8324A740: 917E0018  stw r11, 0x18(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(24 as u32), ctx.r[11].u32 ) };
	// 8324A744: 4AF7C025  bl 0x821c6768
	ctx.lr = 0x8324A748;
	sub_821C6768(ctx, base);
	// 8324A748: 7FA8EB78  mr r8, r29
	ctx.r[8].u64 = ctx.r[29].u64;
	pc = 0x8324A74C; continue 'dispatch;
            }
            0x8324A74C => {
    //   block [0x8324A74C..0x8324A778)
	// 8324A74C: 7D2000A6  mfmsr r9
	ctx.r[9].u64 = ctx.msr;
	// 8324A750: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8324A754: 7D404028  lwarx r10, 0, r8
	// lwarx
	let ea = ctx.r[8].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 8324A758: 394AFFFF  addi r10, r10, -1
	ctx.r[10].s64 = ctx.r[10].s64 + -1;
	// 8324A75C: 7D40412D  stwcx. r10, 0, r8
	// stwcx.
	let addr = ctx.r[8].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[10].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 8324A760: 7D210164  mtmsrd r9, 1
	ctx.msr = (ctx.r[9].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8324A764: 4082FFE8  bne 0x8324a74c
	if !ctx.cr[0].eq {
	pc = 0x8324A74C; continue 'dispatch;
	}
	// 8324A768: 93E10068  stw r31, 0x68(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(104 as u32), ctx.r[31].u32 ) };
	// 8324A76C: 38610078  addi r3, r1, 0x78
	ctx.r[3].s64 = ctx.r[1].s64 + 120;
	// 8324A770: 4AF7BFF9  bl 0x821c6768
	ctx.lr = 0x8324A774;
	sub_821C6768(ctx, base);
	// 8324A774: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	pc = 0x8324A778; continue 'dispatch;
            }
            0x8324A778 => {
    //   block [0x8324A778..0x8324A7D0)
	// 8324A778: 7CC000A6  mfmsr r6
	ctx.r[6].u64 = ctx.msr;
	// 8324A77C: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8324A780: 7CE02828  lwarx r7, 0, r5
	// lwarx
	let ea = ctx.r[5].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[7].u64 = ctx.reserved.u32 as u64;
	// 8324A784: 38E7FFFF  addi r7, r7, -1
	ctx.r[7].s64 = ctx.r[7].s64 + -1;
	// 8324A788: 7CE0292D  stwcx. r7, 0, r5
	// stwcx.
	let addr = ctx.r[5].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[7].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 8324A78C: 7CC10164  mtmsrd r6, 1
	ctx.msr = (ctx.r[6].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8324A790: 4082FFE8  bne 0x8324a778
	if !ctx.cr[0].eq {
	pc = 0x8324A778; continue 'dispatch;
	}
	// 8324A794: 3C80820B  lis r4, -0x7df5
	ctx.r[4].s64 = -2113208320;
	// 8324A798: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324A79C: 38849038  addi r4, r4, -0x6fc8
	ctx.r[4].s64 = ctx.r[4].s64 + -28616;
	// 8324A7A0: 38610078  addi r3, r1, 0x78
	ctx.r[3].s64 = ctx.r[1].s64 + 120;
	// 8324A7A4: 4AFE272D  bl 0x8222ced0
	ctx.lr = 0x8324A7A8;
	sub_8222CED0(ctx, base);
	// 8324A7A8: 38810078  addi r4, r1, 0x78
	ctx.r[4].s64 = ctx.r[1].s64 + 120;
	// 8324A7AC: 3861006C  addi r3, r1, 0x6c
	ctx.r[3].s64 = ctx.r[1].s64 + 108;
	// 8324A7B0: 4AFA4389  bl 0x821eeb38
	ctx.lr = 0x8324A7B4;
	sub_821EEB38(ctx, base);
	// 8324A7B4: 3861006C  addi r3, r1, 0x6c
	ctx.r[3].s64 = ctx.r[1].s64 + 108;
	// 8324A7B8: 4B9B9039  bl 0x82c037f0
	ctx.lr = 0x8324A7BC;
	sub_82C037F0(ctx, base);
	// 8324A7BC: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 8324A7C0: 3861006C  addi r3, r1, 0x6c
	ctx.r[3].s64 = ctx.r[1].s64 + 108;
	// 8324A7C4: 917E001C  stw r11, 0x1c(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(28 as u32), ctx.r[11].u32 ) };
	// 8324A7C8: 4AF7BFA1  bl 0x821c6768
	ctx.lr = 0x8324A7CC;
	sub_821C6768(ctx, base);
	// 8324A7CC: 7FAAEB78  mr r10, r29
	ctx.r[10].u64 = ctx.r[29].u64;
	pc = 0x8324A7D0; continue 'dispatch;
            }
            0x8324A7D0 => {
    //   block [0x8324A7D0..0x8324A7FC)
	// 8324A7D0: 7D6000A6  mfmsr r11
	ctx.r[11].u64 = ctx.msr;
	// 8324A7D4: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8324A7D8: 7C605028  lwarx r3, 0, r10
	// lwarx
	let ea = ctx.r[10].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[3].u64 = ctx.reserved.u32 as u64;
	// 8324A7DC: 3863FFFF  addi r3, r3, -1
	ctx.r[3].s64 = ctx.r[3].s64 + -1;
	// 8324A7E0: 7C60512D  stwcx. r3, 0, r10
	// stwcx.
	let addr = ctx.r[10].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[3].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 8324A7E4: 7D610164  mtmsrd r11, 1
	ctx.msr = (ctx.r[11].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8324A7E8: 4082FFE8  bne 0x8324a7d0
	if !ctx.cr[0].eq {
	pc = 0x8324A7D0; continue 'dispatch;
	}
	// 8324A7EC: 93E1006C  stw r31, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[31].u32 ) };
	// 8324A7F0: 38610078  addi r3, r1, 0x78
	ctx.r[3].s64 = ctx.r[1].s64 + 120;
	// 8324A7F4: 4AF7BF75  bl 0x821c6768
	ctx.lr = 0x8324A7F8;
	sub_821C6768(ctx, base);
	// 8324A7F8: 7FA7EB78  mr r7, r29
	ctx.r[7].u64 = ctx.r[29].u64;
	pc = 0x8324A7FC; continue 'dispatch;
            }
            0x8324A7FC => {
    //   block [0x8324A7FC..0x8324A854)
	// 8324A7FC: 7D0000A6  mfmsr r8
	ctx.r[8].u64 = ctx.msr;
	// 8324A800: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8324A804: 7D203828  lwarx r9, 0, r7
	// lwarx
	let ea = ctx.r[7].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[9].u64 = ctx.reserved.u32 as u64;
	// 8324A808: 3929FFFF  addi r9, r9, -1
	ctx.r[9].s64 = ctx.r[9].s64 + -1;
	// 8324A80C: 7D20392D  stwcx. r9, 0, r7
	// stwcx.
	let addr = ctx.r[7].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[9].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 8324A810: 7D010164  mtmsrd r8, 1
	ctx.msr = (ctx.r[8].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8324A814: 4082FFE8  bne 0x8324a7fc
	if !ctx.cr[0].eq {
	pc = 0x8324A7FC; continue 'dispatch;
	}
	// 8324A818: 3CC0820B  lis r6, -0x7df5
	ctx.r[6].s64 = -2113208320;
	// 8324A81C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324A820: 3886819C  addi r4, r6, -0x7e64
	ctx.r[4].s64 = ctx.r[6].s64 + -32356;
	// 8324A824: 38610078  addi r3, r1, 0x78
	ctx.r[3].s64 = ctx.r[1].s64 + 120;
	// 8324A828: 4AFE26A9  bl 0x8222ced0
	ctx.lr = 0x8324A82C;
	sub_8222CED0(ctx, base);
	// 8324A82C: 38810078  addi r4, r1, 0x78
	ctx.r[4].s64 = ctx.r[1].s64 + 120;
	// 8324A830: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 8324A834: 4AFA4305  bl 0x821eeb38
	ctx.lr = 0x8324A838;
	sub_821EEB38(ctx, base);
	// 8324A838: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 8324A83C: 4B9B8FB5  bl 0x82c037f0
	ctx.lr = 0x8324A840;
	sub_82C037F0(ctx, base);
	// 8324A840: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 8324A844: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 8324A848: 917E0020  stw r11, 0x20(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 8324A84C: 4AF7BF1D  bl 0x821c6768
	ctx.lr = 0x8324A850;
	sub_821C6768(ctx, base);
	// 8324A850: 7FABEB78  mr r11, r29
	ctx.r[11].u64 = ctx.r[29].u64;
	pc = 0x8324A854; continue 'dispatch;
            }
            0x8324A854 => {
    //   block [0x8324A854..0x8324A880)
	// 8324A854: 7C8000A6  mfmsr r4
	ctx.r[4].u64 = ctx.msr;
	// 8324A858: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8324A85C: 7CA05828  lwarx r5, 0, r11
	// lwarx
	let ea = ctx.r[11].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[5].u64 = ctx.reserved.u32 as u64;
	// 8324A860: 38A5FFFF  addi r5, r5, -1
	ctx.r[5].s64 = ctx.r[5].s64 + -1;
	// 8324A864: 7CA0592D  stwcx. r5, 0, r11
	// stwcx.
	let addr = ctx.r[11].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[5].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 8324A868: 7C810164  mtmsrd r4, 1
	ctx.msr = (ctx.r[4].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8324A86C: 4082FFE8  bne 0x8324a854
	if !ctx.cr[0].eq {
	pc = 0x8324A854; continue 'dispatch;
	}
	// 8324A870: 93E10070  stw r31, 0x70(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(112 as u32), ctx.r[31].u32 ) };
	// 8324A874: 38610078  addi r3, r1, 0x78
	ctx.r[3].s64 = ctx.r[1].s64 + 120;
	// 8324A878: 4AF7BEF1  bl 0x821c6768
	ctx.lr = 0x8324A87C;
	sub_821C6768(ctx, base);
	// 8324A87C: 7FA8EB78  mr r8, r29
	ctx.r[8].u64 = ctx.r[29].u64;
	pc = 0x8324A880; continue 'dispatch;
            }
            0x8324A880 => {
    //   block [0x8324A880..0x8324A8D8)
	// 8324A880: 7D2000A6  mfmsr r9
	ctx.r[9].u64 = ctx.msr;
	// 8324A884: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8324A888: 7D404028  lwarx r10, 0, r8
	// lwarx
	let ea = ctx.r[8].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 8324A88C: 394AFFFF  addi r10, r10, -1
	ctx.r[10].s64 = ctx.r[10].s64 + -1;
	// 8324A890: 7D40412D  stwcx. r10, 0, r8
	// stwcx.
	let addr = ctx.r[8].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[10].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 8324A894: 7D210164  mtmsrd r9, 1
	ctx.msr = (ctx.r[9].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8324A898: 4082FFE8  bne 0x8324a880
	if !ctx.cr[0].eq {
	pc = 0x8324A880; continue 'dispatch;
	}
	// 8324A89C: 3CE0820B  lis r7, -0x7df5
	ctx.r[7].s64 = -2113208320;
	// 8324A8A0: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324A8A4: 3887824C  addi r4, r7, -0x7db4
	ctx.r[4].s64 = ctx.r[7].s64 + -32180;
	// 8324A8A8: 38610078  addi r3, r1, 0x78
	ctx.r[3].s64 = ctx.r[1].s64 + 120;
	// 8324A8AC: 4AFE2625  bl 0x8222ced0
	ctx.lr = 0x8324A8B0;
	sub_8222CED0(ctx, base);
	// 8324A8B0: 38810078  addi r4, r1, 0x78
	ctx.r[4].s64 = ctx.r[1].s64 + 120;
	// 8324A8B4: 38610074  addi r3, r1, 0x74
	ctx.r[3].s64 = ctx.r[1].s64 + 116;
	// 8324A8B8: 4AFA4281  bl 0x821eeb38
	ctx.lr = 0x8324A8BC;
	sub_821EEB38(ctx, base);
	// 8324A8BC: 38610074  addi r3, r1, 0x74
	ctx.r[3].s64 = ctx.r[1].s64 + 116;
	// 8324A8C0: 4B9B8F31  bl 0x82c037f0
	ctx.lr = 0x8324A8C4;
	sub_82C037F0(ctx, base);
	// 8324A8C4: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 8324A8C8: 38610074  addi r3, r1, 0x74
	ctx.r[3].s64 = ctx.r[1].s64 + 116;
	// 8324A8CC: 917E0024  stw r11, 0x24(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(36 as u32), ctx.r[11].u32 ) };
	// 8324A8D0: 4AF7BE99  bl 0x821c6768
	ctx.lr = 0x8324A8D4;
	sub_821C6768(ctx, base);
	// 8324A8D4: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	pc = 0x8324A8D8; continue 'dispatch;
            }
            0x8324A8D8 => {
    //   block [0x8324A8D8..0x8324A900)
	// 8324A8D8: 7CA000A6  mfmsr r5
	ctx.r[5].u64 = ctx.msr;
	// 8324A8DC: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8324A8E0: 7CC02028  lwarx r6, 0, r4
	// lwarx
	let ea = ctx.r[4].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[6].u64 = ctx.reserved.u32 as u64;
	// 8324A8E4: 38C6FFFF  addi r6, r6, -1
	ctx.r[6].s64 = ctx.r[6].s64 + -1;
	// 8324A8E8: 7CC0212D  stwcx. r6, 0, r4
	// stwcx.
	let addr = ctx.r[4].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[6].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 8324A8EC: 7CA10164  mtmsrd r5, 1
	ctx.msr = (ctx.r[5].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8324A8F0: 4082FFE8  bne 0x8324a8d8
	if !ctx.cr[0].eq {
	pc = 0x8324A8D8; continue 'dispatch;
	}
	// 8324A8F4: 93E10074  stw r31, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[31].u32 ) };
	// 8324A8F8: 38610078  addi r3, r1, 0x78
	ctx.r[3].s64 = ctx.r[1].s64 + 120;
	// 8324A8FC: 4AF7BE6D  bl 0x821c6768
	ctx.lr = 0x8324A900;
	sub_821C6768(ctx, base);
	pc = 0x8324A900; continue 'dispatch;
            }
            0x8324A900 => {
    //   block [0x8324A900..0x8324A930)
	// 8324A900: 7D6000A6  mfmsr r11
	ctx.r[11].u64 = ctx.msr;
	// 8324A904: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8324A908: 7C60E828  lwarx r3, 0, r29
	// lwarx
	let ea = ctx.r[29].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[3].u64 = ctx.reserved.u32 as u64;
	// 8324A90C: 3863FFFF  addi r3, r3, -1
	ctx.r[3].s64 = ctx.r[3].s64 + -1;
	// 8324A910: 7C60E92D  stwcx. r3, 0, r29
	// stwcx.
	let addr = ctx.r[29].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[3].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 8324A914: 7D610164  mtmsrd r11, 1
	ctx.msr = (ctx.r[11].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8324A918: 4082FFE8  bne 0x8324a900
	if !ctx.cr[0].eq {
	pc = 0x8324A900; continue 'dispatch;
	}
	// 8324A91C: 3D40832A  lis r10, -0x7cd6
	ctx.r[10].s64 = -2094399488;
	// 8324A920: 386A7E10  addi r3, r10, 0x7e10
	ctx.r[3].s64 = ctx.r[10].s64 + 32272;
	// 8324A924: 4BA5F5FD  bl 0x82ca9f20
	ctx.lr = 0x8324A928;
	sub_82CA9F20(ctx, base);
	// 8324A928: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 8324A92C: 4BA5EB30  b 0x82ca945c
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324A930(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324A930 size=192
    let mut pc: u32 = 0x8324A930;
    'dispatch: loop {
        match pc {
            0x8324A930 => {
    //   block [0x8324A930..0x8324A988)
	// 8324A930: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324A934: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324A938: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8324A93C: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324A940: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 8324A944: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324A948: 388B919C  addi r4, r11, -0x6e64
	ctx.r[4].s64 = ctx.r[11].s64 + -28260;
	// 8324A94C: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 8324A950: 4AFE2581  bl 0x8222ced0
	ctx.lr = 0x8324A954;
	sub_8222CED0(ctx, base);
	// 8324A954: 38810054  addi r4, r1, 0x54
	ctx.r[4].s64 = ctx.r[1].s64 + 84;
	// 8324A958: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8324A95C: 4AFA41DD  bl 0x821eeb38
	ctx.lr = 0x8324A960;
	sub_821EEB38(ctx, base);
	// 8324A960: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8324A964: 4B9B8E8D  bl 0x82c037f0
	ctx.lr = 0x8324A968;
	sub_82C037F0(ctx, base);
	// 8324A968: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324A96C: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 8324A970: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8324A974: 916A7808  stw r11, 0x7808(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(30728 as u32), ctx.r[11].u32 ) };
	// 8324A978: 4AF7BDF1  bl 0x821c6768
	ctx.lr = 0x8324A97C;
	sub_821C6768(ctx, base);
	// 8324A97C: 3D208349  lis r9, -0x7cb7
	ctx.r[9].s64 = -2092367872;
	// 8324A980: 3BE97088  addi r31, r9, 0x7088
	ctx.r[31].s64 = ctx.r[9].s64 + 28808;
	// 8324A984: 7FE6FB78  mr r6, r31
	ctx.r[6].u64 = ctx.r[31].u64;
	pc = 0x8324A988; continue 'dispatch;
            }
            0x8324A988 => {
    //   block [0x8324A988..0x8324A9B4)
	// 8324A988: 7CE000A6  mfmsr r7
	ctx.r[7].u64 = ctx.msr;
	// 8324A98C: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8324A990: 7D003028  lwarx r8, 0, r6
	// lwarx
	let ea = ctx.r[6].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[8].u64 = ctx.reserved.u32 as u64;
	// 8324A994: 3908FFFF  addi r8, r8, -1
	ctx.r[8].s64 = ctx.r[8].s64 + -1;
	// 8324A998: 7D00312D  stwcx. r8, 0, r6
	// stwcx.
	let addr = ctx.r[6].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[8].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 8324A99C: 7CE10164  mtmsrd r7, 1
	ctx.msr = (ctx.r[7].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8324A9A0: 4082FFE8  bne 0x8324a988
	if !ctx.cr[0].eq {
	pc = 0x8324A988; continue 'dispatch;
	}
	// 8324A9A4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8324A9A8: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 8324A9AC: 90A10050  stw r5, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[5].u32 ) };
	// 8324A9B0: 4AF7BDB9  bl 0x821c6768
	ctx.lr = 0x8324A9B4;
	sub_821C6768(ctx, base);
	pc = 0x8324A9B4; continue 'dispatch;
            }
            0x8324A9B4 => {
    //   block [0x8324A9B4..0x8324A9F0)
	// 8324A9B4: 7C6000A6  mfmsr r3
	ctx.r[3].u64 = ctx.msr;
	// 8324A9B8: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8324A9BC: 7C80F828  lwarx r4, 0, r31
	// lwarx
	let ea = ctx.r[31].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[4].u64 = ctx.reserved.u32 as u64;
	// 8324A9C0: 3884FFFF  addi r4, r4, -1
	ctx.r[4].s64 = ctx.r[4].s64 + -1;
	// 8324A9C4: 7C80F92D  stwcx. r4, 0, r31
	// stwcx.
	let addr = ctx.r[31].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[4].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 8324A9C8: 7C610164  mtmsrd r3, 1
	ctx.msr = (ctx.r[3].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8324A9CC: 4082FFE8  bne 0x8324a9b4
	if !ctx.cr[0].eq {
	pc = 0x8324A9B4; continue 'dispatch;
	}
	// 8324A9D0: 3D60832A  lis r11, -0x7cd6
	ctx.r[11].s64 = -2094399488;
	// 8324A9D4: 386B7E18  addi r3, r11, 0x7e18
	ctx.r[3].s64 = ctx.r[11].s64 + 32280;
	// 8324A9D8: 4BA5F549  bl 0x82ca9f20
	ctx.lr = 0x8324A9DC;
	sub_82CA9F20(ctx, base);
	// 8324A9DC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8324A9E0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324A9E4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324A9E8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8324A9EC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324A9F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324A9F0 size=192
    let mut pc: u32 = 0x8324A9F0;
    'dispatch: loop {
        match pc {
            0x8324A9F0 => {
    //   block [0x8324A9F0..0x8324AA48)
	// 8324A9F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324A9F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324A9F8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8324A9FC: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324AA00: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 8324AA04: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324AA08: 388B91C4  addi r4, r11, -0x6e3c
	ctx.r[4].s64 = ctx.r[11].s64 + -28220;
	// 8324AA0C: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 8324AA10: 4AFE24C1  bl 0x8222ced0
	ctx.lr = 0x8324AA14;
	sub_8222CED0(ctx, base);
	// 8324AA14: 38810054  addi r4, r1, 0x54
	ctx.r[4].s64 = ctx.r[1].s64 + 84;
	// 8324AA18: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8324AA1C: 4AFA411D  bl 0x821eeb38
	ctx.lr = 0x8324AA20;
	sub_821EEB38(ctx, base);
	// 8324AA20: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8324AA24: 4B9B8DCD  bl 0x82c037f0
	ctx.lr = 0x8324AA28;
	sub_82C037F0(ctx, base);
	// 8324AA28: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324AA2C: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 8324AA30: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8324AA34: 916A780C  stw r11, 0x780c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(30732 as u32), ctx.r[11].u32 ) };
	// 8324AA38: 4AF7BD31  bl 0x821c6768
	ctx.lr = 0x8324AA3C;
	sub_821C6768(ctx, base);
	// 8324AA3C: 3D208349  lis r9, -0x7cb7
	ctx.r[9].s64 = -2092367872;
	// 8324AA40: 3BE97088  addi r31, r9, 0x7088
	ctx.r[31].s64 = ctx.r[9].s64 + 28808;
	// 8324AA44: 7FE6FB78  mr r6, r31
	ctx.r[6].u64 = ctx.r[31].u64;
	pc = 0x8324AA48; continue 'dispatch;
            }
            0x8324AA48 => {
    //   block [0x8324AA48..0x8324AA74)
	// 8324AA48: 7CE000A6  mfmsr r7
	ctx.r[7].u64 = ctx.msr;
	// 8324AA4C: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8324AA50: 7D003028  lwarx r8, 0, r6
	// lwarx
	let ea = ctx.r[6].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[8].u64 = ctx.reserved.u32 as u64;
	// 8324AA54: 3908FFFF  addi r8, r8, -1
	ctx.r[8].s64 = ctx.r[8].s64 + -1;
	// 8324AA58: 7D00312D  stwcx. r8, 0, r6
	// stwcx.
	let addr = ctx.r[6].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[8].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 8324AA5C: 7CE10164  mtmsrd r7, 1
	ctx.msr = (ctx.r[7].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8324AA60: 4082FFE8  bne 0x8324aa48
	if !ctx.cr[0].eq {
	pc = 0x8324AA48; continue 'dispatch;
	}
	// 8324AA64: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8324AA68: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 8324AA6C: 90A10050  stw r5, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[5].u32 ) };
	// 8324AA70: 4AF7BCF9  bl 0x821c6768
	ctx.lr = 0x8324AA74;
	sub_821C6768(ctx, base);
	pc = 0x8324AA74; continue 'dispatch;
            }
            0x8324AA74 => {
    //   block [0x8324AA74..0x8324AAB0)
	// 8324AA74: 7C6000A6  mfmsr r3
	ctx.r[3].u64 = ctx.msr;
	// 8324AA78: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8324AA7C: 7C80F828  lwarx r4, 0, r31
	// lwarx
	let ea = ctx.r[31].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[4].u64 = ctx.reserved.u32 as u64;
	// 8324AA80: 3884FFFF  addi r4, r4, -1
	ctx.r[4].s64 = ctx.r[4].s64 + -1;
	// 8324AA84: 7C80F92D  stwcx. r4, 0, r31
	// stwcx.
	let addr = ctx.r[31].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[4].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 8324AA88: 7C610164  mtmsrd r3, 1
	ctx.msr = (ctx.r[3].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8324AA8C: 4082FFE8  bne 0x8324aa74
	if !ctx.cr[0].eq {
	pc = 0x8324AA74; continue 'dispatch;
	}
	// 8324AA90: 3D60832A  lis r11, -0x7cd6
	ctx.r[11].s64 = -2094399488;
	// 8324AA94: 386B7E20  addi r3, r11, 0x7e20
	ctx.r[3].s64 = ctx.r[11].s64 + 32288;
	// 8324AA98: 4BA5F489  bl 0x82ca9f20
	ctx.lr = 0x8324AA9C;
	sub_82CA9F20(ctx, base);
	// 8324AA9C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8324AAA0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324AAA4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324AAA8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8324AAAC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324AAB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324AAB0 size=52
    let mut pc: u32 = 0x8324AAB0;
    'dispatch: loop {
        match pc {
            0x8324AAB0 => {
    //   block [0x8324AAB0..0x8324AAE4)
	// 8324AAB0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324AAB4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324AAB8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324AABC: 3D60834F  lis r11, -0x7cb1
	ctx.r[11].s64 = -2091974656;
	// 8324AAC0: 386B76C0  addi r3, r11, 0x76c0
	ctx.r[3].s64 = ctx.r[11].s64 + 30400;
	// 8324AAC4: 4806F1C1  bl 0x832b9c84
	ctx.lr = 0x8324AAC8;
	// extern call 0x832B9C84 → crate::xboxkrnl::RtlInitializeCriticalSection
	crate::xboxkrnl::RtlInitializeCriticalSection(ctx, base);
	// 8324AAC8: 3D40832A  lis r10, -0x7cd6
	ctx.r[10].s64 = -2094399488;
	// 8324AACC: 386A7E40  addi r3, r10, 0x7e40
	ctx.r[3].s64 = ctx.r[10].s64 + 32320;
	// 8324AAD0: 4BA5F451  bl 0x82ca9f20
	ctx.lr = 0x8324AAD4;
	sub_82CA9F20(ctx, base);
	// 8324AAD4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324AAD8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324AADC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324AAE0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324AAE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8324AAE8 size=12
    let mut pc: u32 = 0x8324AAE8;
    'dispatch: loop {
        match pc {
            0x8324AAE8 => {
    //   block [0x8324AAE8..0x8324AAF4)
	// 8324AAE8: 3D60832A  lis r11, -0x7cd6
	ctx.r[11].s64 = -2094399488;
	// 8324AAEC: 386B7E30  addi r3, r11, 0x7e30
	ctx.r[3].s64 = ctx.r[11].s64 + 32304;
	// 8324AAF0: 4BA5F430  b 0x82ca9f20
	sub_82CA9F20(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324AAF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324AAF8 size=640
    let mut pc: u32 = 0x8324AAF8;
    'dispatch: loop {
        match pc {
            0x8324AAF8 => {
    //   block [0x8324AAF8..0x8324AB58)
	// 8324AAF8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324AAFC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324AB00: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8324AB04: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8324AB08: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324AB0C: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 8324AB10: 3D40820B  lis r10, -0x7df5
	ctx.r[10].s64 = -2113208320;
	// 8324AB14: 3BEB7810  addi r31, r11, 0x7810
	ctx.r[31].s64 = ctx.r[11].s64 + 30736;
	// 8324AB18: 388A9414  addi r4, r10, -0x6bec
	ctx.r[4].s64 = ctx.r[10].s64 + -27628;
	// 8324AB1C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8324AB20: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324AB24: 4AFE23AD  bl 0x8222ced0
	ctx.lr = 0x8324AB28;
	sub_8222CED0(ctx, base);
	// 8324AB28: 3D20820B  lis r9, -0x7df5
	ctx.r[9].s64 = -2113208320;
	// 8324AB2C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324AB30: 38899404  addi r4, r9, -0x6bfc
	ctx.r[4].s64 = ctx.r[9].s64 + -27644;
	// 8324AB34: 387F0004  addi r3, r31, 4
	ctx.r[3].s64 = ctx.r[31].s64 + 4;
	// 8324AB38: 4AFE2399  bl 0x8222ced0
	ctx.lr = 0x8324AB3C;
	sub_8222CED0(ctx, base);
	// 8324AB3C: 3D008349  lis r8, -0x7cb7
	ctx.r[8].s64 = -2092367872;
	// 8324AB40: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8324AB44: 3BC87088  addi r30, r8, 0x7088
	ctx.r[30].s64 = ctx.r[8].s64 + 28808;
	// 8324AB48: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8324AB4C: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 8324AB50: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 8324AB54: 915F000C  stw r10, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[10].u32 ) };
	pc = 0x8324AB58; continue 'dispatch;
            }
            0x8324AB58 => {
    //   block [0x8324AB58..0x8324ABB8)
	// 8324AB58: 7CC000A6  mfmsr r6
	ctx.r[6].u64 = ctx.msr;
	// 8324AB5C: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8324AB60: 7CE02828  lwarx r7, 0, r5
	// lwarx
	let ea = ctx.r[5].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[7].u64 = ctx.reserved.u32 as u64;
	// 8324AB64: 38E70001  addi r7, r7, 1
	ctx.r[7].s64 = ctx.r[7].s64 + 1;
	// 8324AB68: 7CE0292D  stwcx. r7, 0, r5
	// stwcx.
	let addr = ctx.r[5].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[7].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 8324AB6C: 7CC10164  mtmsrd r6, 1
	ctx.msr = (ctx.r[6].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8324AB70: 4082FFE8  bne 0x8324ab58
	if !ctx.cr[0].eq {
	pc = 0x8324AB58; continue 'dispatch;
	}
	// 8324AB74: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8324AB78: 3C80820B  lis r4, -0x7df5
	ctx.r[4].s64 = -2113208320;
	// 8324AB7C: 917F0010  stw r11, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 8324AB80: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324AB84: 388493FC  addi r4, r4, -0x6c04
	ctx.r[4].s64 = ctx.r[4].s64 + -27652;
	// 8324AB88: 387F0014  addi r3, r31, 0x14
	ctx.r[3].s64 = ctx.r[31].s64 + 20;
	// 8324AB8C: 4AFE2345  bl 0x8222ced0
	ctx.lr = 0x8324AB90;
	sub_8222CED0(ctx, base);
	// 8324AB90: 3C60820B  lis r3, -0x7df5
	ctx.r[3].s64 = -2113208320;
	// 8324AB94: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324AB98: 388393EC  addi r4, r3, -0x6c14
	ctx.r[4].s64 = ctx.r[3].s64 + -27668;
	// 8324AB9C: 387F0018  addi r3, r31, 0x18
	ctx.r[3].s64 = ctx.r[31].s64 + 24;
	// 8324ABA0: 4AFE2331  bl 0x8222ced0
	ctx.lr = 0x8324ABA4;
	sub_8222CED0(ctx, base);
	// 8324ABA4: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8324ABA8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8324ABAC: 917F001C  stw r11, 0x1c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), ctx.r[11].u32 ) };
	// 8324ABB0: 7FC7F378  mr r7, r30
	ctx.r[7].u64 = ctx.r[30].u64;
	// 8324ABB4: 915F0020  stw r10, 0x20(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(32 as u32), ctx.r[10].u32 ) };
	pc = 0x8324ABB8; continue 'dispatch;
            }
            0x8324ABB8 => {
    //   block [0x8324ABB8..0x8324AC18)
	// 8324ABB8: 7D0000A6  mfmsr r8
	ctx.r[8].u64 = ctx.msr;
	// 8324ABBC: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8324ABC0: 7D203828  lwarx r9, 0, r7
	// lwarx
	let ea = ctx.r[7].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[9].u64 = ctx.reserved.u32 as u64;
	// 8324ABC4: 39290001  addi r9, r9, 1
	ctx.r[9].s64 = ctx.r[9].s64 + 1;
	// 8324ABC8: 7D20392D  stwcx. r9, 0, r7
	// stwcx.
	let addr = ctx.r[7].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[9].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 8324ABCC: 7D010164  mtmsrd r8, 1
	ctx.msr = (ctx.r[8].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8324ABD0: 4082FFE8  bne 0x8324abb8
	if !ctx.cr[0].eq {
	pc = 0x8324ABB8; continue 'dispatch;
	}
	// 8324ABD4: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8324ABD8: 3CC0820B  lis r6, -0x7df5
	ctx.r[6].s64 = -2113208320;
	// 8324ABDC: 917F0024  stw r11, 0x24(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(36 as u32), ctx.r[11].u32 ) };
	// 8324ABE0: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324ABE4: 388693E4  addi r4, r6, -0x6c1c
	ctx.r[4].s64 = ctx.r[6].s64 + -27676;
	// 8324ABE8: 387F0028  addi r3, r31, 0x28
	ctx.r[3].s64 = ctx.r[31].s64 + 40;
	// 8324ABEC: 4AFE22E5  bl 0x8222ced0
	ctx.lr = 0x8324ABF0;
	sub_8222CED0(ctx, base);
	// 8324ABF0: 3C80820B  lis r4, -0x7df5
	ctx.r[4].s64 = -2113208320;
	// 8324ABF4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324ABF8: 388493D0  addi r4, r4, -0x6c30
	ctx.r[4].s64 = ctx.r[4].s64 + -27696;
	// 8324ABFC: 387F002C  addi r3, r31, 0x2c
	ctx.r[3].s64 = ctx.r[31].s64 + 44;
	// 8324AC00: 4AFE22D1  bl 0x8222ced0
	ctx.lr = 0x8324AC04;
	sub_8222CED0(ctx, base);
	// 8324AC04: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8324AC08: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8324AC0C: 917F0030  stw r11, 0x30(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(48 as u32), ctx.r[11].u32 ) };
	// 8324AC10: 7FC8F378  mr r8, r30
	ctx.r[8].u64 = ctx.r[30].u64;
	// 8324AC14: 915F0034  stw r10, 0x34(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(52 as u32), ctx.r[10].u32 ) };
	pc = 0x8324AC18; continue 'dispatch;
            }
            0x8324AC18 => {
    //   block [0x8324AC18..0x8324AC74)
	// 8324AC18: 7D2000A6  mfmsr r9
	ctx.r[9].u64 = ctx.msr;
	// 8324AC1C: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8324AC20: 7C604028  lwarx r3, 0, r8
	// lwarx
	let ea = ctx.r[8].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[3].u64 = ctx.reserved.u32 as u64;
	// 8324AC24: 38630001  addi r3, r3, 1
	ctx.r[3].s64 = ctx.r[3].s64 + 1;
	// 8324AC28: 7C60412D  stwcx. r3, 0, r8
	// stwcx.
	let addr = ctx.r[8].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[3].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 8324AC2C: 7D210164  mtmsrd r9, 1
	ctx.msr = (ctx.r[9].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8324AC30: 4082FFE8  bne 0x8324ac18
	if !ctx.cr[0].eq {
	pc = 0x8324AC18; continue 'dispatch;
	}
	// 8324AC34: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8324AC38: 3CE0820B  lis r7, -0x7df5
	ctx.r[7].s64 = -2113208320;
	// 8324AC3C: 917F0038  stw r11, 0x38(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(56 as u32), ctx.r[11].u32 ) };
	// 8324AC40: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324AC44: 388793C4  addi r4, r7, -0x6c3c
	ctx.r[4].s64 = ctx.r[7].s64 + -27708;
	// 8324AC48: 387F003C  addi r3, r31, 0x3c
	ctx.r[3].s64 = ctx.r[31].s64 + 60;
	// 8324AC4C: 4AFE2285  bl 0x8222ced0
	ctx.lr = 0x8324AC50;
	sub_8222CED0(ctx, base);
	// 8324AC50: 3CC0820B  lis r6, -0x7df5
	ctx.r[6].s64 = -2113208320;
	// 8324AC54: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324AC58: 388693B0  addi r4, r6, -0x6c50
	ctx.r[4].s64 = ctx.r[6].s64 + -27728;
	// 8324AC5C: 387F0040  addi r3, r31, 0x40
	ctx.r[3].s64 = ctx.r[31].s64 + 64;
	// 8324AC60: 4AFE2271  bl 0x8222ced0
	ctx.lr = 0x8324AC64;
	sub_8222CED0(ctx, base);
	// 8324AC64: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8324AC68: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8324AC6C: 917F0044  stw r11, 0x44(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(68 as u32), ctx.r[11].u32 ) };
	// 8324AC70: 917F0048  stw r11, 0x48(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(72 as u32), ctx.r[11].u32 ) };
	pc = 0x8324AC74; continue 'dispatch;
            }
            0x8324AC74 => {
    //   block [0x8324AC74..0x8324ACD4)
	// 8324AC74: 7C8000A6  mfmsr r4
	ctx.r[4].u64 = ctx.msr;
	// 8324AC78: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8324AC7C: 7CA01828  lwarx r5, 0, r3
	// lwarx
	let ea = ctx.r[3].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[5].u64 = ctx.reserved.u32 as u64;
	// 8324AC80: 38A50001  addi r5, r5, 1
	ctx.r[5].s64 = ctx.r[5].s64 + 1;
	// 8324AC84: 7CA0192D  stwcx. r5, 0, r3
	// stwcx.
	let addr = ctx.r[3].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[5].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 8324AC88: 7C810164  mtmsrd r4, 1
	ctx.msr = (ctx.r[4].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8324AC8C: 4082FFE8  bne 0x8324ac74
	if !ctx.cr[0].eq {
	pc = 0x8324AC74; continue 'dispatch;
	}
	// 8324AC90: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8324AC94: 3D40820B  lis r10, -0x7df5
	ctx.r[10].s64 = -2113208320;
	// 8324AC98: 917F004C  stw r11, 0x4c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(76 as u32), ctx.r[11].u32 ) };
	// 8324AC9C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324ACA0: 388A93A8  addi r4, r10, -0x6c58
	ctx.r[4].s64 = ctx.r[10].s64 + -27736;
	// 8324ACA4: 387F0050  addi r3, r31, 0x50
	ctx.r[3].s64 = ctx.r[31].s64 + 80;
	// 8324ACA8: 4AFE2229  bl 0x8222ced0
	ctx.lr = 0x8324ACAC;
	sub_8222CED0(ctx, base);
	// 8324ACAC: 3D20820B  lis r9, -0x7df5
	ctx.r[9].s64 = -2113208320;
	// 8324ACB0: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324ACB4: 38899394  addi r4, r9, -0x6c6c
	ctx.r[4].s64 = ctx.r[9].s64 + -27756;
	// 8324ACB8: 387F0054  addi r3, r31, 0x54
	ctx.r[3].s64 = ctx.r[31].s64 + 84;
	// 8324ACBC: 4AFE2215  bl 0x8222ced0
	ctx.lr = 0x8324ACC0;
	sub_8222CED0(ctx, base);
	// 8324ACC0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8324ACC4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8324ACC8: 917F0058  stw r11, 0x58(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(88 as u32), ctx.r[11].u32 ) };
	// 8324ACCC: 7FC6F378  mr r6, r30
	ctx.r[6].u64 = ctx.r[30].u64;
	// 8324ACD0: 915F005C  stw r10, 0x5c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(92 as u32), ctx.r[10].u32 ) };
	pc = 0x8324ACD4; continue 'dispatch;
            }
            0x8324ACD4 => {
    //   block [0x8324ACD4..0x8324AD30)
	// 8324ACD4: 7CE000A6  mfmsr r7
	ctx.r[7].u64 = ctx.msr;
	// 8324ACD8: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8324ACDC: 7D003028  lwarx r8, 0, r6
	// lwarx
	let ea = ctx.r[6].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[8].u64 = ctx.reserved.u32 as u64;
	// 8324ACE0: 39080001  addi r8, r8, 1
	ctx.r[8].s64 = ctx.r[8].s64 + 1;
	// 8324ACE4: 7D00312D  stwcx. r8, 0, r6
	// stwcx.
	let addr = ctx.r[6].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[8].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 8324ACE8: 7CE10164  mtmsrd r7, 1
	ctx.msr = (ctx.r[7].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8324ACEC: 4082FFE8  bne 0x8324acd4
	if !ctx.cr[0].eq {
	pc = 0x8324ACD4; continue 'dispatch;
	}
	// 8324ACF0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8324ACF4: 3C80820B  lis r4, -0x7df5
	ctx.r[4].s64 = -2113208320;
	// 8324ACF8: 917F0060  stw r11, 0x60(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(96 as u32), ctx.r[11].u32 ) };
	// 8324ACFC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324AD00: 3884938C  addi r4, r4, -0x6c74
	ctx.r[4].s64 = ctx.r[4].s64 + -27764;
	// 8324AD04: 387F0064  addi r3, r31, 0x64
	ctx.r[3].s64 = ctx.r[31].s64 + 100;
	// 8324AD08: 4AFE21C9  bl 0x8222ced0
	ctx.lr = 0x8324AD0C;
	sub_8222CED0(ctx, base);
	// 8324AD0C: 3C60820B  lis r3, -0x7df5
	ctx.r[3].s64 = -2113208320;
	// 8324AD10: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324AD14: 38839378  addi r4, r3, -0x6c88
	ctx.r[4].s64 = ctx.r[3].s64 + -27784;
	// 8324AD18: 387F0068  addi r3, r31, 0x68
	ctx.r[3].s64 = ctx.r[31].s64 + 104;
	// 8324AD1C: 4AFE21B5  bl 0x8222ced0
	ctx.lr = 0x8324AD20;
	sub_8222CED0(ctx, base);
	// 8324AD20: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8324AD24: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8324AD28: 917F006C  stw r11, 0x6c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8324AD2C: 915F0070  stw r10, 0x70(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(112 as u32), ctx.r[10].u32 ) };
	pc = 0x8324AD30; continue 'dispatch;
            }
            0x8324AD30 => {
    //   block [0x8324AD30..0x8324AD78)
	// 8324AD30: 7D0000A6  mfmsr r8
	ctx.r[8].u64 = ctx.msr;
	// 8324AD34: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8324AD38: 7D20F028  lwarx r9, 0, r30
	// lwarx
	let ea = ctx.r[30].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[9].u64 = ctx.reserved.u32 as u64;
	// 8324AD3C: 39290001  addi r9, r9, 1
	ctx.r[9].s64 = ctx.r[9].s64 + 1;
	// 8324AD40: 7D20F12D  stwcx. r9, 0, r30
	// stwcx.
	let addr = ctx.r[30].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[9].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 8324AD44: 7D010164  mtmsrd r8, 1
	ctx.msr = (ctx.r[8].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8324AD48: 4082FFE8  bne 0x8324ad30
	if !ctx.cr[0].eq {
	pc = 0x8324AD30; continue 'dispatch;
	}
	// 8324AD4C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8324AD50: 3CE0832A  lis r7, -0x7cd6
	ctx.r[7].s64 = -2094399488;
	// 8324AD54: 917F0074  stw r11, 0x74(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8324AD58: 38677EF8  addi r3, r7, 0x7ef8
	ctx.r[3].s64 = ctx.r[7].s64 + 32504;
	// 8324AD5C: 4BA5F1C5  bl 0x82ca9f20
	ctx.lr = 0x8324AD60;
	sub_82CA9F20(ctx, base);
	// 8324AD60: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8324AD64: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324AD68: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324AD6C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8324AD70: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8324AD74: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324AD78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8324AD78 size=24
    let mut pc: u32 = 0x8324AD78;
    'dispatch: loop {
        match pc {
            0x8324AD78 => {
    //   block [0x8324AD78..0x8324AD90)
	// 8324AD78: 3D408336  lis r10, -0x7cca
	ctx.r[10].s64 = -2093613056;
	// 8324AD7C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8324AD80: 3D20832A  lis r9, -0x7cd6
	ctx.r[9].s64 = -2094399488;
	// 8324AD84: 38697FD8  addi r3, r9, 0x7fd8
	ctx.r[3].s64 = ctx.r[9].s64 + 32728;
	// 8324AD88: 916AC4AC  stw r11, -0x3b54(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-15188 as u32), ctx.r[11].u32 ) };
	// 8324AD8C: 4BA5F194  b 0x82ca9f20
	sub_82CA9F20(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324AD90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324AD90 size=64
    let mut pc: u32 = 0x8324AD90;
    'dispatch: loop {
        match pc {
            0x8324AD90 => {
    //   block [0x8324AD90..0x8324ADD0)
	// 8324AD90: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324AD94: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324AD98: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324AD9C: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 8324ADA0: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324ADA4: 388B96F0  addi r4, r11, -0x6910
	ctx.r[4].s64 = ctx.r[11].s64 + -26896;
	// 8324ADA8: 386A7888  addi r3, r10, 0x7888
	ctx.r[3].s64 = ctx.r[10].s64 + 30856;
	// 8324ADAC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324ADB0: 4AFE2121  bl 0x8222ced0
	ctx.lr = 0x8324ADB4;
	sub_8222CED0(ctx, base);
	// 8324ADB4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8324ADB8: 38698050  addi r3, r9, -0x7fb0
	ctx.r[3].s64 = ctx.r[9].s64 + -32688;
	// 8324ADBC: 4BA5F165  bl 0x82ca9f20
	ctx.lr = 0x8324ADC0;
	sub_82CA9F20(ctx, base);
	// 8324ADC0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324ADC4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324ADC8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324ADCC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324ADD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324ADD0 size=64
    let mut pc: u32 = 0x8324ADD0;
    'dispatch: loop {
        match pc {
            0x8324ADD0 => {
    //   block [0x8324ADD0..0x8324AE10)
	// 8324ADD0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324ADD4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324ADD8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324ADDC: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 8324ADE0: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324ADE4: 388B96F8  addi r4, r11, -0x6908
	ctx.r[4].s64 = ctx.r[11].s64 + -26888;
	// 8324ADE8: 386A788C  addi r3, r10, 0x788c
	ctx.r[3].s64 = ctx.r[10].s64 + 30860;
	// 8324ADEC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324ADF0: 4AFE20E1  bl 0x8222ced0
	ctx.lr = 0x8324ADF4;
	sub_8222CED0(ctx, base);
	// 8324ADF4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8324ADF8: 38698060  addi r3, r9, -0x7fa0
	ctx.r[3].s64 = ctx.r[9].s64 + -32672;
	// 8324ADFC: 4BA5F125  bl 0x82ca9f20
	ctx.lr = 0x8324AE00;
	sub_82CA9F20(ctx, base);
	// 8324AE00: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324AE04: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324AE08: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324AE0C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324AE10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324AE10 size=64
    let mut pc: u32 = 0x8324AE10;
    'dispatch: loop {
        match pc {
            0x8324AE10 => {
    //   block [0x8324AE10..0x8324AE50)
	// 8324AE10: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324AE14: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324AE18: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324AE1C: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 8324AE20: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324AE24: 388B970C  addi r4, r11, -0x68f4
	ctx.r[4].s64 = ctx.r[11].s64 + -26868;
	// 8324AE28: 386A7890  addi r3, r10, 0x7890
	ctx.r[3].s64 = ctx.r[10].s64 + 30864;
	// 8324AE2C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324AE30: 4AFE20A1  bl 0x8222ced0
	ctx.lr = 0x8324AE34;
	sub_8222CED0(ctx, base);
	// 8324AE34: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8324AE38: 38698070  addi r3, r9, -0x7f90
	ctx.r[3].s64 = ctx.r[9].s64 + -32656;
	// 8324AE3C: 4BA5F0E5  bl 0x82ca9f20
	ctx.lr = 0x8324AE40;
	sub_82CA9F20(ctx, base);
	// 8324AE40: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324AE44: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324AE48: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324AE4C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324AE50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324AE50 size=64
    let mut pc: u32 = 0x8324AE50;
    'dispatch: loop {
        match pc {
            0x8324AE50 => {
    //   block [0x8324AE50..0x8324AE90)
	// 8324AE50: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324AE54: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324AE58: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324AE5C: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 8324AE60: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324AE64: 388B9718  addi r4, r11, -0x68e8
	ctx.r[4].s64 = ctx.r[11].s64 + -26856;
	// 8324AE68: 386A7894  addi r3, r10, 0x7894
	ctx.r[3].s64 = ctx.r[10].s64 + 30868;
	// 8324AE6C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324AE70: 4AFE2061  bl 0x8222ced0
	ctx.lr = 0x8324AE74;
	sub_8222CED0(ctx, base);
	// 8324AE74: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8324AE78: 38698080  addi r3, r9, -0x7f80
	ctx.r[3].s64 = ctx.r[9].s64 + -32640;
	// 8324AE7C: 4BA5F0A5  bl 0x82ca9f20
	ctx.lr = 0x8324AE80;
	sub_82CA9F20(ctx, base);
	// 8324AE80: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324AE84: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324AE88: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324AE8C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324AE90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324AE90 size=64
    let mut pc: u32 = 0x8324AE90;
    'dispatch: loop {
        match pc {
            0x8324AE90 => {
    //   block [0x8324AE90..0x8324AED0)
	// 8324AE90: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324AE94: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324AE98: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324AE9C: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 8324AEA0: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324AEA4: 388B9728  addi r4, r11, -0x68d8
	ctx.r[4].s64 = ctx.r[11].s64 + -26840;
	// 8324AEA8: 386A7898  addi r3, r10, 0x7898
	ctx.r[3].s64 = ctx.r[10].s64 + 30872;
	// 8324AEAC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324AEB0: 4AFE2021  bl 0x8222ced0
	ctx.lr = 0x8324AEB4;
	sub_8222CED0(ctx, base);
	// 8324AEB4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8324AEB8: 38698090  addi r3, r9, -0x7f70
	ctx.r[3].s64 = ctx.r[9].s64 + -32624;
	// 8324AEBC: 4BA5F065  bl 0x82ca9f20
	ctx.lr = 0x8324AEC0;
	sub_82CA9F20(ctx, base);
	// 8324AEC0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324AEC4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324AEC8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324AECC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324AED0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324AED0 size=64
    let mut pc: u32 = 0x8324AED0;
    'dispatch: loop {
        match pc {
            0x8324AED0 => {
    //   block [0x8324AED0..0x8324AF10)
	// 8324AED0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324AED4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324AED8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324AEDC: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 8324AEE0: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324AEE4: 388B9738  addi r4, r11, -0x68c8
	ctx.r[4].s64 = ctx.r[11].s64 + -26824;
	// 8324AEE8: 386A789C  addi r3, r10, 0x789c
	ctx.r[3].s64 = ctx.r[10].s64 + 30876;
	// 8324AEEC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324AEF0: 4AFE1FE1  bl 0x8222ced0
	ctx.lr = 0x8324AEF4;
	sub_8222CED0(ctx, base);
	// 8324AEF4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8324AEF8: 386980A0  addi r3, r9, -0x7f60
	ctx.r[3].s64 = ctx.r[9].s64 + -32608;
	// 8324AEFC: 4BA5F025  bl 0x82ca9f20
	ctx.lr = 0x8324AF00;
	sub_82CA9F20(ctx, base);
	// 8324AF00: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324AF04: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324AF08: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324AF0C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324AF10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324AF10 size=64
    let mut pc: u32 = 0x8324AF10;
    'dispatch: loop {
        match pc {
            0x8324AF10 => {
    //   block [0x8324AF10..0x8324AF50)
	// 8324AF10: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324AF14: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324AF18: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324AF1C: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 8324AF20: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324AF24: 388B9744  addi r4, r11, -0x68bc
	ctx.r[4].s64 = ctx.r[11].s64 + -26812;
	// 8324AF28: 386A78A0  addi r3, r10, 0x78a0
	ctx.r[3].s64 = ctx.r[10].s64 + 30880;
	// 8324AF2C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324AF30: 4AFE1FA1  bl 0x8222ced0
	ctx.lr = 0x8324AF34;
	sub_8222CED0(ctx, base);
	// 8324AF34: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8324AF38: 386980B0  addi r3, r9, -0x7f50
	ctx.r[3].s64 = ctx.r[9].s64 + -32592;
	// 8324AF3C: 4BA5EFE5  bl 0x82ca9f20
	ctx.lr = 0x8324AF40;
	sub_82CA9F20(ctx, base);
	// 8324AF40: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324AF44: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324AF48: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324AF4C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324AF50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324AF50 size=64
    let mut pc: u32 = 0x8324AF50;
    'dispatch: loop {
        match pc {
            0x8324AF50 => {
    //   block [0x8324AF50..0x8324AF90)
	// 8324AF50: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324AF54: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324AF58: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324AF5C: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 8324AF60: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324AF64: 388B9754  addi r4, r11, -0x68ac
	ctx.r[4].s64 = ctx.r[11].s64 + -26796;
	// 8324AF68: 386A78A4  addi r3, r10, 0x78a4
	ctx.r[3].s64 = ctx.r[10].s64 + 30884;
	// 8324AF6C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324AF70: 4AFE1F61  bl 0x8222ced0
	ctx.lr = 0x8324AF74;
	sub_8222CED0(ctx, base);
	// 8324AF74: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8324AF78: 386980C0  addi r3, r9, -0x7f40
	ctx.r[3].s64 = ctx.r[9].s64 + -32576;
	// 8324AF7C: 4BA5EFA5  bl 0x82ca9f20
	ctx.lr = 0x8324AF80;
	sub_82CA9F20(ctx, base);
	// 8324AF80: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324AF84: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324AF88: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324AF8C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324AF90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324AF90 size=64
    let mut pc: u32 = 0x8324AF90;
    'dispatch: loop {
        match pc {
            0x8324AF90 => {
    //   block [0x8324AF90..0x8324AFD0)
	// 8324AF90: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324AF94: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324AF98: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324AF9C: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 8324AFA0: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324AFA4: 388B9764  addi r4, r11, -0x689c
	ctx.r[4].s64 = ctx.r[11].s64 + -26780;
	// 8324AFA8: 386A78A8  addi r3, r10, 0x78a8
	ctx.r[3].s64 = ctx.r[10].s64 + 30888;
	// 8324AFAC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324AFB0: 4AFE1F21  bl 0x8222ced0
	ctx.lr = 0x8324AFB4;
	sub_8222CED0(ctx, base);
	// 8324AFB4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8324AFB8: 386980D0  addi r3, r9, -0x7f30
	ctx.r[3].s64 = ctx.r[9].s64 + -32560;
	// 8324AFBC: 4BA5EF65  bl 0x82ca9f20
	ctx.lr = 0x8324AFC0;
	sub_82CA9F20(ctx, base);
	// 8324AFC0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324AFC4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324AFC8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324AFCC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324AFD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324AFD0 size=64
    let mut pc: u32 = 0x8324AFD0;
    'dispatch: loop {
        match pc {
            0x8324AFD0 => {
    //   block [0x8324AFD0..0x8324B010)
	// 8324AFD0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324AFD4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324AFD8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324AFDC: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 8324AFE0: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324AFE4: 388B977C  addi r4, r11, -0x6884
	ctx.r[4].s64 = ctx.r[11].s64 + -26756;
	// 8324AFE8: 386A78AC  addi r3, r10, 0x78ac
	ctx.r[3].s64 = ctx.r[10].s64 + 30892;
	// 8324AFEC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324AFF0: 4AFE1EE1  bl 0x8222ced0
	ctx.lr = 0x8324AFF4;
	sub_8222CED0(ctx, base);
	// 8324AFF4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8324AFF8: 386980E0  addi r3, r9, -0x7f20
	ctx.r[3].s64 = ctx.r[9].s64 + -32544;
	// 8324AFFC: 4BA5EF25  bl 0x82ca9f20
	ctx.lr = 0x8324B000;
	sub_82CA9F20(ctx, base);
	// 8324B000: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324B004: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324B008: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324B00C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324B010(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324B010 size=64
    let mut pc: u32 = 0x8324B010;
    'dispatch: loop {
        match pc {
            0x8324B010 => {
    //   block [0x8324B010..0x8324B050)
	// 8324B010: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324B014: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324B018: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324B01C: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 8324B020: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324B024: 388B9794  addi r4, r11, -0x686c
	ctx.r[4].s64 = ctx.r[11].s64 + -26732;
	// 8324B028: 386A78B0  addi r3, r10, 0x78b0
	ctx.r[3].s64 = ctx.r[10].s64 + 30896;
	// 8324B02C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324B030: 4AFE1EA1  bl 0x8222ced0
	ctx.lr = 0x8324B034;
	sub_8222CED0(ctx, base);
	// 8324B034: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8324B038: 386980F0  addi r3, r9, -0x7f10
	ctx.r[3].s64 = ctx.r[9].s64 + -32528;
	// 8324B03C: 4BA5EEE5  bl 0x82ca9f20
	ctx.lr = 0x8324B040;
	sub_82CA9F20(ctx, base);
	// 8324B040: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324B044: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324B048: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324B04C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324B050(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324B050 size=64
    let mut pc: u32 = 0x8324B050;
    'dispatch: loop {
        match pc {
            0x8324B050 => {
    //   block [0x8324B050..0x8324B090)
	// 8324B050: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324B054: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324B058: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324B05C: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 8324B060: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324B064: 388B97A4  addi r4, r11, -0x685c
	ctx.r[4].s64 = ctx.r[11].s64 + -26716;
	// 8324B068: 386A78B4  addi r3, r10, 0x78b4
	ctx.r[3].s64 = ctx.r[10].s64 + 30900;
	// 8324B06C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324B070: 4AFE1E61  bl 0x8222ced0
	ctx.lr = 0x8324B074;
	sub_8222CED0(ctx, base);
	// 8324B074: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8324B078: 38698100  addi r3, r9, -0x7f00
	ctx.r[3].s64 = ctx.r[9].s64 + -32512;
	// 8324B07C: 4BA5EEA5  bl 0x82ca9f20
	ctx.lr = 0x8324B080;
	sub_82CA9F20(ctx, base);
	// 8324B080: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324B084: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324B088: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324B08C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324B090(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324B090 size=64
    let mut pc: u32 = 0x8324B090;
    'dispatch: loop {
        match pc {
            0x8324B090 => {
    //   block [0x8324B090..0x8324B0D0)
	// 8324B090: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324B094: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324B098: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324B09C: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 8324B0A0: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324B0A4: 388B97B8  addi r4, r11, -0x6848
	ctx.r[4].s64 = ctx.r[11].s64 + -26696;
	// 8324B0A8: 386A78B8  addi r3, r10, 0x78b8
	ctx.r[3].s64 = ctx.r[10].s64 + 30904;
	// 8324B0AC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324B0B0: 4AFE1E21  bl 0x8222ced0
	ctx.lr = 0x8324B0B4;
	sub_8222CED0(ctx, base);
	// 8324B0B4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8324B0B8: 38698110  addi r3, r9, -0x7ef0
	ctx.r[3].s64 = ctx.r[9].s64 + -32496;
	// 8324B0BC: 4BA5EE65  bl 0x82ca9f20
	ctx.lr = 0x8324B0C0;
	sub_82CA9F20(ctx, base);
	// 8324B0C0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324B0C4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324B0C8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324B0CC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324B0D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324B0D0 size=64
    let mut pc: u32 = 0x8324B0D0;
    'dispatch: loop {
        match pc {
            0x8324B0D0 => {
    //   block [0x8324B0D0..0x8324B110)
	// 8324B0D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324B0D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324B0D8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324B0DC: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 8324B0E0: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324B0E4: 388B97C4  addi r4, r11, -0x683c
	ctx.r[4].s64 = ctx.r[11].s64 + -26684;
	// 8324B0E8: 386A78BC  addi r3, r10, 0x78bc
	ctx.r[3].s64 = ctx.r[10].s64 + 30908;
	// 8324B0EC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324B0F0: 4AFE1DE1  bl 0x8222ced0
	ctx.lr = 0x8324B0F4;
	sub_8222CED0(ctx, base);
	// 8324B0F4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8324B0F8: 38698120  addi r3, r9, -0x7ee0
	ctx.r[3].s64 = ctx.r[9].s64 + -32480;
	// 8324B0FC: 4BA5EE25  bl 0x82ca9f20
	ctx.lr = 0x8324B100;
	sub_82CA9F20(ctx, base);
	// 8324B100: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324B104: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324B108: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324B10C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324B110(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324B110 size=64
    let mut pc: u32 = 0x8324B110;
    'dispatch: loop {
        match pc {
            0x8324B110 => {
    //   block [0x8324B110..0x8324B150)
	// 8324B110: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324B114: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324B118: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324B11C: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 8324B120: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324B124: 388B97DC  addi r4, r11, -0x6824
	ctx.r[4].s64 = ctx.r[11].s64 + -26660;
	// 8324B128: 386A78C0  addi r3, r10, 0x78c0
	ctx.r[3].s64 = ctx.r[10].s64 + 30912;
	// 8324B12C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324B130: 4AFE1DA1  bl 0x8222ced0
	ctx.lr = 0x8324B134;
	sub_8222CED0(ctx, base);
	// 8324B134: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8324B138: 38698130  addi r3, r9, -0x7ed0
	ctx.r[3].s64 = ctx.r[9].s64 + -32464;
	// 8324B13C: 4BA5EDE5  bl 0x82ca9f20
	ctx.lr = 0x8324B140;
	sub_82CA9F20(ctx, base);
	// 8324B140: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324B144: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324B148: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324B14C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324B150(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324B150 size=88
    let mut pc: u32 = 0x8324B150;
    'dispatch: loop {
        match pc {
            0x8324B150 => {
    //   block [0x8324B150..0x8324B1A8)
	// 8324B150: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324B154: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324B158: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8324B15C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324B160: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 8324B164: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324B168: 3BEB78C4  addi r31, r11, 0x78c4
	ctx.r[31].s64 = ctx.r[11].s64 + 30916;
	// 8324B16C: 388A7888  addi r4, r10, 0x7888
	ctx.r[4].s64 = ctx.r[10].s64 + 30856;
	// 8324B170: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8324B174: 4AFA50CD  bl 0x821f0240
	ctx.lr = 0x8324B178;
	sub_821F0240(ctx, base);
	// 8324B178: 3D20820B  lis r9, -0x7df5
	ctx.r[9].s64 = -2113208320;
	// 8324B17C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8324B180: 388997FC  addi r4, r9, -0x6804
	ctx.r[4].s64 = ctx.r[9].s64 + -26628;
	// 8324B184: 4AF8F83D  bl 0x821da9c0
	ctx.lr = 0x8324B188;
	sub_821DA9C0(ctx, base);
	// 8324B188: 3D00832B  lis r8, -0x7cd5
	ctx.r[8].s64 = -2094333952;
	// 8324B18C: 38688140  addi r3, r8, -0x7ec0
	ctx.r[3].s64 = ctx.r[8].s64 + -32448;
	// 8324B190: 4BA5ED91  bl 0x82ca9f20
	ctx.lr = 0x8324B194;
	sub_82CA9F20(ctx, base);
	// 8324B194: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324B198: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324B19C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324B1A0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8324B1A4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324B1A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324B1A8 size=64
    let mut pc: u32 = 0x8324B1A8;
    'dispatch: loop {
        match pc {
            0x8324B1A8 => {
    //   block [0x8324B1A8..0x8324B1E8)
	// 8324B1A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324B1AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324B1B0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324B1B4: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 8324B1B8: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324B1BC: 388B980C  addi r4, r11, -0x67f4
	ctx.r[4].s64 = ctx.r[11].s64 + -26612;
	// 8324B1C0: 386A78C8  addi r3, r10, 0x78c8
	ctx.r[3].s64 = ctx.r[10].s64 + 30920;
	// 8324B1C4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324B1C8: 4AFE1D09  bl 0x8222ced0
	ctx.lr = 0x8324B1CC;
	sub_8222CED0(ctx, base);
	// 8324B1CC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8324B1D0: 38698150  addi r3, r9, -0x7eb0
	ctx.r[3].s64 = ctx.r[9].s64 + -32432;
	// 8324B1D4: 4BA5ED4D  bl 0x82ca9f20
	ctx.lr = 0x8324B1D8;
	sub_82CA9F20(ctx, base);
	// 8324B1D8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324B1DC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324B1E0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324B1E4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324B1E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324B1E8 size=64
    let mut pc: u32 = 0x8324B1E8;
    'dispatch: loop {
        match pc {
            0x8324B1E8 => {
    //   block [0x8324B1E8..0x8324B228)
	// 8324B1E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324B1EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324B1F0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324B1F4: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 8324B1F8: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324B1FC: 388B9824  addi r4, r11, -0x67dc
	ctx.r[4].s64 = ctx.r[11].s64 + -26588;
	// 8324B200: 386A78CC  addi r3, r10, 0x78cc
	ctx.r[3].s64 = ctx.r[10].s64 + 30924;
	// 8324B204: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324B208: 4AFE1CC9  bl 0x8222ced0
	ctx.lr = 0x8324B20C;
	sub_8222CED0(ctx, base);
	// 8324B20C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8324B210: 38698160  addi r3, r9, -0x7ea0
	ctx.r[3].s64 = ctx.r[9].s64 + -32416;
	// 8324B214: 4BA5ED0D  bl 0x82ca9f20
	ctx.lr = 0x8324B218;
	sub_82CA9F20(ctx, base);
	// 8324B218: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324B21C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324B220: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324B224: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324B228(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324B228 size=64
    let mut pc: u32 = 0x8324B228;
    'dispatch: loop {
        match pc {
            0x8324B228 => {
    //   block [0x8324B228..0x8324B268)
	// 8324B228: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324B22C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324B230: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324B234: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 8324B238: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324B23C: 388B9838  addi r4, r11, -0x67c8
	ctx.r[4].s64 = ctx.r[11].s64 + -26568;
	// 8324B240: 386A78D0  addi r3, r10, 0x78d0
	ctx.r[3].s64 = ctx.r[10].s64 + 30928;
	// 8324B244: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324B248: 4AFE1C89  bl 0x8222ced0
	ctx.lr = 0x8324B24C;
	sub_8222CED0(ctx, base);
	// 8324B24C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8324B250: 38698170  addi r3, r9, -0x7e90
	ctx.r[3].s64 = ctx.r[9].s64 + -32400;
	// 8324B254: 4BA5ECCD  bl 0x82ca9f20
	ctx.lr = 0x8324B258;
	sub_82CA9F20(ctx, base);
	// 8324B258: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324B25C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324B260: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324B264: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324B268(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324B268 size=64
    let mut pc: u32 = 0x8324B268;
    'dispatch: loop {
        match pc {
            0x8324B268 => {
    //   block [0x8324B268..0x8324B2A8)
	// 8324B268: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324B26C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324B270: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324B274: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 8324B278: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324B27C: 388B9840  addi r4, r11, -0x67c0
	ctx.r[4].s64 = ctx.r[11].s64 + -26560;
	// 8324B280: 386A78D4  addi r3, r10, 0x78d4
	ctx.r[3].s64 = ctx.r[10].s64 + 30932;
	// 8324B284: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324B288: 4AFE1C49  bl 0x8222ced0
	ctx.lr = 0x8324B28C;
	sub_8222CED0(ctx, base);
	// 8324B28C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8324B290: 38698180  addi r3, r9, -0x7e80
	ctx.r[3].s64 = ctx.r[9].s64 + -32384;
	// 8324B294: 4BA5EC8D  bl 0x82ca9f20
	ctx.lr = 0x8324B298;
	sub_82CA9F20(ctx, base);
	// 8324B298: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324B29C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324B2A0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324B2A4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324B2A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324B2A8 size=88
    let mut pc: u32 = 0x8324B2A8;
    'dispatch: loop {
        match pc {
            0x8324B2A8 => {
    //   block [0x8324B2A8..0x8324B300)
	// 8324B2A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324B2AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324B2B0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8324B2B4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324B2B8: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 8324B2BC: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324B2C0: 3BEB78D8  addi r31, r11, 0x78d8
	ctx.r[31].s64 = ctx.r[11].s64 + 30936;
	// 8324B2C4: 388A78D4  addi r4, r10, 0x78d4
	ctx.r[4].s64 = ctx.r[10].s64 + 30932;
	// 8324B2C8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8324B2CC: 4AFA4F75  bl 0x821f0240
	ctx.lr = 0x8324B2D0;
	sub_821F0240(ctx, base);
	// 8324B2D0: 3D20820B  lis r9, -0x7df5
	ctx.r[9].s64 = -2113208320;
	// 8324B2D4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8324B2D8: 38899848  addi r4, r9, -0x67b8
	ctx.r[4].s64 = ctx.r[9].s64 + -26552;
	// 8324B2DC: 4AF8F6E5  bl 0x821da9c0
	ctx.lr = 0x8324B2E0;
	sub_821DA9C0(ctx, base);
	// 8324B2E0: 3D00832B  lis r8, -0x7cd5
	ctx.r[8].s64 = -2094333952;
	// 8324B2E4: 38688190  addi r3, r8, -0x7e70
	ctx.r[3].s64 = ctx.r[8].s64 + -32368;
	// 8324B2E8: 4BA5EC39  bl 0x82ca9f20
	ctx.lr = 0x8324B2EC;
	sub_82CA9F20(ctx, base);
	// 8324B2EC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324B2F0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324B2F4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324B2F8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8324B2FC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324B300(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324B300 size=64
    let mut pc: u32 = 0x8324B300;
    'dispatch: loop {
        match pc {
            0x8324B300 => {
    //   block [0x8324B300..0x8324B340)
	// 8324B300: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324B304: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324B308: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324B30C: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 8324B310: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324B314: 388B984C  addi r4, r11, -0x67b4
	ctx.r[4].s64 = ctx.r[11].s64 + -26548;
	// 8324B318: 386A78DC  addi r3, r10, 0x78dc
	ctx.r[3].s64 = ctx.r[10].s64 + 30940;
	// 8324B31C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324B320: 4AFE1BB1  bl 0x8222ced0
	ctx.lr = 0x8324B324;
	sub_8222CED0(ctx, base);
	// 8324B324: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8324B328: 386981A0  addi r3, r9, -0x7e60
	ctx.r[3].s64 = ctx.r[9].s64 + -32352;
	// 8324B32C: 4BA5EBF5  bl 0x82ca9f20
	ctx.lr = 0x8324B330;
	sub_82CA9F20(ctx, base);
	// 8324B330: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324B334: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324B338: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324B33C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324B340(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324B340 size=64
    let mut pc: u32 = 0x8324B340;
    'dispatch: loop {
        match pc {
            0x8324B340 => {
    //   block [0x8324B340..0x8324B380)
	// 8324B340: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324B344: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324B348: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324B34C: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 8324B350: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324B354: 388B9850  addi r4, r11, -0x67b0
	ctx.r[4].s64 = ctx.r[11].s64 + -26544;
	// 8324B358: 386A78E0  addi r3, r10, 0x78e0
	ctx.r[3].s64 = ctx.r[10].s64 + 30944;
	// 8324B35C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324B360: 4AFE1B71  bl 0x8222ced0
	ctx.lr = 0x8324B364;
	sub_8222CED0(ctx, base);
	// 8324B364: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8324B368: 386981B0  addi r3, r9, -0x7e50
	ctx.r[3].s64 = ctx.r[9].s64 + -32336;
	// 8324B36C: 4BA5EBB5  bl 0x82ca9f20
	ctx.lr = 0x8324B370;
	sub_82CA9F20(ctx, base);
	// 8324B370: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324B374: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324B378: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324B37C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324B380(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324B380 size=64
    let mut pc: u32 = 0x8324B380;
    'dispatch: loop {
        match pc {
            0x8324B380 => {
    //   block [0x8324B380..0x8324B3C0)
	// 8324B380: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324B384: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324B388: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324B38C: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 8324B390: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324B394: 388B9860  addi r4, r11, -0x67a0
	ctx.r[4].s64 = ctx.r[11].s64 + -26528;
	// 8324B398: 386A78E4  addi r3, r10, 0x78e4
	ctx.r[3].s64 = ctx.r[10].s64 + 30948;
	// 8324B39C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324B3A0: 4AFE1B31  bl 0x8222ced0
	ctx.lr = 0x8324B3A4;
	sub_8222CED0(ctx, base);
	// 8324B3A4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8324B3A8: 386981C0  addi r3, r9, -0x7e40
	ctx.r[3].s64 = ctx.r[9].s64 + -32320;
	// 8324B3AC: 4BA5EB75  bl 0x82ca9f20
	ctx.lr = 0x8324B3B0;
	sub_82CA9F20(ctx, base);
	// 8324B3B0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324B3B4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324B3B8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324B3BC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324B3C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324B3C0 size=64
    let mut pc: u32 = 0x8324B3C0;
    'dispatch: loop {
        match pc {
            0x8324B3C0 => {
    //   block [0x8324B3C0..0x8324B400)
	// 8324B3C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324B3C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324B3C8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324B3CC: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 8324B3D0: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324B3D4: 388B987C  addi r4, r11, -0x6784
	ctx.r[4].s64 = ctx.r[11].s64 + -26500;
	// 8324B3D8: 386A78E8  addi r3, r10, 0x78e8
	ctx.r[3].s64 = ctx.r[10].s64 + 30952;
	// 8324B3DC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324B3E0: 4AFE1AF1  bl 0x8222ced0
	ctx.lr = 0x8324B3E4;
	sub_8222CED0(ctx, base);
	// 8324B3E4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8324B3E8: 386981D0  addi r3, r9, -0x7e30
	ctx.r[3].s64 = ctx.r[9].s64 + -32304;
	// 8324B3EC: 4BA5EB35  bl 0x82ca9f20
	ctx.lr = 0x8324B3F0;
	sub_82CA9F20(ctx, base);
	// 8324B3F0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324B3F4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324B3F8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324B3FC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324B400(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324B400 size=64
    let mut pc: u32 = 0x8324B400;
    'dispatch: loop {
        match pc {
            0x8324B400 => {
    //   block [0x8324B400..0x8324B440)
	// 8324B400: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324B404: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324B408: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324B40C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8324B410: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324B414: 388B1AD4  addi r4, r11, 0x1ad4
	ctx.r[4].s64 = ctx.r[11].s64 + 6868;
	// 8324B418: 386A78EC  addi r3, r10, 0x78ec
	ctx.r[3].s64 = ctx.r[10].s64 + 30956;
	// 8324B41C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324B420: 4AFE1AB1  bl 0x8222ced0
	ctx.lr = 0x8324B424;
	sub_8222CED0(ctx, base);
	// 8324B424: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8324B428: 386981E0  addi r3, r9, -0x7e20
	ctx.r[3].s64 = ctx.r[9].s64 + -32288;
	// 8324B42C: 4BA5EAF5  bl 0x82ca9f20
	ctx.lr = 0x8324B430;
	sub_82CA9F20(ctx, base);
	// 8324B430: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324B434: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324B438: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324B43C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324B440(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324B440 size=64
    let mut pc: u32 = 0x8324B440;
    'dispatch: loop {
        match pc {
            0x8324B440 => {
    //   block [0x8324B440..0x8324B480)
	// 8324B440: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324B444: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324B448: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324B44C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8324B450: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324B454: 388B1ADC  addi r4, r11, 0x1adc
	ctx.r[4].s64 = ctx.r[11].s64 + 6876;
	// 8324B458: 386A78F0  addi r3, r10, 0x78f0
	ctx.r[3].s64 = ctx.r[10].s64 + 30960;
	// 8324B45C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324B460: 4AFE1A71  bl 0x8222ced0
	ctx.lr = 0x8324B464;
	sub_8222CED0(ctx, base);
	// 8324B464: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8324B468: 386981F0  addi r3, r9, -0x7e10
	ctx.r[3].s64 = ctx.r[9].s64 + -32272;
	// 8324B46C: 4BA5EAB5  bl 0x82ca9f20
	ctx.lr = 0x8324B470;
	sub_82CA9F20(ctx, base);
	// 8324B470: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324B474: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324B478: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324B47C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324B480(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324B480 size=64
    let mut pc: u32 = 0x8324B480;
    'dispatch: loop {
        match pc {
            0x8324B480 => {
    //   block [0x8324B480..0x8324B4C0)
	// 8324B480: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324B484: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324B488: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324B48C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8324B490: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324B494: 388B1AF0  addi r4, r11, 0x1af0
	ctx.r[4].s64 = ctx.r[11].s64 + 6896;
	// 8324B498: 386A78F4  addi r3, r10, 0x78f4
	ctx.r[3].s64 = ctx.r[10].s64 + 30964;
	// 8324B49C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324B4A0: 4AFE1A31  bl 0x8222ced0
	ctx.lr = 0x8324B4A4;
	sub_8222CED0(ctx, base);
	// 8324B4A4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8324B4A8: 38698200  addi r3, r9, -0x7e00
	ctx.r[3].s64 = ctx.r[9].s64 + -32256;
	// 8324B4AC: 4BA5EA75  bl 0x82ca9f20
	ctx.lr = 0x8324B4B0;
	sub_82CA9F20(ctx, base);
	// 8324B4B0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324B4B4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324B4B8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324B4BC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324B4C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324B4C0 size=64
    let mut pc: u32 = 0x8324B4C0;
    'dispatch: loop {
        match pc {
            0x8324B4C0 => {
    //   block [0x8324B4C0..0x8324B500)
	// 8324B4C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324B4C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324B4C8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324B4CC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8324B4D0: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324B4D4: 388B1AD4  addi r4, r11, 0x1ad4
	ctx.r[4].s64 = ctx.r[11].s64 + 6868;
	// 8324B4D8: 386A78F8  addi r3, r10, 0x78f8
	ctx.r[3].s64 = ctx.r[10].s64 + 30968;
	// 8324B4DC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324B4E0: 4AFE19F1  bl 0x8222ced0
	ctx.lr = 0x8324B4E4;
	sub_8222CED0(ctx, base);
	// 8324B4E4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8324B4E8: 38698210  addi r3, r9, -0x7df0
	ctx.r[3].s64 = ctx.r[9].s64 + -32240;
	// 8324B4EC: 4BA5EA35  bl 0x82ca9f20
	ctx.lr = 0x8324B4F0;
	sub_82CA9F20(ctx, base);
	// 8324B4F0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324B4F4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324B4F8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324B4FC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324B500(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324B500 size=64
    let mut pc: u32 = 0x8324B500;
    'dispatch: loop {
        match pc {
            0x8324B500 => {
    //   block [0x8324B500..0x8324B540)
	// 8324B500: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324B504: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324B508: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324B50C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8324B510: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324B514: 388B1ADC  addi r4, r11, 0x1adc
	ctx.r[4].s64 = ctx.r[11].s64 + 6876;
	// 8324B518: 386A78FC  addi r3, r10, 0x78fc
	ctx.r[3].s64 = ctx.r[10].s64 + 30972;
	// 8324B51C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324B520: 4AFE19B1  bl 0x8222ced0
	ctx.lr = 0x8324B524;
	sub_8222CED0(ctx, base);
	// 8324B524: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8324B528: 38698220  addi r3, r9, -0x7de0
	ctx.r[3].s64 = ctx.r[9].s64 + -32224;
	// 8324B52C: 4BA5E9F5  bl 0x82ca9f20
	ctx.lr = 0x8324B530;
	sub_82CA9F20(ctx, base);
	// 8324B530: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324B534: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324B538: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324B53C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324B540(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324B540 size=64
    let mut pc: u32 = 0x8324B540;
    'dispatch: loop {
        match pc {
            0x8324B540 => {
    //   block [0x8324B540..0x8324B580)
	// 8324B540: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324B544: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324B548: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324B54C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8324B550: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324B554: 388B1AF0  addi r4, r11, 0x1af0
	ctx.r[4].s64 = ctx.r[11].s64 + 6896;
	// 8324B558: 386A7900  addi r3, r10, 0x7900
	ctx.r[3].s64 = ctx.r[10].s64 + 30976;
	// 8324B55C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324B560: 4AFE1971  bl 0x8222ced0
	ctx.lr = 0x8324B564;
	sub_8222CED0(ctx, base);
	// 8324B564: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8324B568: 38698230  addi r3, r9, -0x7dd0
	ctx.r[3].s64 = ctx.r[9].s64 + -32208;
	// 8324B56C: 4BA5E9B5  bl 0x82ca9f20
	ctx.lr = 0x8324B570;
	sub_82CA9F20(ctx, base);
	// 8324B570: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324B574: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324B578: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324B57C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324B580(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324B580 size=96
    let mut pc: u32 = 0x8324B580;
    'dispatch: loop {
        match pc {
            0x8324B580 => {
    //   block [0x8324B580..0x8324B5A4)
	// 8324B580: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324B584: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324B588: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324B58C: 38600014  li r3, 0x14
	ctx.r[3].s64 = 20;
	// 8324B590: 4AFD3CC9  bl 0x8221f258
	ctx.lr = 0x8324B594;
	sub_8221F258(ctx, base);
	// 8324B594: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 8324B598: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8324B59C: 419A0008  beq cr6, 0x8324b5a4
	if ctx.cr[6].eq {
	pc = 0x8324B5A4; continue 'dispatch;
	}
	// 8324B5A0: 916B0000  stw r11, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	pc = 0x8324B5A4; continue 'dispatch;
            }
            0x8324B5A4 => {
    //   block [0x8324B5A4..0x8324B5B0)
	// 8324B5A4: 354B0004  addic. r10, r11, 4
	ctx.xer.ca = (ctx.r[11].u32 > (!(4 as u32)));
	ctx.r[10].s64 = ctx.r[11].s64 + 4;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 8324B5A8: 41820008  beq 0x8324b5b0
	if ctx.cr[0].eq {
	pc = 0x8324B5B0; continue 'dispatch;
	}
	// 8324B5AC: 916A0000  stw r11, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	pc = 0x8324B5B0; continue 'dispatch;
            }
            0x8324B5B0 => {
    //   block [0x8324B5B0..0x8324B5E0)
	// 8324B5B0: 3D208349  lis r9, -0x7cb7
	ctx.r[9].s64 = -2092367872;
	// 8324B5B4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8324B5B8: 39097904  addi r8, r9, 0x7904
	ctx.r[8].s64 = ctx.r[9].s64 + 30980;
	// 8324B5BC: 3CE0832B  lis r7, -0x7cd5
	ctx.r[7].s64 = -2094333952;
	// 8324B5C0: 38678240  addi r3, r7, -0x7dc0
	ctx.r[3].s64 = ctx.r[7].s64 + -32192;
	// 8324B5C4: 91680004  stw r11, 4(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 8324B5C8: 91480008  stw r10, 8(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 8324B5CC: 4BA5E955  bl 0x82ca9f20
	ctx.lr = 0x8324B5D0;
	sub_82CA9F20(ctx, base);
	// 8324B5D0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324B5D4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324B5D8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324B5DC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324B5E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8324B5E0 size=12
    let mut pc: u32 = 0x8324B5E0;
    'dispatch: loop {
        match pc {
            0x8324B5E0 => {
    //   block [0x8324B5E0..0x8324B5EC)
	// 8324B5E0: 3D60832B  lis r11, -0x7cd5
	ctx.r[11].s64 = -2094333952;
	// 8324B5E4: 386B8300  addi r3, r11, -0x7d00
	ctx.r[3].s64 = ctx.r[11].s64 + -32000;
	// 8324B5E8: 4BA5E938  b 0x82ca9f20
	sub_82CA9F20(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324B5F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324B5F0 size=64
    let mut pc: u32 = 0x8324B5F0;
    'dispatch: loop {
        match pc {
            0x8324B5F0 => {
    //   block [0x8324B5F0..0x8324B630)
	// 8324B5F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324B5F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324B5F8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324B5FC: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 8324B600: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324B604: 388B0CA0  addi r4, r11, 0xca0
	ctx.r[4].s64 = ctx.r[11].s64 + 3232;
	// 8324B608: 386A7924  addi r3, r10, 0x7924
	ctx.r[3].s64 = ctx.r[10].s64 + 31012;
	// 8324B60C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324B610: 4AFE18C1  bl 0x8222ced0
	ctx.lr = 0x8324B614;
	sub_8222CED0(ctx, base);
	// 8324B614: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8324B618: 38698358  addi r3, r9, -0x7ca8
	ctx.r[3].s64 = ctx.r[9].s64 + -31912;
	// 8324B61C: 4BA5E905  bl 0x82ca9f20
	ctx.lr = 0x8324B620;
	sub_82CA9F20(ctx, base);
	// 8324B620: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324B624: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324B628: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324B62C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324B630(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324B630 size=64
    let mut pc: u32 = 0x8324B630;
    'dispatch: loop {
        match pc {
            0x8324B630 => {
    //   block [0x8324B630..0x8324B670)
	// 8324B630: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324B634: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324B638: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324B63C: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 8324B640: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324B644: 388B0CA0  addi r4, r11, 0xca0
	ctx.r[4].s64 = ctx.r[11].s64 + 3232;
	// 8324B648: 386A7928  addi r3, r10, 0x7928
	ctx.r[3].s64 = ctx.r[10].s64 + 31016;
	// 8324B64C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324B650: 4AFE1881  bl 0x8222ced0
	ctx.lr = 0x8324B654;
	sub_8222CED0(ctx, base);
	// 8324B654: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8324B658: 38698368  addi r3, r9, -0x7c98
	ctx.r[3].s64 = ctx.r[9].s64 + -31896;
	// 8324B65C: 4BA5E8C5  bl 0x82ca9f20
	ctx.lr = 0x8324B660;
	sub_82CA9F20(ctx, base);
	// 8324B660: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324B664: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324B668: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324B66C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324B670(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324B670 size=144
    let mut pc: u32 = 0x8324B670;
    'dispatch: loop {
        match pc {
            0x8324B670 => {
    //   block [0x8324B670..0x8324B694)
	// 8324B670: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324B674: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324B678: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324B67C: 38600060  li r3, 0x60
	ctx.r[3].s64 = 96;
	// 8324B680: 4AFD3BD9  bl 0x8221f258
	ctx.lr = 0x8324B684;
	sub_8221F258(ctx, base);
	// 8324B684: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8324B688: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8324B68C: 419A0008  beq cr6, 0x8324b694
	if ctx.cr[6].eq {
	pc = 0x8324B694; continue 'dispatch;
	}
	// 8324B690: 91430000  stw r10, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	pc = 0x8324B694; continue 'dispatch;
            }
            0x8324B694 => {
    //   block [0x8324B694..0x8324B6A0)
	// 8324B694: 35630004  addic. r11, r3, 4
	ctx.xer.ca = (ctx.r[3].u32 > (!(4 as u32)));
	ctx.r[11].s64 = ctx.r[3].s64 + 4;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8324B698: 41820008  beq 0x8324b6a0
	if ctx.cr[0].eq {
	pc = 0x8324B6A0; continue 'dispatch;
	}
	// 8324B69C: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	pc = 0x8324B6A0; continue 'dispatch;
            }
            0x8324B6A0 => {
    //   block [0x8324B6A0..0x8324B6AC)
	// 8324B6A0: 35630008  addic. r11, r3, 8
	ctx.xer.ca = (ctx.r[3].u32 > (!(8 as u32)));
	ctx.r[11].s64 = ctx.r[3].s64 + 8;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8324B6A4: 41820008  beq 0x8324b6ac
	if ctx.cr[0].eq {
	pc = 0x8324B6AC; continue 'dispatch;
	}
	// 8324B6A8: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	pc = 0x8324B6AC; continue 'dispatch;
            }
            0x8324B6AC => {
    //   block [0x8324B6AC..0x8324B700)
	// 8324B6AC: 3D208349  lis r9, -0x7cb7
	ctx.r[9].s64 = -2092367872;
	// 8324B6B0: 99430051  stb r10, 0x51(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(81 as u32), ctx.r[10].u8 ) };
	// 8324B6B4: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 8324B6B8: 3909792C  addi r8, r9, 0x792c
	ctx.r[8].s64 = ctx.r[9].s64 + 31020;
	// 8324B6BC: 99630050  stb r11, 0x50(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(80 as u32), ctx.r[11].u8 ) };
	// 8324B6C0: 3CE0832B  lis r7, -0x7cd5
	ctx.r[7].s64 = -2094333952;
	// 8324B6C4: 90680004  stw r3, 4(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(4 as u32), ctx.r[3].u32 ) };
	// 8324B6C8: 99630051  stb r11, 0x51(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(81 as u32), ctx.r[11].u8 ) };
	// 8324B6CC: 38678378  addi r3, r7, -0x7c88
	ctx.r[3].s64 = ctx.r[7].s64 + -31880;
	// 8324B6D0: 81680004  lwz r11, 4(r8)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(4 as u32) ) } as u64;
	// 8324B6D4: 916B0004  stw r11, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 8324B6D8: 81680004  lwz r11, 4(r8)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(4 as u32) ) } as u64;
	// 8324B6DC: 916B0000  stw r11, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8324B6E0: 81680004  lwz r11, 4(r8)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(4 as u32) ) } as u64;
	// 8324B6E4: 916B0008  stw r11, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 8324B6E8: 91480008  stw r10, 8(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 8324B6EC: 4BA5E835  bl 0x82ca9f20
	ctx.lr = 0x8324B6F0;
	sub_82CA9F20(ctx, base);
	// 8324B6F0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324B6F4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324B6F8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324B6FC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324B700(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324B700 size=64
    let mut pc: u32 = 0x8324B700;
    'dispatch: loop {
        match pc {
            0x8324B700 => {
    //   block [0x8324B700..0x8324B740)
	// 8324B700: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324B704: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324B708: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324B70C: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 8324B710: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324B714: 388B9988  addi r4, r11, -0x6678
	ctx.r[4].s64 = ctx.r[11].s64 + -26232;
	// 8324B718: 386A7938  addi r3, r10, 0x7938
	ctx.r[3].s64 = ctx.r[10].s64 + 31032;
	// 8324B71C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324B720: 4AFE17B1  bl 0x8222ced0
	ctx.lr = 0x8324B724;
	sub_8222CED0(ctx, base);
	// 8324B724: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8324B728: 38698388  addi r3, r9, -0x7c78
	ctx.r[3].s64 = ctx.r[9].s64 + -31864;
	// 8324B72C: 4BA5E7F5  bl 0x82ca9f20
	ctx.lr = 0x8324B730;
	sub_82CA9F20(ctx, base);
	// 8324B730: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324B734: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324B738: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324B73C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324B740(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324B740 size=64
    let mut pc: u32 = 0x8324B740;
    'dispatch: loop {
        match pc {
            0x8324B740 => {
    //   block [0x8324B740..0x8324B780)
	// 8324B740: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324B744: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324B748: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324B74C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8324B750: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324B754: 388B1AD4  addi r4, r11, 0x1ad4
	ctx.r[4].s64 = ctx.r[11].s64 + 6868;
	// 8324B758: 386A793C  addi r3, r10, 0x793c
	ctx.r[3].s64 = ctx.r[10].s64 + 31036;
	// 8324B75C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324B760: 4AFE1771  bl 0x8222ced0
	ctx.lr = 0x8324B764;
	sub_8222CED0(ctx, base);
	// 8324B764: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8324B768: 38698398  addi r3, r9, -0x7c68
	ctx.r[3].s64 = ctx.r[9].s64 + -31848;
	// 8324B76C: 4BA5E7B5  bl 0x82ca9f20
	ctx.lr = 0x8324B770;
	sub_82CA9F20(ctx, base);
	// 8324B770: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324B774: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324B778: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324B77C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324B780(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324B780 size=64
    let mut pc: u32 = 0x8324B780;
    'dispatch: loop {
        match pc {
            0x8324B780 => {
    //   block [0x8324B780..0x8324B7C0)
	// 8324B780: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324B784: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324B788: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324B78C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8324B790: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324B794: 388B1ADC  addi r4, r11, 0x1adc
	ctx.r[4].s64 = ctx.r[11].s64 + 6876;
	// 8324B798: 386A7940  addi r3, r10, 0x7940
	ctx.r[3].s64 = ctx.r[10].s64 + 31040;
	// 8324B79C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324B7A0: 4AFE1731  bl 0x8222ced0
	ctx.lr = 0x8324B7A4;
	sub_8222CED0(ctx, base);
	// 8324B7A4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8324B7A8: 386983A8  addi r3, r9, -0x7c58
	ctx.r[3].s64 = ctx.r[9].s64 + -31832;
	// 8324B7AC: 4BA5E775  bl 0x82ca9f20
	ctx.lr = 0x8324B7B0;
	sub_82CA9F20(ctx, base);
	// 8324B7B0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324B7B4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324B7B8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324B7BC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324B7C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324B7C0 size=64
    let mut pc: u32 = 0x8324B7C0;
    'dispatch: loop {
        match pc {
            0x8324B7C0 => {
    //   block [0x8324B7C0..0x8324B800)
	// 8324B7C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324B7C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324B7C8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324B7CC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8324B7D0: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324B7D4: 388B1AF0  addi r4, r11, 0x1af0
	ctx.r[4].s64 = ctx.r[11].s64 + 6896;
	// 8324B7D8: 386A7944  addi r3, r10, 0x7944
	ctx.r[3].s64 = ctx.r[10].s64 + 31044;
	// 8324B7DC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324B7E0: 4AFE16F1  bl 0x8222ced0
	ctx.lr = 0x8324B7E4;
	sub_8222CED0(ctx, base);
	// 8324B7E4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8324B7E8: 386983B8  addi r3, r9, -0x7c48
	ctx.r[3].s64 = ctx.r[9].s64 + -31816;
	// 8324B7EC: 4BA5E735  bl 0x82ca9f20
	ctx.lr = 0x8324B7F0;
	sub_82CA9F20(ctx, base);
	// 8324B7F0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324B7F4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324B7F8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324B7FC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324B800(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324B800 size=64
    let mut pc: u32 = 0x8324B800;
    'dispatch: loop {
        match pc {
            0x8324B800 => {
    //   block [0x8324B800..0x8324B840)
	// 8324B800: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324B804: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324B808: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324B80C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8324B810: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324B814: 388B1AD4  addi r4, r11, 0x1ad4
	ctx.r[4].s64 = ctx.r[11].s64 + 6868;
	// 8324B818: 386A7948  addi r3, r10, 0x7948
	ctx.r[3].s64 = ctx.r[10].s64 + 31048;
	// 8324B81C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324B820: 4AFE16B1  bl 0x8222ced0
	ctx.lr = 0x8324B824;
	sub_8222CED0(ctx, base);
	// 8324B824: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8324B828: 386983C8  addi r3, r9, -0x7c38
	ctx.r[3].s64 = ctx.r[9].s64 + -31800;
	// 8324B82C: 4BA5E6F5  bl 0x82ca9f20
	ctx.lr = 0x8324B830;
	sub_82CA9F20(ctx, base);
	// 8324B830: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324B834: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324B838: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324B83C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324B840(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324B840 size=64
    let mut pc: u32 = 0x8324B840;
    'dispatch: loop {
        match pc {
            0x8324B840 => {
    //   block [0x8324B840..0x8324B880)
	// 8324B840: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324B844: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324B848: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324B84C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8324B850: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324B854: 388B1ADC  addi r4, r11, 0x1adc
	ctx.r[4].s64 = ctx.r[11].s64 + 6876;
	// 8324B858: 386A794C  addi r3, r10, 0x794c
	ctx.r[3].s64 = ctx.r[10].s64 + 31052;
	// 8324B85C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324B860: 4AFE1671  bl 0x8222ced0
	ctx.lr = 0x8324B864;
	sub_8222CED0(ctx, base);
	// 8324B864: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8324B868: 386983D8  addi r3, r9, -0x7c28
	ctx.r[3].s64 = ctx.r[9].s64 + -31784;
	// 8324B86C: 4BA5E6B5  bl 0x82ca9f20
	ctx.lr = 0x8324B870;
	sub_82CA9F20(ctx, base);
	// 8324B870: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324B874: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324B878: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324B87C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324B880(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324B880 size=64
    let mut pc: u32 = 0x8324B880;
    'dispatch: loop {
        match pc {
            0x8324B880 => {
    //   block [0x8324B880..0x8324B8C0)
	// 8324B880: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324B884: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324B888: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324B88C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8324B890: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324B894: 388B1AF0  addi r4, r11, 0x1af0
	ctx.r[4].s64 = ctx.r[11].s64 + 6896;
	// 8324B898: 386A7950  addi r3, r10, 0x7950
	ctx.r[3].s64 = ctx.r[10].s64 + 31056;
	// 8324B89C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324B8A0: 4AFE1631  bl 0x8222ced0
	ctx.lr = 0x8324B8A4;
	sub_8222CED0(ctx, base);
	// 8324B8A4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8324B8A8: 386983E8  addi r3, r9, -0x7c18
	ctx.r[3].s64 = ctx.r[9].s64 + -31768;
	// 8324B8AC: 4BA5E675  bl 0x82ca9f20
	ctx.lr = 0x8324B8B0;
	sub_82CA9F20(ctx, base);
	// 8324B8B0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324B8B4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324B8B8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324B8BC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324B8C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324B8C0 size=64
    let mut pc: u32 = 0x8324B8C0;
    'dispatch: loop {
        match pc {
            0x8324B8C0 => {
    //   block [0x8324B8C0..0x8324B900)
	// 8324B8C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324B8C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324B8C8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324B8CC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8324B8D0: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324B8D4: 388B1AD4  addi r4, r11, 0x1ad4
	ctx.r[4].s64 = ctx.r[11].s64 + 6868;
	// 8324B8D8: 386A7954  addi r3, r10, 0x7954
	ctx.r[3].s64 = ctx.r[10].s64 + 31060;
	// 8324B8DC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324B8E0: 4AFE15F1  bl 0x8222ced0
	ctx.lr = 0x8324B8E4;
	sub_8222CED0(ctx, base);
	// 8324B8E4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8324B8E8: 386983F8  addi r3, r9, -0x7c08
	ctx.r[3].s64 = ctx.r[9].s64 + -31752;
	// 8324B8EC: 4BA5E635  bl 0x82ca9f20
	ctx.lr = 0x8324B8F0;
	sub_82CA9F20(ctx, base);
	// 8324B8F0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324B8F4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324B8F8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324B8FC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324B900(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324B900 size=64
    let mut pc: u32 = 0x8324B900;
    'dispatch: loop {
        match pc {
            0x8324B900 => {
    //   block [0x8324B900..0x8324B940)
	// 8324B900: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324B904: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324B908: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324B90C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8324B910: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324B914: 388B1ADC  addi r4, r11, 0x1adc
	ctx.r[4].s64 = ctx.r[11].s64 + 6876;
	// 8324B918: 386A7958  addi r3, r10, 0x7958
	ctx.r[3].s64 = ctx.r[10].s64 + 31064;
	// 8324B91C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324B920: 4AFE15B1  bl 0x8222ced0
	ctx.lr = 0x8324B924;
	sub_8222CED0(ctx, base);
	// 8324B924: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8324B928: 38698408  addi r3, r9, -0x7bf8
	ctx.r[3].s64 = ctx.r[9].s64 + -31736;
	// 8324B92C: 4BA5E5F5  bl 0x82ca9f20
	ctx.lr = 0x8324B930;
	sub_82CA9F20(ctx, base);
	// 8324B930: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324B934: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324B938: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324B93C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324B940(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324B940 size=64
    let mut pc: u32 = 0x8324B940;
    'dispatch: loop {
        match pc {
            0x8324B940 => {
    //   block [0x8324B940..0x8324B980)
	// 8324B940: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324B944: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324B948: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324B94C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8324B950: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324B954: 388B1AF0  addi r4, r11, 0x1af0
	ctx.r[4].s64 = ctx.r[11].s64 + 6896;
	// 8324B958: 386A795C  addi r3, r10, 0x795c
	ctx.r[3].s64 = ctx.r[10].s64 + 31068;
	// 8324B95C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324B960: 4AFE1571  bl 0x8222ced0
	ctx.lr = 0x8324B964;
	sub_8222CED0(ctx, base);
	// 8324B964: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8324B968: 38698418  addi r3, r9, -0x7be8
	ctx.r[3].s64 = ctx.r[9].s64 + -31720;
	// 8324B96C: 4BA5E5B5  bl 0x82ca9f20
	ctx.lr = 0x8324B970;
	sub_82CA9F20(ctx, base);
	// 8324B970: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324B974: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324B978: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324B97C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324B980(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324B980 size=56
    let mut pc: u32 = 0x8324B980;
    'dispatch: loop {
        match pc {
            0x8324B980 => {
    //   block [0x8324B980..0x8324B9B8)
	// 8324B980: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324B984: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324B988: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324B98C: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 8324B990: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8324B994: 386B9D1C  addi r3, r11, -0x62e4
	ctx.r[3].s64 = ctx.r[11].s64 + -25316;
	// 8324B998: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8324B99C: 4AFA83BD  bl 0x821f3d58
	ctx.lr = 0x8324B9A0;
	sub_821F3D58(ctx, base);
	// 8324B9A0: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324B9A4: 906A7960  stw r3, 0x7960(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(31072 as u32), ctx.r[3].u32 ) };
	// 8324B9A8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324B9AC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324B9B0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324B9B4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324B9B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324B9B8 size=56
    let mut pc: u32 = 0x8324B9B8;
    'dispatch: loop {
        match pc {
            0x8324B9B8 => {
    //   block [0x8324B9B8..0x8324B9F0)
	// 8324B9B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324B9BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324B9C0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324B9C4: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 8324B9C8: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8324B9CC: 386B9D2C  addi r3, r11, -0x62d4
	ctx.r[3].s64 = ctx.r[11].s64 + -25300;
	// 8324B9D0: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8324B9D4: 4AFA8385  bl 0x821f3d58
	ctx.lr = 0x8324B9D8;
	sub_821F3D58(ctx, base);
	// 8324B9D8: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324B9DC: 906A7964  stw r3, 0x7964(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(31076 as u32), ctx.r[3].u32 ) };
	// 8324B9E0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324B9E4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324B9E8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324B9EC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324B9F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324B9F0 size=96
    let mut pc: u32 = 0x8324B9F0;
    'dispatch: loop {
        match pc {
            0x8324B9F0 => {
    //   block [0x8324B9F0..0x8324BA14)
	// 8324B9F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324B9F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324B9F8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324B9FC: 38600080  li r3, 0x80
	ctx.r[3].s64 = 128;
	// 8324BA00: 4AFD3859  bl 0x8221f258
	ctx.lr = 0x8324BA04;
	sub_8221F258(ctx, base);
	// 8324BA04: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 8324BA08: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8324BA0C: 419A0008  beq cr6, 0x8324ba14
	if ctx.cr[6].eq {
	pc = 0x8324BA14; continue 'dispatch;
	}
	// 8324BA10: 916B0000  stw r11, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	pc = 0x8324BA14; continue 'dispatch;
            }
            0x8324BA14 => {
    //   block [0x8324BA14..0x8324BA20)
	// 8324BA14: 354B0004  addic. r10, r11, 4
	ctx.xer.ca = (ctx.r[11].u32 > (!(4 as u32)));
	ctx.r[10].s64 = ctx.r[11].s64 + 4;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 8324BA18: 41820008  beq 0x8324ba20
	if ctx.cr[0].eq {
	pc = 0x8324BA20; continue 'dispatch;
	}
	// 8324BA1C: 916A0000  stw r11, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	pc = 0x8324BA20; continue 'dispatch;
            }
            0x8324BA20 => {
    //   block [0x8324BA20..0x8324BA50)
	// 8324BA20: 3D208349  lis r9, -0x7cb7
	ctx.r[9].s64 = -2092367872;
	// 8324BA24: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8324BA28: 39097968  addi r8, r9, 0x7968
	ctx.r[8].s64 = ctx.r[9].s64 + 31080;
	// 8324BA2C: 3CE0832B  lis r7, -0x7cd5
	ctx.r[7].s64 = -2094333952;
	// 8324BA30: 38678428  addi r3, r7, -0x7bd8
	ctx.r[3].s64 = ctx.r[7].s64 + -31704;
	// 8324BA34: 91680004  stw r11, 4(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 8324BA38: 91480008  stw r10, 8(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 8324BA3C: 4BA5E4E5  bl 0x82ca9f20
	ctx.lr = 0x8324BA40;
	sub_82CA9F20(ctx, base);
	// 8324BA40: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324BA44: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324BA48: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324BA4C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324BA50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324BA50 size=96
    let mut pc: u32 = 0x8324BA50;
    'dispatch: loop {
        match pc {
            0x8324BA50 => {
    //   block [0x8324BA50..0x8324BA74)
	// 8324BA50: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324BA54: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324BA58: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324BA5C: 38600080  li r3, 0x80
	ctx.r[3].s64 = 128;
	// 8324BA60: 4AFD37F9  bl 0x8221f258
	ctx.lr = 0x8324BA64;
	sub_8221F258(ctx, base);
	// 8324BA64: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 8324BA68: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8324BA6C: 419A0008  beq cr6, 0x8324ba74
	if ctx.cr[6].eq {
	pc = 0x8324BA74; continue 'dispatch;
	}
	// 8324BA70: 916B0000  stw r11, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	pc = 0x8324BA74; continue 'dispatch;
            }
            0x8324BA74 => {
    //   block [0x8324BA74..0x8324BA80)
	// 8324BA74: 354B0004  addic. r10, r11, 4
	ctx.xer.ca = (ctx.r[11].u32 > (!(4 as u32)));
	ctx.r[10].s64 = ctx.r[11].s64 + 4;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 8324BA78: 41820008  beq 0x8324ba80
	if ctx.cr[0].eq {
	pc = 0x8324BA80; continue 'dispatch;
	}
	// 8324BA7C: 916A0000  stw r11, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	pc = 0x8324BA80; continue 'dispatch;
            }
            0x8324BA80 => {
    //   block [0x8324BA80..0x8324BAB0)
	// 8324BA80: 3D208349  lis r9, -0x7cb7
	ctx.r[9].s64 = -2092367872;
	// 8324BA84: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8324BA88: 39097974  addi r8, r9, 0x7974
	ctx.r[8].s64 = ctx.r[9].s64 + 31092;
	// 8324BA8C: 3CE0832B  lis r7, -0x7cd5
	ctx.r[7].s64 = -2094333952;
	// 8324BA90: 38678438  addi r3, r7, -0x7bc8
	ctx.r[3].s64 = ctx.r[7].s64 + -31688;
	// 8324BA94: 91680004  stw r11, 4(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 8324BA98: 91480008  stw r10, 8(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 8324BA9C: 4BA5E485  bl 0x82ca9f20
	ctx.lr = 0x8324BAA0;
	sub_82CA9F20(ctx, base);
	// 8324BAA0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324BAA4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324BAA8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324BAAC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324BAB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324BAB0 size=64
    let mut pc: u32 = 0x8324BAB0;
    'dispatch: loop {
        match pc {
            0x8324BAB0 => {
    //   block [0x8324BAB0..0x8324BAF0)
	// 8324BAB0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324BAB4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324BAB8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324BABC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8324BAC0: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324BAC4: 388B052C  addi r4, r11, 0x52c
	ctx.r[4].s64 = ctx.r[11].s64 + 1324;
	// 8324BAC8: 386A7980  addi r3, r10, 0x7980
	ctx.r[3].s64 = ctx.r[10].s64 + 31104;
	// 8324BACC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324BAD0: 4AFE1401  bl 0x8222ced0
	ctx.lr = 0x8324BAD4;
	sub_8222CED0(ctx, base);
	// 8324BAD4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8324BAD8: 38698448  addi r3, r9, -0x7bb8
	ctx.r[3].s64 = ctx.r[9].s64 + -31672;
	// 8324BADC: 4BA5E445  bl 0x82ca9f20
	ctx.lr = 0x8324BAE0;
	sub_82CA9F20(ctx, base);
	// 8324BAE0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324BAE4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324BAE8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324BAEC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324BAF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324BAF0 size=64
    let mut pc: u32 = 0x8324BAF0;
    'dispatch: loop {
        match pc {
            0x8324BAF0 => {
    //   block [0x8324BAF0..0x8324BB30)
	// 8324BAF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324BAF4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324BAF8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324BAFC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8324BB00: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324BB04: 388B1AD4  addi r4, r11, 0x1ad4
	ctx.r[4].s64 = ctx.r[11].s64 + 6868;
	// 8324BB08: 386A7984  addi r3, r10, 0x7984
	ctx.r[3].s64 = ctx.r[10].s64 + 31108;
	// 8324BB0C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324BB10: 4AFE13C1  bl 0x8222ced0
	ctx.lr = 0x8324BB14;
	sub_8222CED0(ctx, base);
	// 8324BB14: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8324BB18: 38698458  addi r3, r9, -0x7ba8
	ctx.r[3].s64 = ctx.r[9].s64 + -31656;
	// 8324BB1C: 4BA5E405  bl 0x82ca9f20
	ctx.lr = 0x8324BB20;
	sub_82CA9F20(ctx, base);
	// 8324BB20: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324BB24: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324BB28: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324BB2C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324BB30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324BB30 size=64
    let mut pc: u32 = 0x8324BB30;
    'dispatch: loop {
        match pc {
            0x8324BB30 => {
    //   block [0x8324BB30..0x8324BB70)
	// 8324BB30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324BB34: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324BB38: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324BB3C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8324BB40: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324BB44: 388B1ADC  addi r4, r11, 0x1adc
	ctx.r[4].s64 = ctx.r[11].s64 + 6876;
	// 8324BB48: 386A7988  addi r3, r10, 0x7988
	ctx.r[3].s64 = ctx.r[10].s64 + 31112;
	// 8324BB4C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324BB50: 4AFE1381  bl 0x8222ced0
	ctx.lr = 0x8324BB54;
	sub_8222CED0(ctx, base);
	// 8324BB54: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8324BB58: 38698468  addi r3, r9, -0x7b98
	ctx.r[3].s64 = ctx.r[9].s64 + -31640;
	// 8324BB5C: 4BA5E3C5  bl 0x82ca9f20
	ctx.lr = 0x8324BB60;
	sub_82CA9F20(ctx, base);
	// 8324BB60: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324BB64: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324BB68: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324BB6C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324BB70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324BB70 size=64
    let mut pc: u32 = 0x8324BB70;
    'dispatch: loop {
        match pc {
            0x8324BB70 => {
    //   block [0x8324BB70..0x8324BBB0)
	// 8324BB70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324BB74: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324BB78: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324BB7C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8324BB80: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324BB84: 388B1AF0  addi r4, r11, 0x1af0
	ctx.r[4].s64 = ctx.r[11].s64 + 6896;
	// 8324BB88: 386A798C  addi r3, r10, 0x798c
	ctx.r[3].s64 = ctx.r[10].s64 + 31116;
	// 8324BB8C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324BB90: 4AFE1341  bl 0x8222ced0
	ctx.lr = 0x8324BB94;
	sub_8222CED0(ctx, base);
	// 8324BB94: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8324BB98: 38698478  addi r3, r9, -0x7b88
	ctx.r[3].s64 = ctx.r[9].s64 + -31624;
	// 8324BB9C: 4BA5E385  bl 0x82ca9f20
	ctx.lr = 0x8324BBA0;
	sub_82CA9F20(ctx, base);
	// 8324BBA0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324BBA4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324BBA8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324BBAC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324BBB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324BBB0 size=64
    let mut pc: u32 = 0x8324BBB0;
    'dispatch: loop {
        match pc {
            0x8324BBB0 => {
    //   block [0x8324BBB0..0x8324BBF0)
	// 8324BBB0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324BBB4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324BBB8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324BBBC: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 8324BBC0: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324BBC4: 388B9FE4  addi r4, r11, -0x601c
	ctx.r[4].s64 = ctx.r[11].s64 + -24604;
	// 8324BBC8: 386A7990  addi r3, r10, 0x7990
	ctx.r[3].s64 = ctx.r[10].s64 + 31120;
	// 8324BBCC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324BBD0: 4AFE1301  bl 0x8222ced0
	ctx.lr = 0x8324BBD4;
	sub_8222CED0(ctx, base);
	// 8324BBD4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8324BBD8: 38698488  addi r3, r9, -0x7b78
	ctx.r[3].s64 = ctx.r[9].s64 + -31608;
	// 8324BBDC: 4BA5E345  bl 0x82ca9f20
	ctx.lr = 0x8324BBE0;
	sub_82CA9F20(ctx, base);
	// 8324BBE0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324BBE4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324BBE8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324BBEC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324BBF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324BBF0 size=56
    let mut pc: u32 = 0x8324BBF0;
    'dispatch: loop {
        match pc {
            0x8324BBF0 => {
    //   block [0x8324BBF0..0x8324BC28)
	// 8324BBF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324BBF4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324BBF8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324BBFC: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 8324BC00: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8324BC04: 386B9FF0  addi r3, r11, -0x6010
	ctx.r[3].s64 = ctx.r[11].s64 + -24592;
	// 8324BC08: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8324BC0C: 4AFA814D  bl 0x821f3d58
	ctx.lr = 0x8324BC10;
	sub_821F3D58(ctx, base);
	// 8324BC10: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324BC14: 906A7994  stw r3, 0x7994(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(31124 as u32), ctx.r[3].u32 ) };
	// 8324BC18: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324BC1C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324BC20: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324BC24: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324BC28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324BC28 size=56
    let mut pc: u32 = 0x8324BC28;
    'dispatch: loop {
        match pc {
            0x8324BC28 => {
    //   block [0x8324BC28..0x8324BC60)
	// 8324BC28: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324BC2C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324BC30: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324BC34: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 8324BC38: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8324BC3C: 386BA01C  addi r3, r11, -0x5fe4
	ctx.r[3].s64 = ctx.r[11].s64 + -24548;
	// 8324BC40: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8324BC44: 4AFA8115  bl 0x821f3d58
	ctx.lr = 0x8324BC48;
	sub_821F3D58(ctx, base);
	// 8324BC48: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324BC4C: 906A7998  stw r3, 0x7998(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(31128 as u32), ctx.r[3].u32 ) };
	// 8324BC50: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324BC54: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324BC58: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324BC5C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324BC60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324BC60 size=56
    let mut pc: u32 = 0x8324BC60;
    'dispatch: loop {
        match pc {
            0x8324BC60 => {
    //   block [0x8324BC60..0x8324BC98)
	// 8324BC60: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324BC64: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324BC68: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324BC6C: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 8324BC70: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8324BC74: 386BA048  addi r3, r11, -0x5fb8
	ctx.r[3].s64 = ctx.r[11].s64 + -24504;
	// 8324BC78: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8324BC7C: 4AFA80DD  bl 0x821f3d58
	ctx.lr = 0x8324BC80;
	sub_821F3D58(ctx, base);
	// 8324BC80: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324BC84: 906A799C  stw r3, 0x799c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(31132 as u32), ctx.r[3].u32 ) };
	// 8324BC88: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324BC8C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324BC90: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324BC94: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324BC98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324BC98 size=56
    let mut pc: u32 = 0x8324BC98;
    'dispatch: loop {
        match pc {
            0x8324BC98 => {
    //   block [0x8324BC98..0x8324BCD0)
	// 8324BC98: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324BC9C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324BCA0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324BCA4: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 8324BCA8: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8324BCAC: 386BA074  addi r3, r11, -0x5f8c
	ctx.r[3].s64 = ctx.r[11].s64 + -24460;
	// 8324BCB0: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8324BCB4: 4AFA80A5  bl 0x821f3d58
	ctx.lr = 0x8324BCB8;
	sub_821F3D58(ctx, base);
	// 8324BCB8: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324BCBC: 906A79A0  stw r3, 0x79a0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(31136 as u32), ctx.r[3].u32 ) };
	// 8324BCC0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324BCC4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324BCC8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324BCCC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324BCD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324BCD0 size=56
    let mut pc: u32 = 0x8324BCD0;
    'dispatch: loop {
        match pc {
            0x8324BCD0 => {
    //   block [0x8324BCD0..0x8324BD08)
	// 8324BCD0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324BCD4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324BCD8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324BCDC: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 8324BCE0: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8324BCE4: 386BA0A0  addi r3, r11, -0x5f60
	ctx.r[3].s64 = ctx.r[11].s64 + -24416;
	// 8324BCE8: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8324BCEC: 4AFA806D  bl 0x821f3d58
	ctx.lr = 0x8324BCF0;
	sub_821F3D58(ctx, base);
	// 8324BCF0: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324BCF4: 906A79A4  stw r3, 0x79a4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(31140 as u32), ctx.r[3].u32 ) };
	// 8324BCF8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324BCFC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324BD00: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324BD04: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324BD08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8324BD08 size=12
    let mut pc: u32 = 0x8324BD08;
    'dispatch: loop {
        match pc {
            0x8324BD08 => {
    //   block [0x8324BD08..0x8324BD14)
	// 8324BD08: 3D60832B  lis r11, -0x7cd5
	ctx.r[11].s64 = -2094333952;
	// 8324BD0C: 386B8498  addi r3, r11, -0x7b68
	ctx.r[3].s64 = ctx.r[11].s64 + -31592;
	// 8324BD10: 4BA5E210  b 0x82ca9f20
	sub_82CA9F20(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324BD18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324BD18 size=64
    let mut pc: u32 = 0x8324BD18;
    'dispatch: loop {
        match pc {
            0x8324BD18 => {
    //   block [0x8324BD18..0x8324BD58)
	// 8324BD18: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324BD1C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324BD20: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324BD24: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 8324BD28: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324BD2C: 388BAA40  addi r4, r11, -0x55c0
	ctx.r[4].s64 = ctx.r[11].s64 + -21952;
	// 8324BD30: 386A79B0  addi r3, r10, 0x79b0
	ctx.r[3].s64 = ctx.r[10].s64 + 31152;
	// 8324BD34: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324BD38: 4AFE1199  bl 0x8222ced0
	ctx.lr = 0x8324BD3C;
	sub_8222CED0(ctx, base);
	// 8324BD3C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8324BD40: 38698510  addi r3, r9, -0x7af0
	ctx.r[3].s64 = ctx.r[9].s64 + -31472;
	// 8324BD44: 4BA5E1DD  bl 0x82ca9f20
	ctx.lr = 0x8324BD48;
	sub_82CA9F20(ctx, base);
	// 8324BD48: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324BD4C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324BD50: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324BD54: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324BD58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324BD58 size=64
    let mut pc: u32 = 0x8324BD58;
    'dispatch: loop {
        match pc {
            0x8324BD58 => {
    //   block [0x8324BD58..0x8324BD98)
	// 8324BD58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324BD5C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324BD60: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324BD64: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8324BD68: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324BD6C: 388B1AD4  addi r4, r11, 0x1ad4
	ctx.r[4].s64 = ctx.r[11].s64 + 6868;
	// 8324BD70: 386A79B4  addi r3, r10, 0x79b4
	ctx.r[3].s64 = ctx.r[10].s64 + 31156;
	// 8324BD74: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324BD78: 4AFE1159  bl 0x8222ced0
	ctx.lr = 0x8324BD7C;
	sub_8222CED0(ctx, base);
	// 8324BD7C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8324BD80: 38698520  addi r3, r9, -0x7ae0
	ctx.r[3].s64 = ctx.r[9].s64 + -31456;
	// 8324BD84: 4BA5E19D  bl 0x82ca9f20
	ctx.lr = 0x8324BD88;
	sub_82CA9F20(ctx, base);
	// 8324BD88: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324BD8C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324BD90: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324BD94: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324BD98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324BD98 size=64
    let mut pc: u32 = 0x8324BD98;
    'dispatch: loop {
        match pc {
            0x8324BD98 => {
    //   block [0x8324BD98..0x8324BDD8)
	// 8324BD98: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324BD9C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324BDA0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324BDA4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8324BDA8: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324BDAC: 388B1ADC  addi r4, r11, 0x1adc
	ctx.r[4].s64 = ctx.r[11].s64 + 6876;
	// 8324BDB0: 386A79B8  addi r3, r10, 0x79b8
	ctx.r[3].s64 = ctx.r[10].s64 + 31160;
	// 8324BDB4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324BDB8: 4AFE1119  bl 0x8222ced0
	ctx.lr = 0x8324BDBC;
	sub_8222CED0(ctx, base);
	// 8324BDBC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8324BDC0: 38698530  addi r3, r9, -0x7ad0
	ctx.r[3].s64 = ctx.r[9].s64 + -31440;
	// 8324BDC4: 4BA5E15D  bl 0x82ca9f20
	ctx.lr = 0x8324BDC8;
	sub_82CA9F20(ctx, base);
	// 8324BDC8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324BDCC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324BDD0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324BDD4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324BDD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324BDD8 size=64
    let mut pc: u32 = 0x8324BDD8;
    'dispatch: loop {
        match pc {
            0x8324BDD8 => {
    //   block [0x8324BDD8..0x8324BE18)
	// 8324BDD8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324BDDC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324BDE0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324BDE4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8324BDE8: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324BDEC: 388B1AF0  addi r4, r11, 0x1af0
	ctx.r[4].s64 = ctx.r[11].s64 + 6896;
	// 8324BDF0: 386A79BC  addi r3, r10, 0x79bc
	ctx.r[3].s64 = ctx.r[10].s64 + 31164;
	// 8324BDF4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324BDF8: 4AFE10D9  bl 0x8222ced0
	ctx.lr = 0x8324BDFC;
	sub_8222CED0(ctx, base);
	// 8324BDFC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8324BE00: 38698540  addi r3, r9, -0x7ac0
	ctx.r[3].s64 = ctx.r[9].s64 + -31424;
	// 8324BE04: 4BA5E11D  bl 0x82ca9f20
	ctx.lr = 0x8324BE08;
	sub_82CA9F20(ctx, base);
	// 8324BE08: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324BE0C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324BE10: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324BE14: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324BE18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324BE18 size=64
    let mut pc: u32 = 0x8324BE18;
    'dispatch: loop {
        match pc {
            0x8324BE18 => {
    //   block [0x8324BE18..0x8324BE58)
	// 8324BE18: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324BE1C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324BE20: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324BE24: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 8324BE28: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324BE2C: 388BABA4  addi r4, r11, -0x545c
	ctx.r[4].s64 = ctx.r[11].s64 + -21596;
	// 8324BE30: 386A79C0  addi r3, r10, 0x79c0
	ctx.r[3].s64 = ctx.r[10].s64 + 31168;
	// 8324BE34: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324BE38: 4AFE1099  bl 0x8222ced0
	ctx.lr = 0x8324BE3C;
	sub_8222CED0(ctx, base);
	// 8324BE3C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8324BE40: 38698550  addi r3, r9, -0x7ab0
	ctx.r[3].s64 = ctx.r[9].s64 + -31408;
	// 8324BE44: 4BA5E0DD  bl 0x82ca9f20
	ctx.lr = 0x8324BE48;
	sub_82CA9F20(ctx, base);
	// 8324BE48: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324BE4C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324BE50: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324BE54: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324BE58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324BE58 size=96
    let mut pc: u32 = 0x8324BE58;
    'dispatch: loop {
        match pc {
            0x8324BE58 => {
    //   block [0x8324BE58..0x8324BE7C)
	// 8324BE58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324BE5C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324BE60: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324BE64: 3860000C  li r3, 0xc
	ctx.r[3].s64 = 12;
	// 8324BE68: 4AFD33F1  bl 0x8221f258
	ctx.lr = 0x8324BE6C;
	sub_8221F258(ctx, base);
	// 8324BE6C: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 8324BE70: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8324BE74: 419A0008  beq cr6, 0x8324be7c
	if ctx.cr[6].eq {
	pc = 0x8324BE7C; continue 'dispatch;
	}
	// 8324BE78: 916B0000  stw r11, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	pc = 0x8324BE7C; continue 'dispatch;
            }
            0x8324BE7C => {
    //   block [0x8324BE7C..0x8324BE88)
	// 8324BE7C: 354B0004  addic. r10, r11, 4
	ctx.xer.ca = (ctx.r[11].u32 > (!(4 as u32)));
	ctx.r[10].s64 = ctx.r[11].s64 + 4;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 8324BE80: 41820008  beq 0x8324be88
	if ctx.cr[0].eq {
	pc = 0x8324BE88; continue 'dispatch;
	}
	// 8324BE84: 916A0000  stw r11, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	pc = 0x8324BE88; continue 'dispatch;
            }
            0x8324BE88 => {
    //   block [0x8324BE88..0x8324BEB8)
	// 8324BE88: 3D208349  lis r9, -0x7cb7
	ctx.r[9].s64 = -2092367872;
	// 8324BE8C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8324BE90: 390979C4  addi r8, r9, 0x79c4
	ctx.r[8].s64 = ctx.r[9].s64 + 31172;
	// 8324BE94: 3CE0832B  lis r7, -0x7cd5
	ctx.r[7].s64 = -2094333952;
	// 8324BE98: 38678560  addi r3, r7, -0x7aa0
	ctx.r[3].s64 = ctx.r[7].s64 + -31392;
	// 8324BE9C: 91680004  stw r11, 4(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 8324BEA0: 91480008  stw r10, 8(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 8324BEA4: 4BA5E07D  bl 0x82ca9f20
	ctx.lr = 0x8324BEA8;
	sub_82CA9F20(ctx, base);
	// 8324BEA8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324BEAC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324BEB0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324BEB4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324BEB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8324BEB8 size=12
    let mut pc: u32 = 0x8324BEB8;
    'dispatch: loop {
        match pc {
            0x8324BEB8 => {
    //   block [0x8324BEB8..0x8324BEC4)
	// 8324BEB8: 3D60832B  lis r11, -0x7cd5
	ctx.r[11].s64 = -2094333952;
	// 8324BEBC: 386B85A8  addi r3, r11, -0x7a58
	ctx.r[3].s64 = ctx.r[11].s64 + -31320;
	// 8324BEC0: 4BA5E060  b 0x82ca9f20
	sub_82CA9F20(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324BEC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8324BEC8 size=48
    let mut pc: u32 = 0x8324BEC8;
    'dispatch: loop {
        match pc {
            0x8324BEC8 => {
    //   block [0x8324BEC8..0x8324BED0)
	// 8324BEC8: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 8324BECC: 390B7088  addi r8, r11, 0x7088
	ctx.r[8].s64 = ctx.r[11].s64 + 28808;
	pc = 0x8324BED0; continue 'dispatch;
            }
            0x8324BED0 => {
    //   block [0x8324BED0..0x8324BEF8)
	// 8324BED0: 7D2000A6  mfmsr r9
	ctx.r[9].u64 = ctx.msr;
	// 8324BED4: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8324BED8: 7D404028  lwarx r10, 0, r8
	// lwarx
	let ea = ctx.r[8].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 8324BEDC: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 8324BEE0: 7D40412D  stwcx. r10, 0, r8
	// stwcx.
	let addr = ctx.r[8].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[10].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 8324BEE4: 7D210164  mtmsrd r9, 1
	ctx.msr = (ctx.r[9].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8324BEE8: 4082FFE8  bne 0x8324bed0
	if !ctx.cr[0].eq {
	pc = 0x8324BED0; continue 'dispatch;
	}
	// 8324BEEC: 3CE0832B  lis r7, -0x7cd5
	ctx.r[7].s64 = -2094333952;
	// 8324BEF0: 38678600  addi r3, r7, -0x7a00
	ctx.r[3].s64 = ctx.r[7].s64 + -31232;
	// 8324BEF4: 4BA5E02C  b 0x82ca9f20
	sub_82CA9F20(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324BEF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8324BEF8 size=48
    let mut pc: u32 = 0x8324BEF8;
    'dispatch: loop {
        match pc {
            0x8324BEF8 => {
    //   block [0x8324BEF8..0x8324BF00)
	// 8324BEF8: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 8324BEFC: 390B7088  addi r8, r11, 0x7088
	ctx.r[8].s64 = ctx.r[11].s64 + 28808;
	pc = 0x8324BF00; continue 'dispatch;
            }
            0x8324BF00 => {
    //   block [0x8324BF00..0x8324BF28)
	// 8324BF00: 7D2000A6  mfmsr r9
	ctx.r[9].u64 = ctx.msr;
	// 8324BF04: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8324BF08: 7D404028  lwarx r10, 0, r8
	// lwarx
	let ea = ctx.r[8].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 8324BF0C: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 8324BF10: 7D40412D  stwcx. r10, 0, r8
	// stwcx.
	let addr = ctx.r[8].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[10].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 8324BF14: 7D210164  mtmsrd r9, 1
	ctx.msr = (ctx.r[9].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8324BF18: 4082FFE8  bne 0x8324bf00
	if !ctx.cr[0].eq {
	pc = 0x8324BF00; continue 'dispatch;
	}
	// 8324BF1C: 3CE0832B  lis r7, -0x7cd5
	ctx.r[7].s64 = -2094333952;
	// 8324BF20: 38678610  addi r3, r7, -0x79f0
	ctx.r[3].s64 = ctx.r[7].s64 + -31216;
	// 8324BF24: 4BA5DFFC  b 0x82ca9f20
	sub_82CA9F20(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324BF28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8324BF28 size=12
    let mut pc: u32 = 0x8324BF28;
    'dispatch: loop {
        match pc {
            0x8324BF28 => {
    //   block [0x8324BF28..0x8324BF34)
	// 8324BF28: 3D60832B  lis r11, -0x7cd5
	ctx.r[11].s64 = -2094333952;
	// 8324BF2C: 386B8620  addi r3, r11, -0x79e0
	ctx.r[3].s64 = ctx.r[11].s64 + -31200;
	// 8324BF30: 4BA5DFF0  b 0x82ca9f20
	sub_82CA9F20(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324BF38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8324BF38 size=48
    let mut pc: u32 = 0x8324BF38;
    'dispatch: loop {
        match pc {
            0x8324BF38 => {
    //   block [0x8324BF38..0x8324BF40)
	// 8324BF38: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 8324BF3C: 390B7088  addi r8, r11, 0x7088
	ctx.r[8].s64 = ctx.r[11].s64 + 28808;
	pc = 0x8324BF40; continue 'dispatch;
            }
            0x8324BF40 => {
    //   block [0x8324BF40..0x8324BF68)
	// 8324BF40: 7D2000A6  mfmsr r9
	ctx.r[9].u64 = ctx.msr;
	// 8324BF44: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8324BF48: 7D404028  lwarx r10, 0, r8
	// lwarx
	let ea = ctx.r[8].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 8324BF4C: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 8324BF50: 7D40412D  stwcx. r10, 0, r8
	// stwcx.
	let addr = ctx.r[8].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[10].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 8324BF54: 7D210164  mtmsrd r9, 1
	ctx.msr = (ctx.r[9].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8324BF58: 4082FFE8  bne 0x8324bf40
	if !ctx.cr[0].eq {
	pc = 0x8324BF40; continue 'dispatch;
	}
	// 8324BF5C: 3CE0832B  lis r7, -0x7cd5
	ctx.r[7].s64 = -2094333952;
	// 8324BF60: 38678630  addi r3, r7, -0x79d0
	ctx.r[3].s64 = ctx.r[7].s64 + -31184;
	// 8324BF64: 4BA5DFBC  b 0x82ca9f20
	sub_82CA9F20(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324BF68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8324BF68 size=48
    let mut pc: u32 = 0x8324BF68;
    'dispatch: loop {
        match pc {
            0x8324BF68 => {
    //   block [0x8324BF68..0x8324BF70)
	// 8324BF68: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 8324BF6C: 390B7088  addi r8, r11, 0x7088
	ctx.r[8].s64 = ctx.r[11].s64 + 28808;
	pc = 0x8324BF70; continue 'dispatch;
            }
            0x8324BF70 => {
    //   block [0x8324BF70..0x8324BF98)
	// 8324BF70: 7D2000A6  mfmsr r9
	ctx.r[9].u64 = ctx.msr;
	// 8324BF74: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8324BF78: 7D404028  lwarx r10, 0, r8
	// lwarx
	let ea = ctx.r[8].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 8324BF7C: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 8324BF80: 7D40412D  stwcx. r10, 0, r8
	// stwcx.
	let addr = ctx.r[8].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[10].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 8324BF84: 7D210164  mtmsrd r9, 1
	ctx.msr = (ctx.r[9].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8324BF88: 4082FFE8  bne 0x8324bf70
	if !ctx.cr[0].eq {
	pc = 0x8324BF70; continue 'dispatch;
	}
	// 8324BF8C: 3CE0832B  lis r7, -0x7cd5
	ctx.r[7].s64 = -2094333952;
	// 8324BF90: 38678640  addi r3, r7, -0x79c0
	ctx.r[3].s64 = ctx.r[7].s64 + -31168;
	// 8324BF94: 4BA5DF8C  b 0x82ca9f20
	sub_82CA9F20(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324BF98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324BF98 size=64
    let mut pc: u32 = 0x8324BF98;
    'dispatch: loop {
        match pc {
            0x8324BF98 => {
    //   block [0x8324BF98..0x8324BFD8)
	// 8324BF98: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324BF9C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324BFA0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324BFA4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8324BFA8: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324BFAC: 388B1AD4  addi r4, r11, 0x1ad4
	ctx.r[4].s64 = ctx.r[11].s64 + 6868;
	// 8324BFB0: 386A7A10  addi r3, r10, 0x7a10
	ctx.r[3].s64 = ctx.r[10].s64 + 31248;
	// 8324BFB4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324BFB8: 4AFE0F19  bl 0x8222ced0
	ctx.lr = 0x8324BFBC;
	sub_8222CED0(ctx, base);
	// 8324BFBC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8324BFC0: 38698650  addi r3, r9, -0x79b0
	ctx.r[3].s64 = ctx.r[9].s64 + -31152;
	// 8324BFC4: 4BA5DF5D  bl 0x82ca9f20
	ctx.lr = 0x8324BFC8;
	sub_82CA9F20(ctx, base);
	// 8324BFC8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324BFCC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324BFD0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324BFD4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324BFD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324BFD8 size=64
    let mut pc: u32 = 0x8324BFD8;
    'dispatch: loop {
        match pc {
            0x8324BFD8 => {
    //   block [0x8324BFD8..0x8324C018)
	// 8324BFD8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324BFDC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324BFE0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324BFE4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8324BFE8: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324BFEC: 388B1ADC  addi r4, r11, 0x1adc
	ctx.r[4].s64 = ctx.r[11].s64 + 6876;
	// 8324BFF0: 386A7A14  addi r3, r10, 0x7a14
	ctx.r[3].s64 = ctx.r[10].s64 + 31252;
	// 8324BFF4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324BFF8: 4AFE0ED9  bl 0x8222ced0
	ctx.lr = 0x8324BFFC;
	sub_8222CED0(ctx, base);
	// 8324BFFC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8324C000: 38698660  addi r3, r9, -0x79a0
	ctx.r[3].s64 = ctx.r[9].s64 + -31136;
	// 8324C004: 4BA5DF1D  bl 0x82ca9f20
	ctx.lr = 0x8324C008;
	sub_82CA9F20(ctx, base);
	// 8324C008: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324C00C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324C010: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324C014: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324C018(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324C018 size=64
    let mut pc: u32 = 0x8324C018;
    'dispatch: loop {
        match pc {
            0x8324C018 => {
    //   block [0x8324C018..0x8324C058)
	// 8324C018: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324C01C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324C020: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324C024: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8324C028: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324C02C: 388B1AF0  addi r4, r11, 0x1af0
	ctx.r[4].s64 = ctx.r[11].s64 + 6896;
	// 8324C030: 386A7A18  addi r3, r10, 0x7a18
	ctx.r[3].s64 = ctx.r[10].s64 + 31256;
	// 8324C034: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324C038: 4AFE0E99  bl 0x8222ced0
	ctx.lr = 0x8324C03C;
	sub_8222CED0(ctx, base);
	// 8324C03C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8324C040: 38698670  addi r3, r9, -0x7990
	ctx.r[3].s64 = ctx.r[9].s64 + -31120;
	// 8324C044: 4BA5DEDD  bl 0x82ca9f20
	ctx.lr = 0x8324C048;
	sub_82CA9F20(ctx, base);
	// 8324C048: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324C04C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324C050: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324C054: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324C058(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324C058 size=64
    let mut pc: u32 = 0x8324C058;
    'dispatch: loop {
        match pc {
            0x8324C058 => {
    //   block [0x8324C058..0x8324C098)
	// 8324C058: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324C05C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324C060: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324C064: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 8324C068: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324C06C: 388BD960  addi r4, r11, -0x26a0
	ctx.r[4].s64 = ctx.r[11].s64 + -9888;
	// 8324C070: 386A7A1C  addi r3, r10, 0x7a1c
	ctx.r[3].s64 = ctx.r[10].s64 + 31260;
	// 8324C074: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324C078: 4AFE0E59  bl 0x8222ced0
	ctx.lr = 0x8324C07C;
	sub_8222CED0(ctx, base);
	// 8324C07C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8324C080: 386986D8  addi r3, r9, -0x7928
	ctx.r[3].s64 = ctx.r[9].s64 + -31016;
	// 8324C084: 4BA5DE9D  bl 0x82ca9f20
	ctx.lr = 0x8324C088;
	sub_82CA9F20(ctx, base);
	// 8324C088: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324C08C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324C090: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324C094: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324C098(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324C098 size=64
    let mut pc: u32 = 0x8324C098;
    'dispatch: loop {
        match pc {
            0x8324C098 => {
    //   block [0x8324C098..0x8324C0D8)
	// 8324C098: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324C09C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324C0A0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324C0A4: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 8324C0A8: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324C0AC: 388BD978  addi r4, r11, -0x2688
	ctx.r[4].s64 = ctx.r[11].s64 + -9864;
	// 8324C0B0: 386A7A20  addi r3, r10, 0x7a20
	ctx.r[3].s64 = ctx.r[10].s64 + 31264;
	// 8324C0B4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324C0B8: 4AFE0E19  bl 0x8222ced0
	ctx.lr = 0x8324C0BC;
	sub_8222CED0(ctx, base);
	// 8324C0BC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8324C0C0: 386986E8  addi r3, r9, -0x7918
	ctx.r[3].s64 = ctx.r[9].s64 + -31000;
	// 8324C0C4: 4BA5DE5D  bl 0x82ca9f20
	ctx.lr = 0x8324C0C8;
	sub_82CA9F20(ctx, base);
	// 8324C0C8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324C0CC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324C0D0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324C0D4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324C0D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324C0D8 size=64
    let mut pc: u32 = 0x8324C0D8;
    'dispatch: loop {
        match pc {
            0x8324C0D8 => {
    //   block [0x8324C0D8..0x8324C118)
	// 8324C0D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324C0DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324C0E0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324C0E4: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 8324C0E8: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324C0EC: 388BD98C  addi r4, r11, -0x2674
	ctx.r[4].s64 = ctx.r[11].s64 + -9844;
	// 8324C0F0: 386A7A24  addi r3, r10, 0x7a24
	ctx.r[3].s64 = ctx.r[10].s64 + 31268;
	// 8324C0F4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324C0F8: 4AFE0DD9  bl 0x8222ced0
	ctx.lr = 0x8324C0FC;
	sub_8222CED0(ctx, base);
	// 8324C0FC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8324C100: 386986F8  addi r3, r9, -0x7908
	ctx.r[3].s64 = ctx.r[9].s64 + -30984;
	// 8324C104: 4BA5DE1D  bl 0x82ca9f20
	ctx.lr = 0x8324C108;
	sub_82CA9F20(ctx, base);
	// 8324C108: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324C10C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324C110: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324C114: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324C118(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324C118 size=64
    let mut pc: u32 = 0x8324C118;
    'dispatch: loop {
        match pc {
            0x8324C118 => {
    //   block [0x8324C118..0x8324C158)
	// 8324C118: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324C11C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324C120: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324C124: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8324C128: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324C12C: 388B052C  addi r4, r11, 0x52c
	ctx.r[4].s64 = ctx.r[11].s64 + 1324;
	// 8324C130: 386A7A28  addi r3, r10, 0x7a28
	ctx.r[3].s64 = ctx.r[10].s64 + 31272;
	// 8324C134: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324C138: 4AFE0D99  bl 0x8222ced0
	ctx.lr = 0x8324C13C;
	sub_8222CED0(ctx, base);
	// 8324C13C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8324C140: 38698708  addi r3, r9, -0x78f8
	ctx.r[3].s64 = ctx.r[9].s64 + -30968;
	// 8324C144: 4BA5DDDD  bl 0x82ca9f20
	ctx.lr = 0x8324C148;
	sub_82CA9F20(ctx, base);
	// 8324C148: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324C14C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324C150: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324C154: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324C158(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324C158 size=56
    let mut pc: u32 = 0x8324C158;
    'dispatch: loop {
        match pc {
            0x8324C158 => {
    //   block [0x8324C158..0x8324C190)
	// 8324C158: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324C15C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324C160: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324C164: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8324C168: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8324C16C: 386B42C8  addi r3, r11, 0x42c8
	ctx.r[3].s64 = ctx.r[11].s64 + 17096;
	// 8324C170: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8324C174: 4AFA7BE5  bl 0x821f3d58
	ctx.lr = 0x8324C178;
	sub_821F3D58(ctx, base);
	// 8324C178: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324C17C: 906A7A2C  stw r3, 0x7a2c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(31276 as u32), ctx.r[3].u32 ) };
	// 8324C180: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324C184: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324C188: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324C18C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324C190(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324C190 size=56
    let mut pc: u32 = 0x8324C190;
    'dispatch: loop {
        match pc {
            0x8324C190 => {
    //   block [0x8324C190..0x8324C1C8)
	// 8324C190: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324C194: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324C198: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324C19C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8324C1A0: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8324C1A4: 386B42DC  addi r3, r11, 0x42dc
	ctx.r[3].s64 = ctx.r[11].s64 + 17116;
	// 8324C1A8: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8324C1AC: 4AFA7BAD  bl 0x821f3d58
	ctx.lr = 0x8324C1B0;
	sub_821F3D58(ctx, base);
	// 8324C1B0: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324C1B4: 906A7A30  stw r3, 0x7a30(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(31280 as u32), ctx.r[3].u32 ) };
	// 8324C1B8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324C1BC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324C1C0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324C1C4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324C1C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324C1C8 size=56
    let mut pc: u32 = 0x8324C1C8;
    'dispatch: loop {
        match pc {
            0x8324C1C8 => {
    //   block [0x8324C1C8..0x8324C200)
	// 8324C1C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324C1CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324C1D0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324C1D4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8324C1D8: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8324C1DC: 386B42E8  addi r3, r11, 0x42e8
	ctx.r[3].s64 = ctx.r[11].s64 + 17128;
	// 8324C1E0: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8324C1E4: 4AFA7B75  bl 0x821f3d58
	ctx.lr = 0x8324C1E8;
	sub_821F3D58(ctx, base);
	// 8324C1E8: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324C1EC: 906A7A34  stw r3, 0x7a34(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(31284 as u32), ctx.r[3].u32 ) };
	// 8324C1F0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324C1F4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324C1F8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324C1FC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324C200(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324C200 size=56
    let mut pc: u32 = 0x8324C200;
    'dispatch: loop {
        match pc {
            0x8324C200 => {
    //   block [0x8324C200..0x8324C238)
	// 8324C200: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324C204: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324C208: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324C20C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8324C210: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8324C214: 386B4300  addi r3, r11, 0x4300
	ctx.r[3].s64 = ctx.r[11].s64 + 17152;
	// 8324C218: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8324C21C: 4AFA7B3D  bl 0x821f3d58
	ctx.lr = 0x8324C220;
	sub_821F3D58(ctx, base);
	// 8324C220: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324C224: 906A7A38  stw r3, 0x7a38(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(31288 as u32), ctx.r[3].u32 ) };
	// 8324C228: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324C22C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324C230: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324C234: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324C238(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324C238 size=56
    let mut pc: u32 = 0x8324C238;
    'dispatch: loop {
        match pc {
            0x8324C238 => {
    //   block [0x8324C238..0x8324C270)
	// 8324C238: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324C23C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324C240: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324C244: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8324C248: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8324C24C: 386B4310  addi r3, r11, 0x4310
	ctx.r[3].s64 = ctx.r[11].s64 + 17168;
	// 8324C250: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8324C254: 4AFA7B05  bl 0x821f3d58
	ctx.lr = 0x8324C258;
	sub_821F3D58(ctx, base);
	// 8324C258: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324C25C: 906A7A3C  stw r3, 0x7a3c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(31292 as u32), ctx.r[3].u32 ) };
	// 8324C260: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324C264: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324C268: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324C26C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324C270(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324C270 size=56
    let mut pc: u32 = 0x8324C270;
    'dispatch: loop {
        match pc {
            0x8324C270 => {
    //   block [0x8324C270..0x8324C2A8)
	// 8324C270: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324C274: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324C278: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324C27C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8324C280: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8324C284: 386B4328  addi r3, r11, 0x4328
	ctx.r[3].s64 = ctx.r[11].s64 + 17192;
	// 8324C288: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8324C28C: 4AFA7ACD  bl 0x821f3d58
	ctx.lr = 0x8324C290;
	sub_821F3D58(ctx, base);
	// 8324C290: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324C294: 906A7A40  stw r3, 0x7a40(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(31296 as u32), ctx.r[3].u32 ) };
	// 8324C298: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324C29C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324C2A0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324C2A4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324C2A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324C2A8 size=56
    let mut pc: u32 = 0x8324C2A8;
    'dispatch: loop {
        match pc {
            0x8324C2A8 => {
    //   block [0x8324C2A8..0x8324C2E0)
	// 8324C2A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324C2AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324C2B0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324C2B4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8324C2B8: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8324C2BC: 386B4338  addi r3, r11, 0x4338
	ctx.r[3].s64 = ctx.r[11].s64 + 17208;
	// 8324C2C0: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8324C2C4: 4AFA7A95  bl 0x821f3d58
	ctx.lr = 0x8324C2C8;
	sub_821F3D58(ctx, base);
	// 8324C2C8: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324C2CC: 906A7A44  stw r3, 0x7a44(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(31300 as u32), ctx.r[3].u32 ) };
	// 8324C2D0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324C2D4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324C2D8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324C2DC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324C2E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324C2E0 size=56
    let mut pc: u32 = 0x8324C2E0;
    'dispatch: loop {
        match pc {
            0x8324C2E0 => {
    //   block [0x8324C2E0..0x8324C318)
	// 8324C2E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324C2E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324C2E8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324C2EC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8324C2F0: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8324C2F4: 386B4350  addi r3, r11, 0x4350
	ctx.r[3].s64 = ctx.r[11].s64 + 17232;
	// 8324C2F8: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8324C2FC: 4AFA7A5D  bl 0x821f3d58
	ctx.lr = 0x8324C300;
	sub_821F3D58(ctx, base);
	// 8324C300: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324C304: 906A7A48  stw r3, 0x7a48(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(31304 as u32), ctx.r[3].u32 ) };
	// 8324C308: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324C30C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324C310: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324C314: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324C318(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324C318 size=56
    let mut pc: u32 = 0x8324C318;
    'dispatch: loop {
        match pc {
            0x8324C318 => {
    //   block [0x8324C318..0x8324C350)
	// 8324C318: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324C31C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324C320: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324C324: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8324C328: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8324C32C: 386B4368  addi r3, r11, 0x4368
	ctx.r[3].s64 = ctx.r[11].s64 + 17256;
	// 8324C330: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8324C334: 4AFA7A25  bl 0x821f3d58
	ctx.lr = 0x8324C338;
	sub_821F3D58(ctx, base);
	// 8324C338: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324C33C: 906A7A4C  stw r3, 0x7a4c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(31308 as u32), ctx.r[3].u32 ) };
	// 8324C340: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324C344: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324C348: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324C34C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324C350(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324C350 size=56
    let mut pc: u32 = 0x8324C350;
    'dispatch: loop {
        match pc {
            0x8324C350 => {
    //   block [0x8324C350..0x8324C388)
	// 8324C350: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324C354: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324C358: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324C35C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8324C360: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8324C364: 386B4380  addi r3, r11, 0x4380
	ctx.r[3].s64 = ctx.r[11].s64 + 17280;
	// 8324C368: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8324C36C: 4AFA79ED  bl 0x821f3d58
	ctx.lr = 0x8324C370;
	sub_821F3D58(ctx, base);
	// 8324C370: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324C374: 906A7A50  stw r3, 0x7a50(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(31312 as u32), ctx.r[3].u32 ) };
	// 8324C378: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324C37C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324C380: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324C384: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324C388(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324C388 size=56
    let mut pc: u32 = 0x8324C388;
    'dispatch: loop {
        match pc {
            0x8324C388 => {
    //   block [0x8324C388..0x8324C3C0)
	// 8324C388: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324C38C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324C390: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324C394: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8324C398: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8324C39C: 386B4394  addi r3, r11, 0x4394
	ctx.r[3].s64 = ctx.r[11].s64 + 17300;
	// 8324C3A0: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8324C3A4: 4AFA79B5  bl 0x821f3d58
	ctx.lr = 0x8324C3A8;
	sub_821F3D58(ctx, base);
	// 8324C3A8: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324C3AC: 906A7A54  stw r3, 0x7a54(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(31316 as u32), ctx.r[3].u32 ) };
	// 8324C3B0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324C3B4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324C3B8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324C3BC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324C3C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324C3C0 size=56
    let mut pc: u32 = 0x8324C3C0;
    'dispatch: loop {
        match pc {
            0x8324C3C0 => {
    //   block [0x8324C3C0..0x8324C3F8)
	// 8324C3C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324C3C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324C3C8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324C3CC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8324C3D0: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8324C3D4: 386B43B4  addi r3, r11, 0x43b4
	ctx.r[3].s64 = ctx.r[11].s64 + 17332;
	// 8324C3D8: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8324C3DC: 4AFA797D  bl 0x821f3d58
	ctx.lr = 0x8324C3E0;
	sub_821F3D58(ctx, base);
	// 8324C3E0: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324C3E4: 906A7A58  stw r3, 0x7a58(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(31320 as u32), ctx.r[3].u32 ) };
	// 8324C3E8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324C3EC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324C3F0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324C3F4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324C3F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324C3F8 size=56
    let mut pc: u32 = 0x8324C3F8;
    'dispatch: loop {
        match pc {
            0x8324C3F8 => {
    //   block [0x8324C3F8..0x8324C430)
	// 8324C3F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324C3FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324C400: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324C404: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8324C408: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8324C40C: 386B43C8  addi r3, r11, 0x43c8
	ctx.r[3].s64 = ctx.r[11].s64 + 17352;
	// 8324C410: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8324C414: 4AFA7945  bl 0x821f3d58
	ctx.lr = 0x8324C418;
	sub_821F3D58(ctx, base);
	// 8324C418: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324C41C: 906A7A5C  stw r3, 0x7a5c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(31324 as u32), ctx.r[3].u32 ) };
	// 8324C420: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324C424: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324C428: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324C42C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324C430(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324C430 size=56
    let mut pc: u32 = 0x8324C430;
    'dispatch: loop {
        match pc {
            0x8324C430 => {
    //   block [0x8324C430..0x8324C468)
	// 8324C430: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324C434: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324C438: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324C43C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8324C440: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8324C444: 386B43D4  addi r3, r11, 0x43d4
	ctx.r[3].s64 = ctx.r[11].s64 + 17364;
	// 8324C448: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8324C44C: 4AFA790D  bl 0x821f3d58
	ctx.lr = 0x8324C450;
	sub_821F3D58(ctx, base);
	// 8324C450: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324C454: 906A7A60  stw r3, 0x7a60(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(31328 as u32), ctx.r[3].u32 ) };
	// 8324C458: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324C45C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324C460: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324C464: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324C468(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324C468 size=56
    let mut pc: u32 = 0x8324C468;
    'dispatch: loop {
        match pc {
            0x8324C468 => {
    //   block [0x8324C468..0x8324C4A0)
	// 8324C468: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324C46C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324C470: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324C474: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8324C478: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8324C47C: 386B43E0  addi r3, r11, 0x43e0
	ctx.r[3].s64 = ctx.r[11].s64 + 17376;
	// 8324C480: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8324C484: 4AFA78D5  bl 0x821f3d58
	ctx.lr = 0x8324C488;
	sub_821F3D58(ctx, base);
	// 8324C488: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324C48C: 906A7A64  stw r3, 0x7a64(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(31332 as u32), ctx.r[3].u32 ) };
	// 8324C490: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324C494: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324C498: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324C49C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324C4A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324C4A0 size=56
    let mut pc: u32 = 0x8324C4A0;
    'dispatch: loop {
        match pc {
            0x8324C4A0 => {
    //   block [0x8324C4A0..0x8324C4D8)
	// 8324C4A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324C4A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324C4A8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324C4AC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8324C4B0: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8324C4B4: 386B43F0  addi r3, r11, 0x43f0
	ctx.r[3].s64 = ctx.r[11].s64 + 17392;
	// 8324C4B8: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8324C4BC: 4AFA789D  bl 0x821f3d58
	ctx.lr = 0x8324C4C0;
	sub_821F3D58(ctx, base);
	// 8324C4C0: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324C4C4: 906A7A68  stw r3, 0x7a68(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(31336 as u32), ctx.r[3].u32 ) };
	// 8324C4C8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324C4CC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324C4D0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324C4D4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324C4D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324C4D8 size=56
    let mut pc: u32 = 0x8324C4D8;
    'dispatch: loop {
        match pc {
            0x8324C4D8 => {
    //   block [0x8324C4D8..0x8324C510)
	// 8324C4D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324C4DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324C4E0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324C4E4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8324C4E8: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8324C4EC: 386B4400  addi r3, r11, 0x4400
	ctx.r[3].s64 = ctx.r[11].s64 + 17408;
	// 8324C4F0: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8324C4F4: 4AFA7865  bl 0x821f3d58
	ctx.lr = 0x8324C4F8;
	sub_821F3D58(ctx, base);
	// 8324C4F8: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324C4FC: 906A7A6C  stw r3, 0x7a6c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(31340 as u32), ctx.r[3].u32 ) };
	// 8324C500: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324C504: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324C508: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324C50C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324C510(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324C510 size=56
    let mut pc: u32 = 0x8324C510;
    'dispatch: loop {
        match pc {
            0x8324C510 => {
    //   block [0x8324C510..0x8324C548)
	// 8324C510: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324C514: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324C518: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324C51C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8324C520: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8324C524: 386B4418  addi r3, r11, 0x4418
	ctx.r[3].s64 = ctx.r[11].s64 + 17432;
	// 8324C528: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8324C52C: 4AFA782D  bl 0x821f3d58
	ctx.lr = 0x8324C530;
	sub_821F3D58(ctx, base);
	// 8324C530: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324C534: 906A7A70  stw r3, 0x7a70(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(31344 as u32), ctx.r[3].u32 ) };
	// 8324C538: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324C53C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324C540: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324C544: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324C548(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324C548 size=56
    let mut pc: u32 = 0x8324C548;
    'dispatch: loop {
        match pc {
            0x8324C548 => {
    //   block [0x8324C548..0x8324C580)
	// 8324C548: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324C54C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324C550: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324C554: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8324C558: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8324C55C: 386B4424  addi r3, r11, 0x4424
	ctx.r[3].s64 = ctx.r[11].s64 + 17444;
	// 8324C560: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8324C564: 4AFA77F5  bl 0x821f3d58
	ctx.lr = 0x8324C568;
	sub_821F3D58(ctx, base);
	// 8324C568: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324C56C: 906A7A74  stw r3, 0x7a74(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(31348 as u32), ctx.r[3].u32 ) };
	// 8324C570: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324C574: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324C578: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324C57C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324C580(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324C580 size=56
    let mut pc: u32 = 0x8324C580;
    'dispatch: loop {
        match pc {
            0x8324C580 => {
    //   block [0x8324C580..0x8324C5B8)
	// 8324C580: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324C584: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324C588: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324C58C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8324C590: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8324C594: 386B4430  addi r3, r11, 0x4430
	ctx.r[3].s64 = ctx.r[11].s64 + 17456;
	// 8324C598: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8324C59C: 4AFA77BD  bl 0x821f3d58
	ctx.lr = 0x8324C5A0;
	sub_821F3D58(ctx, base);
	// 8324C5A0: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324C5A4: 906A7A78  stw r3, 0x7a78(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(31352 as u32), ctx.r[3].u32 ) };
	// 8324C5A8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324C5AC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324C5B0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324C5B4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324C5B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324C5B8 size=56
    let mut pc: u32 = 0x8324C5B8;
    'dispatch: loop {
        match pc {
            0x8324C5B8 => {
    //   block [0x8324C5B8..0x8324C5F0)
	// 8324C5B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324C5BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324C5C0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324C5C4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8324C5C8: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8324C5CC: 386B4440  addi r3, r11, 0x4440
	ctx.r[3].s64 = ctx.r[11].s64 + 17472;
	// 8324C5D0: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8324C5D4: 4AFA7785  bl 0x821f3d58
	ctx.lr = 0x8324C5D8;
	sub_821F3D58(ctx, base);
	// 8324C5D8: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324C5DC: 906A7A7C  stw r3, 0x7a7c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(31356 as u32), ctx.r[3].u32 ) };
	// 8324C5E0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324C5E4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324C5E8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324C5EC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324C5F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324C5F0 size=376
    let mut pc: u32 = 0x8324C5F0;
    'dispatch: loop {
        match pc {
            0x8324C5F0 => {
    //   block [0x8324C5F0..0x8324C768)
	// 8324C5F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324C5F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324C5F8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8324C5FC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324C600: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 8324C604: 3D40820B  lis r10, -0x7df5
	ctx.r[10].s64 = -2113208320;
	// 8324C608: 3BEB7A80  addi r31, r11, 0x7a80
	ctx.r[31].s64 = ctx.r[11].s64 + 31360;
	// 8324C60C: 388A8A50  addi r4, r10, -0x75b0
	ctx.r[4].s64 = ctx.r[10].s64 + -30128;
	// 8324C610: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8324C614: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324C618: 4AFE08B9  bl 0x8222ced0
	ctx.lr = 0x8324C61C;
	sub_8222CED0(ctx, base);
	// 8324C61C: 3D20820B  lis r9, -0x7df5
	ctx.r[9].s64 = -2113208320;
	// 8324C620: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324C624: 38898A30  addi r4, r9, -0x75d0
	ctx.r[4].s64 = ctx.r[9].s64 + -30160;
	// 8324C628: 387F0004  addi r3, r31, 4
	ctx.r[3].s64 = ctx.r[31].s64 + 4;
	// 8324C62C: 4AFE08A5  bl 0x8222ced0
	ctx.lr = 0x8324C630;
	sub_8222CED0(ctx, base);
	// 8324C630: 3D00820B  lis r8, -0x7df5
	ctx.r[8].s64 = -2113208320;
	// 8324C634: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324C638: 38888A10  addi r4, r8, -0x75f0
	ctx.r[4].s64 = ctx.r[8].s64 + -30192;
	// 8324C63C: 387F0008  addi r3, r31, 8
	ctx.r[3].s64 = ctx.r[31].s64 + 8;
	// 8324C640: 4AFE0891  bl 0x8222ced0
	ctx.lr = 0x8324C644;
	sub_8222CED0(ctx, base);
	// 8324C644: 3CE0820B  lis r7, -0x7df5
	ctx.r[7].s64 = -2113208320;
	// 8324C648: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324C64C: 388789F0  addi r4, r7, -0x7610
	ctx.r[4].s64 = ctx.r[7].s64 + -30224;
	// 8324C650: 387F000C  addi r3, r31, 0xc
	ctx.r[3].s64 = ctx.r[31].s64 + 12;
	// 8324C654: 4AFE087D  bl 0x8222ced0
	ctx.lr = 0x8324C658;
	sub_8222CED0(ctx, base);
	// 8324C658: 3CC0820B  lis r6, -0x7df5
	ctx.r[6].s64 = -2113208320;
	// 8324C65C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324C660: 388689D0  addi r4, r6, -0x7630
	ctx.r[4].s64 = ctx.r[6].s64 + -30256;
	// 8324C664: 387F0010  addi r3, r31, 0x10
	ctx.r[3].s64 = ctx.r[31].s64 + 16;
	// 8324C668: 4AFE0869  bl 0x8222ced0
	ctx.lr = 0x8324C66C;
	sub_8222CED0(ctx, base);
	// 8324C66C: 3C80820B  lis r4, -0x7df5
	ctx.r[4].s64 = -2113208320;
	// 8324C670: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324C674: 388489B0  addi r4, r4, -0x7650
	ctx.r[4].s64 = ctx.r[4].s64 + -30288;
	// 8324C678: 387F0014  addi r3, r31, 0x14
	ctx.r[3].s64 = ctx.r[31].s64 + 20;
	// 8324C67C: 4AFE0855  bl 0x8222ced0
	ctx.lr = 0x8324C680;
	sub_8222CED0(ctx, base);
	// 8324C680: 3C60820B  lis r3, -0x7df5
	ctx.r[3].s64 = -2113208320;
	// 8324C684: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324C688: 38838990  addi r4, r3, -0x7670
	ctx.r[4].s64 = ctx.r[3].s64 + -30320;
	// 8324C68C: 387F0018  addi r3, r31, 0x18
	ctx.r[3].s64 = ctx.r[31].s64 + 24;
	// 8324C690: 4AFE0841  bl 0x8222ced0
	ctx.lr = 0x8324C694;
	sub_8222CED0(ctx, base);
	// 8324C694: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 8324C698: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324C69C: 388B8970  addi r4, r11, -0x7690
	ctx.r[4].s64 = ctx.r[11].s64 + -30352;
	// 8324C6A0: 387F001C  addi r3, r31, 0x1c
	ctx.r[3].s64 = ctx.r[31].s64 + 28;
	// 8324C6A4: 4AFE082D  bl 0x8222ced0
	ctx.lr = 0x8324C6A8;
	sub_8222CED0(ctx, base);
	// 8324C6A8: 3D40820B  lis r10, -0x7df5
	ctx.r[10].s64 = -2113208320;
	// 8324C6AC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324C6B0: 388A8950  addi r4, r10, -0x76b0
	ctx.r[4].s64 = ctx.r[10].s64 + -30384;
	// 8324C6B4: 387F0020  addi r3, r31, 0x20
	ctx.r[3].s64 = ctx.r[31].s64 + 32;
	// 8324C6B8: 4AFE0819  bl 0x8222ced0
	ctx.lr = 0x8324C6BC;
	sub_8222CED0(ctx, base);
	// 8324C6BC: 3D20820B  lis r9, -0x7df5
	ctx.r[9].s64 = -2113208320;
	// 8324C6C0: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324C6C4: 38898930  addi r4, r9, -0x76d0
	ctx.r[4].s64 = ctx.r[9].s64 + -30416;
	// 8324C6C8: 387F0024  addi r3, r31, 0x24
	ctx.r[3].s64 = ctx.r[31].s64 + 36;
	// 8324C6CC: 4AFE0805  bl 0x8222ced0
	ctx.lr = 0x8324C6D0;
	sub_8222CED0(ctx, base);
	// 8324C6D0: 3D00820B  lis r8, -0x7df5
	ctx.r[8].s64 = -2113208320;
	// 8324C6D4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324C6D8: 38888914  addi r4, r8, -0x76ec
	ctx.r[4].s64 = ctx.r[8].s64 + -30444;
	// 8324C6DC: 387F0028  addi r3, r31, 0x28
	ctx.r[3].s64 = ctx.r[31].s64 + 40;
	// 8324C6E0: 4AFE07F1  bl 0x8222ced0
	ctx.lr = 0x8324C6E4;
	sub_8222CED0(ctx, base);
	// 8324C6E4: 3CE0820B  lis r7, -0x7df5
	ctx.r[7].s64 = -2113208320;
	// 8324C6E8: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324C6EC: 388788F0  addi r4, r7, -0x7710
	ctx.r[4].s64 = ctx.r[7].s64 + -30480;
	// 8324C6F0: 387F002C  addi r3, r31, 0x2c
	ctx.r[3].s64 = ctx.r[31].s64 + 44;
	// 8324C6F4: 4AFE07DD  bl 0x8222ced0
	ctx.lr = 0x8324C6F8;
	sub_8222CED0(ctx, base);
	// 8324C6F8: 3CC0820B  lis r6, -0x7df5
	ctx.r[6].s64 = -2113208320;
	// 8324C6FC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324C700: 388688CC  addi r4, r6, -0x7734
	ctx.r[4].s64 = ctx.r[6].s64 + -30516;
	// 8324C704: 387F0030  addi r3, r31, 0x30
	ctx.r[3].s64 = ctx.r[31].s64 + 48;
	// 8324C708: 4AFE07C9  bl 0x8222ced0
	ctx.lr = 0x8324C70C;
	sub_8222CED0(ctx, base);
	// 8324C70C: 3C80820B  lis r4, -0x7df5
	ctx.r[4].s64 = -2113208320;
	// 8324C710: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324C714: 388488AC  addi r4, r4, -0x7754
	ctx.r[4].s64 = ctx.r[4].s64 + -30548;
	// 8324C718: 387F0034  addi r3, r31, 0x34
	ctx.r[3].s64 = ctx.r[31].s64 + 52;
	// 8324C71C: 4AFE07B5  bl 0x8222ced0
	ctx.lr = 0x8324C720;
	sub_8222CED0(ctx, base);
	// 8324C720: 3C60820B  lis r3, -0x7df5
	ctx.r[3].s64 = -2113208320;
	// 8324C724: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324C728: 3883888C  addi r4, r3, -0x7774
	ctx.r[4].s64 = ctx.r[3].s64 + -30580;
	// 8324C72C: 387F0038  addi r3, r31, 0x38
	ctx.r[3].s64 = ctx.r[31].s64 + 56;
	// 8324C730: 4AFE07A1  bl 0x8222ced0
	ctx.lr = 0x8324C734;
	sub_8222CED0(ctx, base);
	// 8324C734: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 8324C738: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324C73C: 388B8868  addi r4, r11, -0x7798
	ctx.r[4].s64 = ctx.r[11].s64 + -30616;
	// 8324C740: 387F003C  addi r3, r31, 0x3c
	ctx.r[3].s64 = ctx.r[31].s64 + 60;
	// 8324C744: 4AFE078D  bl 0x8222ced0
	ctx.lr = 0x8324C748;
	sub_8222CED0(ctx, base);
	// 8324C748: 3D40832B  lis r10, -0x7cd5
	ctx.r[10].s64 = -2094333952;
	// 8324C74C: 386A8718  addi r3, r10, -0x78e8
	ctx.r[3].s64 = ctx.r[10].s64 + -30952;
	// 8324C750: 4BA5D7D1  bl 0x82ca9f20
	ctx.lr = 0x8324C754;
	sub_82CA9F20(ctx, base);
	// 8324C754: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324C758: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324C75C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324C760: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8324C764: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324C768(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324C768 size=376
    let mut pc: u32 = 0x8324C768;
    'dispatch: loop {
        match pc {
            0x8324C768 => {
    //   block [0x8324C768..0x8324C8E0)
	// 8324C768: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324C76C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324C770: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8324C774: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324C778: 3D608349  lis r11, -0x7cb7
	ctx.r[11].s64 = -2092367872;
	// 8324C77C: 3D40820B  lis r10, -0x7df5
	ctx.r[10].s64 = -2113208320;
	// 8324C780: 3BEB7AC0  addi r31, r11, 0x7ac0
	ctx.r[31].s64 = ctx.r[11].s64 + 31424;
	// 8324C784: 388A8C58  addi r4, r10, -0x73a8
	ctx.r[4].s64 = ctx.r[10].s64 + -29608;
	// 8324C788: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8324C78C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324C790: 4AFE0741  bl 0x8222ced0
	ctx.lr = 0x8324C794;
	sub_8222CED0(ctx, base);
	// 8324C794: 3D20820B  lis r9, -0x7df5
	ctx.r[9].s64 = -2113208320;
	// 8324C798: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324C79C: 38898C38  addi r4, r9, -0x73c8
	ctx.r[4].s64 = ctx.r[9].s64 + -29640;
	// 8324C7A0: 387F0004  addi r3, r31, 4
	ctx.r[3].s64 = ctx.r[31].s64 + 4;
	// 8324C7A4: 4AFE072D  bl 0x8222ced0
	ctx.lr = 0x8324C7A8;
	sub_8222CED0(ctx, base);
	// 8324C7A8: 3D00820B  lis r8, -0x7df5
	ctx.r[8].s64 = -2113208320;
	// 8324C7AC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324C7B0: 38888C18  addi r4, r8, -0x73e8
	ctx.r[4].s64 = ctx.r[8].s64 + -29672;
	// 8324C7B4: 387F0008  addi r3, r31, 8
	ctx.r[3].s64 = ctx.r[31].s64 + 8;
	// 8324C7B8: 4AFE0719  bl 0x8222ced0
	ctx.lr = 0x8324C7BC;
	sub_8222CED0(ctx, base);
	// 8324C7BC: 3CE0820B  lis r7, -0x7df5
	ctx.r[7].s64 = -2113208320;
	// 8324C7C0: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324C7C4: 38878BF8  addi r4, r7, -0x7408
	ctx.r[4].s64 = ctx.r[7].s64 + -29704;
	// 8324C7C8: 387F000C  addi r3, r31, 0xc
	ctx.r[3].s64 = ctx.r[31].s64 + 12;
	// 8324C7CC: 4AFE0705  bl 0x8222ced0
	ctx.lr = 0x8324C7D0;
	sub_8222CED0(ctx, base);
	// 8324C7D0: 3CC0820B  lis r6, -0x7df5
	ctx.r[6].s64 = -2113208320;
	// 8324C7D4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324C7D8: 38868BD8  addi r4, r6, -0x7428
	ctx.r[4].s64 = ctx.r[6].s64 + -29736;
	// 8324C7DC: 387F0010  addi r3, r31, 0x10
	ctx.r[3].s64 = ctx.r[31].s64 + 16;
	// 8324C7E0: 4AFE06F1  bl 0x8222ced0
	ctx.lr = 0x8324C7E4;
	sub_8222CED0(ctx, base);
	// 8324C7E4: 3C80820B  lis r4, -0x7df5
	ctx.r[4].s64 = -2113208320;
	// 8324C7E8: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324C7EC: 38848BB8  addi r4, r4, -0x7448
	ctx.r[4].s64 = ctx.r[4].s64 + -29768;
	// 8324C7F0: 387F0014  addi r3, r31, 0x14
	ctx.r[3].s64 = ctx.r[31].s64 + 20;
	// 8324C7F4: 4AFE06DD  bl 0x8222ced0
	ctx.lr = 0x8324C7F8;
	sub_8222CED0(ctx, base);
	// 8324C7F8: 3C60820B  lis r3, -0x7df5
	ctx.r[3].s64 = -2113208320;
	// 8324C7FC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324C800: 38838B98  addi r4, r3, -0x7468
	ctx.r[4].s64 = ctx.r[3].s64 + -29800;
	// 8324C804: 387F0018  addi r3, r31, 0x18
	ctx.r[3].s64 = ctx.r[31].s64 + 24;
	// 8324C808: 4AFE06C9  bl 0x8222ced0
	ctx.lr = 0x8324C80C;
	sub_8222CED0(ctx, base);
	// 8324C80C: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 8324C810: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324C814: 388B8B78  addi r4, r11, -0x7488
	ctx.r[4].s64 = ctx.r[11].s64 + -29832;
	// 8324C818: 387F001C  addi r3, r31, 0x1c
	ctx.r[3].s64 = ctx.r[31].s64 + 28;
	// 8324C81C: 4AFE06B5  bl 0x8222ced0
	ctx.lr = 0x8324C820;
	sub_8222CED0(ctx, base);
	// 8324C820: 3D40820B  lis r10, -0x7df5
	ctx.r[10].s64 = -2113208320;
	// 8324C824: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324C828: 388A8B58  addi r4, r10, -0x74a8
	ctx.r[4].s64 = ctx.r[10].s64 + -29864;
	// 8324C82C: 387F0020  addi r3, r31, 0x20
	ctx.r[3].s64 = ctx.r[31].s64 + 32;
	// 8324C830: 4AFE06A1  bl 0x8222ced0
	ctx.lr = 0x8324C834;
	sub_8222CED0(ctx, base);
	// 8324C834: 3D20820B  lis r9, -0x7df5
	ctx.r[9].s64 = -2113208320;
	// 8324C838: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324C83C: 38898B38  addi r4, r9, -0x74c8
	ctx.r[4].s64 = ctx.r[9].s64 + -29896;
	// 8324C840: 387F0024  addi r3, r31, 0x24
	ctx.r[3].s64 = ctx.r[31].s64 + 36;
	// 8324C844: 4AFE068D  bl 0x8222ced0
	ctx.lr = 0x8324C848;
	sub_8222CED0(ctx, base);
	// 8324C848: 3D00820B  lis r8, -0x7df5
	ctx.r[8].s64 = -2113208320;
	// 8324C84C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324C850: 38888B1C  addi r4, r8, -0x74e4
	ctx.r[4].s64 = ctx.r[8].s64 + -29924;
	// 8324C854: 387F0028  addi r3, r31, 0x28
	ctx.r[3].s64 = ctx.r[31].s64 + 40;
	// 8324C858: 4AFE0679  bl 0x8222ced0
	ctx.lr = 0x8324C85C;
	sub_8222CED0(ctx, base);
	// 8324C85C: 3CE0820B  lis r7, -0x7df5
	ctx.r[7].s64 = -2113208320;
	// 8324C860: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324C864: 38878AF8  addi r4, r7, -0x7508
	ctx.r[4].s64 = ctx.r[7].s64 + -29960;
	// 8324C868: 387F002C  addi r3, r31, 0x2c
	ctx.r[3].s64 = ctx.r[31].s64 + 44;
	// 8324C86C: 4AFE0665  bl 0x8222ced0
	ctx.lr = 0x8324C870;
	sub_8222CED0(ctx, base);
	// 8324C870: 3CC0820B  lis r6, -0x7df5
	ctx.r[6].s64 = -2113208320;
	// 8324C874: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324C878: 38868AD4  addi r4, r6, -0x752c
	ctx.r[4].s64 = ctx.r[6].s64 + -29996;
	// 8324C87C: 387F0030  addi r3, r31, 0x30
	ctx.r[3].s64 = ctx.r[31].s64 + 48;
	// 8324C880: 4AFE0651  bl 0x8222ced0
	ctx.lr = 0x8324C884;
	sub_8222CED0(ctx, base);
	// 8324C884: 3C80820B  lis r4, -0x7df5
	ctx.r[4].s64 = -2113208320;
	// 8324C888: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324C88C: 38848AB4  addi r4, r4, -0x754c
	ctx.r[4].s64 = ctx.r[4].s64 + -30028;
	// 8324C890: 387F0034  addi r3, r31, 0x34
	ctx.r[3].s64 = ctx.r[31].s64 + 52;
	// 8324C894: 4AFE063D  bl 0x8222ced0
	ctx.lr = 0x8324C898;
	sub_8222CED0(ctx, base);
	// 8324C898: 3C60820B  lis r3, -0x7df5
	ctx.r[3].s64 = -2113208320;
	// 8324C89C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324C8A0: 38838A94  addi r4, r3, -0x756c
	ctx.r[4].s64 = ctx.r[3].s64 + -30060;
	// 8324C8A4: 387F0038  addi r3, r31, 0x38
	ctx.r[3].s64 = ctx.r[31].s64 + 56;
	// 8324C8A8: 4AFE0629  bl 0x8222ced0
	ctx.lr = 0x8324C8AC;
	sub_8222CED0(ctx, base);
	// 8324C8AC: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 8324C8B0: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324C8B4: 388B8A70  addi r4, r11, -0x7590
	ctx.r[4].s64 = ctx.r[11].s64 + -30096;
	// 8324C8B8: 387F003C  addi r3, r31, 0x3c
	ctx.r[3].s64 = ctx.r[31].s64 + 60;
	// 8324C8BC: 4AFE0615  bl 0x8222ced0
	ctx.lr = 0x8324C8C0;
	sub_8222CED0(ctx, base);
	// 8324C8C0: 3D40832B  lis r10, -0x7cd5
	ctx.r[10].s64 = -2094333952;
	// 8324C8C4: 386A8780  addi r3, r10, -0x7880
	ctx.r[3].s64 = ctx.r[10].s64 + -30848;
	// 8324C8C8: 4BA5D659  bl 0x82ca9f20
	ctx.lr = 0x8324C8CC;
	sub_82CA9F20(ctx, base);
	// 8324C8CC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324C8D0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324C8D4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324C8D8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8324C8DC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324C8E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324C8E0 size=64
    let mut pc: u32 = 0x8324C8E0;
    'dispatch: loop {
        match pc {
            0x8324C8E0 => {
    //   block [0x8324C8E0..0x8324C920)
	// 8324C8E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324C8E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324C8E8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324C8EC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8324C8F0: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324C8F4: 388B1AD4  addi r4, r11, 0x1ad4
	ctx.r[4].s64 = ctx.r[11].s64 + 6868;
	// 8324C8F8: 386A7B00  addi r3, r10, 0x7b00
	ctx.r[3].s64 = ctx.r[10].s64 + 31488;
	// 8324C8FC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324C900: 4AFE05D1  bl 0x8222ced0
	ctx.lr = 0x8324C904;
	sub_8222CED0(ctx, base);
	// 8324C904: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8324C908: 386987E8  addi r3, r9, -0x7818
	ctx.r[3].s64 = ctx.r[9].s64 + -30744;
	// 8324C90C: 4BA5D615  bl 0x82ca9f20
	ctx.lr = 0x8324C910;
	sub_82CA9F20(ctx, base);
	// 8324C910: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324C914: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324C918: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324C91C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324C920(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324C920 size=64
    let mut pc: u32 = 0x8324C920;
    'dispatch: loop {
        match pc {
            0x8324C920 => {
    //   block [0x8324C920..0x8324C960)
	// 8324C920: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324C924: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324C928: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324C92C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8324C930: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324C934: 388B1ADC  addi r4, r11, 0x1adc
	ctx.r[4].s64 = ctx.r[11].s64 + 6876;
	// 8324C938: 386A7B04  addi r3, r10, 0x7b04
	ctx.r[3].s64 = ctx.r[10].s64 + 31492;
	// 8324C93C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324C940: 4AFE0591  bl 0x8222ced0
	ctx.lr = 0x8324C944;
	sub_8222CED0(ctx, base);
	// 8324C944: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8324C948: 386987F8  addi r3, r9, -0x7808
	ctx.r[3].s64 = ctx.r[9].s64 + -30728;
	// 8324C94C: 4BA5D5D5  bl 0x82ca9f20
	ctx.lr = 0x8324C950;
	sub_82CA9F20(ctx, base);
	// 8324C950: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324C954: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324C958: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324C95C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324C960(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324C960 size=64
    let mut pc: u32 = 0x8324C960;
    'dispatch: loop {
        match pc {
            0x8324C960 => {
    //   block [0x8324C960..0x8324C9A0)
	// 8324C960: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324C964: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324C968: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324C96C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8324C970: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324C974: 388B1AF0  addi r4, r11, 0x1af0
	ctx.r[4].s64 = ctx.r[11].s64 + 6896;
	// 8324C978: 386A7B08  addi r3, r10, 0x7b08
	ctx.r[3].s64 = ctx.r[10].s64 + 31496;
	// 8324C97C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324C980: 4AFE0551  bl 0x8222ced0
	ctx.lr = 0x8324C984;
	sub_8222CED0(ctx, base);
	// 8324C984: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8324C988: 38698808  addi r3, r9, -0x77f8
	ctx.r[3].s64 = ctx.r[9].s64 + -30712;
	// 8324C98C: 4BA5D595  bl 0x82ca9f20
	ctx.lr = 0x8324C990;
	sub_82CA9F20(ctx, base);
	// 8324C990: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324C994: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324C998: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324C99C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324C9A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324C9A0 size=56
    let mut pc: u32 = 0x8324C9A0;
    'dispatch: loop {
        match pc {
            0x8324C9A0 => {
    //   block [0x8324C9A0..0x8324C9D8)
	// 8324C9A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324C9A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324C9A8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324C9AC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8324C9B0: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8324C9B4: 386B42C8  addi r3, r11, 0x42c8
	ctx.r[3].s64 = ctx.r[11].s64 + 17096;
	// 8324C9B8: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8324C9BC: 4AFA739D  bl 0x821f3d58
	ctx.lr = 0x8324C9C0;
	sub_821F3D58(ctx, base);
	// 8324C9C0: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324C9C4: 906A7B0C  stw r3, 0x7b0c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(31500 as u32), ctx.r[3].u32 ) };
	// 8324C9C8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324C9CC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324C9D0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324C9D4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324C9D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324C9D8 size=56
    let mut pc: u32 = 0x8324C9D8;
    'dispatch: loop {
        match pc {
            0x8324C9D8 => {
    //   block [0x8324C9D8..0x8324CA10)
	// 8324C9D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324C9DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324C9E0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324C9E4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8324C9E8: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8324C9EC: 386B42DC  addi r3, r11, 0x42dc
	ctx.r[3].s64 = ctx.r[11].s64 + 17116;
	// 8324C9F0: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8324C9F4: 4AFA7365  bl 0x821f3d58
	ctx.lr = 0x8324C9F8;
	sub_821F3D58(ctx, base);
	// 8324C9F8: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324C9FC: 906A7B10  stw r3, 0x7b10(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(31504 as u32), ctx.r[3].u32 ) };
	// 8324CA00: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324CA04: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324CA08: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324CA0C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324CA10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324CA10 size=56
    let mut pc: u32 = 0x8324CA10;
    'dispatch: loop {
        match pc {
            0x8324CA10 => {
    //   block [0x8324CA10..0x8324CA48)
	// 8324CA10: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324CA14: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324CA18: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324CA1C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8324CA20: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8324CA24: 386B42E8  addi r3, r11, 0x42e8
	ctx.r[3].s64 = ctx.r[11].s64 + 17128;
	// 8324CA28: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8324CA2C: 4AFA732D  bl 0x821f3d58
	ctx.lr = 0x8324CA30;
	sub_821F3D58(ctx, base);
	// 8324CA30: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324CA34: 906A7B14  stw r3, 0x7b14(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(31508 as u32), ctx.r[3].u32 ) };
	// 8324CA38: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324CA3C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324CA40: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324CA44: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324CA48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324CA48 size=56
    let mut pc: u32 = 0x8324CA48;
    'dispatch: loop {
        match pc {
            0x8324CA48 => {
    //   block [0x8324CA48..0x8324CA80)
	// 8324CA48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324CA4C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324CA50: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324CA54: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8324CA58: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8324CA5C: 386B4300  addi r3, r11, 0x4300
	ctx.r[3].s64 = ctx.r[11].s64 + 17152;
	// 8324CA60: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8324CA64: 4AFA72F5  bl 0x821f3d58
	ctx.lr = 0x8324CA68;
	sub_821F3D58(ctx, base);
	// 8324CA68: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324CA6C: 906A7B18  stw r3, 0x7b18(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(31512 as u32), ctx.r[3].u32 ) };
	// 8324CA70: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324CA74: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324CA78: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324CA7C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324CA80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324CA80 size=56
    let mut pc: u32 = 0x8324CA80;
    'dispatch: loop {
        match pc {
            0x8324CA80 => {
    //   block [0x8324CA80..0x8324CAB8)
	// 8324CA80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324CA84: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324CA88: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324CA8C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8324CA90: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8324CA94: 386B4310  addi r3, r11, 0x4310
	ctx.r[3].s64 = ctx.r[11].s64 + 17168;
	// 8324CA98: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8324CA9C: 4AFA72BD  bl 0x821f3d58
	ctx.lr = 0x8324CAA0;
	sub_821F3D58(ctx, base);
	// 8324CAA0: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324CAA4: 906A7B1C  stw r3, 0x7b1c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(31516 as u32), ctx.r[3].u32 ) };
	// 8324CAA8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324CAAC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324CAB0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324CAB4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324CAB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324CAB8 size=56
    let mut pc: u32 = 0x8324CAB8;
    'dispatch: loop {
        match pc {
            0x8324CAB8 => {
    //   block [0x8324CAB8..0x8324CAF0)
	// 8324CAB8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324CABC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324CAC0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324CAC4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8324CAC8: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8324CACC: 386B4328  addi r3, r11, 0x4328
	ctx.r[3].s64 = ctx.r[11].s64 + 17192;
	// 8324CAD0: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8324CAD4: 4AFA7285  bl 0x821f3d58
	ctx.lr = 0x8324CAD8;
	sub_821F3D58(ctx, base);
	// 8324CAD8: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324CADC: 906A7B20  stw r3, 0x7b20(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(31520 as u32), ctx.r[3].u32 ) };
	// 8324CAE0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324CAE4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324CAE8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324CAEC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324CAF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324CAF0 size=56
    let mut pc: u32 = 0x8324CAF0;
    'dispatch: loop {
        match pc {
            0x8324CAF0 => {
    //   block [0x8324CAF0..0x8324CB28)
	// 8324CAF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324CAF4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324CAF8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324CAFC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8324CB00: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8324CB04: 386B4338  addi r3, r11, 0x4338
	ctx.r[3].s64 = ctx.r[11].s64 + 17208;
	// 8324CB08: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8324CB0C: 4AFA724D  bl 0x821f3d58
	ctx.lr = 0x8324CB10;
	sub_821F3D58(ctx, base);
	// 8324CB10: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324CB14: 906A7B24  stw r3, 0x7b24(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(31524 as u32), ctx.r[3].u32 ) };
	// 8324CB18: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324CB1C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324CB20: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324CB24: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324CB28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324CB28 size=56
    let mut pc: u32 = 0x8324CB28;
    'dispatch: loop {
        match pc {
            0x8324CB28 => {
    //   block [0x8324CB28..0x8324CB60)
	// 8324CB28: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324CB2C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324CB30: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324CB34: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8324CB38: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8324CB3C: 386B4350  addi r3, r11, 0x4350
	ctx.r[3].s64 = ctx.r[11].s64 + 17232;
	// 8324CB40: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8324CB44: 4AFA7215  bl 0x821f3d58
	ctx.lr = 0x8324CB48;
	sub_821F3D58(ctx, base);
	// 8324CB48: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324CB4C: 906A7B28  stw r3, 0x7b28(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(31528 as u32), ctx.r[3].u32 ) };
	// 8324CB50: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324CB54: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324CB58: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324CB5C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324CB60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324CB60 size=56
    let mut pc: u32 = 0x8324CB60;
    'dispatch: loop {
        match pc {
            0x8324CB60 => {
    //   block [0x8324CB60..0x8324CB98)
	// 8324CB60: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324CB64: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324CB68: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324CB6C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8324CB70: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8324CB74: 386B4368  addi r3, r11, 0x4368
	ctx.r[3].s64 = ctx.r[11].s64 + 17256;
	// 8324CB78: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8324CB7C: 4AFA71DD  bl 0x821f3d58
	ctx.lr = 0x8324CB80;
	sub_821F3D58(ctx, base);
	// 8324CB80: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324CB84: 906A7B2C  stw r3, 0x7b2c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(31532 as u32), ctx.r[3].u32 ) };
	// 8324CB88: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324CB8C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324CB90: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324CB94: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324CB98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324CB98 size=56
    let mut pc: u32 = 0x8324CB98;
    'dispatch: loop {
        match pc {
            0x8324CB98 => {
    //   block [0x8324CB98..0x8324CBD0)
	// 8324CB98: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324CB9C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324CBA0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324CBA4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8324CBA8: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8324CBAC: 386B4380  addi r3, r11, 0x4380
	ctx.r[3].s64 = ctx.r[11].s64 + 17280;
	// 8324CBB0: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8324CBB4: 4AFA71A5  bl 0x821f3d58
	ctx.lr = 0x8324CBB8;
	sub_821F3D58(ctx, base);
	// 8324CBB8: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324CBBC: 906A7B30  stw r3, 0x7b30(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(31536 as u32), ctx.r[3].u32 ) };
	// 8324CBC0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324CBC4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324CBC8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324CBCC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324CBD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324CBD0 size=56
    let mut pc: u32 = 0x8324CBD0;
    'dispatch: loop {
        match pc {
            0x8324CBD0 => {
    //   block [0x8324CBD0..0x8324CC08)
	// 8324CBD0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324CBD4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324CBD8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324CBDC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8324CBE0: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8324CBE4: 386B4394  addi r3, r11, 0x4394
	ctx.r[3].s64 = ctx.r[11].s64 + 17300;
	// 8324CBE8: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8324CBEC: 4AFA716D  bl 0x821f3d58
	ctx.lr = 0x8324CBF0;
	sub_821F3D58(ctx, base);
	// 8324CBF0: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324CBF4: 906A7B34  stw r3, 0x7b34(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(31540 as u32), ctx.r[3].u32 ) };
	// 8324CBF8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324CBFC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324CC00: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324CC04: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324CC08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324CC08 size=56
    let mut pc: u32 = 0x8324CC08;
    'dispatch: loop {
        match pc {
            0x8324CC08 => {
    //   block [0x8324CC08..0x8324CC40)
	// 8324CC08: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324CC0C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324CC10: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324CC14: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8324CC18: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8324CC1C: 386B43B4  addi r3, r11, 0x43b4
	ctx.r[3].s64 = ctx.r[11].s64 + 17332;
	// 8324CC20: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8324CC24: 4AFA7135  bl 0x821f3d58
	ctx.lr = 0x8324CC28;
	sub_821F3D58(ctx, base);
	// 8324CC28: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324CC2C: 906A7B38  stw r3, 0x7b38(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(31544 as u32), ctx.r[3].u32 ) };
	// 8324CC30: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324CC34: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324CC38: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324CC3C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324CC40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324CC40 size=56
    let mut pc: u32 = 0x8324CC40;
    'dispatch: loop {
        match pc {
            0x8324CC40 => {
    //   block [0x8324CC40..0x8324CC78)
	// 8324CC40: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324CC44: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324CC48: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324CC4C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8324CC50: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8324CC54: 386B43C8  addi r3, r11, 0x43c8
	ctx.r[3].s64 = ctx.r[11].s64 + 17352;
	// 8324CC58: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8324CC5C: 4AFA70FD  bl 0x821f3d58
	ctx.lr = 0x8324CC60;
	sub_821F3D58(ctx, base);
	// 8324CC60: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324CC64: 906A7B3C  stw r3, 0x7b3c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(31548 as u32), ctx.r[3].u32 ) };
	// 8324CC68: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324CC6C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324CC70: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324CC74: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324CC78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324CC78 size=56
    let mut pc: u32 = 0x8324CC78;
    'dispatch: loop {
        match pc {
            0x8324CC78 => {
    //   block [0x8324CC78..0x8324CCB0)
	// 8324CC78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324CC7C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324CC80: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324CC84: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8324CC88: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8324CC8C: 386B43D4  addi r3, r11, 0x43d4
	ctx.r[3].s64 = ctx.r[11].s64 + 17364;
	// 8324CC90: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8324CC94: 4AFA70C5  bl 0x821f3d58
	ctx.lr = 0x8324CC98;
	sub_821F3D58(ctx, base);
	// 8324CC98: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324CC9C: 906A7B40  stw r3, 0x7b40(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(31552 as u32), ctx.r[3].u32 ) };
	// 8324CCA0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324CCA4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324CCA8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324CCAC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324CCB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324CCB0 size=56
    let mut pc: u32 = 0x8324CCB0;
    'dispatch: loop {
        match pc {
            0x8324CCB0 => {
    //   block [0x8324CCB0..0x8324CCE8)
	// 8324CCB0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324CCB4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324CCB8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324CCBC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8324CCC0: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8324CCC4: 386B43E0  addi r3, r11, 0x43e0
	ctx.r[3].s64 = ctx.r[11].s64 + 17376;
	// 8324CCC8: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8324CCCC: 4AFA708D  bl 0x821f3d58
	ctx.lr = 0x8324CCD0;
	sub_821F3D58(ctx, base);
	// 8324CCD0: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324CCD4: 906A7B44  stw r3, 0x7b44(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(31556 as u32), ctx.r[3].u32 ) };
	// 8324CCD8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324CCDC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324CCE0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324CCE4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324CCE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324CCE8 size=56
    let mut pc: u32 = 0x8324CCE8;
    'dispatch: loop {
        match pc {
            0x8324CCE8 => {
    //   block [0x8324CCE8..0x8324CD20)
	// 8324CCE8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324CCEC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324CCF0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324CCF4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8324CCF8: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8324CCFC: 386B43F0  addi r3, r11, 0x43f0
	ctx.r[3].s64 = ctx.r[11].s64 + 17392;
	// 8324CD00: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8324CD04: 4AFA7055  bl 0x821f3d58
	ctx.lr = 0x8324CD08;
	sub_821F3D58(ctx, base);
	// 8324CD08: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324CD0C: 906A7B48  stw r3, 0x7b48(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(31560 as u32), ctx.r[3].u32 ) };
	// 8324CD10: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324CD14: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324CD18: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324CD1C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324CD20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324CD20 size=56
    let mut pc: u32 = 0x8324CD20;
    'dispatch: loop {
        match pc {
            0x8324CD20 => {
    //   block [0x8324CD20..0x8324CD58)
	// 8324CD20: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324CD24: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324CD28: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324CD2C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8324CD30: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8324CD34: 386B4400  addi r3, r11, 0x4400
	ctx.r[3].s64 = ctx.r[11].s64 + 17408;
	// 8324CD38: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8324CD3C: 4AFA701D  bl 0x821f3d58
	ctx.lr = 0x8324CD40;
	sub_821F3D58(ctx, base);
	// 8324CD40: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324CD44: 906A7B4C  stw r3, 0x7b4c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(31564 as u32), ctx.r[3].u32 ) };
	// 8324CD48: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324CD4C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324CD50: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324CD54: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324CD58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324CD58 size=56
    let mut pc: u32 = 0x8324CD58;
    'dispatch: loop {
        match pc {
            0x8324CD58 => {
    //   block [0x8324CD58..0x8324CD90)
	// 8324CD58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324CD5C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324CD60: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324CD64: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8324CD68: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8324CD6C: 386B4418  addi r3, r11, 0x4418
	ctx.r[3].s64 = ctx.r[11].s64 + 17432;
	// 8324CD70: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8324CD74: 4AFA6FE5  bl 0x821f3d58
	ctx.lr = 0x8324CD78;
	sub_821F3D58(ctx, base);
	// 8324CD78: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324CD7C: 906A7B50  stw r3, 0x7b50(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(31568 as u32), ctx.r[3].u32 ) };
	// 8324CD80: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324CD84: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324CD88: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324CD8C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324CD90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324CD90 size=56
    let mut pc: u32 = 0x8324CD90;
    'dispatch: loop {
        match pc {
            0x8324CD90 => {
    //   block [0x8324CD90..0x8324CDC8)
	// 8324CD90: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324CD94: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324CD98: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324CD9C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8324CDA0: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8324CDA4: 386B4424  addi r3, r11, 0x4424
	ctx.r[3].s64 = ctx.r[11].s64 + 17444;
	// 8324CDA8: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8324CDAC: 4AFA6FAD  bl 0x821f3d58
	ctx.lr = 0x8324CDB0;
	sub_821F3D58(ctx, base);
	// 8324CDB0: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324CDB4: 906A7B54  stw r3, 0x7b54(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(31572 as u32), ctx.r[3].u32 ) };
	// 8324CDB8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324CDBC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324CDC0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324CDC4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324CDC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324CDC8 size=56
    let mut pc: u32 = 0x8324CDC8;
    'dispatch: loop {
        match pc {
            0x8324CDC8 => {
    //   block [0x8324CDC8..0x8324CE00)
	// 8324CDC8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324CDCC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324CDD0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324CDD4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8324CDD8: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8324CDDC: 386B4430  addi r3, r11, 0x4430
	ctx.r[3].s64 = ctx.r[11].s64 + 17456;
	// 8324CDE0: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8324CDE4: 4AFA6F75  bl 0x821f3d58
	ctx.lr = 0x8324CDE8;
	sub_821F3D58(ctx, base);
	// 8324CDE8: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324CDEC: 906A7B58  stw r3, 0x7b58(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(31576 as u32), ctx.r[3].u32 ) };
	// 8324CDF0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324CDF4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324CDF8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324CDFC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324CE00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324CE00 size=56
    let mut pc: u32 = 0x8324CE00;
    'dispatch: loop {
        match pc {
            0x8324CE00 => {
    //   block [0x8324CE00..0x8324CE38)
	// 8324CE00: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324CE04: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324CE08: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324CE0C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8324CE10: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8324CE14: 386B4440  addi r3, r11, 0x4440
	ctx.r[3].s64 = ctx.r[11].s64 + 17472;
	// 8324CE18: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8324CE1C: 4AFA6F3D  bl 0x821f3d58
	ctx.lr = 0x8324CE20;
	sub_821F3D58(ctx, base);
	// 8324CE20: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324CE24: 906A7B5C  stw r3, 0x7b5c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(31580 as u32), ctx.r[3].u32 ) };
	// 8324CE28: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324CE2C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324CE30: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324CE34: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324CE38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324CE38 size=64
    let mut pc: u32 = 0x8324CE38;
    'dispatch: loop {
        match pc {
            0x8324CE38 => {
    //   block [0x8324CE38..0x8324CE78)
	// 8324CE38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324CE3C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324CE40: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324CE44: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8324CE48: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324CE4C: 388B1AD4  addi r4, r11, 0x1ad4
	ctx.r[4].s64 = ctx.r[11].s64 + 6868;
	// 8324CE50: 386A7B60  addi r3, r10, 0x7b60
	ctx.r[3].s64 = ctx.r[10].s64 + 31584;
	// 8324CE54: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324CE58: 4AFE0079  bl 0x8222ced0
	ctx.lr = 0x8324CE5C;
	sub_8222CED0(ctx, base);
	// 8324CE5C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8324CE60: 38698818  addi r3, r9, -0x77e8
	ctx.r[3].s64 = ctx.r[9].s64 + -30696;
	// 8324CE64: 4BA5D0BD  bl 0x82ca9f20
	ctx.lr = 0x8324CE68;
	sub_82CA9F20(ctx, base);
	// 8324CE68: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324CE6C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324CE70: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324CE74: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324CE78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324CE78 size=64
    let mut pc: u32 = 0x8324CE78;
    'dispatch: loop {
        match pc {
            0x8324CE78 => {
    //   block [0x8324CE78..0x8324CEB8)
	// 8324CE78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324CE7C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324CE80: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324CE84: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8324CE88: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324CE8C: 388B1ADC  addi r4, r11, 0x1adc
	ctx.r[4].s64 = ctx.r[11].s64 + 6876;
	// 8324CE90: 386A7B64  addi r3, r10, 0x7b64
	ctx.r[3].s64 = ctx.r[10].s64 + 31588;
	// 8324CE94: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324CE98: 4AFE0039  bl 0x8222ced0
	ctx.lr = 0x8324CE9C;
	sub_8222CED0(ctx, base);
	// 8324CE9C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8324CEA0: 38698828  addi r3, r9, -0x77d8
	ctx.r[3].s64 = ctx.r[9].s64 + -30680;
	// 8324CEA4: 4BA5D07D  bl 0x82ca9f20
	ctx.lr = 0x8324CEA8;
	sub_82CA9F20(ctx, base);
	// 8324CEA8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324CEAC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324CEB0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324CEB4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324CEB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324CEB8 size=64
    let mut pc: u32 = 0x8324CEB8;
    'dispatch: loop {
        match pc {
            0x8324CEB8 => {
    //   block [0x8324CEB8..0x8324CEF8)
	// 8324CEB8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324CEBC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324CEC0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324CEC4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8324CEC8: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324CECC: 388B1AF0  addi r4, r11, 0x1af0
	ctx.r[4].s64 = ctx.r[11].s64 + 6896;
	// 8324CED0: 386A7B68  addi r3, r10, 0x7b68
	ctx.r[3].s64 = ctx.r[10].s64 + 31592;
	// 8324CED4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324CED8: 4AFDFFF9  bl 0x8222ced0
	ctx.lr = 0x8324CEDC;
	sub_8222CED0(ctx, base);
	// 8324CEDC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8324CEE0: 38698838  addi r3, r9, -0x77c8
	ctx.r[3].s64 = ctx.r[9].s64 + -30664;
	// 8324CEE4: 4BA5D03D  bl 0x82ca9f20
	ctx.lr = 0x8324CEE8;
	sub_82CA9F20(ctx, base);
	// 8324CEE8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324CEEC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324CEF0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324CEF4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324CEF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8324CEF8 size=32
    let mut pc: u32 = 0x8324CEF8;
    'dispatch: loop {
        match pc {
            0x8324CEF8 => {
    //   block [0x8324CEF8..0x8324CF18)
	// 8324CEF8: 3D608332  lis r11, -0x7cce
	ctx.r[11].s64 = -2093875200;
	// 8324CEFC: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324CF00: C80B9660  lfd f0, -0x69a0(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[0].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(-27040 as u32) ) };
	// 8324CF04: FC00001E  fctiwz f0, f0
	ctx.f[0].s64 = if ctx.f[0].f64 > (i32::MAX as f64) { i32::MAX as i64 } else { ctx.f[0].f64.trunc() as i32 as i64 };
	// 8324CF08: D801FFF0  stfd f0, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.f[0].u64 ) };
	// 8324CF0C: 8161FFF4  lwz r11, -0xc(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-12 as u32) ) } as u64;
	// 8324CF10: 916A7B6C  stw r11, 0x7b6c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(31596 as u32), ctx.r[11].u32 ) };
	// 8324CF14: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324CF18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324CF18 size=56
    let mut pc: u32 = 0x8324CF18;
    'dispatch: loop {
        match pc {
            0x8324CF18 => {
    //   block [0x8324CF18..0x8324CF50)
	// 8324CF18: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324CF1C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324CF20: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324CF24: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 8324CF28: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8324CF2C: 386BE0F0  addi r3, r11, -0x1f10
	ctx.r[3].s64 = ctx.r[11].s64 + -7952;
	// 8324CF30: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8324CF34: 4AFA6E25  bl 0x821f3d58
	ctx.lr = 0x8324CF38;
	sub_821F3D58(ctx, base);
	// 8324CF38: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324CF3C: 906A7B70  stw r3, 0x7b70(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(31600 as u32), ctx.r[3].u32 ) };
	// 8324CF40: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324CF44: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324CF48: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324CF4C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324CF50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324CF50 size=52
    let mut pc: u32 = 0x8324CF50;
    'dispatch: loop {
        match pc {
            0x8324CF50 => {
    //   block [0x8324CF50..0x8324CF84)
	// 8324CF50: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324CF54: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324CF58: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324CF5C: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 8324CF60: 386BE104  addi r3, r11, -0x1efc
	ctx.r[3].s64 = ctx.r[11].s64 + -7932;
	// 8324CF64: 4AF3C1DD  bl 0x82189140
	ctx.lr = 0x8324CF68;
	sub_82189140(ctx, base);
	// 8324CF68: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324CF6C: 546B007E  clrlwi r11, r3, 1
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x7FFFFFFFu64;
	// 8324CF70: 916A7B74  stw r11, 0x7b74(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(31604 as u32), ctx.r[11].u32 ) };
	// 8324CF74: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324CF78: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324CF7C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324CF80: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324CF88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324CF88 size=52
    let mut pc: u32 = 0x8324CF88;
    'dispatch: loop {
        match pc {
            0x8324CF88 => {
    //   block [0x8324CF88..0x8324CFBC)
	// 8324CF88: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324CF8C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324CF90: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324CF94: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 8324CF98: 386BE118  addi r3, r11, -0x1ee8
	ctx.r[3].s64 = ctx.r[11].s64 + -7912;
	// 8324CF9C: 4AF3C1A5  bl 0x82189140
	ctx.lr = 0x8324CFA0;
	sub_82189140(ctx, base);
	// 8324CFA0: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324CFA4: 546B007E  clrlwi r11, r3, 1
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x7FFFFFFFu64;
	// 8324CFA8: 916A7B78  stw r11, 0x7b78(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(31608 as u32), ctx.r[11].u32 ) };
	// 8324CFAC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324CFB0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324CFB4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324CFB8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324CFC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324CFC0 size=52
    let mut pc: u32 = 0x8324CFC0;
    'dispatch: loop {
        match pc {
            0x8324CFC0 => {
    //   block [0x8324CFC0..0x8324CFF4)
	// 8324CFC0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324CFC4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324CFC8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324CFCC: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 8324CFD0: 386BE124  addi r3, r11, -0x1edc
	ctx.r[3].s64 = ctx.r[11].s64 + -7900;
	// 8324CFD4: 4AF3C16D  bl 0x82189140
	ctx.lr = 0x8324CFD8;
	sub_82189140(ctx, base);
	// 8324CFD8: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324CFDC: 546B007E  clrlwi r11, r3, 1
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x7FFFFFFFu64;
	// 8324CFE0: 916A7B7C  stw r11, 0x7b7c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(31612 as u32), ctx.r[11].u32 ) };
	// 8324CFE4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324CFE8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324CFEC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324CFF0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324CFF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324CFF8 size=52
    let mut pc: u32 = 0x8324CFF8;
    'dispatch: loop {
        match pc {
            0x8324CFF8 => {
    //   block [0x8324CFF8..0x8324D02C)
	// 8324CFF8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324CFFC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324D000: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324D004: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 8324D008: 386BE134  addi r3, r11, -0x1ecc
	ctx.r[3].s64 = ctx.r[11].s64 + -7884;
	// 8324D00C: 4AF3C135  bl 0x82189140
	ctx.lr = 0x8324D010;
	sub_82189140(ctx, base);
	// 8324D010: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324D014: 546B007E  clrlwi r11, r3, 1
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x7FFFFFFFu64;
	// 8324D018: 916A7B80  stw r11, 0x7b80(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(31616 as u32), ctx.r[11].u32 ) };
	// 8324D01C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324D020: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324D024: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324D028: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324D030(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324D030 size=64
    let mut pc: u32 = 0x8324D030;
    'dispatch: loop {
        match pc {
            0x8324D030 => {
    //   block [0x8324D030..0x8324D070)
	// 8324D030: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324D034: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324D038: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324D03C: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 8324D040: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324D044: 388BE1A0  addi r4, r11, -0x1e60
	ctx.r[4].s64 = ctx.r[11].s64 + -7776;
	// 8324D048: 386A7B84  addi r3, r10, 0x7b84
	ctx.r[3].s64 = ctx.r[10].s64 + 31620;
	// 8324D04C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324D050: 4AFDFE81  bl 0x8222ced0
	ctx.lr = 0x8324D054;
	sub_8222CED0(ctx, base);
	// 8324D054: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8324D058: 38698848  addi r3, r9, -0x77b8
	ctx.r[3].s64 = ctx.r[9].s64 + -30648;
	// 8324D05C: 4BA5CEC5  bl 0x82ca9f20
	ctx.lr = 0x8324D060;
	sub_82CA9F20(ctx, base);
	// 8324D060: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324D064: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324D068: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324D06C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324D070(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324D070 size=64
    let mut pc: u32 = 0x8324D070;
    'dispatch: loop {
        match pc {
            0x8324D070 => {
    //   block [0x8324D070..0x8324D0B0)
	// 8324D070: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324D074: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324D078: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324D07C: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 8324D080: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324D084: 388BE1AC  addi r4, r11, -0x1e54
	ctx.r[4].s64 = ctx.r[11].s64 + -7764;
	// 8324D088: 386A7B88  addi r3, r10, 0x7b88
	ctx.r[3].s64 = ctx.r[10].s64 + 31624;
	// 8324D08C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324D090: 4AFDFE41  bl 0x8222ced0
	ctx.lr = 0x8324D094;
	sub_8222CED0(ctx, base);
	// 8324D094: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8324D098: 38698858  addi r3, r9, -0x77a8
	ctx.r[3].s64 = ctx.r[9].s64 + -30632;
	// 8324D09C: 4BA5CE85  bl 0x82ca9f20
	ctx.lr = 0x8324D0A0;
	sub_82CA9F20(ctx, base);
	// 8324D0A0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324D0A4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324D0A8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324D0AC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324D0B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324D0B0 size=64
    let mut pc: u32 = 0x8324D0B0;
    'dispatch: loop {
        match pc {
            0x8324D0B0 => {
    //   block [0x8324D0B0..0x8324D0F0)
	// 8324D0B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324D0B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324D0B8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324D0BC: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 8324D0C0: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324D0C4: 388BE1C4  addi r4, r11, -0x1e3c
	ctx.r[4].s64 = ctx.r[11].s64 + -7740;
	// 8324D0C8: 386A7B8C  addi r3, r10, 0x7b8c
	ctx.r[3].s64 = ctx.r[10].s64 + 31628;
	// 8324D0CC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324D0D0: 4AFDFE01  bl 0x8222ced0
	ctx.lr = 0x8324D0D4;
	sub_8222CED0(ctx, base);
	// 8324D0D4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8324D0D8: 38698868  addi r3, r9, -0x7798
	ctx.r[3].s64 = ctx.r[9].s64 + -30616;
	// 8324D0DC: 4BA5CE45  bl 0x82ca9f20
	ctx.lr = 0x8324D0E0;
	sub_82CA9F20(ctx, base);
	// 8324D0E0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324D0E4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324D0E8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324D0EC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324D0F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324D0F0 size=64
    let mut pc: u32 = 0x8324D0F0;
    'dispatch: loop {
        match pc {
            0x8324D0F0 => {
    //   block [0x8324D0F0..0x8324D130)
	// 8324D0F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324D0F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324D0F8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324D0FC: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 8324D100: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324D104: 388BE1DC  addi r4, r11, -0x1e24
	ctx.r[4].s64 = ctx.r[11].s64 + -7716;
	// 8324D108: 386A7B90  addi r3, r10, 0x7b90
	ctx.r[3].s64 = ctx.r[10].s64 + 31632;
	// 8324D10C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324D110: 4AFDFDC1  bl 0x8222ced0
	ctx.lr = 0x8324D114;
	sub_8222CED0(ctx, base);
	// 8324D114: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8324D118: 38698878  addi r3, r9, -0x7788
	ctx.r[3].s64 = ctx.r[9].s64 + -30600;
	// 8324D11C: 4BA5CE05  bl 0x82ca9f20
	ctx.lr = 0x8324D120;
	sub_82CA9F20(ctx, base);
	// 8324D120: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324D124: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324D128: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324D12C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324D130(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324D130 size=64
    let mut pc: u32 = 0x8324D130;
    'dispatch: loop {
        match pc {
            0x8324D130 => {
    //   block [0x8324D130..0x8324D170)
	// 8324D130: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324D134: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324D138: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324D13C: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 8324D140: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324D144: 388BE1EC  addi r4, r11, -0x1e14
	ctx.r[4].s64 = ctx.r[11].s64 + -7700;
	// 8324D148: 386A7B94  addi r3, r10, 0x7b94
	ctx.r[3].s64 = ctx.r[10].s64 + 31636;
	// 8324D14C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324D150: 4AFDFD81  bl 0x8222ced0
	ctx.lr = 0x8324D154;
	sub_8222CED0(ctx, base);
	// 8324D154: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8324D158: 38698888  addi r3, r9, -0x7778
	ctx.r[3].s64 = ctx.r[9].s64 + -30584;
	// 8324D15C: 4BA5CDC5  bl 0x82ca9f20
	ctx.lr = 0x8324D160;
	sub_82CA9F20(ctx, base);
	// 8324D160: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324D164: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324D168: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324D16C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324D170(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324D170 size=64
    let mut pc: u32 = 0x8324D170;
    'dispatch: loop {
        match pc {
            0x8324D170 => {
    //   block [0x8324D170..0x8324D1B0)
	// 8324D170: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324D174: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324D178: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324D17C: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 8324D180: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324D184: 388BE208  addi r4, r11, -0x1df8
	ctx.r[4].s64 = ctx.r[11].s64 + -7672;
	// 8324D188: 386A7B98  addi r3, r10, 0x7b98
	ctx.r[3].s64 = ctx.r[10].s64 + 31640;
	// 8324D18C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324D190: 4AFDFD41  bl 0x8222ced0
	ctx.lr = 0x8324D194;
	sub_8222CED0(ctx, base);
	// 8324D194: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8324D198: 38698898  addi r3, r9, -0x7768
	ctx.r[3].s64 = ctx.r[9].s64 + -30568;
	// 8324D19C: 4BA5CD85  bl 0x82ca9f20
	ctx.lr = 0x8324D1A0;
	sub_82CA9F20(ctx, base);
	// 8324D1A0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324D1A4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324D1A8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324D1AC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324D1B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324D1B0 size=64
    let mut pc: u32 = 0x8324D1B0;
    'dispatch: loop {
        match pc {
            0x8324D1B0 => {
    //   block [0x8324D1B0..0x8324D1F0)
	// 8324D1B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324D1B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324D1B8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324D1BC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8324D1C0: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324D1C4: 388B1AD4  addi r4, r11, 0x1ad4
	ctx.r[4].s64 = ctx.r[11].s64 + 6868;
	// 8324D1C8: 386A7B9C  addi r3, r10, 0x7b9c
	ctx.r[3].s64 = ctx.r[10].s64 + 31644;
	// 8324D1CC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324D1D0: 4AFDFD01  bl 0x8222ced0
	ctx.lr = 0x8324D1D4;
	sub_8222CED0(ctx, base);
	// 8324D1D4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8324D1D8: 386988A8  addi r3, r9, -0x7758
	ctx.r[3].s64 = ctx.r[9].s64 + -30552;
	// 8324D1DC: 4BA5CD45  bl 0x82ca9f20
	ctx.lr = 0x8324D1E0;
	sub_82CA9F20(ctx, base);
	// 8324D1E0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324D1E4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324D1E8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324D1EC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324D1F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324D1F0 size=64
    let mut pc: u32 = 0x8324D1F0;
    'dispatch: loop {
        match pc {
            0x8324D1F0 => {
    //   block [0x8324D1F0..0x8324D230)
	// 8324D1F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324D1F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324D1F8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324D1FC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8324D200: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324D204: 388B1ADC  addi r4, r11, 0x1adc
	ctx.r[4].s64 = ctx.r[11].s64 + 6876;
	// 8324D208: 386A7BA0  addi r3, r10, 0x7ba0
	ctx.r[3].s64 = ctx.r[10].s64 + 31648;
	// 8324D20C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324D210: 4AFDFCC1  bl 0x8222ced0
	ctx.lr = 0x8324D214;
	sub_8222CED0(ctx, base);
	// 8324D214: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8324D218: 386988B8  addi r3, r9, -0x7748
	ctx.r[3].s64 = ctx.r[9].s64 + -30536;
	// 8324D21C: 4BA5CD05  bl 0x82ca9f20
	ctx.lr = 0x8324D220;
	sub_82CA9F20(ctx, base);
	// 8324D220: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324D224: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324D228: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324D22C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324D230(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324D230 size=64
    let mut pc: u32 = 0x8324D230;
    'dispatch: loop {
        match pc {
            0x8324D230 => {
    //   block [0x8324D230..0x8324D270)
	// 8324D230: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324D234: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324D238: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324D23C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8324D240: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324D244: 388B1AF0  addi r4, r11, 0x1af0
	ctx.r[4].s64 = ctx.r[11].s64 + 6896;
	// 8324D248: 386A7BA4  addi r3, r10, 0x7ba4
	ctx.r[3].s64 = ctx.r[10].s64 + 31652;
	// 8324D24C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324D250: 4AFDFC81  bl 0x8222ced0
	ctx.lr = 0x8324D254;
	sub_8222CED0(ctx, base);
	// 8324D254: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8324D258: 386988C8  addi r3, r9, -0x7738
	ctx.r[3].s64 = ctx.r[9].s64 + -30520;
	// 8324D25C: 4BA5CCC5  bl 0x82ca9f20
	ctx.lr = 0x8324D260;
	sub_82CA9F20(ctx, base);
	// 8324D260: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324D264: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324D268: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324D26C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324D270(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324D270 size=64
    let mut pc: u32 = 0x8324D270;
    'dispatch: loop {
        match pc {
            0x8324D270 => {
    //   block [0x8324D270..0x8324D2B0)
	// 8324D270: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324D274: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324D278: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324D27C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8324D280: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324D284: 388B052C  addi r4, r11, 0x52c
	ctx.r[4].s64 = ctx.r[11].s64 + 1324;
	// 8324D288: 386A7BA8  addi r3, r10, 0x7ba8
	ctx.r[3].s64 = ctx.r[10].s64 + 31656;
	// 8324D28C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324D290: 4AFDFC41  bl 0x8222ced0
	ctx.lr = 0x8324D294;
	sub_8222CED0(ctx, base);
	// 8324D294: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8324D298: 386988D8  addi r3, r9, -0x7728
	ctx.r[3].s64 = ctx.r[9].s64 + -30504;
	// 8324D29C: 4BA5CC85  bl 0x82ca9f20
	ctx.lr = 0x8324D2A0;
	sub_82CA9F20(ctx, base);
	// 8324D2A0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324D2A4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324D2A8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324D2AC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324D2B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324D2B0 size=56
    let mut pc: u32 = 0x8324D2B0;
    'dispatch: loop {
        match pc {
            0x8324D2B0 => {
    //   block [0x8324D2B0..0x8324D2E8)
	// 8324D2B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324D2B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324D2B8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324D2BC: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 8324D2C0: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8324D2C4: 386BE224  addi r3, r11, -0x1ddc
	ctx.r[3].s64 = ctx.r[11].s64 + -7644;
	// 8324D2C8: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8324D2CC: 4AFA6A8D  bl 0x821f3d58
	ctx.lr = 0x8324D2D0;
	sub_821F3D58(ctx, base);
	// 8324D2D0: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324D2D4: 906A7BAC  stw r3, 0x7bac(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(31660 as u32), ctx.r[3].u32 ) };
	// 8324D2D8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324D2DC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324D2E0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324D2E4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324D2E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324D2E8 size=64
    let mut pc: u32 = 0x8324D2E8;
    'dispatch: loop {
        match pc {
            0x8324D2E8 => {
    //   block [0x8324D2E8..0x8324D328)
	// 8324D2E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324D2EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324D2F0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324D2F4: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 8324D2F8: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324D2FC: 388BE22C  addi r4, r11, -0x1dd4
	ctx.r[4].s64 = ctx.r[11].s64 + -7636;
	// 8324D300: 386A7BB0  addi r3, r10, 0x7bb0
	ctx.r[3].s64 = ctx.r[10].s64 + 31664;
	// 8324D304: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324D308: 4AFDFBC9  bl 0x8222ced0
	ctx.lr = 0x8324D30C;
	sub_8222CED0(ctx, base);
	// 8324D30C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8324D310: 386988E8  addi r3, r9, -0x7718
	ctx.r[3].s64 = ctx.r[9].s64 + -30488;
	// 8324D314: 4BA5CC0D  bl 0x82ca9f20
	ctx.lr = 0x8324D318;
	sub_82CA9F20(ctx, base);
	// 8324D318: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324D31C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324D320: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324D324: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324D328(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324D328 size=64
    let mut pc: u32 = 0x8324D328;
    'dispatch: loop {
        match pc {
            0x8324D328 => {
    //   block [0x8324D328..0x8324D368)
	// 8324D328: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324D32C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324D330: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324D334: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 8324D338: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324D33C: 388BE234  addi r4, r11, -0x1dcc
	ctx.r[4].s64 = ctx.r[11].s64 + -7628;
	// 8324D340: 386A7BB4  addi r3, r10, 0x7bb4
	ctx.r[3].s64 = ctx.r[10].s64 + 31668;
	// 8324D344: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324D348: 4AFDFB89  bl 0x8222ced0
	ctx.lr = 0x8324D34C;
	sub_8222CED0(ctx, base);
	// 8324D34C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8324D350: 386988F8  addi r3, r9, -0x7708
	ctx.r[3].s64 = ctx.r[9].s64 + -30472;
	// 8324D354: 4BA5CBCD  bl 0x82ca9f20
	ctx.lr = 0x8324D358;
	sub_82CA9F20(ctx, base);
	// 8324D358: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324D35C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324D360: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324D364: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324D368(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324D368 size=64
    let mut pc: u32 = 0x8324D368;
    'dispatch: loop {
        match pc {
            0x8324D368 => {
    //   block [0x8324D368..0x8324D3A8)
	// 8324D368: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324D36C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324D370: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324D374: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8324D378: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324D37C: 388B1AD4  addi r4, r11, 0x1ad4
	ctx.r[4].s64 = ctx.r[11].s64 + 6868;
	// 8324D380: 386A7BB8  addi r3, r10, 0x7bb8
	ctx.r[3].s64 = ctx.r[10].s64 + 31672;
	// 8324D384: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324D388: 4AFDFB49  bl 0x8222ced0
	ctx.lr = 0x8324D38C;
	sub_8222CED0(ctx, base);
	// 8324D38C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8324D390: 38698908  addi r3, r9, -0x76f8
	ctx.r[3].s64 = ctx.r[9].s64 + -30456;
	// 8324D394: 4BA5CB8D  bl 0x82ca9f20
	ctx.lr = 0x8324D398;
	sub_82CA9F20(ctx, base);
	// 8324D398: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324D39C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324D3A0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324D3A4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324D3A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324D3A8 size=64
    let mut pc: u32 = 0x8324D3A8;
    'dispatch: loop {
        match pc {
            0x8324D3A8 => {
    //   block [0x8324D3A8..0x8324D3E8)
	// 8324D3A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324D3AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324D3B0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324D3B4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8324D3B8: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324D3BC: 388B1ADC  addi r4, r11, 0x1adc
	ctx.r[4].s64 = ctx.r[11].s64 + 6876;
	// 8324D3C0: 386A7BBC  addi r3, r10, 0x7bbc
	ctx.r[3].s64 = ctx.r[10].s64 + 31676;
	// 8324D3C4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324D3C8: 4AFDFB09  bl 0x8222ced0
	ctx.lr = 0x8324D3CC;
	sub_8222CED0(ctx, base);
	// 8324D3CC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8324D3D0: 38698918  addi r3, r9, -0x76e8
	ctx.r[3].s64 = ctx.r[9].s64 + -30440;
	// 8324D3D4: 4BA5CB4D  bl 0x82ca9f20
	ctx.lr = 0x8324D3D8;
	sub_82CA9F20(ctx, base);
	// 8324D3D8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324D3DC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324D3E0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324D3E4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324D3E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324D3E8 size=64
    let mut pc: u32 = 0x8324D3E8;
    'dispatch: loop {
        match pc {
            0x8324D3E8 => {
    //   block [0x8324D3E8..0x8324D428)
	// 8324D3E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324D3EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324D3F0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324D3F4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8324D3F8: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324D3FC: 388B1AF0  addi r4, r11, 0x1af0
	ctx.r[4].s64 = ctx.r[11].s64 + 6896;
	// 8324D400: 386A7BC0  addi r3, r10, 0x7bc0
	ctx.r[3].s64 = ctx.r[10].s64 + 31680;
	// 8324D404: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324D408: 4AFDFAC9  bl 0x8222ced0
	ctx.lr = 0x8324D40C;
	sub_8222CED0(ctx, base);
	// 8324D40C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8324D410: 38698928  addi r3, r9, -0x76d8
	ctx.r[3].s64 = ctx.r[9].s64 + -30424;
	// 8324D414: 4BA5CB0D  bl 0x82ca9f20
	ctx.lr = 0x8324D418;
	sub_82CA9F20(ctx, base);
	// 8324D418: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324D41C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324D420: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324D424: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324D428(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x8324D428 size=32
    let mut pc: u32 = 0x8324D428;
    'dispatch: loop {
        match pc {
            0x8324D428 => {
    //   block [0x8324D428..0x8324D448)
	// 8324D428: 3D608332  lis r11, -0x7cce
	ctx.r[11].s64 = -2093875200;
	// 8324D42C: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324D430: 392B9A2C  addi r9, r11, -0x65d4
	ctx.r[9].s64 = ctx.r[11].s64 + -26068;
	// 8324D434: C1AB9A2C  lfs f13, -0x65d4(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-26068 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 8324D438: C009FFF8  lfs f0, -8(r9)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(-8 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8324D43C: EC006828  fsubs f0, f0, f13
	ctx.f[0].f64 = (((ctx.f[0].f64 - ctx.f[13].f64) as f32) as f64);
	// 8324D440: D00A7BC4  stfs f0, 0x7bc4(r10)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(31684 as u32), tmp.u32 ) };
	// 8324D444: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324D448(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324D448 size=64
    let mut pc: u32 = 0x8324D448;
    'dispatch: loop {
        match pc {
            0x8324D448 => {
    //   block [0x8324D448..0x8324D488)
	// 8324D448: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324D44C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324D450: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324D454: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8324D458: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324D45C: 388B1AD4  addi r4, r11, 0x1ad4
	ctx.r[4].s64 = ctx.r[11].s64 + 6868;
	// 8324D460: 386A7BC8  addi r3, r10, 0x7bc8
	ctx.r[3].s64 = ctx.r[10].s64 + 31688;
	// 8324D464: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324D468: 4AFDFA69  bl 0x8222ced0
	ctx.lr = 0x8324D46C;
	sub_8222CED0(ctx, base);
	// 8324D46C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8324D470: 38698938  addi r3, r9, -0x76c8
	ctx.r[3].s64 = ctx.r[9].s64 + -30408;
	// 8324D474: 4BA5CAAD  bl 0x82ca9f20
	ctx.lr = 0x8324D478;
	sub_82CA9F20(ctx, base);
	// 8324D478: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324D47C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324D480: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324D484: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324D488(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324D488 size=64
    let mut pc: u32 = 0x8324D488;
    'dispatch: loop {
        match pc {
            0x8324D488 => {
    //   block [0x8324D488..0x8324D4C8)
	// 8324D488: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324D48C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324D490: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324D494: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8324D498: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324D49C: 388B1ADC  addi r4, r11, 0x1adc
	ctx.r[4].s64 = ctx.r[11].s64 + 6876;
	// 8324D4A0: 386A7BCC  addi r3, r10, 0x7bcc
	ctx.r[3].s64 = ctx.r[10].s64 + 31692;
	// 8324D4A4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324D4A8: 4AFDFA29  bl 0x8222ced0
	ctx.lr = 0x8324D4AC;
	sub_8222CED0(ctx, base);
	// 8324D4AC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8324D4B0: 38698948  addi r3, r9, -0x76b8
	ctx.r[3].s64 = ctx.r[9].s64 + -30392;
	// 8324D4B4: 4BA5CA6D  bl 0x82ca9f20
	ctx.lr = 0x8324D4B8;
	sub_82CA9F20(ctx, base);
	// 8324D4B8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324D4BC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324D4C0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324D4C4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324D4C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324D4C8 size=64
    let mut pc: u32 = 0x8324D4C8;
    'dispatch: loop {
        match pc {
            0x8324D4C8 => {
    //   block [0x8324D4C8..0x8324D508)
	// 8324D4C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324D4CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324D4D0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324D4D4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8324D4D8: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324D4DC: 388B1AF0  addi r4, r11, 0x1af0
	ctx.r[4].s64 = ctx.r[11].s64 + 6896;
	// 8324D4E0: 386A7BD0  addi r3, r10, 0x7bd0
	ctx.r[3].s64 = ctx.r[10].s64 + 31696;
	// 8324D4E4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324D4E8: 4AFDF9E9  bl 0x8222ced0
	ctx.lr = 0x8324D4EC;
	sub_8222CED0(ctx, base);
	// 8324D4EC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8324D4F0: 38698958  addi r3, r9, -0x76a8
	ctx.r[3].s64 = ctx.r[9].s64 + -30376;
	// 8324D4F4: 4BA5CA2D  bl 0x82ca9f20
	ctx.lr = 0x8324D4F8;
	sub_82CA9F20(ctx, base);
	// 8324D4F8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324D4FC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324D500: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324D504: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324D508(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324D508 size=64
    let mut pc: u32 = 0x8324D508;
    'dispatch: loop {
        match pc {
            0x8324D508 => {
    //   block [0x8324D508..0x8324D548)
	// 8324D508: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324D50C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324D510: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324D514: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8324D518: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324D51C: 388B052C  addi r4, r11, 0x52c
	ctx.r[4].s64 = ctx.r[11].s64 + 1324;
	// 8324D520: 386A7BD4  addi r3, r10, 0x7bd4
	ctx.r[3].s64 = ctx.r[10].s64 + 31700;
	// 8324D524: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324D528: 4AFDF9A9  bl 0x8222ced0
	ctx.lr = 0x8324D52C;
	sub_8222CED0(ctx, base);
	// 8324D52C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8324D530: 38698968  addi r3, r9, -0x7698
	ctx.r[3].s64 = ctx.r[9].s64 + -30360;
	// 8324D534: 4BA5C9ED  bl 0x82ca9f20
	ctx.lr = 0x8324D538;
	sub_82CA9F20(ctx, base);
	// 8324D538: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324D53C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324D540: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324D544: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324D548(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324D548 size=56
    let mut pc: u32 = 0x8324D548;
    'dispatch: loop {
        match pc {
            0x8324D548 => {
    //   block [0x8324D548..0x8324D580)
	// 8324D548: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324D54C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324D550: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324D554: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 8324D558: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8324D55C: 386BE3B0  addi r3, r11, -0x1c50
	ctx.r[3].s64 = ctx.r[11].s64 + -7248;
	// 8324D560: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8324D564: 4AFA67F5  bl 0x821f3d58
	ctx.lr = 0x8324D568;
	sub_821F3D58(ctx, base);
	// 8324D568: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324D56C: 906A7BD8  stw r3, 0x7bd8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(31704 as u32), ctx.r[3].u32 ) };
	// 8324D570: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324D574: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324D578: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324D57C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324D580(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324D580 size=56
    let mut pc: u32 = 0x8324D580;
    'dispatch: loop {
        match pc {
            0x8324D580 => {
    //   block [0x8324D580..0x8324D5B8)
	// 8324D580: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324D584: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324D588: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324D58C: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 8324D590: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8324D594: 386BE3C0  addi r3, r11, -0x1c40
	ctx.r[3].s64 = ctx.r[11].s64 + -7232;
	// 8324D598: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8324D59C: 4AFA67BD  bl 0x821f3d58
	ctx.lr = 0x8324D5A0;
	sub_821F3D58(ctx, base);
	// 8324D5A0: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324D5A4: 906A7BDC  stw r3, 0x7bdc(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(31708 as u32), ctx.r[3].u32 ) };
	// 8324D5A8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324D5AC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324D5B0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324D5B4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324D5B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324D5B8 size=64
    let mut pc: u32 = 0x8324D5B8;
    'dispatch: loop {
        match pc {
            0x8324D5B8 => {
    //   block [0x8324D5B8..0x8324D5F8)
	// 8324D5B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324D5BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324D5C0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324D5C4: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 8324D5C8: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324D5CC: 388BE3D0  addi r4, r11, -0x1c30
	ctx.r[4].s64 = ctx.r[11].s64 + -7216;
	// 8324D5D0: 386A7BE0  addi r3, r10, 0x7be0
	ctx.r[3].s64 = ctx.r[10].s64 + 31712;
	// 8324D5D4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324D5D8: 4AFDF8F9  bl 0x8222ced0
	ctx.lr = 0x8324D5DC;
	sub_8222CED0(ctx, base);
	// 8324D5DC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8324D5E0: 38698978  addi r3, r9, -0x7688
	ctx.r[3].s64 = ctx.r[9].s64 + -30344;
	// 8324D5E4: 4BA5C93D  bl 0x82ca9f20
	ctx.lr = 0x8324D5E8;
	sub_82CA9F20(ctx, base);
	// 8324D5E8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324D5EC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324D5F0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324D5F4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324D5F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324D5F8 size=64
    let mut pc: u32 = 0x8324D5F8;
    'dispatch: loop {
        match pc {
            0x8324D5F8 => {
    //   block [0x8324D5F8..0x8324D638)
	// 8324D5F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324D5FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324D600: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324D604: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 8324D608: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324D60C: 388BE3E0  addi r4, r11, -0x1c20
	ctx.r[4].s64 = ctx.r[11].s64 + -7200;
	// 8324D610: 386A7BE4  addi r3, r10, 0x7be4
	ctx.r[3].s64 = ctx.r[10].s64 + 31716;
	// 8324D614: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324D618: 4AFDF8B9  bl 0x8222ced0
	ctx.lr = 0x8324D61C;
	sub_8222CED0(ctx, base);
	// 8324D61C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8324D620: 38698988  addi r3, r9, -0x7678
	ctx.r[3].s64 = ctx.r[9].s64 + -30328;
	// 8324D624: 4BA5C8FD  bl 0x82ca9f20
	ctx.lr = 0x8324D628;
	sub_82CA9F20(ctx, base);
	// 8324D628: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324D62C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324D630: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324D634: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324D638(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324D638 size=64
    let mut pc: u32 = 0x8324D638;
    'dispatch: loop {
        match pc {
            0x8324D638 => {
    //   block [0x8324D638..0x8324D678)
	// 8324D638: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324D63C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324D640: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324D644: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 8324D648: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324D64C: 388BE41C  addi r4, r11, -0x1be4
	ctx.r[4].s64 = ctx.r[11].s64 + -7140;
	// 8324D650: 386A7BE8  addi r3, r10, 0x7be8
	ctx.r[3].s64 = ctx.r[10].s64 + 31720;
	// 8324D654: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324D658: 4AFDF879  bl 0x8222ced0
	ctx.lr = 0x8324D65C;
	sub_8222CED0(ctx, base);
	// 8324D65C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8324D660: 38698998  addi r3, r9, -0x7668
	ctx.r[3].s64 = ctx.r[9].s64 + -30312;
	// 8324D664: 4BA5C8BD  bl 0x82ca9f20
	ctx.lr = 0x8324D668;
	sub_82CA9F20(ctx, base);
	// 8324D668: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324D66C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324D670: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324D674: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324D678(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324D678 size=64
    let mut pc: u32 = 0x8324D678;
    'dispatch: loop {
        match pc {
            0x8324D678 => {
    //   block [0x8324D678..0x8324D6B8)
	// 8324D678: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324D67C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324D680: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324D684: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 8324D688: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324D68C: 388BE438  addi r4, r11, -0x1bc8
	ctx.r[4].s64 = ctx.r[11].s64 + -7112;
	// 8324D690: 386A7BEC  addi r3, r10, 0x7bec
	ctx.r[3].s64 = ctx.r[10].s64 + 31724;
	// 8324D694: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324D698: 4AFDF839  bl 0x8222ced0
	ctx.lr = 0x8324D69C;
	sub_8222CED0(ctx, base);
	// 8324D69C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8324D6A0: 386989A8  addi r3, r9, -0x7658
	ctx.r[3].s64 = ctx.r[9].s64 + -30296;
	// 8324D6A4: 4BA5C87D  bl 0x82ca9f20
	ctx.lr = 0x8324D6A8;
	sub_82CA9F20(ctx, base);
	// 8324D6A8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324D6AC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324D6B0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324D6B4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324D6B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324D6B8 size=64
    let mut pc: u32 = 0x8324D6B8;
    'dispatch: loop {
        match pc {
            0x8324D6B8 => {
    //   block [0x8324D6B8..0x8324D6F8)
	// 8324D6B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324D6BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324D6C0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324D6C4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8324D6C8: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324D6CC: 388B052C  addi r4, r11, 0x52c
	ctx.r[4].s64 = ctx.r[11].s64 + 1324;
	// 8324D6D0: 386A7BF0  addi r3, r10, 0x7bf0
	ctx.r[3].s64 = ctx.r[10].s64 + 31728;
	// 8324D6D4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324D6D8: 4AFDF7F9  bl 0x8222ced0
	ctx.lr = 0x8324D6DC;
	sub_8222CED0(ctx, base);
	// 8324D6DC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8324D6E0: 386989B8  addi r3, r9, -0x7648
	ctx.r[3].s64 = ctx.r[9].s64 + -30280;
	// 8324D6E4: 4BA5C83D  bl 0x82ca9f20
	ctx.lr = 0x8324D6E8;
	sub_82CA9F20(ctx, base);
	// 8324D6E8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324D6EC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324D6F0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324D6F4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324D6F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324D6F8 size=64
    let mut pc: u32 = 0x8324D6F8;
    'dispatch: loop {
        match pc {
            0x8324D6F8 => {
    //   block [0x8324D6F8..0x8324D738)
	// 8324D6F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324D6FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324D700: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324D704: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8324D708: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324D70C: 388B052C  addi r4, r11, 0x52c
	ctx.r[4].s64 = ctx.r[11].s64 + 1324;
	// 8324D710: 386A7BF8  addi r3, r10, 0x7bf8
	ctx.r[3].s64 = ctx.r[10].s64 + 31736;
	// 8324D714: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324D718: 4AFDF7B9  bl 0x8222ced0
	ctx.lr = 0x8324D71C;
	sub_8222CED0(ctx, base);
	// 8324D71C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8324D720: 386989C8  addi r3, r9, -0x7638
	ctx.r[3].s64 = ctx.r[9].s64 + -30264;
	// 8324D724: 4BA5C7FD  bl 0x82ca9f20
	ctx.lr = 0x8324D728;
	sub_82CA9F20(ctx, base);
	// 8324D728: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324D72C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324D730: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324D734: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324D738(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324D738 size=64
    let mut pc: u32 = 0x8324D738;
    'dispatch: loop {
        match pc {
            0x8324D738 => {
    //   block [0x8324D738..0x8324D778)
	// 8324D738: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324D73C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324D740: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324D744: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8324D748: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324D74C: 388B1AD4  addi r4, r11, 0x1ad4
	ctx.r[4].s64 = ctx.r[11].s64 + 6868;
	// 8324D750: 386A7BFC  addi r3, r10, 0x7bfc
	ctx.r[3].s64 = ctx.r[10].s64 + 31740;
	// 8324D754: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324D758: 4AFDF779  bl 0x8222ced0
	ctx.lr = 0x8324D75C;
	sub_8222CED0(ctx, base);
	// 8324D75C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8324D760: 386989D8  addi r3, r9, -0x7628
	ctx.r[3].s64 = ctx.r[9].s64 + -30248;
	// 8324D764: 4BA5C7BD  bl 0x82ca9f20
	ctx.lr = 0x8324D768;
	sub_82CA9F20(ctx, base);
	// 8324D768: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324D76C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324D770: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324D774: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324D778(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324D778 size=64
    let mut pc: u32 = 0x8324D778;
    'dispatch: loop {
        match pc {
            0x8324D778 => {
    //   block [0x8324D778..0x8324D7B8)
	// 8324D778: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324D77C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324D780: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324D784: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8324D788: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324D78C: 388B1ADC  addi r4, r11, 0x1adc
	ctx.r[4].s64 = ctx.r[11].s64 + 6876;
	// 8324D790: 386A7C00  addi r3, r10, 0x7c00
	ctx.r[3].s64 = ctx.r[10].s64 + 31744;
	// 8324D794: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324D798: 4AFDF739  bl 0x8222ced0
	ctx.lr = 0x8324D79C;
	sub_8222CED0(ctx, base);
	// 8324D79C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8324D7A0: 386989E8  addi r3, r9, -0x7618
	ctx.r[3].s64 = ctx.r[9].s64 + -30232;
	// 8324D7A4: 4BA5C77D  bl 0x82ca9f20
	ctx.lr = 0x8324D7A8;
	sub_82CA9F20(ctx, base);
	// 8324D7A8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324D7AC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324D7B0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324D7B4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324D7B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324D7B8 size=64
    let mut pc: u32 = 0x8324D7B8;
    'dispatch: loop {
        match pc {
            0x8324D7B8 => {
    //   block [0x8324D7B8..0x8324D7F8)
	// 8324D7B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324D7BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324D7C0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324D7C4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8324D7C8: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324D7CC: 388B1AF0  addi r4, r11, 0x1af0
	ctx.r[4].s64 = ctx.r[11].s64 + 6896;
	// 8324D7D0: 386A7C04  addi r3, r10, 0x7c04
	ctx.r[3].s64 = ctx.r[10].s64 + 31748;
	// 8324D7D4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324D7D8: 4AFDF6F9  bl 0x8222ced0
	ctx.lr = 0x8324D7DC;
	sub_8222CED0(ctx, base);
	// 8324D7DC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8324D7E0: 386989F8  addi r3, r9, -0x7608
	ctx.r[3].s64 = ctx.r[9].s64 + -30216;
	// 8324D7E4: 4BA5C73D  bl 0x82ca9f20
	ctx.lr = 0x8324D7E8;
	sub_82CA9F20(ctx, base);
	// 8324D7E8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324D7EC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324D7F0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324D7F4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324D7F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324D7F8 size=64
    let mut pc: u32 = 0x8324D7F8;
    'dispatch: loop {
        match pc {
            0x8324D7F8 => {
    //   block [0x8324D7F8..0x8324D838)
	// 8324D7F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324D7FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324D800: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324D804: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 8324D808: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324D80C: 388B8D00  addi r4, r11, -0x7300
	ctx.r[4].s64 = ctx.r[11].s64 + -29440;
	// 8324D810: 386A7C08  addi r3, r10, 0x7c08
	ctx.r[3].s64 = ctx.r[10].s64 + 31752;
	// 8324D814: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324D818: 4AFDF6B9  bl 0x8222ced0
	ctx.lr = 0x8324D81C;
	sub_8222CED0(ctx, base);
	// 8324D81C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8324D820: 38698A08  addi r3, r9, -0x75f8
	ctx.r[3].s64 = ctx.r[9].s64 + -30200;
	// 8324D824: 4BA5C6FD  bl 0x82ca9f20
	ctx.lr = 0x8324D828;
	sub_82CA9F20(ctx, base);
	// 8324D828: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324D82C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324D830: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324D834: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324D838(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324D838 size=64
    let mut pc: u32 = 0x8324D838;
    'dispatch: loop {
        match pc {
            0x8324D838 => {
    //   block [0x8324D838..0x8324D878)
	// 8324D838: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324D83C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324D840: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324D844: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 8324D848: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324D84C: 388B8D18  addi r4, r11, -0x72e8
	ctx.r[4].s64 = ctx.r[11].s64 + -29416;
	// 8324D850: 386A7C0C  addi r3, r10, 0x7c0c
	ctx.r[3].s64 = ctx.r[10].s64 + 31756;
	// 8324D854: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324D858: 4AFDF679  bl 0x8222ced0
	ctx.lr = 0x8324D85C;
	sub_8222CED0(ctx, base);
	// 8324D85C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8324D860: 38698A18  addi r3, r9, -0x75e8
	ctx.r[3].s64 = ctx.r[9].s64 + -30184;
	// 8324D864: 4BA5C6BD  bl 0x82ca9f20
	ctx.lr = 0x8324D868;
	sub_82CA9F20(ctx, base);
	// 8324D868: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324D86C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324D870: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324D874: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324D878(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324D878 size=64
    let mut pc: u32 = 0x8324D878;
    'dispatch: loop {
        match pc {
            0x8324D878 => {
    //   block [0x8324D878..0x8324D8B8)
	// 8324D878: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324D87C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324D880: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324D884: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8324D888: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324D88C: 388B1AD4  addi r4, r11, 0x1ad4
	ctx.r[4].s64 = ctx.r[11].s64 + 6868;
	// 8324D890: 386A7C10  addi r3, r10, 0x7c10
	ctx.r[3].s64 = ctx.r[10].s64 + 31760;
	// 8324D894: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324D898: 4AFDF639  bl 0x8222ced0
	ctx.lr = 0x8324D89C;
	sub_8222CED0(ctx, base);
	// 8324D89C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8324D8A0: 38698A28  addi r3, r9, -0x75d8
	ctx.r[3].s64 = ctx.r[9].s64 + -30168;
	// 8324D8A4: 4BA5C67D  bl 0x82ca9f20
	ctx.lr = 0x8324D8A8;
	sub_82CA9F20(ctx, base);
	// 8324D8A8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324D8AC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324D8B0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324D8B4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324D8B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324D8B8 size=64
    let mut pc: u32 = 0x8324D8B8;
    'dispatch: loop {
        match pc {
            0x8324D8B8 => {
    //   block [0x8324D8B8..0x8324D8F8)
	// 8324D8B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324D8BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324D8C0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324D8C4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8324D8C8: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324D8CC: 388B1ADC  addi r4, r11, 0x1adc
	ctx.r[4].s64 = ctx.r[11].s64 + 6876;
	// 8324D8D0: 386A7C14  addi r3, r10, 0x7c14
	ctx.r[3].s64 = ctx.r[10].s64 + 31764;
	// 8324D8D4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324D8D8: 4AFDF5F9  bl 0x8222ced0
	ctx.lr = 0x8324D8DC;
	sub_8222CED0(ctx, base);
	// 8324D8DC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8324D8E0: 38698A38  addi r3, r9, -0x75c8
	ctx.r[3].s64 = ctx.r[9].s64 + -30152;
	// 8324D8E4: 4BA5C63D  bl 0x82ca9f20
	ctx.lr = 0x8324D8E8;
	sub_82CA9F20(ctx, base);
	// 8324D8E8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324D8EC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324D8F0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324D8F4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324D8F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324D8F8 size=64
    let mut pc: u32 = 0x8324D8F8;
    'dispatch: loop {
        match pc {
            0x8324D8F8 => {
    //   block [0x8324D8F8..0x8324D938)
	// 8324D8F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324D8FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324D900: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324D904: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8324D908: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324D90C: 388B1AF0  addi r4, r11, 0x1af0
	ctx.r[4].s64 = ctx.r[11].s64 + 6896;
	// 8324D910: 386A7C18  addi r3, r10, 0x7c18
	ctx.r[3].s64 = ctx.r[10].s64 + 31768;
	// 8324D914: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324D918: 4AFDF5B9  bl 0x8222ced0
	ctx.lr = 0x8324D91C;
	sub_8222CED0(ctx, base);
	// 8324D91C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8324D920: 38698A48  addi r3, r9, -0x75b8
	ctx.r[3].s64 = ctx.r[9].s64 + -30136;
	// 8324D924: 4BA5C5FD  bl 0x82ca9f20
	ctx.lr = 0x8324D928;
	sub_82CA9F20(ctx, base);
	// 8324D928: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324D92C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324D930: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324D934: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8324D938(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8324D938 size=64
    let mut pc: u32 = 0x8324D938;
    'dispatch: loop {
        match pc {
            0x8324D938 => {
    //   block [0x8324D938..0x8324D978)
	// 8324D938: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8324D93C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8324D940: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8324D944: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8324D948: 3D408349  lis r10, -0x7cb7
	ctx.r[10].s64 = -2092367872;
	// 8324D94C: 388B052C  addi r4, r11, 0x52c
	ctx.r[4].s64 = ctx.r[11].s64 + 1324;
	// 8324D950: 386A7C1C  addi r3, r10, 0x7c1c
	ctx.r[3].s64 = ctx.r[10].s64 + 31772;
	// 8324D954: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8324D958: 4AFDF579  bl 0x8222ced0
	ctx.lr = 0x8324D95C;
	sub_8222CED0(ctx, base);
	// 8324D95C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8324D960: 38698A58  addi r3, r9, -0x75a8
	ctx.r[3].s64 = ctx.r[9].s64 + -30120;
	// 8324D964: 4BA5C5BD  bl 0x82ca9f20
	ctx.lr = 0x8324D968;
	sub_82CA9F20(ctx, base);
	// 8324D968: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8324D96C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8324D970: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8324D974: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


