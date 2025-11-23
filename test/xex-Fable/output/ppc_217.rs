pub fn sub_8326D260(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326D260 size=56
    let mut pc: u32 = 0x8326D260;
    'dispatch: loop {
        match pc {
            0x8326D260 => {
    //   block [0x8326D260..0x8326D298)
	// 8326D260: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326D264: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326D268: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326D26C: 3D60820E  lis r11, -0x7df2
	ctx.r[11].s64 = -2113011712;
	// 8326D270: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8326D274: 386B2B20  addi r3, r11, 0x2b20
	ctx.r[3].s64 = ctx.r[11].s64 + 11040;
	// 8326D278: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8326D27C: 4AF86ADD  bl 0x821f3d58
	ctx.lr = 0x8326D280;
	sub_821F3D58(ctx, base);
	// 8326D280: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326D284: 906AC820  stw r3, -0x37e0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-14304 as u32), ctx.r[3].u32 ) };
	// 8326D288: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326D28C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326D290: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326D294: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326D298(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326D298 size=56
    let mut pc: u32 = 0x8326D298;
    'dispatch: loop {
        match pc {
            0x8326D298 => {
    //   block [0x8326D298..0x8326D2D0)
	// 8326D298: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326D29C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326D2A0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326D2A4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8326D2A8: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8326D2AC: 386B4328  addi r3, r11, 0x4328
	ctx.r[3].s64 = ctx.r[11].s64 + 17192;
	// 8326D2B0: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8326D2B4: 4AF86AA5  bl 0x821f3d58
	ctx.lr = 0x8326D2B8;
	sub_821F3D58(ctx, base);
	// 8326D2B8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326D2BC: 906AC824  stw r3, -0x37dc(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-14300 as u32), ctx.r[3].u32 ) };
	// 8326D2C0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326D2C4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326D2C8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326D2CC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326D2D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326D2D0 size=56
    let mut pc: u32 = 0x8326D2D0;
    'dispatch: loop {
        match pc {
            0x8326D2D0 => {
    //   block [0x8326D2D0..0x8326D308)
	// 8326D2D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326D2D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326D2D8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326D2DC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8326D2E0: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8326D2E4: 386B4400  addi r3, r11, 0x4400
	ctx.r[3].s64 = ctx.r[11].s64 + 17408;
	// 8326D2E8: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8326D2EC: 4AF86A6D  bl 0x821f3d58
	ctx.lr = 0x8326D2F0;
	sub_821F3D58(ctx, base);
	// 8326D2F0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326D2F4: 906AC828  stw r3, -0x37d8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-14296 as u32), ctx.r[3].u32 ) };
	// 8326D2F8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326D2FC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326D300: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326D304: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326D308(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326D308 size=56
    let mut pc: u32 = 0x8326D308;
    'dispatch: loop {
        match pc {
            0x8326D308 => {
    //   block [0x8326D308..0x8326D340)
	// 8326D308: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326D30C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326D310: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326D314: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8326D318: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8326D31C: 386B4418  addi r3, r11, 0x4418
	ctx.r[3].s64 = ctx.r[11].s64 + 17432;
	// 8326D320: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8326D324: 4AF86A35  bl 0x821f3d58
	ctx.lr = 0x8326D328;
	sub_821F3D58(ctx, base);
	// 8326D328: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326D32C: 906AC82C  stw r3, -0x37d4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-14292 as u32), ctx.r[3].u32 ) };
	// 8326D330: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326D334: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326D338: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326D33C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326D340(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326D340 size=56
    let mut pc: u32 = 0x8326D340;
    'dispatch: loop {
        match pc {
            0x8326D340 => {
    //   block [0x8326D340..0x8326D378)
	// 8326D340: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326D344: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326D348: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326D34C: 3D60820E  lis r11, -0x7df2
	ctx.r[11].s64 = -2113011712;
	// 8326D350: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8326D354: 386B2B2C  addi r3, r11, 0x2b2c
	ctx.r[3].s64 = ctx.r[11].s64 + 11052;
	// 8326D358: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8326D35C: 4AF869FD  bl 0x821f3d58
	ctx.lr = 0x8326D360;
	sub_821F3D58(ctx, base);
	// 8326D360: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326D364: 906AC830  stw r3, -0x37d0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-14288 as u32), ctx.r[3].u32 ) };
	// 8326D368: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326D36C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326D370: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326D374: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326D378(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326D378 size=56
    let mut pc: u32 = 0x8326D378;
    'dispatch: loop {
        match pc {
            0x8326D378 => {
    //   block [0x8326D378..0x8326D3B0)
	// 8326D378: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326D37C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326D380: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326D384: 3D60820E  lis r11, -0x7df2
	ctx.r[11].s64 = -2113011712;
	// 8326D388: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8326D38C: 386B2B40  addi r3, r11, 0x2b40
	ctx.r[3].s64 = ctx.r[11].s64 + 11072;
	// 8326D390: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8326D394: 4AF869C5  bl 0x821f3d58
	ctx.lr = 0x8326D398;
	sub_821F3D58(ctx, base);
	// 8326D398: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326D39C: 906AC834  stw r3, -0x37cc(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-14284 as u32), ctx.r[3].u32 ) };
	// 8326D3A0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326D3A4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326D3A8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326D3AC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326D3B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326D3B0 size=56
    let mut pc: u32 = 0x8326D3B0;
    'dispatch: loop {
        match pc {
            0x8326D3B0 => {
    //   block [0x8326D3B0..0x8326D3E8)
	// 8326D3B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326D3B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326D3B8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326D3BC: 3D60820E  lis r11, -0x7df2
	ctx.r[11].s64 = -2113011712;
	// 8326D3C0: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8326D3C4: 386B2B54  addi r3, r11, 0x2b54
	ctx.r[3].s64 = ctx.r[11].s64 + 11092;
	// 8326D3C8: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8326D3CC: 4AF8698D  bl 0x821f3d58
	ctx.lr = 0x8326D3D0;
	sub_821F3D58(ctx, base);
	// 8326D3D0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326D3D4: 906AC838  stw r3, -0x37c8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-14280 as u32), ctx.r[3].u32 ) };
	// 8326D3D8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326D3DC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326D3E0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326D3E4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326D3E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326D3E8 size=56
    let mut pc: u32 = 0x8326D3E8;
    'dispatch: loop {
        match pc {
            0x8326D3E8 => {
    //   block [0x8326D3E8..0x8326D420)
	// 8326D3E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326D3EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326D3F0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326D3F4: 3D60820E  lis r11, -0x7df2
	ctx.r[11].s64 = -2113011712;
	// 8326D3F8: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8326D3FC: 386B2B68  addi r3, r11, 0x2b68
	ctx.r[3].s64 = ctx.r[11].s64 + 11112;
	// 8326D400: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8326D404: 4AF86955  bl 0x821f3d58
	ctx.lr = 0x8326D408;
	sub_821F3D58(ctx, base);
	// 8326D408: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326D40C: 906AC83C  stw r3, -0x37c4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-14276 as u32), ctx.r[3].u32 ) };
	// 8326D410: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326D414: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326D418: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326D41C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326D420(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326D420 size=56
    let mut pc: u32 = 0x8326D420;
    'dispatch: loop {
        match pc {
            0x8326D420 => {
    //   block [0x8326D420..0x8326D458)
	// 8326D420: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326D424: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326D428: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326D42C: 3D60820E  lis r11, -0x7df2
	ctx.r[11].s64 = -2113011712;
	// 8326D430: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8326D434: 386B2B7C  addi r3, r11, 0x2b7c
	ctx.r[3].s64 = ctx.r[11].s64 + 11132;
	// 8326D438: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8326D43C: 4AF8691D  bl 0x821f3d58
	ctx.lr = 0x8326D440;
	sub_821F3D58(ctx, base);
	// 8326D440: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326D444: 906AC840  stw r3, -0x37c0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-14272 as u32), ctx.r[3].u32 ) };
	// 8326D448: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326D44C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326D450: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326D454: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326D458(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326D458 size=64
    let mut pc: u32 = 0x8326D458;
    'dispatch: loop {
        match pc {
            0x8326D458 => {
    //   block [0x8326D458..0x8326D498)
	// 8326D458: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326D45C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326D460: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326D464: 3D60820E  lis r11, -0x7df2
	ctx.r[11].s64 = -2113011712;
	// 8326D468: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326D46C: 388B30C8  addi r4, r11, 0x30c8
	ctx.r[4].s64 = ctx.r[11].s64 + 12488;
	// 8326D470: 386AC844  addi r3, r10, -0x37bc
	ctx.r[3].s64 = ctx.r[10].s64 + -14268;
	// 8326D474: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8326D478: 4AFBFA59  bl 0x8222ced0
	ctx.lr = 0x8326D47C;
	sub_8222CED0(ctx, base);
	// 8326D47C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8326D480: 3869EC50  addi r3, r9, -0x13b0
	ctx.r[3].s64 = ctx.r[9].s64 + -5040;
	// 8326D484: 4BA3CA9D  bl 0x82ca9f20
	ctx.lr = 0x8326D488;
	sub_82CA9F20(ctx, base);
	// 8326D488: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326D48C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326D490: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326D494: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326D498(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326D498 size=64
    let mut pc: u32 = 0x8326D498;
    'dispatch: loop {
        match pc {
            0x8326D498 => {
    //   block [0x8326D498..0x8326D4D8)
	// 8326D498: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326D49C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326D4A0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326D4A4: 3D60820E  lis r11, -0x7df2
	ctx.r[11].s64 = -2113011712;
	// 8326D4A8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326D4AC: 388B30D8  addi r4, r11, 0x30d8
	ctx.r[4].s64 = ctx.r[11].s64 + 12504;
	// 8326D4B0: 386AC848  addi r3, r10, -0x37b8
	ctx.r[3].s64 = ctx.r[10].s64 + -14264;
	// 8326D4B4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8326D4B8: 4AFBFA19  bl 0x8222ced0
	ctx.lr = 0x8326D4BC;
	sub_8222CED0(ctx, base);
	// 8326D4BC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8326D4C0: 3869EC60  addi r3, r9, -0x13a0
	ctx.r[3].s64 = ctx.r[9].s64 + -5024;
	// 8326D4C4: 4BA3CA5D  bl 0x82ca9f20
	ctx.lr = 0x8326D4C8;
	sub_82CA9F20(ctx, base);
	// 8326D4C8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326D4CC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326D4D0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326D4D4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326D4D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326D4D8 size=64
    let mut pc: u32 = 0x8326D4D8;
    'dispatch: loop {
        match pc {
            0x8326D4D8 => {
    //   block [0x8326D4D8..0x8326D518)
	// 8326D4D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326D4DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326D4E0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326D4E4: 3D60820E  lis r11, -0x7df2
	ctx.r[11].s64 = -2113011712;
	// 8326D4E8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326D4EC: 388B30F4  addi r4, r11, 0x30f4
	ctx.r[4].s64 = ctx.r[11].s64 + 12532;
	// 8326D4F0: 386AC84C  addi r3, r10, -0x37b4
	ctx.r[3].s64 = ctx.r[10].s64 + -14260;
	// 8326D4F4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8326D4F8: 4AFBF9D9  bl 0x8222ced0
	ctx.lr = 0x8326D4FC;
	sub_8222CED0(ctx, base);
	// 8326D4FC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8326D500: 3869EC70  addi r3, r9, -0x1390
	ctx.r[3].s64 = ctx.r[9].s64 + -5008;
	// 8326D504: 4BA3CA1D  bl 0x82ca9f20
	ctx.lr = 0x8326D508;
	sub_82CA9F20(ctx, base);
	// 8326D508: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326D50C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326D510: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326D514: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326D518(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326D518 size=64
    let mut pc: u32 = 0x8326D518;
    'dispatch: loop {
        match pc {
            0x8326D518 => {
    //   block [0x8326D518..0x8326D558)
	// 8326D518: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326D51C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326D520: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326D524: 3D60820E  lis r11, -0x7df2
	ctx.r[11].s64 = -2113011712;
	// 8326D528: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326D52C: 388B310C  addi r4, r11, 0x310c
	ctx.r[4].s64 = ctx.r[11].s64 + 12556;
	// 8326D530: 386AC850  addi r3, r10, -0x37b0
	ctx.r[3].s64 = ctx.r[10].s64 + -14256;
	// 8326D534: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8326D538: 4AFBF999  bl 0x8222ced0
	ctx.lr = 0x8326D53C;
	sub_8222CED0(ctx, base);
	// 8326D53C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8326D540: 3869EC80  addi r3, r9, -0x1380
	ctx.r[3].s64 = ctx.r[9].s64 + -4992;
	// 8326D544: 4BA3C9DD  bl 0x82ca9f20
	ctx.lr = 0x8326D548;
	sub_82CA9F20(ctx, base);
	// 8326D548: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326D54C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326D550: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326D554: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326D558(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326D558 size=64
    let mut pc: u32 = 0x8326D558;
    'dispatch: loop {
        match pc {
            0x8326D558 => {
    //   block [0x8326D558..0x8326D598)
	// 8326D558: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326D55C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326D560: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326D564: 3D60820E  lis r11, -0x7df2
	ctx.r[11].s64 = -2113011712;
	// 8326D568: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326D56C: 388B3128  addi r4, r11, 0x3128
	ctx.r[4].s64 = ctx.r[11].s64 + 12584;
	// 8326D570: 386AC854  addi r3, r10, -0x37ac
	ctx.r[3].s64 = ctx.r[10].s64 + -14252;
	// 8326D574: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8326D578: 4AFBF959  bl 0x8222ced0
	ctx.lr = 0x8326D57C;
	sub_8222CED0(ctx, base);
	// 8326D57C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8326D580: 3869EC90  addi r3, r9, -0x1370
	ctx.r[3].s64 = ctx.r[9].s64 + -4976;
	// 8326D584: 4BA3C99D  bl 0x82ca9f20
	ctx.lr = 0x8326D588;
	sub_82CA9F20(ctx, base);
	// 8326D588: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326D58C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326D590: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326D594: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326D598(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326D598 size=64
    let mut pc: u32 = 0x8326D598;
    'dispatch: loop {
        match pc {
            0x8326D598 => {
    //   block [0x8326D598..0x8326D5D8)
	// 8326D598: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326D59C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326D5A0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326D5A4: 3D60820C  lis r11, -0x7df4
	ctx.r[11].s64 = -2113142784;
	// 8326D5A8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326D5AC: 388BC484  addi r4, r11, -0x3b7c
	ctx.r[4].s64 = ctx.r[11].s64 + -15228;
	// 8326D5B0: 386AC858  addi r3, r10, -0x37a8
	ctx.r[3].s64 = ctx.r[10].s64 + -14248;
	// 8326D5B4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8326D5B8: 4AFBF919  bl 0x8222ced0
	ctx.lr = 0x8326D5BC;
	sub_8222CED0(ctx, base);
	// 8326D5BC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8326D5C0: 3869ECA0  addi r3, r9, -0x1360
	ctx.r[3].s64 = ctx.r[9].s64 + -4960;
	// 8326D5C4: 4BA3C95D  bl 0x82ca9f20
	ctx.lr = 0x8326D5C8;
	sub_82CA9F20(ctx, base);
	// 8326D5C8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326D5CC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326D5D0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326D5D4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326D5D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326D5D8 size=64
    let mut pc: u32 = 0x8326D5D8;
    'dispatch: loop {
        match pc {
            0x8326D5D8 => {
    //   block [0x8326D5D8..0x8326D618)
	// 8326D5D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326D5DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326D5E0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326D5E4: 3D60820E  lis r11, -0x7df2
	ctx.r[11].s64 = -2113011712;
	// 8326D5E8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326D5EC: 388B118C  addi r4, r11, 0x118c
	ctx.r[4].s64 = ctx.r[11].s64 + 4492;
	// 8326D5F0: 386AC85C  addi r3, r10, -0x37a4
	ctx.r[3].s64 = ctx.r[10].s64 + -14244;
	// 8326D5F4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8326D5F8: 4AFBF8D9  bl 0x8222ced0
	ctx.lr = 0x8326D5FC;
	sub_8222CED0(ctx, base);
	// 8326D5FC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8326D600: 3869ECB0  addi r3, r9, -0x1350
	ctx.r[3].s64 = ctx.r[9].s64 + -4944;
	// 8326D604: 4BA3C91D  bl 0x82ca9f20
	ctx.lr = 0x8326D608;
	sub_82CA9F20(ctx, base);
	// 8326D608: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326D60C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326D610: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326D614: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326D618(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326D618 size=64
    let mut pc: u32 = 0x8326D618;
    'dispatch: loop {
        match pc {
            0x8326D618 => {
    //   block [0x8326D618..0x8326D658)
	// 8326D618: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326D61C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326D620: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326D624: 3D60820C  lis r11, -0x7df4
	ctx.r[11].s64 = -2113142784;
	// 8326D628: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326D62C: 388BC4AC  addi r4, r11, -0x3b54
	ctx.r[4].s64 = ctx.r[11].s64 + -15188;
	// 8326D630: 386AC860  addi r3, r10, -0x37a0
	ctx.r[3].s64 = ctx.r[10].s64 + -14240;
	// 8326D634: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8326D638: 4AFBF899  bl 0x8222ced0
	ctx.lr = 0x8326D63C;
	sub_8222CED0(ctx, base);
	// 8326D63C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8326D640: 3869ECC0  addi r3, r9, -0x1340
	ctx.r[3].s64 = ctx.r[9].s64 + -4928;
	// 8326D644: 4BA3C8DD  bl 0x82ca9f20
	ctx.lr = 0x8326D648;
	sub_82CA9F20(ctx, base);
	// 8326D648: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326D64C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326D650: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326D654: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326D658(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326D658 size=144
    let mut pc: u32 = 0x8326D658;
    'dispatch: loop {
        match pc {
            0x8326D658 => {
    //   block [0x8326D658..0x8326D67C)
	// 8326D658: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326D65C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326D660: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326D664: 38600030  li r3, 0x30
	ctx.r[3].s64 = 48;
	// 8326D668: 4AFB1BF1  bl 0x8221f258
	ctx.lr = 0x8326D66C;
	sub_8221F258(ctx, base);
	// 8326D66C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8326D670: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8326D674: 419A0008  beq cr6, 0x8326d67c
	if ctx.cr[6].eq {
	pc = 0x8326D67C; continue 'dispatch;
	}
	// 8326D678: 91430000  stw r10, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	pc = 0x8326D67C; continue 'dispatch;
            }
            0x8326D67C => {
    //   block [0x8326D67C..0x8326D688)
	// 8326D67C: 35630004  addic. r11, r3, 4
	ctx.xer.ca = (ctx.r[3].u32 > (!(4 as u32)));
	ctx.r[11].s64 = ctx.r[3].s64 + 4;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8326D680: 41820008  beq 0x8326d688
	if ctx.cr[0].eq {
	pc = 0x8326D688; continue 'dispatch;
	}
	// 8326D684: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	pc = 0x8326D688; continue 'dispatch;
            }
            0x8326D688 => {
    //   block [0x8326D688..0x8326D694)
	// 8326D688: 35630008  addic. r11, r3, 8
	ctx.xer.ca = (ctx.r[3].u32 > (!(8 as u32)));
	ctx.r[11].s64 = ctx.r[3].s64 + 8;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8326D68C: 41820008  beq 0x8326d694
	if ctx.cr[0].eq {
	pc = 0x8326D694; continue 'dispatch;
	}
	// 8326D690: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	pc = 0x8326D694; continue 'dispatch;
            }
            0x8326D694 => {
    //   block [0x8326D694..0x8326D6E8)
	// 8326D694: 3D20834A  lis r9, -0x7cb6
	ctx.r[9].s64 = -2092302336;
	// 8326D698: 99430029  stb r10, 0x29(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(41 as u32), ctx.r[10].u8 ) };
	// 8326D69C: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 8326D6A0: 3909C864  addi r8, r9, -0x379c
	ctx.r[8].s64 = ctx.r[9].s64 + -14236;
	// 8326D6A4: 99630028  stb r11, 0x28(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(40 as u32), ctx.r[11].u8 ) };
	// 8326D6A8: 3CE0832B  lis r7, -0x7cd5
	ctx.r[7].s64 = -2094333952;
	// 8326D6AC: 90680004  stw r3, 4(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(4 as u32), ctx.r[3].u32 ) };
	// 8326D6B0: 99630029  stb r11, 0x29(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(41 as u32), ctx.r[11].u8 ) };
	// 8326D6B4: 3867ECD0  addi r3, r7, -0x1330
	ctx.r[3].s64 = ctx.r[7].s64 + -4912;
	// 8326D6B8: 81680004  lwz r11, 4(r8)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(4 as u32) ) } as u64;
	// 8326D6BC: 916B0004  stw r11, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 8326D6C0: 81680004  lwz r11, 4(r8)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(4 as u32) ) } as u64;
	// 8326D6C4: 916B0000  stw r11, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8326D6C8: 81680004  lwz r11, 4(r8)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(4 as u32) ) } as u64;
	// 8326D6CC: 916B0008  stw r11, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 8326D6D0: 91480008  stw r10, 8(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 8326D6D4: 4BA3C84D  bl 0x82ca9f20
	ctx.lr = 0x8326D6D8;
	sub_82CA9F20(ctx, base);
	// 8326D6D8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326D6DC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326D6E0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326D6E4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326D6E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326D6E8 size=144
    let mut pc: u32 = 0x8326D6E8;
    'dispatch: loop {
        match pc {
            0x8326D6E8 => {
    //   block [0x8326D6E8..0x8326D70C)
	// 8326D6E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326D6EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326D6F0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326D6F4: 38600024  li r3, 0x24
	ctx.r[3].s64 = 36;
	// 8326D6F8: 4AFB1B61  bl 0x8221f258
	ctx.lr = 0x8326D6FC;
	sub_8221F258(ctx, base);
	// 8326D6FC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8326D700: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8326D704: 419A0008  beq cr6, 0x8326d70c
	if ctx.cr[6].eq {
	pc = 0x8326D70C; continue 'dispatch;
	}
	// 8326D708: 91430000  stw r10, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	pc = 0x8326D70C; continue 'dispatch;
            }
            0x8326D70C => {
    //   block [0x8326D70C..0x8326D718)
	// 8326D70C: 35630004  addic. r11, r3, 4
	ctx.xer.ca = (ctx.r[3].u32 > (!(4 as u32)));
	ctx.r[11].s64 = ctx.r[3].s64 + 4;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8326D710: 41820008  beq 0x8326d718
	if ctx.cr[0].eq {
	pc = 0x8326D718; continue 'dispatch;
	}
	// 8326D714: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	pc = 0x8326D718; continue 'dispatch;
            }
            0x8326D718 => {
    //   block [0x8326D718..0x8326D724)
	// 8326D718: 35630008  addic. r11, r3, 8
	ctx.xer.ca = (ctx.r[3].u32 > (!(8 as u32)));
	ctx.r[11].s64 = ctx.r[3].s64 + 8;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8326D71C: 41820008  beq 0x8326d724
	if ctx.cr[0].eq {
	pc = 0x8326D724; continue 'dispatch;
	}
	// 8326D720: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	pc = 0x8326D724; continue 'dispatch;
            }
            0x8326D724 => {
    //   block [0x8326D724..0x8326D778)
	// 8326D724: 3D20834A  lis r9, -0x7cb6
	ctx.r[9].s64 = -2092302336;
	// 8326D728: 99430021  stb r10, 0x21(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(33 as u32), ctx.r[10].u8 ) };
	// 8326D72C: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 8326D730: 3909C870  addi r8, r9, -0x3790
	ctx.r[8].s64 = ctx.r[9].s64 + -14224;
	// 8326D734: 99630020  stb r11, 0x20(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(32 as u32), ctx.r[11].u8 ) };
	// 8326D738: 3CE0832B  lis r7, -0x7cd5
	ctx.r[7].s64 = -2094333952;
	// 8326D73C: 90680004  stw r3, 4(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(4 as u32), ctx.r[3].u32 ) };
	// 8326D740: 99630021  stb r11, 0x21(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(33 as u32), ctx.r[11].u8 ) };
	// 8326D744: 3867ECE0  addi r3, r7, -0x1320
	ctx.r[3].s64 = ctx.r[7].s64 + -4896;
	// 8326D748: 81680004  lwz r11, 4(r8)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(4 as u32) ) } as u64;
	// 8326D74C: 916B0004  stw r11, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 8326D750: 81680004  lwz r11, 4(r8)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(4 as u32) ) } as u64;
	// 8326D754: 916B0000  stw r11, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8326D758: 81680004  lwz r11, 4(r8)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(4 as u32) ) } as u64;
	// 8326D75C: 916B0008  stw r11, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 8326D760: 91480008  stw r10, 8(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 8326D764: 4BA3C7BD  bl 0x82ca9f20
	ctx.lr = 0x8326D768;
	sub_82CA9F20(ctx, base);
	// 8326D768: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326D76C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326D770: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326D774: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326D778(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8326D778 size=12
    let mut pc: u32 = 0x8326D778;
    'dispatch: loop {
        match pc {
            0x8326D778 => {
    //   block [0x8326D778..0x8326D784)
	// 8326D778: 3D60832B  lis r11, -0x7cd5
	ctx.r[11].s64 = -2094333952;
	// 8326D77C: 386BECF0  addi r3, r11, -0x1310
	ctx.r[3].s64 = ctx.r[11].s64 + -4880;
	// 8326D780: 4BA3C7A0  b 0x82ca9f20
	sub_82CA9F20(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326D788(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326D788 size=56
    let mut pc: u32 = 0x8326D788;
    'dispatch: loop {
        match pc {
            0x8326D788 => {
    //   block [0x8326D788..0x8326D7C0)
	// 8326D788: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326D78C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326D790: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326D794: 3D60820E  lis r11, -0x7df2
	ctx.r[11].s64 = -2113011712;
	// 8326D798: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8326D79C: 386B4024  addi r3, r11, 0x4024
	ctx.r[3].s64 = ctx.r[11].s64 + 16420;
	// 8326D7A0: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8326D7A4: 4AF865B5  bl 0x821f3d58
	ctx.lr = 0x8326D7A8;
	sub_821F3D58(ctx, base);
	// 8326D7A8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326D7AC: 906AC88C  stw r3, -0x3774(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-14196 as u32), ctx.r[3].u32 ) };
	// 8326D7B0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326D7B4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326D7B8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326D7BC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326D7C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326D7C0 size=56
    let mut pc: u32 = 0x8326D7C0;
    'dispatch: loop {
        match pc {
            0x8326D7C0 => {
    //   block [0x8326D7C0..0x8326D7F8)
	// 8326D7C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326D7C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326D7C8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326D7CC: 3D60820E  lis r11, -0x7df2
	ctx.r[11].s64 = -2113011712;
	// 8326D7D0: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8326D7D4: 386B4034  addi r3, r11, 0x4034
	ctx.r[3].s64 = ctx.r[11].s64 + 16436;
	// 8326D7D8: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8326D7DC: 4AF8657D  bl 0x821f3d58
	ctx.lr = 0x8326D7E0;
	sub_821F3D58(ctx, base);
	// 8326D7E0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326D7E4: 906AC890  stw r3, -0x3770(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-14192 as u32), ctx.r[3].u32 ) };
	// 8326D7E8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326D7EC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326D7F0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326D7F4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326D7F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326D7F8 size=56
    let mut pc: u32 = 0x8326D7F8;
    'dispatch: loop {
        match pc {
            0x8326D7F8 => {
    //   block [0x8326D7F8..0x8326D830)
	// 8326D7F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326D7FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326D800: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326D804: 3D60820E  lis r11, -0x7df2
	ctx.r[11].s64 = -2113011712;
	// 8326D808: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8326D80C: 386B4048  addi r3, r11, 0x4048
	ctx.r[3].s64 = ctx.r[11].s64 + 16456;
	// 8326D810: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8326D814: 4AF86545  bl 0x821f3d58
	ctx.lr = 0x8326D818;
	sub_821F3D58(ctx, base);
	// 8326D818: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326D81C: 906AC894  stw r3, -0x376c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-14188 as u32), ctx.r[3].u32 ) };
	// 8326D820: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326D824: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326D828: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326D82C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326D830(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326D830 size=56
    let mut pc: u32 = 0x8326D830;
    'dispatch: loop {
        match pc {
            0x8326D830 => {
    //   block [0x8326D830..0x8326D868)
	// 8326D830: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326D834: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326D838: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326D83C: 3D60820E  lis r11, -0x7df2
	ctx.r[11].s64 = -2113011712;
	// 8326D840: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8326D844: 386B405C  addi r3, r11, 0x405c
	ctx.r[3].s64 = ctx.r[11].s64 + 16476;
	// 8326D848: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8326D84C: 4AF8650D  bl 0x821f3d58
	ctx.lr = 0x8326D850;
	sub_821F3D58(ctx, base);
	// 8326D850: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326D854: 906AC898  stw r3, -0x3768(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-14184 as u32), ctx.r[3].u32 ) };
	// 8326D858: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326D85C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326D860: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326D864: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326D868(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326D868 size=56
    let mut pc: u32 = 0x8326D868;
    'dispatch: loop {
        match pc {
            0x8326D868 => {
    //   block [0x8326D868..0x8326D8A0)
	// 8326D868: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326D86C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326D870: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326D874: 3D60820E  lis r11, -0x7df2
	ctx.r[11].s64 = -2113011712;
	// 8326D878: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8326D87C: 386B406C  addi r3, r11, 0x406c
	ctx.r[3].s64 = ctx.r[11].s64 + 16492;
	// 8326D880: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8326D884: 4AF864D5  bl 0x821f3d58
	ctx.lr = 0x8326D888;
	sub_821F3D58(ctx, base);
	// 8326D888: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326D88C: 906AC89C  stw r3, -0x3764(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-14180 as u32), ctx.r[3].u32 ) };
	// 8326D890: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326D894: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326D898: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326D89C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326D8A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326D8A0 size=56
    let mut pc: u32 = 0x8326D8A0;
    'dispatch: loop {
        match pc {
            0x8326D8A0 => {
    //   block [0x8326D8A0..0x8326D8D8)
	// 8326D8A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326D8A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326D8A8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326D8AC: 3D60820E  lis r11, -0x7df2
	ctx.r[11].s64 = -2113011712;
	// 8326D8B0: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8326D8B4: 386B407C  addi r3, r11, 0x407c
	ctx.r[3].s64 = ctx.r[11].s64 + 16508;
	// 8326D8B8: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8326D8BC: 4AF8649D  bl 0x821f3d58
	ctx.lr = 0x8326D8C0;
	sub_821F3D58(ctx, base);
	// 8326D8C0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326D8C4: 906AC8A0  stw r3, -0x3760(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-14176 as u32), ctx.r[3].u32 ) };
	// 8326D8C8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326D8CC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326D8D0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326D8D4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326D8D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326D8D8 size=56
    let mut pc: u32 = 0x8326D8D8;
    'dispatch: loop {
        match pc {
            0x8326D8D8 => {
    //   block [0x8326D8D8..0x8326D910)
	// 8326D8D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326D8DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326D8E0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326D8E4: 3D60820E  lis r11, -0x7df2
	ctx.r[11].s64 = -2113011712;
	// 8326D8E8: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8326D8EC: 386B408C  addi r3, r11, 0x408c
	ctx.r[3].s64 = ctx.r[11].s64 + 16524;
	// 8326D8F0: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8326D8F4: 4AF86465  bl 0x821f3d58
	ctx.lr = 0x8326D8F8;
	sub_821F3D58(ctx, base);
	// 8326D8F8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326D8FC: 906AC8A4  stw r3, -0x375c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-14172 as u32), ctx.r[3].u32 ) };
	// 8326D900: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326D904: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326D908: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326D90C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326D910(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326D910 size=56
    let mut pc: u32 = 0x8326D910;
    'dispatch: loop {
        match pc {
            0x8326D910 => {
    //   block [0x8326D910..0x8326D948)
	// 8326D910: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326D914: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326D918: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326D91C: 3D60820E  lis r11, -0x7df2
	ctx.r[11].s64 = -2113011712;
	// 8326D920: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8326D924: 386B2B20  addi r3, r11, 0x2b20
	ctx.r[3].s64 = ctx.r[11].s64 + 11040;
	// 8326D928: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8326D92C: 4AF8642D  bl 0x821f3d58
	ctx.lr = 0x8326D930;
	sub_821F3D58(ctx, base);
	// 8326D930: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326D934: 906AC8A8  stw r3, -0x3758(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-14168 as u32), ctx.r[3].u32 ) };
	// 8326D938: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326D93C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326D940: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326D944: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326D948(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326D948 size=56
    let mut pc: u32 = 0x8326D948;
    'dispatch: loop {
        match pc {
            0x8326D948 => {
    //   block [0x8326D948..0x8326D980)
	// 8326D948: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326D94C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326D950: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326D954: 3D60820E  lis r11, -0x7df2
	ctx.r[11].s64 = -2113011712;
	// 8326D958: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8326D95C: 386B2B2C  addi r3, r11, 0x2b2c
	ctx.r[3].s64 = ctx.r[11].s64 + 11052;
	// 8326D960: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8326D964: 4AF863F5  bl 0x821f3d58
	ctx.lr = 0x8326D968;
	sub_821F3D58(ctx, base);
	// 8326D968: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326D96C: 906AC8AC  stw r3, -0x3754(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-14164 as u32), ctx.r[3].u32 ) };
	// 8326D970: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326D974: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326D978: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326D97C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326D980(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326D980 size=56
    let mut pc: u32 = 0x8326D980;
    'dispatch: loop {
        match pc {
            0x8326D980 => {
    //   block [0x8326D980..0x8326D9B8)
	// 8326D980: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326D984: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326D988: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326D98C: 3D60820E  lis r11, -0x7df2
	ctx.r[11].s64 = -2113011712;
	// 8326D990: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8326D994: 386B40A0  addi r3, r11, 0x40a0
	ctx.r[3].s64 = ctx.r[11].s64 + 16544;
	// 8326D998: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8326D99C: 4AF863BD  bl 0x821f3d58
	ctx.lr = 0x8326D9A0;
	sub_821F3D58(ctx, base);
	// 8326D9A0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326D9A4: 906AC8B0  stw r3, -0x3750(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-14160 as u32), ctx.r[3].u32 ) };
	// 8326D9A8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326D9AC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326D9B0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326D9B4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326D9B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326D9B8 size=56
    let mut pc: u32 = 0x8326D9B8;
    'dispatch: loop {
        match pc {
            0x8326D9B8 => {
    //   block [0x8326D9B8..0x8326D9F0)
	// 8326D9B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326D9BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326D9C0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326D9C4: 3D60820E  lis r11, -0x7df2
	ctx.r[11].s64 = -2113011712;
	// 8326D9C8: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8326D9CC: 386B40B8  addi r3, r11, 0x40b8
	ctx.r[3].s64 = ctx.r[11].s64 + 16568;
	// 8326D9D0: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8326D9D4: 4AF86385  bl 0x821f3d58
	ctx.lr = 0x8326D9D8;
	sub_821F3D58(ctx, base);
	// 8326D9D8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326D9DC: 906AC8B4  stw r3, -0x374c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-14156 as u32), ctx.r[3].u32 ) };
	// 8326D9E0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326D9E4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326D9E8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326D9EC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326D9F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326D9F0 size=56
    let mut pc: u32 = 0x8326D9F0;
    'dispatch: loop {
        match pc {
            0x8326D9F0 => {
    //   block [0x8326D9F0..0x8326DA28)
	// 8326D9F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326D9F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326D9F8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326D9FC: 3D60820E  lis r11, -0x7df2
	ctx.r[11].s64 = -2113011712;
	// 8326DA00: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8326DA04: 386B40C8  addi r3, r11, 0x40c8
	ctx.r[3].s64 = ctx.r[11].s64 + 16584;
	// 8326DA08: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8326DA0C: 4AF8634D  bl 0x821f3d58
	ctx.lr = 0x8326DA10;
	sub_821F3D58(ctx, base);
	// 8326DA10: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326DA14: 906AC8B8  stw r3, -0x3748(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-14152 as u32), ctx.r[3].u32 ) };
	// 8326DA18: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326DA1C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326DA20: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326DA24: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326DA28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326DA28 size=56
    let mut pc: u32 = 0x8326DA28;
    'dispatch: loop {
        match pc {
            0x8326DA28 => {
    //   block [0x8326DA28..0x8326DA60)
	// 8326DA28: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326DA2C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326DA30: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326DA34: 3D60820E  lis r11, -0x7df2
	ctx.r[11].s64 = -2113011712;
	// 8326DA38: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8326DA3C: 386B40DC  addi r3, r11, 0x40dc
	ctx.r[3].s64 = ctx.r[11].s64 + 16604;
	// 8326DA40: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8326DA44: 4AF86315  bl 0x821f3d58
	ctx.lr = 0x8326DA48;
	sub_821F3D58(ctx, base);
	// 8326DA48: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326DA4C: 906AC8BC  stw r3, -0x3744(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-14148 as u32), ctx.r[3].u32 ) };
	// 8326DA50: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326DA54: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326DA58: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326DA5C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326DA60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326DA60 size=56
    let mut pc: u32 = 0x8326DA60;
    'dispatch: loop {
        match pc {
            0x8326DA60 => {
    //   block [0x8326DA60..0x8326DA98)
	// 8326DA60: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326DA64: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326DA68: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326DA6C: 3D60820E  lis r11, -0x7df2
	ctx.r[11].s64 = -2113011712;
	// 8326DA70: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8326DA74: 386B40F4  addi r3, r11, 0x40f4
	ctx.r[3].s64 = ctx.r[11].s64 + 16628;
	// 8326DA78: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8326DA7C: 4AF862DD  bl 0x821f3d58
	ctx.lr = 0x8326DA80;
	sub_821F3D58(ctx, base);
	// 8326DA80: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326DA84: 906AC8C0  stw r3, -0x3740(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-14144 as u32), ctx.r[3].u32 ) };
	// 8326DA88: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326DA8C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326DA90: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326DA94: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326DA98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326DA98 size=56
    let mut pc: u32 = 0x8326DA98;
    'dispatch: loop {
        match pc {
            0x8326DA98 => {
    //   block [0x8326DA98..0x8326DAD0)
	// 8326DA98: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326DA9C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326DAA0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326DAA4: 3D60820E  lis r11, -0x7df2
	ctx.r[11].s64 = -2113011712;
	// 8326DAA8: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8326DAAC: 386B410C  addi r3, r11, 0x410c
	ctx.r[3].s64 = ctx.r[11].s64 + 16652;
	// 8326DAB0: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8326DAB4: 4AF862A5  bl 0x821f3d58
	ctx.lr = 0x8326DAB8;
	sub_821F3D58(ctx, base);
	// 8326DAB8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326DABC: 906AC8C4  stw r3, -0x373c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-14140 as u32), ctx.r[3].u32 ) };
	// 8326DAC0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326DAC4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326DAC8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326DACC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326DAD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326DAD0 size=56
    let mut pc: u32 = 0x8326DAD0;
    'dispatch: loop {
        match pc {
            0x8326DAD0 => {
    //   block [0x8326DAD0..0x8326DB08)
	// 8326DAD0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326DAD4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326DAD8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326DADC: 3D60820E  lis r11, -0x7df2
	ctx.r[11].s64 = -2113011712;
	// 8326DAE0: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8326DAE4: 386B4128  addi r3, r11, 0x4128
	ctx.r[3].s64 = ctx.r[11].s64 + 16680;
	// 8326DAE8: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8326DAEC: 4AF8626D  bl 0x821f3d58
	ctx.lr = 0x8326DAF0;
	sub_821F3D58(ctx, base);
	// 8326DAF0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326DAF4: 906AC8C8  stw r3, -0x3738(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-14136 as u32), ctx.r[3].u32 ) };
	// 8326DAF8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326DAFC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326DB00: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326DB04: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326DB08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326DB08 size=56
    let mut pc: u32 = 0x8326DB08;
    'dispatch: loop {
        match pc {
            0x8326DB08 => {
    //   block [0x8326DB08..0x8326DB40)
	// 8326DB08: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326DB0C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326DB10: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326DB14: 3D60820E  lis r11, -0x7df2
	ctx.r[11].s64 = -2113011712;
	// 8326DB18: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8326DB1C: 386B4144  addi r3, r11, 0x4144
	ctx.r[3].s64 = ctx.r[11].s64 + 16708;
	// 8326DB20: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8326DB24: 4AF86235  bl 0x821f3d58
	ctx.lr = 0x8326DB28;
	sub_821F3D58(ctx, base);
	// 8326DB28: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326DB2C: 906AC8CC  stw r3, -0x3734(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-14132 as u32), ctx.r[3].u32 ) };
	// 8326DB30: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326DB34: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326DB38: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326DB3C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326DB40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326DB40 size=56
    let mut pc: u32 = 0x8326DB40;
    'dispatch: loop {
        match pc {
            0x8326DB40 => {
    //   block [0x8326DB40..0x8326DB78)
	// 8326DB40: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326DB44: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326DB48: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326DB4C: 3D60820E  lis r11, -0x7df2
	ctx.r[11].s64 = -2113011712;
	// 8326DB50: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8326DB54: 386B4154  addi r3, r11, 0x4154
	ctx.r[3].s64 = ctx.r[11].s64 + 16724;
	// 8326DB58: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8326DB5C: 4AF861FD  bl 0x821f3d58
	ctx.lr = 0x8326DB60;
	sub_821F3D58(ctx, base);
	// 8326DB60: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326DB64: 906AC8D0  stw r3, -0x3730(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-14128 as u32), ctx.r[3].u32 ) };
	// 8326DB68: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326DB6C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326DB70: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326DB74: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326DB78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8326DB78 size=76
    let mut pc: u32 = 0x8326DB78;
    'dispatch: loop {
        match pc {
            0x8326DB78 => {
    //   block [0x8326DB78..0x8326DBC4)
	// 8326DB78: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326DB7C: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 8326DB80: 3D00834A  lis r8, -0x7cb6
	ctx.r[8].s64 = -2092302336;
	// 8326DB84: 3CE0834A  lis r7, -0x7cb6
	ctx.r[7].s64 = -2092302336;
	// 8326DB88: 3CC0834A  lis r6, -0x7cb6
	ctx.r[6].s64 = -2092302336;
	// 8326DB8C: 812AC8B4  lwz r9, -0x374c(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-14156 as u32) ) } as u64;
	// 8326DB90: 816BC89C  lwz r11, -0x3764(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-14180 as u32) ) } as u64;
	// 8326DB94: 38A6C8D4  addi r5, r6, -0x372c
	ctx.r[5].s64 = ctx.r[6].s64 + -14124;
	// 8326DB98: 8108C8B8  lwz r8, -0x3748(r8)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(-14152 as u32) ) } as u64;
	// 8326DB9C: 8147C8A4  lwz r10, -0x375c(r7)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(-14172 as u32) ) } as u64;
	// 8326DBA0: 9166C8D4  stw r11, -0x372c(r6)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(-14124 as u32), ctx.r[11].u32 ) };
	// 8326DBA4: 91250004  stw r9, 4(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 8326DBA8: 91650008  stw r11, 8(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 8326DBAC: 9105000C  stw r8, 0xc(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(12 as u32), ctx.r[8].u32 ) };
	// 8326DBB0: 91450010  stw r10, 0x10(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(16 as u32), ctx.r[10].u32 ) };
	// 8326DBB4: 91650014  stw r11, 0x14(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(20 as u32), ctx.r[11].u32 ) };
	// 8326DBB8: 91450018  stw r10, 0x18(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(24 as u32), ctx.r[10].u32 ) };
	// 8326DBBC: 9165001C  stw r11, 0x1c(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(28 as u32), ctx.r[11].u32 ) };
	// 8326DBC0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326DBC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326DBC8 size=164
    let mut pc: u32 = 0x8326DBC8;
    'dispatch: loop {
        match pc {
            0x8326DBC8 => {
    //   block [0x8326DBC8..0x8326DC6C)
	// 8326DBC8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326DBCC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326DBD0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8326DBD4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8326DBD8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326DBDC: 3D60811C  lis r11, -0x7ee4
	ctx.r[11].s64 = -2128871424;
	// 8326DBE0: 3D40820E  lis r10, -0x7df2
	ctx.r[10].s64 = -2113011712;
	// 8326DBE4: 617F9DC5  ori r31, r11, 0x9dc5
	ctx.r[31].u64 = ctx.r[11].u64 | 40389;
	// 8326DBE8: 386A418C  addi r3, r10, 0x418c
	ctx.r[3].s64 = ctx.r[10].s64 + 16780;
	// 8326DBEC: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8326DBF0: 4AF86169  bl 0x821f3d58
	ctx.lr = 0x8326DBF4;
	sub_821F3D58(ctx, base);
	// 8326DBF4: 3D20834A  lis r9, -0x7cb6
	ctx.r[9].s64 = -2092302336;
	// 8326DBF8: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 8326DBFC: 3D00820E  lis r8, -0x7df2
	ctx.r[8].s64 = -2113011712;
	// 8326DC00: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8326DC04: 38684184  addi r3, r8, 0x4184
	ctx.r[3].s64 = ctx.r[8].s64 + 16772;
	// 8326DC08: 9169C8F4  stw r11, -0x370c(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(-14092 as u32), ctx.r[11].u32 ) };
	// 8326DC0C: 3BC9C8F4  addi r30, r9, -0x370c
	ctx.r[30].s64 = ctx.r[9].s64 + -14092;
	// 8326DC10: 4AF86149  bl 0x821f3d58
	ctx.lr = 0x8326DC14;
	sub_821F3D58(ctx, base);
	// 8326DC14: 907E0004  stw r3, 4(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(4 as u32), ctx.r[3].u32 ) };
	// 8326DC18: 3CE0820E  lis r7, -0x7df2
	ctx.r[7].s64 = -2113011712;
	// 8326DC1C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8326DC20: 3867417C  addi r3, r7, 0x417c
	ctx.r[3].s64 = ctx.r[7].s64 + 16764;
	// 8326DC24: 4AF86135  bl 0x821f3d58
	ctx.lr = 0x8326DC28;
	sub_821F3D58(ctx, base);
	// 8326DC28: 907E0008  stw r3, 8(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(8 as u32), ctx.r[3].u32 ) };
	// 8326DC2C: 3CC0820E  lis r6, -0x7df2
	ctx.r[6].s64 = -2113011712;
	// 8326DC30: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8326DC34: 38664174  addi r3, r6, 0x4174
	ctx.r[3].s64 = ctx.r[6].s64 + 16756;
	// 8326DC38: 4AF86121  bl 0x821f3d58
	ctx.lr = 0x8326DC3C;
	sub_821F3D58(ctx, base);
	// 8326DC3C: 907E000C  stw r3, 0xc(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(12 as u32), ctx.r[3].u32 ) };
	// 8326DC40: 3CA0820E  lis r5, -0x7df2
	ctx.r[5].s64 = -2113011712;
	// 8326DC44: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8326DC48: 3865416C  addi r3, r5, 0x416c
	ctx.r[3].s64 = ctx.r[5].s64 + 16748;
	// 8326DC4C: 4AF8610D  bl 0x821f3d58
	ctx.lr = 0x8326DC50;
	sub_821F3D58(ctx, base);
	// 8326DC50: 907E0010  stw r3, 0x10(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(16 as u32), ctx.r[3].u32 ) };
	// 8326DC54: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8326DC58: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326DC5C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326DC60: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8326DC64: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8326DC68: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326DC70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8326DC70 size=12
    let mut pc: u32 = 0x8326DC70;
    'dispatch: loop {
        match pc {
            0x8326DC70 => {
    //   block [0x8326DC70..0x8326DC7C)
	// 8326DC70: 3D60832B  lis r11, -0x7cd5
	ctx.r[11].s64 = -2094333952;
	// 8326DC74: 386BED58  addi r3, r11, -0x12a8
	ctx.r[3].s64 = ctx.r[11].s64 + -4776;
	// 8326DC78: 4BA3C2A8  b 0x82ca9f20
	sub_82CA9F20(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326DC80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326DC80 size=68
    let mut pc: u32 = 0x8326DC80;
    'dispatch: loop {
        match pc {
            0x8326DC80 => {
    //   block [0x8326DC80..0x8326DCC4)
	// 8326DC80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326DC84: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326DC88: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326DC8C: 3D608290  lis r11, -0x7d70
	ctx.r[11].s64 = -2104492032;
	// 8326DC90: 3D40820E  lis r10, -0x7df2
	ctx.r[10].s64 = -2113011712;
	// 8326DC94: 3D20834A  lis r9, -0x7cb6
	ctx.r[9].s64 = -2092302336;
	// 8326DC98: 388A4D9C  addi r4, r10, 0x4d9c
	ctx.r[4].s64 = ctx.r[10].s64 + 19868;
	// 8326DC9C: 3869C918  addi r3, r9, -0x36e8
	ctx.r[3].s64 = ctx.r[9].s64 + -14056;
	// 8326DCA0: 38ABCA90  addi r5, r11, -0x3570
	ctx.r[5].s64 = ctx.r[11].s64 + -13680;
	// 8326DCA4: 4BC2AB3D  bl 0x82e987e0
	ctx.lr = 0x8326DCA8;
	sub_82E987E0(ctx, base);
	// 8326DCA8: 3D00832B  lis r8, -0x7cd5
	ctx.r[8].s64 = -2094333952;
	// 8326DCAC: 3868ED68  addi r3, r8, -0x1298
	ctx.r[3].s64 = ctx.r[8].s64 + -4760;
	// 8326DCB0: 4BA3C271  bl 0x82ca9f20
	ctx.lr = 0x8326DCB4;
	sub_82CA9F20(ctx, base);
	// 8326DCB4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326DCB8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326DCBC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326DCC0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326DCC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326DCC8 size=76
    let mut pc: u32 = 0x8326DCC8;
    'dispatch: loop {
        match pc {
            0x8326DCC8 => {
    //   block [0x8326DCC8..0x8326DD14)
	// 8326DCC8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326DCCC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326DCD0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326DCD4: 3D608334  lis r11, -0x7ccc
	ctx.r[11].s64 = -2093744128;
	// 8326DCD8: 3D408290  lis r10, -0x7d70
	ctx.r[10].s64 = -2104492032;
	// 8326DCDC: 3D20820E  lis r9, -0x7df2
	ctx.r[9].s64 = -2113011712;
	// 8326DCE0: 3D00834A  lis r8, -0x7cb6
	ctx.r[8].s64 = -2092302336;
	// 8326DCE4: 38CB581C  addi r6, r11, 0x581c
	ctx.r[6].s64 = ctx.r[11].s64 + 22556;
	// 8326DCE8: 38894DC8  addi r4, r9, 0x4dc8
	ctx.r[4].s64 = ctx.r[9].s64 + 19912;
	// 8326DCEC: 3868CA2C  addi r3, r8, -0x35d4
	ctx.r[3].s64 = ctx.r[8].s64 + -13780;
	// 8326DCF0: 38AAD860  addi r5, r10, -0x27a0
	ctx.r[5].s64 = ctx.r[10].s64 + -10144;
	// 8326DCF4: 4BC17F05  bl 0x82e85bf8
	ctx.lr = 0x8326DCF8;
	sub_82E85BF8(ctx, base);
	// 8326DCF8: 3CE0832B  lis r7, -0x7cd5
	ctx.r[7].s64 = -2094333952;
	// 8326DCFC: 3867ED80  addi r3, r7, -0x1280
	ctx.r[3].s64 = ctx.r[7].s64 + -4736;
	// 8326DD00: 4BA3C221  bl 0x82ca9f20
	ctx.lr = 0x8326DD04;
	sub_82CA9F20(ctx, base);
	// 8326DD04: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326DD08: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326DD0C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326DD10: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326DD18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x8326DD18 size=24
    let mut pc: u32 = 0x8326DD18;
    'dispatch: loop {
        match pc {
            0x8326DD18 => {
    //   block [0x8326DD18..0x8326DD30)
	// 8326DD18: 3D608332  lis r11, -0x7cce
	ctx.r[11].s64 = -2093875200;
	// 8326DD1C: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326DD20: C00B8EF0  lfs f0, -0x7110(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-28944 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8326DD24: EC000032  fmuls f0, f0, f0
	ctx.f[0].f64 = (((ctx.f[0].f64 * ctx.f[0].f64) as f32) as f64);
	// 8326DD28: D00ACB40  stfs f0, -0x34c0(r10)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-13504 as u32), tmp.u32 ) };
	// 8326DD2C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326DD30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326DD30 size=64
    let mut pc: u32 = 0x8326DD30;
    'dispatch: loop {
        match pc {
            0x8326DD30 => {
    //   block [0x8326DD30..0x8326DD70)
	// 8326DD30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326DD34: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326DD38: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326DD3C: 3D60820E  lis r11, -0x7df2
	ctx.r[11].s64 = -2113011712;
	// 8326DD40: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326DD44: 388B5380  addi r4, r11, 0x5380
	ctx.r[4].s64 = ctx.r[11].s64 + 21376;
	// 8326DD48: 386ACB44  addi r3, r10, -0x34bc
	ctx.r[3].s64 = ctx.r[10].s64 + -13500;
	// 8326DD4C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8326DD50: 4AFBF181  bl 0x8222ced0
	ctx.lr = 0x8326DD54;
	sub_8222CED0(ctx, base);
	// 8326DD54: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8326DD58: 3869ED98  addi r3, r9, -0x1268
	ctx.r[3].s64 = ctx.r[9].s64 + -4712;
	// 8326DD5C: 4BA3C1C5  bl 0x82ca9f20
	ctx.lr = 0x8326DD60;
	sub_82CA9F20(ctx, base);
	// 8326DD60: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326DD64: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326DD68: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326DD6C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326DD70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8326DD70 size=12
    let mut pc: u32 = 0x8326DD70;
    'dispatch: loop {
        match pc {
            0x8326DD70 => {
    //   block [0x8326DD70..0x8326DD7C)
	// 8326DD70: 3D60832B  lis r11, -0x7cd5
	ctx.r[11].s64 = -2094333952;
	// 8326DD74: 386BEDA8  addi r3, r11, -0x1258
	ctx.r[3].s64 = ctx.r[11].s64 + -4696;
	// 8326DD78: 4BA3C1A8  b 0x82ca9f20
	sub_82CA9F20(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326DD80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326DD80 size=64
    let mut pc: u32 = 0x8326DD80;
    'dispatch: loop {
        match pc {
            0x8326DD80 => {
    //   block [0x8326DD80..0x8326DDC0)
	// 8326DD80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326DD84: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326DD88: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326DD8C: 3D60820E  lis r11, -0x7df2
	ctx.r[11].s64 = -2113011712;
	// 8326DD90: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326DD94: 388B53BC  addi r4, r11, 0x53bc
	ctx.r[4].s64 = ctx.r[11].s64 + 21436;
	// 8326DD98: 386ACB58  addi r3, r10, -0x34a8
	ctx.r[3].s64 = ctx.r[10].s64 + -13480;
	// 8326DD9C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8326DDA0: 4AFBF131  bl 0x8222ced0
	ctx.lr = 0x8326DDA4;
	sub_8222CED0(ctx, base);
	// 8326DDA4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8326DDA8: 3869EE00  addi r3, r9, -0x1200
	ctx.r[3].s64 = ctx.r[9].s64 + -4608;
	// 8326DDAC: 4BA3C175  bl 0x82ca9f20
	ctx.lr = 0x8326DDB0;
	sub_82CA9F20(ctx, base);
	// 8326DDB0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326DDB4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326DDB8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326DDBC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326DDC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326DDC0 size=64
    let mut pc: u32 = 0x8326DDC0;
    'dispatch: loop {
        match pc {
            0x8326DDC0 => {
    //   block [0x8326DDC0..0x8326DE00)
	// 8326DDC0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326DDC4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326DDC8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326DDCC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8326DDD0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326DDD4: 388B052C  addi r4, r11, 0x52c
	ctx.r[4].s64 = ctx.r[11].s64 + 1324;
	// 8326DDD8: 386ACB5C  addi r3, r10, -0x34a4
	ctx.r[3].s64 = ctx.r[10].s64 + -13476;
	// 8326DDDC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8326DDE0: 4AFBF0F1  bl 0x8222ced0
	ctx.lr = 0x8326DDE4;
	sub_8222CED0(ctx, base);
	// 8326DDE4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8326DDE8: 3869EE10  addi r3, r9, -0x11f0
	ctx.r[3].s64 = ctx.r[9].s64 + -4592;
	// 8326DDEC: 4BA3C135  bl 0x82ca9f20
	ctx.lr = 0x8326DDF0;
	sub_82CA9F20(ctx, base);
	// 8326DDF0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326DDF4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326DDF8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326DDFC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326DE00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326DE00 size=64
    let mut pc: u32 = 0x8326DE00;
    'dispatch: loop {
        match pc {
            0x8326DE00 => {
    //   block [0x8326DE00..0x8326DE40)
	// 8326DE00: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326DE04: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326DE08: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326DE0C: 3D60820E  lis r11, -0x7df2
	ctx.r[11].s64 = -2113011712;
	// 8326DE10: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326DE14: 388B9620  addi r4, r11, -0x69e0
	ctx.r[4].s64 = ctx.r[11].s64 + -27104;
	// 8326DE18: 386ACB60  addi r3, r10, -0x34a0
	ctx.r[3].s64 = ctx.r[10].s64 + -13472;
	// 8326DE1C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8326DE20: 4AFBF0B1  bl 0x8222ced0
	ctx.lr = 0x8326DE24;
	sub_8222CED0(ctx, base);
	// 8326DE24: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8326DE28: 3869EE20  addi r3, r9, -0x11e0
	ctx.r[3].s64 = ctx.r[9].s64 + -4576;
	// 8326DE2C: 4BA3C0F5  bl 0x82ca9f20
	ctx.lr = 0x8326DE30;
	sub_82CA9F20(ctx, base);
	// 8326DE30: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326DE34: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326DE38: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326DE3C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326DE40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326DE40 size=56
    let mut pc: u32 = 0x8326DE40;
    'dispatch: loop {
        match pc {
            0x8326DE40 => {
    //   block [0x8326DE40..0x8326DE78)
	// 8326DE40: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326DE44: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326DE48: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326DE4C: 3D60820E  lis r11, -0x7df2
	ctx.r[11].s64 = -2113011712;
	// 8326DE50: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8326DE54: 386B5CC8  addi r3, r11, 0x5cc8
	ctx.r[3].s64 = ctx.r[11].s64 + 23752;
	// 8326DE58: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8326DE5C: 4AF85EFD  bl 0x821f3d58
	ctx.lr = 0x8326DE60;
	sub_821F3D58(ctx, base);
	// 8326DE60: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326DE64: 906ACB64  stw r3, -0x349c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-13468 as u32), ctx.r[3].u32 ) };
	// 8326DE68: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326DE6C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326DE70: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326DE74: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326DE78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326DE78 size=56
    let mut pc: u32 = 0x8326DE78;
    'dispatch: loop {
        match pc {
            0x8326DE78 => {
    //   block [0x8326DE78..0x8326DEB0)
	// 8326DE78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326DE7C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326DE80: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326DE84: 3D60820E  lis r11, -0x7df2
	ctx.r[11].s64 = -2113011712;
	// 8326DE88: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8326DE8C: 386B5CDC  addi r3, r11, 0x5cdc
	ctx.r[3].s64 = ctx.r[11].s64 + 23772;
	// 8326DE90: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8326DE94: 4AF85EC5  bl 0x821f3d58
	ctx.lr = 0x8326DE98;
	sub_821F3D58(ctx, base);
	// 8326DE98: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326DE9C: 906ACB68  stw r3, -0x3498(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-13464 as u32), ctx.r[3].u32 ) };
	// 8326DEA0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326DEA4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326DEA8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326DEAC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326DEB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326DEB0 size=56
    let mut pc: u32 = 0x8326DEB0;
    'dispatch: loop {
        match pc {
            0x8326DEB0 => {
    //   block [0x8326DEB0..0x8326DEE8)
	// 8326DEB0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326DEB4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326DEB8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326DEBC: 3D60820E  lis r11, -0x7df2
	ctx.r[11].s64 = -2113011712;
	// 8326DEC0: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8326DEC4: 386B5FA0  addi r3, r11, 0x5fa0
	ctx.r[3].s64 = ctx.r[11].s64 + 24480;
	// 8326DEC8: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8326DECC: 4AF85E8D  bl 0x821f3d58
	ctx.lr = 0x8326DED0;
	sub_821F3D58(ctx, base);
	// 8326DED0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326DED4: 906ACB6C  stw r3, -0x3494(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-13460 as u32), ctx.r[3].u32 ) };
	// 8326DED8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326DEDC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326DEE0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326DEE4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326DEE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326DEE8 size=56
    let mut pc: u32 = 0x8326DEE8;
    'dispatch: loop {
        match pc {
            0x8326DEE8 => {
    //   block [0x8326DEE8..0x8326DF20)
	// 8326DEE8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326DEEC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326DEF0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326DEF4: 3D60820E  lis r11, -0x7df2
	ctx.r[11].s64 = -2113011712;
	// 8326DEF8: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8326DEFC: 386B5FB8  addi r3, r11, 0x5fb8
	ctx.r[3].s64 = ctx.r[11].s64 + 24504;
	// 8326DF00: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8326DF04: 4AF85E55  bl 0x821f3d58
	ctx.lr = 0x8326DF08;
	sub_821F3D58(ctx, base);
	// 8326DF08: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326DF0C: 906ACB70  stw r3, -0x3490(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-13456 as u32), ctx.r[3].u32 ) };
	// 8326DF10: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326DF14: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326DF18: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326DF1C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326DF20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326DF20 size=56
    let mut pc: u32 = 0x8326DF20;
    'dispatch: loop {
        match pc {
            0x8326DF20 => {
    //   block [0x8326DF20..0x8326DF58)
	// 8326DF20: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326DF24: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326DF28: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326DF2C: 3D60820E  lis r11, -0x7df2
	ctx.r[11].s64 = -2113011712;
	// 8326DF30: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8326DF34: 386B6428  addi r3, r11, 0x6428
	ctx.r[3].s64 = ctx.r[11].s64 + 25640;
	// 8326DF38: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8326DF3C: 4AF85E1D  bl 0x821f3d58
	ctx.lr = 0x8326DF40;
	sub_821F3D58(ctx, base);
	// 8326DF40: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326DF44: 906ACB74  stw r3, -0x348c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-13452 as u32), ctx.r[3].u32 ) };
	// 8326DF48: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326DF4C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326DF50: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326DF54: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326DF58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326DF58 size=64
    let mut pc: u32 = 0x8326DF58;
    'dispatch: loop {
        match pc {
            0x8326DF58 => {
    //   block [0x8326DF58..0x8326DF98)
	// 8326DF58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326DF5C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326DF60: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326DF64: 3D60820E  lis r11, -0x7df2
	ctx.r[11].s64 = -2113011712;
	// 8326DF68: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326DF6C: 388B74A8  addi r4, r11, 0x74a8
	ctx.r[4].s64 = ctx.r[11].s64 + 29864;
	// 8326DF70: 386ACB78  addi r3, r10, -0x3488
	ctx.r[3].s64 = ctx.r[10].s64 + -13448;
	// 8326DF74: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8326DF78: 4AFBEF59  bl 0x8222ced0
	ctx.lr = 0x8326DF7C;
	sub_8222CED0(ctx, base);
	// 8326DF7C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8326DF80: 3869EE30  addi r3, r9, -0x11d0
	ctx.r[3].s64 = ctx.r[9].s64 + -4560;
	// 8326DF84: 4BA3BF9D  bl 0x82ca9f20
	ctx.lr = 0x8326DF88;
	sub_82CA9F20(ctx, base);
	// 8326DF88: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326DF8C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326DF90: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326DF94: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326DF98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326DF98 size=64
    let mut pc: u32 = 0x8326DF98;
    'dispatch: loop {
        match pc {
            0x8326DF98 => {
    //   block [0x8326DF98..0x8326DFD8)
	// 8326DF98: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326DF9C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326DFA0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326DFA4: 3D60820E  lis r11, -0x7df2
	ctx.r[11].s64 = -2113011712;
	// 8326DFA8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326DFAC: 388B74B4  addi r4, r11, 0x74b4
	ctx.r[4].s64 = ctx.r[11].s64 + 29876;
	// 8326DFB0: 386ACB7C  addi r3, r10, -0x3484
	ctx.r[3].s64 = ctx.r[10].s64 + -13444;
	// 8326DFB4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8326DFB8: 4AFBEF19  bl 0x8222ced0
	ctx.lr = 0x8326DFBC;
	sub_8222CED0(ctx, base);
	// 8326DFBC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8326DFC0: 3869EE40  addi r3, r9, -0x11c0
	ctx.r[3].s64 = ctx.r[9].s64 + -4544;
	// 8326DFC4: 4BA3BF5D  bl 0x82ca9f20
	ctx.lr = 0x8326DFC8;
	sub_82CA9F20(ctx, base);
	// 8326DFC8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326DFCC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326DFD0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326DFD4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326DFD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326DFD8 size=64
    let mut pc: u32 = 0x8326DFD8;
    'dispatch: loop {
        match pc {
            0x8326DFD8 => {
    //   block [0x8326DFD8..0x8326E018)
	// 8326DFD8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326DFDC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326DFE0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326DFE4: 3D60820E  lis r11, -0x7df2
	ctx.r[11].s64 = -2113011712;
	// 8326DFE8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326DFEC: 388B74C0  addi r4, r11, 0x74c0
	ctx.r[4].s64 = ctx.r[11].s64 + 29888;
	// 8326DFF0: 386ACB80  addi r3, r10, -0x3480
	ctx.r[3].s64 = ctx.r[10].s64 + -13440;
	// 8326DFF4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8326DFF8: 4AFBEED9  bl 0x8222ced0
	ctx.lr = 0x8326DFFC;
	sub_8222CED0(ctx, base);
	// 8326DFFC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8326E000: 3869EE50  addi r3, r9, -0x11b0
	ctx.r[3].s64 = ctx.r[9].s64 + -4528;
	// 8326E004: 4BA3BF1D  bl 0x82ca9f20
	ctx.lr = 0x8326E008;
	sub_82CA9F20(ctx, base);
	// 8326E008: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326E00C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326E010: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326E014: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326E018(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326E018 size=64
    let mut pc: u32 = 0x8326E018;
    'dispatch: loop {
        match pc {
            0x8326E018 => {
    //   block [0x8326E018..0x8326E058)
	// 8326E018: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326E01C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326E020: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326E024: 3D60820E  lis r11, -0x7df2
	ctx.r[11].s64 = -2113011712;
	// 8326E028: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326E02C: 388B74CC  addi r4, r11, 0x74cc
	ctx.r[4].s64 = ctx.r[11].s64 + 29900;
	// 8326E030: 386ACB84  addi r3, r10, -0x347c
	ctx.r[3].s64 = ctx.r[10].s64 + -13436;
	// 8326E034: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8326E038: 4AFBEE99  bl 0x8222ced0
	ctx.lr = 0x8326E03C;
	sub_8222CED0(ctx, base);
	// 8326E03C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8326E040: 3869EE60  addi r3, r9, -0x11a0
	ctx.r[3].s64 = ctx.r[9].s64 + -4512;
	// 8326E044: 4BA3BEDD  bl 0x82ca9f20
	ctx.lr = 0x8326E048;
	sub_82CA9F20(ctx, base);
	// 8326E048: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326E04C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326E050: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326E054: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326E058(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326E058 size=64
    let mut pc: u32 = 0x8326E058;
    'dispatch: loop {
        match pc {
            0x8326E058 => {
    //   block [0x8326E058..0x8326E098)
	// 8326E058: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326E05C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326E060: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326E064: 3D60820E  lis r11, -0x7df2
	ctx.r[11].s64 = -2113011712;
	// 8326E068: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326E06C: 388B74D8  addi r4, r11, 0x74d8
	ctx.r[4].s64 = ctx.r[11].s64 + 29912;
	// 8326E070: 386ACB88  addi r3, r10, -0x3478
	ctx.r[3].s64 = ctx.r[10].s64 + -13432;
	// 8326E074: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8326E078: 4AFBEE59  bl 0x8222ced0
	ctx.lr = 0x8326E07C;
	sub_8222CED0(ctx, base);
	// 8326E07C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8326E080: 3869EE70  addi r3, r9, -0x1190
	ctx.r[3].s64 = ctx.r[9].s64 + -4496;
	// 8326E084: 4BA3BE9D  bl 0x82ca9f20
	ctx.lr = 0x8326E088;
	sub_82CA9F20(ctx, base);
	// 8326E088: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326E08C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326E090: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326E094: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326E098(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326E098 size=64
    let mut pc: u32 = 0x8326E098;
    'dispatch: loop {
        match pc {
            0x8326E098 => {
    //   block [0x8326E098..0x8326E0D8)
	// 8326E098: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326E09C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326E0A0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326E0A4: 3D60820E  lis r11, -0x7df2
	ctx.r[11].s64 = -2113011712;
	// 8326E0A8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326E0AC: 388B74E4  addi r4, r11, 0x74e4
	ctx.r[4].s64 = ctx.r[11].s64 + 29924;
	// 8326E0B0: 386ACB8C  addi r3, r10, -0x3474
	ctx.r[3].s64 = ctx.r[10].s64 + -13428;
	// 8326E0B4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8326E0B8: 4AFBEE19  bl 0x8222ced0
	ctx.lr = 0x8326E0BC;
	sub_8222CED0(ctx, base);
	// 8326E0BC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8326E0C0: 3869EE80  addi r3, r9, -0x1180
	ctx.r[3].s64 = ctx.r[9].s64 + -4480;
	// 8326E0C4: 4BA3BE5D  bl 0x82ca9f20
	ctx.lr = 0x8326E0C8;
	sub_82CA9F20(ctx, base);
	// 8326E0C8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326E0CC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326E0D0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326E0D4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326E0D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8326E0D8 size=32
    let mut pc: u32 = 0x8326E0D8;
    'dispatch: loop {
        match pc {
            0x8326E0D8 => {
    //   block [0x8326E0D8..0x8326E0F8)
	// 8326E0D8: 3D608332  lis r11, -0x7cce
	ctx.r[11].s64 = -2093875200;
	// 8326E0DC: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326E0E0: C80B9660  lfd f0, -0x69a0(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[0].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(-27040 as u32) ) };
	// 8326E0E4: FC00001E  fctiwz f0, f0
	ctx.f[0].s64 = if ctx.f[0].f64 > (i32::MAX as f64) { i32::MAX as i64 } else { ctx.f[0].f64.trunc() as i32 as i64 };
	// 8326E0E8: D801FFF0  stfd f0, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.f[0].u64 ) };
	// 8326E0EC: 8161FFF4  lwz r11, -0xc(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-12 as u32) ) } as u64;
	// 8326E0F0: 916ACB90  stw r11, -0x3470(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-13424 as u32), ctx.r[11].u32 ) };
	// 8326E0F4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326E0F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326E0F8 size=64
    let mut pc: u32 = 0x8326E0F8;
    'dispatch: loop {
        match pc {
            0x8326E0F8 => {
    //   block [0x8326E0F8..0x8326E138)
	// 8326E0F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326E0FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326E100: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326E104: 3D60820E  lis r11, -0x7df2
	ctx.r[11].s64 = -2113011712;
	// 8326E108: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326E10C: 388B7BB8  addi r4, r11, 0x7bb8
	ctx.r[4].s64 = ctx.r[11].s64 + 31672;
	// 8326E110: 386ACB94  addi r3, r10, -0x346c
	ctx.r[3].s64 = ctx.r[10].s64 + -13420;
	// 8326E114: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8326E118: 4AFBEDB9  bl 0x8222ced0
	ctx.lr = 0x8326E11C;
	sub_8222CED0(ctx, base);
	// 8326E11C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8326E120: 3869EE90  addi r3, r9, -0x1170
	ctx.r[3].s64 = ctx.r[9].s64 + -4464;
	// 8326E124: 4BA3BDFD  bl 0x82ca9f20
	ctx.lr = 0x8326E128;
	sub_82CA9F20(ctx, base);
	// 8326E128: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326E12C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326E130: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326E134: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326E138(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326E138 size=64
    let mut pc: u32 = 0x8326E138;
    'dispatch: loop {
        match pc {
            0x8326E138 => {
    //   block [0x8326E138..0x8326E178)
	// 8326E138: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326E13C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326E140: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326E144: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8326E148: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326E14C: 388B1AD4  addi r4, r11, 0x1ad4
	ctx.r[4].s64 = ctx.r[11].s64 + 6868;
	// 8326E150: 386ACB98  addi r3, r10, -0x3468
	ctx.r[3].s64 = ctx.r[10].s64 + -13416;
	// 8326E154: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8326E158: 4AFBED79  bl 0x8222ced0
	ctx.lr = 0x8326E15C;
	sub_8222CED0(ctx, base);
	// 8326E15C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8326E160: 3869EEA0  addi r3, r9, -0x1160
	ctx.r[3].s64 = ctx.r[9].s64 + -4448;
	// 8326E164: 4BA3BDBD  bl 0x82ca9f20
	ctx.lr = 0x8326E168;
	sub_82CA9F20(ctx, base);
	// 8326E168: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326E16C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326E170: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326E174: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326E178(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326E178 size=64
    let mut pc: u32 = 0x8326E178;
    'dispatch: loop {
        match pc {
            0x8326E178 => {
    //   block [0x8326E178..0x8326E1B8)
	// 8326E178: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326E17C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326E180: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326E184: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8326E188: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326E18C: 388B1ADC  addi r4, r11, 0x1adc
	ctx.r[4].s64 = ctx.r[11].s64 + 6876;
	// 8326E190: 386ACB9C  addi r3, r10, -0x3464
	ctx.r[3].s64 = ctx.r[10].s64 + -13412;
	// 8326E194: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8326E198: 4AFBED39  bl 0x8222ced0
	ctx.lr = 0x8326E19C;
	sub_8222CED0(ctx, base);
	// 8326E19C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8326E1A0: 3869EEB0  addi r3, r9, -0x1150
	ctx.r[3].s64 = ctx.r[9].s64 + -4432;
	// 8326E1A4: 4BA3BD7D  bl 0x82ca9f20
	ctx.lr = 0x8326E1A8;
	sub_82CA9F20(ctx, base);
	// 8326E1A8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326E1AC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326E1B0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326E1B4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326E1B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326E1B8 size=64
    let mut pc: u32 = 0x8326E1B8;
    'dispatch: loop {
        match pc {
            0x8326E1B8 => {
    //   block [0x8326E1B8..0x8326E1F8)
	// 8326E1B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326E1BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326E1C0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326E1C4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8326E1C8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326E1CC: 388B1AF0  addi r4, r11, 0x1af0
	ctx.r[4].s64 = ctx.r[11].s64 + 6896;
	// 8326E1D0: 386ACBA0  addi r3, r10, -0x3460
	ctx.r[3].s64 = ctx.r[10].s64 + -13408;
	// 8326E1D4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8326E1D8: 4AFBECF9  bl 0x8222ced0
	ctx.lr = 0x8326E1DC;
	sub_8222CED0(ctx, base);
	// 8326E1DC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8326E1E0: 3869EEC0  addi r3, r9, -0x1140
	ctx.r[3].s64 = ctx.r[9].s64 + -4416;
	// 8326E1E4: 4BA3BD3D  bl 0x82ca9f20
	ctx.lr = 0x8326E1E8;
	sub_82CA9F20(ctx, base);
	// 8326E1E8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326E1EC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326E1F0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326E1F4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326E1F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326E1F8 size=64
    let mut pc: u32 = 0x8326E1F8;
    'dispatch: loop {
        match pc {
            0x8326E1F8 => {
    //   block [0x8326E1F8..0x8326E238)
	// 8326E1F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326E1FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326E200: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326E204: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8326E208: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326E20C: 388B1AD4  addi r4, r11, 0x1ad4
	ctx.r[4].s64 = ctx.r[11].s64 + 6868;
	// 8326E210: 386ACBA4  addi r3, r10, -0x345c
	ctx.r[3].s64 = ctx.r[10].s64 + -13404;
	// 8326E214: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8326E218: 4AFBECB9  bl 0x8222ced0
	ctx.lr = 0x8326E21C;
	sub_8222CED0(ctx, base);
	// 8326E21C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8326E220: 3869EED0  addi r3, r9, -0x1130
	ctx.r[3].s64 = ctx.r[9].s64 + -4400;
	// 8326E224: 4BA3BCFD  bl 0x82ca9f20
	ctx.lr = 0x8326E228;
	sub_82CA9F20(ctx, base);
	// 8326E228: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326E22C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326E230: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326E234: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326E238(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326E238 size=64
    let mut pc: u32 = 0x8326E238;
    'dispatch: loop {
        match pc {
            0x8326E238 => {
    //   block [0x8326E238..0x8326E278)
	// 8326E238: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326E23C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326E240: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326E244: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8326E248: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326E24C: 388B1ADC  addi r4, r11, 0x1adc
	ctx.r[4].s64 = ctx.r[11].s64 + 6876;
	// 8326E250: 386ACBA8  addi r3, r10, -0x3458
	ctx.r[3].s64 = ctx.r[10].s64 + -13400;
	// 8326E254: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8326E258: 4AFBEC79  bl 0x8222ced0
	ctx.lr = 0x8326E25C;
	sub_8222CED0(ctx, base);
	// 8326E25C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8326E260: 3869EEE0  addi r3, r9, -0x1120
	ctx.r[3].s64 = ctx.r[9].s64 + -4384;
	// 8326E264: 4BA3BCBD  bl 0x82ca9f20
	ctx.lr = 0x8326E268;
	sub_82CA9F20(ctx, base);
	// 8326E268: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326E26C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326E270: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326E274: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326E278(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326E278 size=64
    let mut pc: u32 = 0x8326E278;
    'dispatch: loop {
        match pc {
            0x8326E278 => {
    //   block [0x8326E278..0x8326E2B8)
	// 8326E278: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326E27C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326E280: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326E284: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8326E288: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326E28C: 388B1AF0  addi r4, r11, 0x1af0
	ctx.r[4].s64 = ctx.r[11].s64 + 6896;
	// 8326E290: 386ACBAC  addi r3, r10, -0x3454
	ctx.r[3].s64 = ctx.r[10].s64 + -13396;
	// 8326E294: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8326E298: 4AFBEC39  bl 0x8222ced0
	ctx.lr = 0x8326E29C;
	sub_8222CED0(ctx, base);
	// 8326E29C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8326E2A0: 3869EEF0  addi r3, r9, -0x1110
	ctx.r[3].s64 = ctx.r[9].s64 + -4368;
	// 8326E2A4: 4BA3BC7D  bl 0x82ca9f20
	ctx.lr = 0x8326E2A8;
	sub_82CA9F20(ctx, base);
	// 8326E2A8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326E2AC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326E2B0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326E2B4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326E2B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326E2B8 size=56
    let mut pc: u32 = 0x8326E2B8;
    'dispatch: loop {
        match pc {
            0x8326E2B8 => {
    //   block [0x8326E2B8..0x8326E2F0)
	// 8326E2B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326E2BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326E2C0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326E2C4: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8326E2C8: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8326E2CC: 386B123C  addi r3, r11, 0x123c
	ctx.r[3].s64 = ctx.r[11].s64 + 4668;
	// 8326E2D0: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8326E2D4: 4AF85A85  bl 0x821f3d58
	ctx.lr = 0x8326E2D8;
	sub_821F3D58(ctx, base);
	// 8326E2D8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326E2DC: 906ACBB0  stw r3, -0x3450(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-13392 as u32), ctx.r[3].u32 ) };
	// 8326E2E0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326E2E4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326E2E8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326E2EC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326E2F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326E2F0 size=56
    let mut pc: u32 = 0x8326E2F0;
    'dispatch: loop {
        match pc {
            0x8326E2F0 => {
    //   block [0x8326E2F0..0x8326E328)
	// 8326E2F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326E2F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326E2F8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326E2FC: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 8326E300: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8326E304: 386B92EC  addi r3, r11, -0x6d14
	ctx.r[3].s64 = ctx.r[11].s64 + -27924;
	// 8326E308: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8326E30C: 4AF85A4D  bl 0x821f3d58
	ctx.lr = 0x8326E310;
	sub_821F3D58(ctx, base);
	// 8326E310: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326E314: 906ACBB4  stw r3, -0x344c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-13388 as u32), ctx.r[3].u32 ) };
	// 8326E318: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326E31C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326E320: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326E324: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326E328(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326E328 size=56
    let mut pc: u32 = 0x8326E328;
    'dispatch: loop {
        match pc {
            0x8326E328 => {
    //   block [0x8326E328..0x8326E360)
	// 8326E328: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326E32C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326E330: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326E334: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 8326E338: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8326E33C: 386B92F8  addi r3, r11, -0x6d08
	ctx.r[3].s64 = ctx.r[11].s64 + -27912;
	// 8326E340: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8326E344: 4AF85A15  bl 0x821f3d58
	ctx.lr = 0x8326E348;
	sub_821F3D58(ctx, base);
	// 8326E348: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326E34C: 906ACBB8  stw r3, -0x3448(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-13384 as u32), ctx.r[3].u32 ) };
	// 8326E350: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326E354: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326E358: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326E35C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326E360(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326E360 size=56
    let mut pc: u32 = 0x8326E360;
    'dispatch: loop {
        match pc {
            0x8326E360 => {
    //   block [0x8326E360..0x8326E398)
	// 8326E360: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326E364: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326E368: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326E36C: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 8326E370: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8326E374: 386B9428  addi r3, r11, -0x6bd8
	ctx.r[3].s64 = ctx.r[11].s64 + -27608;
	// 8326E378: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8326E37C: 4AF859DD  bl 0x821f3d58
	ctx.lr = 0x8326E380;
	sub_821F3D58(ctx, base);
	// 8326E380: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326E384: 906ACBBC  stw r3, -0x3444(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-13380 as u32), ctx.r[3].u32 ) };
	// 8326E388: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326E38C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326E390: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326E394: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326E398(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326E398 size=56
    let mut pc: u32 = 0x8326E398;
    'dispatch: loop {
        match pc {
            0x8326E398 => {
    //   block [0x8326E398..0x8326E3D0)
	// 8326E398: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326E39C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326E3A0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326E3A4: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 8326E3A8: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8326E3AC: 386B9430  addi r3, r11, -0x6bd0
	ctx.r[3].s64 = ctx.r[11].s64 + -27600;
	// 8326E3B0: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8326E3B4: 4AF859A5  bl 0x821f3d58
	ctx.lr = 0x8326E3B8;
	sub_821F3D58(ctx, base);
	// 8326E3B8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326E3BC: 906ACBC0  stw r3, -0x3440(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-13376 as u32), ctx.r[3].u32 ) };
	// 8326E3C0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326E3C4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326E3C8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326E3CC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326E3D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326E3D0 size=56
    let mut pc: u32 = 0x8326E3D0;
    'dispatch: loop {
        match pc {
            0x8326E3D0 => {
    //   block [0x8326E3D0..0x8326E408)
	// 8326E3D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326E3D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326E3D8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326E3DC: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 8326E3E0: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8326E3E4: 386B943C  addi r3, r11, -0x6bc4
	ctx.r[3].s64 = ctx.r[11].s64 + -27588;
	// 8326E3E8: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8326E3EC: 4AF8596D  bl 0x821f3d58
	ctx.lr = 0x8326E3F0;
	sub_821F3D58(ctx, base);
	// 8326E3F0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326E3F4: 906ACBC4  stw r3, -0x343c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-13372 as u32), ctx.r[3].u32 ) };
	// 8326E3F8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326E3FC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326E400: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326E404: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326E408(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326E408 size=56
    let mut pc: u32 = 0x8326E408;
    'dispatch: loop {
        match pc {
            0x8326E408 => {
    //   block [0x8326E408..0x8326E440)
	// 8326E408: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326E40C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326E410: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326E414: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8326E418: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8326E41C: 386B123C  addi r3, r11, 0x123c
	ctx.r[3].s64 = ctx.r[11].s64 + 4668;
	// 8326E420: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8326E424: 4AF85935  bl 0x821f3d58
	ctx.lr = 0x8326E428;
	sub_821F3D58(ctx, base);
	// 8326E428: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326E42C: 906ACBC8  stw r3, -0x3438(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-13368 as u32), ctx.r[3].u32 ) };
	// 8326E430: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326E434: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326E438: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326E43C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326E440(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326E440 size=56
    let mut pc: u32 = 0x8326E440;
    'dispatch: loop {
        match pc {
            0x8326E440 => {
    //   block [0x8326E440..0x8326E478)
	// 8326E440: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326E444: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326E448: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326E44C: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 8326E450: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8326E454: 386B944C  addi r3, r11, -0x6bb4
	ctx.r[3].s64 = ctx.r[11].s64 + -27572;
	// 8326E458: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8326E45C: 4AF858FD  bl 0x821f3d58
	ctx.lr = 0x8326E460;
	sub_821F3D58(ctx, base);
	// 8326E460: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326E464: 906ACBCC  stw r3, -0x3434(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-13364 as u32), ctx.r[3].u32 ) };
	// 8326E468: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326E46C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326E470: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326E474: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326E478(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326E478 size=56
    let mut pc: u32 = 0x8326E478;
    'dispatch: loop {
        match pc {
            0x8326E478 => {
    //   block [0x8326E478..0x8326E4B0)
	// 8326E478: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326E47C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326E480: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326E484: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 8326E488: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8326E48C: 386B9458  addi r3, r11, -0x6ba8
	ctx.r[3].s64 = ctx.r[11].s64 + -27560;
	// 8326E490: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8326E494: 4AF858C5  bl 0x821f3d58
	ctx.lr = 0x8326E498;
	sub_821F3D58(ctx, base);
	// 8326E498: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326E49C: 906ACBD0  stw r3, -0x3430(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-13360 as u32), ctx.r[3].u32 ) };
	// 8326E4A0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326E4A4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326E4A8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326E4AC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326E4B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326E4B0 size=56
    let mut pc: u32 = 0x8326E4B0;
    'dispatch: loop {
        match pc {
            0x8326E4B0 => {
    //   block [0x8326E4B0..0x8326E4E8)
	// 8326E4B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326E4B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326E4B8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326E4BC: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 8326E4C0: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8326E4C4: 386B9464  addi r3, r11, -0x6b9c
	ctx.r[3].s64 = ctx.r[11].s64 + -27548;
	// 8326E4C8: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8326E4CC: 4AF8588D  bl 0x821f3d58
	ctx.lr = 0x8326E4D0;
	sub_821F3D58(ctx, base);
	// 8326E4D0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326E4D4: 906ACBD4  stw r3, -0x342c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-13356 as u32), ctx.r[3].u32 ) };
	// 8326E4D8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326E4DC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326E4E0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326E4E4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326E4E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326E4E8 size=56
    let mut pc: u32 = 0x8326E4E8;
    'dispatch: loop {
        match pc {
            0x8326E4E8 => {
    //   block [0x8326E4E8..0x8326E520)
	// 8326E4E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326E4EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326E4F0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326E4F4: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 8326E4F8: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8326E4FC: 386B92EC  addi r3, r11, -0x6d14
	ctx.r[3].s64 = ctx.r[11].s64 + -27924;
	// 8326E500: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8326E504: 4AF85855  bl 0x821f3d58
	ctx.lr = 0x8326E508;
	sub_821F3D58(ctx, base);
	// 8326E508: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326E50C: 906ACBD8  stw r3, -0x3428(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-13352 as u32), ctx.r[3].u32 ) };
	// 8326E510: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326E514: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326E518: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326E51C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326E520(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326E520 size=56
    let mut pc: u32 = 0x8326E520;
    'dispatch: loop {
        match pc {
            0x8326E520 => {
    //   block [0x8326E520..0x8326E558)
	// 8326E520: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326E524: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326E528: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326E52C: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 8326E530: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8326E534: 386B92F8  addi r3, r11, -0x6d08
	ctx.r[3].s64 = ctx.r[11].s64 + -27912;
	// 8326E538: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8326E53C: 4AF8581D  bl 0x821f3d58
	ctx.lr = 0x8326E540;
	sub_821F3D58(ctx, base);
	// 8326E540: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326E544: 906ACBDC  stw r3, -0x3424(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-13348 as u32), ctx.r[3].u32 ) };
	// 8326E548: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326E54C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326E550: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326E554: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326E558(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326E558 size=56
    let mut pc: u32 = 0x8326E558;
    'dispatch: loop {
        match pc {
            0x8326E558 => {
    //   block [0x8326E558..0x8326E590)
	// 8326E558: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326E55C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326E560: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326E564: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 8326E568: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8326E56C: 386B9474  addi r3, r11, -0x6b8c
	ctx.r[3].s64 = ctx.r[11].s64 + -27532;
	// 8326E570: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8326E574: 4AF857E5  bl 0x821f3d58
	ctx.lr = 0x8326E578;
	sub_821F3D58(ctx, base);
	// 8326E578: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326E57C: 906ACBE0  stw r3, -0x3420(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-13344 as u32), ctx.r[3].u32 ) };
	// 8326E580: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326E584: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326E588: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326E58C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326E590(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326E590 size=56
    let mut pc: u32 = 0x8326E590;
    'dispatch: loop {
        match pc {
            0x8326E590 => {
    //   block [0x8326E590..0x8326E5C8)
	// 8326E590: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326E594: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326E598: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326E59C: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 8326E5A0: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8326E5A4: 386B9480  addi r3, r11, -0x6b80
	ctx.r[3].s64 = ctx.r[11].s64 + -27520;
	// 8326E5A8: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8326E5AC: 4AF857AD  bl 0x821f3d58
	ctx.lr = 0x8326E5B0;
	sub_821F3D58(ctx, base);
	// 8326E5B0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326E5B4: 906ACBE4  stw r3, -0x341c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-13340 as u32), ctx.r[3].u32 ) };
	// 8326E5B8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326E5BC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326E5C0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326E5C4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326E5C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326E5C8 size=56
    let mut pc: u32 = 0x8326E5C8;
    'dispatch: loop {
        match pc {
            0x8326E5C8 => {
    //   block [0x8326E5C8..0x8326E600)
	// 8326E5C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326E5CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326E5D0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326E5D4: 3D60820C  lis r11, -0x7df4
	ctx.r[11].s64 = -2113142784;
	// 8326E5D8: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8326E5DC: 386BA994  addi r3, r11, -0x566c
	ctx.r[3].s64 = ctx.r[11].s64 + -22124;
	// 8326E5E0: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8326E5E4: 4AF85775  bl 0x821f3d58
	ctx.lr = 0x8326E5E8;
	sub_821F3D58(ctx, base);
	// 8326E5E8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326E5EC: 906ACBE8  stw r3, -0x3418(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-13336 as u32), ctx.r[3].u32 ) };
	// 8326E5F0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326E5F4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326E5F8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326E5FC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326E600(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326E600 size=56
    let mut pc: u32 = 0x8326E600;
    'dispatch: loop {
        match pc {
            0x8326E600 => {
    //   block [0x8326E600..0x8326E638)
	// 8326E600: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326E604: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326E608: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326E60C: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 8326E610: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8326E614: 386B9490  addi r3, r11, -0x6b70
	ctx.r[3].s64 = ctx.r[11].s64 + -27504;
	// 8326E618: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8326E61C: 4AF8573D  bl 0x821f3d58
	ctx.lr = 0x8326E620;
	sub_821F3D58(ctx, base);
	// 8326E620: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326E624: 906ACBEC  stw r3, -0x3414(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-13332 as u32), ctx.r[3].u32 ) };
	// 8326E628: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326E62C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326E630: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326E634: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326E638(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326E638 size=56
    let mut pc: u32 = 0x8326E638;
    'dispatch: loop {
        match pc {
            0x8326E638 => {
    //   block [0x8326E638..0x8326E670)
	// 8326E638: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326E63C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326E640: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326E644: 3D60820C  lis r11, -0x7df4
	ctx.r[11].s64 = -2113142784;
	// 8326E648: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8326E64C: 386BA9A0  addi r3, r11, -0x5660
	ctx.r[3].s64 = ctx.r[11].s64 + -22112;
	// 8326E650: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8326E654: 4AF85705  bl 0x821f3d58
	ctx.lr = 0x8326E658;
	sub_821F3D58(ctx, base);
	// 8326E658: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326E65C: 906ACBF0  stw r3, -0x3410(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-13328 as u32), ctx.r[3].u32 ) };
	// 8326E660: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326E664: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326E668: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326E66C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326E670(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326E670 size=56
    let mut pc: u32 = 0x8326E670;
    'dispatch: loop {
        match pc {
            0x8326E670 => {
    //   block [0x8326E670..0x8326E6A8)
	// 8326E670: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326E674: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326E678: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326E67C: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 8326E680: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8326E684: 386B94A0  addi r3, r11, -0x6b60
	ctx.r[3].s64 = ctx.r[11].s64 + -27488;
	// 8326E688: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8326E68C: 4AF856CD  bl 0x821f3d58
	ctx.lr = 0x8326E690;
	sub_821F3D58(ctx, base);
	// 8326E690: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326E694: 906ACBF4  stw r3, -0x340c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-13324 as u32), ctx.r[3].u32 ) };
	// 8326E698: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326E69C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326E6A0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326E6A4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326E6A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326E6A8 size=56
    let mut pc: u32 = 0x8326E6A8;
    'dispatch: loop {
        match pc {
            0x8326E6A8 => {
    //   block [0x8326E6A8..0x8326E6E0)
	// 8326E6A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326E6AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326E6B0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326E6B4: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 8326E6B8: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8326E6BC: 386B94B8  addi r3, r11, -0x6b48
	ctx.r[3].s64 = ctx.r[11].s64 + -27464;
	// 8326E6C0: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8326E6C4: 4AF85695  bl 0x821f3d58
	ctx.lr = 0x8326E6C8;
	sub_821F3D58(ctx, base);
	// 8326E6C8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326E6CC: 906ACBF8  stw r3, -0x3408(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-13320 as u32), ctx.r[3].u32 ) };
	// 8326E6D0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326E6D4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326E6D8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326E6DC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326E6E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326E6E0 size=56
    let mut pc: u32 = 0x8326E6E0;
    'dispatch: loop {
        match pc {
            0x8326E6E0 => {
    //   block [0x8326E6E0..0x8326E718)
	// 8326E6E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326E6E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326E6E8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326E6EC: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 8326E6F0: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8326E6F4: 386B94C8  addi r3, r11, -0x6b38
	ctx.r[3].s64 = ctx.r[11].s64 + -27448;
	// 8326E6F8: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8326E6FC: 4AF8565D  bl 0x821f3d58
	ctx.lr = 0x8326E700;
	sub_821F3D58(ctx, base);
	// 8326E700: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326E704: 906ACBFC  stw r3, -0x3404(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-13316 as u32), ctx.r[3].u32 ) };
	// 8326E708: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326E70C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326E710: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326E714: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326E718(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326E718 size=56
    let mut pc: u32 = 0x8326E718;
    'dispatch: loop {
        match pc {
            0x8326E718 => {
    //   block [0x8326E718..0x8326E750)
	// 8326E718: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326E71C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326E720: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326E724: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 8326E728: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8326E72C: 386B94DC  addi r3, r11, -0x6b24
	ctx.r[3].s64 = ctx.r[11].s64 + -27428;
	// 8326E730: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8326E734: 4AF85625  bl 0x821f3d58
	ctx.lr = 0x8326E738;
	sub_821F3D58(ctx, base);
	// 8326E738: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326E73C: 906ACC00  stw r3, -0x3400(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-13312 as u32), ctx.r[3].u32 ) };
	// 8326E740: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326E744: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326E748: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326E74C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326E750(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326E750 size=56
    let mut pc: u32 = 0x8326E750;
    'dispatch: loop {
        match pc {
            0x8326E750 => {
    //   block [0x8326E750..0x8326E788)
	// 8326E750: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326E754: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326E758: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326E75C: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 8326E760: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8326E764: 386B1034  addi r3, r11, 0x1034
	ctx.r[3].s64 = ctx.r[11].s64 + 4148;
	// 8326E768: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8326E76C: 4AF855ED  bl 0x821f3d58
	ctx.lr = 0x8326E770;
	sub_821F3D58(ctx, base);
	// 8326E770: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326E774: 906ACC04  stw r3, -0x33fc(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-13308 as u32), ctx.r[3].u32 ) };
	// 8326E778: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326E77C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326E780: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326E784: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326E788(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326E788 size=56
    let mut pc: u32 = 0x8326E788;
    'dispatch: loop {
        match pc {
            0x8326E788 => {
    //   block [0x8326E788..0x8326E7C0)
	// 8326E788: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326E78C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326E790: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326E794: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 8326E798: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8326E79C: 386B94E4  addi r3, r11, -0x6b1c
	ctx.r[3].s64 = ctx.r[11].s64 + -27420;
	// 8326E7A0: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8326E7A4: 4AF855B5  bl 0x821f3d58
	ctx.lr = 0x8326E7A8;
	sub_821F3D58(ctx, base);
	// 8326E7A8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326E7AC: 906ACC08  stw r3, -0x33f8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-13304 as u32), ctx.r[3].u32 ) };
	// 8326E7B0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326E7B4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326E7B8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326E7BC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326E7C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326E7C0 size=56
    let mut pc: u32 = 0x8326E7C0;
    'dispatch: loop {
        match pc {
            0x8326E7C0 => {
    //   block [0x8326E7C0..0x8326E7F8)
	// 8326E7C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326E7C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326E7C8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326E7CC: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 8326E7D0: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8326E7D4: 386B94F4  addi r3, r11, -0x6b0c
	ctx.r[3].s64 = ctx.r[11].s64 + -27404;
	// 8326E7D8: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8326E7DC: 4AF8557D  bl 0x821f3d58
	ctx.lr = 0x8326E7E0;
	sub_821F3D58(ctx, base);
	// 8326E7E0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326E7E4: 906ACC0C  stw r3, -0x33f4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-13300 as u32), ctx.r[3].u32 ) };
	// 8326E7E8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326E7EC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326E7F0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326E7F4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326E7F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326E7F8 size=56
    let mut pc: u32 = 0x8326E7F8;
    'dispatch: loop {
        match pc {
            0x8326E7F8 => {
    //   block [0x8326E7F8..0x8326E830)
	// 8326E7F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326E7FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326E800: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326E804: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 8326E808: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8326E80C: 386B9508  addi r3, r11, -0x6af8
	ctx.r[3].s64 = ctx.r[11].s64 + -27384;
	// 8326E810: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8326E814: 4AF85545  bl 0x821f3d58
	ctx.lr = 0x8326E818;
	sub_821F3D58(ctx, base);
	// 8326E818: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326E81C: 906ACC10  stw r3, -0x33f0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-13296 as u32), ctx.r[3].u32 ) };
	// 8326E820: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326E824: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326E828: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326E82C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326E830(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326E830 size=56
    let mut pc: u32 = 0x8326E830;
    'dispatch: loop {
        match pc {
            0x8326E830 => {
    //   block [0x8326E830..0x8326E868)
	// 8326E830: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326E834: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326E838: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326E83C: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 8326E840: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8326E844: 386B9514  addi r3, r11, -0x6aec
	ctx.r[3].s64 = ctx.r[11].s64 + -27372;
	// 8326E848: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8326E84C: 4AF8550D  bl 0x821f3d58
	ctx.lr = 0x8326E850;
	sub_821F3D58(ctx, base);
	// 8326E850: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326E854: 906ACC14  stw r3, -0x33ec(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-13292 as u32), ctx.r[3].u32 ) };
	// 8326E858: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326E85C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326E860: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326E864: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326E868(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326E868 size=56
    let mut pc: u32 = 0x8326E868;
    'dispatch: loop {
        match pc {
            0x8326E868 => {
    //   block [0x8326E868..0x8326E8A0)
	// 8326E868: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326E86C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326E870: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326E874: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 8326E878: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8326E87C: 386B9520  addi r3, r11, -0x6ae0
	ctx.r[3].s64 = ctx.r[11].s64 + -27360;
	// 8326E880: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8326E884: 4AF854D5  bl 0x821f3d58
	ctx.lr = 0x8326E888;
	sub_821F3D58(ctx, base);
	// 8326E888: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326E88C: 906ACC18  stw r3, -0x33e8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-13288 as u32), ctx.r[3].u32 ) };
	// 8326E890: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326E894: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326E898: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326E89C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326E8A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326E8A0 size=56
    let mut pc: u32 = 0x8326E8A0;
    'dispatch: loop {
        match pc {
            0x8326E8A0 => {
    //   block [0x8326E8A0..0x8326E8D8)
	// 8326E8A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326E8A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326E8A8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326E8AC: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 8326E8B0: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8326E8B4: 386B9528  addi r3, r11, -0x6ad8
	ctx.r[3].s64 = ctx.r[11].s64 + -27352;
	// 8326E8B8: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8326E8BC: 4AF8549D  bl 0x821f3d58
	ctx.lr = 0x8326E8C0;
	sub_821F3D58(ctx, base);
	// 8326E8C0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326E8C4: 906ACC1C  stw r3, -0x33e4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-13284 as u32), ctx.r[3].u32 ) };
	// 8326E8C8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326E8CC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326E8D0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326E8D4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326E8D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326E8D8 size=56
    let mut pc: u32 = 0x8326E8D8;
    'dispatch: loop {
        match pc {
            0x8326E8D8 => {
    //   block [0x8326E8D8..0x8326E910)
	// 8326E8D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326E8DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326E8E0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326E8E4: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8326E8E8: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8326E8EC: 386B2C84  addi r3, r11, 0x2c84
	ctx.r[3].s64 = ctx.r[11].s64 + 11396;
	// 8326E8F0: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8326E8F4: 4AF85465  bl 0x821f3d58
	ctx.lr = 0x8326E8F8;
	sub_821F3D58(ctx, base);
	// 8326E8F8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326E8FC: 906ACC20  stw r3, -0x33e0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-13280 as u32), ctx.r[3].u32 ) };
	// 8326E900: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326E904: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326E908: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326E90C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326E910(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326E910 size=56
    let mut pc: u32 = 0x8326E910;
    'dispatch: loop {
        match pc {
            0x8326E910 => {
    //   block [0x8326E910..0x8326E948)
	// 8326E910: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326E914: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326E918: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326E91C: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 8326E920: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8326E924: 386B9608  addi r3, r11, -0x69f8
	ctx.r[3].s64 = ctx.r[11].s64 + -27128;
	// 8326E928: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8326E92C: 4AF8542D  bl 0x821f3d58
	ctx.lr = 0x8326E930;
	sub_821F3D58(ctx, base);
	// 8326E930: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326E934: 906ACC24  stw r3, -0x33dc(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-13276 as u32), ctx.r[3].u32 ) };
	// 8326E938: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326E93C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326E940: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326E944: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326E948(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326E948 size=56
    let mut pc: u32 = 0x8326E948;
    'dispatch: loop {
        match pc {
            0x8326E948 => {
    //   block [0x8326E948..0x8326E980)
	// 8326E948: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326E94C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326E950: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326E954: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 8326E958: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8326E95C: 386B94DC  addi r3, r11, -0x6b24
	ctx.r[3].s64 = ctx.r[11].s64 + -27428;
	// 8326E960: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8326E964: 4AF853F5  bl 0x821f3d58
	ctx.lr = 0x8326E968;
	sub_821F3D58(ctx, base);
	// 8326E968: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326E96C: 906ACC28  stw r3, -0x33d8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-13272 as u32), ctx.r[3].u32 ) };
	// 8326E970: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326E974: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326E978: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326E97C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326E980(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326E980 size=56
    let mut pc: u32 = 0x8326E980;
    'dispatch: loop {
        match pc {
            0x8326E980 => {
    //   block [0x8326E980..0x8326E9B8)
	// 8326E980: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326E984: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326E988: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326E98C: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 8326E990: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8326E994: 386B1034  addi r3, r11, 0x1034
	ctx.r[3].s64 = ctx.r[11].s64 + 4148;
	// 8326E998: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8326E99C: 4AF853BD  bl 0x821f3d58
	ctx.lr = 0x8326E9A0;
	sub_821F3D58(ctx, base);
	// 8326E9A0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326E9A4: 906ACC2C  stw r3, -0x33d4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-13268 as u32), ctx.r[3].u32 ) };
	// 8326E9A8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326E9AC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326E9B0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326E9B4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326E9B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326E9B8 size=56
    let mut pc: u32 = 0x8326E9B8;
    'dispatch: loop {
        match pc {
            0x8326E9B8 => {
    //   block [0x8326E9B8..0x8326E9F0)
	// 8326E9B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326E9BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326E9C0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326E9C4: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 8326E9C8: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8326E9CC: 386B94E4  addi r3, r11, -0x6b1c
	ctx.r[3].s64 = ctx.r[11].s64 + -27420;
	// 8326E9D0: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8326E9D4: 4AF85385  bl 0x821f3d58
	ctx.lr = 0x8326E9D8;
	sub_821F3D58(ctx, base);
	// 8326E9D8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326E9DC: 906ACC30  stw r3, -0x33d0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-13264 as u32), ctx.r[3].u32 ) };
	// 8326E9E0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326E9E4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326E9E8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326E9EC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326E9F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326E9F0 size=56
    let mut pc: u32 = 0x8326E9F0;
    'dispatch: loop {
        match pc {
            0x8326E9F0 => {
    //   block [0x8326E9F0..0x8326EA28)
	// 8326E9F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326E9F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326E9F8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326E9FC: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 8326EA00: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8326EA04: 386B9620  addi r3, r11, -0x69e0
	ctx.r[3].s64 = ctx.r[11].s64 + -27104;
	// 8326EA08: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8326EA0C: 4AF8534D  bl 0x821f3d58
	ctx.lr = 0x8326EA10;
	sub_821F3D58(ctx, base);
	// 8326EA10: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326EA14: 906ACC34  stw r3, -0x33cc(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-13260 as u32), ctx.r[3].u32 ) };
	// 8326EA18: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326EA1C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326EA20: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326EA24: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326EA28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326EA28 size=56
    let mut pc: u32 = 0x8326EA28;
    'dispatch: loop {
        match pc {
            0x8326EA28 => {
    //   block [0x8326EA28..0x8326EA60)
	// 8326EA28: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326EA2C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326EA30: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326EA34: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 8326EA38: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8326EA3C: 386B962C  addi r3, r11, -0x69d4
	ctx.r[3].s64 = ctx.r[11].s64 + -27092;
	// 8326EA40: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8326EA44: 4AF85315  bl 0x821f3d58
	ctx.lr = 0x8326EA48;
	sub_821F3D58(ctx, base);
	// 8326EA48: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326EA4C: 906ACC38  stw r3, -0x33c8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-13256 as u32), ctx.r[3].u32 ) };
	// 8326EA50: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326EA54: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326EA58: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326EA5C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326EA60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326EA60 size=56
    let mut pc: u32 = 0x8326EA60;
    'dispatch: loop {
        match pc {
            0x8326EA60 => {
    //   block [0x8326EA60..0x8326EA98)
	// 8326EA60: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326EA64: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326EA68: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326EA6C: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8326EA70: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8326EA74: 386B2024  addi r3, r11, 0x2024
	ctx.r[3].s64 = ctx.r[11].s64 + 8228;
	// 8326EA78: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8326EA7C: 4AF852DD  bl 0x821f3d58
	ctx.lr = 0x8326EA80;
	sub_821F3D58(ctx, base);
	// 8326EA80: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326EA84: 906ACC3C  stw r3, -0x33c4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-13252 as u32), ctx.r[3].u32 ) };
	// 8326EA88: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326EA8C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326EA90: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326EA94: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326EA98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326EA98 size=56
    let mut pc: u32 = 0x8326EA98;
    'dispatch: loop {
        match pc {
            0x8326EA98 => {
    //   block [0x8326EA98..0x8326EAD0)
	// 8326EA98: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326EA9C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326EAA0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326EAA4: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 8326EAA8: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8326EAAC: 386B96A0  addi r3, r11, -0x6960
	ctx.r[3].s64 = ctx.r[11].s64 + -26976;
	// 8326EAB0: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8326EAB4: 4AF852A5  bl 0x821f3d58
	ctx.lr = 0x8326EAB8;
	sub_821F3D58(ctx, base);
	// 8326EAB8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326EABC: 906ACC40  stw r3, -0x33c0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-13248 as u32), ctx.r[3].u32 ) };
	// 8326EAC0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326EAC4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326EAC8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326EACC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326EAD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326EAD0 size=56
    let mut pc: u32 = 0x8326EAD0;
    'dispatch: loop {
        match pc {
            0x8326EAD0 => {
    //   block [0x8326EAD0..0x8326EB08)
	// 8326EAD0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326EAD4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326EAD8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326EADC: 3D60820C  lis r11, -0x7df4
	ctx.r[11].s64 = -2113142784;
	// 8326EAE0: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8326EAE4: 386B948C  addi r3, r11, -0x6b74
	ctx.r[3].s64 = ctx.r[11].s64 + -27508;
	// 8326EAE8: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8326EAEC: 4AF8526D  bl 0x821f3d58
	ctx.lr = 0x8326EAF0;
	sub_821F3D58(ctx, base);
	// 8326EAF0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326EAF4: 906ACC44  stw r3, -0x33bc(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-13244 as u32), ctx.r[3].u32 ) };
	// 8326EAF8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326EAFC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326EB00: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326EB04: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326EB08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326EB08 size=56
    let mut pc: u32 = 0x8326EB08;
    'dispatch: loop {
        match pc {
            0x8326EB08 => {
    //   block [0x8326EB08..0x8326EB40)
	// 8326EB08: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326EB0C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326EB10: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326EB14: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 8326EB18: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8326EB1C: 386B0F7C  addi r3, r11, 0xf7c
	ctx.r[3].s64 = ctx.r[11].s64 + 3964;
	// 8326EB20: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8326EB24: 4AF85235  bl 0x821f3d58
	ctx.lr = 0x8326EB28;
	sub_821F3D58(ctx, base);
	// 8326EB28: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326EB2C: 906ACC48  stw r3, -0x33b8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-13240 as u32), ctx.r[3].u32 ) };
	// 8326EB30: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326EB34: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326EB38: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326EB3C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326EB40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326EB40 size=56
    let mut pc: u32 = 0x8326EB40;
    'dispatch: loop {
        match pc {
            0x8326EB40 => {
    //   block [0x8326EB40..0x8326EB78)
	// 8326EB40: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326EB44: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326EB48: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326EB4C: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 8326EB50: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8326EB54: 386B9480  addi r3, r11, -0x6b80
	ctx.r[3].s64 = ctx.r[11].s64 + -27520;
	// 8326EB58: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8326EB5C: 4AF851FD  bl 0x821f3d58
	ctx.lr = 0x8326EB60;
	sub_821F3D58(ctx, base);
	// 8326EB60: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326EB64: 906ACC4C  stw r3, -0x33b4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-13236 as u32), ctx.r[3].u32 ) };
	// 8326EB68: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326EB6C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326EB70: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326EB74: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326EB78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326EB78 size=56
    let mut pc: u32 = 0x8326EB78;
    'dispatch: loop {
        match pc {
            0x8326EB78 => {
    //   block [0x8326EB78..0x8326EBB0)
	// 8326EB78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326EB7C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326EB80: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326EB84: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 8326EB88: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8326EB8C: 386B96AC  addi r3, r11, -0x6954
	ctx.r[3].s64 = ctx.r[11].s64 + -26964;
	// 8326EB90: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8326EB94: 4AF851C5  bl 0x821f3d58
	ctx.lr = 0x8326EB98;
	sub_821F3D58(ctx, base);
	// 8326EB98: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326EB9C: 906ACC50  stw r3, -0x33b0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-13232 as u32), ctx.r[3].u32 ) };
	// 8326EBA0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326EBA4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326EBA8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326EBAC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326EBB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326EBB0 size=56
    let mut pc: u32 = 0x8326EBB0;
    'dispatch: loop {
        match pc {
            0x8326EBB0 => {
    //   block [0x8326EBB0..0x8326EBE8)
	// 8326EBB0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326EBB4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326EBB8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326EBBC: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 8326EBC0: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8326EBC4: 386B96B8  addi r3, r11, -0x6948
	ctx.r[3].s64 = ctx.r[11].s64 + -26952;
	// 8326EBC8: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8326EBCC: 4AF8518D  bl 0x821f3d58
	ctx.lr = 0x8326EBD0;
	sub_821F3D58(ctx, base);
	// 8326EBD0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326EBD4: 906ACC54  stw r3, -0x33ac(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-13228 as u32), ctx.r[3].u32 ) };
	// 8326EBD8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326EBDC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326EBE0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326EBE4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326EBE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326EBE8 size=56
    let mut pc: u32 = 0x8326EBE8;
    'dispatch: loop {
        match pc {
            0x8326EBE8 => {
    //   block [0x8326EBE8..0x8326EC20)
	// 8326EBE8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326EBEC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326EBF0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326EBF4: 3D60820C  lis r11, -0x7df4
	ctx.r[11].s64 = -2113142784;
	// 8326EBF8: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8326EBFC: 386BA994  addi r3, r11, -0x566c
	ctx.r[3].s64 = ctx.r[11].s64 + -22124;
	// 8326EC00: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8326EC04: 4AF85155  bl 0x821f3d58
	ctx.lr = 0x8326EC08;
	sub_821F3D58(ctx, base);
	// 8326EC08: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326EC0C: 906ACC58  stw r3, -0x33a8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-13224 as u32), ctx.r[3].u32 ) };
	// 8326EC10: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326EC14: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326EC18: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326EC1C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326EC20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326EC20 size=56
    let mut pc: u32 = 0x8326EC20;
    'dispatch: loop {
        match pc {
            0x8326EC20 => {
    //   block [0x8326EC20..0x8326EC58)
	// 8326EC20: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326EC24: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326EC28: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326EC2C: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8326EC30: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8326EC34: 386B2C84  addi r3, r11, 0x2c84
	ctx.r[3].s64 = ctx.r[11].s64 + 11396;
	// 8326EC38: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8326EC3C: 4AF8511D  bl 0x821f3d58
	ctx.lr = 0x8326EC40;
	sub_821F3D58(ctx, base);
	// 8326EC40: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326EC44: 906ACC5C  stw r3, -0x33a4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-13220 as u32), ctx.r[3].u32 ) };
	// 8326EC48: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326EC4C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326EC50: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326EC54: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326EC58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326EC58 size=56
    let mut pc: u32 = 0x8326EC58;
    'dispatch: loop {
        match pc {
            0x8326EC58 => {
    //   block [0x8326EC58..0x8326EC90)
	// 8326EC58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326EC5C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326EC60: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326EC64: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 8326EC68: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8326EC6C: 386B96C4  addi r3, r11, -0x693c
	ctx.r[3].s64 = ctx.r[11].s64 + -26940;
	// 8326EC70: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8326EC74: 4AF850E5  bl 0x821f3d58
	ctx.lr = 0x8326EC78;
	sub_821F3D58(ctx, base);
	// 8326EC78: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326EC7C: 906ACC60  stw r3, -0x33a0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-13216 as u32), ctx.r[3].u32 ) };
	// 8326EC80: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326EC84: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326EC88: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326EC8C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326EC90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326EC90 size=56
    let mut pc: u32 = 0x8326EC90;
    'dispatch: loop {
        match pc {
            0x8326EC90 => {
    //   block [0x8326EC90..0x8326ECC8)
	// 8326EC90: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326EC94: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326EC98: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326EC9C: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 8326ECA0: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8326ECA4: 386B96D4  addi r3, r11, -0x692c
	ctx.r[3].s64 = ctx.r[11].s64 + -26924;
	// 8326ECA8: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8326ECAC: 4AF850AD  bl 0x821f3d58
	ctx.lr = 0x8326ECB0;
	sub_821F3D58(ctx, base);
	// 8326ECB0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326ECB4: 906ACC64  stw r3, -0x339c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-13212 as u32), ctx.r[3].u32 ) };
	// 8326ECB8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326ECBC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326ECC0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326ECC4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326ECC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326ECC8 size=64
    let mut pc: u32 = 0x8326ECC8;
    'dispatch: loop {
        match pc {
            0x8326ECC8 => {
    //   block [0x8326ECC8..0x8326ED08)
	// 8326ECC8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326ECCC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326ECD0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326ECD4: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 8326ECD8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326ECDC: 388B9B48  addi r4, r11, -0x64b8
	ctx.r[4].s64 = ctx.r[11].s64 + -25784;
	// 8326ECE0: 386ACC68  addi r3, r10, -0x3398
	ctx.r[3].s64 = ctx.r[10].s64 + -13208;
	// 8326ECE4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8326ECE8: 4AFBE1E9  bl 0x8222ced0
	ctx.lr = 0x8326ECEC;
	sub_8222CED0(ctx, base);
	// 8326ECEC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8326ECF0: 3869EF00  addi r3, r9, -0x1100
	ctx.r[3].s64 = ctx.r[9].s64 + -4352;
	// 8326ECF4: 4BA3B22D  bl 0x82ca9f20
	ctx.lr = 0x8326ECF8;
	sub_82CA9F20(ctx, base);
	// 8326ECF8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326ECFC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326ED00: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326ED04: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326ED08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326ED08 size=64
    let mut pc: u32 = 0x8326ED08;
    'dispatch: loop {
        match pc {
            0x8326ED08 => {
    //   block [0x8326ED08..0x8326ED48)
	// 8326ED08: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326ED0C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326ED10: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326ED14: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 8326ED18: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326ED1C: 388B33AC  addi r4, r11, 0x33ac
	ctx.r[4].s64 = ctx.r[11].s64 + 13228;
	// 8326ED20: 386ACC6C  addi r3, r10, -0x3394
	ctx.r[3].s64 = ctx.r[10].s64 + -13204;
	// 8326ED24: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8326ED28: 4AFBE1A9  bl 0x8222ced0
	ctx.lr = 0x8326ED2C;
	sub_8222CED0(ctx, base);
	// 8326ED2C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8326ED30: 3869EF10  addi r3, r9, -0x10f0
	ctx.r[3].s64 = ctx.r[9].s64 + -4336;
	// 8326ED34: 4BA3B1ED  bl 0x82ca9f20
	ctx.lr = 0x8326ED38;
	sub_82CA9F20(ctx, base);
	// 8326ED38: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326ED3C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326ED40: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326ED44: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326ED48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326ED48 size=64
    let mut pc: u32 = 0x8326ED48;
    'dispatch: loop {
        match pc {
            0x8326ED48 => {
    //   block [0x8326ED48..0x8326ED88)
	// 8326ED48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326ED4C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326ED50: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326ED54: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8326ED58: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326ED5C: 388BDCFC  addi r4, r11, -0x2304
	ctx.r[4].s64 = ctx.r[11].s64 + -8964;
	// 8326ED60: 386ACC70  addi r3, r10, -0x3390
	ctx.r[3].s64 = ctx.r[10].s64 + -13200;
	// 8326ED64: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8326ED68: 4AFBE169  bl 0x8222ced0
	ctx.lr = 0x8326ED6C;
	sub_8222CED0(ctx, base);
	// 8326ED6C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8326ED70: 3869EF20  addi r3, r9, -0x10e0
	ctx.r[3].s64 = ctx.r[9].s64 + -4320;
	// 8326ED74: 4BA3B1AD  bl 0x82ca9f20
	ctx.lr = 0x8326ED78;
	sub_82CA9F20(ctx, base);
	// 8326ED78: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326ED7C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326ED80: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326ED84: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326ED88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326ED88 size=64
    let mut pc: u32 = 0x8326ED88;
    'dispatch: loop {
        match pc {
            0x8326ED88 => {
    //   block [0x8326ED88..0x8326EDC8)
	// 8326ED88: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326ED8C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326ED90: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326ED94: 3D60820C  lis r11, -0x7df4
	ctx.r[11].s64 = -2113142784;
	// 8326ED98: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326ED9C: 388B7700  addi r4, r11, 0x7700
	ctx.r[4].s64 = ctx.r[11].s64 + 30464;
	// 8326EDA0: 386ACC74  addi r3, r10, -0x338c
	ctx.r[3].s64 = ctx.r[10].s64 + -13196;
	// 8326EDA4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8326EDA8: 4AFBE129  bl 0x8222ced0
	ctx.lr = 0x8326EDAC;
	sub_8222CED0(ctx, base);
	// 8326EDAC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8326EDB0: 3869EF30  addi r3, r9, -0x10d0
	ctx.r[3].s64 = ctx.r[9].s64 + -4304;
	// 8326EDB4: 4BA3B16D  bl 0x82ca9f20
	ctx.lr = 0x8326EDB8;
	sub_82CA9F20(ctx, base);
	// 8326EDB8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326EDBC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326EDC0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326EDC4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326EDC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326EDC8 size=64
    let mut pc: u32 = 0x8326EDC8;
    'dispatch: loop {
        match pc {
            0x8326EDC8 => {
    //   block [0x8326EDC8..0x8326EE08)
	// 8326EDC8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326EDCC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326EDD0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326EDD4: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 8326EDD8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326EDDC: 388B2BAC  addi r4, r11, 0x2bac
	ctx.r[4].s64 = ctx.r[11].s64 + 11180;
	// 8326EDE0: 386ACC78  addi r3, r10, -0x3388
	ctx.r[3].s64 = ctx.r[10].s64 + -13192;
	// 8326EDE4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8326EDE8: 4AFBE0E9  bl 0x8222ced0
	ctx.lr = 0x8326EDEC;
	sub_8222CED0(ctx, base);
	// 8326EDEC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8326EDF0: 3869EF40  addi r3, r9, -0x10c0
	ctx.r[3].s64 = ctx.r[9].s64 + -4288;
	// 8326EDF4: 4BA3B12D  bl 0x82ca9f20
	ctx.lr = 0x8326EDF8;
	sub_82CA9F20(ctx, base);
	// 8326EDF8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326EDFC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326EE00: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326EE04: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326EE08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326EE08 size=64
    let mut pc: u32 = 0x8326EE08;
    'dispatch: loop {
        match pc {
            0x8326EE08 => {
    //   block [0x8326EE08..0x8326EE48)
	// 8326EE08: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326EE0C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326EE10: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326EE14: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8326EE18: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326EE1C: 388BDD00  addi r4, r11, -0x2300
	ctx.r[4].s64 = ctx.r[11].s64 + -8960;
	// 8326EE20: 386ACC7C  addi r3, r10, -0x3384
	ctx.r[3].s64 = ctx.r[10].s64 + -13188;
	// 8326EE24: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8326EE28: 4AFBE0A9  bl 0x8222ced0
	ctx.lr = 0x8326EE2C;
	sub_8222CED0(ctx, base);
	// 8326EE2C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8326EE30: 3869EF50  addi r3, r9, -0x10b0
	ctx.r[3].s64 = ctx.r[9].s64 + -4272;
	// 8326EE34: 4BA3B0ED  bl 0x82ca9f20
	ctx.lr = 0x8326EE38;
	sub_82CA9F20(ctx, base);
	// 8326EE38: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326EE3C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326EE40: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326EE44: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326EE48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326EE48 size=64
    let mut pc: u32 = 0x8326EE48;
    'dispatch: loop {
        match pc {
            0x8326EE48 => {
    //   block [0x8326EE48..0x8326EE88)
	// 8326EE48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326EE4C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326EE50: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326EE54: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8326EE58: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326EE5C: 388BDD0C  addi r4, r11, -0x22f4
	ctx.r[4].s64 = ctx.r[11].s64 + -8948;
	// 8326EE60: 386ACC80  addi r3, r10, -0x3380
	ctx.r[3].s64 = ctx.r[10].s64 + -13184;
	// 8326EE64: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8326EE68: 4AFBE069  bl 0x8222ced0
	ctx.lr = 0x8326EE6C;
	sub_8222CED0(ctx, base);
	// 8326EE6C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8326EE70: 3869EF60  addi r3, r9, -0x10a0
	ctx.r[3].s64 = ctx.r[9].s64 + -4256;
	// 8326EE74: 4BA3B0AD  bl 0x82ca9f20
	ctx.lr = 0x8326EE78;
	sub_82CA9F20(ctx, base);
	// 8326EE78: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326EE7C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326EE80: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326EE84: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326EE88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326EE88 size=64
    let mut pc: u32 = 0x8326EE88;
    'dispatch: loop {
        match pc {
            0x8326EE88 => {
    //   block [0x8326EE88..0x8326EEC8)
	// 8326EE88: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326EE8C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326EE90: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326EE94: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8326EE98: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326EE9C: 388BDD18  addi r4, r11, -0x22e8
	ctx.r[4].s64 = ctx.r[11].s64 + -8936;
	// 8326EEA0: 386ACC84  addi r3, r10, -0x337c
	ctx.r[3].s64 = ctx.r[10].s64 + -13180;
	// 8326EEA4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8326EEA8: 4AFBE029  bl 0x8222ced0
	ctx.lr = 0x8326EEAC;
	sub_8222CED0(ctx, base);
	// 8326EEAC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8326EEB0: 3869EF70  addi r3, r9, -0x1090
	ctx.r[3].s64 = ctx.r[9].s64 + -4240;
	// 8326EEB4: 4BA3B06D  bl 0x82ca9f20
	ctx.lr = 0x8326EEB8;
	sub_82CA9F20(ctx, base);
	// 8326EEB8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326EEBC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326EEC0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326EEC4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326EEC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326EEC8 size=64
    let mut pc: u32 = 0x8326EEC8;
    'dispatch: loop {
        match pc {
            0x8326EEC8 => {
    //   block [0x8326EEC8..0x8326EF08)
	// 8326EEC8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326EECC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326EED0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326EED4: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 8326EED8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326EEDC: 388B9B50  addi r4, r11, -0x64b0
	ctx.r[4].s64 = ctx.r[11].s64 + -25776;
	// 8326EEE0: 386ACC88  addi r3, r10, -0x3378
	ctx.r[3].s64 = ctx.r[10].s64 + -13176;
	// 8326EEE4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8326EEE8: 4AFBDFE9  bl 0x8222ced0
	ctx.lr = 0x8326EEEC;
	sub_8222CED0(ctx, base);
	// 8326EEEC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8326EEF0: 3869EF80  addi r3, r9, -0x1080
	ctx.r[3].s64 = ctx.r[9].s64 + -4224;
	// 8326EEF4: 4BA3B02D  bl 0x82ca9f20
	ctx.lr = 0x8326EEF8;
	sub_82CA9F20(ctx, base);
	// 8326EEF8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326EEFC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326EF00: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326EF04: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326EF08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326EF08 size=64
    let mut pc: u32 = 0x8326EF08;
    'dispatch: loop {
        match pc {
            0x8326EF08 => {
    //   block [0x8326EF08..0x8326EF48)
	// 8326EF08: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326EF0C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326EF10: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326EF14: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 8326EF18: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326EF1C: 388B9B74  addi r4, r11, -0x648c
	ctx.r[4].s64 = ctx.r[11].s64 + -25740;
	// 8326EF20: 386ACC8C  addi r3, r10, -0x3374
	ctx.r[3].s64 = ctx.r[10].s64 + -13172;
	// 8326EF24: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8326EF28: 4AFBDFA9  bl 0x8222ced0
	ctx.lr = 0x8326EF2C;
	sub_8222CED0(ctx, base);
	// 8326EF2C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8326EF30: 3869EF90  addi r3, r9, -0x1070
	ctx.r[3].s64 = ctx.r[9].s64 + -4208;
	// 8326EF34: 4BA3AFED  bl 0x82ca9f20
	ctx.lr = 0x8326EF38;
	sub_82CA9F20(ctx, base);
	// 8326EF38: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326EF3C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326EF40: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326EF44: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326EF48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326EF48 size=64
    let mut pc: u32 = 0x8326EF48;
    'dispatch: loop {
        match pc {
            0x8326EF48 => {
    //   block [0x8326EF48..0x8326EF88)
	// 8326EF48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326EF4C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326EF50: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326EF54: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8326EF58: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326EF5C: 388B1AD4  addi r4, r11, 0x1ad4
	ctx.r[4].s64 = ctx.r[11].s64 + 6868;
	// 8326EF60: 386ACC90  addi r3, r10, -0x3370
	ctx.r[3].s64 = ctx.r[10].s64 + -13168;
	// 8326EF64: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8326EF68: 4AFBDF69  bl 0x8222ced0
	ctx.lr = 0x8326EF6C;
	sub_8222CED0(ctx, base);
	// 8326EF6C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8326EF70: 3869EFA0  addi r3, r9, -0x1060
	ctx.r[3].s64 = ctx.r[9].s64 + -4192;
	// 8326EF74: 4BA3AFAD  bl 0x82ca9f20
	ctx.lr = 0x8326EF78;
	sub_82CA9F20(ctx, base);
	// 8326EF78: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326EF7C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326EF80: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326EF84: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326EF88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326EF88 size=64
    let mut pc: u32 = 0x8326EF88;
    'dispatch: loop {
        match pc {
            0x8326EF88 => {
    //   block [0x8326EF88..0x8326EFC8)
	// 8326EF88: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326EF8C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326EF90: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326EF94: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8326EF98: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326EF9C: 388B1ADC  addi r4, r11, 0x1adc
	ctx.r[4].s64 = ctx.r[11].s64 + 6876;
	// 8326EFA0: 386ACC94  addi r3, r10, -0x336c
	ctx.r[3].s64 = ctx.r[10].s64 + -13164;
	// 8326EFA4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8326EFA8: 4AFBDF29  bl 0x8222ced0
	ctx.lr = 0x8326EFAC;
	sub_8222CED0(ctx, base);
	// 8326EFAC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8326EFB0: 3869EFB0  addi r3, r9, -0x1050
	ctx.r[3].s64 = ctx.r[9].s64 + -4176;
	// 8326EFB4: 4BA3AF6D  bl 0x82ca9f20
	ctx.lr = 0x8326EFB8;
	sub_82CA9F20(ctx, base);
	// 8326EFB8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326EFBC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326EFC0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326EFC4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326EFC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326EFC8 size=64
    let mut pc: u32 = 0x8326EFC8;
    'dispatch: loop {
        match pc {
            0x8326EFC8 => {
    //   block [0x8326EFC8..0x8326F008)
	// 8326EFC8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326EFCC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326EFD0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326EFD4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8326EFD8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326EFDC: 388B1AF0  addi r4, r11, 0x1af0
	ctx.r[4].s64 = ctx.r[11].s64 + 6896;
	// 8326EFE0: 386ACC98  addi r3, r10, -0x3368
	ctx.r[3].s64 = ctx.r[10].s64 + -13160;
	// 8326EFE4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8326EFE8: 4AFBDEE9  bl 0x8222ced0
	ctx.lr = 0x8326EFEC;
	sub_8222CED0(ctx, base);
	// 8326EFEC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8326EFF0: 3869EFC0  addi r3, r9, -0x1040
	ctx.r[3].s64 = ctx.r[9].s64 + -4160;
	// 8326EFF4: 4BA3AF2D  bl 0x82ca9f20
	ctx.lr = 0x8326EFF8;
	sub_82CA9F20(ctx, base);
	// 8326EFF8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326EFFC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326F000: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326F004: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326F008(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326F008 size=56
    let mut pc: u32 = 0x8326F008;
    'dispatch: loop {
        match pc {
            0x8326F008 => {
    //   block [0x8326F008..0x8326F040)
	// 8326F008: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326F00C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326F010: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326F014: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8326F018: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8326F01C: 386B42C8  addi r3, r11, 0x42c8
	ctx.r[3].s64 = ctx.r[11].s64 + 17096;
	// 8326F020: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8326F024: 4AF84D35  bl 0x821f3d58
	ctx.lr = 0x8326F028;
	sub_821F3D58(ctx, base);
	// 8326F028: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326F02C: 906ACC9C  stw r3, -0x3364(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-13156 as u32), ctx.r[3].u32 ) };
	// 8326F030: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326F034: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326F038: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326F03C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326F040(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326F040 size=56
    let mut pc: u32 = 0x8326F040;
    'dispatch: loop {
        match pc {
            0x8326F040 => {
    //   block [0x8326F040..0x8326F078)
	// 8326F040: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326F044: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326F048: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326F04C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8326F050: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8326F054: 386B42DC  addi r3, r11, 0x42dc
	ctx.r[3].s64 = ctx.r[11].s64 + 17116;
	// 8326F058: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8326F05C: 4AF84CFD  bl 0x821f3d58
	ctx.lr = 0x8326F060;
	sub_821F3D58(ctx, base);
	// 8326F060: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326F064: 906ACCA0  stw r3, -0x3360(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-13152 as u32), ctx.r[3].u32 ) };
	// 8326F068: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326F06C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326F070: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326F074: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326F078(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326F078 size=56
    let mut pc: u32 = 0x8326F078;
    'dispatch: loop {
        match pc {
            0x8326F078 => {
    //   block [0x8326F078..0x8326F0B0)
	// 8326F078: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326F07C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326F080: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326F084: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8326F088: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8326F08C: 386B42E8  addi r3, r11, 0x42e8
	ctx.r[3].s64 = ctx.r[11].s64 + 17128;
	// 8326F090: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8326F094: 4AF84CC5  bl 0x821f3d58
	ctx.lr = 0x8326F098;
	sub_821F3D58(ctx, base);
	// 8326F098: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326F09C: 906ACCA4  stw r3, -0x335c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-13148 as u32), ctx.r[3].u32 ) };
	// 8326F0A0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326F0A4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326F0A8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326F0AC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326F0B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326F0B0 size=56
    let mut pc: u32 = 0x8326F0B0;
    'dispatch: loop {
        match pc {
            0x8326F0B0 => {
    //   block [0x8326F0B0..0x8326F0E8)
	// 8326F0B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326F0B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326F0B8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326F0BC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8326F0C0: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8326F0C4: 386B4300  addi r3, r11, 0x4300
	ctx.r[3].s64 = ctx.r[11].s64 + 17152;
	// 8326F0C8: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8326F0CC: 4AF84C8D  bl 0x821f3d58
	ctx.lr = 0x8326F0D0;
	sub_821F3D58(ctx, base);
	// 8326F0D0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326F0D4: 906ACCA8  stw r3, -0x3358(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-13144 as u32), ctx.r[3].u32 ) };
	// 8326F0D8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326F0DC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326F0E0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326F0E4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326F0E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326F0E8 size=56
    let mut pc: u32 = 0x8326F0E8;
    'dispatch: loop {
        match pc {
            0x8326F0E8 => {
    //   block [0x8326F0E8..0x8326F120)
	// 8326F0E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326F0EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326F0F0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326F0F4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8326F0F8: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8326F0FC: 386B4310  addi r3, r11, 0x4310
	ctx.r[3].s64 = ctx.r[11].s64 + 17168;
	// 8326F100: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8326F104: 4AF84C55  bl 0x821f3d58
	ctx.lr = 0x8326F108;
	sub_821F3D58(ctx, base);
	// 8326F108: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326F10C: 906ACCAC  stw r3, -0x3354(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-13140 as u32), ctx.r[3].u32 ) };
	// 8326F110: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326F114: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326F118: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326F11C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326F120(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326F120 size=56
    let mut pc: u32 = 0x8326F120;
    'dispatch: loop {
        match pc {
            0x8326F120 => {
    //   block [0x8326F120..0x8326F158)
	// 8326F120: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326F124: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326F128: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326F12C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8326F130: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8326F134: 386B4328  addi r3, r11, 0x4328
	ctx.r[3].s64 = ctx.r[11].s64 + 17192;
	// 8326F138: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8326F13C: 4AF84C1D  bl 0x821f3d58
	ctx.lr = 0x8326F140;
	sub_821F3D58(ctx, base);
	// 8326F140: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326F144: 906ACCB0  stw r3, -0x3350(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-13136 as u32), ctx.r[3].u32 ) };
	// 8326F148: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326F14C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326F150: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326F154: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326F158(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326F158 size=56
    let mut pc: u32 = 0x8326F158;
    'dispatch: loop {
        match pc {
            0x8326F158 => {
    //   block [0x8326F158..0x8326F190)
	// 8326F158: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326F15C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326F160: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326F164: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8326F168: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8326F16C: 386B4338  addi r3, r11, 0x4338
	ctx.r[3].s64 = ctx.r[11].s64 + 17208;
	// 8326F170: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8326F174: 4AF84BE5  bl 0x821f3d58
	ctx.lr = 0x8326F178;
	sub_821F3D58(ctx, base);
	// 8326F178: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326F17C: 906ACCB4  stw r3, -0x334c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-13132 as u32), ctx.r[3].u32 ) };
	// 8326F180: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326F184: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326F188: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326F18C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326F190(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326F190 size=56
    let mut pc: u32 = 0x8326F190;
    'dispatch: loop {
        match pc {
            0x8326F190 => {
    //   block [0x8326F190..0x8326F1C8)
	// 8326F190: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326F194: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326F198: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326F19C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8326F1A0: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8326F1A4: 386B4350  addi r3, r11, 0x4350
	ctx.r[3].s64 = ctx.r[11].s64 + 17232;
	// 8326F1A8: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8326F1AC: 4AF84BAD  bl 0x821f3d58
	ctx.lr = 0x8326F1B0;
	sub_821F3D58(ctx, base);
	// 8326F1B0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326F1B4: 906ACCB8  stw r3, -0x3348(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-13128 as u32), ctx.r[3].u32 ) };
	// 8326F1B8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326F1BC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326F1C0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326F1C4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326F1C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326F1C8 size=56
    let mut pc: u32 = 0x8326F1C8;
    'dispatch: loop {
        match pc {
            0x8326F1C8 => {
    //   block [0x8326F1C8..0x8326F200)
	// 8326F1C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326F1CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326F1D0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326F1D4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8326F1D8: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8326F1DC: 386B4368  addi r3, r11, 0x4368
	ctx.r[3].s64 = ctx.r[11].s64 + 17256;
	// 8326F1E0: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8326F1E4: 4AF84B75  bl 0x821f3d58
	ctx.lr = 0x8326F1E8;
	sub_821F3D58(ctx, base);
	// 8326F1E8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326F1EC: 906ACCBC  stw r3, -0x3344(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-13124 as u32), ctx.r[3].u32 ) };
	// 8326F1F0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326F1F4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326F1F8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326F1FC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326F200(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326F200 size=56
    let mut pc: u32 = 0x8326F200;
    'dispatch: loop {
        match pc {
            0x8326F200 => {
    //   block [0x8326F200..0x8326F238)
	// 8326F200: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326F204: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326F208: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326F20C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8326F210: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8326F214: 386B4380  addi r3, r11, 0x4380
	ctx.r[3].s64 = ctx.r[11].s64 + 17280;
	// 8326F218: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8326F21C: 4AF84B3D  bl 0x821f3d58
	ctx.lr = 0x8326F220;
	sub_821F3D58(ctx, base);
	// 8326F220: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326F224: 906ACCC0  stw r3, -0x3340(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-13120 as u32), ctx.r[3].u32 ) };
	// 8326F228: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326F22C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326F230: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326F234: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326F238(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326F238 size=56
    let mut pc: u32 = 0x8326F238;
    'dispatch: loop {
        match pc {
            0x8326F238 => {
    //   block [0x8326F238..0x8326F270)
	// 8326F238: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326F23C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326F240: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326F244: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8326F248: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8326F24C: 386B4394  addi r3, r11, 0x4394
	ctx.r[3].s64 = ctx.r[11].s64 + 17300;
	// 8326F250: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8326F254: 4AF84B05  bl 0x821f3d58
	ctx.lr = 0x8326F258;
	sub_821F3D58(ctx, base);
	// 8326F258: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326F25C: 906ACCC4  stw r3, -0x333c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-13116 as u32), ctx.r[3].u32 ) };
	// 8326F260: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326F264: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326F268: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326F26C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326F270(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326F270 size=56
    let mut pc: u32 = 0x8326F270;
    'dispatch: loop {
        match pc {
            0x8326F270 => {
    //   block [0x8326F270..0x8326F2A8)
	// 8326F270: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326F274: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326F278: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326F27C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8326F280: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8326F284: 386B43B4  addi r3, r11, 0x43b4
	ctx.r[3].s64 = ctx.r[11].s64 + 17332;
	// 8326F288: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8326F28C: 4AF84ACD  bl 0x821f3d58
	ctx.lr = 0x8326F290;
	sub_821F3D58(ctx, base);
	// 8326F290: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326F294: 906ACCC8  stw r3, -0x3338(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-13112 as u32), ctx.r[3].u32 ) };
	// 8326F298: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326F29C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326F2A0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326F2A4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326F2A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326F2A8 size=56
    let mut pc: u32 = 0x8326F2A8;
    'dispatch: loop {
        match pc {
            0x8326F2A8 => {
    //   block [0x8326F2A8..0x8326F2E0)
	// 8326F2A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326F2AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326F2B0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326F2B4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8326F2B8: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8326F2BC: 386B43C8  addi r3, r11, 0x43c8
	ctx.r[3].s64 = ctx.r[11].s64 + 17352;
	// 8326F2C0: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8326F2C4: 4AF84A95  bl 0x821f3d58
	ctx.lr = 0x8326F2C8;
	sub_821F3D58(ctx, base);
	// 8326F2C8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326F2CC: 906ACCCC  stw r3, -0x3334(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-13108 as u32), ctx.r[3].u32 ) };
	// 8326F2D0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326F2D4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326F2D8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326F2DC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326F2E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326F2E0 size=56
    let mut pc: u32 = 0x8326F2E0;
    'dispatch: loop {
        match pc {
            0x8326F2E0 => {
    //   block [0x8326F2E0..0x8326F318)
	// 8326F2E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326F2E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326F2E8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326F2EC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8326F2F0: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8326F2F4: 386B43D4  addi r3, r11, 0x43d4
	ctx.r[3].s64 = ctx.r[11].s64 + 17364;
	// 8326F2F8: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8326F2FC: 4AF84A5D  bl 0x821f3d58
	ctx.lr = 0x8326F300;
	sub_821F3D58(ctx, base);
	// 8326F300: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326F304: 906ACCD0  stw r3, -0x3330(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-13104 as u32), ctx.r[3].u32 ) };
	// 8326F308: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326F30C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326F310: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326F314: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326F318(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326F318 size=56
    let mut pc: u32 = 0x8326F318;
    'dispatch: loop {
        match pc {
            0x8326F318 => {
    //   block [0x8326F318..0x8326F350)
	// 8326F318: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326F31C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326F320: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326F324: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8326F328: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8326F32C: 386B43E0  addi r3, r11, 0x43e0
	ctx.r[3].s64 = ctx.r[11].s64 + 17376;
	// 8326F330: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8326F334: 4AF84A25  bl 0x821f3d58
	ctx.lr = 0x8326F338;
	sub_821F3D58(ctx, base);
	// 8326F338: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326F33C: 906ACCD4  stw r3, -0x332c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-13100 as u32), ctx.r[3].u32 ) };
	// 8326F340: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326F344: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326F348: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326F34C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326F350(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326F350 size=56
    let mut pc: u32 = 0x8326F350;
    'dispatch: loop {
        match pc {
            0x8326F350 => {
    //   block [0x8326F350..0x8326F388)
	// 8326F350: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326F354: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326F358: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326F35C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8326F360: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8326F364: 386B43F0  addi r3, r11, 0x43f0
	ctx.r[3].s64 = ctx.r[11].s64 + 17392;
	// 8326F368: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8326F36C: 4AF849ED  bl 0x821f3d58
	ctx.lr = 0x8326F370;
	sub_821F3D58(ctx, base);
	// 8326F370: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326F374: 906ACCD8  stw r3, -0x3328(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-13096 as u32), ctx.r[3].u32 ) };
	// 8326F378: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326F37C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326F380: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326F384: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326F388(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326F388 size=56
    let mut pc: u32 = 0x8326F388;
    'dispatch: loop {
        match pc {
            0x8326F388 => {
    //   block [0x8326F388..0x8326F3C0)
	// 8326F388: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326F38C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326F390: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326F394: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8326F398: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8326F39C: 386B4400  addi r3, r11, 0x4400
	ctx.r[3].s64 = ctx.r[11].s64 + 17408;
	// 8326F3A0: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8326F3A4: 4AF849B5  bl 0x821f3d58
	ctx.lr = 0x8326F3A8;
	sub_821F3D58(ctx, base);
	// 8326F3A8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326F3AC: 906ACCDC  stw r3, -0x3324(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-13092 as u32), ctx.r[3].u32 ) };
	// 8326F3B0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326F3B4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326F3B8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326F3BC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326F3C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326F3C0 size=56
    let mut pc: u32 = 0x8326F3C0;
    'dispatch: loop {
        match pc {
            0x8326F3C0 => {
    //   block [0x8326F3C0..0x8326F3F8)
	// 8326F3C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326F3C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326F3C8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326F3CC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8326F3D0: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8326F3D4: 386B4418  addi r3, r11, 0x4418
	ctx.r[3].s64 = ctx.r[11].s64 + 17432;
	// 8326F3D8: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8326F3DC: 4AF8497D  bl 0x821f3d58
	ctx.lr = 0x8326F3E0;
	sub_821F3D58(ctx, base);
	// 8326F3E0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326F3E4: 906ACCE0  stw r3, -0x3320(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-13088 as u32), ctx.r[3].u32 ) };
	// 8326F3E8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326F3EC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326F3F0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326F3F4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326F3F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326F3F8 size=56
    let mut pc: u32 = 0x8326F3F8;
    'dispatch: loop {
        match pc {
            0x8326F3F8 => {
    //   block [0x8326F3F8..0x8326F430)
	// 8326F3F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326F3FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326F400: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326F404: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8326F408: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8326F40C: 386B4424  addi r3, r11, 0x4424
	ctx.r[3].s64 = ctx.r[11].s64 + 17444;
	// 8326F410: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8326F414: 4AF84945  bl 0x821f3d58
	ctx.lr = 0x8326F418;
	sub_821F3D58(ctx, base);
	// 8326F418: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326F41C: 906ACCE4  stw r3, -0x331c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-13084 as u32), ctx.r[3].u32 ) };
	// 8326F420: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326F424: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326F428: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326F42C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326F430(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326F430 size=56
    let mut pc: u32 = 0x8326F430;
    'dispatch: loop {
        match pc {
            0x8326F430 => {
    //   block [0x8326F430..0x8326F468)
	// 8326F430: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326F434: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326F438: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326F43C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8326F440: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8326F444: 386B4430  addi r3, r11, 0x4430
	ctx.r[3].s64 = ctx.r[11].s64 + 17456;
	// 8326F448: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8326F44C: 4AF8490D  bl 0x821f3d58
	ctx.lr = 0x8326F450;
	sub_821F3D58(ctx, base);
	// 8326F450: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326F454: 906ACCE8  stw r3, -0x3318(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-13080 as u32), ctx.r[3].u32 ) };
	// 8326F458: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326F45C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326F460: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326F464: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326F468(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326F468 size=56
    let mut pc: u32 = 0x8326F468;
    'dispatch: loop {
        match pc {
            0x8326F468 => {
    //   block [0x8326F468..0x8326F4A0)
	// 8326F468: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326F46C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326F470: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326F474: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8326F478: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8326F47C: 386B4440  addi r3, r11, 0x4440
	ctx.r[3].s64 = ctx.r[11].s64 + 17472;
	// 8326F480: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8326F484: 4AF848D5  bl 0x821f3d58
	ctx.lr = 0x8326F488;
	sub_821F3D58(ctx, base);
	// 8326F488: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326F48C: 906ACCEC  stw r3, -0x3314(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-13076 as u32), ctx.r[3].u32 ) };
	// 8326F490: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326F494: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326F498: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326F49C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326F4A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326F4A0 size=64
    let mut pc: u32 = 0x8326F4A0;
    'dispatch: loop {
        match pc {
            0x8326F4A0 => {
    //   block [0x8326F4A0..0x8326F4E0)
	// 8326F4A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326F4A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326F4A8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326F4AC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8326F4B0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326F4B4: 388B052C  addi r4, r11, 0x52c
	ctx.r[4].s64 = ctx.r[11].s64 + 1324;
	// 8326F4B8: 386ACCF0  addi r3, r10, -0x3310
	ctx.r[3].s64 = ctx.r[10].s64 + -13072;
	// 8326F4BC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8326F4C0: 4AFBDA11  bl 0x8222ced0
	ctx.lr = 0x8326F4C4;
	sub_8222CED0(ctx, base);
	// 8326F4C4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8326F4C8: 3869EFD0  addi r3, r9, -0x1030
	ctx.r[3].s64 = ctx.r[9].s64 + -4144;
	// 8326F4CC: 4BA3AA55  bl 0x82ca9f20
	ctx.lr = 0x8326F4D0;
	sub_82CA9F20(ctx, base);
	// 8326F4D0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326F4D4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326F4D8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326F4DC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326F4E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326F4E0 size=64
    let mut pc: u32 = 0x8326F4E0;
    'dispatch: loop {
        match pc {
            0x8326F4E0 => {
    //   block [0x8326F4E0..0x8326F520)
	// 8326F4E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326F4E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326F4E8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326F4EC: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 8326F4F0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326F4F4: 388B9D80  addi r4, r11, -0x6280
	ctx.r[4].s64 = ctx.r[11].s64 + -25216;
	// 8326F4F8: 386ACCF4  addi r3, r10, -0x330c
	ctx.r[3].s64 = ctx.r[10].s64 + -13068;
	// 8326F4FC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8326F500: 4AFBD9D1  bl 0x8222ced0
	ctx.lr = 0x8326F504;
	sub_8222CED0(ctx, base);
	// 8326F504: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8326F508: 3869EFE0  addi r3, r9, -0x1020
	ctx.r[3].s64 = ctx.r[9].s64 + -4128;
	// 8326F50C: 4BA3AA15  bl 0x82ca9f20
	ctx.lr = 0x8326F510;
	sub_82CA9F20(ctx, base);
	// 8326F510: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326F514: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326F518: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326F51C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326F520(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326F520 size=64
    let mut pc: u32 = 0x8326F520;
    'dispatch: loop {
        match pc {
            0x8326F520 => {
    //   block [0x8326F520..0x8326F560)
	// 8326F520: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326F524: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326F528: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326F52C: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 8326F530: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326F534: 388B9D94  addi r4, r11, -0x626c
	ctx.r[4].s64 = ctx.r[11].s64 + -25196;
	// 8326F538: 386ACCF8  addi r3, r10, -0x3308
	ctx.r[3].s64 = ctx.r[10].s64 + -13064;
	// 8326F53C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8326F540: 4AFBD991  bl 0x8222ced0
	ctx.lr = 0x8326F544;
	sub_8222CED0(ctx, base);
	// 8326F544: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8326F548: 3869EFF0  addi r3, r9, -0x1010
	ctx.r[3].s64 = ctx.r[9].s64 + -4112;
	// 8326F54C: 4BA3A9D5  bl 0x82ca9f20
	ctx.lr = 0x8326F550;
	sub_82CA9F20(ctx, base);
	// 8326F550: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326F554: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326F558: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326F55C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326F560(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326F560 size=56
    let mut pc: u32 = 0x8326F560;
    'dispatch: loop {
        match pc {
            0x8326F560 => {
    //   block [0x8326F560..0x8326F598)
	// 8326F560: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326F564: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326F568: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326F56C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8326F570: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8326F574: 386B42C8  addi r3, r11, 0x42c8
	ctx.r[3].s64 = ctx.r[11].s64 + 17096;
	// 8326F578: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8326F57C: 4AF847DD  bl 0x821f3d58
	ctx.lr = 0x8326F580;
	sub_821F3D58(ctx, base);
	// 8326F580: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326F584: 906ACCFC  stw r3, -0x3304(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-13060 as u32), ctx.r[3].u32 ) };
	// 8326F588: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326F58C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326F590: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326F594: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326F598(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326F598 size=56
    let mut pc: u32 = 0x8326F598;
    'dispatch: loop {
        match pc {
            0x8326F598 => {
    //   block [0x8326F598..0x8326F5D0)
	// 8326F598: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326F59C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326F5A0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326F5A4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8326F5A8: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8326F5AC: 386B42DC  addi r3, r11, 0x42dc
	ctx.r[3].s64 = ctx.r[11].s64 + 17116;
	// 8326F5B0: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8326F5B4: 4AF847A5  bl 0x821f3d58
	ctx.lr = 0x8326F5B8;
	sub_821F3D58(ctx, base);
	// 8326F5B8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326F5BC: 906ACD00  stw r3, -0x3300(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-13056 as u32), ctx.r[3].u32 ) };
	// 8326F5C0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326F5C4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326F5C8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326F5CC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326F5D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326F5D0 size=56
    let mut pc: u32 = 0x8326F5D0;
    'dispatch: loop {
        match pc {
            0x8326F5D0 => {
    //   block [0x8326F5D0..0x8326F608)
	// 8326F5D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326F5D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326F5D8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326F5DC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8326F5E0: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8326F5E4: 386B42E8  addi r3, r11, 0x42e8
	ctx.r[3].s64 = ctx.r[11].s64 + 17128;
	// 8326F5E8: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8326F5EC: 4AF8476D  bl 0x821f3d58
	ctx.lr = 0x8326F5F0;
	sub_821F3D58(ctx, base);
	// 8326F5F0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326F5F4: 906ACD04  stw r3, -0x32fc(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-13052 as u32), ctx.r[3].u32 ) };
	// 8326F5F8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326F5FC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326F600: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326F604: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326F608(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326F608 size=56
    let mut pc: u32 = 0x8326F608;
    'dispatch: loop {
        match pc {
            0x8326F608 => {
    //   block [0x8326F608..0x8326F640)
	// 8326F608: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326F60C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326F610: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326F614: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8326F618: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8326F61C: 386B4300  addi r3, r11, 0x4300
	ctx.r[3].s64 = ctx.r[11].s64 + 17152;
	// 8326F620: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8326F624: 4AF84735  bl 0x821f3d58
	ctx.lr = 0x8326F628;
	sub_821F3D58(ctx, base);
	// 8326F628: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326F62C: 906ACD08  stw r3, -0x32f8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-13048 as u32), ctx.r[3].u32 ) };
	// 8326F630: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326F634: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326F638: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326F63C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326F640(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326F640 size=56
    let mut pc: u32 = 0x8326F640;
    'dispatch: loop {
        match pc {
            0x8326F640 => {
    //   block [0x8326F640..0x8326F678)
	// 8326F640: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326F644: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326F648: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326F64C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8326F650: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8326F654: 386B4310  addi r3, r11, 0x4310
	ctx.r[3].s64 = ctx.r[11].s64 + 17168;
	// 8326F658: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8326F65C: 4AF846FD  bl 0x821f3d58
	ctx.lr = 0x8326F660;
	sub_821F3D58(ctx, base);
	// 8326F660: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326F664: 906ACD0C  stw r3, -0x32f4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-13044 as u32), ctx.r[3].u32 ) };
	// 8326F668: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326F66C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326F670: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326F674: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326F678(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326F678 size=56
    let mut pc: u32 = 0x8326F678;
    'dispatch: loop {
        match pc {
            0x8326F678 => {
    //   block [0x8326F678..0x8326F6B0)
	// 8326F678: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326F67C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326F680: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326F684: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8326F688: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8326F68C: 386B4328  addi r3, r11, 0x4328
	ctx.r[3].s64 = ctx.r[11].s64 + 17192;
	// 8326F690: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8326F694: 4AF846C5  bl 0x821f3d58
	ctx.lr = 0x8326F698;
	sub_821F3D58(ctx, base);
	// 8326F698: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326F69C: 906ACD10  stw r3, -0x32f0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-13040 as u32), ctx.r[3].u32 ) };
	// 8326F6A0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326F6A4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326F6A8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326F6AC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326F6B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326F6B0 size=56
    let mut pc: u32 = 0x8326F6B0;
    'dispatch: loop {
        match pc {
            0x8326F6B0 => {
    //   block [0x8326F6B0..0x8326F6E8)
	// 8326F6B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326F6B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326F6B8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326F6BC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8326F6C0: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8326F6C4: 386B4338  addi r3, r11, 0x4338
	ctx.r[3].s64 = ctx.r[11].s64 + 17208;
	// 8326F6C8: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8326F6CC: 4AF8468D  bl 0x821f3d58
	ctx.lr = 0x8326F6D0;
	sub_821F3D58(ctx, base);
	// 8326F6D0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326F6D4: 906ACD14  stw r3, -0x32ec(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-13036 as u32), ctx.r[3].u32 ) };
	// 8326F6D8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326F6DC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326F6E0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326F6E4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326F6E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326F6E8 size=56
    let mut pc: u32 = 0x8326F6E8;
    'dispatch: loop {
        match pc {
            0x8326F6E8 => {
    //   block [0x8326F6E8..0x8326F720)
	// 8326F6E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326F6EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326F6F0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326F6F4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8326F6F8: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8326F6FC: 386B4350  addi r3, r11, 0x4350
	ctx.r[3].s64 = ctx.r[11].s64 + 17232;
	// 8326F700: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8326F704: 4AF84655  bl 0x821f3d58
	ctx.lr = 0x8326F708;
	sub_821F3D58(ctx, base);
	// 8326F708: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326F70C: 906ACD18  stw r3, -0x32e8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-13032 as u32), ctx.r[3].u32 ) };
	// 8326F710: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326F714: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326F718: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326F71C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326F720(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326F720 size=56
    let mut pc: u32 = 0x8326F720;
    'dispatch: loop {
        match pc {
            0x8326F720 => {
    //   block [0x8326F720..0x8326F758)
	// 8326F720: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326F724: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326F728: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326F72C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8326F730: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8326F734: 386B4368  addi r3, r11, 0x4368
	ctx.r[3].s64 = ctx.r[11].s64 + 17256;
	// 8326F738: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8326F73C: 4AF8461D  bl 0x821f3d58
	ctx.lr = 0x8326F740;
	sub_821F3D58(ctx, base);
	// 8326F740: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326F744: 906ACD1C  stw r3, -0x32e4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-13028 as u32), ctx.r[3].u32 ) };
	// 8326F748: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326F74C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326F750: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326F754: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326F758(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326F758 size=56
    let mut pc: u32 = 0x8326F758;
    'dispatch: loop {
        match pc {
            0x8326F758 => {
    //   block [0x8326F758..0x8326F790)
	// 8326F758: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326F75C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326F760: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326F764: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8326F768: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8326F76C: 386B4380  addi r3, r11, 0x4380
	ctx.r[3].s64 = ctx.r[11].s64 + 17280;
	// 8326F770: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8326F774: 4AF845E5  bl 0x821f3d58
	ctx.lr = 0x8326F778;
	sub_821F3D58(ctx, base);
	// 8326F778: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326F77C: 906ACD20  stw r3, -0x32e0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-13024 as u32), ctx.r[3].u32 ) };
	// 8326F780: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326F784: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326F788: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326F78C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326F790(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326F790 size=56
    let mut pc: u32 = 0x8326F790;
    'dispatch: loop {
        match pc {
            0x8326F790 => {
    //   block [0x8326F790..0x8326F7C8)
	// 8326F790: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326F794: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326F798: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326F79C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8326F7A0: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8326F7A4: 386B4394  addi r3, r11, 0x4394
	ctx.r[3].s64 = ctx.r[11].s64 + 17300;
	// 8326F7A8: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8326F7AC: 4AF845AD  bl 0x821f3d58
	ctx.lr = 0x8326F7B0;
	sub_821F3D58(ctx, base);
	// 8326F7B0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326F7B4: 906ACD24  stw r3, -0x32dc(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-13020 as u32), ctx.r[3].u32 ) };
	// 8326F7B8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326F7BC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326F7C0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326F7C4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326F7C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326F7C8 size=56
    let mut pc: u32 = 0x8326F7C8;
    'dispatch: loop {
        match pc {
            0x8326F7C8 => {
    //   block [0x8326F7C8..0x8326F800)
	// 8326F7C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326F7CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326F7D0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326F7D4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8326F7D8: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8326F7DC: 386B43B4  addi r3, r11, 0x43b4
	ctx.r[3].s64 = ctx.r[11].s64 + 17332;
	// 8326F7E0: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8326F7E4: 4AF84575  bl 0x821f3d58
	ctx.lr = 0x8326F7E8;
	sub_821F3D58(ctx, base);
	// 8326F7E8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326F7EC: 906ACD28  stw r3, -0x32d8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-13016 as u32), ctx.r[3].u32 ) };
	// 8326F7F0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326F7F4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326F7F8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326F7FC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326F800(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326F800 size=56
    let mut pc: u32 = 0x8326F800;
    'dispatch: loop {
        match pc {
            0x8326F800 => {
    //   block [0x8326F800..0x8326F838)
	// 8326F800: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326F804: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326F808: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326F80C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8326F810: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8326F814: 386B43C8  addi r3, r11, 0x43c8
	ctx.r[3].s64 = ctx.r[11].s64 + 17352;
	// 8326F818: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8326F81C: 4AF8453D  bl 0x821f3d58
	ctx.lr = 0x8326F820;
	sub_821F3D58(ctx, base);
	// 8326F820: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326F824: 906ACD2C  stw r3, -0x32d4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-13012 as u32), ctx.r[3].u32 ) };
	// 8326F828: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326F82C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326F830: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326F834: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326F838(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326F838 size=56
    let mut pc: u32 = 0x8326F838;
    'dispatch: loop {
        match pc {
            0x8326F838 => {
    //   block [0x8326F838..0x8326F870)
	// 8326F838: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326F83C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326F840: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326F844: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8326F848: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8326F84C: 386B43D4  addi r3, r11, 0x43d4
	ctx.r[3].s64 = ctx.r[11].s64 + 17364;
	// 8326F850: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8326F854: 4AF84505  bl 0x821f3d58
	ctx.lr = 0x8326F858;
	sub_821F3D58(ctx, base);
	// 8326F858: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326F85C: 906ACD30  stw r3, -0x32d0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-13008 as u32), ctx.r[3].u32 ) };
	// 8326F860: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326F864: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326F868: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326F86C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326F870(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326F870 size=56
    let mut pc: u32 = 0x8326F870;
    'dispatch: loop {
        match pc {
            0x8326F870 => {
    //   block [0x8326F870..0x8326F8A8)
	// 8326F870: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326F874: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326F878: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326F87C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8326F880: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8326F884: 386B43E0  addi r3, r11, 0x43e0
	ctx.r[3].s64 = ctx.r[11].s64 + 17376;
	// 8326F888: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8326F88C: 4AF844CD  bl 0x821f3d58
	ctx.lr = 0x8326F890;
	sub_821F3D58(ctx, base);
	// 8326F890: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326F894: 906ACD34  stw r3, -0x32cc(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-13004 as u32), ctx.r[3].u32 ) };
	// 8326F898: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326F89C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326F8A0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326F8A4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326F8A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326F8A8 size=56
    let mut pc: u32 = 0x8326F8A8;
    'dispatch: loop {
        match pc {
            0x8326F8A8 => {
    //   block [0x8326F8A8..0x8326F8E0)
	// 8326F8A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326F8AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326F8B0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326F8B4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8326F8B8: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8326F8BC: 386B43F0  addi r3, r11, 0x43f0
	ctx.r[3].s64 = ctx.r[11].s64 + 17392;
	// 8326F8C0: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8326F8C4: 4AF84495  bl 0x821f3d58
	ctx.lr = 0x8326F8C8;
	sub_821F3D58(ctx, base);
	// 8326F8C8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326F8CC: 906ACD38  stw r3, -0x32c8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-13000 as u32), ctx.r[3].u32 ) };
	// 8326F8D0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326F8D4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326F8D8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326F8DC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326F8E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326F8E0 size=56
    let mut pc: u32 = 0x8326F8E0;
    'dispatch: loop {
        match pc {
            0x8326F8E0 => {
    //   block [0x8326F8E0..0x8326F918)
	// 8326F8E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326F8E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326F8E8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326F8EC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8326F8F0: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8326F8F4: 386B4400  addi r3, r11, 0x4400
	ctx.r[3].s64 = ctx.r[11].s64 + 17408;
	// 8326F8F8: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8326F8FC: 4AF8445D  bl 0x821f3d58
	ctx.lr = 0x8326F900;
	sub_821F3D58(ctx, base);
	// 8326F900: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326F904: 906ACD3C  stw r3, -0x32c4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-12996 as u32), ctx.r[3].u32 ) };
	// 8326F908: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326F90C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326F910: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326F914: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326F918(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326F918 size=56
    let mut pc: u32 = 0x8326F918;
    'dispatch: loop {
        match pc {
            0x8326F918 => {
    //   block [0x8326F918..0x8326F950)
	// 8326F918: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326F91C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326F920: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326F924: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8326F928: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8326F92C: 386B4418  addi r3, r11, 0x4418
	ctx.r[3].s64 = ctx.r[11].s64 + 17432;
	// 8326F930: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8326F934: 4AF84425  bl 0x821f3d58
	ctx.lr = 0x8326F938;
	sub_821F3D58(ctx, base);
	// 8326F938: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326F93C: 906ACD40  stw r3, -0x32c0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-12992 as u32), ctx.r[3].u32 ) };
	// 8326F940: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326F944: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326F948: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326F94C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326F950(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326F950 size=56
    let mut pc: u32 = 0x8326F950;
    'dispatch: loop {
        match pc {
            0x8326F950 => {
    //   block [0x8326F950..0x8326F988)
	// 8326F950: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326F954: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326F958: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326F95C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8326F960: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8326F964: 386B4424  addi r3, r11, 0x4424
	ctx.r[3].s64 = ctx.r[11].s64 + 17444;
	// 8326F968: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8326F96C: 4AF843ED  bl 0x821f3d58
	ctx.lr = 0x8326F970;
	sub_821F3D58(ctx, base);
	// 8326F970: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326F974: 906ACD44  stw r3, -0x32bc(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-12988 as u32), ctx.r[3].u32 ) };
	// 8326F978: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326F97C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326F980: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326F984: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326F988(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326F988 size=56
    let mut pc: u32 = 0x8326F988;
    'dispatch: loop {
        match pc {
            0x8326F988 => {
    //   block [0x8326F988..0x8326F9C0)
	// 8326F988: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326F98C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326F990: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326F994: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8326F998: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8326F99C: 386B4430  addi r3, r11, 0x4430
	ctx.r[3].s64 = ctx.r[11].s64 + 17456;
	// 8326F9A0: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8326F9A4: 4AF843B5  bl 0x821f3d58
	ctx.lr = 0x8326F9A8;
	sub_821F3D58(ctx, base);
	// 8326F9A8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326F9AC: 906ACD48  stw r3, -0x32b8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-12984 as u32), ctx.r[3].u32 ) };
	// 8326F9B0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326F9B4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326F9B8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326F9BC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326F9C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326F9C0 size=56
    let mut pc: u32 = 0x8326F9C0;
    'dispatch: loop {
        match pc {
            0x8326F9C0 => {
    //   block [0x8326F9C0..0x8326F9F8)
	// 8326F9C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326F9C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326F9C8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326F9CC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8326F9D0: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8326F9D4: 386B4440  addi r3, r11, 0x4440
	ctx.r[3].s64 = ctx.r[11].s64 + 17472;
	// 8326F9D8: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8326F9DC: 4AF8437D  bl 0x821f3d58
	ctx.lr = 0x8326F9E0;
	sub_821F3D58(ctx, base);
	// 8326F9E0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326F9E4: 906ACD4C  stw r3, -0x32b4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-12980 as u32), ctx.r[3].u32 ) };
	// 8326F9E8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326F9EC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326F9F0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326F9F4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326F9F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326F9F8 size=64
    let mut pc: u32 = 0x8326F9F8;
    'dispatch: loop {
        match pc {
            0x8326F9F8 => {
    //   block [0x8326F9F8..0x8326FA38)
	// 8326F9F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326F9FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326FA00: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326FA04: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 8326FA08: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326FA0C: 388B9EAC  addi r4, r11, -0x6154
	ctx.r[4].s64 = ctx.r[11].s64 + -24916;
	// 8326FA10: 386ACD50  addi r3, r10, -0x32b0
	ctx.r[3].s64 = ctx.r[10].s64 + -12976;
	// 8326FA14: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8326FA18: 4AFBD4B9  bl 0x8222ced0
	ctx.lr = 0x8326FA1C;
	sub_8222CED0(ctx, base);
	// 8326FA1C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8326FA20: 3869F000  addi r3, r9, -0x1000
	ctx.r[3].s64 = ctx.r[9].s64 + -4096;
	// 8326FA24: 4BA3A4FD  bl 0x82ca9f20
	ctx.lr = 0x8326FA28;
	sub_82CA9F20(ctx, base);
	// 8326FA28: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326FA2C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326FA30: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326FA34: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326FA38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326FA38 size=64
    let mut pc: u32 = 0x8326FA38;
    'dispatch: loop {
        match pc {
            0x8326FA38 => {
    //   block [0x8326FA38..0x8326FA78)
	// 8326FA38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326FA3C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326FA40: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326FA44: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8326FA48: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326FA4C: 388BD4FC  addi r4, r11, -0x2b04
	ctx.r[4].s64 = ctx.r[11].s64 + -11012;
	// 8326FA50: 386ACD54  addi r3, r10, -0x32ac
	ctx.r[3].s64 = ctx.r[10].s64 + -12972;
	// 8326FA54: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8326FA58: 4AFBD479  bl 0x8222ced0
	ctx.lr = 0x8326FA5C;
	sub_8222CED0(ctx, base);
	// 8326FA5C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8326FA60: 3869F010  addi r3, r9, -0xff0
	ctx.r[3].s64 = ctx.r[9].s64 + -4080;
	// 8326FA64: 4BA3A4BD  bl 0x82ca9f20
	ctx.lr = 0x8326FA68;
	sub_82CA9F20(ctx, base);
	// 8326FA68: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326FA6C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326FA70: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326FA74: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326FA78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326FA78 size=64
    let mut pc: u32 = 0x8326FA78;
    'dispatch: loop {
        match pc {
            0x8326FA78 => {
    //   block [0x8326FA78..0x8326FAB8)
	// 8326FA78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326FA7C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326FA80: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326FA84: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 8326FA88: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326FA8C: 388B9ED8  addi r4, r11, -0x6128
	ctx.r[4].s64 = ctx.r[11].s64 + -24872;
	// 8326FA90: 386ACD58  addi r3, r10, -0x32a8
	ctx.r[3].s64 = ctx.r[10].s64 + -12968;
	// 8326FA94: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8326FA98: 4AFBD439  bl 0x8222ced0
	ctx.lr = 0x8326FA9C;
	sub_8222CED0(ctx, base);
	// 8326FA9C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8326FAA0: 3869F020  addi r3, r9, -0xfe0
	ctx.r[3].s64 = ctx.r[9].s64 + -4064;
	// 8326FAA4: 4BA3A47D  bl 0x82ca9f20
	ctx.lr = 0x8326FAA8;
	sub_82CA9F20(ctx, base);
	// 8326FAA8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326FAAC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326FAB0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326FAB4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326FAB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326FAB8 size=56
    let mut pc: u32 = 0x8326FAB8;
    'dispatch: loop {
        match pc {
            0x8326FAB8 => {
    //   block [0x8326FAB8..0x8326FAF0)
	// 8326FAB8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326FABC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326FAC0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326FAC4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8326FAC8: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8326FACC: 386B42C8  addi r3, r11, 0x42c8
	ctx.r[3].s64 = ctx.r[11].s64 + 17096;
	// 8326FAD0: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8326FAD4: 4AF84285  bl 0x821f3d58
	ctx.lr = 0x8326FAD8;
	sub_821F3D58(ctx, base);
	// 8326FAD8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326FADC: 906ACD5C  stw r3, -0x32a4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-12964 as u32), ctx.r[3].u32 ) };
	// 8326FAE0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326FAE4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326FAE8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326FAEC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326FAF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326FAF0 size=56
    let mut pc: u32 = 0x8326FAF0;
    'dispatch: loop {
        match pc {
            0x8326FAF0 => {
    //   block [0x8326FAF0..0x8326FB28)
	// 8326FAF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326FAF4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326FAF8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326FAFC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8326FB00: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8326FB04: 386B42DC  addi r3, r11, 0x42dc
	ctx.r[3].s64 = ctx.r[11].s64 + 17116;
	// 8326FB08: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8326FB0C: 4AF8424D  bl 0x821f3d58
	ctx.lr = 0x8326FB10;
	sub_821F3D58(ctx, base);
	// 8326FB10: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326FB14: 906ACD60  stw r3, -0x32a0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-12960 as u32), ctx.r[3].u32 ) };
	// 8326FB18: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326FB1C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326FB20: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326FB24: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326FB28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326FB28 size=56
    let mut pc: u32 = 0x8326FB28;
    'dispatch: loop {
        match pc {
            0x8326FB28 => {
    //   block [0x8326FB28..0x8326FB60)
	// 8326FB28: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326FB2C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326FB30: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326FB34: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8326FB38: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8326FB3C: 386B42E8  addi r3, r11, 0x42e8
	ctx.r[3].s64 = ctx.r[11].s64 + 17128;
	// 8326FB40: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8326FB44: 4AF84215  bl 0x821f3d58
	ctx.lr = 0x8326FB48;
	sub_821F3D58(ctx, base);
	// 8326FB48: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326FB4C: 906ACD64  stw r3, -0x329c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-12956 as u32), ctx.r[3].u32 ) };
	// 8326FB50: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326FB54: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326FB58: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326FB5C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326FB60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326FB60 size=56
    let mut pc: u32 = 0x8326FB60;
    'dispatch: loop {
        match pc {
            0x8326FB60 => {
    //   block [0x8326FB60..0x8326FB98)
	// 8326FB60: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326FB64: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326FB68: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326FB6C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8326FB70: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8326FB74: 386B4300  addi r3, r11, 0x4300
	ctx.r[3].s64 = ctx.r[11].s64 + 17152;
	// 8326FB78: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8326FB7C: 4AF841DD  bl 0x821f3d58
	ctx.lr = 0x8326FB80;
	sub_821F3D58(ctx, base);
	// 8326FB80: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326FB84: 906ACD68  stw r3, -0x3298(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-12952 as u32), ctx.r[3].u32 ) };
	// 8326FB88: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326FB8C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326FB90: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326FB94: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326FB98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326FB98 size=56
    let mut pc: u32 = 0x8326FB98;
    'dispatch: loop {
        match pc {
            0x8326FB98 => {
    //   block [0x8326FB98..0x8326FBD0)
	// 8326FB98: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326FB9C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326FBA0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326FBA4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8326FBA8: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8326FBAC: 386B4310  addi r3, r11, 0x4310
	ctx.r[3].s64 = ctx.r[11].s64 + 17168;
	// 8326FBB0: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8326FBB4: 4AF841A5  bl 0x821f3d58
	ctx.lr = 0x8326FBB8;
	sub_821F3D58(ctx, base);
	// 8326FBB8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326FBBC: 906ACD6C  stw r3, -0x3294(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-12948 as u32), ctx.r[3].u32 ) };
	// 8326FBC0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326FBC4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326FBC8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326FBCC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326FBD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326FBD0 size=56
    let mut pc: u32 = 0x8326FBD0;
    'dispatch: loop {
        match pc {
            0x8326FBD0 => {
    //   block [0x8326FBD0..0x8326FC08)
	// 8326FBD0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326FBD4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326FBD8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326FBDC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8326FBE0: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8326FBE4: 386B4328  addi r3, r11, 0x4328
	ctx.r[3].s64 = ctx.r[11].s64 + 17192;
	// 8326FBE8: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8326FBEC: 4AF8416D  bl 0x821f3d58
	ctx.lr = 0x8326FBF0;
	sub_821F3D58(ctx, base);
	// 8326FBF0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326FBF4: 906ACD70  stw r3, -0x3290(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-12944 as u32), ctx.r[3].u32 ) };
	// 8326FBF8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326FBFC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326FC00: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326FC04: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326FC08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326FC08 size=56
    let mut pc: u32 = 0x8326FC08;
    'dispatch: loop {
        match pc {
            0x8326FC08 => {
    //   block [0x8326FC08..0x8326FC40)
	// 8326FC08: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326FC0C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326FC10: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326FC14: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8326FC18: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8326FC1C: 386B4338  addi r3, r11, 0x4338
	ctx.r[3].s64 = ctx.r[11].s64 + 17208;
	// 8326FC20: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8326FC24: 4AF84135  bl 0x821f3d58
	ctx.lr = 0x8326FC28;
	sub_821F3D58(ctx, base);
	// 8326FC28: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326FC2C: 906ACD74  stw r3, -0x328c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-12940 as u32), ctx.r[3].u32 ) };
	// 8326FC30: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326FC34: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326FC38: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326FC3C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326FC40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326FC40 size=56
    let mut pc: u32 = 0x8326FC40;
    'dispatch: loop {
        match pc {
            0x8326FC40 => {
    //   block [0x8326FC40..0x8326FC78)
	// 8326FC40: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326FC44: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326FC48: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326FC4C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8326FC50: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8326FC54: 386B4350  addi r3, r11, 0x4350
	ctx.r[3].s64 = ctx.r[11].s64 + 17232;
	// 8326FC58: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8326FC5C: 4AF840FD  bl 0x821f3d58
	ctx.lr = 0x8326FC60;
	sub_821F3D58(ctx, base);
	// 8326FC60: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326FC64: 906ACD78  stw r3, -0x3288(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-12936 as u32), ctx.r[3].u32 ) };
	// 8326FC68: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326FC6C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326FC70: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326FC74: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326FC78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326FC78 size=56
    let mut pc: u32 = 0x8326FC78;
    'dispatch: loop {
        match pc {
            0x8326FC78 => {
    //   block [0x8326FC78..0x8326FCB0)
	// 8326FC78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326FC7C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326FC80: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326FC84: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8326FC88: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8326FC8C: 386B4368  addi r3, r11, 0x4368
	ctx.r[3].s64 = ctx.r[11].s64 + 17256;
	// 8326FC90: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8326FC94: 4AF840C5  bl 0x821f3d58
	ctx.lr = 0x8326FC98;
	sub_821F3D58(ctx, base);
	// 8326FC98: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326FC9C: 906ACD7C  stw r3, -0x3284(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-12932 as u32), ctx.r[3].u32 ) };
	// 8326FCA0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326FCA4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326FCA8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326FCAC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326FCB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326FCB0 size=56
    let mut pc: u32 = 0x8326FCB0;
    'dispatch: loop {
        match pc {
            0x8326FCB0 => {
    //   block [0x8326FCB0..0x8326FCE8)
	// 8326FCB0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326FCB4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326FCB8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326FCBC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8326FCC0: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8326FCC4: 386B4380  addi r3, r11, 0x4380
	ctx.r[3].s64 = ctx.r[11].s64 + 17280;
	// 8326FCC8: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8326FCCC: 4AF8408D  bl 0x821f3d58
	ctx.lr = 0x8326FCD0;
	sub_821F3D58(ctx, base);
	// 8326FCD0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326FCD4: 906ACD80  stw r3, -0x3280(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-12928 as u32), ctx.r[3].u32 ) };
	// 8326FCD8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326FCDC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326FCE0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326FCE4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326FCE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326FCE8 size=56
    let mut pc: u32 = 0x8326FCE8;
    'dispatch: loop {
        match pc {
            0x8326FCE8 => {
    //   block [0x8326FCE8..0x8326FD20)
	// 8326FCE8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326FCEC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326FCF0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326FCF4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8326FCF8: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8326FCFC: 386B4394  addi r3, r11, 0x4394
	ctx.r[3].s64 = ctx.r[11].s64 + 17300;
	// 8326FD00: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8326FD04: 4AF84055  bl 0x821f3d58
	ctx.lr = 0x8326FD08;
	sub_821F3D58(ctx, base);
	// 8326FD08: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326FD0C: 906ACD84  stw r3, -0x327c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-12924 as u32), ctx.r[3].u32 ) };
	// 8326FD10: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326FD14: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326FD18: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326FD1C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326FD20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326FD20 size=56
    let mut pc: u32 = 0x8326FD20;
    'dispatch: loop {
        match pc {
            0x8326FD20 => {
    //   block [0x8326FD20..0x8326FD58)
	// 8326FD20: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326FD24: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326FD28: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326FD2C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8326FD30: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8326FD34: 386B43B4  addi r3, r11, 0x43b4
	ctx.r[3].s64 = ctx.r[11].s64 + 17332;
	// 8326FD38: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8326FD3C: 4AF8401D  bl 0x821f3d58
	ctx.lr = 0x8326FD40;
	sub_821F3D58(ctx, base);
	// 8326FD40: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326FD44: 906ACD88  stw r3, -0x3278(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-12920 as u32), ctx.r[3].u32 ) };
	// 8326FD48: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326FD4C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326FD50: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326FD54: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326FD58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326FD58 size=56
    let mut pc: u32 = 0x8326FD58;
    'dispatch: loop {
        match pc {
            0x8326FD58 => {
    //   block [0x8326FD58..0x8326FD90)
	// 8326FD58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326FD5C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326FD60: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326FD64: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8326FD68: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8326FD6C: 386B43C8  addi r3, r11, 0x43c8
	ctx.r[3].s64 = ctx.r[11].s64 + 17352;
	// 8326FD70: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8326FD74: 4AF83FE5  bl 0x821f3d58
	ctx.lr = 0x8326FD78;
	sub_821F3D58(ctx, base);
	// 8326FD78: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326FD7C: 906ACD8C  stw r3, -0x3274(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-12916 as u32), ctx.r[3].u32 ) };
	// 8326FD80: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326FD84: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326FD88: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326FD8C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326FD90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326FD90 size=56
    let mut pc: u32 = 0x8326FD90;
    'dispatch: loop {
        match pc {
            0x8326FD90 => {
    //   block [0x8326FD90..0x8326FDC8)
	// 8326FD90: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326FD94: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326FD98: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326FD9C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8326FDA0: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8326FDA4: 386B43D4  addi r3, r11, 0x43d4
	ctx.r[3].s64 = ctx.r[11].s64 + 17364;
	// 8326FDA8: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8326FDAC: 4AF83FAD  bl 0x821f3d58
	ctx.lr = 0x8326FDB0;
	sub_821F3D58(ctx, base);
	// 8326FDB0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326FDB4: 906ACD90  stw r3, -0x3270(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-12912 as u32), ctx.r[3].u32 ) };
	// 8326FDB8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326FDBC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326FDC0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326FDC4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326FDC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326FDC8 size=56
    let mut pc: u32 = 0x8326FDC8;
    'dispatch: loop {
        match pc {
            0x8326FDC8 => {
    //   block [0x8326FDC8..0x8326FE00)
	// 8326FDC8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326FDCC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326FDD0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326FDD4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8326FDD8: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8326FDDC: 386B43E0  addi r3, r11, 0x43e0
	ctx.r[3].s64 = ctx.r[11].s64 + 17376;
	// 8326FDE0: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8326FDE4: 4AF83F75  bl 0x821f3d58
	ctx.lr = 0x8326FDE8;
	sub_821F3D58(ctx, base);
	// 8326FDE8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326FDEC: 906ACD94  stw r3, -0x326c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-12908 as u32), ctx.r[3].u32 ) };
	// 8326FDF0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326FDF4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326FDF8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326FDFC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326FE00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326FE00 size=56
    let mut pc: u32 = 0x8326FE00;
    'dispatch: loop {
        match pc {
            0x8326FE00 => {
    //   block [0x8326FE00..0x8326FE38)
	// 8326FE00: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326FE04: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326FE08: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326FE0C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8326FE10: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8326FE14: 386B43F0  addi r3, r11, 0x43f0
	ctx.r[3].s64 = ctx.r[11].s64 + 17392;
	// 8326FE18: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8326FE1C: 4AF83F3D  bl 0x821f3d58
	ctx.lr = 0x8326FE20;
	sub_821F3D58(ctx, base);
	// 8326FE20: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326FE24: 906ACD98  stw r3, -0x3268(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-12904 as u32), ctx.r[3].u32 ) };
	// 8326FE28: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326FE2C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326FE30: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326FE34: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326FE38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326FE38 size=56
    let mut pc: u32 = 0x8326FE38;
    'dispatch: loop {
        match pc {
            0x8326FE38 => {
    //   block [0x8326FE38..0x8326FE70)
	// 8326FE38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326FE3C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326FE40: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326FE44: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8326FE48: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8326FE4C: 386B4400  addi r3, r11, 0x4400
	ctx.r[3].s64 = ctx.r[11].s64 + 17408;
	// 8326FE50: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8326FE54: 4AF83F05  bl 0x821f3d58
	ctx.lr = 0x8326FE58;
	sub_821F3D58(ctx, base);
	// 8326FE58: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326FE5C: 906ACD9C  stw r3, -0x3264(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-12900 as u32), ctx.r[3].u32 ) };
	// 8326FE60: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326FE64: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326FE68: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326FE6C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326FE70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326FE70 size=56
    let mut pc: u32 = 0x8326FE70;
    'dispatch: loop {
        match pc {
            0x8326FE70 => {
    //   block [0x8326FE70..0x8326FEA8)
	// 8326FE70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326FE74: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326FE78: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326FE7C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8326FE80: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8326FE84: 386B4418  addi r3, r11, 0x4418
	ctx.r[3].s64 = ctx.r[11].s64 + 17432;
	// 8326FE88: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8326FE8C: 4AF83ECD  bl 0x821f3d58
	ctx.lr = 0x8326FE90;
	sub_821F3D58(ctx, base);
	// 8326FE90: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326FE94: 906ACDA0  stw r3, -0x3260(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-12896 as u32), ctx.r[3].u32 ) };
	// 8326FE98: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326FE9C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326FEA0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326FEA4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326FEA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326FEA8 size=56
    let mut pc: u32 = 0x8326FEA8;
    'dispatch: loop {
        match pc {
            0x8326FEA8 => {
    //   block [0x8326FEA8..0x8326FEE0)
	// 8326FEA8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326FEAC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326FEB0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326FEB4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8326FEB8: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8326FEBC: 386B4424  addi r3, r11, 0x4424
	ctx.r[3].s64 = ctx.r[11].s64 + 17444;
	// 8326FEC0: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8326FEC4: 4AF83E95  bl 0x821f3d58
	ctx.lr = 0x8326FEC8;
	sub_821F3D58(ctx, base);
	// 8326FEC8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326FECC: 906ACDA4  stw r3, -0x325c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-12892 as u32), ctx.r[3].u32 ) };
	// 8326FED0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326FED4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326FED8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326FEDC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326FEE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326FEE0 size=56
    let mut pc: u32 = 0x8326FEE0;
    'dispatch: loop {
        match pc {
            0x8326FEE0 => {
    //   block [0x8326FEE0..0x8326FF18)
	// 8326FEE0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326FEE4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326FEE8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326FEEC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8326FEF0: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8326FEF4: 386B4430  addi r3, r11, 0x4430
	ctx.r[3].s64 = ctx.r[11].s64 + 17456;
	// 8326FEF8: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8326FEFC: 4AF83E5D  bl 0x821f3d58
	ctx.lr = 0x8326FF00;
	sub_821F3D58(ctx, base);
	// 8326FF00: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326FF04: 906ACDA8  stw r3, -0x3258(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-12888 as u32), ctx.r[3].u32 ) };
	// 8326FF08: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326FF0C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326FF10: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326FF14: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326FF18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326FF18 size=56
    let mut pc: u32 = 0x8326FF18;
    'dispatch: loop {
        match pc {
            0x8326FF18 => {
    //   block [0x8326FF18..0x8326FF50)
	// 8326FF18: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326FF1C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326FF20: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326FF24: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8326FF28: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8326FF2C: 386B4440  addi r3, r11, 0x4440
	ctx.r[3].s64 = ctx.r[11].s64 + 17472;
	// 8326FF30: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8326FF34: 4AF83E25  bl 0x821f3d58
	ctx.lr = 0x8326FF38;
	sub_821F3D58(ctx, base);
	// 8326FF38: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326FF3C: 906ACDAC  stw r3, -0x3254(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-12884 as u32), ctx.r[3].u32 ) };
	// 8326FF40: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326FF44: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326FF48: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326FF4C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326FF50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326FF50 size=64
    let mut pc: u32 = 0x8326FF50;
    'dispatch: loop {
        match pc {
            0x8326FF50 => {
    //   block [0x8326FF50..0x8326FF90)
	// 8326FF50: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326FF54: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326FF58: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326FF5C: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 8326FF60: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326FF64: 388B9F54  addi r4, r11, -0x60ac
	ctx.r[4].s64 = ctx.r[11].s64 + -24748;
	// 8326FF68: 386ACDB0  addi r3, r10, -0x3250
	ctx.r[3].s64 = ctx.r[10].s64 + -12880;
	// 8326FF6C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8326FF70: 4AFBCF61  bl 0x8222ced0
	ctx.lr = 0x8326FF74;
	sub_8222CED0(ctx, base);
	// 8326FF74: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8326FF78: 3869F030  addi r3, r9, -0xfd0
	ctx.r[3].s64 = ctx.r[9].s64 + -4048;
	// 8326FF7C: 4BA39FA5  bl 0x82ca9f20
	ctx.lr = 0x8326FF80;
	sub_82CA9F20(ctx, base);
	// 8326FF80: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326FF84: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326FF88: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326FF8C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326FF90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326FF90 size=64
    let mut pc: u32 = 0x8326FF90;
    'dispatch: loop {
        match pc {
            0x8326FF90 => {
    //   block [0x8326FF90..0x8326FFD0)
	// 8326FF90: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326FF94: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326FF98: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326FF9C: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8326FFA0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326FFA4: 388BD4FC  addi r4, r11, -0x2b04
	ctx.r[4].s64 = ctx.r[11].s64 + -11012;
	// 8326FFA8: 386ACDB4  addi r3, r10, -0x324c
	ctx.r[3].s64 = ctx.r[10].s64 + -12876;
	// 8326FFAC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8326FFB0: 4AFBCF21  bl 0x8222ced0
	ctx.lr = 0x8326FFB4;
	sub_8222CED0(ctx, base);
	// 8326FFB4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8326FFB8: 3869F040  addi r3, r9, -0xfc0
	ctx.r[3].s64 = ctx.r[9].s64 + -4032;
	// 8326FFBC: 4BA39F65  bl 0x82ca9f20
	ctx.lr = 0x8326FFC0;
	sub_82CA9F20(ctx, base);
	// 8326FFC0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8326FFC4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8326FFC8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8326FFCC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8326FFD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8326FFD0 size=64
    let mut pc: u32 = 0x8326FFD0;
    'dispatch: loop {
        match pc {
            0x8326FFD0 => {
    //   block [0x8326FFD0..0x83270010)
	// 8326FFD0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8326FFD4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8326FFD8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8326FFDC: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 8326FFE0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8326FFE4: 388B9F80  addi r4, r11, -0x6080
	ctx.r[4].s64 = ctx.r[11].s64 + -24704;
	// 8326FFE8: 386ACDB8  addi r3, r10, -0x3248
	ctx.r[3].s64 = ctx.r[10].s64 + -12872;
	// 8326FFEC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8326FFF0: 4AFBCEE1  bl 0x8222ced0
	ctx.lr = 0x8326FFF4;
	sub_8222CED0(ctx, base);
	// 8326FFF4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 8326FFF8: 3869F050  addi r3, r9, -0xfb0
	ctx.r[3].s64 = ctx.r[9].s64 + -4016;
	// 8326FFFC: 4BA39F25  bl 0x82ca9f20
	ctx.lr = 0x83270000;
	sub_82CA9F20(ctx, base);
	// 83270000: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83270004: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83270008: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327000C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83270010(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83270010 size=56
    let mut pc: u32 = 0x83270010;
    'dispatch: loop {
        match pc {
            0x83270010 => {
    //   block [0x83270010..0x83270048)
	// 83270010: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83270014: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83270018: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327001C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83270020: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83270024: 386B42C8  addi r3, r11, 0x42c8
	ctx.r[3].s64 = ctx.r[11].s64 + 17096;
	// 83270028: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8327002C: 4AF83D2D  bl 0x821f3d58
	ctx.lr = 0x83270030;
	sub_821F3D58(ctx, base);
	// 83270030: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83270034: 906ACDBC  stw r3, -0x3244(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-12868 as u32), ctx.r[3].u32 ) };
	// 83270038: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8327003C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83270040: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83270044: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83270048(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83270048 size=56
    let mut pc: u32 = 0x83270048;
    'dispatch: loop {
        match pc {
            0x83270048 => {
    //   block [0x83270048..0x83270080)
	// 83270048: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8327004C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83270050: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83270054: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83270058: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8327005C: 386B42DC  addi r3, r11, 0x42dc
	ctx.r[3].s64 = ctx.r[11].s64 + 17116;
	// 83270060: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83270064: 4AF83CF5  bl 0x821f3d58
	ctx.lr = 0x83270068;
	sub_821F3D58(ctx, base);
	// 83270068: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8327006C: 906ACDC0  stw r3, -0x3240(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-12864 as u32), ctx.r[3].u32 ) };
	// 83270070: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83270074: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83270078: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8327007C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83270080(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83270080 size=56
    let mut pc: u32 = 0x83270080;
    'dispatch: loop {
        match pc {
            0x83270080 => {
    //   block [0x83270080..0x832700B8)
	// 83270080: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83270084: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83270088: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8327008C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83270090: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83270094: 386B42E8  addi r3, r11, 0x42e8
	ctx.r[3].s64 = ctx.r[11].s64 + 17128;
	// 83270098: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8327009C: 4AF83CBD  bl 0x821f3d58
	ctx.lr = 0x832700A0;
	sub_821F3D58(ctx, base);
	// 832700A0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832700A4: 906ACDC4  stw r3, -0x323c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-12860 as u32), ctx.r[3].u32 ) };
	// 832700A8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832700AC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832700B0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832700B4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832700B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832700B8 size=56
    let mut pc: u32 = 0x832700B8;
    'dispatch: loop {
        match pc {
            0x832700B8 => {
    //   block [0x832700B8..0x832700F0)
	// 832700B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832700BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832700C0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832700C4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 832700C8: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 832700CC: 386B4300  addi r3, r11, 0x4300
	ctx.r[3].s64 = ctx.r[11].s64 + 17152;
	// 832700D0: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 832700D4: 4AF83C85  bl 0x821f3d58
	ctx.lr = 0x832700D8;
	sub_821F3D58(ctx, base);
	// 832700D8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832700DC: 906ACDC8  stw r3, -0x3238(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-12856 as u32), ctx.r[3].u32 ) };
	// 832700E0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832700E4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832700E8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832700EC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


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


