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


pub fn sub_8325E830(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325E830 size=56
    let mut pc: u32 = 0x8325E830;
    'dispatch: loop {
        match pc {
            0x8325E830 => {
    //   block [0x8325E830..0x8325E868)
	// 8325E830: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325E834: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325E838: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325E83C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8325E840: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8325E844: 386B4328  addi r3, r11, 0x4328
	ctx.r[3].s64 = ctx.r[11].s64 + 17192;
	// 8325E848: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8325E84C: 4AF9550D  bl 0x821f3d58
	ctx.lr = 0x8325E850;
	sub_821F3D58(ctx, base);
	// 8325E850: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325E854: 906AAEB8  stw r3, -0x5148(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-20808 as u32), ctx.r[3].u32 ) };
	// 8325E858: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325E85C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325E860: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325E864: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325E868(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325E868 size=56
    let mut pc: u32 = 0x8325E868;
    'dispatch: loop {
        match pc {
            0x8325E868 => {
    //   block [0x8325E868..0x8325E8A0)
	// 8325E868: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325E86C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325E870: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325E874: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8325E878: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8325E87C: 386B4338  addi r3, r11, 0x4338
	ctx.r[3].s64 = ctx.r[11].s64 + 17208;
	// 8325E880: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8325E884: 4AF954D5  bl 0x821f3d58
	ctx.lr = 0x8325E888;
	sub_821F3D58(ctx, base);
	// 8325E888: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325E88C: 906AAEBC  stw r3, -0x5144(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-20804 as u32), ctx.r[3].u32 ) };
	// 8325E890: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325E894: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325E898: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325E89C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325E8A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325E8A0 size=56
    let mut pc: u32 = 0x8325E8A0;
    'dispatch: loop {
        match pc {
            0x8325E8A0 => {
    //   block [0x8325E8A0..0x8325E8D8)
	// 8325E8A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325E8A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325E8A8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325E8AC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8325E8B0: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8325E8B4: 386B4350  addi r3, r11, 0x4350
	ctx.r[3].s64 = ctx.r[11].s64 + 17232;
	// 8325E8B8: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8325E8BC: 4AF9549D  bl 0x821f3d58
	ctx.lr = 0x8325E8C0;
	sub_821F3D58(ctx, base);
	// 8325E8C0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325E8C4: 906AAEC0  stw r3, -0x5140(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-20800 as u32), ctx.r[3].u32 ) };
	// 8325E8C8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325E8CC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325E8D0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325E8D4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325E8D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325E8D8 size=56
    let mut pc: u32 = 0x8325E8D8;
    'dispatch: loop {
        match pc {
            0x8325E8D8 => {
    //   block [0x8325E8D8..0x8325E910)
	// 8325E8D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325E8DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325E8E0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325E8E4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8325E8E8: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8325E8EC: 386B4368  addi r3, r11, 0x4368
	ctx.r[3].s64 = ctx.r[11].s64 + 17256;
	// 8325E8F0: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8325E8F4: 4AF95465  bl 0x821f3d58
	ctx.lr = 0x8325E8F8;
	sub_821F3D58(ctx, base);
	// 8325E8F8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325E8FC: 906AAEC4  stw r3, -0x513c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-20796 as u32), ctx.r[3].u32 ) };
	// 8325E900: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325E904: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325E908: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325E90C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325E910(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325E910 size=56
    let mut pc: u32 = 0x8325E910;
    'dispatch: loop {
        match pc {
            0x8325E910 => {
    //   block [0x8325E910..0x8325E948)
	// 8325E910: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325E914: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325E918: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325E91C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8325E920: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8325E924: 386B4380  addi r3, r11, 0x4380
	ctx.r[3].s64 = ctx.r[11].s64 + 17280;
	// 8325E928: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8325E92C: 4AF9542D  bl 0x821f3d58
	ctx.lr = 0x8325E930;
	sub_821F3D58(ctx, base);
	// 8325E930: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325E934: 906AAEC8  stw r3, -0x5138(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-20792 as u32), ctx.r[3].u32 ) };
	// 8325E938: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325E93C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325E940: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325E944: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325E948(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325E948 size=56
    let mut pc: u32 = 0x8325E948;
    'dispatch: loop {
        match pc {
            0x8325E948 => {
    //   block [0x8325E948..0x8325E980)
	// 8325E948: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325E94C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325E950: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325E954: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8325E958: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8325E95C: 386B4394  addi r3, r11, 0x4394
	ctx.r[3].s64 = ctx.r[11].s64 + 17300;
	// 8325E960: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8325E964: 4AF953F5  bl 0x821f3d58
	ctx.lr = 0x8325E968;
	sub_821F3D58(ctx, base);
	// 8325E968: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325E96C: 906AAECC  stw r3, -0x5134(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-20788 as u32), ctx.r[3].u32 ) };
	// 8325E970: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325E974: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325E978: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325E97C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325E980(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325E980 size=56
    let mut pc: u32 = 0x8325E980;
    'dispatch: loop {
        match pc {
            0x8325E980 => {
    //   block [0x8325E980..0x8325E9B8)
	// 8325E980: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325E984: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325E988: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325E98C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8325E990: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8325E994: 386B43B4  addi r3, r11, 0x43b4
	ctx.r[3].s64 = ctx.r[11].s64 + 17332;
	// 8325E998: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8325E99C: 4AF953BD  bl 0x821f3d58
	ctx.lr = 0x8325E9A0;
	sub_821F3D58(ctx, base);
	// 8325E9A0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325E9A4: 906AAED0  stw r3, -0x5130(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-20784 as u32), ctx.r[3].u32 ) };
	// 8325E9A8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325E9AC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325E9B0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325E9B4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325E9B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325E9B8 size=56
    let mut pc: u32 = 0x8325E9B8;
    'dispatch: loop {
        match pc {
            0x8325E9B8 => {
    //   block [0x8325E9B8..0x8325E9F0)
	// 8325E9B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325E9BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325E9C0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325E9C4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8325E9C8: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8325E9CC: 386B43C8  addi r3, r11, 0x43c8
	ctx.r[3].s64 = ctx.r[11].s64 + 17352;
	// 8325E9D0: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8325E9D4: 4AF95385  bl 0x821f3d58
	ctx.lr = 0x8325E9D8;
	sub_821F3D58(ctx, base);
	// 8325E9D8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325E9DC: 906AAED4  stw r3, -0x512c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-20780 as u32), ctx.r[3].u32 ) };
	// 8325E9E0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325E9E4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325E9E8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325E9EC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325E9F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325E9F0 size=56
    let mut pc: u32 = 0x8325E9F0;
    'dispatch: loop {
        match pc {
            0x8325E9F0 => {
    //   block [0x8325E9F0..0x8325EA28)
	// 8325E9F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325E9F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325E9F8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325E9FC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8325EA00: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8325EA04: 386B43D4  addi r3, r11, 0x43d4
	ctx.r[3].s64 = ctx.r[11].s64 + 17364;
	// 8325EA08: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8325EA0C: 4AF9534D  bl 0x821f3d58
	ctx.lr = 0x8325EA10;
	sub_821F3D58(ctx, base);
	// 8325EA10: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325EA14: 906AAED8  stw r3, -0x5128(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-20776 as u32), ctx.r[3].u32 ) };
	// 8325EA18: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325EA1C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325EA20: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325EA24: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325EA28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325EA28 size=56
    let mut pc: u32 = 0x8325EA28;
    'dispatch: loop {
        match pc {
            0x8325EA28 => {
    //   block [0x8325EA28..0x8325EA60)
	// 8325EA28: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325EA2C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325EA30: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325EA34: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8325EA38: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8325EA3C: 386B43E0  addi r3, r11, 0x43e0
	ctx.r[3].s64 = ctx.r[11].s64 + 17376;
	// 8325EA40: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8325EA44: 4AF95315  bl 0x821f3d58
	ctx.lr = 0x8325EA48;
	sub_821F3D58(ctx, base);
	// 8325EA48: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325EA4C: 906AAEDC  stw r3, -0x5124(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-20772 as u32), ctx.r[3].u32 ) };
	// 8325EA50: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325EA54: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325EA58: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325EA5C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325EA60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325EA60 size=56
    let mut pc: u32 = 0x8325EA60;
    'dispatch: loop {
        match pc {
            0x8325EA60 => {
    //   block [0x8325EA60..0x8325EA98)
	// 8325EA60: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325EA64: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325EA68: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325EA6C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8325EA70: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8325EA74: 386B43F0  addi r3, r11, 0x43f0
	ctx.r[3].s64 = ctx.r[11].s64 + 17392;
	// 8325EA78: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8325EA7C: 4AF952DD  bl 0x821f3d58
	ctx.lr = 0x8325EA80;
	sub_821F3D58(ctx, base);
	// 8325EA80: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325EA84: 906AAEE0  stw r3, -0x5120(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-20768 as u32), ctx.r[3].u32 ) };
	// 8325EA88: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325EA8C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325EA90: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325EA94: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325EA98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325EA98 size=56
    let mut pc: u32 = 0x8325EA98;
    'dispatch: loop {
        match pc {
            0x8325EA98 => {
    //   block [0x8325EA98..0x8325EAD0)
	// 8325EA98: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325EA9C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325EAA0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325EAA4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8325EAA8: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8325EAAC: 386B4400  addi r3, r11, 0x4400
	ctx.r[3].s64 = ctx.r[11].s64 + 17408;
	// 8325EAB0: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8325EAB4: 4AF952A5  bl 0x821f3d58
	ctx.lr = 0x8325EAB8;
	sub_821F3D58(ctx, base);
	// 8325EAB8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325EABC: 906AAEE4  stw r3, -0x511c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-20764 as u32), ctx.r[3].u32 ) };
	// 8325EAC0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325EAC4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325EAC8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325EACC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325EAD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325EAD0 size=56
    let mut pc: u32 = 0x8325EAD0;
    'dispatch: loop {
        match pc {
            0x8325EAD0 => {
    //   block [0x8325EAD0..0x8325EB08)
	// 8325EAD0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325EAD4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325EAD8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325EADC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8325EAE0: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8325EAE4: 386B4418  addi r3, r11, 0x4418
	ctx.r[3].s64 = ctx.r[11].s64 + 17432;
	// 8325EAE8: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8325EAEC: 4AF9526D  bl 0x821f3d58
	ctx.lr = 0x8325EAF0;
	sub_821F3D58(ctx, base);
	// 8325EAF0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325EAF4: 906AAEE8  stw r3, -0x5118(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-20760 as u32), ctx.r[3].u32 ) };
	// 8325EAF8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325EAFC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325EB00: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325EB04: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325EB08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325EB08 size=56
    let mut pc: u32 = 0x8325EB08;
    'dispatch: loop {
        match pc {
            0x8325EB08 => {
    //   block [0x8325EB08..0x8325EB40)
	// 8325EB08: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325EB0C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325EB10: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325EB14: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8325EB18: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8325EB1C: 386B4424  addi r3, r11, 0x4424
	ctx.r[3].s64 = ctx.r[11].s64 + 17444;
	// 8325EB20: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8325EB24: 4AF95235  bl 0x821f3d58
	ctx.lr = 0x8325EB28;
	sub_821F3D58(ctx, base);
	// 8325EB28: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325EB2C: 906AAEEC  stw r3, -0x5114(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-20756 as u32), ctx.r[3].u32 ) };
	// 8325EB30: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325EB34: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325EB38: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325EB3C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325EB40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325EB40 size=56
    let mut pc: u32 = 0x8325EB40;
    'dispatch: loop {
        match pc {
            0x8325EB40 => {
    //   block [0x8325EB40..0x8325EB78)
	// 8325EB40: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325EB44: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325EB48: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325EB4C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8325EB50: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8325EB54: 386B4430  addi r3, r11, 0x4430
	ctx.r[3].s64 = ctx.r[11].s64 + 17456;
	// 8325EB58: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8325EB5C: 4AF951FD  bl 0x821f3d58
	ctx.lr = 0x8325EB60;
	sub_821F3D58(ctx, base);
	// 8325EB60: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325EB64: 906AAEF0  stw r3, -0x5110(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-20752 as u32), ctx.r[3].u32 ) };
	// 8325EB68: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325EB6C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325EB70: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325EB74: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325EB78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325EB78 size=56
    let mut pc: u32 = 0x8325EB78;
    'dispatch: loop {
        match pc {
            0x8325EB78 => {
    //   block [0x8325EB78..0x8325EBB0)
	// 8325EB78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325EB7C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325EB80: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325EB84: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8325EB88: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8325EB8C: 386B4440  addi r3, r11, 0x4440
	ctx.r[3].s64 = ctx.r[11].s64 + 17472;
	// 8325EB90: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8325EB94: 4AF951C5  bl 0x821f3d58
	ctx.lr = 0x8325EB98;
	sub_821F3D58(ctx, base);
	// 8325EB98: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325EB9C: 906AAEF4  stw r3, -0x510c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-20748 as u32), ctx.r[3].u32 ) };
	// 8325EBA0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325EBA4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325EBA8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325EBAC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325EBB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325EBB0 size=64
    let mut pc: u32 = 0x8325EBB0;
    'dispatch: loop {
        match pc {
            0x8325EBB0 => {
    //   block [0x8325EBB0..0x8325EBF0)
	// 8325EBB0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325EBB4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325EBB8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325EBBC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8325EBC0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325EBC4: 388B1AD4  addi r4, r11, 0x1ad4
	ctx.r[4].s64 = ctx.r[11].s64 + 6868;
	// 8325EBC8: 386AAEF8  addi r3, r10, -0x5108
	ctx.r[3].s64 = ctx.r[10].s64 + -20744;
	// 8325EBCC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8325EBD0: 4AFCE301  bl 0x8222ced0
	ctx.lr = 0x8325EBD4;
	sub_8222CED0(ctx, base);
	// 8325EBD4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8325EBD8: 3869C620  addi r3, r9, -0x39e0
	ctx.r[3].s64 = ctx.r[9].s64 + -14816;
	// 8325EBDC: 4BA4B345  bl 0x82ca9f20
	ctx.lr = 0x8325EBE0;
	sub_82CA9F20(ctx, base);
	// 8325EBE0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325EBE4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325EBE8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325EBEC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325EBF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325EBF0 size=64
    let mut pc: u32 = 0x8325EBF0;
    'dispatch: loop {
        match pc {
            0x8325EBF0 => {
    //   block [0x8325EBF0..0x8325EC30)
	// 8325EBF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325EBF4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325EBF8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325EBFC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8325EC00: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325EC04: 388B1ADC  addi r4, r11, 0x1adc
	ctx.r[4].s64 = ctx.r[11].s64 + 6876;
	// 8325EC08: 386AAEFC  addi r3, r10, -0x5104
	ctx.r[3].s64 = ctx.r[10].s64 + -20740;
	// 8325EC0C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8325EC10: 4AFCE2C1  bl 0x8222ced0
	ctx.lr = 0x8325EC14;
	sub_8222CED0(ctx, base);
	// 8325EC14: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8325EC18: 3869C630  addi r3, r9, -0x39d0
	ctx.r[3].s64 = ctx.r[9].s64 + -14800;
	// 8325EC1C: 4BA4B305  bl 0x82ca9f20
	ctx.lr = 0x8325EC20;
	sub_82CA9F20(ctx, base);
	// 8325EC20: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325EC24: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325EC28: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325EC2C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325EC30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325EC30 size=64
    let mut pc: u32 = 0x8325EC30;
    'dispatch: loop {
        match pc {
            0x8325EC30 => {
    //   block [0x8325EC30..0x8325EC70)
	// 8325EC30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325EC34: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325EC38: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325EC3C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8325EC40: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325EC44: 388B1AF0  addi r4, r11, 0x1af0
	ctx.r[4].s64 = ctx.r[11].s64 + 6896;
	// 8325EC48: 386AAF00  addi r3, r10, -0x5100
	ctx.r[3].s64 = ctx.r[10].s64 + -20736;
	// 8325EC4C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8325EC50: 4AFCE281  bl 0x8222ced0
	ctx.lr = 0x8325EC54;
	sub_8222CED0(ctx, base);
	// 8325EC54: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8325EC58: 3869C640  addi r3, r9, -0x39c0
	ctx.r[3].s64 = ctx.r[9].s64 + -14784;
	// 8325EC5C: 4BA4B2C5  bl 0x82ca9f20
	ctx.lr = 0x8325EC60;
	sub_82CA9F20(ctx, base);
	// 8325EC60: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325EC64: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325EC68: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325EC6C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325EC70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325EC70 size=64
    let mut pc: u32 = 0x8325EC70;
    'dispatch: loop {
        match pc {
            0x8325EC70 => {
    //   block [0x8325EC70..0x8325ECB0)
	// 8325EC70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325EC74: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325EC78: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325EC7C: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8325EC80: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325EC84: 388BED44  addi r4, r11, -0x12bc
	ctx.r[4].s64 = ctx.r[11].s64 + -4796;
	// 8325EC88: 386AAF04  addi r3, r10, -0x50fc
	ctx.r[3].s64 = ctx.r[10].s64 + -20732;
	// 8325EC8C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8325EC90: 4AFCE241  bl 0x8222ced0
	ctx.lr = 0x8325EC94;
	sub_8222CED0(ctx, base);
	// 8325EC94: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8325EC98: 3869C650  addi r3, r9, -0x39b0
	ctx.r[3].s64 = ctx.r[9].s64 + -14768;
	// 8325EC9C: 4BA4B285  bl 0x82ca9f20
	ctx.lr = 0x8325ECA0;
	sub_82CA9F20(ctx, base);
	// 8325ECA0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325ECA4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325ECA8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325ECAC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325ECB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325ECB0 size=64
    let mut pc: u32 = 0x8325ECB0;
    'dispatch: loop {
        match pc {
            0x8325ECB0 => {
    //   block [0x8325ECB0..0x8325ECF0)
	// 8325ECB0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325ECB4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325ECB8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325ECBC: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8325ECC0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325ECC4: 388BED50  addi r4, r11, -0x12b0
	ctx.r[4].s64 = ctx.r[11].s64 + -4784;
	// 8325ECC8: 386AAF08  addi r3, r10, -0x50f8
	ctx.r[3].s64 = ctx.r[10].s64 + -20728;
	// 8325ECCC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8325ECD0: 4AFCE201  bl 0x8222ced0
	ctx.lr = 0x8325ECD4;
	sub_8222CED0(ctx, base);
	// 8325ECD4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8325ECD8: 3869C660  addi r3, r9, -0x39a0
	ctx.r[3].s64 = ctx.r[9].s64 + -14752;
	// 8325ECDC: 4BA4B245  bl 0x82ca9f20
	ctx.lr = 0x8325ECE0;
	sub_82CA9F20(ctx, base);
	// 8325ECE0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325ECE4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325ECE8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325ECEC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325ECF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325ECF0 size=56
    let mut pc: u32 = 0x8325ECF0;
    'dispatch: loop {
        match pc {
            0x8325ECF0 => {
    //   block [0x8325ECF0..0x8325ED28)
	// 8325ECF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325ECF4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325ECF8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325ECFC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8325ED00: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8325ED04: 386B42C8  addi r3, r11, 0x42c8
	ctx.r[3].s64 = ctx.r[11].s64 + 17096;
	// 8325ED08: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8325ED0C: 4AF9504D  bl 0x821f3d58
	ctx.lr = 0x8325ED10;
	sub_821F3D58(ctx, base);
	// 8325ED10: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325ED14: 906AAF0C  stw r3, -0x50f4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-20724 as u32), ctx.r[3].u32 ) };
	// 8325ED18: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325ED1C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325ED20: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325ED24: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325ED28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325ED28 size=56
    let mut pc: u32 = 0x8325ED28;
    'dispatch: loop {
        match pc {
            0x8325ED28 => {
    //   block [0x8325ED28..0x8325ED60)
	// 8325ED28: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325ED2C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325ED30: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325ED34: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8325ED38: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8325ED3C: 386B42DC  addi r3, r11, 0x42dc
	ctx.r[3].s64 = ctx.r[11].s64 + 17116;
	// 8325ED40: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8325ED44: 4AF95015  bl 0x821f3d58
	ctx.lr = 0x8325ED48;
	sub_821F3D58(ctx, base);
	// 8325ED48: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325ED4C: 906AAF10  stw r3, -0x50f0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-20720 as u32), ctx.r[3].u32 ) };
	// 8325ED50: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325ED54: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325ED58: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325ED5C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325ED60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325ED60 size=56
    let mut pc: u32 = 0x8325ED60;
    'dispatch: loop {
        match pc {
            0x8325ED60 => {
    //   block [0x8325ED60..0x8325ED98)
	// 8325ED60: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325ED64: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325ED68: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325ED6C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8325ED70: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8325ED74: 386B42E8  addi r3, r11, 0x42e8
	ctx.r[3].s64 = ctx.r[11].s64 + 17128;
	// 8325ED78: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8325ED7C: 4AF94FDD  bl 0x821f3d58
	ctx.lr = 0x8325ED80;
	sub_821F3D58(ctx, base);
	// 8325ED80: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325ED84: 906AAF14  stw r3, -0x50ec(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-20716 as u32), ctx.r[3].u32 ) };
	// 8325ED88: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325ED8C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325ED90: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325ED94: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325ED98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325ED98 size=56
    let mut pc: u32 = 0x8325ED98;
    'dispatch: loop {
        match pc {
            0x8325ED98 => {
    //   block [0x8325ED98..0x8325EDD0)
	// 8325ED98: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325ED9C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325EDA0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325EDA4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8325EDA8: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8325EDAC: 386B4300  addi r3, r11, 0x4300
	ctx.r[3].s64 = ctx.r[11].s64 + 17152;
	// 8325EDB0: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8325EDB4: 4AF94FA5  bl 0x821f3d58
	ctx.lr = 0x8325EDB8;
	sub_821F3D58(ctx, base);
	// 8325EDB8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325EDBC: 906AAF18  stw r3, -0x50e8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-20712 as u32), ctx.r[3].u32 ) };
	// 8325EDC0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325EDC4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325EDC8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325EDCC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325EDD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325EDD0 size=56
    let mut pc: u32 = 0x8325EDD0;
    'dispatch: loop {
        match pc {
            0x8325EDD0 => {
    //   block [0x8325EDD0..0x8325EE08)
	// 8325EDD0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325EDD4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325EDD8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325EDDC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8325EDE0: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8325EDE4: 386B4310  addi r3, r11, 0x4310
	ctx.r[3].s64 = ctx.r[11].s64 + 17168;
	// 8325EDE8: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8325EDEC: 4AF94F6D  bl 0x821f3d58
	ctx.lr = 0x8325EDF0;
	sub_821F3D58(ctx, base);
	// 8325EDF0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325EDF4: 906AAF1C  stw r3, -0x50e4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-20708 as u32), ctx.r[3].u32 ) };
	// 8325EDF8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325EDFC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325EE00: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325EE04: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325EE08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325EE08 size=56
    let mut pc: u32 = 0x8325EE08;
    'dispatch: loop {
        match pc {
            0x8325EE08 => {
    //   block [0x8325EE08..0x8325EE40)
	// 8325EE08: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325EE0C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325EE10: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325EE14: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8325EE18: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8325EE1C: 386B4328  addi r3, r11, 0x4328
	ctx.r[3].s64 = ctx.r[11].s64 + 17192;
	// 8325EE20: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8325EE24: 4AF94F35  bl 0x821f3d58
	ctx.lr = 0x8325EE28;
	sub_821F3D58(ctx, base);
	// 8325EE28: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325EE2C: 906AAF20  stw r3, -0x50e0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-20704 as u32), ctx.r[3].u32 ) };
	// 8325EE30: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325EE34: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325EE38: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325EE3C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325EE40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325EE40 size=56
    let mut pc: u32 = 0x8325EE40;
    'dispatch: loop {
        match pc {
            0x8325EE40 => {
    //   block [0x8325EE40..0x8325EE78)
	// 8325EE40: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325EE44: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325EE48: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325EE4C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8325EE50: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8325EE54: 386B4338  addi r3, r11, 0x4338
	ctx.r[3].s64 = ctx.r[11].s64 + 17208;
	// 8325EE58: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8325EE5C: 4AF94EFD  bl 0x821f3d58
	ctx.lr = 0x8325EE60;
	sub_821F3D58(ctx, base);
	// 8325EE60: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325EE64: 906AAF24  stw r3, -0x50dc(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-20700 as u32), ctx.r[3].u32 ) };
	// 8325EE68: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325EE6C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325EE70: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325EE74: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325EE78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325EE78 size=56
    let mut pc: u32 = 0x8325EE78;
    'dispatch: loop {
        match pc {
            0x8325EE78 => {
    //   block [0x8325EE78..0x8325EEB0)
	// 8325EE78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325EE7C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325EE80: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325EE84: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8325EE88: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8325EE8C: 386B4350  addi r3, r11, 0x4350
	ctx.r[3].s64 = ctx.r[11].s64 + 17232;
	// 8325EE90: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8325EE94: 4AF94EC5  bl 0x821f3d58
	ctx.lr = 0x8325EE98;
	sub_821F3D58(ctx, base);
	// 8325EE98: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325EE9C: 906AAF28  stw r3, -0x50d8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-20696 as u32), ctx.r[3].u32 ) };
	// 8325EEA0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325EEA4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325EEA8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325EEAC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325EEB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325EEB0 size=56
    let mut pc: u32 = 0x8325EEB0;
    'dispatch: loop {
        match pc {
            0x8325EEB0 => {
    //   block [0x8325EEB0..0x8325EEE8)
	// 8325EEB0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325EEB4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325EEB8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325EEBC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8325EEC0: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8325EEC4: 386B4368  addi r3, r11, 0x4368
	ctx.r[3].s64 = ctx.r[11].s64 + 17256;
	// 8325EEC8: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8325EECC: 4AF94E8D  bl 0x821f3d58
	ctx.lr = 0x8325EED0;
	sub_821F3D58(ctx, base);
	// 8325EED0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325EED4: 906AAF2C  stw r3, -0x50d4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-20692 as u32), ctx.r[3].u32 ) };
	// 8325EED8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325EEDC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325EEE0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325EEE4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325EEE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325EEE8 size=56
    let mut pc: u32 = 0x8325EEE8;
    'dispatch: loop {
        match pc {
            0x8325EEE8 => {
    //   block [0x8325EEE8..0x8325EF20)
	// 8325EEE8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325EEEC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325EEF0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325EEF4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8325EEF8: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8325EEFC: 386B4380  addi r3, r11, 0x4380
	ctx.r[3].s64 = ctx.r[11].s64 + 17280;
	// 8325EF00: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8325EF04: 4AF94E55  bl 0x821f3d58
	ctx.lr = 0x8325EF08;
	sub_821F3D58(ctx, base);
	// 8325EF08: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325EF0C: 906AAF30  stw r3, -0x50d0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-20688 as u32), ctx.r[3].u32 ) };
	// 8325EF10: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325EF14: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325EF18: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325EF1C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325EF20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325EF20 size=56
    let mut pc: u32 = 0x8325EF20;
    'dispatch: loop {
        match pc {
            0x8325EF20 => {
    //   block [0x8325EF20..0x8325EF58)
	// 8325EF20: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325EF24: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325EF28: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325EF2C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8325EF30: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8325EF34: 386B4394  addi r3, r11, 0x4394
	ctx.r[3].s64 = ctx.r[11].s64 + 17300;
	// 8325EF38: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8325EF3C: 4AF94E1D  bl 0x821f3d58
	ctx.lr = 0x8325EF40;
	sub_821F3D58(ctx, base);
	// 8325EF40: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325EF44: 906AAF34  stw r3, -0x50cc(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-20684 as u32), ctx.r[3].u32 ) };
	// 8325EF48: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325EF4C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325EF50: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325EF54: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325EF58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325EF58 size=56
    let mut pc: u32 = 0x8325EF58;
    'dispatch: loop {
        match pc {
            0x8325EF58 => {
    //   block [0x8325EF58..0x8325EF90)
	// 8325EF58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325EF5C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325EF60: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325EF64: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8325EF68: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8325EF6C: 386B43B4  addi r3, r11, 0x43b4
	ctx.r[3].s64 = ctx.r[11].s64 + 17332;
	// 8325EF70: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8325EF74: 4AF94DE5  bl 0x821f3d58
	ctx.lr = 0x8325EF78;
	sub_821F3D58(ctx, base);
	// 8325EF78: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325EF7C: 906AAF38  stw r3, -0x50c8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-20680 as u32), ctx.r[3].u32 ) };
	// 8325EF80: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325EF84: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325EF88: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325EF8C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325EF90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325EF90 size=56
    let mut pc: u32 = 0x8325EF90;
    'dispatch: loop {
        match pc {
            0x8325EF90 => {
    //   block [0x8325EF90..0x8325EFC8)
	// 8325EF90: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325EF94: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325EF98: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325EF9C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8325EFA0: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8325EFA4: 386B43C8  addi r3, r11, 0x43c8
	ctx.r[3].s64 = ctx.r[11].s64 + 17352;
	// 8325EFA8: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8325EFAC: 4AF94DAD  bl 0x821f3d58
	ctx.lr = 0x8325EFB0;
	sub_821F3D58(ctx, base);
	// 8325EFB0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325EFB4: 906AAF3C  stw r3, -0x50c4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-20676 as u32), ctx.r[3].u32 ) };
	// 8325EFB8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325EFBC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325EFC0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325EFC4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325EFC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325EFC8 size=56
    let mut pc: u32 = 0x8325EFC8;
    'dispatch: loop {
        match pc {
            0x8325EFC8 => {
    //   block [0x8325EFC8..0x8325F000)
	// 8325EFC8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325EFCC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325EFD0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325EFD4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8325EFD8: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8325EFDC: 386B43D4  addi r3, r11, 0x43d4
	ctx.r[3].s64 = ctx.r[11].s64 + 17364;
	// 8325EFE0: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8325EFE4: 4AF94D75  bl 0x821f3d58
	ctx.lr = 0x8325EFE8;
	sub_821F3D58(ctx, base);
	// 8325EFE8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325EFEC: 906AAF40  stw r3, -0x50c0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-20672 as u32), ctx.r[3].u32 ) };
	// 8325EFF0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325EFF4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325EFF8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325EFFC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325F000(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325F000 size=56
    let mut pc: u32 = 0x8325F000;
    'dispatch: loop {
        match pc {
            0x8325F000 => {
    //   block [0x8325F000..0x8325F038)
	// 8325F000: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325F004: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325F008: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325F00C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8325F010: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8325F014: 386B43E0  addi r3, r11, 0x43e0
	ctx.r[3].s64 = ctx.r[11].s64 + 17376;
	// 8325F018: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8325F01C: 4AF94D3D  bl 0x821f3d58
	ctx.lr = 0x8325F020;
	sub_821F3D58(ctx, base);
	// 8325F020: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325F024: 906AAF44  stw r3, -0x50bc(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-20668 as u32), ctx.r[3].u32 ) };
	// 8325F028: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325F02C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325F030: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325F034: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325F038(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325F038 size=56
    let mut pc: u32 = 0x8325F038;
    'dispatch: loop {
        match pc {
            0x8325F038 => {
    //   block [0x8325F038..0x8325F070)
	// 8325F038: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325F03C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325F040: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325F044: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8325F048: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8325F04C: 386B43F0  addi r3, r11, 0x43f0
	ctx.r[3].s64 = ctx.r[11].s64 + 17392;
	// 8325F050: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8325F054: 4AF94D05  bl 0x821f3d58
	ctx.lr = 0x8325F058;
	sub_821F3D58(ctx, base);
	// 8325F058: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325F05C: 906AAF48  stw r3, -0x50b8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-20664 as u32), ctx.r[3].u32 ) };
	// 8325F060: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325F064: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325F068: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325F06C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325F070(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325F070 size=56
    let mut pc: u32 = 0x8325F070;
    'dispatch: loop {
        match pc {
            0x8325F070 => {
    //   block [0x8325F070..0x8325F0A8)
	// 8325F070: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325F074: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325F078: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325F07C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8325F080: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8325F084: 386B4400  addi r3, r11, 0x4400
	ctx.r[3].s64 = ctx.r[11].s64 + 17408;
	// 8325F088: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8325F08C: 4AF94CCD  bl 0x821f3d58
	ctx.lr = 0x8325F090;
	sub_821F3D58(ctx, base);
	// 8325F090: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325F094: 906AAF4C  stw r3, -0x50b4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-20660 as u32), ctx.r[3].u32 ) };
	// 8325F098: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325F09C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325F0A0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325F0A4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325F0A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325F0A8 size=56
    let mut pc: u32 = 0x8325F0A8;
    'dispatch: loop {
        match pc {
            0x8325F0A8 => {
    //   block [0x8325F0A8..0x8325F0E0)
	// 8325F0A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325F0AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325F0B0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325F0B4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8325F0B8: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8325F0BC: 386B4418  addi r3, r11, 0x4418
	ctx.r[3].s64 = ctx.r[11].s64 + 17432;
	// 8325F0C0: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8325F0C4: 4AF94C95  bl 0x821f3d58
	ctx.lr = 0x8325F0C8;
	sub_821F3D58(ctx, base);
	// 8325F0C8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325F0CC: 906AAF50  stw r3, -0x50b0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-20656 as u32), ctx.r[3].u32 ) };
	// 8325F0D0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325F0D4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325F0D8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325F0DC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325F0E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325F0E0 size=56
    let mut pc: u32 = 0x8325F0E0;
    'dispatch: loop {
        match pc {
            0x8325F0E0 => {
    //   block [0x8325F0E0..0x8325F118)
	// 8325F0E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325F0E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325F0E8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325F0EC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8325F0F0: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8325F0F4: 386B4424  addi r3, r11, 0x4424
	ctx.r[3].s64 = ctx.r[11].s64 + 17444;
	// 8325F0F8: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8325F0FC: 4AF94C5D  bl 0x821f3d58
	ctx.lr = 0x8325F100;
	sub_821F3D58(ctx, base);
	// 8325F100: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325F104: 906AAF54  stw r3, -0x50ac(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-20652 as u32), ctx.r[3].u32 ) };
	// 8325F108: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325F10C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325F110: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325F114: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325F118(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325F118 size=56
    let mut pc: u32 = 0x8325F118;
    'dispatch: loop {
        match pc {
            0x8325F118 => {
    //   block [0x8325F118..0x8325F150)
	// 8325F118: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325F11C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325F120: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325F124: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8325F128: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8325F12C: 386B4430  addi r3, r11, 0x4430
	ctx.r[3].s64 = ctx.r[11].s64 + 17456;
	// 8325F130: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8325F134: 4AF94C25  bl 0x821f3d58
	ctx.lr = 0x8325F138;
	sub_821F3D58(ctx, base);
	// 8325F138: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325F13C: 906AAF58  stw r3, -0x50a8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-20648 as u32), ctx.r[3].u32 ) };
	// 8325F140: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325F144: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325F148: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325F14C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325F150(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325F150 size=56
    let mut pc: u32 = 0x8325F150;
    'dispatch: loop {
        match pc {
            0x8325F150 => {
    //   block [0x8325F150..0x8325F188)
	// 8325F150: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325F154: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325F158: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325F15C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8325F160: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8325F164: 386B4440  addi r3, r11, 0x4440
	ctx.r[3].s64 = ctx.r[11].s64 + 17472;
	// 8325F168: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8325F16C: 4AF94BED  bl 0x821f3d58
	ctx.lr = 0x8325F170;
	sub_821F3D58(ctx, base);
	// 8325F170: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325F174: 906AAF5C  stw r3, -0x50a4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-20644 as u32), ctx.r[3].u32 ) };
	// 8325F178: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325F17C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325F180: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325F184: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325F188(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325F188 size=64
    let mut pc: u32 = 0x8325F188;
    'dispatch: loop {
        match pc {
            0x8325F188 => {
    //   block [0x8325F188..0x8325F1C8)
	// 8325F188: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325F18C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325F190: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325F194: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8325F198: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325F19C: 388BED98  addi r4, r11, -0x1268
	ctx.r[4].s64 = ctx.r[11].s64 + -4712;
	// 8325F1A0: 386AAF60  addi r3, r10, -0x50a0
	ctx.r[3].s64 = ctx.r[10].s64 + -20640;
	// 8325F1A4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8325F1A8: 4AFCDD29  bl 0x8222ced0
	ctx.lr = 0x8325F1AC;
	sub_8222CED0(ctx, base);
	// 8325F1AC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8325F1B0: 3869C670  addi r3, r9, -0x3990
	ctx.r[3].s64 = ctx.r[9].s64 + -14736;
	// 8325F1B4: 4BA4AD6D  bl 0x82ca9f20
	ctx.lr = 0x8325F1B8;
	sub_82CA9F20(ctx, base);
	// 8325F1B8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325F1BC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325F1C0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325F1C4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325F1C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325F1C8 size=64
    let mut pc: u32 = 0x8325F1C8;
    'dispatch: loop {
        match pc {
            0x8325F1C8 => {
    //   block [0x8325F1C8..0x8325F208)
	// 8325F1C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325F1CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325F1D0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325F1D4: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8325F1D8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325F1DC: 388B92D4  addi r4, r11, -0x6d2c
	ctx.r[4].s64 = ctx.r[11].s64 + -27948;
	// 8325F1E0: 386AAF64  addi r3, r10, -0x509c
	ctx.r[3].s64 = ctx.r[10].s64 + -20636;
	// 8325F1E4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8325F1E8: 4AFCDCE9  bl 0x8222ced0
	ctx.lr = 0x8325F1EC;
	sub_8222CED0(ctx, base);
	// 8325F1EC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8325F1F0: 3869C680  addi r3, r9, -0x3980
	ctx.r[3].s64 = ctx.r[9].s64 + -14720;
	// 8325F1F4: 4BA4AD2D  bl 0x82ca9f20
	ctx.lr = 0x8325F1F8;
	sub_82CA9F20(ctx, base);
	// 8325F1F8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325F1FC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325F200: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325F204: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325F208(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325F208 size=60
    let mut pc: u32 = 0x8325F208;
    'dispatch: loop {
        match pc {
            0x8325F208 => {
    //   block [0x8325F208..0x8325F244)
	// 8325F208: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325F20C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325F210: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325F214: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8325F218: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325F21C: 388BEE80  addi r4, r11, -0x1180
	ctx.r[4].s64 = ctx.r[11].s64 + -4480;
	// 8325F220: 386AAF68  addi r3, r10, -0x5098
	ctx.r[3].s64 = ctx.r[10].s64 + -20632;
	// 8325F224: 4B0771E5  bl 0x822d6408
	ctx.lr = 0x8325F228;
	sub_822D6408(ctx, base);
	// 8325F228: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8325F22C: 3869C690  addi r3, r9, -0x3970
	ctx.r[3].s64 = ctx.r[9].s64 + -14704;
	// 8325F230: 4BA4ACF1  bl 0x82ca9f20
	ctx.lr = 0x8325F234;
	sub_82CA9F20(ctx, base);
	// 8325F234: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325F238: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325F23C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325F240: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325F248(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325F248 size=64
    let mut pc: u32 = 0x8325F248;
    'dispatch: loop {
        match pc {
            0x8325F248 => {
    //   block [0x8325F248..0x8325F288)
	// 8325F248: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325F24C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325F250: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325F254: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8325F258: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325F25C: 388BEE84  addi r4, r11, -0x117c
	ctx.r[4].s64 = ctx.r[11].s64 + -4476;
	// 8325F260: 386AAF6C  addi r3, r10, -0x5094
	ctx.r[3].s64 = ctx.r[10].s64 + -20628;
	// 8325F264: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8325F268: 4AFCDC69  bl 0x8222ced0
	ctx.lr = 0x8325F26C;
	sub_8222CED0(ctx, base);
	// 8325F26C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8325F270: 3869C6A0  addi r3, r9, -0x3960
	ctx.r[3].s64 = ctx.r[9].s64 + -14688;
	// 8325F274: 4BA4ACAD  bl 0x82ca9f20
	ctx.lr = 0x8325F278;
	sub_82CA9F20(ctx, base);
	// 8325F278: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325F27C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325F280: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325F284: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325F288(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325F288 size=64
    let mut pc: u32 = 0x8325F288;
    'dispatch: loop {
        match pc {
            0x8325F288 => {
    //   block [0x8325F288..0x8325F2C8)
	// 8325F288: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325F28C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325F290: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325F294: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8325F298: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325F29C: 388BEEAC  addi r4, r11, -0x1154
	ctx.r[4].s64 = ctx.r[11].s64 + -4436;
	// 8325F2A0: 386AAF70  addi r3, r10, -0x5090
	ctx.r[3].s64 = ctx.r[10].s64 + -20624;
	// 8325F2A4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8325F2A8: 4AFCDC29  bl 0x8222ced0
	ctx.lr = 0x8325F2AC;
	sub_8222CED0(ctx, base);
	// 8325F2AC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8325F2B0: 3869C6B0  addi r3, r9, -0x3950
	ctx.r[3].s64 = ctx.r[9].s64 + -14672;
	// 8325F2B4: 4BA4AC6D  bl 0x82ca9f20
	ctx.lr = 0x8325F2B8;
	sub_82CA9F20(ctx, base);
	// 8325F2B8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325F2BC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325F2C0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325F2C4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325F2C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325F2C8 size=64
    let mut pc: u32 = 0x8325F2C8;
    'dispatch: loop {
        match pc {
            0x8325F2C8 => {
    //   block [0x8325F2C8..0x8325F308)
	// 8325F2C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325F2CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325F2D0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325F2D4: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8325F2D8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325F2DC: 388BEED4  addi r4, r11, -0x112c
	ctx.r[4].s64 = ctx.r[11].s64 + -4396;
	// 8325F2E0: 386AAF74  addi r3, r10, -0x508c
	ctx.r[3].s64 = ctx.r[10].s64 + -20620;
	// 8325F2E4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8325F2E8: 4AFCDBE9  bl 0x8222ced0
	ctx.lr = 0x8325F2EC;
	sub_8222CED0(ctx, base);
	// 8325F2EC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8325F2F0: 3869C6C0  addi r3, r9, -0x3940
	ctx.r[3].s64 = ctx.r[9].s64 + -14656;
	// 8325F2F4: 4BA4AC2D  bl 0x82ca9f20
	ctx.lr = 0x8325F2F8;
	sub_82CA9F20(ctx, base);
	// 8325F2F8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325F2FC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325F300: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325F304: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325F308(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325F308 size=64
    let mut pc: u32 = 0x8325F308;
    'dispatch: loop {
        match pc {
            0x8325F308 => {
    //   block [0x8325F308..0x8325F348)
	// 8325F308: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325F30C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325F310: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325F314: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8325F318: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325F31C: 388BEEF8  addi r4, r11, -0x1108
	ctx.r[4].s64 = ctx.r[11].s64 + -4360;
	// 8325F320: 386AAF78  addi r3, r10, -0x5088
	ctx.r[3].s64 = ctx.r[10].s64 + -20616;
	// 8325F324: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8325F328: 4AFCDBA9  bl 0x8222ced0
	ctx.lr = 0x8325F32C;
	sub_8222CED0(ctx, base);
	// 8325F32C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8325F330: 3869C6D0  addi r3, r9, -0x3930
	ctx.r[3].s64 = ctx.r[9].s64 + -14640;
	// 8325F334: 4BA4ABED  bl 0x82ca9f20
	ctx.lr = 0x8325F338;
	sub_82CA9F20(ctx, base);
	// 8325F338: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325F33C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325F340: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325F344: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325F348(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325F348 size=64
    let mut pc: u32 = 0x8325F348;
    'dispatch: loop {
        match pc {
            0x8325F348 => {
    //   block [0x8325F348..0x8325F388)
	// 8325F348: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325F34C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325F350: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325F354: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8325F358: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325F35C: 388BEF04  addi r4, r11, -0x10fc
	ctx.r[4].s64 = ctx.r[11].s64 + -4348;
	// 8325F360: 386AAF7C  addi r3, r10, -0x5084
	ctx.r[3].s64 = ctx.r[10].s64 + -20612;
	// 8325F364: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8325F368: 4AFCDB69  bl 0x8222ced0
	ctx.lr = 0x8325F36C;
	sub_8222CED0(ctx, base);
	// 8325F36C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8325F370: 3869C6E0  addi r3, r9, -0x3920
	ctx.r[3].s64 = ctx.r[9].s64 + -14624;
	// 8325F374: 4BA4ABAD  bl 0x82ca9f20
	ctx.lr = 0x8325F378;
	sub_82CA9F20(ctx, base);
	// 8325F378: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325F37C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325F380: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325F384: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325F388(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325F388 size=64
    let mut pc: u32 = 0x8325F388;
    'dispatch: loop {
        match pc {
            0x8325F388 => {
    //   block [0x8325F388..0x8325F3C8)
	// 8325F388: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325F38C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325F390: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325F394: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8325F398: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325F39C: 388BEF14  addi r4, r11, -0x10ec
	ctx.r[4].s64 = ctx.r[11].s64 + -4332;
	// 8325F3A0: 386AAF80  addi r3, r10, -0x5080
	ctx.r[3].s64 = ctx.r[10].s64 + -20608;
	// 8325F3A4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8325F3A8: 4AFCDB29  bl 0x8222ced0
	ctx.lr = 0x8325F3AC;
	sub_8222CED0(ctx, base);
	// 8325F3AC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8325F3B0: 3869C6F0  addi r3, r9, -0x3910
	ctx.r[3].s64 = ctx.r[9].s64 + -14608;
	// 8325F3B4: 4BA4AB6D  bl 0x82ca9f20
	ctx.lr = 0x8325F3B8;
	sub_82CA9F20(ctx, base);
	// 8325F3B8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325F3BC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325F3C0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325F3C4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325F3C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325F3C8 size=64
    let mut pc: u32 = 0x8325F3C8;
    'dispatch: loop {
        match pc {
            0x8325F3C8 => {
    //   block [0x8325F3C8..0x8325F408)
	// 8325F3C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325F3CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325F3D0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325F3D4: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 8325F3D8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325F3DC: 388BEFA4  addi r4, r11, -0x105c
	ctx.r[4].s64 = ctx.r[11].s64 + -4188;
	// 8325F3E0: 386AAF84  addi r3, r10, -0x507c
	ctx.r[3].s64 = ctx.r[10].s64 + -20604;
	// 8325F3E4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8325F3E8: 4AFCDAE9  bl 0x8222ced0
	ctx.lr = 0x8325F3EC;
	sub_8222CED0(ctx, base);
	// 8325F3EC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8325F3F0: 3869C700  addi r3, r9, -0x3900
	ctx.r[3].s64 = ctx.r[9].s64 + -14592;
	// 8325F3F4: 4BA4AB2D  bl 0x82ca9f20
	ctx.lr = 0x8325F3F8;
	sub_82CA9F20(ctx, base);
	// 8325F3F8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325F3FC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325F400: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325F404: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325F408(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325F408 size=64
    let mut pc: u32 = 0x8325F408;
    'dispatch: loop {
        match pc {
            0x8325F408 => {
    //   block [0x8325F408..0x8325F448)
	// 8325F408: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325F40C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325F410: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325F414: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 8325F418: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325F41C: 388B1104  addi r4, r11, 0x1104
	ctx.r[4].s64 = ctx.r[11].s64 + 4356;
	// 8325F420: 386AAF88  addi r3, r10, -0x5078
	ctx.r[3].s64 = ctx.r[10].s64 + -20600;
	// 8325F424: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8325F428: 4AFCDAA9  bl 0x8222ced0
	ctx.lr = 0x8325F42C;
	sub_8222CED0(ctx, base);
	// 8325F42C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8325F430: 3869C710  addi r3, r9, -0x38f0
	ctx.r[3].s64 = ctx.r[9].s64 + -14576;
	// 8325F434: 4BA4AAED  bl 0x82ca9f20
	ctx.lr = 0x8325F438;
	sub_82CA9F20(ctx, base);
	// 8325F438: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325F43C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325F440: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325F444: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325F448(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325F448 size=64
    let mut pc: u32 = 0x8325F448;
    'dispatch: loop {
        match pc {
            0x8325F448 => {
    //   block [0x8325F448..0x8325F488)
	// 8325F448: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325F44C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325F450: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325F454: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8325F458: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325F45C: 388B052C  addi r4, r11, 0x52c
	ctx.r[4].s64 = ctx.r[11].s64 + 1324;
	// 8325F460: 386AAF8C  addi r3, r10, -0x5074
	ctx.r[3].s64 = ctx.r[10].s64 + -20596;
	// 8325F464: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8325F468: 4AFCDA69  bl 0x8222ced0
	ctx.lr = 0x8325F46C;
	sub_8222CED0(ctx, base);
	// 8325F46C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8325F470: 3869C720  addi r3, r9, -0x38e0
	ctx.r[3].s64 = ctx.r[9].s64 + -14560;
	// 8325F474: 4BA4AAAD  bl 0x82ca9f20
	ctx.lr = 0x8325F478;
	sub_82CA9F20(ctx, base);
	// 8325F478: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325F47C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325F480: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325F484: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325F488(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x8325F488 size=96
    let mut pc: u32 = 0x8325F488;
    'dispatch: loop {
        match pc {
            0x8325F488 => {
    //   block [0x8325F488..0x8325F4E8)
	// 8325F488: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8325F48C: 3921FFF0  addi r9, r1, -0x10
	ctx.r[9].s64 = ctx.r[1].s64 + -16;
	// 8325F490: 3D408209  lis r10, -0x7df7
	ctx.r[10].s64 = -2113339392;
	// 8325F494: 3901FFF4  addi r8, r1, -0xc
	ctx.r[8].s64 = ctx.r[1].s64 + -12;
	// 8325F498: 38E1FFF4  addi r7, r1, -0xc
	ctx.r[7].s64 = ctx.r[1].s64 + -12;
	// 8325F49C: C00B9484  lfs f0, -0x6b7c(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-27516 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8325F4A0: 38C1FFF0  addi r6, r1, -0x10
	ctx.r[6].s64 = ctx.r[1].s64 + -16;
	// 8325F4A4: D001FFF0  stfs f0, -0x10(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), tmp.u32 ) };
	// 8325F4A8: 3CA0834A  lis r5, -0x7cb6
	ctx.r[5].s64 = -2092302336;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325F4E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325F4E8 size=56
    let mut pc: u32 = 0x8325F4E8;
    'dispatch: loop {
        match pc {
            0x8325F4E8 => {
    //   block [0x8325F4E8..0x8325F520)
	// 8325F4E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325F4EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325F4F0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325F4F4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8325F4F8: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8325F4FC: 386B42C8  addi r3, r11, 0x42c8
	ctx.r[3].s64 = ctx.r[11].s64 + 17096;
	// 8325F500: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8325F504: 4AF94855  bl 0x821f3d58
	ctx.lr = 0x8325F508;
	sub_821F3D58(ctx, base);
	// 8325F508: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325F50C: 906AAFA0  stw r3, -0x5060(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-20576 as u32), ctx.r[3].u32 ) };
	// 8325F510: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325F514: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325F518: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325F51C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325F520(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325F520 size=56
    let mut pc: u32 = 0x8325F520;
    'dispatch: loop {
        match pc {
            0x8325F520 => {
    //   block [0x8325F520..0x8325F558)
	// 8325F520: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325F524: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325F528: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325F52C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8325F530: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8325F534: 386B42DC  addi r3, r11, 0x42dc
	ctx.r[3].s64 = ctx.r[11].s64 + 17116;
	// 8325F538: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8325F53C: 4AF9481D  bl 0x821f3d58
	ctx.lr = 0x8325F540;
	sub_821F3D58(ctx, base);
	// 8325F540: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325F544: 906AAFA4  stw r3, -0x505c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-20572 as u32), ctx.r[3].u32 ) };
	// 8325F548: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325F54C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325F550: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325F554: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325F558(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325F558 size=56
    let mut pc: u32 = 0x8325F558;
    'dispatch: loop {
        match pc {
            0x8325F558 => {
    //   block [0x8325F558..0x8325F590)
	// 8325F558: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325F55C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325F560: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325F564: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8325F568: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8325F56C: 386B42E8  addi r3, r11, 0x42e8
	ctx.r[3].s64 = ctx.r[11].s64 + 17128;
	// 8325F570: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8325F574: 4AF947E5  bl 0x821f3d58
	ctx.lr = 0x8325F578;
	sub_821F3D58(ctx, base);
	// 8325F578: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325F57C: 906AAFA8  stw r3, -0x5058(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-20568 as u32), ctx.r[3].u32 ) };
	// 8325F580: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325F584: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325F588: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325F58C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325F590(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325F590 size=56
    let mut pc: u32 = 0x8325F590;
    'dispatch: loop {
        match pc {
            0x8325F590 => {
    //   block [0x8325F590..0x8325F5C8)
	// 8325F590: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325F594: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325F598: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325F59C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8325F5A0: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8325F5A4: 386B4300  addi r3, r11, 0x4300
	ctx.r[3].s64 = ctx.r[11].s64 + 17152;
	// 8325F5A8: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8325F5AC: 4AF947AD  bl 0x821f3d58
	ctx.lr = 0x8325F5B0;
	sub_821F3D58(ctx, base);
	// 8325F5B0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325F5B4: 906AAFAC  stw r3, -0x5054(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-20564 as u32), ctx.r[3].u32 ) };
	// 8325F5B8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325F5BC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325F5C0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325F5C4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325F5C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325F5C8 size=56
    let mut pc: u32 = 0x8325F5C8;
    'dispatch: loop {
        match pc {
            0x8325F5C8 => {
    //   block [0x8325F5C8..0x8325F600)
	// 8325F5C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325F5CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325F5D0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325F5D4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8325F5D8: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8325F5DC: 386B4310  addi r3, r11, 0x4310
	ctx.r[3].s64 = ctx.r[11].s64 + 17168;
	// 8325F5E0: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8325F5E4: 4AF94775  bl 0x821f3d58
	ctx.lr = 0x8325F5E8;
	sub_821F3D58(ctx, base);
	// 8325F5E8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325F5EC: 906AAFB0  stw r3, -0x5050(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-20560 as u32), ctx.r[3].u32 ) };
	// 8325F5F0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325F5F4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325F5F8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325F5FC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325F600(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325F600 size=56
    let mut pc: u32 = 0x8325F600;
    'dispatch: loop {
        match pc {
            0x8325F600 => {
    //   block [0x8325F600..0x8325F638)
	// 8325F600: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325F604: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325F608: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325F60C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8325F610: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8325F614: 386B4328  addi r3, r11, 0x4328
	ctx.r[3].s64 = ctx.r[11].s64 + 17192;
	// 8325F618: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8325F61C: 4AF9473D  bl 0x821f3d58
	ctx.lr = 0x8325F620;
	sub_821F3D58(ctx, base);
	// 8325F620: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325F624: 906AAFB4  stw r3, -0x504c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-20556 as u32), ctx.r[3].u32 ) };
	// 8325F628: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325F62C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325F630: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325F634: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325F638(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325F638 size=56
    let mut pc: u32 = 0x8325F638;
    'dispatch: loop {
        match pc {
            0x8325F638 => {
    //   block [0x8325F638..0x8325F670)
	// 8325F638: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325F63C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325F640: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325F644: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8325F648: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8325F64C: 386B4338  addi r3, r11, 0x4338
	ctx.r[3].s64 = ctx.r[11].s64 + 17208;
	// 8325F650: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8325F654: 4AF94705  bl 0x821f3d58
	ctx.lr = 0x8325F658;
	sub_821F3D58(ctx, base);
	// 8325F658: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325F65C: 906AAFB8  stw r3, -0x5048(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-20552 as u32), ctx.r[3].u32 ) };
	// 8325F660: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325F664: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325F668: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325F66C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325F670(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325F670 size=56
    let mut pc: u32 = 0x8325F670;
    'dispatch: loop {
        match pc {
            0x8325F670 => {
    //   block [0x8325F670..0x8325F6A8)
	// 8325F670: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325F674: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325F678: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325F67C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8325F680: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8325F684: 386B4350  addi r3, r11, 0x4350
	ctx.r[3].s64 = ctx.r[11].s64 + 17232;
	// 8325F688: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8325F68C: 4AF946CD  bl 0x821f3d58
	ctx.lr = 0x8325F690;
	sub_821F3D58(ctx, base);
	// 8325F690: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325F694: 906AAFBC  stw r3, -0x5044(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-20548 as u32), ctx.r[3].u32 ) };
	// 8325F698: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325F69C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325F6A0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325F6A4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325F6A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325F6A8 size=56
    let mut pc: u32 = 0x8325F6A8;
    'dispatch: loop {
        match pc {
            0x8325F6A8 => {
    //   block [0x8325F6A8..0x8325F6E0)
	// 8325F6A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325F6AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325F6B0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325F6B4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8325F6B8: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8325F6BC: 386B4368  addi r3, r11, 0x4368
	ctx.r[3].s64 = ctx.r[11].s64 + 17256;
	// 8325F6C0: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8325F6C4: 4AF94695  bl 0x821f3d58
	ctx.lr = 0x8325F6C8;
	sub_821F3D58(ctx, base);
	// 8325F6C8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325F6CC: 906AAFC0  stw r3, -0x5040(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-20544 as u32), ctx.r[3].u32 ) };
	// 8325F6D0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325F6D4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325F6D8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325F6DC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325F6E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325F6E0 size=56
    let mut pc: u32 = 0x8325F6E0;
    'dispatch: loop {
        match pc {
            0x8325F6E0 => {
    //   block [0x8325F6E0..0x8325F718)
	// 8325F6E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325F6E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325F6E8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325F6EC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8325F6F0: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8325F6F4: 386B4380  addi r3, r11, 0x4380
	ctx.r[3].s64 = ctx.r[11].s64 + 17280;
	// 8325F6F8: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8325F6FC: 4AF9465D  bl 0x821f3d58
	ctx.lr = 0x8325F700;
	sub_821F3D58(ctx, base);
	// 8325F700: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325F704: 906AAFC4  stw r3, -0x503c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-20540 as u32), ctx.r[3].u32 ) };
	// 8325F708: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325F70C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325F710: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325F714: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325F718(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325F718 size=56
    let mut pc: u32 = 0x8325F718;
    'dispatch: loop {
        match pc {
            0x8325F718 => {
    //   block [0x8325F718..0x8325F750)
	// 8325F718: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325F71C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325F720: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325F724: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8325F728: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8325F72C: 386B4394  addi r3, r11, 0x4394
	ctx.r[3].s64 = ctx.r[11].s64 + 17300;
	// 8325F730: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8325F734: 4AF94625  bl 0x821f3d58
	ctx.lr = 0x8325F738;
	sub_821F3D58(ctx, base);
	// 8325F738: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325F73C: 906AAFC8  stw r3, -0x5038(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-20536 as u32), ctx.r[3].u32 ) };
	// 8325F740: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325F744: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325F748: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325F74C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325F750(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325F750 size=56
    let mut pc: u32 = 0x8325F750;
    'dispatch: loop {
        match pc {
            0x8325F750 => {
    //   block [0x8325F750..0x8325F788)
	// 8325F750: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325F754: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325F758: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325F75C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8325F760: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8325F764: 386B43B4  addi r3, r11, 0x43b4
	ctx.r[3].s64 = ctx.r[11].s64 + 17332;
	// 8325F768: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8325F76C: 4AF945ED  bl 0x821f3d58
	ctx.lr = 0x8325F770;
	sub_821F3D58(ctx, base);
	// 8325F770: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325F774: 906AAFCC  stw r3, -0x5034(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-20532 as u32), ctx.r[3].u32 ) };
	// 8325F778: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325F77C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325F780: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325F784: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325F788(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325F788 size=56
    let mut pc: u32 = 0x8325F788;
    'dispatch: loop {
        match pc {
            0x8325F788 => {
    //   block [0x8325F788..0x8325F7C0)
	// 8325F788: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325F78C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325F790: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325F794: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8325F798: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8325F79C: 386B43C8  addi r3, r11, 0x43c8
	ctx.r[3].s64 = ctx.r[11].s64 + 17352;
	// 8325F7A0: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8325F7A4: 4AF945B5  bl 0x821f3d58
	ctx.lr = 0x8325F7A8;
	sub_821F3D58(ctx, base);
	// 8325F7A8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325F7AC: 906AAFD0  stw r3, -0x5030(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-20528 as u32), ctx.r[3].u32 ) };
	// 8325F7B0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325F7B4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325F7B8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325F7BC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325F7C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325F7C0 size=56
    let mut pc: u32 = 0x8325F7C0;
    'dispatch: loop {
        match pc {
            0x8325F7C0 => {
    //   block [0x8325F7C0..0x8325F7F8)
	// 8325F7C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325F7C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325F7C8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325F7CC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8325F7D0: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8325F7D4: 386B43D4  addi r3, r11, 0x43d4
	ctx.r[3].s64 = ctx.r[11].s64 + 17364;
	// 8325F7D8: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8325F7DC: 4AF9457D  bl 0x821f3d58
	ctx.lr = 0x8325F7E0;
	sub_821F3D58(ctx, base);
	// 8325F7E0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325F7E4: 906AAFD4  stw r3, -0x502c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-20524 as u32), ctx.r[3].u32 ) };
	// 8325F7E8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325F7EC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325F7F0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325F7F4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325F7F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325F7F8 size=56
    let mut pc: u32 = 0x8325F7F8;
    'dispatch: loop {
        match pc {
            0x8325F7F8 => {
    //   block [0x8325F7F8..0x8325F830)
	// 8325F7F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325F7FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325F800: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325F804: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8325F808: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8325F80C: 386B43E0  addi r3, r11, 0x43e0
	ctx.r[3].s64 = ctx.r[11].s64 + 17376;
	// 8325F810: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8325F814: 4AF94545  bl 0x821f3d58
	ctx.lr = 0x8325F818;
	sub_821F3D58(ctx, base);
	// 8325F818: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325F81C: 906AAFD8  stw r3, -0x5028(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-20520 as u32), ctx.r[3].u32 ) };
	// 8325F820: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325F824: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325F828: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325F82C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325F830(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325F830 size=56
    let mut pc: u32 = 0x8325F830;
    'dispatch: loop {
        match pc {
            0x8325F830 => {
    //   block [0x8325F830..0x8325F868)
	// 8325F830: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325F834: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325F838: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325F83C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8325F840: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8325F844: 386B43F0  addi r3, r11, 0x43f0
	ctx.r[3].s64 = ctx.r[11].s64 + 17392;
	// 8325F848: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8325F84C: 4AF9450D  bl 0x821f3d58
	ctx.lr = 0x8325F850;
	sub_821F3D58(ctx, base);
	// 8325F850: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325F854: 906AAFDC  stw r3, -0x5024(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-20516 as u32), ctx.r[3].u32 ) };
	// 8325F858: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325F85C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325F860: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325F864: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325F868(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325F868 size=56
    let mut pc: u32 = 0x8325F868;
    'dispatch: loop {
        match pc {
            0x8325F868 => {
    //   block [0x8325F868..0x8325F8A0)
	// 8325F868: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325F86C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325F870: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325F874: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8325F878: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8325F87C: 386B4400  addi r3, r11, 0x4400
	ctx.r[3].s64 = ctx.r[11].s64 + 17408;
	// 8325F880: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8325F884: 4AF944D5  bl 0x821f3d58
	ctx.lr = 0x8325F888;
	sub_821F3D58(ctx, base);
	// 8325F888: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325F88C: 906AAFE0  stw r3, -0x5020(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-20512 as u32), ctx.r[3].u32 ) };
	// 8325F890: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325F894: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325F898: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325F89C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325F8A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325F8A0 size=56
    let mut pc: u32 = 0x8325F8A0;
    'dispatch: loop {
        match pc {
            0x8325F8A0 => {
    //   block [0x8325F8A0..0x8325F8D8)
	// 8325F8A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325F8A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325F8A8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325F8AC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8325F8B0: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8325F8B4: 386B4418  addi r3, r11, 0x4418
	ctx.r[3].s64 = ctx.r[11].s64 + 17432;
	// 8325F8B8: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8325F8BC: 4AF9449D  bl 0x821f3d58
	ctx.lr = 0x8325F8C0;
	sub_821F3D58(ctx, base);
	// 8325F8C0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325F8C4: 906AAFE4  stw r3, -0x501c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-20508 as u32), ctx.r[3].u32 ) };
	// 8325F8C8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325F8CC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325F8D0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325F8D4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325F8D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325F8D8 size=56
    let mut pc: u32 = 0x8325F8D8;
    'dispatch: loop {
        match pc {
            0x8325F8D8 => {
    //   block [0x8325F8D8..0x8325F910)
	// 8325F8D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325F8DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325F8E0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325F8E4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8325F8E8: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8325F8EC: 386B4424  addi r3, r11, 0x4424
	ctx.r[3].s64 = ctx.r[11].s64 + 17444;
	// 8325F8F0: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8325F8F4: 4AF94465  bl 0x821f3d58
	ctx.lr = 0x8325F8F8;
	sub_821F3D58(ctx, base);
	// 8325F8F8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325F8FC: 906AAFE8  stw r3, -0x5018(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-20504 as u32), ctx.r[3].u32 ) };
	// 8325F900: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325F904: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325F908: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325F90C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325F910(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325F910 size=56
    let mut pc: u32 = 0x8325F910;
    'dispatch: loop {
        match pc {
            0x8325F910 => {
    //   block [0x8325F910..0x8325F948)
	// 8325F910: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325F914: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325F918: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325F91C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8325F920: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8325F924: 386B4430  addi r3, r11, 0x4430
	ctx.r[3].s64 = ctx.r[11].s64 + 17456;
	// 8325F928: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8325F92C: 4AF9442D  bl 0x821f3d58
	ctx.lr = 0x8325F930;
	sub_821F3D58(ctx, base);
	// 8325F930: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325F934: 906AAFEC  stw r3, -0x5014(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-20500 as u32), ctx.r[3].u32 ) };
	// 8325F938: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325F93C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325F940: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325F944: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325F948(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325F948 size=56
    let mut pc: u32 = 0x8325F948;
    'dispatch: loop {
        match pc {
            0x8325F948 => {
    //   block [0x8325F948..0x8325F980)
	// 8325F948: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325F94C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325F950: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325F954: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8325F958: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8325F95C: 386B4440  addi r3, r11, 0x4440
	ctx.r[3].s64 = ctx.r[11].s64 + 17472;
	// 8325F960: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8325F964: 4AF943F5  bl 0x821f3d58
	ctx.lr = 0x8325F968;
	sub_821F3D58(ctx, base);
	// 8325F968: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325F96C: 906AAFF0  stw r3, -0x5010(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-20496 as u32), ctx.r[3].u32 ) };
	// 8325F970: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325F974: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325F978: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325F97C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325F980(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325F980 size=64
    let mut pc: u32 = 0x8325F980;
    'dispatch: loop {
        match pc {
            0x8325F980 => {
    //   block [0x8325F980..0x8325F9C0)
	// 8325F980: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325F984: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325F988: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325F98C: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8325F990: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325F994: 388BF1D0  addi r4, r11, -0xe30
	ctx.r[4].s64 = ctx.r[11].s64 + -3632;
	// 8325F998: 386AAFF4  addi r3, r10, -0x500c
	ctx.r[3].s64 = ctx.r[10].s64 + -20492;
	// 8325F99C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8325F9A0: 4AFCD531  bl 0x8222ced0
	ctx.lr = 0x8325F9A4;
	sub_8222CED0(ctx, base);
	// 8325F9A4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8325F9A8: 3869C730  addi r3, r9, -0x38d0
	ctx.r[3].s64 = ctx.r[9].s64 + -14544;
	// 8325F9AC: 4BA4A575  bl 0x82ca9f20
	ctx.lr = 0x8325F9B0;
	sub_82CA9F20(ctx, base);
	// 8325F9B0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325F9B4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325F9B8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325F9BC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325F9C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325F9C0 size=64
    let mut pc: u32 = 0x8325F9C0;
    'dispatch: loop {
        match pc {
            0x8325F9C0 => {
    //   block [0x8325F9C0..0x8325FA00)
	// 8325F9C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325F9C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325F9C8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325F9CC: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8325F9D0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325F9D4: 388BCE9C  addi r4, r11, -0x3164
	ctx.r[4].s64 = ctx.r[11].s64 + -12644;
	// 8325F9D8: 386AAFF8  addi r3, r10, -0x5008
	ctx.r[3].s64 = ctx.r[10].s64 + -20488;
	// 8325F9DC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8325F9E0: 4AFCD4F1  bl 0x8222ced0
	ctx.lr = 0x8325F9E4;
	sub_8222CED0(ctx, base);
	// 8325F9E4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8325F9E8: 3869C740  addi r3, r9, -0x38c0
	ctx.r[3].s64 = ctx.r[9].s64 + -14528;
	// 8325F9EC: 4BA4A535  bl 0x82ca9f20
	ctx.lr = 0x8325F9F0;
	sub_82CA9F20(ctx, base);
	// 8325F9F0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325F9F4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325F9F8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325F9FC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325FA00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325FA00 size=64
    let mut pc: u32 = 0x8325FA00;
    'dispatch: loop {
        match pc {
            0x8325FA00 => {
    //   block [0x8325FA00..0x8325FA40)
	// 8325FA00: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325FA04: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325FA08: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325FA0C: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8325FA10: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325FA14: 388BF1EC  addi r4, r11, -0xe14
	ctx.r[4].s64 = ctx.r[11].s64 + -3604;
	// 8325FA18: 386AAFFC  addi r3, r10, -0x5004
	ctx.r[3].s64 = ctx.r[10].s64 + -20484;
	// 8325FA1C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8325FA20: 4AFCD4B1  bl 0x8222ced0
	ctx.lr = 0x8325FA24;
	sub_8222CED0(ctx, base);
	// 8325FA24: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8325FA28: 3869C750  addi r3, r9, -0x38b0
	ctx.r[3].s64 = ctx.r[9].s64 + -14512;
	// 8325FA2C: 4BA4A4F5  bl 0x82ca9f20
	ctx.lr = 0x8325FA30;
	sub_82CA9F20(ctx, base);
	// 8325FA30: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325FA34: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325FA38: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325FA3C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325FA40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325FA40 size=64
    let mut pc: u32 = 0x8325FA40;
    'dispatch: loop {
        match pc {
            0x8325FA40 => {
    //   block [0x8325FA40..0x8325FA80)
	// 8325FA40: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325FA44: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325FA48: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325FA4C: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8325FA50: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325FA54: 388BF228  addi r4, r11, -0xdd8
	ctx.r[4].s64 = ctx.r[11].s64 + -3544;
	// 8325FA58: 386AB000  addi r3, r10, -0x5000
	ctx.r[3].s64 = ctx.r[10].s64 + -20480;
	// 8325FA5C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8325FA60: 4AFCD471  bl 0x8222ced0
	ctx.lr = 0x8325FA64;
	sub_8222CED0(ctx, base);
	// 8325FA64: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8325FA68: 3869C760  addi r3, r9, -0x38a0
	ctx.r[3].s64 = ctx.r[9].s64 + -14496;
	// 8325FA6C: 4BA4A4B5  bl 0x82ca9f20
	ctx.lr = 0x8325FA70;
	sub_82CA9F20(ctx, base);
	// 8325FA70: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325FA74: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325FA78: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325FA7C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325FA80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325FA80 size=64
    let mut pc: u32 = 0x8325FA80;
    'dispatch: loop {
        match pc {
            0x8325FA80 => {
    //   block [0x8325FA80..0x8325FAC0)
	// 8325FA80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325FA84: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325FA88: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325FA8C: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8325FA90: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325FA94: 388BF24C  addi r4, r11, -0xdb4
	ctx.r[4].s64 = ctx.r[11].s64 + -3508;
	// 8325FA98: 386AB004  addi r3, r10, -0x4ffc
	ctx.r[3].s64 = ctx.r[10].s64 + -20476;
	// 8325FA9C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8325FAA0: 4AFCD431  bl 0x8222ced0
	ctx.lr = 0x8325FAA4;
	sub_8222CED0(ctx, base);
	// 8325FAA4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8325FAA8: 3869C770  addi r3, r9, -0x3890
	ctx.r[3].s64 = ctx.r[9].s64 + -14480;
	// 8325FAAC: 4BA4A475  bl 0x82ca9f20
	ctx.lr = 0x8325FAB0;
	sub_82CA9F20(ctx, base);
	// 8325FAB0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325FAB4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325FAB8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325FABC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325FAC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325FAC0 size=64
    let mut pc: u32 = 0x8325FAC0;
    'dispatch: loop {
        match pc {
            0x8325FAC0 => {
    //   block [0x8325FAC0..0x8325FB00)
	// 8325FAC0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325FAC4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325FAC8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325FACC: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8325FAD0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325FAD4: 388BF270  addi r4, r11, -0xd90
	ctx.r[4].s64 = ctx.r[11].s64 + -3472;
	// 8325FAD8: 386AB008  addi r3, r10, -0x4ff8
	ctx.r[3].s64 = ctx.r[10].s64 + -20472;
	// 8325FADC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8325FAE0: 4AFCD3F1  bl 0x8222ced0
	ctx.lr = 0x8325FAE4;
	sub_8222CED0(ctx, base);
	// 8325FAE4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8325FAE8: 3869C780  addi r3, r9, -0x3880
	ctx.r[3].s64 = ctx.r[9].s64 + -14464;
	// 8325FAEC: 4BA4A435  bl 0x82ca9f20
	ctx.lr = 0x8325FAF0;
	sub_82CA9F20(ctx, base);
	// 8325FAF0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325FAF4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325FAF8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325FAFC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325FB00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325FB00 size=64
    let mut pc: u32 = 0x8325FB00;
    'dispatch: loop {
        match pc {
            0x8325FB00 => {
    //   block [0x8325FB00..0x8325FB40)
	// 8325FB00: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325FB04: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325FB08: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325FB0C: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8325FB10: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325FB14: 388BF290  addi r4, r11, -0xd70
	ctx.r[4].s64 = ctx.r[11].s64 + -3440;
	// 8325FB18: 386AB00C  addi r3, r10, -0x4ff4
	ctx.r[3].s64 = ctx.r[10].s64 + -20468;
	// 8325FB1C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8325FB20: 4AFCD3B1  bl 0x8222ced0
	ctx.lr = 0x8325FB24;
	sub_8222CED0(ctx, base);
	// 8325FB24: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8325FB28: 3869C790  addi r3, r9, -0x3870
	ctx.r[3].s64 = ctx.r[9].s64 + -14448;
	// 8325FB2C: 4BA4A3F5  bl 0x82ca9f20
	ctx.lr = 0x8325FB30;
	sub_82CA9F20(ctx, base);
	// 8325FB30: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325FB34: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325FB38: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325FB3C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325FB40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325FB40 size=64
    let mut pc: u32 = 0x8325FB40;
    'dispatch: loop {
        match pc {
            0x8325FB40 => {
    //   block [0x8325FB40..0x8325FB80)
	// 8325FB40: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325FB44: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325FB48: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325FB4C: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8325FB50: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325FB54: 388BF2B0  addi r4, r11, -0xd50
	ctx.r[4].s64 = ctx.r[11].s64 + -3408;
	// 8325FB58: 386AB010  addi r3, r10, -0x4ff0
	ctx.r[3].s64 = ctx.r[10].s64 + -20464;
	// 8325FB5C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8325FB60: 4AFCD371  bl 0x8222ced0
	ctx.lr = 0x8325FB64;
	sub_8222CED0(ctx, base);
	// 8325FB64: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8325FB68: 3869C7A0  addi r3, r9, -0x3860
	ctx.r[3].s64 = ctx.r[9].s64 + -14432;
	// 8325FB6C: 4BA4A3B5  bl 0x82ca9f20
	ctx.lr = 0x8325FB70;
	sub_82CA9F20(ctx, base);
	// 8325FB70: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325FB74: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325FB78: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325FB7C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325FB80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325FB80 size=64
    let mut pc: u32 = 0x8325FB80;
    'dispatch: loop {
        match pc {
            0x8325FB80 => {
    //   block [0x8325FB80..0x8325FBC0)
	// 8325FB80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325FB84: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325FB88: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325FB8C: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8325FB90: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325FB94: 388BF2DC  addi r4, r11, -0xd24
	ctx.r[4].s64 = ctx.r[11].s64 + -3364;
	// 8325FB98: 386AB014  addi r3, r10, -0x4fec
	ctx.r[3].s64 = ctx.r[10].s64 + -20460;
	// 8325FB9C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8325FBA0: 4AFCD331  bl 0x8222ced0
	ctx.lr = 0x8325FBA4;
	sub_8222CED0(ctx, base);
	// 8325FBA4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8325FBA8: 3869C7B0  addi r3, r9, -0x3850
	ctx.r[3].s64 = ctx.r[9].s64 + -14416;
	// 8325FBAC: 4BA4A375  bl 0x82ca9f20
	ctx.lr = 0x8325FBB0;
	sub_82CA9F20(ctx, base);
	// 8325FBB0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325FBB4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325FBB8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325FBBC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325FBC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325FBC0 size=64
    let mut pc: u32 = 0x8325FBC0;
    'dispatch: loop {
        match pc {
            0x8325FBC0 => {
    //   block [0x8325FBC0..0x8325FC00)
	// 8325FBC0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325FBC4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325FBC8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325FBCC: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8325FBD0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325FBD4: 388BF308  addi r4, r11, -0xcf8
	ctx.r[4].s64 = ctx.r[11].s64 + -3320;
	// 8325FBD8: 386AB018  addi r3, r10, -0x4fe8
	ctx.r[3].s64 = ctx.r[10].s64 + -20456;
	// 8325FBDC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8325FBE0: 4AFCD2F1  bl 0x8222ced0
	ctx.lr = 0x8325FBE4;
	sub_8222CED0(ctx, base);
	// 8325FBE4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8325FBE8: 3869C7C0  addi r3, r9, -0x3840
	ctx.r[3].s64 = ctx.r[9].s64 + -14400;
	// 8325FBEC: 4BA4A335  bl 0x82ca9f20
	ctx.lr = 0x8325FBF0;
	sub_82CA9F20(ctx, base);
	// 8325FBF0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325FBF4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325FBF8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325FBFC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325FC00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325FC00 size=64
    let mut pc: u32 = 0x8325FC00;
    'dispatch: loop {
        match pc {
            0x8325FC00 => {
    //   block [0x8325FC00..0x8325FC40)
	// 8325FC00: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325FC04: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325FC08: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325FC0C: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8325FC10: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325FC14: 388BF334  addi r4, r11, -0xccc
	ctx.r[4].s64 = ctx.r[11].s64 + -3276;
	// 8325FC18: 386AB01C  addi r3, r10, -0x4fe4
	ctx.r[3].s64 = ctx.r[10].s64 + -20452;
	// 8325FC1C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8325FC20: 4AFCD2B1  bl 0x8222ced0
	ctx.lr = 0x8325FC24;
	sub_8222CED0(ctx, base);
	// 8325FC24: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8325FC28: 3869C7D0  addi r3, r9, -0x3830
	ctx.r[3].s64 = ctx.r[9].s64 + -14384;
	// 8325FC2C: 4BA4A2F5  bl 0x82ca9f20
	ctx.lr = 0x8325FC30;
	sub_82CA9F20(ctx, base);
	// 8325FC30: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325FC34: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325FC38: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325FC3C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325FC40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325FC40 size=64
    let mut pc: u32 = 0x8325FC40;
    'dispatch: loop {
        match pc {
            0x8325FC40 => {
    //   block [0x8325FC40..0x8325FC80)
	// 8325FC40: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325FC44: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325FC48: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325FC4C: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8325FC50: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325FC54: 388BF35C  addi r4, r11, -0xca4
	ctx.r[4].s64 = ctx.r[11].s64 + -3236;
	// 8325FC58: 386AB020  addi r3, r10, -0x4fe0
	ctx.r[3].s64 = ctx.r[10].s64 + -20448;
	// 8325FC5C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8325FC60: 4AFCD271  bl 0x8222ced0
	ctx.lr = 0x8325FC64;
	sub_8222CED0(ctx, base);
	// 8325FC64: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8325FC68: 3869C7E0  addi r3, r9, -0x3820
	ctx.r[3].s64 = ctx.r[9].s64 + -14368;
	// 8325FC6C: 4BA4A2B5  bl 0x82ca9f20
	ctx.lr = 0x8325FC70;
	sub_82CA9F20(ctx, base);
	// 8325FC70: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325FC74: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325FC78: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325FC7C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325FC80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325FC80 size=64
    let mut pc: u32 = 0x8325FC80;
    'dispatch: loop {
        match pc {
            0x8325FC80 => {
    //   block [0x8325FC80..0x8325FCC0)
	// 8325FC80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325FC84: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325FC88: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325FC8C: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8325FC90: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325FC94: 388BF384  addi r4, r11, -0xc7c
	ctx.r[4].s64 = ctx.r[11].s64 + -3196;
	// 8325FC98: 386AB024  addi r3, r10, -0x4fdc
	ctx.r[3].s64 = ctx.r[10].s64 + -20444;
	// 8325FC9C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8325FCA0: 4AFCD231  bl 0x8222ced0
	ctx.lr = 0x8325FCA4;
	sub_8222CED0(ctx, base);
	// 8325FCA4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8325FCA8: 3869C7F0  addi r3, r9, -0x3810
	ctx.r[3].s64 = ctx.r[9].s64 + -14352;
	// 8325FCAC: 4BA4A275  bl 0x82ca9f20
	ctx.lr = 0x8325FCB0;
	sub_82CA9F20(ctx, base);
	// 8325FCB0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325FCB4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325FCB8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325FCBC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325FCC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325FCC0 size=64
    let mut pc: u32 = 0x8325FCC0;
    'dispatch: loop {
        match pc {
            0x8325FCC0 => {
    //   block [0x8325FCC0..0x8325FD00)
	// 8325FCC0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325FCC4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325FCC8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325FCCC: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8325FCD0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325FCD4: 388BF3B0  addi r4, r11, -0xc50
	ctx.r[4].s64 = ctx.r[11].s64 + -3152;
	// 8325FCD8: 386AB028  addi r3, r10, -0x4fd8
	ctx.r[3].s64 = ctx.r[10].s64 + -20440;
	// 8325FCDC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8325FCE0: 4AFCD1F1  bl 0x8222ced0
	ctx.lr = 0x8325FCE4;
	sub_8222CED0(ctx, base);
	// 8325FCE4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8325FCE8: 3869C800  addi r3, r9, -0x3800
	ctx.r[3].s64 = ctx.r[9].s64 + -14336;
	// 8325FCEC: 4BA4A235  bl 0x82ca9f20
	ctx.lr = 0x8325FCF0;
	sub_82CA9F20(ctx, base);
	// 8325FCF0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325FCF4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325FCF8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325FCFC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325FD00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325FD00 size=64
    let mut pc: u32 = 0x8325FD00;
    'dispatch: loop {
        match pc {
            0x8325FD00 => {
    //   block [0x8325FD00..0x8325FD40)
	// 8325FD00: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325FD04: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325FD08: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325FD0C: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8325FD10: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325FD14: 388BF3DC  addi r4, r11, -0xc24
	ctx.r[4].s64 = ctx.r[11].s64 + -3108;
	// 8325FD18: 386AB02C  addi r3, r10, -0x4fd4
	ctx.r[3].s64 = ctx.r[10].s64 + -20436;
	// 8325FD1C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8325FD20: 4AFCD1B1  bl 0x8222ced0
	ctx.lr = 0x8325FD24;
	sub_8222CED0(ctx, base);
	// 8325FD24: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8325FD28: 3869C810  addi r3, r9, -0x37f0
	ctx.r[3].s64 = ctx.r[9].s64 + -14320;
	// 8325FD2C: 4BA4A1F5  bl 0x82ca9f20
	ctx.lr = 0x8325FD30;
	sub_82CA9F20(ctx, base);
	// 8325FD30: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325FD34: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325FD38: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325FD3C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325FD40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325FD40 size=64
    let mut pc: u32 = 0x8325FD40;
    'dispatch: loop {
        match pc {
            0x8325FD40 => {
    //   block [0x8325FD40..0x8325FD80)
	// 8325FD40: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325FD44: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325FD48: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325FD4C: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8325FD50: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325FD54: 388BF408  addi r4, r11, -0xbf8
	ctx.r[4].s64 = ctx.r[11].s64 + -3064;
	// 8325FD58: 386AB030  addi r3, r10, -0x4fd0
	ctx.r[3].s64 = ctx.r[10].s64 + -20432;
	// 8325FD5C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8325FD60: 4AFCD171  bl 0x8222ced0
	ctx.lr = 0x8325FD64;
	sub_8222CED0(ctx, base);
	// 8325FD64: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8325FD68: 3869C820  addi r3, r9, -0x37e0
	ctx.r[3].s64 = ctx.r[9].s64 + -14304;
	// 8325FD6C: 4BA4A1B5  bl 0x82ca9f20
	ctx.lr = 0x8325FD70;
	sub_82CA9F20(ctx, base);
	// 8325FD70: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325FD74: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325FD78: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325FD7C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325FD80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325FD80 size=56
    let mut pc: u32 = 0x8325FD80;
    'dispatch: loop {
        match pc {
            0x8325FD80 => {
    //   block [0x8325FD80..0x8325FDB8)
	// 8325FD80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325FD84: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325FD88: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325FD8C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8325FD90: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8325FD94: 386B42C8  addi r3, r11, 0x42c8
	ctx.r[3].s64 = ctx.r[11].s64 + 17096;
	// 8325FD98: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8325FD9C: 4AF93FBD  bl 0x821f3d58
	ctx.lr = 0x8325FDA0;
	sub_821F3D58(ctx, base);
	// 8325FDA0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325FDA4: 906AB034  stw r3, -0x4fcc(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-20428 as u32), ctx.r[3].u32 ) };
	// 8325FDA8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325FDAC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325FDB0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325FDB4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325FDB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325FDB8 size=56
    let mut pc: u32 = 0x8325FDB8;
    'dispatch: loop {
        match pc {
            0x8325FDB8 => {
    //   block [0x8325FDB8..0x8325FDF0)
	// 8325FDB8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325FDBC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325FDC0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325FDC4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8325FDC8: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8325FDCC: 386B42DC  addi r3, r11, 0x42dc
	ctx.r[3].s64 = ctx.r[11].s64 + 17116;
	// 8325FDD0: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8325FDD4: 4AF93F85  bl 0x821f3d58
	ctx.lr = 0x8325FDD8;
	sub_821F3D58(ctx, base);
	// 8325FDD8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325FDDC: 906AB038  stw r3, -0x4fc8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-20424 as u32), ctx.r[3].u32 ) };
	// 8325FDE0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325FDE4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325FDE8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325FDEC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325FDF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325FDF0 size=56
    let mut pc: u32 = 0x8325FDF0;
    'dispatch: loop {
        match pc {
            0x8325FDF0 => {
    //   block [0x8325FDF0..0x8325FE28)
	// 8325FDF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325FDF4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325FDF8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325FDFC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8325FE00: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8325FE04: 386B42E8  addi r3, r11, 0x42e8
	ctx.r[3].s64 = ctx.r[11].s64 + 17128;
	// 8325FE08: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8325FE0C: 4AF93F4D  bl 0x821f3d58
	ctx.lr = 0x8325FE10;
	sub_821F3D58(ctx, base);
	// 8325FE10: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325FE14: 906AB03C  stw r3, -0x4fc4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-20420 as u32), ctx.r[3].u32 ) };
	// 8325FE18: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325FE1C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325FE20: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325FE24: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325FE28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325FE28 size=56
    let mut pc: u32 = 0x8325FE28;
    'dispatch: loop {
        match pc {
            0x8325FE28 => {
    //   block [0x8325FE28..0x8325FE60)
	// 8325FE28: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325FE2C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325FE30: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325FE34: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8325FE38: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8325FE3C: 386B4300  addi r3, r11, 0x4300
	ctx.r[3].s64 = ctx.r[11].s64 + 17152;
	// 8325FE40: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8325FE44: 4AF93F15  bl 0x821f3d58
	ctx.lr = 0x8325FE48;
	sub_821F3D58(ctx, base);
	// 8325FE48: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325FE4C: 906AB040  stw r3, -0x4fc0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-20416 as u32), ctx.r[3].u32 ) };
	// 8325FE50: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325FE54: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325FE58: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325FE5C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325FE60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325FE60 size=56
    let mut pc: u32 = 0x8325FE60;
    'dispatch: loop {
        match pc {
            0x8325FE60 => {
    //   block [0x8325FE60..0x8325FE98)
	// 8325FE60: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325FE64: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325FE68: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325FE6C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8325FE70: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8325FE74: 386B4310  addi r3, r11, 0x4310
	ctx.r[3].s64 = ctx.r[11].s64 + 17168;
	// 8325FE78: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8325FE7C: 4AF93EDD  bl 0x821f3d58
	ctx.lr = 0x8325FE80;
	sub_821F3D58(ctx, base);
	// 8325FE80: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325FE84: 906AB044  stw r3, -0x4fbc(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-20412 as u32), ctx.r[3].u32 ) };
	// 8325FE88: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325FE8C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325FE90: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325FE94: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325FE98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325FE98 size=56
    let mut pc: u32 = 0x8325FE98;
    'dispatch: loop {
        match pc {
            0x8325FE98 => {
    //   block [0x8325FE98..0x8325FED0)
	// 8325FE98: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325FE9C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325FEA0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325FEA4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8325FEA8: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8325FEAC: 386B4328  addi r3, r11, 0x4328
	ctx.r[3].s64 = ctx.r[11].s64 + 17192;
	// 8325FEB0: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8325FEB4: 4AF93EA5  bl 0x821f3d58
	ctx.lr = 0x8325FEB8;
	sub_821F3D58(ctx, base);
	// 8325FEB8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325FEBC: 906AB048  stw r3, -0x4fb8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-20408 as u32), ctx.r[3].u32 ) };
	// 8325FEC0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325FEC4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325FEC8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325FECC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325FED0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325FED0 size=56
    let mut pc: u32 = 0x8325FED0;
    'dispatch: loop {
        match pc {
            0x8325FED0 => {
    //   block [0x8325FED0..0x8325FF08)
	// 8325FED0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325FED4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325FED8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325FEDC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8325FEE0: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8325FEE4: 386B4338  addi r3, r11, 0x4338
	ctx.r[3].s64 = ctx.r[11].s64 + 17208;
	// 8325FEE8: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8325FEEC: 4AF93E6D  bl 0x821f3d58
	ctx.lr = 0x8325FEF0;
	sub_821F3D58(ctx, base);
	// 8325FEF0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325FEF4: 906AB04C  stw r3, -0x4fb4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-20404 as u32), ctx.r[3].u32 ) };
	// 8325FEF8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325FEFC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325FF00: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325FF04: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325FF08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325FF08 size=56
    let mut pc: u32 = 0x8325FF08;
    'dispatch: loop {
        match pc {
            0x8325FF08 => {
    //   block [0x8325FF08..0x8325FF40)
	// 8325FF08: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325FF0C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325FF10: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325FF14: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8325FF18: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8325FF1C: 386B4350  addi r3, r11, 0x4350
	ctx.r[3].s64 = ctx.r[11].s64 + 17232;
	// 8325FF20: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8325FF24: 4AF93E35  bl 0x821f3d58
	ctx.lr = 0x8325FF28;
	sub_821F3D58(ctx, base);
	// 8325FF28: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325FF2C: 906AB050  stw r3, -0x4fb0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-20400 as u32), ctx.r[3].u32 ) };
	// 8325FF30: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325FF34: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325FF38: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325FF3C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325FF40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325FF40 size=56
    let mut pc: u32 = 0x8325FF40;
    'dispatch: loop {
        match pc {
            0x8325FF40 => {
    //   block [0x8325FF40..0x8325FF78)
	// 8325FF40: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325FF44: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325FF48: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325FF4C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8325FF50: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8325FF54: 386B4368  addi r3, r11, 0x4368
	ctx.r[3].s64 = ctx.r[11].s64 + 17256;
	// 8325FF58: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8325FF5C: 4AF93DFD  bl 0x821f3d58
	ctx.lr = 0x8325FF60;
	sub_821F3D58(ctx, base);
	// 8325FF60: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325FF64: 906AB054  stw r3, -0x4fac(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-20396 as u32), ctx.r[3].u32 ) };
	// 8325FF68: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325FF6C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325FF70: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325FF74: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325FF78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325FF78 size=56
    let mut pc: u32 = 0x8325FF78;
    'dispatch: loop {
        match pc {
            0x8325FF78 => {
    //   block [0x8325FF78..0x8325FFB0)
	// 8325FF78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325FF7C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325FF80: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325FF84: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8325FF88: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8325FF8C: 386B4380  addi r3, r11, 0x4380
	ctx.r[3].s64 = ctx.r[11].s64 + 17280;
	// 8325FF90: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8325FF94: 4AF93DC5  bl 0x821f3d58
	ctx.lr = 0x8325FF98;
	sub_821F3D58(ctx, base);
	// 8325FF98: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325FF9C: 906AB058  stw r3, -0x4fa8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-20392 as u32), ctx.r[3].u32 ) };
	// 8325FFA0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325FFA4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325FFA8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325FFAC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325FFB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325FFB0 size=56
    let mut pc: u32 = 0x8325FFB0;
    'dispatch: loop {
        match pc {
            0x8325FFB0 => {
    //   block [0x8325FFB0..0x8325FFE8)
	// 8325FFB0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325FFB4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325FFB8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325FFBC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8325FFC0: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8325FFC4: 386B4394  addi r3, r11, 0x4394
	ctx.r[3].s64 = ctx.r[11].s64 + 17300;
	// 8325FFC8: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8325FFCC: 4AF93D8D  bl 0x821f3d58
	ctx.lr = 0x8325FFD0;
	sub_821F3D58(ctx, base);
	// 8325FFD0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325FFD4: 906AB05C  stw r3, -0x4fa4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-20388 as u32), ctx.r[3].u32 ) };
	// 8325FFD8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325FFDC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325FFE0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325FFE4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8325FFE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8325FFE8 size=56
    let mut pc: u32 = 0x8325FFE8;
    'dispatch: loop {
        match pc {
            0x8325FFE8 => {
    //   block [0x8325FFE8..0x83260020)
	// 8325FFE8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325FFEC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8325FFF0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325FFF4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8325FFF8: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8325FFFC: 386B43B4  addi r3, r11, 0x43b4
	ctx.r[3].s64 = ctx.r[11].s64 + 17332;
	// 83260000: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83260004: 4AF93D55  bl 0x821f3d58
	ctx.lr = 0x83260008;
	sub_821F3D58(ctx, base);
	// 83260008: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326000C: 906AB060  stw r3, -0x4fa0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-20384 as u32), ctx.r[3].u32 ) };
	// 83260010: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83260014: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83260018: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326001C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83260020(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83260020 size=56
    let mut pc: u32 = 0x83260020;
    'dispatch: loop {
        match pc {
            0x83260020 => {
    //   block [0x83260020..0x83260058)
	// 83260020: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83260024: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83260028: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326002C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83260030: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83260034: 386B43C8  addi r3, r11, 0x43c8
	ctx.r[3].s64 = ctx.r[11].s64 + 17352;
	// 83260038: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8326003C: 4AF93D1D  bl 0x821f3d58
	ctx.lr = 0x83260040;
	sub_821F3D58(ctx, base);
	// 83260040: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83260044: 906AB064  stw r3, -0x4f9c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-20380 as u32), ctx.r[3].u32 ) };
	// 83260048: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326004C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83260050: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83260054: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83260058(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83260058 size=56
    let mut pc: u32 = 0x83260058;
    'dispatch: loop {
        match pc {
            0x83260058 => {
    //   block [0x83260058..0x83260090)
	// 83260058: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326005C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83260060: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83260064: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83260068: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8326006C: 386B43D4  addi r3, r11, 0x43d4
	ctx.r[3].s64 = ctx.r[11].s64 + 17364;
	// 83260070: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83260074: 4AF93CE5  bl 0x821f3d58
	ctx.lr = 0x83260078;
	sub_821F3D58(ctx, base);
	// 83260078: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326007C: 906AB068  stw r3, -0x4f98(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-20376 as u32), ctx.r[3].u32 ) };
	// 83260080: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83260084: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83260088: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326008C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83260090(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83260090 size=56
    let mut pc: u32 = 0x83260090;
    'dispatch: loop {
        match pc {
            0x83260090 => {
    //   block [0x83260090..0x832600C8)
	// 83260090: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83260094: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83260098: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326009C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 832600A0: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 832600A4: 386B43E0  addi r3, r11, 0x43e0
	ctx.r[3].s64 = ctx.r[11].s64 + 17376;
	// 832600A8: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 832600AC: 4AF93CAD  bl 0x821f3d58
	ctx.lr = 0x832600B0;
	sub_821F3D58(ctx, base);
	// 832600B0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832600B4: 906AB06C  stw r3, -0x4f94(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-20372 as u32), ctx.r[3].u32 ) };
	// 832600B8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832600BC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832600C0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832600C4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832600C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832600C8 size=56
    let mut pc: u32 = 0x832600C8;
    'dispatch: loop {
        match pc {
            0x832600C8 => {
    //   block [0x832600C8..0x83260100)
	// 832600C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832600CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832600D0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832600D4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 832600D8: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 832600DC: 386B43F0  addi r3, r11, 0x43f0
	ctx.r[3].s64 = ctx.r[11].s64 + 17392;
	// 832600E0: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 832600E4: 4AF93C75  bl 0x821f3d58
	ctx.lr = 0x832600E8;
	sub_821F3D58(ctx, base);
	// 832600E8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832600EC: 906AB070  stw r3, -0x4f90(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-20368 as u32), ctx.r[3].u32 ) };
	// 832600F0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832600F4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832600F8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832600FC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83260100(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83260100 size=56
    let mut pc: u32 = 0x83260100;
    'dispatch: loop {
        match pc {
            0x83260100 => {
    //   block [0x83260100..0x83260138)
	// 83260100: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83260104: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83260108: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326010C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83260110: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83260114: 386B4400  addi r3, r11, 0x4400
	ctx.r[3].s64 = ctx.r[11].s64 + 17408;
	// 83260118: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8326011C: 4AF93C3D  bl 0x821f3d58
	ctx.lr = 0x83260120;
	sub_821F3D58(ctx, base);
	// 83260120: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83260124: 906AB074  stw r3, -0x4f8c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-20364 as u32), ctx.r[3].u32 ) };
	// 83260128: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326012C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83260130: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83260134: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83260138(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83260138 size=56
    let mut pc: u32 = 0x83260138;
    'dispatch: loop {
        match pc {
            0x83260138 => {
    //   block [0x83260138..0x83260170)
	// 83260138: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326013C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83260140: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83260144: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83260148: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8326014C: 386B4418  addi r3, r11, 0x4418
	ctx.r[3].s64 = ctx.r[11].s64 + 17432;
	// 83260150: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83260154: 4AF93C05  bl 0x821f3d58
	ctx.lr = 0x83260158;
	sub_821F3D58(ctx, base);
	// 83260158: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326015C: 906AB078  stw r3, -0x4f88(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-20360 as u32), ctx.r[3].u32 ) };
	// 83260160: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83260164: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83260168: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326016C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83260170(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83260170 size=56
    let mut pc: u32 = 0x83260170;
    'dispatch: loop {
        match pc {
            0x83260170 => {
    //   block [0x83260170..0x832601A8)
	// 83260170: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83260174: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83260178: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326017C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83260180: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83260184: 386B4424  addi r3, r11, 0x4424
	ctx.r[3].s64 = ctx.r[11].s64 + 17444;
	// 83260188: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8326018C: 4AF93BCD  bl 0x821f3d58
	ctx.lr = 0x83260190;
	sub_821F3D58(ctx, base);
	// 83260190: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83260194: 906AB07C  stw r3, -0x4f84(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-20356 as u32), ctx.r[3].u32 ) };
	// 83260198: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326019C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832601A0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832601A4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832601A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832601A8 size=56
    let mut pc: u32 = 0x832601A8;
    'dispatch: loop {
        match pc {
            0x832601A8 => {
    //   block [0x832601A8..0x832601E0)
	// 832601A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832601AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832601B0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832601B4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 832601B8: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 832601BC: 386B4430  addi r3, r11, 0x4430
	ctx.r[3].s64 = ctx.r[11].s64 + 17456;
	// 832601C0: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 832601C4: 4AF93B95  bl 0x821f3d58
	ctx.lr = 0x832601C8;
	sub_821F3D58(ctx, base);
	// 832601C8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832601CC: 906AB080  stw r3, -0x4f80(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-20352 as u32), ctx.r[3].u32 ) };
	// 832601D0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832601D4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832601D8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832601DC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832601E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832601E0 size=56
    let mut pc: u32 = 0x832601E0;
    'dispatch: loop {
        match pc {
            0x832601E0 => {
    //   block [0x832601E0..0x83260218)
	// 832601E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832601E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832601E8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832601EC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 832601F0: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 832601F4: 386B4440  addi r3, r11, 0x4440
	ctx.r[3].s64 = ctx.r[11].s64 + 17472;
	// 832601F8: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 832601FC: 4AF93B5D  bl 0x821f3d58
	ctx.lr = 0x83260200;
	sub_821F3D58(ctx, base);
	// 83260200: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83260204: 906AB084  stw r3, -0x4f7c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-20348 as u32), ctx.r[3].u32 ) };
	// 83260208: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326020C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83260210: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83260214: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83260218(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83260218 size=64
    let mut pc: u32 = 0x83260218;
    'dispatch: loop {
        match pc {
            0x83260218 => {
    //   block [0x83260218..0x83260258)
	// 83260218: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326021C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83260220: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83260224: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 83260228: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326022C: 388BCE9C  addi r4, r11, -0x3164
	ctx.r[4].s64 = ctx.r[11].s64 + -12644;
	// 83260230: 386AB088  addi r3, r10, -0x4f78
	ctx.r[3].s64 = ctx.r[10].s64 + -20344;
	// 83260234: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83260238: 4AFCCC99  bl 0x8222ced0
	ctx.lr = 0x8326023C;
	sub_8222CED0(ctx, base);
	// 8326023C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83260240: 3869C830  addi r3, r9, -0x37d0
	ctx.r[3].s64 = ctx.r[9].s64 + -14288;
	// 83260244: 4BA49CDD  bl 0x82ca9f20
	ctx.lr = 0x83260248;
	sub_82CA9F20(ctx, base);
	// 83260248: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326024C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83260250: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83260254: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83260258(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83260258 size=56
    let mut pc: u32 = 0x83260258;
    'dispatch: loop {
        match pc {
            0x83260258 => {
    //   block [0x83260258..0x83260290)
	// 83260258: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326025C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83260260: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83260264: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83260268: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8326026C: 386B42C8  addi r3, r11, 0x42c8
	ctx.r[3].s64 = ctx.r[11].s64 + 17096;
	// 83260270: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83260274: 4AF93AE5  bl 0x821f3d58
	ctx.lr = 0x83260278;
	sub_821F3D58(ctx, base);
	// 83260278: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326027C: 906AB08C  stw r3, -0x4f74(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-20340 as u32), ctx.r[3].u32 ) };
	// 83260280: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83260284: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83260288: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326028C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83260290(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83260290 size=56
    let mut pc: u32 = 0x83260290;
    'dispatch: loop {
        match pc {
            0x83260290 => {
    //   block [0x83260290..0x832602C8)
	// 83260290: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83260294: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83260298: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326029C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 832602A0: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 832602A4: 386B42DC  addi r3, r11, 0x42dc
	ctx.r[3].s64 = ctx.r[11].s64 + 17116;
	// 832602A8: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 832602AC: 4AF93AAD  bl 0x821f3d58
	ctx.lr = 0x832602B0;
	sub_821F3D58(ctx, base);
	// 832602B0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832602B4: 906AB090  stw r3, -0x4f70(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-20336 as u32), ctx.r[3].u32 ) };
	// 832602B8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832602BC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832602C0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832602C4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832602C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832602C8 size=56
    let mut pc: u32 = 0x832602C8;
    'dispatch: loop {
        match pc {
            0x832602C8 => {
    //   block [0x832602C8..0x83260300)
	// 832602C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832602CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832602D0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832602D4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 832602D8: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 832602DC: 386B42E8  addi r3, r11, 0x42e8
	ctx.r[3].s64 = ctx.r[11].s64 + 17128;
	// 832602E0: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 832602E4: 4AF93A75  bl 0x821f3d58
	ctx.lr = 0x832602E8;
	sub_821F3D58(ctx, base);
	// 832602E8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832602EC: 906AB094  stw r3, -0x4f6c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-20332 as u32), ctx.r[3].u32 ) };
	// 832602F0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832602F4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832602F8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832602FC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83260300(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83260300 size=56
    let mut pc: u32 = 0x83260300;
    'dispatch: loop {
        match pc {
            0x83260300 => {
    //   block [0x83260300..0x83260338)
	// 83260300: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83260304: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83260308: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326030C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83260310: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83260314: 386B4300  addi r3, r11, 0x4300
	ctx.r[3].s64 = ctx.r[11].s64 + 17152;
	// 83260318: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8326031C: 4AF93A3D  bl 0x821f3d58
	ctx.lr = 0x83260320;
	sub_821F3D58(ctx, base);
	// 83260320: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83260324: 906AB098  stw r3, -0x4f68(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-20328 as u32), ctx.r[3].u32 ) };
	// 83260328: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326032C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83260330: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83260334: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83260338(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83260338 size=56
    let mut pc: u32 = 0x83260338;
    'dispatch: loop {
        match pc {
            0x83260338 => {
    //   block [0x83260338..0x83260370)
	// 83260338: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326033C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83260340: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83260344: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83260348: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8326034C: 386B4310  addi r3, r11, 0x4310
	ctx.r[3].s64 = ctx.r[11].s64 + 17168;
	// 83260350: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83260354: 4AF93A05  bl 0x821f3d58
	ctx.lr = 0x83260358;
	sub_821F3D58(ctx, base);
	// 83260358: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326035C: 906AB09C  stw r3, -0x4f64(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-20324 as u32), ctx.r[3].u32 ) };
	// 83260360: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83260364: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83260368: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326036C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83260370(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83260370 size=56
    let mut pc: u32 = 0x83260370;
    'dispatch: loop {
        match pc {
            0x83260370 => {
    //   block [0x83260370..0x832603A8)
	// 83260370: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83260374: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83260378: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326037C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83260380: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83260384: 386B4328  addi r3, r11, 0x4328
	ctx.r[3].s64 = ctx.r[11].s64 + 17192;
	// 83260388: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8326038C: 4AF939CD  bl 0x821f3d58
	ctx.lr = 0x83260390;
	sub_821F3D58(ctx, base);
	// 83260390: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83260394: 906AB0A0  stw r3, -0x4f60(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-20320 as u32), ctx.r[3].u32 ) };
	// 83260398: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326039C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832603A0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832603A4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832603A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832603A8 size=56
    let mut pc: u32 = 0x832603A8;
    'dispatch: loop {
        match pc {
            0x832603A8 => {
    //   block [0x832603A8..0x832603E0)
	// 832603A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832603AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832603B0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832603B4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 832603B8: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 832603BC: 386B4338  addi r3, r11, 0x4338
	ctx.r[3].s64 = ctx.r[11].s64 + 17208;
	// 832603C0: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 832603C4: 4AF93995  bl 0x821f3d58
	ctx.lr = 0x832603C8;
	sub_821F3D58(ctx, base);
	// 832603C8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832603CC: 906AB0A4  stw r3, -0x4f5c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-20316 as u32), ctx.r[3].u32 ) };
	// 832603D0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832603D4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832603D8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832603DC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832603E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832603E0 size=56
    let mut pc: u32 = 0x832603E0;
    'dispatch: loop {
        match pc {
            0x832603E0 => {
    //   block [0x832603E0..0x83260418)
	// 832603E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832603E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832603E8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832603EC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 832603F0: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 832603F4: 386B4350  addi r3, r11, 0x4350
	ctx.r[3].s64 = ctx.r[11].s64 + 17232;
	// 832603F8: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 832603FC: 4AF9395D  bl 0x821f3d58
	ctx.lr = 0x83260400;
	sub_821F3D58(ctx, base);
	// 83260400: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83260404: 906AB0A8  stw r3, -0x4f58(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-20312 as u32), ctx.r[3].u32 ) };
	// 83260408: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326040C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83260410: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83260414: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83260418(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83260418 size=56
    let mut pc: u32 = 0x83260418;
    'dispatch: loop {
        match pc {
            0x83260418 => {
    //   block [0x83260418..0x83260450)
	// 83260418: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326041C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83260420: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83260424: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83260428: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8326042C: 386B4368  addi r3, r11, 0x4368
	ctx.r[3].s64 = ctx.r[11].s64 + 17256;
	// 83260430: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83260434: 4AF93925  bl 0x821f3d58
	ctx.lr = 0x83260438;
	sub_821F3D58(ctx, base);
	// 83260438: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326043C: 906AB0AC  stw r3, -0x4f54(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-20308 as u32), ctx.r[3].u32 ) };
	// 83260440: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83260444: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83260448: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326044C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83260450(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83260450 size=56
    let mut pc: u32 = 0x83260450;
    'dispatch: loop {
        match pc {
            0x83260450 => {
    //   block [0x83260450..0x83260488)
	// 83260450: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83260454: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83260458: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326045C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83260460: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83260464: 386B4380  addi r3, r11, 0x4380
	ctx.r[3].s64 = ctx.r[11].s64 + 17280;
	// 83260468: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8326046C: 4AF938ED  bl 0x821f3d58
	ctx.lr = 0x83260470;
	sub_821F3D58(ctx, base);
	// 83260470: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83260474: 906AB0B0  stw r3, -0x4f50(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-20304 as u32), ctx.r[3].u32 ) };
	// 83260478: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326047C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83260480: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83260484: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83260488(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83260488 size=56
    let mut pc: u32 = 0x83260488;
    'dispatch: loop {
        match pc {
            0x83260488 => {
    //   block [0x83260488..0x832604C0)
	// 83260488: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326048C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83260490: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83260494: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83260498: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8326049C: 386B4394  addi r3, r11, 0x4394
	ctx.r[3].s64 = ctx.r[11].s64 + 17300;
	// 832604A0: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 832604A4: 4AF938B5  bl 0x821f3d58
	ctx.lr = 0x832604A8;
	sub_821F3D58(ctx, base);
	// 832604A8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832604AC: 906AB0B4  stw r3, -0x4f4c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-20300 as u32), ctx.r[3].u32 ) };
	// 832604B0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832604B4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832604B8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832604BC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832604C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832604C0 size=56
    let mut pc: u32 = 0x832604C0;
    'dispatch: loop {
        match pc {
            0x832604C0 => {
    //   block [0x832604C0..0x832604F8)
	// 832604C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832604C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832604C8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832604CC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 832604D0: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 832604D4: 386B43B4  addi r3, r11, 0x43b4
	ctx.r[3].s64 = ctx.r[11].s64 + 17332;
	// 832604D8: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 832604DC: 4AF9387D  bl 0x821f3d58
	ctx.lr = 0x832604E0;
	sub_821F3D58(ctx, base);
	// 832604E0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832604E4: 906AB0B8  stw r3, -0x4f48(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-20296 as u32), ctx.r[3].u32 ) };
	// 832604E8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832604EC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832604F0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832604F4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832604F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832604F8 size=56
    let mut pc: u32 = 0x832604F8;
    'dispatch: loop {
        match pc {
            0x832604F8 => {
    //   block [0x832604F8..0x83260530)
	// 832604F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832604FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83260500: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83260504: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83260508: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8326050C: 386B43C8  addi r3, r11, 0x43c8
	ctx.r[3].s64 = ctx.r[11].s64 + 17352;
	// 83260510: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83260514: 4AF93845  bl 0x821f3d58
	ctx.lr = 0x83260518;
	sub_821F3D58(ctx, base);
	// 83260518: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326051C: 906AB0BC  stw r3, -0x4f44(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-20292 as u32), ctx.r[3].u32 ) };
	// 83260520: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83260524: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83260528: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326052C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83260530(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83260530 size=56
    let mut pc: u32 = 0x83260530;
    'dispatch: loop {
        match pc {
            0x83260530 => {
    //   block [0x83260530..0x83260568)
	// 83260530: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83260534: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83260538: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326053C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83260540: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83260544: 386B43D4  addi r3, r11, 0x43d4
	ctx.r[3].s64 = ctx.r[11].s64 + 17364;
	// 83260548: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8326054C: 4AF9380D  bl 0x821f3d58
	ctx.lr = 0x83260550;
	sub_821F3D58(ctx, base);
	// 83260550: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83260554: 906AB0C0  stw r3, -0x4f40(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-20288 as u32), ctx.r[3].u32 ) };
	// 83260558: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326055C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83260560: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83260564: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83260568(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83260568 size=56
    let mut pc: u32 = 0x83260568;
    'dispatch: loop {
        match pc {
            0x83260568 => {
    //   block [0x83260568..0x832605A0)
	// 83260568: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326056C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83260570: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83260574: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83260578: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8326057C: 386B43E0  addi r3, r11, 0x43e0
	ctx.r[3].s64 = ctx.r[11].s64 + 17376;
	// 83260580: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83260584: 4AF937D5  bl 0x821f3d58
	ctx.lr = 0x83260588;
	sub_821F3D58(ctx, base);
	// 83260588: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326058C: 906AB0C4  stw r3, -0x4f3c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-20284 as u32), ctx.r[3].u32 ) };
	// 83260590: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83260594: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83260598: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326059C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832605A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832605A0 size=56
    let mut pc: u32 = 0x832605A0;
    'dispatch: loop {
        match pc {
            0x832605A0 => {
    //   block [0x832605A0..0x832605D8)
	// 832605A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832605A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832605A8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832605AC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 832605B0: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 832605B4: 386B43F0  addi r3, r11, 0x43f0
	ctx.r[3].s64 = ctx.r[11].s64 + 17392;
	// 832605B8: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 832605BC: 4AF9379D  bl 0x821f3d58
	ctx.lr = 0x832605C0;
	sub_821F3D58(ctx, base);
	// 832605C0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832605C4: 906AB0C8  stw r3, -0x4f38(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-20280 as u32), ctx.r[3].u32 ) };
	// 832605C8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832605CC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832605D0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832605D4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832605D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832605D8 size=56
    let mut pc: u32 = 0x832605D8;
    'dispatch: loop {
        match pc {
            0x832605D8 => {
    //   block [0x832605D8..0x83260610)
	// 832605D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832605DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832605E0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832605E4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 832605E8: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 832605EC: 386B4400  addi r3, r11, 0x4400
	ctx.r[3].s64 = ctx.r[11].s64 + 17408;
	// 832605F0: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 832605F4: 4AF93765  bl 0x821f3d58
	ctx.lr = 0x832605F8;
	sub_821F3D58(ctx, base);
	// 832605F8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832605FC: 906AB0CC  stw r3, -0x4f34(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-20276 as u32), ctx.r[3].u32 ) };
	// 83260600: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83260604: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83260608: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326060C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83260610(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83260610 size=56
    let mut pc: u32 = 0x83260610;
    'dispatch: loop {
        match pc {
            0x83260610 => {
    //   block [0x83260610..0x83260648)
	// 83260610: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83260614: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83260618: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326061C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83260620: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83260624: 386B4418  addi r3, r11, 0x4418
	ctx.r[3].s64 = ctx.r[11].s64 + 17432;
	// 83260628: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8326062C: 4AF9372D  bl 0x821f3d58
	ctx.lr = 0x83260630;
	sub_821F3D58(ctx, base);
	// 83260630: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83260634: 906AB0D0  stw r3, -0x4f30(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-20272 as u32), ctx.r[3].u32 ) };
	// 83260638: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326063C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83260640: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83260644: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83260648(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83260648 size=56
    let mut pc: u32 = 0x83260648;
    'dispatch: loop {
        match pc {
            0x83260648 => {
    //   block [0x83260648..0x83260680)
	// 83260648: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326064C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83260650: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83260654: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83260658: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8326065C: 386B4424  addi r3, r11, 0x4424
	ctx.r[3].s64 = ctx.r[11].s64 + 17444;
	// 83260660: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83260664: 4AF936F5  bl 0x821f3d58
	ctx.lr = 0x83260668;
	sub_821F3D58(ctx, base);
	// 83260668: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326066C: 906AB0D4  stw r3, -0x4f2c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-20268 as u32), ctx.r[3].u32 ) };
	// 83260670: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83260674: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83260678: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326067C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83260680(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83260680 size=56
    let mut pc: u32 = 0x83260680;
    'dispatch: loop {
        match pc {
            0x83260680 => {
    //   block [0x83260680..0x832606B8)
	// 83260680: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83260684: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83260688: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326068C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83260690: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83260694: 386B4430  addi r3, r11, 0x4430
	ctx.r[3].s64 = ctx.r[11].s64 + 17456;
	// 83260698: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8326069C: 4AF936BD  bl 0x821f3d58
	ctx.lr = 0x832606A0;
	sub_821F3D58(ctx, base);
	// 832606A0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832606A4: 906AB0D8  stw r3, -0x4f28(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-20264 as u32), ctx.r[3].u32 ) };
	// 832606A8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832606AC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832606B0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832606B4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832606B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832606B8 size=56
    let mut pc: u32 = 0x832606B8;
    'dispatch: loop {
        match pc {
            0x832606B8 => {
    //   block [0x832606B8..0x832606F0)
	// 832606B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832606BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832606C0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832606C4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 832606C8: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 832606CC: 386B4440  addi r3, r11, 0x4440
	ctx.r[3].s64 = ctx.r[11].s64 + 17472;
	// 832606D0: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 832606D4: 4AF93685  bl 0x821f3d58
	ctx.lr = 0x832606D8;
	sub_821F3D58(ctx, base);
	// 832606D8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832606DC: 906AB0DC  stw r3, -0x4f24(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-20260 as u32), ctx.r[3].u32 ) };
	// 832606E0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832606E4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832606E8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832606EC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832606F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832606F0 size=64
    let mut pc: u32 = 0x832606F0;
    'dispatch: loop {
        match pc {
            0x832606F0 => {
    //   block [0x832606F0..0x83260730)
	// 832606F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832606F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832606F8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832606FC: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 83260700: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83260704: 388BDED0  addi r4, r11, -0x2130
	ctx.r[4].s64 = ctx.r[11].s64 + -8496;
	// 83260708: 386AB0E0  addi r3, r10, -0x4f20
	ctx.r[3].s64 = ctx.r[10].s64 + -20256;
	// 8326070C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83260710: 4AFCC7C1  bl 0x8222ced0
	ctx.lr = 0x83260714;
	sub_8222CED0(ctx, base);
	// 83260714: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83260718: 3869C840  addi r3, r9, -0x37c0
	ctx.r[3].s64 = ctx.r[9].s64 + -14272;
	// 8326071C: 4BA49805  bl 0x82ca9f20
	ctx.lr = 0x83260720;
	sub_82CA9F20(ctx, base);
	// 83260720: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83260724: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83260728: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326072C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83260730(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83260730 size=64
    let mut pc: u32 = 0x83260730;
    'dispatch: loop {
        match pc {
            0x83260730 => {
    //   block [0x83260730..0x83260770)
	// 83260730: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83260734: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83260738: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326073C: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 83260740: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83260744: 388BCE9C  addi r4, r11, -0x3164
	ctx.r[4].s64 = ctx.r[11].s64 + -12644;
	// 83260748: 386AB0E4  addi r3, r10, -0x4f1c
	ctx.r[3].s64 = ctx.r[10].s64 + -20252;
	// 8326074C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83260750: 4AFCC781  bl 0x8222ced0
	ctx.lr = 0x83260754;
	sub_8222CED0(ctx, base);
	// 83260754: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83260758: 3869C850  addi r3, r9, -0x37b0
	ctx.r[3].s64 = ctx.r[9].s64 + -14256;
	// 8326075C: 4BA497C5  bl 0x82ca9f20
	ctx.lr = 0x83260760;
	sub_82CA9F20(ctx, base);
	// 83260760: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83260764: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83260768: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326076C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83260770(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83260770 size=64
    let mut pc: u32 = 0x83260770;
    'dispatch: loop {
        match pc {
            0x83260770 => {
    //   block [0x83260770..0x832607B0)
	// 83260770: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83260774: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83260778: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326077C: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 83260780: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83260784: 388BF5B0  addi r4, r11, -0xa50
	ctx.r[4].s64 = ctx.r[11].s64 + -2640;
	// 83260788: 386AB0E8  addi r3, r10, -0x4f18
	ctx.r[3].s64 = ctx.r[10].s64 + -20248;
	// 8326078C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83260790: 4AFCC741  bl 0x8222ced0
	ctx.lr = 0x83260794;
	sub_8222CED0(ctx, base);
	// 83260794: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83260798: 3869C860  addi r3, r9, -0x37a0
	ctx.r[3].s64 = ctx.r[9].s64 + -14240;
	// 8326079C: 4BA49785  bl 0x82ca9f20
	ctx.lr = 0x832607A0;
	sub_82CA9F20(ctx, base);
	// 832607A0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832607A4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832607A8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832607AC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832607B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832607B0 size=64
    let mut pc: u32 = 0x832607B0;
    'dispatch: loop {
        match pc {
            0x832607B0 => {
    //   block [0x832607B0..0x832607F0)
	// 832607B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832607B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832607B8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832607BC: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 832607C0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832607C4: 388BF5D4  addi r4, r11, -0xa2c
	ctx.r[4].s64 = ctx.r[11].s64 + -2604;
	// 832607C8: 386AB0EC  addi r3, r10, -0x4f14
	ctx.r[3].s64 = ctx.r[10].s64 + -20244;
	// 832607CC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832607D0: 4AFCC701  bl 0x8222ced0
	ctx.lr = 0x832607D4;
	sub_8222CED0(ctx, base);
	// 832607D4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 832607D8: 3869C870  addi r3, r9, -0x3790
	ctx.r[3].s64 = ctx.r[9].s64 + -14224;
	// 832607DC: 4BA49745  bl 0x82ca9f20
	ctx.lr = 0x832607E0;
	sub_82CA9F20(ctx, base);
	// 832607E0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832607E4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832607E8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832607EC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832607F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832607F0 size=64
    let mut pc: u32 = 0x832607F0;
    'dispatch: loop {
        match pc {
            0x832607F0 => {
    //   block [0x832607F0..0x83260830)
	// 832607F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832607F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832607F8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832607FC: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 83260800: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83260804: 388BF5F4  addi r4, r11, -0xa0c
	ctx.r[4].s64 = ctx.r[11].s64 + -2572;
	// 83260808: 386AB0F0  addi r3, r10, -0x4f10
	ctx.r[3].s64 = ctx.r[10].s64 + -20240;
	// 8326080C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83260810: 4AFCC6C1  bl 0x8222ced0
	ctx.lr = 0x83260814;
	sub_8222CED0(ctx, base);
	// 83260814: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83260818: 3869C880  addi r3, r9, -0x3780
	ctx.r[3].s64 = ctx.r[9].s64 + -14208;
	// 8326081C: 4BA49705  bl 0x82ca9f20
	ctx.lr = 0x83260820;
	sub_82CA9F20(ctx, base);
	// 83260820: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83260824: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83260828: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326082C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83260830(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83260830 size=64
    let mut pc: u32 = 0x83260830;
    'dispatch: loop {
        match pc {
            0x83260830 => {
    //   block [0x83260830..0x83260870)
	// 83260830: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83260834: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83260838: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326083C: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 83260840: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83260844: 388BF614  addi r4, r11, -0x9ec
	ctx.r[4].s64 = ctx.r[11].s64 + -2540;
	// 83260848: 386AB0F4  addi r3, r10, -0x4f0c
	ctx.r[3].s64 = ctx.r[10].s64 + -20236;
	// 8326084C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83260850: 4AFCC681  bl 0x8222ced0
	ctx.lr = 0x83260854;
	sub_8222CED0(ctx, base);
	// 83260854: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83260858: 3869C890  addi r3, r9, -0x3770
	ctx.r[3].s64 = ctx.r[9].s64 + -14192;
	// 8326085C: 4BA496C5  bl 0x82ca9f20
	ctx.lr = 0x83260860;
	sub_82CA9F20(ctx, base);
	// 83260860: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83260864: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83260868: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326086C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83260870(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83260870 size=64
    let mut pc: u32 = 0x83260870;
    'dispatch: loop {
        match pc {
            0x83260870 => {
    //   block [0x83260870..0x832608B0)
	// 83260870: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83260874: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83260878: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326087C: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 83260880: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83260884: 388BF638  addi r4, r11, -0x9c8
	ctx.r[4].s64 = ctx.r[11].s64 + -2504;
	// 83260888: 386AB0F8  addi r3, r10, -0x4f08
	ctx.r[3].s64 = ctx.r[10].s64 + -20232;
	// 8326088C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83260890: 4AFCC641  bl 0x8222ced0
	ctx.lr = 0x83260894;
	sub_8222CED0(ctx, base);
	// 83260894: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83260898: 3869C8A0  addi r3, r9, -0x3760
	ctx.r[3].s64 = ctx.r[9].s64 + -14176;
	// 8326089C: 4BA49685  bl 0x82ca9f20
	ctx.lr = 0x832608A0;
	sub_82CA9F20(ctx, base);
	// 832608A0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832608A4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832608A8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832608AC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832608B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832608B0 size=64
    let mut pc: u32 = 0x832608B0;
    'dispatch: loop {
        match pc {
            0x832608B0 => {
    //   block [0x832608B0..0x832608F0)
	// 832608B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832608B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832608B8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832608BC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 832608C0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832608C4: 388B1AD4  addi r4, r11, 0x1ad4
	ctx.r[4].s64 = ctx.r[11].s64 + 6868;
	// 832608C8: 386AB0FC  addi r3, r10, -0x4f04
	ctx.r[3].s64 = ctx.r[10].s64 + -20228;
	// 832608CC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832608D0: 4AFCC601  bl 0x8222ced0
	ctx.lr = 0x832608D4;
	sub_8222CED0(ctx, base);
	// 832608D4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 832608D8: 3869C8B0  addi r3, r9, -0x3750
	ctx.r[3].s64 = ctx.r[9].s64 + -14160;
	// 832608DC: 4BA49645  bl 0x82ca9f20
	ctx.lr = 0x832608E0;
	sub_82CA9F20(ctx, base);
	// 832608E0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832608E4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832608E8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832608EC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832608F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832608F0 size=64
    let mut pc: u32 = 0x832608F0;
    'dispatch: loop {
        match pc {
            0x832608F0 => {
    //   block [0x832608F0..0x83260930)
	// 832608F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832608F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832608F8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832608FC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83260900: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83260904: 388B1ADC  addi r4, r11, 0x1adc
	ctx.r[4].s64 = ctx.r[11].s64 + 6876;
	// 83260908: 386AB100  addi r3, r10, -0x4f00
	ctx.r[3].s64 = ctx.r[10].s64 + -20224;
	// 8326090C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83260910: 4AFCC5C1  bl 0x8222ced0
	ctx.lr = 0x83260914;
	sub_8222CED0(ctx, base);
	// 83260914: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83260918: 3869C8C0  addi r3, r9, -0x3740
	ctx.r[3].s64 = ctx.r[9].s64 + -14144;
	// 8326091C: 4BA49605  bl 0x82ca9f20
	ctx.lr = 0x83260920;
	sub_82CA9F20(ctx, base);
	// 83260920: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83260924: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83260928: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326092C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83260930(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83260930 size=64
    let mut pc: u32 = 0x83260930;
    'dispatch: loop {
        match pc {
            0x83260930 => {
    //   block [0x83260930..0x83260970)
	// 83260930: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83260934: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83260938: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326093C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83260940: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83260944: 388B1AF0  addi r4, r11, 0x1af0
	ctx.r[4].s64 = ctx.r[11].s64 + 6896;
	// 83260948: 386AB104  addi r3, r10, -0x4efc
	ctx.r[3].s64 = ctx.r[10].s64 + -20220;
	// 8326094C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83260950: 4AFCC581  bl 0x8222ced0
	ctx.lr = 0x83260954;
	sub_8222CED0(ctx, base);
	// 83260954: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83260958: 3869C8D0  addi r3, r9, -0x3730
	ctx.r[3].s64 = ctx.r[9].s64 + -14128;
	// 8326095C: 4BA495C5  bl 0x82ca9f20
	ctx.lr = 0x83260960;
	sub_82CA9F20(ctx, base);
	// 83260960: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83260964: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83260968: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326096C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83260970(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83260970 size=56
    let mut pc: u32 = 0x83260970;
    'dispatch: loop {
        match pc {
            0x83260970 => {
    //   block [0x83260970..0x832609A8)
	// 83260970: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83260974: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83260978: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326097C: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 83260980: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83260984: 386BF764  addi r3, r11, -0x89c
	ctx.r[3].s64 = ctx.r[11].s64 + -2204;
	// 83260988: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8326098C: 4AF933CD  bl 0x821f3d58
	ctx.lr = 0x83260990;
	sub_821F3D58(ctx, base);
	// 83260990: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83260994: 906AB108  stw r3, -0x4ef8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-20216 as u32), ctx.r[3].u32 ) };
	// 83260998: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326099C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832609A0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832609A4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832609A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832609A8 size=56
    let mut pc: u32 = 0x832609A8;
    'dispatch: loop {
        match pc {
            0x832609A8 => {
    //   block [0x832609A8..0x832609E0)
	// 832609A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832609AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832609B0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832609B4: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 832609B8: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 832609BC: 386BF770  addi r3, r11, -0x890
	ctx.r[3].s64 = ctx.r[11].s64 + -2192;
	// 832609C0: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 832609C4: 4AF93395  bl 0x821f3d58
	ctx.lr = 0x832609C8;
	sub_821F3D58(ctx, base);
	// 832609C8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832609CC: 906AB10C  stw r3, -0x4ef4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-20212 as u32), ctx.r[3].u32 ) };
	// 832609D0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832609D4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832609D8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832609DC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832609E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832609E0 size=56
    let mut pc: u32 = 0x832609E0;
    'dispatch: loop {
        match pc {
            0x832609E0 => {
    //   block [0x832609E0..0x83260A18)
	// 832609E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832609E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832609E8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832609EC: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 832609F0: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 832609F4: 386BF77C  addi r3, r11, -0x884
	ctx.r[3].s64 = ctx.r[11].s64 + -2180;
	// 832609F8: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 832609FC: 4AF9335D  bl 0x821f3d58
	ctx.lr = 0x83260A00;
	sub_821F3D58(ctx, base);
	// 83260A00: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83260A04: 906AB110  stw r3, -0x4ef0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-20208 as u32), ctx.r[3].u32 ) };
	// 83260A08: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83260A0C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83260A10: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83260A14: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83260A18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83260A18 size=56
    let mut pc: u32 = 0x83260A18;
    'dispatch: loop {
        match pc {
            0x83260A18 => {
    //   block [0x83260A18..0x83260A50)
	// 83260A18: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83260A1C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83260A20: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83260A24: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 83260A28: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83260A2C: 386BF788  addi r3, r11, -0x878
	ctx.r[3].s64 = ctx.r[11].s64 + -2168;
	// 83260A30: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83260A34: 4AF93325  bl 0x821f3d58
	ctx.lr = 0x83260A38;
	sub_821F3D58(ctx, base);
	// 83260A38: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83260A3C: 906AB114  stw r3, -0x4eec(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-20204 as u32), ctx.r[3].u32 ) };
	// 83260A40: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83260A44: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83260A48: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83260A4C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83260A50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83260A50 size=56
    let mut pc: u32 = 0x83260A50;
    'dispatch: loop {
        match pc {
            0x83260A50 => {
    //   block [0x83260A50..0x83260A88)
	// 83260A50: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83260A54: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83260A58: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83260A5C: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 83260A60: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83260A64: 386BF794  addi r3, r11, -0x86c
	ctx.r[3].s64 = ctx.r[11].s64 + -2156;
	// 83260A68: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83260A6C: 4AF932ED  bl 0x821f3d58
	ctx.lr = 0x83260A70;
	sub_821F3D58(ctx, base);
	// 83260A70: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83260A74: 906AB118  stw r3, -0x4ee8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-20200 as u32), ctx.r[3].u32 ) };
	// 83260A78: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83260A7C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83260A80: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83260A84: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


