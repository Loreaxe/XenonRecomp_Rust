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


pub fn sub_83260A88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83260A88 size=56
    let mut pc: u32 = 0x83260A88;
    'dispatch: loop {
        match pc {
            0x83260A88 => {
    //   block [0x83260A88..0x83260AC0)
	// 83260A88: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83260A8C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83260A90: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83260A94: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 83260A98: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83260A9C: 386BF7A4  addi r3, r11, -0x85c
	ctx.r[3].s64 = ctx.r[11].s64 + -2140;
	// 83260AA0: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83260AA4: 4AF932B5  bl 0x821f3d58
	ctx.lr = 0x83260AA8;
	sub_821F3D58(ctx, base);
	// 83260AA8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83260AAC: 906AB11C  stw r3, -0x4ee4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-20196 as u32), ctx.r[3].u32 ) };
	// 83260AB0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83260AB4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83260AB8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83260ABC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83260AC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83260AC0 size=56
    let mut pc: u32 = 0x83260AC0;
    'dispatch: loop {
        match pc {
            0x83260AC0 => {
    //   block [0x83260AC0..0x83260AF8)
	// 83260AC0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83260AC4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83260AC8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83260ACC: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 83260AD0: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83260AD4: 386BF7B4  addi r3, r11, -0x84c
	ctx.r[3].s64 = ctx.r[11].s64 + -2124;
	// 83260AD8: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83260ADC: 4AF9327D  bl 0x821f3d58
	ctx.lr = 0x83260AE0;
	sub_821F3D58(ctx, base);
	// 83260AE0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83260AE4: 906AB120  stw r3, -0x4ee0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-20192 as u32), ctx.r[3].u32 ) };
	// 83260AE8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83260AEC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83260AF0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83260AF4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83260AF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83260AF8 size=56
    let mut pc: u32 = 0x83260AF8;
    'dispatch: loop {
        match pc {
            0x83260AF8 => {
    //   block [0x83260AF8..0x83260B30)
	// 83260AF8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83260AFC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83260B00: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83260B04: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 83260B08: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83260B0C: 386BF7C8  addi r3, r11, -0x838
	ctx.r[3].s64 = ctx.r[11].s64 + -2104;
	// 83260B10: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83260B14: 4AF93245  bl 0x821f3d58
	ctx.lr = 0x83260B18;
	sub_821F3D58(ctx, base);
	// 83260B18: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83260B1C: 906AB124  stw r3, -0x4edc(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-20188 as u32), ctx.r[3].u32 ) };
	// 83260B20: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83260B24: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83260B28: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83260B2C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83260B30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83260B30 size=56
    let mut pc: u32 = 0x83260B30;
    'dispatch: loop {
        match pc {
            0x83260B30 => {
    //   block [0x83260B30..0x83260B68)
	// 83260B30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83260B34: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83260B38: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83260B3C: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 83260B40: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83260B44: 386BF7D8  addi r3, r11, -0x828
	ctx.r[3].s64 = ctx.r[11].s64 + -2088;
	// 83260B48: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83260B4C: 4AF9320D  bl 0x821f3d58
	ctx.lr = 0x83260B50;
	sub_821F3D58(ctx, base);
	// 83260B50: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83260B54: 906AB128  stw r3, -0x4ed8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-20184 as u32), ctx.r[3].u32 ) };
	// 83260B58: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83260B5C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83260B60: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83260B64: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83260B68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83260B68 size=56
    let mut pc: u32 = 0x83260B68;
    'dispatch: loop {
        match pc {
            0x83260B68 => {
    //   block [0x83260B68..0x83260BA0)
	// 83260B68: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83260B6C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83260B70: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83260B74: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 83260B78: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83260B7C: 386BF7E4  addi r3, r11, -0x81c
	ctx.r[3].s64 = ctx.r[11].s64 + -2076;
	// 83260B80: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83260B84: 4AF931D5  bl 0x821f3d58
	ctx.lr = 0x83260B88;
	sub_821F3D58(ctx, base);
	// 83260B88: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83260B8C: 906AB12C  stw r3, -0x4ed4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-20180 as u32), ctx.r[3].u32 ) };
	// 83260B90: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83260B94: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83260B98: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83260B9C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83260BA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83260BA0 size=56
    let mut pc: u32 = 0x83260BA0;
    'dispatch: loop {
        match pc {
            0x83260BA0 => {
    //   block [0x83260BA0..0x83260BD8)
	// 83260BA0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83260BA4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83260BA8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83260BAC: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 83260BB0: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83260BB4: 386B260C  addi r3, r11, 0x260c
	ctx.r[3].s64 = ctx.r[11].s64 + 9740;
	// 83260BB8: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83260BBC: 4AF9319D  bl 0x821f3d58
	ctx.lr = 0x83260BC0;
	sub_821F3D58(ctx, base);
	// 83260BC0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83260BC4: 906AB130  stw r3, -0x4ed0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-20176 as u32), ctx.r[3].u32 ) };
	// 83260BC8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83260BCC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83260BD0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83260BD4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83260BD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83260BD8 size=56
    let mut pc: u32 = 0x83260BD8;
    'dispatch: loop {
        match pc {
            0x83260BD8 => {
    //   block [0x83260BD8..0x83260C10)
	// 83260BD8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83260BDC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83260BE0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83260BE4: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 83260BE8: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83260BEC: 386BF7F0  addi r3, r11, -0x810
	ctx.r[3].s64 = ctx.r[11].s64 + -2064;
	// 83260BF0: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83260BF4: 4AF93165  bl 0x821f3d58
	ctx.lr = 0x83260BF8;
	sub_821F3D58(ctx, base);
	// 83260BF8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83260BFC: 906AB134  stw r3, -0x4ecc(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-20172 as u32), ctx.r[3].u32 ) };
	// 83260C00: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83260C04: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83260C08: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83260C0C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83260C10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83260C10 size=56
    let mut pc: u32 = 0x83260C10;
    'dispatch: loop {
        match pc {
            0x83260C10 => {
    //   block [0x83260C10..0x83260C48)
	// 83260C10: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83260C14: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83260C18: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83260C1C: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 83260C20: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83260C24: 386B26C0  addi r3, r11, 0x26c0
	ctx.r[3].s64 = ctx.r[11].s64 + 9920;
	// 83260C28: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83260C2C: 4AF9312D  bl 0x821f3d58
	ctx.lr = 0x83260C30;
	sub_821F3D58(ctx, base);
	// 83260C30: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83260C34: 906AB138  stw r3, -0x4ec8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-20168 as u32), ctx.r[3].u32 ) };
	// 83260C38: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83260C3C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83260C40: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83260C44: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83260C48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83260C48 size=56
    let mut pc: u32 = 0x83260C48;
    'dispatch: loop {
        match pc {
            0x83260C48 => {
    //   block [0x83260C48..0x83260C80)
	// 83260C48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83260C4C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83260C50: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83260C54: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 83260C58: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83260C5C: 386BF7FC  addi r3, r11, -0x804
	ctx.r[3].s64 = ctx.r[11].s64 + -2052;
	// 83260C60: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83260C64: 4AF930F5  bl 0x821f3d58
	ctx.lr = 0x83260C68;
	sub_821F3D58(ctx, base);
	// 83260C68: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83260C6C: 906AB13C  stw r3, -0x4ec4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-20164 as u32), ctx.r[3].u32 ) };
	// 83260C70: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83260C74: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83260C78: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83260C7C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83260C80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83260C80 size=56
    let mut pc: u32 = 0x83260C80;
    'dispatch: loop {
        match pc {
            0x83260C80 => {
    //   block [0x83260C80..0x83260CB8)
	// 83260C80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83260C84: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83260C88: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83260C8C: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 83260C90: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83260C94: 386B26D8  addi r3, r11, 0x26d8
	ctx.r[3].s64 = ctx.r[11].s64 + 9944;
	// 83260C98: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83260C9C: 4AF930BD  bl 0x821f3d58
	ctx.lr = 0x83260CA0;
	sub_821F3D58(ctx, base);
	// 83260CA0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83260CA4: 906AB140  stw r3, -0x4ec0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-20160 as u32), ctx.r[3].u32 ) };
	// 83260CA8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83260CAC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83260CB0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83260CB4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83260CB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83260CB8 size=56
    let mut pc: u32 = 0x83260CB8;
    'dispatch: loop {
        match pc {
            0x83260CB8 => {
    //   block [0x83260CB8..0x83260CF0)
	// 83260CB8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83260CBC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83260CC0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83260CC4: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 83260CC8: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83260CCC: 386BF808  addi r3, r11, -0x7f8
	ctx.r[3].s64 = ctx.r[11].s64 + -2040;
	// 83260CD0: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83260CD4: 4AF93085  bl 0x821f3d58
	ctx.lr = 0x83260CD8;
	sub_821F3D58(ctx, base);
	// 83260CD8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83260CDC: 906AB144  stw r3, -0x4ebc(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-20156 as u32), ctx.r[3].u32 ) };
	// 83260CE0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83260CE4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83260CE8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83260CEC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83260CF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83260CF0 size=56
    let mut pc: u32 = 0x83260CF0;
    'dispatch: loop {
        match pc {
            0x83260CF0 => {
    //   block [0x83260CF0..0x83260D28)
	// 83260CF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83260CF4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83260CF8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83260CFC: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 83260D00: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83260D04: 386BF818  addi r3, r11, -0x7e8
	ctx.r[3].s64 = ctx.r[11].s64 + -2024;
	// 83260D08: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83260D0C: 4AF9304D  bl 0x821f3d58
	ctx.lr = 0x83260D10;
	sub_821F3D58(ctx, base);
	// 83260D10: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83260D14: 906AB148  stw r3, -0x4eb8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-20152 as u32), ctx.r[3].u32 ) };
	// 83260D18: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83260D1C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83260D20: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83260D24: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83260D28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83260D28 size=56
    let mut pc: u32 = 0x83260D28;
    'dispatch: loop {
        match pc {
            0x83260D28 => {
    //   block [0x83260D28..0x83260D60)
	// 83260D28: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83260D2C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83260D30: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83260D34: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 83260D38: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83260D3C: 386BF820  addi r3, r11, -0x7e0
	ctx.r[3].s64 = ctx.r[11].s64 + -2016;
	// 83260D40: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83260D44: 4AF93015  bl 0x821f3d58
	ctx.lr = 0x83260D48;
	sub_821F3D58(ctx, base);
	// 83260D48: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83260D4C: 906AB14C  stw r3, -0x4eb4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-20148 as u32), ctx.r[3].u32 ) };
	// 83260D50: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83260D54: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83260D58: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83260D5C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83260D60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83260D60 size=64
    let mut pc: u32 = 0x83260D60;
    'dispatch: loop {
        match pc {
            0x83260D60 => {
    //   block [0x83260D60..0x83260DA0)
	// 83260D60: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83260D64: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83260D68: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83260D6C: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 83260D70: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83260D74: 388BF828  addi r4, r11, -0x7d8
	ctx.r[4].s64 = ctx.r[11].s64 + -2008;
	// 83260D78: 386AB150  addi r3, r10, -0x4eb0
	ctx.r[3].s64 = ctx.r[10].s64 + -20144;
	// 83260D7C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83260D80: 4AFCC151  bl 0x8222ced0
	ctx.lr = 0x83260D84;
	sub_8222CED0(ctx, base);
	// 83260D84: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83260D88: 3869C8E0  addi r3, r9, -0x3720
	ctx.r[3].s64 = ctx.r[9].s64 + -14112;
	// 83260D8C: 4BA49195  bl 0x82ca9f20
	ctx.lr = 0x83260D90;
	sub_82CA9F20(ctx, base);
	// 83260D90: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83260D94: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83260D98: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83260D9C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83260DA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83260DA0 size=64
    let mut pc: u32 = 0x83260DA0;
    'dispatch: loop {
        match pc {
            0x83260DA0 => {
    //   block [0x83260DA0..0x83260DE0)
	// 83260DA0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83260DA4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83260DA8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83260DAC: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 83260DB0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83260DB4: 388BF84C  addi r4, r11, -0x7b4
	ctx.r[4].s64 = ctx.r[11].s64 + -1972;
	// 83260DB8: 386AB154  addi r3, r10, -0x4eac
	ctx.r[3].s64 = ctx.r[10].s64 + -20140;
	// 83260DBC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83260DC0: 4AFCC111  bl 0x8222ced0
	ctx.lr = 0x83260DC4;
	sub_8222CED0(ctx, base);
	// 83260DC4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83260DC8: 3869C8F0  addi r3, r9, -0x3710
	ctx.r[3].s64 = ctx.r[9].s64 + -14096;
	// 83260DCC: 4BA49155  bl 0x82ca9f20
	ctx.lr = 0x83260DD0;
	sub_82CA9F20(ctx, base);
	// 83260DD0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83260DD4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83260DD8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83260DDC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83260DE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83260DE0 size=64
    let mut pc: u32 = 0x83260DE0;
    'dispatch: loop {
        match pc {
            0x83260DE0 => {
    //   block [0x83260DE0..0x83260E20)
	// 83260DE0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83260DE4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83260DE8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83260DEC: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 83260DF0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83260DF4: 388BF870  addi r4, r11, -0x790
	ctx.r[4].s64 = ctx.r[11].s64 + -1936;
	// 83260DF8: 386AB158  addi r3, r10, -0x4ea8
	ctx.r[3].s64 = ctx.r[10].s64 + -20136;
	// 83260DFC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83260E00: 4AFCC0D1  bl 0x8222ced0
	ctx.lr = 0x83260E04;
	sub_8222CED0(ctx, base);
	// 83260E04: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83260E08: 3869C900  addi r3, r9, -0x3700
	ctx.r[3].s64 = ctx.r[9].s64 + -14080;
	// 83260E0C: 4BA49115  bl 0x82ca9f20
	ctx.lr = 0x83260E10;
	sub_82CA9F20(ctx, base);
	// 83260E10: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83260E14: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83260E18: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83260E1C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83260E20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83260E20 size=64
    let mut pc: u32 = 0x83260E20;
    'dispatch: loop {
        match pc {
            0x83260E20 => {
    //   block [0x83260E20..0x83260E60)
	// 83260E20: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83260E24: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83260E28: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83260E2C: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 83260E30: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83260E34: 388BA894  addi r4, r11, -0x576c
	ctx.r[4].s64 = ctx.r[11].s64 + -22380;
	// 83260E38: 386AB15C  addi r3, r10, -0x4ea4
	ctx.r[3].s64 = ctx.r[10].s64 + -20132;
	// 83260E3C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83260E40: 4AFCC091  bl 0x8222ced0
	ctx.lr = 0x83260E44;
	sub_8222CED0(ctx, base);
	// 83260E44: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83260E48: 3869C910  addi r3, r9, -0x36f0
	ctx.r[3].s64 = ctx.r[9].s64 + -14064;
	// 83260E4C: 4BA490D5  bl 0x82ca9f20
	ctx.lr = 0x83260E50;
	sub_82CA9F20(ctx, base);
	// 83260E50: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83260E54: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83260E58: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83260E5C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83260E60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83260E60 size=64
    let mut pc: u32 = 0x83260E60;
    'dispatch: loop {
        match pc {
            0x83260E60 => {
    //   block [0x83260E60..0x83260EA0)
	// 83260E60: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83260E64: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83260E68: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83260E6C: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 83260E70: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83260E74: 388BA8B0  addi r4, r11, -0x5750
	ctx.r[4].s64 = ctx.r[11].s64 + -22352;
	// 83260E78: 386AB160  addi r3, r10, -0x4ea0
	ctx.r[3].s64 = ctx.r[10].s64 + -20128;
	// 83260E7C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83260E80: 4AFCC051  bl 0x8222ced0
	ctx.lr = 0x83260E84;
	sub_8222CED0(ctx, base);
	// 83260E84: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83260E88: 3869C920  addi r3, r9, -0x36e0
	ctx.r[3].s64 = ctx.r[9].s64 + -14048;
	// 83260E8C: 4BA49095  bl 0x82ca9f20
	ctx.lr = 0x83260E90;
	sub_82CA9F20(ctx, base);
	// 83260E90: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83260E94: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83260E98: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83260E9C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83260EA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83260EA0 size=64
    let mut pc: u32 = 0x83260EA0;
    'dispatch: loop {
        match pc {
            0x83260EA0 => {
    //   block [0x83260EA0..0x83260EE0)
	// 83260EA0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83260EA4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83260EA8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83260EAC: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 83260EB0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83260EB4: 388BF890  addi r4, r11, -0x770
	ctx.r[4].s64 = ctx.r[11].s64 + -1904;
	// 83260EB8: 386AB164  addi r3, r10, -0x4e9c
	ctx.r[3].s64 = ctx.r[10].s64 + -20124;
	// 83260EBC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83260EC0: 4AFCC011  bl 0x8222ced0
	ctx.lr = 0x83260EC4;
	sub_8222CED0(ctx, base);
	// 83260EC4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83260EC8: 3869C930  addi r3, r9, -0x36d0
	ctx.r[3].s64 = ctx.r[9].s64 + -14032;
	// 83260ECC: 4BA49055  bl 0x82ca9f20
	ctx.lr = 0x83260ED0;
	sub_82CA9F20(ctx, base);
	// 83260ED0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83260ED4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83260ED8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83260EDC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83260EE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83260EE0 size=64
    let mut pc: u32 = 0x83260EE0;
    'dispatch: loop {
        match pc {
            0x83260EE0 => {
    //   block [0x83260EE0..0x83260F20)
	// 83260EE0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83260EE4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83260EE8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83260EEC: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 83260EF0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83260EF4: 388BF8B0  addi r4, r11, -0x750
	ctx.r[4].s64 = ctx.r[11].s64 + -1872;
	// 83260EF8: 386AB168  addi r3, r10, -0x4e98
	ctx.r[3].s64 = ctx.r[10].s64 + -20120;
	// 83260EFC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83260F00: 4AFCBFD1  bl 0x8222ced0
	ctx.lr = 0x83260F04;
	sub_8222CED0(ctx, base);
	// 83260F04: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83260F08: 3869C940  addi r3, r9, -0x36c0
	ctx.r[3].s64 = ctx.r[9].s64 + -14016;
	// 83260F0C: 4BA49015  bl 0x82ca9f20
	ctx.lr = 0x83260F10;
	sub_82CA9F20(ctx, base);
	// 83260F10: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83260F14: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83260F18: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83260F1C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83260F20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83260F20 size=64
    let mut pc: u32 = 0x83260F20;
    'dispatch: loop {
        match pc {
            0x83260F20 => {
    //   block [0x83260F20..0x83260F60)
	// 83260F20: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83260F24: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83260F28: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83260F2C: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 83260F30: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83260F34: 388BE214  addi r4, r11, -0x1dec
	ctx.r[4].s64 = ctx.r[11].s64 + -7660;
	// 83260F38: 386AB16C  addi r3, r10, -0x4e94
	ctx.r[3].s64 = ctx.r[10].s64 + -20116;
	// 83260F3C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83260F40: 4AFCBF91  bl 0x8222ced0
	ctx.lr = 0x83260F44;
	sub_8222CED0(ctx, base);
	// 83260F44: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83260F48: 3869C950  addi r3, r9, -0x36b0
	ctx.r[3].s64 = ctx.r[9].s64 + -14000;
	// 83260F4C: 4BA48FD5  bl 0x82ca9f20
	ctx.lr = 0x83260F50;
	sub_82CA9F20(ctx, base);
	// 83260F50: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83260F54: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83260F58: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83260F5C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83260F60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83260F60 size=64
    let mut pc: u32 = 0x83260F60;
    'dispatch: loop {
        match pc {
            0x83260F60 => {
    //   block [0x83260F60..0x83260FA0)
	// 83260F60: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83260F64: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83260F68: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83260F6C: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 83260F70: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83260F74: 388BF8CC  addi r4, r11, -0x734
	ctx.r[4].s64 = ctx.r[11].s64 + -1844;
	// 83260F78: 386AB170  addi r3, r10, -0x4e90
	ctx.r[3].s64 = ctx.r[10].s64 + -20112;
	// 83260F7C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83260F80: 4AFCBF51  bl 0x8222ced0
	ctx.lr = 0x83260F84;
	sub_8222CED0(ctx, base);
	// 83260F84: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83260F88: 3869C960  addi r3, r9, -0x36a0
	ctx.r[3].s64 = ctx.r[9].s64 + -13984;
	// 83260F8C: 4BA48F95  bl 0x82ca9f20
	ctx.lr = 0x83260F90;
	sub_82CA9F20(ctx, base);
	// 83260F90: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83260F94: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83260F98: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83260F9C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83260FA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83260FA0 size=64
    let mut pc: u32 = 0x83260FA0;
    'dispatch: loop {
        match pc {
            0x83260FA0 => {
    //   block [0x83260FA0..0x83260FE0)
	// 83260FA0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83260FA4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83260FA8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83260FAC: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 83260FB0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83260FB4: 388B28D4  addi r4, r11, 0x28d4
	ctx.r[4].s64 = ctx.r[11].s64 + 10452;
	// 83260FB8: 386AB174  addi r3, r10, -0x4e8c
	ctx.r[3].s64 = ctx.r[10].s64 + -20108;
	// 83260FBC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83260FC0: 4AFCBF11  bl 0x8222ced0
	ctx.lr = 0x83260FC4;
	sub_8222CED0(ctx, base);
	// 83260FC4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83260FC8: 3869C970  addi r3, r9, -0x3690
	ctx.r[3].s64 = ctx.r[9].s64 + -13968;
	// 83260FCC: 4BA48F55  bl 0x82ca9f20
	ctx.lr = 0x83260FD0;
	sub_82CA9F20(ctx, base);
	// 83260FD0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83260FD4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83260FD8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83260FDC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83260FE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83260FE0 size=64
    let mut pc: u32 = 0x83260FE0;
    'dispatch: loop {
        match pc {
            0x83260FE0 => {
    //   block [0x83260FE0..0x83261020)
	// 83260FE0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83260FE4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83260FE8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83260FEC: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 83260FF0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83260FF4: 388B28DC  addi r4, r11, 0x28dc
	ctx.r[4].s64 = ctx.r[11].s64 + 10460;
	// 83260FF8: 386AB178  addi r3, r10, -0x4e88
	ctx.r[3].s64 = ctx.r[10].s64 + -20104;
	// 83260FFC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83261000: 4AFCBED1  bl 0x8222ced0
	ctx.lr = 0x83261004;
	sub_8222CED0(ctx, base);
	// 83261004: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83261008: 3869C980  addi r3, r9, -0x3680
	ctx.r[3].s64 = ctx.r[9].s64 + -13952;
	// 8326100C: 4BA48F15  bl 0x82ca9f20
	ctx.lr = 0x83261010;
	sub_82CA9F20(ctx, base);
	// 83261010: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83261014: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83261018: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326101C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83261020(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83261020 size=64
    let mut pc: u32 = 0x83261020;
    'dispatch: loop {
        match pc {
            0x83261020 => {
    //   block [0x83261020..0x83261060)
	// 83261020: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83261024: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83261028: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326102C: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 83261030: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83261034: 388B28E4  addi r4, r11, 0x28e4
	ctx.r[4].s64 = ctx.r[11].s64 + 10468;
	// 83261038: 386AB17C  addi r3, r10, -0x4e84
	ctx.r[3].s64 = ctx.r[10].s64 + -20100;
	// 8326103C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83261040: 4AFCBE91  bl 0x8222ced0
	ctx.lr = 0x83261044;
	sub_8222CED0(ctx, base);
	// 83261044: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83261048: 3869C990  addi r3, r9, -0x3670
	ctx.r[3].s64 = ctx.r[9].s64 + -13936;
	// 8326104C: 4BA48ED5  bl 0x82ca9f20
	ctx.lr = 0x83261050;
	sub_82CA9F20(ctx, base);
	// 83261050: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83261054: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83261058: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326105C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83261060(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83261060 size=64
    let mut pc: u32 = 0x83261060;
    'dispatch: loop {
        match pc {
            0x83261060 => {
    //   block [0x83261060..0x832610A0)
	// 83261060: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83261064: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83261068: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326106C: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 83261070: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83261074: 388B28F0  addi r4, r11, 0x28f0
	ctx.r[4].s64 = ctx.r[11].s64 + 10480;
	// 83261078: 386AB180  addi r3, r10, -0x4e80
	ctx.r[3].s64 = ctx.r[10].s64 + -20096;
	// 8326107C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83261080: 4AFCBE51  bl 0x8222ced0
	ctx.lr = 0x83261084;
	sub_8222CED0(ctx, base);
	// 83261084: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83261088: 3869C9A0  addi r3, r9, -0x3660
	ctx.r[3].s64 = ctx.r[9].s64 + -13920;
	// 8326108C: 4BA48E95  bl 0x82ca9f20
	ctx.lr = 0x83261090;
	sub_82CA9F20(ctx, base);
	// 83261090: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83261094: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83261098: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326109C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832610A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832610A0 size=64
    let mut pc: u32 = 0x832610A0;
    'dispatch: loop {
        match pc {
            0x832610A0 => {
    //   block [0x832610A0..0x832610E0)
	// 832610A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832610A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832610A8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832610AC: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 832610B0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832610B4: 388B28F8  addi r4, r11, 0x28f8
	ctx.r[4].s64 = ctx.r[11].s64 + 10488;
	// 832610B8: 386AB184  addi r3, r10, -0x4e7c
	ctx.r[3].s64 = ctx.r[10].s64 + -20092;
	// 832610BC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832610C0: 4AFCBE11  bl 0x8222ced0
	ctx.lr = 0x832610C4;
	sub_8222CED0(ctx, base);
	// 832610C4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 832610C8: 3869C9B0  addi r3, r9, -0x3650
	ctx.r[3].s64 = ctx.r[9].s64 + -13904;
	// 832610CC: 4BA48E55  bl 0x82ca9f20
	ctx.lr = 0x832610D0;
	sub_82CA9F20(ctx, base);
	// 832610D0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832610D4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832610D8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832610DC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832610E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832610E0 size=64
    let mut pc: u32 = 0x832610E0;
    'dispatch: loop {
        match pc {
            0x832610E0 => {
    //   block [0x832610E0..0x83261120)
	// 832610E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832610E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832610E8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832610EC: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 832610F0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832610F4: 388B2904  addi r4, r11, 0x2904
	ctx.r[4].s64 = ctx.r[11].s64 + 10500;
	// 832610F8: 386AB188  addi r3, r10, -0x4e78
	ctx.r[3].s64 = ctx.r[10].s64 + -20088;
	// 832610FC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83261100: 4AFCBDD1  bl 0x8222ced0
	ctx.lr = 0x83261104;
	sub_8222CED0(ctx, base);
	// 83261104: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83261108: 3869C9C0  addi r3, r9, -0x3640
	ctx.r[3].s64 = ctx.r[9].s64 + -13888;
	// 8326110C: 4BA48E15  bl 0x82ca9f20
	ctx.lr = 0x83261110;
	sub_82CA9F20(ctx, base);
	// 83261110: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83261114: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83261118: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326111C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83261120(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83261120 size=64
    let mut pc: u32 = 0x83261120;
    'dispatch: loop {
        match pc {
            0x83261120 => {
    //   block [0x83261120..0x83261160)
	// 83261120: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83261124: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83261128: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326112C: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 83261130: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83261134: 388B2914  addi r4, r11, 0x2914
	ctx.r[4].s64 = ctx.r[11].s64 + 10516;
	// 83261138: 386AB18C  addi r3, r10, -0x4e74
	ctx.r[3].s64 = ctx.r[10].s64 + -20084;
	// 8326113C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83261140: 4AFCBD91  bl 0x8222ced0
	ctx.lr = 0x83261144;
	sub_8222CED0(ctx, base);
	// 83261144: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83261148: 3869C9D0  addi r3, r9, -0x3630
	ctx.r[3].s64 = ctx.r[9].s64 + -13872;
	// 8326114C: 4BA48DD5  bl 0x82ca9f20
	ctx.lr = 0x83261150;
	sub_82CA9F20(ctx, base);
	// 83261150: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83261154: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83261158: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326115C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83261160(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83261160 size=64
    let mut pc: u32 = 0x83261160;
    'dispatch: loop {
        match pc {
            0x83261160 => {
    //   block [0x83261160..0x832611A0)
	// 83261160: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83261164: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83261168: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326116C: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 83261170: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83261174: 388B291C  addi r4, r11, 0x291c
	ctx.r[4].s64 = ctx.r[11].s64 + 10524;
	// 83261178: 386AB190  addi r3, r10, -0x4e70
	ctx.r[3].s64 = ctx.r[10].s64 + -20080;
	// 8326117C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83261180: 4AFCBD51  bl 0x8222ced0
	ctx.lr = 0x83261184;
	sub_8222CED0(ctx, base);
	// 83261184: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83261188: 3869C9E0  addi r3, r9, -0x3620
	ctx.r[3].s64 = ctx.r[9].s64 + -13856;
	// 8326118C: 4BA48D95  bl 0x82ca9f20
	ctx.lr = 0x83261190;
	sub_82CA9F20(ctx, base);
	// 83261190: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83261194: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83261198: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326119C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832611A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832611A0 size=64
    let mut pc: u32 = 0x832611A0;
    'dispatch: loop {
        match pc {
            0x832611A0 => {
    //   block [0x832611A0..0x832611E0)
	// 832611A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832611A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832611A8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832611AC: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 832611B0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832611B4: 388B292C  addi r4, r11, 0x292c
	ctx.r[4].s64 = ctx.r[11].s64 + 10540;
	// 832611B8: 386AB194  addi r3, r10, -0x4e6c
	ctx.r[3].s64 = ctx.r[10].s64 + -20076;
	// 832611BC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832611C0: 4AFCBD11  bl 0x8222ced0
	ctx.lr = 0x832611C4;
	sub_8222CED0(ctx, base);
	// 832611C4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 832611C8: 3869C9F0  addi r3, r9, -0x3610
	ctx.r[3].s64 = ctx.r[9].s64 + -13840;
	// 832611CC: 4BA48D55  bl 0x82ca9f20
	ctx.lr = 0x832611D0;
	sub_82CA9F20(ctx, base);
	// 832611D0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832611D4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832611D8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832611DC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832611E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832611E0 size=64
    let mut pc: u32 = 0x832611E0;
    'dispatch: loop {
        match pc {
            0x832611E0 => {
    //   block [0x832611E0..0x83261220)
	// 832611E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832611E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832611E8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832611EC: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 832611F0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832611F4: 388B2934  addi r4, r11, 0x2934
	ctx.r[4].s64 = ctx.r[11].s64 + 10548;
	// 832611F8: 386AB198  addi r3, r10, -0x4e68
	ctx.r[3].s64 = ctx.r[10].s64 + -20072;
	// 832611FC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83261200: 4AFCBCD1  bl 0x8222ced0
	ctx.lr = 0x83261204;
	sub_8222CED0(ctx, base);
	// 83261204: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83261208: 3869CA00  addi r3, r9, -0x3600
	ctx.r[3].s64 = ctx.r[9].s64 + -13824;
	// 8326120C: 4BA48D15  bl 0x82ca9f20
	ctx.lr = 0x83261210;
	sub_82CA9F20(ctx, base);
	// 83261210: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83261214: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83261218: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326121C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83261220(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83261220 size=64
    let mut pc: u32 = 0x83261220;
    'dispatch: loop {
        match pc {
            0x83261220 => {
    //   block [0x83261220..0x83261260)
	// 83261220: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83261224: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83261228: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326122C: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 83261230: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83261234: 388B293C  addi r4, r11, 0x293c
	ctx.r[4].s64 = ctx.r[11].s64 + 10556;
	// 83261238: 386AB19C  addi r3, r10, -0x4e64
	ctx.r[3].s64 = ctx.r[10].s64 + -20068;
	// 8326123C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83261240: 4AFCBC91  bl 0x8222ced0
	ctx.lr = 0x83261244;
	sub_8222CED0(ctx, base);
	// 83261244: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83261248: 3869CA10  addi r3, r9, -0x35f0
	ctx.r[3].s64 = ctx.r[9].s64 + -13808;
	// 8326124C: 4BA48CD5  bl 0x82ca9f20
	ctx.lr = 0x83261250;
	sub_82CA9F20(ctx, base);
	// 83261250: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83261254: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83261258: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326125C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83261260(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83261260 size=64
    let mut pc: u32 = 0x83261260;
    'dispatch: loop {
        match pc {
            0x83261260 => {
    //   block [0x83261260..0x832612A0)
	// 83261260: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83261264: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83261268: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326126C: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 83261270: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83261274: 388B2948  addi r4, r11, 0x2948
	ctx.r[4].s64 = ctx.r[11].s64 + 10568;
	// 83261278: 386AB1A0  addi r3, r10, -0x4e60
	ctx.r[3].s64 = ctx.r[10].s64 + -20064;
	// 8326127C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83261280: 4AFCBC51  bl 0x8222ced0
	ctx.lr = 0x83261284;
	sub_8222CED0(ctx, base);
	// 83261284: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83261288: 3869CA20  addi r3, r9, -0x35e0
	ctx.r[3].s64 = ctx.r[9].s64 + -13792;
	// 8326128C: 4BA48C95  bl 0x82ca9f20
	ctx.lr = 0x83261290;
	sub_82CA9F20(ctx, base);
	// 83261290: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83261294: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83261298: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326129C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832612A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832612A0 size=64
    let mut pc: u32 = 0x832612A0;
    'dispatch: loop {
        match pc {
            0x832612A0 => {
    //   block [0x832612A0..0x832612E0)
	// 832612A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832612A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832612A8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832612AC: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 832612B0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832612B4: 388B2950  addi r4, r11, 0x2950
	ctx.r[4].s64 = ctx.r[11].s64 + 10576;
	// 832612B8: 386AB1A4  addi r3, r10, -0x4e5c
	ctx.r[3].s64 = ctx.r[10].s64 + -20060;
	// 832612BC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832612C0: 4AFCBC11  bl 0x8222ced0
	ctx.lr = 0x832612C4;
	sub_8222CED0(ctx, base);
	// 832612C4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 832612C8: 3869CA30  addi r3, r9, -0x35d0
	ctx.r[3].s64 = ctx.r[9].s64 + -13776;
	// 832612CC: 4BA48C55  bl 0x82ca9f20
	ctx.lr = 0x832612D0;
	sub_82CA9F20(ctx, base);
	// 832612D0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832612D4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832612D8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832612DC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832612E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832612E0 size=64
    let mut pc: u32 = 0x832612E0;
    'dispatch: loop {
        match pc {
            0x832612E0 => {
    //   block [0x832612E0..0x83261320)
	// 832612E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832612E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832612E8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832612EC: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 832612F0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832612F4: 388B2958  addi r4, r11, 0x2958
	ctx.r[4].s64 = ctx.r[11].s64 + 10584;
	// 832612F8: 386AB1A8  addi r3, r10, -0x4e58
	ctx.r[3].s64 = ctx.r[10].s64 + -20056;
	// 832612FC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83261300: 4AFCBBD1  bl 0x8222ced0
	ctx.lr = 0x83261304;
	sub_8222CED0(ctx, base);
	// 83261304: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83261308: 3869CA40  addi r3, r9, -0x35c0
	ctx.r[3].s64 = ctx.r[9].s64 + -13760;
	// 8326130C: 4BA48C15  bl 0x82ca9f20
	ctx.lr = 0x83261310;
	sub_82CA9F20(ctx, base);
	// 83261310: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83261314: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83261318: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326131C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83261320(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83261320 size=64
    let mut pc: u32 = 0x83261320;
    'dispatch: loop {
        match pc {
            0x83261320 => {
    //   block [0x83261320..0x83261360)
	// 83261320: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83261324: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83261328: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326132C: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 83261330: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83261334: 388B2960  addi r4, r11, 0x2960
	ctx.r[4].s64 = ctx.r[11].s64 + 10592;
	// 83261338: 386AB1AC  addi r3, r10, -0x4e54
	ctx.r[3].s64 = ctx.r[10].s64 + -20052;
	// 8326133C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83261340: 4AFCBB91  bl 0x8222ced0
	ctx.lr = 0x83261344;
	sub_8222CED0(ctx, base);
	// 83261344: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83261348: 3869CA50  addi r3, r9, -0x35b0
	ctx.r[3].s64 = ctx.r[9].s64 + -13744;
	// 8326134C: 4BA48BD5  bl 0x82ca9f20
	ctx.lr = 0x83261350;
	sub_82CA9F20(ctx, base);
	// 83261350: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83261354: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83261358: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326135C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83261360(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83261360 size=64
    let mut pc: u32 = 0x83261360;
    'dispatch: loop {
        match pc {
            0x83261360 => {
    //   block [0x83261360..0x832613A0)
	// 83261360: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83261364: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83261368: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326136C: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 83261370: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83261374: 388B2968  addi r4, r11, 0x2968
	ctx.r[4].s64 = ctx.r[11].s64 + 10600;
	// 83261378: 386AB1B0  addi r3, r10, -0x4e50
	ctx.r[3].s64 = ctx.r[10].s64 + -20048;
	// 8326137C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83261380: 4AFCBB51  bl 0x8222ced0
	ctx.lr = 0x83261384;
	sub_8222CED0(ctx, base);
	// 83261384: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83261388: 3869CA60  addi r3, r9, -0x35a0
	ctx.r[3].s64 = ctx.r[9].s64 + -13728;
	// 8326138C: 4BA48B95  bl 0x82ca9f20
	ctx.lr = 0x83261390;
	sub_82CA9F20(ctx, base);
	// 83261390: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83261394: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83261398: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326139C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832613A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832613A0 size=64
    let mut pc: u32 = 0x832613A0;
    'dispatch: loop {
        match pc {
            0x832613A0 => {
    //   block [0x832613A0..0x832613E0)
	// 832613A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832613A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832613A8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832613AC: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 832613B0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832613B4: 388B2970  addi r4, r11, 0x2970
	ctx.r[4].s64 = ctx.r[11].s64 + 10608;
	// 832613B8: 386AB1B4  addi r3, r10, -0x4e4c
	ctx.r[3].s64 = ctx.r[10].s64 + -20044;
	// 832613BC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832613C0: 4AFCBB11  bl 0x8222ced0
	ctx.lr = 0x832613C4;
	sub_8222CED0(ctx, base);
	// 832613C4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 832613C8: 3869CA70  addi r3, r9, -0x3590
	ctx.r[3].s64 = ctx.r[9].s64 + -13712;
	// 832613CC: 4BA48B55  bl 0x82ca9f20
	ctx.lr = 0x832613D0;
	sub_82CA9F20(ctx, base);
	// 832613D0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832613D4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832613D8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832613DC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832613E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832613E0 size=64
    let mut pc: u32 = 0x832613E0;
    'dispatch: loop {
        match pc {
            0x832613E0 => {
    //   block [0x832613E0..0x83261420)
	// 832613E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832613E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832613E8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832613EC: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 832613F0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832613F4: 388B2978  addi r4, r11, 0x2978
	ctx.r[4].s64 = ctx.r[11].s64 + 10616;
	// 832613F8: 386AB1B8  addi r3, r10, -0x4e48
	ctx.r[3].s64 = ctx.r[10].s64 + -20040;
	// 832613FC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83261400: 4AFCBAD1  bl 0x8222ced0
	ctx.lr = 0x83261404;
	sub_8222CED0(ctx, base);
	// 83261404: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83261408: 3869CA80  addi r3, r9, -0x3580
	ctx.r[3].s64 = ctx.r[9].s64 + -13696;
	// 8326140C: 4BA48B15  bl 0x82ca9f20
	ctx.lr = 0x83261410;
	sub_82CA9F20(ctx, base);
	// 83261410: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83261414: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83261418: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326141C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83261420(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83261420 size=64
    let mut pc: u32 = 0x83261420;
    'dispatch: loop {
        match pc {
            0x83261420 => {
    //   block [0x83261420..0x83261460)
	// 83261420: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83261424: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83261428: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326142C: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 83261430: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83261434: 388B2980  addi r4, r11, 0x2980
	ctx.r[4].s64 = ctx.r[11].s64 + 10624;
	// 83261438: 386AB1BC  addi r3, r10, -0x4e44
	ctx.r[3].s64 = ctx.r[10].s64 + -20036;
	// 8326143C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83261440: 4AFCBA91  bl 0x8222ced0
	ctx.lr = 0x83261444;
	sub_8222CED0(ctx, base);
	// 83261444: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83261448: 3869CA90  addi r3, r9, -0x3570
	ctx.r[3].s64 = ctx.r[9].s64 + -13680;
	// 8326144C: 4BA48AD5  bl 0x82ca9f20
	ctx.lr = 0x83261450;
	sub_82CA9F20(ctx, base);
	// 83261450: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83261454: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83261458: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326145C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83261460(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83261460 size=64
    let mut pc: u32 = 0x83261460;
    'dispatch: loop {
        match pc {
            0x83261460 => {
    //   block [0x83261460..0x832614A0)
	// 83261460: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83261464: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83261468: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326146C: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 83261470: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83261474: 388B298C  addi r4, r11, 0x298c
	ctx.r[4].s64 = ctx.r[11].s64 + 10636;
	// 83261478: 386AB1C0  addi r3, r10, -0x4e40
	ctx.r[3].s64 = ctx.r[10].s64 + -20032;
	// 8326147C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83261480: 4AFCBA51  bl 0x8222ced0
	ctx.lr = 0x83261484;
	sub_8222CED0(ctx, base);
	// 83261484: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83261488: 3869CAA0  addi r3, r9, -0x3560
	ctx.r[3].s64 = ctx.r[9].s64 + -13664;
	// 8326148C: 4BA48A95  bl 0x82ca9f20
	ctx.lr = 0x83261490;
	sub_82CA9F20(ctx, base);
	// 83261490: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83261494: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83261498: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326149C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832614A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832614A0 size=64
    let mut pc: u32 = 0x832614A0;
    'dispatch: loop {
        match pc {
            0x832614A0 => {
    //   block [0x832614A0..0x832614E0)
	// 832614A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832614A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832614A8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832614AC: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 832614B0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832614B4: 388B8CD0  addi r4, r11, -0x7330
	ctx.r[4].s64 = ctx.r[11].s64 + -29488;
	// 832614B8: 386AB1C4  addi r3, r10, -0x4e3c
	ctx.r[3].s64 = ctx.r[10].s64 + -20028;
	// 832614BC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832614C0: 4AFCBA11  bl 0x8222ced0
	ctx.lr = 0x832614C4;
	sub_8222CED0(ctx, base);
	// 832614C4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 832614C8: 3869CAB0  addi r3, r9, -0x3550
	ctx.r[3].s64 = ctx.r[9].s64 + -13648;
	// 832614CC: 4BA48A55  bl 0x82ca9f20
	ctx.lr = 0x832614D0;
	sub_82CA9F20(ctx, base);
	// 832614D0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832614D4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832614D8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832614DC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832614E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832614E0 size=56
    let mut pc: u32 = 0x832614E0;
    'dispatch: loop {
        match pc {
            0x832614E0 => {
    //   block [0x832614E0..0x83261518)
	// 832614E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832614E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832614E8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832614EC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 832614F0: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 832614F4: 386B42C8  addi r3, r11, 0x42c8
	ctx.r[3].s64 = ctx.r[11].s64 + 17096;
	// 832614F8: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 832614FC: 4AF9285D  bl 0x821f3d58
	ctx.lr = 0x83261500;
	sub_821F3D58(ctx, base);
	// 83261500: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83261504: 906AB1C8  stw r3, -0x4e38(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-20024 as u32), ctx.r[3].u32 ) };
	// 83261508: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326150C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83261510: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83261514: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83261518(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83261518 size=56
    let mut pc: u32 = 0x83261518;
    'dispatch: loop {
        match pc {
            0x83261518 => {
    //   block [0x83261518..0x83261550)
	// 83261518: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326151C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83261520: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83261524: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83261528: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8326152C: 386B42DC  addi r3, r11, 0x42dc
	ctx.r[3].s64 = ctx.r[11].s64 + 17116;
	// 83261530: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83261534: 4AF92825  bl 0x821f3d58
	ctx.lr = 0x83261538;
	sub_821F3D58(ctx, base);
	// 83261538: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326153C: 906AB1CC  stw r3, -0x4e34(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-20020 as u32), ctx.r[3].u32 ) };
	// 83261540: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83261544: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83261548: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326154C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83261550(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83261550 size=56
    let mut pc: u32 = 0x83261550;
    'dispatch: loop {
        match pc {
            0x83261550 => {
    //   block [0x83261550..0x83261588)
	// 83261550: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83261554: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83261558: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326155C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83261560: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83261564: 386B42E8  addi r3, r11, 0x42e8
	ctx.r[3].s64 = ctx.r[11].s64 + 17128;
	// 83261568: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8326156C: 4AF927ED  bl 0x821f3d58
	ctx.lr = 0x83261570;
	sub_821F3D58(ctx, base);
	// 83261570: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83261574: 906AB1D0  stw r3, -0x4e30(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-20016 as u32), ctx.r[3].u32 ) };
	// 83261578: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326157C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83261580: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83261584: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83261588(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83261588 size=56
    let mut pc: u32 = 0x83261588;
    'dispatch: loop {
        match pc {
            0x83261588 => {
    //   block [0x83261588..0x832615C0)
	// 83261588: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326158C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83261590: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83261594: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83261598: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8326159C: 386B4300  addi r3, r11, 0x4300
	ctx.r[3].s64 = ctx.r[11].s64 + 17152;
	// 832615A0: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 832615A4: 4AF927B5  bl 0x821f3d58
	ctx.lr = 0x832615A8;
	sub_821F3D58(ctx, base);
	// 832615A8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832615AC: 906AB1D4  stw r3, -0x4e2c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-20012 as u32), ctx.r[3].u32 ) };
	// 832615B0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832615B4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832615B8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832615BC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832615C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832615C0 size=56
    let mut pc: u32 = 0x832615C0;
    'dispatch: loop {
        match pc {
            0x832615C0 => {
    //   block [0x832615C0..0x832615F8)
	// 832615C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832615C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832615C8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832615CC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 832615D0: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 832615D4: 386B4310  addi r3, r11, 0x4310
	ctx.r[3].s64 = ctx.r[11].s64 + 17168;
	// 832615D8: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 832615DC: 4AF9277D  bl 0x821f3d58
	ctx.lr = 0x832615E0;
	sub_821F3D58(ctx, base);
	// 832615E0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832615E4: 906AB1D8  stw r3, -0x4e28(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-20008 as u32), ctx.r[3].u32 ) };
	// 832615E8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832615EC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832615F0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832615F4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832615F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832615F8 size=56
    let mut pc: u32 = 0x832615F8;
    'dispatch: loop {
        match pc {
            0x832615F8 => {
    //   block [0x832615F8..0x83261630)
	// 832615F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832615FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83261600: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83261604: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83261608: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8326160C: 386B4328  addi r3, r11, 0x4328
	ctx.r[3].s64 = ctx.r[11].s64 + 17192;
	// 83261610: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83261614: 4AF92745  bl 0x821f3d58
	ctx.lr = 0x83261618;
	sub_821F3D58(ctx, base);
	// 83261618: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326161C: 906AB1DC  stw r3, -0x4e24(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-20004 as u32), ctx.r[3].u32 ) };
	// 83261620: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83261624: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83261628: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326162C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83261630(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83261630 size=56
    let mut pc: u32 = 0x83261630;
    'dispatch: loop {
        match pc {
            0x83261630 => {
    //   block [0x83261630..0x83261668)
	// 83261630: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83261634: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83261638: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326163C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83261640: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83261644: 386B4338  addi r3, r11, 0x4338
	ctx.r[3].s64 = ctx.r[11].s64 + 17208;
	// 83261648: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8326164C: 4AF9270D  bl 0x821f3d58
	ctx.lr = 0x83261650;
	sub_821F3D58(ctx, base);
	// 83261650: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83261654: 906AB1E0  stw r3, -0x4e20(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-20000 as u32), ctx.r[3].u32 ) };
	// 83261658: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326165C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83261660: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83261664: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83261668(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83261668 size=56
    let mut pc: u32 = 0x83261668;
    'dispatch: loop {
        match pc {
            0x83261668 => {
    //   block [0x83261668..0x832616A0)
	// 83261668: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326166C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83261670: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83261674: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83261678: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8326167C: 386B4350  addi r3, r11, 0x4350
	ctx.r[3].s64 = ctx.r[11].s64 + 17232;
	// 83261680: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83261684: 4AF926D5  bl 0x821f3d58
	ctx.lr = 0x83261688;
	sub_821F3D58(ctx, base);
	// 83261688: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326168C: 906AB1E4  stw r3, -0x4e1c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-19996 as u32), ctx.r[3].u32 ) };
	// 83261690: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83261694: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83261698: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326169C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832616A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832616A0 size=56
    let mut pc: u32 = 0x832616A0;
    'dispatch: loop {
        match pc {
            0x832616A0 => {
    //   block [0x832616A0..0x832616D8)
	// 832616A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832616A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832616A8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832616AC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 832616B0: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 832616B4: 386B4368  addi r3, r11, 0x4368
	ctx.r[3].s64 = ctx.r[11].s64 + 17256;
	// 832616B8: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 832616BC: 4AF9269D  bl 0x821f3d58
	ctx.lr = 0x832616C0;
	sub_821F3D58(ctx, base);
	// 832616C0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832616C4: 906AB1E8  stw r3, -0x4e18(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-19992 as u32), ctx.r[3].u32 ) };
	// 832616C8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832616CC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832616D0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832616D4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832616D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832616D8 size=56
    let mut pc: u32 = 0x832616D8;
    'dispatch: loop {
        match pc {
            0x832616D8 => {
    //   block [0x832616D8..0x83261710)
	// 832616D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832616DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832616E0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832616E4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 832616E8: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 832616EC: 386B4380  addi r3, r11, 0x4380
	ctx.r[3].s64 = ctx.r[11].s64 + 17280;
	// 832616F0: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 832616F4: 4AF92665  bl 0x821f3d58
	ctx.lr = 0x832616F8;
	sub_821F3D58(ctx, base);
	// 832616F8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832616FC: 906AB1EC  stw r3, -0x4e14(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-19988 as u32), ctx.r[3].u32 ) };
	// 83261700: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83261704: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83261708: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326170C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83261710(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83261710 size=56
    let mut pc: u32 = 0x83261710;
    'dispatch: loop {
        match pc {
            0x83261710 => {
    //   block [0x83261710..0x83261748)
	// 83261710: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83261714: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83261718: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326171C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83261720: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83261724: 386B4394  addi r3, r11, 0x4394
	ctx.r[3].s64 = ctx.r[11].s64 + 17300;
	// 83261728: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8326172C: 4AF9262D  bl 0x821f3d58
	ctx.lr = 0x83261730;
	sub_821F3D58(ctx, base);
	// 83261730: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83261734: 906AB1F0  stw r3, -0x4e10(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-19984 as u32), ctx.r[3].u32 ) };
	// 83261738: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326173C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83261740: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83261744: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83261748(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83261748 size=56
    let mut pc: u32 = 0x83261748;
    'dispatch: loop {
        match pc {
            0x83261748 => {
    //   block [0x83261748..0x83261780)
	// 83261748: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326174C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83261750: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83261754: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83261758: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8326175C: 386B43B4  addi r3, r11, 0x43b4
	ctx.r[3].s64 = ctx.r[11].s64 + 17332;
	// 83261760: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83261764: 4AF925F5  bl 0x821f3d58
	ctx.lr = 0x83261768;
	sub_821F3D58(ctx, base);
	// 83261768: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326176C: 906AB1F4  stw r3, -0x4e0c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-19980 as u32), ctx.r[3].u32 ) };
	// 83261770: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83261774: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83261778: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326177C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83261780(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83261780 size=56
    let mut pc: u32 = 0x83261780;
    'dispatch: loop {
        match pc {
            0x83261780 => {
    //   block [0x83261780..0x832617B8)
	// 83261780: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83261784: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83261788: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326178C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83261790: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83261794: 386B43C8  addi r3, r11, 0x43c8
	ctx.r[3].s64 = ctx.r[11].s64 + 17352;
	// 83261798: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8326179C: 4AF925BD  bl 0x821f3d58
	ctx.lr = 0x832617A0;
	sub_821F3D58(ctx, base);
	// 832617A0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832617A4: 906AB1F8  stw r3, -0x4e08(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-19976 as u32), ctx.r[3].u32 ) };
	// 832617A8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832617AC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832617B0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832617B4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832617B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832617B8 size=56
    let mut pc: u32 = 0x832617B8;
    'dispatch: loop {
        match pc {
            0x832617B8 => {
    //   block [0x832617B8..0x832617F0)
	// 832617B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832617BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832617C0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832617C4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 832617C8: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 832617CC: 386B43D4  addi r3, r11, 0x43d4
	ctx.r[3].s64 = ctx.r[11].s64 + 17364;
	// 832617D0: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 832617D4: 4AF92585  bl 0x821f3d58
	ctx.lr = 0x832617D8;
	sub_821F3D58(ctx, base);
	// 832617D8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832617DC: 906AB1FC  stw r3, -0x4e04(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-19972 as u32), ctx.r[3].u32 ) };
	// 832617E0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832617E4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832617E8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832617EC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832617F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832617F0 size=56
    let mut pc: u32 = 0x832617F0;
    'dispatch: loop {
        match pc {
            0x832617F0 => {
    //   block [0x832617F0..0x83261828)
	// 832617F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832617F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832617F8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832617FC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83261800: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83261804: 386B43E0  addi r3, r11, 0x43e0
	ctx.r[3].s64 = ctx.r[11].s64 + 17376;
	// 83261808: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8326180C: 4AF9254D  bl 0x821f3d58
	ctx.lr = 0x83261810;
	sub_821F3D58(ctx, base);
	// 83261810: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83261814: 906AB200  stw r3, -0x4e00(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-19968 as u32), ctx.r[3].u32 ) };
	// 83261818: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326181C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83261820: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83261824: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83261828(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83261828 size=56
    let mut pc: u32 = 0x83261828;
    'dispatch: loop {
        match pc {
            0x83261828 => {
    //   block [0x83261828..0x83261860)
	// 83261828: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326182C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83261830: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83261834: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83261838: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8326183C: 386B43F0  addi r3, r11, 0x43f0
	ctx.r[3].s64 = ctx.r[11].s64 + 17392;
	// 83261840: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83261844: 4AF92515  bl 0x821f3d58
	ctx.lr = 0x83261848;
	sub_821F3D58(ctx, base);
	// 83261848: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326184C: 906AB204  stw r3, -0x4dfc(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-19964 as u32), ctx.r[3].u32 ) };
	// 83261850: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83261854: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83261858: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326185C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83261860(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83261860 size=56
    let mut pc: u32 = 0x83261860;
    'dispatch: loop {
        match pc {
            0x83261860 => {
    //   block [0x83261860..0x83261898)
	// 83261860: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83261864: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83261868: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326186C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83261870: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83261874: 386B4400  addi r3, r11, 0x4400
	ctx.r[3].s64 = ctx.r[11].s64 + 17408;
	// 83261878: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8326187C: 4AF924DD  bl 0x821f3d58
	ctx.lr = 0x83261880;
	sub_821F3D58(ctx, base);
	// 83261880: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83261884: 906AB208  stw r3, -0x4df8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-19960 as u32), ctx.r[3].u32 ) };
	// 83261888: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326188C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83261890: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83261894: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83261898(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83261898 size=56
    let mut pc: u32 = 0x83261898;
    'dispatch: loop {
        match pc {
            0x83261898 => {
    //   block [0x83261898..0x832618D0)
	// 83261898: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326189C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832618A0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832618A4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 832618A8: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 832618AC: 386B4418  addi r3, r11, 0x4418
	ctx.r[3].s64 = ctx.r[11].s64 + 17432;
	// 832618B0: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 832618B4: 4AF924A5  bl 0x821f3d58
	ctx.lr = 0x832618B8;
	sub_821F3D58(ctx, base);
	// 832618B8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832618BC: 906AB20C  stw r3, -0x4df4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-19956 as u32), ctx.r[3].u32 ) };
	// 832618C0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832618C4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832618C8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832618CC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832618D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832618D0 size=56
    let mut pc: u32 = 0x832618D0;
    'dispatch: loop {
        match pc {
            0x832618D0 => {
    //   block [0x832618D0..0x83261908)
	// 832618D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832618D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832618D8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832618DC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 832618E0: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 832618E4: 386B4424  addi r3, r11, 0x4424
	ctx.r[3].s64 = ctx.r[11].s64 + 17444;
	// 832618E8: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 832618EC: 4AF9246D  bl 0x821f3d58
	ctx.lr = 0x832618F0;
	sub_821F3D58(ctx, base);
	// 832618F0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832618F4: 906AB210  stw r3, -0x4df0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-19952 as u32), ctx.r[3].u32 ) };
	// 832618F8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832618FC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83261900: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83261904: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83261908(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83261908 size=56
    let mut pc: u32 = 0x83261908;
    'dispatch: loop {
        match pc {
            0x83261908 => {
    //   block [0x83261908..0x83261940)
	// 83261908: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326190C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83261910: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83261914: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83261918: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8326191C: 386B4430  addi r3, r11, 0x4430
	ctx.r[3].s64 = ctx.r[11].s64 + 17456;
	// 83261920: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83261924: 4AF92435  bl 0x821f3d58
	ctx.lr = 0x83261928;
	sub_821F3D58(ctx, base);
	// 83261928: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326192C: 906AB214  stw r3, -0x4dec(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-19948 as u32), ctx.r[3].u32 ) };
	// 83261930: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83261934: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83261938: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326193C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83261940(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83261940 size=56
    let mut pc: u32 = 0x83261940;
    'dispatch: loop {
        match pc {
            0x83261940 => {
    //   block [0x83261940..0x83261978)
	// 83261940: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83261944: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83261948: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326194C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83261950: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83261954: 386B4440  addi r3, r11, 0x4440
	ctx.r[3].s64 = ctx.r[11].s64 + 17472;
	// 83261958: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8326195C: 4AF923FD  bl 0x821f3d58
	ctx.lr = 0x83261960;
	sub_821F3D58(ctx, base);
	// 83261960: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83261964: 906AB218  stw r3, -0x4de8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-19944 as u32), ctx.r[3].u32 ) };
	// 83261968: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326196C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83261970: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83261974: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83261978(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83261978 size=64
    let mut pc: u32 = 0x83261978;
    'dispatch: loop {
        match pc {
            0x83261978 => {
    //   block [0x83261978..0x832619B8)
	// 83261978: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326197C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83261980: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83261984: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83261988: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326198C: 388B1AD4  addi r4, r11, 0x1ad4
	ctx.r[4].s64 = ctx.r[11].s64 + 6868;
	// 83261990: 386AB21C  addi r3, r10, -0x4de4
	ctx.r[3].s64 = ctx.r[10].s64 + -19940;
	// 83261994: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83261998: 4AFCB539  bl 0x8222ced0
	ctx.lr = 0x8326199C;
	sub_8222CED0(ctx, base);
	// 8326199C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 832619A0: 3869CAC0  addi r3, r9, -0x3540
	ctx.r[3].s64 = ctx.r[9].s64 + -13632;
	// 832619A4: 4BA4857D  bl 0x82ca9f20
	ctx.lr = 0x832619A8;
	sub_82CA9F20(ctx, base);
	// 832619A8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832619AC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832619B0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832619B4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832619B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832619B8 size=64
    let mut pc: u32 = 0x832619B8;
    'dispatch: loop {
        match pc {
            0x832619B8 => {
    //   block [0x832619B8..0x832619F8)
	// 832619B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832619BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832619C0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832619C4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 832619C8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832619CC: 388B1ADC  addi r4, r11, 0x1adc
	ctx.r[4].s64 = ctx.r[11].s64 + 6876;
	// 832619D0: 386AB220  addi r3, r10, -0x4de0
	ctx.r[3].s64 = ctx.r[10].s64 + -19936;
	// 832619D4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832619D8: 4AFCB4F9  bl 0x8222ced0
	ctx.lr = 0x832619DC;
	sub_8222CED0(ctx, base);
	// 832619DC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 832619E0: 3869CAD0  addi r3, r9, -0x3530
	ctx.r[3].s64 = ctx.r[9].s64 + -13616;
	// 832619E4: 4BA4853D  bl 0x82ca9f20
	ctx.lr = 0x832619E8;
	sub_82CA9F20(ctx, base);
	// 832619E8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832619EC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832619F0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832619F4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832619F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832619F8 size=64
    let mut pc: u32 = 0x832619F8;
    'dispatch: loop {
        match pc {
            0x832619F8 => {
    //   block [0x832619F8..0x83261A38)
	// 832619F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832619FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83261A00: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83261A04: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83261A08: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83261A0C: 388B1AF0  addi r4, r11, 0x1af0
	ctx.r[4].s64 = ctx.r[11].s64 + 6896;
	// 83261A10: 386AB224  addi r3, r10, -0x4ddc
	ctx.r[3].s64 = ctx.r[10].s64 + -19932;
	// 83261A14: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83261A18: 4AFCB4B9  bl 0x8222ced0
	ctx.lr = 0x83261A1C;
	sub_8222CED0(ctx, base);
	// 83261A1C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83261A20: 3869CAE0  addi r3, r9, -0x3520
	ctx.r[3].s64 = ctx.r[9].s64 + -13600;
	// 83261A24: 4BA484FD  bl 0x82ca9f20
	ctx.lr = 0x83261A28;
	sub_82CA9F20(ctx, base);
	// 83261A28: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83261A2C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83261A30: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83261A34: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83261A38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83261A38 size=64
    let mut pc: u32 = 0x83261A38;
    'dispatch: loop {
        match pc {
            0x83261A38 => {
    //   block [0x83261A38..0x83261A78)
	// 83261A38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83261A3C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83261A40: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83261A44: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 83261A48: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83261A4C: 388BF970  addi r4, r11, -0x690
	ctx.r[4].s64 = ctx.r[11].s64 + -1680;
	// 83261A50: 386AB228  addi r3, r10, -0x4dd8
	ctx.r[3].s64 = ctx.r[10].s64 + -19928;
	// 83261A54: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83261A58: 4AFCB479  bl 0x8222ced0
	ctx.lr = 0x83261A5C;
	sub_8222CED0(ctx, base);
	// 83261A5C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83261A60: 3869CAF0  addi r3, r9, -0x3510
	ctx.r[3].s64 = ctx.r[9].s64 + -13584;
	// 83261A64: 4BA484BD  bl 0x82ca9f20
	ctx.lr = 0x83261A68;
	sub_82CA9F20(ctx, base);
	// 83261A68: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83261A6C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83261A70: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83261A74: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83261A78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83261A78 size=64
    let mut pc: u32 = 0x83261A78;
    'dispatch: loop {
        match pc {
            0x83261A78 => {
    //   block [0x83261A78..0x83261AB8)
	// 83261A78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83261A7C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83261A80: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83261A84: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 83261A88: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83261A8C: 388BF990  addi r4, r11, -0x670
	ctx.r[4].s64 = ctx.r[11].s64 + -1648;
	// 83261A90: 386AB22C  addi r3, r10, -0x4dd4
	ctx.r[3].s64 = ctx.r[10].s64 + -19924;
	// 83261A94: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83261A98: 4AFCB439  bl 0x8222ced0
	ctx.lr = 0x83261A9C;
	sub_8222CED0(ctx, base);
	// 83261A9C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83261AA0: 3869CB00  addi r3, r9, -0x3500
	ctx.r[3].s64 = ctx.r[9].s64 + -13568;
	// 83261AA4: 4BA4847D  bl 0x82ca9f20
	ctx.lr = 0x83261AA8;
	sub_82CA9F20(ctx, base);
	// 83261AA8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83261AAC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83261AB0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83261AB4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83261AB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83261AB8 size=64
    let mut pc: u32 = 0x83261AB8;
    'dispatch: loop {
        match pc {
            0x83261AB8 => {
    //   block [0x83261AB8..0x83261AF8)
	// 83261AB8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83261ABC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83261AC0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83261AC4: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 83261AC8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83261ACC: 388BF9A0  addi r4, r11, -0x660
	ctx.r[4].s64 = ctx.r[11].s64 + -1632;
	// 83261AD0: 386AB230  addi r3, r10, -0x4dd0
	ctx.r[3].s64 = ctx.r[10].s64 + -19920;
	// 83261AD4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83261AD8: 4AFCB3F9  bl 0x8222ced0
	ctx.lr = 0x83261ADC;
	sub_8222CED0(ctx, base);
	// 83261ADC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83261AE0: 3869CB10  addi r3, r9, -0x34f0
	ctx.r[3].s64 = ctx.r[9].s64 + -13552;
	// 83261AE4: 4BA4843D  bl 0x82ca9f20
	ctx.lr = 0x83261AE8;
	sub_82CA9F20(ctx, base);
	// 83261AE8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83261AEC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83261AF0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83261AF4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83261AF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83261AF8 size=56
    let mut pc: u32 = 0x83261AF8;
    'dispatch: loop {
        match pc {
            0x83261AF8 => {
    //   block [0x83261AF8..0x83261B30)
	// 83261AF8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83261AFC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83261B00: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83261B04: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83261B08: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83261B0C: 386B42C8  addi r3, r11, 0x42c8
	ctx.r[3].s64 = ctx.r[11].s64 + 17096;
	// 83261B10: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83261B14: 4AF92245  bl 0x821f3d58
	ctx.lr = 0x83261B18;
	sub_821F3D58(ctx, base);
	// 83261B18: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83261B1C: 906AB234  stw r3, -0x4dcc(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-19916 as u32), ctx.r[3].u32 ) };
	// 83261B20: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83261B24: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83261B28: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83261B2C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83261B30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83261B30 size=56
    let mut pc: u32 = 0x83261B30;
    'dispatch: loop {
        match pc {
            0x83261B30 => {
    //   block [0x83261B30..0x83261B68)
	// 83261B30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83261B34: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83261B38: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83261B3C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83261B40: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83261B44: 386B42DC  addi r3, r11, 0x42dc
	ctx.r[3].s64 = ctx.r[11].s64 + 17116;
	// 83261B48: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83261B4C: 4AF9220D  bl 0x821f3d58
	ctx.lr = 0x83261B50;
	sub_821F3D58(ctx, base);
	// 83261B50: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83261B54: 906AB238  stw r3, -0x4dc8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-19912 as u32), ctx.r[3].u32 ) };
	// 83261B58: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83261B5C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83261B60: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83261B64: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83261B68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83261B68 size=56
    let mut pc: u32 = 0x83261B68;
    'dispatch: loop {
        match pc {
            0x83261B68 => {
    //   block [0x83261B68..0x83261BA0)
	// 83261B68: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83261B6C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83261B70: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83261B74: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83261B78: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83261B7C: 386B42E8  addi r3, r11, 0x42e8
	ctx.r[3].s64 = ctx.r[11].s64 + 17128;
	// 83261B80: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83261B84: 4AF921D5  bl 0x821f3d58
	ctx.lr = 0x83261B88;
	sub_821F3D58(ctx, base);
	// 83261B88: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83261B8C: 906AB23C  stw r3, -0x4dc4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-19908 as u32), ctx.r[3].u32 ) };
	// 83261B90: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83261B94: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83261B98: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83261B9C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83261BA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83261BA0 size=56
    let mut pc: u32 = 0x83261BA0;
    'dispatch: loop {
        match pc {
            0x83261BA0 => {
    //   block [0x83261BA0..0x83261BD8)
	// 83261BA0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83261BA4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83261BA8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83261BAC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83261BB0: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83261BB4: 386B4300  addi r3, r11, 0x4300
	ctx.r[3].s64 = ctx.r[11].s64 + 17152;
	// 83261BB8: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83261BBC: 4AF9219D  bl 0x821f3d58
	ctx.lr = 0x83261BC0;
	sub_821F3D58(ctx, base);
	// 83261BC0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83261BC4: 906AB240  stw r3, -0x4dc0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-19904 as u32), ctx.r[3].u32 ) };
	// 83261BC8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83261BCC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83261BD0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83261BD4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83261BD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83261BD8 size=56
    let mut pc: u32 = 0x83261BD8;
    'dispatch: loop {
        match pc {
            0x83261BD8 => {
    //   block [0x83261BD8..0x83261C10)
	// 83261BD8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83261BDC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83261BE0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83261BE4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83261BE8: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83261BEC: 386B4310  addi r3, r11, 0x4310
	ctx.r[3].s64 = ctx.r[11].s64 + 17168;
	// 83261BF0: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83261BF4: 4AF92165  bl 0x821f3d58
	ctx.lr = 0x83261BF8;
	sub_821F3D58(ctx, base);
	// 83261BF8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83261BFC: 906AB244  stw r3, -0x4dbc(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-19900 as u32), ctx.r[3].u32 ) };
	// 83261C00: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83261C04: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83261C08: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83261C0C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83261C10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83261C10 size=56
    let mut pc: u32 = 0x83261C10;
    'dispatch: loop {
        match pc {
            0x83261C10 => {
    //   block [0x83261C10..0x83261C48)
	// 83261C10: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83261C14: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83261C18: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83261C1C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83261C20: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83261C24: 386B4328  addi r3, r11, 0x4328
	ctx.r[3].s64 = ctx.r[11].s64 + 17192;
	// 83261C28: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83261C2C: 4AF9212D  bl 0x821f3d58
	ctx.lr = 0x83261C30;
	sub_821F3D58(ctx, base);
	// 83261C30: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83261C34: 906AB248  stw r3, -0x4db8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-19896 as u32), ctx.r[3].u32 ) };
	// 83261C38: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83261C3C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83261C40: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83261C44: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83261C48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83261C48 size=56
    let mut pc: u32 = 0x83261C48;
    'dispatch: loop {
        match pc {
            0x83261C48 => {
    //   block [0x83261C48..0x83261C80)
	// 83261C48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83261C4C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83261C50: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83261C54: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83261C58: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83261C5C: 386B4338  addi r3, r11, 0x4338
	ctx.r[3].s64 = ctx.r[11].s64 + 17208;
	// 83261C60: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83261C64: 4AF920F5  bl 0x821f3d58
	ctx.lr = 0x83261C68;
	sub_821F3D58(ctx, base);
	// 83261C68: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83261C6C: 906AB24C  stw r3, -0x4db4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-19892 as u32), ctx.r[3].u32 ) };
	// 83261C70: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83261C74: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83261C78: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83261C7C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83261C80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83261C80 size=56
    let mut pc: u32 = 0x83261C80;
    'dispatch: loop {
        match pc {
            0x83261C80 => {
    //   block [0x83261C80..0x83261CB8)
	// 83261C80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83261C84: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83261C88: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83261C8C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83261C90: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83261C94: 386B4350  addi r3, r11, 0x4350
	ctx.r[3].s64 = ctx.r[11].s64 + 17232;
	// 83261C98: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83261C9C: 4AF920BD  bl 0x821f3d58
	ctx.lr = 0x83261CA0;
	sub_821F3D58(ctx, base);
	// 83261CA0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83261CA4: 906AB250  stw r3, -0x4db0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-19888 as u32), ctx.r[3].u32 ) };
	// 83261CA8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83261CAC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83261CB0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83261CB4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83261CB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83261CB8 size=56
    let mut pc: u32 = 0x83261CB8;
    'dispatch: loop {
        match pc {
            0x83261CB8 => {
    //   block [0x83261CB8..0x83261CF0)
	// 83261CB8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83261CBC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83261CC0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83261CC4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83261CC8: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83261CCC: 386B4368  addi r3, r11, 0x4368
	ctx.r[3].s64 = ctx.r[11].s64 + 17256;
	// 83261CD0: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83261CD4: 4AF92085  bl 0x821f3d58
	ctx.lr = 0x83261CD8;
	sub_821F3D58(ctx, base);
	// 83261CD8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83261CDC: 906AB254  stw r3, -0x4dac(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-19884 as u32), ctx.r[3].u32 ) };
	// 83261CE0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83261CE4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83261CE8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83261CEC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83261CF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83261CF0 size=56
    let mut pc: u32 = 0x83261CF0;
    'dispatch: loop {
        match pc {
            0x83261CF0 => {
    //   block [0x83261CF0..0x83261D28)
	// 83261CF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83261CF4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83261CF8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83261CFC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83261D00: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83261D04: 386B4380  addi r3, r11, 0x4380
	ctx.r[3].s64 = ctx.r[11].s64 + 17280;
	// 83261D08: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83261D0C: 4AF9204D  bl 0x821f3d58
	ctx.lr = 0x83261D10;
	sub_821F3D58(ctx, base);
	// 83261D10: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83261D14: 906AB258  stw r3, -0x4da8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-19880 as u32), ctx.r[3].u32 ) };
	// 83261D18: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83261D1C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83261D20: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83261D24: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83261D28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83261D28 size=56
    let mut pc: u32 = 0x83261D28;
    'dispatch: loop {
        match pc {
            0x83261D28 => {
    //   block [0x83261D28..0x83261D60)
	// 83261D28: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83261D2C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83261D30: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83261D34: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83261D38: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83261D3C: 386B4394  addi r3, r11, 0x4394
	ctx.r[3].s64 = ctx.r[11].s64 + 17300;
	// 83261D40: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83261D44: 4AF92015  bl 0x821f3d58
	ctx.lr = 0x83261D48;
	sub_821F3D58(ctx, base);
	// 83261D48: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83261D4C: 906AB25C  stw r3, -0x4da4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-19876 as u32), ctx.r[3].u32 ) };
	// 83261D50: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83261D54: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83261D58: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83261D5C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83261D60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83261D60 size=56
    let mut pc: u32 = 0x83261D60;
    'dispatch: loop {
        match pc {
            0x83261D60 => {
    //   block [0x83261D60..0x83261D98)
	// 83261D60: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83261D64: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83261D68: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83261D6C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83261D70: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83261D74: 386B43B4  addi r3, r11, 0x43b4
	ctx.r[3].s64 = ctx.r[11].s64 + 17332;
	// 83261D78: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83261D7C: 4AF91FDD  bl 0x821f3d58
	ctx.lr = 0x83261D80;
	sub_821F3D58(ctx, base);
	// 83261D80: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83261D84: 906AB260  stw r3, -0x4da0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-19872 as u32), ctx.r[3].u32 ) };
	// 83261D88: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83261D8C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83261D90: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83261D94: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83261D98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83261D98 size=56
    let mut pc: u32 = 0x83261D98;
    'dispatch: loop {
        match pc {
            0x83261D98 => {
    //   block [0x83261D98..0x83261DD0)
	// 83261D98: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83261D9C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83261DA0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83261DA4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83261DA8: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83261DAC: 386B43C8  addi r3, r11, 0x43c8
	ctx.r[3].s64 = ctx.r[11].s64 + 17352;
	// 83261DB0: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83261DB4: 4AF91FA5  bl 0x821f3d58
	ctx.lr = 0x83261DB8;
	sub_821F3D58(ctx, base);
	// 83261DB8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83261DBC: 906AB264  stw r3, -0x4d9c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-19868 as u32), ctx.r[3].u32 ) };
	// 83261DC0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83261DC4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83261DC8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83261DCC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83261DD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83261DD0 size=56
    let mut pc: u32 = 0x83261DD0;
    'dispatch: loop {
        match pc {
            0x83261DD0 => {
    //   block [0x83261DD0..0x83261E08)
	// 83261DD0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83261DD4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83261DD8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83261DDC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83261DE0: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83261DE4: 386B43D4  addi r3, r11, 0x43d4
	ctx.r[3].s64 = ctx.r[11].s64 + 17364;
	// 83261DE8: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83261DEC: 4AF91F6D  bl 0x821f3d58
	ctx.lr = 0x83261DF0;
	sub_821F3D58(ctx, base);
	// 83261DF0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83261DF4: 906AB268  stw r3, -0x4d98(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-19864 as u32), ctx.r[3].u32 ) };
	// 83261DF8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83261DFC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83261E00: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83261E04: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83261E08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83261E08 size=56
    let mut pc: u32 = 0x83261E08;
    'dispatch: loop {
        match pc {
            0x83261E08 => {
    //   block [0x83261E08..0x83261E40)
	// 83261E08: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83261E0C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83261E10: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83261E14: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83261E18: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83261E1C: 386B43E0  addi r3, r11, 0x43e0
	ctx.r[3].s64 = ctx.r[11].s64 + 17376;
	// 83261E20: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83261E24: 4AF91F35  bl 0x821f3d58
	ctx.lr = 0x83261E28;
	sub_821F3D58(ctx, base);
	// 83261E28: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83261E2C: 906AB26C  stw r3, -0x4d94(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-19860 as u32), ctx.r[3].u32 ) };
	// 83261E30: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83261E34: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83261E38: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83261E3C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83261E40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83261E40 size=56
    let mut pc: u32 = 0x83261E40;
    'dispatch: loop {
        match pc {
            0x83261E40 => {
    //   block [0x83261E40..0x83261E78)
	// 83261E40: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83261E44: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83261E48: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83261E4C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83261E50: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83261E54: 386B43F0  addi r3, r11, 0x43f0
	ctx.r[3].s64 = ctx.r[11].s64 + 17392;
	// 83261E58: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83261E5C: 4AF91EFD  bl 0x821f3d58
	ctx.lr = 0x83261E60;
	sub_821F3D58(ctx, base);
	// 83261E60: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83261E64: 906AB270  stw r3, -0x4d90(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-19856 as u32), ctx.r[3].u32 ) };
	// 83261E68: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83261E6C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83261E70: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83261E74: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83261E78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83261E78 size=56
    let mut pc: u32 = 0x83261E78;
    'dispatch: loop {
        match pc {
            0x83261E78 => {
    //   block [0x83261E78..0x83261EB0)
	// 83261E78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83261E7C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83261E80: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83261E84: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83261E88: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83261E8C: 386B4400  addi r3, r11, 0x4400
	ctx.r[3].s64 = ctx.r[11].s64 + 17408;
	// 83261E90: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83261E94: 4AF91EC5  bl 0x821f3d58
	ctx.lr = 0x83261E98;
	sub_821F3D58(ctx, base);
	// 83261E98: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83261E9C: 906AB274  stw r3, -0x4d8c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-19852 as u32), ctx.r[3].u32 ) };
	// 83261EA0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83261EA4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83261EA8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83261EAC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83261EB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83261EB0 size=56
    let mut pc: u32 = 0x83261EB0;
    'dispatch: loop {
        match pc {
            0x83261EB0 => {
    //   block [0x83261EB0..0x83261EE8)
	// 83261EB0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83261EB4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83261EB8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83261EBC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83261EC0: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83261EC4: 386B4418  addi r3, r11, 0x4418
	ctx.r[3].s64 = ctx.r[11].s64 + 17432;
	// 83261EC8: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83261ECC: 4AF91E8D  bl 0x821f3d58
	ctx.lr = 0x83261ED0;
	sub_821F3D58(ctx, base);
	// 83261ED0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83261ED4: 906AB278  stw r3, -0x4d88(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-19848 as u32), ctx.r[3].u32 ) };
	// 83261ED8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83261EDC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83261EE0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83261EE4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83261EE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83261EE8 size=56
    let mut pc: u32 = 0x83261EE8;
    'dispatch: loop {
        match pc {
            0x83261EE8 => {
    //   block [0x83261EE8..0x83261F20)
	// 83261EE8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83261EEC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83261EF0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83261EF4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83261EF8: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83261EFC: 386B4424  addi r3, r11, 0x4424
	ctx.r[3].s64 = ctx.r[11].s64 + 17444;
	// 83261F00: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83261F04: 4AF91E55  bl 0x821f3d58
	ctx.lr = 0x83261F08;
	sub_821F3D58(ctx, base);
	// 83261F08: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83261F0C: 906AB27C  stw r3, -0x4d84(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-19844 as u32), ctx.r[3].u32 ) };
	// 83261F10: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83261F14: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83261F18: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83261F1C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83261F20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83261F20 size=56
    let mut pc: u32 = 0x83261F20;
    'dispatch: loop {
        match pc {
            0x83261F20 => {
    //   block [0x83261F20..0x83261F58)
	// 83261F20: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83261F24: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83261F28: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83261F2C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83261F30: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83261F34: 386B4430  addi r3, r11, 0x4430
	ctx.r[3].s64 = ctx.r[11].s64 + 17456;
	// 83261F38: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83261F3C: 4AF91E1D  bl 0x821f3d58
	ctx.lr = 0x83261F40;
	sub_821F3D58(ctx, base);
	// 83261F40: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83261F44: 906AB280  stw r3, -0x4d80(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-19840 as u32), ctx.r[3].u32 ) };
	// 83261F48: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83261F4C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83261F50: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83261F54: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83261F58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83261F58 size=56
    let mut pc: u32 = 0x83261F58;
    'dispatch: loop {
        match pc {
            0x83261F58 => {
    //   block [0x83261F58..0x83261F90)
	// 83261F58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83261F5C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83261F60: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83261F64: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83261F68: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83261F6C: 386B4440  addi r3, r11, 0x4440
	ctx.r[3].s64 = ctx.r[11].s64 + 17472;
	// 83261F70: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83261F74: 4AF91DE5  bl 0x821f3d58
	ctx.lr = 0x83261F78;
	sub_821F3D58(ctx, base);
	// 83261F78: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83261F7C: 906AB284  stw r3, -0x4d7c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-19836 as u32), ctx.r[3].u32 ) };
	// 83261F80: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83261F84: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83261F88: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83261F8C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83261F90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83261F90 size=64
    let mut pc: u32 = 0x83261F90;
    'dispatch: loop {
        match pc {
            0x83261F90 => {
    //   block [0x83261F90..0x83261FD0)
	// 83261F90: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83261F94: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83261F98: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83261F9C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83261FA0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83261FA4: 388B052C  addi r4, r11, 0x52c
	ctx.r[4].s64 = ctx.r[11].s64 + 1324;
	// 83261FA8: 386AB288  addi r3, r10, -0x4d78
	ctx.r[3].s64 = ctx.r[10].s64 + -19832;
	// 83261FAC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83261FB0: 4AFCAF21  bl 0x8222ced0
	ctx.lr = 0x83261FB4;
	sub_8222CED0(ctx, base);
	// 83261FB4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83261FB8: 3869CB20  addi r3, r9, -0x34e0
	ctx.r[3].s64 = ctx.r[9].s64 + -13536;
	// 83261FBC: 4BA47F65  bl 0x82ca9f20
	ctx.lr = 0x83261FC0;
	sub_82CA9F20(ctx, base);
	// 83261FC0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83261FC4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83261FC8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83261FCC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83261FD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83261FD0 size=64
    let mut pc: u32 = 0x83261FD0;
    'dispatch: loop {
        match pc {
            0x83261FD0 => {
    //   block [0x83261FD0..0x83262010)
	// 83261FD0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83261FD4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83261FD8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83261FDC: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 83261FE0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83261FE4: 388BCE9C  addi r4, r11, -0x3164
	ctx.r[4].s64 = ctx.r[11].s64 + -12644;
	// 83261FE8: 386AB28C  addi r3, r10, -0x4d74
	ctx.r[3].s64 = ctx.r[10].s64 + -19828;
	// 83261FEC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83261FF0: 4AFCAEE1  bl 0x8222ced0
	ctx.lr = 0x83261FF4;
	sub_8222CED0(ctx, base);
	// 83261FF4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83261FF8: 3869CB30  addi r3, r9, -0x34d0
	ctx.r[3].s64 = ctx.r[9].s64 + -13520;
	// 83261FFC: 4BA47F25  bl 0x82ca9f20
	ctx.lr = 0x83262000;
	sub_82CA9F20(ctx, base);
	// 83262000: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83262004: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83262008: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326200C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83262010(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83262010 size=64
    let mut pc: u32 = 0x83262010;
    'dispatch: loop {
        match pc {
            0x83262010 => {
    //   block [0x83262010..0x83262050)
	// 83262010: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83262014: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83262018: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326201C: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 83262020: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83262024: 388BFAF0  addi r4, r11, -0x510
	ctx.r[4].s64 = ctx.r[11].s64 + -1296;
	// 83262028: 386AB290  addi r3, r10, -0x4d70
	ctx.r[3].s64 = ctx.r[10].s64 + -19824;
	// 8326202C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83262030: 4AFCAEA1  bl 0x8222ced0
	ctx.lr = 0x83262034;
	sub_8222CED0(ctx, base);
	// 83262034: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83262038: 3869CB40  addi r3, r9, -0x34c0
	ctx.r[3].s64 = ctx.r[9].s64 + -13504;
	// 8326203C: 4BA47EE5  bl 0x82ca9f20
	ctx.lr = 0x83262040;
	sub_82CA9F20(ctx, base);
	// 83262040: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83262044: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83262048: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326204C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83262050(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83262050 size=64
    let mut pc: u32 = 0x83262050;
    'dispatch: loop {
        match pc {
            0x83262050 => {
    //   block [0x83262050..0x83262090)
	// 83262050: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83262054: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83262058: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326205C: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 83262060: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83262064: 388BFB34  addi r4, r11, -0x4cc
	ctx.r[4].s64 = ctx.r[11].s64 + -1228;
	// 83262068: 386AB294  addi r3, r10, -0x4d6c
	ctx.r[3].s64 = ctx.r[10].s64 + -19820;
	// 8326206C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83262070: 4AFCAE61  bl 0x8222ced0
	ctx.lr = 0x83262074;
	sub_8222CED0(ctx, base);
	// 83262074: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83262078: 3869CB50  addi r3, r9, -0x34b0
	ctx.r[3].s64 = ctx.r[9].s64 + -13488;
	// 8326207C: 4BA47EA5  bl 0x82ca9f20
	ctx.lr = 0x83262080;
	sub_82CA9F20(ctx, base);
	// 83262080: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83262084: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83262088: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326208C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83262090(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83262090 size=64
    let mut pc: u32 = 0x83262090;
    'dispatch: loop {
        match pc {
            0x83262090 => {
    //   block [0x83262090..0x832620D0)
	// 83262090: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83262094: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83262098: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326209C: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 832620A0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832620A4: 388BDED0  addi r4, r11, -0x2130
	ctx.r[4].s64 = ctx.r[11].s64 + -8496;
	// 832620A8: 386AB298  addi r3, r10, -0x4d68
	ctx.r[3].s64 = ctx.r[10].s64 + -19816;
	// 832620AC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832620B0: 4AFCAE21  bl 0x8222ced0
	ctx.lr = 0x832620B4;
	sub_8222CED0(ctx, base);
	// 832620B4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 832620B8: 3869CB60  addi r3, r9, -0x34a0
	ctx.r[3].s64 = ctx.r[9].s64 + -13472;
	// 832620BC: 4BA47E65  bl 0x82ca9f20
	ctx.lr = 0x832620C0;
	sub_82CA9F20(ctx, base);
	// 832620C0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832620C4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832620C8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832620CC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832620D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832620D0 size=64
    let mut pc: u32 = 0x832620D0;
    'dispatch: loop {
        match pc {
            0x832620D0 => {
    //   block [0x832620D0..0x83262110)
	// 832620D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832620D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832620D8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832620DC: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 832620E0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832620E4: 388BFB64  addi r4, r11, -0x49c
	ctx.r[4].s64 = ctx.r[11].s64 + -1180;
	// 832620E8: 386AB29C  addi r3, r10, -0x4d64
	ctx.r[3].s64 = ctx.r[10].s64 + -19812;
	// 832620EC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832620F0: 4AFCADE1  bl 0x8222ced0
	ctx.lr = 0x832620F4;
	sub_8222CED0(ctx, base);
	// 832620F4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 832620F8: 3869CB70  addi r3, r9, -0x3490
	ctx.r[3].s64 = ctx.r[9].s64 + -13456;
	// 832620FC: 4BA47E25  bl 0x82ca9f20
	ctx.lr = 0x83262100;
	sub_82CA9F20(ctx, base);
	// 83262100: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83262104: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83262108: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326210C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83262110(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83262110 size=64
    let mut pc: u32 = 0x83262110;
    'dispatch: loop {
        match pc {
            0x83262110 => {
    //   block [0x83262110..0x83262150)
	// 83262110: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83262114: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83262118: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326211C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83262120: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83262124: 388B1AD4  addi r4, r11, 0x1ad4
	ctx.r[4].s64 = ctx.r[11].s64 + 6868;
	// 83262128: 386AB2A0  addi r3, r10, -0x4d60
	ctx.r[3].s64 = ctx.r[10].s64 + -19808;
	// 8326212C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83262130: 4AFCADA1  bl 0x8222ced0
	ctx.lr = 0x83262134;
	sub_8222CED0(ctx, base);
	// 83262134: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83262138: 3869CB80  addi r3, r9, -0x3480
	ctx.r[3].s64 = ctx.r[9].s64 + -13440;
	// 8326213C: 4BA47DE5  bl 0x82ca9f20
	ctx.lr = 0x83262140;
	sub_82CA9F20(ctx, base);
	// 83262140: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83262144: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83262148: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326214C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83262150(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83262150 size=64
    let mut pc: u32 = 0x83262150;
    'dispatch: loop {
        match pc {
            0x83262150 => {
    //   block [0x83262150..0x83262190)
	// 83262150: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83262154: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83262158: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326215C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83262160: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83262164: 388B1ADC  addi r4, r11, 0x1adc
	ctx.r[4].s64 = ctx.r[11].s64 + 6876;
	// 83262168: 386AB2A4  addi r3, r10, -0x4d5c
	ctx.r[3].s64 = ctx.r[10].s64 + -19804;
	// 8326216C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83262170: 4AFCAD61  bl 0x8222ced0
	ctx.lr = 0x83262174;
	sub_8222CED0(ctx, base);
	// 83262174: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83262178: 3869CB90  addi r3, r9, -0x3470
	ctx.r[3].s64 = ctx.r[9].s64 + -13424;
	// 8326217C: 4BA47DA5  bl 0x82ca9f20
	ctx.lr = 0x83262180;
	sub_82CA9F20(ctx, base);
	// 83262180: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83262184: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83262188: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326218C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83262190(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83262190 size=64
    let mut pc: u32 = 0x83262190;
    'dispatch: loop {
        match pc {
            0x83262190 => {
    //   block [0x83262190..0x832621D0)
	// 83262190: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83262194: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83262198: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326219C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 832621A0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832621A4: 388B1AF0  addi r4, r11, 0x1af0
	ctx.r[4].s64 = ctx.r[11].s64 + 6896;
	// 832621A8: 386AB2A8  addi r3, r10, -0x4d58
	ctx.r[3].s64 = ctx.r[10].s64 + -19800;
	// 832621AC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832621B0: 4AFCAD21  bl 0x8222ced0
	ctx.lr = 0x832621B4;
	sub_8222CED0(ctx, base);
	// 832621B4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 832621B8: 3869CBA0  addi r3, r9, -0x3460
	ctx.r[3].s64 = ctx.r[9].s64 + -13408;
	// 832621BC: 4BA47D65  bl 0x82ca9f20
	ctx.lr = 0x832621C0;
	sub_82CA9F20(ctx, base);
	// 832621C0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832621C4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832621C8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832621CC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832621D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x832621D0 size=96
    let mut pc: u32 = 0x832621D0;
    'dispatch: loop {
        match pc {
            0x832621D0 => {
    //   block [0x832621D0..0x83262230)
	// 832621D0: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 832621D4: 3941FFF0  addi r10, r1, -0x10
	ctx.r[10].s64 = ctx.r[1].s64 + -16;
	// 832621D8: 3921FFF4  addi r9, r1, -0xc
	ctx.r[9].s64 = ctx.r[1].s64 + -12;
	// 832621DC: 3D008210  lis r8, -0x7df0
	ctx.r[8].s64 = -2112880640;
	// 832621E0: 38E1FFF4  addi r7, r1, -0xc
	ctx.r[7].s64 = ctx.r[1].s64 + -12;
	// 832621E4: C00B9484  lfs f0, -0x6b7c(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-27516 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 832621E8: 38C1FFF0  addi r6, r1, -0x10
	ctx.r[6].s64 = ctx.r[1].s64 + -16;
	// 832621EC: D001FFF0  stfs f0, -0x10(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), tmp.u32 ) };
	// 832621F0: 3CA0834A  lis r5, -0x7cb6
	ctx.r[5].s64 = -2092302336;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83262230(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x83262230 size=96
    let mut pc: u32 = 0x83262230;
    'dispatch: loop {
        match pc {
            0x83262230 => {
    //   block [0x83262230..0x83262290)
	// 83262230: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83262234: 3941FFF4  addi r10, r1, -0xc
	ctx.r[10].s64 = ctx.r[1].s64 + -12;
	// 83262238: 392B9490  addi r9, r11, -0x6b70
	ctx.r[9].s64 = ctx.r[11].s64 + -27504;
	// 8326223C: 3901FFF8  addi r8, r1, -8
	ctx.r[8].s64 = ctx.r[1].s64 + -8;
	// 83262240: 38E1FFF0  addi r7, r1, -0x10
	ctx.r[7].s64 = ctx.r[1].s64 + -16;
	// 83262244: C00B9490  lfs f0, -0x6b70(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-27504 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 83262248: 38C1FFF8  addi r6, r1, -8
	ctx.r[6].s64 = ctx.r[1].s64 + -8;
	// 8326224C: D001FFF0  stfs f0, -0x10(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), tmp.u32 ) };
	// 83262250: 3CA0834A  lis r5, -0x7cb6
	ctx.r[5].s64 = -2092302336;
	// 83262254: C009FFF4  lfs f0, -0xc(r9)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(-12 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 83262258: D001FFF4  stfs f0, -0xc(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-12 as u32), tmp.u32 ) };
	// 8326225C: 3885B2C0  addi r4, r5, -0x4d40
	ctx.r[4].s64 = ctx.r[5].s64 + -19776;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83262290(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x83262290 size=96
    let mut pc: u32 = 0x83262290;
    'dispatch: loop {
        match pc {
            0x83262290 => {
    //   block [0x83262290..0x832622F0)
	// 83262290: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83262294: 3901FFF0  addi r8, r1, -0x10
	ctx.r[8].s64 = ctx.r[1].s64 + -16;
	// 83262298: 392B9490  addi r9, r11, -0x6b70
	ctx.r[9].s64 = ctx.r[11].s64 + -27504;
	// 8326229C: 3941FFF4  addi r10, r1, -0xc
	ctx.r[10].s64 = ctx.r[1].s64 + -12;
	// 832622A0: 38E1FFF4  addi r7, r1, -0xc
	ctx.r[7].s64 = ctx.r[1].s64 + -12;
	// 832622A4: C00B9490  lfs f0, -0x6b70(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-27504 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 832622A8: 38C1FFF0  addi r6, r1, -0x10
	ctx.r[6].s64 = ctx.r[1].s64 + -16;
	// 832622AC: D001FFF0  stfs f0, -0x10(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), tmp.u32 ) };
	// 832622B0: 3CA0834A  lis r5, -0x7cb6
	ctx.r[5].s64 = -2092302336;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832622F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x832622F0 size=112
    let mut pc: u32 = 0x832622F0;
    'dispatch: loop {
        match pc {
            0x832622F0 => {
    //   block [0x832622F0..0x83262360)
	// 832622F0: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 832622F4: 3901FFF0  addi r8, r1, -0x10
	ctx.r[8].s64 = ctx.r[1].s64 + -16;
	// 832622F8: 392BD074  addi r9, r11, -0x2f8c
	ctx.r[9].s64 = ctx.r[11].s64 + -12172;
	// 832622FC: 3D408210  lis r10, -0x7df0
	ctx.r[10].s64 = -2112880640;
	// 83262300: 38E1FFF4  addi r7, r1, -0xc
	ctx.r[7].s64 = ctx.r[1].s64 + -12;
	// 83262304: 3CC08210  lis r6, -0x7df0
	ctx.r[6].s64 = -2112880640;
	// 83262308: C1ABD074  lfs f13, -0x2f8c(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-12172 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 8326230C: 38A1FFF4  addi r5, r1, -0xc
	ctx.r[5].s64 = ctx.r[1].s64 + -12;
	// 83262310: C009C410  lfs f0, -0x3bf0(r9)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(-15344 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 83262314: 3881FFF0  addi r4, r1, -0x10
	ctx.r[4].s64 = ctx.r[1].s64 + -16;
	// 83262318: D001FFF0  stfs f0, -0x10(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), tmp.u32 ) };
	// 8326231C: 3C60834A  lis r3, -0x7cb6
	ctx.r[3].s64 = -2092302336;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83262360(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83262360 size=64
    let mut pc: u32 = 0x83262360;
    'dispatch: loop {
        match pc {
            0x83262360 => {
    //   block [0x83262360..0x832623A0)
	// 83262360: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83262364: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83262368: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326236C: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 83262370: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83262374: 388BFF08  addi r4, r11, -0xf8
	ctx.r[4].s64 = ctx.r[11].s64 + -248;
	// 83262378: 386AB2AC  addi r3, r10, -0x4d54
	ctx.r[3].s64 = ctx.r[10].s64 + -19796;
	// 8326237C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83262380: 4AFCAB51  bl 0x8222ced0
	ctx.lr = 0x83262384;
	sub_8222CED0(ctx, base);
	// 83262384: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83262388: 3869CBB0  addi r3, r9, -0x3450
	ctx.r[3].s64 = ctx.r[9].s64 + -13392;
	// 8326238C: 4BA47B95  bl 0x82ca9f20
	ctx.lr = 0x83262390;
	sub_82CA9F20(ctx, base);
	// 83262390: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83262394: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83262398: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326239C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832623A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832623A0 size=64
    let mut pc: u32 = 0x832623A0;
    'dispatch: loop {
        match pc {
            0x832623A0 => {
    //   block [0x832623A0..0x832623E0)
	// 832623A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832623A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832623A8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832623AC: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 832623B0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832623B4: 388BFF18  addi r4, r11, -0xe8
	ctx.r[4].s64 = ctx.r[11].s64 + -232;
	// 832623B8: 386AB2F0  addi r3, r10, -0x4d10
	ctx.r[3].s64 = ctx.r[10].s64 + -19728;
	// 832623BC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832623C0: 4AFCAB11  bl 0x8222ced0
	ctx.lr = 0x832623C4;
	sub_8222CED0(ctx, base);
	// 832623C4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 832623C8: 3869CBC0  addi r3, r9, -0x3440
	ctx.r[3].s64 = ctx.r[9].s64 + -13376;
	// 832623CC: 4BA47B55  bl 0x82ca9f20
	ctx.lr = 0x832623D0;
	sub_82CA9F20(ctx, base);
	// 832623D0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832623D4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832623D8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832623DC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832623E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832623E0 size=64
    let mut pc: u32 = 0x832623E0;
    'dispatch: loop {
        match pc {
            0x832623E0 => {
    //   block [0x832623E0..0x83262420)
	// 832623E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832623E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832623E8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832623EC: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 832623F0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832623F4: 388BFF28  addi r4, r11, -0xd8
	ctx.r[4].s64 = ctx.r[11].s64 + -216;
	// 832623F8: 386AB2F4  addi r3, r10, -0x4d0c
	ctx.r[3].s64 = ctx.r[10].s64 + -19724;
	// 832623FC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83262400: 4AFCAAD1  bl 0x8222ced0
	ctx.lr = 0x83262404;
	sub_8222CED0(ctx, base);
	// 83262404: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83262408: 3869CBD0  addi r3, r9, -0x3430
	ctx.r[3].s64 = ctx.r[9].s64 + -13360;
	// 8326240C: 4BA47B15  bl 0x82ca9f20
	ctx.lr = 0x83262410;
	sub_82CA9F20(ctx, base);
	// 83262410: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83262414: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83262418: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326241C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83262420(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x83262420 size=112
    let mut pc: u32 = 0x83262420;
    'dispatch: loop {
        match pc {
            0x83262420 => {
    //   block [0x83262420..0x83262490)
	// 83262420: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83262424: 3901FFF0  addi r8, r1, -0x10
	ctx.r[8].s64 = ctx.r[1].s64 + -16;
	// 83262428: 392BB7A4  addi r9, r11, -0x485c
	ctx.r[9].s64 = ctx.r[11].s64 + -18524;
	// 8326242C: 3941FFF4  addi r10, r1, -0xc
	ctx.r[10].s64 = ctx.r[1].s64 + -12;
	// 83262430: 3CE08210  lis r7, -0x7df0
	ctx.r[7].s64 = -2112880640;
	// 83262434: C00BB7A4  lfs f0, -0x485c(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-18524 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 83262438: 3CC08210  lis r6, -0x7df0
	ctx.r[6].s64 = -2112880640;
	// 8326243C: D001FFF0  stfs f0, -0x10(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), tmp.u32 ) };
	// 83262440: 38A1FFF4  addi r5, r1, -0xc
	ctx.r[5].s64 = ctx.r[1].s64 + -12;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83262490(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x83262490 size=100
    let mut pc: u32 = 0x83262490;
    'dispatch: loop {
        match pc {
            0x83262490 => {
    //   block [0x83262490..0x832624F4)
	// 83262490: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83262494: 3941FFF4  addi r10, r1, -0xc
	ctx.r[10].s64 = ctx.r[1].s64 + -12;
	// 83262498: 392B92D4  addi r9, r11, -0x6d2c
	ctx.r[9].s64 = ctx.r[11].s64 + -27948;
	// 8326249C: 3901FFF8  addi r8, r1, -8
	ctx.r[8].s64 = ctx.r[1].s64 + -8;
	// 832624A0: 38E1FFF0  addi r7, r1, -0x10
	ctx.r[7].s64 = ctx.r[1].s64 + -16;
	// 832624A4: C00B92D4  lfs f0, -0x6d2c(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-27948 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 832624A8: 38C1FFF8  addi r6, r1, -8
	ctx.r[6].s64 = ctx.r[1].s64 + -8;
	// 832624AC: D001FFF0  stfs f0, -0x10(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), tmp.u32 ) };
	// 832624B0: 3CA0834A  lis r5, -0x7cb6
	ctx.r[5].s64 = -2092302336;
	// 832624B4: C1A901B0  lfs f13, 0x1b0(r9)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(432 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 832624B8: D1A1FFF4  stfs f13, -0xc(r1)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-12 as u32), tmp.u32 ) };
	// 832624BC: 3885B310  addi r4, r5, -0x4cf0
	ctx.r[4].s64 = ctx.r[5].s64 + -19696;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832624F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x832624F8 size=96
    let mut pc: u32 = 0x832624F8;
    'dispatch: loop {
        match pc {
            0x832624F8 => {
    //   block [0x832624F8..0x83262558)
	// 832624F8: 3D40820A  lis r10, -0x7df6
	ctx.r[10].s64 = -2113273856;
	// 832624FC: 3921FFF0  addi r9, r1, -0x10
	ctx.r[9].s64 = ctx.r[1].s64 + -16;
	// 83262500: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 83262504: 3901FFF4  addi r8, r1, -0xc
	ctx.r[8].s64 = ctx.r[1].s64 + -12;
	// 83262508: 38E1FFF4  addi r7, r1, -0xc
	ctx.r[7].s64 = ctx.r[1].s64 + -12;
	// 8326250C: C1AA9490  lfs f13, -0x6b70(r10)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-27504 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 83262510: 38C1FFF0  addi r6, r1, -0x10
	ctx.r[6].s64 = ctx.r[1].s64 + -16;
	// 83262514: D1A1FFF0  stfs f13, -0x10(r1)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), tmp.u32 ) };
	// 83262518: 3CA0834A  lis r5, -0x7cb6
	ctx.r[5].s64 = -2092302336;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83262558(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x83262558 size=108
    let mut pc: u32 = 0x83262558;
    'dispatch: loop {
        match pc {
            0x83262558 => {
    //   block [0x83262558..0x832625C4)
	// 83262558: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8326255C: 3941FFF4  addi r10, r1, -0xc
	ctx.r[10].s64 = ctx.r[1].s64 + -12;
	// 83262560: 392BCFCC  addi r9, r11, -0x3034
	ctx.r[9].s64 = ctx.r[11].s64 + -12340;
	// 83262564: 3901FFF8  addi r8, r1, -8
	ctx.r[8].s64 = ctx.r[1].s64 + -8;
	// 83262568: 3CE08210  lis r7, -0x7df0
	ctx.r[7].s64 = -2112880640;
	// 8326256C: C00BCFCC  lfs f0, -0x3034(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-12340 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 83262570: 38C1FFF8  addi r6, r1, -8
	ctx.r[6].s64 = ctx.r[1].s64 + -8;
	// 83262574: D001FFF0  stfs f0, -0x10(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), tmp.u32 ) };
	// 83262578: 38A1FFF0  addi r5, r1, -0x10
	ctx.r[5].s64 = ctx.r[1].s64 + -16;
	// 8326257C: C009C4B8  lfs f0, -0x3b48(r9)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(-15176 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 83262580: 3C80834A  lis r4, -0x7cb6
	ctx.r[4].s64 = -2092302336;
	// 83262584: D001FFF4  stfs f0, -0xc(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-12 as u32), tmp.u32 ) };
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832625C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x832625C8 size=108
    let mut pc: u32 = 0x832625C8;
    'dispatch: loop {
        match pc {
            0x832625C8 => {
    //   block [0x832625C8..0x83262634)
	// 832625C8: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 832625CC: 3901FFF0  addi r8, r1, -0x10
	ctx.r[8].s64 = ctx.r[1].s64 + -16;
	// 832625D0: 392B92D4  addi r9, r11, -0x6d2c
	ctx.r[9].s64 = ctx.r[11].s64 + -27948;
	// 832625D4: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 832625D8: 38E1FFF4  addi r7, r1, -0xc
	ctx.r[7].s64 = ctx.r[1].s64 + -12;
	// 832625DC: 38C1FFF4  addi r6, r1, -0xc
	ctx.r[6].s64 = ctx.r[1].s64 + -12;
	// 832625E0: C1AB92D4  lfs f13, -0x6d2c(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-27948 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 832625E4: 38A1FFF0  addi r5, r1, -0x10
	ctx.r[5].s64 = ctx.r[1].s64 + -16;
	// 832625E8: C00901B0  lfs f0, 0x1b0(r9)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(432 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 832625EC: 3C80834A  lis r4, -0x7cb6
	ctx.r[4].s64 = -2092302336;
	// 832625F0: D001FFF0  stfs f0, -0x10(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), tmp.u32 ) };
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83262638(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x83262638 size=108
    let mut pc: u32 = 0x83262638;
    'dispatch: loop {
        match pc {
            0x83262638 => {
    //   block [0x83262638..0x832626A4)
	// 83262638: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8326263C: 3941FFF4  addi r10, r1, -0xc
	ctx.r[10].s64 = ctx.r[1].s64 + -12;
	// 83262640: 392BE0DC  addi r9, r11, -0x1f24
	ctx.r[9].s64 = ctx.r[11].s64 + -7972;
	// 83262644: 3901FFF8  addi r8, r1, -8
	ctx.r[8].s64 = ctx.r[1].s64 + -8;
	// 83262648: 3CE08210  lis r7, -0x7df0
	ctx.r[7].s64 = -2112880640;
	// 8326264C: C00BE0DC  lfs f0, -0x1f24(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-7972 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 83262650: 38C1FFF0  addi r6, r1, -0x10
	ctx.r[6].s64 = ctx.r[1].s64 + -16;
	// 83262654: D001FFF0  stfs f0, -0x10(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), tmp.u32 ) };
	// 83262658: 38A1FFF8  addi r5, r1, -8
	ctx.r[5].s64 = ctx.r[1].s64 + -8;
	// 8326265C: C009B3B4  lfs f0, -0x4c4c(r9)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(-19532 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 83262660: 3C80834A  lis r4, -0x7cb6
	ctx.r[4].s64 = -2092302336;
	// 83262664: D001FFF4  stfs f0, -0xc(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-12 as u32), tmp.u32 ) };
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832626A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832626A8 size=64
    let mut pc: u32 = 0x832626A8;
    'dispatch: loop {
        match pc {
            0x832626A8 => {
    //   block [0x832626A8..0x832626E8)
	// 832626A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832626AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832626B0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832626B4: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 832626B8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832626BC: 388B35B0  addi r4, r11, 0x35b0
	ctx.r[4].s64 = ctx.r[11].s64 + 13744;
	// 832626C0: 386AB2F8  addi r3, r10, -0x4d08
	ctx.r[3].s64 = ctx.r[10].s64 + -19720;
	// 832626C4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832626C8: 4AFCA809  bl 0x8222ced0
	ctx.lr = 0x832626CC;
	sub_8222CED0(ctx, base);
	// 832626CC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 832626D0: 3869CBE0  addi r3, r9, -0x3420
	ctx.r[3].s64 = ctx.r[9].s64 + -13344;
	// 832626D4: 4BA4784D  bl 0x82ca9f20
	ctx.lr = 0x832626D8;
	sub_82CA9F20(ctx, base);
	// 832626D8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832626DC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832626E0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832626E4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832626E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x832626E8 size=96
    let mut pc: u32 = 0x832626E8;
    'dispatch: loop {
        match pc {
            0x832626E8 => {
    //   block [0x832626E8..0x83262748)
	// 832626E8: 3D40820A  lis r10, -0x7df6
	ctx.r[10].s64 = -2113273856;
	// 832626EC: 3921FFF0  addi r9, r1, -0x10
	ctx.r[9].s64 = ctx.r[1].s64 + -16;
	// 832626F0: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 832626F4: 3901FFF4  addi r8, r1, -0xc
	ctx.r[8].s64 = ctx.r[1].s64 + -12;
	// 832626F8: 38E1FFF4  addi r7, r1, -0xc
	ctx.r[7].s64 = ctx.r[1].s64 + -12;
	// 832626FC: C1AA9490  lfs f13, -0x6b70(r10)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-27504 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 83262700: 38C1FFF0  addi r6, r1, -0x10
	ctx.r[6].s64 = ctx.r[1].s64 + -16;
	// 83262704: D1A1FFF0  stfs f13, -0x10(r1)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), tmp.u32 ) };
	// 83262708: 3CA0834A  lis r5, -0x7cb6
	ctx.r[5].s64 = -2092302336;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83262748(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x83262748 size=96
    let mut pc: u32 = 0x83262748;
    'dispatch: loop {
        match pc {
            0x83262748 => {
    //   block [0x83262748..0x832627A8)
	// 83262748: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8326274C: 3941FFF4  addi r10, r1, -0xc
	ctx.r[10].s64 = ctx.r[1].s64 + -12;
	// 83262750: 392B9490  addi r9, r11, -0x6b70
	ctx.r[9].s64 = ctx.r[11].s64 + -27504;
	// 83262754: 3901FFF8  addi r8, r1, -8
	ctx.r[8].s64 = ctx.r[1].s64 + -8;
	// 83262758: 38E1FFF0  addi r7, r1, -0x10
	ctx.r[7].s64 = ctx.r[1].s64 + -16;
	// 8326275C: C00B9490  lfs f0, -0x6b70(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-27504 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 83262760: 38C1FFF8  addi r6, r1, -8
	ctx.r[6].s64 = ctx.r[1].s64 + -8;
	// 83262764: D001FFF0  stfs f0, -0x10(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), tmp.u32 ) };
	// 83262768: 3CA0834A  lis r5, -0x7cb6
	ctx.r[5].s64 = -2092302336;
	// 8326276C: C009FFF4  lfs f0, -0xc(r9)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(-12 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 83262770: D001FFF4  stfs f0, -0xc(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-12 as u32), tmp.u32 ) };
	// 83262774: 3885B370  addi r4, r5, -0x4c90
	ctx.r[4].s64 = ctx.r[5].s64 + -19600;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832627A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832627A8 size=376
    let mut pc: u32 = 0x832627A8;
    'dispatch: loop {
        match pc {
            0x832627A8 => {
    //   block [0x832627A8..0x83262920)
	// 832627A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832627AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832627B0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 832627B4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832627B8: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832627BC: 3D40820B  lis r10, -0x7df5
	ctx.r[10].s64 = -2113208320;
	// 832627C0: 3BEBB380  addi r31, r11, -0x4c80
	ctx.r[31].s64 = ctx.r[11].s64 + -19584;
	// 832627C4: 388A8A50  addi r4, r10, -0x75b0
	ctx.r[4].s64 = ctx.r[10].s64 + -30128;
	// 832627C8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 832627CC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832627D0: 4AFCA701  bl 0x8222ced0
	ctx.lr = 0x832627D4;
	sub_8222CED0(ctx, base);
	// 832627D4: 3D20820B  lis r9, -0x7df5
	ctx.r[9].s64 = -2113208320;
	// 832627D8: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832627DC: 38898A30  addi r4, r9, -0x75d0
	ctx.r[4].s64 = ctx.r[9].s64 + -30160;
	// 832627E0: 387F0004  addi r3, r31, 4
	ctx.r[3].s64 = ctx.r[31].s64 + 4;
	// 832627E4: 4AFCA6ED  bl 0x8222ced0
	ctx.lr = 0x832627E8;
	sub_8222CED0(ctx, base);
	// 832627E8: 3D00820B  lis r8, -0x7df5
	ctx.r[8].s64 = -2113208320;
	// 832627EC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832627F0: 38888A10  addi r4, r8, -0x75f0
	ctx.r[4].s64 = ctx.r[8].s64 + -30192;
	// 832627F4: 387F0008  addi r3, r31, 8
	ctx.r[3].s64 = ctx.r[31].s64 + 8;
	// 832627F8: 4AFCA6D9  bl 0x8222ced0
	ctx.lr = 0x832627FC;
	sub_8222CED0(ctx, base);
	// 832627FC: 3CE0820B  lis r7, -0x7df5
	ctx.r[7].s64 = -2113208320;
	// 83262800: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83262804: 388789F0  addi r4, r7, -0x7610
	ctx.r[4].s64 = ctx.r[7].s64 + -30224;
	// 83262808: 387F000C  addi r3, r31, 0xc
	ctx.r[3].s64 = ctx.r[31].s64 + 12;
	// 8326280C: 4AFCA6C5  bl 0x8222ced0
	ctx.lr = 0x83262810;
	sub_8222CED0(ctx, base);
	// 83262810: 3CC0820B  lis r6, -0x7df5
	ctx.r[6].s64 = -2113208320;
	// 83262814: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83262818: 388689D0  addi r4, r6, -0x7630
	ctx.r[4].s64 = ctx.r[6].s64 + -30256;
	// 8326281C: 387F0010  addi r3, r31, 0x10
	ctx.r[3].s64 = ctx.r[31].s64 + 16;
	// 83262820: 4AFCA6B1  bl 0x8222ced0
	ctx.lr = 0x83262824;
	sub_8222CED0(ctx, base);
	// 83262824: 3C80820B  lis r4, -0x7df5
	ctx.r[4].s64 = -2113208320;
	// 83262828: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8326282C: 388489B0  addi r4, r4, -0x7650
	ctx.r[4].s64 = ctx.r[4].s64 + -30288;
	// 83262830: 387F0014  addi r3, r31, 0x14
	ctx.r[3].s64 = ctx.r[31].s64 + 20;
	// 83262834: 4AFCA69D  bl 0x8222ced0
	ctx.lr = 0x83262838;
	sub_8222CED0(ctx, base);
	// 83262838: 3C60820B  lis r3, -0x7df5
	ctx.r[3].s64 = -2113208320;
	// 8326283C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83262840: 38838990  addi r4, r3, -0x7670
	ctx.r[4].s64 = ctx.r[3].s64 + -30320;
	// 83262844: 387F0018  addi r3, r31, 0x18
	ctx.r[3].s64 = ctx.r[31].s64 + 24;
	// 83262848: 4AFCA689  bl 0x8222ced0
	ctx.lr = 0x8326284C;
	sub_8222CED0(ctx, base);
	// 8326284C: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 83262850: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83262854: 388B8970  addi r4, r11, -0x7690
	ctx.r[4].s64 = ctx.r[11].s64 + -30352;
	// 83262858: 387F001C  addi r3, r31, 0x1c
	ctx.r[3].s64 = ctx.r[31].s64 + 28;
	// 8326285C: 4AFCA675  bl 0x8222ced0
	ctx.lr = 0x83262860;
	sub_8222CED0(ctx, base);
	// 83262860: 3D40820B  lis r10, -0x7df5
	ctx.r[10].s64 = -2113208320;
	// 83262864: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83262868: 388A8950  addi r4, r10, -0x76b0
	ctx.r[4].s64 = ctx.r[10].s64 + -30384;
	// 8326286C: 387F0020  addi r3, r31, 0x20
	ctx.r[3].s64 = ctx.r[31].s64 + 32;
	// 83262870: 4AFCA661  bl 0x8222ced0
	ctx.lr = 0x83262874;
	sub_8222CED0(ctx, base);
	// 83262874: 3D20820B  lis r9, -0x7df5
	ctx.r[9].s64 = -2113208320;
	// 83262878: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8326287C: 38898930  addi r4, r9, -0x76d0
	ctx.r[4].s64 = ctx.r[9].s64 + -30416;
	// 83262880: 387F0024  addi r3, r31, 0x24
	ctx.r[3].s64 = ctx.r[31].s64 + 36;
	// 83262884: 4AFCA64D  bl 0x8222ced0
	ctx.lr = 0x83262888;
	sub_8222CED0(ctx, base);
	// 83262888: 3D00820B  lis r8, -0x7df5
	ctx.r[8].s64 = -2113208320;
	// 8326288C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83262890: 38888914  addi r4, r8, -0x76ec
	ctx.r[4].s64 = ctx.r[8].s64 + -30444;
	// 83262894: 387F0028  addi r3, r31, 0x28
	ctx.r[3].s64 = ctx.r[31].s64 + 40;
	// 83262898: 4AFCA639  bl 0x8222ced0
	ctx.lr = 0x8326289C;
	sub_8222CED0(ctx, base);
	// 8326289C: 3CE0820B  lis r7, -0x7df5
	ctx.r[7].s64 = -2113208320;
	// 832628A0: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832628A4: 388788F0  addi r4, r7, -0x7710
	ctx.r[4].s64 = ctx.r[7].s64 + -30480;
	// 832628A8: 387F002C  addi r3, r31, 0x2c
	ctx.r[3].s64 = ctx.r[31].s64 + 44;
	// 832628AC: 4AFCA625  bl 0x8222ced0
	ctx.lr = 0x832628B0;
	sub_8222CED0(ctx, base);
	// 832628B0: 3CC0820B  lis r6, -0x7df5
	ctx.r[6].s64 = -2113208320;
	// 832628B4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832628B8: 388688CC  addi r4, r6, -0x7734
	ctx.r[4].s64 = ctx.r[6].s64 + -30516;
	// 832628BC: 387F0030  addi r3, r31, 0x30
	ctx.r[3].s64 = ctx.r[31].s64 + 48;
	// 832628C0: 4AFCA611  bl 0x8222ced0
	ctx.lr = 0x832628C4;
	sub_8222CED0(ctx, base);
	// 832628C4: 3C80820B  lis r4, -0x7df5
	ctx.r[4].s64 = -2113208320;
	// 832628C8: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832628CC: 388488AC  addi r4, r4, -0x7754
	ctx.r[4].s64 = ctx.r[4].s64 + -30548;
	// 832628D0: 387F0034  addi r3, r31, 0x34
	ctx.r[3].s64 = ctx.r[31].s64 + 52;
	// 832628D4: 4AFCA5FD  bl 0x8222ced0
	ctx.lr = 0x832628D8;
	sub_8222CED0(ctx, base);
	// 832628D8: 3C60820B  lis r3, -0x7df5
	ctx.r[3].s64 = -2113208320;
	// 832628DC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832628E0: 3883888C  addi r4, r3, -0x7774
	ctx.r[4].s64 = ctx.r[3].s64 + -30580;
	// 832628E4: 387F0038  addi r3, r31, 0x38
	ctx.r[3].s64 = ctx.r[31].s64 + 56;
	// 832628E8: 4AFCA5E9  bl 0x8222ced0
	ctx.lr = 0x832628EC;
	sub_8222CED0(ctx, base);
	// 832628EC: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 832628F0: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832628F4: 388B8868  addi r4, r11, -0x7798
	ctx.r[4].s64 = ctx.r[11].s64 + -30616;
	// 832628F8: 387F003C  addi r3, r31, 0x3c
	ctx.r[3].s64 = ctx.r[31].s64 + 60;
	// 832628FC: 4AFCA5D5  bl 0x8222ced0
	ctx.lr = 0x83262900;
	sub_8222CED0(ctx, base);
	// 83262900: 3D40832B  lis r10, -0x7cd5
	ctx.r[10].s64 = -2094333952;
	// 83262904: 386ACBF0  addi r3, r10, -0x3410
	ctx.r[3].s64 = ctx.r[10].s64 + -13328;
	// 83262908: 4BA47619  bl 0x82ca9f20
	ctx.lr = 0x8326290C;
	sub_82CA9F20(ctx, base);
	// 8326290C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83262910: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83262914: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83262918: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8326291C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83262920(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83262920 size=376
    let mut pc: u32 = 0x83262920;
    'dispatch: loop {
        match pc {
            0x83262920 => {
    //   block [0x83262920..0x83262A98)
	// 83262920: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83262924: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83262928: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8326292C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83262930: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 83262934: 3D40820B  lis r10, -0x7df5
	ctx.r[10].s64 = -2113208320;
	// 83262938: 3BEBB3C0  addi r31, r11, -0x4c40
	ctx.r[31].s64 = ctx.r[11].s64 + -19520;
	// 8326293C: 388A8C58  addi r4, r10, -0x73a8
	ctx.r[4].s64 = ctx.r[10].s64 + -29608;
	// 83262940: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83262944: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83262948: 4AFCA589  bl 0x8222ced0
	ctx.lr = 0x8326294C;
	sub_8222CED0(ctx, base);
	// 8326294C: 3D20820B  lis r9, -0x7df5
	ctx.r[9].s64 = -2113208320;
	// 83262950: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83262954: 38898C38  addi r4, r9, -0x73c8
	ctx.r[4].s64 = ctx.r[9].s64 + -29640;
	// 83262958: 387F0004  addi r3, r31, 4
	ctx.r[3].s64 = ctx.r[31].s64 + 4;
	// 8326295C: 4AFCA575  bl 0x8222ced0
	ctx.lr = 0x83262960;
	sub_8222CED0(ctx, base);
	// 83262960: 3D00820B  lis r8, -0x7df5
	ctx.r[8].s64 = -2113208320;
	// 83262964: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83262968: 38888C18  addi r4, r8, -0x73e8
	ctx.r[4].s64 = ctx.r[8].s64 + -29672;
	// 8326296C: 387F0008  addi r3, r31, 8
	ctx.r[3].s64 = ctx.r[31].s64 + 8;
	// 83262970: 4AFCA561  bl 0x8222ced0
	ctx.lr = 0x83262974;
	sub_8222CED0(ctx, base);
	// 83262974: 3CE0820B  lis r7, -0x7df5
	ctx.r[7].s64 = -2113208320;
	// 83262978: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8326297C: 38878BF8  addi r4, r7, -0x7408
	ctx.r[4].s64 = ctx.r[7].s64 + -29704;
	// 83262980: 387F000C  addi r3, r31, 0xc
	ctx.r[3].s64 = ctx.r[31].s64 + 12;
	// 83262984: 4AFCA54D  bl 0x8222ced0
	ctx.lr = 0x83262988;
	sub_8222CED0(ctx, base);
	// 83262988: 3CC0820B  lis r6, -0x7df5
	ctx.r[6].s64 = -2113208320;
	// 8326298C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83262990: 38868BD8  addi r4, r6, -0x7428
	ctx.r[4].s64 = ctx.r[6].s64 + -29736;
	// 83262994: 387F0010  addi r3, r31, 0x10
	ctx.r[3].s64 = ctx.r[31].s64 + 16;
	// 83262998: 4AFCA539  bl 0x8222ced0
	ctx.lr = 0x8326299C;
	sub_8222CED0(ctx, base);
	// 8326299C: 3C80820B  lis r4, -0x7df5
	ctx.r[4].s64 = -2113208320;
	// 832629A0: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832629A4: 38848BB8  addi r4, r4, -0x7448
	ctx.r[4].s64 = ctx.r[4].s64 + -29768;
	// 832629A8: 387F0014  addi r3, r31, 0x14
	ctx.r[3].s64 = ctx.r[31].s64 + 20;
	// 832629AC: 4AFCA525  bl 0x8222ced0
	ctx.lr = 0x832629B0;
	sub_8222CED0(ctx, base);
	// 832629B0: 3C60820B  lis r3, -0x7df5
	ctx.r[3].s64 = -2113208320;
	// 832629B4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832629B8: 38838B98  addi r4, r3, -0x7468
	ctx.r[4].s64 = ctx.r[3].s64 + -29800;
	// 832629BC: 387F0018  addi r3, r31, 0x18
	ctx.r[3].s64 = ctx.r[31].s64 + 24;
	// 832629C0: 4AFCA511  bl 0x8222ced0
	ctx.lr = 0x832629C4;
	sub_8222CED0(ctx, base);
	// 832629C4: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 832629C8: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832629CC: 388B8B78  addi r4, r11, -0x7488
	ctx.r[4].s64 = ctx.r[11].s64 + -29832;
	// 832629D0: 387F001C  addi r3, r31, 0x1c
	ctx.r[3].s64 = ctx.r[31].s64 + 28;
	// 832629D4: 4AFCA4FD  bl 0x8222ced0
	ctx.lr = 0x832629D8;
	sub_8222CED0(ctx, base);
	// 832629D8: 3D40820B  lis r10, -0x7df5
	ctx.r[10].s64 = -2113208320;
	// 832629DC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832629E0: 388A8B58  addi r4, r10, -0x74a8
	ctx.r[4].s64 = ctx.r[10].s64 + -29864;
	// 832629E4: 387F0020  addi r3, r31, 0x20
	ctx.r[3].s64 = ctx.r[31].s64 + 32;
	// 832629E8: 4AFCA4E9  bl 0x8222ced0
	ctx.lr = 0x832629EC;
	sub_8222CED0(ctx, base);
	// 832629EC: 3D20820B  lis r9, -0x7df5
	ctx.r[9].s64 = -2113208320;
	// 832629F0: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832629F4: 38898B38  addi r4, r9, -0x74c8
	ctx.r[4].s64 = ctx.r[9].s64 + -29896;
	// 832629F8: 387F0024  addi r3, r31, 0x24
	ctx.r[3].s64 = ctx.r[31].s64 + 36;
	// 832629FC: 4AFCA4D5  bl 0x8222ced0
	ctx.lr = 0x83262A00;
	sub_8222CED0(ctx, base);
	// 83262A00: 3D00820B  lis r8, -0x7df5
	ctx.r[8].s64 = -2113208320;
	// 83262A04: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83262A08: 38888B1C  addi r4, r8, -0x74e4
	ctx.r[4].s64 = ctx.r[8].s64 + -29924;
	// 83262A0C: 387F0028  addi r3, r31, 0x28
	ctx.r[3].s64 = ctx.r[31].s64 + 40;
	// 83262A10: 4AFCA4C1  bl 0x8222ced0
	ctx.lr = 0x83262A14;
	sub_8222CED0(ctx, base);
	// 83262A14: 3CE0820B  lis r7, -0x7df5
	ctx.r[7].s64 = -2113208320;
	// 83262A18: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83262A1C: 38878AF8  addi r4, r7, -0x7508
	ctx.r[4].s64 = ctx.r[7].s64 + -29960;
	// 83262A20: 387F002C  addi r3, r31, 0x2c
	ctx.r[3].s64 = ctx.r[31].s64 + 44;
	// 83262A24: 4AFCA4AD  bl 0x8222ced0
	ctx.lr = 0x83262A28;
	sub_8222CED0(ctx, base);
	// 83262A28: 3CC0820B  lis r6, -0x7df5
	ctx.r[6].s64 = -2113208320;
	// 83262A2C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83262A30: 38868AD4  addi r4, r6, -0x752c
	ctx.r[4].s64 = ctx.r[6].s64 + -29996;
	// 83262A34: 387F0030  addi r3, r31, 0x30
	ctx.r[3].s64 = ctx.r[31].s64 + 48;
	// 83262A38: 4AFCA499  bl 0x8222ced0
	ctx.lr = 0x83262A3C;
	sub_8222CED0(ctx, base);
	// 83262A3C: 3C80820B  lis r4, -0x7df5
	ctx.r[4].s64 = -2113208320;
	// 83262A40: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83262A44: 38848AB4  addi r4, r4, -0x754c
	ctx.r[4].s64 = ctx.r[4].s64 + -30028;
	// 83262A48: 387F0034  addi r3, r31, 0x34
	ctx.r[3].s64 = ctx.r[31].s64 + 52;
	// 83262A4C: 4AFCA485  bl 0x8222ced0
	ctx.lr = 0x83262A50;
	sub_8222CED0(ctx, base);
	// 83262A50: 3C60820B  lis r3, -0x7df5
	ctx.r[3].s64 = -2113208320;
	// 83262A54: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83262A58: 38838A94  addi r4, r3, -0x756c
	ctx.r[4].s64 = ctx.r[3].s64 + -30060;
	// 83262A5C: 387F0038  addi r3, r31, 0x38
	ctx.r[3].s64 = ctx.r[31].s64 + 56;
	// 83262A60: 4AFCA471  bl 0x8222ced0
	ctx.lr = 0x83262A64;
	sub_8222CED0(ctx, base);
	// 83262A64: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 83262A68: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83262A6C: 388B8A70  addi r4, r11, -0x7590
	ctx.r[4].s64 = ctx.r[11].s64 + -30096;
	// 83262A70: 387F003C  addi r3, r31, 0x3c
	ctx.r[3].s64 = ctx.r[31].s64 + 60;
	// 83262A74: 4AFCA45D  bl 0x8222ced0
	ctx.lr = 0x83262A78;
	sub_8222CED0(ctx, base);
	// 83262A78: 3D40832B  lis r10, -0x7cd5
	ctx.r[10].s64 = -2094333952;
	// 83262A7C: 386ACC58  addi r3, r10, -0x33a8
	ctx.r[3].s64 = ctx.r[10].s64 + -13224;
	// 83262A80: 4BA474A1  bl 0x82ca9f20
	ctx.lr = 0x83262A84;
	sub_82CA9F20(ctx, base);
	// 83262A84: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83262A88: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83262A8C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83262A90: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83262A94: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83262A98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83262A98 size=56
    let mut pc: u32 = 0x83262A98;
    'dispatch: loop {
        match pc {
            0x83262A98 => {
    //   block [0x83262A98..0x83262AD0)
	// 83262A98: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83262A9C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83262AA0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83262AA4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83262AA8: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83262AAC: 386B42C8  addi r3, r11, 0x42c8
	ctx.r[3].s64 = ctx.r[11].s64 + 17096;
	// 83262AB0: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83262AB4: 4AF912A5  bl 0x821f3d58
	ctx.lr = 0x83262AB8;
	sub_821F3D58(ctx, base);
	// 83262AB8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83262ABC: 906AB400  stw r3, -0x4c00(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-19456 as u32), ctx.r[3].u32 ) };
	// 83262AC0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83262AC4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83262AC8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83262ACC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83262AD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83262AD0 size=56
    let mut pc: u32 = 0x83262AD0;
    'dispatch: loop {
        match pc {
            0x83262AD0 => {
    //   block [0x83262AD0..0x83262B08)
	// 83262AD0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83262AD4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83262AD8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83262ADC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83262AE0: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83262AE4: 386B42DC  addi r3, r11, 0x42dc
	ctx.r[3].s64 = ctx.r[11].s64 + 17116;
	// 83262AE8: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83262AEC: 4AF9126D  bl 0x821f3d58
	ctx.lr = 0x83262AF0;
	sub_821F3D58(ctx, base);
	// 83262AF0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83262AF4: 906AB404  stw r3, -0x4bfc(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-19452 as u32), ctx.r[3].u32 ) };
	// 83262AF8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83262AFC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83262B00: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83262B04: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83262B08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83262B08 size=56
    let mut pc: u32 = 0x83262B08;
    'dispatch: loop {
        match pc {
            0x83262B08 => {
    //   block [0x83262B08..0x83262B40)
	// 83262B08: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83262B0C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83262B10: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83262B14: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83262B18: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83262B1C: 386B42E8  addi r3, r11, 0x42e8
	ctx.r[3].s64 = ctx.r[11].s64 + 17128;
	// 83262B20: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83262B24: 4AF91235  bl 0x821f3d58
	ctx.lr = 0x83262B28;
	sub_821F3D58(ctx, base);
	// 83262B28: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83262B2C: 906AB408  stw r3, -0x4bf8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-19448 as u32), ctx.r[3].u32 ) };
	// 83262B30: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83262B34: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83262B38: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83262B3C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83262B40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83262B40 size=56
    let mut pc: u32 = 0x83262B40;
    'dispatch: loop {
        match pc {
            0x83262B40 => {
    //   block [0x83262B40..0x83262B78)
	// 83262B40: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83262B44: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83262B48: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83262B4C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83262B50: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83262B54: 386B4300  addi r3, r11, 0x4300
	ctx.r[3].s64 = ctx.r[11].s64 + 17152;
	// 83262B58: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83262B5C: 4AF911FD  bl 0x821f3d58
	ctx.lr = 0x83262B60;
	sub_821F3D58(ctx, base);
	// 83262B60: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83262B64: 906AB40C  stw r3, -0x4bf4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-19444 as u32), ctx.r[3].u32 ) };
	// 83262B68: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83262B6C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83262B70: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83262B74: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83262B78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83262B78 size=56
    let mut pc: u32 = 0x83262B78;
    'dispatch: loop {
        match pc {
            0x83262B78 => {
    //   block [0x83262B78..0x83262BB0)
	// 83262B78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83262B7C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83262B80: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83262B84: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83262B88: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83262B8C: 386B4310  addi r3, r11, 0x4310
	ctx.r[3].s64 = ctx.r[11].s64 + 17168;
	// 83262B90: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83262B94: 4AF911C5  bl 0x821f3d58
	ctx.lr = 0x83262B98;
	sub_821F3D58(ctx, base);
	// 83262B98: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83262B9C: 906AB410  stw r3, -0x4bf0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-19440 as u32), ctx.r[3].u32 ) };
	// 83262BA0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83262BA4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83262BA8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83262BAC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83262BB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83262BB0 size=56
    let mut pc: u32 = 0x83262BB0;
    'dispatch: loop {
        match pc {
            0x83262BB0 => {
    //   block [0x83262BB0..0x83262BE8)
	// 83262BB0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83262BB4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83262BB8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83262BBC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83262BC0: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83262BC4: 386B4328  addi r3, r11, 0x4328
	ctx.r[3].s64 = ctx.r[11].s64 + 17192;
	// 83262BC8: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83262BCC: 4AF9118D  bl 0x821f3d58
	ctx.lr = 0x83262BD0;
	sub_821F3D58(ctx, base);
	// 83262BD0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83262BD4: 906AB414  stw r3, -0x4bec(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-19436 as u32), ctx.r[3].u32 ) };
	// 83262BD8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83262BDC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83262BE0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83262BE4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83262BE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83262BE8 size=56
    let mut pc: u32 = 0x83262BE8;
    'dispatch: loop {
        match pc {
            0x83262BE8 => {
    //   block [0x83262BE8..0x83262C20)
	// 83262BE8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83262BEC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83262BF0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83262BF4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83262BF8: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83262BFC: 386B4338  addi r3, r11, 0x4338
	ctx.r[3].s64 = ctx.r[11].s64 + 17208;
	// 83262C00: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83262C04: 4AF91155  bl 0x821f3d58
	ctx.lr = 0x83262C08;
	sub_821F3D58(ctx, base);
	// 83262C08: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83262C0C: 906AB418  stw r3, -0x4be8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-19432 as u32), ctx.r[3].u32 ) };
	// 83262C10: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83262C14: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83262C18: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83262C1C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83262C20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83262C20 size=56
    let mut pc: u32 = 0x83262C20;
    'dispatch: loop {
        match pc {
            0x83262C20 => {
    //   block [0x83262C20..0x83262C58)
	// 83262C20: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83262C24: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83262C28: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83262C2C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83262C30: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83262C34: 386B4350  addi r3, r11, 0x4350
	ctx.r[3].s64 = ctx.r[11].s64 + 17232;
	// 83262C38: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83262C3C: 4AF9111D  bl 0x821f3d58
	ctx.lr = 0x83262C40;
	sub_821F3D58(ctx, base);
	// 83262C40: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83262C44: 906AB41C  stw r3, -0x4be4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-19428 as u32), ctx.r[3].u32 ) };
	// 83262C48: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83262C4C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83262C50: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83262C54: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83262C58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83262C58 size=56
    let mut pc: u32 = 0x83262C58;
    'dispatch: loop {
        match pc {
            0x83262C58 => {
    //   block [0x83262C58..0x83262C90)
	// 83262C58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83262C5C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83262C60: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83262C64: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83262C68: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83262C6C: 386B4368  addi r3, r11, 0x4368
	ctx.r[3].s64 = ctx.r[11].s64 + 17256;
	// 83262C70: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83262C74: 4AF910E5  bl 0x821f3d58
	ctx.lr = 0x83262C78;
	sub_821F3D58(ctx, base);
	// 83262C78: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83262C7C: 906AB420  stw r3, -0x4be0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-19424 as u32), ctx.r[3].u32 ) };
	// 83262C80: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83262C84: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83262C88: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83262C8C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83262C90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83262C90 size=56
    let mut pc: u32 = 0x83262C90;
    'dispatch: loop {
        match pc {
            0x83262C90 => {
    //   block [0x83262C90..0x83262CC8)
	// 83262C90: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83262C94: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83262C98: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83262C9C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83262CA0: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83262CA4: 386B4380  addi r3, r11, 0x4380
	ctx.r[3].s64 = ctx.r[11].s64 + 17280;
	// 83262CA8: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83262CAC: 4AF910AD  bl 0x821f3d58
	ctx.lr = 0x83262CB0;
	sub_821F3D58(ctx, base);
	// 83262CB0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83262CB4: 906AB424  stw r3, -0x4bdc(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-19420 as u32), ctx.r[3].u32 ) };
	// 83262CB8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83262CBC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83262CC0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83262CC4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83262CC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83262CC8 size=56
    let mut pc: u32 = 0x83262CC8;
    'dispatch: loop {
        match pc {
            0x83262CC8 => {
    //   block [0x83262CC8..0x83262D00)
	// 83262CC8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83262CCC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83262CD0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83262CD4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83262CD8: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83262CDC: 386B4394  addi r3, r11, 0x4394
	ctx.r[3].s64 = ctx.r[11].s64 + 17300;
	// 83262CE0: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83262CE4: 4AF91075  bl 0x821f3d58
	ctx.lr = 0x83262CE8;
	sub_821F3D58(ctx, base);
	// 83262CE8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83262CEC: 906AB428  stw r3, -0x4bd8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-19416 as u32), ctx.r[3].u32 ) };
	// 83262CF0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83262CF4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83262CF8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83262CFC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83262D00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83262D00 size=56
    let mut pc: u32 = 0x83262D00;
    'dispatch: loop {
        match pc {
            0x83262D00 => {
    //   block [0x83262D00..0x83262D38)
	// 83262D00: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83262D04: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83262D08: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83262D0C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83262D10: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83262D14: 386B43B4  addi r3, r11, 0x43b4
	ctx.r[3].s64 = ctx.r[11].s64 + 17332;
	// 83262D18: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83262D1C: 4AF9103D  bl 0x821f3d58
	ctx.lr = 0x83262D20;
	sub_821F3D58(ctx, base);
	// 83262D20: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83262D24: 906AB42C  stw r3, -0x4bd4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-19412 as u32), ctx.r[3].u32 ) };
	// 83262D28: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83262D2C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83262D30: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83262D34: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83262D38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83262D38 size=56
    let mut pc: u32 = 0x83262D38;
    'dispatch: loop {
        match pc {
            0x83262D38 => {
    //   block [0x83262D38..0x83262D70)
	// 83262D38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83262D3C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83262D40: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83262D44: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83262D48: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83262D4C: 386B43C8  addi r3, r11, 0x43c8
	ctx.r[3].s64 = ctx.r[11].s64 + 17352;
	// 83262D50: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83262D54: 4AF91005  bl 0x821f3d58
	ctx.lr = 0x83262D58;
	sub_821F3D58(ctx, base);
	// 83262D58: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83262D5C: 906AB430  stw r3, -0x4bd0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-19408 as u32), ctx.r[3].u32 ) };
	// 83262D60: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83262D64: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83262D68: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83262D6C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83262D70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83262D70 size=56
    let mut pc: u32 = 0x83262D70;
    'dispatch: loop {
        match pc {
            0x83262D70 => {
    //   block [0x83262D70..0x83262DA8)
	// 83262D70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83262D74: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83262D78: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83262D7C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83262D80: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83262D84: 386B43D4  addi r3, r11, 0x43d4
	ctx.r[3].s64 = ctx.r[11].s64 + 17364;
	// 83262D88: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83262D8C: 4AF90FCD  bl 0x821f3d58
	ctx.lr = 0x83262D90;
	sub_821F3D58(ctx, base);
	// 83262D90: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83262D94: 906AB434  stw r3, -0x4bcc(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-19404 as u32), ctx.r[3].u32 ) };
	// 83262D98: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83262D9C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83262DA0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83262DA4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83262DA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83262DA8 size=56
    let mut pc: u32 = 0x83262DA8;
    'dispatch: loop {
        match pc {
            0x83262DA8 => {
    //   block [0x83262DA8..0x83262DE0)
	// 83262DA8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83262DAC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83262DB0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83262DB4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83262DB8: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83262DBC: 386B43E0  addi r3, r11, 0x43e0
	ctx.r[3].s64 = ctx.r[11].s64 + 17376;
	// 83262DC0: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83262DC4: 4AF90F95  bl 0x821f3d58
	ctx.lr = 0x83262DC8;
	sub_821F3D58(ctx, base);
	// 83262DC8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83262DCC: 906AB438  stw r3, -0x4bc8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-19400 as u32), ctx.r[3].u32 ) };
	// 83262DD0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83262DD4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83262DD8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83262DDC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83262DE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83262DE0 size=56
    let mut pc: u32 = 0x83262DE0;
    'dispatch: loop {
        match pc {
            0x83262DE0 => {
    //   block [0x83262DE0..0x83262E18)
	// 83262DE0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83262DE4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83262DE8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83262DEC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83262DF0: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83262DF4: 386B43F0  addi r3, r11, 0x43f0
	ctx.r[3].s64 = ctx.r[11].s64 + 17392;
	// 83262DF8: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83262DFC: 4AF90F5D  bl 0x821f3d58
	ctx.lr = 0x83262E00;
	sub_821F3D58(ctx, base);
	// 83262E00: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83262E04: 906AB43C  stw r3, -0x4bc4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-19396 as u32), ctx.r[3].u32 ) };
	// 83262E08: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83262E0C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83262E10: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83262E14: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83262E18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83262E18 size=56
    let mut pc: u32 = 0x83262E18;
    'dispatch: loop {
        match pc {
            0x83262E18 => {
    //   block [0x83262E18..0x83262E50)
	// 83262E18: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83262E1C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83262E20: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83262E24: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83262E28: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83262E2C: 386B4400  addi r3, r11, 0x4400
	ctx.r[3].s64 = ctx.r[11].s64 + 17408;
	// 83262E30: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83262E34: 4AF90F25  bl 0x821f3d58
	ctx.lr = 0x83262E38;
	sub_821F3D58(ctx, base);
	// 83262E38: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83262E3C: 906AB440  stw r3, -0x4bc0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-19392 as u32), ctx.r[3].u32 ) };
	// 83262E40: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83262E44: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83262E48: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83262E4C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83262E50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83262E50 size=56
    let mut pc: u32 = 0x83262E50;
    'dispatch: loop {
        match pc {
            0x83262E50 => {
    //   block [0x83262E50..0x83262E88)
	// 83262E50: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83262E54: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83262E58: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83262E5C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83262E60: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83262E64: 386B4418  addi r3, r11, 0x4418
	ctx.r[3].s64 = ctx.r[11].s64 + 17432;
	// 83262E68: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83262E6C: 4AF90EED  bl 0x821f3d58
	ctx.lr = 0x83262E70;
	sub_821F3D58(ctx, base);
	// 83262E70: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83262E74: 906AB444  stw r3, -0x4bbc(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-19388 as u32), ctx.r[3].u32 ) };
	// 83262E78: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83262E7C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83262E80: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83262E84: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83262E88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83262E88 size=56
    let mut pc: u32 = 0x83262E88;
    'dispatch: loop {
        match pc {
            0x83262E88 => {
    //   block [0x83262E88..0x83262EC0)
	// 83262E88: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83262E8C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83262E90: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83262E94: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83262E98: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83262E9C: 386B4424  addi r3, r11, 0x4424
	ctx.r[3].s64 = ctx.r[11].s64 + 17444;
	// 83262EA0: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83262EA4: 4AF90EB5  bl 0x821f3d58
	ctx.lr = 0x83262EA8;
	sub_821F3D58(ctx, base);
	// 83262EA8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83262EAC: 906AB448  stw r3, -0x4bb8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-19384 as u32), ctx.r[3].u32 ) };
	// 83262EB0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83262EB4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83262EB8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83262EBC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83262EC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83262EC0 size=56
    let mut pc: u32 = 0x83262EC0;
    'dispatch: loop {
        match pc {
            0x83262EC0 => {
    //   block [0x83262EC0..0x83262EF8)
	// 83262EC0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83262EC4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83262EC8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83262ECC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83262ED0: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83262ED4: 386B4430  addi r3, r11, 0x4430
	ctx.r[3].s64 = ctx.r[11].s64 + 17456;
	// 83262ED8: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83262EDC: 4AF90E7D  bl 0x821f3d58
	ctx.lr = 0x83262EE0;
	sub_821F3D58(ctx, base);
	// 83262EE0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83262EE4: 906AB44C  stw r3, -0x4bb4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-19380 as u32), ctx.r[3].u32 ) };
	// 83262EE8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83262EEC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83262EF0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83262EF4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83262EF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83262EF8 size=56
    let mut pc: u32 = 0x83262EF8;
    'dispatch: loop {
        match pc {
            0x83262EF8 => {
    //   block [0x83262EF8..0x83262F30)
	// 83262EF8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83262EFC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83262F00: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83262F04: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83262F08: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83262F0C: 386B4440  addi r3, r11, 0x4440
	ctx.r[3].s64 = ctx.r[11].s64 + 17472;
	// 83262F10: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83262F14: 4AF90E45  bl 0x821f3d58
	ctx.lr = 0x83262F18;
	sub_821F3D58(ctx, base);
	// 83262F18: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83262F1C: 906AB450  stw r3, -0x4bb0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-19376 as u32), ctx.r[3].u32 ) };
	// 83262F20: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83262F24: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83262F28: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83262F2C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83262F30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83262F30 size=64
    let mut pc: u32 = 0x83262F30;
    'dispatch: loop {
        match pc {
            0x83262F30 => {
    //   block [0x83262F30..0x83262F70)
	// 83262F30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83262F34: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83262F38: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83262F3C: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 83262F40: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83262F44: 388BDED0  addi r4, r11, -0x2130
	ctx.r[4].s64 = ctx.r[11].s64 + -8496;
	// 83262F48: 386AB454  addi r3, r10, -0x4bac
	ctx.r[3].s64 = ctx.r[10].s64 + -19372;
	// 83262F4C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83262F50: 4AFC9F81  bl 0x8222ced0
	ctx.lr = 0x83262F54;
	sub_8222CED0(ctx, base);
	// 83262F54: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83262F58: 3869CCC0  addi r3, r9, -0x3340
	ctx.r[3].s64 = ctx.r[9].s64 + -13120;
	// 83262F5C: 4BA46FC5  bl 0x82ca9f20
	ctx.lr = 0x83262F60;
	sub_82CA9F20(ctx, base);
	// 83262F60: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83262F64: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83262F68: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83262F6C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83262F70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83262F70 size=64
    let mut pc: u32 = 0x83262F70;
    'dispatch: loop {
        match pc {
            0x83262F70 => {
    //   block [0x83262F70..0x83262FB0)
	// 83262F70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83262F74: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83262F78: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83262F7C: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 83262F80: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83262F84: 388BCE9C  addi r4, r11, -0x3164
	ctx.r[4].s64 = ctx.r[11].s64 + -12644;
	// 83262F88: 386AB458  addi r3, r10, -0x4ba8
	ctx.r[3].s64 = ctx.r[10].s64 + -19368;
	// 83262F8C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83262F90: 4AFC9F41  bl 0x8222ced0
	ctx.lr = 0x83262F94;
	sub_8222CED0(ctx, base);
	// 83262F94: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83262F98: 3869CCD0  addi r3, r9, -0x3330
	ctx.r[3].s64 = ctx.r[9].s64 + -13104;
	// 83262F9C: 4BA46F85  bl 0x82ca9f20
	ctx.lr = 0x83262FA0;
	sub_82CA9F20(ctx, base);
	// 83262FA0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83262FA4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83262FA8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83262FAC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83262FB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83262FB0 size=64
    let mut pc: u32 = 0x83262FB0;
    'dispatch: loop {
        match pc {
            0x83262FB0 => {
    //   block [0x83262FB0..0x83262FF0)
	// 83262FB0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83262FB4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83262FB8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83262FBC: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 83262FC0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83262FC4: 388BFF7C  addi r4, r11, -0x84
	ctx.r[4].s64 = ctx.r[11].s64 + -132;
	// 83262FC8: 386AB45C  addi r3, r10, -0x4ba4
	ctx.r[3].s64 = ctx.r[10].s64 + -19364;
	// 83262FCC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83262FD0: 4AFC9F01  bl 0x8222ced0
	ctx.lr = 0x83262FD4;
	sub_8222CED0(ctx, base);
	// 83262FD4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83262FD8: 3869CCE0  addi r3, r9, -0x3320
	ctx.r[3].s64 = ctx.r[9].s64 + -13088;
	// 83262FDC: 4BA46F45  bl 0x82ca9f20
	ctx.lr = 0x83262FE0;
	sub_82CA9F20(ctx, base);
	// 83262FE0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83262FE4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83262FE8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83262FEC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83262FF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83262FF0 size=64
    let mut pc: u32 = 0x83262FF0;
    'dispatch: loop {
        match pc {
            0x83262FF0 => {
    //   block [0x83262FF0..0x83263030)
	// 83262FF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83262FF4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83262FF8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83262FFC: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 83263000: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83263004: 388BFFA0  addi r4, r11, -0x60
	ctx.r[4].s64 = ctx.r[11].s64 + -96;
	// 83263008: 386AB460  addi r3, r10, -0x4ba0
	ctx.r[3].s64 = ctx.r[10].s64 + -19360;
	// 8326300C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83263010: 4AFC9EC1  bl 0x8222ced0
	ctx.lr = 0x83263014;
	sub_8222CED0(ctx, base);
	// 83263014: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83263018: 3869CCF0  addi r3, r9, -0x3310
	ctx.r[3].s64 = ctx.r[9].s64 + -13072;
	// 8326301C: 4BA46F05  bl 0x82ca9f20
	ctx.lr = 0x83263020;
	sub_82CA9F20(ctx, base);
	// 83263020: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83263024: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83263028: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326302C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83263030(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83263030 size=64
    let mut pc: u32 = 0x83263030;
    'dispatch: loop {
        match pc {
            0x83263030 => {
    //   block [0x83263030..0x83263070)
	// 83263030: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83263034: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83263038: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326303C: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 83263040: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83263044: 388BFFC4  addi r4, r11, -0x3c
	ctx.r[4].s64 = ctx.r[11].s64 + -60;
	// 83263048: 386AB464  addi r3, r10, -0x4b9c
	ctx.r[3].s64 = ctx.r[10].s64 + -19356;
	// 8326304C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83263050: 4AFC9E81  bl 0x8222ced0
	ctx.lr = 0x83263054;
	sub_8222CED0(ctx, base);
	// 83263054: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83263058: 3869CD00  addi r3, r9, -0x3300
	ctx.r[3].s64 = ctx.r[9].s64 + -13056;
	// 8326305C: 4BA46EC5  bl 0x82ca9f20
	ctx.lr = 0x83263060;
	sub_82CA9F20(ctx, base);
	// 83263060: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83263064: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83263068: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326306C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83263070(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83263070 size=64
    let mut pc: u32 = 0x83263070;
    'dispatch: loop {
        match pc {
            0x83263070 => {
    //   block [0x83263070..0x832630B0)
	// 83263070: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83263074: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83263078: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326307C: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 83263080: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83263084: 388BFFF0  addi r4, r11, -0x10
	ctx.r[4].s64 = ctx.r[11].s64 + -16;
	// 83263088: 386AB468  addi r3, r10, -0x4b98
	ctx.r[3].s64 = ctx.r[10].s64 + -19352;
	// 8326308C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83263090: 4AFC9E41  bl 0x8222ced0
	ctx.lr = 0x83263094;
	sub_8222CED0(ctx, base);
	// 83263094: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83263098: 3869CD10  addi r3, r9, -0x32f0
	ctx.r[3].s64 = ctx.r[9].s64 + -13040;
	// 8326309C: 4BA46E85  bl 0x82ca9f20
	ctx.lr = 0x832630A0;
	sub_82CA9F20(ctx, base);
	// 832630A0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832630A4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832630A8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832630AC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832630B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832630B0 size=64
    let mut pc: u32 = 0x832630B0;
    'dispatch: loop {
        match pc {
            0x832630B0 => {
    //   block [0x832630B0..0x832630F0)
	// 832630B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832630B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832630B8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832630BC: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 832630C0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832630C4: 388B0010  addi r4, r11, 0x10
	ctx.r[4].s64 = ctx.r[11].s64 + 16;
	// 832630C8: 386AB46C  addi r3, r10, -0x4b94
	ctx.r[3].s64 = ctx.r[10].s64 + -19348;
	// 832630CC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832630D0: 4AFC9E01  bl 0x8222ced0
	ctx.lr = 0x832630D4;
	sub_8222CED0(ctx, base);
	// 832630D4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 832630D8: 3869CD20  addi r3, r9, -0x32e0
	ctx.r[3].s64 = ctx.r[9].s64 + -13024;
	// 832630DC: 4BA46E45  bl 0x82ca9f20
	ctx.lr = 0x832630E0;
	sub_82CA9F20(ctx, base);
	// 832630E0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832630E4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832630E8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832630EC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832630F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832630F0 size=64
    let mut pc: u32 = 0x832630F0;
    'dispatch: loop {
        match pc {
            0x832630F0 => {
    //   block [0x832630F0..0x83263130)
	// 832630F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832630F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832630F8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832630FC: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 83263100: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83263104: 388B0030  addi r4, r11, 0x30
	ctx.r[4].s64 = ctx.r[11].s64 + 48;
	// 83263108: 386AB470  addi r3, r10, -0x4b90
	ctx.r[3].s64 = ctx.r[10].s64 + -19344;
	// 8326310C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83263110: 4AFC9DC1  bl 0x8222ced0
	ctx.lr = 0x83263114;
	sub_8222CED0(ctx, base);
	// 83263114: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83263118: 3869CD30  addi r3, r9, -0x32d0
	ctx.r[3].s64 = ctx.r[9].s64 + -13008;
	// 8326311C: 4BA46E05  bl 0x82ca9f20
	ctx.lr = 0x83263120;
	sub_82CA9F20(ctx, base);
	// 83263120: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83263124: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83263128: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326312C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83263130(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83263130 size=64
    let mut pc: u32 = 0x83263130;
    'dispatch: loop {
        match pc {
            0x83263130 => {
    //   block [0x83263130..0x83263170)
	// 83263130: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83263134: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83263138: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326313C: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 83263140: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83263144: 388B0058  addi r4, r11, 0x58
	ctx.r[4].s64 = ctx.r[11].s64 + 88;
	// 83263148: 386AB474  addi r3, r10, -0x4b8c
	ctx.r[3].s64 = ctx.r[10].s64 + -19340;
	// 8326314C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83263150: 4AFC9D81  bl 0x8222ced0
	ctx.lr = 0x83263154;
	sub_8222CED0(ctx, base);
	// 83263154: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83263158: 3869CD40  addi r3, r9, -0x32c0
	ctx.r[3].s64 = ctx.r[9].s64 + -12992;
	// 8326315C: 4BA46DC5  bl 0x82ca9f20
	ctx.lr = 0x83263160;
	sub_82CA9F20(ctx, base);
	// 83263160: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83263164: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83263168: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326316C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83263170(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83263170 size=64
    let mut pc: u32 = 0x83263170;
    'dispatch: loop {
        match pc {
            0x83263170 => {
    //   block [0x83263170..0x832631B0)
	// 83263170: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83263174: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83263178: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326317C: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 83263180: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83263184: 388B0084  addi r4, r11, 0x84
	ctx.r[4].s64 = ctx.r[11].s64 + 132;
	// 83263188: 386AB478  addi r3, r10, -0x4b88
	ctx.r[3].s64 = ctx.r[10].s64 + -19336;
	// 8326318C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83263190: 4AFC9D41  bl 0x8222ced0
	ctx.lr = 0x83263194;
	sub_8222CED0(ctx, base);
	// 83263194: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83263198: 3869CD50  addi r3, r9, -0x32b0
	ctx.r[3].s64 = ctx.r[9].s64 + -12976;
	// 8326319C: 4BA46D85  bl 0x82ca9f20
	ctx.lr = 0x832631A0;
	sub_82CA9F20(ctx, base);
	// 832631A0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832631A4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832631A8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832631AC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832631B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832631B0 size=64
    let mut pc: u32 = 0x832631B0;
    'dispatch: loop {
        match pc {
            0x832631B0 => {
    //   block [0x832631B0..0x832631F0)
	// 832631B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832631B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832631B8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832631BC: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 832631C0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832631C4: 388B00A4  addi r4, r11, 0xa4
	ctx.r[4].s64 = ctx.r[11].s64 + 164;
	// 832631C8: 386AB47C  addi r3, r10, -0x4b84
	ctx.r[3].s64 = ctx.r[10].s64 + -19332;
	// 832631CC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832631D0: 4AFC9D01  bl 0x8222ced0
	ctx.lr = 0x832631D4;
	sub_8222CED0(ctx, base);
	// 832631D4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 832631D8: 3869CD60  addi r3, r9, -0x32a0
	ctx.r[3].s64 = ctx.r[9].s64 + -12960;
	// 832631DC: 4BA46D45  bl 0x82ca9f20
	ctx.lr = 0x832631E0;
	sub_82CA9F20(ctx, base);
	// 832631E0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832631E4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832631E8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832631EC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832631F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832631F0 size=56
    let mut pc: u32 = 0x832631F0;
    'dispatch: loop {
        match pc {
            0x832631F0 => {
    //   block [0x832631F0..0x83263228)
	// 832631F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832631F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832631F8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832631FC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83263200: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83263204: 386B42C8  addi r3, r11, 0x42c8
	ctx.r[3].s64 = ctx.r[11].s64 + 17096;
	// 83263208: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8326320C: 4AF90B4D  bl 0x821f3d58
	ctx.lr = 0x83263210;
	sub_821F3D58(ctx, base);
	// 83263210: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83263214: 906AB480  stw r3, -0x4b80(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-19328 as u32), ctx.r[3].u32 ) };
	// 83263218: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326321C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83263220: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83263224: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83263228(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83263228 size=56
    let mut pc: u32 = 0x83263228;
    'dispatch: loop {
        match pc {
            0x83263228 => {
    //   block [0x83263228..0x83263260)
	// 83263228: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326322C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83263230: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83263234: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83263238: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8326323C: 386B42DC  addi r3, r11, 0x42dc
	ctx.r[3].s64 = ctx.r[11].s64 + 17116;
	// 83263240: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83263244: 4AF90B15  bl 0x821f3d58
	ctx.lr = 0x83263248;
	sub_821F3D58(ctx, base);
	// 83263248: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326324C: 906AB484  stw r3, -0x4b7c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-19324 as u32), ctx.r[3].u32 ) };
	// 83263250: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83263254: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83263258: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326325C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83263260(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83263260 size=56
    let mut pc: u32 = 0x83263260;
    'dispatch: loop {
        match pc {
            0x83263260 => {
    //   block [0x83263260..0x83263298)
	// 83263260: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83263264: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83263268: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326326C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83263270: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83263274: 386B42E8  addi r3, r11, 0x42e8
	ctx.r[3].s64 = ctx.r[11].s64 + 17128;
	// 83263278: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8326327C: 4AF90ADD  bl 0x821f3d58
	ctx.lr = 0x83263280;
	sub_821F3D58(ctx, base);
	// 83263280: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83263284: 906AB488  stw r3, -0x4b78(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-19320 as u32), ctx.r[3].u32 ) };
	// 83263288: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326328C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83263290: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83263294: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83263298(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83263298 size=56
    let mut pc: u32 = 0x83263298;
    'dispatch: loop {
        match pc {
            0x83263298 => {
    //   block [0x83263298..0x832632D0)
	// 83263298: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326329C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832632A0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832632A4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 832632A8: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 832632AC: 386B4300  addi r3, r11, 0x4300
	ctx.r[3].s64 = ctx.r[11].s64 + 17152;
	// 832632B0: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 832632B4: 4AF90AA5  bl 0x821f3d58
	ctx.lr = 0x832632B8;
	sub_821F3D58(ctx, base);
	// 832632B8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832632BC: 906AB48C  stw r3, -0x4b74(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-19316 as u32), ctx.r[3].u32 ) };
	// 832632C0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832632C4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832632C8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832632CC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832632D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832632D0 size=56
    let mut pc: u32 = 0x832632D0;
    'dispatch: loop {
        match pc {
            0x832632D0 => {
    //   block [0x832632D0..0x83263308)
	// 832632D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832632D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832632D8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832632DC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 832632E0: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 832632E4: 386B4310  addi r3, r11, 0x4310
	ctx.r[3].s64 = ctx.r[11].s64 + 17168;
	// 832632E8: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 832632EC: 4AF90A6D  bl 0x821f3d58
	ctx.lr = 0x832632F0;
	sub_821F3D58(ctx, base);
	// 832632F0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832632F4: 906AB490  stw r3, -0x4b70(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-19312 as u32), ctx.r[3].u32 ) };
	// 832632F8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832632FC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83263300: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83263304: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83263308(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83263308 size=56
    let mut pc: u32 = 0x83263308;
    'dispatch: loop {
        match pc {
            0x83263308 => {
    //   block [0x83263308..0x83263340)
	// 83263308: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326330C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83263310: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83263314: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83263318: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8326331C: 386B4328  addi r3, r11, 0x4328
	ctx.r[3].s64 = ctx.r[11].s64 + 17192;
	// 83263320: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83263324: 4AF90A35  bl 0x821f3d58
	ctx.lr = 0x83263328;
	sub_821F3D58(ctx, base);
	// 83263328: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326332C: 906AB494  stw r3, -0x4b6c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-19308 as u32), ctx.r[3].u32 ) };
	// 83263330: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83263334: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83263338: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326333C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83263340(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83263340 size=56
    let mut pc: u32 = 0x83263340;
    'dispatch: loop {
        match pc {
            0x83263340 => {
    //   block [0x83263340..0x83263378)
	// 83263340: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83263344: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83263348: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326334C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83263350: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83263354: 386B4338  addi r3, r11, 0x4338
	ctx.r[3].s64 = ctx.r[11].s64 + 17208;
	// 83263358: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8326335C: 4AF909FD  bl 0x821f3d58
	ctx.lr = 0x83263360;
	sub_821F3D58(ctx, base);
	// 83263360: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83263364: 906AB498  stw r3, -0x4b68(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-19304 as u32), ctx.r[3].u32 ) };
	// 83263368: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326336C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83263370: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83263374: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83263378(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83263378 size=56
    let mut pc: u32 = 0x83263378;
    'dispatch: loop {
        match pc {
            0x83263378 => {
    //   block [0x83263378..0x832633B0)
	// 83263378: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326337C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83263380: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83263384: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83263388: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8326338C: 386B4350  addi r3, r11, 0x4350
	ctx.r[3].s64 = ctx.r[11].s64 + 17232;
	// 83263390: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83263394: 4AF909C5  bl 0x821f3d58
	ctx.lr = 0x83263398;
	sub_821F3D58(ctx, base);
	// 83263398: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326339C: 906AB49C  stw r3, -0x4b64(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-19300 as u32), ctx.r[3].u32 ) };
	// 832633A0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832633A4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832633A8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832633AC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832633B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832633B0 size=56
    let mut pc: u32 = 0x832633B0;
    'dispatch: loop {
        match pc {
            0x832633B0 => {
    //   block [0x832633B0..0x832633E8)
	// 832633B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832633B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832633B8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832633BC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 832633C0: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 832633C4: 386B4368  addi r3, r11, 0x4368
	ctx.r[3].s64 = ctx.r[11].s64 + 17256;
	// 832633C8: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 832633CC: 4AF9098D  bl 0x821f3d58
	ctx.lr = 0x832633D0;
	sub_821F3D58(ctx, base);
	// 832633D0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832633D4: 906AB4A0  stw r3, -0x4b60(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-19296 as u32), ctx.r[3].u32 ) };
	// 832633D8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832633DC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832633E0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832633E4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832633E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832633E8 size=56
    let mut pc: u32 = 0x832633E8;
    'dispatch: loop {
        match pc {
            0x832633E8 => {
    //   block [0x832633E8..0x83263420)
	// 832633E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832633EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832633F0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832633F4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 832633F8: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 832633FC: 386B4380  addi r3, r11, 0x4380
	ctx.r[3].s64 = ctx.r[11].s64 + 17280;
	// 83263400: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83263404: 4AF90955  bl 0x821f3d58
	ctx.lr = 0x83263408;
	sub_821F3D58(ctx, base);
	// 83263408: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326340C: 906AB4A4  stw r3, -0x4b5c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-19292 as u32), ctx.r[3].u32 ) };
	// 83263410: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83263414: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83263418: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326341C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83263420(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83263420 size=56
    let mut pc: u32 = 0x83263420;
    'dispatch: loop {
        match pc {
            0x83263420 => {
    //   block [0x83263420..0x83263458)
	// 83263420: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83263424: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83263428: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326342C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83263430: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83263434: 386B4394  addi r3, r11, 0x4394
	ctx.r[3].s64 = ctx.r[11].s64 + 17300;
	// 83263438: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8326343C: 4AF9091D  bl 0x821f3d58
	ctx.lr = 0x83263440;
	sub_821F3D58(ctx, base);
	// 83263440: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83263444: 906AB4A8  stw r3, -0x4b58(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-19288 as u32), ctx.r[3].u32 ) };
	// 83263448: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326344C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83263450: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83263454: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83263458(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83263458 size=56
    let mut pc: u32 = 0x83263458;
    'dispatch: loop {
        match pc {
            0x83263458 => {
    //   block [0x83263458..0x83263490)
	// 83263458: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326345C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83263460: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83263464: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83263468: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8326346C: 386B43B4  addi r3, r11, 0x43b4
	ctx.r[3].s64 = ctx.r[11].s64 + 17332;
	// 83263470: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83263474: 4AF908E5  bl 0x821f3d58
	ctx.lr = 0x83263478;
	sub_821F3D58(ctx, base);
	// 83263478: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326347C: 906AB4AC  stw r3, -0x4b54(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-19284 as u32), ctx.r[3].u32 ) };
	// 83263480: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83263484: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83263488: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326348C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83263490(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83263490 size=56
    let mut pc: u32 = 0x83263490;
    'dispatch: loop {
        match pc {
            0x83263490 => {
    //   block [0x83263490..0x832634C8)
	// 83263490: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83263494: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83263498: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326349C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 832634A0: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 832634A4: 386B43C8  addi r3, r11, 0x43c8
	ctx.r[3].s64 = ctx.r[11].s64 + 17352;
	// 832634A8: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 832634AC: 4AF908AD  bl 0x821f3d58
	ctx.lr = 0x832634B0;
	sub_821F3D58(ctx, base);
	// 832634B0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832634B4: 906AB4B0  stw r3, -0x4b50(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-19280 as u32), ctx.r[3].u32 ) };
	// 832634B8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832634BC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832634C0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832634C4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832634C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832634C8 size=56
    let mut pc: u32 = 0x832634C8;
    'dispatch: loop {
        match pc {
            0x832634C8 => {
    //   block [0x832634C8..0x83263500)
	// 832634C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832634CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832634D0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832634D4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 832634D8: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 832634DC: 386B43D4  addi r3, r11, 0x43d4
	ctx.r[3].s64 = ctx.r[11].s64 + 17364;
	// 832634E0: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 832634E4: 4AF90875  bl 0x821f3d58
	ctx.lr = 0x832634E8;
	sub_821F3D58(ctx, base);
	// 832634E8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832634EC: 906AB4B4  stw r3, -0x4b4c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-19276 as u32), ctx.r[3].u32 ) };
	// 832634F0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832634F4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832634F8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832634FC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83263500(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83263500 size=56
    let mut pc: u32 = 0x83263500;
    'dispatch: loop {
        match pc {
            0x83263500 => {
    //   block [0x83263500..0x83263538)
	// 83263500: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83263504: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83263508: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326350C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83263510: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83263514: 386B43E0  addi r3, r11, 0x43e0
	ctx.r[3].s64 = ctx.r[11].s64 + 17376;
	// 83263518: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8326351C: 4AF9083D  bl 0x821f3d58
	ctx.lr = 0x83263520;
	sub_821F3D58(ctx, base);
	// 83263520: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83263524: 906AB4B8  stw r3, -0x4b48(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-19272 as u32), ctx.r[3].u32 ) };
	// 83263528: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326352C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83263530: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83263534: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83263538(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83263538 size=56
    let mut pc: u32 = 0x83263538;
    'dispatch: loop {
        match pc {
            0x83263538 => {
    //   block [0x83263538..0x83263570)
	// 83263538: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326353C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83263540: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83263544: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83263548: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8326354C: 386B43F0  addi r3, r11, 0x43f0
	ctx.r[3].s64 = ctx.r[11].s64 + 17392;
	// 83263550: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83263554: 4AF90805  bl 0x821f3d58
	ctx.lr = 0x83263558;
	sub_821F3D58(ctx, base);
	// 83263558: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326355C: 906AB4BC  stw r3, -0x4b44(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-19268 as u32), ctx.r[3].u32 ) };
	// 83263560: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83263564: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83263568: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326356C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83263570(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83263570 size=56
    let mut pc: u32 = 0x83263570;
    'dispatch: loop {
        match pc {
            0x83263570 => {
    //   block [0x83263570..0x832635A8)
	// 83263570: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83263574: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83263578: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326357C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83263580: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83263584: 386B4400  addi r3, r11, 0x4400
	ctx.r[3].s64 = ctx.r[11].s64 + 17408;
	// 83263588: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8326358C: 4AF907CD  bl 0x821f3d58
	ctx.lr = 0x83263590;
	sub_821F3D58(ctx, base);
	// 83263590: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83263594: 906AB4C0  stw r3, -0x4b40(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-19264 as u32), ctx.r[3].u32 ) };
	// 83263598: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326359C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832635A0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832635A4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832635A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832635A8 size=56
    let mut pc: u32 = 0x832635A8;
    'dispatch: loop {
        match pc {
            0x832635A8 => {
    //   block [0x832635A8..0x832635E0)
	// 832635A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832635AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832635B0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832635B4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 832635B8: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 832635BC: 386B4418  addi r3, r11, 0x4418
	ctx.r[3].s64 = ctx.r[11].s64 + 17432;
	// 832635C0: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 832635C4: 4AF90795  bl 0x821f3d58
	ctx.lr = 0x832635C8;
	sub_821F3D58(ctx, base);
	// 832635C8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832635CC: 906AB4C4  stw r3, -0x4b3c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-19260 as u32), ctx.r[3].u32 ) };
	// 832635D0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832635D4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832635D8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832635DC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832635E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832635E0 size=56
    let mut pc: u32 = 0x832635E0;
    'dispatch: loop {
        match pc {
            0x832635E0 => {
    //   block [0x832635E0..0x83263618)
	// 832635E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832635E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832635E8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832635EC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 832635F0: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 832635F4: 386B4424  addi r3, r11, 0x4424
	ctx.r[3].s64 = ctx.r[11].s64 + 17444;
	// 832635F8: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 832635FC: 4AF9075D  bl 0x821f3d58
	ctx.lr = 0x83263600;
	sub_821F3D58(ctx, base);
	// 83263600: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83263604: 906AB4C8  stw r3, -0x4b38(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-19256 as u32), ctx.r[3].u32 ) };
	// 83263608: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326360C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83263610: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83263614: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83263618(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83263618 size=56
    let mut pc: u32 = 0x83263618;
    'dispatch: loop {
        match pc {
            0x83263618 => {
    //   block [0x83263618..0x83263650)
	// 83263618: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326361C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83263620: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83263624: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83263628: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8326362C: 386B4430  addi r3, r11, 0x4430
	ctx.r[3].s64 = ctx.r[11].s64 + 17456;
	// 83263630: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83263634: 4AF90725  bl 0x821f3d58
	ctx.lr = 0x83263638;
	sub_821F3D58(ctx, base);
	// 83263638: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326363C: 906AB4CC  stw r3, -0x4b34(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-19252 as u32), ctx.r[3].u32 ) };
	// 83263640: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83263644: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83263648: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326364C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83263650(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83263650 size=56
    let mut pc: u32 = 0x83263650;
    'dispatch: loop {
        match pc {
            0x83263650 => {
    //   block [0x83263650..0x83263688)
	// 83263650: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83263654: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83263658: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326365C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83263660: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83263664: 386B4440  addi r3, r11, 0x4440
	ctx.r[3].s64 = ctx.r[11].s64 + 17472;
	// 83263668: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8326366C: 4AF906ED  bl 0x821f3d58
	ctx.lr = 0x83263670;
	sub_821F3D58(ctx, base);
	// 83263670: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83263674: 906AB4D0  stw r3, -0x4b30(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-19248 as u32), ctx.r[3].u32 ) };
	// 83263678: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326367C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83263680: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83263684: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83263688(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83263688 size=64
    let mut pc: u32 = 0x83263688;
    'dispatch: loop {
        match pc {
            0x83263688 => {
    //   block [0x83263688..0x832636C8)
	// 83263688: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326368C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83263690: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83263694: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 83263698: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326369C: 388BF408  addi r4, r11, -0xbf8
	ctx.r[4].s64 = ctx.r[11].s64 + -3064;
	// 832636A0: 386AB4D4  addi r3, r10, -0x4b2c
	ctx.r[3].s64 = ctx.r[10].s64 + -19244;
	// 832636A4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832636A8: 4AFC9829  bl 0x8222ced0
	ctx.lr = 0x832636AC;
	sub_8222CED0(ctx, base);
	// 832636AC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 832636B0: 3869CD70  addi r3, r9, -0x3290
	ctx.r[3].s64 = ctx.r[9].s64 + -12944;
	// 832636B4: 4BA4686D  bl 0x82ca9f20
	ctx.lr = 0x832636B8;
	sub_82CA9F20(ctx, base);
	// 832636B8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832636BC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832636C0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832636C4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832636C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832636C8 size=64
    let mut pc: u32 = 0x832636C8;
    'dispatch: loop {
        match pc {
            0x832636C8 => {
    //   block [0x832636C8..0x83263708)
	// 832636C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832636CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832636D0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832636D4: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 832636D8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832636DC: 388BCE9C  addi r4, r11, -0x3164
	ctx.r[4].s64 = ctx.r[11].s64 + -12644;
	// 832636E0: 386AB4D8  addi r3, r10, -0x4b28
	ctx.r[3].s64 = ctx.r[10].s64 + -19240;
	// 832636E4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832636E8: 4AFC97E9  bl 0x8222ced0
	ctx.lr = 0x832636EC;
	sub_8222CED0(ctx, base);
	// 832636EC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 832636F0: 3869CD80  addi r3, r9, -0x3280
	ctx.r[3].s64 = ctx.r[9].s64 + -12928;
	// 832636F4: 4BA4682D  bl 0x82ca9f20
	ctx.lr = 0x832636F8;
	sub_82CA9F20(ctx, base);
	// 832636F8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832636FC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83263700: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83263704: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83263708(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83263708 size=64
    let mut pc: u32 = 0x83263708;
    'dispatch: loop {
        match pc {
            0x83263708 => {
    //   block [0x83263708..0x83263748)
	// 83263708: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326370C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83263710: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83263714: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83263718: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326371C: 388B052C  addi r4, r11, 0x52c
	ctx.r[4].s64 = ctx.r[11].s64 + 1324;
	// 83263720: 386AB4DC  addi r3, r10, -0x4b24
	ctx.r[3].s64 = ctx.r[10].s64 + -19236;
	// 83263724: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83263728: 4AFC97A9  bl 0x8222ced0
	ctx.lr = 0x8326372C;
	sub_8222CED0(ctx, base);
	// 8326372C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83263730: 3869CD90  addi r3, r9, -0x3270
	ctx.r[3].s64 = ctx.r[9].s64 + -12912;
	// 83263734: 4BA467ED  bl 0x82ca9f20
	ctx.lr = 0x83263738;
	sub_82CA9F20(ctx, base);
	// 83263738: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326373C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83263740: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83263744: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83263748(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x83263748 size=96
    let mut pc: u32 = 0x83263748;
    'dispatch: loop {
        match pc {
            0x83263748 => {
    //   block [0x83263748..0x832637A8)
	// 83263748: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8326374C: 3901FFF0  addi r8, r1, -0x10
	ctx.r[8].s64 = ctx.r[1].s64 + -16;
	// 83263750: 392B9484  addi r9, r11, -0x6b7c
	ctx.r[9].s64 = ctx.r[11].s64 + -27516;
	// 83263754: 3941FFF4  addi r10, r1, -0xc
	ctx.r[10].s64 = ctx.r[1].s64 + -12;
	// 83263758: 38E1FFF4  addi r7, r1, -0xc
	ctx.r[7].s64 = ctx.r[1].s64 + -12;
	// 8326375C: C00B9484  lfs f0, -0x6b7c(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-27516 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 83263760: 38C1FFF0  addi r6, r1, -0x10
	ctx.r[6].s64 = ctx.r[1].s64 + -16;
	// 83263764: D001FFF0  stfs f0, -0x10(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), tmp.u32 ) };
	// 83263768: 3CA0834A  lis r5, -0x7cb6
	ctx.r[5].s64 = -2092302336;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832637A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x832637A8 size=96
    let mut pc: u32 = 0x832637A8;
    'dispatch: loop {
        match pc {
            0x832637A8 => {
    //   block [0x832637A8..0x83263808)
	// 832637A8: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 832637AC: 3901FFF0  addi r8, r1, -0x10
	ctx.r[8].s64 = ctx.r[1].s64 + -16;
	// 832637B0: 392B9484  addi r9, r11, -0x6b7c
	ctx.r[9].s64 = ctx.r[11].s64 + -27516;
	// 832637B4: 3941FFF4  addi r10, r1, -0xc
	ctx.r[10].s64 = ctx.r[1].s64 + -12;
	// 832637B8: 38E1FFF4  addi r7, r1, -0xc
	ctx.r[7].s64 = ctx.r[1].s64 + -12;
	// 832637BC: C00B9484  lfs f0, -0x6b7c(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-27516 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 832637C0: 38C1FFF0  addi r6, r1, -0x10
	ctx.r[6].s64 = ctx.r[1].s64 + -16;
	// 832637C4: D001FFF0  stfs f0, -0x10(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), tmp.u32 ) };
	// 832637C8: 3CA0834A  lis r5, -0x7cb6
	ctx.r[5].s64 = -2092302336;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83263808(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x83263808 size=96
    let mut pc: u32 = 0x83263808;
    'dispatch: loop {
        match pc {
            0x83263808 => {
    //   block [0x83263808..0x83263868)
	// 83263808: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8326380C: 3901FFF0  addi r8, r1, -0x10
	ctx.r[8].s64 = ctx.r[1].s64 + -16;
	// 83263810: 392B9490  addi r9, r11, -0x6b70
	ctx.r[9].s64 = ctx.r[11].s64 + -27504;
	// 83263814: 3941FFF4  addi r10, r1, -0xc
	ctx.r[10].s64 = ctx.r[1].s64 + -12;
	// 83263818: 38E1FFF4  addi r7, r1, -0xc
	ctx.r[7].s64 = ctx.r[1].s64 + -12;
	// 8326381C: C00B9490  lfs f0, -0x6b70(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-27504 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 83263820: 38C1FFF0  addi r6, r1, -0x10
	ctx.r[6].s64 = ctx.r[1].s64 + -16;
	// 83263824: D001FFF0  stfs f0, -0x10(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), tmp.u32 ) };
	// 83263828: 3CA0834A  lis r5, -0x7cb6
	ctx.r[5].s64 = -2092302336;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83263868(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83263868 size=64
    let mut pc: u32 = 0x83263868;
    'dispatch: loop {
        match pc {
            0x83263868 => {
    //   block [0x83263868..0x832638A8)
	// 83263868: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326386C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83263870: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83263874: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 83263878: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326387C: 388B0298  addi r4, r11, 0x298
	ctx.r[4].s64 = ctx.r[11].s64 + 664;
	// 83263880: 386AB510  addi r3, r10, -0x4af0
	ctx.r[3].s64 = ctx.r[10].s64 + -19184;
	// 83263884: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83263888: 4AFC9649  bl 0x8222ced0
	ctx.lr = 0x8326388C;
	sub_8222CED0(ctx, base);
	// 8326388C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83263890: 3869CDA0  addi r3, r9, -0x3260
	ctx.r[3].s64 = ctx.r[9].s64 + -12896;
	// 83263894: 4BA4668D  bl 0x82ca9f20
	ctx.lr = 0x83263898;
	sub_82CA9F20(ctx, base);
	// 83263898: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326389C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832638A0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832638A4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832638A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832638A8 size=64
    let mut pc: u32 = 0x832638A8;
    'dispatch: loop {
        match pc {
            0x832638A8 => {
    //   block [0x832638A8..0x832638E8)
	// 832638A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832638AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832638B0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832638B4: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 832638B8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832638BC: 388B02AC  addi r4, r11, 0x2ac
	ctx.r[4].s64 = ctx.r[11].s64 + 684;
	// 832638C0: 386AB514  addi r3, r10, -0x4aec
	ctx.r[3].s64 = ctx.r[10].s64 + -19180;
	// 832638C4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832638C8: 4AFC9609  bl 0x8222ced0
	ctx.lr = 0x832638CC;
	sub_8222CED0(ctx, base);
	// 832638CC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 832638D0: 3869CDB0  addi r3, r9, -0x3250
	ctx.r[3].s64 = ctx.r[9].s64 + -12880;
	// 832638D4: 4BA4664D  bl 0x82ca9f20
	ctx.lr = 0x832638D8;
	sub_82CA9F20(ctx, base);
	// 832638D8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832638DC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832638E0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832638E4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832638E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832638E8 size=64
    let mut pc: u32 = 0x832638E8;
    'dispatch: loop {
        match pc {
            0x832638E8 => {
    //   block [0x832638E8..0x83263928)
	// 832638E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832638EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832638F0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832638F4: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 832638F8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832638FC: 388B02B8  addi r4, r11, 0x2b8
	ctx.r[4].s64 = ctx.r[11].s64 + 696;
	// 83263900: 386AB518  addi r3, r10, -0x4ae8
	ctx.r[3].s64 = ctx.r[10].s64 + -19176;
	// 83263904: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83263908: 4AFC95C9  bl 0x8222ced0
	ctx.lr = 0x8326390C;
	sub_8222CED0(ctx, base);
	// 8326390C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83263910: 3869CDC0  addi r3, r9, -0x3240
	ctx.r[3].s64 = ctx.r[9].s64 + -12864;
	// 83263914: 4BA4660D  bl 0x82ca9f20
	ctx.lr = 0x83263918;
	sub_82CA9F20(ctx, base);
	// 83263918: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326391C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83263920: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83263924: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83263928(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83263928 size=64
    let mut pc: u32 = 0x83263928;
    'dispatch: loop {
        match pc {
            0x83263928 => {
    //   block [0x83263928..0x83263968)
	// 83263928: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326392C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83263930: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83263934: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 83263938: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326393C: 388B02C4  addi r4, r11, 0x2c4
	ctx.r[4].s64 = ctx.r[11].s64 + 708;
	// 83263940: 386AB51C  addi r3, r10, -0x4ae4
	ctx.r[3].s64 = ctx.r[10].s64 + -19172;
	// 83263944: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83263948: 4AFC9589  bl 0x8222ced0
	ctx.lr = 0x8326394C;
	sub_8222CED0(ctx, base);
	// 8326394C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83263950: 3869CDD0  addi r3, r9, -0x3230
	ctx.r[3].s64 = ctx.r[9].s64 + -12848;
	// 83263954: 4BA465CD  bl 0x82ca9f20
	ctx.lr = 0x83263958;
	sub_82CA9F20(ctx, base);
	// 83263958: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326395C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83263960: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83263964: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83263968(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83263968 size=64
    let mut pc: u32 = 0x83263968;
    'dispatch: loop {
        match pc {
            0x83263968 => {
    //   block [0x83263968..0x832639A8)
	// 83263968: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326396C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83263970: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83263974: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 83263978: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326397C: 388B02CC  addi r4, r11, 0x2cc
	ctx.r[4].s64 = ctx.r[11].s64 + 716;
	// 83263980: 386AB520  addi r3, r10, -0x4ae0
	ctx.r[3].s64 = ctx.r[10].s64 + -19168;
	// 83263984: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83263988: 4AFC9549  bl 0x8222ced0
	ctx.lr = 0x8326398C;
	sub_8222CED0(ctx, base);
	// 8326398C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83263990: 3869CDE0  addi r3, r9, -0x3220
	ctx.r[3].s64 = ctx.r[9].s64 + -12832;
	// 83263994: 4BA4658D  bl 0x82ca9f20
	ctx.lr = 0x83263998;
	sub_82CA9F20(ctx, base);
	// 83263998: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326399C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832639A0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832639A4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832639A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832639A8 size=64
    let mut pc: u32 = 0x832639A8;
    'dispatch: loop {
        match pc {
            0x832639A8 => {
    //   block [0x832639A8..0x832639E8)
	// 832639A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832639AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832639B0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832639B4: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 832639B8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832639BC: 388BABCC  addi r4, r11, -0x5434
	ctx.r[4].s64 = ctx.r[11].s64 + -21556;
	// 832639C0: 386AB524  addi r3, r10, -0x4adc
	ctx.r[3].s64 = ctx.r[10].s64 + -19164;
	// 832639C4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832639C8: 4AFC9509  bl 0x8222ced0
	ctx.lr = 0x832639CC;
	sub_8222CED0(ctx, base);
	// 832639CC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 832639D0: 3869CDF0  addi r3, r9, -0x3210
	ctx.r[3].s64 = ctx.r[9].s64 + -12816;
	// 832639D4: 4BA4654D  bl 0x82ca9f20
	ctx.lr = 0x832639D8;
	sub_82CA9F20(ctx, base);
	// 832639D8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832639DC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832639E0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832639E4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832639E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832639E8 size=64
    let mut pc: u32 = 0x832639E8;
    'dispatch: loop {
        match pc {
            0x832639E8 => {
    //   block [0x832639E8..0x83263A28)
	// 832639E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832639EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832639F0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832639F4: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 832639F8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832639FC: 388BABD0  addi r4, r11, -0x5430
	ctx.r[4].s64 = ctx.r[11].s64 + -21552;
	// 83263A00: 386AB528  addi r3, r10, -0x4ad8
	ctx.r[3].s64 = ctx.r[10].s64 + -19160;
	// 83263A04: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83263A08: 4AFC94C9  bl 0x8222ced0
	ctx.lr = 0x83263A0C;
	sub_8222CED0(ctx, base);
	// 83263A0C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83263A10: 3869CE00  addi r3, r9, -0x3200
	ctx.r[3].s64 = ctx.r[9].s64 + -12800;
	// 83263A14: 4BA4650D  bl 0x82ca9f20
	ctx.lr = 0x83263A18;
	sub_82CA9F20(ctx, base);
	// 83263A18: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83263A1C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83263A20: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83263A24: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83263A28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83263A28 size=64
    let mut pc: u32 = 0x83263A28;
    'dispatch: loop {
        match pc {
            0x83263A28 => {
    //   block [0x83263A28..0x83263A68)
	// 83263A28: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83263A2C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83263A30: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83263A34: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 83263A38: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83263A3C: 388BABD8  addi r4, r11, -0x5428
	ctx.r[4].s64 = ctx.r[11].s64 + -21544;
	// 83263A40: 386AB52C  addi r3, r10, -0x4ad4
	ctx.r[3].s64 = ctx.r[10].s64 + -19156;
	// 83263A44: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83263A48: 4AFC9489  bl 0x8222ced0
	ctx.lr = 0x83263A4C;
	sub_8222CED0(ctx, base);
	// 83263A4C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83263A50: 3869CE10  addi r3, r9, -0x31f0
	ctx.r[3].s64 = ctx.r[9].s64 + -12784;
	// 83263A54: 4BA464CD  bl 0x82ca9f20
	ctx.lr = 0x83263A58;
	sub_82CA9F20(ctx, base);
	// 83263A58: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83263A5C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83263A60: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83263A64: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83263A68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83263A68 size=64
    let mut pc: u32 = 0x83263A68;
    'dispatch: loop {
        match pc {
            0x83263A68 => {
    //   block [0x83263A68..0x83263AA8)
	// 83263A68: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83263A6C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83263A70: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83263A74: 3D60820C  lis r11, -0x7df4
	ctx.r[11].s64 = -2113142784;
	// 83263A78: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83263A7C: 388B34A0  addi r4, r11, 0x34a0
	ctx.r[4].s64 = ctx.r[11].s64 + 13472;
	// 83263A80: 386AB530  addi r3, r10, -0x4ad0
	ctx.r[3].s64 = ctx.r[10].s64 + -19152;
	// 83263A84: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83263A88: 4AFC9449  bl 0x8222ced0
	ctx.lr = 0x83263A8C;
	sub_8222CED0(ctx, base);
	// 83263A8C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83263A90: 3869CE20  addi r3, r9, -0x31e0
	ctx.r[3].s64 = ctx.r[9].s64 + -12768;
	// 83263A94: 4BA4648D  bl 0x82ca9f20
	ctx.lr = 0x83263A98;
	sub_82CA9F20(ctx, base);
	// 83263A98: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83263A9C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83263AA0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83263AA4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83263AA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83263AA8 size=64
    let mut pc: u32 = 0x83263AA8;
    'dispatch: loop {
        match pc {
            0x83263AA8 => {
    //   block [0x83263AA8..0x83263AE8)
	// 83263AA8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83263AAC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83263AB0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83263AB4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83263AB8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83263ABC: 388B1AD4  addi r4, r11, 0x1ad4
	ctx.r[4].s64 = ctx.r[11].s64 + 6868;
	// 83263AC0: 386AB534  addi r3, r10, -0x4acc
	ctx.r[3].s64 = ctx.r[10].s64 + -19148;
	// 83263AC4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83263AC8: 4AFC9409  bl 0x8222ced0
	ctx.lr = 0x83263ACC;
	sub_8222CED0(ctx, base);
	// 83263ACC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83263AD0: 3869CE30  addi r3, r9, -0x31d0
	ctx.r[3].s64 = ctx.r[9].s64 + -12752;
	// 83263AD4: 4BA4644D  bl 0x82ca9f20
	ctx.lr = 0x83263AD8;
	sub_82CA9F20(ctx, base);
	// 83263AD8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83263ADC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83263AE0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83263AE4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83263AE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83263AE8 size=64
    let mut pc: u32 = 0x83263AE8;
    'dispatch: loop {
        match pc {
            0x83263AE8 => {
    //   block [0x83263AE8..0x83263B28)
	// 83263AE8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83263AEC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83263AF0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83263AF4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83263AF8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83263AFC: 388B1ADC  addi r4, r11, 0x1adc
	ctx.r[4].s64 = ctx.r[11].s64 + 6876;
	// 83263B00: 386AB538  addi r3, r10, -0x4ac8
	ctx.r[3].s64 = ctx.r[10].s64 + -19144;
	// 83263B04: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83263B08: 4AFC93C9  bl 0x8222ced0
	ctx.lr = 0x83263B0C;
	sub_8222CED0(ctx, base);
	// 83263B0C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83263B10: 3869CE40  addi r3, r9, -0x31c0
	ctx.r[3].s64 = ctx.r[9].s64 + -12736;
	// 83263B14: 4BA4640D  bl 0x82ca9f20
	ctx.lr = 0x83263B18;
	sub_82CA9F20(ctx, base);
	// 83263B18: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83263B1C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83263B20: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83263B24: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83263B28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83263B28 size=64
    let mut pc: u32 = 0x83263B28;
    'dispatch: loop {
        match pc {
            0x83263B28 => {
    //   block [0x83263B28..0x83263B68)
	// 83263B28: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83263B2C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83263B30: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83263B34: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83263B38: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83263B3C: 388B1AF0  addi r4, r11, 0x1af0
	ctx.r[4].s64 = ctx.r[11].s64 + 6896;
	// 83263B40: 386AB53C  addi r3, r10, -0x4ac4
	ctx.r[3].s64 = ctx.r[10].s64 + -19140;
	// 83263B44: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83263B48: 4AFC9389  bl 0x8222ced0
	ctx.lr = 0x83263B4C;
	sub_8222CED0(ctx, base);
	// 83263B4C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83263B50: 3869CE50  addi r3, r9, -0x31b0
	ctx.r[3].s64 = ctx.r[9].s64 + -12720;
	// 83263B54: 4BA463CD  bl 0x82ca9f20
	ctx.lr = 0x83263B58;
	sub_82CA9F20(ctx, base);
	// 83263B58: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83263B5C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83263B60: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83263B64: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83263B68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83263B68 size=56
    let mut pc: u32 = 0x83263B68;
    'dispatch: loop {
        match pc {
            0x83263B68 => {
    //   block [0x83263B68..0x83263BA0)
	// 83263B68: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83263B6C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83263B70: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83263B74: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83263B78: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83263B7C: 386B42C8  addi r3, r11, 0x42c8
	ctx.r[3].s64 = ctx.r[11].s64 + 17096;
	// 83263B80: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83263B84: 4AF901D5  bl 0x821f3d58
	ctx.lr = 0x83263B88;
	sub_821F3D58(ctx, base);
	// 83263B88: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83263B8C: 906AB540  stw r3, -0x4ac0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-19136 as u32), ctx.r[3].u32 ) };
	// 83263B90: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83263B94: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83263B98: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83263B9C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83263BA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83263BA0 size=56
    let mut pc: u32 = 0x83263BA0;
    'dispatch: loop {
        match pc {
            0x83263BA0 => {
    //   block [0x83263BA0..0x83263BD8)
	// 83263BA0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83263BA4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83263BA8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83263BAC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83263BB0: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83263BB4: 386B42DC  addi r3, r11, 0x42dc
	ctx.r[3].s64 = ctx.r[11].s64 + 17116;
	// 83263BB8: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83263BBC: 4AF9019D  bl 0x821f3d58
	ctx.lr = 0x83263BC0;
	sub_821F3D58(ctx, base);
	// 83263BC0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83263BC4: 906AB544  stw r3, -0x4abc(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-19132 as u32), ctx.r[3].u32 ) };
	// 83263BC8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83263BCC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83263BD0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83263BD4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83263BD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83263BD8 size=56
    let mut pc: u32 = 0x83263BD8;
    'dispatch: loop {
        match pc {
            0x83263BD8 => {
    //   block [0x83263BD8..0x83263C10)
	// 83263BD8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83263BDC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83263BE0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83263BE4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83263BE8: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83263BEC: 386B42E8  addi r3, r11, 0x42e8
	ctx.r[3].s64 = ctx.r[11].s64 + 17128;
	// 83263BF0: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83263BF4: 4AF90165  bl 0x821f3d58
	ctx.lr = 0x83263BF8;
	sub_821F3D58(ctx, base);
	// 83263BF8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83263BFC: 906AB548  stw r3, -0x4ab8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-19128 as u32), ctx.r[3].u32 ) };
	// 83263C00: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83263C04: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83263C08: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83263C0C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83263C10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83263C10 size=56
    let mut pc: u32 = 0x83263C10;
    'dispatch: loop {
        match pc {
            0x83263C10 => {
    //   block [0x83263C10..0x83263C48)
	// 83263C10: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83263C14: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83263C18: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83263C1C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83263C20: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83263C24: 386B4300  addi r3, r11, 0x4300
	ctx.r[3].s64 = ctx.r[11].s64 + 17152;
	// 83263C28: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83263C2C: 4AF9012D  bl 0x821f3d58
	ctx.lr = 0x83263C30;
	sub_821F3D58(ctx, base);
	// 83263C30: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83263C34: 906AB54C  stw r3, -0x4ab4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-19124 as u32), ctx.r[3].u32 ) };
	// 83263C38: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83263C3C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83263C40: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83263C44: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83263C48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83263C48 size=56
    let mut pc: u32 = 0x83263C48;
    'dispatch: loop {
        match pc {
            0x83263C48 => {
    //   block [0x83263C48..0x83263C80)
	// 83263C48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83263C4C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83263C50: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83263C54: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83263C58: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83263C5C: 386B4310  addi r3, r11, 0x4310
	ctx.r[3].s64 = ctx.r[11].s64 + 17168;
	// 83263C60: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83263C64: 4AF900F5  bl 0x821f3d58
	ctx.lr = 0x83263C68;
	sub_821F3D58(ctx, base);
	// 83263C68: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83263C6C: 906AB550  stw r3, -0x4ab0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-19120 as u32), ctx.r[3].u32 ) };
	// 83263C70: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83263C74: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83263C78: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83263C7C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83263C80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83263C80 size=56
    let mut pc: u32 = 0x83263C80;
    'dispatch: loop {
        match pc {
            0x83263C80 => {
    //   block [0x83263C80..0x83263CB8)
	// 83263C80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83263C84: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83263C88: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83263C8C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83263C90: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83263C94: 386B4328  addi r3, r11, 0x4328
	ctx.r[3].s64 = ctx.r[11].s64 + 17192;
	// 83263C98: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83263C9C: 4AF900BD  bl 0x821f3d58
	ctx.lr = 0x83263CA0;
	sub_821F3D58(ctx, base);
	// 83263CA0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83263CA4: 906AB554  stw r3, -0x4aac(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-19116 as u32), ctx.r[3].u32 ) };
	// 83263CA8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83263CAC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83263CB0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83263CB4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83263CB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83263CB8 size=56
    let mut pc: u32 = 0x83263CB8;
    'dispatch: loop {
        match pc {
            0x83263CB8 => {
    //   block [0x83263CB8..0x83263CF0)
	// 83263CB8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83263CBC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83263CC0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83263CC4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83263CC8: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83263CCC: 386B4338  addi r3, r11, 0x4338
	ctx.r[3].s64 = ctx.r[11].s64 + 17208;
	// 83263CD0: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83263CD4: 4AF90085  bl 0x821f3d58
	ctx.lr = 0x83263CD8;
	sub_821F3D58(ctx, base);
	// 83263CD8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83263CDC: 906AB558  stw r3, -0x4aa8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-19112 as u32), ctx.r[3].u32 ) };
	// 83263CE0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83263CE4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83263CE8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83263CEC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83263CF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83263CF0 size=56
    let mut pc: u32 = 0x83263CF0;
    'dispatch: loop {
        match pc {
            0x83263CF0 => {
    //   block [0x83263CF0..0x83263D28)
	// 83263CF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83263CF4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83263CF8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83263CFC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83263D00: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83263D04: 386B4350  addi r3, r11, 0x4350
	ctx.r[3].s64 = ctx.r[11].s64 + 17232;
	// 83263D08: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83263D0C: 4AF9004D  bl 0x821f3d58
	ctx.lr = 0x83263D10;
	sub_821F3D58(ctx, base);
	// 83263D10: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83263D14: 906AB55C  stw r3, -0x4aa4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-19108 as u32), ctx.r[3].u32 ) };
	// 83263D18: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83263D1C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83263D20: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83263D24: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83263D28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83263D28 size=56
    let mut pc: u32 = 0x83263D28;
    'dispatch: loop {
        match pc {
            0x83263D28 => {
    //   block [0x83263D28..0x83263D60)
	// 83263D28: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83263D2C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83263D30: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83263D34: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83263D38: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83263D3C: 386B4368  addi r3, r11, 0x4368
	ctx.r[3].s64 = ctx.r[11].s64 + 17256;
	// 83263D40: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83263D44: 4AF90015  bl 0x821f3d58
	ctx.lr = 0x83263D48;
	sub_821F3D58(ctx, base);
	// 83263D48: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83263D4C: 906AB560  stw r3, -0x4aa0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-19104 as u32), ctx.r[3].u32 ) };
	// 83263D50: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83263D54: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83263D58: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83263D5C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83263D60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83263D60 size=56
    let mut pc: u32 = 0x83263D60;
    'dispatch: loop {
        match pc {
            0x83263D60 => {
    //   block [0x83263D60..0x83263D98)
	// 83263D60: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83263D64: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83263D68: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83263D6C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83263D70: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83263D74: 386B4380  addi r3, r11, 0x4380
	ctx.r[3].s64 = ctx.r[11].s64 + 17280;
	// 83263D78: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83263D7C: 4AF8FFDD  bl 0x821f3d58
	ctx.lr = 0x83263D80;
	sub_821F3D58(ctx, base);
	// 83263D80: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83263D84: 906AB564  stw r3, -0x4a9c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-19100 as u32), ctx.r[3].u32 ) };
	// 83263D88: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83263D8C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83263D90: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83263D94: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83263D98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83263D98 size=56
    let mut pc: u32 = 0x83263D98;
    'dispatch: loop {
        match pc {
            0x83263D98 => {
    //   block [0x83263D98..0x83263DD0)
	// 83263D98: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83263D9C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83263DA0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83263DA4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83263DA8: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83263DAC: 386B4394  addi r3, r11, 0x4394
	ctx.r[3].s64 = ctx.r[11].s64 + 17300;
	// 83263DB0: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83263DB4: 4AF8FFA5  bl 0x821f3d58
	ctx.lr = 0x83263DB8;
	sub_821F3D58(ctx, base);
	// 83263DB8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83263DBC: 906AB568  stw r3, -0x4a98(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-19096 as u32), ctx.r[3].u32 ) };
	// 83263DC0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83263DC4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83263DC8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83263DCC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83263DD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83263DD0 size=56
    let mut pc: u32 = 0x83263DD0;
    'dispatch: loop {
        match pc {
            0x83263DD0 => {
    //   block [0x83263DD0..0x83263E08)
	// 83263DD0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83263DD4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83263DD8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83263DDC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83263DE0: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83263DE4: 386B43B4  addi r3, r11, 0x43b4
	ctx.r[3].s64 = ctx.r[11].s64 + 17332;
	// 83263DE8: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83263DEC: 4AF8FF6D  bl 0x821f3d58
	ctx.lr = 0x83263DF0;
	sub_821F3D58(ctx, base);
	// 83263DF0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83263DF4: 906AB56C  stw r3, -0x4a94(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-19092 as u32), ctx.r[3].u32 ) };
	// 83263DF8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83263DFC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83263E00: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83263E04: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83263E08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83263E08 size=56
    let mut pc: u32 = 0x83263E08;
    'dispatch: loop {
        match pc {
            0x83263E08 => {
    //   block [0x83263E08..0x83263E40)
	// 83263E08: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83263E0C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83263E10: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83263E14: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83263E18: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83263E1C: 386B43C8  addi r3, r11, 0x43c8
	ctx.r[3].s64 = ctx.r[11].s64 + 17352;
	// 83263E20: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83263E24: 4AF8FF35  bl 0x821f3d58
	ctx.lr = 0x83263E28;
	sub_821F3D58(ctx, base);
	// 83263E28: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83263E2C: 906AB570  stw r3, -0x4a90(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-19088 as u32), ctx.r[3].u32 ) };
	// 83263E30: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83263E34: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83263E38: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83263E3C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83263E40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83263E40 size=56
    let mut pc: u32 = 0x83263E40;
    'dispatch: loop {
        match pc {
            0x83263E40 => {
    //   block [0x83263E40..0x83263E78)
	// 83263E40: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83263E44: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83263E48: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83263E4C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83263E50: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83263E54: 386B43D4  addi r3, r11, 0x43d4
	ctx.r[3].s64 = ctx.r[11].s64 + 17364;
	// 83263E58: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83263E5C: 4AF8FEFD  bl 0x821f3d58
	ctx.lr = 0x83263E60;
	sub_821F3D58(ctx, base);
	// 83263E60: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83263E64: 906AB574  stw r3, -0x4a8c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-19084 as u32), ctx.r[3].u32 ) };
	// 83263E68: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83263E6C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83263E70: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83263E74: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


