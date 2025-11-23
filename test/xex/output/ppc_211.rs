pub fn sub_83251940(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83251940 size=56
    let mut pc: u32 = 0x83251940;
    'dispatch: loop {
        match pc {
            0x83251940 => {
    //   block [0x83251940..0x83251978)
	// 83251940: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83251944: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83251948: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325194C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83251950: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83251954: 386B4394  addi r3, r11, 0x4394
	ctx.r[3].s64 = ctx.r[11].s64 + 17300;
	// 83251958: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8325195C: 4AFA23FD  bl 0x821f3d58
	ctx.lr = 0x83251960;
	sub_821F3D58(ctx, base);
	// 83251960: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83251964: 906A806C  stw r3, -0x7f94(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-32660 as u32), ctx.r[3].u32 ) };
	// 83251968: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325196C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83251970: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83251974: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83251978(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83251978 size=56
    let mut pc: u32 = 0x83251978;
    'dispatch: loop {
        match pc {
            0x83251978 => {
    //   block [0x83251978..0x832519B0)
	// 83251978: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325197C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83251980: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83251984: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83251988: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8325198C: 386B43B4  addi r3, r11, 0x43b4
	ctx.r[3].s64 = ctx.r[11].s64 + 17332;
	// 83251990: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83251994: 4AFA23C5  bl 0x821f3d58
	ctx.lr = 0x83251998;
	sub_821F3D58(ctx, base);
	// 83251998: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325199C: 906A8070  stw r3, -0x7f90(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-32656 as u32), ctx.r[3].u32 ) };
	// 832519A0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832519A4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832519A8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832519AC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832519B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832519B0 size=56
    let mut pc: u32 = 0x832519B0;
    'dispatch: loop {
        match pc {
            0x832519B0 => {
    //   block [0x832519B0..0x832519E8)
	// 832519B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832519B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832519B8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832519BC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 832519C0: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 832519C4: 386B43C8  addi r3, r11, 0x43c8
	ctx.r[3].s64 = ctx.r[11].s64 + 17352;
	// 832519C8: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 832519CC: 4AFA238D  bl 0x821f3d58
	ctx.lr = 0x832519D0;
	sub_821F3D58(ctx, base);
	// 832519D0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832519D4: 906A8074  stw r3, -0x7f8c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-32652 as u32), ctx.r[3].u32 ) };
	// 832519D8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832519DC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832519E0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832519E4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832519E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832519E8 size=56
    let mut pc: u32 = 0x832519E8;
    'dispatch: loop {
        match pc {
            0x832519E8 => {
    //   block [0x832519E8..0x83251A20)
	// 832519E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832519EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832519F0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832519F4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 832519F8: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 832519FC: 386B43D4  addi r3, r11, 0x43d4
	ctx.r[3].s64 = ctx.r[11].s64 + 17364;
	// 83251A00: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83251A04: 4AFA2355  bl 0x821f3d58
	ctx.lr = 0x83251A08;
	sub_821F3D58(ctx, base);
	// 83251A08: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83251A0C: 906A8078  stw r3, -0x7f88(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-32648 as u32), ctx.r[3].u32 ) };
	// 83251A10: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83251A14: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83251A18: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83251A1C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83251A20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83251A20 size=56
    let mut pc: u32 = 0x83251A20;
    'dispatch: loop {
        match pc {
            0x83251A20 => {
    //   block [0x83251A20..0x83251A58)
	// 83251A20: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83251A24: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83251A28: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83251A2C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83251A30: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83251A34: 386B43E0  addi r3, r11, 0x43e0
	ctx.r[3].s64 = ctx.r[11].s64 + 17376;
	// 83251A38: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83251A3C: 4AFA231D  bl 0x821f3d58
	ctx.lr = 0x83251A40;
	sub_821F3D58(ctx, base);
	// 83251A40: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83251A44: 906A807C  stw r3, -0x7f84(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-32644 as u32), ctx.r[3].u32 ) };
	// 83251A48: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83251A4C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83251A50: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83251A54: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83251A58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83251A58 size=56
    let mut pc: u32 = 0x83251A58;
    'dispatch: loop {
        match pc {
            0x83251A58 => {
    //   block [0x83251A58..0x83251A90)
	// 83251A58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83251A5C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83251A60: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83251A64: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83251A68: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83251A6C: 386B43F0  addi r3, r11, 0x43f0
	ctx.r[3].s64 = ctx.r[11].s64 + 17392;
	// 83251A70: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83251A74: 4AFA22E5  bl 0x821f3d58
	ctx.lr = 0x83251A78;
	sub_821F3D58(ctx, base);
	// 83251A78: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83251A7C: 906A8080  stw r3, -0x7f80(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-32640 as u32), ctx.r[3].u32 ) };
	// 83251A80: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83251A84: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83251A88: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83251A8C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83251A90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83251A90 size=56
    let mut pc: u32 = 0x83251A90;
    'dispatch: loop {
        match pc {
            0x83251A90 => {
    //   block [0x83251A90..0x83251AC8)
	// 83251A90: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83251A94: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83251A98: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83251A9C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83251AA0: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83251AA4: 386B4400  addi r3, r11, 0x4400
	ctx.r[3].s64 = ctx.r[11].s64 + 17408;
	// 83251AA8: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83251AAC: 4AFA22AD  bl 0x821f3d58
	ctx.lr = 0x83251AB0;
	sub_821F3D58(ctx, base);
	// 83251AB0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83251AB4: 906A8084  stw r3, -0x7f7c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-32636 as u32), ctx.r[3].u32 ) };
	// 83251AB8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83251ABC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83251AC0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83251AC4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83251AC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83251AC8 size=56
    let mut pc: u32 = 0x83251AC8;
    'dispatch: loop {
        match pc {
            0x83251AC8 => {
    //   block [0x83251AC8..0x83251B00)
	// 83251AC8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83251ACC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83251AD0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83251AD4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83251AD8: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83251ADC: 386B4418  addi r3, r11, 0x4418
	ctx.r[3].s64 = ctx.r[11].s64 + 17432;
	// 83251AE0: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83251AE4: 4AFA2275  bl 0x821f3d58
	ctx.lr = 0x83251AE8;
	sub_821F3D58(ctx, base);
	// 83251AE8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83251AEC: 906A8088  stw r3, -0x7f78(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-32632 as u32), ctx.r[3].u32 ) };
	// 83251AF0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83251AF4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83251AF8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83251AFC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83251B00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83251B00 size=56
    let mut pc: u32 = 0x83251B00;
    'dispatch: loop {
        match pc {
            0x83251B00 => {
    //   block [0x83251B00..0x83251B38)
	// 83251B00: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83251B04: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83251B08: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83251B0C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83251B10: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83251B14: 386B4424  addi r3, r11, 0x4424
	ctx.r[3].s64 = ctx.r[11].s64 + 17444;
	// 83251B18: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83251B1C: 4AFA223D  bl 0x821f3d58
	ctx.lr = 0x83251B20;
	sub_821F3D58(ctx, base);
	// 83251B20: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83251B24: 906A808C  stw r3, -0x7f74(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-32628 as u32), ctx.r[3].u32 ) };
	// 83251B28: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83251B2C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83251B30: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83251B34: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83251B38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83251B38 size=56
    let mut pc: u32 = 0x83251B38;
    'dispatch: loop {
        match pc {
            0x83251B38 => {
    //   block [0x83251B38..0x83251B70)
	// 83251B38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83251B3C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83251B40: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83251B44: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83251B48: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83251B4C: 386B4430  addi r3, r11, 0x4430
	ctx.r[3].s64 = ctx.r[11].s64 + 17456;
	// 83251B50: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83251B54: 4AFA2205  bl 0x821f3d58
	ctx.lr = 0x83251B58;
	sub_821F3D58(ctx, base);
	// 83251B58: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83251B5C: 906A8090  stw r3, -0x7f70(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-32624 as u32), ctx.r[3].u32 ) };
	// 83251B60: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83251B64: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83251B68: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83251B6C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83251B70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83251B70 size=56
    let mut pc: u32 = 0x83251B70;
    'dispatch: loop {
        match pc {
            0x83251B70 => {
    //   block [0x83251B70..0x83251BA8)
	// 83251B70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83251B74: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83251B78: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83251B7C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83251B80: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83251B84: 386B4440  addi r3, r11, 0x4440
	ctx.r[3].s64 = ctx.r[11].s64 + 17472;
	// 83251B88: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83251B8C: 4AFA21CD  bl 0x821f3d58
	ctx.lr = 0x83251B90;
	sub_821F3D58(ctx, base);
	// 83251B90: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83251B94: 906A8094  stw r3, -0x7f6c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-32620 as u32), ctx.r[3].u32 ) };
	// 83251B98: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83251B9C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83251BA0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83251BA4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83251BA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83251BA8 size=64
    let mut pc: u32 = 0x83251BA8;
    'dispatch: loop {
        match pc {
            0x83251BA8 => {
    //   block [0x83251BA8..0x83251BE8)
	// 83251BA8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83251BAC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83251BB0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83251BB4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83251BB8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83251BBC: 388B1AD4  addi r4, r11, 0x1ad4
	ctx.r[4].s64 = ctx.r[11].s64 + 6868;
	// 83251BC0: 386A8098  addi r3, r10, -0x7f68
	ctx.r[3].s64 = ctx.r[10].s64 + -32616;
	// 83251BC4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83251BC8: 4AFDB309  bl 0x8222ced0
	ctx.lr = 0x83251BCC;
	sub_8222CED0(ctx, base);
	// 83251BCC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83251BD0: 38699788  addi r3, r9, -0x6878
	ctx.r[3].s64 = ctx.r[9].s64 + -26744;
	// 83251BD4: 4BA5834D  bl 0x82ca9f20
	ctx.lr = 0x83251BD8;
	sub_82CA9F20(ctx, base);
	// 83251BD8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83251BDC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83251BE0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83251BE4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83251BE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83251BE8 size=64
    let mut pc: u32 = 0x83251BE8;
    'dispatch: loop {
        match pc {
            0x83251BE8 => {
    //   block [0x83251BE8..0x83251C28)
	// 83251BE8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83251BEC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83251BF0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83251BF4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83251BF8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83251BFC: 388B1ADC  addi r4, r11, 0x1adc
	ctx.r[4].s64 = ctx.r[11].s64 + 6876;
	// 83251C00: 386A809C  addi r3, r10, -0x7f64
	ctx.r[3].s64 = ctx.r[10].s64 + -32612;
	// 83251C04: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83251C08: 4AFDB2C9  bl 0x8222ced0
	ctx.lr = 0x83251C0C;
	sub_8222CED0(ctx, base);
	// 83251C0C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83251C10: 38699798  addi r3, r9, -0x6868
	ctx.r[3].s64 = ctx.r[9].s64 + -26728;
	// 83251C14: 4BA5830D  bl 0x82ca9f20
	ctx.lr = 0x83251C18;
	sub_82CA9F20(ctx, base);
	// 83251C18: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83251C1C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83251C20: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83251C24: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83251C28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83251C28 size=64
    let mut pc: u32 = 0x83251C28;
    'dispatch: loop {
        match pc {
            0x83251C28 => {
    //   block [0x83251C28..0x83251C68)
	// 83251C28: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83251C2C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83251C30: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83251C34: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83251C38: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83251C3C: 388B1AF0  addi r4, r11, 0x1af0
	ctx.r[4].s64 = ctx.r[11].s64 + 6896;
	// 83251C40: 386A80A0  addi r3, r10, -0x7f60
	ctx.r[3].s64 = ctx.r[10].s64 + -32608;
	// 83251C44: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83251C48: 4AFDB289  bl 0x8222ced0
	ctx.lr = 0x83251C4C;
	sub_8222CED0(ctx, base);
	// 83251C4C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83251C50: 386997A8  addi r3, r9, -0x6858
	ctx.r[3].s64 = ctx.r[9].s64 + -26712;
	// 83251C54: 4BA582CD  bl 0x82ca9f20
	ctx.lr = 0x83251C58;
	sub_82CA9F20(ctx, base);
	// 83251C58: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83251C5C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83251C60: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83251C64: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83251C68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83251C68 size=64
    let mut pc: u32 = 0x83251C68;
    'dispatch: loop {
        match pc {
            0x83251C68 => {
    //   block [0x83251C68..0x83251CA8)
	// 83251C68: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83251C6C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83251C70: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83251C74: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83251C78: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83251C7C: 388B052C  addi r4, r11, 0x52c
	ctx.r[4].s64 = ctx.r[11].s64 + 1324;
	// 83251C80: 386A80A4  addi r3, r10, -0x7f5c
	ctx.r[3].s64 = ctx.r[10].s64 + -32604;
	// 83251C84: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83251C88: 4AFDB249  bl 0x8222ced0
	ctx.lr = 0x83251C8C;
	sub_8222CED0(ctx, base);
	// 83251C8C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83251C90: 386997B8  addi r3, r9, -0x6848
	ctx.r[3].s64 = ctx.r[9].s64 + -26696;
	// 83251C94: 4BA5828D  bl 0x82ca9f20
	ctx.lr = 0x83251C98;
	sub_82CA9F20(ctx, base);
	// 83251C98: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83251C9C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83251CA0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83251CA4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83251CA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83251CA8 size=64
    let mut pc: u32 = 0x83251CA8;
    'dispatch: loop {
        match pc {
            0x83251CA8 => {
    //   block [0x83251CA8..0x83251CE8)
	// 83251CA8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83251CAC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83251CB0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83251CB4: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 83251CB8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83251CBC: 388B2F98  addi r4, r11, 0x2f98
	ctx.r[4].s64 = ctx.r[11].s64 + 12184;
	// 83251CC0: 386A80A8  addi r3, r10, -0x7f58
	ctx.r[3].s64 = ctx.r[10].s64 + -32600;
	// 83251CC4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83251CC8: 4AFDB209  bl 0x8222ced0
	ctx.lr = 0x83251CCC;
	sub_8222CED0(ctx, base);
	// 83251CCC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83251CD0: 386997C8  addi r3, r9, -0x6838
	ctx.r[3].s64 = ctx.r[9].s64 + -26680;
	// 83251CD4: 4BA5824D  bl 0x82ca9f20
	ctx.lr = 0x83251CD8;
	sub_82CA9F20(ctx, base);
	// 83251CD8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83251CDC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83251CE0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83251CE4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83251CE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83251CE8 size=64
    let mut pc: u32 = 0x83251CE8;
    'dispatch: loop {
        match pc {
            0x83251CE8 => {
    //   block [0x83251CE8..0x83251D28)
	// 83251CE8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83251CEC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83251CF0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83251CF4: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 83251CF8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83251CFC: 388B2FBC  addi r4, r11, 0x2fbc
	ctx.r[4].s64 = ctx.r[11].s64 + 12220;
	// 83251D00: 386A80AC  addi r3, r10, -0x7f54
	ctx.r[3].s64 = ctx.r[10].s64 + -32596;
	// 83251D04: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83251D08: 4AFDB1C9  bl 0x8222ced0
	ctx.lr = 0x83251D0C;
	sub_8222CED0(ctx, base);
	// 83251D0C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83251D10: 386997D8  addi r3, r9, -0x6828
	ctx.r[3].s64 = ctx.r[9].s64 + -26664;
	// 83251D14: 4BA5820D  bl 0x82ca9f20
	ctx.lr = 0x83251D18;
	sub_82CA9F20(ctx, base);
	// 83251D18: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83251D1C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83251D20: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83251D24: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83251D28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83251D28 size=64
    let mut pc: u32 = 0x83251D28;
    'dispatch: loop {
        match pc {
            0x83251D28 => {
    //   block [0x83251D28..0x83251D68)
	// 83251D28: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83251D2C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83251D30: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83251D34: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 83251D38: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83251D3C: 388B2FE0  addi r4, r11, 0x2fe0
	ctx.r[4].s64 = ctx.r[11].s64 + 12256;
	// 83251D40: 386A80B0  addi r3, r10, -0x7f50
	ctx.r[3].s64 = ctx.r[10].s64 + -32592;
	// 83251D44: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83251D48: 4AFDB189  bl 0x8222ced0
	ctx.lr = 0x83251D4C;
	sub_8222CED0(ctx, base);
	// 83251D4C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83251D50: 386997E8  addi r3, r9, -0x6818
	ctx.r[3].s64 = ctx.r[9].s64 + -26648;
	// 83251D54: 4BA581CD  bl 0x82ca9f20
	ctx.lr = 0x83251D58;
	sub_82CA9F20(ctx, base);
	// 83251D58: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83251D5C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83251D60: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83251D64: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83251D68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83251D68 size=64
    let mut pc: u32 = 0x83251D68;
    'dispatch: loop {
        match pc {
            0x83251D68 => {
    //   block [0x83251D68..0x83251DA8)
	// 83251D68: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83251D6C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83251D70: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83251D74: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 83251D78: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83251D7C: 388B3000  addi r4, r11, 0x3000
	ctx.r[4].s64 = ctx.r[11].s64 + 12288;
	// 83251D80: 386A80B4  addi r3, r10, -0x7f4c
	ctx.r[3].s64 = ctx.r[10].s64 + -32588;
	// 83251D84: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83251D88: 4AFDB149  bl 0x8222ced0
	ctx.lr = 0x83251D8C;
	sub_8222CED0(ctx, base);
	// 83251D8C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83251D90: 386997F8  addi r3, r9, -0x6808
	ctx.r[3].s64 = ctx.r[9].s64 + -26632;
	// 83251D94: 4BA5818D  bl 0x82ca9f20
	ctx.lr = 0x83251D98;
	sub_82CA9F20(ctx, base);
	// 83251D98: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83251D9C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83251DA0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83251DA4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83251DA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83251DA8 size=64
    let mut pc: u32 = 0x83251DA8;
    'dispatch: loop {
        match pc {
            0x83251DA8 => {
    //   block [0x83251DA8..0x83251DE8)
	// 83251DA8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83251DAC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83251DB0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83251DB4: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 83251DB8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83251DBC: 388B3028  addi r4, r11, 0x3028
	ctx.r[4].s64 = ctx.r[11].s64 + 12328;
	// 83251DC0: 386A80B8  addi r3, r10, -0x7f48
	ctx.r[3].s64 = ctx.r[10].s64 + -32584;
	// 83251DC4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83251DC8: 4AFDB109  bl 0x8222ced0
	ctx.lr = 0x83251DCC;
	sub_8222CED0(ctx, base);
	// 83251DCC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83251DD0: 38699808  addi r3, r9, -0x67f8
	ctx.r[3].s64 = ctx.r[9].s64 + -26616;
	// 83251DD4: 4BA5814D  bl 0x82ca9f20
	ctx.lr = 0x83251DD8;
	sub_82CA9F20(ctx, base);
	// 83251DD8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83251DDC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83251DE0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83251DE4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83251DE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83251DE8 size=64
    let mut pc: u32 = 0x83251DE8;
    'dispatch: loop {
        match pc {
            0x83251DE8 => {
    //   block [0x83251DE8..0x83251E28)
	// 83251DE8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83251DEC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83251DF0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83251DF4: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 83251DF8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83251DFC: 388B3050  addi r4, r11, 0x3050
	ctx.r[4].s64 = ctx.r[11].s64 + 12368;
	// 83251E00: 386A80BC  addi r3, r10, -0x7f44
	ctx.r[3].s64 = ctx.r[10].s64 + -32580;
	// 83251E04: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83251E08: 4AFDB0C9  bl 0x8222ced0
	ctx.lr = 0x83251E0C;
	sub_8222CED0(ctx, base);
	// 83251E0C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83251E10: 38699818  addi r3, r9, -0x67e8
	ctx.r[3].s64 = ctx.r[9].s64 + -26600;
	// 83251E14: 4BA5810D  bl 0x82ca9f20
	ctx.lr = 0x83251E18;
	sub_82CA9F20(ctx, base);
	// 83251E18: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83251E1C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83251E20: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83251E24: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83251E28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83251E28 size=64
    let mut pc: u32 = 0x83251E28;
    'dispatch: loop {
        match pc {
            0x83251E28 => {
    //   block [0x83251E28..0x83251E68)
	// 83251E28: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83251E2C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83251E30: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83251E34: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 83251E38: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83251E3C: 388B3078  addi r4, r11, 0x3078
	ctx.r[4].s64 = ctx.r[11].s64 + 12408;
	// 83251E40: 386A80C0  addi r3, r10, -0x7f40
	ctx.r[3].s64 = ctx.r[10].s64 + -32576;
	// 83251E44: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83251E48: 4AFDB089  bl 0x8222ced0
	ctx.lr = 0x83251E4C;
	sub_8222CED0(ctx, base);
	// 83251E4C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83251E50: 38699828  addi r3, r9, -0x67d8
	ctx.r[3].s64 = ctx.r[9].s64 + -26584;
	// 83251E54: 4BA580CD  bl 0x82ca9f20
	ctx.lr = 0x83251E58;
	sub_82CA9F20(ctx, base);
	// 83251E58: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83251E5C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83251E60: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83251E64: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83251E68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83251E68 size=64
    let mut pc: u32 = 0x83251E68;
    'dispatch: loop {
        match pc {
            0x83251E68 => {
    //   block [0x83251E68..0x83251EA8)
	// 83251E68: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83251E6C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83251E70: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83251E74: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 83251E78: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83251E7C: 388B3098  addi r4, r11, 0x3098
	ctx.r[4].s64 = ctx.r[11].s64 + 12440;
	// 83251E80: 386A80C4  addi r3, r10, -0x7f3c
	ctx.r[3].s64 = ctx.r[10].s64 + -32572;
	// 83251E84: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83251E88: 4AFDB049  bl 0x8222ced0
	ctx.lr = 0x83251E8C;
	sub_8222CED0(ctx, base);
	// 83251E8C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83251E90: 38699838  addi r3, r9, -0x67c8
	ctx.r[3].s64 = ctx.r[9].s64 + -26568;
	// 83251E94: 4BA5808D  bl 0x82ca9f20
	ctx.lr = 0x83251E98;
	sub_82CA9F20(ctx, base);
	// 83251E98: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83251E9C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83251EA0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83251EA4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83251EA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83251EA8 size=64
    let mut pc: u32 = 0x83251EA8;
    'dispatch: loop {
        match pc {
            0x83251EA8 => {
    //   block [0x83251EA8..0x83251EE8)
	// 83251EA8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83251EAC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83251EB0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83251EB4: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 83251EB8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83251EBC: 388B30BC  addi r4, r11, 0x30bc
	ctx.r[4].s64 = ctx.r[11].s64 + 12476;
	// 83251EC0: 386A80C8  addi r3, r10, -0x7f38
	ctx.r[3].s64 = ctx.r[10].s64 + -32568;
	// 83251EC4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83251EC8: 4AFDB009  bl 0x8222ced0
	ctx.lr = 0x83251ECC;
	sub_8222CED0(ctx, base);
	// 83251ECC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83251ED0: 38699848  addi r3, r9, -0x67b8
	ctx.r[3].s64 = ctx.r[9].s64 + -26552;
	// 83251ED4: 4BA5804D  bl 0x82ca9f20
	ctx.lr = 0x83251ED8;
	sub_82CA9F20(ctx, base);
	// 83251ED8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83251EDC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83251EE0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83251EE4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83251EE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83251EE8 size=64
    let mut pc: u32 = 0x83251EE8;
    'dispatch: loop {
        match pc {
            0x83251EE8 => {
    //   block [0x83251EE8..0x83251F28)
	// 83251EE8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83251EEC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83251EF0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83251EF4: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 83251EF8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83251EFC: 388B30D8  addi r4, r11, 0x30d8
	ctx.r[4].s64 = ctx.r[11].s64 + 12504;
	// 83251F00: 386A80CC  addi r3, r10, -0x7f34
	ctx.r[3].s64 = ctx.r[10].s64 + -32564;
	// 83251F04: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83251F08: 4AFDAFC9  bl 0x8222ced0
	ctx.lr = 0x83251F0C;
	sub_8222CED0(ctx, base);
	// 83251F0C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83251F10: 38699858  addi r3, r9, -0x67a8
	ctx.r[3].s64 = ctx.r[9].s64 + -26536;
	// 83251F14: 4BA5800D  bl 0x82ca9f20
	ctx.lr = 0x83251F18;
	sub_82CA9F20(ctx, base);
	// 83251F18: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83251F1C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83251F20: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83251F24: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83251F28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83251F28 size=64
    let mut pc: u32 = 0x83251F28;
    'dispatch: loop {
        match pc {
            0x83251F28 => {
    //   block [0x83251F28..0x83251F68)
	// 83251F28: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83251F2C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83251F30: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83251F34: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 83251F38: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83251F3C: 388B30F8  addi r4, r11, 0x30f8
	ctx.r[4].s64 = ctx.r[11].s64 + 12536;
	// 83251F40: 386A80D0  addi r3, r10, -0x7f30
	ctx.r[3].s64 = ctx.r[10].s64 + -32560;
	// 83251F44: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83251F48: 4AFDAF89  bl 0x8222ced0
	ctx.lr = 0x83251F4C;
	sub_8222CED0(ctx, base);
	// 83251F4C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83251F50: 38699868  addi r3, r9, -0x6798
	ctx.r[3].s64 = ctx.r[9].s64 + -26520;
	// 83251F54: 4BA57FCD  bl 0x82ca9f20
	ctx.lr = 0x83251F58;
	sub_82CA9F20(ctx, base);
	// 83251F58: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83251F5C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83251F60: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83251F64: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83251F68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83251F68 size=64
    let mut pc: u32 = 0x83251F68;
    'dispatch: loop {
        match pc {
            0x83251F68 => {
    //   block [0x83251F68..0x83251FA8)
	// 83251F68: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83251F6C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83251F70: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83251F74: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 83251F78: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83251F7C: 388B3108  addi r4, r11, 0x3108
	ctx.r[4].s64 = ctx.r[11].s64 + 12552;
	// 83251F80: 386A80D4  addi r3, r10, -0x7f2c
	ctx.r[3].s64 = ctx.r[10].s64 + -32556;
	// 83251F84: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83251F88: 4AFDAF49  bl 0x8222ced0
	ctx.lr = 0x83251F8C;
	sub_8222CED0(ctx, base);
	// 83251F8C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83251F90: 38699878  addi r3, r9, -0x6788
	ctx.r[3].s64 = ctx.r[9].s64 + -26504;
	// 83251F94: 4BA57F8D  bl 0x82ca9f20
	ctx.lr = 0x83251F98;
	sub_82CA9F20(ctx, base);
	// 83251F98: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83251F9C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83251FA0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83251FA4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83251FA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83251FA8 size=64
    let mut pc: u32 = 0x83251FA8;
    'dispatch: loop {
        match pc {
            0x83251FA8 => {
    //   block [0x83251FA8..0x83251FE8)
	// 83251FA8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83251FAC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83251FB0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83251FB4: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 83251FB8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83251FBC: 388B3118  addi r4, r11, 0x3118
	ctx.r[4].s64 = ctx.r[11].s64 + 12568;
	// 83251FC0: 386A80D8  addi r3, r10, -0x7f28
	ctx.r[3].s64 = ctx.r[10].s64 + -32552;
	// 83251FC4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83251FC8: 4AFDAF09  bl 0x8222ced0
	ctx.lr = 0x83251FCC;
	sub_8222CED0(ctx, base);
	// 83251FCC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83251FD0: 38699888  addi r3, r9, -0x6778
	ctx.r[3].s64 = ctx.r[9].s64 + -26488;
	// 83251FD4: 4BA57F4D  bl 0x82ca9f20
	ctx.lr = 0x83251FD8;
	sub_82CA9F20(ctx, base);
	// 83251FD8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83251FDC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83251FE0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83251FE4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83251FE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83251FE8 size=64
    let mut pc: u32 = 0x83251FE8;
    'dispatch: loop {
        match pc {
            0x83251FE8 => {
    //   block [0x83251FE8..0x83252028)
	// 83251FE8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83251FEC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83251FF0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83251FF4: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 83251FF8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83251FFC: 388B3124  addi r4, r11, 0x3124
	ctx.r[4].s64 = ctx.r[11].s64 + 12580;
	// 83252000: 386A80DC  addi r3, r10, -0x7f24
	ctx.r[3].s64 = ctx.r[10].s64 + -32548;
	// 83252004: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83252008: 4AFDAEC9  bl 0x8222ced0
	ctx.lr = 0x8325200C;
	sub_8222CED0(ctx, base);
	// 8325200C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83252010: 38699898  addi r3, r9, -0x6768
	ctx.r[3].s64 = ctx.r[9].s64 + -26472;
	// 83252014: 4BA57F0D  bl 0x82ca9f20
	ctx.lr = 0x83252018;
	sub_82CA9F20(ctx, base);
	// 83252018: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325201C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83252020: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83252024: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83252028(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83252028 size=64
    let mut pc: u32 = 0x83252028;
    'dispatch: loop {
        match pc {
            0x83252028 => {
    //   block [0x83252028..0x83252068)
	// 83252028: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325202C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83252030: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83252034: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 83252038: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325203C: 388B3138  addi r4, r11, 0x3138
	ctx.r[4].s64 = ctx.r[11].s64 + 12600;
	// 83252040: 386A80E0  addi r3, r10, -0x7f20
	ctx.r[3].s64 = ctx.r[10].s64 + -32544;
	// 83252044: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83252048: 4AFDAE89  bl 0x8222ced0
	ctx.lr = 0x8325204C;
	sub_8222CED0(ctx, base);
	// 8325204C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83252050: 386998A8  addi r3, r9, -0x6758
	ctx.r[3].s64 = ctx.r[9].s64 + -26456;
	// 83252054: 4BA57ECD  bl 0x82ca9f20
	ctx.lr = 0x83252058;
	sub_82CA9F20(ctx, base);
	// 83252058: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325205C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83252060: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83252064: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83252068(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83252068 size=64
    let mut pc: u32 = 0x83252068;
    'dispatch: loop {
        match pc {
            0x83252068 => {
    //   block [0x83252068..0x832520A8)
	// 83252068: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325206C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83252070: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83252074: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 83252078: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325207C: 388B3154  addi r4, r11, 0x3154
	ctx.r[4].s64 = ctx.r[11].s64 + 12628;
	// 83252080: 386A80E4  addi r3, r10, -0x7f1c
	ctx.r[3].s64 = ctx.r[10].s64 + -32540;
	// 83252084: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83252088: 4AFDAE49  bl 0x8222ced0
	ctx.lr = 0x8325208C;
	sub_8222CED0(ctx, base);
	// 8325208C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83252090: 386998B8  addi r3, r9, -0x6748
	ctx.r[3].s64 = ctx.r[9].s64 + -26440;
	// 83252094: 4BA57E8D  bl 0x82ca9f20
	ctx.lr = 0x83252098;
	sub_82CA9F20(ctx, base);
	// 83252098: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325209C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832520A0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832520A4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832520A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832520A8 size=64
    let mut pc: u32 = 0x832520A8;
    'dispatch: loop {
        match pc {
            0x832520A8 => {
    //   block [0x832520A8..0x832520E8)
	// 832520A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832520AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832520B0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832520B4: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 832520B8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832520BC: 388B317C  addi r4, r11, 0x317c
	ctx.r[4].s64 = ctx.r[11].s64 + 12668;
	// 832520C0: 386A80E8  addi r3, r10, -0x7f18
	ctx.r[3].s64 = ctx.r[10].s64 + -32536;
	// 832520C4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832520C8: 4AFDAE09  bl 0x8222ced0
	ctx.lr = 0x832520CC;
	sub_8222CED0(ctx, base);
	// 832520CC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 832520D0: 386998C8  addi r3, r9, -0x6738
	ctx.r[3].s64 = ctx.r[9].s64 + -26424;
	// 832520D4: 4BA57E4D  bl 0x82ca9f20
	ctx.lr = 0x832520D8;
	sub_82CA9F20(ctx, base);
	// 832520D8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832520DC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832520E0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832520E4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832520E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832520E8 size=64
    let mut pc: u32 = 0x832520E8;
    'dispatch: loop {
        match pc {
            0x832520E8 => {
    //   block [0x832520E8..0x83252128)
	// 832520E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832520EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832520F0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832520F4: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 832520F8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832520FC: 388B3198  addi r4, r11, 0x3198
	ctx.r[4].s64 = ctx.r[11].s64 + 12696;
	// 83252100: 386A80EC  addi r3, r10, -0x7f14
	ctx.r[3].s64 = ctx.r[10].s64 + -32532;
	// 83252104: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83252108: 4AFDADC9  bl 0x8222ced0
	ctx.lr = 0x8325210C;
	sub_8222CED0(ctx, base);
	// 8325210C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83252110: 386998D8  addi r3, r9, -0x6728
	ctx.r[3].s64 = ctx.r[9].s64 + -26408;
	// 83252114: 4BA57E0D  bl 0x82ca9f20
	ctx.lr = 0x83252118;
	sub_82CA9F20(ctx, base);
	// 83252118: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325211C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83252120: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83252124: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83252128(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83252128 size=64
    let mut pc: u32 = 0x83252128;
    'dispatch: loop {
        match pc {
            0x83252128 => {
    //   block [0x83252128..0x83252168)
	// 83252128: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325212C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83252130: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83252134: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 83252138: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325213C: 388B31B8  addi r4, r11, 0x31b8
	ctx.r[4].s64 = ctx.r[11].s64 + 12728;
	// 83252140: 386A80F0  addi r3, r10, -0x7f10
	ctx.r[3].s64 = ctx.r[10].s64 + -32528;
	// 83252144: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83252148: 4AFDAD89  bl 0x8222ced0
	ctx.lr = 0x8325214C;
	sub_8222CED0(ctx, base);
	// 8325214C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83252150: 386998E8  addi r3, r9, -0x6718
	ctx.r[3].s64 = ctx.r[9].s64 + -26392;
	// 83252154: 4BA57DCD  bl 0x82ca9f20
	ctx.lr = 0x83252158;
	sub_82CA9F20(ctx, base);
	// 83252158: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325215C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83252160: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83252164: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83252168(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83252168 size=64
    let mut pc: u32 = 0x83252168;
    'dispatch: loop {
        match pc {
            0x83252168 => {
    //   block [0x83252168..0x832521A8)
	// 83252168: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325216C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83252170: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83252174: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 83252178: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325217C: 388B31DC  addi r4, r11, 0x31dc
	ctx.r[4].s64 = ctx.r[11].s64 + 12764;
	// 83252180: 386A80F4  addi r3, r10, -0x7f0c
	ctx.r[3].s64 = ctx.r[10].s64 + -32524;
	// 83252184: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83252188: 4AFDAD49  bl 0x8222ced0
	ctx.lr = 0x8325218C;
	sub_8222CED0(ctx, base);
	// 8325218C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83252190: 386998F8  addi r3, r9, -0x6708
	ctx.r[3].s64 = ctx.r[9].s64 + -26376;
	// 83252194: 4BA57D8D  bl 0x82ca9f20
	ctx.lr = 0x83252198;
	sub_82CA9F20(ctx, base);
	// 83252198: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325219C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832521A0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832521A4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832521A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832521A8 size=64
    let mut pc: u32 = 0x832521A8;
    'dispatch: loop {
        match pc {
            0x832521A8 => {
    //   block [0x832521A8..0x832521E8)
	// 832521A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832521AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832521B0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832521B4: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 832521B8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832521BC: 388B3204  addi r4, r11, 0x3204
	ctx.r[4].s64 = ctx.r[11].s64 + 12804;
	// 832521C0: 386A80F8  addi r3, r10, -0x7f08
	ctx.r[3].s64 = ctx.r[10].s64 + -32520;
	// 832521C4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832521C8: 4AFDAD09  bl 0x8222ced0
	ctx.lr = 0x832521CC;
	sub_8222CED0(ctx, base);
	// 832521CC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 832521D0: 38699908  addi r3, r9, -0x66f8
	ctx.r[3].s64 = ctx.r[9].s64 + -26360;
	// 832521D4: 4BA57D4D  bl 0x82ca9f20
	ctx.lr = 0x832521D8;
	sub_82CA9F20(ctx, base);
	// 832521D8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832521DC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832521E0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832521E4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832521E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832521E8 size=64
    let mut pc: u32 = 0x832521E8;
    'dispatch: loop {
        match pc {
            0x832521E8 => {
    //   block [0x832521E8..0x83252228)
	// 832521E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832521EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832521F0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832521F4: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 832521F8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832521FC: 388B3238  addi r4, r11, 0x3238
	ctx.r[4].s64 = ctx.r[11].s64 + 12856;
	// 83252200: 386A80FC  addi r3, r10, -0x7f04
	ctx.r[3].s64 = ctx.r[10].s64 + -32516;
	// 83252204: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83252208: 4AFDACC9  bl 0x8222ced0
	ctx.lr = 0x8325220C;
	sub_8222CED0(ctx, base);
	// 8325220C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83252210: 38699918  addi r3, r9, -0x66e8
	ctx.r[3].s64 = ctx.r[9].s64 + -26344;
	// 83252214: 4BA57D0D  bl 0x82ca9f20
	ctx.lr = 0x83252218;
	sub_82CA9F20(ctx, base);
	// 83252218: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325221C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83252220: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83252224: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83252228(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83252228 size=64
    let mut pc: u32 = 0x83252228;
    'dispatch: loop {
        match pc {
            0x83252228 => {
    //   block [0x83252228..0x83252268)
	// 83252228: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325222C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83252230: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83252234: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 83252238: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325223C: 388B3274  addi r4, r11, 0x3274
	ctx.r[4].s64 = ctx.r[11].s64 + 12916;
	// 83252240: 386A8100  addi r3, r10, -0x7f00
	ctx.r[3].s64 = ctx.r[10].s64 + -32512;
	// 83252244: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83252248: 4AFDAC89  bl 0x8222ced0
	ctx.lr = 0x8325224C;
	sub_8222CED0(ctx, base);
	// 8325224C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83252250: 38699928  addi r3, r9, -0x66d8
	ctx.r[3].s64 = ctx.r[9].s64 + -26328;
	// 83252254: 4BA57CCD  bl 0x82ca9f20
	ctx.lr = 0x83252258;
	sub_82CA9F20(ctx, base);
	// 83252258: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325225C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83252260: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83252264: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83252268(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83252268 size=64
    let mut pc: u32 = 0x83252268;
    'dispatch: loop {
        match pc {
            0x83252268 => {
    //   block [0x83252268..0x832522A8)
	// 83252268: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325226C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83252270: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83252274: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 83252278: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325227C: 388B32A8  addi r4, r11, 0x32a8
	ctx.r[4].s64 = ctx.r[11].s64 + 12968;
	// 83252280: 386A8104  addi r3, r10, -0x7efc
	ctx.r[3].s64 = ctx.r[10].s64 + -32508;
	// 83252284: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83252288: 4AFDAC49  bl 0x8222ced0
	ctx.lr = 0x8325228C;
	sub_8222CED0(ctx, base);
	// 8325228C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83252290: 38699938  addi r3, r9, -0x66c8
	ctx.r[3].s64 = ctx.r[9].s64 + -26312;
	// 83252294: 4BA57C8D  bl 0x82ca9f20
	ctx.lr = 0x83252298;
	sub_82CA9F20(ctx, base);
	// 83252298: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325229C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832522A0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832522A4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832522A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832522A8 size=64
    let mut pc: u32 = 0x832522A8;
    'dispatch: loop {
        match pc {
            0x832522A8 => {
    //   block [0x832522A8..0x832522E8)
	// 832522A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832522AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832522B0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832522B4: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 832522B8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832522BC: 388B32E4  addi r4, r11, 0x32e4
	ctx.r[4].s64 = ctx.r[11].s64 + 13028;
	// 832522C0: 386A8108  addi r3, r10, -0x7ef8
	ctx.r[3].s64 = ctx.r[10].s64 + -32504;
	// 832522C4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832522C8: 4AFDAC09  bl 0x8222ced0
	ctx.lr = 0x832522CC;
	sub_8222CED0(ctx, base);
	// 832522CC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 832522D0: 38699948  addi r3, r9, -0x66b8
	ctx.r[3].s64 = ctx.r[9].s64 + -26296;
	// 832522D4: 4BA57C4D  bl 0x82ca9f20
	ctx.lr = 0x832522D8;
	sub_82CA9F20(ctx, base);
	// 832522D8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832522DC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832522E0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832522E4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832522E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832522E8 size=64
    let mut pc: u32 = 0x832522E8;
    'dispatch: loop {
        match pc {
            0x832522E8 => {
    //   block [0x832522E8..0x83252328)
	// 832522E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832522EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832522F0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832522F4: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 832522F8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832522FC: 388B330C  addi r4, r11, 0x330c
	ctx.r[4].s64 = ctx.r[11].s64 + 13068;
	// 83252300: 386A810C  addi r3, r10, -0x7ef4
	ctx.r[3].s64 = ctx.r[10].s64 + -32500;
	// 83252304: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83252308: 4AFDABC9  bl 0x8222ced0
	ctx.lr = 0x8325230C;
	sub_8222CED0(ctx, base);
	// 8325230C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83252310: 38699958  addi r3, r9, -0x66a8
	ctx.r[3].s64 = ctx.r[9].s64 + -26280;
	// 83252314: 4BA57C0D  bl 0x82ca9f20
	ctx.lr = 0x83252318;
	sub_82CA9F20(ctx, base);
	// 83252318: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325231C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83252320: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83252324: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83252328(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83252328 size=64
    let mut pc: u32 = 0x83252328;
    'dispatch: loop {
        match pc {
            0x83252328 => {
    //   block [0x83252328..0x83252368)
	// 83252328: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325232C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83252330: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83252334: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 83252338: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325233C: 388B3334  addi r4, r11, 0x3334
	ctx.r[4].s64 = ctx.r[11].s64 + 13108;
	// 83252340: 386A8110  addi r3, r10, -0x7ef0
	ctx.r[3].s64 = ctx.r[10].s64 + -32496;
	// 83252344: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83252348: 4AFDAB89  bl 0x8222ced0
	ctx.lr = 0x8325234C;
	sub_8222CED0(ctx, base);
	// 8325234C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83252350: 38699968  addi r3, r9, -0x6698
	ctx.r[3].s64 = ctx.r[9].s64 + -26264;
	// 83252354: 4BA57BCD  bl 0x82ca9f20
	ctx.lr = 0x83252358;
	sub_82CA9F20(ctx, base);
	// 83252358: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325235C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83252360: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83252364: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83252368(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83252368 size=64
    let mut pc: u32 = 0x83252368;
    'dispatch: loop {
        match pc {
            0x83252368 => {
    //   block [0x83252368..0x832523A8)
	// 83252368: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325236C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83252370: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83252374: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 83252378: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325237C: 388B335C  addi r4, r11, 0x335c
	ctx.r[4].s64 = ctx.r[11].s64 + 13148;
	// 83252380: 386A8114  addi r3, r10, -0x7eec
	ctx.r[3].s64 = ctx.r[10].s64 + -32492;
	// 83252384: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83252388: 4AFDAB49  bl 0x8222ced0
	ctx.lr = 0x8325238C;
	sub_8222CED0(ctx, base);
	// 8325238C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83252390: 38699978  addi r3, r9, -0x6688
	ctx.r[3].s64 = ctx.r[9].s64 + -26248;
	// 83252394: 4BA57B8D  bl 0x82ca9f20
	ctx.lr = 0x83252398;
	sub_82CA9F20(ctx, base);
	// 83252398: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325239C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832523A0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832523A4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832523A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832523A8 size=192
    let mut pc: u32 = 0x832523A8;
    'dispatch: loop {
        match pc {
            0x832523A8 => {
    //   block [0x832523A8..0x83252400)
	// 832523A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832523AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832523B0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 832523B4: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832523B8: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 832523BC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832523C0: 388B3390  addi r4, r11, 0x3390
	ctx.r[4].s64 = ctx.r[11].s64 + 13200;
	// 832523C4: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 832523C8: 4AFDAB09  bl 0x8222ced0
	ctx.lr = 0x832523CC;
	sub_8222CED0(ctx, base);
	// 832523CC: 38810054  addi r4, r1, 0x54
	ctx.r[4].s64 = ctx.r[1].s64 + 84;
	// 832523D0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 832523D4: 4AF9C765  bl 0x821eeb38
	ctx.lr = 0x832523D8;
	sub_821EEB38(ctx, base);
	// 832523D8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 832523DC: 4B9B1415  bl 0x82c037f0
	ctx.lr = 0x832523E0;
	sub_82C037F0(ctx, base);
	// 832523E0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832523E4: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 832523E8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 832523EC: 916A8118  stw r11, -0x7ee8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-32488 as u32), ctx.r[11].u32 ) };
	// 832523F0: 4AF74379  bl 0x821c6768
	ctx.lr = 0x832523F4;
	sub_821C6768(ctx, base);
	// 832523F4: 3D208349  lis r9, -0x7cb7
	ctx.r[9].s64 = -2092367872;
	// 832523F8: 3BE97088  addi r31, r9, 0x7088
	ctx.r[31].s64 = ctx.r[9].s64 + 28808;
	// 832523FC: 7FE6FB78  mr r6, r31
	ctx.r[6].u64 = ctx.r[31].u64;
	pc = 0x83252400; continue 'dispatch;
            }
            0x83252400 => {
    //   block [0x83252400..0x8325242C)
	// 83252400: 7CE000A6  mfmsr r7
	ctx.r[7].u64 = ctx.msr;
	// 83252404: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 83252408: 7D003028  lwarx r8, 0, r6
	// lwarx
	let ea = ctx.r[6].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[8].u64 = ctx.reserved.u32 as u64;
	// 8325240C: 3908FFFF  addi r8, r8, -1
	ctx.r[8].s64 = ctx.r[8].s64 + -1;
	// 83252410: 7D00312D  stwcx. r8, 0, r6
	// stwcx.
	let addr = ctx.r[6].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[8].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 83252414: 7CE10164  mtmsrd r7, 1
	ctx.msr = (ctx.r[7].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 83252418: 4082FFE8  bne 0x83252400
	if !ctx.cr[0].eq {
	pc = 0x83252400; continue 'dispatch;
	}
	// 8325241C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 83252420: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 83252424: 90A10050  stw r5, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[5].u32 ) };
	// 83252428: 4AF74341  bl 0x821c6768
	ctx.lr = 0x8325242C;
	sub_821C6768(ctx, base);
	pc = 0x8325242C; continue 'dispatch;
            }
            0x8325242C => {
    //   block [0x8325242C..0x83252468)
	// 8325242C: 7C6000A6  mfmsr r3
	ctx.r[3].u64 = ctx.msr;
	// 83252430: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 83252434: 7C80F828  lwarx r4, 0, r31
	// lwarx
	let ea = ctx.r[31].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[4].u64 = ctx.reserved.u32 as u64;
	// 83252438: 3884FFFF  addi r4, r4, -1
	ctx.r[4].s64 = ctx.r[4].s64 + -1;
	// 8325243C: 7C80F92D  stwcx. r4, 0, r31
	// stwcx.
	let addr = ctx.r[31].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[4].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 83252440: 7C610164  mtmsrd r3, 1
	ctx.msr = (ctx.r[3].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 83252444: 4082FFE8  bne 0x8325242c
	if !ctx.cr[0].eq {
	pc = 0x8325242C; continue 'dispatch;
	}
	// 83252448: 3D60832B  lis r11, -0x7cd5
	ctx.r[11].s64 = -2094333952;
	// 8325244C: 386B9988  addi r3, r11, -0x6678
	ctx.r[3].s64 = ctx.r[11].s64 + -26232;
	// 83252450: 4BA57AD1  bl 0x82ca9f20
	ctx.lr = 0x83252454;
	sub_82CA9F20(ctx, base);
	// 83252454: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83252458: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325245C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83252460: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83252464: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83252468(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83252468 size=64
    let mut pc: u32 = 0x83252468;
    'dispatch: loop {
        match pc {
            0x83252468 => {
    //   block [0x83252468..0x832524A8)
	// 83252468: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325246C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83252470: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83252474: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 83252478: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325247C: 388B33AC  addi r4, r11, 0x33ac
	ctx.r[4].s64 = ctx.r[11].s64 + 13228;
	// 83252480: 386A811C  addi r3, r10, -0x7ee4
	ctx.r[3].s64 = ctx.r[10].s64 + -32484;
	// 83252484: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83252488: 4AFDAA49  bl 0x8222ced0
	ctx.lr = 0x8325248C;
	sub_8222CED0(ctx, base);
	// 8325248C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83252490: 38699990  addi r3, r9, -0x6670
	ctx.r[3].s64 = ctx.r[9].s64 + -26224;
	// 83252494: 4BA57A8D  bl 0x82ca9f20
	ctx.lr = 0x83252498;
	sub_82CA9F20(ctx, base);
	// 83252498: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325249C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832524A0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832524A4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832524A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832524A8 size=64
    let mut pc: u32 = 0x832524A8;
    'dispatch: loop {
        match pc {
            0x832524A8 => {
    //   block [0x832524A8..0x832524E8)
	// 832524A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832524AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832524B0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832524B4: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 832524B8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832524BC: 388B33D0  addi r4, r11, 0x33d0
	ctx.r[4].s64 = ctx.r[11].s64 + 13264;
	// 832524C0: 386A8120  addi r3, r10, -0x7ee0
	ctx.r[3].s64 = ctx.r[10].s64 + -32480;
	// 832524C4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832524C8: 4AFDAA09  bl 0x8222ced0
	ctx.lr = 0x832524CC;
	sub_8222CED0(ctx, base);
	// 832524CC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 832524D0: 386999A0  addi r3, r9, -0x6660
	ctx.r[3].s64 = ctx.r[9].s64 + -26208;
	// 832524D4: 4BA57A4D  bl 0x82ca9f20
	ctx.lr = 0x832524D8;
	sub_82CA9F20(ctx, base);
	// 832524D8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832524DC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832524E0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832524E4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832524E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832524E8 size=64
    let mut pc: u32 = 0x832524E8;
    'dispatch: loop {
        match pc {
            0x832524E8 => {
    //   block [0x832524E8..0x83252528)
	// 832524E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832524EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832524F0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832524F4: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 832524F8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832524FC: 388B33E8  addi r4, r11, 0x33e8
	ctx.r[4].s64 = ctx.r[11].s64 + 13288;
	// 83252500: 386A8124  addi r3, r10, -0x7edc
	ctx.r[3].s64 = ctx.r[10].s64 + -32476;
	// 83252504: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83252508: 4AFDA9C9  bl 0x8222ced0
	ctx.lr = 0x8325250C;
	sub_8222CED0(ctx, base);
	// 8325250C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83252510: 386999B0  addi r3, r9, -0x6650
	ctx.r[3].s64 = ctx.r[9].s64 + -26192;
	// 83252514: 4BA57A0D  bl 0x82ca9f20
	ctx.lr = 0x83252518;
	sub_82CA9F20(ctx, base);
	// 83252518: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325251C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83252520: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83252524: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83252528(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83252528 size=64
    let mut pc: u32 = 0x83252528;
    'dispatch: loop {
        match pc {
            0x83252528 => {
    //   block [0x83252528..0x83252568)
	// 83252528: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325252C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83252530: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83252534: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 83252538: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325253C: 388B3404  addi r4, r11, 0x3404
	ctx.r[4].s64 = ctx.r[11].s64 + 13316;
	// 83252540: 386A8128  addi r3, r10, -0x7ed8
	ctx.r[3].s64 = ctx.r[10].s64 + -32472;
	// 83252544: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83252548: 4AFDA989  bl 0x8222ced0
	ctx.lr = 0x8325254C;
	sub_8222CED0(ctx, base);
	// 8325254C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83252550: 386999C0  addi r3, r9, -0x6640
	ctx.r[3].s64 = ctx.r[9].s64 + -26176;
	// 83252554: 4BA579CD  bl 0x82ca9f20
	ctx.lr = 0x83252558;
	sub_82CA9F20(ctx, base);
	// 83252558: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325255C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83252560: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83252564: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83252568(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83252568 size=64
    let mut pc: u32 = 0x83252568;
    'dispatch: loop {
        match pc {
            0x83252568 => {
    //   block [0x83252568..0x832525A8)
	// 83252568: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325256C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83252570: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83252574: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 83252578: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325257C: 388B3410  addi r4, r11, 0x3410
	ctx.r[4].s64 = ctx.r[11].s64 + 13328;
	// 83252580: 386A812C  addi r3, r10, -0x7ed4
	ctx.r[3].s64 = ctx.r[10].s64 + -32468;
	// 83252584: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83252588: 4AFDA949  bl 0x8222ced0
	ctx.lr = 0x8325258C;
	sub_8222CED0(ctx, base);
	// 8325258C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83252590: 386999D0  addi r3, r9, -0x6630
	ctx.r[3].s64 = ctx.r[9].s64 + -26160;
	// 83252594: 4BA5798D  bl 0x82ca9f20
	ctx.lr = 0x83252598;
	sub_82CA9F20(ctx, base);
	// 83252598: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325259C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832525A0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832525A4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832525A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832525A8 size=196
    let mut pc: u32 = 0x832525A8;
    'dispatch: loop {
        match pc {
            0x832525A8 => {
    //   block [0x832525A8..0x8325266C)
	// 832525A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832525AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832525B0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 832525B4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832525B8: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832525BC: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 832525C0: 3BEB8130  addi r31, r11, -0x7ed0
	ctx.r[31].s64 = ctx.r[11].s64 + -32464;
	// 832525C4: 388A0CA0  addi r4, r10, 0xca0
	ctx.r[4].s64 = ctx.r[10].s64 + 3232;
	// 832525C8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 832525CC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832525D0: 4AFDA901  bl 0x8222ced0
	ctx.lr = 0x832525D4;
	sub_8222CED0(ctx, base);
	// 832525D4: 3D20820B  lis r9, -0x7df5
	ctx.r[9].s64 = -2113208320;
	// 832525D8: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832525DC: 388934F8  addi r4, r9, 0x34f8
	ctx.r[4].s64 = ctx.r[9].s64 + 13560;
	// 832525E0: 387F0004  addi r3, r31, 4
	ctx.r[3].s64 = ctx.r[31].s64 + 4;
	// 832525E4: 4AFDA8ED  bl 0x8222ced0
	ctx.lr = 0x832525E8;
	sub_8222CED0(ctx, base);
	// 832525E8: 3D00820B  lis r8, -0x7df5
	ctx.r[8].s64 = -2113208320;
	// 832525EC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832525F0: 388834D0  addi r4, r8, 0x34d0
	ctx.r[4].s64 = ctx.r[8].s64 + 13520;
	// 832525F4: 387F0008  addi r3, r31, 8
	ctx.r[3].s64 = ctx.r[31].s64 + 8;
	// 832525F8: 4AFDA8D9  bl 0x8222ced0
	ctx.lr = 0x832525FC;
	sub_8222CED0(ctx, base);
	// 832525FC: 3CE0820B  lis r7, -0x7df5
	ctx.r[7].s64 = -2113208320;
	// 83252600: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83252604: 388734A8  addi r4, r7, 0x34a8
	ctx.r[4].s64 = ctx.r[7].s64 + 13480;
	// 83252608: 387F000C  addi r3, r31, 0xc
	ctx.r[3].s64 = ctx.r[31].s64 + 12;
	// 8325260C: 4AFDA8C5  bl 0x8222ced0
	ctx.lr = 0x83252610;
	sub_8222CED0(ctx, base);
	// 83252610: 3CC0820B  lis r6, -0x7df5
	ctx.r[6].s64 = -2113208320;
	// 83252614: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83252618: 38863480  addi r4, r6, 0x3480
	ctx.r[4].s64 = ctx.r[6].s64 + 13440;
	// 8325261C: 387F0010  addi r3, r31, 0x10
	ctx.r[3].s64 = ctx.r[31].s64 + 16;
	// 83252620: 4AFDA8B1  bl 0x8222ced0
	ctx.lr = 0x83252624;
	sub_8222CED0(ctx, base);
	// 83252624: 3C80820B  lis r4, -0x7df5
	ctx.r[4].s64 = -2113208320;
	// 83252628: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8325262C: 38843458  addi r4, r4, 0x3458
	ctx.r[4].s64 = ctx.r[4].s64 + 13400;
	// 83252630: 387F0014  addi r3, r31, 0x14
	ctx.r[3].s64 = ctx.r[31].s64 + 20;
	// 83252634: 4AFDA89D  bl 0x8222ced0
	ctx.lr = 0x83252638;
	sub_8222CED0(ctx, base);
	// 83252638: 3C60820B  lis r3, -0x7df5
	ctx.r[3].s64 = -2113208320;
	// 8325263C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83252640: 38833430  addi r4, r3, 0x3430
	ctx.r[4].s64 = ctx.r[3].s64 + 13360;
	// 83252644: 387F0018  addi r3, r31, 0x18
	ctx.r[3].s64 = ctx.r[31].s64 + 24;
	// 83252648: 4AFDA889  bl 0x8222ced0
	ctx.lr = 0x8325264C;
	sub_8222CED0(ctx, base);
	// 8325264C: 3D60832B  lis r11, -0x7cd5
	ctx.r[11].s64 = -2094333952;
	// 83252650: 386B99E0  addi r3, r11, -0x6620
	ctx.r[3].s64 = ctx.r[11].s64 + -26144;
	// 83252654: 4BA578CD  bl 0x82ca9f20
	ctx.lr = 0x83252658;
	sub_82CA9F20(ctx, base);
	// 83252658: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325265C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83252660: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83252664: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83252668: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83252670(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83252670 size=196
    let mut pc: u32 = 0x83252670;
    'dispatch: loop {
        match pc {
            0x83252670 => {
    //   block [0x83252670..0x83252734)
	// 83252670: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83252674: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83252678: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8325267C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83252680: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 83252684: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 83252688: 3BEB814C  addi r31, r11, -0x7eb4
	ctx.r[31].s64 = ctx.r[11].s64 + -32436;
	// 8325268C: 388A0CA0  addi r4, r10, 0xca0
	ctx.r[4].s64 = ctx.r[10].s64 + 3232;
	// 83252690: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83252694: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83252698: 4AFDA839  bl 0x8222ced0
	ctx.lr = 0x8325269C;
	sub_8222CED0(ctx, base);
	// 8325269C: 3D20820B  lis r9, -0x7df5
	ctx.r[9].s64 = -2113208320;
	// 832526A0: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832526A4: 38893598  addi r4, r9, 0x3598
	ctx.r[4].s64 = ctx.r[9].s64 + 13720;
	// 832526A8: 387F0004  addi r3, r31, 4
	ctx.r[3].s64 = ctx.r[31].s64 + 4;
	// 832526AC: 4AFDA825  bl 0x8222ced0
	ctx.lr = 0x832526B0;
	sub_8222CED0(ctx, base);
	// 832526B0: 3D00820B  lis r8, -0x7df5
	ctx.r[8].s64 = -2113208320;
	// 832526B4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832526B8: 38883580  addi r4, r8, 0x3580
	ctx.r[4].s64 = ctx.r[8].s64 + 13696;
	// 832526BC: 387F0008  addi r3, r31, 8
	ctx.r[3].s64 = ctx.r[31].s64 + 8;
	// 832526C0: 4AFDA811  bl 0x8222ced0
	ctx.lr = 0x832526C4;
	sub_8222CED0(ctx, base);
	// 832526C4: 3CE0820B  lis r7, -0x7df5
	ctx.r[7].s64 = -2113208320;
	// 832526C8: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832526CC: 38873568  addi r4, r7, 0x3568
	ctx.r[4].s64 = ctx.r[7].s64 + 13672;
	// 832526D0: 387F000C  addi r3, r31, 0xc
	ctx.r[3].s64 = ctx.r[31].s64 + 12;
	// 832526D4: 4AFDA7FD  bl 0x8222ced0
	ctx.lr = 0x832526D8;
	sub_8222CED0(ctx, base);
	// 832526D8: 3CC0820B  lis r6, -0x7df5
	ctx.r[6].s64 = -2113208320;
	// 832526DC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832526E0: 38863550  addi r4, r6, 0x3550
	ctx.r[4].s64 = ctx.r[6].s64 + 13648;
	// 832526E4: 387F0010  addi r3, r31, 0x10
	ctx.r[3].s64 = ctx.r[31].s64 + 16;
	// 832526E8: 4AFDA7E9  bl 0x8222ced0
	ctx.lr = 0x832526EC;
	sub_8222CED0(ctx, base);
	// 832526EC: 3C80820B  lis r4, -0x7df5
	ctx.r[4].s64 = -2113208320;
	// 832526F0: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832526F4: 38843538  addi r4, r4, 0x3538
	ctx.r[4].s64 = ctx.r[4].s64 + 13624;
	// 832526F8: 387F0014  addi r3, r31, 0x14
	ctx.r[3].s64 = ctx.r[31].s64 + 20;
	// 832526FC: 4AFDA7D5  bl 0x8222ced0
	ctx.lr = 0x83252700;
	sub_8222CED0(ctx, base);
	// 83252700: 3C60820B  lis r3, -0x7df5
	ctx.r[3].s64 = -2113208320;
	// 83252704: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83252708: 38833520  addi r4, r3, 0x3520
	ctx.r[4].s64 = ctx.r[3].s64 + 13600;
	// 8325270C: 387F0018  addi r3, r31, 0x18
	ctx.r[3].s64 = ctx.r[31].s64 + 24;
	// 83252710: 4AFDA7C1  bl 0x8222ced0
	ctx.lr = 0x83252714;
	sub_8222CED0(ctx, base);
	// 83252714: 3D60832B  lis r11, -0x7cd5
	ctx.r[11].s64 = -2094333952;
	// 83252718: 386B9A48  addi r3, r11, -0x65b8
	ctx.r[3].s64 = ctx.r[11].s64 + -26040;
	// 8325271C: 4BA57805  bl 0x82ca9f20
	ctx.lr = 0x83252720;
	sub_82CA9F20(ctx, base);
	// 83252720: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83252724: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83252728: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325272C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83252730: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83252738(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x83252738 size=108
    let mut pc: u32 = 0x83252738;
    'dispatch: loop {
        match pc {
            0x83252738 => {
    //   block [0x83252738..0x832527A4)
	// 83252738: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8325273C: 3941FFF4  addi r10, r1, -0xc
	ctx.r[10].s64 = ctx.r[1].s64 + -12;
	// 83252740: 392B92CC  addi r9, r11, -0x6d34
	ctx.r[9].s64 = ctx.r[11].s64 + -27956;
	// 83252744: 3901FFF8  addi r8, r1, -8
	ctx.r[8].s64 = ctx.r[1].s64 + -8;
	// 83252748: 3CE08210  lis r7, -0x7df0
	ctx.r[7].s64 = -2112880640;
	// 8325274C: C00B92CC  lfs f0, -0x6d34(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-27956 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 83252750: 38C1FFF8  addi r6, r1, -8
	ctx.r[6].s64 = ctx.r[1].s64 + -8;
	// 83252754: D001FFF0  stfs f0, -0x10(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), tmp.u32 ) };
	// 83252758: 38A1FFF0  addi r5, r1, -0x10
	ctx.r[5].s64 = ctx.r[1].s64 + -16;
	// 8325275C: C00901B8  lfs f0, 0x1b8(r9)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(440 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 83252760: 3C80834A  lis r4, -0x7cb6
	ctx.r[4].s64 = -2092302336;
	// 83252764: D001FFF4  stfs f0, -0xc(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-12 as u32), tmp.u32 ) };
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832527A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x832527A8 size=100
    let mut pc: u32 = 0x832527A8;
    'dispatch: loop {
        match pc {
            0x832527A8 => {
    //   block [0x832527A8..0x8325280C)
	// 832527A8: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 832527AC: 3941FFF4  addi r10, r1, -0xc
	ctx.r[10].s64 = ctx.r[1].s64 + -12;
	// 832527B0: 392B92D4  addi r9, r11, -0x6d2c
	ctx.r[9].s64 = ctx.r[11].s64 + -27948;
	// 832527B4: 3901FFF8  addi r8, r1, -8
	ctx.r[8].s64 = ctx.r[1].s64 + -8;
	// 832527B8: 38E1FFF0  addi r7, r1, -0x10
	ctx.r[7].s64 = ctx.r[1].s64 + -16;
	// 832527BC: C00B92D4  lfs f0, -0x6d2c(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-27948 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 832527C0: 38C1FFF8  addi r6, r1, -8
	ctx.r[6].s64 = ctx.r[1].s64 + -8;
	// 832527C4: D001FFF0  stfs f0, -0x10(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), tmp.u32 ) };
	// 832527C8: 3CA0834A  lis r5, -0x7cb6
	ctx.r[5].s64 = -2092302336;
	// 832527CC: C1A901B0  lfs f13, 0x1b0(r9)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(432 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 832527D0: D1A1FFF4  stfs f13, -0xc(r1)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-12 as u32), tmp.u32 ) };
	// 832527D4: 38858180  addi r4, r5, -0x7e80
	ctx.r[4].s64 = ctx.r[5].s64 + -32384;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83252810(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x83252810 size=96
    let mut pc: u32 = 0x83252810;
    'dispatch: loop {
        match pc {
            0x83252810 => {
    //   block [0x83252810..0x83252870)
	// 83252810: 3D40820A  lis r10, -0x7df6
	ctx.r[10].s64 = -2113273856;
	// 83252814: 3921FFF0  addi r9, r1, -0x10
	ctx.r[9].s64 = ctx.r[1].s64 + -16;
	// 83252818: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 8325281C: 3901FFF4  addi r8, r1, -0xc
	ctx.r[8].s64 = ctx.r[1].s64 + -12;
	// 83252820: 38E1FFF4  addi r7, r1, -0xc
	ctx.r[7].s64 = ctx.r[1].s64 + -12;
	// 83252824: C1AA9490  lfs f13, -0x6b70(r10)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-27504 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 83252828: 38C1FFF0  addi r6, r1, -0x10
	ctx.r[6].s64 = ctx.r[1].s64 + -16;
	// 8325282C: D1A1FFF0  stfs f13, -0x10(r1)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), tmp.u32 ) };
	// 83252830: 3CA0834A  lis r5, -0x7cb6
	ctx.r[5].s64 = -2092302336;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83252870(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x83252870 size=112
    let mut pc: u32 = 0x83252870;
    'dispatch: loop {
        match pc {
            0x83252870 => {
    //   block [0x83252870..0x832528E0)
	// 83252870: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83252874: 3901FFF0  addi r8, r1, -0x10
	ctx.r[8].s64 = ctx.r[1].s64 + -16;
	// 83252878: 392BB480  addi r9, r11, -0x4b80
	ctx.r[9].s64 = ctx.r[11].s64 + -19328;
	// 8325287C: 3941FFF4  addi r10, r1, -0xc
	ctx.r[10].s64 = ctx.r[1].s64 + -12;
	// 83252880: 3CE08210  lis r7, -0x7df0
	ctx.r[7].s64 = -2112880640;
	// 83252884: C00BB480  lfs f0, -0x4b80(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-19328 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 83252888: 3CC08210  lis r6, -0x7df0
	ctx.r[6].s64 = -2112880640;
	// 8325288C: D001FFF0  stfs f0, -0x10(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), tmp.u32 ) };
	// 83252890: 38A1FFF4  addi r5, r1, -0xc
	ctx.r[5].s64 = ctx.r[1].s64 + -12;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832528E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x832528E0 size=108
    let mut pc: u32 = 0x832528E0;
    'dispatch: loop {
        match pc {
            0x832528E0 => {
    //   block [0x832528E0..0x8325294C)
	// 832528E0: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 832528E4: 3901FFF0  addi r8, r1, -0x10
	ctx.r[8].s64 = ctx.r[1].s64 + -16;
	// 832528E8: 392B92D4  addi r9, r11, -0x6d2c
	ctx.r[9].s64 = ctx.r[11].s64 + -27948;
	// 832528EC: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 832528F0: 38E1FFF4  addi r7, r1, -0xc
	ctx.r[7].s64 = ctx.r[1].s64 + -12;
	// 832528F4: 38C1FFF4  addi r6, r1, -0xc
	ctx.r[6].s64 = ctx.r[1].s64 + -12;
	// 832528F8: C1AB92D4  lfs f13, -0x6d2c(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-27948 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 832528FC: 38A1FFF0  addi r5, r1, -0x10
	ctx.r[5].s64 = ctx.r[1].s64 + -16;
	// 83252900: C00901B0  lfs f0, 0x1b0(r9)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(432 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 83252904: 3C80834A  lis r4, -0x7cb6
	ctx.r[4].s64 = -2092302336;
	// 83252908: D001FFF0  stfs f0, -0x10(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), tmp.u32 ) };
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83252950(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x83252950 size=108
    let mut pc: u32 = 0x83252950;
    'dispatch: loop {
        match pc {
            0x83252950 => {
    //   block [0x83252950..0x832529BC)
	// 83252950: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83252954: 3941FFF4  addi r10, r1, -0xc
	ctx.r[10].s64 = ctx.r[1].s64 + -12;
	// 83252958: 392BE0DC  addi r9, r11, -0x1f24
	ctx.r[9].s64 = ctx.r[11].s64 + -7972;
	// 8325295C: 3901FFF8  addi r8, r1, -8
	ctx.r[8].s64 = ctx.r[1].s64 + -8;
	// 83252960: 3CE08210  lis r7, -0x7df0
	ctx.r[7].s64 = -2112880640;
	// 83252964: C00BE0DC  lfs f0, -0x1f24(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-7972 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 83252968: 38C1FFF0  addi r6, r1, -0x10
	ctx.r[6].s64 = ctx.r[1].s64 + -16;
	// 8325296C: D001FFF0  stfs f0, -0x10(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), tmp.u32 ) };
	// 83252970: 38A1FFF8  addi r5, r1, -8
	ctx.r[5].s64 = ctx.r[1].s64 + -8;
	// 83252974: C009B3B4  lfs f0, -0x4c4c(r9)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(-19532 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 83252978: 3C80834A  lis r4, -0x7cb6
	ctx.r[4].s64 = -2092302336;
	// 8325297C: D001FFF4  stfs f0, -0xc(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-12 as u32), tmp.u32 ) };
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832529C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832529C0 size=64
    let mut pc: u32 = 0x832529C0;
    'dispatch: loop {
        match pc {
            0x832529C0 => {
    //   block [0x832529C0..0x83252A00)
	// 832529C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832529C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832529C8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832529CC: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 832529D0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832529D4: 388B35B0  addi r4, r11, 0x35b0
	ctx.r[4].s64 = ctx.r[11].s64 + 13744;
	// 832529D8: 386A8168  addi r3, r10, -0x7e98
	ctx.r[3].s64 = ctx.r[10].s64 + -32408;
	// 832529DC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832529E0: 4AFDA4F1  bl 0x8222ced0
	ctx.lr = 0x832529E4;
	sub_8222CED0(ctx, base);
	// 832529E4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 832529E8: 38699AB0  addi r3, r9, -0x6550
	ctx.r[3].s64 = ctx.r[9].s64 + -25936;
	// 832529EC: 4BA57535  bl 0x82ca9f20
	ctx.lr = 0x832529F0;
	sub_82CA9F20(ctx, base);
	// 832529F0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832529F4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832529F8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832529FC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83252A00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83252A00 size=64
    let mut pc: u32 = 0x83252A00;
    'dispatch: loop {
        match pc {
            0x83252A00 => {
    //   block [0x83252A00..0x83252A40)
	// 83252A00: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83252A04: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83252A08: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83252A0C: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 83252A10: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83252A14: 388B35D4  addi r4, r11, 0x35d4
	ctx.r[4].s64 = ctx.r[11].s64 + 13780;
	// 83252A18: 386A816C  addi r3, r10, -0x7e94
	ctx.r[3].s64 = ctx.r[10].s64 + -32404;
	// 83252A1C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83252A20: 4AFDA4B1  bl 0x8222ced0
	ctx.lr = 0x83252A24;
	sub_8222CED0(ctx, base);
	// 83252A24: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83252A28: 38699AC0  addi r3, r9, -0x6540
	ctx.r[3].s64 = ctx.r[9].s64 + -25920;
	// 83252A2C: 4BA574F5  bl 0x82ca9f20
	ctx.lr = 0x83252A30;
	sub_82CA9F20(ctx, base);
	// 83252A30: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83252A34: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83252A38: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83252A3C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83252A40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83252A40 size=64
    let mut pc: u32 = 0x83252A40;
    'dispatch: loop {
        match pc {
            0x83252A40 => {
    //   block [0x83252A40..0x83252A80)
	// 83252A40: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83252A44: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83252A48: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83252A4C: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 83252A50: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83252A54: 388B35F8  addi r4, r11, 0x35f8
	ctx.r[4].s64 = ctx.r[11].s64 + 13816;
	// 83252A58: 386A81D0  addi r3, r10, -0x7e30
	ctx.r[3].s64 = ctx.r[10].s64 + -32304;
	// 83252A5C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83252A60: 4AFDA471  bl 0x8222ced0
	ctx.lr = 0x83252A64;
	sub_8222CED0(ctx, base);
	// 83252A64: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83252A68: 38699AD0  addi r3, r9, -0x6530
	ctx.r[3].s64 = ctx.r[9].s64 + -25904;
	// 83252A6C: 4BA574B5  bl 0x82ca9f20
	ctx.lr = 0x83252A70;
	sub_82CA9F20(ctx, base);
	// 83252A70: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83252A74: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83252A78: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83252A7C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83252A80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83252A80 size=64
    let mut pc: u32 = 0x83252A80;
    'dispatch: loop {
        match pc {
            0x83252A80 => {
    //   block [0x83252A80..0x83252AC0)
	// 83252A80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83252A84: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83252A88: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83252A8C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83252A90: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83252A94: 388B1AD4  addi r4, r11, 0x1ad4
	ctx.r[4].s64 = ctx.r[11].s64 + 6868;
	// 83252A98: 386A81D4  addi r3, r10, -0x7e2c
	ctx.r[3].s64 = ctx.r[10].s64 + -32300;
	// 83252A9C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83252AA0: 4AFDA431  bl 0x8222ced0
	ctx.lr = 0x83252AA4;
	sub_8222CED0(ctx, base);
	// 83252AA4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83252AA8: 38699AE0  addi r3, r9, -0x6520
	ctx.r[3].s64 = ctx.r[9].s64 + -25888;
	// 83252AAC: 4BA57475  bl 0x82ca9f20
	ctx.lr = 0x83252AB0;
	sub_82CA9F20(ctx, base);
	// 83252AB0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83252AB4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83252AB8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83252ABC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83252AC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83252AC0 size=64
    let mut pc: u32 = 0x83252AC0;
    'dispatch: loop {
        match pc {
            0x83252AC0 => {
    //   block [0x83252AC0..0x83252B00)
	// 83252AC0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83252AC4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83252AC8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83252ACC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83252AD0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83252AD4: 388B1ADC  addi r4, r11, 0x1adc
	ctx.r[4].s64 = ctx.r[11].s64 + 6876;
	// 83252AD8: 386A81D8  addi r3, r10, -0x7e28
	ctx.r[3].s64 = ctx.r[10].s64 + -32296;
	// 83252ADC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83252AE0: 4AFDA3F1  bl 0x8222ced0
	ctx.lr = 0x83252AE4;
	sub_8222CED0(ctx, base);
	// 83252AE4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83252AE8: 38699AF0  addi r3, r9, -0x6510
	ctx.r[3].s64 = ctx.r[9].s64 + -25872;
	// 83252AEC: 4BA57435  bl 0x82ca9f20
	ctx.lr = 0x83252AF0;
	sub_82CA9F20(ctx, base);
	// 83252AF0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83252AF4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83252AF8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83252AFC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83252B00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83252B00 size=64
    let mut pc: u32 = 0x83252B00;
    'dispatch: loop {
        match pc {
            0x83252B00 => {
    //   block [0x83252B00..0x83252B40)
	// 83252B00: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83252B04: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83252B08: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83252B0C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83252B10: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83252B14: 388B1AF0  addi r4, r11, 0x1af0
	ctx.r[4].s64 = ctx.r[11].s64 + 6896;
	// 83252B18: 386A81DC  addi r3, r10, -0x7e24
	ctx.r[3].s64 = ctx.r[10].s64 + -32292;
	// 83252B1C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83252B20: 4AFDA3B1  bl 0x8222ced0
	ctx.lr = 0x83252B24;
	sub_8222CED0(ctx, base);
	// 83252B24: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83252B28: 38699B00  addi r3, r9, -0x6500
	ctx.r[3].s64 = ctx.r[9].s64 + -25856;
	// 83252B2C: 4BA573F5  bl 0x82ca9f20
	ctx.lr = 0x83252B30;
	sub_82CA9F20(ctx, base);
	// 83252B30: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83252B34: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83252B38: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83252B3C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83252B40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83252B40 size=64
    let mut pc: u32 = 0x83252B40;
    'dispatch: loop {
        match pc {
            0x83252B40 => {
    //   block [0x83252B40..0x83252B80)
	// 83252B40: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83252B44: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83252B48: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83252B4C: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 83252B50: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83252B54: 388B3F60  addi r4, r11, 0x3f60
	ctx.r[4].s64 = ctx.r[11].s64 + 16224;
	// 83252B58: 386A81E0  addi r3, r10, -0x7e20
	ctx.r[3].s64 = ctx.r[10].s64 + -32288;
	// 83252B5C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83252B60: 4AFDA371  bl 0x8222ced0
	ctx.lr = 0x83252B64;
	sub_8222CED0(ctx, base);
	// 83252B64: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83252B68: 38699B10  addi r3, r9, -0x64f0
	ctx.r[3].s64 = ctx.r[9].s64 + -25840;
	// 83252B6C: 4BA573B5  bl 0x82ca9f20
	ctx.lr = 0x83252B70;
	sub_82CA9F20(ctx, base);
	// 83252B70: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83252B74: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83252B78: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83252B7C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83252B80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83252B80 size=64
    let mut pc: u32 = 0x83252B80;
    'dispatch: loop {
        match pc {
            0x83252B80 => {
    //   block [0x83252B80..0x83252BC0)
	// 83252B80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83252B84: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83252B88: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83252B8C: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 83252B90: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83252B94: 388B3F68  addi r4, r11, 0x3f68
	ctx.r[4].s64 = ctx.r[11].s64 + 16232;
	// 83252B98: 386A81E4  addi r3, r10, -0x7e1c
	ctx.r[3].s64 = ctx.r[10].s64 + -32284;
	// 83252B9C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83252BA0: 4AFDA331  bl 0x8222ced0
	ctx.lr = 0x83252BA4;
	sub_8222CED0(ctx, base);
	// 83252BA4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83252BA8: 38699B20  addi r3, r9, -0x64e0
	ctx.r[3].s64 = ctx.r[9].s64 + -25824;
	// 83252BAC: 4BA57375  bl 0x82ca9f20
	ctx.lr = 0x83252BB0;
	sub_82CA9F20(ctx, base);
	// 83252BB0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83252BB4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83252BB8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83252BBC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83252BC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83252BC0 size=64
    let mut pc: u32 = 0x83252BC0;
    'dispatch: loop {
        match pc {
            0x83252BC0 => {
    //   block [0x83252BC0..0x83252C00)
	// 83252BC0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83252BC4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83252BC8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83252BCC: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 83252BD0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83252BD4: 388B3F74  addi r4, r11, 0x3f74
	ctx.r[4].s64 = ctx.r[11].s64 + 16244;
	// 83252BD8: 386A81E8  addi r3, r10, -0x7e18
	ctx.r[3].s64 = ctx.r[10].s64 + -32280;
	// 83252BDC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83252BE0: 4AFDA2F1  bl 0x8222ced0
	ctx.lr = 0x83252BE4;
	sub_8222CED0(ctx, base);
	// 83252BE4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83252BE8: 38699B30  addi r3, r9, -0x64d0
	ctx.r[3].s64 = ctx.r[9].s64 + -25808;
	// 83252BEC: 4BA57335  bl 0x82ca9f20
	ctx.lr = 0x83252BF0;
	sub_82CA9F20(ctx, base);
	// 83252BF0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83252BF4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83252BF8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83252BFC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83252C00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83252C00 size=64
    let mut pc: u32 = 0x83252C00;
    'dispatch: loop {
        match pc {
            0x83252C00 => {
    //   block [0x83252C00..0x83252C40)
	// 83252C00: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83252C04: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83252C08: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83252C0C: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 83252C10: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83252C14: 388B3F80  addi r4, r11, 0x3f80
	ctx.r[4].s64 = ctx.r[11].s64 + 16256;
	// 83252C18: 386A81EC  addi r3, r10, -0x7e14
	ctx.r[3].s64 = ctx.r[10].s64 + -32276;
	// 83252C1C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83252C20: 4AFDA2B1  bl 0x8222ced0
	ctx.lr = 0x83252C24;
	sub_8222CED0(ctx, base);
	// 83252C24: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83252C28: 38699B40  addi r3, r9, -0x64c0
	ctx.r[3].s64 = ctx.r[9].s64 + -25792;
	// 83252C2C: 4BA572F5  bl 0x82ca9f20
	ctx.lr = 0x83252C30;
	sub_82CA9F20(ctx, base);
	// 83252C30: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83252C34: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83252C38: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83252C3C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83252C40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83252C40 size=64
    let mut pc: u32 = 0x83252C40;
    'dispatch: loop {
        match pc {
            0x83252C40 => {
    //   block [0x83252C40..0x83252C80)
	// 83252C40: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83252C44: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83252C48: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83252C4C: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 83252C50: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83252C54: 388B3F90  addi r4, r11, 0x3f90
	ctx.r[4].s64 = ctx.r[11].s64 + 16272;
	// 83252C58: 386A81F0  addi r3, r10, -0x7e10
	ctx.r[3].s64 = ctx.r[10].s64 + -32272;
	// 83252C5C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83252C60: 4AFDA271  bl 0x8222ced0
	ctx.lr = 0x83252C64;
	sub_8222CED0(ctx, base);
	// 83252C64: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83252C68: 38699B50  addi r3, r9, -0x64b0
	ctx.r[3].s64 = ctx.r[9].s64 + -25776;
	// 83252C6C: 4BA572B5  bl 0x82ca9f20
	ctx.lr = 0x83252C70;
	sub_82CA9F20(ctx, base);
	// 83252C70: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83252C74: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83252C78: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83252C7C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83252C80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83252C80 size=64
    let mut pc: u32 = 0x83252C80;
    'dispatch: loop {
        match pc {
            0x83252C80 => {
    //   block [0x83252C80..0x83252CC0)
	// 83252C80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83252C84: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83252C88: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83252C8C: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 83252C90: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83252C94: 388B3F9C  addi r4, r11, 0x3f9c
	ctx.r[4].s64 = ctx.r[11].s64 + 16284;
	// 83252C98: 386A81F4  addi r3, r10, -0x7e0c
	ctx.r[3].s64 = ctx.r[10].s64 + -32268;
	// 83252C9C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83252CA0: 4AFDA231  bl 0x8222ced0
	ctx.lr = 0x83252CA4;
	sub_8222CED0(ctx, base);
	// 83252CA4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83252CA8: 38699B60  addi r3, r9, -0x64a0
	ctx.r[3].s64 = ctx.r[9].s64 + -25760;
	// 83252CAC: 4BA57275  bl 0x82ca9f20
	ctx.lr = 0x83252CB0;
	sub_82CA9F20(ctx, base);
	// 83252CB0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83252CB4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83252CB8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83252CBC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83252CC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83252CC0 size=64
    let mut pc: u32 = 0x83252CC0;
    'dispatch: loop {
        match pc {
            0x83252CC0 => {
    //   block [0x83252CC0..0x83252D00)
	// 83252CC0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83252CC4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83252CC8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83252CCC: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 83252CD0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83252CD4: 388B3FAC  addi r4, r11, 0x3fac
	ctx.r[4].s64 = ctx.r[11].s64 + 16300;
	// 83252CD8: 386A81F8  addi r3, r10, -0x7e08
	ctx.r[3].s64 = ctx.r[10].s64 + -32264;
	// 83252CDC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83252CE0: 4AFDA1F1  bl 0x8222ced0
	ctx.lr = 0x83252CE4;
	sub_8222CED0(ctx, base);
	// 83252CE4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83252CE8: 38699B70  addi r3, r9, -0x6490
	ctx.r[3].s64 = ctx.r[9].s64 + -25744;
	// 83252CEC: 4BA57235  bl 0x82ca9f20
	ctx.lr = 0x83252CF0;
	sub_82CA9F20(ctx, base);
	// 83252CF0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83252CF4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83252CF8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83252CFC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83252D00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83252D00 size=56
    let mut pc: u32 = 0x83252D00;
    'dispatch: loop {
        match pc {
            0x83252D00 => {
    //   block [0x83252D00..0x83252D38)
	// 83252D00: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83252D04: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83252D08: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83252D0C: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 83252D10: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83252D14: 386B3FB4  addi r3, r11, 0x3fb4
	ctx.r[3].s64 = ctx.r[11].s64 + 16308;
	// 83252D18: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83252D1C: 4AFA103D  bl 0x821f3d58
	ctx.lr = 0x83252D20;
	sub_821F3D58(ctx, base);
	// 83252D20: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83252D24: 906A81FC  stw r3, -0x7e04(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-32260 as u32), ctx.r[3].u32 ) };
	// 83252D28: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83252D2C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83252D30: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83252D34: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83252D38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83252D38 size=64
    let mut pc: u32 = 0x83252D38;
    'dispatch: loop {
        match pc {
            0x83252D38 => {
    //   block [0x83252D38..0x83252D78)
	// 83252D38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83252D3C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83252D40: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83252D44: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 83252D48: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83252D4C: 388B3FD0  addi r4, r11, 0x3fd0
	ctx.r[4].s64 = ctx.r[11].s64 + 16336;
	// 83252D50: 386A8200  addi r3, r10, -0x7e00
	ctx.r[3].s64 = ctx.r[10].s64 + -32256;
	// 83252D54: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83252D58: 4AFDA179  bl 0x8222ced0
	ctx.lr = 0x83252D5C;
	sub_8222CED0(ctx, base);
	// 83252D5C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83252D60: 38699B80  addi r3, r9, -0x6480
	ctx.r[3].s64 = ctx.r[9].s64 + -25728;
	// 83252D64: 4BA571BD  bl 0x82ca9f20
	ctx.lr = 0x83252D68;
	sub_82CA9F20(ctx, base);
	// 83252D68: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83252D6C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83252D70: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83252D74: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83252D78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83252D78 size=64
    let mut pc: u32 = 0x83252D78;
    'dispatch: loop {
        match pc {
            0x83252D78 => {
    //   block [0x83252D78..0x83252DB8)
	// 83252D78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83252D7C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83252D80: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83252D84: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83252D88: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83252D8C: 388B052C  addi r4, r11, 0x52c
	ctx.r[4].s64 = ctx.r[11].s64 + 1324;
	// 83252D90: 386A8204  addi r3, r10, -0x7dfc
	ctx.r[3].s64 = ctx.r[10].s64 + -32252;
	// 83252D94: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83252D98: 4AFDA139  bl 0x8222ced0
	ctx.lr = 0x83252D9C;
	sub_8222CED0(ctx, base);
	// 83252D9C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83252DA0: 38699B90  addi r3, r9, -0x6470
	ctx.r[3].s64 = ctx.r[9].s64 + -25712;
	// 83252DA4: 4BA5717D  bl 0x82ca9f20
	ctx.lr = 0x83252DA8;
	sub_82CA9F20(ctx, base);
	// 83252DA8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83252DAC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83252DB0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83252DB4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83252DB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83252DB8 size=64
    let mut pc: u32 = 0x83252DB8;
    'dispatch: loop {
        match pc {
            0x83252DB8 => {
    //   block [0x83252DB8..0x83252DF8)
	// 83252DB8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83252DBC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83252DC0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83252DC4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83252DC8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83252DCC: 388B1AD4  addi r4, r11, 0x1ad4
	ctx.r[4].s64 = ctx.r[11].s64 + 6868;
	// 83252DD0: 386A8208  addi r3, r10, -0x7df8
	ctx.r[3].s64 = ctx.r[10].s64 + -32248;
	// 83252DD4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83252DD8: 4AFDA0F9  bl 0x8222ced0
	ctx.lr = 0x83252DDC;
	sub_8222CED0(ctx, base);
	// 83252DDC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83252DE0: 38699BA0  addi r3, r9, -0x6460
	ctx.r[3].s64 = ctx.r[9].s64 + -25696;
	// 83252DE4: 4BA5713D  bl 0x82ca9f20
	ctx.lr = 0x83252DE8;
	sub_82CA9F20(ctx, base);
	// 83252DE8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83252DEC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83252DF0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83252DF4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83252DF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83252DF8 size=64
    let mut pc: u32 = 0x83252DF8;
    'dispatch: loop {
        match pc {
            0x83252DF8 => {
    //   block [0x83252DF8..0x83252E38)
	// 83252DF8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83252DFC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83252E00: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83252E04: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83252E08: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83252E0C: 388B1ADC  addi r4, r11, 0x1adc
	ctx.r[4].s64 = ctx.r[11].s64 + 6876;
	// 83252E10: 386A820C  addi r3, r10, -0x7df4
	ctx.r[3].s64 = ctx.r[10].s64 + -32244;
	// 83252E14: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83252E18: 4AFDA0B9  bl 0x8222ced0
	ctx.lr = 0x83252E1C;
	sub_8222CED0(ctx, base);
	// 83252E1C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83252E20: 38699BB0  addi r3, r9, -0x6450
	ctx.r[3].s64 = ctx.r[9].s64 + -25680;
	// 83252E24: 4BA570FD  bl 0x82ca9f20
	ctx.lr = 0x83252E28;
	sub_82CA9F20(ctx, base);
	// 83252E28: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83252E2C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83252E30: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83252E34: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83252E38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83252E38 size=64
    let mut pc: u32 = 0x83252E38;
    'dispatch: loop {
        match pc {
            0x83252E38 => {
    //   block [0x83252E38..0x83252E78)
	// 83252E38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83252E3C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83252E40: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83252E44: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83252E48: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83252E4C: 388B1AF0  addi r4, r11, 0x1af0
	ctx.r[4].s64 = ctx.r[11].s64 + 6896;
	// 83252E50: 386A8210  addi r3, r10, -0x7df0
	ctx.r[3].s64 = ctx.r[10].s64 + -32240;
	// 83252E54: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83252E58: 4AFDA079  bl 0x8222ced0
	ctx.lr = 0x83252E5C;
	sub_8222CED0(ctx, base);
	// 83252E5C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83252E60: 38699BC0  addi r3, r9, -0x6440
	ctx.r[3].s64 = ctx.r[9].s64 + -25664;
	// 83252E64: 4BA570BD  bl 0x82ca9f20
	ctx.lr = 0x83252E68;
	sub_82CA9F20(ctx, base);
	// 83252E68: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83252E6C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83252E70: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83252E74: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83252E78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83252E78 size=144
    let mut pc: u32 = 0x83252E78;
    'dispatch: loop {
        match pc {
            0x83252E78 => {
    //   block [0x83252E78..0x83252E9C)
	// 83252E78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83252E7C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83252E80: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83252E84: 38600020  li r3, 0x20
	ctx.r[3].s64 = 32;
	// 83252E88: 4AFCC3D1  bl 0x8221f258
	ctx.lr = 0x83252E8C;
	sub_8221F258(ctx, base);
	// 83252E8C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 83252E90: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 83252E94: 419A0008  beq cr6, 0x83252e9c
	if ctx.cr[6].eq {
	pc = 0x83252E9C; continue 'dispatch;
	}
	// 83252E98: 91430000  stw r10, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	pc = 0x83252E9C; continue 'dispatch;
            }
            0x83252E9C => {
    //   block [0x83252E9C..0x83252EA8)
	// 83252E9C: 35630004  addic. r11, r3, 4
	ctx.xer.ca = (ctx.r[3].u32 > (!(4 as u32)));
	ctx.r[11].s64 = ctx.r[3].s64 + 4;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83252EA0: 41820008  beq 0x83252ea8
	if ctx.cr[0].eq {
	pc = 0x83252EA8; continue 'dispatch;
	}
	// 83252EA4: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	pc = 0x83252EA8; continue 'dispatch;
            }
            0x83252EA8 => {
    //   block [0x83252EA8..0x83252EB4)
	// 83252EA8: 35630008  addic. r11, r3, 8
	ctx.xer.ca = (ctx.r[3].u32 > (!(8 as u32)));
	ctx.r[11].s64 = ctx.r[3].s64 + 8;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83252EAC: 41820008  beq 0x83252eb4
	if ctx.cr[0].eq {
	pc = 0x83252EB4; continue 'dispatch;
	}
	// 83252EB0: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	pc = 0x83252EB4; continue 'dispatch;
            }
            0x83252EB4 => {
    //   block [0x83252EB4..0x83252F08)
	// 83252EB4: 3D20834A  lis r9, -0x7cb6
	ctx.r[9].s64 = -2092302336;
	// 83252EB8: 9943001D  stb r10, 0x1d(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(29 as u32), ctx.r[10].u8 ) };
	// 83252EBC: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 83252EC0: 39098214  addi r8, r9, -0x7dec
	ctx.r[8].s64 = ctx.r[9].s64 + -32236;
	// 83252EC4: 9963001C  stb r11, 0x1c(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(28 as u32), ctx.r[11].u8 ) };
	// 83252EC8: 3CE0832B  lis r7, -0x7cd5
	ctx.r[7].s64 = -2094333952;
	// 83252ECC: 90680004  stw r3, 4(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(4 as u32), ctx.r[3].u32 ) };
	// 83252ED0: 9963001D  stb r11, 0x1d(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(29 as u32), ctx.r[11].u8 ) };
	// 83252ED4: 38679BD0  addi r3, r7, -0x6430
	ctx.r[3].s64 = ctx.r[7].s64 + -25648;
	// 83252ED8: 81680004  lwz r11, 4(r8)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(4 as u32) ) } as u64;
	// 83252EDC: 916B0004  stw r11, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 83252EE0: 81680004  lwz r11, 4(r8)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(4 as u32) ) } as u64;
	// 83252EE4: 916B0000  stw r11, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83252EE8: 81680004  lwz r11, 4(r8)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(4 as u32) ) } as u64;
	// 83252EEC: 916B0008  stw r11, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 83252EF0: 91480008  stw r10, 8(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 83252EF4: 4BA5702D  bl 0x82ca9f20
	ctx.lr = 0x83252EF8;
	sub_82CA9F20(ctx, base);
	// 83252EF8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83252EFC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83252F00: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83252F04: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83252F08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83252F08 size=144
    let mut pc: u32 = 0x83252F08;
    'dispatch: loop {
        match pc {
            0x83252F08 => {
    //   block [0x83252F08..0x83252F2C)
	// 83252F08: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83252F0C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83252F10: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83252F14: 38600018  li r3, 0x18
	ctx.r[3].s64 = 24;
	// 83252F18: 4AFCC341  bl 0x8221f258
	ctx.lr = 0x83252F1C;
	sub_8221F258(ctx, base);
	// 83252F1C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 83252F20: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 83252F24: 419A0008  beq cr6, 0x83252f2c
	if ctx.cr[6].eq {
	pc = 0x83252F2C; continue 'dispatch;
	}
	// 83252F28: 91430000  stw r10, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	pc = 0x83252F2C; continue 'dispatch;
            }
            0x83252F2C => {
    //   block [0x83252F2C..0x83252F38)
	// 83252F2C: 35630004  addic. r11, r3, 4
	ctx.xer.ca = (ctx.r[3].u32 > (!(4 as u32)));
	ctx.r[11].s64 = ctx.r[3].s64 + 4;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83252F30: 41820008  beq 0x83252f38
	if ctx.cr[0].eq {
	pc = 0x83252F38; continue 'dispatch;
	}
	// 83252F34: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	pc = 0x83252F38; continue 'dispatch;
            }
            0x83252F38 => {
    //   block [0x83252F38..0x83252F44)
	// 83252F38: 35630008  addic. r11, r3, 8
	ctx.xer.ca = (ctx.r[3].u32 > (!(8 as u32)));
	ctx.r[11].s64 = ctx.r[3].s64 + 8;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83252F3C: 41820008  beq 0x83252f44
	if ctx.cr[0].eq {
	pc = 0x83252F44; continue 'dispatch;
	}
	// 83252F40: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	pc = 0x83252F44; continue 'dispatch;
            }
            0x83252F44 => {
    //   block [0x83252F44..0x83252F98)
	// 83252F44: 3D20834A  lis r9, -0x7cb6
	ctx.r[9].s64 = -2092302336;
	// 83252F48: 99430015  stb r10, 0x15(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(21 as u32), ctx.r[10].u8 ) };
	// 83252F4C: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 83252F50: 39098220  addi r8, r9, -0x7de0
	ctx.r[8].s64 = ctx.r[9].s64 + -32224;
	// 83252F54: 99630014  stb r11, 0x14(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(20 as u32), ctx.r[11].u8 ) };
	// 83252F58: 3CE0832B  lis r7, -0x7cd5
	ctx.r[7].s64 = -2094333952;
	// 83252F5C: 90680004  stw r3, 4(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(4 as u32), ctx.r[3].u32 ) };
	// 83252F60: 99630015  stb r11, 0x15(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(21 as u32), ctx.r[11].u8 ) };
	// 83252F64: 38679BE0  addi r3, r7, -0x6420
	ctx.r[3].s64 = ctx.r[7].s64 + -25632;
	// 83252F68: 81680004  lwz r11, 4(r8)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(4 as u32) ) } as u64;
	// 83252F6C: 916B0004  stw r11, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 83252F70: 81680004  lwz r11, 4(r8)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(4 as u32) ) } as u64;
	// 83252F74: 916B0000  stw r11, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83252F78: 81680004  lwz r11, 4(r8)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(4 as u32) ) } as u64;
	// 83252F7C: 916B0008  stw r11, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 83252F80: 91480008  stw r10, 8(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 83252F84: 4BA56F9D  bl 0x82ca9f20
	ctx.lr = 0x83252F88;
	sub_82CA9F20(ctx, base);
	// 83252F88: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83252F8C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83252F90: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83252F94: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83252F98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83252F98 size=64
    let mut pc: u32 = 0x83252F98;
    'dispatch: loop {
        match pc {
            0x83252F98 => {
    //   block [0x83252F98..0x83252FD8)
	// 83252F98: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83252F9C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83252FA0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83252FA4: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 83252FA8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83252FAC: 388B6F20  addi r4, r11, 0x6f20
	ctx.r[4].s64 = ctx.r[11].s64 + 28448;
	// 83252FB0: 386A822C  addi r3, r10, -0x7dd4
	ctx.r[3].s64 = ctx.r[10].s64 + -32212;
	// 83252FB4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83252FB8: 4AFD9F19  bl 0x8222ced0
	ctx.lr = 0x83252FBC;
	sub_8222CED0(ctx, base);
	// 83252FBC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83252FC0: 38699BF0  addi r3, r9, -0x6410
	ctx.r[3].s64 = ctx.r[9].s64 + -25616;
	// 83252FC4: 4BA56F5D  bl 0x82ca9f20
	ctx.lr = 0x83252FC8;
	sub_82CA9F20(ctx, base);
	// 83252FC8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83252FCC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83252FD0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83252FD4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83252FD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83252FD8 size=64
    let mut pc: u32 = 0x83252FD8;
    'dispatch: loop {
        match pc {
            0x83252FD8 => {
    //   block [0x83252FD8..0x83253018)
	// 83252FD8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83252FDC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83252FE0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83252FE4: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 83252FE8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83252FEC: 388B6F48  addi r4, r11, 0x6f48
	ctx.r[4].s64 = ctx.r[11].s64 + 28488;
	// 83252FF0: 386A8230  addi r3, r10, -0x7dd0
	ctx.r[3].s64 = ctx.r[10].s64 + -32208;
	// 83252FF4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83252FF8: 4AFD9ED9  bl 0x8222ced0
	ctx.lr = 0x83252FFC;
	sub_8222CED0(ctx, base);
	// 83252FFC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83253000: 38699C00  addi r3, r9, -0x6400
	ctx.r[3].s64 = ctx.r[9].s64 + -25600;
	// 83253004: 4BA56F1D  bl 0x82ca9f20
	ctx.lr = 0x83253008;
	sub_82CA9F20(ctx, base);
	// 83253008: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325300C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83253010: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83253014: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83253018(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83253018 size=64
    let mut pc: u32 = 0x83253018;
    'dispatch: loop {
        match pc {
            0x83253018 => {
    //   block [0x83253018..0x83253058)
	// 83253018: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325301C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83253020: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83253024: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 83253028: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325302C: 388B6F64  addi r4, r11, 0x6f64
	ctx.r[4].s64 = ctx.r[11].s64 + 28516;
	// 83253030: 386A8234  addi r3, r10, -0x7dcc
	ctx.r[3].s64 = ctx.r[10].s64 + -32204;
	// 83253034: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83253038: 4AFD9E99  bl 0x8222ced0
	ctx.lr = 0x8325303C;
	sub_8222CED0(ctx, base);
	// 8325303C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83253040: 38699C10  addi r3, r9, -0x63f0
	ctx.r[3].s64 = ctx.r[9].s64 + -25584;
	// 83253044: 4BA56EDD  bl 0x82ca9f20
	ctx.lr = 0x83253048;
	sub_82CA9F20(ctx, base);
	// 83253048: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325304C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83253050: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83253054: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83253058(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83253058 size=56
    let mut pc: u32 = 0x83253058;
    'dispatch: loop {
        match pc {
            0x83253058 => {
    //   block [0x83253058..0x83253090)
	// 83253058: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325305C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83253060: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83253064: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83253068: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8325306C: 386B42C8  addi r3, r11, 0x42c8
	ctx.r[3].s64 = ctx.r[11].s64 + 17096;
	// 83253070: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83253074: 4AFA0CE5  bl 0x821f3d58
	ctx.lr = 0x83253078;
	sub_821F3D58(ctx, base);
	// 83253078: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325307C: 906A8238  stw r3, -0x7dc8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-32200 as u32), ctx.r[3].u32 ) };
	// 83253080: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83253084: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83253088: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325308C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83253090(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83253090 size=56
    let mut pc: u32 = 0x83253090;
    'dispatch: loop {
        match pc {
            0x83253090 => {
    //   block [0x83253090..0x832530C8)
	// 83253090: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83253094: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83253098: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325309C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 832530A0: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 832530A4: 386B42DC  addi r3, r11, 0x42dc
	ctx.r[3].s64 = ctx.r[11].s64 + 17116;
	// 832530A8: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 832530AC: 4AFA0CAD  bl 0x821f3d58
	ctx.lr = 0x832530B0;
	sub_821F3D58(ctx, base);
	// 832530B0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832530B4: 906A823C  stw r3, -0x7dc4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-32196 as u32), ctx.r[3].u32 ) };
	// 832530B8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832530BC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832530C0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832530C4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832530C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832530C8 size=56
    let mut pc: u32 = 0x832530C8;
    'dispatch: loop {
        match pc {
            0x832530C8 => {
    //   block [0x832530C8..0x83253100)
	// 832530C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832530CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832530D0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832530D4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 832530D8: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 832530DC: 386B42E8  addi r3, r11, 0x42e8
	ctx.r[3].s64 = ctx.r[11].s64 + 17128;
	// 832530E0: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 832530E4: 4AFA0C75  bl 0x821f3d58
	ctx.lr = 0x832530E8;
	sub_821F3D58(ctx, base);
	// 832530E8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832530EC: 906A8240  stw r3, -0x7dc0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-32192 as u32), ctx.r[3].u32 ) };
	// 832530F0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832530F4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832530F8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832530FC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83253100(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83253100 size=56
    let mut pc: u32 = 0x83253100;
    'dispatch: loop {
        match pc {
            0x83253100 => {
    //   block [0x83253100..0x83253138)
	// 83253100: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83253104: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83253108: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325310C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83253110: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83253114: 386B4300  addi r3, r11, 0x4300
	ctx.r[3].s64 = ctx.r[11].s64 + 17152;
	// 83253118: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8325311C: 4AFA0C3D  bl 0x821f3d58
	ctx.lr = 0x83253120;
	sub_821F3D58(ctx, base);
	// 83253120: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83253124: 906A8244  stw r3, -0x7dbc(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-32188 as u32), ctx.r[3].u32 ) };
	// 83253128: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325312C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83253130: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83253134: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83253138(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83253138 size=56
    let mut pc: u32 = 0x83253138;
    'dispatch: loop {
        match pc {
            0x83253138 => {
    //   block [0x83253138..0x83253170)
	// 83253138: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325313C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83253140: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83253144: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83253148: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8325314C: 386B4310  addi r3, r11, 0x4310
	ctx.r[3].s64 = ctx.r[11].s64 + 17168;
	// 83253150: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83253154: 4AFA0C05  bl 0x821f3d58
	ctx.lr = 0x83253158;
	sub_821F3D58(ctx, base);
	// 83253158: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325315C: 906A8248  stw r3, -0x7db8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-32184 as u32), ctx.r[3].u32 ) };
	// 83253160: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83253164: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83253168: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325316C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83253170(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83253170 size=56
    let mut pc: u32 = 0x83253170;
    'dispatch: loop {
        match pc {
            0x83253170 => {
    //   block [0x83253170..0x832531A8)
	// 83253170: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83253174: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83253178: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325317C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83253180: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83253184: 386B4328  addi r3, r11, 0x4328
	ctx.r[3].s64 = ctx.r[11].s64 + 17192;
	// 83253188: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8325318C: 4AFA0BCD  bl 0x821f3d58
	ctx.lr = 0x83253190;
	sub_821F3D58(ctx, base);
	// 83253190: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83253194: 906A824C  stw r3, -0x7db4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-32180 as u32), ctx.r[3].u32 ) };
	// 83253198: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325319C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832531A0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832531A4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832531A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832531A8 size=56
    let mut pc: u32 = 0x832531A8;
    'dispatch: loop {
        match pc {
            0x832531A8 => {
    //   block [0x832531A8..0x832531E0)
	// 832531A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832531AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832531B0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832531B4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 832531B8: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 832531BC: 386B4338  addi r3, r11, 0x4338
	ctx.r[3].s64 = ctx.r[11].s64 + 17208;
	// 832531C0: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 832531C4: 4AFA0B95  bl 0x821f3d58
	ctx.lr = 0x832531C8;
	sub_821F3D58(ctx, base);
	// 832531C8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832531CC: 906A8250  stw r3, -0x7db0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-32176 as u32), ctx.r[3].u32 ) };
	// 832531D0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832531D4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832531D8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832531DC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832531E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832531E0 size=56
    let mut pc: u32 = 0x832531E0;
    'dispatch: loop {
        match pc {
            0x832531E0 => {
    //   block [0x832531E0..0x83253218)
	// 832531E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832531E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832531E8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832531EC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 832531F0: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 832531F4: 386B4350  addi r3, r11, 0x4350
	ctx.r[3].s64 = ctx.r[11].s64 + 17232;
	// 832531F8: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 832531FC: 4AFA0B5D  bl 0x821f3d58
	ctx.lr = 0x83253200;
	sub_821F3D58(ctx, base);
	// 83253200: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83253204: 906A8254  stw r3, -0x7dac(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-32172 as u32), ctx.r[3].u32 ) };
	// 83253208: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325320C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83253210: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83253214: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83253218(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83253218 size=56
    let mut pc: u32 = 0x83253218;
    'dispatch: loop {
        match pc {
            0x83253218 => {
    //   block [0x83253218..0x83253250)
	// 83253218: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325321C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83253220: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83253224: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83253228: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8325322C: 386B4368  addi r3, r11, 0x4368
	ctx.r[3].s64 = ctx.r[11].s64 + 17256;
	// 83253230: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83253234: 4AFA0B25  bl 0x821f3d58
	ctx.lr = 0x83253238;
	sub_821F3D58(ctx, base);
	// 83253238: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325323C: 906A8258  stw r3, -0x7da8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-32168 as u32), ctx.r[3].u32 ) };
	// 83253240: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83253244: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83253248: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325324C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83253250(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83253250 size=56
    let mut pc: u32 = 0x83253250;
    'dispatch: loop {
        match pc {
            0x83253250 => {
    //   block [0x83253250..0x83253288)
	// 83253250: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83253254: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83253258: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325325C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83253260: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83253264: 386B4380  addi r3, r11, 0x4380
	ctx.r[3].s64 = ctx.r[11].s64 + 17280;
	// 83253268: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8325326C: 4AFA0AED  bl 0x821f3d58
	ctx.lr = 0x83253270;
	sub_821F3D58(ctx, base);
	// 83253270: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83253274: 906A825C  stw r3, -0x7da4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-32164 as u32), ctx.r[3].u32 ) };
	// 83253278: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325327C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83253280: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83253284: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83253288(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83253288 size=56
    let mut pc: u32 = 0x83253288;
    'dispatch: loop {
        match pc {
            0x83253288 => {
    //   block [0x83253288..0x832532C0)
	// 83253288: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325328C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83253290: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83253294: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83253298: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8325329C: 386B4394  addi r3, r11, 0x4394
	ctx.r[3].s64 = ctx.r[11].s64 + 17300;
	// 832532A0: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 832532A4: 4AFA0AB5  bl 0x821f3d58
	ctx.lr = 0x832532A8;
	sub_821F3D58(ctx, base);
	// 832532A8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832532AC: 906A8260  stw r3, -0x7da0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-32160 as u32), ctx.r[3].u32 ) };
	// 832532B0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832532B4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832532B8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832532BC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832532C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832532C0 size=56
    let mut pc: u32 = 0x832532C0;
    'dispatch: loop {
        match pc {
            0x832532C0 => {
    //   block [0x832532C0..0x832532F8)
	// 832532C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832532C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832532C8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832532CC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 832532D0: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 832532D4: 386B43B4  addi r3, r11, 0x43b4
	ctx.r[3].s64 = ctx.r[11].s64 + 17332;
	// 832532D8: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 832532DC: 4AFA0A7D  bl 0x821f3d58
	ctx.lr = 0x832532E0;
	sub_821F3D58(ctx, base);
	// 832532E0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832532E4: 906A8264  stw r3, -0x7d9c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-32156 as u32), ctx.r[3].u32 ) };
	// 832532E8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832532EC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832532F0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832532F4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832532F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832532F8 size=56
    let mut pc: u32 = 0x832532F8;
    'dispatch: loop {
        match pc {
            0x832532F8 => {
    //   block [0x832532F8..0x83253330)
	// 832532F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832532FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83253300: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83253304: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83253308: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8325330C: 386B43C8  addi r3, r11, 0x43c8
	ctx.r[3].s64 = ctx.r[11].s64 + 17352;
	// 83253310: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83253314: 4AFA0A45  bl 0x821f3d58
	ctx.lr = 0x83253318;
	sub_821F3D58(ctx, base);
	// 83253318: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325331C: 906A8268  stw r3, -0x7d98(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-32152 as u32), ctx.r[3].u32 ) };
	// 83253320: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83253324: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83253328: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325332C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83253330(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83253330 size=56
    let mut pc: u32 = 0x83253330;
    'dispatch: loop {
        match pc {
            0x83253330 => {
    //   block [0x83253330..0x83253368)
	// 83253330: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83253334: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83253338: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325333C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83253340: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83253344: 386B43D4  addi r3, r11, 0x43d4
	ctx.r[3].s64 = ctx.r[11].s64 + 17364;
	// 83253348: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8325334C: 4AFA0A0D  bl 0x821f3d58
	ctx.lr = 0x83253350;
	sub_821F3D58(ctx, base);
	// 83253350: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83253354: 906A826C  stw r3, -0x7d94(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-32148 as u32), ctx.r[3].u32 ) };
	// 83253358: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325335C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83253360: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83253364: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83253368(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83253368 size=56
    let mut pc: u32 = 0x83253368;
    'dispatch: loop {
        match pc {
            0x83253368 => {
    //   block [0x83253368..0x832533A0)
	// 83253368: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325336C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83253370: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83253374: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83253378: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8325337C: 386B43E0  addi r3, r11, 0x43e0
	ctx.r[3].s64 = ctx.r[11].s64 + 17376;
	// 83253380: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83253384: 4AFA09D5  bl 0x821f3d58
	ctx.lr = 0x83253388;
	sub_821F3D58(ctx, base);
	// 83253388: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325338C: 906A8270  stw r3, -0x7d90(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-32144 as u32), ctx.r[3].u32 ) };
	// 83253390: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83253394: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83253398: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325339C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832533A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832533A0 size=56
    let mut pc: u32 = 0x832533A0;
    'dispatch: loop {
        match pc {
            0x832533A0 => {
    //   block [0x832533A0..0x832533D8)
	// 832533A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832533A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832533A8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832533AC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 832533B0: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 832533B4: 386B43F0  addi r3, r11, 0x43f0
	ctx.r[3].s64 = ctx.r[11].s64 + 17392;
	// 832533B8: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 832533BC: 4AFA099D  bl 0x821f3d58
	ctx.lr = 0x832533C0;
	sub_821F3D58(ctx, base);
	// 832533C0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832533C4: 906A8274  stw r3, -0x7d8c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-32140 as u32), ctx.r[3].u32 ) };
	// 832533C8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832533CC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832533D0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832533D4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832533D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832533D8 size=56
    let mut pc: u32 = 0x832533D8;
    'dispatch: loop {
        match pc {
            0x832533D8 => {
    //   block [0x832533D8..0x83253410)
	// 832533D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832533DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832533E0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832533E4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 832533E8: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 832533EC: 386B4400  addi r3, r11, 0x4400
	ctx.r[3].s64 = ctx.r[11].s64 + 17408;
	// 832533F0: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 832533F4: 4AFA0965  bl 0x821f3d58
	ctx.lr = 0x832533F8;
	sub_821F3D58(ctx, base);
	// 832533F8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832533FC: 906A8278  stw r3, -0x7d88(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-32136 as u32), ctx.r[3].u32 ) };
	// 83253400: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83253404: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83253408: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325340C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83253410(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83253410 size=56
    let mut pc: u32 = 0x83253410;
    'dispatch: loop {
        match pc {
            0x83253410 => {
    //   block [0x83253410..0x83253448)
	// 83253410: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83253414: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83253418: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325341C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83253420: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83253424: 386B4418  addi r3, r11, 0x4418
	ctx.r[3].s64 = ctx.r[11].s64 + 17432;
	// 83253428: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8325342C: 4AFA092D  bl 0x821f3d58
	ctx.lr = 0x83253430;
	sub_821F3D58(ctx, base);
	// 83253430: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83253434: 906A827C  stw r3, -0x7d84(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-32132 as u32), ctx.r[3].u32 ) };
	// 83253438: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325343C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83253440: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83253444: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83253448(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83253448 size=56
    let mut pc: u32 = 0x83253448;
    'dispatch: loop {
        match pc {
            0x83253448 => {
    //   block [0x83253448..0x83253480)
	// 83253448: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325344C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83253450: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83253454: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83253458: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8325345C: 386B4424  addi r3, r11, 0x4424
	ctx.r[3].s64 = ctx.r[11].s64 + 17444;
	// 83253460: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83253464: 4AFA08F5  bl 0x821f3d58
	ctx.lr = 0x83253468;
	sub_821F3D58(ctx, base);
	// 83253468: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325346C: 906A8280  stw r3, -0x7d80(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-32128 as u32), ctx.r[3].u32 ) };
	// 83253470: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83253474: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83253478: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325347C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83253480(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83253480 size=56
    let mut pc: u32 = 0x83253480;
    'dispatch: loop {
        match pc {
            0x83253480 => {
    //   block [0x83253480..0x832534B8)
	// 83253480: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83253484: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83253488: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325348C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83253490: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 83253494: 386B4430  addi r3, r11, 0x4430
	ctx.r[3].s64 = ctx.r[11].s64 + 17456;
	// 83253498: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 8325349C: 4AFA08BD  bl 0x821f3d58
	ctx.lr = 0x832534A0;
	sub_821F3D58(ctx, base);
	// 832534A0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832534A4: 906A8284  stw r3, -0x7d7c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-32124 as u32), ctx.r[3].u32 ) };
	// 832534A8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832534AC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832534B0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832534B4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832534B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832534B8 size=56
    let mut pc: u32 = 0x832534B8;
    'dispatch: loop {
        match pc {
            0x832534B8 => {
    //   block [0x832534B8..0x832534F0)
	// 832534B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832534BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832534C0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832534C4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 832534C8: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 832534CC: 386B4440  addi r3, r11, 0x4440
	ctx.r[3].s64 = ctx.r[11].s64 + 17472;
	// 832534D0: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 832534D4: 4AFA0885  bl 0x821f3d58
	ctx.lr = 0x832534D8;
	sub_821F3D58(ctx, base);
	// 832534D8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832534DC: 906A8288  stw r3, -0x7d78(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-32120 as u32), ctx.r[3].u32 ) };
	// 832534E0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832534E4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832534E8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832534EC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832534F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832534F0 size=64
    let mut pc: u32 = 0x832534F0;
    'dispatch: loop {
        match pc {
            0x832534F0 => {
    //   block [0x832534F0..0x83253530)
	// 832534F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832534F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832534F8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832534FC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83253500: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83253504: 388B052C  addi r4, r11, 0x52c
	ctx.r[4].s64 = ctx.r[11].s64 + 1324;
	// 83253508: 386A828C  addi r3, r10, -0x7d74
	ctx.r[3].s64 = ctx.r[10].s64 + -32116;
	// 8325350C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83253510: 4AFD99C1  bl 0x8222ced0
	ctx.lr = 0x83253514;
	sub_8222CED0(ctx, base);
	// 83253514: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83253518: 38699C90  addi r3, r9, -0x6370
	ctx.r[3].s64 = ctx.r[9].s64 + -25456;
	// 8325351C: 4BA56A05  bl 0x82ca9f20
	ctx.lr = 0x83253520;
	sub_82CA9F20(ctx, base);
	// 83253520: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83253524: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83253528: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325352C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83253530(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83253530 size=64
    let mut pc: u32 = 0x83253530;
    'dispatch: loop {
        match pc {
            0x83253530 => {
    //   block [0x83253530..0x83253570)
	// 83253530: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83253534: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83253538: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325353C: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 83253540: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83253544: 388B7F40  addi r4, r11, 0x7f40
	ctx.r[4].s64 = ctx.r[11].s64 + 32576;
	// 83253548: 386A8290  addi r3, r10, -0x7d70
	ctx.r[3].s64 = ctx.r[10].s64 + -32112;
	// 8325354C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83253550: 4AFD9981  bl 0x8222ced0
	ctx.lr = 0x83253554;
	sub_8222CED0(ctx, base);
	// 83253554: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83253558: 38699CA0  addi r3, r9, -0x6360
	ctx.r[3].s64 = ctx.r[9].s64 + -25440;
	// 8325355C: 4BA569C5  bl 0x82ca9f20
	ctx.lr = 0x83253560;
	sub_82CA9F20(ctx, base);
	// 83253560: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83253564: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83253568: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325356C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83253570(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83253570 size=64
    let mut pc: u32 = 0x83253570;
    'dispatch: loop {
        match pc {
            0x83253570 => {
    //   block [0x83253570..0x832535B0)
	// 83253570: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83253574: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83253578: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325357C: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 83253580: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83253584: 388B7F78  addi r4, r11, 0x7f78
	ctx.r[4].s64 = ctx.r[11].s64 + 32632;
	// 83253588: 386A8294  addi r3, r10, -0x7d6c
	ctx.r[3].s64 = ctx.r[10].s64 + -32108;
	// 8325358C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83253590: 4AFD9941  bl 0x8222ced0
	ctx.lr = 0x83253594;
	sub_8222CED0(ctx, base);
	// 83253594: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83253598: 38699CB0  addi r3, r9, -0x6350
	ctx.r[3].s64 = ctx.r[9].s64 + -25424;
	// 8325359C: 4BA56985  bl 0x82ca9f20
	ctx.lr = 0x832535A0;
	sub_82CA9F20(ctx, base);
	// 832535A0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832535A4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832535A8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832535AC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832535B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832535B0 size=64
    let mut pc: u32 = 0x832535B0;
    'dispatch: loop {
        match pc {
            0x832535B0 => {
    //   block [0x832535B0..0x832535F0)
	// 832535B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832535B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832535B8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832535BC: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 832535C0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832535C4: 388B7FB8  addi r4, r11, 0x7fb8
	ctx.r[4].s64 = ctx.r[11].s64 + 32696;
	// 832535C8: 386A8298  addi r3, r10, -0x7d68
	ctx.r[3].s64 = ctx.r[10].s64 + -32104;
	// 832535CC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832535D0: 4AFD9901  bl 0x8222ced0
	ctx.lr = 0x832535D4;
	sub_8222CED0(ctx, base);
	// 832535D4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 832535D8: 38699CC0  addi r3, r9, -0x6340
	ctx.r[3].s64 = ctx.r[9].s64 + -25408;
	// 832535DC: 4BA56945  bl 0x82ca9f20
	ctx.lr = 0x832535E0;
	sub_82CA9F20(ctx, base);
	// 832535E0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832535E4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832535E8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832535EC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832535F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832535F0 size=64
    let mut pc: u32 = 0x832535F0;
    'dispatch: loop {
        match pc {
            0x832535F0 => {
    //   block [0x832535F0..0x83253630)
	// 832535F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832535F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832535F8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832535FC: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 83253600: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83253604: 388B7FF0  addi r4, r11, 0x7ff0
	ctx.r[4].s64 = ctx.r[11].s64 + 32752;
	// 83253608: 386A829C  addi r3, r10, -0x7d64
	ctx.r[3].s64 = ctx.r[10].s64 + -32100;
	// 8325360C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83253610: 4AFD98C1  bl 0x8222ced0
	ctx.lr = 0x83253614;
	sub_8222CED0(ctx, base);
	// 83253614: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83253618: 38699CD0  addi r3, r9, -0x6330
	ctx.r[3].s64 = ctx.r[9].s64 + -25392;
	// 8325361C: 4BA56905  bl 0x82ca9f20
	ctx.lr = 0x83253620;
	sub_82CA9F20(ctx, base);
	// 83253620: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83253624: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83253628: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325362C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83253630(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83253630 size=64
    let mut pc: u32 = 0x83253630;
    'dispatch: loop {
        match pc {
            0x83253630 => {
    //   block [0x83253630..0x83253670)
	// 83253630: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83253634: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83253638: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325363C: 3D60820C  lis r11, -0x7df4
	ctx.r[11].s64 = -2113142784;
	// 83253640: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83253644: 388B8028  addi r4, r11, -0x7fd8
	ctx.r[4].s64 = ctx.r[11].s64 + -32728;
	// 83253648: 386A82A0  addi r3, r10, -0x7d60
	ctx.r[3].s64 = ctx.r[10].s64 + -32096;
	// 8325364C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83253650: 4AFD9881  bl 0x8222ced0
	ctx.lr = 0x83253654;
	sub_8222CED0(ctx, base);
	// 83253654: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83253658: 38699CE0  addi r3, r9, -0x6320
	ctx.r[3].s64 = ctx.r[9].s64 + -25376;
	// 8325365C: 4BA568C5  bl 0x82ca9f20
	ctx.lr = 0x83253660;
	sub_82CA9F20(ctx, base);
	// 83253660: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83253664: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83253668: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325366C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83253670(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83253670 size=296
    let mut pc: u32 = 0x83253670;
    'dispatch: loop {
        match pc {
            0x83253670 => {
    //   block [0x83253670..0x83253798)
	// 83253670: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83253674: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83253678: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8325367C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83253680: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 83253684: 3D40820C  lis r10, -0x7df4
	ctx.r[10].s64 = -2113142784;
	// 83253688: 3BEB82A4  addi r31, r11, -0x7d5c
	ctx.r[31].s64 = ctx.r[11].s64 + -32092;
	// 8325368C: 388A8108  addi r4, r10, -0x7ef8
	ctx.r[4].s64 = ctx.r[10].s64 + -32504;
	// 83253690: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83253694: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83253698: 4AFD9839  bl 0x8222ced0
	ctx.lr = 0x8325369C;
	sub_8222CED0(ctx, base);
	// 8325369C: 3D20820C  lis r9, -0x7df4
	ctx.r[9].s64 = -2113142784;
	// 832536A0: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832536A4: 388980F8  addi r4, r9, -0x7f08
	ctx.r[4].s64 = ctx.r[9].s64 + -32520;
	// 832536A8: 387F0004  addi r3, r31, 4
	ctx.r[3].s64 = ctx.r[31].s64 + 4;
	// 832536AC: 4AFD9825  bl 0x8222ced0
	ctx.lr = 0x832536B0;
	sub_8222CED0(ctx, base);
	// 832536B0: 3D00820C  lis r8, -0x7df4
	ctx.r[8].s64 = -2113142784;
	// 832536B4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832536B8: 388880E8  addi r4, r8, -0x7f18
	ctx.r[4].s64 = ctx.r[8].s64 + -32536;
	// 832536BC: 387F0008  addi r3, r31, 8
	ctx.r[3].s64 = ctx.r[31].s64 + 8;
	// 832536C0: 4AFD9811  bl 0x8222ced0
	ctx.lr = 0x832536C4;
	sub_8222CED0(ctx, base);
	// 832536C4: 3CE0820C  lis r7, -0x7df4
	ctx.r[7].s64 = -2113142784;
	// 832536C8: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832536CC: 388780D4  addi r4, r7, -0x7f2c
	ctx.r[4].s64 = ctx.r[7].s64 + -32556;
	// 832536D0: 387F000C  addi r3, r31, 0xc
	ctx.r[3].s64 = ctx.r[31].s64 + 12;
	// 832536D4: 4AFD97FD  bl 0x8222ced0
	ctx.lr = 0x832536D8;
	sub_8222CED0(ctx, base);
	// 832536D8: 3CC0820C  lis r6, -0x7df4
	ctx.r[6].s64 = -2113142784;
	// 832536DC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832536E0: 388680C0  addi r4, r6, -0x7f40
	ctx.r[4].s64 = ctx.r[6].s64 + -32576;
	// 832536E4: 387F0010  addi r3, r31, 0x10
	ctx.r[3].s64 = ctx.r[31].s64 + 16;
	// 832536E8: 4AFD97E9  bl 0x8222ced0
	ctx.lr = 0x832536EC;
	sub_8222CED0(ctx, base);
	// 832536EC: 3C80820C  lis r4, -0x7df4
	ctx.r[4].s64 = -2113142784;
	// 832536F0: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832536F4: 388480AC  addi r4, r4, -0x7f54
	ctx.r[4].s64 = ctx.r[4].s64 + -32596;
	// 832536F8: 387F0014  addi r3, r31, 0x14
	ctx.r[3].s64 = ctx.r[31].s64 + 20;
	// 832536FC: 4AFD97D5  bl 0x8222ced0
	ctx.lr = 0x83253700;
	sub_8222CED0(ctx, base);
	// 83253700: 3C60820C  lis r3, -0x7df4
	ctx.r[3].s64 = -2113142784;
	// 83253704: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83253708: 3883809C  addi r4, r3, -0x7f64
	ctx.r[4].s64 = ctx.r[3].s64 + -32612;
	// 8325370C: 387F0018  addi r3, r31, 0x18
	ctx.r[3].s64 = ctx.r[31].s64 + 24;
	// 83253710: 4AFD97C1  bl 0x8222ced0
	ctx.lr = 0x83253714;
	sub_8222CED0(ctx, base);
	// 83253714: 3D60820C  lis r11, -0x7df4
	ctx.r[11].s64 = -2113142784;
	// 83253718: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8325371C: 388B8088  addi r4, r11, -0x7f78
	ctx.r[4].s64 = ctx.r[11].s64 + -32632;
	// 83253720: 387F001C  addi r3, r31, 0x1c
	ctx.r[3].s64 = ctx.r[31].s64 + 28;
	// 83253724: 4AFD97AD  bl 0x8222ced0
	ctx.lr = 0x83253728;
	sub_8222CED0(ctx, base);
	// 83253728: 3D40820C  lis r10, -0x7df4
	ctx.r[10].s64 = -2113142784;
	// 8325372C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83253730: 388A8078  addi r4, r10, -0x7f88
	ctx.r[4].s64 = ctx.r[10].s64 + -32648;
	// 83253734: 387F0020  addi r3, r31, 0x20
	ctx.r[3].s64 = ctx.r[31].s64 + 32;
	// 83253738: 4AFD9799  bl 0x8222ced0
	ctx.lr = 0x8325373C;
	sub_8222CED0(ctx, base);
	// 8325373C: 3D20820C  lis r9, -0x7df4
	ctx.r[9].s64 = -2113142784;
	// 83253740: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83253744: 3889806C  addi r4, r9, -0x7f94
	ctx.r[4].s64 = ctx.r[9].s64 + -32660;
	// 83253748: 387F0024  addi r3, r31, 0x24
	ctx.r[3].s64 = ctx.r[31].s64 + 36;
	// 8325374C: 4AFD9785  bl 0x8222ced0
	ctx.lr = 0x83253750;
	sub_8222CED0(ctx, base);
	// 83253750: 3D00820C  lis r8, -0x7df4
	ctx.r[8].s64 = -2113142784;
	// 83253754: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83253758: 38888060  addi r4, r8, -0x7fa0
	ctx.r[4].s64 = ctx.r[8].s64 + -32672;
	// 8325375C: 387F0028  addi r3, r31, 0x28
	ctx.r[3].s64 = ctx.r[31].s64 + 40;
	// 83253760: 4AFD9771  bl 0x8222ced0
	ctx.lr = 0x83253764;
	sub_8222CED0(ctx, base);
	// 83253764: 3CE0820B  lis r7, -0x7df5
	ctx.r[7].s64 = -2113208320;
	// 83253768: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8325376C: 3887F10C  addi r4, r7, -0xef4
	ctx.r[4].s64 = ctx.r[7].s64 + -3828;
	// 83253770: 387F002C  addi r3, r31, 0x2c
	ctx.r[3].s64 = ctx.r[31].s64 + 44;
	// 83253774: 4AFD975D  bl 0x8222ced0
	ctx.lr = 0x83253778;
	sub_8222CED0(ctx, base);
	// 83253778: 3CC0832B  lis r6, -0x7cd5
	ctx.r[6].s64 = -2094333952;
	// 8325377C: 38669CF0  addi r3, r6, -0x6310
	ctx.r[3].s64 = ctx.r[6].s64 + -25360;
	// 83253780: 4BA567A1  bl 0x82ca9f20
	ctx.lr = 0x83253784;
	sub_82CA9F20(ctx, base);
	// 83253784: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83253788: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8325378C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83253790: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83253794: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83253798(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83253798 size=296
    let mut pc: u32 = 0x83253798;
    'dispatch: loop {
        match pc {
            0x83253798 => {
    //   block [0x83253798..0x832538C0)
	// 83253798: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325379C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832537A0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 832537A4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832537A8: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832537AC: 3D40820C  lis r10, -0x7df4
	ctx.r[10].s64 = -2113142784;
	// 832537B0: 3BEB82D4  addi r31, r11, -0x7d2c
	ctx.r[31].s64 = ctx.r[11].s64 + -32044;
	// 832537B4: 388A8268  addi r4, r10, -0x7d98
	ctx.r[4].s64 = ctx.r[10].s64 + -32152;
	// 832537B8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 832537BC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832537C0: 4AFD9711  bl 0x8222ced0
	ctx.lr = 0x832537C4;
	sub_8222CED0(ctx, base);
	// 832537C4: 3D20820C  lis r9, -0x7df4
	ctx.r[9].s64 = -2113142784;
	// 832537C8: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832537CC: 3889824C  addi r4, r9, -0x7db4
	ctx.r[4].s64 = ctx.r[9].s64 + -32180;
	// 832537D0: 387F0004  addi r3, r31, 4
	ctx.r[3].s64 = ctx.r[31].s64 + 4;
	// 832537D4: 4AFD96FD  bl 0x8222ced0
	ctx.lr = 0x832537D8;
	sub_8222CED0(ctx, base);
	// 832537D8: 3D00820C  lis r8, -0x7df4
	ctx.r[8].s64 = -2113142784;
	// 832537DC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832537E0: 38888230  addi r4, r8, -0x7dd0
	ctx.r[4].s64 = ctx.r[8].s64 + -32208;
	// 832537E4: 387F0008  addi r3, r31, 8
	ctx.r[3].s64 = ctx.r[31].s64 + 8;
	// 832537E8: 4AFD96E9  bl 0x8222ced0
	ctx.lr = 0x832537EC;
	sub_8222CED0(ctx, base);
	// 832537EC: 3CE0820C  lis r7, -0x7df4
	ctx.r[7].s64 = -2113142784;
	// 832537F0: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832537F4: 38878214  addi r4, r7, -0x7dec
	ctx.r[4].s64 = ctx.r[7].s64 + -32236;
	// 832537F8: 387F000C  addi r3, r31, 0xc
	ctx.r[3].s64 = ctx.r[31].s64 + 12;
	// 832537FC: 4AFD96D5  bl 0x8222ced0
	ctx.lr = 0x83253800;
	sub_8222CED0(ctx, base);
	// 83253800: 3CC0820C  lis r6, -0x7df4
	ctx.r[6].s64 = -2113142784;
	// 83253804: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83253808: 388681F0  addi r4, r6, -0x7e10
	ctx.r[4].s64 = ctx.r[6].s64 + -32272;
	// 8325380C: 387F0010  addi r3, r31, 0x10
	ctx.r[3].s64 = ctx.r[31].s64 + 16;
	// 83253810: 4AFD96C1  bl 0x8222ced0
	ctx.lr = 0x83253814;
	sub_8222CED0(ctx, base);
	// 83253814: 3C80820C  lis r4, -0x7df4
	ctx.r[4].s64 = -2113142784;
	// 83253818: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8325381C: 388481D0  addi r4, r4, -0x7e30
	ctx.r[4].s64 = ctx.r[4].s64 + -32304;
	// 83253820: 387F0014  addi r3, r31, 0x14
	ctx.r[3].s64 = ctx.r[31].s64 + 20;
	// 83253824: 4AFD96AD  bl 0x8222ced0
	ctx.lr = 0x83253828;
	sub_8222CED0(ctx, base);
	// 83253828: 3C60820C  lis r3, -0x7df4
	ctx.r[3].s64 = -2113142784;
	// 8325382C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83253830: 388381B0  addi r4, r3, -0x7e50
	ctx.r[4].s64 = ctx.r[3].s64 + -32336;
	// 83253834: 387F0018  addi r3, r31, 0x18
	ctx.r[3].s64 = ctx.r[31].s64 + 24;
	// 83253838: 4AFD9699  bl 0x8222ced0
	ctx.lr = 0x8325383C;
	sub_8222CED0(ctx, base);
	// 8325383C: 3D60820C  lis r11, -0x7df4
	ctx.r[11].s64 = -2113142784;
	// 83253840: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83253844: 388B8190  addi r4, r11, -0x7e70
	ctx.r[4].s64 = ctx.r[11].s64 + -32368;
	// 83253848: 387F001C  addi r3, r31, 0x1c
	ctx.r[3].s64 = ctx.r[31].s64 + 28;
	// 8325384C: 4AFD9685  bl 0x8222ced0
	ctx.lr = 0x83253850;
	sub_8222CED0(ctx, base);
	// 83253850: 3D40820C  lis r10, -0x7df4
	ctx.r[10].s64 = -2113142784;
	// 83253854: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83253858: 388A8174  addi r4, r10, -0x7e8c
	ctx.r[4].s64 = ctx.r[10].s64 + -32396;
	// 8325385C: 387F0020  addi r3, r31, 0x20
	ctx.r[3].s64 = ctx.r[31].s64 + 32;
	// 83253860: 4AFD9671  bl 0x8222ced0
	ctx.lr = 0x83253864;
	sub_8222CED0(ctx, base);
	// 83253864: 3D20820C  lis r9, -0x7df4
	ctx.r[9].s64 = -2113142784;
	// 83253868: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8325386C: 38898158  addi r4, r9, -0x7ea8
	ctx.r[4].s64 = ctx.r[9].s64 + -32424;
	// 83253870: 387F0024  addi r3, r31, 0x24
	ctx.r[3].s64 = ctx.r[31].s64 + 36;
	// 83253874: 4AFD965D  bl 0x8222ced0
	ctx.lr = 0x83253878;
	sub_8222CED0(ctx, base);
	// 83253878: 3D00820C  lis r8, -0x7df4
	ctx.r[8].s64 = -2113142784;
	// 8325387C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83253880: 3888813C  addi r4, r8, -0x7ec4
	ctx.r[4].s64 = ctx.r[8].s64 + -32452;
	// 83253884: 387F0028  addi r3, r31, 0x28
	ctx.r[3].s64 = ctx.r[31].s64 + 40;
	// 83253888: 4AFD9649  bl 0x8222ced0
	ctx.lr = 0x8325388C;
	sub_8222CED0(ctx, base);
	// 8325388C: 3CE0820C  lis r7, -0x7df4
	ctx.r[7].s64 = -2113142784;
	// 83253890: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83253894: 3887811C  addi r4, r7, -0x7ee4
	ctx.r[4].s64 = ctx.r[7].s64 + -32484;
	// 83253898: 387F002C  addi r3, r31, 0x2c
	ctx.r[3].s64 = ctx.r[31].s64 + 44;
	// 8325389C: 4AFD9635  bl 0x8222ced0
	ctx.lr = 0x832538A0;
	sub_8222CED0(ctx, base);
	// 832538A0: 3CC0832B  lis r6, -0x7cd5
	ctx.r[6].s64 = -2094333952;
	// 832538A4: 38669D58  addi r3, r6, -0x62a8
	ctx.r[3].s64 = ctx.r[6].s64 + -25256;
	// 832538A8: 4BA56679  bl 0x82ca9f20
	ctx.lr = 0x832538AC;
	sub_82CA9F20(ctx, base);
	// 832538AC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832538B0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832538B4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832538B8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 832538BC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832538C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832538C0 size=456
    let mut pc: u32 = 0x832538C0;
    'dispatch: loop {
        match pc {
            0x832538C0 => {
    //   block [0x832538C0..0x83253A88)
	// 832538C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832538C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832538C8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 832538CC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832538D0: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832538D4: 3D40820C  lis r10, -0x7df4
	ctx.r[10].s64 = -2113142784;
	// 832538D8: 3BEB8318  addi r31, r11, -0x7ce8
	ctx.r[31].s64 = ctx.r[11].s64 + -31976;
	// 832538DC: 388A8540  addi r4, r10, -0x7ac0
	ctx.r[4].s64 = ctx.r[10].s64 + -31424;
	// 832538E0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 832538E4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832538E8: 4AFD95E9  bl 0x8222ced0
	ctx.lr = 0x832538EC;
	sub_8222CED0(ctx, base);
	// 832538EC: 3D20820C  lis r9, -0x7df4
	ctx.r[9].s64 = -2113142784;
	// 832538F0: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832538F4: 38898520  addi r4, r9, -0x7ae0
	ctx.r[4].s64 = ctx.r[9].s64 + -31456;
	// 832538F8: 387F0004  addi r3, r31, 4
	ctx.r[3].s64 = ctx.r[31].s64 + 4;
	// 832538FC: 4AFD95D5  bl 0x8222ced0
	ctx.lr = 0x83253900;
	sub_8222CED0(ctx, base);
	// 83253900: 3D00820C  lis r8, -0x7df4
	ctx.r[8].s64 = -2113142784;
	// 83253904: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83253908: 38888500  addi r4, r8, -0x7b00
	ctx.r[4].s64 = ctx.r[8].s64 + -31488;
	// 8325390C: 387F0008  addi r3, r31, 8
	ctx.r[3].s64 = ctx.r[31].s64 + 8;
	// 83253910: 4AFD95C1  bl 0x8222ced0
	ctx.lr = 0x83253914;
	sub_8222CED0(ctx, base);
	// 83253914: 3CE0820C  lis r7, -0x7df4
	ctx.r[7].s64 = -2113142784;
	// 83253918: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8325391C: 388784E0  addi r4, r7, -0x7b20
	ctx.r[4].s64 = ctx.r[7].s64 + -31520;
	// 83253920: 387F000C  addi r3, r31, 0xc
	ctx.r[3].s64 = ctx.r[31].s64 + 12;
	// 83253924: 4AFD95AD  bl 0x8222ced0
	ctx.lr = 0x83253928;
	sub_8222CED0(ctx, base);
	// 83253928: 3CC0820C  lis r6, -0x7df4
	ctx.r[6].s64 = -2113142784;
	// 8325392C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83253930: 388684C0  addi r4, r6, -0x7b40
	ctx.r[4].s64 = ctx.r[6].s64 + -31552;
	// 83253934: 387F0010  addi r3, r31, 0x10
	ctx.r[3].s64 = ctx.r[31].s64 + 16;
	// 83253938: 4AFD9599  bl 0x8222ced0
	ctx.lr = 0x8325393C;
	sub_8222CED0(ctx, base);
	// 8325393C: 3C80820C  lis r4, -0x7df4
	ctx.r[4].s64 = -2113142784;
	// 83253940: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83253944: 388484A0  addi r4, r4, -0x7b60
	ctx.r[4].s64 = ctx.r[4].s64 + -31584;
	// 83253948: 387F0014  addi r3, r31, 0x14
	ctx.r[3].s64 = ctx.r[31].s64 + 20;
	// 8325394C: 4AFD9585  bl 0x8222ced0
	ctx.lr = 0x83253950;
	sub_8222CED0(ctx, base);
	// 83253950: 3C60820C  lis r3, -0x7df4
	ctx.r[3].s64 = -2113142784;
	// 83253954: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83253958: 38838480  addi r4, r3, -0x7b80
	ctx.r[4].s64 = ctx.r[3].s64 + -31616;
	// 8325395C: 387F0018  addi r3, r31, 0x18
	ctx.r[3].s64 = ctx.r[31].s64 + 24;
	// 83253960: 4AFD9571  bl 0x8222ced0
	ctx.lr = 0x83253964;
	sub_8222CED0(ctx, base);
	// 83253964: 3D60820C  lis r11, -0x7df4
	ctx.r[11].s64 = -2113142784;
	// 83253968: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8325396C: 388B8460  addi r4, r11, -0x7ba0
	ctx.r[4].s64 = ctx.r[11].s64 + -31648;
	// 83253970: 387F001C  addi r3, r31, 0x1c
	ctx.r[3].s64 = ctx.r[31].s64 + 28;
	// 83253974: 4AFD955D  bl 0x8222ced0
	ctx.lr = 0x83253978;
	sub_8222CED0(ctx, base);
	// 83253978: 3D40820C  lis r10, -0x7df4
	ctx.r[10].s64 = -2113142784;
	// 8325397C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83253980: 388A843C  addi r4, r10, -0x7bc4
	ctx.r[4].s64 = ctx.r[10].s64 + -31684;
	// 83253984: 387F0020  addi r3, r31, 0x20
	ctx.r[3].s64 = ctx.r[31].s64 + 32;
	// 83253988: 4AFD9549  bl 0x8222ced0
	ctx.lr = 0x8325398C;
	sub_8222CED0(ctx, base);
	// 8325398C: 3D20820C  lis r9, -0x7df4
	ctx.r[9].s64 = -2113142784;
	// 83253990: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83253994: 38898418  addi r4, r9, -0x7be8
	ctx.r[4].s64 = ctx.r[9].s64 + -31720;
	// 83253998: 387F0024  addi r3, r31, 0x24
	ctx.r[3].s64 = ctx.r[31].s64 + 36;
	// 8325399C: 4AFD9535  bl 0x8222ced0
	ctx.lr = 0x832539A0;
	sub_8222CED0(ctx, base);
	// 832539A0: 3D00820C  lis r8, -0x7df4
	ctx.r[8].s64 = -2113142784;
	// 832539A4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832539A8: 388883F8  addi r4, r8, -0x7c08
	ctx.r[4].s64 = ctx.r[8].s64 + -31752;
	// 832539AC: 387F0028  addi r3, r31, 0x28
	ctx.r[3].s64 = ctx.r[31].s64 + 40;
	// 832539B0: 4AFD9521  bl 0x8222ced0
	ctx.lr = 0x832539B4;
	sub_8222CED0(ctx, base);
	// 832539B4: 3CE0820C  lis r7, -0x7df4
	ctx.r[7].s64 = -2113142784;
	// 832539B8: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832539BC: 388783D8  addi r4, r7, -0x7c28
	ctx.r[4].s64 = ctx.r[7].s64 + -31784;
	// 832539C0: 387F002C  addi r3, r31, 0x2c
	ctx.r[3].s64 = ctx.r[31].s64 + 44;
	// 832539C4: 4AFD950D  bl 0x8222ced0
	ctx.lr = 0x832539C8;
	sub_8222CED0(ctx, base);
	// 832539C8: 3CC0820C  lis r6, -0x7df4
	ctx.r[6].s64 = -2113142784;
	// 832539CC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832539D0: 388683AC  addi r4, r6, -0x7c54
	ctx.r[4].s64 = ctx.r[6].s64 + -31828;
	// 832539D4: 387F0030  addi r3, r31, 0x30
	ctx.r[3].s64 = ctx.r[31].s64 + 48;
	// 832539D8: 4AFD94F9  bl 0x8222ced0
	ctx.lr = 0x832539DC;
	sub_8222CED0(ctx, base);
	// 832539DC: 3C80820C  lis r4, -0x7df4
	ctx.r[4].s64 = -2113142784;
	// 832539E0: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832539E4: 38848384  addi r4, r4, -0x7c7c
	ctx.r[4].s64 = ctx.r[4].s64 + -31868;
	// 832539E8: 387F0034  addi r3, r31, 0x34
	ctx.r[3].s64 = ctx.r[31].s64 + 52;
	// 832539EC: 4AFD94E5  bl 0x8222ced0
	ctx.lr = 0x832539F0;
	sub_8222CED0(ctx, base);
	// 832539F0: 3C60820C  lis r3, -0x7df4
	ctx.r[3].s64 = -2113142784;
	// 832539F4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832539F8: 38838358  addi r4, r3, -0x7ca8
	ctx.r[4].s64 = ctx.r[3].s64 + -31912;
	// 832539FC: 387F0038  addi r3, r31, 0x38
	ctx.r[3].s64 = ctx.r[31].s64 + 56;
	// 83253A00: 4AFD94D1  bl 0x8222ced0
	ctx.lr = 0x83253A04;
	sub_8222CED0(ctx, base);
	// 83253A04: 3D60820C  lis r11, -0x7df4
	ctx.r[11].s64 = -2113142784;
	// 83253A08: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83253A0C: 388B8330  addi r4, r11, -0x7cd0
	ctx.r[4].s64 = ctx.r[11].s64 + -31952;
	// 83253A10: 387F003C  addi r3, r31, 0x3c
	ctx.r[3].s64 = ctx.r[31].s64 + 60;
	// 83253A14: 4AFD94BD  bl 0x8222ced0
	ctx.lr = 0x83253A18;
	sub_8222CED0(ctx, base);
	// 83253A18: 3D40820C  lis r10, -0x7df4
	ctx.r[10].s64 = -2113142784;
	// 83253A1C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83253A20: 388A8308  addi r4, r10, -0x7cf8
	ctx.r[4].s64 = ctx.r[10].s64 + -31992;
	// 83253A24: 387F0040  addi r3, r31, 0x40
	ctx.r[3].s64 = ctx.r[31].s64 + 64;
	// 83253A28: 4AFD94A9  bl 0x8222ced0
	ctx.lr = 0x83253A2C;
	sub_8222CED0(ctx, base);
	// 83253A2C: 3D20820C  lis r9, -0x7df4
	ctx.r[9].s64 = -2113142784;
	// 83253A30: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83253A34: 388982E0  addi r4, r9, -0x7d20
	ctx.r[4].s64 = ctx.r[9].s64 + -32032;
	// 83253A38: 387F0044  addi r3, r31, 0x44
	ctx.r[3].s64 = ctx.r[31].s64 + 68;
	// 83253A3C: 4AFD9495  bl 0x8222ced0
	ctx.lr = 0x83253A40;
	sub_8222CED0(ctx, base);
	// 83253A40: 3D00820C  lis r8, -0x7df4
	ctx.r[8].s64 = -2113142784;
	// 83253A44: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83253A48: 388882B4  addi r4, r8, -0x7d4c
	ctx.r[4].s64 = ctx.r[8].s64 + -32076;
	// 83253A4C: 387F0048  addi r3, r31, 0x48
	ctx.r[3].s64 = ctx.r[31].s64 + 72;
	// 83253A50: 4AFD9481  bl 0x8222ced0
	ctx.lr = 0x83253A54;
	sub_8222CED0(ctx, base);
	// 83253A54: 3CE0820C  lis r7, -0x7df4
	ctx.r[7].s64 = -2113142784;
	// 83253A58: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83253A5C: 38878288  addi r4, r7, -0x7d78
	ctx.r[4].s64 = ctx.r[7].s64 + -32120;
	// 83253A60: 387F004C  addi r3, r31, 0x4c
	ctx.r[3].s64 = ctx.r[31].s64 + 76;
	// 83253A64: 4AFD946D  bl 0x8222ced0
	ctx.lr = 0x83253A68;
	sub_8222CED0(ctx, base);
	// 83253A68: 3CC0832B  lis r6, -0x7cd5
	ctx.r[6].s64 = -2094333952;
	// 83253A6C: 38669DC0  addi r3, r6, -0x6240
	ctx.r[3].s64 = ctx.r[6].s64 + -25152;
	// 83253A70: 4BA564B1  bl 0x82ca9f20
	ctx.lr = 0x83253A74;
	sub_82CA9F20(ctx, base);
	// 83253A74: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83253A78: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83253A7C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83253A80: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83253A84: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83253A88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83253A88 size=476
    let mut pc: u32 = 0x83253A88;
    'dispatch: loop {
        match pc {
            0x83253A88 => {
    //   block [0x83253A88..0x83253C64)
	// 83253A88: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83253A8C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83253A90: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83253A94: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83253A98: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 83253A9C: 3D40820B  lis r10, -0x7df5
	ctx.r[10].s64 = -2113208320;
	// 83253AA0: 3BEB8368  addi r31, r11, -0x7c98
	ctx.r[31].s64 = ctx.r[11].s64 + -31896;
	// 83253AA4: 388A25D4  addi r4, r10, 0x25d4
	ctx.r[4].s64 = ctx.r[10].s64 + 9684;
	// 83253AA8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83253AAC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83253AB0: 4AFD9421  bl 0x8222ced0
	ctx.lr = 0x83253AB4;
	sub_8222CED0(ctx, base);
	// 83253AB4: 3D20820C  lis r9, -0x7df4
	ctx.r[9].s64 = -2113142784;
	// 83253AB8: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83253ABC: 38898644  addi r4, r9, -0x79bc
	ctx.r[4].s64 = ctx.r[9].s64 + -31164;
	// 83253AC0: 387F0004  addi r3, r31, 4
	ctx.r[3].s64 = ctx.r[31].s64 + 4;
	// 83253AC4: 4AFD940D  bl 0x8222ced0
	ctx.lr = 0x83253AC8;
	sub_8222CED0(ctx, base);
	// 83253AC8: 3D00820C  lis r8, -0x7df4
	ctx.r[8].s64 = -2113142784;
	// 83253ACC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83253AD0: 38888634  addi r4, r8, -0x79cc
	ctx.r[4].s64 = ctx.r[8].s64 + -31180;
	// 83253AD4: 387F0008  addi r3, r31, 8
	ctx.r[3].s64 = ctx.r[31].s64 + 8;
	// 83253AD8: 4AFD93F9  bl 0x8222ced0
	ctx.lr = 0x83253ADC;
	sub_8222CED0(ctx, base);
	// 83253ADC: 3CE0820C  lis r7, -0x7df4
	ctx.r[7].s64 = -2113142784;
	// 83253AE0: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83253AE4: 38878628  addi r4, r7, -0x79d8
	ctx.r[4].s64 = ctx.r[7].s64 + -31192;
	// 83253AE8: 387F000C  addi r3, r31, 0xc
	ctx.r[3].s64 = ctx.r[31].s64 + 12;
	// 83253AEC: 4AFD93E5  bl 0x8222ced0
	ctx.lr = 0x83253AF0;
	sub_8222CED0(ctx, base);
	// 83253AF0: 3CC0820C  lis r6, -0x7df4
	ctx.r[6].s64 = -2113142784;
	// 83253AF4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83253AF8: 38868618  addi r4, r6, -0x79e8
	ctx.r[4].s64 = ctx.r[6].s64 + -31208;
	// 83253AFC: 387F0010  addi r3, r31, 0x10
	ctx.r[3].s64 = ctx.r[31].s64 + 16;
	// 83253B00: 4AFD93D1  bl 0x8222ced0
	ctx.lr = 0x83253B04;
	sub_8222CED0(ctx, base);
	// 83253B04: 3C80820C  lis r4, -0x7df4
	ctx.r[4].s64 = -2113142784;
	// 83253B08: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83253B0C: 38848608  addi r4, r4, -0x79f8
	ctx.r[4].s64 = ctx.r[4].s64 + -31224;
	// 83253B10: 387F0014  addi r3, r31, 0x14
	ctx.r[3].s64 = ctx.r[31].s64 + 20;
	// 83253B14: 4AFD93BD  bl 0x8222ced0
	ctx.lr = 0x83253B18;
	sub_8222CED0(ctx, base);
	// 83253B18: 3C60820C  lis r3, -0x7df4
	ctx.r[3].s64 = -2113142784;
	// 83253B1C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83253B20: 38838600  addi r4, r3, -0x7a00
	ctx.r[4].s64 = ctx.r[3].s64 + -31232;
	// 83253B24: 387F0018  addi r3, r31, 0x18
	ctx.r[3].s64 = ctx.r[31].s64 + 24;
	// 83253B28: 4AFD93A9  bl 0x8222ced0
	ctx.lr = 0x83253B2C;
	sub_8222CED0(ctx, base);
	// 83253B2C: 3D60820C  lis r11, -0x7df4
	ctx.r[11].s64 = -2113142784;
	// 83253B30: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83253B34: 388B85F0  addi r4, r11, -0x7a10
	ctx.r[4].s64 = ctx.r[11].s64 + -31248;
	// 83253B38: 387F001C  addi r3, r31, 0x1c
	ctx.r[3].s64 = ctx.r[31].s64 + 28;
	// 83253B3C: 4AFD9395  bl 0x8222ced0
	ctx.lr = 0x83253B40;
	sub_8222CED0(ctx, base);
	// 83253B40: 3D40820B  lis r10, -0x7df5
	ctx.r[10].s64 = -2113208320;
	// 83253B44: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83253B48: 388A25E4  addi r4, r10, 0x25e4
	ctx.r[4].s64 = ctx.r[10].s64 + 9700;
	// 83253B4C: 387F0020  addi r3, r31, 0x20
	ctx.r[3].s64 = ctx.r[31].s64 + 32;
	// 83253B50: 4AFD9381  bl 0x8222ced0
	ctx.lr = 0x83253B54;
	sub_8222CED0(ctx, base);
	// 83253B54: 3D20820C  lis r9, -0x7df4
	ctx.r[9].s64 = -2113142784;
	// 83253B58: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83253B5C: 388985E4  addi r4, r9, -0x7a1c
	ctx.r[4].s64 = ctx.r[9].s64 + -31260;
	// 83253B60: 387F0024  addi r3, r31, 0x24
	ctx.r[3].s64 = ctx.r[31].s64 + 36;
	// 83253B64: 4AFD936D  bl 0x8222ced0
	ctx.lr = 0x83253B68;
	sub_8222CED0(ctx, base);
	// 83253B68: 3D00820C  lis r8, -0x7df4
	ctx.r[8].s64 = -2113142784;
	// 83253B6C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83253B70: 388885D8  addi r4, r8, -0x7a28
	ctx.r[4].s64 = ctx.r[8].s64 + -31272;
	// 83253B74: 387F0028  addi r3, r31, 0x28
	ctx.r[3].s64 = ctx.r[31].s64 + 40;
	// 83253B78: 4AFD9359  bl 0x8222ced0
	ctx.lr = 0x83253B7C;
	sub_8222CED0(ctx, base);
	// 83253B7C: 3CE0820C  lis r7, -0x7df4
	ctx.r[7].s64 = -2113142784;
	// 83253B80: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83253B84: 388785CC  addi r4, r7, -0x7a34
	ctx.r[4].s64 = ctx.r[7].s64 + -31284;
	// 83253B88: 387F002C  addi r3, r31, 0x2c
	ctx.r[3].s64 = ctx.r[31].s64 + 44;
	// 83253B8C: 4AFD9345  bl 0x8222ced0
	ctx.lr = 0x83253B90;
	sub_8222CED0(ctx, base);
	// 83253B90: 3CC0820B  lis r6, -0x7df5
	ctx.r[6].s64 = -2113208320;
	// 83253B94: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83253B98: 38863FD8  addi r4, r6, 0x3fd8
	ctx.r[4].s64 = ctx.r[6].s64 + 16344;
	// 83253B9C: 387F0030  addi r3, r31, 0x30
	ctx.r[3].s64 = ctx.r[31].s64 + 48;
	// 83253BA0: 4AFD9331  bl 0x8222ced0
	ctx.lr = 0x83253BA4;
	sub_8222CED0(ctx, base);
	// 83253BA4: 3C80820B  lis r4, -0x7df5
	ctx.r[4].s64 = -2113208320;
	// 83253BA8: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83253BAC: 388426C0  addi r4, r4, 0x26c0
	ctx.r[4].s64 = ctx.r[4].s64 + 9920;
	// 83253BB0: 387F0034  addi r3, r31, 0x34
	ctx.r[3].s64 = ctx.r[31].s64 + 52;
	// 83253BB4: 4AFD931D  bl 0x8222ced0
	ctx.lr = 0x83253BB8;
	sub_8222CED0(ctx, base);
	// 83253BB8: 3C60820C  lis r3, -0x7df4
	ctx.r[3].s64 = -2113142784;
	// 83253BBC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83253BC0: 388385BC  addi r4, r3, -0x7a44
	ctx.r[4].s64 = ctx.r[3].s64 + -31300;
	// 83253BC4: 387F0038  addi r3, r31, 0x38
	ctx.r[3].s64 = ctx.r[31].s64 + 56;
	// 83253BC8: 4AFD9309  bl 0x8222ced0
	ctx.lr = 0x83253BCC;
	sub_8222CED0(ctx, base);
	// 83253BCC: 3D60820C  lis r11, -0x7df4
	ctx.r[11].s64 = -2113142784;
	// 83253BD0: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83253BD4: 388B85AC  addi r4, r11, -0x7a54
	ctx.r[4].s64 = ctx.r[11].s64 + -31316;
	// 83253BD8: 387F003C  addi r3, r31, 0x3c
	ctx.r[3].s64 = ctx.r[31].s64 + 60;
	// 83253BDC: 4AFD92F5  bl 0x8222ced0
	ctx.lr = 0x83253BE0;
	sub_8222CED0(ctx, base);
	// 83253BE0: 3D40820C  lis r10, -0x7df4
	ctx.r[10].s64 = -2113142784;
	// 83253BE4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83253BE8: 388A859C  addi r4, r10, -0x7a64
	ctx.r[4].s64 = ctx.r[10].s64 + -31332;
	// 83253BEC: 387F0040  addi r3, r31, 0x40
	ctx.r[3].s64 = ctx.r[31].s64 + 64;
	// 83253BF0: 4AFD92E1  bl 0x8222ced0
	ctx.lr = 0x83253BF4;
	sub_8222CED0(ctx, base);
	// 83253BF4: 3D20820C  lis r9, -0x7df4
	ctx.r[9].s64 = -2113142784;
	// 83253BF8: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83253BFC: 3889858C  addi r4, r9, -0x7a74
	ctx.r[4].s64 = ctx.r[9].s64 + -31348;
	// 83253C00: 387F0044  addi r3, r31, 0x44
	ctx.r[3].s64 = ctx.r[31].s64 + 68;
	// 83253C04: 4AFD92CD  bl 0x8222ced0
	ctx.lr = 0x83253C08;
	sub_8222CED0(ctx, base);
	// 83253C08: 3D00820C  lis r8, -0x7df4
	ctx.r[8].s64 = -2113142784;
	// 83253C0C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83253C10: 38888580  addi r4, r8, -0x7a80
	ctx.r[4].s64 = ctx.r[8].s64 + -31360;
	// 83253C14: 387F0048  addi r3, r31, 0x48
	ctx.r[3].s64 = ctx.r[31].s64 + 72;
	// 83253C18: 4AFD92B9  bl 0x8222ced0
	ctx.lr = 0x83253C1C;
	sub_8222CED0(ctx, base);
	// 83253C1C: 3CE0820C  lis r7, -0x7df4
	ctx.r[7].s64 = -2113142784;
	// 83253C20: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83253C24: 38878574  addi r4, r7, -0x7a8c
	ctx.r[4].s64 = ctx.r[7].s64 + -31372;
	// 83253C28: 387F004C  addi r3, r31, 0x4c
	ctx.r[3].s64 = ctx.r[31].s64 + 76;
	// 83253C2C: 4AFD92A5  bl 0x8222ced0
	ctx.lr = 0x83253C30;
	sub_8222CED0(ctx, base);
	// 83253C30: 3CC0820C  lis r6, -0x7df4
	ctx.r[6].s64 = -2113142784;
	// 83253C34: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83253C38: 38868564  addi r4, r6, -0x7a9c
	ctx.r[4].s64 = ctx.r[6].s64 + -31388;
	// 83253C3C: 387F0050  addi r3, r31, 0x50
	ctx.r[3].s64 = ctx.r[31].s64 + 80;
	// 83253C40: 4AFD9291  bl 0x8222ced0
	ctx.lr = 0x83253C44;
	sub_8222CED0(ctx, base);
	// 83253C44: 3CA0832B  lis r5, -0x7cd5
	ctx.r[5].s64 = -2094333952;
	// 83253C48: 38659E28  addi r3, r5, -0x61d8
	ctx.r[3].s64 = ctx.r[5].s64 + -25048;
	// 83253C4C: 4BA562D5  bl 0x82ca9f20
	ctx.lr = 0x83253C50;
	sub_82CA9F20(ctx, base);
	// 83253C50: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83253C54: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83253C58: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83253C5C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83253C60: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83253C68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83253C68 size=476
    let mut pc: u32 = 0x83253C68;
    'dispatch: loop {
        match pc {
            0x83253C68 => {
    //   block [0x83253C68..0x83253E44)
	// 83253C68: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83253C6C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83253C70: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83253C74: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83253C78: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 83253C7C: 3D40820C  lis r10, -0x7df4
	ctx.r[10].s64 = -2113142784;
	// 83253C80: 3BEB83D0  addi r31, r11, -0x7c30
	ctx.r[31].s64 = ctx.r[11].s64 + -31792;
	// 83253C84: 388A8900  addi r4, r10, -0x7700
	ctx.r[4].s64 = ctx.r[10].s64 + -30464;
	// 83253C88: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83253C8C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83253C90: 4AFD9241  bl 0x8222ced0
	ctx.lr = 0x83253C94;
	sub_8222CED0(ctx, base);
	// 83253C94: 3D20820C  lis r9, -0x7df4
	ctx.r[9].s64 = -2113142784;
	// 83253C98: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83253C9C: 388988E0  addi r4, r9, -0x7720
	ctx.r[4].s64 = ctx.r[9].s64 + -30496;
	// 83253CA0: 387F0004  addi r3, r31, 4
	ctx.r[3].s64 = ctx.r[31].s64 + 4;
	// 83253CA4: 4AFD922D  bl 0x8222ced0
	ctx.lr = 0x83253CA8;
	sub_8222CED0(ctx, base);
	// 83253CA8: 3D00820C  lis r8, -0x7df4
	ctx.r[8].s64 = -2113142784;
	// 83253CAC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83253CB0: 388888BC  addi r4, r8, -0x7744
	ctx.r[4].s64 = ctx.r[8].s64 + -30532;
	// 83253CB4: 387F0008  addi r3, r31, 8
	ctx.r[3].s64 = ctx.r[31].s64 + 8;
	// 83253CB8: 4AFD9219  bl 0x8222ced0
	ctx.lr = 0x83253CBC;
	sub_8222CED0(ctx, base);
	// 83253CBC: 3CE0820C  lis r7, -0x7df4
	ctx.r[7].s64 = -2113142784;
	// 83253CC0: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83253CC4: 38878898  addi r4, r7, -0x7768
	ctx.r[4].s64 = ctx.r[7].s64 + -30568;
	// 83253CC8: 387F000C  addi r3, r31, 0xc
	ctx.r[3].s64 = ctx.r[31].s64 + 12;
	// 83253CCC: 4AFD9205  bl 0x8222ced0
	ctx.lr = 0x83253CD0;
	sub_8222CED0(ctx, base);
	// 83253CD0: 3CC0820C  lis r6, -0x7df4
	ctx.r[6].s64 = -2113142784;
	// 83253CD4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83253CD8: 38868870  addi r4, r6, -0x7790
	ctx.r[4].s64 = ctx.r[6].s64 + -30608;
	// 83253CDC: 387F0010  addi r3, r31, 0x10
	ctx.r[3].s64 = ctx.r[31].s64 + 16;
	// 83253CE0: 4AFD91F1  bl 0x8222ced0
	ctx.lr = 0x83253CE4;
	sub_8222CED0(ctx, base);
	// 83253CE4: 3C80820C  lis r4, -0x7df4
	ctx.r[4].s64 = -2113142784;
	// 83253CE8: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83253CEC: 3884884C  addi r4, r4, -0x77b4
	ctx.r[4].s64 = ctx.r[4].s64 + -30644;
	// 83253CF0: 387F0014  addi r3, r31, 0x14
	ctx.r[3].s64 = ctx.r[31].s64 + 20;
	// 83253CF4: 4AFD91DD  bl 0x8222ced0
	ctx.lr = 0x83253CF8;
	sub_8222CED0(ctx, base);
	// 83253CF8: 3C60820C  lis r3, -0x7df4
	ctx.r[3].s64 = -2113142784;
	// 83253CFC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83253D00: 3883882C  addi r4, r3, -0x77d4
	ctx.r[4].s64 = ctx.r[3].s64 + -30676;
	// 83253D04: 387F0018  addi r3, r31, 0x18
	ctx.r[3].s64 = ctx.r[31].s64 + 24;
	// 83253D08: 4AFD91C9  bl 0x8222ced0
	ctx.lr = 0x83253D0C;
	sub_8222CED0(ctx, base);
	// 83253D0C: 3D60820C  lis r11, -0x7df4
	ctx.r[11].s64 = -2113142784;
	// 83253D10: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83253D14: 388B8808  addi r4, r11, -0x77f8
	ctx.r[4].s64 = ctx.r[11].s64 + -30712;
	// 83253D18: 387F001C  addi r3, r31, 0x1c
	ctx.r[3].s64 = ctx.r[31].s64 + 28;
	// 83253D1C: 4AFD91B5  bl 0x8222ced0
	ctx.lr = 0x83253D20;
	sub_8222CED0(ctx, base);
	// 83253D20: 3D40820C  lis r10, -0x7df4
	ctx.r[10].s64 = -2113142784;
	// 83253D24: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83253D28: 388A87E8  addi r4, r10, -0x7818
	ctx.r[4].s64 = ctx.r[10].s64 + -30744;
	// 83253D2C: 387F0020  addi r3, r31, 0x20
	ctx.r[3].s64 = ctx.r[31].s64 + 32;
	// 83253D30: 4AFD91A1  bl 0x8222ced0
	ctx.lr = 0x83253D34;
	sub_8222CED0(ctx, base);
	// 83253D34: 3D20820C  lis r9, -0x7df4
	ctx.r[9].s64 = -2113142784;
	// 83253D38: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83253D3C: 388987C8  addi r4, r9, -0x7838
	ctx.r[4].s64 = ctx.r[9].s64 + -30776;
	// 83253D40: 387F0024  addi r3, r31, 0x24
	ctx.r[3].s64 = ctx.r[31].s64 + 36;
	// 83253D44: 4AFD918D  bl 0x8222ced0
	ctx.lr = 0x83253D48;
	sub_8222CED0(ctx, base);
	// 83253D48: 3D00820C  lis r8, -0x7df4
	ctx.r[8].s64 = -2113142784;
	// 83253D4C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83253D50: 388887A8  addi r4, r8, -0x7858
	ctx.r[4].s64 = ctx.r[8].s64 + -30808;
	// 83253D54: 387F0028  addi r3, r31, 0x28
	ctx.r[3].s64 = ctx.r[31].s64 + 40;
	// 83253D58: 4AFD9179  bl 0x8222ced0
	ctx.lr = 0x83253D5C;
	sub_8222CED0(ctx, base);
	// 83253D5C: 3CE0820C  lis r7, -0x7df4
	ctx.r[7].s64 = -2113142784;
	// 83253D60: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83253D64: 38878784  addi r4, r7, -0x787c
	ctx.r[4].s64 = ctx.r[7].s64 + -30844;
	// 83253D68: 387F002C  addi r3, r31, 0x2c
	ctx.r[3].s64 = ctx.r[31].s64 + 44;
	// 83253D6C: 4AFD9165  bl 0x8222ced0
	ctx.lr = 0x83253D70;
	sub_8222CED0(ctx, base);
	// 83253D70: 3CC0820C  lis r6, -0x7df4
	ctx.r[6].s64 = -2113142784;
	// 83253D74: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83253D78: 38868764  addi r4, r6, -0x789c
	ctx.r[4].s64 = ctx.r[6].s64 + -30876;
	// 83253D7C: 387F0030  addi r3, r31, 0x30
	ctx.r[3].s64 = ctx.r[31].s64 + 48;
	// 83253D80: 4AFD9151  bl 0x8222ced0
	ctx.lr = 0x83253D84;
	sub_8222CED0(ctx, base);
	// 83253D84: 3C80820C  lis r4, -0x7df4
	ctx.r[4].s64 = -2113142784;
	// 83253D88: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83253D8C: 38848748  addi r4, r4, -0x78b8
	ctx.r[4].s64 = ctx.r[4].s64 + -30904;
	// 83253D90: 387F0034  addi r3, r31, 0x34
	ctx.r[3].s64 = ctx.r[31].s64 + 52;
	// 83253D94: 4AFD913D  bl 0x8222ced0
	ctx.lr = 0x83253D98;
	sub_8222CED0(ctx, base);
	// 83253D98: 3C60820C  lis r3, -0x7df4
	ctx.r[3].s64 = -2113142784;
	// 83253D9C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83253DA0: 38838720  addi r4, r3, -0x78e0
	ctx.r[4].s64 = ctx.r[3].s64 + -30944;
	// 83253DA4: 387F0038  addi r3, r31, 0x38
	ctx.r[3].s64 = ctx.r[31].s64 + 56;
	// 83253DA8: 4AFD9129  bl 0x8222ced0
	ctx.lr = 0x83253DAC;
	sub_8222CED0(ctx, base);
	// 83253DAC: 3D60820C  lis r11, -0x7df4
	ctx.r[11].s64 = -2113142784;
	// 83253DB0: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83253DB4: 388B86FC  addi r4, r11, -0x7904
	ctx.r[4].s64 = ctx.r[11].s64 + -30980;
	// 83253DB8: 387F003C  addi r3, r31, 0x3c
	ctx.r[3].s64 = ctx.r[31].s64 + 60;
	// 83253DBC: 4AFD9115  bl 0x8222ced0
	ctx.lr = 0x83253DC0;
	sub_8222CED0(ctx, base);
	// 83253DC0: 3D40820C  lis r10, -0x7df4
	ctx.r[10].s64 = -2113142784;
	// 83253DC4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83253DC8: 388A86D8  addi r4, r10, -0x7928
	ctx.r[4].s64 = ctx.r[10].s64 + -31016;
	// 83253DCC: 387F0040  addi r3, r31, 0x40
	ctx.r[3].s64 = ctx.r[31].s64 + 64;
	// 83253DD0: 4AFD9101  bl 0x8222ced0
	ctx.lr = 0x83253DD4;
	sub_8222CED0(ctx, base);
	// 83253DD4: 3D20820C  lis r9, -0x7df4
	ctx.r[9].s64 = -2113142784;
	// 83253DD8: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83253DDC: 388986B4  addi r4, r9, -0x794c
	ctx.r[4].s64 = ctx.r[9].s64 + -31052;
	// 83253DE0: 387F0044  addi r3, r31, 0x44
	ctx.r[3].s64 = ctx.r[31].s64 + 68;
	// 83253DE4: 4AFD90ED  bl 0x8222ced0
	ctx.lr = 0x83253DE8;
	sub_8222CED0(ctx, base);
	// 83253DE8: 3D00820C  lis r8, -0x7df4
	ctx.r[8].s64 = -2113142784;
	// 83253DEC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83253DF0: 38888694  addi r4, r8, -0x796c
	ctx.r[4].s64 = ctx.r[8].s64 + -31084;
	// 83253DF4: 387F0048  addi r3, r31, 0x48
	ctx.r[3].s64 = ctx.r[31].s64 + 72;
	// 83253DF8: 4AFD90D9  bl 0x8222ced0
	ctx.lr = 0x83253DFC;
	sub_8222CED0(ctx, base);
	// 83253DFC: 3CE0820C  lis r7, -0x7df4
	ctx.r[7].s64 = -2113142784;
	// 83253E00: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83253E04: 38878674  addi r4, r7, -0x798c
	ctx.r[4].s64 = ctx.r[7].s64 + -31116;
	// 83253E08: 387F004C  addi r3, r31, 0x4c
	ctx.r[3].s64 = ctx.r[31].s64 + 76;
	// 83253E0C: 4AFD90C5  bl 0x8222ced0
	ctx.lr = 0x83253E10;
	sub_8222CED0(ctx, base);
	// 83253E10: 3CC0820C  lis r6, -0x7df4
	ctx.r[6].s64 = -2113142784;
	// 83253E14: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83253E18: 38868650  addi r4, r6, -0x79b0
	ctx.r[4].s64 = ctx.r[6].s64 + -31152;
	// 83253E1C: 387F0050  addi r3, r31, 0x50
	ctx.r[3].s64 = ctx.r[31].s64 + 80;
	// 83253E20: 4AFD90B1  bl 0x8222ced0
	ctx.lr = 0x83253E24;
	sub_8222CED0(ctx, base);
	// 83253E24: 3CA0832B  lis r5, -0x7cd5
	ctx.r[5].s64 = -2094333952;
	// 83253E28: 38659E90  addi r3, r5, -0x6170
	ctx.r[3].s64 = ctx.r[5].s64 + -24944;
	// 83253E2C: 4BA560F5  bl 0x82ca9f20
	ctx.lr = 0x83253E30;
	sub_82CA9F20(ctx, base);
	// 83253E30: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83253E34: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83253E38: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83253E3C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83253E40: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83253E48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83253E48 size=136
    let mut pc: u32 = 0x83253E48;
    'dispatch: loop {
        match pc {
            0x83253E48 => {
    //   block [0x83253E48..0x83253ED0)
	// 83253E48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83253E4C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83253E50: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83253E54: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83253E58: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 83253E5C: 3D40820C  lis r10, -0x7df4
	ctx.r[10].s64 = -2113142784;
	// 83253E60: 3BEB8424  addi r31, r11, -0x7bdc
	ctx.r[31].s64 = ctx.r[11].s64 + -31708;
	// 83253E64: 388A89B8  addi r4, r10, -0x7648
	ctx.r[4].s64 = ctx.r[10].s64 + -30280;
	// 83253E68: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83253E6C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83253E70: 4AFD9061  bl 0x8222ced0
	ctx.lr = 0x83253E74;
	sub_8222CED0(ctx, base);
	// 83253E74: 3D20820C  lis r9, -0x7df4
	ctx.r[9].s64 = -2113142784;
	// 83253E78: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83253E7C: 38898988  addi r4, r9, -0x7678
	ctx.r[4].s64 = ctx.r[9].s64 + -30328;
	// 83253E80: 387F0004  addi r3, r31, 4
	ctx.r[3].s64 = ctx.r[31].s64 + 4;
	// 83253E84: 4AFD904D  bl 0x8222ced0
	ctx.lr = 0x83253E88;
	sub_8222CED0(ctx, base);
	// 83253E88: 3D00820C  lis r8, -0x7df4
	ctx.r[8].s64 = -2113142784;
	// 83253E8C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83253E90: 38888958  addi r4, r8, -0x76a8
	ctx.r[4].s64 = ctx.r[8].s64 + -30376;
	// 83253E94: 387F0008  addi r3, r31, 8
	ctx.r[3].s64 = ctx.r[31].s64 + 8;
	// 83253E98: 4AFD9039  bl 0x8222ced0
	ctx.lr = 0x83253E9C;
	sub_8222CED0(ctx, base);
	// 83253E9C: 3CE0820C  lis r7, -0x7df4
	ctx.r[7].s64 = -2113142784;
	// 83253EA0: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83253EA4: 38878928  addi r4, r7, -0x76d8
	ctx.r[4].s64 = ctx.r[7].s64 + -30424;
	// 83253EA8: 387F000C  addi r3, r31, 0xc
	ctx.r[3].s64 = ctx.r[31].s64 + 12;
	// 83253EAC: 4AFD9025  bl 0x8222ced0
	ctx.lr = 0x83253EB0;
	sub_8222CED0(ctx, base);
	// 83253EB0: 3CC0832B  lis r6, -0x7cd5
	ctx.r[6].s64 = -2094333952;
	// 83253EB4: 38669EF8  addi r3, r6, -0x6108
	ctx.r[3].s64 = ctx.r[6].s64 + -24840;
	// 83253EB8: 4BA56069  bl 0x82ca9f20
	ctx.lr = 0x83253EBC;
	sub_82CA9F20(ctx, base);
	// 83253EBC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83253EC0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83253EC4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83253EC8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83253ECC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83253ED0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83253ED0 size=156
    let mut pc: u32 = 0x83253ED0;
    'dispatch: loop {
        match pc {
            0x83253ED0 => {
    //   block [0x83253ED0..0x83253F6C)
	// 83253ED0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83253ED4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83253ED8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83253EDC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83253EE0: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 83253EE4: 3D40820C  lis r10, -0x7df4
	ctx.r[10].s64 = -2113142784;
	// 83253EE8: 3BEB8304  addi r31, r11, -0x7cfc
	ctx.r[31].s64 = ctx.r[11].s64 + -31996;
	// 83253EEC: 388A8AC8  addi r4, r10, -0x7538
	ctx.r[4].s64 = ctx.r[10].s64 + -30008;
	// 83253EF0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83253EF4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83253EF8: 4AFD8FD9  bl 0x8222ced0
	ctx.lr = 0x83253EFC;
	sub_8222CED0(ctx, base);
	// 83253EFC: 3D20820C  lis r9, -0x7df4
	ctx.r[9].s64 = -2113142784;
	// 83253F00: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83253F04: 38898A90  addi r4, r9, -0x7570
	ctx.r[4].s64 = ctx.r[9].s64 + -30064;
	// 83253F08: 387F0004  addi r3, r31, 4
	ctx.r[3].s64 = ctx.r[31].s64 + 4;
	// 83253F0C: 4AFD8FC5  bl 0x8222ced0
	ctx.lr = 0x83253F10;
	sub_8222CED0(ctx, base);
	// 83253F10: 3D00820C  lis r8, -0x7df4
	ctx.r[8].s64 = -2113142784;
	// 83253F14: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83253F18: 38888A58  addi r4, r8, -0x75a8
	ctx.r[4].s64 = ctx.r[8].s64 + -30120;
	// 83253F1C: 387F0008  addi r3, r31, 8
	ctx.r[3].s64 = ctx.r[31].s64 + 8;
	// 83253F20: 4AFD8FB1  bl 0x8222ced0
	ctx.lr = 0x83253F24;
	sub_8222CED0(ctx, base);
	// 83253F24: 3CE0820C  lis r7, -0x7df4
	ctx.r[7].s64 = -2113142784;
	// 83253F28: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83253F2C: 38878A24  addi r4, r7, -0x75dc
	ctx.r[4].s64 = ctx.r[7].s64 + -30172;
	// 83253F30: 387F000C  addi r3, r31, 0xc
	ctx.r[3].s64 = ctx.r[31].s64 + 12;
	// 83253F34: 4AFD8F9D  bl 0x8222ced0
	ctx.lr = 0x83253F38;
	sub_8222CED0(ctx, base);
	// 83253F38: 3CC0820C  lis r6, -0x7df4
	ctx.r[6].s64 = -2113142784;
	// 83253F3C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83253F40: 388689EC  addi r4, r6, -0x7614
	ctx.r[4].s64 = ctx.r[6].s64 + -30228;
	// 83253F44: 387F0010  addi r3, r31, 0x10
	ctx.r[3].s64 = ctx.r[31].s64 + 16;
	// 83253F48: 4AFD8F89  bl 0x8222ced0
	ctx.lr = 0x83253F4C;
	sub_8222CED0(ctx, base);
	// 83253F4C: 3CA0832B  lis r5, -0x7cd5
	ctx.r[5].s64 = -2094333952;
	// 83253F50: 38659F60  addi r3, r5, -0x60a0
	ctx.r[3].s64 = ctx.r[5].s64 + -24736;
	// 83253F54: 4BA55FCD  bl 0x82ca9f20
	ctx.lr = 0x83253F58;
	sub_82CA9F20(ctx, base);
	// 83253F58: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83253F5C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83253F60: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83253F64: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83253F68: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83253F70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83253F70 size=156
    let mut pc: u32 = 0x83253F70;
    'dispatch: loop {
        match pc {
            0x83253F70 => {
    //   block [0x83253F70..0x8325400C)
	// 83253F70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83253F74: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83253F78: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83253F7C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83253F80: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 83253F84: 3D40820C  lis r10, -0x7df4
	ctx.r[10].s64 = -2113142784;
	// 83253F88: 3BEB83BC  addi r31, r11, -0x7c44
	ctx.r[31].s64 = ctx.r[11].s64 + -31812;
	// 83253F8C: 388A8BE8  addi r4, r10, -0x7418
	ctx.r[4].s64 = ctx.r[10].s64 + -29720;
	// 83253F90: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83253F94: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83253F98: 4AFD8F39  bl 0x8222ced0
	ctx.lr = 0x83253F9C;
	sub_8222CED0(ctx, base);
	// 83253F9C: 3D20820C  lis r9, -0x7df4
	ctx.r[9].s64 = -2113142784;
	// 83253FA0: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83253FA4: 38898BB0  addi r4, r9, -0x7450
	ctx.r[4].s64 = ctx.r[9].s64 + -29776;
	// 83253FA8: 387F0004  addi r3, r31, 4
	ctx.r[3].s64 = ctx.r[31].s64 + 4;
	// 83253FAC: 4AFD8F25  bl 0x8222ced0
	ctx.lr = 0x83253FB0;
	sub_8222CED0(ctx, base);
	// 83253FB0: 3D00820C  lis r8, -0x7df4
	ctx.r[8].s64 = -2113142784;
	// 83253FB4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83253FB8: 38888B78  addi r4, r8, -0x7488
	ctx.r[4].s64 = ctx.r[8].s64 + -29832;
	// 83253FBC: 387F0008  addi r3, r31, 8
	ctx.r[3].s64 = ctx.r[31].s64 + 8;
	// 83253FC0: 4AFD8F11  bl 0x8222ced0
	ctx.lr = 0x83253FC4;
	sub_8222CED0(ctx, base);
	// 83253FC4: 3CE0820C  lis r7, -0x7df4
	ctx.r[7].s64 = -2113142784;
	// 83253FC8: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83253FCC: 38878B40  addi r4, r7, -0x74c0
	ctx.r[4].s64 = ctx.r[7].s64 + -29888;
	// 83253FD0: 387F000C  addi r3, r31, 0xc
	ctx.r[3].s64 = ctx.r[31].s64 + 12;
	// 83253FD4: 4AFD8EFD  bl 0x8222ced0
	ctx.lr = 0x83253FD8;
	sub_8222CED0(ctx, base);
	// 83253FD8: 3CC0820C  lis r6, -0x7df4
	ctx.r[6].s64 = -2113142784;
	// 83253FDC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83253FE0: 38868B04  addi r4, r6, -0x74fc
	ctx.r[4].s64 = ctx.r[6].s64 + -29948;
	// 83253FE4: 387F0010  addi r3, r31, 0x10
	ctx.r[3].s64 = ctx.r[31].s64 + 16;
	// 83253FE8: 4AFD8EE9  bl 0x8222ced0
	ctx.lr = 0x83253FEC;
	sub_8222CED0(ctx, base);
	// 83253FEC: 3CA0832B  lis r5, -0x7cd5
	ctx.r[5].s64 = -2094333952;
	// 83253FF0: 38659FC8  addi r3, r5, -0x6038
	ctx.r[3].s64 = ctx.r[5].s64 + -24632;
	// 83253FF4: 4BA55F2D  bl 0x82ca9f20
	ctx.lr = 0x83253FF8;
	sub_82CA9F20(ctx, base);
	// 83253FF8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83253FFC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83254000: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83254004: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83254008: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83254010(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83254010 size=192
    let mut pc: u32 = 0x83254010;
    'dispatch: loop {
        match pc {
            0x83254010 => {
    //   block [0x83254010..0x83254068)
	// 83254010: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83254014: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83254018: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8325401C: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83254020: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 83254024: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83254028: 388B3390  addi r4, r11, 0x3390
	ctx.r[4].s64 = ctx.r[11].s64 + 13200;
	// 8325402C: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 83254030: 4AFD8EA1  bl 0x8222ced0
	ctx.lr = 0x83254034;
	sub_8222CED0(ctx, base);
	// 83254034: 38810054  addi r4, r1, 0x54
	ctx.r[4].s64 = ctx.r[1].s64 + 84;
	// 83254038: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8325403C: 4AF9AAFD  bl 0x821eeb38
	ctx.lr = 0x83254040;
	sub_821EEB38(ctx, base);
	// 83254040: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 83254044: 4B9AF7AD  bl 0x82c037f0
	ctx.lr = 0x83254048;
	sub_82C037F0(ctx, base);
	// 83254048: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325404C: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 83254050: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 83254054: 916A8434  stw r11, -0x7bcc(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-31692 as u32), ctx.r[11].u32 ) };
	// 83254058: 4AF72711  bl 0x821c6768
	ctx.lr = 0x8325405C;
	sub_821C6768(ctx, base);
	// 8325405C: 3D208349  lis r9, -0x7cb7
	ctx.r[9].s64 = -2092367872;
	// 83254060: 3BE97088  addi r31, r9, 0x7088
	ctx.r[31].s64 = ctx.r[9].s64 + 28808;
	// 83254064: 7FE6FB78  mr r6, r31
	ctx.r[6].u64 = ctx.r[31].u64;
	pc = 0x83254068; continue 'dispatch;
            }
            0x83254068 => {
    //   block [0x83254068..0x83254094)
	// 83254068: 7CE000A6  mfmsr r7
	ctx.r[7].u64 = ctx.msr;
	// 8325406C: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 83254070: 7D003028  lwarx r8, 0, r6
	// lwarx
	let ea = ctx.r[6].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[8].u64 = ctx.reserved.u32 as u64;
	// 83254074: 3908FFFF  addi r8, r8, -1
	ctx.r[8].s64 = ctx.r[8].s64 + -1;
	// 83254078: 7D00312D  stwcx. r8, 0, r6
	// stwcx.
	let addr = ctx.r[6].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[8].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 8325407C: 7CE10164  mtmsrd r7, 1
	ctx.msr = (ctx.r[7].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 83254080: 4082FFE8  bne 0x83254068
	if !ctx.cr[0].eq {
	pc = 0x83254068; continue 'dispatch;
	}
	// 83254084: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 83254088: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 8325408C: 90A10050  stw r5, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[5].u32 ) };
	// 83254090: 4AF726D9  bl 0x821c6768
	ctx.lr = 0x83254094;
	sub_821C6768(ctx, base);
	pc = 0x83254094; continue 'dispatch;
            }
            0x83254094 => {
    //   block [0x83254094..0x832540D0)
	// 83254094: 7C6000A6  mfmsr r3
	ctx.r[3].u64 = ctx.msr;
	// 83254098: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8325409C: 7C80F828  lwarx r4, 0, r31
	// lwarx
	let ea = ctx.r[31].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[4].u64 = ctx.reserved.u32 as u64;
	// 832540A0: 3884FFFF  addi r4, r4, -1
	ctx.r[4].s64 = ctx.r[4].s64 + -1;
	// 832540A4: 7C80F92D  stwcx. r4, 0, r31
	// stwcx.
	let addr = ctx.r[31].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[4].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 832540A8: 7C610164  mtmsrd r3, 1
	ctx.msr = (ctx.r[3].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 832540AC: 4082FFE8  bne 0x83254094
	if !ctx.cr[0].eq {
	pc = 0x83254094; continue 'dispatch;
	}
	// 832540B0: 3D60832B  lis r11, -0x7cd5
	ctx.r[11].s64 = -2094333952;
	// 832540B4: 386BA030  addi r3, r11, -0x5fd0
	ctx.r[3].s64 = ctx.r[11].s64 + -24528;
	// 832540B8: 4BA55E69  bl 0x82ca9f20
	ctx.lr = 0x832540BC;
	sub_82CA9F20(ctx, base);
	// 832540BC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 832540C0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832540C4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832540C8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 832540CC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832540D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832540D0 size=64
    let mut pc: u32 = 0x832540D0;
    'dispatch: loop {
        match pc {
            0x832540D0 => {
    //   block [0x832540D0..0x83254110)
	// 832540D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832540D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832540D8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832540DC: 3D60820C  lis r11, -0x7df4
	ctx.r[11].s64 = -2113142784;
	// 832540E0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832540E4: 388B9A0C  addi r4, r11, -0x65f4
	ctx.r[4].s64 = ctx.r[11].s64 + -26100;
	// 832540E8: 386A8438  addi r3, r10, -0x7bc8
	ctx.r[3].s64 = ctx.r[10].s64 + -31688;
	// 832540EC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832540F0: 4AFD8DE1  bl 0x8222ced0
	ctx.lr = 0x832540F4;
	sub_8222CED0(ctx, base);
	// 832540F4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 832540F8: 3869A038  addi r3, r9, -0x5fc8
	ctx.r[3].s64 = ctx.r[9].s64 + -24520;
	// 832540FC: 4BA55E25  bl 0x82ca9f20
	ctx.lr = 0x83254100;
	sub_82CA9F20(ctx, base);
	// 83254100: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83254104: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83254108: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325410C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83254110(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83254110 size=64
    let mut pc: u32 = 0x83254110;
    'dispatch: loop {
        match pc {
            0x83254110 => {
    //   block [0x83254110..0x83254150)
	// 83254110: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83254114: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83254118: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325411C: 3D60820C  lis r11, -0x7df4
	ctx.r[11].s64 = -2113142784;
	// 83254120: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83254124: 388B9A14  addi r4, r11, -0x65ec
	ctx.r[4].s64 = ctx.r[11].s64 + -26092;
	// 83254128: 386A843C  addi r3, r10, -0x7bc4
	ctx.r[3].s64 = ctx.r[10].s64 + -31684;
	// 8325412C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83254130: 4AFD8DA1  bl 0x8222ced0
	ctx.lr = 0x83254134;
	sub_8222CED0(ctx, base);
	// 83254134: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83254138: 3869A048  addi r3, r9, -0x5fb8
	ctx.r[3].s64 = ctx.r[9].s64 + -24504;
	// 8325413C: 4BA55DE5  bl 0x82ca9f20
	ctx.lr = 0x83254140;
	sub_82CA9F20(ctx, base);
	// 83254140: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83254144: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83254148: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325414C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83254150(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83254150 size=64
    let mut pc: u32 = 0x83254150;
    'dispatch: loop {
        match pc {
            0x83254150 => {
    //   block [0x83254150..0x83254190)
	// 83254150: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83254154: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83254158: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325415C: 3D60820C  lis r11, -0x7df4
	ctx.r[11].s64 = -2113142784;
	// 83254160: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83254164: 388B9A24  addi r4, r11, -0x65dc
	ctx.r[4].s64 = ctx.r[11].s64 + -26076;
	// 83254168: 386A8440  addi r3, r10, -0x7bc0
	ctx.r[3].s64 = ctx.r[10].s64 + -31680;
	// 8325416C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83254170: 4AFD8D61  bl 0x8222ced0
	ctx.lr = 0x83254174;
	sub_8222CED0(ctx, base);
	// 83254174: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83254178: 3869A058  addi r3, r9, -0x5fa8
	ctx.r[3].s64 = ctx.r[9].s64 + -24488;
	// 8325417C: 4BA55DA5  bl 0x82ca9f20
	ctx.lr = 0x83254180;
	sub_82CA9F20(ctx, base);
	// 83254180: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83254184: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83254188: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325418C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83254190(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83254190 size=64
    let mut pc: u32 = 0x83254190;
    'dispatch: loop {
        match pc {
            0x83254190 => {
    //   block [0x83254190..0x832541D0)
	// 83254190: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83254194: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83254198: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325419C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 832541A0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832541A4: 388B1AD4  addi r4, r11, 0x1ad4
	ctx.r[4].s64 = ctx.r[11].s64 + 6868;
	// 832541A8: 386A8444  addi r3, r10, -0x7bbc
	ctx.r[3].s64 = ctx.r[10].s64 + -31676;
	// 832541AC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832541B0: 4AFD8D21  bl 0x8222ced0
	ctx.lr = 0x832541B4;
	sub_8222CED0(ctx, base);
	// 832541B4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 832541B8: 3869A068  addi r3, r9, -0x5f98
	ctx.r[3].s64 = ctx.r[9].s64 + -24472;
	// 832541BC: 4BA55D65  bl 0x82ca9f20
	ctx.lr = 0x832541C0;
	sub_82CA9F20(ctx, base);
	// 832541C0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832541C4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832541C8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832541CC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832541D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832541D0 size=64
    let mut pc: u32 = 0x832541D0;
    'dispatch: loop {
        match pc {
            0x832541D0 => {
    //   block [0x832541D0..0x83254210)
	// 832541D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832541D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832541D8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832541DC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 832541E0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832541E4: 388B1ADC  addi r4, r11, 0x1adc
	ctx.r[4].s64 = ctx.r[11].s64 + 6876;
	// 832541E8: 386A8448  addi r3, r10, -0x7bb8
	ctx.r[3].s64 = ctx.r[10].s64 + -31672;
	// 832541EC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832541F0: 4AFD8CE1  bl 0x8222ced0
	ctx.lr = 0x832541F4;
	sub_8222CED0(ctx, base);
	// 832541F4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 832541F8: 3869A078  addi r3, r9, -0x5f88
	ctx.r[3].s64 = ctx.r[9].s64 + -24456;
	// 832541FC: 4BA55D25  bl 0x82ca9f20
	ctx.lr = 0x83254200;
	sub_82CA9F20(ctx, base);
	// 83254200: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83254204: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83254208: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325420C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83254210(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83254210 size=64
    let mut pc: u32 = 0x83254210;
    'dispatch: loop {
        match pc {
            0x83254210 => {
    //   block [0x83254210..0x83254250)
	// 83254210: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83254214: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83254218: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325421C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83254220: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83254224: 388B1AF0  addi r4, r11, 0x1af0
	ctx.r[4].s64 = ctx.r[11].s64 + 6896;
	// 83254228: 386A844C  addi r3, r10, -0x7bb4
	ctx.r[3].s64 = ctx.r[10].s64 + -31668;
	// 8325422C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83254230: 4AFD8CA1  bl 0x8222ced0
	ctx.lr = 0x83254234;
	sub_8222CED0(ctx, base);
	// 83254234: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83254238: 3869A088  addi r3, r9, -0x5f78
	ctx.r[3].s64 = ctx.r[9].s64 + -24440;
	// 8325423C: 4BA55CE5  bl 0x82ca9f20
	ctx.lr = 0x83254240;
	sub_82CA9F20(ctx, base);
	// 83254240: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83254244: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83254248: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325424C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83254250(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83254250 size=64
    let mut pc: u32 = 0x83254250;
    'dispatch: loop {
        match pc {
            0x83254250 => {
    //   block [0x83254250..0x83254290)
	// 83254250: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83254254: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83254258: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325425C: 3D60820C  lis r11, -0x7df4
	ctx.r[11].s64 = -2113142784;
	// 83254260: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83254264: 388BA188  addi r4, r11, -0x5e78
	ctx.r[4].s64 = ctx.r[11].s64 + -24184;
	// 83254268: 386A8450  addi r3, r10, -0x7bb0
	ctx.r[3].s64 = ctx.r[10].s64 + -31664;
	// 8325426C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83254270: 4AFD8C61  bl 0x8222ced0
	ctx.lr = 0x83254274;
	sub_8222CED0(ctx, base);
	// 83254274: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83254278: 3869A098  addi r3, r9, -0x5f68
	ctx.r[3].s64 = ctx.r[9].s64 + -24424;
	// 8325427C: 4BA55CA5  bl 0x82ca9f20
	ctx.lr = 0x83254280;
	sub_82CA9F20(ctx, base);
	// 83254280: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83254284: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83254288: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325428C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83254290(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83254290 size=52
    let mut pc: u32 = 0x83254290;
    'dispatch: loop {
        match pc {
            0x83254290 => {
    //   block [0x83254290..0x832542C4)
	// 83254290: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83254294: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83254298: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325429C: 3D60834A  lis r11, -0x7cb6
	ctx.r[11].s64 = -2092302336;
	// 832542A0: 386B8454  addi r3, r11, -0x7bac
	ctx.r[3].s64 = ctx.r[11].s64 + -31660;
	// 832542A4: 4B2308CD  bl 0x82484b70
	ctx.lr = 0x832542A8;
	sub_82484B70(ctx, base);
	// 832542A8: 3D40832B  lis r10, -0x7cd5
	ctx.r[10].s64 = -2094333952;
	// 832542AC: 386AA0A8  addi r3, r10, -0x5f58
	ctx.r[3].s64 = ctx.r[10].s64 + -24408;
	// 832542B0: 4BA55C71  bl 0x82ca9f20
	ctx.lr = 0x832542B4;
	sub_82CA9F20(ctx, base);
	// 832542B4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832542B8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832542BC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832542C0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832542C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832542C8 size=12
    let mut pc: u32 = 0x832542C8;
    'dispatch: loop {
        match pc {
            0x832542C8 => {
    //   block [0x832542C8..0x832542D4)
	// 832542C8: 3D60832B  lis r11, -0x7cd5
	ctx.r[11].s64 = -2094333952;
	// 832542CC: 386BA0B8  addi r3, r11, -0x5f48
	ctx.r[3].s64 = ctx.r[11].s64 + -24392;
	// 832542D0: 4BA55C50  b 0x82ca9f20
	sub_82CA9F20(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832542D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832542D8 size=64
    let mut pc: u32 = 0x832542D8;
    'dispatch: loop {
        match pc {
            0x832542D8 => {
    //   block [0x832542D8..0x83254318)
	// 832542D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832542DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832542E0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832542E4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 832542E8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832542EC: 388B1AD4  addi r4, r11, 0x1ad4
	ctx.r[4].s64 = ctx.r[11].s64 + 6868;
	// 832542F0: 386A8460  addi r3, r10, -0x7ba0
	ctx.r[3].s64 = ctx.r[10].s64 + -31648;
	// 832542F4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832542F8: 4AFD8BD9  bl 0x8222ced0
	ctx.lr = 0x832542FC;
	sub_8222CED0(ctx, base);
	// 832542FC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83254300: 3869A100  addi r3, r9, -0x5f00
	ctx.r[3].s64 = ctx.r[9].s64 + -24320;
	// 83254304: 4BA55C1D  bl 0x82ca9f20
	ctx.lr = 0x83254308;
	sub_82CA9F20(ctx, base);
	// 83254308: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325430C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83254310: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83254314: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83254318(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83254318 size=64
    let mut pc: u32 = 0x83254318;
    'dispatch: loop {
        match pc {
            0x83254318 => {
    //   block [0x83254318..0x83254358)
	// 83254318: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325431C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83254320: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83254324: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83254328: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325432C: 388B1ADC  addi r4, r11, 0x1adc
	ctx.r[4].s64 = ctx.r[11].s64 + 6876;
	// 83254330: 386A8464  addi r3, r10, -0x7b9c
	ctx.r[3].s64 = ctx.r[10].s64 + -31644;
	// 83254334: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83254338: 4AFD8B99  bl 0x8222ced0
	ctx.lr = 0x8325433C;
	sub_8222CED0(ctx, base);
	// 8325433C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83254340: 3869A110  addi r3, r9, -0x5ef0
	ctx.r[3].s64 = ctx.r[9].s64 + -24304;
	// 83254344: 4BA55BDD  bl 0x82ca9f20
	ctx.lr = 0x83254348;
	sub_82CA9F20(ctx, base);
	// 83254348: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325434C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83254350: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83254354: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83254358(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83254358 size=64
    let mut pc: u32 = 0x83254358;
    'dispatch: loop {
        match pc {
            0x83254358 => {
    //   block [0x83254358..0x83254398)
	// 83254358: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325435C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83254360: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83254364: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83254368: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325436C: 388B1AF0  addi r4, r11, 0x1af0
	ctx.r[4].s64 = ctx.r[11].s64 + 6896;
	// 83254370: 386A8468  addi r3, r10, -0x7b98
	ctx.r[3].s64 = ctx.r[10].s64 + -31640;
	// 83254374: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83254378: 4AFD8B59  bl 0x8222ced0
	ctx.lr = 0x8325437C;
	sub_8222CED0(ctx, base);
	// 8325437C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83254380: 3869A120  addi r3, r9, -0x5ee0
	ctx.r[3].s64 = ctx.r[9].s64 + -24288;
	// 83254384: 4BA55B9D  bl 0x82ca9f20
	ctx.lr = 0x83254388;
	sub_82CA9F20(ctx, base);
	// 83254388: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325438C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83254390: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83254394: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83254398(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83254398 size=64
    let mut pc: u32 = 0x83254398;
    'dispatch: loop {
        match pc {
            0x83254398 => {
    //   block [0x83254398..0x832543D8)
	// 83254398: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325439C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832543A0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832543A4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 832543A8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832543AC: 388B1AD4  addi r4, r11, 0x1ad4
	ctx.r[4].s64 = ctx.r[11].s64 + 6868;
	// 832543B0: 386A846C  addi r3, r10, -0x7b94
	ctx.r[3].s64 = ctx.r[10].s64 + -31636;
	// 832543B4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832543B8: 4AFD8B19  bl 0x8222ced0
	ctx.lr = 0x832543BC;
	sub_8222CED0(ctx, base);
	// 832543BC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 832543C0: 3869A140  addi r3, r9, -0x5ec0
	ctx.r[3].s64 = ctx.r[9].s64 + -24256;
	// 832543C4: 4BA55B5D  bl 0x82ca9f20
	ctx.lr = 0x832543C8;
	sub_82CA9F20(ctx, base);
	// 832543C8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832543CC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832543D0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832543D4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832543D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832543D8 size=64
    let mut pc: u32 = 0x832543D8;
    'dispatch: loop {
        match pc {
            0x832543D8 => {
    //   block [0x832543D8..0x83254418)
	// 832543D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832543DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832543E0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832543E4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 832543E8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832543EC: 388B1ADC  addi r4, r11, 0x1adc
	ctx.r[4].s64 = ctx.r[11].s64 + 6876;
	// 832543F0: 386A8470  addi r3, r10, -0x7b90
	ctx.r[3].s64 = ctx.r[10].s64 + -31632;
	// 832543F4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832543F8: 4AFD8AD9  bl 0x8222ced0
	ctx.lr = 0x832543FC;
	sub_8222CED0(ctx, base);
	// 832543FC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83254400: 3869A150  addi r3, r9, -0x5eb0
	ctx.r[3].s64 = ctx.r[9].s64 + -24240;
	// 83254404: 4BA55B1D  bl 0x82ca9f20
	ctx.lr = 0x83254408;
	sub_82CA9F20(ctx, base);
	// 83254408: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325440C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83254410: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83254414: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83254418(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83254418 size=64
    let mut pc: u32 = 0x83254418;
    'dispatch: loop {
        match pc {
            0x83254418 => {
    //   block [0x83254418..0x83254458)
	// 83254418: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325441C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83254420: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83254424: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83254428: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325442C: 388B1AF0  addi r4, r11, 0x1af0
	ctx.r[4].s64 = ctx.r[11].s64 + 6896;
	// 83254430: 386A8474  addi r3, r10, -0x7b8c
	ctx.r[3].s64 = ctx.r[10].s64 + -31628;
	// 83254434: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83254438: 4AFD8A99  bl 0x8222ced0
	ctx.lr = 0x8325443C;
	sub_8222CED0(ctx, base);
	// 8325443C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83254440: 3869A160  addi r3, r9, -0x5ea0
	ctx.r[3].s64 = ctx.r[9].s64 + -24224;
	// 83254444: 4BA55ADD  bl 0x82ca9f20
	ctx.lr = 0x83254448;
	sub_82CA9F20(ctx, base);
	// 83254448: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325444C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83254450: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83254454: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83254458(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83254458 size=64
    let mut pc: u32 = 0x83254458;
    'dispatch: loop {
        match pc {
            0x83254458 => {
    //   block [0x83254458..0x83254498)
	// 83254458: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325445C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83254460: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83254464: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83254468: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325446C: 388B1AD4  addi r4, r11, 0x1ad4
	ctx.r[4].s64 = ctx.r[11].s64 + 6868;
	// 83254470: 386A8478  addi r3, r10, -0x7b88
	ctx.r[3].s64 = ctx.r[10].s64 + -31624;
	// 83254474: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83254478: 4AFD8A59  bl 0x8222ced0
	ctx.lr = 0x8325447C;
	sub_8222CED0(ctx, base);
	// 8325447C: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83254480: 3869A170  addi r3, r9, -0x5e90
	ctx.r[3].s64 = ctx.r[9].s64 + -24208;
	// 83254484: 4BA55A9D  bl 0x82ca9f20
	ctx.lr = 0x83254488;
	sub_82CA9F20(ctx, base);
	// 83254488: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325448C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83254490: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83254494: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83254498(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83254498 size=64
    let mut pc: u32 = 0x83254498;
    'dispatch: loop {
        match pc {
            0x83254498 => {
    //   block [0x83254498..0x832544D8)
	// 83254498: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325449C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832544A0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832544A4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 832544A8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832544AC: 388B1ADC  addi r4, r11, 0x1adc
	ctx.r[4].s64 = ctx.r[11].s64 + 6876;
	// 832544B0: 386A847C  addi r3, r10, -0x7b84
	ctx.r[3].s64 = ctx.r[10].s64 + -31620;
	// 832544B4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832544B8: 4AFD8A19  bl 0x8222ced0
	ctx.lr = 0x832544BC;
	sub_8222CED0(ctx, base);
	// 832544BC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 832544C0: 3869A180  addi r3, r9, -0x5e80
	ctx.r[3].s64 = ctx.r[9].s64 + -24192;
	// 832544C4: 4BA55A5D  bl 0x82ca9f20
	ctx.lr = 0x832544C8;
	sub_82CA9F20(ctx, base);
	// 832544C8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832544CC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832544D0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832544D4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832544D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832544D8 size=64
    let mut pc: u32 = 0x832544D8;
    'dispatch: loop {
        match pc {
            0x832544D8 => {
    //   block [0x832544D8..0x83254518)
	// 832544D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832544DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832544E0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832544E4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 832544E8: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832544EC: 388B1AF0  addi r4, r11, 0x1af0
	ctx.r[4].s64 = ctx.r[11].s64 + 6896;
	// 832544F0: 386A8480  addi r3, r10, -0x7b80
	ctx.r[3].s64 = ctx.r[10].s64 + -31616;
	// 832544F4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832544F8: 4AFD89D9  bl 0x8222ced0
	ctx.lr = 0x832544FC;
	sub_8222CED0(ctx, base);
	// 832544FC: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83254500: 3869A190  addi r3, r9, -0x5e70
	ctx.r[3].s64 = ctx.r[9].s64 + -24176;
	// 83254504: 4BA55A1D  bl 0x82ca9f20
	ctx.lr = 0x83254508;
	sub_82CA9F20(ctx, base);
	// 83254508: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8325450C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83254510: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83254514: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83254518(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83254518 size=56
    let mut pc: u32 = 0x83254518;
    'dispatch: loop {
        match pc {
            0x83254518 => {
    //   block [0x83254518..0x83254550)
	// 83254518: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8325451C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83254520: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83254524: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 83254528: 3C80811C  lis r4, -0x7ee4
	ctx.r[4].s64 = -2128871424;
	// 8325452C: 386B25D4  addi r3, r11, 0x25d4
	ctx.r[3].s64 = ctx.r[11].s64 + 9684;
	// 83254530: 60849DC5  ori r4, r4, 0x9dc5
	ctx.r[4].u64 = ctx.r[4].u64 | 40389;
	// 83254534: 4AF9F825  bl 0x821f3d58
	ctx.lr = 0x83254538;
	sub_821F3D58(ctx, base);
	// 83254538: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 8325453C: 906A8484  stw r3, -0x7b7c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-31612 as u32), ctx.r[3].u32 ) };
	// 83254540: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83254544: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83254548: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325454C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83254550(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83254550 size=12
    let mut pc: u32 = 0x83254550;
    'dispatch: loop {
        match pc {
            0x83254550 => {
    //   block [0x83254550..0x8325455C)
	// 83254550: 3D60832B  lis r11, -0x7cd5
	ctx.r[11].s64 = -2094333952;
	// 83254554: 386BA1A0  addi r3, r11, -0x5e60
	ctx.r[3].s64 = ctx.r[11].s64 + -24160;
	// 83254558: 4BA559C8  b 0x82ca9f20
	sub_82CA9F20(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83254560(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83254560 size=12
    let mut pc: u32 = 0x83254560;
    'dispatch: loop {
        match pc {
            0x83254560 => {
    //   block [0x83254560..0x8325456C)
	// 83254560: 3D60832B  lis r11, -0x7cd5
	ctx.r[11].s64 = -2094333952;
	// 83254564: 386BA208  addi r3, r11, -0x5df8
	ctx.r[3].s64 = ctx.r[11].s64 + -24056;
	// 83254568: 4BA559B8  b 0x82ca9f20
	sub_82CA9F20(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83254570(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83254570 size=64
    let mut pc: u32 = 0x83254570;
    'dispatch: loop {
        match pc {
            0x83254570 => {
    //   block [0x83254570..0x832545B0)
	// 83254570: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83254574: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83254578: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325457C: 3D60820C  lis r11, -0x7df4
	ctx.r[11].s64 = -2113142784;
	// 83254580: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83254584: 388BCA34  addi r4, r11, -0x35cc
	ctx.r[4].s64 = ctx.r[11].s64 + -13772;
	// 83254588: 386A84A8  addi r3, r10, -0x7b58
	ctx.r[3].s64 = ctx.r[10].s64 + -31576;
	// 8325458C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83254590: 4AFD8941  bl 0x8222ced0
	ctx.lr = 0x83254594;
	sub_8222CED0(ctx, base);
	// 83254594: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83254598: 3869A270  addi r3, r9, -0x5d90
	ctx.r[3].s64 = ctx.r[9].s64 + -23952;
	// 8325459C: 4BA55985  bl 0x82ca9f20
	ctx.lr = 0x832545A0;
	sub_82CA9F20(ctx, base);
	// 832545A0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832545A4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832545A8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832545AC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832545B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832545B0 size=64
    let mut pc: u32 = 0x832545B0;
    'dispatch: loop {
        match pc {
            0x832545B0 => {
    //   block [0x832545B0..0x832545F0)
	// 832545B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832545B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832545B8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832545BC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 832545C0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832545C4: 388B1AD4  addi r4, r11, 0x1ad4
	ctx.r[4].s64 = ctx.r[11].s64 + 6868;
	// 832545C8: 386A84AC  addi r3, r10, -0x7b54
	ctx.r[3].s64 = ctx.r[10].s64 + -31572;
	// 832545CC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832545D0: 4AFD8901  bl 0x8222ced0
	ctx.lr = 0x832545D4;
	sub_8222CED0(ctx, base);
	// 832545D4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 832545D8: 3869A280  addi r3, r9, -0x5d80
	ctx.r[3].s64 = ctx.r[9].s64 + -23936;
	// 832545DC: 4BA55945  bl 0x82ca9f20
	ctx.lr = 0x832545E0;
	sub_82CA9F20(ctx, base);
	// 832545E0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832545E4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832545E8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832545EC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832545F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832545F0 size=64
    let mut pc: u32 = 0x832545F0;
    'dispatch: loop {
        match pc {
            0x832545F0 => {
    //   block [0x832545F0..0x83254630)
	// 832545F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832545F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832545F8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832545FC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83254600: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83254604: 388B1ADC  addi r4, r11, 0x1adc
	ctx.r[4].s64 = ctx.r[11].s64 + 6876;
	// 83254608: 386A84B0  addi r3, r10, -0x7b50
	ctx.r[3].s64 = ctx.r[10].s64 + -31568;
	// 8325460C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83254610: 4AFD88C1  bl 0x8222ced0
	ctx.lr = 0x83254614;
	sub_8222CED0(ctx, base);
	// 83254614: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83254618: 3869A290  addi r3, r9, -0x5d70
	ctx.r[3].s64 = ctx.r[9].s64 + -23920;
	// 8325461C: 4BA55905  bl 0x82ca9f20
	ctx.lr = 0x83254620;
	sub_82CA9F20(ctx, base);
	// 83254620: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83254624: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83254628: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325462C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83254630(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83254630 size=64
    let mut pc: u32 = 0x83254630;
    'dispatch: loop {
        match pc {
            0x83254630 => {
    //   block [0x83254630..0x83254670)
	// 83254630: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83254634: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83254638: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325463C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83254640: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83254644: 388B1AF0  addi r4, r11, 0x1af0
	ctx.r[4].s64 = ctx.r[11].s64 + 6896;
	// 83254648: 386A84B4  addi r3, r10, -0x7b4c
	ctx.r[3].s64 = ctx.r[10].s64 + -31564;
	// 8325464C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83254650: 4AFD8881  bl 0x8222ced0
	ctx.lr = 0x83254654;
	sub_8222CED0(ctx, base);
	// 83254654: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83254658: 3869A2A0  addi r3, r9, -0x5d60
	ctx.r[3].s64 = ctx.r[9].s64 + -23904;
	// 8325465C: 4BA558C5  bl 0x82ca9f20
	ctx.lr = 0x83254660;
	sub_82CA9F20(ctx, base);
	// 83254660: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83254664: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83254668: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325466C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83254670(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83254670 size=64
    let mut pc: u32 = 0x83254670;
    'dispatch: loop {
        match pc {
            0x83254670 => {
    //   block [0x83254670..0x832546B0)
	// 83254670: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83254674: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83254678: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325467C: 3D60820C  lis r11, -0x7df4
	ctx.r[11].s64 = -2113142784;
	// 83254680: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83254684: 388BCDB4  addi r4, r11, -0x324c
	ctx.r[4].s64 = ctx.r[11].s64 + -12876;
	// 83254688: 386A84B8  addi r3, r10, -0x7b48
	ctx.r[3].s64 = ctx.r[10].s64 + -31560;
	// 8325468C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83254690: 4AFD8841  bl 0x8222ced0
	ctx.lr = 0x83254694;
	sub_8222CED0(ctx, base);
	// 83254694: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83254698: 3869A2B0  addi r3, r9, -0x5d50
	ctx.r[3].s64 = ctx.r[9].s64 + -23888;
	// 8325469C: 4BA55885  bl 0x82ca9f20
	ctx.lr = 0x832546A0;
	sub_82CA9F20(ctx, base);
	// 832546A0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832546A4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832546A8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832546AC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832546B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832546B0 size=64
    let mut pc: u32 = 0x832546B0;
    'dispatch: loop {
        match pc {
            0x832546B0 => {
    //   block [0x832546B0..0x832546F0)
	// 832546B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832546B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832546B8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832546BC: 3D60820C  lis r11, -0x7df4
	ctx.r[11].s64 = -2113142784;
	// 832546C0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832546C4: 388BCDBC  addi r4, r11, -0x3244
	ctx.r[4].s64 = ctx.r[11].s64 + -12868;
	// 832546C8: 386A84BC  addi r3, r10, -0x7b44
	ctx.r[3].s64 = ctx.r[10].s64 + -31556;
	// 832546CC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832546D0: 4AFD8801  bl 0x8222ced0
	ctx.lr = 0x832546D4;
	sub_8222CED0(ctx, base);
	// 832546D4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 832546D8: 3869A2C0  addi r3, r9, -0x5d40
	ctx.r[3].s64 = ctx.r[9].s64 + -23872;
	// 832546DC: 4BA55845  bl 0x82ca9f20
	ctx.lr = 0x832546E0;
	sub_82CA9F20(ctx, base);
	// 832546E0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832546E4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832546E8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832546EC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832546F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832546F0 size=64
    let mut pc: u32 = 0x832546F0;
    'dispatch: loop {
        match pc {
            0x832546F0 => {
    //   block [0x832546F0..0x83254730)
	// 832546F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832546F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832546F8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832546FC: 3D60820C  lis r11, -0x7df4
	ctx.r[11].s64 = -2113142784;
	// 83254700: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83254704: 388BCDC4  addi r4, r11, -0x323c
	ctx.r[4].s64 = ctx.r[11].s64 + -12860;
	// 83254708: 386A84C0  addi r3, r10, -0x7b40
	ctx.r[3].s64 = ctx.r[10].s64 + -31552;
	// 8325470C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83254710: 4AFD87C1  bl 0x8222ced0
	ctx.lr = 0x83254714;
	sub_8222CED0(ctx, base);
	// 83254714: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83254718: 3869A2D0  addi r3, r9, -0x5d30
	ctx.r[3].s64 = ctx.r[9].s64 + -23856;
	// 8325471C: 4BA55805  bl 0x82ca9f20
	ctx.lr = 0x83254720;
	sub_82CA9F20(ctx, base);
	// 83254720: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83254724: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83254728: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325472C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83254730(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83254730 size=64
    let mut pc: u32 = 0x83254730;
    'dispatch: loop {
        match pc {
            0x83254730 => {
    //   block [0x83254730..0x83254770)
	// 83254730: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83254734: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83254738: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325473C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83254740: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83254744: 388B052C  addi r4, r11, 0x52c
	ctx.r[4].s64 = ctx.r[11].s64 + 1324;
	// 83254748: 386A84C4  addi r3, r10, -0x7b3c
	ctx.r[3].s64 = ctx.r[10].s64 + -31548;
	// 8325474C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83254750: 4AFD8781  bl 0x8222ced0
	ctx.lr = 0x83254754;
	sub_8222CED0(ctx, base);
	// 83254754: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83254758: 3869A2E0  addi r3, r9, -0x5d20
	ctx.r[3].s64 = ctx.r[9].s64 + -23840;
	// 8325475C: 4BA557C5  bl 0x82ca9f20
	ctx.lr = 0x83254760;
	sub_82CA9F20(ctx, base);
	// 83254760: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83254764: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83254768: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325476C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83254770(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83254770 size=64
    let mut pc: u32 = 0x83254770;
    'dispatch: loop {
        match pc {
            0x83254770 => {
    //   block [0x83254770..0x832547B0)
	// 83254770: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83254774: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83254778: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325477C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83254780: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83254784: 388B1AD4  addi r4, r11, 0x1ad4
	ctx.r[4].s64 = ctx.r[11].s64 + 6868;
	// 83254788: 386A84C8  addi r3, r10, -0x7b38
	ctx.r[3].s64 = ctx.r[10].s64 + -31544;
	// 8325478C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83254790: 4AFD8741  bl 0x8222ced0
	ctx.lr = 0x83254794;
	sub_8222CED0(ctx, base);
	// 83254794: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83254798: 3869A2F0  addi r3, r9, -0x5d10
	ctx.r[3].s64 = ctx.r[9].s64 + -23824;
	// 8325479C: 4BA55785  bl 0x82ca9f20
	ctx.lr = 0x832547A0;
	sub_82CA9F20(ctx, base);
	// 832547A0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832547A4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832547A8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832547AC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832547B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832547B0 size=64
    let mut pc: u32 = 0x832547B0;
    'dispatch: loop {
        match pc {
            0x832547B0 => {
    //   block [0x832547B0..0x832547F0)
	// 832547B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832547B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832547B8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832547BC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 832547C0: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 832547C4: 388B1ADC  addi r4, r11, 0x1adc
	ctx.r[4].s64 = ctx.r[11].s64 + 6876;
	// 832547C8: 386A84CC  addi r3, r10, -0x7b34
	ctx.r[3].s64 = ctx.r[10].s64 + -31540;
	// 832547CC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 832547D0: 4AFD8701  bl 0x8222ced0
	ctx.lr = 0x832547D4;
	sub_8222CED0(ctx, base);
	// 832547D4: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 832547D8: 3869A300  addi r3, r9, -0x5d00
	ctx.r[3].s64 = ctx.r[9].s64 + -23808;
	// 832547DC: 4BA55745  bl 0x82ca9f20
	ctx.lr = 0x832547E0;
	sub_82CA9F20(ctx, base);
	// 832547E0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 832547E4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832547E8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832547EC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832547F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832547F0 size=64
    let mut pc: u32 = 0x832547F0;
    'dispatch: loop {
        match pc {
            0x832547F0 => {
    //   block [0x832547F0..0x83254830)
	// 832547F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832547F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832547F8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832547FC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 83254800: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83254804: 388B1AF0  addi r4, r11, 0x1af0
	ctx.r[4].s64 = ctx.r[11].s64 + 6896;
	// 83254808: 386A84D0  addi r3, r10, -0x7b30
	ctx.r[3].s64 = ctx.r[10].s64 + -31536;
	// 8325480C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83254810: 4AFD86C1  bl 0x8222ced0
	ctx.lr = 0x83254814;
	sub_8222CED0(ctx, base);
	// 83254814: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83254818: 3869A310  addi r3, r9, -0x5cf0
	ctx.r[3].s64 = ctx.r[9].s64 + -23792;
	// 8325481C: 4BA55705  bl 0x82ca9f20
	ctx.lr = 0x83254820;
	sub_82CA9F20(ctx, base);
	// 83254820: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83254824: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83254828: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325482C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83254830(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83254830 size=64
    let mut pc: u32 = 0x83254830;
    'dispatch: loop {
        match pc {
            0x83254830 => {
    //   block [0x83254830..0x83254870)
	// 83254830: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83254834: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83254838: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8325483C: 3D60820C  lis r11, -0x7df4
	ctx.r[11].s64 = -2113142784;
	// 83254840: 3D40834A  lis r10, -0x7cb6
	ctx.r[10].s64 = -2092302336;
	// 83254844: 388BEBA8  addi r4, r11, -0x1458
	ctx.r[4].s64 = ctx.r[11].s64 + -5208;
	// 83254848: 386A84D4  addi r3, r10, -0x7b2c
	ctx.r[3].s64 = ctx.r[10].s64 + -31532;
	// 8325484C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83254850: 4AFD8681  bl 0x8222ced0
	ctx.lr = 0x83254854;
	sub_8222CED0(ctx, base);
	// 83254854: 3D20832B  lis r9, -0x7cd5
	ctx.r[9].s64 = -2094333952;
	// 83254858: 3869A320  addi r3, r9, -0x5ce0
	ctx.r[3].s64 = ctx.r[9].s64 + -23776;
	// 8325485C: 4BA556C5  bl 0x82ca9f20
	ctx.lr = 0x83254860;
	sub_82CA9F20(ctx, base);
	// 83254860: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83254864: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83254868: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8325486C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


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


