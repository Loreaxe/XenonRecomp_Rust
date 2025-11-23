pub fn sub_8262BEC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8262BEC8 size=112
    let mut pc: u32 = 0x8262BEC8;
    'dispatch: loop {
        match pc {
            0x8262BEC8 => {
    //   block [0x8262BEC8..0x8262BF38)
	// 8262BEC8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262BECC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8262BED0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8262BED4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262BED8: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8262BEDC: 38AA4B7C  addi r5, r10, 0x4b7c
	ctx.r[5].s64 = ctx.r[10].s64 + 19324;
	// 8262BEE0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8262BEE4: 390B8F14  addi r8, r11, -0x70ec
	ctx.r[8].s64 = ctx.r[11].s64 + -28908;
	// 8262BEE8: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8262BEEC: 388A4BE8  addi r4, r10, 0x4be8
	ctx.r[4].s64 = ctx.r[10].s64 + 19432;
	// 8262BEF0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8262BEF4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262BEF8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8262BEFC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8262BF00: 386A4BAC  addi r3, r10, 0x4bac
	ctx.r[3].s64 = ctx.r[10].s64 + 19372;
	// 8262BF04: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8262BF08: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8262BF0C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8262BF10: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8262BF14: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8262BF18: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8262BF1C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8262BF20: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8262BF24: 4BE3AEFD  bl 0x82466e20
	ctx.lr = 0x8262BF28;
	sub_82466E20(ctx, base);
	// 8262BF28: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8262BF2C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8262BF30: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8262BF34: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8262BF38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8262BF38 size=112
    let mut pc: u32 = 0x8262BF38;
    'dispatch: loop {
        match pc {
            0x8262BF38 => {
    //   block [0x8262BF38..0x8262BFA8)
	// 8262BF38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262BF3C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8262BF40: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8262BF44: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262BF48: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8262BF4C: 38AA4BAC  addi r5, r10, 0x4bac
	ctx.r[5].s64 = ctx.r[10].s64 + 19372;
	// 8262BF50: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8262BF54: 390B8F2C  addi r8, r11, -0x70d4
	ctx.r[8].s64 = ctx.r[11].s64 + -28884;
	// 8262BF58: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8262BF5C: 388A4BF8  addi r4, r10, 0x4bf8
	ctx.r[4].s64 = ctx.r[10].s64 + 19448;
	// 8262BF60: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8262BF64: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262BF68: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8262BF6C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8262BF70: 386A4BDC  addi r3, r10, 0x4bdc
	ctx.r[3].s64 = ctx.r[10].s64 + 19420;
	// 8262BF74: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8262BF78: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8262BF7C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8262BF80: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8262BF84: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8262BF88: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8262BF8C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8262BF90: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8262BF94: 4BE3AE8D  bl 0x82466e20
	ctx.lr = 0x8262BF98;
	sub_82466E20(ctx, base);
	// 8262BF98: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8262BF9C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8262BFA0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8262BFA4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8262BFA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8262BFA8 size=116
    let mut pc: u32 = 0x8262BFA8;
    'dispatch: loop {
        match pc {
            0x8262BFA8 => {
    //   block [0x8262BFA8..0x8262C01C)
	// 8262BFA8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262BFAC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8262BFB0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8262BFB4: 3D408277  lis r10, -0x7d89
	ctx.r[10].s64 = -2106130432;
	// 8262BFB8: 38E00007  li r7, 7
	ctx.r[7].s64 = 7;
	// 8262BFBC: 390A8F48  addi r8, r10, -0x70b8
	ctx.r[8].s64 = ctx.r[10].s64 + -28856;
	// 8262BFC0: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262BFC4: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 8262BFC8: 38AA23EC  addi r5, r10, 0x23ec
	ctx.r[5].s64 = ctx.r[10].s64 + 9196;
	// 8262BFCC: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8262BFD0: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8262BFD4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8262BFD8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8262BFDC: 388A4C10  addi r4, r10, 0x4c10
	ctx.r[4].s64 = ctx.r[10].s64 + 19472;
	// 8262BFE0: 396B3B0C  addi r11, r11, 0x3b0c
	ctx.r[11].s64 = ctx.r[11].s64 + 15116;
	// 8262BFE4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262BFE8: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8262BFEC: 386A4C0C  addi r3, r10, 0x4c0c
	ctx.r[3].s64 = ctx.r[10].s64 + 19468;
	// 8262BFF0: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 8262BFF4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8262BFF8: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 8262BFFC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8262C000: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8262C004: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8262C008: 4BE3AE19  bl 0x82466e20
	ctx.lr = 0x8262C00C;
	sub_82466E20(ctx, base);
	// 8262C00C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8262C010: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8262C014: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8262C018: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8262C020(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8262C020 size=48
    let mut pc: u32 = 0x8262C020;
    'dispatch: loop {
        match pc {
            0x8262C020 => {
    //   block [0x8262C020..0x8262C050)
	// 8262C020: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8262C024: 814B8FF8  lwz r10, -0x7008(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-28680 as u32) ) } as u64;
	// 8262C028: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8262C02C: 396BB728  addi r11, r11, -0x48d8
	ctx.r[11].s64 = ctx.r[11].s64 + -18648;
	// 8262C030: 914B0050  stw r10, 0x50(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 8262C034: 3D408277  lis r10, -0x7d89
	ctx.r[10].s64 = -2106130432;
	// 8262C038: 814A8FF4  lwz r10, -0x700c(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-28684 as u32) ) } as u64;
	// 8262C03C: 914B0128  stw r10, 0x128(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(296 as u32), ctx.r[10].u32 ) };
	// 8262C040: 3D408277  lis r10, -0x7d89
	ctx.r[10].s64 = -2106130432;
	// 8262C044: 814A8FF0  lwz r10, -0x7010(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-28688 as u32) ) } as u64;
	// 8262C048: 914B0278  stw r10, 0x278(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(632 as u32), ctx.r[10].u32 ) };
	// 8262C04C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8262C050(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8262C050 size=116
    let mut pc: u32 = 0x8262C050;
    'dispatch: loop {
        match pc {
            0x8262C050 => {
    //   block [0x8262C050..0x8262C0C4)
	// 8262C050: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262C054: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8262C058: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8262C05C: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 8262C060: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262C064: 392B3BE0  addi r9, r11, 0x3be0
	ctx.r[9].s64 = ctx.r[11].s64 + 15328;
	// 8262C068: 38AA23EC  addi r5, r10, 0x23ec
	ctx.r[5].s64 = ctx.r[10].s64 + 9196;
	// 8262C06C: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8262C070: 38E90050  addi r7, r9, 0x50
	ctx.r[7].s64 = ctx.r[9].s64 + 80;
	// 8262C074: 38C0001F  li r6, 0x1f
	ctx.r[6].s64 = 31;
	// 8262C078: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8262C07C: 388A4FC8  addi r4, r10, 0x4fc8
	ctx.r[4].s64 = ctx.r[10].s64 + 20424;
	// 8262C080: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8262C084: 396BB728  addi r11, r11, -0x48d8
	ctx.r[11].s64 = ctx.r[11].s64 + -18648;
	// 8262C088: 90E10064  stw r7, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[7].u32 ) };
	// 8262C08C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262C090: 90C1005C  stw r6, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[6].u32 ) };
	// 8262C094: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8262C098: 386A4C3C  addi r3, r10, 0x4c3c
	ctx.r[3].s64 = ctx.r[10].s64 + 19516;
	// 8262C09C: 39400004  li r10, 4
	ctx.r[10].s64 = 4;
	// 8262C0A0: 91010074  stw r8, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[8].u32 ) };
	// 8262C0A4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8262C0A8: 9101006C  stw r8, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[8].u32 ) };
	// 8262C0AC: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8262C0B0: 4BE3AD71  bl 0x82466e20
	ctx.lr = 0x8262C0B4;
	sub_82466E20(ctx, base);
	// 8262C0B4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8262C0B8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8262C0BC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8262C0C0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8262C0C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8262C0C8 size=116
    let mut pc: u32 = 0x8262C0C8;
    'dispatch: loop {
        match pc {
            0x8262C0C8 => {
    //   block [0x8262C0C8..0x8262C13C)
	// 8262C0C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262C0CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8262C0D0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8262C0D4: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8262C0D8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8262C0DC: 390B9000  addi r8, r11, -0x7000
	ctx.r[8].s64 = ctx.r[11].s64 + -28672;
	// 8262C0E0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8262C0E4: 392A3D2C  addi r9, r10, 0x3d2c
	ctx.r[9].s64 = ctx.r[10].s64 + 15660;
	// 8262C0E8: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262C0EC: 38E00006  li r7, 6
	ctx.r[7].s64 = 6;
	// 8262C0F0: 38AA23EC  addi r5, r10, 0x23ec
	ctx.r[5].s64 = ctx.r[10].s64 + 9196;
	// 8262C0F4: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8262C0F8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8262C0FC: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8262C100: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8262C104: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8262C108: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8262C10C: 3D608294  lis r11, -0x7d6c
	ctx.r[11].s64 = -2104229888;
	// 8262C110: 388A4FD8  addi r4, r10, 0x4fd8
	ctx.r[4].s64 = ctx.r[10].s64 + 20440;
	// 8262C114: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8262C118: 386B4C6C  addi r3, r11, 0x4c6c
	ctx.r[3].s64 = ctx.r[11].s64 + 19564;
	// 8262C11C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8262C120: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8262C124: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8262C128: 4BE3ACF9  bl 0x82466e20
	ctx.lr = 0x8262C12C;
	sub_82466E20(ctx, base);
	// 8262C12C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8262C130: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8262C134: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8262C138: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8262C140(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8262C140 size=112
    let mut pc: u32 = 0x8262C140;
    'dispatch: loop {
        match pc {
            0x8262C140 => {
    //   block [0x8262C140..0x8262C1B0)
	// 8262C140: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262C144: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8262C148: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8262C14C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262C150: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8262C154: 38AA23EC  addi r5, r10, 0x23ec
	ctx.r[5].s64 = ctx.r[10].s64 + 9196;
	// 8262C158: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8262C15C: 390B9090  addi r8, r11, -0x6f70
	ctx.r[8].s64 = ctx.r[11].s64 + -28528;
	// 8262C160: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8262C164: 388A4FE8  addi r4, r10, 0x4fe8
	ctx.r[4].s64 = ctx.r[10].s64 + 20456;
	// 8262C168: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8262C16C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262C170: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8262C174: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8262C178: 386A4C9C  addi r3, r10, 0x4c9c
	ctx.r[3].s64 = ctx.r[10].s64 + 19612;
	// 8262C17C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8262C180: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8262C184: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8262C188: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8262C18C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8262C190: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8262C194: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8262C198: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8262C19C: 4BE3AC85  bl 0x82466e20
	ctx.lr = 0x8262C1A0;
	sub_82466E20(ctx, base);
	// 8262C1A0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8262C1A4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8262C1A8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8262C1AC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8262C1B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8262C1B0 size=112
    let mut pc: u32 = 0x8262C1B0;
    'dispatch: loop {
        match pc {
            0x8262C1B0 => {
    //   block [0x8262C1B0..0x8262C220)
	// 8262C1B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262C1B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8262C1B8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8262C1BC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262C1C0: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8262C1C4: 38AA2ECC  addi r5, r10, 0x2ecc
	ctx.r[5].s64 = ctx.r[10].s64 + 11980;
	// 8262C1C8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8262C1CC: 390B90A8  addi r8, r11, -0x6f58
	ctx.r[8].s64 = ctx.r[11].s64 + -28504;
	// 8262C1D0: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8262C1D4: 388A5000  addi r4, r10, 0x5000
	ctx.r[4].s64 = ctx.r[10].s64 + 20480;
	// 8262C1D8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8262C1DC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262C1E0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8262C1E4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8262C1E8: 386A4CCC  addi r3, r10, 0x4ccc
	ctx.r[3].s64 = ctx.r[10].s64 + 19660;
	// 8262C1EC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8262C1F0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8262C1F4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8262C1F8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8262C1FC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8262C200: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8262C204: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8262C208: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8262C20C: 4BE3AC15  bl 0x82466e20
	ctx.lr = 0x8262C210;
	sub_82466E20(ctx, base);
	// 8262C210: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8262C214: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8262C218: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8262C21C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8262C220(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8262C220 size=108
    let mut pc: u32 = 0x8262C220;
    'dispatch: loop {
        match pc {
            0x8262C220 => {
    //   block [0x8262C220..0x8262C28C)
	// 8262C220: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262C224: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8262C228: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8262C22C: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8262C230: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8262C234: 38EB90C0  addi r7, r11, -0x6f40
	ctx.r[7].s64 = ctx.r[11].s64 + -28480;
	// 8262C238: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 8262C23C: 388A5014  addi r4, r10, 0x5014
	ctx.r[4].s64 = ctx.r[10].s64 + 20500;
	// 8262C240: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8262C244: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262C248: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8262C24C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8262C250: 386A4CFC  addi r3, r10, 0x4cfc
	ctx.r[3].s64 = ctx.r[10].s64 + 19708;
	// 8262C254: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8262C258: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8262C25C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8262C260: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8262C264: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8262C268: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8262C26C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8262C270: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8262C274: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8262C278: 4BE3ABA9  bl 0x82466e20
	ctx.lr = 0x8262C27C;
	sub_82466E20(ctx, base);
	// 8262C27C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8262C280: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8262C284: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8262C288: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8262C290(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8262C290 size=112
    let mut pc: u32 = 0x8262C290;
    'dispatch: loop {
        match pc {
            0x8262C290 => {
    //   block [0x8262C290..0x8262C300)
	// 8262C290: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262C294: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8262C298: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8262C29C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262C2A0: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8262C2A4: 38AA23EC  addi r5, r10, 0x23ec
	ctx.r[5].s64 = ctx.r[10].s64 + 9196;
	// 8262C2A8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8262C2AC: 390B90D8  addi r8, r11, -0x6f28
	ctx.r[8].s64 = ctx.r[11].s64 + -28456;
	// 8262C2B0: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 8262C2B4: 388A5028  addi r4, r10, 0x5028
	ctx.r[4].s64 = ctx.r[10].s64 + 20520;
	// 8262C2B8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8262C2BC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262C2C0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8262C2C4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8262C2C8: 386A4D2C  addi r3, r10, 0x4d2c
	ctx.r[3].s64 = ctx.r[10].s64 + 19756;
	// 8262C2CC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8262C2D0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8262C2D4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8262C2D8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8262C2DC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8262C2E0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8262C2E4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8262C2E8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8262C2EC: 4BE3AB35  bl 0x82466e20
	ctx.lr = 0x8262C2F0;
	sub_82466E20(ctx, base);
	// 8262C2F0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8262C2F4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8262C2F8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8262C2FC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8262C300(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8262C300 size=108
    let mut pc: u32 = 0x8262C300;
    'dispatch: loop {
        match pc {
            0x8262C300 => {
    //   block [0x8262C300..0x8262C36C)
	// 8262C300: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262C304: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8262C308: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8262C30C: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8262C310: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8262C314: 38EB9120  addi r7, r11, -0x6ee0
	ctx.r[7].s64 = ctx.r[11].s64 + -28384;
	// 8262C318: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 8262C31C: 388A505C  addi r4, r10, 0x505c
	ctx.r[4].s64 = ctx.r[10].s64 + 20572;
	// 8262C320: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8262C324: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262C328: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8262C32C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8262C330: 386A4D5C  addi r3, r10, 0x4d5c
	ctx.r[3].s64 = ctx.r[10].s64 + 19804;
	// 8262C334: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8262C338: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8262C33C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8262C340: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8262C344: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8262C348: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8262C34C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8262C350: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8262C354: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8262C358: 4BE3AAC9  bl 0x82466e20
	ctx.lr = 0x8262C35C;
	sub_82466E20(ctx, base);
	// 8262C35C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8262C360: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8262C364: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8262C368: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8262C370(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8262C370 size=112
    let mut pc: u32 = 0x8262C370;
    'dispatch: loop {
        match pc {
            0x8262C370 => {
    //   block [0x8262C370..0x8262C3E0)
	// 8262C370: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262C374: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8262C378: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8262C37C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262C380: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8262C384: 38AA23EC  addi r5, r10, 0x23ec
	ctx.r[5].s64 = ctx.r[10].s64 + 9196;
	// 8262C388: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8262C38C: 390B9138  addi r8, r11, -0x6ec8
	ctx.r[8].s64 = ctx.r[11].s64 + -28360;
	// 8262C390: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 8262C394: 388A5070  addi r4, r10, 0x5070
	ctx.r[4].s64 = ctx.r[10].s64 + 20592;
	// 8262C398: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8262C39C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262C3A0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8262C3A4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8262C3A8: 386A4D8C  addi r3, r10, 0x4d8c
	ctx.r[3].s64 = ctx.r[10].s64 + 19852;
	// 8262C3AC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8262C3B0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8262C3B4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8262C3B8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8262C3BC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8262C3C0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8262C3C4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8262C3C8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8262C3CC: 4BE3AA55  bl 0x82466e20
	ctx.lr = 0x8262C3D0;
	sub_82466E20(ctx, base);
	// 8262C3D0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8262C3D4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8262C3D8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8262C3DC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8262C3E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8262C3E0 size=112
    let mut pc: u32 = 0x8262C3E0;
    'dispatch: loop {
        match pc {
            0x8262C3E0 => {
    //   block [0x8262C3E0..0x8262C450)
	// 8262C3E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262C3E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8262C3E8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8262C3EC: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8262C3F0: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8262C3F4: 392A3D84  addi r9, r10, 0x3d84
	ctx.r[9].s64 = ctx.r[10].s64 + 15748;
	// 8262C3F8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8262C3FC: 390B9170  addi r8, r11, -0x6e90
	ctx.r[8].s64 = ctx.r[11].s64 + -28304;
	// 8262C400: 38E00007  li r7, 7
	ctx.r[7].s64 = 7;
	// 8262C404: 388A508C  addi r4, r10, 0x508c
	ctx.r[4].s64 = ctx.r[10].s64 + 20620;
	// 8262C408: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8262C40C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262C410: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8262C414: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8262C418: 386A4DBC  addi r3, r10, 0x4dbc
	ctx.r[3].s64 = ctx.r[10].s64 + 19900;
	// 8262C41C: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8262C420: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8262C424: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8262C428: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8262C42C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8262C430: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8262C434: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8262C438: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8262C43C: 4BE3A9E5  bl 0x82466e20
	ctx.lr = 0x8262C440;
	sub_82466E20(ctx, base);
	// 8262C440: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8262C444: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8262C448: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8262C44C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8262C450(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8262C450 size=116
    let mut pc: u32 = 0x8262C450;
    'dispatch: loop {
        match pc {
            0x8262C450 => {
    //   block [0x8262C450..0x8262C4C4)
	// 8262C450: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262C454: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8262C458: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8262C45C: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8262C460: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8262C464: 390B9218  addi r8, r11, -0x6de8
	ctx.r[8].s64 = ctx.r[11].s64 + -28136;
	// 8262C468: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8262C46C: 392A3D58  addi r9, r10, 0x3d58
	ctx.r[9].s64 = ctx.r[10].s64 + 15704;
	// 8262C470: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262C474: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
	// 8262C478: 38AA403C  addi r5, r10, 0x403c
	ctx.r[5].s64 = ctx.r[10].s64 + 16444;
	// 8262C47C: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8262C480: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8262C484: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8262C488: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8262C48C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8262C490: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8262C494: 3D608294  lis r11, -0x7d6c
	ctx.r[11].s64 = -2104229888;
	// 8262C498: 388A50AC  addi r4, r10, 0x50ac
	ctx.r[4].s64 = ctx.r[10].s64 + 20652;
	// 8262C49C: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8262C4A0: 386B4DEC  addi r3, r11, 0x4dec
	ctx.r[3].s64 = ctx.r[11].s64 + 19948;
	// 8262C4A4: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8262C4A8: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8262C4AC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8262C4B0: 4BE3A971  bl 0x82466e20
	ctx.lr = 0x8262C4B4;
	sub_82466E20(ctx, base);
	// 8262C4B4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8262C4B8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8262C4BC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8262C4C0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8262C4C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8262C4C8 size=112
    let mut pc: u32 = 0x8262C4C8;
    'dispatch: loop {
        match pc {
            0x8262C4C8 => {
    //   block [0x8262C4C8..0x8262C538)
	// 8262C4C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262C4CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8262C4D0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8262C4D4: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8262C4D8: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8262C4DC: 392A3DB0  addi r9, r10, 0x3db0
	ctx.r[9].s64 = ctx.r[10].s64 + 15792;
	// 8262C4E0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8262C4E4: 390B9238  addi r8, r11, -0x6dc8
	ctx.r[8].s64 = ctx.r[11].s64 + -28104;
	// 8262C4E8: 38E00004  li r7, 4
	ctx.r[7].s64 = 4;
	// 8262C4EC: 388A50C4  addi r4, r10, 0x50c4
	ctx.r[4].s64 = ctx.r[10].s64 + 20676;
	// 8262C4F0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8262C4F4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262C4F8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8262C4FC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8262C500: 386A4E1C  addi r3, r10, 0x4e1c
	ctx.r[3].s64 = ctx.r[10].s64 + 19996;
	// 8262C504: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8262C508: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8262C50C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8262C510: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8262C514: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8262C518: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8262C51C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8262C520: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8262C524: 4BE3A8FD  bl 0x82466e20
	ctx.lr = 0x8262C528;
	sub_82466E20(ctx, base);
	// 8262C528: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8262C52C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8262C530: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8262C534: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8262C538(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8262C538 size=112
    let mut pc: u32 = 0x8262C538;
    'dispatch: loop {
        match pc {
            0x8262C538 => {
    //   block [0x8262C538..0x8262C5A8)
	// 8262C538: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262C53C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8262C540: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8262C544: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262C548: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8262C54C: 38AA403C  addi r5, r10, 0x403c
	ctx.r[5].s64 = ctx.r[10].s64 + 16444;
	// 8262C550: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8262C554: 390B9298  addi r8, r11, -0x6d68
	ctx.r[8].s64 = ctx.r[11].s64 + -28008;
	// 8262C558: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8262C55C: 388A50E0  addi r4, r10, 0x50e0
	ctx.r[4].s64 = ctx.r[10].s64 + 20704;
	// 8262C560: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8262C564: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262C568: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8262C56C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8262C570: 386A4E4C  addi r3, r10, 0x4e4c
	ctx.r[3].s64 = ctx.r[10].s64 + 20044;
	// 8262C574: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8262C578: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8262C57C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8262C580: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8262C584: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8262C588: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8262C58C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8262C590: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8262C594: 4BE3A88D  bl 0x82466e20
	ctx.lr = 0x8262C598;
	sub_82466E20(ctx, base);
	// 8262C598: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8262C59C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8262C5A0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8262C5A4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8262C5A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8262C5A8 size=112
    let mut pc: u32 = 0x8262C5A8;
    'dispatch: loop {
        match pc {
            0x8262C5A8 => {
    //   block [0x8262C5A8..0x8262C618)
	// 8262C5A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262C5AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8262C5B0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8262C5B4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262C5B8: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8262C5BC: 38AA3F4C  addi r5, r10, 0x3f4c
	ctx.r[5].s64 = ctx.r[10].s64 + 16204;
	// 8262C5C0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8262C5C4: 390B92B0  addi r8, r11, -0x6d50
	ctx.r[8].s64 = ctx.r[11].s64 + -27984;
	// 8262C5C8: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 8262C5CC: 388A50F4  addi r4, r10, 0x50f4
	ctx.r[4].s64 = ctx.r[10].s64 + 20724;
	// 8262C5D0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8262C5D4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262C5D8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8262C5DC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8262C5E0: 386A4E7C  addi r3, r10, 0x4e7c
	ctx.r[3].s64 = ctx.r[10].s64 + 20092;
	// 8262C5E4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8262C5E8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8262C5EC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8262C5F0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8262C5F4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8262C5F8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8262C5FC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8262C600: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8262C604: 4BE3A81D  bl 0x82466e20
	ctx.lr = 0x8262C608;
	sub_82466E20(ctx, base);
	// 8262C608: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8262C60C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8262C610: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8262C614: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8262C618(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8262C618 size=112
    let mut pc: u32 = 0x8262C618;
    'dispatch: loop {
        match pc {
            0x8262C618 => {
    //   block [0x8262C618..0x8262C688)
	// 8262C618: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262C61C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8262C620: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8262C624: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262C628: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8262C62C: 38AA3F4C  addi r5, r10, 0x3f4c
	ctx.r[5].s64 = ctx.r[10].s64 + 16204;
	// 8262C630: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8262C634: 390B92F8  addi r8, r11, -0x6d08
	ctx.r[8].s64 = ctx.r[11].s64 + -27912;
	// 8262C638: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 8262C63C: 388A510C  addi r4, r10, 0x510c
	ctx.r[4].s64 = ctx.r[10].s64 + 20748;
	// 8262C640: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8262C644: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262C648: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8262C64C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8262C650: 386A4EAC  addi r3, r10, 0x4eac
	ctx.r[3].s64 = ctx.r[10].s64 + 20140;
	// 8262C654: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8262C658: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8262C65C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8262C660: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8262C664: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8262C668: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8262C66C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8262C670: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8262C674: 4BE3A7AD  bl 0x82466e20
	ctx.lr = 0x8262C678;
	sub_82466E20(ctx, base);
	// 8262C678: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8262C67C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8262C680: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8262C684: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8262C688(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8262C688 size=112
    let mut pc: u32 = 0x8262C688;
    'dispatch: loop {
        match pc {
            0x8262C688 => {
    //   block [0x8262C688..0x8262C6F8)
	// 8262C688: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262C68C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8262C690: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8262C694: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262C698: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8262C69C: 38AA3F7C  addi r5, r10, 0x3f7c
	ctx.r[5].s64 = ctx.r[10].s64 + 16252;
	// 8262C6A0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8262C6A4: 390B9358  addi r8, r11, -0x6ca8
	ctx.r[8].s64 = ctx.r[11].s64 + -27816;
	// 8262C6A8: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 8262C6AC: 388A511C  addi r4, r10, 0x511c
	ctx.r[4].s64 = ctx.r[10].s64 + 20764;
	// 8262C6B0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8262C6B4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262C6B8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8262C6BC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8262C6C0: 386A4EDC  addi r3, r10, 0x4edc
	ctx.r[3].s64 = ctx.r[10].s64 + 20188;
	// 8262C6C4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8262C6C8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8262C6CC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8262C6D0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8262C6D4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8262C6D8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8262C6DC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8262C6E0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8262C6E4: 4BE3A73D  bl 0x82466e20
	ctx.lr = 0x8262C6E8;
	sub_82466E20(ctx, base);
	// 8262C6E8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8262C6EC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8262C6F0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8262C6F4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8262C6F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8262C6F8 size=112
    let mut pc: u32 = 0x8262C6F8;
    'dispatch: loop {
        match pc {
            0x8262C6F8 => {
    //   block [0x8262C6F8..0x8262C768)
	// 8262C6F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262C6FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8262C700: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8262C704: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262C708: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8262C70C: 38AA3F7C  addi r5, r10, 0x3f7c
	ctx.r[5].s64 = ctx.r[10].s64 + 16252;
	// 8262C710: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8262C714: 390B93B8  addi r8, r11, -0x6c48
	ctx.r[8].s64 = ctx.r[11].s64 + -27720;
	// 8262C718: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 8262C71C: 388A512C  addi r4, r10, 0x512c
	ctx.r[4].s64 = ctx.r[10].s64 + 20780;
	// 8262C720: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8262C724: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262C728: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8262C72C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8262C730: 386A4F0C  addi r3, r10, 0x4f0c
	ctx.r[3].s64 = ctx.r[10].s64 + 20236;
	// 8262C734: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8262C738: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8262C73C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8262C740: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8262C744: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8262C748: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8262C74C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8262C750: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8262C754: 4BE3A6CD  bl 0x82466e20
	ctx.lr = 0x8262C758;
	sub_82466E20(ctx, base);
	// 8262C758: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8262C75C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8262C760: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8262C764: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8262C768(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8262C768 size=112
    let mut pc: u32 = 0x8262C768;
    'dispatch: loop {
        match pc {
            0x8262C768 => {
    //   block [0x8262C768..0x8262C7D8)
	// 8262C768: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262C76C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8262C770: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8262C774: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262C778: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8262C77C: 38AA3F4C  addi r5, r10, 0x3f4c
	ctx.r[5].s64 = ctx.r[10].s64 + 16204;
	// 8262C780: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8262C784: 390B9418  addi r8, r11, -0x6be8
	ctx.r[8].s64 = ctx.r[11].s64 + -27624;
	// 8262C788: 39200008  li r9, 8
	ctx.r[9].s64 = 8;
	// 8262C78C: 388A5140  addi r4, r10, 0x5140
	ctx.r[4].s64 = ctx.r[10].s64 + 20800;
	// 8262C790: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8262C794: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262C798: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8262C79C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8262C7A0: 386A4F3C  addi r3, r10, 0x4f3c
	ctx.r[3].s64 = ctx.r[10].s64 + 20284;
	// 8262C7A4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8262C7A8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8262C7AC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8262C7B0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8262C7B4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8262C7B8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8262C7BC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8262C7C0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8262C7C4: 4BE3A65D  bl 0x82466e20
	ctx.lr = 0x8262C7C8;
	sub_82466E20(ctx, base);
	// 8262C7C8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8262C7CC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8262C7D0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8262C7D4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8262C7D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8262C7D8 size=112
    let mut pc: u32 = 0x8262C7D8;
    'dispatch: loop {
        match pc {
            0x8262C7D8 => {
    //   block [0x8262C7D8..0x8262C848)
	// 8262C7D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262C7DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8262C7E0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8262C7E4: 3D408277  lis r10, -0x7d89
	ctx.r[10].s64 = -2106130432;
	// 8262C7E8: 39000013  li r8, 0x13
	ctx.r[8].s64 = 19;
	// 8262C7EC: 38EA94D8  addi r7, r10, -0x6b28
	ctx.r[7].s64 = ctx.r[10].s64 + -27432;
	// 8262C7F0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8262C7F4: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 8262C7F8: 388A5150  addi r4, r10, 0x5150
	ctx.r[4].s64 = ctx.r[10].s64 + 20816;
	// 8262C7FC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8262C800: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8262C804: 396B3DC8  addi r11, r11, 0x3dc8
	ctx.r[11].s64 = ctx.r[11].s64 + 15816;
	// 8262C808: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8262C80C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262C810: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8262C814: 386A4F6C  addi r3, r10, 0x4f6c
	ctx.r[3].s64 = ctx.r[10].s64 + 20332;
	// 8262C818: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8262C81C: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 8262C820: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8262C824: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 8262C828: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8262C82C: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8262C830: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8262C834: 4BE3A5ED  bl 0x82466e20
	ctx.lr = 0x8262C838;
	sub_82466E20(ctx, base);
	// 8262C838: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8262C83C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8262C840: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8262C844: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8262C848(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8262C848 size=112
    let mut pc: u32 = 0x8262C848;
    'dispatch: loop {
        match pc {
            0x8262C848 => {
    //   block [0x8262C848..0x8262C8B8)
	// 8262C848: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262C84C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8262C850: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8262C854: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262C858: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8262C85C: 38AA2F8C  addi r5, r10, 0x2f8c
	ctx.r[5].s64 = ctx.r[10].s64 + 12172;
	// 8262C860: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8262C864: 390B96A0  addi r8, r11, -0x6960
	ctx.r[8].s64 = ctx.r[11].s64 + -26976;
	// 8262C868: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8262C86C: 388A5168  addi r4, r10, 0x5168
	ctx.r[4].s64 = ctx.r[10].s64 + 20840;
	// 8262C870: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8262C874: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262C878: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8262C87C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8262C880: 386A4F9C  addi r3, r10, 0x4f9c
	ctx.r[3].s64 = ctx.r[10].s64 + 20380;
	// 8262C884: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8262C888: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8262C88C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8262C890: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8262C894: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 8262C898: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8262C89C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8262C8A0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8262C8A4: 4BE3A57D  bl 0x82466e20
	ctx.lr = 0x8262C8A8;
	sub_82466E20(ctx, base);
	// 8262C8A8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8262C8AC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8262C8B0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8262C8B4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8262C8B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8262C8B8 size=112
    let mut pc: u32 = 0x8262C8B8;
    'dispatch: loop {
        match pc {
            0x8262C8B8 => {
    //   block [0x8262C8B8..0x8262C928)
	// 8262C8B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262C8BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8262C8C0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8262C8C4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262C8C8: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8262C8CC: 38AA2F8C  addi r5, r10, 0x2f8c
	ctx.r[5].s64 = ctx.r[10].s64 + 12172;
	// 8262C8D0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8262C8D4: 390B96B8  addi r8, r11, -0x6948
	ctx.r[8].s64 = ctx.r[11].s64 + -26952;
	// 8262C8D8: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8262C8DC: 388A5184  addi r4, r10, 0x5184
	ctx.r[4].s64 = ctx.r[10].s64 + 20868;
	// 8262C8E0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8262C8E4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262C8E8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8262C8EC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8262C8F0: 386A4FCC  addi r3, r10, 0x4fcc
	ctx.r[3].s64 = ctx.r[10].s64 + 20428;
	// 8262C8F4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8262C8F8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8262C8FC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8262C900: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8262C904: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 8262C908: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8262C90C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8262C910: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8262C914: 4BE3A50D  bl 0x82466e20
	ctx.lr = 0x8262C918;
	sub_82466E20(ctx, base);
	// 8262C918: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8262C91C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8262C920: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8262C924: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8262C928(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8262C928 size=112
    let mut pc: u32 = 0x8262C928;
    'dispatch: loop {
        match pc {
            0x8262C928 => {
    //   block [0x8262C928..0x8262C998)
	// 8262C928: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262C92C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8262C930: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8262C934: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262C938: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8262C93C: 38AA2F8C  addi r5, r10, 0x2f8c
	ctx.r[5].s64 = ctx.r[10].s64 + 12172;
	// 8262C940: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8262C944: 390B96D0  addi r8, r11, -0x6930
	ctx.r[8].s64 = ctx.r[11].s64 + -26928;
	// 8262C948: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 8262C94C: 388A51A4  addi r4, r10, 0x51a4
	ctx.r[4].s64 = ctx.r[10].s64 + 20900;
	// 8262C950: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8262C954: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262C958: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8262C95C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8262C960: 386A4FFC  addi r3, r10, 0x4ffc
	ctx.r[3].s64 = ctx.r[10].s64 + 20476;
	// 8262C964: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8262C968: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8262C96C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8262C970: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8262C974: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8262C978: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8262C97C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8262C980: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8262C984: 4BE3A49D  bl 0x82466e20
	ctx.lr = 0x8262C988;
	sub_82466E20(ctx, base);
	// 8262C988: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8262C98C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8262C990: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8262C994: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8262C998(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8262C998 size=108
    let mut pc: u32 = 0x8262C998;
    'dispatch: loop {
        match pc {
            0x8262C998 => {
    //   block [0x8262C998..0x8262CA04)
	// 8262C998: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262C99C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8262C9A0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8262C9A4: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8262C9A8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8262C9AC: 38EB9700  addi r7, r11, -0x6900
	ctx.r[7].s64 = ctx.r[11].s64 + -26880;
	// 8262C9B0: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 8262C9B4: 388A51BC  addi r4, r10, 0x51bc
	ctx.r[4].s64 = ctx.r[10].s64 + 20924;
	// 8262C9B8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8262C9BC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262C9C0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8262C9C4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8262C9C8: 386A502C  addi r3, r10, 0x502c
	ctx.r[3].s64 = ctx.r[10].s64 + 20524;
	// 8262C9CC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8262C9D0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8262C9D4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8262C9D8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8262C9DC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8262C9E0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8262C9E4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8262C9E8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8262C9EC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8262C9F0: 4BE3A431  bl 0x82466e20
	ctx.lr = 0x8262C9F4;
	sub_82466E20(ctx, base);
	// 8262C9F4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8262C9F8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8262C9FC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8262CA00: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8262CA08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8262CA08 size=112
    let mut pc: u32 = 0x8262CA08;
    'dispatch: loop {
        match pc {
            0x8262CA08 => {
    //   block [0x8262CA08..0x8262CA78)
	// 8262CA08: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262CA0C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8262CA10: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8262CA14: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262CA18: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8262CA1C: 38AA2F8C  addi r5, r10, 0x2f8c
	ctx.r[5].s64 = ctx.r[10].s64 + 12172;
	// 8262CA20: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8262CA24: 390B9730  addi r8, r11, -0x68d0
	ctx.r[8].s64 = ctx.r[11].s64 + -26832;
	// 8262CA28: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8262CA2C: 388A51E4  addi r4, r10, 0x51e4
	ctx.r[4].s64 = ctx.r[10].s64 + 20964;
	// 8262CA30: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8262CA34: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262CA38: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8262CA3C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8262CA40: 386A505C  addi r3, r10, 0x505c
	ctx.r[3].s64 = ctx.r[10].s64 + 20572;
	// 8262CA44: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8262CA48: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8262CA4C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8262CA50: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8262CA54: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 8262CA58: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8262CA5C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8262CA60: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8262CA64: 4BE3A3BD  bl 0x82466e20
	ctx.lr = 0x8262CA68;
	sub_82466E20(ctx, base);
	// 8262CA68: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8262CA6C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8262CA70: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8262CA74: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8262CA78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8262CA78 size=108
    let mut pc: u32 = 0x8262CA78;
    'dispatch: loop {
        match pc {
            0x8262CA78 => {
    //   block [0x8262CA78..0x8262CAE4)
	// 8262CA78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262CA7C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8262CA80: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8262CA84: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8262CA88: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8262CA8C: 38EB9748  addi r7, r11, -0x68b8
	ctx.r[7].s64 = ctx.r[11].s64 + -26808;
	// 8262CA90: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 8262CA94: 388A5200  addi r4, r10, 0x5200
	ctx.r[4].s64 = ctx.r[10].s64 + 20992;
	// 8262CA98: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8262CA9C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262CAA0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8262CAA4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8262CAA8: 386A508C  addi r3, r10, 0x508c
	ctx.r[3].s64 = ctx.r[10].s64 + 20620;
	// 8262CAAC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8262CAB0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8262CAB4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8262CAB8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8262CABC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8262CAC0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8262CAC4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8262CAC8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8262CACC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8262CAD0: 4BE3A351  bl 0x82466e20
	ctx.lr = 0x8262CAD4;
	sub_82466E20(ctx, base);
	// 8262CAD4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8262CAD8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8262CADC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8262CAE0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8262CAE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8262CAE8 size=108
    let mut pc: u32 = 0x8262CAE8;
    'dispatch: loop {
        match pc {
            0x8262CAE8 => {
    //   block [0x8262CAE8..0x8262CB54)
	// 8262CAE8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262CAEC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8262CAF0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8262CAF4: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8262CAF8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8262CAFC: 38EB9778  addi r7, r11, -0x6888
	ctx.r[7].s64 = ctx.r[11].s64 + -26760;
	// 8262CB00: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 8262CB04: 388A521C  addi r4, r10, 0x521c
	ctx.r[4].s64 = ctx.r[10].s64 + 21020;
	// 8262CB08: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8262CB0C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262CB10: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8262CB14: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8262CB18: 386A50BC  addi r3, r10, 0x50bc
	ctx.r[3].s64 = ctx.r[10].s64 + 20668;
	// 8262CB1C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8262CB20: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8262CB24: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8262CB28: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8262CB2C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8262CB30: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8262CB34: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8262CB38: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8262CB3C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8262CB40: 4BE3A2E1  bl 0x82466e20
	ctx.lr = 0x8262CB44;
	sub_82466E20(ctx, base);
	// 8262CB44: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8262CB48: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8262CB4C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8262CB50: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8262CB58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8262CB58 size=112
    let mut pc: u32 = 0x8262CB58;
    'dispatch: loop {
        match pc {
            0x8262CB58 => {
    //   block [0x8262CB58..0x8262CBC8)
	// 8262CB58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262CB5C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8262CB60: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8262CB64: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262CB68: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8262CB6C: 38AA23EC  addi r5, r10, 0x23ec
	ctx.r[5].s64 = ctx.r[10].s64 + 9196;
	// 8262CB70: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8262CB74: 390B97C0  addi r8, r11, -0x6840
	ctx.r[8].s64 = ctx.r[11].s64 + -26688;
	// 8262CB78: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 8262CB7C: 388A523C  addi r4, r10, 0x523c
	ctx.r[4].s64 = ctx.r[10].s64 + 21052;
	// 8262CB80: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8262CB84: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262CB88: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8262CB8C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8262CB90: 386A50EC  addi r3, r10, 0x50ec
	ctx.r[3].s64 = ctx.r[10].s64 + 20716;
	// 8262CB94: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8262CB98: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8262CB9C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8262CBA0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8262CBA4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8262CBA8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8262CBAC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8262CBB0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8262CBB4: 4BE3A26D  bl 0x82466e20
	ctx.lr = 0x8262CBB8;
	sub_82466E20(ctx, base);
	// 8262CBB8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8262CBBC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8262CBC0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8262CBC4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8262CBC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8262CBC8 size=112
    let mut pc: u32 = 0x8262CBC8;
    'dispatch: loop {
        match pc {
            0x8262CBC8 => {
    //   block [0x8262CBC8..0x8262CC38)
	// 8262CBC8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262CBCC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8262CBD0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8262CBD4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262CBD8: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8262CBDC: 38AA3F7C  addi r5, r10, 0x3f7c
	ctx.r[5].s64 = ctx.r[10].s64 + 16252;
	// 8262CBE0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8262CBE4: 390B9808  addi r8, r11, -0x67f8
	ctx.r[8].s64 = ctx.r[11].s64 + -26616;
	// 8262CBE8: 39200006  li r9, 6
	ctx.r[9].s64 = 6;
	// 8262CBEC: 388A5254  addi r4, r10, 0x5254
	ctx.r[4].s64 = ctx.r[10].s64 + 21076;
	// 8262CBF0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8262CBF4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262CBF8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8262CBFC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8262CC00: 386A511C  addi r3, r10, 0x511c
	ctx.r[3].s64 = ctx.r[10].s64 + 20764;
	// 8262CC04: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8262CC08: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8262CC0C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8262CC10: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8262CC14: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8262CC18: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8262CC1C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8262CC20: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8262CC24: 4BE3A1FD  bl 0x82466e20
	ctx.lr = 0x8262CC28;
	sub_82466E20(ctx, base);
	// 8262CC28: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8262CC2C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8262CC30: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8262CC34: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8262CC38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8262CC38 size=108
    let mut pc: u32 = 0x8262CC38;
    'dispatch: loop {
        match pc {
            0x8262CC38 => {
    //   block [0x8262CC38..0x8262CCA4)
	// 8262CC38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262CC3C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8262CC40: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8262CC44: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8262CC48: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8262CC4C: 38EB9898  addi r7, r11, -0x6768
	ctx.r[7].s64 = ctx.r[11].s64 + -26472;
	// 8262CC50: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 8262CC54: 388A5268  addi r4, r10, 0x5268
	ctx.r[4].s64 = ctx.r[10].s64 + 21096;
	// 8262CC58: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8262CC5C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262CC60: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8262CC64: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8262CC68: 386A514C  addi r3, r10, 0x514c
	ctx.r[3].s64 = ctx.r[10].s64 + 20812;
	// 8262CC6C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8262CC70: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8262CC74: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8262CC78: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8262CC7C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8262CC80: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8262CC84: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8262CC88: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8262CC8C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8262CC90: 4BE3A191  bl 0x82466e20
	ctx.lr = 0x8262CC94;
	sub_82466E20(ctx, base);
	// 8262CC94: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8262CC98: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8262CC9C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8262CCA0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8262CCA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8262CCA8 size=108
    let mut pc: u32 = 0x8262CCA8;
    'dispatch: loop {
        match pc {
            0x8262CCA8 => {
    //   block [0x8262CCA8..0x8262CD14)
	// 8262CCA8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262CCAC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8262CCB0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8262CCB4: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8262CCB8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8262CCBC: 38EB98E0  addi r7, r11, -0x6720
	ctx.r[7].s64 = ctx.r[11].s64 + -26400;
	// 8262CCC0: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 8262CCC4: 388A5284  addi r4, r10, 0x5284
	ctx.r[4].s64 = ctx.r[10].s64 + 21124;
	// 8262CCC8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8262CCCC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262CCD0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8262CCD4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8262CCD8: 386A517C  addi r3, r10, 0x517c
	ctx.r[3].s64 = ctx.r[10].s64 + 20860;
	// 8262CCDC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8262CCE0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8262CCE4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8262CCE8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8262CCEC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8262CCF0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8262CCF4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8262CCF8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8262CCFC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8262CD00: 4BE3A121  bl 0x82466e20
	ctx.lr = 0x8262CD04;
	sub_82466E20(ctx, base);
	// 8262CD04: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8262CD08: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8262CD0C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8262CD10: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8262CD18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8262CD18 size=108
    let mut pc: u32 = 0x8262CD18;
    'dispatch: loop {
        match pc {
            0x8262CD18 => {
    //   block [0x8262CD18..0x8262CD84)
	// 8262CD18: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262CD1C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8262CD20: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8262CD24: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8262CD28: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8262CD2C: 38EB9910  addi r7, r11, -0x66f0
	ctx.r[7].s64 = ctx.r[11].s64 + -26352;
	// 8262CD30: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 8262CD34: 388A52A4  addi r4, r10, 0x52a4
	ctx.r[4].s64 = ctx.r[10].s64 + 21156;
	// 8262CD38: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8262CD3C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262CD40: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8262CD44: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8262CD48: 386A51AC  addi r3, r10, 0x51ac
	ctx.r[3].s64 = ctx.r[10].s64 + 20908;
	// 8262CD4C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8262CD50: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8262CD54: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8262CD58: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8262CD5C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8262CD60: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8262CD64: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8262CD68: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8262CD6C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8262CD70: 4BE3A0B1  bl 0x82466e20
	ctx.lr = 0x8262CD74;
	sub_82466E20(ctx, base);
	// 8262CD74: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8262CD78: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8262CD7C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8262CD80: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8262CD88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8262CD88 size=112
    let mut pc: u32 = 0x8262CD88;
    'dispatch: loop {
        match pc {
            0x8262CD88 => {
    //   block [0x8262CD88..0x8262CDF8)
	// 8262CD88: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262CD8C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8262CD90: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8262CD94: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262CD98: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8262CD9C: 38AA23EC  addi r5, r10, 0x23ec
	ctx.r[5].s64 = ctx.r[10].s64 + 9196;
	// 8262CDA0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8262CDA4: 390B9940  addi r8, r11, -0x66c0
	ctx.r[8].s64 = ctx.r[11].s64 + -26304;
	// 8262CDA8: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 8262CDAC: 388A52BC  addi r4, r10, 0x52bc
	ctx.r[4].s64 = ctx.r[10].s64 + 21180;
	// 8262CDB0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8262CDB4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262CDB8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8262CDBC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8262CDC0: 386A51DC  addi r3, r10, 0x51dc
	ctx.r[3].s64 = ctx.r[10].s64 + 20956;
	// 8262CDC4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8262CDC8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8262CDCC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8262CDD0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8262CDD4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8262CDD8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8262CDDC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8262CDE0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8262CDE4: 4BE3A03D  bl 0x82466e20
	ctx.lr = 0x8262CDE8;
	sub_82466E20(ctx, base);
	// 8262CDE8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8262CDEC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8262CDF0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8262CDF4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8262CDF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8262CDF8 size=112
    let mut pc: u32 = 0x8262CDF8;
    'dispatch: loop {
        match pc {
            0x8262CDF8 => {
    //   block [0x8262CDF8..0x8262CE68)
	// 8262CDF8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262CDFC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8262CE00: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8262CE04: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262CE08: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8262CE0C: 38AA23EC  addi r5, r10, 0x23ec
	ctx.r[5].s64 = ctx.r[10].s64 + 9196;
	// 8262CE10: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8262CE14: 390B9970  addi r8, r11, -0x6690
	ctx.r[8].s64 = ctx.r[11].s64 + -26256;
	// 8262CE18: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8262CE1C: 388A52CC  addi r4, r10, 0x52cc
	ctx.r[4].s64 = ctx.r[10].s64 + 21196;
	// 8262CE20: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8262CE24: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262CE28: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8262CE2C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8262CE30: 386A520C  addi r3, r10, 0x520c
	ctx.r[3].s64 = ctx.r[10].s64 + 21004;
	// 8262CE34: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8262CE38: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8262CE3C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8262CE40: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8262CE44: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8262CE48: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8262CE4C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8262CE50: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8262CE54: 4BE39FCD  bl 0x82466e20
	ctx.lr = 0x8262CE58;
	sub_82466E20(ctx, base);
	// 8262CE58: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8262CE5C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8262CE60: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8262CE64: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8262CE68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8262CE68 size=112
    let mut pc: u32 = 0x8262CE68;
    'dispatch: loop {
        match pc {
            0x8262CE68 => {
    //   block [0x8262CE68..0x8262CED8)
	// 8262CE68: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262CE6C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8262CE70: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8262CE74: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262CE78: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8262CE7C: 38AA23EC  addi r5, r10, 0x23ec
	ctx.r[5].s64 = ctx.r[10].s64 + 9196;
	// 8262CE80: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8262CE84: 390B9988  addi r8, r11, -0x6678
	ctx.r[8].s64 = ctx.r[11].s64 + -26232;
	// 8262CE88: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8262CE8C: 388A52E8  addi r4, r10, 0x52e8
	ctx.r[4].s64 = ctx.r[10].s64 + 21224;
	// 8262CE90: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8262CE94: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262CE98: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8262CE9C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8262CEA0: 386A523C  addi r3, r10, 0x523c
	ctx.r[3].s64 = ctx.r[10].s64 + 21052;
	// 8262CEA4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8262CEA8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8262CEAC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8262CEB0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8262CEB4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8262CEB8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8262CEBC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8262CEC0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8262CEC4: 4BE39F5D  bl 0x82466e20
	ctx.lr = 0x8262CEC8;
	sub_82466E20(ctx, base);
	// 8262CEC8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8262CECC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8262CED0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8262CED4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8262CED8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8262CED8 size=108
    let mut pc: u32 = 0x8262CED8;
    'dispatch: loop {
        match pc {
            0x8262CED8 => {
    //   block [0x8262CED8..0x8262CF44)
	// 8262CED8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262CEDC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8262CEE0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8262CEE4: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8262CEE8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8262CEEC: 38EB99A0  addi r7, r11, -0x6660
	ctx.r[7].s64 = ctx.r[11].s64 + -26208;
	// 8262CEF0: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 8262CEF4: 388A5308  addi r4, r10, 0x5308
	ctx.r[4].s64 = ctx.r[10].s64 + 21256;
	// 8262CEF8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8262CEFC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262CF00: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8262CF04: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8262CF08: 386A526C  addi r3, r10, 0x526c
	ctx.r[3].s64 = ctx.r[10].s64 + 21100;
	// 8262CF0C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8262CF10: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8262CF14: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8262CF18: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8262CF1C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8262CF20: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8262CF24: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8262CF28: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8262CF2C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8262CF30: 4BE39EF1  bl 0x82466e20
	ctx.lr = 0x8262CF34;
	sub_82466E20(ctx, base);
	// 8262CF34: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8262CF38: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8262CF3C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8262CF40: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8262CF48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8262CF48 size=112
    let mut pc: u32 = 0x8262CF48;
    'dispatch: loop {
        match pc {
            0x8262CF48 => {
    //   block [0x8262CF48..0x8262CFB8)
	// 8262CF48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262CF4C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8262CF50: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8262CF54: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262CF58: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8262CF5C: 38AA23EC  addi r5, r10, 0x23ec
	ctx.r[5].s64 = ctx.r[10].s64 + 9196;
	// 8262CF60: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8262CF64: 390B99D0  addi r8, r11, -0x6630
	ctx.r[8].s64 = ctx.r[11].s64 + -26160;
	// 8262CF68: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8262CF6C: 388A533C  addi r4, r10, 0x533c
	ctx.r[4].s64 = ctx.r[10].s64 + 21308;
	// 8262CF70: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8262CF74: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262CF78: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8262CF7C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8262CF80: 386A529C  addi r3, r10, 0x529c
	ctx.r[3].s64 = ctx.r[10].s64 + 21148;
	// 8262CF84: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8262CF88: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8262CF8C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8262CF90: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8262CF94: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8262CF98: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8262CF9C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8262CFA0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8262CFA4: 4BE39E7D  bl 0x82466e20
	ctx.lr = 0x8262CFA8;
	sub_82466E20(ctx, base);
	// 8262CFA8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8262CFAC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8262CFB0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8262CFB4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8262CFB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8262CFB8 size=108
    let mut pc: u32 = 0x8262CFB8;
    'dispatch: loop {
        match pc {
            0x8262CFB8 => {
    //   block [0x8262CFB8..0x8262D024)
	// 8262CFB8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262CFBC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8262CFC0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8262CFC4: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8262CFC8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8262CFCC: 38EB99E8  addi r7, r11, -0x6618
	ctx.r[7].s64 = ctx.r[11].s64 + -26136;
	// 8262CFD0: 3900000A  li r8, 0xa
	ctx.r[8].s64 = 10;
	// 8262CFD4: 388A535C  addi r4, r10, 0x535c
	ctx.r[4].s64 = ctx.r[10].s64 + 21340;
	// 8262CFD8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8262CFDC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262CFE0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8262CFE4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8262CFE8: 386A52CC  addi r3, r10, 0x52cc
	ctx.r[3].s64 = ctx.r[10].s64 + 21196;
	// 8262CFEC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8262CFF0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8262CFF4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8262CFF8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8262CFFC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8262D000: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8262D004: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8262D008: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8262D00C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8262D010: 4BE39E11  bl 0x82466e20
	ctx.lr = 0x8262D014;
	sub_82466E20(ctx, base);
	// 8262D014: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8262D018: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8262D01C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8262D020: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8262D028(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8262D028 size=112
    let mut pc: u32 = 0x8262D028;
    'dispatch: loop {
        match pc {
            0x8262D028 => {
    //   block [0x8262D028..0x8262D098)
	// 8262D028: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262D02C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8262D030: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8262D034: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262D038: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8262D03C: 38AA23EC  addi r5, r10, 0x23ec
	ctx.r[5].s64 = ctx.r[10].s64 + 9196;
	// 8262D040: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8262D044: 390B9AD8  addi r8, r11, -0x6528
	ctx.r[8].s64 = ctx.r[11].s64 + -25896;
	// 8262D048: 39200012  li r9, 0x12
	ctx.r[9].s64 = 18;
	// 8262D04C: 388A5380  addi r4, r10, 0x5380
	ctx.r[4].s64 = ctx.r[10].s64 + 21376;
	// 8262D050: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8262D054: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262D058: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8262D05C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8262D060: 386A52FC  addi r3, r10, 0x52fc
	ctx.r[3].s64 = ctx.r[10].s64 + 21244;
	// 8262D064: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8262D068: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8262D06C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8262D070: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8262D074: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8262D078: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8262D07C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8262D080: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8262D084: 4BE39D9D  bl 0x82466e20
	ctx.lr = 0x8262D088;
	sub_82466E20(ctx, base);
	// 8262D088: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8262D08C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8262D090: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8262D094: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8262D098(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8262D098 size=108
    let mut pc: u32 = 0x8262D098;
    'dispatch: loop {
        match pc {
            0x8262D098 => {
    //   block [0x8262D098..0x8262D104)
	// 8262D098: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262D09C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8262D0A0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8262D0A4: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8262D0A8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8262D0AC: 38EB9C88  addi r7, r11, -0x6378
	ctx.r[7].s64 = ctx.r[11].s64 + -25464;
	// 8262D0B0: 39000011  li r8, 0x11
	ctx.r[8].s64 = 17;
	// 8262D0B4: 388A5390  addi r4, r10, 0x5390
	ctx.r[4].s64 = ctx.r[10].s64 + 21392;
	// 8262D0B8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8262D0BC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262D0C0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8262D0C4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8262D0C8: 386A532C  addi r3, r10, 0x532c
	ctx.r[3].s64 = ctx.r[10].s64 + 21292;
	// 8262D0CC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8262D0D0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8262D0D4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8262D0D8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8262D0DC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8262D0E0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8262D0E4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8262D0E8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8262D0EC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8262D0F0: 4BE39D31  bl 0x82466e20
	ctx.lr = 0x8262D0F4;
	sub_82466E20(ctx, base);
	// 8262D0F4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8262D0F8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8262D0FC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8262D100: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8262D108(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8262D108 size=112
    let mut pc: u32 = 0x8262D108;
    'dispatch: loop {
        match pc {
            0x8262D108 => {
    //   block [0x8262D108..0x8262D178)
	// 8262D108: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262D10C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8262D110: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8262D114: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262D118: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8262D11C: 38AA3F7C  addi r5, r10, 0x3f7c
	ctx.r[5].s64 = ctx.r[10].s64 + 16252;
	// 8262D120: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8262D124: 390B9E20  addi r8, r11, -0x61e0
	ctx.r[8].s64 = ctx.r[11].s64 + -25056;
	// 8262D128: 39200019  li r9, 0x19
	ctx.r[9].s64 = 25;
	// 8262D12C: 388A53AC  addi r4, r10, 0x53ac
	ctx.r[4].s64 = ctx.r[10].s64 + 21420;
	// 8262D130: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8262D134: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262D138: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8262D13C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8262D140: 386A535C  addi r3, r10, 0x535c
	ctx.r[3].s64 = ctx.r[10].s64 + 21340;
	// 8262D144: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8262D148: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8262D14C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8262D150: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8262D154: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8262D158: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8262D15C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8262D160: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8262D164: 4BE39CBD  bl 0x82466e20
	ctx.lr = 0x8262D168;
	sub_82466E20(ctx, base);
	// 8262D168: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8262D16C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8262D170: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8262D174: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8262D178(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8262D178 size=100
    let mut pc: u32 = 0x8262D178;
    'dispatch: loop {
        match pc {
            0x8262D178 => {
    //   block [0x8262D178..0x8262D1DC)
	// 8262D178: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262D17C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8262D180: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8262D184: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262D188: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8262D18C: 38AA23EC  addi r5, r10, 0x23ec
	ctx.r[5].s64 = ctx.r[10].s64 + 9196;
	// 8262D190: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8262D194: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8262D198: 388A53C0  addi r4, r10, 0x53c0
	ctx.r[4].s64 = ctx.r[10].s64 + 21440;
	// 8262D19C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262D1A0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8262D1A4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8262D1A8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8262D1AC: 386A538C  addi r3, r10, 0x538c
	ctx.r[3].s64 = ctx.r[10].s64 + 21388;
	// 8262D1B0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8262D1B4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8262D1B8: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8262D1BC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8262D1C0: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8262D1C4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8262D1C8: 4BE39C59  bl 0x82466e20
	ctx.lr = 0x8262D1CC;
	sub_82466E20(ctx, base);
	// 8262D1CC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8262D1D0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8262D1D4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8262D1D8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8262D1E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8262D1E0 size=112
    let mut pc: u32 = 0x8262D1E0;
    'dispatch: loop {
        match pc {
            0x8262D1E0 => {
    //   block [0x8262D1E0..0x8262D250)
	// 8262D1E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262D1E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8262D1E8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8262D1EC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262D1F0: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8262D1F4: 38AA538C  addi r5, r10, 0x538c
	ctx.r[5].s64 = ctx.r[10].s64 + 21388;
	// 8262D1F8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8262D1FC: 390BA078  addi r8, r11, -0x5f88
	ctx.r[8].s64 = ctx.r[11].s64 + -24456;
	// 8262D200: 39200005  li r9, 5
	ctx.r[9].s64 = 5;
	// 8262D204: 388A53D8  addi r4, r10, 0x53d8
	ctx.r[4].s64 = ctx.r[10].s64 + 21464;
	// 8262D208: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8262D20C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262D210: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8262D214: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8262D218: 386A53BC  addi r3, r10, 0x53bc
	ctx.r[3].s64 = ctx.r[10].s64 + 21436;
	// 8262D21C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8262D220: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8262D224: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8262D228: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8262D22C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8262D230: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8262D234: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8262D238: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8262D23C: 4BE39BE5  bl 0x82466e20
	ctx.lr = 0x8262D240;
	sub_82466E20(ctx, base);
	// 8262D240: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8262D244: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8262D248: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8262D24C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8262D250(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8262D250 size=100
    let mut pc: u32 = 0x8262D250;
    'dispatch: loop {
        match pc {
            0x8262D250 => {
    //   block [0x8262D250..0x8262D2B4)
	// 8262D250: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262D254: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8262D258: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8262D25C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262D260: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8262D264: 38AA23EC  addi r5, r10, 0x23ec
	ctx.r[5].s64 = ctx.r[10].s64 + 9196;
	// 8262D268: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8262D26C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8262D270: 388A53F8  addi r4, r10, 0x53f8
	ctx.r[4].s64 = ctx.r[10].s64 + 21496;
	// 8262D274: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262D278: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8262D27C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8262D280: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8262D284: 386A53EC  addi r3, r10, 0x53ec
	ctx.r[3].s64 = ctx.r[10].s64 + 21484;
	// 8262D288: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8262D28C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8262D290: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8262D294: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8262D298: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8262D29C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8262D2A0: 4BE39B81  bl 0x82466e20
	ctx.lr = 0x8262D2A4;
	sub_82466E20(ctx, base);
	// 8262D2A4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8262D2A8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8262D2AC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8262D2B0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8262D2B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8262D2B8 size=108
    let mut pc: u32 = 0x8262D2B8;
    'dispatch: loop {
        match pc {
            0x8262D2B8 => {
    //   block [0x8262D2B8..0x8262D324)
	// 8262D2B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262D2BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8262D2C0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8262D2C4: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8262D2C8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8262D2CC: 38EBA0F0  addi r7, r11, -0x5f10
	ctx.r[7].s64 = ctx.r[11].s64 + -24336;
	// 8262D2D0: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 8262D2D4: 388A5408  addi r4, r10, 0x5408
	ctx.r[4].s64 = ctx.r[10].s64 + 21512;
	// 8262D2D8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8262D2DC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262D2E0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8262D2E4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8262D2E8: 386A541C  addi r3, r10, 0x541c
	ctx.r[3].s64 = ctx.r[10].s64 + 21532;
	// 8262D2EC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8262D2F0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8262D2F4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8262D2F8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8262D2FC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8262D300: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8262D304: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8262D308: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8262D30C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8262D310: 4BE39B11  bl 0x82466e20
	ctx.lr = 0x8262D314;
	sub_82466E20(ctx, base);
	// 8262D314: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8262D318: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8262D31C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8262D320: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8262D328(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8262D328 size=112
    let mut pc: u32 = 0x8262D328;
    'dispatch: loop {
        match pc {
            0x8262D328 => {
    //   block [0x8262D328..0x8262D398)
	// 8262D328: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262D32C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8262D330: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8262D334: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262D338: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8262D33C: 38AA53EC  addi r5, r10, 0x53ec
	ctx.r[5].s64 = ctx.r[10].s64 + 21484;
	// 8262D340: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8262D344: 390BA138  addi r8, r11, -0x5ec8
	ctx.r[8].s64 = ctx.r[11].s64 + -24264;
	// 8262D348: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 8262D34C: 388A5434  addi r4, r10, 0x5434
	ctx.r[4].s64 = ctx.r[10].s64 + 21556;
	// 8262D350: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8262D354: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262D358: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8262D35C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8262D360: 386A544C  addi r3, r10, 0x544c
	ctx.r[3].s64 = ctx.r[10].s64 + 21580;
	// 8262D364: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8262D368: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8262D36C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8262D370: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8262D374: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8262D378: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8262D37C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8262D380: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8262D384: 4BE39A9D  bl 0x82466e20
	ctx.lr = 0x8262D388;
	sub_82466E20(ctx, base);
	// 8262D388: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8262D38C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8262D390: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8262D394: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8262D398(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8262D398 size=100
    let mut pc: u32 = 0x8262D398;
    'dispatch: loop {
        match pc {
            0x8262D398 => {
    //   block [0x8262D398..0x8262D3FC)
	// 8262D398: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262D39C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8262D3A0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8262D3A4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262D3A8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8262D3AC: 38AA23EC  addi r5, r10, 0x23ec
	ctx.r[5].s64 = ctx.r[10].s64 + 9196;
	// 8262D3B0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8262D3B4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8262D3B8: 388A544C  addi r4, r10, 0x544c
	ctx.r[4].s64 = ctx.r[10].s64 + 21580;
	// 8262D3BC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262D3C0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8262D3C4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8262D3C8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8262D3CC: 386A547C  addi r3, r10, 0x547c
	ctx.r[3].s64 = ctx.r[10].s64 + 21628;
	// 8262D3D0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8262D3D4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8262D3D8: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8262D3DC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8262D3E0: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8262D3E4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8262D3E8: 4BE39A39  bl 0x82466e20
	ctx.lr = 0x8262D3EC;
	sub_82466E20(ctx, base);
	// 8262D3EC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8262D3F0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8262D3F4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8262D3F8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8262D400(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8262D400 size=100
    let mut pc: u32 = 0x8262D400;
    'dispatch: loop {
        match pc {
            0x8262D400 => {
    //   block [0x8262D400..0x8262D464)
	// 8262D400: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262D404: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8262D408: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8262D40C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262D410: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8262D414: 38AA23EC  addi r5, r10, 0x23ec
	ctx.r[5].s64 = ctx.r[10].s64 + 9196;
	// 8262D418: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8262D41C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8262D420: 388A5468  addi r4, r10, 0x5468
	ctx.r[4].s64 = ctx.r[10].s64 + 21608;
	// 8262D424: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262D428: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8262D42C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8262D430: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8262D434: 386A54AC  addi r3, r10, 0x54ac
	ctx.r[3].s64 = ctx.r[10].s64 + 21676;
	// 8262D438: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8262D43C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8262D440: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8262D444: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8262D448: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8262D44C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8262D450: 4BE399D1  bl 0x82466e20
	ctx.lr = 0x8262D454;
	sub_82466E20(ctx, base);
	// 8262D454: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8262D458: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8262D45C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8262D460: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8262D468(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8262D468 size=112
    let mut pc: u32 = 0x8262D468;
    'dispatch: loop {
        match pc {
            0x8262D468 => {
    //   block [0x8262D468..0x8262D4D8)
	// 8262D468: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262D46C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8262D470: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8262D474: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262D478: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8262D47C: 38AA547C  addi r5, r10, 0x547c
	ctx.r[5].s64 = ctx.r[10].s64 + 21628;
	// 8262D480: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8262D484: 390BA168  addi r8, r11, -0x5e98
	ctx.r[8].s64 = ctx.r[11].s64 + -24216;
	// 8262D488: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 8262D48C: 388A5480  addi r4, r10, 0x5480
	ctx.r[4].s64 = ctx.r[10].s64 + 21632;
	// 8262D490: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8262D494: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262D498: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8262D49C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8262D4A0: 386A54DC  addi r3, r10, 0x54dc
	ctx.r[3].s64 = ctx.r[10].s64 + 21724;
	// 8262D4A4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8262D4A8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8262D4AC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8262D4B0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8262D4B4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8262D4B8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8262D4BC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8262D4C0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8262D4C4: 4BE3995D  bl 0x82466e20
	ctx.lr = 0x8262D4C8;
	sub_82466E20(ctx, base);
	// 8262D4C8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8262D4CC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8262D4D0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8262D4D4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8262D4D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8262D4D8 size=112
    let mut pc: u32 = 0x8262D4D8;
    'dispatch: loop {
        match pc {
            0x8262D4D8 => {
    //   block [0x8262D4D8..0x8262D548)
	// 8262D4D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262D4DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8262D4E0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8262D4E4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262D4E8: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8262D4EC: 38AA54AC  addi r5, r10, 0x54ac
	ctx.r[5].s64 = ctx.r[10].s64 + 21676;
	// 8262D4F0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8262D4F4: 390BA1C8  addi r8, r11, -0x5e38
	ctx.r[8].s64 = ctx.r[11].s64 + -24120;
	// 8262D4F8: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 8262D4FC: 388A54A4  addi r4, r10, 0x54a4
	ctx.r[4].s64 = ctx.r[10].s64 + 21668;
	// 8262D500: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8262D504: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262D508: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8262D50C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8262D510: 386A550C  addi r3, r10, 0x550c
	ctx.r[3].s64 = ctx.r[10].s64 + 21772;
	// 8262D514: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8262D518: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8262D51C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8262D520: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8262D524: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8262D528: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8262D52C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8262D530: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8262D534: 4BE398ED  bl 0x82466e20
	ctx.lr = 0x8262D538;
	sub_82466E20(ctx, base);
	// 8262D538: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8262D53C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8262D540: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8262D544: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8262D548(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8262D548 size=100
    let mut pc: u32 = 0x8262D548;
    'dispatch: loop {
        match pc {
            0x8262D548 => {
    //   block [0x8262D548..0x8262D5AC)
	// 8262D548: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262D54C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8262D550: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8262D554: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262D558: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8262D55C: 38AA23EC  addi r5, r10, 0x23ec
	ctx.r[5].s64 = ctx.r[10].s64 + 9196;
	// 8262D560: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8262D564: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8262D568: 388A54C8  addi r4, r10, 0x54c8
	ctx.r[4].s64 = ctx.r[10].s64 + 21704;
	// 8262D56C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262D570: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8262D574: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8262D578: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8262D57C: 386A553C  addi r3, r10, 0x553c
	ctx.r[3].s64 = ctx.r[10].s64 + 21820;
	// 8262D580: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8262D584: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8262D588: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8262D58C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8262D590: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8262D594: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8262D598: 4BE39889  bl 0x82466e20
	ctx.lr = 0x8262D59C;
	sub_82466E20(ctx, base);
	// 8262D59C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8262D5A0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8262D5A4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8262D5A8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8262D5B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8262D5B0 size=112
    let mut pc: u32 = 0x8262D5B0;
    'dispatch: loop {
        match pc {
            0x8262D5B0 => {
    //   block [0x8262D5B0..0x8262D620)
	// 8262D5B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262D5B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8262D5B8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8262D5BC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262D5C0: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8262D5C4: 38AA553C  addi r5, r10, 0x553c
	ctx.r[5].s64 = ctx.r[10].s64 + 21820;
	// 8262D5C8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8262D5CC: 390BA228  addi r8, r11, -0x5dd8
	ctx.r[8].s64 = ctx.r[11].s64 + -24024;
	// 8262D5D0: 3920000A  li r9, 0xa
	ctx.r[9].s64 = 10;
	// 8262D5D4: 388A54D8  addi r4, r10, 0x54d8
	ctx.r[4].s64 = ctx.r[10].s64 + 21720;
	// 8262D5D8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8262D5DC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262D5E0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8262D5E4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8262D5E8: 386A556C  addi r3, r10, 0x556c
	ctx.r[3].s64 = ctx.r[10].s64 + 21868;
	// 8262D5EC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8262D5F0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8262D5F4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8262D5F8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8262D5FC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8262D600: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8262D604: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8262D608: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8262D60C: 4BE39815  bl 0x82466e20
	ctx.lr = 0x8262D610;
	sub_82466E20(ctx, base);
	// 8262D610: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8262D614: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8262D618: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8262D61C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8262D620(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8262D620 size=108
    let mut pc: u32 = 0x8262D620;
    'dispatch: loop {
        match pc {
            0x8262D620 => {
    //   block [0x8262D620..0x8262D68C)
	// 8262D620: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262D624: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8262D628: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8262D62C: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8262D630: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8262D634: 38EBA318  addi r7, r11, -0x5ce8
	ctx.r[7].s64 = ctx.r[11].s64 + -23784;
	// 8262D638: 3900000A  li r8, 0xa
	ctx.r[8].s64 = 10;
	// 8262D63C: 388A54F0  addi r4, r10, 0x54f0
	ctx.r[4].s64 = ctx.r[10].s64 + 21744;
	// 8262D640: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8262D644: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262D648: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8262D64C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8262D650: 386A559C  addi r3, r10, 0x559c
	ctx.r[3].s64 = ctx.r[10].s64 + 21916;
	// 8262D654: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8262D658: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8262D65C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8262D660: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8262D664: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8262D668: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8262D66C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8262D670: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8262D674: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8262D678: 4BE397A9  bl 0x82466e20
	ctx.lr = 0x8262D67C;
	sub_82466E20(ctx, base);
	// 8262D67C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8262D680: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8262D684: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8262D688: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8262D690(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8262D690 size=108
    let mut pc: u32 = 0x8262D690;
    'dispatch: loop {
        match pc {
            0x8262D690 => {
    //   block [0x8262D690..0x8262D6FC)
	// 8262D690: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262D694: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8262D698: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8262D69C: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8262D6A0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8262D6A4: 38EBA408  addi r7, r11, -0x5bf8
	ctx.r[7].s64 = ctx.r[11].s64 + -23544;
	// 8262D6A8: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 8262D6AC: 388A551C  addi r4, r10, 0x551c
	ctx.r[4].s64 = ctx.r[10].s64 + 21788;
	// 8262D6B0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8262D6B4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262D6B8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8262D6BC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8262D6C0: 386A55CC  addi r3, r10, 0x55cc
	ctx.r[3].s64 = ctx.r[10].s64 + 21964;
	// 8262D6C4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8262D6C8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8262D6CC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8262D6D0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8262D6D4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8262D6D8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8262D6DC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8262D6E0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8262D6E4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8262D6E8: 4BE39739  bl 0x82466e20
	ctx.lr = 0x8262D6EC;
	sub_82466E20(ctx, base);
	// 8262D6EC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8262D6F0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8262D6F4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8262D6F8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8262D700(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8262D700 size=108
    let mut pc: u32 = 0x8262D700;
    'dispatch: loop {
        match pc {
            0x8262D700 => {
    //   block [0x8262D700..0x8262D76C)
	// 8262D700: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262D704: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8262D708: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8262D70C: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8262D710: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8262D714: 38EBA450  addi r7, r11, -0x5bb0
	ctx.r[7].s64 = ctx.r[11].s64 + -23472;
	// 8262D718: 39000009  li r8, 9
	ctx.r[8].s64 = 9;
	// 8262D71C: 388A553C  addi r4, r10, 0x553c
	ctx.r[4].s64 = ctx.r[10].s64 + 21820;
	// 8262D720: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8262D724: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262D728: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8262D72C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8262D730: 386A55FC  addi r3, r10, 0x55fc
	ctx.r[3].s64 = ctx.r[10].s64 + 22012;
	// 8262D734: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8262D738: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8262D73C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8262D740: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8262D744: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8262D748: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8262D74C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8262D750: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8262D754: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8262D758: 4BE396C9  bl 0x82466e20
	ctx.lr = 0x8262D75C;
	sub_82466E20(ctx, base);
	// 8262D75C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8262D760: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8262D764: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8262D768: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8262D770(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8262D770 size=108
    let mut pc: u32 = 0x8262D770;
    'dispatch: loop {
        match pc {
            0x8262D770 => {
    //   block [0x8262D770..0x8262D7DC)
	// 8262D770: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262D774: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8262D778: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8262D77C: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8262D780: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8262D784: 38EBA528  addi r7, r11, -0x5ad8
	ctx.r[7].s64 = ctx.r[11].s64 + -23256;
	// 8262D788: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 8262D78C: 388A5560  addi r4, r10, 0x5560
	ctx.r[4].s64 = ctx.r[10].s64 + 21856;
	// 8262D790: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8262D794: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262D798: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8262D79C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8262D7A0: 386A562C  addi r3, r10, 0x562c
	ctx.r[3].s64 = ctx.r[10].s64 + 22060;
	// 8262D7A4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8262D7A8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8262D7AC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8262D7B0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8262D7B4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8262D7B8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8262D7BC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8262D7C0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8262D7C4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8262D7C8: 4BE39659  bl 0x82466e20
	ctx.lr = 0x8262D7CC;
	sub_82466E20(ctx, base);
	// 8262D7CC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8262D7D0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8262D7D4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8262D7D8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8262D7E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8262D7E0 size=100
    let mut pc: u32 = 0x8262D7E0;
    'dispatch: loop {
        match pc {
            0x8262D7E0 => {
    //   block [0x8262D7E0..0x8262D844)
	// 8262D7E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262D7E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8262D7E8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8262D7EC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262D7F0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8262D7F4: 38AA23EC  addi r5, r10, 0x23ec
	ctx.r[5].s64 = ctx.r[10].s64 + 9196;
	// 8262D7F8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8262D7FC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8262D800: 388A5578  addi r4, r10, 0x5578
	ctx.r[4].s64 = ctx.r[10].s64 + 21880;
	// 8262D804: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262D808: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8262D80C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8262D810: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8262D814: 386A565C  addi r3, r10, 0x565c
	ctx.r[3].s64 = ctx.r[10].s64 + 22108;
	// 8262D818: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8262D81C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8262D820: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8262D824: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8262D828: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8262D82C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8262D830: 4BE395F1  bl 0x82466e20
	ctx.lr = 0x8262D834;
	sub_82466E20(ctx, base);
	// 8262D834: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8262D838: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8262D83C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8262D840: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8262D848(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8262D848 size=112
    let mut pc: u32 = 0x8262D848;
    'dispatch: loop {
        match pc {
            0x8262D848 => {
    //   block [0x8262D848..0x8262D8B8)
	// 8262D848: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262D84C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8262D850: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8262D854: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262D858: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8262D85C: 38AA565C  addi r5, r10, 0x565c
	ctx.r[5].s64 = ctx.r[10].s64 + 22108;
	// 8262D860: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8262D864: 390BA540  addi r8, r11, -0x5ac0
	ctx.r[8].s64 = ctx.r[11].s64 + -23232;
	// 8262D868: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 8262D86C: 388A558C  addi r4, r10, 0x558c
	ctx.r[4].s64 = ctx.r[10].s64 + 21900;
	// 8262D870: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8262D874: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262D878: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8262D87C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8262D880: 386A568C  addi r3, r10, 0x568c
	ctx.r[3].s64 = ctx.r[10].s64 + 22156;
	// 8262D884: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8262D888: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8262D88C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8262D890: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8262D894: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8262D898: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8262D89C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8262D8A0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8262D8A4: 4BE3957D  bl 0x82466e20
	ctx.lr = 0x8262D8A8;
	sub_82466E20(ctx, base);
	// 8262D8A8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8262D8AC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8262D8B0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8262D8B4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8262D8B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8262D8B8 size=108
    let mut pc: u32 = 0x8262D8B8;
    'dispatch: loop {
        match pc {
            0x8262D8B8 => {
    //   block [0x8262D8B8..0x8262D924)
	// 8262D8B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262D8BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8262D8C0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8262D8C4: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8262D8C8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8262D8CC: 38EBA588  addi r7, r11, -0x5a78
	ctx.r[7].s64 = ctx.r[11].s64 + -23160;
	// 8262D8D0: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 8262D8D4: 388A55A8  addi r4, r10, 0x55a8
	ctx.r[4].s64 = ctx.r[10].s64 + 21928;
	// 8262D8D8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8262D8DC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262D8E0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8262D8E4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8262D8E8: 386A56BC  addi r3, r10, 0x56bc
	ctx.r[3].s64 = ctx.r[10].s64 + 22204;
	// 8262D8EC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8262D8F0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8262D8F4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8262D8F8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8262D8FC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8262D900: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8262D904: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8262D908: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8262D90C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8262D910: 4BE39511  bl 0x82466e20
	ctx.lr = 0x8262D914;
	sub_82466E20(ctx, base);
	// 8262D914: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8262D918: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8262D91C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8262D920: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8262D928(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8262D928 size=112
    let mut pc: u32 = 0x8262D928;
    'dispatch: loop {
        match pc {
            0x8262D928 => {
    //   block [0x8262D928..0x8262D998)
	// 8262D928: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262D92C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8262D930: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8262D934: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262D938: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8262D93C: 38AA23EC  addi r5, r10, 0x23ec
	ctx.r[5].s64 = ctx.r[10].s64 + 9196;
	// 8262D940: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8262D944: 390BA5D0  addi r8, r11, -0x5a30
	ctx.r[8].s64 = ctx.r[11].s64 + -23088;
	// 8262D948: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8262D94C: 388A55D8  addi r4, r10, 0x55d8
	ctx.r[4].s64 = ctx.r[10].s64 + 21976;
	// 8262D950: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8262D954: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262D958: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8262D95C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8262D960: 386A56EC  addi r3, r10, 0x56ec
	ctx.r[3].s64 = ctx.r[10].s64 + 22252;
	// 8262D964: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8262D968: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8262D96C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8262D970: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8262D974: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8262D978: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8262D97C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8262D980: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8262D984: 4BE3949D  bl 0x82466e20
	ctx.lr = 0x8262D988;
	sub_82466E20(ctx, base);
	// 8262D988: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8262D98C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8262D990: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8262D994: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8262D998(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8262D998 size=108
    let mut pc: u32 = 0x8262D998;
    'dispatch: loop {
        match pc {
            0x8262D998 => {
    //   block [0x8262D998..0x8262DA04)
	// 8262D998: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262D99C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8262D9A0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8262D9A4: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8262D9A8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8262D9AC: 38EBA5E8  addi r7, r11, -0x5a18
	ctx.r[7].s64 = ctx.r[11].s64 + -23064;
	// 8262D9B0: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 8262D9B4: 388A55EC  addi r4, r10, 0x55ec
	ctx.r[4].s64 = ctx.r[10].s64 + 21996;
	// 8262D9B8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8262D9BC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262D9C0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8262D9C4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8262D9C8: 386A571C  addi r3, r10, 0x571c
	ctx.r[3].s64 = ctx.r[10].s64 + 22300;
	// 8262D9CC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8262D9D0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8262D9D4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8262D9D8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8262D9DC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8262D9E0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8262D9E4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8262D9E8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8262D9EC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8262D9F0: 4BE39431  bl 0x82466e20
	ctx.lr = 0x8262D9F4;
	sub_82466E20(ctx, base);
	// 8262D9F4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8262D9F8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8262D9FC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8262DA00: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8262DA08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8262DA08 size=112
    let mut pc: u32 = 0x8262DA08;
    'dispatch: loop {
        match pc {
            0x8262DA08 => {
    //   block [0x8262DA08..0x8262DA78)
	// 8262DA08: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262DA0C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8262DA10: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8262DA14: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262DA18: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8262DA1C: 38AA56EC  addi r5, r10, 0x56ec
	ctx.r[5].s64 = ctx.r[10].s64 + 22252;
	// 8262DA20: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8262DA24: 390BA630  addi r8, r11, -0x59d0
	ctx.r[8].s64 = ctx.r[11].s64 + -22992;
	// 8262DA28: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8262DA2C: 388A5628  addi r4, r10, 0x5628
	ctx.r[4].s64 = ctx.r[10].s64 + 22056;
	// 8262DA30: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8262DA34: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262DA38: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8262DA3C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8262DA40: 386A574C  addi r3, r10, 0x574c
	ctx.r[3].s64 = ctx.r[10].s64 + 22348;
	// 8262DA44: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8262DA48: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8262DA4C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8262DA50: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8262DA54: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8262DA58: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8262DA5C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8262DA60: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8262DA64: 4BE393BD  bl 0x82466e20
	ctx.lr = 0x8262DA68;
	sub_82466E20(ctx, base);
	// 8262DA68: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8262DA6C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8262DA70: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8262DA74: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8262DA78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8262DA78 size=100
    let mut pc: u32 = 0x8262DA78;
    'dispatch: loop {
        match pc {
            0x8262DA78 => {
    //   block [0x8262DA78..0x8262DADC)
	// 8262DA78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262DA7C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8262DA80: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8262DA84: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262DA88: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8262DA8C: 38AA23EC  addi r5, r10, 0x23ec
	ctx.r[5].s64 = ctx.r[10].s64 + 9196;
	// 8262DA90: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8262DA94: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8262DA98: 388A5644  addi r4, r10, 0x5644
	ctx.r[4].s64 = ctx.r[10].s64 + 22084;
	// 8262DA9C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262DAA0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8262DAA4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8262DAA8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8262DAAC: 386A577C  addi r3, r10, 0x577c
	ctx.r[3].s64 = ctx.r[10].s64 + 22396;
	// 8262DAB0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8262DAB4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8262DAB8: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8262DABC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8262DAC0: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8262DAC4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8262DAC8: 4BE39359  bl 0x82466e20
	ctx.lr = 0x8262DACC;
	sub_82466E20(ctx, base);
	// 8262DACC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8262DAD0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8262DAD4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8262DAD8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8262DAE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8262DAE0 size=112
    let mut pc: u32 = 0x8262DAE0;
    'dispatch: loop {
        match pc {
            0x8262DAE0 => {
    //   block [0x8262DAE0..0x8262DB50)
	// 8262DAE0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262DAE4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8262DAE8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8262DAEC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262DAF0: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8262DAF4: 38AA577C  addi r5, r10, 0x577c
	ctx.r[5].s64 = ctx.r[10].s64 + 22396;
	// 8262DAF8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8262DAFC: 390BA648  addi r8, r11, -0x59b8
	ctx.r[8].s64 = ctx.r[11].s64 + -22968;
	// 8262DB00: 39200007  li r9, 7
	ctx.r[9].s64 = 7;
	// 8262DB04: 388A565C  addi r4, r10, 0x565c
	ctx.r[4].s64 = ctx.r[10].s64 + 22108;
	// 8262DB08: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8262DB0C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262DB10: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8262DB14: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8262DB18: 386A57AC  addi r3, r10, 0x57ac
	ctx.r[3].s64 = ctx.r[10].s64 + 22444;
	// 8262DB1C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8262DB20: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8262DB24: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8262DB28: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8262DB2C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8262DB30: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8262DB34: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8262DB38: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8262DB3C: 4BE392E5  bl 0x82466e20
	ctx.lr = 0x8262DB40;
	sub_82466E20(ctx, base);
	// 8262DB40: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8262DB44: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8262DB48: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8262DB4C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8262DB50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8262DB50 size=108
    let mut pc: u32 = 0x8262DB50;
    'dispatch: loop {
        match pc {
            0x8262DB50 => {
    //   block [0x8262DB50..0x8262DBBC)
	// 8262DB50: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262DB54: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8262DB58: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8262DB5C: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8262DB60: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8262DB64: 38EBA6F0  addi r7, r11, -0x5910
	ctx.r[7].s64 = ctx.r[11].s64 + -22800;
	// 8262DB68: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 8262DB6C: 388A567C  addi r4, r10, 0x567c
	ctx.r[4].s64 = ctx.r[10].s64 + 22140;
	// 8262DB70: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8262DB74: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262DB78: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8262DB7C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8262DB80: 386A57DC  addi r3, r10, 0x57dc
	ctx.r[3].s64 = ctx.r[10].s64 + 22492;
	// 8262DB84: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8262DB88: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8262DB8C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8262DB90: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8262DB94: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8262DB98: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8262DB9C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8262DBA0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8262DBA4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8262DBA8: 4BE39279  bl 0x82466e20
	ctx.lr = 0x8262DBAC;
	sub_82466E20(ctx, base);
	// 8262DBAC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8262DBB0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8262DBB4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8262DBB8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8262DBC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8262DBC0 size=112
    let mut pc: u32 = 0x8262DBC0;
    'dispatch: loop {
        match pc {
            0x8262DBC0 => {
    //   block [0x8262DBC0..0x8262DC30)
	// 8262DBC0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262DBC4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8262DBC8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8262DBCC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262DBD0: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8262DBD4: 38AA23EC  addi r5, r10, 0x23ec
	ctx.r[5].s64 = ctx.r[10].s64 + 9196;
	// 8262DBD8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8262DBDC: 390BA720  addi r8, r11, -0x58e0
	ctx.r[8].s64 = ctx.r[11].s64 + -22752;
	// 8262DBE0: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 8262DBE4: 388A568C  addi r4, r10, 0x568c
	ctx.r[4].s64 = ctx.r[10].s64 + 22156;
	// 8262DBE8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8262DBEC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262DBF0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8262DBF4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8262DBF8: 386A580C  addi r3, r10, 0x580c
	ctx.r[3].s64 = ctx.r[10].s64 + 22540;
	// 8262DBFC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8262DC00: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8262DC04: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8262DC08: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8262DC0C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8262DC10: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8262DC14: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8262DC18: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8262DC1C: 4BE39205  bl 0x82466e20
	ctx.lr = 0x8262DC20;
	sub_82466E20(ctx, base);
	// 8262DC20: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8262DC24: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8262DC28: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8262DC2C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8262DC30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8262DC30 size=112
    let mut pc: u32 = 0x8262DC30;
    'dispatch: loop {
        match pc {
            0x8262DC30 => {
    //   block [0x8262DC30..0x8262DCA0)
	// 8262DC30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262DC34: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8262DC38: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8262DC3C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262DC40: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8262DC44: 38AA23EC  addi r5, r10, 0x23ec
	ctx.r[5].s64 = ctx.r[10].s64 + 9196;
	// 8262DC48: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8262DC4C: 390BA768  addi r8, r11, -0x5898
	ctx.r[8].s64 = ctx.r[11].s64 + -22680;
	// 8262DC50: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 8262DC54: 388A56A0  addi r4, r10, 0x56a0
	ctx.r[4].s64 = ctx.r[10].s64 + 22176;
	// 8262DC58: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8262DC5C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262DC60: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8262DC64: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8262DC68: 386A583C  addi r3, r10, 0x583c
	ctx.r[3].s64 = ctx.r[10].s64 + 22588;
	// 8262DC6C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8262DC70: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8262DC74: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8262DC78: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8262DC7C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8262DC80: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8262DC84: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8262DC88: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8262DC8C: 4BE39195  bl 0x82466e20
	ctx.lr = 0x8262DC90;
	sub_82466E20(ctx, base);
	// 8262DC90: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8262DC94: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8262DC98: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8262DC9C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8262DCA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8262DCA0 size=100
    let mut pc: u32 = 0x8262DCA0;
    'dispatch: loop {
        match pc {
            0x8262DCA0 => {
    //   block [0x8262DCA0..0x8262DD04)
	// 8262DCA0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262DCA4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8262DCA8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8262DCAC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262DCB0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8262DCB4: 38AA23EC  addi r5, r10, 0x23ec
	ctx.r[5].s64 = ctx.r[10].s64 + 9196;
	// 8262DCB8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8262DCBC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8262DCC0: 388A56B0  addi r4, r10, 0x56b0
	ctx.r[4].s64 = ctx.r[10].s64 + 22192;
	// 8262DCC4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262DCC8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8262DCCC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8262DCD0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8262DCD4: 386A586C  addi r3, r10, 0x586c
	ctx.r[3].s64 = ctx.r[10].s64 + 22636;
	// 8262DCD8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8262DCDC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8262DCE0: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8262DCE4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8262DCE8: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8262DCEC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8262DCF0: 4BE39131  bl 0x82466e20
	ctx.lr = 0x8262DCF4;
	sub_82466E20(ctx, base);
	// 8262DCF4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8262DCF8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8262DCFC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8262DD00: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8262DD08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8262DD08 size=112
    let mut pc: u32 = 0x8262DD08;
    'dispatch: loop {
        match pc {
            0x8262DD08 => {
    //   block [0x8262DD08..0x8262DD78)
	// 8262DD08: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262DD0C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8262DD10: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8262DD14: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262DD18: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8262DD1C: 38AA586C  addi r5, r10, 0x586c
	ctx.r[5].s64 = ctx.r[10].s64 + 22636;
	// 8262DD20: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8262DD24: 390BA7B0  addi r8, r11, -0x5850
	ctx.r[8].s64 = ctx.r[11].s64 + -22608;
	// 8262DD28: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 8262DD2C: 388A56C8  addi r4, r10, 0x56c8
	ctx.r[4].s64 = ctx.r[10].s64 + 22216;
	// 8262DD30: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8262DD34: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262DD38: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8262DD3C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8262DD40: 386A589C  addi r3, r10, 0x589c
	ctx.r[3].s64 = ctx.r[10].s64 + 22684;
	// 8262DD44: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8262DD48: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8262DD4C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8262DD50: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8262DD54: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8262DD58: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8262DD5C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8262DD60: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8262DD64: 4BE390BD  bl 0x82466e20
	ctx.lr = 0x8262DD68;
	sub_82466E20(ctx, base);
	// 8262DD68: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8262DD6C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8262DD70: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8262DD74: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8262DD78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8262DD78 size=112
    let mut pc: u32 = 0x8262DD78;
    'dispatch: loop {
        match pc {
            0x8262DD78 => {
    //   block [0x8262DD78..0x8262DDE8)
	// 8262DD78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262DD7C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8262DD80: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8262DD84: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262DD88: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8262DD8C: 38AA23EC  addi r5, r10, 0x23ec
	ctx.r[5].s64 = ctx.r[10].s64 + 9196;
	// 8262DD90: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8262DD94: 390BA7F8  addi r8, r11, -0x5808
	ctx.r[8].s64 = ctx.r[11].s64 + -22536;
	// 8262DD98: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8262DD9C: 388A56E8  addi r4, r10, 0x56e8
	ctx.r[4].s64 = ctx.r[10].s64 + 22248;
	// 8262DDA0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8262DDA4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262DDA8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8262DDAC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8262DDB0: 386A58CC  addi r3, r10, 0x58cc
	ctx.r[3].s64 = ctx.r[10].s64 + 22732;
	// 8262DDB4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8262DDB8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8262DDBC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8262DDC0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8262DDC4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8262DDC8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8262DDCC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8262DDD0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8262DDD4: 4BE3904D  bl 0x82466e20
	ctx.lr = 0x8262DDD8;
	sub_82466E20(ctx, base);
	// 8262DDD8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8262DDDC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8262DDE0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8262DDE4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8262DDE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8262DDE8 size=112
    let mut pc: u32 = 0x8262DDE8;
    'dispatch: loop {
        match pc {
            0x8262DDE8 => {
    //   block [0x8262DDE8..0x8262DE58)
	// 8262DDE8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262DDEC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8262DDF0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8262DDF4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262DDF8: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8262DDFC: 38AA23EC  addi r5, r10, 0x23ec
	ctx.r[5].s64 = ctx.r[10].s64 + 9196;
	// 8262DE00: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8262DE04: 390BA810  addi r8, r11, -0x57f0
	ctx.r[8].s64 = ctx.r[11].s64 + -22512;
	// 8262DE08: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8262DE0C: 388A5700  addi r4, r10, 0x5700
	ctx.r[4].s64 = ctx.r[10].s64 + 22272;
	// 8262DE10: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8262DE14: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262DE18: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8262DE1C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8262DE20: 386A58FC  addi r3, r10, 0x58fc
	ctx.r[3].s64 = ctx.r[10].s64 + 22780;
	// 8262DE24: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8262DE28: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8262DE2C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8262DE30: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8262DE34: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 8262DE38: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8262DE3C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8262DE40: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8262DE44: 4BE38FDD  bl 0x82466e20
	ctx.lr = 0x8262DE48;
	sub_82466E20(ctx, base);
	// 8262DE48: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8262DE4C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8262DE50: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8262DE54: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8262DE58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8262DE58 size=112
    let mut pc: u32 = 0x8262DE58;
    'dispatch: loop {
        match pc {
            0x8262DE58 => {
    //   block [0x8262DE58..0x8262DEC8)
	// 8262DE58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262DE5C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8262DE60: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8262DE64: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262DE68: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8262DE6C: 38AA58CC  addi r5, r10, 0x58cc
	ctx.r[5].s64 = ctx.r[10].s64 + 22732;
	// 8262DE70: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8262DE74: 390BA828  addi r8, r11, -0x57d8
	ctx.r[8].s64 = ctx.r[11].s64 + -22488;
	// 8262DE78: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 8262DE7C: 388A571C  addi r4, r10, 0x571c
	ctx.r[4].s64 = ctx.r[10].s64 + 22300;
	// 8262DE80: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8262DE84: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262DE88: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8262DE8C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8262DE90: 386A592C  addi r3, r10, 0x592c
	ctx.r[3].s64 = ctx.r[10].s64 + 22828;
	// 8262DE94: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8262DE98: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8262DE9C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8262DEA0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8262DEA4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8262DEA8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8262DEAC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8262DEB0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8262DEB4: 4BE38F6D  bl 0x82466e20
	ctx.lr = 0x8262DEB8;
	sub_82466E20(ctx, base);
	// 8262DEB8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8262DEBC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8262DEC0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8262DEC4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8262DEC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8262DEC8 size=72
    let mut pc: u32 = 0x8262DEC8;
    'dispatch: loop {
        match pc {
            0x8262DEC8 => {
    //   block [0x8262DEC8..0x8262DF10)
	// 8262DEC8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262DECC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8262DED0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8262DED4: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 8262DED8: 38A00004  li r5, 4
	ctx.r[5].s64 = 4;
	// 8262DEDC: 38CB2A70  addi r6, r11, 0x2a70
	ctx.r[6].s64 = ctx.r[11].s64 + 10864;
	// 8262DEE0: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 8262DEE4: 388B3E20  addi r4, r11, 0x3e20
	ctx.r[4].s64 = ctx.r[11].s64 + 15904;
	// 8262DEE8: 3D608294  lis r11, -0x7d6c
	ctx.r[11].s64 = -2104229888;
	// 8262DEEC: 386B595C  addi r3, r11, 0x595c
	ctx.r[3].s64 = ctx.r[11].s64 + 22876;
	// 8262DEF0: 4BE4DB99  bl 0x8247ba88
	ctx.lr = 0x8262DEF4;
	sub_8247BA88(ctx, base);
	// 8262DEF4: 3D608271  lis r11, -0x7d8f
	ctx.r[11].s64 = -2106523648;
	// 8262DEF8: 386BCD68  addi r3, r11, -0x3298
	ctx.r[3].s64 = ctx.r[11].s64 + -12952;
	// 8262DEFC: 4BF04C3D  bl 0x82532b38
	ctx.lr = 0x8262DF00;
	sub_82532B38(ctx, base);
	// 8262DF00: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8262DF04: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8262DF08: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8262DF0C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8262DF10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8262DF10 size=108
    let mut pc: u32 = 0x8262DF10;
    'dispatch: loop {
        match pc {
            0x8262DF10 => {
    //   block [0x8262DF10..0x8262DF7C)
	// 8262DF10: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262DF14: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8262DF18: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8262DF1C: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8262DF20: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8262DF24: 38EBBA10  addi r7, r11, -0x45f0
	ctx.r[7].s64 = ctx.r[11].s64 + -17904;
	// 8262DF28: 39000005  li r8, 5
	ctx.r[8].s64 = 5;
	// 8262DF2C: 388A2B24  addi r4, r10, 0x2b24
	ctx.r[4].s64 = ctx.r[10].s64 + 11044;
	// 8262DF30: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8262DF34: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262DF38: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8262DF3C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8262DF40: 386A5974  addi r3, r10, 0x5974
	ctx.r[3].s64 = ctx.r[10].s64 + 22900;
	// 8262DF44: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8262DF48: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8262DF4C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8262DF50: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8262DF54: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8262DF58: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8262DF5C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8262DF60: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8262DF64: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8262DF68: 4BE38EB9  bl 0x82466e20
	ctx.lr = 0x8262DF6C;
	sub_82466E20(ctx, base);
	// 8262DF6C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8262DF70: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8262DF74: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8262DF78: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8262DF80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8262DF80 size=24
    let mut pc: u32 = 0x8262DF80;
    'dispatch: loop {
        match pc {
            0x8262DF80 => {
    //   block [0x8262DF80..0x8262DF98)
	// 8262DF80: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8262DF84: 3D408277  lis r10, -0x7d89
	ctx.r[10].s64 = -2106130432;
	// 8262DF88: 394A1930  addi r10, r10, 0x1930
	ctx.r[10].s64 = ctx.r[10].s64 + 6448;
	// 8262DF8C: 816BBA88  lwz r11, -0x4578(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-17784 as u32) ) } as u64;
	// 8262DF90: 916A0038  stw r11, 0x38(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(56 as u32), ctx.r[11].u32 ) };
	// 8262DF94: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8262DF98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8262DF98 size=112
    let mut pc: u32 = 0x8262DF98;
    'dispatch: loop {
        match pc {
            0x8262DF98 => {
    //   block [0x8262DF98..0x8262E008)
	// 8262DF98: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262DF9C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8262DFA0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8262DFA4: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 8262DFA8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8262DFAC: 392B46EC  addi r9, r11, 0x46ec
	ctx.r[9].s64 = ctx.r[11].s64 + 18156;
	// 8262DFB0: 38C00003  li r6, 3
	ctx.r[6].s64 = 3;
	// 8262DFB4: 38E90014  addi r7, r9, 0x14
	ctx.r[7].s64 = ctx.r[9].s64 + 20;
	// 8262DFB8: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8262DFBC: 388A2B3C  addi r4, r10, 0x2b3c
	ctx.r[4].s64 = ctx.r[10].s64 + 11068;
	// 8262DFC0: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8262DFC4: 396B1930  addi r11, r11, 0x1930
	ctx.r[11].s64 = ctx.r[11].s64 + 6448;
	// 8262DFC8: 90C1005C  stw r6, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[6].u32 ) };
	// 8262DFCC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262DFD0: 90E10064  stw r7, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[7].u32 ) };
	// 8262DFD4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8262DFD8: 386A59A4  addi r3, r10, 0x59a4
	ctx.r[3].s64 = ctx.r[10].s64 + 22948;
	// 8262DFDC: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8262DFE0: 91010074  stw r8, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[8].u32 ) };
	// 8262DFE4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8262DFE8: 9101006C  stw r8, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[8].u32 ) };
	// 8262DFEC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8262DFF0: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8262DFF4: 4BE38E2D  bl 0x82466e20
	ctx.lr = 0x8262DFF8;
	sub_82466E20(ctx, base);
	// 8262DFF8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8262DFFC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8262E000: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8262E004: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8262E008(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8262E008 size=108
    let mut pc: u32 = 0x8262E008;
    'dispatch: loop {
        match pc {
            0x8262E008 => {
    //   block [0x8262E008..0x8262E074)
	// 8262E008: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262E00C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8262E010: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8262E014: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8262E018: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8262E01C: 38EBBA8C  addi r7, r11, -0x4574
	ctx.r[7].s64 = ctx.r[11].s64 + -17780;
	// 8262E020: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 8262E024: 388A2B50  addi r4, r10, 0x2b50
	ctx.r[4].s64 = ctx.r[10].s64 + 11088;
	// 8262E028: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8262E02C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262E030: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8262E034: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8262E038: 386A59D4  addi r3, r10, 0x59d4
	ctx.r[3].s64 = ctx.r[10].s64 + 22996;
	// 8262E03C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8262E040: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8262E044: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8262E048: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8262E04C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8262E050: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8262E054: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8262E058: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8262E05C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8262E060: 4BE38DC1  bl 0x82466e20
	ctx.lr = 0x8262E064;
	sub_82466E20(ctx, base);
	// 8262E064: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8262E068: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8262E06C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8262E070: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8262E078(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8262E078 size=108
    let mut pc: u32 = 0x8262E078;
    'dispatch: loop {
        match pc {
            0x8262E078 => {
    //   block [0x8262E078..0x8262E0E4)
	// 8262E078: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262E07C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8262E080: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8262E084: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8262E088: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8262E08C: 38EBBABC  addi r7, r11, -0x4544
	ctx.r[7].s64 = ctx.r[11].s64 + -17732;
	// 8262E090: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 8262E094: 388A2B6C  addi r4, r10, 0x2b6c
	ctx.r[4].s64 = ctx.r[10].s64 + 11116;
	// 8262E098: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8262E09C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262E0A0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8262E0A4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8262E0A8: 386A5A04  addi r3, r10, 0x5a04
	ctx.r[3].s64 = ctx.r[10].s64 + 23044;
	// 8262E0AC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8262E0B0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8262E0B4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8262E0B8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8262E0BC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8262E0C0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8262E0C4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8262E0C8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8262E0CC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8262E0D0: 4BE38D51  bl 0x82466e20
	ctx.lr = 0x8262E0D4;
	sub_82466E20(ctx, base);
	// 8262E0D4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8262E0D8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8262E0DC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8262E0E0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8262E0E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8262E0E8 size=112
    let mut pc: u32 = 0x8262E0E8;
    'dispatch: loop {
        match pc {
            0x8262E0E8 => {
    //   block [0x8262E0E8..0x8262E158)
	// 8262E0E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262E0EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8262E0F0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8262E0F4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262E0F8: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8262E0FC: 38AA64E4  addi r5, r10, 0x64e4
	ctx.r[5].s64 = ctx.r[10].s64 + 25828;
	// 8262E100: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8262E104: 390BBAF0  addi r8, r11, -0x4510
	ctx.r[8].s64 = ctx.r[11].s64 + -17680;
	// 8262E108: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 8262E10C: 388A2BB4  addi r4, r10, 0x2bb4
	ctx.r[4].s64 = ctx.r[10].s64 + 11188;
	// 8262E110: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8262E114: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262E118: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8262E11C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8262E120: 386A5A34  addi r3, r10, 0x5a34
	ctx.r[3].s64 = ctx.r[10].s64 + 23092;
	// 8262E124: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8262E128: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8262E12C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8262E130: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8262E134: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8262E138: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8262E13C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8262E140: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8262E144: 4BE38CDD  bl 0x82466e20
	ctx.lr = 0x8262E148;
	sub_82466E20(ctx, base);
	// 8262E148: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8262E14C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8262E150: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8262E154: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8262E158(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8262E158 size=108
    let mut pc: u32 = 0x8262E158;
    'dispatch: loop {
        match pc {
            0x8262E158 => {
    //   block [0x8262E158..0x8262E1C4)
	// 8262E158: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262E15C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8262E160: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8262E164: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8262E168: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8262E16C: 38EBBB50  addi r7, r11, -0x44b0
	ctx.r[7].s64 = ctx.r[11].s64 + -17584;
	// 8262E170: 39000005  li r8, 5
	ctx.r[8].s64 = 5;
	// 8262E174: 388A2BC8  addi r4, r10, 0x2bc8
	ctx.r[4].s64 = ctx.r[10].s64 + 11208;
	// 8262E178: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8262E17C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262E180: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8262E184: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8262E188: 386A5A64  addi r3, r10, 0x5a64
	ctx.r[3].s64 = ctx.r[10].s64 + 23140;
	// 8262E18C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8262E190: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8262E194: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8262E198: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8262E19C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8262E1A0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8262E1A4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8262E1A8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8262E1AC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8262E1B0: 4BE38C71  bl 0x82466e20
	ctx.lr = 0x8262E1B4;
	sub_82466E20(ctx, base);
	// 8262E1B4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8262E1B8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8262E1BC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8262E1C0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8262E1C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8262E1C8 size=112
    let mut pc: u32 = 0x8262E1C8;
    'dispatch: loop {
        match pc {
            0x8262E1C8 => {
    //   block [0x8262E1C8..0x8262E238)
	// 8262E1C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262E1CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8262E1D0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8262E1D4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262E1D8: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8262E1DC: 38AA5A34  addi r5, r10, 0x5a34
	ctx.r[5].s64 = ctx.r[10].s64 + 23092;
	// 8262E1E0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8262E1E4: 390BBBC8  addi r8, r11, -0x4438
	ctx.r[8].s64 = ctx.r[11].s64 + -17464;
	// 8262E1E8: 39200007  li r9, 7
	ctx.r[9].s64 = 7;
	// 8262E1EC: 388A2C00  addi r4, r10, 0x2c00
	ctx.r[4].s64 = ctx.r[10].s64 + 11264;
	// 8262E1F0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8262E1F4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262E1F8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8262E1FC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8262E200: 386A5A94  addi r3, r10, 0x5a94
	ctx.r[3].s64 = ctx.r[10].s64 + 23188;
	// 8262E204: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8262E208: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8262E20C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8262E210: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8262E214: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8262E218: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8262E21C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8262E220: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8262E224: 4BE38BFD  bl 0x82466e20
	ctx.lr = 0x8262E228;
	sub_82466E20(ctx, base);
	// 8262E228: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8262E22C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8262E230: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8262E234: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8262E238(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8262E238 size=112
    let mut pc: u32 = 0x8262E238;
    'dispatch: loop {
        match pc {
            0x8262E238 => {
    //   block [0x8262E238..0x8262E2A8)
	// 8262E238: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262E23C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8262E240: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8262E244: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262E248: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8262E24C: 38AA5A34  addi r5, r10, 0x5a34
	ctx.r[5].s64 = ctx.r[10].s64 + 23092;
	// 8262E250: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8262E254: 390BBC70  addi r8, r11, -0x4390
	ctx.r[8].s64 = ctx.r[11].s64 + -17296;
	// 8262E258: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8262E25C: 388A2C24  addi r4, r10, 0x2c24
	ctx.r[4].s64 = ctx.r[10].s64 + 11300;
	// 8262E260: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8262E264: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262E268: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8262E26C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8262E270: 386A5AC4  addi r3, r10, 0x5ac4
	ctx.r[3].s64 = ctx.r[10].s64 + 23236;
	// 8262E274: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8262E278: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8262E27C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8262E280: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8262E284: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8262E288: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8262E28C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8262E290: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8262E294: 4BE38B8D  bl 0x82466e20
	ctx.lr = 0x8262E298;
	sub_82466E20(ctx, base);
	// 8262E298: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8262E29C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8262E2A0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8262E2A4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8262E2A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8262E2A8 size=108
    let mut pc: u32 = 0x8262E2A8;
    'dispatch: loop {
        match pc {
            0x8262E2A8 => {
    //   block [0x8262E2A8..0x8262E314)
	// 8262E2A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262E2AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8262E2B0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8262E2B4: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8262E2B8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8262E2BC: 38EBBC88  addi r7, r11, -0x4378
	ctx.r[7].s64 = ctx.r[11].s64 + -17272;
	// 8262E2C0: 39000005  li r8, 5
	ctx.r[8].s64 = 5;
	// 8262E2C4: 388A2C44  addi r4, r10, 0x2c44
	ctx.r[4].s64 = ctx.r[10].s64 + 11332;
	// 8262E2C8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8262E2CC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262E2D0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8262E2D4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8262E2D8: 386A5AF4  addi r3, r10, 0x5af4
	ctx.r[3].s64 = ctx.r[10].s64 + 23284;
	// 8262E2DC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8262E2E0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8262E2E4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8262E2E8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8262E2EC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8262E2F0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8262E2F4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8262E2F8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8262E2FC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8262E300: 4BE38B21  bl 0x82466e20
	ctx.lr = 0x8262E304;
	sub_82466E20(ctx, base);
	// 8262E304: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8262E308: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8262E30C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8262E310: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8262E318(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8262E318 size=112
    let mut pc: u32 = 0x8262E318;
    'dispatch: loop {
        match pc {
            0x8262E318 => {
    //   block [0x8262E318..0x8262E388)
	// 8262E318: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262E31C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8262E320: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8262E324: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262E328: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8262E32C: 38AA5A34  addi r5, r10, 0x5a34
	ctx.r[5].s64 = ctx.r[10].s64 + 23092;
	// 8262E330: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8262E334: 390BBD00  addi r8, r11, -0x4300
	ctx.r[8].s64 = ctx.r[11].s64 + -17152;
	// 8262E338: 39200007  li r9, 7
	ctx.r[9].s64 = 7;
	// 8262E33C: 388A2C74  addi r4, r10, 0x2c74
	ctx.r[4].s64 = ctx.r[10].s64 + 11380;
	// 8262E340: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8262E344: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262E348: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8262E34C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8262E350: 386A5B24  addi r3, r10, 0x5b24
	ctx.r[3].s64 = ctx.r[10].s64 + 23332;
	// 8262E354: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8262E358: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8262E35C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8262E360: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8262E364: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8262E368: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8262E36C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8262E370: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8262E374: 4BE38AAD  bl 0x82466e20
	ctx.lr = 0x8262E378;
	sub_82466E20(ctx, base);
	// 8262E378: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8262E37C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8262E380: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8262E384: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8262E388(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8262E388 size=108
    let mut pc: u32 = 0x8262E388;
    'dispatch: loop {
        match pc {
            0x8262E388 => {
    //   block [0x8262E388..0x8262E3F4)
	// 8262E388: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262E38C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8262E390: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8262E394: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8262E398: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8262E39C: 38EBBDA8  addi r7, r11, -0x4258
	ctx.r[7].s64 = ctx.r[11].s64 + -16984;
	// 8262E3A0: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 8262E3A4: 388A2C90  addi r4, r10, 0x2c90
	ctx.r[4].s64 = ctx.r[10].s64 + 11408;
	// 8262E3A8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8262E3AC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262E3B0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8262E3B4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8262E3B8: 386A5B54  addi r3, r10, 0x5b54
	ctx.r[3].s64 = ctx.r[10].s64 + 23380;
	// 8262E3BC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8262E3C0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8262E3C4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8262E3C8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8262E3CC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8262E3D0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8262E3D4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8262E3D8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8262E3DC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8262E3E0: 4BE38A41  bl 0x82466e20
	ctx.lr = 0x8262E3E4;
	sub_82466E20(ctx, base);
	// 8262E3E4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8262E3E8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8262E3EC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8262E3F0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8262E3F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8262E3F8 size=108
    let mut pc: u32 = 0x8262E3F8;
    'dispatch: loop {
        match pc {
            0x8262E3F8 => {
    //   block [0x8262E3F8..0x8262E464)
	// 8262E3F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262E3FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8262E400: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8262E404: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8262E408: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8262E40C: 38EBBDC0  addi r7, r11, -0x4240
	ctx.r[7].s64 = ctx.r[11].s64 + -16960;
	// 8262E410: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 8262E414: 388A2CA8  addi r4, r10, 0x2ca8
	ctx.r[4].s64 = ctx.r[10].s64 + 11432;
	// 8262E418: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8262E41C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262E420: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8262E424: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8262E428: 386A5B84  addi r3, r10, 0x5b84
	ctx.r[3].s64 = ctx.r[10].s64 + 23428;
	// 8262E42C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8262E430: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8262E434: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8262E438: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8262E43C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8262E440: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8262E444: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8262E448: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8262E44C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8262E450: 4BE389D1  bl 0x82466e20
	ctx.lr = 0x8262E454;
	sub_82466E20(ctx, base);
	// 8262E454: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8262E458: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8262E45C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8262E460: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8262E468(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8262E468 size=116
    let mut pc: u32 = 0x8262E468;
    'dispatch: loop {
        match pc {
            0x8262E468 => {
    //   block [0x8262E468..0x8262E4DC)
	// 8262E468: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262E46C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8262E470: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8262E474: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8262E478: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8262E47C: 390BBE20  addi r8, r11, -0x41e0
	ctx.r[8].s64 = ctx.r[11].s64 + -16864;
	// 8262E480: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8262E484: 392A4728  addi r9, r10, 0x4728
	ctx.r[9].s64 = ctx.r[10].s64 + 18216;
	// 8262E488: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262E48C: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
	// 8262E490: 38AA64E4  addi r5, r10, 0x64e4
	ctx.r[5].s64 = ctx.r[10].s64 + 25828;
	// 8262E494: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8262E498: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8262E49C: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8262E4A0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8262E4A4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8262E4A8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8262E4AC: 3D608294  lis r11, -0x7d6c
	ctx.r[11].s64 = -2104229888;
	// 8262E4B0: 388A2CB8  addi r4, r10, 0x2cb8
	ctx.r[4].s64 = ctx.r[10].s64 + 11448;
	// 8262E4B4: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8262E4B8: 386B5BB4  addi r3, r11, 0x5bb4
	ctx.r[3].s64 = ctx.r[11].s64 + 23476;
	// 8262E4BC: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8262E4C0: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8262E4C4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8262E4C8: 4BE38959  bl 0x82466e20
	ctx.lr = 0x8262E4CC;
	sub_82466E20(ctx, base);
	// 8262E4CC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8262E4D0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8262E4D4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8262E4D8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8262E4E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8262E4E0 size=108
    let mut pc: u32 = 0x8262E4E0;
    'dispatch: loop {
        match pc {
            0x8262E4E0 => {
    //   block [0x8262E4E0..0x8262E54C)
	// 8262E4E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262E4E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8262E4E8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8262E4EC: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8262E4F0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8262E4F4: 38EBBE38  addi r7, r11, -0x41c8
	ctx.r[7].s64 = ctx.r[11].s64 + -16840;
	// 8262E4F8: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 8262E4FC: 388A2CCC  addi r4, r10, 0x2ccc
	ctx.r[4].s64 = ctx.r[10].s64 + 11468;
	// 8262E500: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8262E504: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262E508: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8262E50C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8262E510: 386A5BE4  addi r3, r10, 0x5be4
	ctx.r[3].s64 = ctx.r[10].s64 + 23524;
	// 8262E514: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8262E518: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8262E51C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8262E520: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8262E524: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8262E528: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8262E52C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8262E530: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8262E534: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8262E538: 4BE388E9  bl 0x82466e20
	ctx.lr = 0x8262E53C;
	sub_82466E20(ctx, base);
	// 8262E53C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8262E540: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8262E544: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8262E548: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8262E550(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8262E550 size=108
    let mut pc: u32 = 0x8262E550;
    'dispatch: loop {
        match pc {
            0x8262E550 => {
    //   block [0x8262E550..0x8262E5BC)
	// 8262E550: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262E554: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8262E558: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8262E55C: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8262E560: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8262E564: 38EBBE80  addi r7, r11, -0x4180
	ctx.r[7].s64 = ctx.r[11].s64 + -16768;
	// 8262E568: 39000006  li r8, 6
	ctx.r[8].s64 = 6;
	// 8262E56C: 388A2CF0  addi r4, r10, 0x2cf0
	ctx.r[4].s64 = ctx.r[10].s64 + 11504;
	// 8262E570: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8262E574: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262E578: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8262E57C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8262E580: 386A5C14  addi r3, r10, 0x5c14
	ctx.r[3].s64 = ctx.r[10].s64 + 23572;
	// 8262E584: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8262E588: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8262E58C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8262E590: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8262E594: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8262E598: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8262E59C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8262E5A0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8262E5A4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8262E5A8: 4BE38879  bl 0x82466e20
	ctx.lr = 0x8262E5AC;
	sub_82466E20(ctx, base);
	// 8262E5AC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8262E5B0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8262E5B4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8262E5B8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8262E5C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8262E5C0 size=108
    let mut pc: u32 = 0x8262E5C0;
    'dispatch: loop {
        match pc {
            0x8262E5C0 => {
    //   block [0x8262E5C0..0x8262E62C)
	// 8262E5C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262E5C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8262E5C8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8262E5CC: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8262E5D0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8262E5D4: 38EBBF10  addi r7, r11, -0x40f0
	ctx.r[7].s64 = ctx.r[11].s64 + -16624;
	// 8262E5D8: 39000006  li r8, 6
	ctx.r[8].s64 = 6;
	// 8262E5DC: 388A2D14  addi r4, r10, 0x2d14
	ctx.r[4].s64 = ctx.r[10].s64 + 11540;
	// 8262E5E0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8262E5E4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262E5E8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8262E5EC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8262E5F0: 386A5C44  addi r3, r10, 0x5c44
	ctx.r[3].s64 = ctx.r[10].s64 + 23620;
	// 8262E5F4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8262E5F8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8262E5FC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8262E600: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8262E604: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8262E608: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8262E60C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8262E610: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8262E614: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8262E618: 4BE38809  bl 0x82466e20
	ctx.lr = 0x8262E61C;
	sub_82466E20(ctx, base);
	// 8262E61C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8262E620: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8262E624: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8262E628: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8262E630(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8262E630 size=100
    let mut pc: u32 = 0x8262E630;
    'dispatch: loop {
        match pc {
            0x8262E630 => {
    //   block [0x8262E630..0x8262E694)
	// 8262E630: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262E634: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8262E638: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8262E63C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262E640: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8262E644: 38AA64E4  addi r5, r10, 0x64e4
	ctx.r[5].s64 = ctx.r[10].s64 + 25828;
	// 8262E648: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8262E64C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8262E650: 388A2D2C  addi r4, r10, 0x2d2c
	ctx.r[4].s64 = ctx.r[10].s64 + 11564;
	// 8262E654: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262E658: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8262E65C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8262E660: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8262E664: 386A5C74  addi r3, r10, 0x5c74
	ctx.r[3].s64 = ctx.r[10].s64 + 23668;
	// 8262E668: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8262E66C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8262E670: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8262E674: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8262E678: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8262E67C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8262E680: 4BE387A1  bl 0x82466e20
	ctx.lr = 0x8262E684;
	sub_82466E20(ctx, base);
	// 8262E684: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8262E688: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8262E68C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8262E690: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8262E698(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8262E698 size=112
    let mut pc: u32 = 0x8262E698;
    'dispatch: loop {
        match pc {
            0x8262E698 => {
    //   block [0x8262E698..0x8262E708)
	// 8262E698: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262E69C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8262E6A0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8262E6A4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262E6A8: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8262E6AC: 38AA5C74  addi r5, r10, 0x5c74
	ctx.r[5].s64 = ctx.r[10].s64 + 23668;
	// 8262E6B0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8262E6B4: 390BBFA0  addi r8, r11, -0x4060
	ctx.r[8].s64 = ctx.r[11].s64 + -16480;
	// 8262E6B8: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 8262E6BC: 388A2D48  addi r4, r10, 0x2d48
	ctx.r[4].s64 = ctx.r[10].s64 + 11592;
	// 8262E6C0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8262E6C4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262E6C8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8262E6CC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8262E6D0: 386A5CA4  addi r3, r10, 0x5ca4
	ctx.r[3].s64 = ctx.r[10].s64 + 23716;
	// 8262E6D4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8262E6D8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8262E6DC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8262E6E0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8262E6E4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8262E6E8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8262E6EC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8262E6F0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8262E6F4: 4BE3872D  bl 0x82466e20
	ctx.lr = 0x8262E6F8;
	sub_82466E20(ctx, base);
	// 8262E6F8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8262E6FC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8262E700: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8262E704: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8262E708(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8262E708 size=108
    let mut pc: u32 = 0x8262E708;
    'dispatch: loop {
        match pc {
            0x8262E708 => {
    //   block [0x8262E708..0x8262E774)
	// 8262E708: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262E70C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8262E710: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8262E714: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8262E718: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8262E71C: 38EBC000  addi r7, r11, -0x4000
	ctx.r[7].s64 = ctx.r[11].s64 + -16384;
	// 8262E720: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 8262E724: 388A2D68  addi r4, r10, 0x2d68
	ctx.r[4].s64 = ctx.r[10].s64 + 11624;
	// 8262E728: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8262E72C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262E730: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8262E734: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8262E738: 386A5CD4  addi r3, r10, 0x5cd4
	ctx.r[3].s64 = ctx.r[10].s64 + 23764;
	// 8262E73C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8262E740: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8262E744: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8262E748: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8262E74C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8262E750: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8262E754: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8262E758: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8262E75C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8262E760: 4BE386C1  bl 0x82466e20
	ctx.lr = 0x8262E764;
	sub_82466E20(ctx, base);
	// 8262E764: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8262E768: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8262E76C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8262E770: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8262E778(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8262E778 size=108
    let mut pc: u32 = 0x8262E778;
    'dispatch: loop {
        match pc {
            0x8262E778 => {
    //   block [0x8262E778..0x8262E7E4)
	// 8262E778: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262E77C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8262E780: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8262E784: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8262E788: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8262E78C: 38EBC030  addi r7, r11, -0x3fd0
	ctx.r[7].s64 = ctx.r[11].s64 + -16336;
	// 8262E790: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 8262E794: 388A2D70  addi r4, r10, 0x2d70
	ctx.r[4].s64 = ctx.r[10].s64 + 11632;
	// 8262E798: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8262E79C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262E7A0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8262E7A4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8262E7A8: 386A5D04  addi r3, r10, 0x5d04
	ctx.r[3].s64 = ctx.r[10].s64 + 23812;
	// 8262E7AC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8262E7B0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8262E7B4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8262E7B8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8262E7BC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8262E7C0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8262E7C4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8262E7C8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8262E7CC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8262E7D0: 4BE38651  bl 0x82466e20
	ctx.lr = 0x8262E7D4;
	sub_82466E20(ctx, base);
	// 8262E7D4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8262E7D8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8262E7DC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8262E7E0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8262E7E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8262E7E8 size=108
    let mut pc: u32 = 0x8262E7E8;
    'dispatch: loop {
        match pc {
            0x8262E7E8 => {
    //   block [0x8262E7E8..0x8262E854)
	// 8262E7E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262E7EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8262E7F0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8262E7F4: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8262E7F8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8262E7FC: 38EBC090  addi r7, r11, -0x3f70
	ctx.r[7].s64 = ctx.r[11].s64 + -16240;
	// 8262E800: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 8262E804: 388A2D84  addi r4, r10, 0x2d84
	ctx.r[4].s64 = ctx.r[10].s64 + 11652;
	// 8262E808: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8262E80C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262E810: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8262E814: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8262E818: 386A5D34  addi r3, r10, 0x5d34
	ctx.r[3].s64 = ctx.r[10].s64 + 23860;
	// 8262E81C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8262E820: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8262E824: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8262E828: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8262E82C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8262E830: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8262E834: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8262E838: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8262E83C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8262E840: 4BE385E1  bl 0x82466e20
	ctx.lr = 0x8262E844;
	sub_82466E20(ctx, base);
	// 8262E844: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8262E848: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8262E84C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8262E850: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8262E858(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8262E858 size=112
    let mut pc: u32 = 0x8262E858;
    'dispatch: loop {
        match pc {
            0x8262E858 => {
    //   block [0x8262E858..0x8262E8C8)
	// 8262E858: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262E85C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8262E860: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8262E864: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8262E868: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8262E86C: 392A475C  addi r9, r10, 0x475c
	ctx.r[9].s64 = ctx.r[10].s64 + 18268;
	// 8262E870: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8262E874: 390BC0F8  addi r8, r11, -0x3f08
	ctx.r[8].s64 = ctx.r[11].s64 + -16136;
	// 8262E878: 38E0000B  li r7, 0xb
	ctx.r[7].s64 = 11;
	// 8262E87C: 388A2E00  addi r4, r10, 0x2e00
	ctx.r[4].s64 = ctx.r[10].s64 + 11776;
	// 8262E880: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8262E884: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262E888: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8262E88C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8262E890: 386A5D64  addi r3, r10, 0x5d64
	ctx.r[3].s64 = ctx.r[10].s64 + 23908;
	// 8262E894: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8262E898: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8262E89C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8262E8A0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8262E8A4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8262E8A8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8262E8AC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8262E8B0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8262E8B4: 4BE3856D  bl 0x82466e20
	ctx.lr = 0x8262E8B8;
	sub_82466E20(ctx, base);
	// 8262E8B8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8262E8BC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8262E8C0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8262E8C4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8262E8C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8262E8C8 size=108
    let mut pc: u32 = 0x8262E8C8;
    'dispatch: loop {
        match pc {
            0x8262E8C8 => {
    //   block [0x8262E8C8..0x8262E934)
	// 8262E8C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262E8CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8262E8D0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8262E8D4: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8262E8D8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8262E8DC: 38EBC200  addi r7, r11, -0x3e00
	ctx.r[7].s64 = ctx.r[11].s64 + -15872;
	// 8262E8E0: 3900000D  li r8, 0xd
	ctx.r[8].s64 = 13;
	// 8262E8E4: 388A2E30  addi r4, r10, 0x2e30
	ctx.r[4].s64 = ctx.r[10].s64 + 11824;
	// 8262E8E8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8262E8EC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262E8F0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8262E8F4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8262E8F8: 386A5D94  addi r3, r10, 0x5d94
	ctx.r[3].s64 = ctx.r[10].s64 + 23956;
	// 8262E8FC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8262E900: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8262E904: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8262E908: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8262E90C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8262E910: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8262E914: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8262E918: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8262E91C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8262E920: 4BE38501  bl 0x82466e20
	ctx.lr = 0x8262E924;
	sub_82466E20(ctx, base);
	// 8262E924: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8262E928: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8262E92C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8262E930: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8262E938(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8262E938 size=108
    let mut pc: u32 = 0x8262E938;
    'dispatch: loop {
        match pc {
            0x8262E938 => {
    //   block [0x8262E938..0x8262E9A4)
	// 8262E938: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262E93C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8262E940: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8262E944: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8262E948: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8262E94C: 38EBC338  addi r7, r11, -0x3cc8
	ctx.r[7].s64 = ctx.r[11].s64 + -15560;
	// 8262E950: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 8262E954: 388A2E64  addi r4, r10, 0x2e64
	ctx.r[4].s64 = ctx.r[10].s64 + 11876;
	// 8262E958: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8262E95C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262E960: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8262E964: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8262E968: 386A5DC4  addi r3, r10, 0x5dc4
	ctx.r[3].s64 = ctx.r[10].s64 + 24004;
	// 8262E96C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8262E970: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8262E974: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8262E978: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8262E97C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8262E980: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8262E984: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8262E988: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8262E98C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8262E990: 4BE38491  bl 0x82466e20
	ctx.lr = 0x8262E994;
	sub_82466E20(ctx, base);
	// 8262E994: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8262E998: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8262E99C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8262E9A0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8262E9A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8262E9A8 size=108
    let mut pc: u32 = 0x8262E9A8;
    'dispatch: loop {
        match pc {
            0x8262E9A8 => {
    //   block [0x8262E9A8..0x8262EA14)
	// 8262E9A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262E9AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8262E9B0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8262E9B4: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8262E9B8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8262E9BC: 38EBC350  addi r7, r11, -0x3cb0
	ctx.r[7].s64 = ctx.r[11].s64 + -15536;
	// 8262E9C0: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 8262E9C4: 388A5948  addi r4, r10, 0x5948
	ctx.r[4].s64 = ctx.r[10].s64 + 22856;
	// 8262E9C8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8262E9CC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262E9D0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8262E9D4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8262E9D8: 386A5DF4  addi r3, r10, 0x5df4
	ctx.r[3].s64 = ctx.r[10].s64 + 24052;
	// 8262E9DC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8262E9E0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8262E9E4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8262E9E8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8262E9EC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8262E9F0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8262E9F4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8262E9F8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8262E9FC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8262EA00: 4BE38421  bl 0x82466e20
	ctx.lr = 0x8262EA04;
	sub_82466E20(ctx, base);
	// 8262EA04: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8262EA08: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8262EA0C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8262EA10: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8262EA18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8262EA18 size=108
    let mut pc: u32 = 0x8262EA18;
    'dispatch: loop {
        match pc {
            0x8262EA18 => {
    //   block [0x8262EA18..0x8262EA84)
	// 8262EA18: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262EA1C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8262EA20: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8262EA24: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8262EA28: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8262EA2C: 38EBC368  addi r7, r11, -0x3c98
	ctx.r[7].s64 = ctx.r[11].s64 + -15512;
	// 8262EA30: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 8262EA34: 388A595C  addi r4, r10, 0x595c
	ctx.r[4].s64 = ctx.r[10].s64 + 22876;
	// 8262EA38: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8262EA3C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262EA40: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8262EA44: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8262EA48: 386A5E24  addi r3, r10, 0x5e24
	ctx.r[3].s64 = ctx.r[10].s64 + 24100;
	// 8262EA4C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8262EA50: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8262EA54: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8262EA58: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8262EA5C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8262EA60: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8262EA64: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8262EA68: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8262EA6C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8262EA70: 4BE383B1  bl 0x82466e20
	ctx.lr = 0x8262EA74;
	sub_82466E20(ctx, base);
	// 8262EA74: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8262EA78: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8262EA7C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8262EA80: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8262EA88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8262EA88 size=100
    let mut pc: u32 = 0x8262EA88;
    'dispatch: loop {
        match pc {
            0x8262EA88 => {
    //   block [0x8262EA88..0x8262EAEC)
	// 8262EA88: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262EA8C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8262EA90: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8262EA94: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262EA98: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8262EA9C: 38AA62D4  addi r5, r10, 0x62d4
	ctx.r[5].s64 = ctx.r[10].s64 + 25300;
	// 8262EAA0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8262EAA4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8262EAA8: 388A2E70  addi r4, r10, 0x2e70
	ctx.r[4].s64 = ctx.r[10].s64 + 11888;
	// 8262EAAC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262EAB0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8262EAB4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8262EAB8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8262EABC: 386A5E54  addi r3, r10, 0x5e54
	ctx.r[3].s64 = ctx.r[10].s64 + 24148;
	// 8262EAC0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8262EAC4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8262EAC8: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8262EACC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8262EAD0: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8262EAD4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8262EAD8: 4BE38349  bl 0x82466e20
	ctx.lr = 0x8262EADC;
	sub_82466E20(ctx, base);
	// 8262EADC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8262EAE0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8262EAE4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8262EAE8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8262EAF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8262EAF0 size=112
    let mut pc: u32 = 0x8262EAF0;
    'dispatch: loop {
        match pc {
            0x8262EAF0 => {
    //   block [0x8262EAF0..0x8262EB60)
	// 8262EAF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262EAF4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8262EAF8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8262EAFC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262EB00: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8262EB04: 38AA5E54  addi r5, r10, 0x5e54
	ctx.r[5].s64 = ctx.r[10].s64 + 24148;
	// 8262EB08: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8262EB0C: 390BC398  addi r8, r11, -0x3c68
	ctx.r[8].s64 = ctx.r[11].s64 + -15464;
	// 8262EB10: 39200006  li r9, 6
	ctx.r[9].s64 = 6;
	// 8262EB14: 388A2EDC  addi r4, r10, 0x2edc
	ctx.r[4].s64 = ctx.r[10].s64 + 11996;
	// 8262EB18: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8262EB1C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262EB20: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8262EB24: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8262EB28: 386A5E84  addi r3, r10, 0x5e84
	ctx.r[3].s64 = ctx.r[10].s64 + 24196;
	// 8262EB2C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8262EB30: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8262EB34: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8262EB38: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8262EB3C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8262EB40: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8262EB44: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8262EB48: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8262EB4C: 4BE382D5  bl 0x82466e20
	ctx.lr = 0x8262EB50;
	sub_82466E20(ctx, base);
	// 8262EB50: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8262EB54: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8262EB58: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8262EB5C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8262EB60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8262EB60 size=100
    let mut pc: u32 = 0x8262EB60;
    'dispatch: loop {
        match pc {
            0x8262EB60 => {
    //   block [0x8262EB60..0x8262EBC4)
	// 8262EB60: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262EB64: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8262EB68: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8262EB6C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262EB70: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8262EB74: 38AA5E84  addi r5, r10, 0x5e84
	ctx.r[5].s64 = ctx.r[10].s64 + 24196;
	// 8262EB78: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8262EB7C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8262EB80: 388A2EF8  addi r4, r10, 0x2ef8
	ctx.r[4].s64 = ctx.r[10].s64 + 12024;
	// 8262EB84: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262EB88: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8262EB8C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8262EB90: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8262EB94: 386A5EB4  addi r3, r10, 0x5eb4
	ctx.r[3].s64 = ctx.r[10].s64 + 24244;
	// 8262EB98: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8262EB9C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8262EBA0: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8262EBA4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8262EBA8: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8262EBAC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8262EBB0: 4BE38271  bl 0x82466e20
	ctx.lr = 0x8262EBB4;
	sub_82466E20(ctx, base);
	// 8262EBB4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8262EBB8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8262EBBC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8262EBC0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8262EBC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8262EBC8 size=108
    let mut pc: u32 = 0x8262EBC8;
    'dispatch: loop {
        match pc {
            0x8262EBC8 => {
    //   block [0x8262EBC8..0x8262EC34)
	// 8262EBC8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262EBCC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8262EBD0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8262EBD4: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8262EBD8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8262EBDC: 38EBC428  addi r7, r11, -0x3bd8
	ctx.r[7].s64 = ctx.r[11].s64 + -15320;
	// 8262EBE0: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 8262EBE4: 388A2F1C  addi r4, r10, 0x2f1c
	ctx.r[4].s64 = ctx.r[10].s64 + 12060;
	// 8262EBE8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8262EBEC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262EBF0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8262EBF4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8262EBF8: 386A5EE4  addi r3, r10, 0x5ee4
	ctx.r[3].s64 = ctx.r[10].s64 + 24292;
	// 8262EBFC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8262EC00: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8262EC04: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8262EC08: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8262EC0C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8262EC10: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8262EC14: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8262EC18: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8262EC1C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8262EC20: 4BE38201  bl 0x82466e20
	ctx.lr = 0x8262EC24;
	sub_82466E20(ctx, base);
	// 8262EC24: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8262EC28: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8262EC2C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8262EC30: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8262EC38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8262EC38 size=112
    let mut pc: u32 = 0x8262EC38;
    'dispatch: loop {
        match pc {
            0x8262EC38 => {
    //   block [0x8262EC38..0x8262ECA8)
	// 8262EC38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262EC3C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8262EC40: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8262EC44: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262EC48: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8262EC4C: 38AA5E54  addi r5, r10, 0x5e54
	ctx.r[5].s64 = ctx.r[10].s64 + 24148;
	// 8262EC50: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8262EC54: 390BC470  addi r8, r11, -0x3b90
	ctx.r[8].s64 = ctx.r[11].s64 + -15248;
	// 8262EC58: 39200005  li r9, 5
	ctx.r[9].s64 = 5;
	// 8262EC5C: 388A2F38  addi r4, r10, 0x2f38
	ctx.r[4].s64 = ctx.r[10].s64 + 12088;
	// 8262EC60: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8262EC64: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262EC68: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8262EC6C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8262EC70: 386A5F14  addi r3, r10, 0x5f14
	ctx.r[3].s64 = ctx.r[10].s64 + 24340;
	// 8262EC74: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8262EC78: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8262EC7C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8262EC80: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8262EC84: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8262EC88: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8262EC8C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8262EC90: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8262EC94: 4BE3818D  bl 0x82466e20
	ctx.lr = 0x8262EC98;
	sub_82466E20(ctx, base);
	// 8262EC98: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8262EC9C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8262ECA0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8262ECA4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8262ECA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8262ECA8 size=108
    let mut pc: u32 = 0x8262ECA8;
    'dispatch: loop {
        match pc {
            0x8262ECA8 => {
    //   block [0x8262ECA8..0x8262ED14)
	// 8262ECA8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262ECAC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8262ECB0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8262ECB4: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8262ECB8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8262ECBC: 38EBC4E8  addi r7, r11, -0x3b18
	ctx.r[7].s64 = ctx.r[11].s64 + -15128;
	// 8262ECC0: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 8262ECC4: 388A2F4C  addi r4, r10, 0x2f4c
	ctx.r[4].s64 = ctx.r[10].s64 + 12108;
	// 8262ECC8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8262ECCC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262ECD0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8262ECD4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8262ECD8: 386A5F44  addi r3, r10, 0x5f44
	ctx.r[3].s64 = ctx.r[10].s64 + 24388;
	// 8262ECDC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8262ECE0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8262ECE4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8262ECE8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8262ECEC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8262ECF0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8262ECF4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8262ECF8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8262ECFC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8262ED00: 4BE38121  bl 0x82466e20
	ctx.lr = 0x8262ED04;
	sub_82466E20(ctx, base);
	// 8262ED04: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8262ED08: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8262ED0C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8262ED10: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8262ED18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8262ED18 size=24
    let mut pc: u32 = 0x8262ED18;
    'dispatch: loop {
        match pc {
            0x8262ED18 => {
    //   block [0x8262ED18..0x8262ED30)
	// 8262ED18: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8262ED1C: 3D408277  lis r10, -0x7d89
	ctx.r[10].s64 = -2106130432;
	// 8262ED20: 394A1978  addi r10, r10, 0x1978
	ctx.r[10].s64 = ctx.r[10].s64 + 6520;
	// 8262ED24: 816BC0F4  lwz r11, -0x3f0c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-16140 as u32) ) } as u64;
	// 8262ED28: 916A0008  stw r11, 8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 8262ED2C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8262ED30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8262ED30 size=116
    let mut pc: u32 = 0x8262ED30;
    'dispatch: loop {
        match pc {
            0x8262ED30 => {
    //   block [0x8262ED30..0x8262EDA4)
	// 8262ED30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262ED34: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8262ED38: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8262ED3C: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 8262ED40: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262ED44: 392B4788  addi r9, r11, 0x4788
	ctx.r[9].s64 = ctx.r[11].s64 + 18312;
	// 8262ED48: 38AA5E54  addi r5, r10, 0x5e54
	ctx.r[5].s64 = ctx.r[10].s64 + 24148;
	// 8262ED4C: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8262ED50: 38E90014  addi r7, r9, 0x14
	ctx.r[7].s64 = ctx.r[9].s64 + 20;
	// 8262ED54: 38C0000C  li r6, 0xc
	ctx.r[6].s64 = 12;
	// 8262ED58: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8262ED5C: 388A3040  addi r4, r10, 0x3040
	ctx.r[4].s64 = ctx.r[10].s64 + 12352;
	// 8262ED60: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8262ED64: 396B1978  addi r11, r11, 0x1978
	ctx.r[11].s64 = ctx.r[11].s64 + 6520;
	// 8262ED68: 90E10064  stw r7, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[7].u32 ) };
	// 8262ED6C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262ED70: 90C1005C  stw r6, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[6].u32 ) };
	// 8262ED74: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8262ED78: 386A5F74  addi r3, r10, 0x5f74
	ctx.r[3].s64 = ctx.r[10].s64 + 24436;
	// 8262ED7C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8262ED80: 91010074  stw r8, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[8].u32 ) };
	// 8262ED84: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8262ED88: 9101006C  stw r8, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[8].u32 ) };
	// 8262ED8C: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8262ED90: 4BE38091  bl 0x82466e20
	ctx.lr = 0x8262ED94;
	sub_82466E20(ctx, base);
	// 8262ED94: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8262ED98: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8262ED9C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8262EDA0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8262EDA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8262EDA8 size=112
    let mut pc: u32 = 0x8262EDA8;
    'dispatch: loop {
        match pc {
            0x8262EDA8 => {
    //   block [0x8262EDA8..0x8262EE18)
	// 8262EDA8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262EDAC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8262EDB0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8262EDB4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262EDB8: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8262EDBC: 38AA5E54  addi r5, r10, 0x5e54
	ctx.r[5].s64 = ctx.r[10].s64 + 24148;
	// 8262EDC0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8262EDC4: 390BC530  addi r8, r11, -0x3ad0
	ctx.r[8].s64 = ctx.r[11].s64 + -15056;
	// 8262EDC8: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 8262EDCC: 388A596C  addi r4, r10, 0x596c
	ctx.r[4].s64 = ctx.r[10].s64 + 22892;
	// 8262EDD0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8262EDD4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262EDD8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8262EDDC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8262EDE0: 386A5FA4  addi r3, r10, 0x5fa4
	ctx.r[3].s64 = ctx.r[10].s64 + 24484;
	// 8262EDE4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8262EDE8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8262EDEC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8262EDF0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8262EDF4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8262EDF8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8262EDFC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8262EE00: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8262EE04: 4BE3801D  bl 0x82466e20
	ctx.lr = 0x8262EE08;
	sub_82466E20(ctx, base);
	// 8262EE08: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8262EE0C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8262EE10: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8262EE14: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8262EE18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8262EE18 size=112
    let mut pc: u32 = 0x8262EE18;
    'dispatch: loop {
        match pc {
            0x8262EE18 => {
    //   block [0x8262EE18..0x8262EE88)
	// 8262EE18: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262EE1C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8262EE20: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8262EE24: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262EE28: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8262EE2C: 38AA5E54  addi r5, r10, 0x5e54
	ctx.r[5].s64 = ctx.r[10].s64 + 24148;
	// 8262EE30: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8262EE34: 390BC560  addi r8, r11, -0x3aa0
	ctx.r[8].s64 = ctx.r[11].s64 + -15008;
	// 8262EE38: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8262EE3C: 388A30D8  addi r4, r10, 0x30d8
	ctx.r[4].s64 = ctx.r[10].s64 + 12504;
	// 8262EE40: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8262EE44: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262EE48: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8262EE4C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8262EE50: 386A5FD4  addi r3, r10, 0x5fd4
	ctx.r[3].s64 = ctx.r[10].s64 + 24532;
	// 8262EE54: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8262EE58: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8262EE5C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8262EE60: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8262EE64: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8262EE68: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8262EE6C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8262EE70: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8262EE74: 4BE37FAD  bl 0x82466e20
	ctx.lr = 0x8262EE78;
	sub_82466E20(ctx, base);
	// 8262EE78: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8262EE7C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8262EE80: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8262EE84: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8262EE88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8262EE88 size=112
    let mut pc: u32 = 0x8262EE88;
    'dispatch: loop {
        match pc {
            0x8262EE88 => {
    //   block [0x8262EE88..0x8262EEF8)
	// 8262EE88: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262EE8C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8262EE90: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8262EE94: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262EE98: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8262EE9C: 38AA5E54  addi r5, r10, 0x5e54
	ctx.r[5].s64 = ctx.r[10].s64 + 24148;
	// 8262EEA0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8262EEA4: 390BC578  addi r8, r11, -0x3a88
	ctx.r[8].s64 = ctx.r[11].s64 + -14984;
	// 8262EEA8: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 8262EEAC: 388A5984  addi r4, r10, 0x5984
	ctx.r[4].s64 = ctx.r[10].s64 + 22916;
	// 8262EEB0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8262EEB4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262EEB8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8262EEBC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8262EEC0: 386A6004  addi r3, r10, 0x6004
	ctx.r[3].s64 = ctx.r[10].s64 + 24580;
	// 8262EEC4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8262EEC8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8262EECC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8262EED0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8262EED4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8262EED8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8262EEDC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8262EEE0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8262EEE4: 4BE37F3D  bl 0x82466e20
	ctx.lr = 0x8262EEE8;
	sub_82466E20(ctx, base);
	// 8262EEE8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8262EEEC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8262EEF0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8262EEF4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8262EEF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8262EEF8 size=100
    let mut pc: u32 = 0x8262EEF8;
    'dispatch: loop {
        match pc {
            0x8262EEF8 => {
    //   block [0x8262EEF8..0x8262EF5C)
	// 8262EEF8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262EEFC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8262EF00: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8262EF04: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262EF08: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8262EF0C: 38AA62D4  addi r5, r10, 0x62d4
	ctx.r[5].s64 = ctx.r[10].s64 + 25300;
	// 8262EF10: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8262EF14: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8262EF18: 388A30F4  addi r4, r10, 0x30f4
	ctx.r[4].s64 = ctx.r[10].s64 + 12532;
	// 8262EF1C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262EF20: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8262EF24: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8262EF28: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8262EF2C: 386A6034  addi r3, r10, 0x6034
	ctx.r[3].s64 = ctx.r[10].s64 + 24628;
	// 8262EF30: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8262EF34: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8262EF38: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8262EF3C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8262EF40: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8262EF44: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8262EF48: 4BE37ED9  bl 0x82466e20
	ctx.lr = 0x8262EF4C;
	sub_82466E20(ctx, base);
	// 8262EF4C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8262EF50: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8262EF54: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8262EF58: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8262EF60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8262EF60 size=116
    let mut pc: u32 = 0x8262EF60;
    'dispatch: loop {
        match pc {
            0x8262EF60 => {
    //   block [0x8262EF60..0x8262EFD4)
	// 8262EF60: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262EF64: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8262EF68: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8262EF6C: 3D408277  lis r10, -0x7d89
	ctx.r[10].s64 = -2106130432;
	// 8262EF70: 38E00011  li r7, 0x11
	ctx.r[7].s64 = 17;
	// 8262EF74: 390AC5A8  addi r8, r10, -0x3a58
	ctx.r[8].s64 = ctx.r[10].s64 + -14936;
	// 8262EF78: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262EF7C: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 8262EF80: 38AA6034  addi r5, r10, 0x6034
	ctx.r[5].s64 = ctx.r[10].s64 + 24628;
	// 8262EF84: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8262EF88: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8262EF8C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8262EF90: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8262EF94: 388A31E0  addi r4, r10, 0x31e0
	ctx.r[4].s64 = ctx.r[10].s64 + 12768;
	// 8262EF98: 396B47D0  addi r11, r11, 0x47d0
	ctx.r[11].s64 = ctx.r[11].s64 + 18384;
	// 8262EF9C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262EFA0: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8262EFA4: 386A6064  addi r3, r10, 0x6064
	ctx.r[3].s64 = ctx.r[10].s64 + 24676;
	// 8262EFA8: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 8262EFAC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8262EFB0: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 8262EFB4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8262EFB8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8262EFBC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8262EFC0: 4BE37E61  bl 0x82466e20
	ctx.lr = 0x8262EFC4;
	sub_82466E20(ctx, base);
	// 8262EFC4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8262EFC8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8262EFCC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8262EFD0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8262EFD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8262EFD8 size=112
    let mut pc: u32 = 0x8262EFD8;
    'dispatch: loop {
        match pc {
            0x8262EFD8 => {
    //   block [0x8262EFD8..0x8262F048)
	// 8262EFD8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262EFDC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8262EFE0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8262EFE4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262EFE8: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8262EFEC: 38AA6034  addi r5, r10, 0x6034
	ctx.r[5].s64 = ctx.r[10].s64 + 24628;
	// 8262EFF0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8262EFF4: 390BC740  addi r8, r11, -0x38c0
	ctx.r[8].s64 = ctx.r[11].s64 + -14528;
	// 8262EFF8: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8262EFFC: 388A5998  addi r4, r10, 0x5998
	ctx.r[4].s64 = ctx.r[10].s64 + 22936;
	// 8262F000: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8262F004: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262F008: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8262F00C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8262F010: 386A6094  addi r3, r10, 0x6094
	ctx.r[3].s64 = ctx.r[10].s64 + 24724;
	// 8262F014: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8262F018: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8262F01C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8262F020: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8262F024: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8262F028: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8262F02C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8262F030: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8262F034: 4BE37DED  bl 0x82466e20
	ctx.lr = 0x8262F038;
	sub_82466E20(ctx, base);
	// 8262F038: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8262F03C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8262F040: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8262F044: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8262F048(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8262F048 size=112
    let mut pc: u32 = 0x8262F048;
    'dispatch: loop {
        match pc {
            0x8262F048 => {
    //   block [0x8262F048..0x8262F0B8)
	// 8262F048: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262F04C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8262F050: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8262F054: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262F058: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8262F05C: 38AA6034  addi r5, r10, 0x6034
	ctx.r[5].s64 = ctx.r[10].s64 + 24628;
	// 8262F060: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8262F064: 390BC758  addi r8, r11, -0x38a8
	ctx.r[8].s64 = ctx.r[11].s64 + -14504;
	// 8262F068: 39200007  li r9, 7
	ctx.r[9].s64 = 7;
	// 8262F06C: 388A3210  addi r4, r10, 0x3210
	ctx.r[4].s64 = ctx.r[10].s64 + 12816;
	// 8262F070: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8262F074: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262F078: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8262F07C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8262F080: 386A60C4  addi r3, r10, 0x60c4
	ctx.r[3].s64 = ctx.r[10].s64 + 24772;
	// 8262F084: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8262F088: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8262F08C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8262F090: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8262F094: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8262F098: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8262F09C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8262F0A0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8262F0A4: 4BE37D7D  bl 0x82466e20
	ctx.lr = 0x8262F0A8;
	sub_82466E20(ctx, base);
	// 8262F0A8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8262F0AC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8262F0B0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8262F0B4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8262F0B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8262F0B8 size=112
    let mut pc: u32 = 0x8262F0B8;
    'dispatch: loop {
        match pc {
            0x8262F0B8 => {
    //   block [0x8262F0B8..0x8262F128)
	// 8262F0B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262F0BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8262F0C0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8262F0C4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262F0C8: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8262F0CC: 38AA6034  addi r5, r10, 0x6034
	ctx.r[5].s64 = ctx.r[10].s64 + 24628;
	// 8262F0D0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8262F0D4: 390BC800  addi r8, r11, -0x3800
	ctx.r[8].s64 = ctx.r[11].s64 + -14336;
	// 8262F0D8: 39200009  li r9, 9
	ctx.r[9].s64 = 9;
	// 8262F0DC: 388A3260  addi r4, r10, 0x3260
	ctx.r[4].s64 = ctx.r[10].s64 + 12896;
	// 8262F0E0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8262F0E4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262F0E8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8262F0EC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8262F0F0: 386A60F4  addi r3, r10, 0x60f4
	ctx.r[3].s64 = ctx.r[10].s64 + 24820;
	// 8262F0F4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8262F0F8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8262F0FC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8262F100: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8262F104: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8262F108: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8262F10C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8262F110: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8262F114: 4BE37D0D  bl 0x82466e20
	ctx.lr = 0x8262F118;
	sub_82466E20(ctx, base);
	// 8262F118: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8262F11C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8262F120: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8262F124: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8262F128(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8262F128 size=112
    let mut pc: u32 = 0x8262F128;
    'dispatch: loop {
        match pc {
            0x8262F128 => {
    //   block [0x8262F128..0x8262F198)
	// 8262F128: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262F12C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8262F130: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8262F134: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262F138: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8262F13C: 38AA6034  addi r5, r10, 0x6034
	ctx.r[5].s64 = ctx.r[10].s64 + 24628;
	// 8262F140: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8262F144: 390BC8D8  addi r8, r11, -0x3728
	ctx.r[8].s64 = ctx.r[11].s64 + -14120;
	// 8262F148: 3920000B  li r9, 0xb
	ctx.r[9].s64 = 11;
	// 8262F14C: 388A3274  addi r4, r10, 0x3274
	ctx.r[4].s64 = ctx.r[10].s64 + 12916;
	// 8262F150: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8262F154: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262F158: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8262F15C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8262F160: 386A6124  addi r3, r10, 0x6124
	ctx.r[3].s64 = ctx.r[10].s64 + 24868;
	// 8262F164: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8262F168: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8262F16C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8262F170: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8262F174: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8262F178: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8262F17C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8262F180: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8262F184: 4BE37C9D  bl 0x82466e20
	ctx.lr = 0x8262F188;
	sub_82466E20(ctx, base);
	// 8262F188: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8262F18C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8262F190: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8262F194: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8262F198(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8262F198 size=112
    let mut pc: u32 = 0x8262F198;
    'dispatch: loop {
        match pc {
            0x8262F198 => {
    //   block [0x8262F198..0x8262F208)
	// 8262F198: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262F19C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8262F1A0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8262F1A4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262F1A8: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8262F1AC: 38AA6034  addi r5, r10, 0x6034
	ctx.r[5].s64 = ctx.r[10].s64 + 24628;
	// 8262F1B0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8262F1B4: 390BC9E0  addi r8, r11, -0x3620
	ctx.r[8].s64 = ctx.r[11].s64 + -13856;
	// 8262F1B8: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 8262F1BC: 388A43BC  addi r4, r10, 0x43bc
	ctx.r[4].s64 = ctx.r[10].s64 + 17340;
	// 8262F1C0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8262F1C4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262F1C8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8262F1CC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8262F1D0: 386A6154  addi r3, r10, 0x6154
	ctx.r[3].s64 = ctx.r[10].s64 + 24916;
	// 8262F1D4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8262F1D8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8262F1DC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8262F1E0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8262F1E4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8262F1E8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8262F1EC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8262F1F0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8262F1F4: 4BE37C2D  bl 0x82466e20
	ctx.lr = 0x8262F1F8;
	sub_82466E20(ctx, base);
	// 8262F1F8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8262F1FC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8262F200: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8262F204: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8262F208(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8262F208 size=24
    let mut pc: u32 = 0x8262F208;
    'dispatch: loop {
        match pc {
            0x8262F208 => {
    //   block [0x8262F208..0x8262F220)
	// 8262F208: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8262F20C: 3D408277  lis r10, -0x7d89
	ctx.r[10].s64 = -2106130432;
	// 8262F210: 394A1A98  addi r10, r10, 0x1a98
	ctx.r[10].s64 = ctx.r[10].s64 + 6808;
	// 8262F214: 816BCA10  lwz r11, -0x35f0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-13808 as u32) ) } as u64;
	// 8262F218: 916A0068  stw r11, 0x68(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(104 as u32), ctx.r[11].u32 ) };
	// 8262F21C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8262F220(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8262F220 size=116
    let mut pc: u32 = 0x8262F220;
    'dispatch: loop {
        match pc {
            0x8262F220 => {
    //   block [0x8262F220..0x8262F294)
	// 8262F220: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262F224: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8262F228: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8262F22C: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 8262F230: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262F234: 392B482C  addi r9, r11, 0x482c
	ctx.r[9].s64 = ctx.r[11].s64 + 18476;
	// 8262F238: 38AA6034  addi r5, r10, 0x6034
	ctx.r[5].s64 = ctx.r[10].s64 + 24628;
	// 8262F23C: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8262F240: 38E90014  addi r7, r9, 0x14
	ctx.r[7].s64 = ctx.r[9].s64 + 20;
	// 8262F244: 38C00011  li r6, 0x11
	ctx.r[6].s64 = 17;
	// 8262F248: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8262F24C: 388A4454  addi r4, r10, 0x4454
	ctx.r[4].s64 = ctx.r[10].s64 + 17492;
	// 8262F250: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8262F254: 396B1A98  addi r11, r11, 0x1a98
	ctx.r[11].s64 = ctx.r[11].s64 + 6808;
	// 8262F258: 90E10064  stw r7, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[7].u32 ) };
	// 8262F25C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262F260: 90C1005C  stw r6, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[6].u32 ) };
	// 8262F264: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8262F268: 386A6184  addi r3, r10, 0x6184
	ctx.r[3].s64 = ctx.r[10].s64 + 24964;
	// 8262F26C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8262F270: 91010074  stw r8, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[8].u32 ) };
	// 8262F274: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8262F278: 9101006C  stw r8, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[8].u32 ) };
	// 8262F27C: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8262F280: 4BE37BA1  bl 0x82466e20
	ctx.lr = 0x8262F284;
	sub_82466E20(ctx, base);
	// 8262F284: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8262F288: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8262F28C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8262F290: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8262F298(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8262F298 size=116
    let mut pc: u32 = 0x8262F298;
    'dispatch: loop {
        match pc {
            0x8262F298 => {
    //   block [0x8262F298..0x8262F30C)
	// 8262F298: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262F29C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8262F2A0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8262F2A4: 3D408277  lis r10, -0x7d89
	ctx.r[10].s64 = -2106130432;
	// 8262F2A8: 38E00002  li r7, 2
	ctx.r[7].s64 = 2;
	// 8262F2AC: 390ACA14  addi r8, r10, -0x35ec
	ctx.r[8].s64 = ctx.r[10].s64 + -13804;
	// 8262F2B0: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262F2B4: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 8262F2B8: 38AA64E4  addi r5, r10, 0x64e4
	ctx.r[5].s64 = ctx.r[10].s64 + 25828;
	// 8262F2BC: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8262F2C0: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8262F2C4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8262F2C8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8262F2CC: 388A32A4  addi r4, r10, 0x32a4
	ctx.r[4].s64 = ctx.r[10].s64 + 12964;
	// 8262F2D0: 396B489C  addi r11, r11, 0x489c
	ctx.r[11].s64 = ctx.r[11].s64 + 18588;
	// 8262F2D4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262F2D8: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8262F2DC: 386A61B4  addi r3, r10, 0x61b4
	ctx.r[3].s64 = ctx.r[10].s64 + 25012;
	// 8262F2E0: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 8262F2E4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8262F2E8: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 8262F2EC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8262F2F0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8262F2F4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8262F2F8: 4BE37B29  bl 0x82466e20
	ctx.lr = 0x8262F2FC;
	sub_82466E20(ctx, base);
	// 8262F2FC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8262F300: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8262F304: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8262F308: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8262F310(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8262F310 size=24
    let mut pc: u32 = 0x8262F310;
    'dispatch: loop {
        match pc {
            0x8262F310 => {
    //   block [0x8262F310..0x8262F328)
	// 8262F310: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8262F314: 3D408277  lis r10, -0x7d89
	ctx.r[10].s64 = -2106130432;
	// 8262F318: 394A1C30  addi r10, r10, 0x1c30
	ctx.r[10].s64 = ctx.r[10].s64 + 7216;
	// 8262F31C: 816BCA44  lwz r11, -0x35bc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-13756 as u32) ) } as u64;
	// 8262F320: 916A00C8  stw r11, 0xc8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(200 as u32), ctx.r[11].u32 ) };
	// 8262F324: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8262F328(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8262F328 size=116
    let mut pc: u32 = 0x8262F328;
    'dispatch: loop {
        match pc {
            0x8262F328 => {
    //   block [0x8262F328..0x8262F39C)
	// 8262F328: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262F32C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8262F330: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8262F334: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 8262F338: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262F33C: 392B48C0  addi r9, r11, 0x48c0
	ctx.r[9].s64 = ctx.r[11].s64 + 18624;
	// 8262F340: 38AA6034  addi r5, r10, 0x6034
	ctx.r[5].s64 = ctx.r[10].s64 + 24628;
	// 8262F344: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8262F348: 38E90018  addi r7, r9, 0x18
	ctx.r[7].s64 = ctx.r[9].s64 + 24;
	// 8262F34C: 38C00016  li r6, 0x16
	ctx.r[6].s64 = 22;
	// 8262F350: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8262F354: 388A3300  addi r4, r10, 0x3300
	ctx.r[4].s64 = ctx.r[10].s64 + 13056;
	// 8262F358: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8262F35C: 396B1C30  addi r11, r11, 0x1c30
	ctx.r[11].s64 = ctx.r[11].s64 + 7216;
	// 8262F360: 90E10064  stw r7, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[7].u32 ) };
	// 8262F364: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262F368: 90C1005C  stw r6, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[6].u32 ) };
	// 8262F36C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8262F370: 386A61E4  addi r3, r10, 0x61e4
	ctx.r[3].s64 = ctx.r[10].s64 + 25060;
	// 8262F374: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8262F378: 91010074  stw r8, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[8].u32 ) };
	// 8262F37C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8262F380: 9101006C  stw r8, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[8].u32 ) };
	// 8262F384: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8262F388: 4BE37A99  bl 0x82466e20
	ctx.lr = 0x8262F38C;
	sub_82466E20(ctx, base);
	// 8262F38C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8262F390: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8262F394: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8262F398: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8262F3A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8262F3A0 size=108
    let mut pc: u32 = 0x8262F3A0;
    'dispatch: loop {
        match pc {
            0x8262F3A0 => {
    //   block [0x8262F3A0..0x8262F40C)
	// 8262F3A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262F3A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8262F3A8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8262F3AC: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8262F3B0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8262F3B4: 38EBCA48  addi r7, r11, -0x35b8
	ctx.r[7].s64 = ctx.r[11].s64 + -13752;
	// 8262F3B8: 3900000C  li r8, 0xc
	ctx.r[8].s64 = 12;
	// 8262F3BC: 388A59F4  addi r4, r10, 0x59f4
	ctx.r[4].s64 = ctx.r[10].s64 + 23028;
	// 8262F3C0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8262F3C4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262F3C8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8262F3CC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8262F3D0: 386A6214  addi r3, r10, 0x6214
	ctx.r[3].s64 = ctx.r[10].s64 + 25108;
	// 8262F3D4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8262F3D8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8262F3DC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8262F3E0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8262F3E4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8262F3E8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8262F3EC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8262F3F0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8262F3F4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8262F3F8: 4BE37A29  bl 0x82466e20
	ctx.lr = 0x8262F3FC;
	sub_82466E20(ctx, base);
	// 8262F3FC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8262F400: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8262F404: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8262F408: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8262F410(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8262F410 size=116
    let mut pc: u32 = 0x8262F410;
    'dispatch: loop {
        match pc {
            0x8262F410 => {
    //   block [0x8262F410..0x8262F484)
	// 8262F410: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262F414: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8262F418: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8262F41C: 3D408277  lis r10, -0x7d89
	ctx.r[10].s64 = -2106130432;
	// 8262F420: 38E0000E  li r7, 0xe
	ctx.r[7].s64 = 14;
	// 8262F424: 390ACB68  addi r8, r10, -0x3498
	ctx.r[8].s64 = ctx.r[10].s64 + -13464;
	// 8262F428: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262F42C: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 8262F430: 38AA6034  addi r5, r10, 0x6034
	ctx.r[5].s64 = ctx.r[10].s64 + 24628;
	// 8262F434: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8262F438: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8262F43C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8262F440: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8262F444: 388A3394  addi r4, r10, 0x3394
	ctx.r[4].s64 = ctx.r[10].s64 + 13204;
	// 8262F448: 396B4940  addi r11, r11, 0x4940
	ctx.r[11].s64 = ctx.r[11].s64 + 18752;
	// 8262F44C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262F450: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8262F454: 386A6244  addi r3, r10, 0x6244
	ctx.r[3].s64 = ctx.r[10].s64 + 25156;
	// 8262F458: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 8262F45C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8262F460: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 8262F464: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8262F468: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8262F46C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8262F470: 4BE379B1  bl 0x82466e20
	ctx.lr = 0x8262F474;
	sub_82466E20(ctx, base);
	// 8262F474: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8262F478: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8262F47C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8262F480: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8262F488(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8262F488 size=112
    let mut pc: u32 = 0x8262F488;
    'dispatch: loop {
        match pc {
            0x8262F488 => {
    //   block [0x8262F488..0x8262F4F8)
	// 8262F488: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262F48C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8262F490: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8262F494: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262F498: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8262F49C: 38AA60F4  addi r5, r10, 0x60f4
	ctx.r[5].s64 = ctx.r[10].s64 + 24820;
	// 8262F4A0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8262F4A4: 390BCCB8  addi r8, r11, -0x3348
	ctx.r[8].s64 = ctx.r[11].s64 + -13128;
	// 8262F4A8: 39200005  li r9, 5
	ctx.r[9].s64 = 5;
	// 8262F4AC: 388A33D4  addi r4, r10, 0x33d4
	ctx.r[4].s64 = ctx.r[10].s64 + 13268;
	// 8262F4B0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8262F4B4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262F4B8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8262F4BC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8262F4C0: 386A6274  addi r3, r10, 0x6274
	ctx.r[3].s64 = ctx.r[10].s64 + 25204;
	// 8262F4C4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8262F4C8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8262F4CC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8262F4D0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8262F4D4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8262F4D8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8262F4DC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8262F4E0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8262F4E4: 4BE3793D  bl 0x82466e20
	ctx.lr = 0x8262F4E8;
	sub_82466E20(ctx, base);
	// 8262F4E8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8262F4EC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8262F4F0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8262F4F4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8262F4F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8262F4F8 size=24
    let mut pc: u32 = 0x8262F4F8;
    'dispatch: loop {
        match pc {
            0x8262F4F8 => {
    //   block [0x8262F4F8..0x8262F510)
	// 8262F4F8: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8262F4FC: 3D408277  lis r10, -0x7d89
	ctx.r[10].s64 = -2106130432;
	// 8262F500: 394A1E40  addi r10, r10, 0x1e40
	ctx.r[10].s64 = ctx.r[10].s64 + 7744;
	// 8262F504: 816BCD30  lwz r11, -0x32d0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-13008 as u32) ) } as u64;
	// 8262F508: 916A0008  stw r11, 8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 8262F50C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8262F510(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8262F510 size=116
    let mut pc: u32 = 0x8262F510;
    'dispatch: loop {
        match pc {
            0x8262F510 => {
    //   block [0x8262F510..0x8262F584)
	// 8262F510: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262F514: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8262F518: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8262F51C: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8262F520: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8262F524: 390B1E40  addi r8, r11, 0x1e40
	ctx.r[8].s64 = ctx.r[11].s64 + 7744;
	// 8262F528: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8262F52C: 392A4998  addi r9, r10, 0x4998
	ctx.r[9].s64 = ctx.r[10].s64 + 18840;
	// 8262F530: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262F534: 38E00006  li r7, 6
	ctx.r[7].s64 = 6;
	// 8262F538: 38AA6034  addi r5, r10, 0x6034
	ctx.r[5].s64 = ctx.r[10].s64 + 24628;
	// 8262F53C: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8262F540: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8262F544: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8262F548: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8262F54C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8262F550: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8262F554: 3D608294  lis r11, -0x7d6c
	ctx.r[11].s64 = -2104229888;
	// 8262F558: 388A5A2C  addi r4, r10, 0x5a2c
	ctx.r[4].s64 = ctx.r[10].s64 + 23084;
	// 8262F55C: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8262F560: 386B62A4  addi r3, r11, 0x62a4
	ctx.r[3].s64 = ctx.r[11].s64 + 25252;
	// 8262F564: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8262F568: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8262F56C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8262F570: 4BE378B1  bl 0x82466e20
	ctx.lr = 0x8262F574;
	sub_82466E20(ctx, base);
	// 8262F574: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8262F578: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8262F57C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8262F580: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8262F588(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8262F588 size=112
    let mut pc: u32 = 0x8262F588;
    'dispatch: loop {
        match pc {
            0x8262F588 => {
    //   block [0x8262F588..0x8262F5F8)
	// 8262F588: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262F58C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8262F590: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8262F594: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262F598: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8262F59C: 38AA64E4  addi r5, r10, 0x64e4
	ctx.r[5].s64 = ctx.r[10].s64 + 25828;
	// 8262F5A0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8262F5A4: 390BCD34  addi r8, r11, -0x32cc
	ctx.r[8].s64 = ctx.r[11].s64 + -13004;
	// 8262F5A8: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8262F5AC: 388A33FC  addi r4, r10, 0x33fc
	ctx.r[4].s64 = ctx.r[10].s64 + 13308;
	// 8262F5B0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8262F5B4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262F5B8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8262F5BC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8262F5C0: 386A62D4  addi r3, r10, 0x62d4
	ctx.r[3].s64 = ctx.r[10].s64 + 25300;
	// 8262F5C4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8262F5C8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8262F5CC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8262F5D0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8262F5D4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8262F5D8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8262F5DC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8262F5E0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8262F5E4: 4BE3783D  bl 0x82466e20
	ctx.lr = 0x8262F5E8;
	sub_82466E20(ctx, base);
	// 8262F5E8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8262F5EC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8262F5F0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8262F5F4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8262F5F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8262F5F8 size=108
    let mut pc: u32 = 0x8262F5F8;
    'dispatch: loop {
        match pc {
            0x8262F5F8 => {
    //   block [0x8262F5F8..0x8262F664)
	// 8262F5F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262F5FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8262F600: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8262F604: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8262F608: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8262F60C: 38EBCD50  addi r7, r11, -0x32b0
	ctx.r[7].s64 = ctx.r[11].s64 + -12976;
	// 8262F610: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 8262F614: 388A35CC  addi r4, r10, 0x35cc
	ctx.r[4].s64 = ctx.r[10].s64 + 13772;
	// 8262F618: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8262F61C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262F620: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8262F624: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8262F628: 386A6304  addi r3, r10, 0x6304
	ctx.r[3].s64 = ctx.r[10].s64 + 25348;
	// 8262F62C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8262F630: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8262F634: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8262F638: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8262F63C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8262F640: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8262F644: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8262F648: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8262F64C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8262F650: 4BE377D1  bl 0x82466e20
	ctx.lr = 0x8262F654;
	sub_82466E20(ctx, base);
	// 8262F654: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8262F658: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8262F65C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8262F660: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8262F668(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8262F668 size=108
    let mut pc: u32 = 0x8262F668;
    'dispatch: loop {
        match pc {
            0x8262F668 => {
    //   block [0x8262F668..0x8262F6D4)
	// 8262F668: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262F66C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8262F670: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8262F674: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8262F678: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8262F67C: 38EBCD98  addi r7, r11, -0x3268
	ctx.r[7].s64 = ctx.r[11].s64 + -12904;
	// 8262F680: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 8262F684: 388A3610  addi r4, r10, 0x3610
	ctx.r[4].s64 = ctx.r[10].s64 + 13840;
	// 8262F688: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8262F68C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262F690: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8262F694: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8262F698: 386A6334  addi r3, r10, 0x6334
	ctx.r[3].s64 = ctx.r[10].s64 + 25396;
	// 8262F69C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8262F6A0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8262F6A4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8262F6A8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8262F6AC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8262F6B0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8262F6B4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8262F6B8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8262F6BC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8262F6C0: 4BE37761  bl 0x82466e20
	ctx.lr = 0x8262F6C4;
	sub_82466E20(ctx, base);
	// 8262F6C4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8262F6C8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8262F6CC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8262F6D0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8262F6D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8262F6D8 size=116
    let mut pc: u32 = 0x8262F6D8;
    'dispatch: loop {
        match pc {
            0x8262F6D8 => {
    //   block [0x8262F6D8..0x8262F74C)
	// 8262F6D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262F6DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8262F6E0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8262F6E4: 3D408277  lis r10, -0x7d89
	ctx.r[10].s64 = -2106130432;
	// 8262F6E8: 38E0000E  li r7, 0xe
	ctx.r[7].s64 = 14;
	// 8262F6EC: 390ACDE0  addi r8, r10, -0x3220
	ctx.r[8].s64 = ctx.r[10].s64 + -12832;
	// 8262F6F0: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262F6F4: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 8262F6F8: 38AA5E54  addi r5, r10, 0x5e54
	ctx.r[5].s64 = ctx.r[10].s64 + 24148;
	// 8262F6FC: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8262F700: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8262F704: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8262F708: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8262F70C: 388A362C  addi r4, r10, 0x362c
	ctx.r[4].s64 = ctx.r[10].s64 + 13868;
	// 8262F710: 396B49B0  addi r11, r11, 0x49b0
	ctx.r[11].s64 = ctx.r[11].s64 + 18864;
	// 8262F714: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262F718: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8262F71C: 386A6364  addi r3, r10, 0x6364
	ctx.r[3].s64 = ctx.r[10].s64 + 25444;
	// 8262F720: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 8262F724: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8262F728: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 8262F72C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8262F730: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8262F734: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8262F738: 4BE376E9  bl 0x82466e20
	ctx.lr = 0x8262F73C;
	sub_82466E20(ctx, base);
	// 8262F73C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8262F740: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8262F744: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8262F748: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8262F750(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8262F750 size=104
    let mut pc: u32 = 0x8262F750;
    'dispatch: loop {
        match pc {
            0x8262F750 => {
    //   block [0x8262F750..0x8262F7B8)
	// 8262F750: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262F754: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8262F758: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8262F75C: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8262F760: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8262F764: 392A4A0C  addi r9, r10, 0x4a0c
	ctx.r[9].s64 = ctx.r[10].s64 + 18956;
	// 8262F768: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262F76C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8262F770: 38AA64E4  addi r5, r10, 0x64e4
	ctx.r[5].s64 = ctx.r[10].s64 + 25828;
	// 8262F774: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8262F778: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8262F77C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8262F780: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8262F784: 388A5A40  addi r4, r10, 0x5a40
	ctx.r[4].s64 = ctx.r[10].s64 + 23104;
	// 8262F788: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8262F78C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262F790: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8262F794: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8262F798: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8262F79C: 386A6394  addi r3, r10, 0x6394
	ctx.r[3].s64 = ctx.r[10].s64 + 25492;
	// 8262F7A0: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8262F7A4: 4BE3767D  bl 0x82466e20
	ctx.lr = 0x8262F7A8;
	sub_82466E20(ctx, base);
	// 8262F7A8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8262F7AC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8262F7B0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8262F7B4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8262F7B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8262F7B8 size=108
    let mut pc: u32 = 0x8262F7B8;
    'dispatch: loop {
        match pc {
            0x8262F7B8 => {
    //   block [0x8262F7B8..0x8262F824)
	// 8262F7B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262F7BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8262F7C0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8262F7C4: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8262F7C8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8262F7CC: 38EBCF30  addi r7, r11, -0x30d0
	ctx.r[7].s64 = ctx.r[11].s64 + -12496;
	// 8262F7D0: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 8262F7D4: 388A251C  addi r4, r10, 0x251c
	ctx.r[4].s64 = ctx.r[10].s64 + 9500;
	// 8262F7D8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8262F7DC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262F7E0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8262F7E4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8262F7E8: 386A63C4  addi r3, r10, 0x63c4
	ctx.r[3].s64 = ctx.r[10].s64 + 25540;
	// 8262F7EC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8262F7F0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8262F7F4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8262F7F8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8262F7FC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8262F800: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8262F804: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8262F808: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8262F80C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8262F810: 4BE37611  bl 0x82466e20
	ctx.lr = 0x8262F814;
	sub_82466E20(ctx, base);
	// 8262F814: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8262F818: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8262F81C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8262F820: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8262F828(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8262F828 size=24
    let mut pc: u32 = 0x8262F828;
    'dispatch: loop {
        match pc {
            0x8262F828 => {
    //   block [0x8262F828..0x8262F840)
	// 8262F828: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8262F82C: 3D408277  lis r10, -0x7d89
	ctx.r[10].s64 = -2106130432;
	// 8262F830: 394A1ED0  addi r10, r10, 0x1ed0
	ctx.r[10].s64 = ctx.r[10].s64 + 7888;
	// 8262F834: 816BCF78  lwz r11, -0x3088(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-12424 as u32) ) } as u64;
	// 8262F838: 916A0020  stw r11, 0x20(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 8262F83C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8262F840(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8262F840 size=116
    let mut pc: u32 = 0x8262F840;
    'dispatch: loop {
        match pc {
            0x8262F840 => {
    //   block [0x8262F840..0x8262F8B4)
	// 8262F840: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262F844: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8262F848: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8262F84C: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8262F850: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8262F854: 390B1ED0  addi r8, r11, 0x1ed0
	ctx.r[8].s64 = ctx.r[11].s64 + 7888;
	// 8262F858: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8262F85C: 392A4A48  addi r9, r10, 0x4a48
	ctx.r[9].s64 = ctx.r[10].s64 + 19016;
	// 8262F860: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262F864: 38E00006  li r7, 6
	ctx.r[7].s64 = 6;
	// 8262F868: 38AA6394  addi r5, r10, 0x6394
	ctx.r[5].s64 = ctx.r[10].s64 + 25492;
	// 8262F86C: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8262F870: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8262F874: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8262F878: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8262F87C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8262F880: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8262F884: 3D608294  lis r11, -0x7d6c
	ctx.r[11].s64 = -2104229888;
	// 8262F888: 388A5A5C  addi r4, r10, 0x5a5c
	ctx.r[4].s64 = ctx.r[10].s64 + 23132;
	// 8262F88C: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8262F890: 386B63F4  addi r3, r11, 0x63f4
	ctx.r[3].s64 = ctx.r[11].s64 + 25588;
	// 8262F894: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8262F898: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8262F89C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8262F8A0: 4BE37581  bl 0x82466e20
	ctx.lr = 0x8262F8A4;
	sub_82466E20(ctx, base);
	// 8262F8A4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8262F8A8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8262F8AC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8262F8B0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8262F8B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8262F8B8 size=108
    let mut pc: u32 = 0x8262F8B8;
    'dispatch: loop {
        match pc {
            0x8262F8B8 => {
    //   block [0x8262F8B8..0x8262F924)
	// 8262F8B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262F8BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8262F8C0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8262F8C4: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8262F8C8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8262F8CC: 38EBCF80  addi r7, r11, -0x3080
	ctx.r[7].s64 = ctx.r[11].s64 + -12416;
	// 8262F8D0: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 8262F8D4: 388A3770  addi r4, r10, 0x3770
	ctx.r[4].s64 = ctx.r[10].s64 + 14192;
	// 8262F8D8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8262F8DC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262F8E0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8262F8E4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8262F8E8: 386A6424  addi r3, r10, 0x6424
	ctx.r[3].s64 = ctx.r[10].s64 + 25636;
	// 8262F8EC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8262F8F0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8262F8F4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8262F8F8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8262F8FC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8262F900: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8262F904: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8262F908: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8262F90C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8262F910: 4BE37511  bl 0x82466e20
	ctx.lr = 0x8262F914;
	sub_82466E20(ctx, base);
	// 8262F914: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8262F918: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8262F91C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8262F920: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8262F928(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8262F928 size=108
    let mut pc: u32 = 0x8262F928;
    'dispatch: loop {
        match pc {
            0x8262F928 => {
    //   block [0x8262F928..0x8262F994)
	// 8262F928: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262F92C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8262F930: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8262F934: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8262F938: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8262F93C: 38EBCFC8  addi r7, r11, -0x3038
	ctx.r[7].s64 = ctx.r[11].s64 + -12344;
	// 8262F940: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 8262F944: 388A3788  addi r4, r10, 0x3788
	ctx.r[4].s64 = ctx.r[10].s64 + 14216;
	// 8262F948: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8262F94C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262F950: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8262F954: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8262F958: 386A6454  addi r3, r10, 0x6454
	ctx.r[3].s64 = ctx.r[10].s64 + 25684;
	// 8262F95C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8262F960: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8262F964: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8262F968: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8262F96C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8262F970: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8262F974: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8262F978: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8262F97C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8262F980: 4BE374A1  bl 0x82466e20
	ctx.lr = 0x8262F984;
	sub_82466E20(ctx, base);
	// 8262F984: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8262F988: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8262F98C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8262F990: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8262F998(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8262F998 size=108
    let mut pc: u32 = 0x8262F998;
    'dispatch: loop {
        match pc {
            0x8262F998 => {
    //   block [0x8262F998..0x8262FA04)
	// 8262F998: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262F99C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8262F9A0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8262F9A4: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8262F9A8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8262F9AC: 38EBCFF8  addi r7, r11, -0x3008
	ctx.r[7].s64 = ctx.r[11].s64 + -12296;
	// 8262F9B0: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 8262F9B4: 388A37A0  addi r4, r10, 0x37a0
	ctx.r[4].s64 = ctx.r[10].s64 + 14240;
	// 8262F9B8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8262F9BC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262F9C0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8262F9C4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8262F9C8: 386A6484  addi r3, r10, 0x6484
	ctx.r[3].s64 = ctx.r[10].s64 + 25732;
	// 8262F9CC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8262F9D0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8262F9D4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8262F9D8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8262F9DC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8262F9E0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8262F9E4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8262F9E8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8262F9EC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8262F9F0: 4BE37431  bl 0x82466e20
	ctx.lr = 0x8262F9F4;
	sub_82466E20(ctx, base);
	// 8262F9F4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8262F9F8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8262F9FC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8262FA00: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8262FA08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8262FA08 size=96
    let mut pc: u32 = 0x8262FA08;
    'dispatch: loop {
        match pc {
            0x8262FA08 => {
    //   block [0x8262FA08..0x8262FA68)
	// 8262FA08: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262FA0C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8262FA10: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8262FA14: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 8262FA18: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8262FA1C: 388A6DD4  addi r4, r10, 0x6dd4
	ctx.r[4].s64 = ctx.r[10].s64 + 28116;
	// 8262FA20: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262FA24: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8262FA28: 386A64B4  addi r3, r10, 0x64b4
	ctx.r[3].s64 = ctx.r[10].s64 + 25780;
	// 8262FA2C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8262FA30: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8262FA34: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 8262FA38: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8262FA3C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8262FA40: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8262FA44: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8262FA48: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8262FA4C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8262FA50: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8262FA54: 4BE373CD  bl 0x82466e20
	ctx.lr = 0x8262FA58;
	sub_82466E20(ctx, base);
	// 8262FA58: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8262FA5C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8262FA60: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8262FA64: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8262FA68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8262FA68 size=112
    let mut pc: u32 = 0x8262FA68;
    'dispatch: loop {
        match pc {
            0x8262FA68 => {
    //   block [0x8262FA68..0x8262FAD8)
	// 8262FA68: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262FA6C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8262FA70: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8262FA74: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262FA78: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8262FA7C: 38AA64B4  addi r5, r10, 0x64b4
	ctx.r[5].s64 = ctx.r[10].s64 + 25780;
	// 8262FA80: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 8262FA84: 390BD010  addi r8, r11, -0x2ff0
	ctx.r[8].s64 = ctx.r[11].s64 + -12272;
	// 8262FA88: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 8262FA8C: 388A7380  addi r4, r10, 0x7380
	ctx.r[4].s64 = ctx.r[10].s64 + 29568;
	// 8262FA90: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8262FA94: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262FA98: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8262FA9C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8262FAA0: 386A64E4  addi r3, r10, 0x64e4
	ctx.r[3].s64 = ctx.r[10].s64 + 25828;
	// 8262FAA4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8262FAA8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8262FAAC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8262FAB0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8262FAB4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8262FAB8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8262FABC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8262FAC0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8262FAC4: 4BE3735D  bl 0x82466e20
	ctx.lr = 0x8262FAC8;
	sub_82466E20(ctx, base);
	// 8262FAC8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8262FACC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8262FAD0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8262FAD4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8262FAD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8262FAD8 size=112
    let mut pc: u32 = 0x8262FAD8;
    'dispatch: loop {
        match pc {
            0x8262FAD8 => {
    //   block [0x8262FAD8..0x8262FB48)
	// 8262FAD8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262FADC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8262FAE0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8262FAE4: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8262FAE8: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8262FAEC: 392A4A64  addi r9, r10, 0x4a64
	ctx.r[9].s64 = ctx.r[10].s64 + 19044;
	// 8262FAF0: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 8262FAF4: 390BD040  addi r8, r11, -0x2fc0
	ctx.r[8].s64 = ctx.r[11].s64 + -12224;
	// 8262FAF8: 38E00007  li r7, 7
	ctx.r[7].s64 = 7;
	// 8262FAFC: 388A7468  addi r4, r10, 0x7468
	ctx.r[4].s64 = ctx.r[10].s64 + 29800;
	// 8262FB00: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8262FB04: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262FB08: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8262FB0C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8262FB10: 386A6514  addi r3, r10, 0x6514
	ctx.r[3].s64 = ctx.r[10].s64 + 25876;
	// 8262FB14: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8262FB18: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8262FB1C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8262FB20: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8262FB24: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8262FB28: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8262FB2C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8262FB30: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8262FB34: 4BE372ED  bl 0x82466e20
	ctx.lr = 0x8262FB38;
	sub_82466E20(ctx, base);
	// 8262FB38: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8262FB3C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8262FB40: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8262FB44: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8262FB48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8262FB48 size=108
    let mut pc: u32 = 0x8262FB48;
    'dispatch: loop {
        match pc {
            0x8262FB48 => {
    //   block [0x8262FB48..0x8262FBB4)
	// 8262FB48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262FB4C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8262FB50: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8262FB54: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8262FB58: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 8262FB5C: 38EBD0E8  addi r7, r11, -0x2f18
	ctx.r[7].s64 = ctx.r[11].s64 + -12056;
	// 8262FB60: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 8262FB64: 388A7888  addi r4, r10, 0x7888
	ctx.r[4].s64 = ctx.r[10].s64 + 30856;
	// 8262FB68: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8262FB6C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262FB70: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8262FB74: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8262FB78: 386A6544  addi r3, r10, 0x6544
	ctx.r[3].s64 = ctx.r[10].s64 + 25924;
	// 8262FB7C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8262FB80: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8262FB84: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8262FB88: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8262FB8C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8262FB90: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8262FB94: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8262FB98: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8262FB9C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8262FBA0: 4BE37281  bl 0x82466e20
	ctx.lr = 0x8262FBA4;
	sub_82466E20(ctx, base);
	// 8262FBA4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8262FBA8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8262FBAC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8262FBB0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8262FBB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8262FBB8 size=108
    let mut pc: u32 = 0x8262FBB8;
    'dispatch: loop {
        match pc {
            0x8262FBB8 => {
    //   block [0x8262FBB8..0x8262FC24)
	// 8262FBB8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262FBBC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8262FBC0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8262FBC4: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8262FBC8: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 8262FBCC: 38EBD118  addi r7, r11, -0x2ee8
	ctx.r[7].s64 = ctx.r[11].s64 + -12008;
	// 8262FBD0: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 8262FBD4: 388A7898  addi r4, r10, 0x7898
	ctx.r[4].s64 = ctx.r[10].s64 + 30872;
	// 8262FBD8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8262FBDC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262FBE0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8262FBE4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8262FBE8: 386A6574  addi r3, r10, 0x6574
	ctx.r[3].s64 = ctx.r[10].s64 + 25972;
	// 8262FBEC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8262FBF0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8262FBF4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8262FBF8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8262FBFC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8262FC00: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8262FC04: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8262FC08: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8262FC0C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8262FC10: 4BE37211  bl 0x82466e20
	ctx.lr = 0x8262FC14;
	sub_82466E20(ctx, base);
	// 8262FC14: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8262FC18: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8262FC1C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8262FC20: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8262FC28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8262FC28 size=28
    let mut pc: u32 = 0x8262FC28;
    'dispatch: loop {
        match pc {
            0x8262FC28 => {
    //   block [0x8262FC28..0x8262FC44)
	// 8262FC28: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8262FC2C: 3D408277  lis r10, -0x7d89
	ctx.r[10].s64 = -2106130432;
	// 8262FC30: 394A1F60  addi r10, r10, 0x1f60
	ctx.r[10].s64 = ctx.r[10].s64 + 8032;
	// 8262FC34: 816BD148  lwz r11, -0x2eb8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-11960 as u32) ) } as u64;
	// 8262FC38: 916A0050  stw r11, 0x50(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 8262FC3C: 916A0068  stw r11, 0x68(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(104 as u32), ctx.r[11].u32 ) };
	// 8262FC40: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8262FC48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8262FC48 size=112
    let mut pc: u32 = 0x8262FC48;
    'dispatch: loop {
        match pc {
            0x8262FC48 => {
    //   block [0x8262FC48..0x8262FCB8)
	// 8262FC48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262FC4C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8262FC50: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8262FC54: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8262FC58: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8262FC5C: 392A4BD8  addi r9, r10, 0x4bd8
	ctx.r[9].s64 = ctx.r[10].s64 + 19416;
	// 8262FC60: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 8262FC64: 390B1F60  addi r8, r11, 0x1f60
	ctx.r[8].s64 = ctx.r[11].s64 + 8032;
	// 8262FC68: 38E00008  li r7, 8
	ctx.r[7].s64 = 8;
	// 8262FC6C: 388A72F4  addi r4, r10, 0x72f4
	ctx.r[4].s64 = ctx.r[10].s64 + 29428;
	// 8262FC70: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8262FC74: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262FC78: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8262FC7C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8262FC80: 386A65A4  addi r3, r10, 0x65a4
	ctx.r[3].s64 = ctx.r[10].s64 + 26020;
	// 8262FC84: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8262FC88: 39400003  li r10, 3
	ctx.r[10].s64 = 3;
	// 8262FC8C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8262FC90: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8262FC94: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8262FC98: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8262FC9C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8262FCA0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8262FCA4: 4BE3717D  bl 0x82466e20
	ctx.lr = 0x8262FCA8;
	sub_82466E20(ctx, base);
	// 8262FCA8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8262FCAC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8262FCB0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8262FCB4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8262FCB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8262FCB8 size=108
    let mut pc: u32 = 0x8262FCB8;
    'dispatch: loop {
        match pc {
            0x8262FCB8 => {
    //   block [0x8262FCB8..0x8262FD24)
	// 8262FCB8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262FCBC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8262FCC0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8262FCC4: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8262FCC8: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 8262FCCC: 38EBD154  addi r7, r11, -0x2eac
	ctx.r[7].s64 = ctx.r[11].s64 + -11948;
	// 8262FCD0: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 8262FCD4: 388A7788  addi r4, r10, 0x7788
	ctx.r[4].s64 = ctx.r[10].s64 + 30600;
	// 8262FCD8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8262FCDC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262FCE0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8262FCE4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8262FCE8: 386A65D4  addi r3, r10, 0x65d4
	ctx.r[3].s64 = ctx.r[10].s64 + 26068;
	// 8262FCEC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8262FCF0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8262FCF4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8262FCF8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8262FCFC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8262FD00: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8262FD04: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8262FD08: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8262FD0C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8262FD10: 4BE37111  bl 0x82466e20
	ctx.lr = 0x8262FD14;
	sub_82466E20(ctx, base);
	// 8262FD14: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8262FD18: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8262FD1C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8262FD20: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8262FD28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8262FD28 size=108
    let mut pc: u32 = 0x8262FD28;
    'dispatch: loop {
        match pc {
            0x8262FD28 => {
    //   block [0x8262FD28..0x8262FD94)
	// 8262FD28: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262FD2C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8262FD30: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8262FD34: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8262FD38: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 8262FD3C: 38EBD184  addi r7, r11, -0x2e7c
	ctx.r[7].s64 = ctx.r[11].s64 + -11900;
	// 8262FD40: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 8262FD44: 388A77AC  addi r4, r10, 0x77ac
	ctx.r[4].s64 = ctx.r[10].s64 + 30636;
	// 8262FD48: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8262FD4C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262FD50: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8262FD54: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8262FD58: 386A6604  addi r3, r10, 0x6604
	ctx.r[3].s64 = ctx.r[10].s64 + 26116;
	// 8262FD5C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8262FD60: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8262FD64: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8262FD68: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8262FD6C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8262FD70: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8262FD74: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8262FD78: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8262FD7C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8262FD80: 4BE370A1  bl 0x82466e20
	ctx.lr = 0x8262FD84;
	sub_82466E20(ctx, base);
	// 8262FD84: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8262FD88: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8262FD8C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8262FD90: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8262FD98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8262FD98 size=24
    let mut pc: u32 = 0x8262FD98;
    'dispatch: loop {
        match pc {
            0x8262FD98 => {
    //   block [0x8262FD98..0x8262FDB0)
	// 8262FD98: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8262FD9C: 3D408277  lis r10, -0x7d89
	ctx.r[10].s64 = -2106130432;
	// 8262FDA0: 394A2020  addi r10, r10, 0x2020
	ctx.r[10].s64 = ctx.r[10].s64 + 8224;
	// 8262FDA4: 816BD19C  lwz r11, -0x2e64(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-11876 as u32) ) } as u64;
	// 8262FDA8: 916A0050  stw r11, 0x50(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 8262FDAC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8262FDB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8262FDB0 size=112
    let mut pc: u32 = 0x8262FDB0;
    'dispatch: loop {
        match pc {
            0x8262FDB0 => {
    //   block [0x8262FDB0..0x8262FE20)
	// 8262FDB0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262FDB4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8262FDB8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8262FDBC: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8262FDC0: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8262FDC4: 392A4C2C  addi r9, r10, 0x4c2c
	ctx.r[9].s64 = ctx.r[10].s64 + 19500;
	// 8262FDC8: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 8262FDCC: 390B2020  addi r8, r11, 0x2020
	ctx.r[8].s64 = ctx.r[11].s64 + 8224;
	// 8262FDD0: 38E00006  li r7, 6
	ctx.r[7].s64 = 6;
	// 8262FDD4: 388A77C8  addi r4, r10, 0x77c8
	ctx.r[4].s64 = ctx.r[10].s64 + 30664;
	// 8262FDD8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8262FDDC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262FDE0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8262FDE4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8262FDE8: 386A6634  addi r3, r10, 0x6634
	ctx.r[3].s64 = ctx.r[10].s64 + 26164;
	// 8262FDEC: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8262FDF0: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8262FDF4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8262FDF8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8262FDFC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8262FE00: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8262FE04: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8262FE08: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8262FE0C: 4BE37015  bl 0x82466e20
	ctx.lr = 0x8262FE10;
	sub_82466E20(ctx, base);
	// 8262FE10: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8262FE14: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8262FE18: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8262FE1C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8262FE20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8262FE20 size=108
    let mut pc: u32 = 0x8262FE20;
    'dispatch: loop {
        match pc {
            0x8262FE20 => {
    //   block [0x8262FE20..0x8262FE8C)
	// 8262FE20: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262FE24: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8262FE28: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8262FE2C: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8262FE30: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 8262FE34: 38EBD1A0  addi r7, r11, -0x2e60
	ctx.r[7].s64 = ctx.r[11].s64 + -11872;
	// 8262FE38: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 8262FE3C: 388A7BB0  addi r4, r10, 0x7bb0
	ctx.r[4].s64 = ctx.r[10].s64 + 31664;
	// 8262FE40: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8262FE44: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262FE48: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8262FE4C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8262FE50: 386A6664  addi r3, r10, 0x6664
	ctx.r[3].s64 = ctx.r[10].s64 + 26212;
	// 8262FE54: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8262FE58: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8262FE5C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8262FE60: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8262FE64: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8262FE68: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8262FE6C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8262FE70: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8262FE74: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8262FE78: 4BE36FA9  bl 0x82466e20
	ctx.lr = 0x8262FE7C;
	sub_82466E20(ctx, base);
	// 8262FE7C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8262FE80: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8262FE84: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8262FE88: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8262FE90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8262FE90 size=108
    let mut pc: u32 = 0x8262FE90;
    'dispatch: loop {
        match pc {
            0x8262FE90 => {
    //   block [0x8262FE90..0x8262FEFC)
	// 8262FE90: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262FE94: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8262FE98: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8262FE9C: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8262FEA0: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 8262FEA4: 38EBD1D0  addi r7, r11, -0x2e30
	ctx.r[7].s64 = ctx.r[11].s64 + -11824;
	// 8262FEA8: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 8262FEAC: 388A7F88  addi r4, r10, 0x7f88
	ctx.r[4].s64 = ctx.r[10].s64 + 32648;
	// 8262FEB0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8262FEB4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262FEB8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8262FEBC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8262FEC0: 386A6694  addi r3, r10, 0x6694
	ctx.r[3].s64 = ctx.r[10].s64 + 26260;
	// 8262FEC4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8262FEC8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8262FECC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8262FED0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8262FED4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8262FED8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8262FEDC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8262FEE0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8262FEE4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8262FEE8: 4BE36F39  bl 0x82466e20
	ctx.lr = 0x8262FEEC;
	sub_82466E20(ctx, base);
	// 8262FEEC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8262FEF0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8262FEF4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8262FEF8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8262FF00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8262FF00 size=112
    let mut pc: u32 = 0x8262FF00;
    'dispatch: loop {
        match pc {
            0x8262FF00 => {
    //   block [0x8262FF00..0x8262FF70)
	// 8262FF00: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262FF04: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8262FF08: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8262FF0C: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8262FF10: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8262FF14: 392A4C50  addi r9, r10, 0x4c50
	ctx.r[9].s64 = ctx.r[10].s64 + 19536;
	// 8262FF18: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8262FF1C: 390BD208  addi r8, r11, -0x2df8
	ctx.r[8].s64 = ctx.r[11].s64 + -11768;
	// 8262FF20: 38E00004  li r7, 4
	ctx.r[7].s64 = 4;
	// 8262FF24: 388A82E8  addi r4, r10, -0x7d18
	ctx.r[4].s64 = ctx.r[10].s64 + -32024;
	// 8262FF28: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8262FF2C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262FF30: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8262FF34: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8262FF38: 386A66C4  addi r3, r10, 0x66c4
	ctx.r[3].s64 = ctx.r[10].s64 + 26308;
	// 8262FF3C: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8262FF40: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8262FF44: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8262FF48: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8262FF4C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8262FF50: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8262FF54: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8262FF58: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8262FF5C: 4BE36EC5  bl 0x82466e20
	ctx.lr = 0x8262FF60;
	sub_82466E20(ctx, base);
	// 8262FF60: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8262FF64: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8262FF68: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8262FF6C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8262FF70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8262FF70 size=108
    let mut pc: u32 = 0x8262FF70;
    'dispatch: loop {
        match pc {
            0x8262FF70 => {
    //   block [0x8262FF70..0x8262FFDC)
	// 8262FF70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262FF74: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8262FF78: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8262FF7C: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8262FF80: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 8262FF84: 38EBD268  addi r7, r11, -0x2d98
	ctx.r[7].s64 = ctx.r[11].s64 + -11672;
	// 8262FF88: 39000008  li r8, 8
	ctx.r[8].s64 = 8;
	// 8262FF8C: 388A7F28  addi r4, r10, 0x7f28
	ctx.r[4].s64 = ctx.r[10].s64 + 32552;
	// 8262FF90: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8262FF94: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262FF98: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8262FF9C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8262FFA0: 386A66F4  addi r3, r10, 0x66f4
	ctx.r[3].s64 = ctx.r[10].s64 + 26356;
	// 8262FFA4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8262FFA8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8262FFAC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8262FFB0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8262FFB4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8262FFB8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8262FFBC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8262FFC0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8262FFC4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8262FFC8: 4BE36E59  bl 0x82466e20
	ctx.lr = 0x8262FFCC;
	sub_82466E20(ctx, base);
	// 8262FFCC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8262FFD0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8262FFD4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8262FFD8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8262FFE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8262FFE0 size=108
    let mut pc: u32 = 0x8262FFE0;
    'dispatch: loop {
        match pc {
            0x8262FFE0 => {
    //   block [0x8262FFE0..0x8263004C)
	// 8262FFE0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262FFE4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8262FFE8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8262FFEC: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8262FFF0: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 8262FFF4: 38EBD328  addi r7, r11, -0x2cd8
	ctx.r[7].s64 = ctx.r[11].s64 + -11480;
	// 8262FFF8: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 8262FFFC: 388A78C0  addi r4, r10, 0x78c0
	ctx.r[4].s64 = ctx.r[10].s64 + 30912;
	// 82630000: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82630004: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82630008: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8263000C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82630010: 386A6724  addi r3, r10, 0x6724
	ctx.r[3].s64 = ctx.r[10].s64 + 26404;
	// 82630014: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82630018: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8263001C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82630020: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82630024: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82630028: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8263002C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82630030: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82630034: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82630038: 4BE36DE9  bl 0x82466e20
	ctx.lr = 0x8263003C;
	sub_82466E20(ctx, base);
	// 8263003C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82630040: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82630044: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82630048: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82630050(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82630050 size=108
    let mut pc: u32 = 0x82630050;
    'dispatch: loop {
        match pc {
            0x82630050 => {
    //   block [0x82630050..0x826300BC)
	// 82630050: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82630054: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82630058: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263005C: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 82630060: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82630064: 38EBD340  addi r7, r11, -0x2cc0
	ctx.r[7].s64 = ctx.r[11].s64 + -11456;
	// 82630068: 39000005  li r8, 5
	ctx.r[8].s64 = 5;
	// 8263006C: 388A8060  addi r4, r10, -0x7fa0
	ctx.r[4].s64 = ctx.r[10].s64 + -32672;
	// 82630070: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82630074: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82630078: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8263007C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82630080: 386A6754  addi r3, r10, 0x6754
	ctx.r[3].s64 = ctx.r[10].s64 + 26452;
	// 82630084: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82630088: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8263008C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82630090: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82630094: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82630098: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8263009C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826300A0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826300A4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826300A8: 4BE36D79  bl 0x82466e20
	ctx.lr = 0x826300AC;
	sub_82466E20(ctx, base);
	// 826300AC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826300B0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826300B4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826300B8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826300C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826300C0 size=24
    let mut pc: u32 = 0x826300C0;
    'dispatch: loop {
        match pc {
            0x826300C0 => {
    //   block [0x826300C0..0x826300D8)
	// 826300C0: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 826300C4: 3D408277  lis r10, -0x7d89
	ctx.r[10].s64 = -2106130432;
	// 826300C8: 394A20B0  addi r10, r10, 0x20b0
	ctx.r[10].s64 = ctx.r[10].s64 + 8368;
	// 826300CC: 816BD204  lwz r11, -0x2dfc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-11772 as u32) ) } as u64;
	// 826300D0: 916A0020  stw r11, 0x20(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 826300D4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826300D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826300D8 size=108
    let mut pc: u32 = 0x826300D8;
    'dispatch: loop {
        match pc {
            0x826300D8 => {
    //   block [0x826300D8..0x82630144)
	// 826300D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826300DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826300E0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826300E4: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 826300E8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826300EC: 38EB20B0  addi r7, r11, 0x20b0
	ctx.r[7].s64 = ctx.r[11].s64 + 8368;
	// 826300F0: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826300F4: 388AB020  addi r4, r10, -0x4fe0
	ctx.r[4].s64 = ctx.r[10].s64 + -20448;
	// 826300F8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826300FC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82630100: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82630104: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82630108: 386A6784  addi r3, r10, 0x6784
	ctx.r[3].s64 = ctx.r[10].s64 + 26500;
	// 8263010C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82630110: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82630114: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82630118: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8263011C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82630120: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82630124: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82630128: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8263012C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82630130: 4BE36CF1  bl 0x82466e20
	ctx.lr = 0x82630134;
	sub_82466E20(ctx, base);
	// 82630134: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82630138: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8263013C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82630140: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82630148(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82630148 size=24
    let mut pc: u32 = 0x82630148;
    'dispatch: loop {
        match pc {
            0x82630148 => {
    //   block [0x82630148..0x82630160)
	// 82630148: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8263014C: 3D408277  lis r10, -0x7d89
	ctx.r[10].s64 = -2106130432;
	// 82630150: 394A20E0  addi r10, r10, 0x20e0
	ctx.r[10].s64 = ctx.r[10].s64 + 8416;
	// 82630154: 816BD204  lwz r11, -0x2dfc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-11772 as u32) ) } as u64;
	// 82630158: 916A0020  stw r11, 0x20(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 8263015C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82630160(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82630160 size=108
    let mut pc: u32 = 0x82630160;
    'dispatch: loop {
        match pc {
            0x82630160 => {
    //   block [0x82630160..0x826301CC)
	// 82630160: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82630164: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82630168: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263016C: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 82630170: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82630174: 38EB20E0  addi r7, r11, 0x20e0
	ctx.r[7].s64 = ctx.r[11].s64 + 8416;
	// 82630178: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 8263017C: 388A9C48  addi r4, r10, -0x63b8
	ctx.r[4].s64 = ctx.r[10].s64 + -25528;
	// 82630180: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82630184: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82630188: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8263018C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82630190: 386A67B4  addi r3, r10, 0x67b4
	ctx.r[3].s64 = ctx.r[10].s64 + 26548;
	// 82630194: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82630198: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8263019C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826301A0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826301A4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826301A8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826301AC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826301B0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826301B4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826301B8: 4BE36C69  bl 0x82466e20
	ctx.lr = 0x826301BC;
	sub_82466E20(ctx, base);
	// 826301BC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826301C0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826301C4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826301C8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826301D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826301D0 size=108
    let mut pc: u32 = 0x826301D0;
    'dispatch: loop {
        match pc {
            0x826301D0 => {
    //   block [0x826301D0..0x8263023C)
	// 826301D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826301D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826301D8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826301DC: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 826301E0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826301E4: 38EBD3B8  addi r7, r11, -0x2c48
	ctx.r[7].s64 = ctx.r[11].s64 + -11336;
	// 826301E8: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826301EC: 388AA080  addi r4, r10, -0x5f80
	ctx.r[4].s64 = ctx.r[10].s64 + -24448;
	// 826301F0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826301F4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826301F8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826301FC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82630200: 386A67E4  addi r3, r10, 0x67e4
	ctx.r[3].s64 = ctx.r[10].s64 + 26596;
	// 82630204: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82630208: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8263020C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82630210: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82630214: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82630218: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8263021C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82630220: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82630224: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82630228: 4BE36BF9  bl 0x82466e20
	ctx.lr = 0x8263022C;
	sub_82466E20(ctx, base);
	// 8263022C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82630230: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82630234: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82630238: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82630240(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82630240 size=24
    let mut pc: u32 = 0x82630240;
    'dispatch: loop {
        match pc {
            0x82630240 => {
    //   block [0x82630240..0x82630258)
	// 82630240: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 82630244: 3D408277  lis r10, -0x7d89
	ctx.r[10].s64 = -2106130432;
	// 82630248: 394A2110  addi r10, r10, 0x2110
	ctx.r[10].s64 = ctx.r[10].s64 + 8464;
	// 8263024C: 816BD204  lwz r11, -0x2dfc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-11772 as u32) ) } as u64;
	// 82630250: 916A0020  stw r11, 0x20(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 82630254: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82630258(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82630258 size=108
    let mut pc: u32 = 0x82630258;
    'dispatch: loop {
        match pc {
            0x82630258 => {
    //   block [0x82630258..0x826302C4)
	// 82630258: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263025C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82630260: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82630264: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 82630268: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8263026C: 38EB2110  addi r7, r11, 0x2110
	ctx.r[7].s64 = ctx.r[11].s64 + 8464;
	// 82630270: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 82630274: 388A9BE8  addi r4, r10, -0x6418
	ctx.r[4].s64 = ctx.r[10].s64 + -25624;
	// 82630278: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8263027C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82630280: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82630284: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82630288: 386A6814  addi r3, r10, 0x6814
	ctx.r[3].s64 = ctx.r[10].s64 + 26644;
	// 8263028C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82630290: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82630294: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82630298: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8263029C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826302A0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826302A4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826302A8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826302AC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826302B0: 4BE36B71  bl 0x82466e20
	ctx.lr = 0x826302B4;
	sub_82466E20(ctx, base);
	// 826302B4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826302B8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826302BC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826302C0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826302C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826302C8 size=112
    let mut pc: u32 = 0x826302C8;
    'dispatch: loop {
        match pc {
            0x826302C8 => {
    //   block [0x826302C8..0x82630338)
	// 826302C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826302CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826302D0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826302D4: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826302D8: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 826302DC: 392A4C94  addi r9, r10, 0x4c94
	ctx.r[9].s64 = ctx.r[10].s64 + 19604;
	// 826302E0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826302E4: 390BD3D0  addi r8, r11, -0x2c30
	ctx.r[8].s64 = ctx.r[11].s64 + -11312;
	// 826302E8: 38E00002  li r7, 2
	ctx.r[7].s64 = 2;
	// 826302EC: 388AA778  addi r4, r10, -0x5888
	ctx.r[4].s64 = ctx.r[10].s64 + -22664;
	// 826302F0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826302F4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826302F8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826302FC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82630300: 386A6844  addi r3, r10, 0x6844
	ctx.r[3].s64 = ctx.r[10].s64 + 26692;
	// 82630304: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82630308: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8263030C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82630310: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82630314: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82630318: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8263031C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82630320: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82630324: 4BE36AFD  bl 0x82466e20
	ctx.lr = 0x82630328;
	sub_82466E20(ctx, base);
	// 82630328: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8263032C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82630330: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82630334: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82630338(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82630338 size=108
    let mut pc: u32 = 0x82630338;
    'dispatch: loop {
        match pc {
            0x82630338 => {
    //   block [0x82630338..0x826303A4)
	// 82630338: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263033C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82630340: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82630344: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 82630348: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8263034C: 38EBD400  addi r7, r11, -0x2c00
	ctx.r[7].s64 = ctx.r[11].s64 + -11264;
	// 82630350: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 82630354: 388AA350  addi r4, r10, -0x5cb0
	ctx.r[4].s64 = ctx.r[10].s64 + -23728;
	// 82630358: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8263035C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82630360: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82630364: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82630368: 386A6874  addi r3, r10, 0x6874
	ctx.r[3].s64 = ctx.r[10].s64 + 26740;
	// 8263036C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82630370: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82630374: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82630378: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8263037C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82630380: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82630384: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82630388: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8263038C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82630390: 4BE36A91  bl 0x82466e20
	ctx.lr = 0x82630394;
	sub_82466E20(ctx, base);
	// 82630394: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82630398: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8263039C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826303A0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826303A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826303A8 size=108
    let mut pc: u32 = 0x826303A8;
    'dispatch: loop {
        match pc {
            0x826303A8 => {
    //   block [0x826303A8..0x82630414)
	// 826303A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826303AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826303B0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826303B4: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 826303B8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826303BC: 38EBD430  addi r7, r11, -0x2bd0
	ctx.r[7].s64 = ctx.r[11].s64 + -11216;
	// 826303C0: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826303C4: 388AAA44  addi r4, r10, -0x55bc
	ctx.r[4].s64 = ctx.r[10].s64 + -21948;
	// 826303C8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826303CC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826303D0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826303D4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826303D8: 386A68A4  addi r3, r10, 0x68a4
	ctx.r[3].s64 = ctx.r[10].s64 + 26788;
	// 826303DC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826303E0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826303E4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826303E8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826303EC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826303F0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826303F4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826303F8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826303FC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82630400: 4BE36A21  bl 0x82466e20
	ctx.lr = 0x82630404;
	sub_82466E20(ctx, base);
	// 82630404: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82630408: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8263040C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82630410: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82630418(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82630418 size=112
    let mut pc: u32 = 0x82630418;
    'dispatch: loop {
        match pc {
            0x82630418 => {
    //   block [0x82630418..0x82630488)
	// 82630418: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263041C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82630420: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82630424: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82630428: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8263042C: 38AA6904  addi r5, r10, 0x6904
	ctx.r[5].s64 = ctx.r[10].s64 + 26884;
	// 82630430: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82630434: 390BD460  addi r8, r11, -0x2ba0
	ctx.r[8].s64 = ctx.r[11].s64 + -11168;
	// 82630438: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8263043C: 388AA9A0  addi r4, r10, -0x5660
	ctx.r[4].s64 = ctx.r[10].s64 + -22112;
	// 82630440: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82630444: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82630448: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8263044C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82630450: 386A68D4  addi r3, r10, 0x68d4
	ctx.r[3].s64 = ctx.r[10].s64 + 26836;
	// 82630454: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82630458: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8263045C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82630460: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82630464: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82630468: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8263046C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82630470: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82630474: 4BE369AD  bl 0x82466e20
	ctx.lr = 0x82630478;
	sub_82466E20(ctx, base);
	// 82630478: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8263047C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82630480: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82630484: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82630488(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82630488 size=108
    let mut pc: u32 = 0x82630488;
    'dispatch: loop {
        match pc {
            0x82630488 => {
    //   block [0x82630488..0x826304F4)
	// 82630488: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263048C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82630490: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82630494: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 82630498: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8263049C: 38EBD478  addi r7, r11, -0x2b88
	ctx.r[7].s64 = ctx.r[11].s64 + -11144;
	// 826304A0: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826304A4: 388AA640  addi r4, r10, -0x59c0
	ctx.r[4].s64 = ctx.r[10].s64 + -22976;
	// 826304A8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826304AC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826304B0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826304B4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826304B8: 386A6904  addi r3, r10, 0x6904
	ctx.r[3].s64 = ctx.r[10].s64 + 26884;
	// 826304BC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826304C0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826304C4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826304C8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826304CC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826304D0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826304D4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826304D8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826304DC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826304E0: 4BE36941  bl 0x82466e20
	ctx.lr = 0x826304E4;
	sub_82466E20(ctx, base);
	// 826304E4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826304E8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826304EC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826304F0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826304F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826304F8 size=108
    let mut pc: u32 = 0x826304F8;
    'dispatch: loop {
        match pc {
            0x826304F8 => {
    //   block [0x826304F8..0x82630564)
	// 826304F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826304FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82630500: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82630504: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 82630508: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8263050C: 38EBD4A8  addi r7, r11, -0x2b58
	ctx.r[7].s64 = ctx.r[11].s64 + -11096;
	// 82630510: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 82630514: 388AA2E0  addi r4, r10, -0x5d20
	ctx.r[4].s64 = ctx.r[10].s64 + -23840;
	// 82630518: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8263051C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82630520: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82630524: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82630528: 386A6934  addi r3, r10, 0x6934
	ctx.r[3].s64 = ctx.r[10].s64 + 26932;
	// 8263052C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82630530: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82630534: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82630538: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8263053C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82630540: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82630544: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82630548: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8263054C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82630550: 4BE368D1  bl 0x82466e20
	ctx.lr = 0x82630554;
	sub_82466E20(ctx, base);
	// 82630554: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82630558: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8263055C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82630560: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82630568(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82630568 size=108
    let mut pc: u32 = 0x82630568;
    'dispatch: loop {
        match pc {
            0x82630568 => {
    //   block [0x82630568..0x826305D4)
	// 82630568: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263056C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82630570: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82630574: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 82630578: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8263057C: 38EBD4C0  addi r7, r11, -0x2b40
	ctx.r[7].s64 = ctx.r[11].s64 + -11072;
	// 82630580: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 82630584: 388AA304  addi r4, r10, -0x5cfc
	ctx.r[4].s64 = ctx.r[10].s64 + -23804;
	// 82630588: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8263058C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82630590: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82630594: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82630598: 386A6964  addi r3, r10, 0x6964
	ctx.r[3].s64 = ctx.r[10].s64 + 26980;
	// 8263059C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826305A0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826305A4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826305A8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826305AC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826305B0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826305B4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826305B8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826305BC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826305C0: 4BE36861  bl 0x82466e20
	ctx.lr = 0x826305C4;
	sub_82466E20(ctx, base);
	// 826305C4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826305C8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826305CC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826305D0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826305D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826305D8 size=108
    let mut pc: u32 = 0x826305D8;
    'dispatch: loop {
        match pc {
            0x826305D8 => {
    //   block [0x826305D8..0x82630644)
	// 826305D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826305DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826305E0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826305E4: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 826305E8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826305EC: 38EBD4F0  addi r7, r11, -0x2b10
	ctx.r[7].s64 = ctx.r[11].s64 + -11024;
	// 826305F0: 39000007  li r8, 7
	ctx.r[8].s64 = 7;
	// 826305F4: 388AAED0  addi r4, r10, -0x5130
	ctx.r[4].s64 = ctx.r[10].s64 + -20784;
	// 826305F8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826305FC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82630600: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82630604: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82630608: 386A6994  addi r3, r10, 0x6994
	ctx.r[3].s64 = ctx.r[10].s64 + 27028;
	// 8263060C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82630610: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82630614: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82630618: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8263061C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82630620: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82630624: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82630628: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8263062C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82630630: 4BE367F1  bl 0x82466e20
	ctx.lr = 0x82630634;
	sub_82466E20(ctx, base);
	// 82630634: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82630638: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8263063C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82630640: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82630648(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82630648 size=108
    let mut pc: u32 = 0x82630648;
    'dispatch: loop {
        match pc {
            0x82630648 => {
    //   block [0x82630648..0x826306B4)
	// 82630648: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263064C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82630650: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82630654: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 82630658: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8263065C: 38EBD598  addi r7, r11, -0x2a68
	ctx.r[7].s64 = ctx.r[11].s64 + -10856;
	// 82630660: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 82630664: 388AA130  addi r4, r10, -0x5ed0
	ctx.r[4].s64 = ctx.r[10].s64 + -24272;
	// 82630668: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8263066C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82630670: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82630674: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82630678: 386A69C4  addi r3, r10, 0x69c4
	ctx.r[3].s64 = ctx.r[10].s64 + 27076;
	// 8263067C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82630680: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82630684: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82630688: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8263068C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82630690: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82630694: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82630698: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8263069C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826306A0: 4BE36781  bl 0x82466e20
	ctx.lr = 0x826306A4;
	sub_82466E20(ctx, base);
	// 826306A4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826306A8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826306AC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826306B0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826306B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826306B8 size=108
    let mut pc: u32 = 0x826306B8;
    'dispatch: loop {
        match pc {
            0x826306B8 => {
    //   block [0x826306B8..0x82630724)
	// 826306B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826306BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826306C0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826306C4: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 826306C8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826306CC: 38EBD5C8  addi r7, r11, -0x2a38
	ctx.r[7].s64 = ctx.r[11].s64 + -10808;
	// 826306D0: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826306D4: 388AA148  addi r4, r10, -0x5eb8
	ctx.r[4].s64 = ctx.r[10].s64 + -24248;
	// 826306D8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826306DC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826306E0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826306E4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826306E8: 386A69F4  addi r3, r10, 0x69f4
	ctx.r[3].s64 = ctx.r[10].s64 + 27124;
	// 826306EC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826306F0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826306F4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826306F8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826306FC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82630700: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82630704: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82630708: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8263070C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82630710: 4BE36711  bl 0x82466e20
	ctx.lr = 0x82630714;
	sub_82466E20(ctx, base);
	// 82630714: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82630718: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8263071C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82630720: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82630728(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82630728 size=108
    let mut pc: u32 = 0x82630728;
    'dispatch: loop {
        match pc {
            0x82630728 => {
    //   block [0x82630728..0x82630794)
	// 82630728: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263072C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82630730: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82630734: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 82630738: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8263073C: 38EBD5E0  addi r7, r11, -0x2a20
	ctx.r[7].s64 = ctx.r[11].s64 + -10784;
	// 82630740: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 82630744: 388AB290  addi r4, r10, -0x4d70
	ctx.r[4].s64 = ctx.r[10].s64 + -19824;
	// 82630748: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8263074C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82630750: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82630754: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82630758: 386A6A24  addi r3, r10, 0x6a24
	ctx.r[3].s64 = ctx.r[10].s64 + 27172;
	// 8263075C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82630760: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82630764: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82630768: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8263076C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82630770: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82630774: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82630778: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8263077C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82630780: 4BE366A1  bl 0x82466e20
	ctx.lr = 0x82630784;
	sub_82466E20(ctx, base);
	// 82630784: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82630788: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8263078C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82630790: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82630798(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82630798 size=108
    let mut pc: u32 = 0x82630798;
    'dispatch: loop {
        match pc {
            0x82630798 => {
    //   block [0x82630798..0x82630804)
	// 82630798: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263079C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826307A0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826307A4: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 826307A8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826307AC: 38EBD610  addi r7, r11, -0x29f0
	ctx.r[7].s64 = ctx.r[11].s64 + -10736;
	// 826307B0: 39000008  li r8, 8
	ctx.r[8].s64 = 8;
	// 826307B4: 388AB2A8  addi r4, r10, -0x4d58
	ctx.r[4].s64 = ctx.r[10].s64 + -19800;
	// 826307B8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826307BC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826307C0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826307C4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826307C8: 386A6A54  addi r3, r10, 0x6a54
	ctx.r[3].s64 = ctx.r[10].s64 + 27220;
	// 826307CC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826307D0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826307D4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826307D8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826307DC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826307E0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826307E4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826307E8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826307EC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826307F0: 4BE36631  bl 0x82466e20
	ctx.lr = 0x826307F4;
	sub_82466E20(ctx, base);
	// 826307F4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826307F8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826307FC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82630800: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82630808(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82630808 size=24
    let mut pc: u32 = 0x82630808;
    'dispatch: loop {
        match pc {
            0x82630808 => {
    //   block [0x82630808..0x82630820)
	// 82630808: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8263080C: 3D408277  lis r10, -0x7d89
	ctx.r[10].s64 = -2106130432;
	// 82630810: 394A2140  addi r10, r10, 0x2140
	ctx.r[10].s64 = ctx.r[10].s64 + 8512;
	// 82630814: 816BD6D0  lwz r11, -0x2930(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-10544 as u32) ) } as u64;
	// 82630818: 916A0008  stw r11, 8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 8263081C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82630820(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82630820 size=112
    let mut pc: u32 = 0x82630820;
    'dispatch: loop {
        match pc {
            0x82630820 => {
    //   block [0x82630820..0x82630890)
	// 82630820: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82630824: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82630828: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263082C: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82630830: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 82630834: 392A4CC0  addi r9, r10, 0x4cc0
	ctx.r[9].s64 = ctx.r[10].s64 + 19648;
	// 82630838: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8263083C: 390B2140  addi r8, r11, 0x2140
	ctx.r[8].s64 = ctx.r[11].s64 + 8512;
	// 82630840: 38E00005  li r7, 5
	ctx.r[7].s64 = 5;
	// 82630844: 388AB098  addi r4, r10, -0x4f68
	ctx.r[4].s64 = ctx.r[10].s64 + -20328;
	// 82630848: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8263084C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82630850: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82630854: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82630858: 386A6A84  addi r3, r10, 0x6a84
	ctx.r[3].s64 = ctx.r[10].s64 + 27268;
	// 8263085C: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82630860: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82630864: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82630868: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8263086C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82630870: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82630874: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82630878: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8263087C: 4BE365A5  bl 0x82466e20
	ctx.lr = 0x82630880;
	sub_82466E20(ctx, base);
	// 82630880: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82630884: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82630888: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8263088C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82630890(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82630890 size=108
    let mut pc: u32 = 0x82630890;
    'dispatch: loop {
        match pc {
            0x82630890 => {
    //   block [0x82630890..0x826308FC)
	// 82630890: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82630894: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82630898: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263089C: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 826308A0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826308A4: 38EBD6D8  addi r7, r11, -0x2928
	ctx.r[7].s64 = ctx.r[11].s64 + -10536;
	// 826308A8: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826308AC: 388AAC70  addi r4, r10, -0x5390
	ctx.r[4].s64 = ctx.r[10].s64 + -21392;
	// 826308B0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826308B4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826308B8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826308BC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826308C0: 386A6AB4  addi r3, r10, 0x6ab4
	ctx.r[3].s64 = ctx.r[10].s64 + 27316;
	// 826308C4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826308C8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826308CC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826308D0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826308D4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826308D8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826308DC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826308E0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826308E4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826308E8: 4BE36539  bl 0x82466e20
	ctx.lr = 0x826308EC;
	sub_82466E20(ctx, base);
	// 826308EC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826308F0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826308F4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826308F8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82630900(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82630900 size=112
    let mut pc: u32 = 0x82630900;
    'dispatch: loop {
        match pc {
            0x82630900 => {
    //   block [0x82630900..0x82630970)
	// 82630900: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82630904: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82630908: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263090C: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82630910: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 82630914: 392A4D04  addi r9, r10, 0x4d04
	ctx.r[9].s64 = ctx.r[10].s64 + 19716;
	// 82630918: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8263091C: 390BD708  addi r8, r11, -0x28f8
	ctx.r[8].s64 = ctx.r[11].s64 + -10488;
	// 82630920: 38E00008  li r7, 8
	ctx.r[7].s64 = 8;
	// 82630924: 388AAC88  addi r4, r10, -0x5378
	ctx.r[4].s64 = ctx.r[10].s64 + -21368;
	// 82630928: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8263092C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82630930: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82630934: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82630938: 386A6AE4  addi r3, r10, 0x6ae4
	ctx.r[3].s64 = ctx.r[10].s64 + 27364;
	// 8263093C: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82630940: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82630944: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82630948: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8263094C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82630950: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82630954: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82630958: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8263095C: 4BE364C5  bl 0x82466e20
	ctx.lr = 0x82630960;
	sub_82466E20(ctx, base);
	// 82630960: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82630964: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82630968: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8263096C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82630970(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82630970 size=24
    let mut pc: u32 = 0x82630970;
    'dispatch: loop {
        match pc {
            0x82630970 => {
    //   block [0x82630970..0x82630988)
	// 82630970: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 82630974: 3D408277  lis r10, -0x7d89
	ctx.r[10].s64 = -2106130432;
	// 82630978: 394A21B8  addi r10, r10, 0x21b8
	ctx.r[10].s64 = ctx.r[10].s64 + 8632;
	// 8263097C: 816BD7C8  lwz r11, -0x2838(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-10296 as u32) ) } as u64;
	// 82630980: 916A0020  stw r11, 0x20(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 82630984: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82630988(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82630988 size=112
    let mut pc: u32 = 0x82630988;
    'dispatch: loop {
        match pc {
            0x82630988 => {
    //   block [0x82630988..0x826309F8)
	// 82630988: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263098C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82630990: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82630994: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82630998: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8263099C: 392A4D40  addi r9, r10, 0x4d40
	ctx.r[9].s64 = ctx.r[10].s64 + 19776;
	// 826309A0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826309A4: 390B21B8  addi r8, r11, 0x21b8
	ctx.r[8].s64 = ctx.r[11].s64 + 8632;
	// 826309A8: 38E00003  li r7, 3
	ctx.r[7].s64 = 3;
	// 826309AC: 388AB004  addi r4, r10, -0x4ffc
	ctx.r[4].s64 = ctx.r[10].s64 + -20476;
	// 826309B0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826309B4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826309B8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826309BC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826309C0: 386A6B14  addi r3, r10, 0x6b14
	ctx.r[3].s64 = ctx.r[10].s64 + 27412;
	// 826309C4: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826309C8: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826309CC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826309D0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826309D4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826309D8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826309DC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826309E0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826309E4: 4BE3643D  bl 0x82466e20
	ctx.lr = 0x826309E8;
	sub_82466E20(ctx, base);
	// 826309E8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826309EC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826309F0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826309F4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826309F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826309F8 size=108
    let mut pc: u32 = 0x826309F8;
    'dispatch: loop {
        match pc {
            0x826309F8 => {
    //   block [0x826309F8..0x82630A64)
	// 826309F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826309FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82630A00: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82630A04: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 82630A08: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82630A0C: 38EBD7CC  addi r7, r11, -0x2834
	ctx.r[7].s64 = ctx.r[11].s64 + -10292;
	// 82630A10: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 82630A14: 388AAF00  addi r4, r10, -0x5100
	ctx.r[4].s64 = ctx.r[10].s64 + -20736;
	// 82630A18: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82630A1C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82630A20: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82630A24: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82630A28: 386A6B44  addi r3, r10, 0x6b44
	ctx.r[3].s64 = ctx.r[10].s64 + 27460;
	// 82630A2C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82630A30: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82630A34: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82630A38: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82630A3C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82630A40: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82630A44: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82630A48: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82630A4C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82630A50: 4BE363D1  bl 0x82466e20
	ctx.lr = 0x82630A54;
	sub_82466E20(ctx, base);
	// 82630A54: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82630A58: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82630A5C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82630A60: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82630A68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82630A68 size=108
    let mut pc: u32 = 0x82630A68;
    'dispatch: loop {
        match pc {
            0x82630A68 => {
    //   block [0x82630A68..0x82630AD4)
	// 82630A68: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82630A6C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82630A70: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82630A74: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 82630A78: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82630A7C: 38EBD7E4  addi r7, r11, -0x281c
	ctx.r[7].s64 = ctx.r[11].s64 + -10268;
	// 82630A80: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 82630A84: 388A9D38  addi r4, r10, -0x62c8
	ctx.r[4].s64 = ctx.r[10].s64 + -25288;
	// 82630A88: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82630A8C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82630A90: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82630A94: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82630A98: 386A6B74  addi r3, r10, 0x6b74
	ctx.r[3].s64 = ctx.r[10].s64 + 27508;
	// 82630A9C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82630AA0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82630AA4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82630AA8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82630AAC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82630AB0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82630AB4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82630AB8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82630ABC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82630AC0: 4BE36361  bl 0x82466e20
	ctx.lr = 0x82630AC4;
	sub_82466E20(ctx, base);
	// 82630AC4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82630AC8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82630ACC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82630AD0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82630AD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82630AD8 size=24
    let mut pc: u32 = 0x82630AD8;
    'dispatch: loop {
        match pc {
            0x82630AD8 => {
    //   block [0x82630AD8..0x82630AF0)
	// 82630AD8: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 82630ADC: 3D408277  lis r10, -0x7d89
	ctx.r[10].s64 = -2106130432;
	// 82630AE0: 394A2200  addi r10, r10, 0x2200
	ctx.r[10].s64 = ctx.r[10].s64 + 8704;
	// 82630AE4: 816BD814  lwz r11, -0x27ec(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-10220 as u32) ) } as u64;
	// 82630AE8: 916A0008  stw r11, 8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82630AEC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82630AF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82630AF0 size=112
    let mut pc: u32 = 0x82630AF0;
    'dispatch: loop {
        match pc {
            0x82630AF0 => {
    //   block [0x82630AF0..0x82630B60)
	// 82630AF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82630AF4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82630AF8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82630AFC: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82630B00: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 82630B04: 392A4D7C  addi r9, r10, 0x4d7c
	ctx.r[9].s64 = ctx.r[10].s64 + 19836;
	// 82630B08: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82630B0C: 390B2200  addi r8, r11, 0x2200
	ctx.r[8].s64 = ctx.r[11].s64 + 8704;
	// 82630B10: 38E00005  li r7, 5
	ctx.r[7].s64 = 5;
	// 82630B14: 388AB450  addi r4, r10, -0x4bb0
	ctx.r[4].s64 = ctx.r[10].s64 + -19376;
	// 82630B18: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82630B1C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82630B20: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82630B24: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82630B28: 386A6BA4  addi r3, r10, 0x6ba4
	ctx.r[3].s64 = ctx.r[10].s64 + 27556;
	// 82630B2C: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82630B30: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82630B34: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82630B38: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82630B3C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82630B40: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82630B44: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82630B48: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82630B4C: 4BE362D5  bl 0x82466e20
	ctx.lr = 0x82630B50;
	sub_82466E20(ctx, base);
	// 82630B50: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82630B54: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82630B58: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82630B5C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82630B60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82630B60 size=108
    let mut pc: u32 = 0x82630B60;
    'dispatch: loop {
        match pc {
            0x82630B60 => {
    //   block [0x82630B60..0x82630BCC)
	// 82630B60: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82630B64: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82630B68: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82630B6C: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 82630B70: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82630B74: 38EBD818  addi r7, r11, -0x27e8
	ctx.r[7].s64 = ctx.r[11].s64 + -10216;
	// 82630B78: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 82630B7C: 388A9CF4  addi r4, r10, -0x630c
	ctx.r[4].s64 = ctx.r[10].s64 + -25356;
	// 82630B80: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82630B84: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82630B88: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82630B8C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82630B90: 386A6BD4  addi r3, r10, 0x6bd4
	ctx.r[3].s64 = ctx.r[10].s64 + 27604;
	// 82630B94: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82630B98: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82630B9C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82630BA0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82630BA4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82630BA8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82630BAC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82630BB0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82630BB4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82630BB8: 4BE36269  bl 0x82466e20
	ctx.lr = 0x82630BBC;
	sub_82466E20(ctx, base);
	// 82630BBC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82630BC0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82630BC4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82630BC8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82630BD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82630BD0 size=108
    let mut pc: u32 = 0x82630BD0;
    'dispatch: loop {
        match pc {
            0x82630BD0 => {
    //   block [0x82630BD0..0x82630C3C)
	// 82630BD0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82630BD4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82630BD8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82630BDC: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 82630BE0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82630BE4: 38EBD830  addi r7, r11, -0x27d0
	ctx.r[7].s64 = ctx.r[11].s64 + -10192;
	// 82630BE8: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 82630BEC: 388AB148  addi r4, r10, -0x4eb8
	ctx.r[4].s64 = ctx.r[10].s64 + -20152;
	// 82630BF0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82630BF4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82630BF8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82630BFC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82630C00: 386A6C04  addi r3, r10, 0x6c04
	ctx.r[3].s64 = ctx.r[10].s64 + 27652;
	// 82630C04: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82630C08: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82630C0C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82630C10: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82630C14: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82630C18: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82630C1C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82630C20: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82630C24: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82630C28: 4BE361F9  bl 0x82466e20
	ctx.lr = 0x82630C2C;
	sub_82466E20(ctx, base);
	// 82630C2C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82630C30: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82630C34: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82630C38: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82630C40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82630C40 size=108
    let mut pc: u32 = 0x82630C40;
    'dispatch: loop {
        match pc {
            0x82630C40 => {
    //   block [0x82630C40..0x82630CAC)
	// 82630C40: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82630C44: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82630C48: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82630C4C: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 82630C50: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82630C54: 38EBD878  addi r7, r11, -0x2788
	ctx.r[7].s64 = ctx.r[11].s64 + -10120;
	// 82630C58: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 82630C5C: 388AA9FC  addi r4, r10, -0x5604
	ctx.r[4].s64 = ctx.r[10].s64 + -22020;
	// 82630C60: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82630C64: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82630C68: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82630C6C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82630C70: 386A6C34  addi r3, r10, 0x6c34
	ctx.r[3].s64 = ctx.r[10].s64 + 27700;
	// 82630C74: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82630C78: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82630C7C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82630C80: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82630C84: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82630C88: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82630C8C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82630C90: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82630C94: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82630C98: 4BE36189  bl 0x82466e20
	ctx.lr = 0x82630C9C;
	sub_82466E20(ctx, base);
	// 82630C9C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82630CA0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82630CA4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82630CA8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82630CB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82630CB0 size=108
    let mut pc: u32 = 0x82630CB0;
    'dispatch: loop {
        match pc {
            0x82630CB0 => {
    //   block [0x82630CB0..0x82630D1C)
	// 82630CB0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82630CB4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82630CB8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82630CBC: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 82630CC0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82630CC4: 38EBD8A8  addi r7, r11, -0x2758
	ctx.r[7].s64 = ctx.r[11].s64 + -10072;
	// 82630CC8: 3900000C  li r8, 0xc
	ctx.r[8].s64 = 12;
	// 82630CCC: 388AA978  addi r4, r10, -0x5688
	ctx.r[4].s64 = ctx.r[10].s64 + -22152;
	// 82630CD0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82630CD4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82630CD8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82630CDC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82630CE0: 386A6C64  addi r3, r10, 0x6c64
	ctx.r[3].s64 = ctx.r[10].s64 + 27748;
	// 82630CE4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82630CE8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82630CEC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82630CF0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82630CF4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82630CF8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82630CFC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82630D00: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82630D04: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82630D08: 4BE36119  bl 0x82466e20
	ctx.lr = 0x82630D0C;
	sub_82466E20(ctx, base);
	// 82630D0C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82630D10: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82630D14: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82630D18: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82630D20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82630D20 size=108
    let mut pc: u32 = 0x82630D20;
    'dispatch: loop {
        match pc {
            0x82630D20 => {
    //   block [0x82630D20..0x82630D8C)
	// 82630D20: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82630D24: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82630D28: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82630D2C: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 82630D30: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82630D34: 38EBD9C8  addi r7, r11, -0x2638
	ctx.r[7].s64 = ctx.r[11].s64 + -9784;
	// 82630D38: 39000006  li r8, 6
	ctx.r[8].s64 = 6;
	// 82630D3C: 388AB380  addi r4, r10, -0x4c80
	ctx.r[4].s64 = ctx.r[10].s64 + -19584;
	// 82630D40: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82630D44: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82630D48: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82630D4C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82630D50: 386A6C94  addi r3, r10, 0x6c94
	ctx.r[3].s64 = ctx.r[10].s64 + 27796;
	// 82630D54: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82630D58: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82630D5C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82630D60: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82630D64: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82630D68: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82630D6C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82630D70: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82630D74: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82630D78: 4BE360A9  bl 0x82466e20
	ctx.lr = 0x82630D7C;
	sub_82466E20(ctx, base);
	// 82630D7C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82630D80: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82630D84: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82630D88: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82630D90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82630D90 size=108
    let mut pc: u32 = 0x82630D90;
    'dispatch: loop {
        match pc {
            0x82630D90 => {
    //   block [0x82630D90..0x82630DFC)
	// 82630D90: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82630D94: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82630D98: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82630D9C: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 82630DA0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82630DA4: 38EBDA58  addi r7, r11, -0x25a8
	ctx.r[7].s64 = ctx.r[11].s64 + -9640;
	// 82630DA8: 39000008  li r8, 8
	ctx.r[8].s64 = 8;
	// 82630DAC: 388AA228  addi r4, r10, -0x5dd8
	ctx.r[4].s64 = ctx.r[10].s64 + -24024;
	// 82630DB0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82630DB4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82630DB8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82630DBC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82630DC0: 386A6CC4  addi r3, r10, 0x6cc4
	ctx.r[3].s64 = ctx.r[10].s64 + 27844;
	// 82630DC4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82630DC8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82630DCC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82630DD0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82630DD4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82630DD8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82630DDC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82630DE0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82630DE4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82630DE8: 4BE36039  bl 0x82466e20
	ctx.lr = 0x82630DEC;
	sub_82466E20(ctx, base);
	// 82630DEC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82630DF0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82630DF4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82630DF8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82630E00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82630E00 size=108
    let mut pc: u32 = 0x82630E00;
    'dispatch: loop {
        match pc {
            0x82630E00 => {
    //   block [0x82630E00..0x82630E6C)
	// 82630E00: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82630E04: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82630E08: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82630E0C: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 82630E10: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82630E14: 38EBDB18  addi r7, r11, -0x24e8
	ctx.r[7].s64 = ctx.r[11].s64 + -9448;
	// 82630E18: 39000009  li r8, 9
	ctx.r[8].s64 = 9;
	// 82630E1C: 388AA000  addi r4, r10, -0x6000
	ctx.r[4].s64 = ctx.r[10].s64 + -24576;
	// 82630E20: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82630E24: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82630E28: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82630E2C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82630E30: 386A6CF4  addi r3, r10, 0x6cf4
	ctx.r[3].s64 = ctx.r[10].s64 + 27892;
	// 82630E34: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82630E38: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82630E3C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82630E40: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82630E44: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82630E48: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82630E4C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82630E50: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82630E54: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82630E58: 4BE35FC9  bl 0x82466e20
	ctx.lr = 0x82630E5C;
	sub_82466E20(ctx, base);
	// 82630E5C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82630E60: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82630E64: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82630E68: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82630E70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82630E70 size=108
    let mut pc: u32 = 0x82630E70;
    'dispatch: loop {
        match pc {
            0x82630E70 => {
    //   block [0x82630E70..0x82630EDC)
	// 82630E70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82630E74: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82630E78: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82630E7C: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 82630E80: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82630E84: 38EBDBF0  addi r7, r11, -0x2410
	ctx.r[7].s64 = ctx.r[11].s64 + -9232;
	// 82630E88: 39000008  li r8, 8
	ctx.r[8].s64 = 8;
	// 82630E8C: 388A9F00  addi r4, r10, -0x6100
	ctx.r[4].s64 = ctx.r[10].s64 + -24832;
	// 82630E90: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82630E94: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82630E98: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82630E9C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82630EA0: 386A6D24  addi r3, r10, 0x6d24
	ctx.r[3].s64 = ctx.r[10].s64 + 27940;
	// 82630EA4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82630EA8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82630EAC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82630EB0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82630EB4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82630EB8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82630EBC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82630EC0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82630EC4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82630EC8: 4BE35F59  bl 0x82466e20
	ctx.lr = 0x82630ECC;
	sub_82466E20(ctx, base);
	// 82630ECC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82630ED0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82630ED4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82630ED8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82630EE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82630EE0 size=108
    let mut pc: u32 = 0x82630EE0;
    'dispatch: loop {
        match pc {
            0x82630EE0 => {
    //   block [0x82630EE0..0x82630F4C)
	// 82630EE0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82630EE4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82630EE8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82630EEC: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 82630EF0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82630EF4: 38EBDCB0  addi r7, r11, -0x2350
	ctx.r[7].s64 = ctx.r[11].s64 + -9040;
	// 82630EF8: 39000007  li r8, 7
	ctx.r[8].s64 = 7;
	// 82630EFC: 388AADE0  addi r4, r10, -0x5220
	ctx.r[4].s64 = ctx.r[10].s64 + -21024;
	// 82630F00: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82630F04: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82630F08: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82630F0C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82630F10: 386A6D54  addi r3, r10, 0x6d54
	ctx.r[3].s64 = ctx.r[10].s64 + 27988;
	// 82630F14: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82630F18: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82630F1C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82630F20: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82630F24: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82630F28: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82630F2C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82630F30: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82630F34: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82630F38: 4BE35EE9  bl 0x82466e20
	ctx.lr = 0x82630F3C;
	sub_82466E20(ctx, base);
	// 82630F3C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82630F40: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82630F44: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82630F48: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82630F50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82630F50 size=112
    let mut pc: u32 = 0x82630F50;
    'dispatch: loop {
        match pc {
            0x82630F50 => {
    //   block [0x82630F50..0x82630FC0)
	// 82630F50: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82630F54: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82630F58: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82630F5C: 3D408277  lis r10, -0x7d89
	ctx.r[10].s64 = -2106130432;
	// 82630F60: 3900000C  li r8, 0xc
	ctx.r[8].s64 = 12;
	// 82630F64: 38EADD58  addi r7, r10, -0x22a8
	ctx.r[7].s64 = ctx.r[10].s64 + -8872;
	// 82630F68: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82630F6C: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82630F70: 388AA5FC  addi r4, r10, -0x5a04
	ctx.r[4].s64 = ctx.r[10].s64 + -23044;
	// 82630F74: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82630F78: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82630F7C: 396B4D90  addi r11, r11, 0x4d90
	ctx.r[11].s64 = ctx.r[11].s64 + 19856;
	// 82630F80: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82630F84: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82630F88: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82630F8C: 386A6D84  addi r3, r10, 0x6d84
	ctx.r[3].s64 = ctx.r[10].s64 + 28036;
	// 82630F90: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82630F94: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 82630F98: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82630F9C: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 82630FA0: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82630FA4: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82630FA8: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82630FAC: 4BE35E75  bl 0x82466e20
	ctx.lr = 0x82630FB0;
	sub_82466E20(ctx, base);
	// 82630FB0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82630FB4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82630FB8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82630FBC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82630FC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82630FC0 size=108
    let mut pc: u32 = 0x82630FC0;
    'dispatch: loop {
        match pc {
            0x82630FC0 => {
    //   block [0x82630FC0..0x8263102C)
	// 82630FC0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82630FC4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82630FC8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82630FCC: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 82630FD0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82630FD4: 38EBDE78  addi r7, r11, -0x2188
	ctx.r[7].s64 = ctx.r[11].s64 + -8584;
	// 82630FD8: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 82630FDC: 388AAD28  addi r4, r10, -0x52d8
	ctx.r[4].s64 = ctx.r[10].s64 + -21208;
	// 82630FE0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82630FE4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82630FE8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82630FEC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82630FF0: 386A6DB4  addi r3, r10, 0x6db4
	ctx.r[3].s64 = ctx.r[10].s64 + 28084;
	// 82630FF4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82630FF8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82630FFC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82631000: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82631004: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82631008: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8263100C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82631010: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82631014: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82631018: 4BE35E09  bl 0x82466e20
	ctx.lr = 0x8263101C;
	sub_82466E20(ctx, base);
	// 8263101C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82631020: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82631024: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82631028: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82631030(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82631030 size=108
    let mut pc: u32 = 0x82631030;
    'dispatch: loop {
        match pc {
            0x82631030 => {
    //   block [0x82631030..0x8263109C)
	// 82631030: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82631034: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82631038: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263103C: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 82631040: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82631044: 38EBDED8  addi r7, r11, -0x2128
	ctx.r[7].s64 = ctx.r[11].s64 + -8488;
	// 82631048: 3900000B  li r8, 0xb
	ctx.r[8].s64 = 11;
	// 8263104C: 388A9070  addi r4, r10, -0x6f90
	ctx.r[4].s64 = ctx.r[10].s64 + -28560;
	// 82631050: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82631054: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82631058: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8263105C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82631060: 386A6DE4  addi r3, r10, 0x6de4
	ctx.r[3].s64 = ctx.r[10].s64 + 28132;
	// 82631064: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82631068: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8263106C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82631070: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82631074: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82631078: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8263107C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82631080: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82631084: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82631088: 4BE35D99  bl 0x82466e20
	ctx.lr = 0x8263108C;
	sub_82466E20(ctx, base);
	// 8263108C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82631090: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82631094: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82631098: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826310A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826310A0 size=108
    let mut pc: u32 = 0x826310A0;
    'dispatch: loop {
        match pc {
            0x826310A0 => {
    //   block [0x826310A0..0x8263110C)
	// 826310A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826310A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826310A8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826310AC: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 826310B0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826310B4: 38EBDFE0  addi r7, r11, -0x2020
	ctx.r[7].s64 = ctx.r[11].s64 + -8224;
	// 826310B8: 39000009  li r8, 9
	ctx.r[8].s64 = 9;
	// 826310BC: 388A8E98  addi r4, r10, -0x7168
	ctx.r[4].s64 = ctx.r[10].s64 + -29032;
	// 826310C0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826310C4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826310C8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826310CC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826310D0: 386A6E14  addi r3, r10, 0x6e14
	ctx.r[3].s64 = ctx.r[10].s64 + 28180;
	// 826310D4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826310D8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826310DC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826310E0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826310E4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826310E8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826310EC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826310F0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826310F4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826310F8: 4BE35D29  bl 0x82466e20
	ctx.lr = 0x826310FC;
	sub_82466E20(ctx, base);
	// 826310FC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82631100: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82631104: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82631108: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82631110(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82631110 size=108
    let mut pc: u32 = 0x82631110;
    'dispatch: loop {
        match pc {
            0x82631110 => {
    //   block [0x82631110..0x8263117C)
	// 82631110: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82631114: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82631118: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263111C: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 82631120: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82631124: 38EBE0B8  addi r7, r11, -0x1f48
	ctx.r[7].s64 = ctx.r[11].s64 + -8008;
	// 82631128: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 8263112C: 388A8CF0  addi r4, r10, -0x7310
	ctx.r[4].s64 = ctx.r[10].s64 + -29456;
	// 82631130: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82631134: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82631138: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8263113C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82631140: 386A6E44  addi r3, r10, 0x6e44
	ctx.r[3].s64 = ctx.r[10].s64 + 28228;
	// 82631144: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82631148: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8263114C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82631150: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82631154: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82631158: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8263115C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82631160: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82631164: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82631168: 4BE35CB9  bl 0x82466e20
	ctx.lr = 0x8263116C;
	sub_82466E20(ctx, base);
	// 8263116C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82631170: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82631174: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82631178: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82631180(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82631180 size=108
    let mut pc: u32 = 0x82631180;
    'dispatch: loop {
        match pc {
            0x82631180 => {
    //   block [0x82631180..0x826311EC)
	// 82631180: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82631184: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82631188: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263118C: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 82631190: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82631194: 38EBE100  addi r7, r11, -0x1f00
	ctx.r[7].s64 = ctx.r[11].s64 + -7936;
	// 82631198: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 8263119C: 388A8D14  addi r4, r10, -0x72ec
	ctx.r[4].s64 = ctx.r[10].s64 + -29420;
	// 826311A0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826311A4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826311A8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826311AC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826311B0: 386A6E74  addi r3, r10, 0x6e74
	ctx.r[3].s64 = ctx.r[10].s64 + 28276;
	// 826311B4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826311B8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826311BC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826311C0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826311C4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826311C8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826311CC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826311D0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826311D4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826311D8: 4BE35C49  bl 0x82466e20
	ctx.lr = 0x826311DC;
	sub_82466E20(ctx, base);
	// 826311DC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826311E0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826311E4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826311E8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826311F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826311F0 size=108
    let mut pc: u32 = 0x826311F0;
    'dispatch: loop {
        match pc {
            0x826311F0 => {
    //   block [0x826311F0..0x8263125C)
	// 826311F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826311F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826311F8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826311FC: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 82631200: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82631204: 38EBE118  addi r7, r11, -0x1ee8
	ctx.r[7].s64 = ctx.r[11].s64 + -7912;
	// 82631208: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 8263120C: 388A37C4  addi r4, r10, 0x37c4
	ctx.r[4].s64 = ctx.r[10].s64 + 14276;
	// 82631210: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82631214: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82631218: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8263121C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82631220: 386A6EA4  addi r3, r10, 0x6ea4
	ctx.r[3].s64 = ctx.r[10].s64 + 28324;
	// 82631224: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82631228: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8263122C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82631230: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82631234: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82631238: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8263123C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82631240: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82631244: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82631248: 4BE35BD9  bl 0x82466e20
	ctx.lr = 0x8263124C;
	sub_82466E20(ctx, base);
	// 8263124C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82631250: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82631254: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82631258: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82631260(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82631260 size=108
    let mut pc: u32 = 0x82631260;
    'dispatch: loop {
        match pc {
            0x82631260 => {
    //   block [0x82631260..0x826312CC)
	// 82631260: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82631264: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82631268: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263126C: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 82631270: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82631274: 38EBE160  addi r7, r11, -0x1ea0
	ctx.r[7].s64 = ctx.r[11].s64 + -7840;
	// 82631278: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 8263127C: 388A37F4  addi r4, r10, 0x37f4
	ctx.r[4].s64 = ctx.r[10].s64 + 14324;
	// 82631280: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82631284: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82631288: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8263128C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82631290: 386A6ED4  addi r3, r10, 0x6ed4
	ctx.r[3].s64 = ctx.r[10].s64 + 28372;
	// 82631294: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82631298: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8263129C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826312A0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826312A4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826312A8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826312AC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826312B0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826312B4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826312B8: 4BE35B69  bl 0x82466e20
	ctx.lr = 0x826312BC;
	sub_82466E20(ctx, base);
	// 826312BC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826312C0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826312C4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826312C8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826312D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826312D0 size=112
    let mut pc: u32 = 0x826312D0;
    'dispatch: loop {
        match pc {
            0x826312D0 => {
    //   block [0x826312D0..0x82631340)
	// 826312D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826312D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826312D8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826312DC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826312E0: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 826312E4: 38AA64E4  addi r5, r10, 0x64e4
	ctx.r[5].s64 = ctx.r[10].s64 + 25828;
	// 826312E8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826312EC: 390BE178  addi r8, r11, -0x1e88
	ctx.r[8].s64 = ctx.r[11].s64 + -7816;
	// 826312F0: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 826312F4: 388A3810  addi r4, r10, 0x3810
	ctx.r[4].s64 = ctx.r[10].s64 + 14352;
	// 826312F8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826312FC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82631300: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82631304: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82631308: 386A6F04  addi r3, r10, 0x6f04
	ctx.r[3].s64 = ctx.r[10].s64 + 28420;
	// 8263130C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82631310: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82631314: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82631318: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8263131C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82631320: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82631324: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82631328: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8263132C: 4BE35AF5  bl 0x82466e20
	ctx.lr = 0x82631330;
	sub_82466E20(ctx, base);
	// 82631330: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82631334: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82631338: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8263133C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82631340(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82631340 size=108
    let mut pc: u32 = 0x82631340;
    'dispatch: loop {
        match pc {
            0x82631340 => {
    //   block [0x82631340..0x826313AC)
	// 82631340: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82631344: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82631348: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263134C: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 82631350: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82631354: 38EBE1C0  addi r7, r11, -0x1e40
	ctx.r[7].s64 = ctx.r[11].s64 + -7744;
	// 82631358: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 8263135C: 388A3A30  addi r4, r10, 0x3a30
	ctx.r[4].s64 = ctx.r[10].s64 + 14896;
	// 82631360: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82631364: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82631368: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8263136C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82631370: 386A6F34  addi r3, r10, 0x6f34
	ctx.r[3].s64 = ctx.r[10].s64 + 28468;
	// 82631374: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82631378: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8263137C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82631380: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82631384: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82631388: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8263138C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82631390: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82631394: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82631398: 4BE35A89  bl 0x82466e20
	ctx.lr = 0x8263139C;
	sub_82466E20(ctx, base);
	// 8263139C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826313A0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826313A4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826313A8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826313B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826313B0 size=112
    let mut pc: u32 = 0x826313B0;
    'dispatch: loop {
        match pc {
            0x826313B0 => {
    //   block [0x826313B0..0x82631420)
	// 826313B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826313B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826313B8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826313BC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826313C0: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 826313C4: 38AA6F34  addi r5, r10, 0x6f34
	ctx.r[5].s64 = ctx.r[10].s64 + 28468;
	// 826313C8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826313CC: 390BE220  addi r8, r11, -0x1de0
	ctx.r[8].s64 = ctx.r[11].s64 + -7648;
	// 826313D0: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 826313D4: 388A3A3C  addi r4, r10, 0x3a3c
	ctx.r[4].s64 = ctx.r[10].s64 + 14908;
	// 826313D8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826313DC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826313E0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826313E4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826313E8: 386A6F64  addi r3, r10, 0x6f64
	ctx.r[3].s64 = ctx.r[10].s64 + 28516;
	// 826313EC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826313F0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826313F4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826313F8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826313FC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82631400: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82631404: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82631408: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8263140C: 4BE35A15  bl 0x82466e20
	ctx.lr = 0x82631410;
	sub_82466E20(ctx, base);
	// 82631410: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82631414: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82631418: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8263141C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82631420(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82631420 size=96
    let mut pc: u32 = 0x82631420;
    'dispatch: loop {
        match pc {
            0x82631420 => {
    //   block [0x82631420..0x82631480)
	// 82631420: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82631424: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82631428: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263142C: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82631430: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82631434: 388A3A4C  addi r4, r10, 0x3a4c
	ctx.r[4].s64 = ctx.r[10].s64 + 14924;
	// 82631438: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8263143C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82631440: 386A6F94  addi r3, r10, 0x6f94
	ctx.r[3].s64 = ctx.r[10].s64 + 28564;
	// 82631444: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82631448: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8263144C: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 82631450: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82631454: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82631458: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8263145C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82631460: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82631464: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82631468: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8263146C: 4BE359B5  bl 0x82466e20
	ctx.lr = 0x82631470;
	sub_82466E20(ctx, base);
	// 82631470: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82631474: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82631478: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8263147C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82631480(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82631480 size=112
    let mut pc: u32 = 0x82631480;
    'dispatch: loop {
        match pc {
            0x82631480 => {
    //   block [0x82631480..0x826314F0)
	// 82631480: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82631484: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82631488: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263148C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82631490: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 82631494: 38AA87F4  addi r5, r10, -0x780c
	ctx.r[5].s64 = ctx.r[10].s64 + -30732;
	// 82631498: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8263149C: 390BE268  addi r8, r11, -0x1d98
	ctx.r[8].s64 = ctx.r[11].s64 + -7576;
	// 826314A0: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 826314A4: 388A3A64  addi r4, r10, 0x3a64
	ctx.r[4].s64 = ctx.r[10].s64 + 14948;
	// 826314A8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826314AC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826314B0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826314B4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826314B8: 386A6FC4  addi r3, r10, 0x6fc4
	ctx.r[3].s64 = ctx.r[10].s64 + 28612;
	// 826314BC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826314C0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826314C4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826314C8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826314CC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826314D0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826314D4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826314D8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826314DC: 4BE35945  bl 0x82466e20
	ctx.lr = 0x826314E0;
	sub_82466E20(ctx, base);
	// 826314E0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826314E4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826314E8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826314EC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826314F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826314F0 size=96
    let mut pc: u32 = 0x826314F0;
    'dispatch: loop {
        match pc {
            0x826314F0 => {
    //   block [0x826314F0..0x82631550)
	// 826314F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826314F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826314F8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826314FC: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82631500: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82631504: 388A3A7C  addi r4, r10, 0x3a7c
	ctx.r[4].s64 = ctx.r[10].s64 + 14972;
	// 82631508: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8263150C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82631510: 386A6FF4  addi r3, r10, 0x6ff4
	ctx.r[3].s64 = ctx.r[10].s64 + 28660;
	// 82631514: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82631518: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8263151C: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 82631520: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82631524: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82631528: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8263152C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82631530: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82631534: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82631538: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8263153C: 4BE358E5  bl 0x82466e20
	ctx.lr = 0x82631540;
	sub_82466E20(ctx, base);
	// 82631540: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82631544: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82631548: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8263154C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82631550(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82631550 size=100
    let mut pc: u32 = 0x82631550;
    'dispatch: loop {
        match pc {
            0x82631550 => {
    //   block [0x82631550..0x826315B4)
	// 82631550: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82631554: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82631558: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263155C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82631560: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82631564: 38AA64E4  addi r5, r10, 0x64e4
	ctx.r[5].s64 = ctx.r[10].s64 + 25828;
	// 82631568: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8263156C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82631570: 388A3A9C  addi r4, r10, 0x3a9c
	ctx.r[4].s64 = ctx.r[10].s64 + 15004;
	// 82631574: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82631578: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8263157C: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 82631580: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82631584: 386A7024  addi r3, r10, 0x7024
	ctx.r[3].s64 = ctx.r[10].s64 + 28708;
	// 82631588: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8263158C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82631590: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82631594: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82631598: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8263159C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826315A0: 4BE35881  bl 0x82466e20
	ctx.lr = 0x826315A4;
	sub_82466E20(ctx, base);
	// 826315A4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826315A8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826315AC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826315B0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826315B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826315B8 size=96
    let mut pc: u32 = 0x826315B8;
    'dispatch: loop {
        match pc {
            0x826315B8 => {
    //   block [0x826315B8..0x82631618)
	// 826315B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826315BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826315C0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826315C4: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826315C8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826315CC: 388A3AC4  addi r4, r10, 0x3ac4
	ctx.r[4].s64 = ctx.r[10].s64 + 15044;
	// 826315D0: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826315D4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826315D8: 386A7054  addi r3, r10, 0x7054
	ctx.r[3].s64 = ctx.r[10].s64 + 28756;
	// 826315DC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826315E0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826315E4: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826315E8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826315EC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826315F0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826315F4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826315F8: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826315FC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82631600: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82631604: 4BE3581D  bl 0x82466e20
	ctx.lr = 0x82631608;
	sub_82466E20(ctx, base);
	// 82631608: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8263160C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82631610: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82631614: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82631618(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82631618 size=112
    let mut pc: u32 = 0x82631618;
    'dispatch: loop {
        match pc {
            0x82631618 => {
    //   block [0x82631618..0x82631688)
	// 82631618: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263161C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82631620: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82631624: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82631628: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8263162C: 38AA7024  addi r5, r10, 0x7024
	ctx.r[5].s64 = ctx.r[10].s64 + 28708;
	// 82631630: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82631634: 390BE2C8  addi r8, r11, -0x1d38
	ctx.r[8].s64 = ctx.r[11].s64 + -7480;
	// 82631638: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 8263163C: 388A3AF8  addi r4, r10, 0x3af8
	ctx.r[4].s64 = ctx.r[10].s64 + 15096;
	// 82631640: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82631644: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82631648: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8263164C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82631650: 386A7084  addi r3, r10, 0x7084
	ctx.r[3].s64 = ctx.r[10].s64 + 28804;
	// 82631654: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82631658: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8263165C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82631660: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82631664: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82631668: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8263166C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82631670: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82631674: 4BE357AD  bl 0x82466e20
	ctx.lr = 0x82631678;
	sub_82466E20(ctx, base);
	// 82631678: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8263167C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82631680: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82631684: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82631688(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82631688 size=112
    let mut pc: u32 = 0x82631688;
    'dispatch: loop {
        match pc {
            0x82631688 => {
    //   block [0x82631688..0x826316F8)
	// 82631688: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263168C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82631690: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82631694: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82631698: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8263169C: 38AA7024  addi r5, r10, 0x7024
	ctx.r[5].s64 = ctx.r[10].s64 + 28708;
	// 826316A0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826316A4: 390BE2F8  addi r8, r11, -0x1d08
	ctx.r[8].s64 = ctx.r[11].s64 + -7432;
	// 826316A8: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826316AC: 388A3B08  addi r4, r10, 0x3b08
	ctx.r[4].s64 = ctx.r[10].s64 + 15112;
	// 826316B0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826316B4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826316B8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826316BC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826316C0: 386A70B4  addi r3, r10, 0x70b4
	ctx.r[3].s64 = ctx.r[10].s64 + 28852;
	// 826316C4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826316C8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826316CC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826316D0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826316D4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826316D8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826316DC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826316E0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826316E4: 4BE3573D  bl 0x82466e20
	ctx.lr = 0x826316E8;
	sub_82466E20(ctx, base);
	// 826316E8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826316EC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826316F0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826316F4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826316F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826316F8 size=100
    let mut pc: u32 = 0x826316F8;
    'dispatch: loop {
        match pc {
            0x826316F8 => {
    //   block [0x826316F8..0x8263175C)
	// 826316F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826316FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82631700: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82631704: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82631708: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8263170C: 38AA7024  addi r5, r10, 0x7024
	ctx.r[5].s64 = ctx.r[10].s64 + 28708;
	// 82631710: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82631714: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82631718: 388A3B20  addi r4, r10, 0x3b20
	ctx.r[4].s64 = ctx.r[10].s64 + 15136;
	// 8263171C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82631720: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82631724: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82631728: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8263172C: 386A70E4  addi r3, r10, 0x70e4
	ctx.r[3].s64 = ctx.r[10].s64 + 28900;
	// 82631730: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82631734: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82631738: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8263173C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82631740: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82631744: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82631748: 4BE356D9  bl 0x82466e20
	ctx.lr = 0x8263174C;
	sub_82466E20(ctx, base);
	// 8263174C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82631750: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82631754: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82631758: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82631760(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82631760 size=96
    let mut pc: u32 = 0x82631760;
    'dispatch: loop {
        match pc {
            0x82631760 => {
    //   block [0x82631760..0x826317C0)
	// 82631760: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82631764: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82631768: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263176C: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82631770: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82631774: 388A3B38  addi r4, r10, 0x3b38
	ctx.r[4].s64 = ctx.r[10].s64 + 15160;
	// 82631778: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8263177C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82631780: 386A7114  addi r3, r10, 0x7114
	ctx.r[3].s64 = ctx.r[10].s64 + 28948;
	// 82631784: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82631788: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8263178C: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 82631790: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82631794: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82631798: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8263179C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826317A0: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826317A4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826317A8: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826317AC: 4BE35675  bl 0x82466e20
	ctx.lr = 0x826317B0;
	sub_82466E20(ctx, base);
	// 826317B0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826317B4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826317B8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826317BC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826317C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826317C0 size=112
    let mut pc: u32 = 0x826317C0;
    'dispatch: loop {
        match pc {
            0x826317C0 => {
    //   block [0x826317C0..0x82631830)
	// 826317C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826317C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826317C8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826317CC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826317D0: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 826317D4: 38AA64E4  addi r5, r10, 0x64e4
	ctx.r[5].s64 = ctx.r[10].s64 + 25828;
	// 826317D8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826317DC: 390BE310  addi r8, r11, -0x1cf0
	ctx.r[8].s64 = ctx.r[11].s64 + -7408;
	// 826317E0: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826317E4: 388A3B54  addi r4, r10, 0x3b54
	ctx.r[4].s64 = ctx.r[10].s64 + 15188;
	// 826317E8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826317EC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826317F0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826317F4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826317F8: 386A7144  addi r3, r10, 0x7144
	ctx.r[3].s64 = ctx.r[10].s64 + 28996;
	// 826317FC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82631800: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82631804: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82631808: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8263180C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82631810: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82631814: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82631818: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8263181C: 4BE35605  bl 0x82466e20
	ctx.lr = 0x82631820;
	sub_82466E20(ctx, base);
	// 82631820: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82631824: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82631828: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8263182C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82631830(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82631830 size=96
    let mut pc: u32 = 0x82631830;
    'dispatch: loop {
        match pc {
            0x82631830 => {
    //   block [0x82631830..0x82631890)
	// 82631830: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82631834: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82631838: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263183C: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82631840: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82631844: 388A3B5C  addi r4, r10, 0x3b5c
	ctx.r[4].s64 = ctx.r[10].s64 + 15196;
	// 82631848: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8263184C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82631850: 386A7174  addi r3, r10, 0x7174
	ctx.r[3].s64 = ctx.r[10].s64 + 29044;
	// 82631854: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82631858: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8263185C: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 82631860: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82631864: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82631868: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8263186C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82631870: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82631874: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82631878: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8263187C: 4BE355A5  bl 0x82466e20
	ctx.lr = 0x82631880;
	sub_82466E20(ctx, base);
	// 82631880: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82631884: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82631888: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8263188C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82631890(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82631890 size=112
    let mut pc: u32 = 0x82631890;
    'dispatch: loop {
        match pc {
            0x82631890 => {
    //   block [0x82631890..0x82631900)
	// 82631890: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82631894: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82631898: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263189C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826318A0: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 826318A4: 38AA7174  addi r5, r10, 0x7174
	ctx.r[5].s64 = ctx.r[10].s64 + 29044;
	// 826318A8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826318AC: 390BE328  addi r8, r11, -0x1cd8
	ctx.r[8].s64 = ctx.r[11].s64 + -7384;
	// 826318B0: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826318B4: 388A3B70  addi r4, r10, 0x3b70
	ctx.r[4].s64 = ctx.r[10].s64 + 15216;
	// 826318B8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826318BC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826318C0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826318C4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826318C8: 386A71A4  addi r3, r10, 0x71a4
	ctx.r[3].s64 = ctx.r[10].s64 + 29092;
	// 826318CC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826318D0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826318D4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826318D8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826318DC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826318E0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826318E4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826318E8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826318EC: 4BE35535  bl 0x82466e20
	ctx.lr = 0x826318F0;
	sub_82466E20(ctx, base);
	// 826318F0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826318F4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826318F8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826318FC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82631900(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82631900 size=108
    let mut pc: u32 = 0x82631900;
    'dispatch: loop {
        match pc {
            0x82631900 => {
    //   block [0x82631900..0x8263196C)
	// 82631900: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82631904: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82631908: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263190C: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 82631910: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82631914: 38EBE340  addi r7, r11, -0x1cc0
	ctx.r[7].s64 = ctx.r[11].s64 + -7360;
	// 82631918: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 8263191C: 388A3B88  addi r4, r10, 0x3b88
	ctx.r[4].s64 = ctx.r[10].s64 + 15240;
	// 82631920: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82631924: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82631928: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8263192C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82631930: 386A71D4  addi r3, r10, 0x71d4
	ctx.r[3].s64 = ctx.r[10].s64 + 29140;
	// 82631934: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82631938: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8263193C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82631940: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82631944: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82631948: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8263194C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82631950: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82631954: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82631958: 4BE354C9  bl 0x82466e20
	ctx.lr = 0x8263195C;
	sub_82466E20(ctx, base);
	// 8263195C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82631960: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82631964: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82631968: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82631970(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82631970 size=112
    let mut pc: u32 = 0x82631970;
    'dispatch: loop {
        match pc {
            0x82631970 => {
    //   block [0x82631970..0x826319E0)
	// 82631970: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82631974: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82631978: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263197C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82631980: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 82631984: 38AA72F4  addi r5, r10, 0x72f4
	ctx.r[5].s64 = ctx.r[10].s64 + 29428;
	// 82631988: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8263198C: 390BE3A0  addi r8, r11, -0x1c60
	ctx.r[8].s64 = ctx.r[11].s64 + -7264;
	// 82631990: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82631994: 388A3B9C  addi r4, r10, 0x3b9c
	ctx.r[4].s64 = ctx.r[10].s64 + 15260;
	// 82631998: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8263199C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826319A0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826319A4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826319A8: 386A7204  addi r3, r10, 0x7204
	ctx.r[3].s64 = ctx.r[10].s64 + 29188;
	// 826319AC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826319B0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826319B4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826319B8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826319BC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826319C0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826319C4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826319C8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826319CC: 4BE35455  bl 0x82466e20
	ctx.lr = 0x826319D0;
	sub_82466E20(ctx, base);
	// 826319D0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826319D4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826319D8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826319DC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826319E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826319E0 size=112
    let mut pc: u32 = 0x826319E0;
    'dispatch: loop {
        match pc {
            0x826319E0 => {
    //   block [0x826319E0..0x82631A50)
	// 826319E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826319E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826319E8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826319EC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826319F0: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 826319F4: 38AA7144  addi r5, r10, 0x7144
	ctx.r[5].s64 = ctx.r[10].s64 + 28996;
	// 826319F8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826319FC: 390BE3B8  addi r8, r11, -0x1c48
	ctx.r[8].s64 = ctx.r[11].s64 + -7240;
	// 82631A00: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 82631A04: 388A3BA8  addi r4, r10, 0x3ba8
	ctx.r[4].s64 = ctx.r[10].s64 + 15272;
	// 82631A08: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82631A0C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82631A10: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82631A14: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82631A18: 386A7234  addi r3, r10, 0x7234
	ctx.r[3].s64 = ctx.r[10].s64 + 29236;
	// 82631A1C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82631A20: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82631A24: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82631A28: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82631A2C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82631A30: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82631A34: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82631A38: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82631A3C: 4BE353E5  bl 0x82466e20
	ctx.lr = 0x82631A40;
	sub_82466E20(ctx, base);
	// 82631A40: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82631A44: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82631A48: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82631A4C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82631A50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82631A50 size=112
    let mut pc: u32 = 0x82631A50;
    'dispatch: loop {
        match pc {
            0x82631A50 => {
    //   block [0x82631A50..0x82631AC0)
	// 82631A50: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82631A54: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82631A58: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82631A5C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82631A60: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 82631A64: 38AA7144  addi r5, r10, 0x7144
	ctx.r[5].s64 = ctx.r[10].s64 + 28996;
	// 82631A68: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82631A6C: 390BE3E8  addi r8, r11, -0x1c18
	ctx.r[8].s64 = ctx.r[11].s64 + -7192;
	// 82631A70: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82631A74: 388A3BB4  addi r4, r10, 0x3bb4
	ctx.r[4].s64 = ctx.r[10].s64 + 15284;
	// 82631A78: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82631A7C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82631A80: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82631A84: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82631A88: 386A7264  addi r3, r10, 0x7264
	ctx.r[3].s64 = ctx.r[10].s64 + 29284;
	// 82631A8C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82631A90: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82631A94: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82631A98: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82631A9C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82631AA0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82631AA4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82631AA8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82631AAC: 4BE35375  bl 0x82466e20
	ctx.lr = 0x82631AB0;
	sub_82466E20(ctx, base);
	// 82631AB0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82631AB4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82631AB8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82631ABC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82631AC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82631AC0 size=116
    let mut pc: u32 = 0x82631AC0;
    'dispatch: loop {
        match pc {
            0x82631AC0 => {
    //   block [0x82631AC0..0x82631B34)
	// 82631AC0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82631AC4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82631AC8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82631ACC: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 82631AD0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82631AD4: 390BE404  addi r8, r11, -0x1bfc
	ctx.r[8].s64 = ctx.r[11].s64 + -7164;
	// 82631AD8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82631ADC: 392A4E08  addi r9, r10, 0x4e08
	ctx.r[9].s64 = ctx.r[10].s64 + 19976;
	// 82631AE0: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82631AE4: 38E00002  li r7, 2
	ctx.r[7].s64 = 2;
	// 82631AE8: 38AA72F4  addi r5, r10, 0x72f4
	ctx.r[5].s64 = ctx.r[10].s64 + 29428;
	// 82631AEC: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82631AF0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82631AF4: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82631AF8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82631AFC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82631B00: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82631B04: 3D608294  lis r11, -0x7d6c
	ctx.r[11].s64 = -2104229888;
	// 82631B08: 388A3BC4  addi r4, r10, 0x3bc4
	ctx.r[4].s64 = ctx.r[10].s64 + 15300;
	// 82631B0C: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82631B10: 386B7294  addi r3, r11, 0x7294
	ctx.r[3].s64 = ctx.r[11].s64 + 29332;
	// 82631B14: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82631B18: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82631B1C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82631B20: 4BE35301  bl 0x82466e20
	ctx.lr = 0x82631B24;
	sub_82466E20(ctx, base);
	// 82631B24: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82631B28: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82631B2C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82631B30: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82631B38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82631B38 size=112
    let mut pc: u32 = 0x82631B38;
    'dispatch: loop {
        match pc {
            0x82631B38 => {
    //   block [0x82631B38..0x82631BA8)
	// 82631B38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82631B3C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82631B40: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82631B44: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82631B48: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 82631B4C: 38AA7144  addi r5, r10, 0x7144
	ctx.r[5].s64 = ctx.r[10].s64 + 28996;
	// 82631B50: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82631B54: 390BE434  addi r8, r11, -0x1bcc
	ctx.r[8].s64 = ctx.r[11].s64 + -7116;
	// 82631B58: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82631B5C: 388A3BD4  addi r4, r10, 0x3bd4
	ctx.r[4].s64 = ctx.r[10].s64 + 15316;
	// 82631B60: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82631B64: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82631B68: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82631B6C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82631B70: 386A72C4  addi r3, r10, 0x72c4
	ctx.r[3].s64 = ctx.r[10].s64 + 29380;
	// 82631B74: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82631B78: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82631B7C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82631B80: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82631B84: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 82631B88: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82631B8C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82631B90: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82631B94: 4BE3528D  bl 0x82466e20
	ctx.lr = 0x82631B98;
	sub_82466E20(ctx, base);
	// 82631B98: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82631B9C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82631BA0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82631BA4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82631BA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82631BA8 size=112
    let mut pc: u32 = 0x82631BA8;
    'dispatch: loop {
        match pc {
            0x82631BA8 => {
    //   block [0x82631BA8..0x82631C18)
	// 82631BA8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82631BAC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82631BB0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82631BB4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82631BB8: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 82631BBC: 38AA77A4  addi r5, r10, 0x77a4
	ctx.r[5].s64 = ctx.r[10].s64 + 30628;
	// 82631BC0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82631BC4: 390BE44C  addi r8, r11, -0x1bb4
	ctx.r[8].s64 = ctx.r[11].s64 + -7092;
	// 82631BC8: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82631BCC: 388A3BE8  addi r4, r10, 0x3be8
	ctx.r[4].s64 = ctx.r[10].s64 + 15336;
	// 82631BD0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82631BD4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82631BD8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82631BDC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82631BE0: 386A72F4  addi r3, r10, 0x72f4
	ctx.r[3].s64 = ctx.r[10].s64 + 29428;
	// 82631BE4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82631BE8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82631BEC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82631BF0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82631BF4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82631BF8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82631BFC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82631C00: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82631C04: 4BE3521D  bl 0x82466e20
	ctx.lr = 0x82631C08;
	sub_82466E20(ctx, base);
	// 82631C08: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82631C0C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82631C10: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82631C14: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82631C18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82631C18 size=112
    let mut pc: u32 = 0x82631C18;
    'dispatch: loop {
        match pc {
            0x82631C18 => {
    //   block [0x82631C18..0x82631C88)
	// 82631C18: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82631C1C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82631C20: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82631C24: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82631C28: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 82631C2C: 38AA7504  addi r5, r10, 0x7504
	ctx.r[5].s64 = ctx.r[10].s64 + 29956;
	// 82631C30: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82631C34: 390BE464  addi r8, r11, -0x1b9c
	ctx.r[8].s64 = ctx.r[11].s64 + -7068;
	// 82631C38: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82631C3C: 388A3BF8  addi r4, r10, 0x3bf8
	ctx.r[4].s64 = ctx.r[10].s64 + 15352;
	// 82631C40: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82631C44: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82631C48: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82631C4C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82631C50: 386A7324  addi r3, r10, 0x7324
	ctx.r[3].s64 = ctx.r[10].s64 + 29476;
	// 82631C54: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82631C58: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82631C5C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82631C60: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82631C64: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82631C68: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82631C6C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82631C70: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82631C74: 4BE351AD  bl 0x82466e20
	ctx.lr = 0x82631C78;
	sub_82466E20(ctx, base);
	// 82631C78: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82631C7C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82631C80: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82631C84: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82631C88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82631C88 size=112
    let mut pc: u32 = 0x82631C88;
    'dispatch: loop {
        match pc {
            0x82631C88 => {
    //   block [0x82631C88..0x82631CF8)
	// 82631C88: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82631C8C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82631C90: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82631C94: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82631C98: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 82631C9C: 38AA72C4  addi r5, r10, 0x72c4
	ctx.r[5].s64 = ctx.r[10].s64 + 29380;
	// 82631CA0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82631CA4: 390BE480  addi r8, r11, -0x1b80
	ctx.r[8].s64 = ctx.r[11].s64 + -7040;
	// 82631CA8: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 82631CAC: 388A3C0C  addi r4, r10, 0x3c0c
	ctx.r[4].s64 = ctx.r[10].s64 + 15372;
	// 82631CB0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82631CB4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82631CB8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82631CBC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82631CC0: 386A7354  addi r3, r10, 0x7354
	ctx.r[3].s64 = ctx.r[10].s64 + 29524;
	// 82631CC4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82631CC8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82631CCC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82631CD0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82631CD4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82631CD8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82631CDC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82631CE0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82631CE4: 4BE3513D  bl 0x82466e20
	ctx.lr = 0x82631CE8;
	sub_82466E20(ctx, base);
	// 82631CE8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82631CEC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82631CF0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82631CF4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82631CF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82631CF8 size=112
    let mut pc: u32 = 0x82631CF8;
    'dispatch: loop {
        match pc {
            0x82631CF8 => {
    //   block [0x82631CF8..0x82631D68)
	// 82631CF8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82631CFC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82631D00: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82631D04: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82631D08: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 82631D0C: 38AA72F4  addi r5, r10, 0x72f4
	ctx.r[5].s64 = ctx.r[10].s64 + 29428;
	// 82631D10: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82631D14: 390BE4C8  addi r8, r11, -0x1b38
	ctx.r[8].s64 = ctx.r[11].s64 + -6968;
	// 82631D18: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 82631D1C: 388A3C24  addi r4, r10, 0x3c24
	ctx.r[4].s64 = ctx.r[10].s64 + 15396;
	// 82631D20: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82631D24: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82631D28: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82631D2C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82631D30: 386A7384  addi r3, r10, 0x7384
	ctx.r[3].s64 = ctx.r[10].s64 + 29572;
	// 82631D34: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82631D38: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82631D3C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82631D40: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82631D44: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82631D48: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82631D4C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82631D50: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82631D54: 4BE350CD  bl 0x82466e20
	ctx.lr = 0x82631D58;
	sub_82466E20(ctx, base);
	// 82631D58: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82631D5C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82631D60: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82631D64: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82631D68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82631D68 size=112
    let mut pc: u32 = 0x82631D68;
    'dispatch: loop {
        match pc {
            0x82631D68 => {
    //   block [0x82631D68..0x82631DD8)
	// 82631D68: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82631D6C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82631D70: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82631D74: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82631D78: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 82631D7C: 38AA72F4  addi r5, r10, 0x72f4
	ctx.r[5].s64 = ctx.r[10].s64 + 29428;
	// 82631D80: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82631D84: 390BE4F8  addi r8, r11, -0x1b08
	ctx.r[8].s64 = ctx.r[11].s64 + -6920;
	// 82631D88: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 82631D8C: 388A3C3C  addi r4, r10, 0x3c3c
	ctx.r[4].s64 = ctx.r[10].s64 + 15420;
	// 82631D90: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82631D94: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82631D98: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82631D9C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82631DA0: 386A73B4  addi r3, r10, 0x73b4
	ctx.r[3].s64 = ctx.r[10].s64 + 29620;
	// 82631DA4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82631DA8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82631DAC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82631DB0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82631DB4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82631DB8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82631DBC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82631DC0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82631DC4: 4BE3505D  bl 0x82466e20
	ctx.lr = 0x82631DC8;
	sub_82466E20(ctx, base);
	// 82631DC8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82631DCC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82631DD0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82631DD4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82631DD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82631DD8 size=108
    let mut pc: u32 = 0x82631DD8;
    'dispatch: loop {
        match pc {
            0x82631DD8 => {
    //   block [0x82631DD8..0x82631E44)
	// 82631DD8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82631DDC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82631DE0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82631DE4: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 82631DE8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82631DEC: 38EBE528  addi r7, r11, -0x1ad8
	ctx.r[7].s64 = ctx.r[11].s64 + -6872;
	// 82631DF0: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 82631DF4: 388A3C54  addi r4, r10, 0x3c54
	ctx.r[4].s64 = ctx.r[10].s64 + 15444;
	// 82631DF8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82631DFC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82631E00: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82631E04: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82631E08: 386A73E4  addi r3, r10, 0x73e4
	ctx.r[3].s64 = ctx.r[10].s64 + 29668;
	// 82631E0C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82631E10: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82631E14: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82631E18: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82631E1C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82631E20: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82631E24: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82631E28: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82631E2C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82631E30: 4BE34FF1  bl 0x82466e20
	ctx.lr = 0x82631E34;
	sub_82466E20(ctx, base);
	// 82631E34: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82631E38: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82631E3C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82631E40: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82631E48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82631E48 size=112
    let mut pc: u32 = 0x82631E48;
    'dispatch: loop {
        match pc {
            0x82631E48 => {
    //   block [0x82631E48..0x82631EB8)
	// 82631E48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82631E4C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82631E50: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82631E54: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82631E58: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 82631E5C: 38AA72F4  addi r5, r10, 0x72f4
	ctx.r[5].s64 = ctx.r[10].s64 + 29428;
	// 82631E60: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82631E64: 390BE570  addi r8, r11, -0x1a90
	ctx.r[8].s64 = ctx.r[11].s64 + -6800;
	// 82631E68: 39200005  li r9, 5
	ctx.r[9].s64 = 5;
	// 82631E6C: 388A3C78  addi r4, r10, 0x3c78
	ctx.r[4].s64 = ctx.r[10].s64 + 15480;
	// 82631E70: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82631E74: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82631E78: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82631E7C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82631E80: 386A7414  addi r3, r10, 0x7414
	ctx.r[3].s64 = ctx.r[10].s64 + 29716;
	// 82631E84: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82631E88: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82631E8C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82631E90: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82631E94: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82631E98: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82631E9C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82631EA0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82631EA4: 4BE34F7D  bl 0x82466e20
	ctx.lr = 0x82631EA8;
	sub_82466E20(ctx, base);
	// 82631EA8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82631EAC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82631EB0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82631EB4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82631EB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82631EB8 size=116
    let mut pc: u32 = 0x82631EB8;
    'dispatch: loop {
        match pc {
            0x82631EB8 => {
    //   block [0x82631EB8..0x82631F2C)
	// 82631EB8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82631EBC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82631EC0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82631EC4: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 82631EC8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82631ECC: 390BE5E8  addi r8, r11, -0x1a18
	ctx.r[8].s64 = ctx.r[11].s64 + -6680;
	// 82631ED0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82631ED4: 392A4E44  addi r9, r10, 0x4e44
	ctx.r[9].s64 = ctx.r[10].s64 + 20036;
	// 82631ED8: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82631EDC: 38E00005  li r7, 5
	ctx.r[7].s64 = 5;
	// 82631EE0: 38AA72F4  addi r5, r10, 0x72f4
	ctx.r[5].s64 = ctx.r[10].s64 + 29428;
	// 82631EE4: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82631EE8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82631EEC: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82631EF0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82631EF4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82631EF8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82631EFC: 3D608294  lis r11, -0x7d6c
	ctx.r[11].s64 = -2104229888;
	// 82631F00: 388A3C90  addi r4, r10, 0x3c90
	ctx.r[4].s64 = ctx.r[10].s64 + 15504;
	// 82631F04: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82631F08: 386B7444  addi r3, r11, 0x7444
	ctx.r[3].s64 = ctx.r[11].s64 + 29764;
	// 82631F0C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82631F10: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82631F14: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82631F18: 4BE34F09  bl 0x82466e20
	ctx.lr = 0x82631F1C;
	sub_82466E20(ctx, base);
	// 82631F1C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82631F20: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82631F24: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82631F28: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82631F30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82631F30 size=100
    let mut pc: u32 = 0x82631F30;
    'dispatch: loop {
        match pc {
            0x82631F30 => {
    //   block [0x82631F30..0x82631F94)
	// 82631F30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82631F34: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82631F38: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82631F3C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82631F40: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82631F44: 38AA7594  addi r5, r10, 0x7594
	ctx.r[5].s64 = ctx.r[10].s64 + 30100;
	// 82631F48: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82631F4C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82631F50: 388A3DB4  addi r4, r10, 0x3db4
	ctx.r[4].s64 = ctx.r[10].s64 + 15796;
	// 82631F54: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82631F58: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82631F5C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82631F60: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82631F64: 386A7474  addi r3, r10, 0x7474
	ctx.r[3].s64 = ctx.r[10].s64 + 29812;
	// 82631F68: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82631F6C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82631F70: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82631F74: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82631F78: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82631F7C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82631F80: 4BE34EA1  bl 0x82466e20
	ctx.lr = 0x82631F84;
	sub_82466E20(ctx, base);
	// 82631F84: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82631F88: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82631F8C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82631F90: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82631F98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82631F98 size=100
    let mut pc: u32 = 0x82631F98;
    'dispatch: loop {
        match pc {
            0x82631F98 => {
    //   block [0x82631F98..0x82631FFC)
	// 82631F98: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82631F9C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82631FA0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82631FA4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82631FA8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82631FAC: 38AA7144  addi r5, r10, 0x7144
	ctx.r[5].s64 = ctx.r[10].s64 + 28996;
	// 82631FB0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82631FB4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82631FB8: 388A3DC4  addi r4, r10, 0x3dc4
	ctx.r[4].s64 = ctx.r[10].s64 + 15812;
	// 82631FBC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82631FC0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82631FC4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82631FC8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82631FCC: 386A74A4  addi r3, r10, 0x74a4
	ctx.r[3].s64 = ctx.r[10].s64 + 29860;
	// 82631FD0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82631FD4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82631FD8: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82631FDC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82631FE0: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82631FE4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82631FE8: 4BE34E39  bl 0x82466e20
	ctx.lr = 0x82631FEC;
	sub_82466E20(ctx, base);
	// 82631FEC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82631FF0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82631FF4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82631FF8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82632000(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82632000 size=108
    let mut pc: u32 = 0x82632000;
    'dispatch: loop {
        match pc {
            0x82632000 => {
    //   block [0x82632000..0x8263206C)
	// 82632000: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82632004: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82632008: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263200C: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 82632010: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82632014: 38EBE660  addi r7, r11, -0x19a0
	ctx.r[7].s64 = ctx.r[11].s64 + -6560;
	// 82632018: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 8263201C: 388A3DD8  addi r4, r10, 0x3dd8
	ctx.r[4].s64 = ctx.r[10].s64 + 15832;
	// 82632020: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82632024: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82632028: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8263202C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82632030: 386A74D4  addi r3, r10, 0x74d4
	ctx.r[3].s64 = ctx.r[10].s64 + 29908;
	// 82632034: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82632038: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8263203C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82632040: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82632044: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82632048: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8263204C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82632050: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82632054: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82632058: 4BE34DC9  bl 0x82466e20
	ctx.lr = 0x8263205C;
	sub_82466E20(ctx, base);
	// 8263205C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82632060: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82632064: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82632068: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82632070(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82632070 size=112
    let mut pc: u32 = 0x82632070;
    'dispatch: loop {
        match pc {
            0x82632070 => {
    //   block [0x82632070..0x826320E0)
	// 82632070: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82632074: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82632078: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263207C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82632080: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 82632084: 38AA72C4  addi r5, r10, 0x72c4
	ctx.r[5].s64 = ctx.r[10].s64 + 29380;
	// 82632088: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8263208C: 390BE690  addi r8, r11, -0x1970
	ctx.r[8].s64 = ctx.r[11].s64 + -6512;
	// 82632090: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82632094: 388A3DF0  addi r4, r10, 0x3df0
	ctx.r[4].s64 = ctx.r[10].s64 + 15856;
	// 82632098: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8263209C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826320A0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826320A4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826320A8: 386A7504  addi r3, r10, 0x7504
	ctx.r[3].s64 = ctx.r[10].s64 + 29956;
	// 826320AC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826320B0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826320B4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826320B8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826320BC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826320C0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826320C4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826320C8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826320CC: 4BE34D55  bl 0x82466e20
	ctx.lr = 0x826320D0;
	sub_82466E20(ctx, base);
	// 826320D0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826320D4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826320D8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826320DC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826320E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826320E0 size=108
    let mut pc: u32 = 0x826320E0;
    'dispatch: loop {
        match pc {
            0x826320E0 => {
    //   block [0x826320E0..0x8263214C)
	// 826320E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826320E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826320E8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826320EC: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 826320F0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826320F4: 38EBE6A8  addi r7, r11, -0x1958
	ctx.r[7].s64 = ctx.r[11].s64 + -6488;
	// 826320F8: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826320FC: 388A3DFC  addi r4, r10, 0x3dfc
	ctx.r[4].s64 = ctx.r[10].s64 + 15868;
	// 82632100: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82632104: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82632108: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8263210C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82632110: 386A7534  addi r3, r10, 0x7534
	ctx.r[3].s64 = ctx.r[10].s64 + 30004;
	// 82632114: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82632118: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8263211C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82632120: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82632124: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82632128: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8263212C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82632130: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82632134: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82632138: 4BE34CE9  bl 0x82466e20
	ctx.lr = 0x8263213C;
	sub_82466E20(ctx, base);
	// 8263213C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82632140: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82632144: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82632148: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82632150(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82632150 size=28
    let mut pc: u32 = 0x82632150;
    'dispatch: loop {
        match pc {
            0x82632150 => {
    //   block [0x82632150..0x8263216C)
	// 82632150: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 82632154: 3D408277  lis r10, -0x7d89
	ctx.r[10].s64 = -2106130432;
	// 82632158: 394A2278  addi r10, r10, 0x2278
	ctx.r[10].s64 = ctx.r[10].s64 + 8824;
	// 8263215C: 816BE6C0  lwz r11, -0x1940(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-6464 as u32) ) } as u64;
	// 82632160: 916A0068  stw r11, 0x68(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(104 as u32), ctx.r[11].u32 ) };
	// 82632164: 916A0080  stw r11, 0x80(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(128 as u32), ctx.r[11].u32 ) };
	// 82632168: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82632170(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82632170 size=108
    let mut pc: u32 = 0x82632170;
    'dispatch: loop {
        match pc {
            0x82632170 => {
    //   block [0x82632170..0x826321DC)
	// 82632170: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82632174: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82632178: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263217C: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 82632180: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82632184: 38EB2278  addi r7, r11, 0x2278
	ctx.r[7].s64 = ctx.r[11].s64 + 8824;
	// 82632188: 3900000D  li r8, 0xd
	ctx.r[8].s64 = 13;
	// 8263218C: 388A3E1C  addi r4, r10, 0x3e1c
	ctx.r[4].s64 = ctx.r[10].s64 + 15900;
	// 82632190: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82632194: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82632198: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8263219C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826321A0: 386A7564  addi r3, r10, 0x7564
	ctx.r[3].s64 = ctx.r[10].s64 + 30052;
	// 826321A4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826321A8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826321AC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826321B0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826321B4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826321B8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826321BC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826321C0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826321C4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826321C8: 4BE34C59  bl 0x82466e20
	ctx.lr = 0x826321CC;
	sub_82466E20(ctx, base);
	// 826321CC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826321D0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826321D4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826321D8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826321E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826321E0 size=116
    let mut pc: u32 = 0x826321E0;
    'dispatch: loop {
        match pc {
            0x826321E0 => {
    //   block [0x826321E0..0x82632254)
	// 826321E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826321E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826321E8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826321EC: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 826321F0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826321F4: 390BE6C8  addi r8, r11, -0x1938
	ctx.r[8].s64 = ctx.r[11].s64 + -6456;
	// 826321F8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826321FC: 392A4E98  addi r9, r10, 0x4e98
	ctx.r[9].s64 = ctx.r[10].s64 + 20120;
	// 82632200: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82632204: 38E00005  li r7, 5
	ctx.r[7].s64 = 5;
	// 82632208: 38AA72C4  addi r5, r10, 0x72c4
	ctx.r[5].s64 = ctx.r[10].s64 + 29380;
	// 8263220C: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82632210: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82632214: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82632218: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8263221C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82632220: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82632224: 3D608294  lis r11, -0x7d6c
	ctx.r[11].s64 = -2104229888;
	// 82632228: 388A3E30  addi r4, r10, 0x3e30
	ctx.r[4].s64 = ctx.r[10].s64 + 15920;
	// 8263222C: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82632230: 386B7594  addi r3, r11, 0x7594
	ctx.r[3].s64 = ctx.r[11].s64 + 30100;
	// 82632234: 39400002  li r10, 2
	ctx.r[10].s64 = 2;
	// 82632238: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8263223C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82632240: 4BE34BE1  bl 0x82466e20
	ctx.lr = 0x82632244;
	sub_82466E20(ctx, base);
	// 82632244: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82632248: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8263224C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82632250: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82632258(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82632258 size=112
    let mut pc: u32 = 0x82632258;
    'dispatch: loop {
        match pc {
            0x82632258 => {
    //   block [0x82632258..0x826322C8)
	// 82632258: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263225C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82632260: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82632264: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82632268: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8263226C: 38AA7264  addi r5, r10, 0x7264
	ctx.r[5].s64 = ctx.r[10].s64 + 29284;
	// 82632270: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82632274: 390BE740  addi r8, r11, -0x18c0
	ctx.r[8].s64 = ctx.r[11].s64 + -6336;
	// 82632278: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8263227C: 388A3E3C  addi r4, r10, 0x3e3c
	ctx.r[4].s64 = ctx.r[10].s64 + 15932;
	// 82632280: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82632284: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82632288: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8263228C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82632290: 386A75C4  addi r3, r10, 0x75c4
	ctx.r[3].s64 = ctx.r[10].s64 + 30148;
	// 82632294: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82632298: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8263229C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826322A0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826322A4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826322A8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826322AC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826322B0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826322B4: 4BE34B6D  bl 0x82466e20
	ctx.lr = 0x826322B8;
	sub_82466E20(ctx, base);
	// 826322B8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826322BC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826322C0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826322C4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826322C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826322C8 size=108
    let mut pc: u32 = 0x826322C8;
    'dispatch: loop {
        match pc {
            0x826322C8 => {
    //   block [0x826322C8..0x82632334)
	// 826322C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826322CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826322D0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826322D4: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 826322D8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826322DC: 38EBE758  addi r7, r11, -0x18a8
	ctx.r[7].s64 = ctx.r[11].s64 + -6312;
	// 826322E0: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826322E4: 388A3E98  addi r4, r10, 0x3e98
	ctx.r[4].s64 = ctx.r[10].s64 + 16024;
	// 826322E8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826322EC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826322F0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826322F4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826322F8: 386A75F4  addi r3, r10, 0x75f4
	ctx.r[3].s64 = ctx.r[10].s64 + 30196;
	// 826322FC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82632300: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82632304: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82632308: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8263230C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82632310: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82632314: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82632318: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8263231C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82632320: 4BE34B01  bl 0x82466e20
	ctx.lr = 0x82632324;
	sub_82466E20(ctx, base);
	// 82632324: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82632328: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8263232C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82632330: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82632338(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82632338 size=112
    let mut pc: u32 = 0x82632338;
    'dispatch: loop {
        match pc {
            0x82632338 => {
    //   block [0x82632338..0x826323A8)
	// 82632338: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263233C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82632340: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82632344: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82632348: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8263234C: 38AA7144  addi r5, r10, 0x7144
	ctx.r[5].s64 = ctx.r[10].s64 + 28996;
	// 82632350: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82632354: 390BE788  addi r8, r11, -0x1878
	ctx.r[8].s64 = ctx.r[11].s64 + -6264;
	// 82632358: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 8263235C: 388A3EAC  addi r4, r10, 0x3eac
	ctx.r[4].s64 = ctx.r[10].s64 + 16044;
	// 82632360: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82632364: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82632368: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8263236C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82632370: 386A7624  addi r3, r10, 0x7624
	ctx.r[3].s64 = ctx.r[10].s64 + 30244;
	// 82632374: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82632378: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8263237C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82632380: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82632384: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82632388: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8263238C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82632390: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82632394: 4BE34A8D  bl 0x82466e20
	ctx.lr = 0x82632398;
	sub_82466E20(ctx, base);
	// 82632398: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8263239C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826323A0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826323A4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826323A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826323A8 size=112
    let mut pc: u32 = 0x826323A8;
    'dispatch: loop {
        match pc {
            0x826323A8 => {
    //   block [0x826323A8..0x82632418)
	// 826323A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826323AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826323B0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826323B4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826323B8: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 826323BC: 38AA77A4  addi r5, r10, 0x77a4
	ctx.r[5].s64 = ctx.r[10].s64 + 30628;
	// 826323C0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826323C4: 390BE7B8  addi r8, r11, -0x1848
	ctx.r[8].s64 = ctx.r[11].s64 + -6216;
	// 826323C8: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826323CC: 388A3EBC  addi r4, r10, 0x3ebc
	ctx.r[4].s64 = ctx.r[10].s64 + 16060;
	// 826323D0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826323D4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826323D8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826323DC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826323E0: 386A7654  addi r3, r10, 0x7654
	ctx.r[3].s64 = ctx.r[10].s64 + 30292;
	// 826323E4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826323E8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826323EC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826323F0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826323F4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826323F8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826323FC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82632400: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82632404: 4BE34A1D  bl 0x82466e20
	ctx.lr = 0x82632408;
	sub_82466E20(ctx, base);
	// 82632408: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8263240C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82632410: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82632414: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82632418(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82632418 size=100
    let mut pc: u32 = 0x82632418;
    'dispatch: loop {
        match pc {
            0x82632418 => {
    //   block [0x82632418..0x8263247C)
	// 82632418: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263241C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82632420: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82632424: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82632428: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8263242C: 38AA7144  addi r5, r10, 0x7144
	ctx.r[5].s64 = ctx.r[10].s64 + 28996;
	// 82632430: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82632434: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82632438: 388A3F14  addi r4, r10, 0x3f14
	ctx.r[4].s64 = ctx.r[10].s64 + 16148;
	// 8263243C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82632440: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82632444: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82632448: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8263244C: 386A7684  addi r3, r10, 0x7684
	ctx.r[3].s64 = ctx.r[10].s64 + 30340;
	// 82632450: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82632454: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82632458: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8263245C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82632460: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82632464: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82632468: 4BE349B9  bl 0x82466e20
	ctx.lr = 0x8263246C;
	sub_82466E20(ctx, base);
	// 8263246C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82632470: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82632474: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82632478: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82632480(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82632480 size=112
    let mut pc: u32 = 0x82632480;
    'dispatch: loop {
        match pc {
            0x82632480 => {
    //   block [0x82632480..0x826324F0)
	// 82632480: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82632484: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82632488: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263248C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82632490: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 82632494: 38AA74A4  addi r5, r10, 0x74a4
	ctx.r[5].s64 = ctx.r[10].s64 + 29860;
	// 82632498: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8263249C: 390BE7E8  addi r8, r11, -0x1818
	ctx.r[8].s64 = ctx.r[11].s64 + -6168;
	// 826324A0: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 826324A4: 388A3F2C  addi r4, r10, 0x3f2c
	ctx.r[4].s64 = ctx.r[10].s64 + 16172;
	// 826324A8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826324AC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826324B0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826324B4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826324B8: 386A76B4  addi r3, r10, 0x76b4
	ctx.r[3].s64 = ctx.r[10].s64 + 30388;
	// 826324BC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826324C0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826324C4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826324C8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826324CC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826324D0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826324D4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826324D8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826324DC: 4BE34945  bl 0x82466e20
	ctx.lr = 0x826324E0;
	sub_82466E20(ctx, base);
	// 826324E0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826324E4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826324E8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826324EC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826324F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826324F0 size=112
    let mut pc: u32 = 0x826324F0;
    'dispatch: loop {
        match pc {
            0x826324F0 => {
    //   block [0x826324F0..0x82632560)
	// 826324F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826324F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826324F8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826324FC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82632500: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 82632504: 38AA74A4  addi r5, r10, 0x74a4
	ctx.r[5].s64 = ctx.r[10].s64 + 29860;
	// 82632508: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8263250C: 390BE830  addi r8, r11, -0x17d0
	ctx.r[8].s64 = ctx.r[11].s64 + -6096;
	// 82632510: 39200007  li r9, 7
	ctx.r[9].s64 = 7;
	// 82632514: 388A3F3C  addi r4, r10, 0x3f3c
	ctx.r[4].s64 = ctx.r[10].s64 + 16188;
	// 82632518: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8263251C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82632520: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82632524: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82632528: 386A76E4  addi r3, r10, 0x76e4
	ctx.r[3].s64 = ctx.r[10].s64 + 30436;
	// 8263252C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82632530: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82632534: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82632538: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8263253C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82632540: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82632544: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82632548: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8263254C: 4BE348D5  bl 0x82466e20
	ctx.lr = 0x82632550;
	sub_82466E20(ctx, base);
	// 82632550: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82632554: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82632558: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8263255C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82632560(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82632560 size=108
    let mut pc: u32 = 0x82632560;
    'dispatch: loop {
        match pc {
            0x82632560 => {
    //   block [0x82632560..0x826325CC)
	// 82632560: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82632564: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82632568: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263256C: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 82632570: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82632574: 38EBE8D8  addi r7, r11, -0x1728
	ctx.r[7].s64 = ctx.r[11].s64 + -5928;
	// 82632578: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 8263257C: 388A3F58  addi r4, r10, 0x3f58
	ctx.r[4].s64 = ctx.r[10].s64 + 16216;
	// 82632580: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82632584: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82632588: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8263258C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82632590: 386A7714  addi r3, r10, 0x7714
	ctx.r[3].s64 = ctx.r[10].s64 + 30484;
	// 82632594: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82632598: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8263259C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826325A0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826325A4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826325A8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826325AC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826325B0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826325B4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826325B8: 4BE34869  bl 0x82466e20
	ctx.lr = 0x826325BC;
	sub_82466E20(ctx, base);
	// 826325BC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826325C0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826325C4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826325C8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826325D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826325D0 size=112
    let mut pc: u32 = 0x826325D0;
    'dispatch: loop {
        match pc {
            0x826325D0 => {
    //   block [0x826325D0..0x82632640)
	// 826325D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826325D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826325D8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826325DC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826325E0: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 826325E4: 38AA72C4  addi r5, r10, 0x72c4
	ctx.r[5].s64 = ctx.r[10].s64 + 29380;
	// 826325E8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826325EC: 390BE920  addi r8, r11, -0x16e0
	ctx.r[8].s64 = ctx.r[11].s64 + -5856;
	// 826325F0: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 826325F4: 388A3F74  addi r4, r10, 0x3f74
	ctx.r[4].s64 = ctx.r[10].s64 + 16244;
	// 826325F8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826325FC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82632600: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82632604: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82632608: 386A7744  addi r3, r10, 0x7744
	ctx.r[3].s64 = ctx.r[10].s64 + 30532;
	// 8263260C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82632610: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82632614: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82632618: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8263261C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82632620: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82632624: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82632628: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8263262C: 4BE347F5  bl 0x82466e20
	ctx.lr = 0x82632630;
	sub_82466E20(ctx, base);
	// 82632630: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82632634: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82632638: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8263263C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


