pub fn sub_832722D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832722D8 size=56
    let mut pc: u32 = 0x832722D8;
    'dispatch: loop {
        match pc {
            0x832722D8 => {
    //   block [0x832722D8..0x83272310)
	// 832722D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832722DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832722E0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832722E4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 832722E8: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 832722EC: 386B43B4  addi r3, r11, 0x43b4
	ctx.r[3].s64 = ctx.r[11].s64 + 17332;
	// 832722F0: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 832722F4: 4AF81A65  bl 0x821f3d58
	ctx.lr = 0x832722F8;
	sub_821F3D58(ctx, base);
	// 832722F8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832722FC: 906AD018  stw r3, -0x2fe8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-12264 as u32), ctx.r[3].u32 ) };
	// 83272300: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83272304: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83272308: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327230C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83272310(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83272310 size=56
    let mut pc: u32 = 0x83272310;
    'dispatch: loop {
        match pc {
            0x83272310 => {
    //   block [0x83272310..0x83272348)
	// 83272310: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83272314: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83272318: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327231C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83272320: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83272324: 386B43C8  addi r3, r11, 0x43c8
	ctx.r[3].s64 = ctx.r[11].s64 + 17352;
	// 83272328: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8327232C: 4AF81A2D  bl 0x821f3d58
	ctx.lr = 0x83272330;
	sub_821F3D58(ctx, base);
	// 83272330: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83272334: 906AD01C  stw r3, -0x2fe4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-12260 as u32), ctx.r[3].u32 ) };
	// 83272338: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327233C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83272340: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83272344: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83272348(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83272348 size=56
    let mut pc: u32 = 0x83272348;
    'dispatch: loop {
        match pc {
            0x83272348 => {
    //   block [0x83272348..0x83272380)
	// 83272348: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327234C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83272350: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83272354: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83272358: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8327235C: 386B43D4  addi r3, r11, 0x43d4
	ctx.r[3].s64 = ctx.r[11].s64 + 17364;
	// 83272360: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83272364: 4AF819F5  bl 0x821f3d58
	ctx.lr = 0x83272368;
	sub_821F3D58(ctx, base);
	// 83272368: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327236C: 906AD020  stw r3, -0x2fe0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-12256 as u32), ctx.r[3].u32 ) };
	// 83272370: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83272374: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83272378: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327237C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83272380(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83272380 size=56
    let mut pc: u32 = 0x83272380;
    'dispatch: loop {
        match pc {
            0x83272380 => {
    //   block [0x83272380..0x832723B8)
	// 83272380: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83272384: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83272388: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327238C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83272390: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83272394: 386B43E0  addi r3, r11, 0x43e0
	ctx.r[3].s64 = ctx.r[11].s64 + 17376;
	// 83272398: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8327239C: 4AF819BD  bl 0x821f3d58
	ctx.lr = 0x832723A0;
	sub_821F3D58(ctx, base);
	// 832723A0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832723A4: 906AD024  stw r3, -0x2fdc(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-12252 as u32), ctx.r[3].u32 ) };
	// 832723A8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832723AC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832723B0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832723B4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832723B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832723B8 size=56
    let mut pc: u32 = 0x832723B8;
    'dispatch: loop {
        match pc {
            0x832723B8 => {
    //   block [0x832723B8..0x832723F0)
	// 832723B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832723BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832723C0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832723C4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 832723C8: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 832723CC: 386B43F0  addi r3, r11, 0x43f0
	ctx.r[3].s64 = ctx.r[11].s64 + 17392;
	// 832723D0: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 832723D4: 4AF81985  bl 0x821f3d58
	ctx.lr = 0x832723D8;
	sub_821F3D58(ctx, base);
	// 832723D8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832723DC: 906AD028  stw r3, -0x2fd8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-12248 as u32), ctx.r[3].u32 ) };
	// 832723E0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832723E4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832723E8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832723EC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832723F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832723F0 size=56
    let mut pc: u32 = 0x832723F0;
    'dispatch: loop {
        match pc {
            0x832723F0 => {
    //   block [0x832723F0..0x83272428)
	// 832723F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832723F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832723F8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832723FC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83272400: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83272404: 386B4400  addi r3, r11, 0x4400
	ctx.r[3].s64 = ctx.r[11].s64 + 17408;
	// 83272408: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8327240C: 4AF8194D  bl 0x821f3d58
	ctx.lr = 0x83272410;
	sub_821F3D58(ctx, base);
	// 83272410: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83272414: 906AD02C  stw r3, -0x2fd4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-12244 as u32), ctx.r[3].u32 ) };
	// 83272418: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327241C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83272420: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83272424: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83272428(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83272428 size=56
    let mut pc: u32 = 0x83272428;
    'dispatch: loop {
        match pc {
            0x83272428 => {
    //   block [0x83272428..0x83272460)
	// 83272428: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327242C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83272430: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83272434: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83272438: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8327243C: 386B4418  addi r3, r11, 0x4418
	ctx.r[3].s64 = ctx.r[11].s64 + 17432;
	// 83272440: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83272444: 4AF81915  bl 0x821f3d58
	ctx.lr = 0x83272448;
	sub_821F3D58(ctx, base);
	// 83272448: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327244C: 906AD030  stw r3, -0x2fd0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-12240 as u32), ctx.r[3].u32 ) };
	// 83272450: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83272454: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83272458: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327245C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83272460(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83272460 size=56
    let mut pc: u32 = 0x83272460;
    'dispatch: loop {
        match pc {
            0x83272460 => {
    //   block [0x83272460..0x83272498)
	// 83272460: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83272464: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83272468: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327246C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83272470: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83272474: 386B4424  addi r3, r11, 0x4424
	ctx.r[3].s64 = ctx.r[11].s64 + 17444;
	// 83272478: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8327247C: 4AF818DD  bl 0x821f3d58
	ctx.lr = 0x83272480;
	sub_821F3D58(ctx, base);
	// 83272480: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83272484: 906AD034  stw r3, -0x2fcc(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-12236 as u32), ctx.r[3].u32 ) };
	// 83272488: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327248C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83272490: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83272494: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83272498(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83272498 size=56
    let mut pc: u32 = 0x83272498;
    'dispatch: loop {
        match pc {
            0x83272498 => {
    //   block [0x83272498..0x832724D0)
	// 83272498: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327249C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832724A0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832724A4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 832724A8: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 832724AC: 386B4430  addi r3, r11, 0x4430
	ctx.r[3].s64 = ctx.r[11].s64 + 17456;
	// 832724B0: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 832724B4: 4AF818A5  bl 0x821f3d58
	ctx.lr = 0x832724B8;
	sub_821F3D58(ctx, base);
	// 832724B8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832724BC: 906AD038  stw r3, -0x2fc8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-12232 as u32), ctx.r[3].u32 ) };
	// 832724C0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832724C4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832724C8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832724CC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832724D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832724D0 size=56
    let mut pc: u32 = 0x832724D0;
    'dispatch: loop {
        match pc {
            0x832724D0 => {
    //   block [0x832724D0..0x83272508)
	// 832724D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832724D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832724D8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832724DC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 832724E0: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 832724E4: 386B4440  addi r3, r11, 0x4440
	ctx.r[3].s64 = ctx.r[11].s64 + 17472;
	// 832724E8: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 832724EC: 4AF8186D  bl 0x821f3d58
	ctx.lr = 0x832724F0;
	sub_821F3D58(ctx, base);
	// 832724F0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832724F4: 906AD03C  stw r3, -0x2fc4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-12228 as u32), ctx.r[3].u32 ) };
	// 832724F8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832724FC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83272500: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83272504: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83272508(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83272508 size=64
    let mut pc: u32 = 0x83272508;
    'dispatch: loop {
        match pc {
            0x83272508 => {
    //   block [0x83272508..0x83272548)
	// 83272508: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327250C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83272510: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83272514: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 83272518: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327251C: 388B9BEC  addi r4, r11, -0x6414
	ctx.r[4].s64 = ctx.r[11].s64 + -25620;
	// 83272520: 386AD040  addi r3, r10, -0x2fc0
	ctx.r[3].s64 = ctx.r[10].s64 + -12224;
	// 83272524: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83272528: 4AFBA9A9  bl 0x8222ced0
	ctx.lr = 0x8327252C;
	sub_8222CED0(ctx, base);
	// 8327252C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83272530: 3869F3E0  addi r3, r9, -0xc20
	ctx.r[3].s64 = ctx.r[9].s64 + -3104;
	// 83272534: 4BA379ED  bl 0x82ca9f20
	ctx.lr = 0x83272538;
	sub_82CA9F20(ctx, base);
	// 83272538: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327253C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83272540: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83272544: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83272548(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83272548 size=64
    let mut pc: u32 = 0x83272548;
    'dispatch: loop {
        match pc {
            0x83272548 => {
    //   block [0x83272548..0x83272588)
	// 83272548: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327254C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83272550: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83272554: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 83272558: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327255C: 388BADBC  addi r4, r11, -0x5244
	ctx.r[4].s64 = ctx.r[11].s64 + -21060;
	// 83272560: 386AD044  addi r3, r10, -0x2fbc
	ctx.r[3].s64 = ctx.r[10].s64 + -12220;
	// 83272564: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83272568: 4AFBA969  bl 0x8222ced0
	ctx.lr = 0x8327256C;
	sub_8222CED0(ctx, base);
	// 8327256C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83272570: 3869F3F0  addi r3, r9, -0xc10
	ctx.r[3].s64 = ctx.r[9].s64 + -3088;
	// 83272574: 4BA379AD  bl 0x82ca9f20
	ctx.lr = 0x83272578;
	sub_82CA9F20(ctx, base);
	// 83272578: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327257C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83272580: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83272584: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83272588(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83272588 size=64
    let mut pc: u32 = 0x83272588;
    'dispatch: loop {
        match pc {
            0x83272588 => {
    //   block [0x83272588..0x832725C8)
	// 83272588: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327258C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83272590: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83272594: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 83272598: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327259C: 388BADC4  addi r4, r11, -0x523c
	ctx.r[4].s64 = ctx.r[11].s64 + -21052;
	// 832725A0: 386AD048  addi r3, r10, -0x2fb8
	ctx.r[3].s64 = ctx.r[10].s64 + -12216;
	// 832725A4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832725A8: 4AFBA929  bl 0x8222ced0
	ctx.lr = 0x832725AC;
	sub_8222CED0(ctx, base);
	// 832725AC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 832725B0: 3869F400  addi r3, r9, -0xc00
	ctx.r[3].s64 = ctx.r[9].s64 + -3072;
	// 832725B4: 4BA3796D  bl 0x82ca9f20
	ctx.lr = 0x832725B8;
	sub_82CA9F20(ctx, base);
	// 832725B8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832725BC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832725C0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832725C4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832725C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832725C8 size=64
    let mut pc: u32 = 0x832725C8;
    'dispatch: loop {
        match pc {
            0x832725C8 => {
    //   block [0x832725C8..0x83272608)
	// 832725C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832725CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832725D0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832725D4: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 832725D8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832725DC: 388BADD0  addi r4, r11, -0x5230
	ctx.r[4].s64 = ctx.r[11].s64 + -21040;
	// 832725E0: 386AD04C  addi r3, r10, -0x2fb4
	ctx.r[3].s64 = ctx.r[10].s64 + -12212;
	// 832725E4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832725E8: 4AFBA8E9  bl 0x8222ced0
	ctx.lr = 0x832725EC;
	sub_8222CED0(ctx, base);
	// 832725EC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 832725F0: 3869F410  addi r3, r9, -0xbf0
	ctx.r[3].s64 = ctx.r[9].s64 + -3056;
	// 832725F4: 4BA3792D  bl 0x82ca9f20
	ctx.lr = 0x832725F8;
	sub_82CA9F20(ctx, base);
	// 832725F8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832725FC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83272600: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83272604: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83272608(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83272608 size=64
    let mut pc: u32 = 0x83272608;
    'dispatch: loop {
        match pc {
            0x83272608 => {
    //   block [0x83272608..0x83272648)
	// 83272608: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327260C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83272610: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83272614: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 83272618: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327261C: 388BADDC  addi r4, r11, -0x5224
	ctx.r[4].s64 = ctx.r[11].s64 + -21028;
	// 83272620: 386AD050  addi r3, r10, -0x2fb0
	ctx.r[3].s64 = ctx.r[10].s64 + -12208;
	// 83272624: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83272628: 4AFBA8A9  bl 0x8222ced0
	ctx.lr = 0x8327262C;
	sub_8222CED0(ctx, base);
	// 8327262C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83272630: 3869F420  addi r3, r9, -0xbe0
	ctx.r[3].s64 = ctx.r[9].s64 + -3040;
	// 83272634: 4BA378ED  bl 0x82ca9f20
	ctx.lr = 0x83272638;
	sub_82CA9F20(ctx, base);
	// 83272638: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327263C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83272640: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83272644: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83272648(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83272648 size=64
    let mut pc: u32 = 0x83272648;
    'dispatch: loop {
        match pc {
            0x83272648 => {
    //   block [0x83272648..0x83272688)
	// 83272648: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327264C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83272650: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83272654: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 83272658: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327265C: 388BADE8  addi r4, r11, -0x5218
	ctx.r[4].s64 = ctx.r[11].s64 + -21016;
	// 83272660: 386AD054  addi r3, r10, -0x2fac
	ctx.r[3].s64 = ctx.r[10].s64 + -12204;
	// 83272664: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83272668: 4AFBA869  bl 0x8222ced0
	ctx.lr = 0x8327266C;
	sub_8222CED0(ctx, base);
	// 8327266C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83272670: 3869F430  addi r3, r9, -0xbd0
	ctx.r[3].s64 = ctx.r[9].s64 + -3024;
	// 83272674: 4BA378AD  bl 0x82ca9f20
	ctx.lr = 0x83272678;
	sub_82CA9F20(ctx, base);
	// 83272678: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327267C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83272680: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83272684: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83272688(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83272688 size=64
    let mut pc: u32 = 0x83272688;
    'dispatch: loop {
        match pc {
            0x83272688 => {
    //   block [0x83272688..0x832726C8)
	// 83272688: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327268C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83272690: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83272694: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 83272698: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327269C: 388BADF4  addi r4, r11, -0x520c
	ctx.r[4].s64 = ctx.r[11].s64 + -21004;
	// 832726A0: 386AD058  addi r3, r10, -0x2fa8
	ctx.r[3].s64 = ctx.r[10].s64 + -12200;
	// 832726A4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832726A8: 4AFBA829  bl 0x8222ced0
	ctx.lr = 0x832726AC;
	sub_8222CED0(ctx, base);
	// 832726AC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 832726B0: 3869F440  addi r3, r9, -0xbc0
	ctx.r[3].s64 = ctx.r[9].s64 + -3008;
	// 832726B4: 4BA3786D  bl 0x82ca9f20
	ctx.lr = 0x832726B8;
	sub_82CA9F20(ctx, base);
	// 832726B8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832726BC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832726C0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832726C4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832726C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832726C8 size=64
    let mut pc: u32 = 0x832726C8;
    'dispatch: loop {
        match pc {
            0x832726C8 => {
    //   block [0x832726C8..0x83272708)
	// 832726C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832726CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832726D0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832726D4: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 832726D8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832726DC: 388BAE00  addi r4, r11, -0x5200
	ctx.r[4].s64 = ctx.r[11].s64 + -20992;
	// 832726E0: 386AD05C  addi r3, r10, -0x2fa4
	ctx.r[3].s64 = ctx.r[10].s64 + -12196;
	// 832726E4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832726E8: 4AFBA7E9  bl 0x8222ced0
	ctx.lr = 0x832726EC;
	sub_8222CED0(ctx, base);
	// 832726EC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 832726F0: 3869F450  addi r3, r9, -0xbb0
	ctx.r[3].s64 = ctx.r[9].s64 + -2992;
	// 832726F4: 4BA3782D  bl 0x82ca9f20
	ctx.lr = 0x832726F8;
	sub_82CA9F20(ctx, base);
	// 832726F8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832726FC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83272700: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83272704: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83272708(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83272708 size=64
    let mut pc: u32 = 0x83272708;
    'dispatch: loop {
        match pc {
            0x83272708 => {
    //   block [0x83272708..0x83272748)
	// 83272708: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327270C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83272710: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83272714: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 83272718: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327271C: 388B29F4  addi r4, r11, 0x29f4
	ctx.r[4].s64 = ctx.r[11].s64 + 10740;
	// 83272720: 386AD060  addi r3, r10, -0x2fa0
	ctx.r[3].s64 = ctx.r[10].s64 + -12192;
	// 83272724: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83272728: 4AFBA7A9  bl 0x8222ced0
	ctx.lr = 0x8327272C;
	sub_8222CED0(ctx, base);
	// 8327272C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83272730: 3869F460  addi r3, r9, -0xba0
	ctx.r[3].s64 = ctx.r[9].s64 + -2976;
	// 83272734: 4BA377ED  bl 0x82ca9f20
	ctx.lr = 0x83272738;
	sub_82CA9F20(ctx, base);
	// 83272738: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327273C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83272740: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83272744: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83272748(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83272748 size=56
    let mut pc: u32 = 0x83272748;
    'dispatch: loop {
        match pc {
            0x83272748 => {
    //   block [0x83272748..0x83272780)
	// 83272748: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327274C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83272750: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83272754: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83272758: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8327275C: 386B42C8  addi r3, r11, 0x42c8
	ctx.r[3].s64 = ctx.r[11].s64 + 17096;
	// 83272760: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83272764: 4AF815F5  bl 0x821f3d58
	ctx.lr = 0x83272768;
	sub_821F3D58(ctx, base);
	// 83272768: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327276C: 906AD064  stw r3, -0x2f9c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-12188 as u32), ctx.r[3].u32 ) };
	// 83272770: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83272774: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83272778: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327277C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83272780(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83272780 size=56
    let mut pc: u32 = 0x83272780;
    'dispatch: loop {
        match pc {
            0x83272780 => {
    //   block [0x83272780..0x832727B8)
	// 83272780: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83272784: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83272788: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327278C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83272790: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83272794: 386B42DC  addi r3, r11, 0x42dc
	ctx.r[3].s64 = ctx.r[11].s64 + 17116;
	// 83272798: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8327279C: 4AF815BD  bl 0x821f3d58
	ctx.lr = 0x832727A0;
	sub_821F3D58(ctx, base);
	// 832727A0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832727A4: 906AD068  stw r3, -0x2f98(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-12184 as u32), ctx.r[3].u32 ) };
	// 832727A8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832727AC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832727B0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832727B4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832727B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832727B8 size=56
    let mut pc: u32 = 0x832727B8;
    'dispatch: loop {
        match pc {
            0x832727B8 => {
    //   block [0x832727B8..0x832727F0)
	// 832727B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832727BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832727C0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832727C4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 832727C8: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 832727CC: 386B42E8  addi r3, r11, 0x42e8
	ctx.r[3].s64 = ctx.r[11].s64 + 17128;
	// 832727D0: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 832727D4: 4AF81585  bl 0x821f3d58
	ctx.lr = 0x832727D8;
	sub_821F3D58(ctx, base);
	// 832727D8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832727DC: 906AD06C  stw r3, -0x2f94(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-12180 as u32), ctx.r[3].u32 ) };
	// 832727E0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832727E4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832727E8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832727EC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832727F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832727F0 size=56
    let mut pc: u32 = 0x832727F0;
    'dispatch: loop {
        match pc {
            0x832727F0 => {
    //   block [0x832727F0..0x83272828)
	// 832727F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832727F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832727F8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832727FC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83272800: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83272804: 386B4300  addi r3, r11, 0x4300
	ctx.r[3].s64 = ctx.r[11].s64 + 17152;
	// 83272808: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8327280C: 4AF8154D  bl 0x821f3d58
	ctx.lr = 0x83272810;
	sub_821F3D58(ctx, base);
	// 83272810: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83272814: 906AD070  stw r3, -0x2f90(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-12176 as u32), ctx.r[3].u32 ) };
	// 83272818: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327281C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83272820: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83272824: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83272828(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83272828 size=56
    let mut pc: u32 = 0x83272828;
    'dispatch: loop {
        match pc {
            0x83272828 => {
    //   block [0x83272828..0x83272860)
	// 83272828: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327282C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83272830: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83272834: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83272838: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8327283C: 386B4310  addi r3, r11, 0x4310
	ctx.r[3].s64 = ctx.r[11].s64 + 17168;
	// 83272840: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83272844: 4AF81515  bl 0x821f3d58
	ctx.lr = 0x83272848;
	sub_821F3D58(ctx, base);
	// 83272848: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327284C: 906AD074  stw r3, -0x2f8c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-12172 as u32), ctx.r[3].u32 ) };
	// 83272850: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83272854: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83272858: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327285C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83272860(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83272860 size=56
    let mut pc: u32 = 0x83272860;
    'dispatch: loop {
        match pc {
            0x83272860 => {
    //   block [0x83272860..0x83272898)
	// 83272860: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83272864: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83272868: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327286C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83272870: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83272874: 386B4328  addi r3, r11, 0x4328
	ctx.r[3].s64 = ctx.r[11].s64 + 17192;
	// 83272878: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8327287C: 4AF814DD  bl 0x821f3d58
	ctx.lr = 0x83272880;
	sub_821F3D58(ctx, base);
	// 83272880: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83272884: 906AD078  stw r3, -0x2f88(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-12168 as u32), ctx.r[3].u32 ) };
	// 83272888: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327288C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83272890: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83272894: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83272898(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83272898 size=56
    let mut pc: u32 = 0x83272898;
    'dispatch: loop {
        match pc {
            0x83272898 => {
    //   block [0x83272898..0x832728D0)
	// 83272898: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327289C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832728A0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832728A4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 832728A8: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 832728AC: 386B4338  addi r3, r11, 0x4338
	ctx.r[3].s64 = ctx.r[11].s64 + 17208;
	// 832728B0: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 832728B4: 4AF814A5  bl 0x821f3d58
	ctx.lr = 0x832728B8;
	sub_821F3D58(ctx, base);
	// 832728B8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832728BC: 906AD07C  stw r3, -0x2f84(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-12164 as u32), ctx.r[3].u32 ) };
	// 832728C0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832728C4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832728C8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832728CC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832728D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832728D0 size=56
    let mut pc: u32 = 0x832728D0;
    'dispatch: loop {
        match pc {
            0x832728D0 => {
    //   block [0x832728D0..0x83272908)
	// 832728D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832728D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832728D8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832728DC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 832728E0: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 832728E4: 386B4350  addi r3, r11, 0x4350
	ctx.r[3].s64 = ctx.r[11].s64 + 17232;
	// 832728E8: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 832728EC: 4AF8146D  bl 0x821f3d58
	ctx.lr = 0x832728F0;
	sub_821F3D58(ctx, base);
	// 832728F0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832728F4: 906AD080  stw r3, -0x2f80(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-12160 as u32), ctx.r[3].u32 ) };
	// 832728F8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832728FC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83272900: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83272904: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83272908(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83272908 size=56
    let mut pc: u32 = 0x83272908;
    'dispatch: loop {
        match pc {
            0x83272908 => {
    //   block [0x83272908..0x83272940)
	// 83272908: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327290C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83272910: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83272914: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83272918: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8327291C: 386B4368  addi r3, r11, 0x4368
	ctx.r[3].s64 = ctx.r[11].s64 + 17256;
	// 83272920: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83272924: 4AF81435  bl 0x821f3d58
	ctx.lr = 0x83272928;
	sub_821F3D58(ctx, base);
	// 83272928: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327292C: 906AD084  stw r3, -0x2f7c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-12156 as u32), ctx.r[3].u32 ) };
	// 83272930: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83272934: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83272938: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327293C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83272940(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83272940 size=56
    let mut pc: u32 = 0x83272940;
    'dispatch: loop {
        match pc {
            0x83272940 => {
    //   block [0x83272940..0x83272978)
	// 83272940: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83272944: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83272948: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327294C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83272950: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83272954: 386B4380  addi r3, r11, 0x4380
	ctx.r[3].s64 = ctx.r[11].s64 + 17280;
	// 83272958: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8327295C: 4AF813FD  bl 0x821f3d58
	ctx.lr = 0x83272960;
	sub_821F3D58(ctx, base);
	// 83272960: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83272964: 906AD088  stw r3, -0x2f78(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-12152 as u32), ctx.r[3].u32 ) };
	// 83272968: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327296C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83272970: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83272974: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83272978(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83272978 size=56
    let mut pc: u32 = 0x83272978;
    'dispatch: loop {
        match pc {
            0x83272978 => {
    //   block [0x83272978..0x832729B0)
	// 83272978: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327297C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83272980: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83272984: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83272988: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8327298C: 386B4394  addi r3, r11, 0x4394
	ctx.r[3].s64 = ctx.r[11].s64 + 17300;
	// 83272990: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83272994: 4AF813C5  bl 0x821f3d58
	ctx.lr = 0x83272998;
	sub_821F3D58(ctx, base);
	// 83272998: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327299C: 906AD08C  stw r3, -0x2f74(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-12148 as u32), ctx.r[3].u32 ) };
	// 832729A0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832729A4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832729A8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832729AC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832729B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832729B0 size=56
    let mut pc: u32 = 0x832729B0;
    'dispatch: loop {
        match pc {
            0x832729B0 => {
    //   block [0x832729B0..0x832729E8)
	// 832729B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832729B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832729B8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832729BC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 832729C0: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 832729C4: 386B43B4  addi r3, r11, 0x43b4
	ctx.r[3].s64 = ctx.r[11].s64 + 17332;
	// 832729C8: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 832729CC: 4AF8138D  bl 0x821f3d58
	ctx.lr = 0x832729D0;
	sub_821F3D58(ctx, base);
	// 832729D0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832729D4: 906AD090  stw r3, -0x2f70(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-12144 as u32), ctx.r[3].u32 ) };
	// 832729D8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832729DC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832729E0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832729E4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832729E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832729E8 size=56
    let mut pc: u32 = 0x832729E8;
    'dispatch: loop {
        match pc {
            0x832729E8 => {
    //   block [0x832729E8..0x83272A20)
	// 832729E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832729EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832729F0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832729F4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 832729F8: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 832729FC: 386B43C8  addi r3, r11, 0x43c8
	ctx.r[3].s64 = ctx.r[11].s64 + 17352;
	// 83272A00: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83272A04: 4AF81355  bl 0x821f3d58
	ctx.lr = 0x83272A08;
	sub_821F3D58(ctx, base);
	// 83272A08: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83272A0C: 906AD094  stw r3, -0x2f6c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-12140 as u32), ctx.r[3].u32 ) };
	// 83272A10: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83272A14: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83272A18: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83272A1C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83272A20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83272A20 size=56
    let mut pc: u32 = 0x83272A20;
    'dispatch: loop {
        match pc {
            0x83272A20 => {
    //   block [0x83272A20..0x83272A58)
	// 83272A20: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83272A24: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83272A28: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83272A2C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83272A30: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83272A34: 386B43D4  addi r3, r11, 0x43d4
	ctx.r[3].s64 = ctx.r[11].s64 + 17364;
	// 83272A38: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83272A3C: 4AF8131D  bl 0x821f3d58
	ctx.lr = 0x83272A40;
	sub_821F3D58(ctx, base);
	// 83272A40: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83272A44: 906AD098  stw r3, -0x2f68(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-12136 as u32), ctx.r[3].u32 ) };
	// 83272A48: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83272A4C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83272A50: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83272A54: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83272A58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83272A58 size=56
    let mut pc: u32 = 0x83272A58;
    'dispatch: loop {
        match pc {
            0x83272A58 => {
    //   block [0x83272A58..0x83272A90)
	// 83272A58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83272A5C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83272A60: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83272A64: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83272A68: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83272A6C: 386B43E0  addi r3, r11, 0x43e0
	ctx.r[3].s64 = ctx.r[11].s64 + 17376;
	// 83272A70: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83272A74: 4AF812E5  bl 0x821f3d58
	ctx.lr = 0x83272A78;
	sub_821F3D58(ctx, base);
	// 83272A78: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83272A7C: 906AD09C  stw r3, -0x2f64(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-12132 as u32), ctx.r[3].u32 ) };
	// 83272A80: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83272A84: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83272A88: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83272A8C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83272A90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83272A90 size=56
    let mut pc: u32 = 0x83272A90;
    'dispatch: loop {
        match pc {
            0x83272A90 => {
    //   block [0x83272A90..0x83272AC8)
	// 83272A90: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83272A94: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83272A98: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83272A9C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83272AA0: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83272AA4: 386B43F0  addi r3, r11, 0x43f0
	ctx.r[3].s64 = ctx.r[11].s64 + 17392;
	// 83272AA8: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83272AAC: 4AF812AD  bl 0x821f3d58
	ctx.lr = 0x83272AB0;
	sub_821F3D58(ctx, base);
	// 83272AB0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83272AB4: 906AD0A0  stw r3, -0x2f60(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-12128 as u32), ctx.r[3].u32 ) };
	// 83272AB8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83272ABC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83272AC0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83272AC4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83272AC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83272AC8 size=56
    let mut pc: u32 = 0x83272AC8;
    'dispatch: loop {
        match pc {
            0x83272AC8 => {
    //   block [0x83272AC8..0x83272B00)
	// 83272AC8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83272ACC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83272AD0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83272AD4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83272AD8: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83272ADC: 386B4400  addi r3, r11, 0x4400
	ctx.r[3].s64 = ctx.r[11].s64 + 17408;
	// 83272AE0: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83272AE4: 4AF81275  bl 0x821f3d58
	ctx.lr = 0x83272AE8;
	sub_821F3D58(ctx, base);
	// 83272AE8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83272AEC: 906AD0A4  stw r3, -0x2f5c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-12124 as u32), ctx.r[3].u32 ) };
	// 83272AF0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83272AF4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83272AF8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83272AFC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83272B00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83272B00 size=56
    let mut pc: u32 = 0x83272B00;
    'dispatch: loop {
        match pc {
            0x83272B00 => {
    //   block [0x83272B00..0x83272B38)
	// 83272B00: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83272B04: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83272B08: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83272B0C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83272B10: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83272B14: 386B4418  addi r3, r11, 0x4418
	ctx.r[3].s64 = ctx.r[11].s64 + 17432;
	// 83272B18: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83272B1C: 4AF8123D  bl 0x821f3d58
	ctx.lr = 0x83272B20;
	sub_821F3D58(ctx, base);
	// 83272B20: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83272B24: 906AD0A8  stw r3, -0x2f58(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-12120 as u32), ctx.r[3].u32 ) };
	// 83272B28: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83272B2C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83272B30: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83272B34: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83272B38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83272B38 size=56
    let mut pc: u32 = 0x83272B38;
    'dispatch: loop {
        match pc {
            0x83272B38 => {
    //   block [0x83272B38..0x83272B70)
	// 83272B38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83272B3C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83272B40: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83272B44: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83272B48: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83272B4C: 386B4424  addi r3, r11, 0x4424
	ctx.r[3].s64 = ctx.r[11].s64 + 17444;
	// 83272B50: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83272B54: 4AF81205  bl 0x821f3d58
	ctx.lr = 0x83272B58;
	sub_821F3D58(ctx, base);
	// 83272B58: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83272B5C: 906AD0AC  stw r3, -0x2f54(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-12116 as u32), ctx.r[3].u32 ) };
	// 83272B60: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83272B64: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83272B68: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83272B6C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83272B70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83272B70 size=56
    let mut pc: u32 = 0x83272B70;
    'dispatch: loop {
        match pc {
            0x83272B70 => {
    //   block [0x83272B70..0x83272BA8)
	// 83272B70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83272B74: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83272B78: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83272B7C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83272B80: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83272B84: 386B4430  addi r3, r11, 0x4430
	ctx.r[3].s64 = ctx.r[11].s64 + 17456;
	// 83272B88: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83272B8C: 4AF811CD  bl 0x821f3d58
	ctx.lr = 0x83272B90;
	sub_821F3D58(ctx, base);
	// 83272B90: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83272B94: 906AD0B0  stw r3, -0x2f50(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-12112 as u32), ctx.r[3].u32 ) };
	// 83272B98: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83272B9C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83272BA0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83272BA4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83272BA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83272BA8 size=56
    let mut pc: u32 = 0x83272BA8;
    'dispatch: loop {
        match pc {
            0x83272BA8 => {
    //   block [0x83272BA8..0x83272BE0)
	// 83272BA8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83272BAC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83272BB0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83272BB4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83272BB8: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83272BBC: 386B4440  addi r3, r11, 0x4440
	ctx.r[3].s64 = ctx.r[11].s64 + 17472;
	// 83272BC0: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83272BC4: 4AF81195  bl 0x821f3d58
	ctx.lr = 0x83272BC8;
	sub_821F3D58(ctx, base);
	// 83272BC8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83272BCC: 906AD0B4  stw r3, -0x2f4c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-12108 as u32), ctx.r[3].u32 ) };
	// 83272BD0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83272BD4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83272BD8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83272BDC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83272BE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83272BE0 size=60
    let mut pc: u32 = 0x83272BE0;
    'dispatch: loop {
        match pc {
            0x83272BE0 => {
    //   block [0x83272BE0..0x83272C1C)
	// 83272BE0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83272BE4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83272BE8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83272BEC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83272BF0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83272BF4: 388B3120  addi r4, r11, 0x3120
	ctx.r[4].s64 = ctx.r[11].s64 + 12576;
	// 83272BF8: 386AD0B8  addi r3, r10, -0x2f48
	ctx.r[3].s64 = ctx.r[10].s64 + -12104;
	// 83272BFC: 4B06380D  bl 0x822d6408
	ctx.lr = 0x83272C00;
	sub_822D6408(ctx, base);
	// 83272C00: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83272C04: 3869F470  addi r3, r9, -0xb90
	ctx.r[3].s64 = ctx.r[9].s64 + -2960;
	// 83272C08: 4BA37319  bl 0x82ca9f20
	ctx.lr = 0x83272C0C;
	sub_82CA9F20(ctx, base);
	// 83272C0C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83272C10: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83272C14: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83272C18: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83272C20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83272C20 size=64
    let mut pc: u32 = 0x83272C20;
    'dispatch: loop {
        match pc {
            0x83272C20 => {
    //   block [0x83272C20..0x83272C60)
	// 83272C20: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83272C24: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83272C28: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83272C2C: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 83272C30: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83272C34: 388BB120  addi r4, r11, -0x4ee0
	ctx.r[4].s64 = ctx.r[11].s64 + -20192;
	// 83272C38: 386AD0BC  addi r3, r10, -0x2f44
	ctx.r[3].s64 = ctx.r[10].s64 + -12100;
	// 83272C3C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83272C40: 4AFBA291  bl 0x8222ced0
	ctx.lr = 0x83272C44;
	sub_8222CED0(ctx, base);
	// 83272C44: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83272C48: 3869F480  addi r3, r9, -0xb80
	ctx.r[3].s64 = ctx.r[9].s64 + -2944;
	// 83272C4C: 4BA372D5  bl 0x82ca9f20
	ctx.lr = 0x83272C50;
	sub_82CA9F20(ctx, base);
	// 83272C50: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83272C54: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83272C58: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83272C5C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83272C60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83272C60 size=64
    let mut pc: u32 = 0x83272C60;
    'dispatch: loop {
        match pc {
            0x83272C60 => {
    //   block [0x83272C60..0x83272CA0)
	// 83272C60: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83272C64: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83272C68: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83272C6C: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 83272C70: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83272C74: 388BB14C  addi r4, r11, -0x4eb4
	ctx.r[4].s64 = ctx.r[11].s64 + -20148;
	// 83272C78: 386AD0C0  addi r3, r10, -0x2f40
	ctx.r[3].s64 = ctx.r[10].s64 + -12096;
	// 83272C7C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83272C80: 4AFBA251  bl 0x8222ced0
	ctx.lr = 0x83272C84;
	sub_8222CED0(ctx, base);
	// 83272C84: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83272C88: 3869F490  addi r3, r9, -0xb70
	ctx.r[3].s64 = ctx.r[9].s64 + -2928;
	// 83272C8C: 4BA37295  bl 0x82ca9f20
	ctx.lr = 0x83272C90;
	sub_82CA9F20(ctx, base);
	// 83272C90: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83272C94: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83272C98: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83272C9C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83272CA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83272CA0 size=64
    let mut pc: u32 = 0x83272CA0;
    'dispatch: loop {
        match pc {
            0x83272CA0 => {
    //   block [0x83272CA0..0x83272CE0)
	// 83272CA0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83272CA4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83272CA8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83272CAC: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 83272CB0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83272CB4: 388BB178  addi r4, r11, -0x4e88
	ctx.r[4].s64 = ctx.r[11].s64 + -20104;
	// 83272CB8: 386AD0C4  addi r3, r10, -0x2f3c
	ctx.r[3].s64 = ctx.r[10].s64 + -12092;
	// 83272CBC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83272CC0: 4AFBA211  bl 0x8222ced0
	ctx.lr = 0x83272CC4;
	sub_8222CED0(ctx, base);
	// 83272CC4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83272CC8: 3869F4A0  addi r3, r9, -0xb60
	ctx.r[3].s64 = ctx.r[9].s64 + -2912;
	// 83272CCC: 4BA37255  bl 0x82ca9f20
	ctx.lr = 0x83272CD0;
	sub_82CA9F20(ctx, base);
	// 83272CD0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83272CD4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83272CD8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83272CDC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83272CE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83272CE0 size=64
    let mut pc: u32 = 0x83272CE0;
    'dispatch: loop {
        match pc {
            0x83272CE0 => {
    //   block [0x83272CE0..0x83272D20)
	// 83272CE0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83272CE4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83272CE8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83272CEC: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 83272CF0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83272CF4: 388BB1A4  addi r4, r11, -0x4e5c
	ctx.r[4].s64 = ctx.r[11].s64 + -20060;
	// 83272CF8: 386AD0C8  addi r3, r10, -0x2f38
	ctx.r[3].s64 = ctx.r[10].s64 + -12088;
	// 83272CFC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83272D00: 4AFBA1D1  bl 0x8222ced0
	ctx.lr = 0x83272D04;
	sub_8222CED0(ctx, base);
	// 83272D04: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83272D08: 3869F4B0  addi r3, r9, -0xb50
	ctx.r[3].s64 = ctx.r[9].s64 + -2896;
	// 83272D0C: 4BA37215  bl 0x82ca9f20
	ctx.lr = 0x83272D10;
	sub_82CA9F20(ctx, base);
	// 83272D10: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83272D14: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83272D18: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83272D1C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83272D20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83272D20 size=64
    let mut pc: u32 = 0x83272D20;
    'dispatch: loop {
        match pc {
            0x83272D20 => {
    //   block [0x83272D20..0x83272D60)
	// 83272D20: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83272D24: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83272D28: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83272D2C: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 83272D30: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83272D34: 388BB1D0  addi r4, r11, -0x4e30
	ctx.r[4].s64 = ctx.r[11].s64 + -20016;
	// 83272D38: 386AD0CC  addi r3, r10, -0x2f34
	ctx.r[3].s64 = ctx.r[10].s64 + -12084;
	// 83272D3C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83272D40: 4AFBA191  bl 0x8222ced0
	ctx.lr = 0x83272D44;
	sub_8222CED0(ctx, base);
	// 83272D44: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83272D48: 3869F4C0  addi r3, r9, -0xb40
	ctx.r[3].s64 = ctx.r[9].s64 + -2880;
	// 83272D4C: 4BA371D5  bl 0x82ca9f20
	ctx.lr = 0x83272D50;
	sub_82CA9F20(ctx, base);
	// 83272D50: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83272D54: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83272D58: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83272D5C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83272D60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83272D60 size=64
    let mut pc: u32 = 0x83272D60;
    'dispatch: loop {
        match pc {
            0x83272D60 => {
    //   block [0x83272D60..0x83272DA0)
	// 83272D60: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83272D64: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83272D68: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83272D6C: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 83272D70: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83272D74: 388BB1F8  addi r4, r11, -0x4e08
	ctx.r[4].s64 = ctx.r[11].s64 + -19976;
	// 83272D78: 386AD0D0  addi r3, r10, -0x2f30
	ctx.r[3].s64 = ctx.r[10].s64 + -12080;
	// 83272D7C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83272D80: 4AFBA151  bl 0x8222ced0
	ctx.lr = 0x83272D84;
	sub_8222CED0(ctx, base);
	// 83272D84: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83272D88: 3869F4D0  addi r3, r9, -0xb30
	ctx.r[3].s64 = ctx.r[9].s64 + -2864;
	// 83272D8C: 4BA37195  bl 0x82ca9f20
	ctx.lr = 0x83272D90;
	sub_82CA9F20(ctx, base);
	// 83272D90: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83272D94: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83272D98: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83272D9C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83272DA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83272DA0 size=64
    let mut pc: u32 = 0x83272DA0;
    'dispatch: loop {
        match pc {
            0x83272DA0 => {
    //   block [0x83272DA0..0x83272DE0)
	// 83272DA0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83272DA4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83272DA8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83272DAC: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 83272DB0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83272DB4: 388BB22C  addi r4, r11, -0x4dd4
	ctx.r[4].s64 = ctx.r[11].s64 + -19924;
	// 83272DB8: 386AD0D4  addi r3, r10, -0x2f2c
	ctx.r[3].s64 = ctx.r[10].s64 + -12076;
	// 83272DBC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83272DC0: 4AFBA111  bl 0x8222ced0
	ctx.lr = 0x83272DC4;
	sub_8222CED0(ctx, base);
	// 83272DC4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83272DC8: 3869F4E0  addi r3, r9, -0xb20
	ctx.r[3].s64 = ctx.r[9].s64 + -2848;
	// 83272DCC: 4BA37155  bl 0x82ca9f20
	ctx.lr = 0x83272DD0;
	sub_82CA9F20(ctx, base);
	// 83272DD0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83272DD4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83272DD8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83272DDC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83272DE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83272DE0 size=64
    let mut pc: u32 = 0x83272DE0;
    'dispatch: loop {
        match pc {
            0x83272DE0 => {
    //   block [0x83272DE0..0x83272E20)
	// 83272DE0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83272DE4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83272DE8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83272DEC: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 83272DF0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83272DF4: 388BB260  addi r4, r11, -0x4da0
	ctx.r[4].s64 = ctx.r[11].s64 + -19872;
	// 83272DF8: 386AD0D8  addi r3, r10, -0x2f28
	ctx.r[3].s64 = ctx.r[10].s64 + -12072;
	// 83272DFC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83272E00: 4AFBA0D1  bl 0x8222ced0
	ctx.lr = 0x83272E04;
	sub_8222CED0(ctx, base);
	// 83272E04: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83272E08: 3869F4F0  addi r3, r9, -0xb10
	ctx.r[3].s64 = ctx.r[9].s64 + -2832;
	// 83272E0C: 4BA37115  bl 0x82ca9f20
	ctx.lr = 0x83272E10;
	sub_82CA9F20(ctx, base);
	// 83272E10: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83272E14: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83272E18: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83272E1C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83272E20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83272E20 size=64
    let mut pc: u32 = 0x83272E20;
    'dispatch: loop {
        match pc {
            0x83272E20 => {
    //   block [0x83272E20..0x83272E60)
	// 83272E20: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83272E24: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83272E28: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83272E2C: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 83272E30: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83272E34: 388BB290  addi r4, r11, -0x4d70
	ctx.r[4].s64 = ctx.r[11].s64 + -19824;
	// 83272E38: 386AD0DC  addi r3, r10, -0x2f24
	ctx.r[3].s64 = ctx.r[10].s64 + -12068;
	// 83272E3C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83272E40: 4AFBA091  bl 0x8222ced0
	ctx.lr = 0x83272E44;
	sub_8222CED0(ctx, base);
	// 83272E44: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83272E48: 3869F500  addi r3, r9, -0xb00
	ctx.r[3].s64 = ctx.r[9].s64 + -2816;
	// 83272E4C: 4BA370D5  bl 0x82ca9f20
	ctx.lr = 0x83272E50;
	sub_82CA9F20(ctx, base);
	// 83272E50: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83272E54: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83272E58: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83272E5C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83272E60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83272E60 size=64
    let mut pc: u32 = 0x83272E60;
    'dispatch: loop {
        match pc {
            0x83272E60 => {
    //   block [0x83272E60..0x83272EA0)
	// 83272E60: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83272E64: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83272E68: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83272E6C: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 83272E70: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83272E74: 388BB2C4  addi r4, r11, -0x4d3c
	ctx.r[4].s64 = ctx.r[11].s64 + -19772;
	// 83272E78: 386AD0E0  addi r3, r10, -0x2f20
	ctx.r[3].s64 = ctx.r[10].s64 + -12064;
	// 83272E7C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83272E80: 4AFBA051  bl 0x8222ced0
	ctx.lr = 0x83272E84;
	sub_8222CED0(ctx, base);
	// 83272E84: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83272E88: 3869F510  addi r3, r9, -0xaf0
	ctx.r[3].s64 = ctx.r[9].s64 + -2800;
	// 83272E8C: 4BA37095  bl 0x82ca9f20
	ctx.lr = 0x83272E90;
	sub_82CA9F20(ctx, base);
	// 83272E90: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83272E94: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83272E98: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83272E9C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83272EA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83272EA0 size=64
    let mut pc: u32 = 0x83272EA0;
    'dispatch: loop {
        match pc {
            0x83272EA0 => {
    //   block [0x83272EA0..0x83272EE0)
	// 83272EA0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83272EA4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83272EA8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83272EAC: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 83272EB0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83272EB4: 388BB2F4  addi r4, r11, -0x4d0c
	ctx.r[4].s64 = ctx.r[11].s64 + -19724;
	// 83272EB8: 386AD0E4  addi r3, r10, -0x2f1c
	ctx.r[3].s64 = ctx.r[10].s64 + -12060;
	// 83272EBC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83272EC0: 4AFBA011  bl 0x8222ced0
	ctx.lr = 0x83272EC4;
	sub_8222CED0(ctx, base);
	// 83272EC4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83272EC8: 3869F520  addi r3, r9, -0xae0
	ctx.r[3].s64 = ctx.r[9].s64 + -2784;
	// 83272ECC: 4BA37055  bl 0x82ca9f20
	ctx.lr = 0x83272ED0;
	sub_82CA9F20(ctx, base);
	// 83272ED0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83272ED4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83272ED8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83272EDC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83272EE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83272EE0 size=64
    let mut pc: u32 = 0x83272EE0;
    'dispatch: loop {
        match pc {
            0x83272EE0 => {
    //   block [0x83272EE0..0x83272F20)
	// 83272EE0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83272EE4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83272EE8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83272EEC: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 83272EF0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83272EF4: 388BD4FC  addi r4, r11, -0x2b04
	ctx.r[4].s64 = ctx.r[11].s64 + -11012;
	// 83272EF8: 386AD0E8  addi r3, r10, -0x2f18
	ctx.r[3].s64 = ctx.r[10].s64 + -12056;
	// 83272EFC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83272F00: 4AFB9FD1  bl 0x8222ced0
	ctx.lr = 0x83272F04;
	sub_8222CED0(ctx, base);
	// 83272F04: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83272F08: 3869F530  addi r3, r9, -0xad0
	ctx.r[3].s64 = ctx.r[9].s64 + -2768;
	// 83272F0C: 4BA37015  bl 0x82ca9f20
	ctx.lr = 0x83272F10;
	sub_82CA9F20(ctx, base);
	// 83272F10: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83272F14: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83272F18: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83272F1C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83272F20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83272F20 size=64
    let mut pc: u32 = 0x83272F20;
    'dispatch: loop {
        match pc {
            0x83272F20 => {
    //   block [0x83272F20..0x83272F60)
	// 83272F20: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83272F24: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83272F28: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83272F2C: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 83272F30: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83272F34: 388BB320  addi r4, r11, -0x4ce0
	ctx.r[4].s64 = ctx.r[11].s64 + -19680;
	// 83272F38: 386AD0EC  addi r3, r10, -0x2f14
	ctx.r[3].s64 = ctx.r[10].s64 + -12052;
	// 83272F3C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83272F40: 4AFB9F91  bl 0x8222ced0
	ctx.lr = 0x83272F44;
	sub_8222CED0(ctx, base);
	// 83272F44: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83272F48: 3869F540  addi r3, r9, -0xac0
	ctx.r[3].s64 = ctx.r[9].s64 + -2752;
	// 83272F4C: 4BA36FD5  bl 0x82ca9f20
	ctx.lr = 0x83272F50;
	sub_82CA9F20(ctx, base);
	// 83272F50: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83272F54: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83272F58: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83272F5C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83272F60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83272F60 size=64
    let mut pc: u32 = 0x83272F60;
    'dispatch: loop {
        match pc {
            0x83272F60 => {
    //   block [0x83272F60..0x83272FA0)
	// 83272F60: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83272F64: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83272F68: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83272F6C: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 83272F70: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83272F74: 388B9ED8  addi r4, r11, -0x6128
	ctx.r[4].s64 = ctx.r[11].s64 + -24872;
	// 83272F78: 386AD0F0  addi r3, r10, -0x2f10
	ctx.r[3].s64 = ctx.r[10].s64 + -12048;
	// 83272F7C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83272F80: 4AFB9F51  bl 0x8222ced0
	ctx.lr = 0x83272F84;
	sub_8222CED0(ctx, base);
	// 83272F84: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83272F88: 3869F550  addi r3, r9, -0xab0
	ctx.r[3].s64 = ctx.r[9].s64 + -2736;
	// 83272F8C: 4BA36F95  bl 0x82ca9f20
	ctx.lr = 0x83272F90;
	sub_82CA9F20(ctx, base);
	// 83272F90: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83272F94: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83272F98: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83272F9C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83272FA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83272FA0 size=64
    let mut pc: u32 = 0x83272FA0;
    'dispatch: loop {
        match pc {
            0x83272FA0 => {
    //   block [0x83272FA0..0x83272FE0)
	// 83272FA0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83272FA4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83272FA8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83272FAC: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 83272FB0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83272FB4: 388BB344  addi r4, r11, -0x4cbc
	ctx.r[4].s64 = ctx.r[11].s64 + -19644;
	// 83272FB8: 386AD0F4  addi r3, r10, -0x2f0c
	ctx.r[3].s64 = ctx.r[10].s64 + -12044;
	// 83272FBC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83272FC0: 4AFB9F11  bl 0x8222ced0
	ctx.lr = 0x83272FC4;
	sub_8222CED0(ctx, base);
	// 83272FC4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83272FC8: 3869F560  addi r3, r9, -0xaa0
	ctx.r[3].s64 = ctx.r[9].s64 + -2720;
	// 83272FCC: 4BA36F55  bl 0x82ca9f20
	ctx.lr = 0x83272FD0;
	sub_82CA9F20(ctx, base);
	// 83272FD0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83272FD4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83272FD8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83272FDC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83272FE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83272FE0 size=64
    let mut pc: u32 = 0x83272FE0;
    'dispatch: loop {
        match pc {
            0x83272FE0 => {
    //   block [0x83272FE0..0x83273020)
	// 83272FE0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83272FE4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83272FE8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83272FEC: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 83272FF0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83272FF4: 388BB370  addi r4, r11, -0x4c90
	ctx.r[4].s64 = ctx.r[11].s64 + -19600;
	// 83272FF8: 386AD0F8  addi r3, r10, -0x2f08
	ctx.r[3].s64 = ctx.r[10].s64 + -12040;
	// 83272FFC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83273000: 4AFB9ED1  bl 0x8222ced0
	ctx.lr = 0x83273004;
	sub_8222CED0(ctx, base);
	// 83273004: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83273008: 3869F570  addi r3, r9, -0xa90
	ctx.r[3].s64 = ctx.r[9].s64 + -2704;
	// 8327300C: 4BA36F15  bl 0x82ca9f20
	ctx.lr = 0x83273010;
	sub_82CA9F20(ctx, base);
	// 83273010: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83273014: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83273018: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327301C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83273020(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83273020 size=64
    let mut pc: u32 = 0x83273020;
    'dispatch: loop {
        match pc {
            0x83273020 => {
    //   block [0x83273020..0x83273060)
	// 83273020: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83273024: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83273028: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327302C: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 83273030: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83273034: 388BB39C  addi r4, r11, -0x4c64
	ctx.r[4].s64 = ctx.r[11].s64 + -19556;
	// 83273038: 386AD0FC  addi r3, r10, -0x2f04
	ctx.r[3].s64 = ctx.r[10].s64 + -12036;
	// 8327303C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83273040: 4AFB9E91  bl 0x8222ced0
	ctx.lr = 0x83273044;
	sub_8222CED0(ctx, base);
	// 83273044: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83273048: 3869F580  addi r3, r9, -0xa80
	ctx.r[3].s64 = ctx.r[9].s64 + -2688;
	// 8327304C: 4BA36ED5  bl 0x82ca9f20
	ctx.lr = 0x83273050;
	sub_82CA9F20(ctx, base);
	// 83273050: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83273054: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83273058: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327305C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83273060(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83273060 size=64
    let mut pc: u32 = 0x83273060;
    'dispatch: loop {
        match pc {
            0x83273060 => {
    //   block [0x83273060..0x832730A0)
	// 83273060: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83273064: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83273068: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327306C: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 83273070: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83273074: 388BB3C8  addi r4, r11, -0x4c38
	ctx.r[4].s64 = ctx.r[11].s64 + -19512;
	// 83273078: 386AD100  addi r3, r10, -0x2f00
	ctx.r[3].s64 = ctx.r[10].s64 + -12032;
	// 8327307C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83273080: 4AFB9E51  bl 0x8222ced0
	ctx.lr = 0x83273084;
	sub_8222CED0(ctx, base);
	// 83273084: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83273088: 3869F590  addi r3, r9, -0xa70
	ctx.r[3].s64 = ctx.r[9].s64 + -2672;
	// 8327308C: 4BA36E95  bl 0x82ca9f20
	ctx.lr = 0x83273090;
	sub_82CA9F20(ctx, base);
	// 83273090: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83273094: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83273098: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327309C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832730A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832730A0 size=64
    let mut pc: u32 = 0x832730A0;
    'dispatch: loop {
        match pc {
            0x832730A0 => {
    //   block [0x832730A0..0x832730E0)
	// 832730A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832730A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832730A8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832730AC: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 832730B0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832730B4: 388BB3FC  addi r4, r11, -0x4c04
	ctx.r[4].s64 = ctx.r[11].s64 + -19460;
	// 832730B8: 386AD104  addi r3, r10, -0x2efc
	ctx.r[3].s64 = ctx.r[10].s64 + -12028;
	// 832730BC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832730C0: 4AFB9E11  bl 0x8222ced0
	ctx.lr = 0x832730C4;
	sub_8222CED0(ctx, base);
	// 832730C4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 832730C8: 3869F5A0  addi r3, r9, -0xa60
	ctx.r[3].s64 = ctx.r[9].s64 + -2656;
	// 832730CC: 4BA36E55  bl 0x82ca9f20
	ctx.lr = 0x832730D0;
	sub_82CA9F20(ctx, base);
	// 832730D0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832730D4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832730D8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832730DC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832730E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832730E0 size=64
    let mut pc: u32 = 0x832730E0;
    'dispatch: loop {
        match pc {
            0x832730E0 => {
    //   block [0x832730E0..0x83273120)
	// 832730E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832730E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832730E8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832730EC: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 832730F0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832730F4: 388BB430  addi r4, r11, -0x4bd0
	ctx.r[4].s64 = ctx.r[11].s64 + -19408;
	// 832730F8: 386AD108  addi r3, r10, -0x2ef8
	ctx.r[3].s64 = ctx.r[10].s64 + -12024;
	// 832730FC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83273100: 4AFB9DD1  bl 0x8222ced0
	ctx.lr = 0x83273104;
	sub_8222CED0(ctx, base);
	// 83273104: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83273108: 3869F5B0  addi r3, r9, -0xa50
	ctx.r[3].s64 = ctx.r[9].s64 + -2640;
	// 8327310C: 4BA36E15  bl 0x82ca9f20
	ctx.lr = 0x83273110;
	sub_82CA9F20(ctx, base);
	// 83273110: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83273114: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83273118: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327311C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83273120(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83273120 size=64
    let mut pc: u32 = 0x83273120;
    'dispatch: loop {
        match pc {
            0x83273120 => {
    //   block [0x83273120..0x83273160)
	// 83273120: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83273124: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83273128: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327312C: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 83273130: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83273134: 388BB464  addi r4, r11, -0x4b9c
	ctx.r[4].s64 = ctx.r[11].s64 + -19356;
	// 83273138: 386AD10C  addi r3, r10, -0x2ef4
	ctx.r[3].s64 = ctx.r[10].s64 + -12020;
	// 8327313C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83273140: 4AFB9D91  bl 0x8222ced0
	ctx.lr = 0x83273144;
	sub_8222CED0(ctx, base);
	// 83273144: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83273148: 3869F5C0  addi r3, r9, -0xa40
	ctx.r[3].s64 = ctx.r[9].s64 + -2624;
	// 8327314C: 4BA36DD5  bl 0x82ca9f20
	ctx.lr = 0x83273150;
	sub_82CA9F20(ctx, base);
	// 83273150: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83273154: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83273158: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327315C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83273160(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83273160 size=64
    let mut pc: u32 = 0x83273160;
    'dispatch: loop {
        match pc {
            0x83273160 => {
    //   block [0x83273160..0x832731A0)
	// 83273160: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83273164: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83273168: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327316C: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 83273170: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83273174: 388BB344  addi r4, r11, -0x4cbc
	ctx.r[4].s64 = ctx.r[11].s64 + -19644;
	// 83273178: 386AD110  addi r3, r10, -0x2ef0
	ctx.r[3].s64 = ctx.r[10].s64 + -12016;
	// 8327317C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83273180: 4AFB9D51  bl 0x8222ced0
	ctx.lr = 0x83273184;
	sub_8222CED0(ctx, base);
	// 83273184: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83273188: 3869F5D0  addi r3, r9, -0xa30
	ctx.r[3].s64 = ctx.r[9].s64 + -2608;
	// 8327318C: 4BA36D95  bl 0x82ca9f20
	ctx.lr = 0x83273190;
	sub_82CA9F20(ctx, base);
	// 83273190: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83273194: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83273198: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327319C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832731A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832731A0 size=64
    let mut pc: u32 = 0x832731A0;
    'dispatch: loop {
        match pc {
            0x832731A0 => {
    //   block [0x832731A0..0x832731E0)
	// 832731A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832731A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832731A8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832731AC: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 832731B0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832731B4: 388BB370  addi r4, r11, -0x4c90
	ctx.r[4].s64 = ctx.r[11].s64 + -19600;
	// 832731B8: 386AD114  addi r3, r10, -0x2eec
	ctx.r[3].s64 = ctx.r[10].s64 + -12012;
	// 832731BC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832731C0: 4AFB9D11  bl 0x8222ced0
	ctx.lr = 0x832731C4;
	sub_8222CED0(ctx, base);
	// 832731C4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 832731C8: 3869F5E0  addi r3, r9, -0xa20
	ctx.r[3].s64 = ctx.r[9].s64 + -2592;
	// 832731CC: 4BA36D55  bl 0x82ca9f20
	ctx.lr = 0x832731D0;
	sub_82CA9F20(ctx, base);
	// 832731D0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832731D4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832731D8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832731DC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832731E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832731E0 size=64
    let mut pc: u32 = 0x832731E0;
    'dispatch: loop {
        match pc {
            0x832731E0 => {
    //   block [0x832731E0..0x83273220)
	// 832731E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832731E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832731E8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832731EC: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 832731F0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832731F4: 388BB490  addi r4, r11, -0x4b70
	ctx.r[4].s64 = ctx.r[11].s64 + -19312;
	// 832731F8: 386AD118  addi r3, r10, -0x2ee8
	ctx.r[3].s64 = ctx.r[10].s64 + -12008;
	// 832731FC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83273200: 4AFB9CD1  bl 0x8222ced0
	ctx.lr = 0x83273204;
	sub_8222CED0(ctx, base);
	// 83273204: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83273208: 3869F5F0  addi r3, r9, -0xa10
	ctx.r[3].s64 = ctx.r[9].s64 + -2576;
	// 8327320C: 4BA36D15  bl 0x82ca9f20
	ctx.lr = 0x83273210;
	sub_82CA9F20(ctx, base);
	// 83273210: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83273214: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83273218: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327321C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83273220(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83273220 size=64
    let mut pc: u32 = 0x83273220;
    'dispatch: loop {
        match pc {
            0x83273220 => {
    //   block [0x83273220..0x83273260)
	// 83273220: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83273224: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83273228: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327322C: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 83273230: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83273234: 388BB3C8  addi r4, r11, -0x4c38
	ctx.r[4].s64 = ctx.r[11].s64 + -19512;
	// 83273238: 386AD11C  addi r3, r10, -0x2ee4
	ctx.r[3].s64 = ctx.r[10].s64 + -12004;
	// 8327323C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83273240: 4AFB9C91  bl 0x8222ced0
	ctx.lr = 0x83273244;
	sub_8222CED0(ctx, base);
	// 83273244: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83273248: 3869F600  addi r3, r9, -0xa00
	ctx.r[3].s64 = ctx.r[9].s64 + -2560;
	// 8327324C: 4BA36CD5  bl 0x82ca9f20
	ctx.lr = 0x83273250;
	sub_82CA9F20(ctx, base);
	// 83273250: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83273254: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83273258: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327325C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83273260(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83273260 size=64
    let mut pc: u32 = 0x83273260;
    'dispatch: loop {
        match pc {
            0x83273260 => {
    //   block [0x83273260..0x832732A0)
	// 83273260: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83273264: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83273268: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327326C: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 83273270: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83273274: 388BB3FC  addi r4, r11, -0x4c04
	ctx.r[4].s64 = ctx.r[11].s64 + -19460;
	// 83273278: 386AD120  addi r3, r10, -0x2ee0
	ctx.r[3].s64 = ctx.r[10].s64 + -12000;
	// 8327327C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83273280: 4AFB9C51  bl 0x8222ced0
	ctx.lr = 0x83273284;
	sub_8222CED0(ctx, base);
	// 83273284: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83273288: 3869F610  addi r3, r9, -0x9f0
	ctx.r[3].s64 = ctx.r[9].s64 + -2544;
	// 8327328C: 4BA36C95  bl 0x82ca9f20
	ctx.lr = 0x83273290;
	sub_82CA9F20(ctx, base);
	// 83273290: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83273294: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83273298: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327329C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832732A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832732A0 size=64
    let mut pc: u32 = 0x832732A0;
    'dispatch: loop {
        match pc {
            0x832732A0 => {
    //   block [0x832732A0..0x832732E0)
	// 832732A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832732A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832732A8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832732AC: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 832732B0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832732B4: 388BB464  addi r4, r11, -0x4b9c
	ctx.r[4].s64 = ctx.r[11].s64 + -19356;
	// 832732B8: 386AD124  addi r3, r10, -0x2edc
	ctx.r[3].s64 = ctx.r[10].s64 + -11996;
	// 832732BC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832732C0: 4AFB9C11  bl 0x8222ced0
	ctx.lr = 0x832732C4;
	sub_8222CED0(ctx, base);
	// 832732C4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 832732C8: 3869F620  addi r3, r9, -0x9e0
	ctx.r[3].s64 = ctx.r[9].s64 + -2528;
	// 832732CC: 4BA36C55  bl 0x82ca9f20
	ctx.lr = 0x832732D0;
	sub_82CA9F20(ctx, base);
	// 832732D0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832732D4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832732D8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832732DC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832732E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832732E0 size=64
    let mut pc: u32 = 0x832732E0;
    'dispatch: loop {
        match pc {
            0x832732E0 => {
    //   block [0x832732E0..0x83273320)
	// 832732E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832732E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832732E8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832732EC: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 832732F0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832732F4: 388BB490  addi r4, r11, -0x4b70
	ctx.r[4].s64 = ctx.r[11].s64 + -19312;
	// 832732F8: 386AD128  addi r3, r10, -0x2ed8
	ctx.r[3].s64 = ctx.r[10].s64 + -11992;
	// 832732FC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83273300: 4AFB9BD1  bl 0x8222ced0
	ctx.lr = 0x83273304;
	sub_8222CED0(ctx, base);
	// 83273304: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83273308: 3869F630  addi r3, r9, -0x9d0
	ctx.r[3].s64 = ctx.r[9].s64 + -2512;
	// 8327330C: 4BA36C15  bl 0x82ca9f20
	ctx.lr = 0x83273310;
	sub_82CA9F20(ctx, base);
	// 83273310: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83273314: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83273318: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327331C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83273320(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83273320 size=56
    let mut pc: u32 = 0x83273320;
    'dispatch: loop {
        match pc {
            0x83273320 => {
    //   block [0x83273320..0x83273358)
	// 83273320: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83273324: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83273328: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327332C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83273330: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83273334: 386B42C8  addi r3, r11, 0x42c8
	ctx.r[3].s64 = ctx.r[11].s64 + 17096;
	// 83273338: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8327333C: 4AF80A1D  bl 0x821f3d58
	ctx.lr = 0x83273340;
	sub_821F3D58(ctx, base);
	// 83273340: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83273344: 906AD12C  stw r3, -0x2ed4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-11988 as u32), ctx.r[3].u32 ) };
	// 83273348: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327334C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83273350: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83273354: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83273358(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83273358 size=56
    let mut pc: u32 = 0x83273358;
    'dispatch: loop {
        match pc {
            0x83273358 => {
    //   block [0x83273358..0x83273390)
	// 83273358: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327335C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83273360: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83273364: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83273368: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8327336C: 386B42DC  addi r3, r11, 0x42dc
	ctx.r[3].s64 = ctx.r[11].s64 + 17116;
	// 83273370: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83273374: 4AF809E5  bl 0x821f3d58
	ctx.lr = 0x83273378;
	sub_821F3D58(ctx, base);
	// 83273378: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327337C: 906AD130  stw r3, -0x2ed0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-11984 as u32), ctx.r[3].u32 ) };
	// 83273380: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83273384: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83273388: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327338C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83273390(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83273390 size=56
    let mut pc: u32 = 0x83273390;
    'dispatch: loop {
        match pc {
            0x83273390 => {
    //   block [0x83273390..0x832733C8)
	// 83273390: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83273394: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83273398: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327339C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 832733A0: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 832733A4: 386B42E8  addi r3, r11, 0x42e8
	ctx.r[3].s64 = ctx.r[11].s64 + 17128;
	// 832733A8: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 832733AC: 4AF809AD  bl 0x821f3d58
	ctx.lr = 0x832733B0;
	sub_821F3D58(ctx, base);
	// 832733B0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832733B4: 906AD134  stw r3, -0x2ecc(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-11980 as u32), ctx.r[3].u32 ) };
	// 832733B8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832733BC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832733C0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832733C4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832733C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832733C8 size=56
    let mut pc: u32 = 0x832733C8;
    'dispatch: loop {
        match pc {
            0x832733C8 => {
    //   block [0x832733C8..0x83273400)
	// 832733C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832733CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832733D0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832733D4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 832733D8: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 832733DC: 386B4300  addi r3, r11, 0x4300
	ctx.r[3].s64 = ctx.r[11].s64 + 17152;
	// 832733E0: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 832733E4: 4AF80975  bl 0x821f3d58
	ctx.lr = 0x832733E8;
	sub_821F3D58(ctx, base);
	// 832733E8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832733EC: 906AD138  stw r3, -0x2ec8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-11976 as u32), ctx.r[3].u32 ) };
	// 832733F0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832733F4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832733F8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832733FC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83273400(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83273400 size=56
    let mut pc: u32 = 0x83273400;
    'dispatch: loop {
        match pc {
            0x83273400 => {
    //   block [0x83273400..0x83273438)
	// 83273400: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83273404: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83273408: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327340C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83273410: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83273414: 386B4310  addi r3, r11, 0x4310
	ctx.r[3].s64 = ctx.r[11].s64 + 17168;
	// 83273418: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8327341C: 4AF8093D  bl 0x821f3d58
	ctx.lr = 0x83273420;
	sub_821F3D58(ctx, base);
	// 83273420: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83273424: 906AD13C  stw r3, -0x2ec4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-11972 as u32), ctx.r[3].u32 ) };
	// 83273428: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327342C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83273430: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83273434: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83273438(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83273438 size=56
    let mut pc: u32 = 0x83273438;
    'dispatch: loop {
        match pc {
            0x83273438 => {
    //   block [0x83273438..0x83273470)
	// 83273438: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327343C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83273440: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83273444: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83273448: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8327344C: 386B4328  addi r3, r11, 0x4328
	ctx.r[3].s64 = ctx.r[11].s64 + 17192;
	// 83273450: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83273454: 4AF80905  bl 0x821f3d58
	ctx.lr = 0x83273458;
	sub_821F3D58(ctx, base);
	// 83273458: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327345C: 906AD140  stw r3, -0x2ec0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-11968 as u32), ctx.r[3].u32 ) };
	// 83273460: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83273464: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83273468: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327346C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83273470(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83273470 size=56
    let mut pc: u32 = 0x83273470;
    'dispatch: loop {
        match pc {
            0x83273470 => {
    //   block [0x83273470..0x832734A8)
	// 83273470: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83273474: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83273478: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327347C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83273480: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83273484: 386B4338  addi r3, r11, 0x4338
	ctx.r[3].s64 = ctx.r[11].s64 + 17208;
	// 83273488: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8327348C: 4AF808CD  bl 0x821f3d58
	ctx.lr = 0x83273490;
	sub_821F3D58(ctx, base);
	// 83273490: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83273494: 906AD144  stw r3, -0x2ebc(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-11964 as u32), ctx.r[3].u32 ) };
	// 83273498: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327349C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832734A0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832734A4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832734A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832734A8 size=56
    let mut pc: u32 = 0x832734A8;
    'dispatch: loop {
        match pc {
            0x832734A8 => {
    //   block [0x832734A8..0x832734E0)
	// 832734A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832734AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832734B0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832734B4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 832734B8: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 832734BC: 386B4350  addi r3, r11, 0x4350
	ctx.r[3].s64 = ctx.r[11].s64 + 17232;
	// 832734C0: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 832734C4: 4AF80895  bl 0x821f3d58
	ctx.lr = 0x832734C8;
	sub_821F3D58(ctx, base);
	// 832734C8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832734CC: 906AD148  stw r3, -0x2eb8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-11960 as u32), ctx.r[3].u32 ) };
	// 832734D0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832734D4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832734D8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832734DC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832734E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832734E0 size=56
    let mut pc: u32 = 0x832734E0;
    'dispatch: loop {
        match pc {
            0x832734E0 => {
    //   block [0x832734E0..0x83273518)
	// 832734E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832734E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832734E8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832734EC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 832734F0: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 832734F4: 386B4368  addi r3, r11, 0x4368
	ctx.r[3].s64 = ctx.r[11].s64 + 17256;
	// 832734F8: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 832734FC: 4AF8085D  bl 0x821f3d58
	ctx.lr = 0x83273500;
	sub_821F3D58(ctx, base);
	// 83273500: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83273504: 906AD14C  stw r3, -0x2eb4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-11956 as u32), ctx.r[3].u32 ) };
	// 83273508: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327350C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83273510: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83273514: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83273518(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83273518 size=56
    let mut pc: u32 = 0x83273518;
    'dispatch: loop {
        match pc {
            0x83273518 => {
    //   block [0x83273518..0x83273550)
	// 83273518: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327351C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83273520: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83273524: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83273528: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8327352C: 386B4380  addi r3, r11, 0x4380
	ctx.r[3].s64 = ctx.r[11].s64 + 17280;
	// 83273530: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83273534: 4AF80825  bl 0x821f3d58
	ctx.lr = 0x83273538;
	sub_821F3D58(ctx, base);
	// 83273538: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327353C: 906AD150  stw r3, -0x2eb0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-11952 as u32), ctx.r[3].u32 ) };
	// 83273540: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83273544: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83273548: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327354C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83273550(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83273550 size=56
    let mut pc: u32 = 0x83273550;
    'dispatch: loop {
        match pc {
            0x83273550 => {
    //   block [0x83273550..0x83273588)
	// 83273550: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83273554: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83273558: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327355C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83273560: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83273564: 386B4394  addi r3, r11, 0x4394
	ctx.r[3].s64 = ctx.r[11].s64 + 17300;
	// 83273568: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8327356C: 4AF807ED  bl 0x821f3d58
	ctx.lr = 0x83273570;
	sub_821F3D58(ctx, base);
	// 83273570: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83273574: 906AD154  stw r3, -0x2eac(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-11948 as u32), ctx.r[3].u32 ) };
	// 83273578: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327357C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83273580: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83273584: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83273588(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83273588 size=56
    let mut pc: u32 = 0x83273588;
    'dispatch: loop {
        match pc {
            0x83273588 => {
    //   block [0x83273588..0x832735C0)
	// 83273588: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327358C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83273590: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83273594: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83273598: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8327359C: 386B43B4  addi r3, r11, 0x43b4
	ctx.r[3].s64 = ctx.r[11].s64 + 17332;
	// 832735A0: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 832735A4: 4AF807B5  bl 0x821f3d58
	ctx.lr = 0x832735A8;
	sub_821F3D58(ctx, base);
	// 832735A8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832735AC: 906AD158  stw r3, -0x2ea8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-11944 as u32), ctx.r[3].u32 ) };
	// 832735B0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832735B4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832735B8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832735BC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832735C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832735C0 size=56
    let mut pc: u32 = 0x832735C0;
    'dispatch: loop {
        match pc {
            0x832735C0 => {
    //   block [0x832735C0..0x832735F8)
	// 832735C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832735C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832735C8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832735CC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 832735D0: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 832735D4: 386B43C8  addi r3, r11, 0x43c8
	ctx.r[3].s64 = ctx.r[11].s64 + 17352;
	// 832735D8: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 832735DC: 4AF8077D  bl 0x821f3d58
	ctx.lr = 0x832735E0;
	sub_821F3D58(ctx, base);
	// 832735E0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832735E4: 906AD15C  stw r3, -0x2ea4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-11940 as u32), ctx.r[3].u32 ) };
	// 832735E8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832735EC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832735F0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832735F4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832735F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832735F8 size=56
    let mut pc: u32 = 0x832735F8;
    'dispatch: loop {
        match pc {
            0x832735F8 => {
    //   block [0x832735F8..0x83273630)
	// 832735F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832735FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83273600: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83273604: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83273608: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8327360C: 386B43D4  addi r3, r11, 0x43d4
	ctx.r[3].s64 = ctx.r[11].s64 + 17364;
	// 83273610: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83273614: 4AF80745  bl 0x821f3d58
	ctx.lr = 0x83273618;
	sub_821F3D58(ctx, base);
	// 83273618: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327361C: 906AD160  stw r3, -0x2ea0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-11936 as u32), ctx.r[3].u32 ) };
	// 83273620: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83273624: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83273628: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327362C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83273630(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83273630 size=56
    let mut pc: u32 = 0x83273630;
    'dispatch: loop {
        match pc {
            0x83273630 => {
    //   block [0x83273630..0x83273668)
	// 83273630: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83273634: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83273638: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327363C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83273640: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83273644: 386B43E0  addi r3, r11, 0x43e0
	ctx.r[3].s64 = ctx.r[11].s64 + 17376;
	// 83273648: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8327364C: 4AF8070D  bl 0x821f3d58
	ctx.lr = 0x83273650;
	sub_821F3D58(ctx, base);
	// 83273650: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83273654: 906AD164  stw r3, -0x2e9c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-11932 as u32), ctx.r[3].u32 ) };
	// 83273658: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327365C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83273660: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83273664: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83273668(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83273668 size=56
    let mut pc: u32 = 0x83273668;
    'dispatch: loop {
        match pc {
            0x83273668 => {
    //   block [0x83273668..0x832736A0)
	// 83273668: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327366C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83273670: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83273674: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83273678: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8327367C: 386B43F0  addi r3, r11, 0x43f0
	ctx.r[3].s64 = ctx.r[11].s64 + 17392;
	// 83273680: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83273684: 4AF806D5  bl 0x821f3d58
	ctx.lr = 0x83273688;
	sub_821F3D58(ctx, base);
	// 83273688: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327368C: 906AD168  stw r3, -0x2e98(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-11928 as u32), ctx.r[3].u32 ) };
	// 83273690: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83273694: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83273698: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327369C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832736A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832736A0 size=56
    let mut pc: u32 = 0x832736A0;
    'dispatch: loop {
        match pc {
            0x832736A0 => {
    //   block [0x832736A0..0x832736D8)
	// 832736A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832736A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832736A8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832736AC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 832736B0: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 832736B4: 386B4400  addi r3, r11, 0x4400
	ctx.r[3].s64 = ctx.r[11].s64 + 17408;
	// 832736B8: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 832736BC: 4AF8069D  bl 0x821f3d58
	ctx.lr = 0x832736C0;
	sub_821F3D58(ctx, base);
	// 832736C0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832736C4: 906AD16C  stw r3, -0x2e94(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-11924 as u32), ctx.r[3].u32 ) };
	// 832736C8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832736CC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832736D0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832736D4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832736D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832736D8 size=56
    let mut pc: u32 = 0x832736D8;
    'dispatch: loop {
        match pc {
            0x832736D8 => {
    //   block [0x832736D8..0x83273710)
	// 832736D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832736DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832736E0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832736E4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 832736E8: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 832736EC: 386B4418  addi r3, r11, 0x4418
	ctx.r[3].s64 = ctx.r[11].s64 + 17432;
	// 832736F0: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 832736F4: 4AF80665  bl 0x821f3d58
	ctx.lr = 0x832736F8;
	sub_821F3D58(ctx, base);
	// 832736F8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832736FC: 906AD170  stw r3, -0x2e90(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-11920 as u32), ctx.r[3].u32 ) };
	// 83273700: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83273704: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83273708: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327370C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83273710(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83273710 size=56
    let mut pc: u32 = 0x83273710;
    'dispatch: loop {
        match pc {
            0x83273710 => {
    //   block [0x83273710..0x83273748)
	// 83273710: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83273714: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83273718: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327371C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83273720: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83273724: 386B4424  addi r3, r11, 0x4424
	ctx.r[3].s64 = ctx.r[11].s64 + 17444;
	// 83273728: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8327372C: 4AF8062D  bl 0x821f3d58
	ctx.lr = 0x83273730;
	sub_821F3D58(ctx, base);
	// 83273730: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83273734: 906AD174  stw r3, -0x2e8c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-11916 as u32), ctx.r[3].u32 ) };
	// 83273738: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327373C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83273740: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83273744: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83273748(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83273748 size=56
    let mut pc: u32 = 0x83273748;
    'dispatch: loop {
        match pc {
            0x83273748 => {
    //   block [0x83273748..0x83273780)
	// 83273748: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327374C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83273750: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83273754: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83273758: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8327375C: 386B4430  addi r3, r11, 0x4430
	ctx.r[3].s64 = ctx.r[11].s64 + 17456;
	// 83273760: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83273764: 4AF805F5  bl 0x821f3d58
	ctx.lr = 0x83273768;
	sub_821F3D58(ctx, base);
	// 83273768: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327376C: 906AD178  stw r3, -0x2e88(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-11912 as u32), ctx.r[3].u32 ) };
	// 83273770: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83273774: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83273778: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327377C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83273780(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83273780 size=56
    let mut pc: u32 = 0x83273780;
    'dispatch: loop {
        match pc {
            0x83273780 => {
    //   block [0x83273780..0x832737B8)
	// 83273780: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83273784: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83273788: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327378C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83273790: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83273794: 386B4440  addi r3, r11, 0x4440
	ctx.r[3].s64 = ctx.r[11].s64 + 17472;
	// 83273798: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8327379C: 4AF805BD  bl 0x821f3d58
	ctx.lr = 0x832737A0;
	sub_821F3D58(ctx, base);
	// 832737A0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832737A4: 906AD17C  stw r3, -0x2e84(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-11908 as u32), ctx.r[3].u32 ) };
	// 832737A8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832737AC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832737B0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832737B4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832737B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832737B8 size=64
    let mut pc: u32 = 0x832737B8;
    'dispatch: loop {
        match pc {
            0x832737B8 => {
    //   block [0x832737B8..0x832737F8)
	// 832737B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832737BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832737C0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832737C4: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 832737C8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832737CC: 388BB830  addi r4, r11, -0x47d0
	ctx.r[4].s64 = ctx.r[11].s64 + -18384;
	// 832737D0: 386AD180  addi r3, r10, -0x2e80
	ctx.r[3].s64 = ctx.r[10].s64 + -11904;
	// 832737D4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832737D8: 4AFB96F9  bl 0x8222ced0
	ctx.lr = 0x832737DC;
	sub_8222CED0(ctx, base);
	// 832737DC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 832737E0: 3869F640  addi r3, r9, -0x9c0
	ctx.r[3].s64 = ctx.r[9].s64 + -2496;
	// 832737E4: 4BA3673D  bl 0x82ca9f20
	ctx.lr = 0x832737E8;
	sub_82CA9F20(ctx, base);
	// 832737E8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832737EC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832737F0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832737F4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832737F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832737F8 size=64
    let mut pc: u32 = 0x832737F8;
    'dispatch: loop {
        match pc {
            0x832737F8 => {
    //   block [0x832737F8..0x83273838)
	// 832737F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832737FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83273800: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83273804: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 83273808: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327380C: 388BB85C  addi r4, r11, -0x47a4
	ctx.r[4].s64 = ctx.r[11].s64 + -18340;
	// 83273810: 386AD184  addi r3, r10, -0x2e7c
	ctx.r[3].s64 = ctx.r[10].s64 + -11900;
	// 83273814: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83273818: 4AFB96B9  bl 0x8222ced0
	ctx.lr = 0x8327381C;
	sub_8222CED0(ctx, base);
	// 8327381C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83273820: 3869F650  addi r3, r9, -0x9b0
	ctx.r[3].s64 = ctx.r[9].s64 + -2480;
	// 83273824: 4BA366FD  bl 0x82ca9f20
	ctx.lr = 0x83273828;
	sub_82CA9F20(ctx, base);
	// 83273828: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327382C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83273830: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83273834: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83273838(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83273838 size=64
    let mut pc: u32 = 0x83273838;
    'dispatch: loop {
        match pc {
            0x83273838 => {
    //   block [0x83273838..0x83273878)
	// 83273838: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327383C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83273840: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83273844: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 83273848: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327384C: 388BB898  addi r4, r11, -0x4768
	ctx.r[4].s64 = ctx.r[11].s64 + -18280;
	// 83273850: 386AD188  addi r3, r10, -0x2e78
	ctx.r[3].s64 = ctx.r[10].s64 + -11896;
	// 83273854: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83273858: 4AFB9679  bl 0x8222ced0
	ctx.lr = 0x8327385C;
	sub_8222CED0(ctx, base);
	// 8327385C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83273860: 3869F660  addi r3, r9, -0x9a0
	ctx.r[3].s64 = ctx.r[9].s64 + -2464;
	// 83273864: 4BA366BD  bl 0x82ca9f20
	ctx.lr = 0x83273868;
	sub_82CA9F20(ctx, base);
	// 83273868: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327386C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83273870: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83273874: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83273878(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83273878 size=64
    let mut pc: u32 = 0x83273878;
    'dispatch: loop {
        match pc {
            0x83273878 => {
    //   block [0x83273878..0x832738B8)
	// 83273878: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327387C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83273880: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83273884: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 83273888: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327388C: 388BD4FC  addi r4, r11, -0x2b04
	ctx.r[4].s64 = ctx.r[11].s64 + -11012;
	// 83273890: 386AD18C  addi r3, r10, -0x2e74
	ctx.r[3].s64 = ctx.r[10].s64 + -11892;
	// 83273894: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83273898: 4AFB9639  bl 0x8222ced0
	ctx.lr = 0x8327389C;
	sub_8222CED0(ctx, base);
	// 8327389C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 832738A0: 3869F670  addi r3, r9, -0x990
	ctx.r[3].s64 = ctx.r[9].s64 + -2448;
	// 832738A4: 4BA3667D  bl 0x82ca9f20
	ctx.lr = 0x832738A8;
	sub_82CA9F20(ctx, base);
	// 832738A8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832738AC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832738B0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832738B4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832738B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832738B8 size=64
    let mut pc: u32 = 0x832738B8;
    'dispatch: loop {
        match pc {
            0x832738B8 => {
    //   block [0x832738B8..0x832738F8)
	// 832738B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832738BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832738C0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832738C4: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 832738C8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832738CC: 388BB8D4  addi r4, r11, -0x472c
	ctx.r[4].s64 = ctx.r[11].s64 + -18220;
	// 832738D0: 386AD190  addi r3, r10, -0x2e70
	ctx.r[3].s64 = ctx.r[10].s64 + -11888;
	// 832738D4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832738D8: 4AFB95F9  bl 0x8222ced0
	ctx.lr = 0x832738DC;
	sub_8222CED0(ctx, base);
	// 832738DC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 832738E0: 3869F680  addi r3, r9, -0x980
	ctx.r[3].s64 = ctx.r[9].s64 + -2432;
	// 832738E4: 4BA3663D  bl 0x82ca9f20
	ctx.lr = 0x832738E8;
	sub_82CA9F20(ctx, base);
	// 832738E8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832738EC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832738F0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832738F4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832738F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832738F8 size=64
    let mut pc: u32 = 0x832738F8;
    'dispatch: loop {
        match pc {
            0x832738F8 => {
    //   block [0x832738F8..0x83273938)
	// 832738F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832738FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83273900: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83273904: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 83273908: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327390C: 388B9F80  addi r4, r11, -0x6080
	ctx.r[4].s64 = ctx.r[11].s64 + -24704;
	// 83273910: 386AD194  addi r3, r10, -0x2e6c
	ctx.r[3].s64 = ctx.r[10].s64 + -11884;
	// 83273914: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83273918: 4AFB95B9  bl 0x8222ced0
	ctx.lr = 0x8327391C;
	sub_8222CED0(ctx, base);
	// 8327391C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83273920: 3869F690  addi r3, r9, -0x970
	ctx.r[3].s64 = ctx.r[9].s64 + -2416;
	// 83273924: 4BA365FD  bl 0x82ca9f20
	ctx.lr = 0x83273928;
	sub_82CA9F20(ctx, base);
	// 83273928: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327392C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83273930: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83273934: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83273938(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83273938 size=56
    let mut pc: u32 = 0x83273938;
    'dispatch: loop {
        match pc {
            0x83273938 => {
    //   block [0x83273938..0x83273970)
	// 83273938: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327393C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83273940: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83273944: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83273948: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8327394C: 386B42C8  addi r3, r11, 0x42c8
	ctx.r[3].s64 = ctx.r[11].s64 + 17096;
	// 83273950: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83273954: 4AF80405  bl 0x821f3d58
	ctx.lr = 0x83273958;
	sub_821F3D58(ctx, base);
	// 83273958: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327395C: 906AD198  stw r3, -0x2e68(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-11880 as u32), ctx.r[3].u32 ) };
	// 83273960: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83273964: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83273968: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327396C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83273970(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83273970 size=56
    let mut pc: u32 = 0x83273970;
    'dispatch: loop {
        match pc {
            0x83273970 => {
    //   block [0x83273970..0x832739A8)
	// 83273970: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83273974: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83273978: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327397C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83273980: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83273984: 386B42DC  addi r3, r11, 0x42dc
	ctx.r[3].s64 = ctx.r[11].s64 + 17116;
	// 83273988: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8327398C: 4AF803CD  bl 0x821f3d58
	ctx.lr = 0x83273990;
	sub_821F3D58(ctx, base);
	// 83273990: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83273994: 906AD19C  stw r3, -0x2e64(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-11876 as u32), ctx.r[3].u32 ) };
	// 83273998: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327399C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832739A0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832739A4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832739A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832739A8 size=56
    let mut pc: u32 = 0x832739A8;
    'dispatch: loop {
        match pc {
            0x832739A8 => {
    //   block [0x832739A8..0x832739E0)
	// 832739A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832739AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832739B0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832739B4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 832739B8: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 832739BC: 386B42E8  addi r3, r11, 0x42e8
	ctx.r[3].s64 = ctx.r[11].s64 + 17128;
	// 832739C0: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 832739C4: 4AF80395  bl 0x821f3d58
	ctx.lr = 0x832739C8;
	sub_821F3D58(ctx, base);
	// 832739C8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832739CC: 906AD1A0  stw r3, -0x2e60(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-11872 as u32), ctx.r[3].u32 ) };
	// 832739D0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832739D4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832739D8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832739DC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832739E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832739E0 size=56
    let mut pc: u32 = 0x832739E0;
    'dispatch: loop {
        match pc {
            0x832739E0 => {
    //   block [0x832739E0..0x83273A18)
	// 832739E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832739E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832739E8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832739EC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 832739F0: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 832739F4: 386B4300  addi r3, r11, 0x4300
	ctx.r[3].s64 = ctx.r[11].s64 + 17152;
	// 832739F8: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 832739FC: 4AF8035D  bl 0x821f3d58
	ctx.lr = 0x83273A00;
	sub_821F3D58(ctx, base);
	// 83273A00: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83273A04: 906AD1A4  stw r3, -0x2e5c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-11868 as u32), ctx.r[3].u32 ) };
	// 83273A08: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83273A0C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83273A10: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83273A14: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83273A18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83273A18 size=56
    let mut pc: u32 = 0x83273A18;
    'dispatch: loop {
        match pc {
            0x83273A18 => {
    //   block [0x83273A18..0x83273A50)
	// 83273A18: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83273A1C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83273A20: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83273A24: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83273A28: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83273A2C: 386B4310  addi r3, r11, 0x4310
	ctx.r[3].s64 = ctx.r[11].s64 + 17168;
	// 83273A30: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83273A34: 4AF80325  bl 0x821f3d58
	ctx.lr = 0x83273A38;
	sub_821F3D58(ctx, base);
	// 83273A38: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83273A3C: 906AD1A8  stw r3, -0x2e58(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-11864 as u32), ctx.r[3].u32 ) };
	// 83273A40: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83273A44: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83273A48: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83273A4C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83273A50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83273A50 size=56
    let mut pc: u32 = 0x83273A50;
    'dispatch: loop {
        match pc {
            0x83273A50 => {
    //   block [0x83273A50..0x83273A88)
	// 83273A50: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83273A54: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83273A58: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83273A5C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83273A60: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83273A64: 386B4328  addi r3, r11, 0x4328
	ctx.r[3].s64 = ctx.r[11].s64 + 17192;
	// 83273A68: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83273A6C: 4AF802ED  bl 0x821f3d58
	ctx.lr = 0x83273A70;
	sub_821F3D58(ctx, base);
	// 83273A70: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83273A74: 906AD1AC  stw r3, -0x2e54(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-11860 as u32), ctx.r[3].u32 ) };
	// 83273A78: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83273A7C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83273A80: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83273A84: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83273A88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83273A88 size=56
    let mut pc: u32 = 0x83273A88;
    'dispatch: loop {
        match pc {
            0x83273A88 => {
    //   block [0x83273A88..0x83273AC0)
	// 83273A88: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83273A8C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83273A90: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83273A94: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83273A98: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83273A9C: 386B4338  addi r3, r11, 0x4338
	ctx.r[3].s64 = ctx.r[11].s64 + 17208;
	// 83273AA0: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83273AA4: 4AF802B5  bl 0x821f3d58
	ctx.lr = 0x83273AA8;
	sub_821F3D58(ctx, base);
	// 83273AA8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83273AAC: 906AD1B0  stw r3, -0x2e50(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-11856 as u32), ctx.r[3].u32 ) };
	// 83273AB0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83273AB4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83273AB8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83273ABC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83273AC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83273AC0 size=56
    let mut pc: u32 = 0x83273AC0;
    'dispatch: loop {
        match pc {
            0x83273AC0 => {
    //   block [0x83273AC0..0x83273AF8)
	// 83273AC0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83273AC4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83273AC8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83273ACC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83273AD0: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83273AD4: 386B4350  addi r3, r11, 0x4350
	ctx.r[3].s64 = ctx.r[11].s64 + 17232;
	// 83273AD8: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83273ADC: 4AF8027D  bl 0x821f3d58
	ctx.lr = 0x83273AE0;
	sub_821F3D58(ctx, base);
	// 83273AE0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83273AE4: 906AD1B4  stw r3, -0x2e4c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-11852 as u32), ctx.r[3].u32 ) };
	// 83273AE8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83273AEC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83273AF0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83273AF4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83273AF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83273AF8 size=56
    let mut pc: u32 = 0x83273AF8;
    'dispatch: loop {
        match pc {
            0x83273AF8 => {
    //   block [0x83273AF8..0x83273B30)
	// 83273AF8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83273AFC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83273B00: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83273B04: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83273B08: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83273B0C: 386B4368  addi r3, r11, 0x4368
	ctx.r[3].s64 = ctx.r[11].s64 + 17256;
	// 83273B10: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83273B14: 4AF80245  bl 0x821f3d58
	ctx.lr = 0x83273B18;
	sub_821F3D58(ctx, base);
	// 83273B18: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83273B1C: 906AD1B8  stw r3, -0x2e48(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-11848 as u32), ctx.r[3].u32 ) };
	// 83273B20: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83273B24: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83273B28: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83273B2C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83273B30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83273B30 size=56
    let mut pc: u32 = 0x83273B30;
    'dispatch: loop {
        match pc {
            0x83273B30 => {
    //   block [0x83273B30..0x83273B68)
	// 83273B30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83273B34: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83273B38: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83273B3C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83273B40: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83273B44: 386B4380  addi r3, r11, 0x4380
	ctx.r[3].s64 = ctx.r[11].s64 + 17280;
	// 83273B48: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83273B4C: 4AF8020D  bl 0x821f3d58
	ctx.lr = 0x83273B50;
	sub_821F3D58(ctx, base);
	// 83273B50: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83273B54: 906AD1BC  stw r3, -0x2e44(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-11844 as u32), ctx.r[3].u32 ) };
	// 83273B58: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83273B5C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83273B60: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83273B64: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83273B68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83273B68 size=56
    let mut pc: u32 = 0x83273B68;
    'dispatch: loop {
        match pc {
            0x83273B68 => {
    //   block [0x83273B68..0x83273BA0)
	// 83273B68: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83273B6C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83273B70: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83273B74: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83273B78: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83273B7C: 386B4394  addi r3, r11, 0x4394
	ctx.r[3].s64 = ctx.r[11].s64 + 17300;
	// 83273B80: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83273B84: 4AF801D5  bl 0x821f3d58
	ctx.lr = 0x83273B88;
	sub_821F3D58(ctx, base);
	// 83273B88: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83273B8C: 906AD1C0  stw r3, -0x2e40(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-11840 as u32), ctx.r[3].u32 ) };
	// 83273B90: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83273B94: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83273B98: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83273B9C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83273BA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83273BA0 size=56
    let mut pc: u32 = 0x83273BA0;
    'dispatch: loop {
        match pc {
            0x83273BA0 => {
    //   block [0x83273BA0..0x83273BD8)
	// 83273BA0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83273BA4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83273BA8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83273BAC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83273BB0: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83273BB4: 386B43B4  addi r3, r11, 0x43b4
	ctx.r[3].s64 = ctx.r[11].s64 + 17332;
	// 83273BB8: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83273BBC: 4AF8019D  bl 0x821f3d58
	ctx.lr = 0x83273BC0;
	sub_821F3D58(ctx, base);
	// 83273BC0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83273BC4: 906AD1C4  stw r3, -0x2e3c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-11836 as u32), ctx.r[3].u32 ) };
	// 83273BC8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83273BCC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83273BD0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83273BD4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83273BD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83273BD8 size=56
    let mut pc: u32 = 0x83273BD8;
    'dispatch: loop {
        match pc {
            0x83273BD8 => {
    //   block [0x83273BD8..0x83273C10)
	// 83273BD8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83273BDC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83273BE0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83273BE4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83273BE8: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83273BEC: 386B43C8  addi r3, r11, 0x43c8
	ctx.r[3].s64 = ctx.r[11].s64 + 17352;
	// 83273BF0: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83273BF4: 4AF80165  bl 0x821f3d58
	ctx.lr = 0x83273BF8;
	sub_821F3D58(ctx, base);
	// 83273BF8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83273BFC: 906AD1C8  stw r3, -0x2e38(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-11832 as u32), ctx.r[3].u32 ) };
	// 83273C00: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83273C04: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83273C08: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83273C0C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83273C10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83273C10 size=56
    let mut pc: u32 = 0x83273C10;
    'dispatch: loop {
        match pc {
            0x83273C10 => {
    //   block [0x83273C10..0x83273C48)
	// 83273C10: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83273C14: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83273C18: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83273C1C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83273C20: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83273C24: 386B43D4  addi r3, r11, 0x43d4
	ctx.r[3].s64 = ctx.r[11].s64 + 17364;
	// 83273C28: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83273C2C: 4AF8012D  bl 0x821f3d58
	ctx.lr = 0x83273C30;
	sub_821F3D58(ctx, base);
	// 83273C30: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83273C34: 906AD1CC  stw r3, -0x2e34(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-11828 as u32), ctx.r[3].u32 ) };
	// 83273C38: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83273C3C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83273C40: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83273C44: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83273C48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83273C48 size=56
    let mut pc: u32 = 0x83273C48;
    'dispatch: loop {
        match pc {
            0x83273C48 => {
    //   block [0x83273C48..0x83273C80)
	// 83273C48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83273C4C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83273C50: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83273C54: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83273C58: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83273C5C: 386B43E0  addi r3, r11, 0x43e0
	ctx.r[3].s64 = ctx.r[11].s64 + 17376;
	// 83273C60: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83273C64: 4AF800F5  bl 0x821f3d58
	ctx.lr = 0x83273C68;
	sub_821F3D58(ctx, base);
	// 83273C68: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83273C6C: 906AD1D0  stw r3, -0x2e30(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-11824 as u32), ctx.r[3].u32 ) };
	// 83273C70: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83273C74: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83273C78: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83273C7C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83273C80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83273C80 size=56
    let mut pc: u32 = 0x83273C80;
    'dispatch: loop {
        match pc {
            0x83273C80 => {
    //   block [0x83273C80..0x83273CB8)
	// 83273C80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83273C84: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83273C88: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83273C8C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83273C90: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83273C94: 386B43F0  addi r3, r11, 0x43f0
	ctx.r[3].s64 = ctx.r[11].s64 + 17392;
	// 83273C98: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83273C9C: 4AF800BD  bl 0x821f3d58
	ctx.lr = 0x83273CA0;
	sub_821F3D58(ctx, base);
	// 83273CA0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83273CA4: 906AD1D4  stw r3, -0x2e2c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-11820 as u32), ctx.r[3].u32 ) };
	// 83273CA8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83273CAC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83273CB0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83273CB4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83273CB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83273CB8 size=56
    let mut pc: u32 = 0x83273CB8;
    'dispatch: loop {
        match pc {
            0x83273CB8 => {
    //   block [0x83273CB8..0x83273CF0)
	// 83273CB8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83273CBC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83273CC0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83273CC4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83273CC8: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83273CCC: 386B4400  addi r3, r11, 0x4400
	ctx.r[3].s64 = ctx.r[11].s64 + 17408;
	// 83273CD0: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83273CD4: 4AF80085  bl 0x821f3d58
	ctx.lr = 0x83273CD8;
	sub_821F3D58(ctx, base);
	// 83273CD8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83273CDC: 906AD1D8  stw r3, -0x2e28(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-11816 as u32), ctx.r[3].u32 ) };
	// 83273CE0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83273CE4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83273CE8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83273CEC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83273CF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83273CF0 size=56
    let mut pc: u32 = 0x83273CF0;
    'dispatch: loop {
        match pc {
            0x83273CF0 => {
    //   block [0x83273CF0..0x83273D28)
	// 83273CF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83273CF4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83273CF8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83273CFC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83273D00: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83273D04: 386B4418  addi r3, r11, 0x4418
	ctx.r[3].s64 = ctx.r[11].s64 + 17432;
	// 83273D08: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83273D0C: 4AF8004D  bl 0x821f3d58
	ctx.lr = 0x83273D10;
	sub_821F3D58(ctx, base);
	// 83273D10: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83273D14: 906AD1DC  stw r3, -0x2e24(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-11812 as u32), ctx.r[3].u32 ) };
	// 83273D18: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83273D1C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83273D20: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83273D24: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83273D28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83273D28 size=56
    let mut pc: u32 = 0x83273D28;
    'dispatch: loop {
        match pc {
            0x83273D28 => {
    //   block [0x83273D28..0x83273D60)
	// 83273D28: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83273D2C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83273D30: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83273D34: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83273D38: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83273D3C: 386B4424  addi r3, r11, 0x4424
	ctx.r[3].s64 = ctx.r[11].s64 + 17444;
	// 83273D40: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83273D44: 4AF80015  bl 0x821f3d58
	ctx.lr = 0x83273D48;
	sub_821F3D58(ctx, base);
	// 83273D48: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83273D4C: 906AD1E0  stw r3, -0x2e20(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-11808 as u32), ctx.r[3].u32 ) };
	// 83273D50: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83273D54: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83273D58: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83273D5C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83273D60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83273D60 size=56
    let mut pc: u32 = 0x83273D60;
    'dispatch: loop {
        match pc {
            0x83273D60 => {
    //   block [0x83273D60..0x83273D98)
	// 83273D60: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83273D64: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83273D68: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83273D6C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83273D70: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83273D74: 386B4430  addi r3, r11, 0x4430
	ctx.r[3].s64 = ctx.r[11].s64 + 17456;
	// 83273D78: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83273D7C: 4AF7FFDD  bl 0x821f3d58
	ctx.lr = 0x83273D80;
	sub_821F3D58(ctx, base);
	// 83273D80: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83273D84: 906AD1E4  stw r3, -0x2e1c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-11804 as u32), ctx.r[3].u32 ) };
	// 83273D88: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83273D8C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83273D90: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83273D94: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83273D98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83273D98 size=56
    let mut pc: u32 = 0x83273D98;
    'dispatch: loop {
        match pc {
            0x83273D98 => {
    //   block [0x83273D98..0x83273DD0)
	// 83273D98: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83273D9C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83273DA0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83273DA4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83273DA8: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83273DAC: 386B4440  addi r3, r11, 0x4440
	ctx.r[3].s64 = ctx.r[11].s64 + 17472;
	// 83273DB0: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83273DB4: 4AF7FFA5  bl 0x821f3d58
	ctx.lr = 0x83273DB8;
	sub_821F3D58(ctx, base);
	// 83273DB8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83273DBC: 906AD1E8  stw r3, -0x2e18(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-11800 as u32), ctx.r[3].u32 ) };
	// 83273DC0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83273DC4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83273DC8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83273DCC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83273DD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83273DD0 size=64
    let mut pc: u32 = 0x83273DD0;
    'dispatch: loop {
        match pc {
            0x83273DD0 => {
    //   block [0x83273DD0..0x83273E10)
	// 83273DD0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83273DD4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83273DD8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83273DDC: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 83273DE0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83273DE4: 388BA694  addi r4, r11, -0x596c
	ctx.r[4].s64 = ctx.r[11].s64 + -22892;
	// 83273DE8: 386AD1EC  addi r3, r10, -0x2e14
	ctx.r[3].s64 = ctx.r[10].s64 + -11796;
	// 83273DEC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83273DF0: 4AFB90E1  bl 0x8222ced0
	ctx.lr = 0x83273DF4;
	sub_8222CED0(ctx, base);
	// 83273DF4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83273DF8: 3869F6A0  addi r3, r9, -0x960
	ctx.r[3].s64 = ctx.r[9].s64 + -2400;
	// 83273DFC: 4BA36125  bl 0x82ca9f20
	ctx.lr = 0x83273E00;
	sub_82CA9F20(ctx, base);
	// 83273E00: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83273E04: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83273E08: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83273E0C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83273E10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83273E10 size=64
    let mut pc: u32 = 0x83273E10;
    'dispatch: loop {
        match pc {
            0x83273E10 => {
    //   block [0x83273E10..0x83273E50)
	// 83273E10: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83273E14: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83273E18: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83273E1C: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 83273E20: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83273E24: 388BA6C4  addi r4, r11, -0x593c
	ctx.r[4].s64 = ctx.r[11].s64 + -22844;
	// 83273E28: 386AD1F0  addi r3, r10, -0x2e10
	ctx.r[3].s64 = ctx.r[10].s64 + -11792;
	// 83273E2C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83273E30: 4AFB90A1  bl 0x8222ced0
	ctx.lr = 0x83273E34;
	sub_8222CED0(ctx, base);
	// 83273E34: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83273E38: 3869F6B0  addi r3, r9, -0x950
	ctx.r[3].s64 = ctx.r[9].s64 + -2384;
	// 83273E3C: 4BA360E5  bl 0x82ca9f20
	ctx.lr = 0x83273E40;
	sub_82CA9F20(ctx, base);
	// 83273E40: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83273E44: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83273E48: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83273E4C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83273E50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83273E50 size=64
    let mut pc: u32 = 0x83273E50;
    'dispatch: loop {
        match pc {
            0x83273E50 => {
    //   block [0x83273E50..0x83273E90)
	// 83273E50: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83273E54: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83273E58: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83273E5C: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 83273E60: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83273E64: 388BB96C  addi r4, r11, -0x4694
	ctx.r[4].s64 = ctx.r[11].s64 + -18068;
	// 83273E68: 386AD1F4  addi r3, r10, -0x2e0c
	ctx.r[3].s64 = ctx.r[10].s64 + -11788;
	// 83273E6C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83273E70: 4AFB9061  bl 0x8222ced0
	ctx.lr = 0x83273E74;
	sub_8222CED0(ctx, base);
	// 83273E74: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83273E78: 3869F6C0  addi r3, r9, -0x940
	ctx.r[3].s64 = ctx.r[9].s64 + -2368;
	// 83273E7C: 4BA360A5  bl 0x82ca9f20
	ctx.lr = 0x83273E80;
	sub_82CA9F20(ctx, base);
	// 83273E80: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83273E84: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83273E88: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83273E8C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83273E90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83273E90 size=64
    let mut pc: u32 = 0x83273E90;
    'dispatch: loop {
        match pc {
            0x83273E90 => {
    //   block [0x83273E90..0x83273ED0)
	// 83273E90: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83273E94: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83273E98: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83273E9C: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 83273EA0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83273EA4: 388BA878  addi r4, r11, -0x5788
	ctx.r[4].s64 = ctx.r[11].s64 + -22408;
	// 83273EA8: 386AD1F8  addi r3, r10, -0x2e08
	ctx.r[3].s64 = ctx.r[10].s64 + -11784;
	// 83273EAC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83273EB0: 4AFB9021  bl 0x8222ced0
	ctx.lr = 0x83273EB4;
	sub_8222CED0(ctx, base);
	// 83273EB4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83273EB8: 3869F6D0  addi r3, r9, -0x930
	ctx.r[3].s64 = ctx.r[9].s64 + -2352;
	// 83273EBC: 4BA36065  bl 0x82ca9f20
	ctx.lr = 0x83273EC0;
	sub_82CA9F20(ctx, base);
	// 83273EC0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83273EC4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83273EC8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83273ECC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83273ED0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83273ED0 size=64
    let mut pc: u32 = 0x83273ED0;
    'dispatch: loop {
        match pc {
            0x83273ED0 => {
    //   block [0x83273ED0..0x83273F10)
	// 83273ED0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83273ED4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83273ED8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83273EDC: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 83273EE0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83273EE4: 388BDEE0  addi r4, r11, -0x2120
	ctx.r[4].s64 = ctx.r[11].s64 + -8480;
	// 83273EE8: 386AD1FC  addi r3, r10, -0x2e04
	ctx.r[3].s64 = ctx.r[10].s64 + -11780;
	// 83273EEC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83273EF0: 4AFB8FE1  bl 0x8222ced0
	ctx.lr = 0x83273EF4;
	sub_8222CED0(ctx, base);
	// 83273EF4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83273EF8: 3869F6E0  addi r3, r9, -0x920
	ctx.r[3].s64 = ctx.r[9].s64 + -2336;
	// 83273EFC: 4BA36025  bl 0x82ca9f20
	ctx.lr = 0x83273F00;
	sub_82CA9F20(ctx, base);
	// 83273F00: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83273F04: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83273F08: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83273F0C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83273F10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83273F10 size=64
    let mut pc: u32 = 0x83273F10;
    'dispatch: loop {
        match pc {
            0x83273F10 => {
    //   block [0x83273F10..0x83273F50)
	// 83273F10: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83273F14: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83273F18: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83273F1C: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 83273F20: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83273F24: 388BB9A4  addi r4, r11, -0x465c
	ctx.r[4].s64 = ctx.r[11].s64 + -18012;
	// 83273F28: 386AD200  addi r3, r10, -0x2e00
	ctx.r[3].s64 = ctx.r[10].s64 + -11776;
	// 83273F2C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83273F30: 4AFB8FA1  bl 0x8222ced0
	ctx.lr = 0x83273F34;
	sub_8222CED0(ctx, base);
	// 83273F34: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83273F38: 3869F6F0  addi r3, r9, -0x910
	ctx.r[3].s64 = ctx.r[9].s64 + -2320;
	// 83273F3C: 4BA35FE5  bl 0x82ca9f20
	ctx.lr = 0x83273F40;
	sub_82CA9F20(ctx, base);
	// 83273F40: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83273F44: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83273F48: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83273F4C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83273F50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83273F50 size=64
    let mut pc: u32 = 0x83273F50;
    'dispatch: loop {
        match pc {
            0x83273F50 => {
    //   block [0x83273F50..0x83273F90)
	// 83273F50: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83273F54: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83273F58: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83273F5C: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 83273F60: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83273F64: 388BA924  addi r4, r11, -0x56dc
	ctx.r[4].s64 = ctx.r[11].s64 + -22236;
	// 83273F68: 386AD204  addi r3, r10, -0x2dfc
	ctx.r[3].s64 = ctx.r[10].s64 + -11772;
	// 83273F6C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83273F70: 4AFB8F61  bl 0x8222ced0
	ctx.lr = 0x83273F74;
	sub_8222CED0(ctx, base);
	// 83273F74: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83273F78: 3869F700  addi r3, r9, -0x900
	ctx.r[3].s64 = ctx.r[9].s64 + -2304;
	// 83273F7C: 4BA35FA5  bl 0x82ca9f20
	ctx.lr = 0x83273F80;
	sub_82CA9F20(ctx, base);
	// 83273F80: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83273F84: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83273F88: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83273F8C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83273F90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83273F90 size=64
    let mut pc: u32 = 0x83273F90;
    'dispatch: loop {
        match pc {
            0x83273F90 => {
    //   block [0x83273F90..0x83273FD0)
	// 83273F90: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83273F94: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83273F98: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83273F9C: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 83273FA0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83273FA4: 388BBA20  addi r4, r11, -0x45e0
	ctx.r[4].s64 = ctx.r[11].s64 + -17888;
	// 83273FA8: 386AD208  addi r3, r10, -0x2df8
	ctx.r[3].s64 = ctx.r[10].s64 + -11768;
	// 83273FAC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83273FB0: 4AFB8F21  bl 0x8222ced0
	ctx.lr = 0x83273FB4;
	sub_8222CED0(ctx, base);
	// 83273FB4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83273FB8: 3869F710  addi r3, r9, -0x8f0
	ctx.r[3].s64 = ctx.r[9].s64 + -2288;
	// 83273FBC: 4BA35F65  bl 0x82ca9f20
	ctx.lr = 0x83273FC0;
	sub_82CA9F20(ctx, base);
	// 83273FC0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83273FC4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83273FC8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83273FCC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83273FD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83273FD0 size=64
    let mut pc: u32 = 0x83273FD0;
    'dispatch: loop {
        match pc {
            0x83273FD0 => {
    //   block [0x83273FD0..0x83274010)
	// 83273FD0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83273FD4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83273FD8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83273FDC: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 83273FE0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83273FE4: 388BBA54  addi r4, r11, -0x45ac
	ctx.r[4].s64 = ctx.r[11].s64 + -17836;
	// 83273FE8: 386AD20C  addi r3, r10, -0x2df4
	ctx.r[3].s64 = ctx.r[10].s64 + -11764;
	// 83273FEC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83273FF0: 4AFB8EE1  bl 0x8222ced0
	ctx.lr = 0x83273FF4;
	sub_8222CED0(ctx, base);
	// 83273FF4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83273FF8: 3869F720  addi r3, r9, -0x8e0
	ctx.r[3].s64 = ctx.r[9].s64 + -2272;
	// 83273FFC: 4BA35F25  bl 0x82ca9f20
	ctx.lr = 0x83274000;
	sub_82CA9F20(ctx, base);
	// 83274000: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83274004: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83274008: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327400C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83274010(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83274010 size=64
    let mut pc: u32 = 0x83274010;
    'dispatch: loop {
        match pc {
            0x83274010 => {
    //   block [0x83274010..0x83274050)
	// 83274010: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83274014: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83274018: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327401C: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 83274020: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83274024: 388BBA88  addi r4, r11, -0x4578
	ctx.r[4].s64 = ctx.r[11].s64 + -17784;
	// 83274028: 386AD210  addi r3, r10, -0x2df0
	ctx.r[3].s64 = ctx.r[10].s64 + -11760;
	// 8327402C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83274030: 4AFB8EA1  bl 0x8222ced0
	ctx.lr = 0x83274034;
	sub_8222CED0(ctx, base);
	// 83274034: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83274038: 3869F730  addi r3, r9, -0x8d0
	ctx.r[3].s64 = ctx.r[9].s64 + -2256;
	// 8327403C: 4BA35EE5  bl 0x82ca9f20
	ctx.lr = 0x83274040;
	sub_82CA9F20(ctx, base);
	// 83274040: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83274044: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83274048: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327404C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83274050(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83274050 size=64
    let mut pc: u32 = 0x83274050;
    'dispatch: loop {
        match pc {
            0x83274050 => {
    //   block [0x83274050..0x83274090)
	// 83274050: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83274054: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83274058: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327405C: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 83274060: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83274064: 388BBABC  addi r4, r11, -0x4544
	ctx.r[4].s64 = ctx.r[11].s64 + -17732;
	// 83274068: 386AD214  addi r3, r10, -0x2dec
	ctx.r[3].s64 = ctx.r[10].s64 + -11756;
	// 8327406C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83274070: 4AFB8E61  bl 0x8222ced0
	ctx.lr = 0x83274074;
	sub_8222CED0(ctx, base);
	// 83274074: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83274078: 3869F740  addi r3, r9, -0x8c0
	ctx.r[3].s64 = ctx.r[9].s64 + -2240;
	// 8327407C: 4BA35EA5  bl 0x82ca9f20
	ctx.lr = 0x83274080;
	sub_82CA9F20(ctx, base);
	// 83274080: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83274084: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83274088: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327408C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83274090(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83274090 size=64
    let mut pc: u32 = 0x83274090;
    'dispatch: loop {
        match pc {
            0x83274090 => {
    //   block [0x83274090..0x832740D0)
	// 83274090: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83274094: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83274098: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327409C: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 832740A0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832740A4: 388BBAF8  addi r4, r11, -0x4508
	ctx.r[4].s64 = ctx.r[11].s64 + -17672;
	// 832740A8: 386AD218  addi r3, r10, -0x2de8
	ctx.r[3].s64 = ctx.r[10].s64 + -11752;
	// 832740AC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832740B0: 4AFB8E21  bl 0x8222ced0
	ctx.lr = 0x832740B4;
	sub_8222CED0(ctx, base);
	// 832740B4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 832740B8: 3869F750  addi r3, r9, -0x8b0
	ctx.r[3].s64 = ctx.r[9].s64 + -2224;
	// 832740BC: 4BA35E65  bl 0x82ca9f20
	ctx.lr = 0x832740C0;
	sub_82CA9F20(ctx, base);
	// 832740C0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832740C4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832740C8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832740CC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832740D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832740D0 size=64
    let mut pc: u32 = 0x832740D0;
    'dispatch: loop {
        match pc {
            0x832740D0 => {
    //   block [0x832740D0..0x83274110)
	// 832740D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832740D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832740D8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832740DC: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 832740E0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832740E4: 388BA8BC  addi r4, r11, -0x5744
	ctx.r[4].s64 = ctx.r[11].s64 + -22340;
	// 832740E8: 386AD21C  addi r3, r10, -0x2de4
	ctx.r[3].s64 = ctx.r[10].s64 + -11748;
	// 832740EC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832740F0: 4AFB8DE1  bl 0x8222ced0
	ctx.lr = 0x832740F4;
	sub_8222CED0(ctx, base);
	// 832740F4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 832740F8: 3869F760  addi r3, r9, -0x8a0
	ctx.r[3].s64 = ctx.r[9].s64 + -2208;
	// 832740FC: 4BA35E25  bl 0x82ca9f20
	ctx.lr = 0x83274100;
	sub_82CA9F20(ctx, base);
	// 83274100: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83274104: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83274108: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327410C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83274110(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83274110 size=56
    let mut pc: u32 = 0x83274110;
    'dispatch: loop {
        match pc {
            0x83274110 => {
    //   block [0x83274110..0x83274148)
	// 83274110: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83274114: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83274118: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327411C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83274120: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83274124: 386B42C8  addi r3, r11, 0x42c8
	ctx.r[3].s64 = ctx.r[11].s64 + 17096;
	// 83274128: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8327412C: 4AF7FC2D  bl 0x821f3d58
	ctx.lr = 0x83274130;
	sub_821F3D58(ctx, base);
	// 83274130: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83274134: 906AD220  stw r3, -0x2de0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-11744 as u32), ctx.r[3].u32 ) };
	// 83274138: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327413C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83274140: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83274144: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83274148(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83274148 size=56
    let mut pc: u32 = 0x83274148;
    'dispatch: loop {
        match pc {
            0x83274148 => {
    //   block [0x83274148..0x83274180)
	// 83274148: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327414C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83274150: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83274154: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83274158: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8327415C: 386B42DC  addi r3, r11, 0x42dc
	ctx.r[3].s64 = ctx.r[11].s64 + 17116;
	// 83274160: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83274164: 4AF7FBF5  bl 0x821f3d58
	ctx.lr = 0x83274168;
	sub_821F3D58(ctx, base);
	// 83274168: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327416C: 906AD224  stw r3, -0x2ddc(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-11740 as u32), ctx.r[3].u32 ) };
	// 83274170: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83274174: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83274178: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327417C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83274180(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83274180 size=56
    let mut pc: u32 = 0x83274180;
    'dispatch: loop {
        match pc {
            0x83274180 => {
    //   block [0x83274180..0x832741B8)
	// 83274180: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83274184: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83274188: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327418C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83274190: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83274194: 386B42E8  addi r3, r11, 0x42e8
	ctx.r[3].s64 = ctx.r[11].s64 + 17128;
	// 83274198: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8327419C: 4AF7FBBD  bl 0x821f3d58
	ctx.lr = 0x832741A0;
	sub_821F3D58(ctx, base);
	// 832741A0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832741A4: 906AD228  stw r3, -0x2dd8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-11736 as u32), ctx.r[3].u32 ) };
	// 832741A8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832741AC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832741B0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832741B4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832741B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832741B8 size=56
    let mut pc: u32 = 0x832741B8;
    'dispatch: loop {
        match pc {
            0x832741B8 => {
    //   block [0x832741B8..0x832741F0)
	// 832741B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832741BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832741C0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832741C4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 832741C8: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 832741CC: 386B4300  addi r3, r11, 0x4300
	ctx.r[3].s64 = ctx.r[11].s64 + 17152;
	// 832741D0: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 832741D4: 4AF7FB85  bl 0x821f3d58
	ctx.lr = 0x832741D8;
	sub_821F3D58(ctx, base);
	// 832741D8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832741DC: 906AD22C  stw r3, -0x2dd4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-11732 as u32), ctx.r[3].u32 ) };
	// 832741E0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832741E4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832741E8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832741EC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832741F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832741F0 size=56
    let mut pc: u32 = 0x832741F0;
    'dispatch: loop {
        match pc {
            0x832741F0 => {
    //   block [0x832741F0..0x83274228)
	// 832741F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832741F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832741F8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832741FC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83274200: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83274204: 386B4310  addi r3, r11, 0x4310
	ctx.r[3].s64 = ctx.r[11].s64 + 17168;
	// 83274208: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8327420C: 4AF7FB4D  bl 0x821f3d58
	ctx.lr = 0x83274210;
	sub_821F3D58(ctx, base);
	// 83274210: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83274214: 906AD230  stw r3, -0x2dd0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-11728 as u32), ctx.r[3].u32 ) };
	// 83274218: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327421C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83274220: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83274224: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83274228(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83274228 size=56
    let mut pc: u32 = 0x83274228;
    'dispatch: loop {
        match pc {
            0x83274228 => {
    //   block [0x83274228..0x83274260)
	// 83274228: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327422C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83274230: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83274234: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83274238: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8327423C: 386B4328  addi r3, r11, 0x4328
	ctx.r[3].s64 = ctx.r[11].s64 + 17192;
	// 83274240: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83274244: 4AF7FB15  bl 0x821f3d58
	ctx.lr = 0x83274248;
	sub_821F3D58(ctx, base);
	// 83274248: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327424C: 906AD234  stw r3, -0x2dcc(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-11724 as u32), ctx.r[3].u32 ) };
	// 83274250: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83274254: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83274258: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327425C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83274260(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83274260 size=56
    let mut pc: u32 = 0x83274260;
    'dispatch: loop {
        match pc {
            0x83274260 => {
    //   block [0x83274260..0x83274298)
	// 83274260: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83274264: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83274268: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327426C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83274270: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83274274: 386B4338  addi r3, r11, 0x4338
	ctx.r[3].s64 = ctx.r[11].s64 + 17208;
	// 83274278: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8327427C: 4AF7FADD  bl 0x821f3d58
	ctx.lr = 0x83274280;
	sub_821F3D58(ctx, base);
	// 83274280: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83274284: 906AD238  stw r3, -0x2dc8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-11720 as u32), ctx.r[3].u32 ) };
	// 83274288: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327428C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83274290: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83274294: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83274298(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83274298 size=56
    let mut pc: u32 = 0x83274298;
    'dispatch: loop {
        match pc {
            0x83274298 => {
    //   block [0x83274298..0x832742D0)
	// 83274298: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327429C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832742A0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832742A4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 832742A8: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 832742AC: 386B4350  addi r3, r11, 0x4350
	ctx.r[3].s64 = ctx.r[11].s64 + 17232;
	// 832742B0: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 832742B4: 4AF7FAA5  bl 0x821f3d58
	ctx.lr = 0x832742B8;
	sub_821F3D58(ctx, base);
	// 832742B8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832742BC: 906AD23C  stw r3, -0x2dc4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-11716 as u32), ctx.r[3].u32 ) };
	// 832742C0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832742C4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832742C8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832742CC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832742D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832742D0 size=56
    let mut pc: u32 = 0x832742D0;
    'dispatch: loop {
        match pc {
            0x832742D0 => {
    //   block [0x832742D0..0x83274308)
	// 832742D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832742D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832742D8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832742DC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 832742E0: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 832742E4: 386B4368  addi r3, r11, 0x4368
	ctx.r[3].s64 = ctx.r[11].s64 + 17256;
	// 832742E8: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 832742EC: 4AF7FA6D  bl 0x821f3d58
	ctx.lr = 0x832742F0;
	sub_821F3D58(ctx, base);
	// 832742F0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832742F4: 906AD240  stw r3, -0x2dc0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-11712 as u32), ctx.r[3].u32 ) };
	// 832742F8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832742FC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83274300: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83274304: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83274308(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83274308 size=56
    let mut pc: u32 = 0x83274308;
    'dispatch: loop {
        match pc {
            0x83274308 => {
    //   block [0x83274308..0x83274340)
	// 83274308: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327430C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83274310: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83274314: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83274318: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8327431C: 386B4380  addi r3, r11, 0x4380
	ctx.r[3].s64 = ctx.r[11].s64 + 17280;
	// 83274320: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83274324: 4AF7FA35  bl 0x821f3d58
	ctx.lr = 0x83274328;
	sub_821F3D58(ctx, base);
	// 83274328: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327432C: 906AD244  stw r3, -0x2dbc(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-11708 as u32), ctx.r[3].u32 ) };
	// 83274330: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83274334: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83274338: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327433C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83274340(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83274340 size=56
    let mut pc: u32 = 0x83274340;
    'dispatch: loop {
        match pc {
            0x83274340 => {
    //   block [0x83274340..0x83274378)
	// 83274340: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83274344: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83274348: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327434C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83274350: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83274354: 386B4394  addi r3, r11, 0x4394
	ctx.r[3].s64 = ctx.r[11].s64 + 17300;
	// 83274358: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8327435C: 4AF7F9FD  bl 0x821f3d58
	ctx.lr = 0x83274360;
	sub_821F3D58(ctx, base);
	// 83274360: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83274364: 906AD248  stw r3, -0x2db8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-11704 as u32), ctx.r[3].u32 ) };
	// 83274368: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327436C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83274370: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83274374: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83274378(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83274378 size=56
    let mut pc: u32 = 0x83274378;
    'dispatch: loop {
        match pc {
            0x83274378 => {
    //   block [0x83274378..0x832743B0)
	// 83274378: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327437C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83274380: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83274384: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83274388: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8327438C: 386B43B4  addi r3, r11, 0x43b4
	ctx.r[3].s64 = ctx.r[11].s64 + 17332;
	// 83274390: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83274394: 4AF7F9C5  bl 0x821f3d58
	ctx.lr = 0x83274398;
	sub_821F3D58(ctx, base);
	// 83274398: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327439C: 906AD24C  stw r3, -0x2db4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-11700 as u32), ctx.r[3].u32 ) };
	// 832743A0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832743A4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832743A8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832743AC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832743B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832743B0 size=56
    let mut pc: u32 = 0x832743B0;
    'dispatch: loop {
        match pc {
            0x832743B0 => {
    //   block [0x832743B0..0x832743E8)
	// 832743B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832743B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832743B8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832743BC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 832743C0: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 832743C4: 386B43C8  addi r3, r11, 0x43c8
	ctx.r[3].s64 = ctx.r[11].s64 + 17352;
	// 832743C8: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 832743CC: 4AF7F98D  bl 0x821f3d58
	ctx.lr = 0x832743D0;
	sub_821F3D58(ctx, base);
	// 832743D0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832743D4: 906AD250  stw r3, -0x2db0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-11696 as u32), ctx.r[3].u32 ) };
	// 832743D8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832743DC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832743E0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832743E4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832743E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832743E8 size=56
    let mut pc: u32 = 0x832743E8;
    'dispatch: loop {
        match pc {
            0x832743E8 => {
    //   block [0x832743E8..0x83274420)
	// 832743E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832743EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832743F0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832743F4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 832743F8: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 832743FC: 386B43D4  addi r3, r11, 0x43d4
	ctx.r[3].s64 = ctx.r[11].s64 + 17364;
	// 83274400: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83274404: 4AF7F955  bl 0x821f3d58
	ctx.lr = 0x83274408;
	sub_821F3D58(ctx, base);
	// 83274408: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327440C: 906AD254  stw r3, -0x2dac(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-11692 as u32), ctx.r[3].u32 ) };
	// 83274410: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83274414: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83274418: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327441C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83274420(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83274420 size=56
    let mut pc: u32 = 0x83274420;
    'dispatch: loop {
        match pc {
            0x83274420 => {
    //   block [0x83274420..0x83274458)
	// 83274420: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83274424: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83274428: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327442C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83274430: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83274434: 386B43E0  addi r3, r11, 0x43e0
	ctx.r[3].s64 = ctx.r[11].s64 + 17376;
	// 83274438: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8327443C: 4AF7F91D  bl 0x821f3d58
	ctx.lr = 0x83274440;
	sub_821F3D58(ctx, base);
	// 83274440: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83274444: 906AD258  stw r3, -0x2da8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-11688 as u32), ctx.r[3].u32 ) };
	// 83274448: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327444C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83274450: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83274454: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83274458(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83274458 size=56
    let mut pc: u32 = 0x83274458;
    'dispatch: loop {
        match pc {
            0x83274458 => {
    //   block [0x83274458..0x83274490)
	// 83274458: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327445C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83274460: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83274464: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83274468: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8327446C: 386B43F0  addi r3, r11, 0x43f0
	ctx.r[3].s64 = ctx.r[11].s64 + 17392;
	// 83274470: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83274474: 4AF7F8E5  bl 0x821f3d58
	ctx.lr = 0x83274478;
	sub_821F3D58(ctx, base);
	// 83274478: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327447C: 906AD25C  stw r3, -0x2da4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-11684 as u32), ctx.r[3].u32 ) };
	// 83274480: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83274484: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83274488: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327448C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83274490(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83274490 size=56
    let mut pc: u32 = 0x83274490;
    'dispatch: loop {
        match pc {
            0x83274490 => {
    //   block [0x83274490..0x832744C8)
	// 83274490: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83274494: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83274498: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327449C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 832744A0: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 832744A4: 386B4400  addi r3, r11, 0x4400
	ctx.r[3].s64 = ctx.r[11].s64 + 17408;
	// 832744A8: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 832744AC: 4AF7F8AD  bl 0x821f3d58
	ctx.lr = 0x832744B0;
	sub_821F3D58(ctx, base);
	// 832744B0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832744B4: 906AD260  stw r3, -0x2da0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-11680 as u32), ctx.r[3].u32 ) };
	// 832744B8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832744BC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832744C0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832744C4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832744C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832744C8 size=56
    let mut pc: u32 = 0x832744C8;
    'dispatch: loop {
        match pc {
            0x832744C8 => {
    //   block [0x832744C8..0x83274500)
	// 832744C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832744CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832744D0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832744D4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 832744D8: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 832744DC: 386B4418  addi r3, r11, 0x4418
	ctx.r[3].s64 = ctx.r[11].s64 + 17432;
	// 832744E0: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 832744E4: 4AF7F875  bl 0x821f3d58
	ctx.lr = 0x832744E8;
	sub_821F3D58(ctx, base);
	// 832744E8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832744EC: 906AD264  stw r3, -0x2d9c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-11676 as u32), ctx.r[3].u32 ) };
	// 832744F0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832744F4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832744F8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832744FC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83274500(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83274500 size=56
    let mut pc: u32 = 0x83274500;
    'dispatch: loop {
        match pc {
            0x83274500 => {
    //   block [0x83274500..0x83274538)
	// 83274500: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83274504: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83274508: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327450C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83274510: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83274514: 386B4424  addi r3, r11, 0x4424
	ctx.r[3].s64 = ctx.r[11].s64 + 17444;
	// 83274518: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8327451C: 4AF7F83D  bl 0x821f3d58
	ctx.lr = 0x83274520;
	sub_821F3D58(ctx, base);
	// 83274520: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83274524: 906AD268  stw r3, -0x2d98(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-11672 as u32), ctx.r[3].u32 ) };
	// 83274528: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327452C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83274530: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83274534: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83274538(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83274538 size=56
    let mut pc: u32 = 0x83274538;
    'dispatch: loop {
        match pc {
            0x83274538 => {
    //   block [0x83274538..0x83274570)
	// 83274538: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327453C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83274540: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83274544: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83274548: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8327454C: 386B4430  addi r3, r11, 0x4430
	ctx.r[3].s64 = ctx.r[11].s64 + 17456;
	// 83274550: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83274554: 4AF7F805  bl 0x821f3d58
	ctx.lr = 0x83274558;
	sub_821F3D58(ctx, base);
	// 83274558: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327455C: 906AD26C  stw r3, -0x2d94(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-11668 as u32), ctx.r[3].u32 ) };
	// 83274560: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83274564: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83274568: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327456C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83274570(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83274570 size=56
    let mut pc: u32 = 0x83274570;
    'dispatch: loop {
        match pc {
            0x83274570 => {
    //   block [0x83274570..0x832745A8)
	// 83274570: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83274574: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83274578: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327457C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83274580: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83274584: 386B4440  addi r3, r11, 0x4440
	ctx.r[3].s64 = ctx.r[11].s64 + 17472;
	// 83274588: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8327458C: 4AF7F7CD  bl 0x821f3d58
	ctx.lr = 0x83274590;
	sub_821F3D58(ctx, base);
	// 83274590: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83274594: 906AD270  stw r3, -0x2d90(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-11664 as u32), ctx.r[3].u32 ) };
	// 83274598: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327459C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832745A0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832745A4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832745A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832745A8 size=64
    let mut pc: u32 = 0x832745A8;
    'dispatch: loop {
        match pc {
            0x832745A8 => {
    //   block [0x832745A8..0x832745E8)
	// 832745A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832745AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832745B0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832745B4: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 832745B8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832745BC: 388BA9A4  addi r4, r11, -0x565c
	ctx.r[4].s64 = ctx.r[11].s64 + -22108;
	// 832745C0: 386AD274  addi r3, r10, -0x2d8c
	ctx.r[3].s64 = ctx.r[10].s64 + -11660;
	// 832745C4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832745C8: 4AFB8909  bl 0x8222ced0
	ctx.lr = 0x832745CC;
	sub_8222CED0(ctx, base);
	// 832745CC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 832745D0: 3869F770  addi r3, r9, -0x890
	ctx.r[3].s64 = ctx.r[9].s64 + -2192;
	// 832745D4: 4BA3594D  bl 0x82ca9f20
	ctx.lr = 0x832745D8;
	sub_82CA9F20(ctx, base);
	// 832745D8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832745DC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832745E0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832745E4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832745E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832745E8 size=64
    let mut pc: u32 = 0x832745E8;
    'dispatch: loop {
        match pc {
            0x832745E8 => {
    //   block [0x832745E8..0x83274628)
	// 832745E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832745EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832745F0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832745F4: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 832745F8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832745FC: 388BBBB8  addi r4, r11, -0x4448
	ctx.r[4].s64 = ctx.r[11].s64 + -17480;
	// 83274600: 386AD278  addi r3, r10, -0x2d88
	ctx.r[3].s64 = ctx.r[10].s64 + -11656;
	// 83274604: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83274608: 4AFB88C9  bl 0x8222ced0
	ctx.lr = 0x8327460C;
	sub_8222CED0(ctx, base);
	// 8327460C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83274610: 3869F780  addi r3, r9, -0x880
	ctx.r[3].s64 = ctx.r[9].s64 + -2176;
	// 83274614: 4BA3590D  bl 0x82ca9f20
	ctx.lr = 0x83274618;
	sub_82CA9F20(ctx, base);
	// 83274618: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327461C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83274620: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83274624: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83274628(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83274628 size=64
    let mut pc: u32 = 0x83274628;
    'dispatch: loop {
        match pc {
            0x83274628 => {
    //   block [0x83274628..0x83274668)
	// 83274628: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327462C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83274630: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83274634: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 83274638: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327463C: 388BBBF0  addi r4, r11, -0x4410
	ctx.r[4].s64 = ctx.r[11].s64 + -17424;
	// 83274640: 386AD27C  addi r3, r10, -0x2d84
	ctx.r[3].s64 = ctx.r[10].s64 + -11652;
	// 83274644: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83274648: 4AFB8889  bl 0x8222ced0
	ctx.lr = 0x8327464C;
	sub_8222CED0(ctx, base);
	// 8327464C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83274650: 3869F790  addi r3, r9, -0x870
	ctx.r[3].s64 = ctx.r[9].s64 + -2160;
	// 83274654: 4BA358CD  bl 0x82ca9f20
	ctx.lr = 0x83274658;
	sub_82CA9F20(ctx, base);
	// 83274658: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327465C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83274660: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83274664: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83274668(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83274668 size=64
    let mut pc: u32 = 0x83274668;
    'dispatch: loop {
        match pc {
            0x83274668 => {
    //   block [0x83274668..0x832746A8)
	// 83274668: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327466C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83274670: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83274674: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 83274678: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327467C: 388BBB54  addi r4, r11, -0x44ac
	ctx.r[4].s64 = ctx.r[11].s64 + -17580;
	// 83274680: 386AD280  addi r3, r10, -0x2d80
	ctx.r[3].s64 = ctx.r[10].s64 + -11648;
	// 83274684: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83274688: 4AFB8849  bl 0x8222ced0
	ctx.lr = 0x8327468C;
	sub_8222CED0(ctx, base);
	// 8327468C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83274690: 3869F7A0  addi r3, r9, -0x860
	ctx.r[3].s64 = ctx.r[9].s64 + -2144;
	// 83274694: 4BA3588D  bl 0x82ca9f20
	ctx.lr = 0x83274698;
	sub_82CA9F20(ctx, base);
	// 83274698: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327469C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832746A0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832746A4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832746A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832746A8 size=64
    let mut pc: u32 = 0x832746A8;
    'dispatch: loop {
        match pc {
            0x832746A8 => {
    //   block [0x832746A8..0x832746E8)
	// 832746A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832746AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832746B0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832746B4: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 832746B8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832746BC: 388BBB34  addi r4, r11, -0x44cc
	ctx.r[4].s64 = ctx.r[11].s64 + -17612;
	// 832746C0: 386AD284  addi r3, r10, -0x2d7c
	ctx.r[3].s64 = ctx.r[10].s64 + -11644;
	// 832746C4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832746C8: 4AFB8809  bl 0x8222ced0
	ctx.lr = 0x832746CC;
	sub_8222CED0(ctx, base);
	// 832746CC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 832746D0: 3869F7B0  addi r3, r9, -0x850
	ctx.r[3].s64 = ctx.r[9].s64 + -2128;
	// 832746D4: 4BA3584D  bl 0x82ca9f20
	ctx.lr = 0x832746D8;
	sub_82CA9F20(ctx, base);
	// 832746D8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832746DC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832746E0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832746E4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832746E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832746E8 size=64
    let mut pc: u32 = 0x832746E8;
    'dispatch: loop {
        match pc {
            0x832746E8 => {
    //   block [0x832746E8..0x83274728)
	// 832746E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832746EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832746F0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832746F4: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 832746F8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832746FC: 388B9F80  addi r4, r11, -0x6080
	ctx.r[4].s64 = ctx.r[11].s64 + -24704;
	// 83274700: 386AD288  addi r3, r10, -0x2d78
	ctx.r[3].s64 = ctx.r[10].s64 + -11640;
	// 83274704: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83274708: 4AFB87C9  bl 0x8222ced0
	ctx.lr = 0x8327470C;
	sub_8222CED0(ctx, base);
	// 8327470C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83274710: 3869F7C0  addi r3, r9, -0x840
	ctx.r[3].s64 = ctx.r[9].s64 + -2112;
	// 83274714: 4BA3580D  bl 0x82ca9f20
	ctx.lr = 0x83274718;
	sub_82CA9F20(ctx, base);
	// 83274718: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327471C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83274720: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83274724: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83274728(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83274728 size=64
    let mut pc: u32 = 0x83274728;
    'dispatch: loop {
        match pc {
            0x83274728 => {
    //   block [0x83274728..0x83274768)
	// 83274728: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327472C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83274730: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83274734: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 83274738: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327473C: 388BBC28  addi r4, r11, -0x43d8
	ctx.r[4].s64 = ctx.r[11].s64 + -17368;
	// 83274740: 386AD28C  addi r3, r10, -0x2d74
	ctx.r[3].s64 = ctx.r[10].s64 + -11636;
	// 83274744: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83274748: 4AFB8789  bl 0x8222ced0
	ctx.lr = 0x8327474C;
	sub_8222CED0(ctx, base);
	// 8327474C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83274750: 3869F7D0  addi r3, r9, -0x830
	ctx.r[3].s64 = ctx.r[9].s64 + -2096;
	// 83274754: 4BA357CD  bl 0x82ca9f20
	ctx.lr = 0x83274758;
	sub_82CA9F20(ctx, base);
	// 83274758: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327475C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83274760: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83274764: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83274768(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83274768 size=64
    let mut pc: u32 = 0x83274768;
    'dispatch: loop {
        match pc {
            0x83274768 => {
    //   block [0x83274768..0x832747A8)
	// 83274768: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327476C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83274770: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83274774: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 83274778: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327477C: 388BBC64  addi r4, r11, -0x439c
	ctx.r[4].s64 = ctx.r[11].s64 + -17308;
	// 83274780: 386AD290  addi r3, r10, -0x2d70
	ctx.r[3].s64 = ctx.r[10].s64 + -11632;
	// 83274784: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83274788: 4AFB8749  bl 0x8222ced0
	ctx.lr = 0x8327478C;
	sub_8222CED0(ctx, base);
	// 8327478C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83274790: 3869F7E0  addi r3, r9, -0x820
	ctx.r[3].s64 = ctx.r[9].s64 + -2080;
	// 83274794: 4BA3578D  bl 0x82ca9f20
	ctx.lr = 0x83274798;
	sub_82CA9F20(ctx, base);
	// 83274798: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327479C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832747A0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832747A4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832747A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832747A8 size=64
    let mut pc: u32 = 0x832747A8;
    'dispatch: loop {
        match pc {
            0x832747A8 => {
    //   block [0x832747A8..0x832747E8)
	// 832747A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832747AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832747B0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832747B4: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 832747B8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832747BC: 388BBCB0  addi r4, r11, -0x4350
	ctx.r[4].s64 = ctx.r[11].s64 + -17232;
	// 832747C0: 386AD294  addi r3, r10, -0x2d6c
	ctx.r[3].s64 = ctx.r[10].s64 + -11628;
	// 832747C4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832747C8: 4AFB8709  bl 0x8222ced0
	ctx.lr = 0x832747CC;
	sub_8222CED0(ctx, base);
	// 832747CC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 832747D0: 3869F7F0  addi r3, r9, -0x810
	ctx.r[3].s64 = ctx.r[9].s64 + -2064;
	// 832747D4: 4BA3574D  bl 0x82ca9f20
	ctx.lr = 0x832747D8;
	sub_82CA9F20(ctx, base);
	// 832747D8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832747DC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832747E0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832747E4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832747E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832747E8 size=64
    let mut pc: u32 = 0x832747E8;
    'dispatch: loop {
        match pc {
            0x832747E8 => {
    //   block [0x832747E8..0x83274828)
	// 832747E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832747EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832747F0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832747F4: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 832747F8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832747FC: 388BBCE4  addi r4, r11, -0x431c
	ctx.r[4].s64 = ctx.r[11].s64 + -17180;
	// 83274800: 386AD298  addi r3, r10, -0x2d68
	ctx.r[3].s64 = ctx.r[10].s64 + -11624;
	// 83274804: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83274808: 4AFB86C9  bl 0x8222ced0
	ctx.lr = 0x8327480C;
	sub_8222CED0(ctx, base);
	// 8327480C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83274810: 3869F800  addi r3, r9, -0x800
	ctx.r[3].s64 = ctx.r[9].s64 + -2048;
	// 83274814: 4BA3570D  bl 0x82ca9f20
	ctx.lr = 0x83274818;
	sub_82CA9F20(ctx, base);
	// 83274818: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327481C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83274820: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83274824: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83274828(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83274828 size=64
    let mut pc: u32 = 0x83274828;
    'dispatch: loop {
        match pc {
            0x83274828 => {
    //   block [0x83274828..0x83274868)
	// 83274828: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327482C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83274830: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83274834: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 83274838: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327483C: 388BBD18  addi r4, r11, -0x42e8
	ctx.r[4].s64 = ctx.r[11].s64 + -17128;
	// 83274840: 386AD29C  addi r3, r10, -0x2d64
	ctx.r[3].s64 = ctx.r[10].s64 + -11620;
	// 83274844: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83274848: 4AFB8689  bl 0x8222ced0
	ctx.lr = 0x8327484C;
	sub_8222CED0(ctx, base);
	// 8327484C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83274850: 3869F810  addi r3, r9, -0x7f0
	ctx.r[3].s64 = ctx.r[9].s64 + -2032;
	// 83274854: 4BA356CD  bl 0x82ca9f20
	ctx.lr = 0x83274858;
	sub_82CA9F20(ctx, base);
	// 83274858: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327485C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83274860: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83274864: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83274868(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83274868 size=64
    let mut pc: u32 = 0x83274868;
    'dispatch: loop {
        match pc {
            0x83274868 => {
    //   block [0x83274868..0x832748A8)
	// 83274868: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327486C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83274870: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83274874: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 83274878: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327487C: 388BBD4C  addi r4, r11, -0x42b4
	ctx.r[4].s64 = ctx.r[11].s64 + -17076;
	// 83274880: 386AD2A0  addi r3, r10, -0x2d60
	ctx.r[3].s64 = ctx.r[10].s64 + -11616;
	// 83274884: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83274888: 4AFB8649  bl 0x8222ced0
	ctx.lr = 0x8327488C;
	sub_8222CED0(ctx, base);
	// 8327488C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83274890: 3869F820  addi r3, r9, -0x7e0
	ctx.r[3].s64 = ctx.r[9].s64 + -2016;
	// 83274894: 4BA3568D  bl 0x82ca9f20
	ctx.lr = 0x83274898;
	sub_82CA9F20(ctx, base);
	// 83274898: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327489C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832748A0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832748A4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832748A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832748A8 size=64
    let mut pc: u32 = 0x832748A8;
    'dispatch: loop {
        match pc {
            0x832748A8 => {
    //   block [0x832748A8..0x832748E8)
	// 832748A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832748AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832748B0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832748B4: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 832748B8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832748BC: 388BBD88  addi r4, r11, -0x4278
	ctx.r[4].s64 = ctx.r[11].s64 + -17016;
	// 832748C0: 386AD2A4  addi r3, r10, -0x2d5c
	ctx.r[3].s64 = ctx.r[10].s64 + -11612;
	// 832748C4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832748C8: 4AFB8609  bl 0x8222ced0
	ctx.lr = 0x832748CC;
	sub_8222CED0(ctx, base);
	// 832748CC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 832748D0: 3869F830  addi r3, r9, -0x7d0
	ctx.r[3].s64 = ctx.r[9].s64 + -2000;
	// 832748D4: 4BA3564D  bl 0x82ca9f20
	ctx.lr = 0x832748D8;
	sub_82CA9F20(ctx, base);
	// 832748D8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832748DC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832748E0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832748E4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832748E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832748E8 size=64
    let mut pc: u32 = 0x832748E8;
    'dispatch: loop {
        match pc {
            0x832748E8 => {
    //   block [0x832748E8..0x83274928)
	// 832748E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832748EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832748F0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832748F4: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 832748F8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832748FC: 388BBDC4  addi r4, r11, -0x423c
	ctx.r[4].s64 = ctx.r[11].s64 + -16956;
	// 83274900: 386AD2A8  addi r3, r10, -0x2d58
	ctx.r[3].s64 = ctx.r[10].s64 + -11608;
	// 83274904: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83274908: 4AFB85C9  bl 0x8222ced0
	ctx.lr = 0x8327490C;
	sub_8222CED0(ctx, base);
	// 8327490C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83274910: 3869F840  addi r3, r9, -0x7c0
	ctx.r[3].s64 = ctx.r[9].s64 + -1984;
	// 83274914: 4BA3560D  bl 0x82ca9f20
	ctx.lr = 0x83274918;
	sub_82CA9F20(ctx, base);
	// 83274918: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327491C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83274920: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83274924: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83274928(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83274928 size=64
    let mut pc: u32 = 0x83274928;
    'dispatch: loop {
        match pc {
            0x83274928 => {
    //   block [0x83274928..0x83274968)
	// 83274928: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327492C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83274930: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83274934: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 83274938: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327493C: 388BBDF4  addi r4, r11, -0x420c
	ctx.r[4].s64 = ctx.r[11].s64 + -16908;
	// 83274940: 386AD2AC  addi r3, r10, -0x2d54
	ctx.r[3].s64 = ctx.r[10].s64 + -11604;
	// 83274944: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83274948: 4AFB8589  bl 0x8222ced0
	ctx.lr = 0x8327494C;
	sub_8222CED0(ctx, base);
	// 8327494C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83274950: 3869F850  addi r3, r9, -0x7b0
	ctx.r[3].s64 = ctx.r[9].s64 + -1968;
	// 83274954: 4BA355CD  bl 0x82ca9f20
	ctx.lr = 0x83274958;
	sub_82CA9F20(ctx, base);
	// 83274958: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327495C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83274960: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83274964: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83274968(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83274968 size=64
    let mut pc: u32 = 0x83274968;
    'dispatch: loop {
        match pc {
            0x83274968 => {
    //   block [0x83274968..0x832749A8)
	// 83274968: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327496C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83274970: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83274974: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 83274978: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327497C: 388BA9D4  addi r4, r11, -0x562c
	ctx.r[4].s64 = ctx.r[11].s64 + -22060;
	// 83274980: 386AD2B0  addi r3, r10, -0x2d50
	ctx.r[3].s64 = ctx.r[10].s64 + -11600;
	// 83274984: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83274988: 4AFB8549  bl 0x8222ced0
	ctx.lr = 0x8327498C;
	sub_8222CED0(ctx, base);
	// 8327498C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83274990: 3869F860  addi r3, r9, -0x7a0
	ctx.r[3].s64 = ctx.r[9].s64 + -1952;
	// 83274994: 4BA3558D  bl 0x82ca9f20
	ctx.lr = 0x83274998;
	sub_82CA9F20(ctx, base);
	// 83274998: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327499C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832749A0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832749A4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832749A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832749A8 size=64
    let mut pc: u32 = 0x832749A8;
    'dispatch: loop {
        match pc {
            0x832749A8 => {
    //   block [0x832749A8..0x832749E8)
	// 832749A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832749AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832749B0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832749B4: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 832749B8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832749BC: 388B29F4  addi r4, r11, 0x29f4
	ctx.r[4].s64 = ctx.r[11].s64 + 10740;
	// 832749C0: 386AD2B4  addi r3, r10, -0x2d4c
	ctx.r[3].s64 = ctx.r[10].s64 + -11596;
	// 832749C4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832749C8: 4AFB8509  bl 0x8222ced0
	ctx.lr = 0x832749CC;
	sub_8222CED0(ctx, base);
	// 832749CC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 832749D0: 3869F870  addi r3, r9, -0x790
	ctx.r[3].s64 = ctx.r[9].s64 + -1936;
	// 832749D4: 4BA3554D  bl 0x82ca9f20
	ctx.lr = 0x832749D8;
	sub_82CA9F20(ctx, base);
	// 832749D8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832749DC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832749E0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832749E4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832749E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832749E8 size=64
    let mut pc: u32 = 0x832749E8;
    'dispatch: loop {
        match pc {
            0x832749E8 => {
    //   block [0x832749E8..0x83274A28)
	// 832749E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832749EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832749F0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832749F4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 832749F8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832749FC: 388B052C  addi r4, r11, 0x52c
	ctx.r[4].s64 = ctx.r[11].s64 + 1324;
	// 83274A00: 386AD2B8  addi r3, r10, -0x2d48
	ctx.r[3].s64 = ctx.r[10].s64 + -11592;
	// 83274A04: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83274A08: 4AFB84C9  bl 0x8222ced0
	ctx.lr = 0x83274A0C;
	sub_8222CED0(ctx, base);
	// 83274A0C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83274A10: 3869F880  addi r3, r9, -0x780
	ctx.r[3].s64 = ctx.r[9].s64 + -1920;
	// 83274A14: 4BA3550D  bl 0x82ca9f20
	ctx.lr = 0x83274A18;
	sub_82CA9F20(ctx, base);
	// 83274A18: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83274A1C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83274A20: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83274A24: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83274A28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83274A28 size=64
    let mut pc: u32 = 0x83274A28;
    'dispatch: loop {
        match pc {
            0x83274A28 => {
    //   block [0x83274A28..0x83274A68)
	// 83274A28: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83274A2C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83274A30: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83274A34: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 83274A38: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83274A3C: 388BBE88  addi r4, r11, -0x4178
	ctx.r[4].s64 = ctx.r[11].s64 + -16760;
	// 83274A40: 386AD2BC  addi r3, r10, -0x2d44
	ctx.r[3].s64 = ctx.r[10].s64 + -11588;
	// 83274A44: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83274A48: 4AFB8489  bl 0x8222ced0
	ctx.lr = 0x83274A4C;
	sub_8222CED0(ctx, base);
	// 83274A4C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83274A50: 3869F890  addi r3, r9, -0x770
	ctx.r[3].s64 = ctx.r[9].s64 + -1904;
	// 83274A54: 4BA354CD  bl 0x82ca9f20
	ctx.lr = 0x83274A58;
	sub_82CA9F20(ctx, base);
	// 83274A58: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83274A5C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83274A60: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83274A64: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83274A68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83274A68 size=64
    let mut pc: u32 = 0x83274A68;
    'dispatch: loop {
        match pc {
            0x83274A68 => {
    //   block [0x83274A68..0x83274AA8)
	// 83274A68: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83274A6C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83274A70: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83274A74: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 83274A78: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83274A7C: 388BBE94  addi r4, r11, -0x416c
	ctx.r[4].s64 = ctx.r[11].s64 + -16748;
	// 83274A80: 386AD2C0  addi r3, r10, -0x2d40
	ctx.r[3].s64 = ctx.r[10].s64 + -11584;
	// 83274A84: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83274A88: 4AFB8449  bl 0x8222ced0
	ctx.lr = 0x83274A8C;
	sub_8222CED0(ctx, base);
	// 83274A8C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83274A90: 3869F8A0  addi r3, r9, -0x760
	ctx.r[3].s64 = ctx.r[9].s64 + -1888;
	// 83274A94: 4BA3548D  bl 0x82ca9f20
	ctx.lr = 0x83274A98;
	sub_82CA9F20(ctx, base);
	// 83274A98: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83274A9C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83274AA0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83274AA4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83274AA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83274AA8 size=64
    let mut pc: u32 = 0x83274AA8;
    'dispatch: loop {
        match pc {
            0x83274AA8 => {
    //   block [0x83274AA8..0x83274AE8)
	// 83274AA8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83274AAC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83274AB0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83274AB4: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 83274AB8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83274ABC: 388BBEA0  addi r4, r11, -0x4160
	ctx.r[4].s64 = ctx.r[11].s64 + -16736;
	// 83274AC0: 386AD2C4  addi r3, r10, -0x2d3c
	ctx.r[3].s64 = ctx.r[10].s64 + -11580;
	// 83274AC4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83274AC8: 4AFB8409  bl 0x8222ced0
	ctx.lr = 0x83274ACC;
	sub_8222CED0(ctx, base);
	// 83274ACC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83274AD0: 3869F8B0  addi r3, r9, -0x750
	ctx.r[3].s64 = ctx.r[9].s64 + -1872;
	// 83274AD4: 4BA3544D  bl 0x82ca9f20
	ctx.lr = 0x83274AD8;
	sub_82CA9F20(ctx, base);
	// 83274AD8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83274ADC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83274AE0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83274AE4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83274AE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83274AE8 size=64
    let mut pc: u32 = 0x83274AE8;
    'dispatch: loop {
        match pc {
            0x83274AE8 => {
    //   block [0x83274AE8..0x83274B28)
	// 83274AE8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83274AEC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83274AF0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83274AF4: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 83274AF8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83274AFC: 388BBEB0  addi r4, r11, -0x4150
	ctx.r[4].s64 = ctx.r[11].s64 + -16720;
	// 83274B00: 386AD2C8  addi r3, r10, -0x2d38
	ctx.r[3].s64 = ctx.r[10].s64 + -11576;
	// 83274B04: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83274B08: 4AFB83C9  bl 0x8222ced0
	ctx.lr = 0x83274B0C;
	sub_8222CED0(ctx, base);
	// 83274B0C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83274B10: 3869F8C0  addi r3, r9, -0x740
	ctx.r[3].s64 = ctx.r[9].s64 + -1856;
	// 83274B14: 4BA3540D  bl 0x82ca9f20
	ctx.lr = 0x83274B18;
	sub_82CA9F20(ctx, base);
	// 83274B18: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83274B1C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83274B20: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83274B24: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83274B28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83274B28 size=64
    let mut pc: u32 = 0x83274B28;
    'dispatch: loop {
        match pc {
            0x83274B28 => {
    //   block [0x83274B28..0x83274B68)
	// 83274B28: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83274B2C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83274B30: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83274B34: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 83274B38: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83274B3C: 388BBEBC  addi r4, r11, -0x4144
	ctx.r[4].s64 = ctx.r[11].s64 + -16708;
	// 83274B40: 386AD2CC  addi r3, r10, -0x2d34
	ctx.r[3].s64 = ctx.r[10].s64 + -11572;
	// 83274B44: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83274B48: 4AFB8389  bl 0x8222ced0
	ctx.lr = 0x83274B4C;
	sub_8222CED0(ctx, base);
	// 83274B4C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83274B50: 3869F8D0  addi r3, r9, -0x730
	ctx.r[3].s64 = ctx.r[9].s64 + -1840;
	// 83274B54: 4BA353CD  bl 0x82ca9f20
	ctx.lr = 0x83274B58;
	sub_82CA9F20(ctx, base);
	// 83274B58: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83274B5C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83274B60: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83274B64: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83274B68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83274B68 size=64
    let mut pc: u32 = 0x83274B68;
    'dispatch: loop {
        match pc {
            0x83274B68 => {
    //   block [0x83274B68..0x83274BA8)
	// 83274B68: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83274B6C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83274B70: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83274B74: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 83274B78: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83274B7C: 388BBEC8  addi r4, r11, -0x4138
	ctx.r[4].s64 = ctx.r[11].s64 + -16696;
	// 83274B80: 386AD2D0  addi r3, r10, -0x2d30
	ctx.r[3].s64 = ctx.r[10].s64 + -11568;
	// 83274B84: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83274B88: 4AFB8349  bl 0x8222ced0
	ctx.lr = 0x83274B8C;
	sub_8222CED0(ctx, base);
	// 83274B8C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83274B90: 3869F8E0  addi r3, r9, -0x720
	ctx.r[3].s64 = ctx.r[9].s64 + -1824;
	// 83274B94: 4BA3538D  bl 0x82ca9f20
	ctx.lr = 0x83274B98;
	sub_82CA9F20(ctx, base);
	// 83274B98: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83274B9C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83274BA0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83274BA4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83274BA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83274BA8 size=64
    let mut pc: u32 = 0x83274BA8;
    'dispatch: loop {
        match pc {
            0x83274BA8 => {
    //   block [0x83274BA8..0x83274BE8)
	// 83274BA8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83274BAC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83274BB0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83274BB4: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 83274BB8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83274BBC: 388BBEDC  addi r4, r11, -0x4124
	ctx.r[4].s64 = ctx.r[11].s64 + -16676;
	// 83274BC0: 386AD2D4  addi r3, r10, -0x2d2c
	ctx.r[3].s64 = ctx.r[10].s64 + -11564;
	// 83274BC4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83274BC8: 4AFB8309  bl 0x8222ced0
	ctx.lr = 0x83274BCC;
	sub_8222CED0(ctx, base);
	// 83274BCC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83274BD0: 3869F8F0  addi r3, r9, -0x710
	ctx.r[3].s64 = ctx.r[9].s64 + -1808;
	// 83274BD4: 4BA3534D  bl 0x82ca9f20
	ctx.lr = 0x83274BD8;
	sub_82CA9F20(ctx, base);
	// 83274BD8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83274BDC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83274BE0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83274BE4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83274BE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83274BE8 size=64
    let mut pc: u32 = 0x83274BE8;
    'dispatch: loop {
        match pc {
            0x83274BE8 => {
    //   block [0x83274BE8..0x83274C28)
	// 83274BE8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83274BEC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83274BF0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83274BF4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83274BF8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83274BFC: 388B052C  addi r4, r11, 0x52c
	ctx.r[4].s64 = ctx.r[11].s64 + 1324;
	// 83274C00: 386AD2D8  addi r3, r10, -0x2d28
	ctx.r[3].s64 = ctx.r[10].s64 + -11560;
	// 83274C04: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83274C08: 4AFB82C9  bl 0x8222ced0
	ctx.lr = 0x83274C0C;
	sub_8222CED0(ctx, base);
	// 83274C0C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83274C10: 3869F900  addi r3, r9, -0x700
	ctx.r[3].s64 = ctx.r[9].s64 + -1792;
	// 83274C14: 4BA3530D  bl 0x82ca9f20
	ctx.lr = 0x83274C18;
	sub_82CA9F20(ctx, base);
	// 83274C18: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83274C1C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83274C20: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83274C24: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83274C28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83274C28 size=64
    let mut pc: u32 = 0x83274C28;
    'dispatch: loop {
        match pc {
            0x83274C28 => {
    //   block [0x83274C28..0x83274C68)
	// 83274C28: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83274C2C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83274C30: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83274C34: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83274C38: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83274C3C: 388B1AD4  addi r4, r11, 0x1ad4
	ctx.r[4].s64 = ctx.r[11].s64 + 6868;
	// 83274C40: 386AD2DC  addi r3, r10, -0x2d24
	ctx.r[3].s64 = ctx.r[10].s64 + -11556;
	// 83274C44: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83274C48: 4AFB8289  bl 0x8222ced0
	ctx.lr = 0x83274C4C;
	sub_8222CED0(ctx, base);
	// 83274C4C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83274C50: 3869F910  addi r3, r9, -0x6f0
	ctx.r[3].s64 = ctx.r[9].s64 + -1776;
	// 83274C54: 4BA352CD  bl 0x82ca9f20
	ctx.lr = 0x83274C58;
	sub_82CA9F20(ctx, base);
	// 83274C58: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83274C5C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83274C60: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83274C64: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83274C68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83274C68 size=64
    let mut pc: u32 = 0x83274C68;
    'dispatch: loop {
        match pc {
            0x83274C68 => {
    //   block [0x83274C68..0x83274CA8)
	// 83274C68: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83274C6C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83274C70: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83274C74: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83274C78: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83274C7C: 388B1ADC  addi r4, r11, 0x1adc
	ctx.r[4].s64 = ctx.r[11].s64 + 6876;
	// 83274C80: 386AD2E0  addi r3, r10, -0x2d20
	ctx.r[3].s64 = ctx.r[10].s64 + -11552;
	// 83274C84: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83274C88: 4AFB8249  bl 0x8222ced0
	ctx.lr = 0x83274C8C;
	sub_8222CED0(ctx, base);
	// 83274C8C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83274C90: 3869F920  addi r3, r9, -0x6e0
	ctx.r[3].s64 = ctx.r[9].s64 + -1760;
	// 83274C94: 4BA3528D  bl 0x82ca9f20
	ctx.lr = 0x83274C98;
	sub_82CA9F20(ctx, base);
	// 83274C98: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83274C9C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83274CA0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83274CA4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83274CA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83274CA8 size=64
    let mut pc: u32 = 0x83274CA8;
    'dispatch: loop {
        match pc {
            0x83274CA8 => {
    //   block [0x83274CA8..0x83274CE8)
	// 83274CA8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83274CAC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83274CB0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83274CB4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83274CB8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83274CBC: 388B1AF0  addi r4, r11, 0x1af0
	ctx.r[4].s64 = ctx.r[11].s64 + 6896;
	// 83274CC0: 386AD2E4  addi r3, r10, -0x2d1c
	ctx.r[3].s64 = ctx.r[10].s64 + -11548;
	// 83274CC4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83274CC8: 4AFB8209  bl 0x8222ced0
	ctx.lr = 0x83274CCC;
	sub_8222CED0(ctx, base);
	// 83274CCC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83274CD0: 3869F930  addi r3, r9, -0x6d0
	ctx.r[3].s64 = ctx.r[9].s64 + -1744;
	// 83274CD4: 4BA3524D  bl 0x82ca9f20
	ctx.lr = 0x83274CD8;
	sub_82CA9F20(ctx, base);
	// 83274CD8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83274CDC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83274CE0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83274CE4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83274CE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83274CE8 size=56
    let mut pc: u32 = 0x83274CE8;
    'dispatch: loop {
        match pc {
            0x83274CE8 => {
    //   block [0x83274CE8..0x83274D20)
	// 83274CE8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83274CEC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83274CF0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83274CF4: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 83274CF8: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83274CFC: 386BD418  addi r3, r11, -0x2be8
	ctx.r[3].s64 = ctx.r[11].s64 + -11240;
	// 83274D00: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83274D04: 4AF7F055  bl 0x821f3d58
	ctx.lr = 0x83274D08;
	sub_821F3D58(ctx, base);
	// 83274D08: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83274D0C: 906AD2E8  stw r3, -0x2d18(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-11544 as u32), ctx.r[3].u32 ) };
	// 83274D10: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83274D14: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83274D18: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83274D1C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83274D20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83274D20 size=56
    let mut pc: u32 = 0x83274D20;
    'dispatch: loop {
        match pc {
            0x83274D20 => {
    //   block [0x83274D20..0x83274D58)
	// 83274D20: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83274D24: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83274D28: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83274D2C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83274D30: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83274D34: 386B42C8  addi r3, r11, 0x42c8
	ctx.r[3].s64 = ctx.r[11].s64 + 17096;
	// 83274D38: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83274D3C: 4AF7F01D  bl 0x821f3d58
	ctx.lr = 0x83274D40;
	sub_821F3D58(ctx, base);
	// 83274D40: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83274D44: 906AD2EC  stw r3, -0x2d14(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-11540 as u32), ctx.r[3].u32 ) };
	// 83274D48: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83274D4C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83274D50: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83274D54: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83274D58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83274D58 size=56
    let mut pc: u32 = 0x83274D58;
    'dispatch: loop {
        match pc {
            0x83274D58 => {
    //   block [0x83274D58..0x83274D90)
	// 83274D58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83274D5C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83274D60: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83274D64: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83274D68: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83274D6C: 386B42DC  addi r3, r11, 0x42dc
	ctx.r[3].s64 = ctx.r[11].s64 + 17116;
	// 83274D70: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83274D74: 4AF7EFE5  bl 0x821f3d58
	ctx.lr = 0x83274D78;
	sub_821F3D58(ctx, base);
	// 83274D78: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83274D7C: 906AD2F0  stw r3, -0x2d10(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-11536 as u32), ctx.r[3].u32 ) };
	// 83274D80: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83274D84: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83274D88: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83274D8C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83274D90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83274D90 size=56
    let mut pc: u32 = 0x83274D90;
    'dispatch: loop {
        match pc {
            0x83274D90 => {
    //   block [0x83274D90..0x83274DC8)
	// 83274D90: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83274D94: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83274D98: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83274D9C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83274DA0: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83274DA4: 386B42E8  addi r3, r11, 0x42e8
	ctx.r[3].s64 = ctx.r[11].s64 + 17128;
	// 83274DA8: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83274DAC: 4AF7EFAD  bl 0x821f3d58
	ctx.lr = 0x83274DB0;
	sub_821F3D58(ctx, base);
	// 83274DB0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83274DB4: 906AD2F4  stw r3, -0x2d0c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-11532 as u32), ctx.r[3].u32 ) };
	// 83274DB8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83274DBC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83274DC0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83274DC4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83274DC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83274DC8 size=56
    let mut pc: u32 = 0x83274DC8;
    'dispatch: loop {
        match pc {
            0x83274DC8 => {
    //   block [0x83274DC8..0x83274E00)
	// 83274DC8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83274DCC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83274DD0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83274DD4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83274DD8: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83274DDC: 386B4300  addi r3, r11, 0x4300
	ctx.r[3].s64 = ctx.r[11].s64 + 17152;
	// 83274DE0: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83274DE4: 4AF7EF75  bl 0x821f3d58
	ctx.lr = 0x83274DE8;
	sub_821F3D58(ctx, base);
	// 83274DE8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83274DEC: 906AD2F8  stw r3, -0x2d08(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-11528 as u32), ctx.r[3].u32 ) };
	// 83274DF0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83274DF4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83274DF8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83274DFC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83274E00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83274E00 size=56
    let mut pc: u32 = 0x83274E00;
    'dispatch: loop {
        match pc {
            0x83274E00 => {
    //   block [0x83274E00..0x83274E38)
	// 83274E00: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83274E04: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83274E08: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83274E0C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83274E10: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83274E14: 386B4310  addi r3, r11, 0x4310
	ctx.r[3].s64 = ctx.r[11].s64 + 17168;
	// 83274E18: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83274E1C: 4AF7EF3D  bl 0x821f3d58
	ctx.lr = 0x83274E20;
	sub_821F3D58(ctx, base);
	// 83274E20: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83274E24: 906AD2FC  stw r3, -0x2d04(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-11524 as u32), ctx.r[3].u32 ) };
	// 83274E28: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83274E2C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83274E30: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83274E34: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83274E38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83274E38 size=56
    let mut pc: u32 = 0x83274E38;
    'dispatch: loop {
        match pc {
            0x83274E38 => {
    //   block [0x83274E38..0x83274E70)
	// 83274E38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83274E3C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83274E40: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83274E44: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83274E48: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83274E4C: 386B4328  addi r3, r11, 0x4328
	ctx.r[3].s64 = ctx.r[11].s64 + 17192;
	// 83274E50: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83274E54: 4AF7EF05  bl 0x821f3d58
	ctx.lr = 0x83274E58;
	sub_821F3D58(ctx, base);
	// 83274E58: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83274E5C: 906AD300  stw r3, -0x2d00(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-11520 as u32), ctx.r[3].u32 ) };
	// 83274E60: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83274E64: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83274E68: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83274E6C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83274E70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83274E70 size=56
    let mut pc: u32 = 0x83274E70;
    'dispatch: loop {
        match pc {
            0x83274E70 => {
    //   block [0x83274E70..0x83274EA8)
	// 83274E70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83274E74: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83274E78: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83274E7C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83274E80: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83274E84: 386B4338  addi r3, r11, 0x4338
	ctx.r[3].s64 = ctx.r[11].s64 + 17208;
	// 83274E88: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83274E8C: 4AF7EECD  bl 0x821f3d58
	ctx.lr = 0x83274E90;
	sub_821F3D58(ctx, base);
	// 83274E90: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83274E94: 906AD304  stw r3, -0x2cfc(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-11516 as u32), ctx.r[3].u32 ) };
	// 83274E98: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83274E9C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83274EA0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83274EA4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83274EA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83274EA8 size=56
    let mut pc: u32 = 0x83274EA8;
    'dispatch: loop {
        match pc {
            0x83274EA8 => {
    //   block [0x83274EA8..0x83274EE0)
	// 83274EA8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83274EAC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83274EB0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83274EB4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83274EB8: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83274EBC: 386B4350  addi r3, r11, 0x4350
	ctx.r[3].s64 = ctx.r[11].s64 + 17232;
	// 83274EC0: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83274EC4: 4AF7EE95  bl 0x821f3d58
	ctx.lr = 0x83274EC8;
	sub_821F3D58(ctx, base);
	// 83274EC8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83274ECC: 906AD308  stw r3, -0x2cf8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-11512 as u32), ctx.r[3].u32 ) };
	// 83274ED0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83274ED4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83274ED8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83274EDC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83274EE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83274EE0 size=56
    let mut pc: u32 = 0x83274EE0;
    'dispatch: loop {
        match pc {
            0x83274EE0 => {
    //   block [0x83274EE0..0x83274F18)
	// 83274EE0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83274EE4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83274EE8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83274EEC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83274EF0: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83274EF4: 386B4368  addi r3, r11, 0x4368
	ctx.r[3].s64 = ctx.r[11].s64 + 17256;
	// 83274EF8: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83274EFC: 4AF7EE5D  bl 0x821f3d58
	ctx.lr = 0x83274F00;
	sub_821F3D58(ctx, base);
	// 83274F00: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83274F04: 906AD30C  stw r3, -0x2cf4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-11508 as u32), ctx.r[3].u32 ) };
	// 83274F08: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83274F0C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83274F10: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83274F14: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83274F18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83274F18 size=56
    let mut pc: u32 = 0x83274F18;
    'dispatch: loop {
        match pc {
            0x83274F18 => {
    //   block [0x83274F18..0x83274F50)
	// 83274F18: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83274F1C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83274F20: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83274F24: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83274F28: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83274F2C: 386B4380  addi r3, r11, 0x4380
	ctx.r[3].s64 = ctx.r[11].s64 + 17280;
	// 83274F30: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83274F34: 4AF7EE25  bl 0x821f3d58
	ctx.lr = 0x83274F38;
	sub_821F3D58(ctx, base);
	// 83274F38: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83274F3C: 906AD310  stw r3, -0x2cf0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-11504 as u32), ctx.r[3].u32 ) };
	// 83274F40: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83274F44: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83274F48: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83274F4C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83274F50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83274F50 size=56
    let mut pc: u32 = 0x83274F50;
    'dispatch: loop {
        match pc {
            0x83274F50 => {
    //   block [0x83274F50..0x83274F88)
	// 83274F50: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83274F54: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83274F58: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83274F5C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83274F60: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83274F64: 386B4394  addi r3, r11, 0x4394
	ctx.r[3].s64 = ctx.r[11].s64 + 17300;
	// 83274F68: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83274F6C: 4AF7EDED  bl 0x821f3d58
	ctx.lr = 0x83274F70;
	sub_821F3D58(ctx, base);
	// 83274F70: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83274F74: 906AD314  stw r3, -0x2cec(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-11500 as u32), ctx.r[3].u32 ) };
	// 83274F78: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83274F7C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83274F80: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83274F84: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83274F88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83274F88 size=56
    let mut pc: u32 = 0x83274F88;
    'dispatch: loop {
        match pc {
            0x83274F88 => {
    //   block [0x83274F88..0x83274FC0)
	// 83274F88: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83274F8C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83274F90: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83274F94: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83274F98: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83274F9C: 386B43B4  addi r3, r11, 0x43b4
	ctx.r[3].s64 = ctx.r[11].s64 + 17332;
	// 83274FA0: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83274FA4: 4AF7EDB5  bl 0x821f3d58
	ctx.lr = 0x83274FA8;
	sub_821F3D58(ctx, base);
	// 83274FA8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83274FAC: 906AD318  stw r3, -0x2ce8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-11496 as u32), ctx.r[3].u32 ) };
	// 83274FB0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83274FB4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83274FB8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83274FBC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83274FC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83274FC0 size=56
    let mut pc: u32 = 0x83274FC0;
    'dispatch: loop {
        match pc {
            0x83274FC0 => {
    //   block [0x83274FC0..0x83274FF8)
	// 83274FC0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83274FC4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83274FC8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83274FCC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83274FD0: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83274FD4: 386B43C8  addi r3, r11, 0x43c8
	ctx.r[3].s64 = ctx.r[11].s64 + 17352;
	// 83274FD8: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83274FDC: 4AF7ED7D  bl 0x821f3d58
	ctx.lr = 0x83274FE0;
	sub_821F3D58(ctx, base);
	// 83274FE0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83274FE4: 906AD31C  stw r3, -0x2ce4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-11492 as u32), ctx.r[3].u32 ) };
	// 83274FE8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83274FEC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83274FF0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83274FF4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83274FF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83274FF8 size=56
    let mut pc: u32 = 0x83274FF8;
    'dispatch: loop {
        match pc {
            0x83274FF8 => {
    //   block [0x83274FF8..0x83275030)
	// 83274FF8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83274FFC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83275000: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83275004: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83275008: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8327500C: 386B43D4  addi r3, r11, 0x43d4
	ctx.r[3].s64 = ctx.r[11].s64 + 17364;
	// 83275010: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83275014: 4AF7ED45  bl 0x821f3d58
	ctx.lr = 0x83275018;
	sub_821F3D58(ctx, base);
	// 83275018: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327501C: 906AD320  stw r3, -0x2ce0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-11488 as u32), ctx.r[3].u32 ) };
	// 83275020: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83275024: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83275028: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327502C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83275030(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83275030 size=56
    let mut pc: u32 = 0x83275030;
    'dispatch: loop {
        match pc {
            0x83275030 => {
    //   block [0x83275030..0x83275068)
	// 83275030: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83275034: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83275038: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327503C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83275040: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83275044: 386B43E0  addi r3, r11, 0x43e0
	ctx.r[3].s64 = ctx.r[11].s64 + 17376;
	// 83275048: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8327504C: 4AF7ED0D  bl 0x821f3d58
	ctx.lr = 0x83275050;
	sub_821F3D58(ctx, base);
	// 83275050: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83275054: 906AD324  stw r3, -0x2cdc(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-11484 as u32), ctx.r[3].u32 ) };
	// 83275058: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327505C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83275060: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83275064: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83275068(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83275068 size=56
    let mut pc: u32 = 0x83275068;
    'dispatch: loop {
        match pc {
            0x83275068 => {
    //   block [0x83275068..0x832750A0)
	// 83275068: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327506C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83275070: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83275074: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83275078: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8327507C: 386B43F0  addi r3, r11, 0x43f0
	ctx.r[3].s64 = ctx.r[11].s64 + 17392;
	// 83275080: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83275084: 4AF7ECD5  bl 0x821f3d58
	ctx.lr = 0x83275088;
	sub_821F3D58(ctx, base);
	// 83275088: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327508C: 906AD328  stw r3, -0x2cd8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-11480 as u32), ctx.r[3].u32 ) };
	// 83275090: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83275094: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83275098: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327509C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832750A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832750A0 size=56
    let mut pc: u32 = 0x832750A0;
    'dispatch: loop {
        match pc {
            0x832750A0 => {
    //   block [0x832750A0..0x832750D8)
	// 832750A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832750A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832750A8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832750AC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 832750B0: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 832750B4: 386B4400  addi r3, r11, 0x4400
	ctx.r[3].s64 = ctx.r[11].s64 + 17408;
	// 832750B8: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 832750BC: 4AF7EC9D  bl 0x821f3d58
	ctx.lr = 0x832750C0;
	sub_821F3D58(ctx, base);
	// 832750C0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832750C4: 906AD32C  stw r3, -0x2cd4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-11476 as u32), ctx.r[3].u32 ) };
	// 832750C8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832750CC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832750D0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832750D4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832750D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832750D8 size=56
    let mut pc: u32 = 0x832750D8;
    'dispatch: loop {
        match pc {
            0x832750D8 => {
    //   block [0x832750D8..0x83275110)
	// 832750D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832750DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832750E0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832750E4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 832750E8: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 832750EC: 386B4418  addi r3, r11, 0x4418
	ctx.r[3].s64 = ctx.r[11].s64 + 17432;
	// 832750F0: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 832750F4: 4AF7EC65  bl 0x821f3d58
	ctx.lr = 0x832750F8;
	sub_821F3D58(ctx, base);
	// 832750F8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832750FC: 906AD330  stw r3, -0x2cd0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-11472 as u32), ctx.r[3].u32 ) };
	// 83275100: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83275104: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83275108: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327510C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83275110(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83275110 size=56
    let mut pc: u32 = 0x83275110;
    'dispatch: loop {
        match pc {
            0x83275110 => {
    //   block [0x83275110..0x83275148)
	// 83275110: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83275114: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83275118: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327511C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83275120: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83275124: 386B4424  addi r3, r11, 0x4424
	ctx.r[3].s64 = ctx.r[11].s64 + 17444;
	// 83275128: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8327512C: 4AF7EC2D  bl 0x821f3d58
	ctx.lr = 0x83275130;
	sub_821F3D58(ctx, base);
	// 83275130: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83275134: 906AD334  stw r3, -0x2ccc(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-11468 as u32), ctx.r[3].u32 ) };
	// 83275138: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327513C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83275140: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83275144: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83275148(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83275148 size=56
    let mut pc: u32 = 0x83275148;
    'dispatch: loop {
        match pc {
            0x83275148 => {
    //   block [0x83275148..0x83275180)
	// 83275148: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327514C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83275150: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83275154: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83275158: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8327515C: 386B4430  addi r3, r11, 0x4430
	ctx.r[3].s64 = ctx.r[11].s64 + 17456;
	// 83275160: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83275164: 4AF7EBF5  bl 0x821f3d58
	ctx.lr = 0x83275168;
	sub_821F3D58(ctx, base);
	// 83275168: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327516C: 906AD338  stw r3, -0x2cc8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-11464 as u32), ctx.r[3].u32 ) };
	// 83275170: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83275174: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83275178: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327517C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83275180(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83275180 size=56
    let mut pc: u32 = 0x83275180;
    'dispatch: loop {
        match pc {
            0x83275180 => {
    //   block [0x83275180..0x832751B8)
	// 83275180: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83275184: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83275188: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327518C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83275190: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83275194: 386B4440  addi r3, r11, 0x4440
	ctx.r[3].s64 = ctx.r[11].s64 + 17472;
	// 83275198: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8327519C: 4AF7EBBD  bl 0x821f3d58
	ctx.lr = 0x832751A0;
	sub_821F3D58(ctx, base);
	// 832751A0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832751A4: 906AD33C  stw r3, -0x2cc4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-11460 as u32), ctx.r[3].u32 ) };
	// 832751A8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832751AC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832751B0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832751B4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832751B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832751B8 size=64
    let mut pc: u32 = 0x832751B8;
    'dispatch: loop {
        match pc {
            0x832751B8 => {
    //   block [0x832751B8..0x832751F8)
	// 832751B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832751BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832751C0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832751C4: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 832751C8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832751CC: 388B7230  addi r4, r11, 0x7230
	ctx.r[4].s64 = ctx.r[11].s64 + 29232;
	// 832751D0: 386AD340  addi r3, r10, -0x2cc0
	ctx.r[3].s64 = ctx.r[10].s64 + -11456;
	// 832751D4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832751D8: 4AFB7CF9  bl 0x8222ced0
	ctx.lr = 0x832751DC;
	sub_8222CED0(ctx, base);
	// 832751DC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 832751E0: 3869F940  addi r3, r9, -0x6c0
	ctx.r[3].s64 = ctx.r[9].s64 + -1728;
	// 832751E4: 4BA34D3D  bl 0x82ca9f20
	ctx.lr = 0x832751E8;
	sub_82CA9F20(ctx, base);
	// 832751E8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832751EC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832751F0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832751F4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832751F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832751F8 size=64
    let mut pc: u32 = 0x832751F8;
    'dispatch: loop {
        match pc {
            0x832751F8 => {
    //   block [0x832751F8..0x83275238)
	// 832751F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832751FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83275200: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83275204: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 83275208: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327520C: 388BC06C  addi r4, r11, -0x3f94
	ctx.r[4].s64 = ctx.r[11].s64 + -16276;
	// 83275210: 386AD344  addi r3, r10, -0x2cbc
	ctx.r[3].s64 = ctx.r[10].s64 + -11452;
	// 83275214: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83275218: 4AFB7CB9  bl 0x8222ced0
	ctx.lr = 0x8327521C;
	sub_8222CED0(ctx, base);
	// 8327521C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83275220: 3869F950  addi r3, r9, -0x6b0
	ctx.r[3].s64 = ctx.r[9].s64 + -1712;
	// 83275224: 4BA34CFD  bl 0x82ca9f20
	ctx.lr = 0x83275228;
	sub_82CA9F20(ctx, base);
	// 83275228: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327522C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83275230: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83275234: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83275238(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83275238 size=64
    let mut pc: u32 = 0x83275238;
    'dispatch: loop {
        match pc {
            0x83275238 => {
    //   block [0x83275238..0x83275278)
	// 83275238: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327523C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83275240: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83275244: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 83275248: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327524C: 388BC074  addi r4, r11, -0x3f8c
	ctx.r[4].s64 = ctx.r[11].s64 + -16268;
	// 83275250: 386AD348  addi r3, r10, -0x2cb8
	ctx.r[3].s64 = ctx.r[10].s64 + -11448;
	// 83275254: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83275258: 4AFB7C79  bl 0x8222ced0
	ctx.lr = 0x8327525C;
	sub_8222CED0(ctx, base);
	// 8327525C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83275260: 3869F960  addi r3, r9, -0x6a0
	ctx.r[3].s64 = ctx.r[9].s64 + -1696;
	// 83275264: 4BA34CBD  bl 0x82ca9f20
	ctx.lr = 0x83275268;
	sub_82CA9F20(ctx, base);
	// 83275268: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327526C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83275270: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83275274: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83275278(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83275278 size=64
    let mut pc: u32 = 0x83275278;
    'dispatch: loop {
        match pc {
            0x83275278 => {
    //   block [0x83275278..0x832752B8)
	// 83275278: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327527C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83275280: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83275284: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 83275288: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327528C: 388BC07C  addi r4, r11, -0x3f84
	ctx.r[4].s64 = ctx.r[11].s64 + -16260;
	// 83275290: 386AD34C  addi r3, r10, -0x2cb4
	ctx.r[3].s64 = ctx.r[10].s64 + -11444;
	// 83275294: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83275298: 4AFB7C39  bl 0x8222ced0
	ctx.lr = 0x8327529C;
	sub_8222CED0(ctx, base);
	// 8327529C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 832752A0: 3869F970  addi r3, r9, -0x690
	ctx.r[3].s64 = ctx.r[9].s64 + -1680;
	// 832752A4: 4BA34C7D  bl 0x82ca9f20
	ctx.lr = 0x832752A8;
	sub_82CA9F20(ctx, base);
	// 832752A8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832752AC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832752B0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832752B4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832752B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832752B8 size=64
    let mut pc: u32 = 0x832752B8;
    'dispatch: loop {
        match pc {
            0x832752B8 => {
    //   block [0x832752B8..0x832752F8)
	// 832752B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832752BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832752C0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832752C4: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 832752C8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832752CC: 388B0420  addi r4, r11, 0x420
	ctx.r[4].s64 = ctx.r[11].s64 + 1056;
	// 832752D0: 386AD350  addi r3, r10, -0x2cb0
	ctx.r[3].s64 = ctx.r[10].s64 + -11440;
	// 832752D4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832752D8: 4AFB7BF9  bl 0x8222ced0
	ctx.lr = 0x832752DC;
	sub_8222CED0(ctx, base);
	// 832752DC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 832752E0: 3869F980  addi r3, r9, -0x680
	ctx.r[3].s64 = ctx.r[9].s64 + -1664;
	// 832752E4: 4BA34C3D  bl 0x82ca9f20
	ctx.lr = 0x832752E8;
	sub_82CA9F20(ctx, base);
	// 832752E8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832752EC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832752F0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832752F4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832752F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832752F8 size=64
    let mut pc: u32 = 0x832752F8;
    'dispatch: loop {
        match pc {
            0x832752F8 => {
    //   block [0x832752F8..0x83275338)
	// 832752F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832752FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83275300: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83275304: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 83275308: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327530C: 388B9C5C  addi r4, r11, -0x63a4
	ctx.r[4].s64 = ctx.r[11].s64 + -25508;
	// 83275310: 386AD354  addi r3, r10, -0x2cac
	ctx.r[3].s64 = ctx.r[10].s64 + -11436;
	// 83275314: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83275318: 4AFB7BB9  bl 0x8222ced0
	ctx.lr = 0x8327531C;
	sub_8222CED0(ctx, base);
	// 8327531C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83275320: 3869F990  addi r3, r9, -0x670
	ctx.r[3].s64 = ctx.r[9].s64 + -1648;
	// 83275324: 4BA34BFD  bl 0x82ca9f20
	ctx.lr = 0x83275328;
	sub_82CA9F20(ctx, base);
	// 83275328: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327532C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83275330: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83275334: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83275338(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83275338 size=56
    let mut pc: u32 = 0x83275338;
    'dispatch: loop {
        match pc {
            0x83275338 => {
    //   block [0x83275338..0x83275370)
	// 83275338: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327533C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83275340: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83275344: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83275348: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8327534C: 386B42C8  addi r3, r11, 0x42c8
	ctx.r[3].s64 = ctx.r[11].s64 + 17096;
	// 83275350: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83275354: 4AF7EA05  bl 0x821f3d58
	ctx.lr = 0x83275358;
	sub_821F3D58(ctx, base);
	// 83275358: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327535C: 906AD358  stw r3, -0x2ca8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-11432 as u32), ctx.r[3].u32 ) };
	// 83275360: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83275364: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83275368: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327536C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83275370(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83275370 size=56
    let mut pc: u32 = 0x83275370;
    'dispatch: loop {
        match pc {
            0x83275370 => {
    //   block [0x83275370..0x832753A8)
	// 83275370: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83275374: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83275378: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327537C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83275380: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83275384: 386B42DC  addi r3, r11, 0x42dc
	ctx.r[3].s64 = ctx.r[11].s64 + 17116;
	// 83275388: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8327538C: 4AF7E9CD  bl 0x821f3d58
	ctx.lr = 0x83275390;
	sub_821F3D58(ctx, base);
	// 83275390: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83275394: 906AD35C  stw r3, -0x2ca4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-11428 as u32), ctx.r[3].u32 ) };
	// 83275398: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327539C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832753A0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832753A4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832753A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832753A8 size=56
    let mut pc: u32 = 0x832753A8;
    'dispatch: loop {
        match pc {
            0x832753A8 => {
    //   block [0x832753A8..0x832753E0)
	// 832753A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832753AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832753B0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832753B4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 832753B8: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 832753BC: 386B42E8  addi r3, r11, 0x42e8
	ctx.r[3].s64 = ctx.r[11].s64 + 17128;
	// 832753C0: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 832753C4: 4AF7E995  bl 0x821f3d58
	ctx.lr = 0x832753C8;
	sub_821F3D58(ctx, base);
	// 832753C8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832753CC: 906AD360  stw r3, -0x2ca0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-11424 as u32), ctx.r[3].u32 ) };
	// 832753D0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832753D4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832753D8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832753DC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832753E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832753E0 size=56
    let mut pc: u32 = 0x832753E0;
    'dispatch: loop {
        match pc {
            0x832753E0 => {
    //   block [0x832753E0..0x83275418)
	// 832753E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832753E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832753E8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832753EC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 832753F0: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 832753F4: 386B4300  addi r3, r11, 0x4300
	ctx.r[3].s64 = ctx.r[11].s64 + 17152;
	// 832753F8: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 832753FC: 4AF7E95D  bl 0x821f3d58
	ctx.lr = 0x83275400;
	sub_821F3D58(ctx, base);
	// 83275400: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83275404: 906AD364  stw r3, -0x2c9c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-11420 as u32), ctx.r[3].u32 ) };
	// 83275408: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327540C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83275410: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83275414: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83275418(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83275418 size=56
    let mut pc: u32 = 0x83275418;
    'dispatch: loop {
        match pc {
            0x83275418 => {
    //   block [0x83275418..0x83275450)
	// 83275418: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327541C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83275420: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83275424: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83275428: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8327542C: 386B4310  addi r3, r11, 0x4310
	ctx.r[3].s64 = ctx.r[11].s64 + 17168;
	// 83275430: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83275434: 4AF7E925  bl 0x821f3d58
	ctx.lr = 0x83275438;
	sub_821F3D58(ctx, base);
	// 83275438: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327543C: 906AD368  stw r3, -0x2c98(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-11416 as u32), ctx.r[3].u32 ) };
	// 83275440: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83275444: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83275448: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327544C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83275450(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83275450 size=56
    let mut pc: u32 = 0x83275450;
    'dispatch: loop {
        match pc {
            0x83275450 => {
    //   block [0x83275450..0x83275488)
	// 83275450: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83275454: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83275458: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327545C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83275460: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83275464: 386B4328  addi r3, r11, 0x4328
	ctx.r[3].s64 = ctx.r[11].s64 + 17192;
	// 83275468: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8327546C: 4AF7E8ED  bl 0x821f3d58
	ctx.lr = 0x83275470;
	sub_821F3D58(ctx, base);
	// 83275470: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83275474: 906AD36C  stw r3, -0x2c94(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-11412 as u32), ctx.r[3].u32 ) };
	// 83275478: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327547C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83275480: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83275484: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83275488(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83275488 size=56
    let mut pc: u32 = 0x83275488;
    'dispatch: loop {
        match pc {
            0x83275488 => {
    //   block [0x83275488..0x832754C0)
	// 83275488: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327548C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83275490: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83275494: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83275498: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8327549C: 386B4338  addi r3, r11, 0x4338
	ctx.r[3].s64 = ctx.r[11].s64 + 17208;
	// 832754A0: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 832754A4: 4AF7E8B5  bl 0x821f3d58
	ctx.lr = 0x832754A8;
	sub_821F3D58(ctx, base);
	// 832754A8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832754AC: 906AD370  stw r3, -0x2c90(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-11408 as u32), ctx.r[3].u32 ) };
	// 832754B0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832754B4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832754B8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832754BC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832754C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832754C0 size=56
    let mut pc: u32 = 0x832754C0;
    'dispatch: loop {
        match pc {
            0x832754C0 => {
    //   block [0x832754C0..0x832754F8)
	// 832754C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832754C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832754C8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832754CC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 832754D0: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 832754D4: 386B4350  addi r3, r11, 0x4350
	ctx.r[3].s64 = ctx.r[11].s64 + 17232;
	// 832754D8: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 832754DC: 4AF7E87D  bl 0x821f3d58
	ctx.lr = 0x832754E0;
	sub_821F3D58(ctx, base);
	// 832754E0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832754E4: 906AD374  stw r3, -0x2c8c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-11404 as u32), ctx.r[3].u32 ) };
	// 832754E8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832754EC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832754F0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832754F4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832754F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832754F8 size=56
    let mut pc: u32 = 0x832754F8;
    'dispatch: loop {
        match pc {
            0x832754F8 => {
    //   block [0x832754F8..0x83275530)
	// 832754F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832754FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83275500: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83275504: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83275508: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8327550C: 386B4368  addi r3, r11, 0x4368
	ctx.r[3].s64 = ctx.r[11].s64 + 17256;
	// 83275510: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83275514: 4AF7E845  bl 0x821f3d58
	ctx.lr = 0x83275518;
	sub_821F3D58(ctx, base);
	// 83275518: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327551C: 906AD378  stw r3, -0x2c88(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-11400 as u32), ctx.r[3].u32 ) };
	// 83275520: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83275524: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83275528: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327552C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83275530(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83275530 size=56
    let mut pc: u32 = 0x83275530;
    'dispatch: loop {
        match pc {
            0x83275530 => {
    //   block [0x83275530..0x83275568)
	// 83275530: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83275534: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83275538: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327553C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83275540: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83275544: 386B4380  addi r3, r11, 0x4380
	ctx.r[3].s64 = ctx.r[11].s64 + 17280;
	// 83275548: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8327554C: 4AF7E80D  bl 0x821f3d58
	ctx.lr = 0x83275550;
	sub_821F3D58(ctx, base);
	// 83275550: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83275554: 906AD37C  stw r3, -0x2c84(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-11396 as u32), ctx.r[3].u32 ) };
	// 83275558: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327555C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83275560: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83275564: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83275568(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83275568 size=56
    let mut pc: u32 = 0x83275568;
    'dispatch: loop {
        match pc {
            0x83275568 => {
    //   block [0x83275568..0x832755A0)
	// 83275568: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327556C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83275570: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83275574: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83275578: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8327557C: 386B4394  addi r3, r11, 0x4394
	ctx.r[3].s64 = ctx.r[11].s64 + 17300;
	// 83275580: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83275584: 4AF7E7D5  bl 0x821f3d58
	ctx.lr = 0x83275588;
	sub_821F3D58(ctx, base);
	// 83275588: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327558C: 906AD380  stw r3, -0x2c80(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-11392 as u32), ctx.r[3].u32 ) };
	// 83275590: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83275594: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83275598: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327559C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832755A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832755A0 size=56
    let mut pc: u32 = 0x832755A0;
    'dispatch: loop {
        match pc {
            0x832755A0 => {
    //   block [0x832755A0..0x832755D8)
	// 832755A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832755A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832755A8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832755AC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 832755B0: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 832755B4: 386B43B4  addi r3, r11, 0x43b4
	ctx.r[3].s64 = ctx.r[11].s64 + 17332;
	// 832755B8: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 832755BC: 4AF7E79D  bl 0x821f3d58
	ctx.lr = 0x832755C0;
	sub_821F3D58(ctx, base);
	// 832755C0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832755C4: 906AD384  stw r3, -0x2c7c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-11388 as u32), ctx.r[3].u32 ) };
	// 832755C8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832755CC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832755D0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832755D4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832755D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832755D8 size=56
    let mut pc: u32 = 0x832755D8;
    'dispatch: loop {
        match pc {
            0x832755D8 => {
    //   block [0x832755D8..0x83275610)
	// 832755D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832755DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832755E0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832755E4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 832755E8: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 832755EC: 386B43C8  addi r3, r11, 0x43c8
	ctx.r[3].s64 = ctx.r[11].s64 + 17352;
	// 832755F0: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 832755F4: 4AF7E765  bl 0x821f3d58
	ctx.lr = 0x832755F8;
	sub_821F3D58(ctx, base);
	// 832755F8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832755FC: 906AD388  stw r3, -0x2c78(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-11384 as u32), ctx.r[3].u32 ) };
	// 83275600: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83275604: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83275608: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327560C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83275610(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83275610 size=56
    let mut pc: u32 = 0x83275610;
    'dispatch: loop {
        match pc {
            0x83275610 => {
    //   block [0x83275610..0x83275648)
	// 83275610: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83275614: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83275618: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327561C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83275620: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83275624: 386B43D4  addi r3, r11, 0x43d4
	ctx.r[3].s64 = ctx.r[11].s64 + 17364;
	// 83275628: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8327562C: 4AF7E72D  bl 0x821f3d58
	ctx.lr = 0x83275630;
	sub_821F3D58(ctx, base);
	// 83275630: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83275634: 906AD38C  stw r3, -0x2c74(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-11380 as u32), ctx.r[3].u32 ) };
	// 83275638: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327563C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83275640: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83275644: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83275648(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83275648 size=56
    let mut pc: u32 = 0x83275648;
    'dispatch: loop {
        match pc {
            0x83275648 => {
    //   block [0x83275648..0x83275680)
	// 83275648: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327564C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83275650: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83275654: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83275658: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8327565C: 386B43E0  addi r3, r11, 0x43e0
	ctx.r[3].s64 = ctx.r[11].s64 + 17376;
	// 83275660: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83275664: 4AF7E6F5  bl 0x821f3d58
	ctx.lr = 0x83275668;
	sub_821F3D58(ctx, base);
	// 83275668: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327566C: 906AD390  stw r3, -0x2c70(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-11376 as u32), ctx.r[3].u32 ) };
	// 83275670: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83275674: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83275678: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327567C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83275680(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83275680 size=56
    let mut pc: u32 = 0x83275680;
    'dispatch: loop {
        match pc {
            0x83275680 => {
    //   block [0x83275680..0x832756B8)
	// 83275680: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83275684: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83275688: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327568C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83275690: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83275694: 386B43F0  addi r3, r11, 0x43f0
	ctx.r[3].s64 = ctx.r[11].s64 + 17392;
	// 83275698: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8327569C: 4AF7E6BD  bl 0x821f3d58
	ctx.lr = 0x832756A0;
	sub_821F3D58(ctx, base);
	// 832756A0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832756A4: 906AD394  stw r3, -0x2c6c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-11372 as u32), ctx.r[3].u32 ) };
	// 832756A8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832756AC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832756B0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832756B4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832756B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832756B8 size=56
    let mut pc: u32 = 0x832756B8;
    'dispatch: loop {
        match pc {
            0x832756B8 => {
    //   block [0x832756B8..0x832756F0)
	// 832756B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832756BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832756C0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832756C4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 832756C8: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 832756CC: 386B4400  addi r3, r11, 0x4400
	ctx.r[3].s64 = ctx.r[11].s64 + 17408;
	// 832756D0: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 832756D4: 4AF7E685  bl 0x821f3d58
	ctx.lr = 0x832756D8;
	sub_821F3D58(ctx, base);
	// 832756D8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832756DC: 906AD398  stw r3, -0x2c68(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-11368 as u32), ctx.r[3].u32 ) };
	// 832756E0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832756E4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832756E8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832756EC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832756F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832756F0 size=56
    let mut pc: u32 = 0x832756F0;
    'dispatch: loop {
        match pc {
            0x832756F0 => {
    //   block [0x832756F0..0x83275728)
	// 832756F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832756F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832756F8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832756FC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83275700: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83275704: 386B4418  addi r3, r11, 0x4418
	ctx.r[3].s64 = ctx.r[11].s64 + 17432;
	// 83275708: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8327570C: 4AF7E64D  bl 0x821f3d58
	ctx.lr = 0x83275710;
	sub_821F3D58(ctx, base);
	// 83275710: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83275714: 906AD39C  stw r3, -0x2c64(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-11364 as u32), ctx.r[3].u32 ) };
	// 83275718: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327571C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83275720: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83275724: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83275728(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83275728 size=56
    let mut pc: u32 = 0x83275728;
    'dispatch: loop {
        match pc {
            0x83275728 => {
    //   block [0x83275728..0x83275760)
	// 83275728: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327572C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83275730: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83275734: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83275738: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8327573C: 386B4424  addi r3, r11, 0x4424
	ctx.r[3].s64 = ctx.r[11].s64 + 17444;
	// 83275740: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83275744: 4AF7E615  bl 0x821f3d58
	ctx.lr = 0x83275748;
	sub_821F3D58(ctx, base);
	// 83275748: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327574C: 906AD3A0  stw r3, -0x2c60(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-11360 as u32), ctx.r[3].u32 ) };
	// 83275750: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83275754: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83275758: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327575C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83275760(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83275760 size=56
    let mut pc: u32 = 0x83275760;
    'dispatch: loop {
        match pc {
            0x83275760 => {
    //   block [0x83275760..0x83275798)
	// 83275760: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83275764: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83275768: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327576C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83275770: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83275774: 386B4430  addi r3, r11, 0x4430
	ctx.r[3].s64 = ctx.r[11].s64 + 17456;
	// 83275778: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8327577C: 4AF7E5DD  bl 0x821f3d58
	ctx.lr = 0x83275780;
	sub_821F3D58(ctx, base);
	// 83275780: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83275784: 906AD3A4  stw r3, -0x2c5c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-11356 as u32), ctx.r[3].u32 ) };
	// 83275788: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327578C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83275790: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83275794: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83275798(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83275798 size=56
    let mut pc: u32 = 0x83275798;
    'dispatch: loop {
        match pc {
            0x83275798 => {
    //   block [0x83275798..0x832757D0)
	// 83275798: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327579C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832757A0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832757A4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 832757A8: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 832757AC: 386B4440  addi r3, r11, 0x4440
	ctx.r[3].s64 = ctx.r[11].s64 + 17472;
	// 832757B0: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 832757B4: 4AF7E5A5  bl 0x821f3d58
	ctx.lr = 0x832757B8;
	sub_821F3D58(ctx, base);
	// 832757B8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832757BC: 906AD3A8  stw r3, -0x2c58(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-11352 as u32), ctx.r[3].u32 ) };
	// 832757C0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832757C4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832757C8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832757CC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832757D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832757D0 size=64
    let mut pc: u32 = 0x832757D0;
    'dispatch: loop {
        match pc {
            0x832757D0 => {
    //   block [0x832757D0..0x83275810)
	// 832757D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832757D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832757D8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832757DC: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 832757E0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832757E4: 388B07C0  addi r4, r11, 0x7c0
	ctx.r[4].s64 = ctx.r[11].s64 + 1984;
	// 832757E8: 386AD3AC  addi r3, r10, -0x2c54
	ctx.r[3].s64 = ctx.r[10].s64 + -11348;
	// 832757EC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832757F0: 4AFB76E1  bl 0x8222ced0
	ctx.lr = 0x832757F4;
	sub_8222CED0(ctx, base);
	// 832757F4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 832757F8: 3869F9A0  addi r3, r9, -0x660
	ctx.r[3].s64 = ctx.r[9].s64 + -1632;
	// 832757FC: 4BA34725  bl 0x82ca9f20
	ctx.lr = 0x83275800;
	sub_82CA9F20(ctx, base);
	// 83275800: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83275804: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83275808: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327580C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83275810(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83275810 size=64
    let mut pc: u32 = 0x83275810;
    'dispatch: loop {
        match pc {
            0x83275810 => {
    //   block [0x83275810..0x83275850)
	// 83275810: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83275814: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83275818: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327581C: 3D60820C  lis r11, -0x7df4
	ctx.r[11].s64 = -2113142784;
	// 83275820: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83275824: 388BDCDC  addi r4, r11, -0x2324
	ctx.r[4].s64 = ctx.r[11].s64 + -8996;
	// 83275828: 386AD3B0  addi r3, r10, -0x2c50
	ctx.r[3].s64 = ctx.r[10].s64 + -11344;
	// 8327582C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83275830: 4AFB76A1  bl 0x8222ced0
	ctx.lr = 0x83275834;
	sub_8222CED0(ctx, base);
	// 83275834: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83275838: 3869F9B0  addi r3, r9, -0x650
	ctx.r[3].s64 = ctx.r[9].s64 + -1616;
	// 8327583C: 4BA346E5  bl 0x82ca9f20
	ctx.lr = 0x83275840;
	sub_82CA9F20(ctx, base);
	// 83275840: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83275844: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83275848: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327584C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83275850(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83275850 size=64
    let mut pc: u32 = 0x83275850;
    'dispatch: loop {
        match pc {
            0x83275850 => {
    //   block [0x83275850..0x83275890)
	// 83275850: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83275854: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83275858: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327585C: 3D60820C  lis r11, -0x7df4
	ctx.r[11].s64 = -2113142784;
	// 83275860: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83275864: 388BDCF4  addi r4, r11, -0x230c
	ctx.r[4].s64 = ctx.r[11].s64 + -8972;
	// 83275868: 386AD3B4  addi r3, r10, -0x2c4c
	ctx.r[3].s64 = ctx.r[10].s64 + -11340;
	// 8327586C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83275870: 4AFB7661  bl 0x8222ced0
	ctx.lr = 0x83275874;
	sub_8222CED0(ctx, base);
	// 83275874: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83275878: 3869F9C0  addi r3, r9, -0x640
	ctx.r[3].s64 = ctx.r[9].s64 + -1600;
	// 8327587C: 4BA346A5  bl 0x82ca9f20
	ctx.lr = 0x83275880;
	sub_82CA9F20(ctx, base);
	// 83275880: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83275884: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83275888: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327588C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83275890(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83275890 size=64
    let mut pc: u32 = 0x83275890;
    'dispatch: loop {
        match pc {
            0x83275890 => {
    //   block [0x83275890..0x832758D0)
	// 83275890: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83275894: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83275898: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327589C: 3D60820C  lis r11, -0x7df4
	ctx.r[11].s64 = -2113142784;
	// 832758A0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832758A4: 388BDD08  addi r4, r11, -0x22f8
	ctx.r[4].s64 = ctx.r[11].s64 + -8952;
	// 832758A8: 386AD3B8  addi r3, r10, -0x2c48
	ctx.r[3].s64 = ctx.r[10].s64 + -11336;
	// 832758AC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832758B0: 4AFB7621  bl 0x8222ced0
	ctx.lr = 0x832758B4;
	sub_8222CED0(ctx, base);
	// 832758B4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 832758B8: 3869F9D0  addi r3, r9, -0x630
	ctx.r[3].s64 = ctx.r[9].s64 + -1584;
	// 832758BC: 4BA34665  bl 0x82ca9f20
	ctx.lr = 0x832758C0;
	sub_82CA9F20(ctx, base);
	// 832758C0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832758C4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832758C8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832758CC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832758D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832758D0 size=64
    let mut pc: u32 = 0x832758D0;
    'dispatch: loop {
        match pc {
            0x832758D0 => {
    //   block [0x832758D0..0x83275910)
	// 832758D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832758D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832758D8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832758DC: 3D60820C  lis r11, -0x7df4
	ctx.r[11].s64 = -2113142784;
	// 832758E0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832758E4: 388BDD1C  addi r4, r11, -0x22e4
	ctx.r[4].s64 = ctx.r[11].s64 + -8932;
	// 832758E8: 386AD3BC  addi r3, r10, -0x2c44
	ctx.r[3].s64 = ctx.r[10].s64 + -11332;
	// 832758EC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832758F0: 4AFB75E1  bl 0x8222ced0
	ctx.lr = 0x832758F4;
	sub_8222CED0(ctx, base);
	// 832758F4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 832758F8: 3869F9E0  addi r3, r9, -0x620
	ctx.r[3].s64 = ctx.r[9].s64 + -1568;
	// 832758FC: 4BA34625  bl 0x82ca9f20
	ctx.lr = 0x83275900;
	sub_82CA9F20(ctx, base);
	// 83275900: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83275904: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83275908: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327590C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83275910(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83275910 size=64
    let mut pc: u32 = 0x83275910;
    'dispatch: loop {
        match pc {
            0x83275910 => {
    //   block [0x83275910..0x83275950)
	// 83275910: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83275914: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83275918: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327591C: 3D60820C  lis r11, -0x7df4
	ctx.r[11].s64 = -2113142784;
	// 83275920: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83275924: 388BDD30  addi r4, r11, -0x22d0
	ctx.r[4].s64 = ctx.r[11].s64 + -8912;
	// 83275928: 386AD3C0  addi r3, r10, -0x2c40
	ctx.r[3].s64 = ctx.r[10].s64 + -11328;
	// 8327592C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83275930: 4AFB75A1  bl 0x8222ced0
	ctx.lr = 0x83275934;
	sub_8222CED0(ctx, base);
	// 83275934: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83275938: 3869F9F0  addi r3, r9, -0x610
	ctx.r[3].s64 = ctx.r[9].s64 + -1552;
	// 8327593C: 4BA345E5  bl 0x82ca9f20
	ctx.lr = 0x83275940;
	sub_82CA9F20(ctx, base);
	// 83275940: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83275944: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83275948: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327594C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83275950(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83275950 size=64
    let mut pc: u32 = 0x83275950;
    'dispatch: loop {
        match pc {
            0x83275950 => {
    //   block [0x83275950..0x83275990)
	// 83275950: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83275954: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83275958: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327595C: 3D60820C  lis r11, -0x7df4
	ctx.r[11].s64 = -2113142784;
	// 83275960: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83275964: 388BDD44  addi r4, r11, -0x22bc
	ctx.r[4].s64 = ctx.r[11].s64 + -8892;
	// 83275968: 386AD3C4  addi r3, r10, -0x2c3c
	ctx.r[3].s64 = ctx.r[10].s64 + -11324;
	// 8327596C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83275970: 4AFB7561  bl 0x8222ced0
	ctx.lr = 0x83275974;
	sub_8222CED0(ctx, base);
	// 83275974: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83275978: 3869FA00  addi r3, r9, -0x600
	ctx.r[3].s64 = ctx.r[9].s64 + -1536;
	// 8327597C: 4BA345A5  bl 0x82ca9f20
	ctx.lr = 0x83275980;
	sub_82CA9F20(ctx, base);
	// 83275980: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83275984: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83275988: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327598C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83275990(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83275990 size=64
    let mut pc: u32 = 0x83275990;
    'dispatch: loop {
        match pc {
            0x83275990 => {
    //   block [0x83275990..0x832759D0)
	// 83275990: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83275994: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83275998: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327599C: 3D60820C  lis r11, -0x7df4
	ctx.r[11].s64 = -2113142784;
	// 832759A0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832759A4: 388BDD58  addi r4, r11, -0x22a8
	ctx.r[4].s64 = ctx.r[11].s64 + -8872;
	// 832759A8: 386AD3C8  addi r3, r10, -0x2c38
	ctx.r[3].s64 = ctx.r[10].s64 + -11320;
	// 832759AC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832759B0: 4AFB7521  bl 0x8222ced0
	ctx.lr = 0x832759B4;
	sub_8222CED0(ctx, base);
	// 832759B4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 832759B8: 3869FA10  addi r3, r9, -0x5f0
	ctx.r[3].s64 = ctx.r[9].s64 + -1520;
	// 832759BC: 4BA34565  bl 0x82ca9f20
	ctx.lr = 0x832759C0;
	sub_82CA9F20(ctx, base);
	// 832759C0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832759C4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832759C8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832759CC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832759D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832759D0 size=64
    let mut pc: u32 = 0x832759D0;
    'dispatch: loop {
        match pc {
            0x832759D0 => {
    //   block [0x832759D0..0x83275A10)
	// 832759D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832759D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832759D8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832759DC: 3D60820C  lis r11, -0x7df4
	ctx.r[11].s64 = -2113142784;
	// 832759E0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832759E4: 388BDD70  addi r4, r11, -0x2290
	ctx.r[4].s64 = ctx.r[11].s64 + -8848;
	// 832759E8: 386AD3CC  addi r3, r10, -0x2c34
	ctx.r[3].s64 = ctx.r[10].s64 + -11316;
	// 832759EC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832759F0: 4AFB74E1  bl 0x8222ced0
	ctx.lr = 0x832759F4;
	sub_8222CED0(ctx, base);
	// 832759F4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 832759F8: 3869FA20  addi r3, r9, -0x5e0
	ctx.r[3].s64 = ctx.r[9].s64 + -1504;
	// 832759FC: 4BA34525  bl 0x82ca9f20
	ctx.lr = 0x83275A00;
	sub_82CA9F20(ctx, base);
	// 83275A00: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83275A04: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83275A08: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83275A0C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83275A10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83275A10 size=56
    let mut pc: u32 = 0x83275A10;
    'dispatch: loop {
        match pc {
            0x83275A10 => {
    //   block [0x83275A10..0x83275A48)
	// 83275A10: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83275A14: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83275A18: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83275A1C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83275A20: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83275A24: 386B42C8  addi r3, r11, 0x42c8
	ctx.r[3].s64 = ctx.r[11].s64 + 17096;
	// 83275A28: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83275A2C: 4AF7E32D  bl 0x821f3d58
	ctx.lr = 0x83275A30;
	sub_821F3D58(ctx, base);
	// 83275A30: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83275A34: 906AD3D0  stw r3, -0x2c30(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-11312 as u32), ctx.r[3].u32 ) };
	// 83275A38: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83275A3C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83275A40: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83275A44: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83275A48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83275A48 size=56
    let mut pc: u32 = 0x83275A48;
    'dispatch: loop {
        match pc {
            0x83275A48 => {
    //   block [0x83275A48..0x83275A80)
	// 83275A48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83275A4C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83275A50: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83275A54: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83275A58: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83275A5C: 386B42DC  addi r3, r11, 0x42dc
	ctx.r[3].s64 = ctx.r[11].s64 + 17116;
	// 83275A60: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83275A64: 4AF7E2F5  bl 0x821f3d58
	ctx.lr = 0x83275A68;
	sub_821F3D58(ctx, base);
	// 83275A68: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83275A6C: 906AD3D4  stw r3, -0x2c2c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-11308 as u32), ctx.r[3].u32 ) };
	// 83275A70: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83275A74: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83275A78: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83275A7C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83275A80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83275A80 size=56
    let mut pc: u32 = 0x83275A80;
    'dispatch: loop {
        match pc {
            0x83275A80 => {
    //   block [0x83275A80..0x83275AB8)
	// 83275A80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83275A84: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83275A88: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83275A8C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83275A90: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83275A94: 386B42E8  addi r3, r11, 0x42e8
	ctx.r[3].s64 = ctx.r[11].s64 + 17128;
	// 83275A98: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83275A9C: 4AF7E2BD  bl 0x821f3d58
	ctx.lr = 0x83275AA0;
	sub_821F3D58(ctx, base);
	// 83275AA0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83275AA4: 906AD3D8  stw r3, -0x2c28(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-11304 as u32), ctx.r[3].u32 ) };
	// 83275AA8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83275AAC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83275AB0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83275AB4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83275AB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83275AB8 size=56
    let mut pc: u32 = 0x83275AB8;
    'dispatch: loop {
        match pc {
            0x83275AB8 => {
    //   block [0x83275AB8..0x83275AF0)
	// 83275AB8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83275ABC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83275AC0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83275AC4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83275AC8: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83275ACC: 386B4300  addi r3, r11, 0x4300
	ctx.r[3].s64 = ctx.r[11].s64 + 17152;
	// 83275AD0: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83275AD4: 4AF7E285  bl 0x821f3d58
	ctx.lr = 0x83275AD8;
	sub_821F3D58(ctx, base);
	// 83275AD8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83275ADC: 906AD3DC  stw r3, -0x2c24(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-11300 as u32), ctx.r[3].u32 ) };
	// 83275AE0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83275AE4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83275AE8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83275AEC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83275AF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83275AF0 size=56
    let mut pc: u32 = 0x83275AF0;
    'dispatch: loop {
        match pc {
            0x83275AF0 => {
    //   block [0x83275AF0..0x83275B28)
	// 83275AF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83275AF4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83275AF8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83275AFC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83275B00: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83275B04: 386B4310  addi r3, r11, 0x4310
	ctx.r[3].s64 = ctx.r[11].s64 + 17168;
	// 83275B08: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83275B0C: 4AF7E24D  bl 0x821f3d58
	ctx.lr = 0x83275B10;
	sub_821F3D58(ctx, base);
	// 83275B10: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83275B14: 906AD3E0  stw r3, -0x2c20(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-11296 as u32), ctx.r[3].u32 ) };
	// 83275B18: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83275B1C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83275B20: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83275B24: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83275B28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83275B28 size=56
    let mut pc: u32 = 0x83275B28;
    'dispatch: loop {
        match pc {
            0x83275B28 => {
    //   block [0x83275B28..0x83275B60)
	// 83275B28: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83275B2C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83275B30: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83275B34: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83275B38: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83275B3C: 386B4328  addi r3, r11, 0x4328
	ctx.r[3].s64 = ctx.r[11].s64 + 17192;
	// 83275B40: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83275B44: 4AF7E215  bl 0x821f3d58
	ctx.lr = 0x83275B48;
	sub_821F3D58(ctx, base);
	// 83275B48: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83275B4C: 906AD3E4  stw r3, -0x2c1c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-11292 as u32), ctx.r[3].u32 ) };
	// 83275B50: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83275B54: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83275B58: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83275B5C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83275B60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83275B60 size=56
    let mut pc: u32 = 0x83275B60;
    'dispatch: loop {
        match pc {
            0x83275B60 => {
    //   block [0x83275B60..0x83275B98)
	// 83275B60: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83275B64: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83275B68: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83275B6C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83275B70: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83275B74: 386B4338  addi r3, r11, 0x4338
	ctx.r[3].s64 = ctx.r[11].s64 + 17208;
	// 83275B78: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83275B7C: 4AF7E1DD  bl 0x821f3d58
	ctx.lr = 0x83275B80;
	sub_821F3D58(ctx, base);
	// 83275B80: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83275B84: 906AD3E8  stw r3, -0x2c18(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-11288 as u32), ctx.r[3].u32 ) };
	// 83275B88: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83275B8C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83275B90: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83275B94: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83275B98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83275B98 size=56
    let mut pc: u32 = 0x83275B98;
    'dispatch: loop {
        match pc {
            0x83275B98 => {
    //   block [0x83275B98..0x83275BD0)
	// 83275B98: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83275B9C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83275BA0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83275BA4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83275BA8: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83275BAC: 386B4350  addi r3, r11, 0x4350
	ctx.r[3].s64 = ctx.r[11].s64 + 17232;
	// 83275BB0: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83275BB4: 4AF7E1A5  bl 0x821f3d58
	ctx.lr = 0x83275BB8;
	sub_821F3D58(ctx, base);
	// 83275BB8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83275BBC: 906AD3EC  stw r3, -0x2c14(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-11284 as u32), ctx.r[3].u32 ) };
	// 83275BC0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83275BC4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83275BC8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83275BCC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83275BD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83275BD0 size=56
    let mut pc: u32 = 0x83275BD0;
    'dispatch: loop {
        match pc {
            0x83275BD0 => {
    //   block [0x83275BD0..0x83275C08)
	// 83275BD0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83275BD4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83275BD8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83275BDC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83275BE0: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83275BE4: 386B4368  addi r3, r11, 0x4368
	ctx.r[3].s64 = ctx.r[11].s64 + 17256;
	// 83275BE8: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83275BEC: 4AF7E16D  bl 0x821f3d58
	ctx.lr = 0x83275BF0;
	sub_821F3D58(ctx, base);
	// 83275BF0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83275BF4: 906AD3F0  stw r3, -0x2c10(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-11280 as u32), ctx.r[3].u32 ) };
	// 83275BF8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83275BFC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83275C00: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83275C04: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83275C08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83275C08 size=56
    let mut pc: u32 = 0x83275C08;
    'dispatch: loop {
        match pc {
            0x83275C08 => {
    //   block [0x83275C08..0x83275C40)
	// 83275C08: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83275C0C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83275C10: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83275C14: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83275C18: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83275C1C: 386B4380  addi r3, r11, 0x4380
	ctx.r[3].s64 = ctx.r[11].s64 + 17280;
	// 83275C20: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83275C24: 4AF7E135  bl 0x821f3d58
	ctx.lr = 0x83275C28;
	sub_821F3D58(ctx, base);
	// 83275C28: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83275C2C: 906AD3F4  stw r3, -0x2c0c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-11276 as u32), ctx.r[3].u32 ) };
	// 83275C30: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83275C34: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83275C38: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83275C3C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83275C40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83275C40 size=56
    let mut pc: u32 = 0x83275C40;
    'dispatch: loop {
        match pc {
            0x83275C40 => {
    //   block [0x83275C40..0x83275C78)
	// 83275C40: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83275C44: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83275C48: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83275C4C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83275C50: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83275C54: 386B4394  addi r3, r11, 0x4394
	ctx.r[3].s64 = ctx.r[11].s64 + 17300;
	// 83275C58: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83275C5C: 4AF7E0FD  bl 0x821f3d58
	ctx.lr = 0x83275C60;
	sub_821F3D58(ctx, base);
	// 83275C60: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83275C64: 906AD3F8  stw r3, -0x2c08(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-11272 as u32), ctx.r[3].u32 ) };
	// 83275C68: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83275C6C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83275C70: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83275C74: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83275C78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83275C78 size=56
    let mut pc: u32 = 0x83275C78;
    'dispatch: loop {
        match pc {
            0x83275C78 => {
    //   block [0x83275C78..0x83275CB0)
	// 83275C78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83275C7C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83275C80: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83275C84: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83275C88: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83275C8C: 386B43B4  addi r3, r11, 0x43b4
	ctx.r[3].s64 = ctx.r[11].s64 + 17332;
	// 83275C90: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83275C94: 4AF7E0C5  bl 0x821f3d58
	ctx.lr = 0x83275C98;
	sub_821F3D58(ctx, base);
	// 83275C98: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83275C9C: 906AD3FC  stw r3, -0x2c04(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-11268 as u32), ctx.r[3].u32 ) };
	// 83275CA0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83275CA4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83275CA8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83275CAC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


