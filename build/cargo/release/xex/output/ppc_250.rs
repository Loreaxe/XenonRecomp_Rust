pub fn sub_832700F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832700F0 size=56
    let mut pc: u32 = 0x832700F0;
    'dispatch: loop {
        match pc {
            0x832700F0 => {
    //   block [0x832700F0..0x83270128)
	// 832700F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832700F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832700F8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832700FC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83270100: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83270104: 386B4310  addi r3, r11, 0x4310
	ctx.r[3].s64 = ctx.r[11].s64 + 17168;
	// 83270108: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8327010C: 4AF83C4D  bl 0x821f3d58
	ctx.lr = 0x83270110;
	sub_821F3D58(ctx, base);
	// 83270110: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83270114: 906ACDCC  stw r3, -0x3234(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-12852 as u32), ctx.r[3].u32 ) };
	// 83270118: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327011C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83270120: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83270124: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83270128(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83270128 size=56
    let mut pc: u32 = 0x83270128;
    'dispatch: loop {
        match pc {
            0x83270128 => {
    //   block [0x83270128..0x83270160)
	// 83270128: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327012C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83270130: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83270134: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83270138: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8327013C: 386B4328  addi r3, r11, 0x4328
	ctx.r[3].s64 = ctx.r[11].s64 + 17192;
	// 83270140: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83270144: 4AF83C15  bl 0x821f3d58
	ctx.lr = 0x83270148;
	sub_821F3D58(ctx, base);
	// 83270148: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327014C: 906ACDD0  stw r3, -0x3230(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-12848 as u32), ctx.r[3].u32 ) };
	// 83270150: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83270154: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83270158: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327015C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83270160(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83270160 size=56
    let mut pc: u32 = 0x83270160;
    'dispatch: loop {
        match pc {
            0x83270160 => {
    //   block [0x83270160..0x83270198)
	// 83270160: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83270164: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83270168: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327016C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83270170: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83270174: 386B4338  addi r3, r11, 0x4338
	ctx.r[3].s64 = ctx.r[11].s64 + 17208;
	// 83270178: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8327017C: 4AF83BDD  bl 0x821f3d58
	ctx.lr = 0x83270180;
	sub_821F3D58(ctx, base);
	// 83270180: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83270184: 906ACDD4  stw r3, -0x322c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-12844 as u32), ctx.r[3].u32 ) };
	// 83270188: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327018C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83270190: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83270194: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83270198(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83270198 size=56
    let mut pc: u32 = 0x83270198;
    'dispatch: loop {
        match pc {
            0x83270198 => {
    //   block [0x83270198..0x832701D0)
	// 83270198: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327019C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832701A0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832701A4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 832701A8: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 832701AC: 386B4350  addi r3, r11, 0x4350
	ctx.r[3].s64 = ctx.r[11].s64 + 17232;
	// 832701B0: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 832701B4: 4AF83BA5  bl 0x821f3d58
	ctx.lr = 0x832701B8;
	sub_821F3D58(ctx, base);
	// 832701B8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832701BC: 906ACDD8  stw r3, -0x3228(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-12840 as u32), ctx.r[3].u32 ) };
	// 832701C0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832701C4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832701C8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832701CC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832701D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832701D0 size=56
    let mut pc: u32 = 0x832701D0;
    'dispatch: loop {
        match pc {
            0x832701D0 => {
    //   block [0x832701D0..0x83270208)
	// 832701D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832701D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832701D8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832701DC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 832701E0: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 832701E4: 386B4368  addi r3, r11, 0x4368
	ctx.r[3].s64 = ctx.r[11].s64 + 17256;
	// 832701E8: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 832701EC: 4AF83B6D  bl 0x821f3d58
	ctx.lr = 0x832701F0;
	sub_821F3D58(ctx, base);
	// 832701F0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832701F4: 906ACDDC  stw r3, -0x3224(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-12836 as u32), ctx.r[3].u32 ) };
	// 832701F8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832701FC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83270200: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83270204: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83270208(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83270208 size=56
    let mut pc: u32 = 0x83270208;
    'dispatch: loop {
        match pc {
            0x83270208 => {
    //   block [0x83270208..0x83270240)
	// 83270208: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327020C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83270210: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83270214: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83270218: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8327021C: 386B4380  addi r3, r11, 0x4380
	ctx.r[3].s64 = ctx.r[11].s64 + 17280;
	// 83270220: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83270224: 4AF83B35  bl 0x821f3d58
	ctx.lr = 0x83270228;
	sub_821F3D58(ctx, base);
	// 83270228: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327022C: 906ACDE0  stw r3, -0x3220(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-12832 as u32), ctx.r[3].u32 ) };
	// 83270230: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83270234: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83270238: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327023C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83270240(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83270240 size=56
    let mut pc: u32 = 0x83270240;
    'dispatch: loop {
        match pc {
            0x83270240 => {
    //   block [0x83270240..0x83270278)
	// 83270240: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83270244: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83270248: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327024C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83270250: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83270254: 386B4394  addi r3, r11, 0x4394
	ctx.r[3].s64 = ctx.r[11].s64 + 17300;
	// 83270258: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8327025C: 4AF83AFD  bl 0x821f3d58
	ctx.lr = 0x83270260;
	sub_821F3D58(ctx, base);
	// 83270260: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83270264: 906ACDE4  stw r3, -0x321c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-12828 as u32), ctx.r[3].u32 ) };
	// 83270268: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327026C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83270270: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83270274: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83270278(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83270278 size=56
    let mut pc: u32 = 0x83270278;
    'dispatch: loop {
        match pc {
            0x83270278 => {
    //   block [0x83270278..0x832702B0)
	// 83270278: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327027C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83270280: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83270284: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83270288: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8327028C: 386B43B4  addi r3, r11, 0x43b4
	ctx.r[3].s64 = ctx.r[11].s64 + 17332;
	// 83270290: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83270294: 4AF83AC5  bl 0x821f3d58
	ctx.lr = 0x83270298;
	sub_821F3D58(ctx, base);
	// 83270298: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327029C: 906ACDE8  stw r3, -0x3218(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-12824 as u32), ctx.r[3].u32 ) };
	// 832702A0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832702A4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832702A8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832702AC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832702B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832702B0 size=56
    let mut pc: u32 = 0x832702B0;
    'dispatch: loop {
        match pc {
            0x832702B0 => {
    //   block [0x832702B0..0x832702E8)
	// 832702B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832702B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832702B8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832702BC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 832702C0: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 832702C4: 386B43C8  addi r3, r11, 0x43c8
	ctx.r[3].s64 = ctx.r[11].s64 + 17352;
	// 832702C8: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 832702CC: 4AF83A8D  bl 0x821f3d58
	ctx.lr = 0x832702D0;
	sub_821F3D58(ctx, base);
	// 832702D0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832702D4: 906ACDEC  stw r3, -0x3214(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-12820 as u32), ctx.r[3].u32 ) };
	// 832702D8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832702DC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832702E0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832702E4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832702E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832702E8 size=56
    let mut pc: u32 = 0x832702E8;
    'dispatch: loop {
        match pc {
            0x832702E8 => {
    //   block [0x832702E8..0x83270320)
	// 832702E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832702EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832702F0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832702F4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 832702F8: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 832702FC: 386B43D4  addi r3, r11, 0x43d4
	ctx.r[3].s64 = ctx.r[11].s64 + 17364;
	// 83270300: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83270304: 4AF83A55  bl 0x821f3d58
	ctx.lr = 0x83270308;
	sub_821F3D58(ctx, base);
	// 83270308: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327030C: 906ACDF0  stw r3, -0x3210(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-12816 as u32), ctx.r[3].u32 ) };
	// 83270310: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83270314: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83270318: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327031C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83270320(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83270320 size=56
    let mut pc: u32 = 0x83270320;
    'dispatch: loop {
        match pc {
            0x83270320 => {
    //   block [0x83270320..0x83270358)
	// 83270320: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83270324: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83270328: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327032C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83270330: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83270334: 386B43E0  addi r3, r11, 0x43e0
	ctx.r[3].s64 = ctx.r[11].s64 + 17376;
	// 83270338: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8327033C: 4AF83A1D  bl 0x821f3d58
	ctx.lr = 0x83270340;
	sub_821F3D58(ctx, base);
	// 83270340: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83270344: 906ACDF4  stw r3, -0x320c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-12812 as u32), ctx.r[3].u32 ) };
	// 83270348: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327034C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83270350: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83270354: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83270358(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83270358 size=56
    let mut pc: u32 = 0x83270358;
    'dispatch: loop {
        match pc {
            0x83270358 => {
    //   block [0x83270358..0x83270390)
	// 83270358: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327035C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83270360: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83270364: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83270368: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8327036C: 386B43F0  addi r3, r11, 0x43f0
	ctx.r[3].s64 = ctx.r[11].s64 + 17392;
	// 83270370: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83270374: 4AF839E5  bl 0x821f3d58
	ctx.lr = 0x83270378;
	sub_821F3D58(ctx, base);
	// 83270378: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327037C: 906ACDF8  stw r3, -0x3208(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-12808 as u32), ctx.r[3].u32 ) };
	// 83270380: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83270384: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83270388: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327038C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83270390(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83270390 size=56
    let mut pc: u32 = 0x83270390;
    'dispatch: loop {
        match pc {
            0x83270390 => {
    //   block [0x83270390..0x832703C8)
	// 83270390: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83270394: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83270398: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327039C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 832703A0: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 832703A4: 386B4400  addi r3, r11, 0x4400
	ctx.r[3].s64 = ctx.r[11].s64 + 17408;
	// 832703A8: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 832703AC: 4AF839AD  bl 0x821f3d58
	ctx.lr = 0x832703B0;
	sub_821F3D58(ctx, base);
	// 832703B0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832703B4: 906ACDFC  stw r3, -0x3204(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-12804 as u32), ctx.r[3].u32 ) };
	// 832703B8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832703BC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832703C0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832703C4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832703C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832703C8 size=56
    let mut pc: u32 = 0x832703C8;
    'dispatch: loop {
        match pc {
            0x832703C8 => {
    //   block [0x832703C8..0x83270400)
	// 832703C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832703CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832703D0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832703D4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 832703D8: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 832703DC: 386B4418  addi r3, r11, 0x4418
	ctx.r[3].s64 = ctx.r[11].s64 + 17432;
	// 832703E0: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 832703E4: 4AF83975  bl 0x821f3d58
	ctx.lr = 0x832703E8;
	sub_821F3D58(ctx, base);
	// 832703E8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832703EC: 906ACE00  stw r3, -0x3200(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-12800 as u32), ctx.r[3].u32 ) };
	// 832703F0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832703F4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832703F8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832703FC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83270400(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83270400 size=56
    let mut pc: u32 = 0x83270400;
    'dispatch: loop {
        match pc {
            0x83270400 => {
    //   block [0x83270400..0x83270438)
	// 83270400: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83270404: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83270408: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327040C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83270410: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83270414: 386B4424  addi r3, r11, 0x4424
	ctx.r[3].s64 = ctx.r[11].s64 + 17444;
	// 83270418: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8327041C: 4AF8393D  bl 0x821f3d58
	ctx.lr = 0x83270420;
	sub_821F3D58(ctx, base);
	// 83270420: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83270424: 906ACE04  stw r3, -0x31fc(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-12796 as u32), ctx.r[3].u32 ) };
	// 83270428: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327042C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83270430: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83270434: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83270438(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83270438 size=56
    let mut pc: u32 = 0x83270438;
    'dispatch: loop {
        match pc {
            0x83270438 => {
    //   block [0x83270438..0x83270470)
	// 83270438: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327043C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83270440: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83270444: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83270448: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8327044C: 386B4430  addi r3, r11, 0x4430
	ctx.r[3].s64 = ctx.r[11].s64 + 17456;
	// 83270450: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83270454: 4AF83905  bl 0x821f3d58
	ctx.lr = 0x83270458;
	sub_821F3D58(ctx, base);
	// 83270458: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327045C: 906ACE08  stw r3, -0x31f8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-12792 as u32), ctx.r[3].u32 ) };
	// 83270460: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83270464: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83270468: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327046C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83270470(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83270470 size=56
    let mut pc: u32 = 0x83270470;
    'dispatch: loop {
        match pc {
            0x83270470 => {
    //   block [0x83270470..0x832704A8)
	// 83270470: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83270474: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83270478: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327047C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83270480: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83270484: 386B4440  addi r3, r11, 0x4440
	ctx.r[3].s64 = ctx.r[11].s64 + 17472;
	// 83270488: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8327048C: 4AF838CD  bl 0x821f3d58
	ctx.lr = 0x83270490;
	sub_821F3D58(ctx, base);
	// 83270490: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83270494: 906ACE0C  stw r3, -0x31f4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-12788 as u32), ctx.r[3].u32 ) };
	// 83270498: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327049C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832704A0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832704A4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832704A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832704A8 size=64
    let mut pc: u32 = 0x832704A8;
    'dispatch: loop {
        match pc {
            0x832704A8 => {
    //   block [0x832704A8..0x832704E8)
	// 832704A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832704AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832704B0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832704B4: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 832704B8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832704BC: 388B9FD4  addi r4, r11, -0x602c
	ctx.r[4].s64 = ctx.r[11].s64 + -24620;
	// 832704C0: 386ACE10  addi r3, r10, -0x31f0
	ctx.r[3].s64 = ctx.r[10].s64 + -12784;
	// 832704C4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832704C8: 4AFBCA09  bl 0x8222ced0
	ctx.lr = 0x832704CC;
	sub_8222CED0(ctx, base);
	// 832704CC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 832704D0: 3869F060  addi r3, r9, -0xfa0
	ctx.r[3].s64 = ctx.r[9].s64 + -4000;
	// 832704D4: 4BA39A4D  bl 0x82ca9f20
	ctx.lr = 0x832704D8;
	sub_82CA9F20(ctx, base);
	// 832704D8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832704DC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832704E0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832704E4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832704E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832704E8 size=64
    let mut pc: u32 = 0x832704E8;
    'dispatch: loop {
        match pc {
            0x832704E8 => {
    //   block [0x832704E8..0x83270528)
	// 832704E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832704EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832704F0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832704F4: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 832704F8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832704FC: 388BD4FC  addi r4, r11, -0x2b04
	ctx.r[4].s64 = ctx.r[11].s64 + -11012;
	// 83270500: 386ACE14  addi r3, r10, -0x31ec
	ctx.r[3].s64 = ctx.r[10].s64 + -12780;
	// 83270504: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83270508: 4AFBC9C9  bl 0x8222ced0
	ctx.lr = 0x8327050C;
	sub_8222CED0(ctx, base);
	// 8327050C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83270510: 3869F070  addi r3, r9, -0xf90
	ctx.r[3].s64 = ctx.r[9].s64 + -3984;
	// 83270514: 4BA39A0D  bl 0x82ca9f20
	ctx.lr = 0x83270518;
	sub_82CA9F20(ctx, base);
	// 83270518: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327051C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83270520: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83270524: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83270528(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83270528 size=64
    let mut pc: u32 = 0x83270528;
    'dispatch: loop {
        match pc {
            0x83270528 => {
    //   block [0x83270528..0x83270568)
	// 83270528: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327052C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83270530: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83270534: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 83270538: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327053C: 388BA000  addi r4, r11, -0x6000
	ctx.r[4].s64 = ctx.r[11].s64 + -24576;
	// 83270540: 386ACE18  addi r3, r10, -0x31e8
	ctx.r[3].s64 = ctx.r[10].s64 + -12776;
	// 83270544: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83270548: 4AFBC989  bl 0x8222ced0
	ctx.lr = 0x8327054C;
	sub_8222CED0(ctx, base);
	// 8327054C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83270550: 3869F080  addi r3, r9, -0xf80
	ctx.r[3].s64 = ctx.r[9].s64 + -3968;
	// 83270554: 4BA399CD  bl 0x82ca9f20
	ctx.lr = 0x83270558;
	sub_82CA9F20(ctx, base);
	// 83270558: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327055C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83270560: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83270564: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83270568(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83270568 size=56
    let mut pc: u32 = 0x83270568;
    'dispatch: loop {
        match pc {
            0x83270568 => {
    //   block [0x83270568..0x832705A0)
	// 83270568: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327056C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83270570: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83270574: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83270578: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8327057C: 386B42C8  addi r3, r11, 0x42c8
	ctx.r[3].s64 = ctx.r[11].s64 + 17096;
	// 83270580: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83270584: 4AF837D5  bl 0x821f3d58
	ctx.lr = 0x83270588;
	sub_821F3D58(ctx, base);
	// 83270588: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327058C: 906ACE1C  stw r3, -0x31e4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-12772 as u32), ctx.r[3].u32 ) };
	// 83270590: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83270594: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83270598: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327059C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832705A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832705A0 size=56
    let mut pc: u32 = 0x832705A0;
    'dispatch: loop {
        match pc {
            0x832705A0 => {
    //   block [0x832705A0..0x832705D8)
	// 832705A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832705A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832705A8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832705AC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 832705B0: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 832705B4: 386B42DC  addi r3, r11, 0x42dc
	ctx.r[3].s64 = ctx.r[11].s64 + 17116;
	// 832705B8: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 832705BC: 4AF8379D  bl 0x821f3d58
	ctx.lr = 0x832705C0;
	sub_821F3D58(ctx, base);
	// 832705C0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832705C4: 906ACE20  stw r3, -0x31e0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-12768 as u32), ctx.r[3].u32 ) };
	// 832705C8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832705CC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832705D0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832705D4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832705D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832705D8 size=56
    let mut pc: u32 = 0x832705D8;
    'dispatch: loop {
        match pc {
            0x832705D8 => {
    //   block [0x832705D8..0x83270610)
	// 832705D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832705DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832705E0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832705E4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 832705E8: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 832705EC: 386B42E8  addi r3, r11, 0x42e8
	ctx.r[3].s64 = ctx.r[11].s64 + 17128;
	// 832705F0: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 832705F4: 4AF83765  bl 0x821f3d58
	ctx.lr = 0x832705F8;
	sub_821F3D58(ctx, base);
	// 832705F8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832705FC: 906ACE24  stw r3, -0x31dc(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-12764 as u32), ctx.r[3].u32 ) };
	// 83270600: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83270604: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83270608: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327060C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83270610(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83270610 size=56
    let mut pc: u32 = 0x83270610;
    'dispatch: loop {
        match pc {
            0x83270610 => {
    //   block [0x83270610..0x83270648)
	// 83270610: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83270614: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83270618: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327061C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83270620: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83270624: 386B4300  addi r3, r11, 0x4300
	ctx.r[3].s64 = ctx.r[11].s64 + 17152;
	// 83270628: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8327062C: 4AF8372D  bl 0x821f3d58
	ctx.lr = 0x83270630;
	sub_821F3D58(ctx, base);
	// 83270630: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83270634: 906ACE28  stw r3, -0x31d8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-12760 as u32), ctx.r[3].u32 ) };
	// 83270638: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327063C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83270640: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83270644: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83270648(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83270648 size=56
    let mut pc: u32 = 0x83270648;
    'dispatch: loop {
        match pc {
            0x83270648 => {
    //   block [0x83270648..0x83270680)
	// 83270648: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327064C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83270650: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83270654: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83270658: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8327065C: 386B4310  addi r3, r11, 0x4310
	ctx.r[3].s64 = ctx.r[11].s64 + 17168;
	// 83270660: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83270664: 4AF836F5  bl 0x821f3d58
	ctx.lr = 0x83270668;
	sub_821F3D58(ctx, base);
	// 83270668: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327066C: 906ACE2C  stw r3, -0x31d4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-12756 as u32), ctx.r[3].u32 ) };
	// 83270670: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83270674: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83270678: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327067C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83270680(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83270680 size=56
    let mut pc: u32 = 0x83270680;
    'dispatch: loop {
        match pc {
            0x83270680 => {
    //   block [0x83270680..0x832706B8)
	// 83270680: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83270684: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83270688: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327068C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83270690: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83270694: 386B4328  addi r3, r11, 0x4328
	ctx.r[3].s64 = ctx.r[11].s64 + 17192;
	// 83270698: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8327069C: 4AF836BD  bl 0x821f3d58
	ctx.lr = 0x832706A0;
	sub_821F3D58(ctx, base);
	// 832706A0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832706A4: 906ACE30  stw r3, -0x31d0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-12752 as u32), ctx.r[3].u32 ) };
	// 832706A8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832706AC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832706B0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832706B4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832706B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832706B8 size=56
    let mut pc: u32 = 0x832706B8;
    'dispatch: loop {
        match pc {
            0x832706B8 => {
    //   block [0x832706B8..0x832706F0)
	// 832706B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832706BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832706C0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832706C4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 832706C8: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 832706CC: 386B4338  addi r3, r11, 0x4338
	ctx.r[3].s64 = ctx.r[11].s64 + 17208;
	// 832706D0: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 832706D4: 4AF83685  bl 0x821f3d58
	ctx.lr = 0x832706D8;
	sub_821F3D58(ctx, base);
	// 832706D8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832706DC: 906ACE34  stw r3, -0x31cc(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-12748 as u32), ctx.r[3].u32 ) };
	// 832706E0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832706E4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832706E8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832706EC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832706F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832706F0 size=56
    let mut pc: u32 = 0x832706F0;
    'dispatch: loop {
        match pc {
            0x832706F0 => {
    //   block [0x832706F0..0x83270728)
	// 832706F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832706F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832706F8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832706FC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83270700: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83270704: 386B4350  addi r3, r11, 0x4350
	ctx.r[3].s64 = ctx.r[11].s64 + 17232;
	// 83270708: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8327070C: 4AF8364D  bl 0x821f3d58
	ctx.lr = 0x83270710;
	sub_821F3D58(ctx, base);
	// 83270710: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83270714: 906ACE38  stw r3, -0x31c8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-12744 as u32), ctx.r[3].u32 ) };
	// 83270718: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327071C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83270720: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83270724: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83270728(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83270728 size=56
    let mut pc: u32 = 0x83270728;
    'dispatch: loop {
        match pc {
            0x83270728 => {
    //   block [0x83270728..0x83270760)
	// 83270728: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327072C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83270730: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83270734: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83270738: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8327073C: 386B4368  addi r3, r11, 0x4368
	ctx.r[3].s64 = ctx.r[11].s64 + 17256;
	// 83270740: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83270744: 4AF83615  bl 0x821f3d58
	ctx.lr = 0x83270748;
	sub_821F3D58(ctx, base);
	// 83270748: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327074C: 906ACE3C  stw r3, -0x31c4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-12740 as u32), ctx.r[3].u32 ) };
	// 83270750: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83270754: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83270758: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327075C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83270760(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83270760 size=56
    let mut pc: u32 = 0x83270760;
    'dispatch: loop {
        match pc {
            0x83270760 => {
    //   block [0x83270760..0x83270798)
	// 83270760: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83270764: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83270768: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327076C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83270770: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83270774: 386B4380  addi r3, r11, 0x4380
	ctx.r[3].s64 = ctx.r[11].s64 + 17280;
	// 83270778: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8327077C: 4AF835DD  bl 0x821f3d58
	ctx.lr = 0x83270780;
	sub_821F3D58(ctx, base);
	// 83270780: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83270784: 906ACE40  stw r3, -0x31c0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-12736 as u32), ctx.r[3].u32 ) };
	// 83270788: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327078C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83270790: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83270794: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83270798(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83270798 size=56
    let mut pc: u32 = 0x83270798;
    'dispatch: loop {
        match pc {
            0x83270798 => {
    //   block [0x83270798..0x832707D0)
	// 83270798: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327079C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832707A0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832707A4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 832707A8: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 832707AC: 386B4394  addi r3, r11, 0x4394
	ctx.r[3].s64 = ctx.r[11].s64 + 17300;
	// 832707B0: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 832707B4: 4AF835A5  bl 0x821f3d58
	ctx.lr = 0x832707B8;
	sub_821F3D58(ctx, base);
	// 832707B8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832707BC: 906ACE44  stw r3, -0x31bc(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-12732 as u32), ctx.r[3].u32 ) };
	// 832707C0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832707C4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832707C8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832707CC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832707D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832707D0 size=56
    let mut pc: u32 = 0x832707D0;
    'dispatch: loop {
        match pc {
            0x832707D0 => {
    //   block [0x832707D0..0x83270808)
	// 832707D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832707D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832707D8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832707DC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 832707E0: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 832707E4: 386B43B4  addi r3, r11, 0x43b4
	ctx.r[3].s64 = ctx.r[11].s64 + 17332;
	// 832707E8: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 832707EC: 4AF8356D  bl 0x821f3d58
	ctx.lr = 0x832707F0;
	sub_821F3D58(ctx, base);
	// 832707F0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832707F4: 906ACE48  stw r3, -0x31b8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-12728 as u32), ctx.r[3].u32 ) };
	// 832707F8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832707FC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83270800: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83270804: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83270808(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83270808 size=56
    let mut pc: u32 = 0x83270808;
    'dispatch: loop {
        match pc {
            0x83270808 => {
    //   block [0x83270808..0x83270840)
	// 83270808: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327080C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83270810: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83270814: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83270818: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8327081C: 386B43C8  addi r3, r11, 0x43c8
	ctx.r[3].s64 = ctx.r[11].s64 + 17352;
	// 83270820: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83270824: 4AF83535  bl 0x821f3d58
	ctx.lr = 0x83270828;
	sub_821F3D58(ctx, base);
	// 83270828: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327082C: 906ACE4C  stw r3, -0x31b4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-12724 as u32), ctx.r[3].u32 ) };
	// 83270830: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83270834: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83270838: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327083C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83270840(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83270840 size=56
    let mut pc: u32 = 0x83270840;
    'dispatch: loop {
        match pc {
            0x83270840 => {
    //   block [0x83270840..0x83270878)
	// 83270840: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83270844: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83270848: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327084C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83270850: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83270854: 386B43D4  addi r3, r11, 0x43d4
	ctx.r[3].s64 = ctx.r[11].s64 + 17364;
	// 83270858: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8327085C: 4AF834FD  bl 0x821f3d58
	ctx.lr = 0x83270860;
	sub_821F3D58(ctx, base);
	// 83270860: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83270864: 906ACE50  stw r3, -0x31b0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-12720 as u32), ctx.r[3].u32 ) };
	// 83270868: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327086C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83270870: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83270874: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83270878(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83270878 size=56
    let mut pc: u32 = 0x83270878;
    'dispatch: loop {
        match pc {
            0x83270878 => {
    //   block [0x83270878..0x832708B0)
	// 83270878: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327087C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83270880: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83270884: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83270888: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8327088C: 386B43E0  addi r3, r11, 0x43e0
	ctx.r[3].s64 = ctx.r[11].s64 + 17376;
	// 83270890: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83270894: 4AF834C5  bl 0x821f3d58
	ctx.lr = 0x83270898;
	sub_821F3D58(ctx, base);
	// 83270898: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327089C: 906ACE54  stw r3, -0x31ac(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-12716 as u32), ctx.r[3].u32 ) };
	// 832708A0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832708A4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832708A8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832708AC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832708B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832708B0 size=56
    let mut pc: u32 = 0x832708B0;
    'dispatch: loop {
        match pc {
            0x832708B0 => {
    //   block [0x832708B0..0x832708E8)
	// 832708B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832708B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832708B8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832708BC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 832708C0: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 832708C4: 386B43F0  addi r3, r11, 0x43f0
	ctx.r[3].s64 = ctx.r[11].s64 + 17392;
	// 832708C8: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 832708CC: 4AF8348D  bl 0x821f3d58
	ctx.lr = 0x832708D0;
	sub_821F3D58(ctx, base);
	// 832708D0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832708D4: 906ACE58  stw r3, -0x31a8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-12712 as u32), ctx.r[3].u32 ) };
	// 832708D8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832708DC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832708E0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832708E4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832708E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832708E8 size=56
    let mut pc: u32 = 0x832708E8;
    'dispatch: loop {
        match pc {
            0x832708E8 => {
    //   block [0x832708E8..0x83270920)
	// 832708E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832708EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832708F0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832708F4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 832708F8: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 832708FC: 386B4400  addi r3, r11, 0x4400
	ctx.r[3].s64 = ctx.r[11].s64 + 17408;
	// 83270900: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83270904: 4AF83455  bl 0x821f3d58
	ctx.lr = 0x83270908;
	sub_821F3D58(ctx, base);
	// 83270908: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327090C: 906ACE5C  stw r3, -0x31a4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-12708 as u32), ctx.r[3].u32 ) };
	// 83270910: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83270914: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83270918: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327091C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83270920(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83270920 size=56
    let mut pc: u32 = 0x83270920;
    'dispatch: loop {
        match pc {
            0x83270920 => {
    //   block [0x83270920..0x83270958)
	// 83270920: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83270924: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83270928: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327092C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83270930: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83270934: 386B4418  addi r3, r11, 0x4418
	ctx.r[3].s64 = ctx.r[11].s64 + 17432;
	// 83270938: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8327093C: 4AF8341D  bl 0x821f3d58
	ctx.lr = 0x83270940;
	sub_821F3D58(ctx, base);
	// 83270940: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83270944: 906ACE60  stw r3, -0x31a0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-12704 as u32), ctx.r[3].u32 ) };
	// 83270948: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327094C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83270950: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83270954: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83270958(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83270958 size=56
    let mut pc: u32 = 0x83270958;
    'dispatch: loop {
        match pc {
            0x83270958 => {
    //   block [0x83270958..0x83270990)
	// 83270958: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327095C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83270960: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83270964: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83270968: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8327096C: 386B4424  addi r3, r11, 0x4424
	ctx.r[3].s64 = ctx.r[11].s64 + 17444;
	// 83270970: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83270974: 4AF833E5  bl 0x821f3d58
	ctx.lr = 0x83270978;
	sub_821F3D58(ctx, base);
	// 83270978: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327097C: 906ACE64  stw r3, -0x319c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-12700 as u32), ctx.r[3].u32 ) };
	// 83270980: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83270984: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83270988: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327098C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83270990(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83270990 size=56
    let mut pc: u32 = 0x83270990;
    'dispatch: loop {
        match pc {
            0x83270990 => {
    //   block [0x83270990..0x832709C8)
	// 83270990: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83270994: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83270998: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327099C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 832709A0: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 832709A4: 386B4430  addi r3, r11, 0x4430
	ctx.r[3].s64 = ctx.r[11].s64 + 17456;
	// 832709A8: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 832709AC: 4AF833AD  bl 0x821f3d58
	ctx.lr = 0x832709B0;
	sub_821F3D58(ctx, base);
	// 832709B0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832709B4: 906ACE68  stw r3, -0x3198(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-12696 as u32), ctx.r[3].u32 ) };
	// 832709B8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832709BC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832709C0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832709C4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832709C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832709C8 size=56
    let mut pc: u32 = 0x832709C8;
    'dispatch: loop {
        match pc {
            0x832709C8 => {
    //   block [0x832709C8..0x83270A00)
	// 832709C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832709CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832709D0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832709D4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 832709D8: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 832709DC: 386B4440  addi r3, r11, 0x4440
	ctx.r[3].s64 = ctx.r[11].s64 + 17472;
	// 832709E0: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 832709E4: 4AF83375  bl 0x821f3d58
	ctx.lr = 0x832709E8;
	sub_821F3D58(ctx, base);
	// 832709E8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832709EC: 906ACE6C  stw r3, -0x3194(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-12692 as u32), ctx.r[3].u32 ) };
	// 832709F0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832709F4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832709F8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832709FC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83270A00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83270A00 size=60
    let mut pc: u32 = 0x83270A00;
    'dispatch: loop {
        match pc {
            0x83270A00 => {
    //   block [0x83270A00..0x83270A3C)
	// 83270A00: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83270A04: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83270A08: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83270A0C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83270A10: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83270A14: 388B2DEC  addi r4, r11, 0x2dec
	ctx.r[4].s64 = ctx.r[11].s64 + 11756;
	// 83270A18: 386ACE70  addi r3, r10, -0x3190
	ctx.r[3].s64 = ctx.r[10].s64 + -12688;
	// 83270A1C: 4B0659ED  bl 0x822d6408
	ctx.lr = 0x83270A20;
	sub_822D6408(ctx, base);
	// 83270A20: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83270A24: 3869F090  addi r3, r9, -0xf70
	ctx.r[3].s64 = ctx.r[9].s64 + -3952;
	// 83270A28: 4BA394F9  bl 0x82ca9f20
	ctx.lr = 0x83270A2C;
	sub_82CA9F20(ctx, base);
	// 83270A2C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83270A30: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83270A34: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83270A38: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83270A40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83270A40 size=64
    let mut pc: u32 = 0x83270A40;
    'dispatch: loop {
        match pc {
            0x83270A40 => {
    //   block [0x83270A40..0x83270A80)
	// 83270A40: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83270A44: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83270A48: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83270A4C: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 83270A50: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83270A54: 388BA080  addi r4, r11, -0x5f80
	ctx.r[4].s64 = ctx.r[11].s64 + -24448;
	// 83270A58: 386ACE74  addi r3, r10, -0x318c
	ctx.r[3].s64 = ctx.r[10].s64 + -12684;
	// 83270A5C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83270A60: 4AFBC471  bl 0x8222ced0
	ctx.lr = 0x83270A64;
	sub_8222CED0(ctx, base);
	// 83270A64: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83270A68: 3869F0A0  addi r3, r9, -0xf60
	ctx.r[3].s64 = ctx.r[9].s64 + -3936;
	// 83270A6C: 4BA394B5  bl 0x82ca9f20
	ctx.lr = 0x83270A70;
	sub_82CA9F20(ctx, base);
	// 83270A70: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83270A74: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83270A78: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83270A7C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83270A80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83270A80 size=64
    let mut pc: u32 = 0x83270A80;
    'dispatch: loop {
        match pc {
            0x83270A80 => {
    //   block [0x83270A80..0x83270AC0)
	// 83270A80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83270A84: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83270A88: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83270A8C: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 83270A90: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83270A94: 388BA098  addi r4, r11, -0x5f68
	ctx.r[4].s64 = ctx.r[11].s64 + -24424;
	// 83270A98: 386ACE78  addi r3, r10, -0x3188
	ctx.r[3].s64 = ctx.r[10].s64 + -12680;
	// 83270A9C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83270AA0: 4AFBC431  bl 0x8222ced0
	ctx.lr = 0x83270AA4;
	sub_8222CED0(ctx, base);
	// 83270AA4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83270AA8: 3869F0B0  addi r3, r9, -0xf50
	ctx.r[3].s64 = ctx.r[9].s64 + -3920;
	// 83270AAC: 4BA39475  bl 0x82ca9f20
	ctx.lr = 0x83270AB0;
	sub_82CA9F20(ctx, base);
	// 83270AB0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83270AB4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83270AB8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83270ABC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83270AC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83270AC0 size=64
    let mut pc: u32 = 0x83270AC0;
    'dispatch: loop {
        match pc {
            0x83270AC0 => {
    //   block [0x83270AC0..0x83270B00)
	// 83270AC0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83270AC4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83270AC8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83270ACC: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 83270AD0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83270AD4: 388BA0C0  addi r4, r11, -0x5f40
	ctx.r[4].s64 = ctx.r[11].s64 + -24384;
	// 83270AD8: 386ACE7C  addi r3, r10, -0x3184
	ctx.r[3].s64 = ctx.r[10].s64 + -12676;
	// 83270ADC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83270AE0: 4AFBC3F1  bl 0x8222ced0
	ctx.lr = 0x83270AE4;
	sub_8222CED0(ctx, base);
	// 83270AE4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83270AE8: 3869F0C0  addi r3, r9, -0xf40
	ctx.r[3].s64 = ctx.r[9].s64 + -3904;
	// 83270AEC: 4BA39435  bl 0x82ca9f20
	ctx.lr = 0x83270AF0;
	sub_82CA9F20(ctx, base);
	// 83270AF0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83270AF4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83270AF8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83270AFC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83270B00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83270B00 size=64
    let mut pc: u32 = 0x83270B00;
    'dispatch: loop {
        match pc {
            0x83270B00 => {
    //   block [0x83270B00..0x83270B40)
	// 83270B00: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83270B04: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83270B08: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83270B0C: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 83270B10: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83270B14: 388BA0EC  addi r4, r11, -0x5f14
	ctx.r[4].s64 = ctx.r[11].s64 + -24340;
	// 83270B18: 386ACE80  addi r3, r10, -0x3180
	ctx.r[3].s64 = ctx.r[10].s64 + -12672;
	// 83270B1C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83270B20: 4AFBC3B1  bl 0x8222ced0
	ctx.lr = 0x83270B24;
	sub_8222CED0(ctx, base);
	// 83270B24: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83270B28: 3869F0D0  addi r3, r9, -0xf30
	ctx.r[3].s64 = ctx.r[9].s64 + -3888;
	// 83270B2C: 4BA393F5  bl 0x82ca9f20
	ctx.lr = 0x83270B30;
	sub_82CA9F20(ctx, base);
	// 83270B30: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83270B34: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83270B38: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83270B3C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83270B40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83270B40 size=64
    let mut pc: u32 = 0x83270B40;
    'dispatch: loop {
        match pc {
            0x83270B40 => {
    //   block [0x83270B40..0x83270B80)
	// 83270B40: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83270B44: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83270B48: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83270B4C: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 83270B50: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83270B54: 388BA124  addi r4, r11, -0x5edc
	ctx.r[4].s64 = ctx.r[11].s64 + -24284;
	// 83270B58: 386ACE84  addi r3, r10, -0x317c
	ctx.r[3].s64 = ctx.r[10].s64 + -12668;
	// 83270B5C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83270B60: 4AFBC371  bl 0x8222ced0
	ctx.lr = 0x83270B64;
	sub_8222CED0(ctx, base);
	// 83270B64: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83270B68: 3869F0E0  addi r3, r9, -0xf20
	ctx.r[3].s64 = ctx.r[9].s64 + -3872;
	// 83270B6C: 4BA393B5  bl 0x82ca9f20
	ctx.lr = 0x83270B70;
	sub_82CA9F20(ctx, base);
	// 83270B70: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83270B74: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83270B78: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83270B7C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83270B80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83270B80 size=64
    let mut pc: u32 = 0x83270B80;
    'dispatch: loop {
        match pc {
            0x83270B80 => {
    //   block [0x83270B80..0x83270BC0)
	// 83270B80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83270B84: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83270B88: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83270B8C: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 83270B90: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83270B94: 388BA158  addi r4, r11, -0x5ea8
	ctx.r[4].s64 = ctx.r[11].s64 + -24232;
	// 83270B98: 386ACE88  addi r3, r10, -0x3178
	ctx.r[3].s64 = ctx.r[10].s64 + -12664;
	// 83270B9C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83270BA0: 4AFBC331  bl 0x8222ced0
	ctx.lr = 0x83270BA4;
	sub_8222CED0(ctx, base);
	// 83270BA4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83270BA8: 3869F0F0  addi r3, r9, -0xf10
	ctx.r[3].s64 = ctx.r[9].s64 + -3856;
	// 83270BAC: 4BA39375  bl 0x82ca9f20
	ctx.lr = 0x83270BB0;
	sub_82CA9F20(ctx, base);
	// 83270BB0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83270BB4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83270BB8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83270BBC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83270BC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83270BC0 size=64
    let mut pc: u32 = 0x83270BC0;
    'dispatch: loop {
        match pc {
            0x83270BC0 => {
    //   block [0x83270BC0..0x83270C00)
	// 83270BC0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83270BC4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83270BC8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83270BCC: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 83270BD0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83270BD4: 388BA198  addi r4, r11, -0x5e68
	ctx.r[4].s64 = ctx.r[11].s64 + -24168;
	// 83270BD8: 386ACE8C  addi r3, r10, -0x3174
	ctx.r[3].s64 = ctx.r[10].s64 + -12660;
	// 83270BDC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83270BE0: 4AFBC2F1  bl 0x8222ced0
	ctx.lr = 0x83270BE4;
	sub_8222CED0(ctx, base);
	// 83270BE4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83270BE8: 3869F100  addi r3, r9, -0xf00
	ctx.r[3].s64 = ctx.r[9].s64 + -3840;
	// 83270BEC: 4BA39335  bl 0x82ca9f20
	ctx.lr = 0x83270BF0;
	sub_82CA9F20(ctx, base);
	// 83270BF0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83270BF4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83270BF8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83270BFC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83270C00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83270C00 size=64
    let mut pc: u32 = 0x83270C00;
    'dispatch: loop {
        match pc {
            0x83270C00 => {
    //   block [0x83270C00..0x83270C40)
	// 83270C00: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83270C04: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83270C08: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83270C0C: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 83270C10: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83270C14: 388BA1D8  addi r4, r11, -0x5e28
	ctx.r[4].s64 = ctx.r[11].s64 + -24104;
	// 83270C18: 386ACE90  addi r3, r10, -0x3170
	ctx.r[3].s64 = ctx.r[10].s64 + -12656;
	// 83270C1C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83270C20: 4AFBC2B1  bl 0x8222ced0
	ctx.lr = 0x83270C24;
	sub_8222CED0(ctx, base);
	// 83270C24: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83270C28: 3869F110  addi r3, r9, -0xef0
	ctx.r[3].s64 = ctx.r[9].s64 + -3824;
	// 83270C2C: 4BA392F5  bl 0x82ca9f20
	ctx.lr = 0x83270C30;
	sub_82CA9F20(ctx, base);
	// 83270C30: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83270C34: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83270C38: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83270C3C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83270C40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83270C40 size=64
    let mut pc: u32 = 0x83270C40;
    'dispatch: loop {
        match pc {
            0x83270C40 => {
    //   block [0x83270C40..0x83270C80)
	// 83270C40: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83270C44: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83270C48: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83270C4C: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 83270C50: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83270C54: 388BA210  addi r4, r11, -0x5df0
	ctx.r[4].s64 = ctx.r[11].s64 + -24048;
	// 83270C58: 386ACE94  addi r3, r10, -0x316c
	ctx.r[3].s64 = ctx.r[10].s64 + -12652;
	// 83270C5C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83270C60: 4AFBC271  bl 0x8222ced0
	ctx.lr = 0x83270C64;
	sub_8222CED0(ctx, base);
	// 83270C64: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83270C68: 3869F120  addi r3, r9, -0xee0
	ctx.r[3].s64 = ctx.r[9].s64 + -3808;
	// 83270C6C: 4BA392B5  bl 0x82ca9f20
	ctx.lr = 0x83270C70;
	sub_82CA9F20(ctx, base);
	// 83270C70: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83270C74: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83270C78: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83270C7C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83270C80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83270C80 size=64
    let mut pc: u32 = 0x83270C80;
    'dispatch: loop {
        match pc {
            0x83270C80 => {
    //   block [0x83270C80..0x83270CC0)
	// 83270C80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83270C84: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83270C88: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83270C8C: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 83270C90: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83270C94: 388BA258  addi r4, r11, -0x5da8
	ctx.r[4].s64 = ctx.r[11].s64 + -23976;
	// 83270C98: 386ACE98  addi r3, r10, -0x3168
	ctx.r[3].s64 = ctx.r[10].s64 + -12648;
	// 83270C9C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83270CA0: 4AFBC231  bl 0x8222ced0
	ctx.lr = 0x83270CA4;
	sub_8222CED0(ctx, base);
	// 83270CA4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83270CA8: 3869F130  addi r3, r9, -0xed0
	ctx.r[3].s64 = ctx.r[9].s64 + -3792;
	// 83270CAC: 4BA39275  bl 0x82ca9f20
	ctx.lr = 0x83270CB0;
	sub_82CA9F20(ctx, base);
	// 83270CB0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83270CB4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83270CB8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83270CBC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83270CC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83270CC0 size=64
    let mut pc: u32 = 0x83270CC0;
    'dispatch: loop {
        match pc {
            0x83270CC0 => {
    //   block [0x83270CC0..0x83270D00)
	// 83270CC0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83270CC4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83270CC8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83270CCC: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 83270CD0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83270CD4: 388BA2A0  addi r4, r11, -0x5d60
	ctx.r[4].s64 = ctx.r[11].s64 + -23904;
	// 83270CD8: 386ACE9C  addi r3, r10, -0x3164
	ctx.r[3].s64 = ctx.r[10].s64 + -12644;
	// 83270CDC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83270CE0: 4AFBC1F1  bl 0x8222ced0
	ctx.lr = 0x83270CE4;
	sub_8222CED0(ctx, base);
	// 83270CE4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83270CE8: 3869F140  addi r3, r9, -0xec0
	ctx.r[3].s64 = ctx.r[9].s64 + -3776;
	// 83270CEC: 4BA39235  bl 0x82ca9f20
	ctx.lr = 0x83270CF0;
	sub_82CA9F20(ctx, base);
	// 83270CF0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83270CF4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83270CF8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83270CFC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83270D00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83270D00 size=64
    let mut pc: u32 = 0x83270D00;
    'dispatch: loop {
        match pc {
            0x83270D00 => {
    //   block [0x83270D00..0x83270D40)
	// 83270D00: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83270D04: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83270D08: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83270D0C: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 83270D10: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83270D14: 388BA2E4  addi r4, r11, -0x5d1c
	ctx.r[4].s64 = ctx.r[11].s64 + -23836;
	// 83270D18: 386ACEA0  addi r3, r10, -0x3160
	ctx.r[3].s64 = ctx.r[10].s64 + -12640;
	// 83270D1C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83270D20: 4AFBC1B1  bl 0x8222ced0
	ctx.lr = 0x83270D24;
	sub_8222CED0(ctx, base);
	// 83270D24: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83270D28: 3869F150  addi r3, r9, -0xeb0
	ctx.r[3].s64 = ctx.r[9].s64 + -3760;
	// 83270D2C: 4BA391F5  bl 0x82ca9f20
	ctx.lr = 0x83270D30;
	sub_82CA9F20(ctx, base);
	// 83270D30: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83270D34: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83270D38: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83270D3C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83270D40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83270D40 size=64
    let mut pc: u32 = 0x83270D40;
    'dispatch: loop {
        match pc {
            0x83270D40 => {
    //   block [0x83270D40..0x83270D80)
	// 83270D40: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83270D44: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83270D48: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83270D4C: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 83270D50: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83270D54: 388BA318  addi r4, r11, -0x5ce8
	ctx.r[4].s64 = ctx.r[11].s64 + -23784;
	// 83270D58: 386ACEA4  addi r3, r10, -0x315c
	ctx.r[3].s64 = ctx.r[10].s64 + -12636;
	// 83270D5C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83270D60: 4AFBC171  bl 0x8222ced0
	ctx.lr = 0x83270D64;
	sub_8222CED0(ctx, base);
	// 83270D64: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83270D68: 3869F160  addi r3, r9, -0xea0
	ctx.r[3].s64 = ctx.r[9].s64 + -3744;
	// 83270D6C: 4BA391B5  bl 0x82ca9f20
	ctx.lr = 0x83270D70;
	sub_82CA9F20(ctx, base);
	// 83270D70: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83270D74: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83270D78: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83270D7C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83270D80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83270D80 size=64
    let mut pc: u32 = 0x83270D80;
    'dispatch: loop {
        match pc {
            0x83270D80 => {
    //   block [0x83270D80..0x83270DC0)
	// 83270D80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83270D84: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83270D88: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83270D8C: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 83270D90: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83270D94: 388BA350  addi r4, r11, -0x5cb0
	ctx.r[4].s64 = ctx.r[11].s64 + -23728;
	// 83270D98: 386ACEA8  addi r3, r10, -0x3158
	ctx.r[3].s64 = ctx.r[10].s64 + -12632;
	// 83270D9C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83270DA0: 4AFBC131  bl 0x8222ced0
	ctx.lr = 0x83270DA4;
	sub_8222CED0(ctx, base);
	// 83270DA4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83270DA8: 3869F170  addi r3, r9, -0xe90
	ctx.r[3].s64 = ctx.r[9].s64 + -3728;
	// 83270DAC: 4BA39175  bl 0x82ca9f20
	ctx.lr = 0x83270DB0;
	sub_82CA9F20(ctx, base);
	// 83270DB0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83270DB4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83270DB8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83270DBC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83270DC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83270DC0 size=64
    let mut pc: u32 = 0x83270DC0;
    'dispatch: loop {
        match pc {
            0x83270DC0 => {
    //   block [0x83270DC0..0x83270E00)
	// 83270DC0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83270DC4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83270DC8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83270DCC: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 83270DD0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83270DD4: 388BA390  addi r4, r11, -0x5c70
	ctx.r[4].s64 = ctx.r[11].s64 + -23664;
	// 83270DD8: 386ACEAC  addi r3, r10, -0x3154
	ctx.r[3].s64 = ctx.r[10].s64 + -12628;
	// 83270DDC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83270DE0: 4AFBC0F1  bl 0x8222ced0
	ctx.lr = 0x83270DE4;
	sub_8222CED0(ctx, base);
	// 83270DE4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83270DE8: 3869F180  addi r3, r9, -0xe80
	ctx.r[3].s64 = ctx.r[9].s64 + -3712;
	// 83270DEC: 4BA39135  bl 0x82ca9f20
	ctx.lr = 0x83270DF0;
	sub_82CA9F20(ctx, base);
	// 83270DF0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83270DF4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83270DF8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83270DFC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83270E00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83270E00 size=64
    let mut pc: u32 = 0x83270E00;
    'dispatch: loop {
        match pc {
            0x83270E00 => {
    //   block [0x83270E00..0x83270E40)
	// 83270E00: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83270E04: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83270E08: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83270E0C: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 83270E10: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83270E14: 388BA3D4  addi r4, r11, -0x5c2c
	ctx.r[4].s64 = ctx.r[11].s64 + -23596;
	// 83270E18: 386ACEB0  addi r3, r10, -0x3150
	ctx.r[3].s64 = ctx.r[10].s64 + -12624;
	// 83270E1C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83270E20: 4AFBC0B1  bl 0x8222ced0
	ctx.lr = 0x83270E24;
	sub_8222CED0(ctx, base);
	// 83270E24: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83270E28: 3869F190  addi r3, r9, -0xe70
	ctx.r[3].s64 = ctx.r[9].s64 + -3696;
	// 83270E2C: 4BA390F5  bl 0x82ca9f20
	ctx.lr = 0x83270E30;
	sub_82CA9F20(ctx, base);
	// 83270E30: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83270E34: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83270E38: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83270E3C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83270E40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83270E40 size=64
    let mut pc: u32 = 0x83270E40;
    'dispatch: loop {
        match pc {
            0x83270E40 => {
    //   block [0x83270E40..0x83270E80)
	// 83270E40: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83270E44: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83270E48: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83270E4C: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 83270E50: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83270E54: 388BA408  addi r4, r11, -0x5bf8
	ctx.r[4].s64 = ctx.r[11].s64 + -23544;
	// 83270E58: 386ACEB4  addi r3, r10, -0x314c
	ctx.r[3].s64 = ctx.r[10].s64 + -12620;
	// 83270E5C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83270E60: 4AFBC071  bl 0x8222ced0
	ctx.lr = 0x83270E64;
	sub_8222CED0(ctx, base);
	// 83270E64: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83270E68: 3869F1A0  addi r3, r9, -0xe60
	ctx.r[3].s64 = ctx.r[9].s64 + -3680;
	// 83270E6C: 4BA390B5  bl 0x82ca9f20
	ctx.lr = 0x83270E70;
	sub_82CA9F20(ctx, base);
	// 83270E70: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83270E74: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83270E78: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83270E7C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83270E80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83270E80 size=64
    let mut pc: u32 = 0x83270E80;
    'dispatch: loop {
        match pc {
            0x83270E80 => {
    //   block [0x83270E80..0x83270EC0)
	// 83270E80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83270E84: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83270E88: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83270E8C: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 83270E90: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83270E94: 388BA458  addi r4, r11, -0x5ba8
	ctx.r[4].s64 = ctx.r[11].s64 + -23464;
	// 83270E98: 386ACEB8  addi r3, r10, -0x3148
	ctx.r[3].s64 = ctx.r[10].s64 + -12616;
	// 83270E9C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83270EA0: 4AFBC031  bl 0x8222ced0
	ctx.lr = 0x83270EA4;
	sub_8222CED0(ctx, base);
	// 83270EA4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83270EA8: 3869F1B0  addi r3, r9, -0xe50
	ctx.r[3].s64 = ctx.r[9].s64 + -3664;
	// 83270EAC: 4BA39075  bl 0x82ca9f20
	ctx.lr = 0x83270EB0;
	sub_82CA9F20(ctx, base);
	// 83270EB0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83270EB4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83270EB8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83270EBC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83270EC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83270EC0 size=64
    let mut pc: u32 = 0x83270EC0;
    'dispatch: loop {
        match pc {
            0x83270EC0 => {
    //   block [0x83270EC0..0x83270F00)
	// 83270EC0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83270EC4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83270EC8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83270ECC: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 83270ED0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83270ED4: 388BA490  addi r4, r11, -0x5b70
	ctx.r[4].s64 = ctx.r[11].s64 + -23408;
	// 83270ED8: 386ACEBC  addi r3, r10, -0x3144
	ctx.r[3].s64 = ctx.r[10].s64 + -12612;
	// 83270EDC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83270EE0: 4AFBBFF1  bl 0x8222ced0
	ctx.lr = 0x83270EE4;
	sub_8222CED0(ctx, base);
	// 83270EE4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83270EE8: 3869F1C0  addi r3, r9, -0xe40
	ctx.r[3].s64 = ctx.r[9].s64 + -3648;
	// 83270EEC: 4BA39035  bl 0x82ca9f20
	ctx.lr = 0x83270EF0;
	sub_82CA9F20(ctx, base);
	// 83270EF0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83270EF4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83270EF8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83270EFC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83270F00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83270F00 size=64
    let mut pc: u32 = 0x83270F00;
    'dispatch: loop {
        match pc {
            0x83270F00 => {
    //   block [0x83270F00..0x83270F40)
	// 83270F00: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83270F04: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83270F08: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83270F0C: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 83270F10: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83270F14: 388BA4E0  addi r4, r11, -0x5b20
	ctx.r[4].s64 = ctx.r[11].s64 + -23328;
	// 83270F18: 386ACEC0  addi r3, r10, -0x3140
	ctx.r[3].s64 = ctx.r[10].s64 + -12608;
	// 83270F1C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83270F20: 4AFBBFB1  bl 0x8222ced0
	ctx.lr = 0x83270F24;
	sub_8222CED0(ctx, base);
	// 83270F24: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83270F28: 3869F1D0  addi r3, r9, -0xe30
	ctx.r[3].s64 = ctx.r[9].s64 + -3632;
	// 83270F2C: 4BA38FF5  bl 0x82ca9f20
	ctx.lr = 0x83270F30;
	sub_82CA9F20(ctx, base);
	// 83270F30: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83270F34: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83270F38: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83270F3C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83270F40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83270F40 size=64
    let mut pc: u32 = 0x83270F40;
    'dispatch: loop {
        match pc {
            0x83270F40 => {
    //   block [0x83270F40..0x83270F80)
	// 83270F40: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83270F44: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83270F48: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83270F4C: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 83270F50: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83270F54: 388BA50C  addi r4, r11, -0x5af4
	ctx.r[4].s64 = ctx.r[11].s64 + -23284;
	// 83270F58: 386ACEC4  addi r3, r10, -0x313c
	ctx.r[3].s64 = ctx.r[10].s64 + -12604;
	// 83270F5C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83270F60: 4AFBBF71  bl 0x8222ced0
	ctx.lr = 0x83270F64;
	sub_8222CED0(ctx, base);
	// 83270F64: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83270F68: 3869F1E0  addi r3, r9, -0xe20
	ctx.r[3].s64 = ctx.r[9].s64 + -3616;
	// 83270F6C: 4BA38FB5  bl 0x82ca9f20
	ctx.lr = 0x83270F70;
	sub_82CA9F20(ctx, base);
	// 83270F70: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83270F74: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83270F78: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83270F7C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83270F80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83270F80 size=56
    let mut pc: u32 = 0x83270F80;
    'dispatch: loop {
        match pc {
            0x83270F80 => {
    //   block [0x83270F80..0x83270FB8)
	// 83270F80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83270F84: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83270F88: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83270F8C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83270F90: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83270F94: 386B42C8  addi r3, r11, 0x42c8
	ctx.r[3].s64 = ctx.r[11].s64 + 17096;
	// 83270F98: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83270F9C: 4AF82DBD  bl 0x821f3d58
	ctx.lr = 0x83270FA0;
	sub_821F3D58(ctx, base);
	// 83270FA0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83270FA4: 906ACEC8  stw r3, -0x3138(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-12600 as u32), ctx.r[3].u32 ) };
	// 83270FA8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83270FAC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83270FB0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83270FB4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83270FB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83270FB8 size=56
    let mut pc: u32 = 0x83270FB8;
    'dispatch: loop {
        match pc {
            0x83270FB8 => {
    //   block [0x83270FB8..0x83270FF0)
	// 83270FB8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83270FBC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83270FC0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83270FC4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83270FC8: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83270FCC: 386B42DC  addi r3, r11, 0x42dc
	ctx.r[3].s64 = ctx.r[11].s64 + 17116;
	// 83270FD0: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83270FD4: 4AF82D85  bl 0x821f3d58
	ctx.lr = 0x83270FD8;
	sub_821F3D58(ctx, base);
	// 83270FD8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83270FDC: 906ACECC  stw r3, -0x3134(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-12596 as u32), ctx.r[3].u32 ) };
	// 83270FE0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83270FE4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83270FE8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83270FEC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83270FF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83270FF0 size=56
    let mut pc: u32 = 0x83270FF0;
    'dispatch: loop {
        match pc {
            0x83270FF0 => {
    //   block [0x83270FF0..0x83271028)
	// 83270FF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83270FF4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83270FF8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83270FFC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83271000: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83271004: 386B42E8  addi r3, r11, 0x42e8
	ctx.r[3].s64 = ctx.r[11].s64 + 17128;
	// 83271008: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8327100C: 4AF82D4D  bl 0x821f3d58
	ctx.lr = 0x83271010;
	sub_821F3D58(ctx, base);
	// 83271010: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83271014: 906ACED0  stw r3, -0x3130(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-12592 as u32), ctx.r[3].u32 ) };
	// 83271018: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327101C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83271020: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83271024: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83271028(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83271028 size=56
    let mut pc: u32 = 0x83271028;
    'dispatch: loop {
        match pc {
            0x83271028 => {
    //   block [0x83271028..0x83271060)
	// 83271028: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327102C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83271030: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83271034: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83271038: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8327103C: 386B4300  addi r3, r11, 0x4300
	ctx.r[3].s64 = ctx.r[11].s64 + 17152;
	// 83271040: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83271044: 4AF82D15  bl 0x821f3d58
	ctx.lr = 0x83271048;
	sub_821F3D58(ctx, base);
	// 83271048: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327104C: 906ACED4  stw r3, -0x312c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-12588 as u32), ctx.r[3].u32 ) };
	// 83271050: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83271054: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83271058: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327105C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83271060(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83271060 size=56
    let mut pc: u32 = 0x83271060;
    'dispatch: loop {
        match pc {
            0x83271060 => {
    //   block [0x83271060..0x83271098)
	// 83271060: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83271064: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83271068: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327106C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83271070: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83271074: 386B4310  addi r3, r11, 0x4310
	ctx.r[3].s64 = ctx.r[11].s64 + 17168;
	// 83271078: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8327107C: 4AF82CDD  bl 0x821f3d58
	ctx.lr = 0x83271080;
	sub_821F3D58(ctx, base);
	// 83271080: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83271084: 906ACED8  stw r3, -0x3128(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-12584 as u32), ctx.r[3].u32 ) };
	// 83271088: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327108C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83271090: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83271094: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83271098(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83271098 size=56
    let mut pc: u32 = 0x83271098;
    'dispatch: loop {
        match pc {
            0x83271098 => {
    //   block [0x83271098..0x832710D0)
	// 83271098: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327109C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832710A0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832710A4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 832710A8: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 832710AC: 386B4328  addi r3, r11, 0x4328
	ctx.r[3].s64 = ctx.r[11].s64 + 17192;
	// 832710B0: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 832710B4: 4AF82CA5  bl 0x821f3d58
	ctx.lr = 0x832710B8;
	sub_821F3D58(ctx, base);
	// 832710B8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832710BC: 906ACEDC  stw r3, -0x3124(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-12580 as u32), ctx.r[3].u32 ) };
	// 832710C0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832710C4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832710C8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832710CC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832710D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832710D0 size=56
    let mut pc: u32 = 0x832710D0;
    'dispatch: loop {
        match pc {
            0x832710D0 => {
    //   block [0x832710D0..0x83271108)
	// 832710D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832710D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832710D8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832710DC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 832710E0: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 832710E4: 386B4338  addi r3, r11, 0x4338
	ctx.r[3].s64 = ctx.r[11].s64 + 17208;
	// 832710E8: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 832710EC: 4AF82C6D  bl 0x821f3d58
	ctx.lr = 0x832710F0;
	sub_821F3D58(ctx, base);
	// 832710F0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832710F4: 906ACEE0  stw r3, -0x3120(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-12576 as u32), ctx.r[3].u32 ) };
	// 832710F8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832710FC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83271100: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83271104: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83271108(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83271108 size=56
    let mut pc: u32 = 0x83271108;
    'dispatch: loop {
        match pc {
            0x83271108 => {
    //   block [0x83271108..0x83271140)
	// 83271108: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327110C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83271110: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83271114: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83271118: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8327111C: 386B4350  addi r3, r11, 0x4350
	ctx.r[3].s64 = ctx.r[11].s64 + 17232;
	// 83271120: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83271124: 4AF82C35  bl 0x821f3d58
	ctx.lr = 0x83271128;
	sub_821F3D58(ctx, base);
	// 83271128: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327112C: 906ACEE4  stw r3, -0x311c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-12572 as u32), ctx.r[3].u32 ) };
	// 83271130: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83271134: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83271138: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327113C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83271140(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83271140 size=56
    let mut pc: u32 = 0x83271140;
    'dispatch: loop {
        match pc {
            0x83271140 => {
    //   block [0x83271140..0x83271178)
	// 83271140: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83271144: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83271148: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327114C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83271150: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83271154: 386B4368  addi r3, r11, 0x4368
	ctx.r[3].s64 = ctx.r[11].s64 + 17256;
	// 83271158: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8327115C: 4AF82BFD  bl 0x821f3d58
	ctx.lr = 0x83271160;
	sub_821F3D58(ctx, base);
	// 83271160: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83271164: 906ACEE8  stw r3, -0x3118(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-12568 as u32), ctx.r[3].u32 ) };
	// 83271168: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327116C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83271170: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83271174: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83271178(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83271178 size=56
    let mut pc: u32 = 0x83271178;
    'dispatch: loop {
        match pc {
            0x83271178 => {
    //   block [0x83271178..0x832711B0)
	// 83271178: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327117C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83271180: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83271184: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83271188: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8327118C: 386B4380  addi r3, r11, 0x4380
	ctx.r[3].s64 = ctx.r[11].s64 + 17280;
	// 83271190: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83271194: 4AF82BC5  bl 0x821f3d58
	ctx.lr = 0x83271198;
	sub_821F3D58(ctx, base);
	// 83271198: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327119C: 906ACEEC  stw r3, -0x3114(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-12564 as u32), ctx.r[3].u32 ) };
	// 832711A0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832711A4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832711A8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832711AC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832711B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832711B0 size=56
    let mut pc: u32 = 0x832711B0;
    'dispatch: loop {
        match pc {
            0x832711B0 => {
    //   block [0x832711B0..0x832711E8)
	// 832711B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832711B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832711B8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832711BC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 832711C0: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 832711C4: 386B4394  addi r3, r11, 0x4394
	ctx.r[3].s64 = ctx.r[11].s64 + 17300;
	// 832711C8: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 832711CC: 4AF82B8D  bl 0x821f3d58
	ctx.lr = 0x832711D0;
	sub_821F3D58(ctx, base);
	// 832711D0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832711D4: 906ACEF0  stw r3, -0x3110(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-12560 as u32), ctx.r[3].u32 ) };
	// 832711D8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832711DC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832711E0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832711E4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832711E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832711E8 size=56
    let mut pc: u32 = 0x832711E8;
    'dispatch: loop {
        match pc {
            0x832711E8 => {
    //   block [0x832711E8..0x83271220)
	// 832711E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832711EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832711F0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832711F4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 832711F8: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 832711FC: 386B43B4  addi r3, r11, 0x43b4
	ctx.r[3].s64 = ctx.r[11].s64 + 17332;
	// 83271200: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83271204: 4AF82B55  bl 0x821f3d58
	ctx.lr = 0x83271208;
	sub_821F3D58(ctx, base);
	// 83271208: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327120C: 906ACEF4  stw r3, -0x310c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-12556 as u32), ctx.r[3].u32 ) };
	// 83271210: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83271214: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83271218: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327121C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83271220(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83271220 size=56
    let mut pc: u32 = 0x83271220;
    'dispatch: loop {
        match pc {
            0x83271220 => {
    //   block [0x83271220..0x83271258)
	// 83271220: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83271224: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83271228: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327122C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83271230: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83271234: 386B43C8  addi r3, r11, 0x43c8
	ctx.r[3].s64 = ctx.r[11].s64 + 17352;
	// 83271238: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8327123C: 4AF82B1D  bl 0x821f3d58
	ctx.lr = 0x83271240;
	sub_821F3D58(ctx, base);
	// 83271240: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83271244: 906ACEF8  stw r3, -0x3108(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-12552 as u32), ctx.r[3].u32 ) };
	// 83271248: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327124C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83271250: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83271254: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83271258(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83271258 size=56
    let mut pc: u32 = 0x83271258;
    'dispatch: loop {
        match pc {
            0x83271258 => {
    //   block [0x83271258..0x83271290)
	// 83271258: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327125C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83271260: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83271264: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83271268: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8327126C: 386B43D4  addi r3, r11, 0x43d4
	ctx.r[3].s64 = ctx.r[11].s64 + 17364;
	// 83271270: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83271274: 4AF82AE5  bl 0x821f3d58
	ctx.lr = 0x83271278;
	sub_821F3D58(ctx, base);
	// 83271278: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327127C: 906ACEFC  stw r3, -0x3104(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-12548 as u32), ctx.r[3].u32 ) };
	// 83271280: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83271284: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83271288: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327128C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83271290(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83271290 size=56
    let mut pc: u32 = 0x83271290;
    'dispatch: loop {
        match pc {
            0x83271290 => {
    //   block [0x83271290..0x832712C8)
	// 83271290: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83271294: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83271298: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327129C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 832712A0: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 832712A4: 386B43E0  addi r3, r11, 0x43e0
	ctx.r[3].s64 = ctx.r[11].s64 + 17376;
	// 832712A8: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 832712AC: 4AF82AAD  bl 0x821f3d58
	ctx.lr = 0x832712B0;
	sub_821F3D58(ctx, base);
	// 832712B0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832712B4: 906ACF00  stw r3, -0x3100(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-12544 as u32), ctx.r[3].u32 ) };
	// 832712B8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832712BC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832712C0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832712C4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832712C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832712C8 size=56
    let mut pc: u32 = 0x832712C8;
    'dispatch: loop {
        match pc {
            0x832712C8 => {
    //   block [0x832712C8..0x83271300)
	// 832712C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832712CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832712D0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832712D4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 832712D8: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 832712DC: 386B43F0  addi r3, r11, 0x43f0
	ctx.r[3].s64 = ctx.r[11].s64 + 17392;
	// 832712E0: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 832712E4: 4AF82A75  bl 0x821f3d58
	ctx.lr = 0x832712E8;
	sub_821F3D58(ctx, base);
	// 832712E8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832712EC: 906ACF04  stw r3, -0x30fc(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-12540 as u32), ctx.r[3].u32 ) };
	// 832712F0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832712F4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832712F8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832712FC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83271300(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83271300 size=56
    let mut pc: u32 = 0x83271300;
    'dispatch: loop {
        match pc {
            0x83271300 => {
    //   block [0x83271300..0x83271338)
	// 83271300: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83271304: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83271308: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327130C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83271310: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83271314: 386B4400  addi r3, r11, 0x4400
	ctx.r[3].s64 = ctx.r[11].s64 + 17408;
	// 83271318: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8327131C: 4AF82A3D  bl 0x821f3d58
	ctx.lr = 0x83271320;
	sub_821F3D58(ctx, base);
	// 83271320: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83271324: 906ACF08  stw r3, -0x30f8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-12536 as u32), ctx.r[3].u32 ) };
	// 83271328: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327132C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83271330: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83271334: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83271338(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83271338 size=56
    let mut pc: u32 = 0x83271338;
    'dispatch: loop {
        match pc {
            0x83271338 => {
    //   block [0x83271338..0x83271370)
	// 83271338: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327133C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83271340: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83271344: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83271348: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8327134C: 386B4418  addi r3, r11, 0x4418
	ctx.r[3].s64 = ctx.r[11].s64 + 17432;
	// 83271350: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83271354: 4AF82A05  bl 0x821f3d58
	ctx.lr = 0x83271358;
	sub_821F3D58(ctx, base);
	// 83271358: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327135C: 906ACF0C  stw r3, -0x30f4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-12532 as u32), ctx.r[3].u32 ) };
	// 83271360: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83271364: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83271368: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327136C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83271370(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83271370 size=56
    let mut pc: u32 = 0x83271370;
    'dispatch: loop {
        match pc {
            0x83271370 => {
    //   block [0x83271370..0x832713A8)
	// 83271370: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83271374: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83271378: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327137C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83271380: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83271384: 386B4424  addi r3, r11, 0x4424
	ctx.r[3].s64 = ctx.r[11].s64 + 17444;
	// 83271388: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8327138C: 4AF829CD  bl 0x821f3d58
	ctx.lr = 0x83271390;
	sub_821F3D58(ctx, base);
	// 83271390: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83271394: 906ACF10  stw r3, -0x30f0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-12528 as u32), ctx.r[3].u32 ) };
	// 83271398: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327139C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832713A0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832713A4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832713A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832713A8 size=56
    let mut pc: u32 = 0x832713A8;
    'dispatch: loop {
        match pc {
            0x832713A8 => {
    //   block [0x832713A8..0x832713E0)
	// 832713A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832713AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832713B0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832713B4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 832713B8: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 832713BC: 386B4430  addi r3, r11, 0x4430
	ctx.r[3].s64 = ctx.r[11].s64 + 17456;
	// 832713C0: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 832713C4: 4AF82995  bl 0x821f3d58
	ctx.lr = 0x832713C8;
	sub_821F3D58(ctx, base);
	// 832713C8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832713CC: 906ACF14  stw r3, -0x30ec(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-12524 as u32), ctx.r[3].u32 ) };
	// 832713D0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832713D4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832713D8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832713DC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832713E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832713E0 size=56
    let mut pc: u32 = 0x832713E0;
    'dispatch: loop {
        match pc {
            0x832713E0 => {
    //   block [0x832713E0..0x83271418)
	// 832713E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832713E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832713E8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832713EC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 832713F0: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 832713F4: 386B4440  addi r3, r11, 0x4440
	ctx.r[3].s64 = ctx.r[11].s64 + 17472;
	// 832713F8: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 832713FC: 4AF8295D  bl 0x821f3d58
	ctx.lr = 0x83271400;
	sub_821F3D58(ctx, base);
	// 83271400: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83271404: 906ACF18  stw r3, -0x30e8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-12520 as u32), ctx.r[3].u32 ) };
	// 83271408: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327140C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83271410: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83271414: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83271418(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83271418 size=64
    let mut pc: u32 = 0x83271418;
    'dispatch: loop {
        match pc {
            0x83271418 => {
    //   block [0x83271418..0x83271458)
	// 83271418: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327141C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83271420: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83271424: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 83271428: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327142C: 388BA694  addi r4, r11, -0x596c
	ctx.r[4].s64 = ctx.r[11].s64 + -22892;
	// 83271430: 386ACF1C  addi r3, r10, -0x30e4
	ctx.r[3].s64 = ctx.r[10].s64 + -12516;
	// 83271434: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83271438: 4AFBBA99  bl 0x8222ced0
	ctx.lr = 0x8327143C;
	sub_8222CED0(ctx, base);
	// 8327143C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83271440: 3869F1F0  addi r3, r9, -0xe10
	ctx.r[3].s64 = ctx.r[9].s64 + -3600;
	// 83271444: 4BA38ADD  bl 0x82ca9f20
	ctx.lr = 0x83271448;
	sub_82CA9F20(ctx, base);
	// 83271448: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327144C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83271450: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83271454: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83271458(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83271458 size=64
    let mut pc: u32 = 0x83271458;
    'dispatch: loop {
        match pc {
            0x83271458 => {
    //   block [0x83271458..0x83271498)
	// 83271458: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327145C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83271460: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83271464: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 83271468: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327146C: 388BA6C4  addi r4, r11, -0x593c
	ctx.r[4].s64 = ctx.r[11].s64 + -22844;
	// 83271470: 386ACF20  addi r3, r10, -0x30e0
	ctx.r[3].s64 = ctx.r[10].s64 + -12512;
	// 83271474: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83271478: 4AFBBA59  bl 0x8222ced0
	ctx.lr = 0x8327147C;
	sub_8222CED0(ctx, base);
	// 8327147C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83271480: 3869F200  addi r3, r9, -0xe00
	ctx.r[3].s64 = ctx.r[9].s64 + -3584;
	// 83271484: 4BA38A9D  bl 0x82ca9f20
	ctx.lr = 0x83271488;
	sub_82CA9F20(ctx, base);
	// 83271488: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327148C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83271490: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83271494: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83271498(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83271498 size=64
    let mut pc: u32 = 0x83271498;
    'dispatch: loop {
        match pc {
            0x83271498 => {
    //   block [0x83271498..0x832714D8)
	// 83271498: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327149C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832714A0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832714A4: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 832714A8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832714AC: 388BA6F4  addi r4, r11, -0x590c
	ctx.r[4].s64 = ctx.r[11].s64 + -22796;
	// 832714B0: 386ACF24  addi r3, r10, -0x30dc
	ctx.r[3].s64 = ctx.r[10].s64 + -12508;
	// 832714B4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832714B8: 4AFBBA19  bl 0x8222ced0
	ctx.lr = 0x832714BC;
	sub_8222CED0(ctx, base);
	// 832714BC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 832714C0: 3869F210  addi r3, r9, -0xdf0
	ctx.r[3].s64 = ctx.r[9].s64 + -3568;
	// 832714C4: 4BA38A5D  bl 0x82ca9f20
	ctx.lr = 0x832714C8;
	sub_82CA9F20(ctx, base);
	// 832714C8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832714CC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832714D0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832714D4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832714D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832714D8 size=64
    let mut pc: u32 = 0x832714D8;
    'dispatch: loop {
        match pc {
            0x832714D8 => {
    //   block [0x832714D8..0x83271518)
	// 832714D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832714DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832714E0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832714E4: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 832714E8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832714EC: 388BA724  addi r4, r11, -0x58dc
	ctx.r[4].s64 = ctx.r[11].s64 + -22748;
	// 832714F0: 386ACF28  addi r3, r10, -0x30d8
	ctx.r[3].s64 = ctx.r[10].s64 + -12504;
	// 832714F4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832714F8: 4AFBB9D9  bl 0x8222ced0
	ctx.lr = 0x832714FC;
	sub_8222CED0(ctx, base);
	// 832714FC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83271500: 3869F220  addi r3, r9, -0xde0
	ctx.r[3].s64 = ctx.r[9].s64 + -3552;
	// 83271504: 4BA38A1D  bl 0x82ca9f20
	ctx.lr = 0x83271508;
	sub_82CA9F20(ctx, base);
	// 83271508: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327150C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83271510: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83271514: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83271518(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83271518 size=64
    let mut pc: u32 = 0x83271518;
    'dispatch: loop {
        match pc {
            0x83271518 => {
    //   block [0x83271518..0x83271558)
	// 83271518: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327151C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83271520: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83271524: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 83271528: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327152C: 388BA760  addi r4, r11, -0x58a0
	ctx.r[4].s64 = ctx.r[11].s64 + -22688;
	// 83271530: 386ACF2C  addi r3, r10, -0x30d4
	ctx.r[3].s64 = ctx.r[10].s64 + -12500;
	// 83271534: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83271538: 4AFBB999  bl 0x8222ced0
	ctx.lr = 0x8327153C;
	sub_8222CED0(ctx, base);
	// 8327153C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83271540: 3869F230  addi r3, r9, -0xdd0
	ctx.r[3].s64 = ctx.r[9].s64 + -3536;
	// 83271544: 4BA389DD  bl 0x82ca9f20
	ctx.lr = 0x83271548;
	sub_82CA9F20(ctx, base);
	// 83271548: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327154C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83271550: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83271554: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83271558(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83271558 size=64
    let mut pc: u32 = 0x83271558;
    'dispatch: loop {
        match pc {
            0x83271558 => {
    //   block [0x83271558..0x83271598)
	// 83271558: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327155C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83271560: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83271564: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 83271568: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327156C: 388BA7A0  addi r4, r11, -0x5860
	ctx.r[4].s64 = ctx.r[11].s64 + -22624;
	// 83271570: 386ACF30  addi r3, r10, -0x30d0
	ctx.r[3].s64 = ctx.r[10].s64 + -12496;
	// 83271574: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83271578: 4AFBB959  bl 0x8222ced0
	ctx.lr = 0x8327157C;
	sub_8222CED0(ctx, base);
	// 8327157C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83271580: 3869F240  addi r3, r9, -0xdc0
	ctx.r[3].s64 = ctx.r[9].s64 + -3520;
	// 83271584: 4BA3899D  bl 0x82ca9f20
	ctx.lr = 0x83271588;
	sub_82CA9F20(ctx, base);
	// 83271588: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327158C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83271590: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83271594: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83271598(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83271598 size=64
    let mut pc: u32 = 0x83271598;
    'dispatch: loop {
        match pc {
            0x83271598 => {
    //   block [0x83271598..0x832715D8)
	// 83271598: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327159C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832715A0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832715A4: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 832715A8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832715AC: 388BA7E8  addi r4, r11, -0x5818
	ctx.r[4].s64 = ctx.r[11].s64 + -22552;
	// 832715B0: 386ACF34  addi r3, r10, -0x30cc
	ctx.r[3].s64 = ctx.r[10].s64 + -12492;
	// 832715B4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832715B8: 4AFBB919  bl 0x8222ced0
	ctx.lr = 0x832715BC;
	sub_8222CED0(ctx, base);
	// 832715BC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 832715C0: 3869F250  addi r3, r9, -0xdb0
	ctx.r[3].s64 = ctx.r[9].s64 + -3504;
	// 832715C4: 4BA3895D  bl 0x82ca9f20
	ctx.lr = 0x832715C8;
	sub_82CA9F20(ctx, base);
	// 832715C8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832715CC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832715D0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832715D4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832715D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832715D8 size=64
    let mut pc: u32 = 0x832715D8;
    'dispatch: loop {
        match pc {
            0x832715D8 => {
    //   block [0x832715D8..0x83271618)
	// 832715D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832715DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832715E0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832715E4: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 832715E8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832715EC: 388BA830  addi r4, r11, -0x57d0
	ctx.r[4].s64 = ctx.r[11].s64 + -22480;
	// 832715F0: 386ACF38  addi r3, r10, -0x30c8
	ctx.r[3].s64 = ctx.r[10].s64 + -12488;
	// 832715F4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832715F8: 4AFBB8D9  bl 0x8222ced0
	ctx.lr = 0x832715FC;
	sub_8222CED0(ctx, base);
	// 832715FC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83271600: 3869F260  addi r3, r9, -0xda0
	ctx.r[3].s64 = ctx.r[9].s64 + -3488;
	// 83271604: 4BA3891D  bl 0x82ca9f20
	ctx.lr = 0x83271608;
	sub_82CA9F20(ctx, base);
	// 83271608: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327160C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83271610: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83271614: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83271618(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83271618 size=64
    let mut pc: u32 = 0x83271618;
    'dispatch: loop {
        match pc {
            0x83271618 => {
    //   block [0x83271618..0x83271658)
	// 83271618: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327161C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83271620: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83271624: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 83271628: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327162C: 388BA878  addi r4, r11, -0x5788
	ctx.r[4].s64 = ctx.r[11].s64 + -22408;
	// 83271630: 386ACF3C  addi r3, r10, -0x30c4
	ctx.r[3].s64 = ctx.r[10].s64 + -12484;
	// 83271634: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83271638: 4AFBB899  bl 0x8222ced0
	ctx.lr = 0x8327163C;
	sub_8222CED0(ctx, base);
	// 8327163C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83271640: 3869F270  addi r3, r9, -0xd90
	ctx.r[3].s64 = ctx.r[9].s64 + -3472;
	// 83271644: 4BA388DD  bl 0x82ca9f20
	ctx.lr = 0x83271648;
	sub_82CA9F20(ctx, base);
	// 83271648: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327164C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83271650: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83271654: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83271658(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83271658 size=64
    let mut pc: u32 = 0x83271658;
    'dispatch: loop {
        match pc {
            0x83271658 => {
    //   block [0x83271658..0x83271698)
	// 83271658: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327165C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83271660: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83271664: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 83271668: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327166C: 388BA8BC  addi r4, r11, -0x5744
	ctx.r[4].s64 = ctx.r[11].s64 + -22340;
	// 83271670: 386ACF40  addi r3, r10, -0x30c0
	ctx.r[3].s64 = ctx.r[10].s64 + -12480;
	// 83271674: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83271678: 4AFBB859  bl 0x8222ced0
	ctx.lr = 0x8327167C;
	sub_8222CED0(ctx, base);
	// 8327167C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83271680: 3869F280  addi r3, r9, -0xd80
	ctx.r[3].s64 = ctx.r[9].s64 + -3456;
	// 83271684: 4BA3889D  bl 0x82ca9f20
	ctx.lr = 0x83271688;
	sub_82CA9F20(ctx, base);
	// 83271688: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327168C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83271690: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83271694: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83271698(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83271698 size=64
    let mut pc: u32 = 0x83271698;
    'dispatch: loop {
        match pc {
            0x83271698 => {
    //   block [0x83271698..0x832716D8)
	// 83271698: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327169C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832716A0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832716A4: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 832716A8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832716AC: 388BA8EC  addi r4, r11, -0x5714
	ctx.r[4].s64 = ctx.r[11].s64 + -22292;
	// 832716B0: 386ACF44  addi r3, r10, -0x30bc
	ctx.r[3].s64 = ctx.r[10].s64 + -12476;
	// 832716B4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832716B8: 4AFBB819  bl 0x8222ced0
	ctx.lr = 0x832716BC;
	sub_8222CED0(ctx, base);
	// 832716BC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 832716C0: 3869F290  addi r3, r9, -0xd70
	ctx.r[3].s64 = ctx.r[9].s64 + -3440;
	// 832716C4: 4BA3885D  bl 0x82ca9f20
	ctx.lr = 0x832716C8;
	sub_82CA9F20(ctx, base);
	// 832716C8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832716CC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832716D0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832716D4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832716D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832716D8 size=64
    let mut pc: u32 = 0x832716D8;
    'dispatch: loop {
        match pc {
            0x832716D8 => {
    //   block [0x832716D8..0x83271718)
	// 832716D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832716DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832716E0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832716E4: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 832716E8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832716EC: 388BA924  addi r4, r11, -0x56dc
	ctx.r[4].s64 = ctx.r[11].s64 + -22236;
	// 832716F0: 386ACF48  addi r3, r10, -0x30b8
	ctx.r[3].s64 = ctx.r[10].s64 + -12472;
	// 832716F4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832716F8: 4AFBB7D9  bl 0x8222ced0
	ctx.lr = 0x832716FC;
	sub_8222CED0(ctx, base);
	// 832716FC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83271700: 3869F2A0  addi r3, r9, -0xd60
	ctx.r[3].s64 = ctx.r[9].s64 + -3424;
	// 83271704: 4BA3881D  bl 0x82ca9f20
	ctx.lr = 0x83271708;
	sub_82CA9F20(ctx, base);
	// 83271708: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327170C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83271710: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83271714: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83271718(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83271718 size=64
    let mut pc: u32 = 0x83271718;
    'dispatch: loop {
        match pc {
            0x83271718 => {
    //   block [0x83271718..0x83271758)
	// 83271718: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327171C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83271720: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83271724: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 83271728: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327172C: 388BDF1C  addi r4, r11, -0x20e4
	ctx.r[4].s64 = ctx.r[11].s64 + -8420;
	// 83271730: 386ACF4C  addi r3, r10, -0x30b4
	ctx.r[3].s64 = ctx.r[10].s64 + -12468;
	// 83271734: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83271738: 4AFBB799  bl 0x8222ced0
	ctx.lr = 0x8327173C;
	sub_8222CED0(ctx, base);
	// 8327173C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83271740: 3869F2B0  addi r3, r9, -0xd50
	ctx.r[3].s64 = ctx.r[9].s64 + -3408;
	// 83271744: 4BA387DD  bl 0x82ca9f20
	ctx.lr = 0x83271748;
	sub_82CA9F20(ctx, base);
	// 83271748: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327174C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83271750: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83271754: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83271758(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83271758 size=56
    let mut pc: u32 = 0x83271758;
    'dispatch: loop {
        match pc {
            0x83271758 => {
    //   block [0x83271758..0x83271790)
	// 83271758: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327175C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83271760: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83271764: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83271768: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8327176C: 386B42C8  addi r3, r11, 0x42c8
	ctx.r[3].s64 = ctx.r[11].s64 + 17096;
	// 83271770: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83271774: 4AF825E5  bl 0x821f3d58
	ctx.lr = 0x83271778;
	sub_821F3D58(ctx, base);
	// 83271778: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327177C: 906ACF50  stw r3, -0x30b0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-12464 as u32), ctx.r[3].u32 ) };
	// 83271780: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83271784: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83271788: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327178C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83271790(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83271790 size=56
    let mut pc: u32 = 0x83271790;
    'dispatch: loop {
        match pc {
            0x83271790 => {
    //   block [0x83271790..0x832717C8)
	// 83271790: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83271794: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83271798: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327179C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 832717A0: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 832717A4: 386B42DC  addi r3, r11, 0x42dc
	ctx.r[3].s64 = ctx.r[11].s64 + 17116;
	// 832717A8: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 832717AC: 4AF825AD  bl 0x821f3d58
	ctx.lr = 0x832717B0;
	sub_821F3D58(ctx, base);
	// 832717B0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832717B4: 906ACF54  stw r3, -0x30ac(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-12460 as u32), ctx.r[3].u32 ) };
	// 832717B8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832717BC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832717C0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832717C4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832717C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832717C8 size=56
    let mut pc: u32 = 0x832717C8;
    'dispatch: loop {
        match pc {
            0x832717C8 => {
    //   block [0x832717C8..0x83271800)
	// 832717C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832717CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832717D0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832717D4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 832717D8: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 832717DC: 386B42E8  addi r3, r11, 0x42e8
	ctx.r[3].s64 = ctx.r[11].s64 + 17128;
	// 832717E0: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 832717E4: 4AF82575  bl 0x821f3d58
	ctx.lr = 0x832717E8;
	sub_821F3D58(ctx, base);
	// 832717E8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832717EC: 906ACF58  stw r3, -0x30a8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-12456 as u32), ctx.r[3].u32 ) };
	// 832717F0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832717F4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832717F8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832717FC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83271800(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83271800 size=56
    let mut pc: u32 = 0x83271800;
    'dispatch: loop {
        match pc {
            0x83271800 => {
    //   block [0x83271800..0x83271838)
	// 83271800: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83271804: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83271808: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327180C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83271810: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83271814: 386B4300  addi r3, r11, 0x4300
	ctx.r[3].s64 = ctx.r[11].s64 + 17152;
	// 83271818: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8327181C: 4AF8253D  bl 0x821f3d58
	ctx.lr = 0x83271820;
	sub_821F3D58(ctx, base);
	// 83271820: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83271824: 906ACF5C  stw r3, -0x30a4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-12452 as u32), ctx.r[3].u32 ) };
	// 83271828: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327182C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83271830: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83271834: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83271838(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83271838 size=56
    let mut pc: u32 = 0x83271838;
    'dispatch: loop {
        match pc {
            0x83271838 => {
    //   block [0x83271838..0x83271870)
	// 83271838: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327183C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83271840: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83271844: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83271848: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8327184C: 386B4310  addi r3, r11, 0x4310
	ctx.r[3].s64 = ctx.r[11].s64 + 17168;
	// 83271850: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83271854: 4AF82505  bl 0x821f3d58
	ctx.lr = 0x83271858;
	sub_821F3D58(ctx, base);
	// 83271858: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327185C: 906ACF60  stw r3, -0x30a0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-12448 as u32), ctx.r[3].u32 ) };
	// 83271860: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83271864: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83271868: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327186C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83271870(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83271870 size=56
    let mut pc: u32 = 0x83271870;
    'dispatch: loop {
        match pc {
            0x83271870 => {
    //   block [0x83271870..0x832718A8)
	// 83271870: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83271874: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83271878: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327187C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83271880: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83271884: 386B4328  addi r3, r11, 0x4328
	ctx.r[3].s64 = ctx.r[11].s64 + 17192;
	// 83271888: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8327188C: 4AF824CD  bl 0x821f3d58
	ctx.lr = 0x83271890;
	sub_821F3D58(ctx, base);
	// 83271890: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83271894: 906ACF64  stw r3, -0x309c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-12444 as u32), ctx.r[3].u32 ) };
	// 83271898: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327189C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832718A0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832718A4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832718A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832718A8 size=56
    let mut pc: u32 = 0x832718A8;
    'dispatch: loop {
        match pc {
            0x832718A8 => {
    //   block [0x832718A8..0x832718E0)
	// 832718A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832718AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832718B0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832718B4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 832718B8: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 832718BC: 386B4338  addi r3, r11, 0x4338
	ctx.r[3].s64 = ctx.r[11].s64 + 17208;
	// 832718C0: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 832718C4: 4AF82495  bl 0x821f3d58
	ctx.lr = 0x832718C8;
	sub_821F3D58(ctx, base);
	// 832718C8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832718CC: 906ACF68  stw r3, -0x3098(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-12440 as u32), ctx.r[3].u32 ) };
	// 832718D0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832718D4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832718D8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832718DC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832718E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832718E0 size=56
    let mut pc: u32 = 0x832718E0;
    'dispatch: loop {
        match pc {
            0x832718E0 => {
    //   block [0x832718E0..0x83271918)
	// 832718E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832718E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832718E8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832718EC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 832718F0: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 832718F4: 386B4350  addi r3, r11, 0x4350
	ctx.r[3].s64 = ctx.r[11].s64 + 17232;
	// 832718F8: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 832718FC: 4AF8245D  bl 0x821f3d58
	ctx.lr = 0x83271900;
	sub_821F3D58(ctx, base);
	// 83271900: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83271904: 906ACF6C  stw r3, -0x3094(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-12436 as u32), ctx.r[3].u32 ) };
	// 83271908: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327190C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83271910: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83271914: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83271918(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83271918 size=56
    let mut pc: u32 = 0x83271918;
    'dispatch: loop {
        match pc {
            0x83271918 => {
    //   block [0x83271918..0x83271950)
	// 83271918: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327191C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83271920: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83271924: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83271928: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8327192C: 386B4368  addi r3, r11, 0x4368
	ctx.r[3].s64 = ctx.r[11].s64 + 17256;
	// 83271930: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83271934: 4AF82425  bl 0x821f3d58
	ctx.lr = 0x83271938;
	sub_821F3D58(ctx, base);
	// 83271938: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327193C: 906ACF70  stw r3, -0x3090(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-12432 as u32), ctx.r[3].u32 ) };
	// 83271940: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83271944: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83271948: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327194C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83271950(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83271950 size=56
    let mut pc: u32 = 0x83271950;
    'dispatch: loop {
        match pc {
            0x83271950 => {
    //   block [0x83271950..0x83271988)
	// 83271950: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83271954: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83271958: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327195C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83271960: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83271964: 386B4380  addi r3, r11, 0x4380
	ctx.r[3].s64 = ctx.r[11].s64 + 17280;
	// 83271968: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8327196C: 4AF823ED  bl 0x821f3d58
	ctx.lr = 0x83271970;
	sub_821F3D58(ctx, base);
	// 83271970: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83271974: 906ACF74  stw r3, -0x308c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-12428 as u32), ctx.r[3].u32 ) };
	// 83271978: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327197C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83271980: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83271984: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83271988(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83271988 size=56
    let mut pc: u32 = 0x83271988;
    'dispatch: loop {
        match pc {
            0x83271988 => {
    //   block [0x83271988..0x832719C0)
	// 83271988: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327198C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83271990: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83271994: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83271998: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8327199C: 386B4394  addi r3, r11, 0x4394
	ctx.r[3].s64 = ctx.r[11].s64 + 17300;
	// 832719A0: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 832719A4: 4AF823B5  bl 0x821f3d58
	ctx.lr = 0x832719A8;
	sub_821F3D58(ctx, base);
	// 832719A8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832719AC: 906ACF78  stw r3, -0x3088(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-12424 as u32), ctx.r[3].u32 ) };
	// 832719B0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832719B4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832719B8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832719BC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832719C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832719C0 size=56
    let mut pc: u32 = 0x832719C0;
    'dispatch: loop {
        match pc {
            0x832719C0 => {
    //   block [0x832719C0..0x832719F8)
	// 832719C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832719C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832719C8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832719CC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 832719D0: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 832719D4: 386B43B4  addi r3, r11, 0x43b4
	ctx.r[3].s64 = ctx.r[11].s64 + 17332;
	// 832719D8: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 832719DC: 4AF8237D  bl 0x821f3d58
	ctx.lr = 0x832719E0;
	sub_821F3D58(ctx, base);
	// 832719E0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832719E4: 906ACF7C  stw r3, -0x3084(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-12420 as u32), ctx.r[3].u32 ) };
	// 832719E8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832719EC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832719F0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832719F4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832719F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832719F8 size=56
    let mut pc: u32 = 0x832719F8;
    'dispatch: loop {
        match pc {
            0x832719F8 => {
    //   block [0x832719F8..0x83271A30)
	// 832719F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832719FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83271A00: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83271A04: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83271A08: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83271A0C: 386B43C8  addi r3, r11, 0x43c8
	ctx.r[3].s64 = ctx.r[11].s64 + 17352;
	// 83271A10: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83271A14: 4AF82345  bl 0x821f3d58
	ctx.lr = 0x83271A18;
	sub_821F3D58(ctx, base);
	// 83271A18: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83271A1C: 906ACF80  stw r3, -0x3080(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-12416 as u32), ctx.r[3].u32 ) };
	// 83271A20: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83271A24: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83271A28: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83271A2C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83271A30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83271A30 size=56
    let mut pc: u32 = 0x83271A30;
    'dispatch: loop {
        match pc {
            0x83271A30 => {
    //   block [0x83271A30..0x83271A68)
	// 83271A30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83271A34: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83271A38: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83271A3C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83271A40: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83271A44: 386B43D4  addi r3, r11, 0x43d4
	ctx.r[3].s64 = ctx.r[11].s64 + 17364;
	// 83271A48: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83271A4C: 4AF8230D  bl 0x821f3d58
	ctx.lr = 0x83271A50;
	sub_821F3D58(ctx, base);
	// 83271A50: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83271A54: 906ACF84  stw r3, -0x307c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-12412 as u32), ctx.r[3].u32 ) };
	// 83271A58: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83271A5C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83271A60: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83271A64: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83271A68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83271A68 size=56
    let mut pc: u32 = 0x83271A68;
    'dispatch: loop {
        match pc {
            0x83271A68 => {
    //   block [0x83271A68..0x83271AA0)
	// 83271A68: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83271A6C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83271A70: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83271A74: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83271A78: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83271A7C: 386B43E0  addi r3, r11, 0x43e0
	ctx.r[3].s64 = ctx.r[11].s64 + 17376;
	// 83271A80: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83271A84: 4AF822D5  bl 0x821f3d58
	ctx.lr = 0x83271A88;
	sub_821F3D58(ctx, base);
	// 83271A88: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83271A8C: 906ACF88  stw r3, -0x3078(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-12408 as u32), ctx.r[3].u32 ) };
	// 83271A90: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83271A94: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83271A98: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83271A9C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83271AA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83271AA0 size=56
    let mut pc: u32 = 0x83271AA0;
    'dispatch: loop {
        match pc {
            0x83271AA0 => {
    //   block [0x83271AA0..0x83271AD8)
	// 83271AA0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83271AA4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83271AA8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83271AAC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83271AB0: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83271AB4: 386B43F0  addi r3, r11, 0x43f0
	ctx.r[3].s64 = ctx.r[11].s64 + 17392;
	// 83271AB8: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83271ABC: 4AF8229D  bl 0x821f3d58
	ctx.lr = 0x83271AC0;
	sub_821F3D58(ctx, base);
	// 83271AC0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83271AC4: 906ACF8C  stw r3, -0x3074(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-12404 as u32), ctx.r[3].u32 ) };
	// 83271AC8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83271ACC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83271AD0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83271AD4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83271AD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83271AD8 size=56
    let mut pc: u32 = 0x83271AD8;
    'dispatch: loop {
        match pc {
            0x83271AD8 => {
    //   block [0x83271AD8..0x83271B10)
	// 83271AD8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83271ADC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83271AE0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83271AE4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83271AE8: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83271AEC: 386B4400  addi r3, r11, 0x4400
	ctx.r[3].s64 = ctx.r[11].s64 + 17408;
	// 83271AF0: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83271AF4: 4AF82265  bl 0x821f3d58
	ctx.lr = 0x83271AF8;
	sub_821F3D58(ctx, base);
	// 83271AF8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83271AFC: 906ACF90  stw r3, -0x3070(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-12400 as u32), ctx.r[3].u32 ) };
	// 83271B00: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83271B04: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83271B08: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83271B0C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83271B10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83271B10 size=56
    let mut pc: u32 = 0x83271B10;
    'dispatch: loop {
        match pc {
            0x83271B10 => {
    //   block [0x83271B10..0x83271B48)
	// 83271B10: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83271B14: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83271B18: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83271B1C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83271B20: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83271B24: 386B4418  addi r3, r11, 0x4418
	ctx.r[3].s64 = ctx.r[11].s64 + 17432;
	// 83271B28: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83271B2C: 4AF8222D  bl 0x821f3d58
	ctx.lr = 0x83271B30;
	sub_821F3D58(ctx, base);
	// 83271B30: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83271B34: 906ACF94  stw r3, -0x306c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-12396 as u32), ctx.r[3].u32 ) };
	// 83271B38: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83271B3C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83271B40: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83271B44: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83271B48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83271B48 size=56
    let mut pc: u32 = 0x83271B48;
    'dispatch: loop {
        match pc {
            0x83271B48 => {
    //   block [0x83271B48..0x83271B80)
	// 83271B48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83271B4C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83271B50: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83271B54: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83271B58: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83271B5C: 386B4424  addi r3, r11, 0x4424
	ctx.r[3].s64 = ctx.r[11].s64 + 17444;
	// 83271B60: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83271B64: 4AF821F5  bl 0x821f3d58
	ctx.lr = 0x83271B68;
	sub_821F3D58(ctx, base);
	// 83271B68: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83271B6C: 906ACF98  stw r3, -0x3068(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-12392 as u32), ctx.r[3].u32 ) };
	// 83271B70: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83271B74: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83271B78: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83271B7C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83271B80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83271B80 size=56
    let mut pc: u32 = 0x83271B80;
    'dispatch: loop {
        match pc {
            0x83271B80 => {
    //   block [0x83271B80..0x83271BB8)
	// 83271B80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83271B84: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83271B88: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83271B8C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83271B90: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83271B94: 386B4430  addi r3, r11, 0x4430
	ctx.r[3].s64 = ctx.r[11].s64 + 17456;
	// 83271B98: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83271B9C: 4AF821BD  bl 0x821f3d58
	ctx.lr = 0x83271BA0;
	sub_821F3D58(ctx, base);
	// 83271BA0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83271BA4: 906ACF9C  stw r3, -0x3064(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-12388 as u32), ctx.r[3].u32 ) };
	// 83271BA8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83271BAC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83271BB0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83271BB4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83271BB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83271BB8 size=56
    let mut pc: u32 = 0x83271BB8;
    'dispatch: loop {
        match pc {
            0x83271BB8 => {
    //   block [0x83271BB8..0x83271BF0)
	// 83271BB8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83271BBC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83271BC0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83271BC4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83271BC8: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83271BCC: 386B4440  addi r3, r11, 0x4440
	ctx.r[3].s64 = ctx.r[11].s64 + 17472;
	// 83271BD0: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83271BD4: 4AF82185  bl 0x821f3d58
	ctx.lr = 0x83271BD8;
	sub_821F3D58(ctx, base);
	// 83271BD8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83271BDC: 906ACFA0  stw r3, -0x3060(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-12384 as u32), ctx.r[3].u32 ) };
	// 83271BE0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83271BE4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83271BE8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83271BEC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83271BF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83271BF0 size=64
    let mut pc: u32 = 0x83271BF0;
    'dispatch: loop {
        match pc {
            0x83271BF0 => {
    //   block [0x83271BF0..0x83271C30)
	// 83271BF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83271BF4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83271BF8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83271BFC: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 83271C00: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83271C04: 388BA9A4  addi r4, r11, -0x565c
	ctx.r[4].s64 = ctx.r[11].s64 + -22108;
	// 83271C08: 386ACFA4  addi r3, r10, -0x305c
	ctx.r[3].s64 = ctx.r[10].s64 + -12380;
	// 83271C0C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83271C10: 4AFBB2C1  bl 0x8222ced0
	ctx.lr = 0x83271C14;
	sub_8222CED0(ctx, base);
	// 83271C14: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83271C18: 3869F2C0  addi r3, r9, -0xd40
	ctx.r[3].s64 = ctx.r[9].s64 + -3392;
	// 83271C1C: 4BA38305  bl 0x82ca9f20
	ctx.lr = 0x83271C20;
	sub_82CA9F20(ctx, base);
	// 83271C20: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83271C24: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83271C28: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83271C2C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83271C30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83271C30 size=64
    let mut pc: u32 = 0x83271C30;
    'dispatch: loop {
        match pc {
            0x83271C30 => {
    //   block [0x83271C30..0x83271C70)
	// 83271C30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83271C34: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83271C38: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83271C3C: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 83271C40: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83271C44: 388BA9D4  addi r4, r11, -0x562c
	ctx.r[4].s64 = ctx.r[11].s64 + -22060;
	// 83271C48: 386ACFA8  addi r3, r10, -0x3058
	ctx.r[3].s64 = ctx.r[10].s64 + -12376;
	// 83271C4C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83271C50: 4AFBB281  bl 0x8222ced0
	ctx.lr = 0x83271C54;
	sub_8222CED0(ctx, base);
	// 83271C54: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83271C58: 3869F2D0  addi r3, r9, -0xd30
	ctx.r[3].s64 = ctx.r[9].s64 + -3376;
	// 83271C5C: 4BA382C5  bl 0x82ca9f20
	ctx.lr = 0x83271C60;
	sub_82CA9F20(ctx, base);
	// 83271C60: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83271C64: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83271C68: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83271C6C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83271C70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83271C70 size=64
    let mut pc: u32 = 0x83271C70;
    'dispatch: loop {
        match pc {
            0x83271C70 => {
    //   block [0x83271C70..0x83271CB0)
	// 83271C70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83271C74: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83271C78: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83271C7C: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 83271C80: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83271C84: 388BAA08  addi r4, r11, -0x55f8
	ctx.r[4].s64 = ctx.r[11].s64 + -22008;
	// 83271C88: 386ACFAC  addi r3, r10, -0x3054
	ctx.r[3].s64 = ctx.r[10].s64 + -12372;
	// 83271C8C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83271C90: 4AFBB241  bl 0x8222ced0
	ctx.lr = 0x83271C94;
	sub_8222CED0(ctx, base);
	// 83271C94: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83271C98: 3869F2E0  addi r3, r9, -0xd20
	ctx.r[3].s64 = ctx.r[9].s64 + -3360;
	// 83271C9C: 4BA38285  bl 0x82ca9f20
	ctx.lr = 0x83271CA0;
	sub_82CA9F20(ctx, base);
	// 83271CA0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83271CA4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83271CA8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83271CAC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83271CB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83271CB0 size=64
    let mut pc: u32 = 0x83271CB0;
    'dispatch: loop {
        match pc {
            0x83271CB0 => {
    //   block [0x83271CB0..0x83271CF0)
	// 83271CB0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83271CB4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83271CB8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83271CBC: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 83271CC0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83271CC4: 388BAA50  addi r4, r11, -0x55b0
	ctx.r[4].s64 = ctx.r[11].s64 + -21936;
	// 83271CC8: 386ACFB0  addi r3, r10, -0x3050
	ctx.r[3].s64 = ctx.r[10].s64 + -12368;
	// 83271CCC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83271CD0: 4AFBB201  bl 0x8222ced0
	ctx.lr = 0x83271CD4;
	sub_8222CED0(ctx, base);
	// 83271CD4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83271CD8: 3869F2F0  addi r3, r9, -0xd10
	ctx.r[3].s64 = ctx.r[9].s64 + -3344;
	// 83271CDC: 4BA38245  bl 0x82ca9f20
	ctx.lr = 0x83271CE0;
	sub_82CA9F20(ctx, base);
	// 83271CE0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83271CE4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83271CE8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83271CEC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83271CF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83271CF0 size=64
    let mut pc: u32 = 0x83271CF0;
    'dispatch: loop {
        match pc {
            0x83271CF0 => {
    //   block [0x83271CF0..0x83271D30)
	// 83271CF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83271CF4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83271CF8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83271CFC: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 83271D00: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83271D04: 388BAA88  addi r4, r11, -0x5578
	ctx.r[4].s64 = ctx.r[11].s64 + -21880;
	// 83271D08: 386ACFB4  addi r3, r10, -0x304c
	ctx.r[3].s64 = ctx.r[10].s64 + -12364;
	// 83271D0C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83271D10: 4AFBB1C1  bl 0x8222ced0
	ctx.lr = 0x83271D14;
	sub_8222CED0(ctx, base);
	// 83271D14: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83271D18: 3869F300  addi r3, r9, -0xd00
	ctx.r[3].s64 = ctx.r[9].s64 + -3328;
	// 83271D1C: 4BA38205  bl 0x82ca9f20
	ctx.lr = 0x83271D20;
	sub_82CA9F20(ctx, base);
	// 83271D20: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83271D24: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83271D28: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83271D2C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83271D30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83271D30 size=64
    let mut pc: u32 = 0x83271D30;
    'dispatch: loop {
        match pc {
            0x83271D30 => {
    //   block [0x83271D30..0x83271D70)
	// 83271D30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83271D34: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83271D38: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83271D3C: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 83271D40: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83271D44: 388BAAC4  addi r4, r11, -0x553c
	ctx.r[4].s64 = ctx.r[11].s64 + -21820;
	// 83271D48: 386ACFB8  addi r3, r10, -0x3048
	ctx.r[3].s64 = ctx.r[10].s64 + -12360;
	// 83271D4C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83271D50: 4AFBB181  bl 0x8222ced0
	ctx.lr = 0x83271D54;
	sub_8222CED0(ctx, base);
	// 83271D54: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83271D58: 3869F310  addi r3, r9, -0xcf0
	ctx.r[3].s64 = ctx.r[9].s64 + -3312;
	// 83271D5C: 4BA381C5  bl 0x82ca9f20
	ctx.lr = 0x83271D60;
	sub_82CA9F20(ctx, base);
	// 83271D60: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83271D64: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83271D68: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83271D6C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83271D70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83271D70 size=64
    let mut pc: u32 = 0x83271D70;
    'dispatch: loop {
        match pc {
            0x83271D70 => {
    //   block [0x83271D70..0x83271DB0)
	// 83271D70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83271D74: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83271D78: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83271D7C: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 83271D80: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83271D84: 388BAAF4  addi r4, r11, -0x550c
	ctx.r[4].s64 = ctx.r[11].s64 + -21772;
	// 83271D88: 386ACFBC  addi r3, r10, -0x3044
	ctx.r[3].s64 = ctx.r[10].s64 + -12356;
	// 83271D8C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83271D90: 4AFBB141  bl 0x8222ced0
	ctx.lr = 0x83271D94;
	sub_8222CED0(ctx, base);
	// 83271D94: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83271D98: 3869F320  addi r3, r9, -0xce0
	ctx.r[3].s64 = ctx.r[9].s64 + -3296;
	// 83271D9C: 4BA38185  bl 0x82ca9f20
	ctx.lr = 0x83271DA0;
	sub_82CA9F20(ctx, base);
	// 83271DA0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83271DA4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83271DA8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83271DAC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83271DB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83271DB0 size=64
    let mut pc: u32 = 0x83271DB0;
    'dispatch: loop {
        match pc {
            0x83271DB0 => {
    //   block [0x83271DB0..0x83271DF0)
	// 83271DB0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83271DB4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83271DB8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83271DBC: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 83271DC0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83271DC4: 388BAB2C  addi r4, r11, -0x54d4
	ctx.r[4].s64 = ctx.r[11].s64 + -21716;
	// 83271DC8: 386ACFC0  addi r3, r10, -0x3040
	ctx.r[3].s64 = ctx.r[10].s64 + -12352;
	// 83271DCC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83271DD0: 4AFBB101  bl 0x8222ced0
	ctx.lr = 0x83271DD4;
	sub_8222CED0(ctx, base);
	// 83271DD4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83271DD8: 3869F330  addi r3, r9, -0xcd0
	ctx.r[3].s64 = ctx.r[9].s64 + -3280;
	// 83271DDC: 4BA38145  bl 0x82ca9f20
	ctx.lr = 0x83271DE0;
	sub_82CA9F20(ctx, base);
	// 83271DE0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83271DE4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83271DE8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83271DEC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83271DF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83271DF0 size=64
    let mut pc: u32 = 0x83271DF0;
    'dispatch: loop {
        match pc {
            0x83271DF0 => {
    //   block [0x83271DF0..0x83271E30)
	// 83271DF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83271DF4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83271DF8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83271DFC: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 83271E00: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83271E04: 388BAB64  addi r4, r11, -0x549c
	ctx.r[4].s64 = ctx.r[11].s64 + -21660;
	// 83271E08: 386ACFC4  addi r3, r10, -0x303c
	ctx.r[3].s64 = ctx.r[10].s64 + -12348;
	// 83271E0C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83271E10: 4AFBB0C1  bl 0x8222ced0
	ctx.lr = 0x83271E14;
	sub_8222CED0(ctx, base);
	// 83271E14: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83271E18: 3869F340  addi r3, r9, -0xcc0
	ctx.r[3].s64 = ctx.r[9].s64 + -3264;
	// 83271E1C: 4BA38105  bl 0x82ca9f20
	ctx.lr = 0x83271E20;
	sub_82CA9F20(ctx, base);
	// 83271E20: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83271E24: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83271E28: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83271E2C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83271E30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83271E30 size=64
    let mut pc: u32 = 0x83271E30;
    'dispatch: loop {
        match pc {
            0x83271E30 => {
    //   block [0x83271E30..0x83271E70)
	// 83271E30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83271E34: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83271E38: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83271E3C: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 83271E40: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83271E44: 388BABA4  addi r4, r11, -0x545c
	ctx.r[4].s64 = ctx.r[11].s64 + -21596;
	// 83271E48: 386ACFC8  addi r3, r10, -0x3038
	ctx.r[3].s64 = ctx.r[10].s64 + -12344;
	// 83271E4C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83271E50: 4AFBB081  bl 0x8222ced0
	ctx.lr = 0x83271E54;
	sub_8222CED0(ctx, base);
	// 83271E54: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83271E58: 3869F350  addi r3, r9, -0xcb0
	ctx.r[3].s64 = ctx.r[9].s64 + -3248;
	// 83271E5C: 4BA380C5  bl 0x82ca9f20
	ctx.lr = 0x83271E60;
	sub_82CA9F20(ctx, base);
	// 83271E60: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83271E64: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83271E68: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83271E6C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83271E70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83271E70 size=64
    let mut pc: u32 = 0x83271E70;
    'dispatch: loop {
        match pc {
            0x83271E70 => {
    //   block [0x83271E70..0x83271EB0)
	// 83271E70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83271E74: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83271E78: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83271E7C: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 83271E80: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83271E84: 388BABE4  addi r4, r11, -0x541c
	ctx.r[4].s64 = ctx.r[11].s64 + -21532;
	// 83271E88: 386ACFCC  addi r3, r10, -0x3034
	ctx.r[3].s64 = ctx.r[10].s64 + -12340;
	// 83271E8C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83271E90: 4AFBB041  bl 0x8222ced0
	ctx.lr = 0x83271E94;
	sub_8222CED0(ctx, base);
	// 83271E94: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83271E98: 3869F360  addi r3, r9, -0xca0
	ctx.r[3].s64 = ctx.r[9].s64 + -3232;
	// 83271E9C: 4BA38085  bl 0x82ca9f20
	ctx.lr = 0x83271EA0;
	sub_82CA9F20(ctx, base);
	// 83271EA0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83271EA4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83271EA8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83271EAC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83271EB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83271EB0 size=64
    let mut pc: u32 = 0x83271EB0;
    'dispatch: loop {
        match pc {
            0x83271EB0 => {
    //   block [0x83271EB0..0x83271EF0)
	// 83271EB0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83271EB4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83271EB8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83271EBC: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 83271EC0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83271EC4: 388BAC20  addi r4, r11, -0x53e0
	ctx.r[4].s64 = ctx.r[11].s64 + -21472;
	// 83271EC8: 386ACFD0  addi r3, r10, -0x3030
	ctx.r[3].s64 = ctx.r[10].s64 + -12336;
	// 83271ECC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83271ED0: 4AFBB001  bl 0x8222ced0
	ctx.lr = 0x83271ED4;
	sub_8222CED0(ctx, base);
	// 83271ED4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83271ED8: 3869F370  addi r3, r9, -0xc90
	ctx.r[3].s64 = ctx.r[9].s64 + -3216;
	// 83271EDC: 4BA38045  bl 0x82ca9f20
	ctx.lr = 0x83271EE0;
	sub_82CA9F20(ctx, base);
	// 83271EE0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83271EE4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83271EE8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83271EEC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83271EF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83271EF0 size=64
    let mut pc: u32 = 0x83271EF0;
    'dispatch: loop {
        match pc {
            0x83271EF0 => {
    //   block [0x83271EF0..0x83271F30)
	// 83271EF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83271EF4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83271EF8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83271EFC: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 83271F00: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83271F04: 388BAC58  addi r4, r11, -0x53a8
	ctx.r[4].s64 = ctx.r[11].s64 + -21416;
	// 83271F08: 386ACFD4  addi r3, r10, -0x302c
	ctx.r[3].s64 = ctx.r[10].s64 + -12332;
	// 83271F0C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83271F10: 4AFBAFC1  bl 0x8222ced0
	ctx.lr = 0x83271F14;
	sub_8222CED0(ctx, base);
	// 83271F14: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83271F18: 3869F380  addi r3, r9, -0xc80
	ctx.r[3].s64 = ctx.r[9].s64 + -3200;
	// 83271F1C: 4BA38005  bl 0x82ca9f20
	ctx.lr = 0x83271F20;
	sub_82CA9F20(ctx, base);
	// 83271F20: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83271F24: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83271F28: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83271F2C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83271F30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83271F30 size=64
    let mut pc: u32 = 0x83271F30;
    'dispatch: loop {
        match pc {
            0x83271F30 => {
    //   block [0x83271F30..0x83271F70)
	// 83271F30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83271F34: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83271F38: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83271F3C: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 83271F40: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83271F44: 388BAC90  addi r4, r11, -0x5370
	ctx.r[4].s64 = ctx.r[11].s64 + -21360;
	// 83271F48: 386ACFD8  addi r3, r10, -0x3028
	ctx.r[3].s64 = ctx.r[10].s64 + -12328;
	// 83271F4C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83271F50: 4AFBAF81  bl 0x8222ced0
	ctx.lr = 0x83271F54;
	sub_8222CED0(ctx, base);
	// 83271F54: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83271F58: 3869F390  addi r3, r9, -0xc70
	ctx.r[3].s64 = ctx.r[9].s64 + -3184;
	// 83271F5C: 4BA37FC5  bl 0x82ca9f20
	ctx.lr = 0x83271F60;
	sub_82CA9F20(ctx, base);
	// 83271F60: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83271F64: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83271F68: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83271F6C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83271F70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83271F70 size=64
    let mut pc: u32 = 0x83271F70;
    'dispatch: loop {
        match pc {
            0x83271F70 => {
    //   block [0x83271F70..0x83271FB0)
	// 83271F70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83271F74: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83271F78: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83271F7C: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 83271F80: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83271F84: 388BACD0  addi r4, r11, -0x5330
	ctx.r[4].s64 = ctx.r[11].s64 + -21296;
	// 83271F88: 386ACFDC  addi r3, r10, -0x3024
	ctx.r[3].s64 = ctx.r[10].s64 + -12324;
	// 83271F8C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83271F90: 4AFBAF41  bl 0x8222ced0
	ctx.lr = 0x83271F94;
	sub_8222CED0(ctx, base);
	// 83271F94: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83271F98: 3869F3A0  addi r3, r9, -0xc60
	ctx.r[3].s64 = ctx.r[9].s64 + -3168;
	// 83271F9C: 4BA37F85  bl 0x82ca9f20
	ctx.lr = 0x83271FA0;
	sub_82CA9F20(ctx, base);
	// 83271FA0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83271FA4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83271FA8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83271FAC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83271FB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83271FB0 size=64
    let mut pc: u32 = 0x83271FB0;
    'dispatch: loop {
        match pc {
            0x83271FB0 => {
    //   block [0x83271FB0..0x83271FF0)
	// 83271FB0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83271FB4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83271FB8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83271FBC: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 83271FC0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83271FC4: 388BAD08  addi r4, r11, -0x52f8
	ctx.r[4].s64 = ctx.r[11].s64 + -21240;
	// 83271FC8: 386ACFE0  addi r3, r10, -0x3020
	ctx.r[3].s64 = ctx.r[10].s64 + -12320;
	// 83271FCC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83271FD0: 4AFBAF01  bl 0x8222ced0
	ctx.lr = 0x83271FD4;
	sub_8222CED0(ctx, base);
	// 83271FD4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83271FD8: 3869F3B0  addi r3, r9, -0xc50
	ctx.r[3].s64 = ctx.r[9].s64 + -3152;
	// 83271FDC: 4BA37F45  bl 0x82ca9f20
	ctx.lr = 0x83271FE0;
	sub_82CA9F20(ctx, base);
	// 83271FE0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83271FE4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83271FE8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83271FEC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83271FF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83271FF0 size=64
    let mut pc: u32 = 0x83271FF0;
    'dispatch: loop {
        match pc {
            0x83271FF0 => {
    //   block [0x83271FF0..0x83272030)
	// 83271FF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83271FF4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83271FF8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83271FFC: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 83272000: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83272004: 388BAD48  addi r4, r11, -0x52b8
	ctx.r[4].s64 = ctx.r[11].s64 + -21176;
	// 83272008: 386ACFE4  addi r3, r10, -0x301c
	ctx.r[3].s64 = ctx.r[10].s64 + -12316;
	// 8327200C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83272010: 4AFBAEC1  bl 0x8222ced0
	ctx.lr = 0x83272014;
	sub_8222CED0(ctx, base);
	// 83272014: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83272018: 3869F3C0  addi r3, r9, -0xc40
	ctx.r[3].s64 = ctx.r[9].s64 + -3136;
	// 8327201C: 4BA37F05  bl 0x82ca9f20
	ctx.lr = 0x83272020;
	sub_82CA9F20(ctx, base);
	// 83272020: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83272024: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83272028: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327202C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83272030(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83272030 size=64
    let mut pc: u32 = 0x83272030;
    'dispatch: loop {
        match pc {
            0x83272030 => {
    //   block [0x83272030..0x83272070)
	// 83272030: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83272034: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83272038: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327203C: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 83272040: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83272044: 388B9F80  addi r4, r11, -0x6080
	ctx.r[4].s64 = ctx.r[11].s64 + -24704;
	// 83272048: 386ACFE8  addi r3, r10, -0x3018
	ctx.r[3].s64 = ctx.r[10].s64 + -12312;
	// 8327204C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83272050: 4AFBAE81  bl 0x8222ced0
	ctx.lr = 0x83272054;
	sub_8222CED0(ctx, base);
	// 83272054: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83272058: 3869F3D0  addi r3, r9, -0xc30
	ctx.r[3].s64 = ctx.r[9].s64 + -3120;
	// 8327205C: 4BA37EC5  bl 0x82ca9f20
	ctx.lr = 0x83272060;
	sub_82CA9F20(ctx, base);
	// 83272060: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83272064: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83272068: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327206C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83272070(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83272070 size=56
    let mut pc: u32 = 0x83272070;
    'dispatch: loop {
        match pc {
            0x83272070 => {
    //   block [0x83272070..0x832720A8)
	// 83272070: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83272074: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83272078: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327207C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83272080: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83272084: 386B42C8  addi r3, r11, 0x42c8
	ctx.r[3].s64 = ctx.r[11].s64 + 17096;
	// 83272088: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8327208C: 4AF81CCD  bl 0x821f3d58
	ctx.lr = 0x83272090;
	sub_821F3D58(ctx, base);
	// 83272090: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83272094: 906ACFEC  stw r3, -0x3014(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-12308 as u32), ctx.r[3].u32 ) };
	// 83272098: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327209C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832720A0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832720A4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832720A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832720A8 size=56
    let mut pc: u32 = 0x832720A8;
    'dispatch: loop {
        match pc {
            0x832720A8 => {
    //   block [0x832720A8..0x832720E0)
	// 832720A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832720AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832720B0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832720B4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 832720B8: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 832720BC: 386B42DC  addi r3, r11, 0x42dc
	ctx.r[3].s64 = ctx.r[11].s64 + 17116;
	// 832720C0: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 832720C4: 4AF81C95  bl 0x821f3d58
	ctx.lr = 0x832720C8;
	sub_821F3D58(ctx, base);
	// 832720C8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832720CC: 906ACFF0  stw r3, -0x3010(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-12304 as u32), ctx.r[3].u32 ) };
	// 832720D0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832720D4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832720D8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832720DC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832720E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832720E0 size=56
    let mut pc: u32 = 0x832720E0;
    'dispatch: loop {
        match pc {
            0x832720E0 => {
    //   block [0x832720E0..0x83272118)
	// 832720E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832720E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832720E8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832720EC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 832720F0: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 832720F4: 386B42E8  addi r3, r11, 0x42e8
	ctx.r[3].s64 = ctx.r[11].s64 + 17128;
	// 832720F8: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 832720FC: 4AF81C5D  bl 0x821f3d58
	ctx.lr = 0x83272100;
	sub_821F3D58(ctx, base);
	// 83272100: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83272104: 906ACFF4  stw r3, -0x300c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-12300 as u32), ctx.r[3].u32 ) };
	// 83272108: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327210C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83272110: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83272114: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83272118(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83272118 size=56
    let mut pc: u32 = 0x83272118;
    'dispatch: loop {
        match pc {
            0x83272118 => {
    //   block [0x83272118..0x83272150)
	// 83272118: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327211C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83272120: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83272124: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83272128: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8327212C: 386B4300  addi r3, r11, 0x4300
	ctx.r[3].s64 = ctx.r[11].s64 + 17152;
	// 83272130: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83272134: 4AF81C25  bl 0x821f3d58
	ctx.lr = 0x83272138;
	sub_821F3D58(ctx, base);
	// 83272138: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327213C: 906ACFF8  stw r3, -0x3008(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-12296 as u32), ctx.r[3].u32 ) };
	// 83272140: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83272144: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83272148: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327214C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83272150(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83272150 size=56
    let mut pc: u32 = 0x83272150;
    'dispatch: loop {
        match pc {
            0x83272150 => {
    //   block [0x83272150..0x83272188)
	// 83272150: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83272154: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83272158: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327215C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83272160: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83272164: 386B4310  addi r3, r11, 0x4310
	ctx.r[3].s64 = ctx.r[11].s64 + 17168;
	// 83272168: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8327216C: 4AF81BED  bl 0x821f3d58
	ctx.lr = 0x83272170;
	sub_821F3D58(ctx, base);
	// 83272170: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83272174: 906ACFFC  stw r3, -0x3004(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-12292 as u32), ctx.r[3].u32 ) };
	// 83272178: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327217C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83272180: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83272184: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83272188(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83272188 size=56
    let mut pc: u32 = 0x83272188;
    'dispatch: loop {
        match pc {
            0x83272188 => {
    //   block [0x83272188..0x832721C0)
	// 83272188: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327218C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83272190: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83272194: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83272198: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8327219C: 386B4328  addi r3, r11, 0x4328
	ctx.r[3].s64 = ctx.r[11].s64 + 17192;
	// 832721A0: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 832721A4: 4AF81BB5  bl 0x821f3d58
	ctx.lr = 0x832721A8;
	sub_821F3D58(ctx, base);
	// 832721A8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832721AC: 906AD000  stw r3, -0x3000(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-12288 as u32), ctx.r[3].u32 ) };
	// 832721B0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832721B4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832721B8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832721BC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832721C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832721C0 size=56
    let mut pc: u32 = 0x832721C0;
    'dispatch: loop {
        match pc {
            0x832721C0 => {
    //   block [0x832721C0..0x832721F8)
	// 832721C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832721C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832721C8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832721CC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 832721D0: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 832721D4: 386B4338  addi r3, r11, 0x4338
	ctx.r[3].s64 = ctx.r[11].s64 + 17208;
	// 832721D8: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 832721DC: 4AF81B7D  bl 0x821f3d58
	ctx.lr = 0x832721E0;
	sub_821F3D58(ctx, base);
	// 832721E0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832721E4: 906AD004  stw r3, -0x2ffc(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-12284 as u32), ctx.r[3].u32 ) };
	// 832721E8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832721EC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832721F0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832721F4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832721F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832721F8 size=56
    let mut pc: u32 = 0x832721F8;
    'dispatch: loop {
        match pc {
            0x832721F8 => {
    //   block [0x832721F8..0x83272230)
	// 832721F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832721FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83272200: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83272204: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83272208: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8327220C: 386B4350  addi r3, r11, 0x4350
	ctx.r[3].s64 = ctx.r[11].s64 + 17232;
	// 83272210: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83272214: 4AF81B45  bl 0x821f3d58
	ctx.lr = 0x83272218;
	sub_821F3D58(ctx, base);
	// 83272218: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327221C: 906AD008  stw r3, -0x2ff8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-12280 as u32), ctx.r[3].u32 ) };
	// 83272220: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83272224: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83272228: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327222C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83272230(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83272230 size=56
    let mut pc: u32 = 0x83272230;
    'dispatch: loop {
        match pc {
            0x83272230 => {
    //   block [0x83272230..0x83272268)
	// 83272230: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83272234: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83272238: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327223C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83272240: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83272244: 386B4368  addi r3, r11, 0x4368
	ctx.r[3].s64 = ctx.r[11].s64 + 17256;
	// 83272248: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8327224C: 4AF81B0D  bl 0x821f3d58
	ctx.lr = 0x83272250;
	sub_821F3D58(ctx, base);
	// 83272250: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83272254: 906AD00C  stw r3, -0x2ff4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-12276 as u32), ctx.r[3].u32 ) };
	// 83272258: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327225C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83272260: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83272264: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83272268(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83272268 size=56
    let mut pc: u32 = 0x83272268;
    'dispatch: loop {
        match pc {
            0x83272268 => {
    //   block [0x83272268..0x832722A0)
	// 83272268: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327226C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83272270: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83272274: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83272278: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8327227C: 386B4380  addi r3, r11, 0x4380
	ctx.r[3].s64 = ctx.r[11].s64 + 17280;
	// 83272280: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83272284: 4AF81AD5  bl 0x821f3d58
	ctx.lr = 0x83272288;
	sub_821F3D58(ctx, base);
	// 83272288: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327228C: 906AD010  stw r3, -0x2ff0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-12272 as u32), ctx.r[3].u32 ) };
	// 83272290: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83272294: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83272298: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327229C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832722A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832722A0 size=56
    let mut pc: u32 = 0x832722A0;
    'dispatch: loop {
        match pc {
            0x832722A0 => {
    //   block [0x832722A0..0x832722D8)
	// 832722A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832722A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832722A8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832722AC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 832722B0: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 832722B4: 386B4394  addi r3, r11, 0x4394
	ctx.r[3].s64 = ctx.r[11].s64 + 17300;
	// 832722B8: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 832722BC: 4AF81A9D  bl 0x821f3d58
	ctx.lr = 0x832722C0;
	sub_821F3D58(ctx, base);
	// 832722C0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832722C4: 906AD014  stw r3, -0x2fec(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-12268 as u32), ctx.r[3].u32 ) };
	// 832722C8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832722CC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832722D0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832722D4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


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


