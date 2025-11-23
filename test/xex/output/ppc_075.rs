pub fn sub_825EBDB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825EBDB8 size=108
    let mut pc: u32 = 0x825EBDB8;
    'dispatch: loop {
        match pc {
            0x825EBDB8 => {
    //   block [0x825EBDB8..0x825EBE24)
	// 825EBDB8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825EBDBC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825EBDC0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825EBDC4: 3D608273  lis r11, -0x7d8d
	ctx.r[11].s64 = -2106392576;
	// 825EBDC8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825EBDCC: 38EB5C30  addi r7, r11, 0x5c30
	ctx.r[7].s64 = ctx.r[11].s64 + 23600;
	// 825EBDD0: 39000006  li r8, 6
	ctx.r[8].s64 = 6;
	// 825EBDD4: 388A2C44  addi r4, r10, 0x2c44
	ctx.r[4].s64 = ctx.r[10].s64 + 11332;
	// 825EBDD8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825EBDDC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825EBDE0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 825EBDE4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825EBDE8: 386A9C84  addi r3, r10, -0x637c
	ctx.r[3].s64 = ctx.r[10].s64 + -25468;
	// 825EBDEC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 825EBDF0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825EBDF4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825EBDF8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825EBDFC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825EBE00: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825EBE04: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825EBE08: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825EBE0C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825EBE10: 4BE7B011  bl 0x82466e20
	ctx.lr = 0x825EBE14;
	sub_82466E20(ctx, base);
	// 825EBE14: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825EBE18: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825EBE1C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825EBE20: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825EBE28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825EBE28 size=112
    let mut pc: u32 = 0x825EBE28;
    'dispatch: loop {
        match pc {
            0x825EBE28 => {
    //   block [0x825EBE28..0x825EBE98)
	// 825EBE28: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825EBE2C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825EBE30: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825EBE34: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825EBE38: 3D608273  lis r11, -0x7d8d
	ctx.r[11].s64 = -2106392576;
	// 825EBE3C: 38AA9BC4  addi r5, r10, -0x643c
	ctx.r[5].s64 = ctx.r[10].s64 + -25660;
	// 825EBE40: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825EBE44: 390B5CC0  addi r8, r11, 0x5cc0
	ctx.r[8].s64 = ctx.r[11].s64 + 23744;
	// 825EBE48: 3920000A  li r9, 0xa
	ctx.r[9].s64 = 10;
	// 825EBE4C: 388A2C74  addi r4, r10, 0x2c74
	ctx.r[4].s64 = ctx.r[10].s64 + 11380;
	// 825EBE50: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825EBE54: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825EBE58: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825EBE5C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825EBE60: 386A9CB4  addi r3, r10, -0x634c
	ctx.r[3].s64 = ctx.r[10].s64 + -25420;
	// 825EBE64: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 825EBE68: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825EBE6C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825EBE70: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825EBE74: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825EBE78: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825EBE7C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825EBE80: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825EBE84: 4BE7AF9D  bl 0x82466e20
	ctx.lr = 0x825EBE88;
	sub_82466E20(ctx, base);
	// 825EBE88: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825EBE8C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825EBE90: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825EBE94: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825EBE98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825EBE98 size=108
    let mut pc: u32 = 0x825EBE98;
    'dispatch: loop {
        match pc {
            0x825EBE98 => {
    //   block [0x825EBE98..0x825EBF04)
	// 825EBE98: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825EBE9C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825EBEA0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825EBEA4: 3D608273  lis r11, -0x7d8d
	ctx.r[11].s64 = -2106392576;
	// 825EBEA8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825EBEAC: 38EB5DB0  addi r7, r11, 0x5db0
	ctx.r[7].s64 = ctx.r[11].s64 + 23984;
	// 825EBEB0: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 825EBEB4: 388A2C90  addi r4, r10, 0x2c90
	ctx.r[4].s64 = ctx.r[10].s64 + 11408;
	// 825EBEB8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825EBEBC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825EBEC0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 825EBEC4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825EBEC8: 386A9CE4  addi r3, r10, -0x631c
	ctx.r[3].s64 = ctx.r[10].s64 + -25372;
	// 825EBECC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 825EBED0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825EBED4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825EBED8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825EBEDC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825EBEE0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825EBEE4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825EBEE8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825EBEEC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825EBEF0: 4BE7AF31  bl 0x82466e20
	ctx.lr = 0x825EBEF4;
	sub_82466E20(ctx, base);
	// 825EBEF4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825EBEF8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825EBEFC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825EBF00: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825EBF08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825EBF08 size=108
    let mut pc: u32 = 0x825EBF08;
    'dispatch: loop {
        match pc {
            0x825EBF08 => {
    //   block [0x825EBF08..0x825EBF74)
	// 825EBF08: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825EBF0C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825EBF10: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825EBF14: 3D608273  lis r11, -0x7d8d
	ctx.r[11].s64 = -2106392576;
	// 825EBF18: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825EBF1C: 38EB5DC8  addi r7, r11, 0x5dc8
	ctx.r[7].s64 = ctx.r[11].s64 + 24008;
	// 825EBF20: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 825EBF24: 388A2CA8  addi r4, r10, 0x2ca8
	ctx.r[4].s64 = ctx.r[10].s64 + 11432;
	// 825EBF28: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825EBF2C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825EBF30: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 825EBF34: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825EBF38: 386A9D14  addi r3, r10, -0x62ec
	ctx.r[3].s64 = ctx.r[10].s64 + -25324;
	// 825EBF3C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 825EBF40: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825EBF44: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825EBF48: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825EBF4C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825EBF50: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825EBF54: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825EBF58: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825EBF5C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825EBF60: 4BE7AEC1  bl 0x82466e20
	ctx.lr = 0x825EBF64;
	sub_82466E20(ctx, base);
	// 825EBF64: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825EBF68: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825EBF6C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825EBF70: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825EBF78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825EBF78 size=116
    let mut pc: u32 = 0x825EBF78;
    'dispatch: loop {
        match pc {
            0x825EBF78 => {
    //   block [0x825EBF78..0x825EBFEC)
	// 825EBF78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825EBF7C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825EBF80: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825EBF84: 3D608273  lis r11, -0x7d8d
	ctx.r[11].s64 = -2106392576;
	// 825EBF88: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825EBF8C: 390B5E2C  addi r8, r11, 0x5e2c
	ctx.r[8].s64 = ctx.r[11].s64 + 24108;
	// 825EBF90: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825EBF94: 392A0E2C  addi r9, r10, 0xe2c
	ctx.r[9].s64 = ctx.r[10].s64 + 3628;
	// 825EBF98: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825EBF9C: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
	// 825EBFA0: 38AAAF74  addi r5, r10, -0x508c
	ctx.r[5].s64 = ctx.r[10].s64 + -20620;
	// 825EBFA4: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825EBFA8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825EBFAC: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825EBFB0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825EBFB4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825EBFB8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825EBFBC: 3D608293  lis r11, -0x7d6d
	ctx.r[11].s64 = -2104295424;
	// 825EBFC0: 388A2CB8  addi r4, r10, 0x2cb8
	ctx.r[4].s64 = ctx.r[10].s64 + 11448;
	// 825EBFC4: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 825EBFC8: 386B9D44  addi r3, r11, -0x62bc
	ctx.r[3].s64 = ctx.r[11].s64 + -25276;
	// 825EBFCC: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 825EBFD0: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825EBFD4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825EBFD8: 4BE7AE49  bl 0x82466e20
	ctx.lr = 0x825EBFDC;
	sub_82466E20(ctx, base);
	// 825EBFDC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825EBFE0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825EBFE4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825EBFE8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825EBFF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825EBFF0 size=108
    let mut pc: u32 = 0x825EBFF0;
    'dispatch: loop {
        match pc {
            0x825EBFF0 => {
    //   block [0x825EBFF0..0x825EC05C)
	// 825EBFF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825EBFF4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825EBFF8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825EBFFC: 3D608273  lis r11, -0x7d8d
	ctx.r[11].s64 = -2106392576;
	// 825EC000: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825EC004: 38EB5E48  addi r7, r11, 0x5e48
	ctx.r[7].s64 = ctx.r[11].s64 + 24136;
	// 825EC008: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 825EC00C: 388A2CCC  addi r4, r10, 0x2ccc
	ctx.r[4].s64 = ctx.r[10].s64 + 11468;
	// 825EC010: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825EC014: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825EC018: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 825EC01C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825EC020: 386A9D74  addi r3, r10, -0x628c
	ctx.r[3].s64 = ctx.r[10].s64 + -25228;
	// 825EC024: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 825EC028: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825EC02C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825EC030: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825EC034: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825EC038: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825EC03C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825EC040: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825EC044: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825EC048: 4BE7ADD9  bl 0x82466e20
	ctx.lr = 0x825EC04C;
	sub_82466E20(ctx, base);
	// 825EC04C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825EC050: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825EC054: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825EC058: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825EC060(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825EC060 size=108
    let mut pc: u32 = 0x825EC060;
    'dispatch: loop {
        match pc {
            0x825EC060 => {
    //   block [0x825EC060..0x825EC0CC)
	// 825EC060: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825EC064: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825EC068: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825EC06C: 3D608273  lis r11, -0x7d8d
	ctx.r[11].s64 = -2106392576;
	// 825EC070: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825EC074: 38EB5E90  addi r7, r11, 0x5e90
	ctx.r[7].s64 = ctx.r[11].s64 + 24208;
	// 825EC078: 39000006  li r8, 6
	ctx.r[8].s64 = 6;
	// 825EC07C: 388A2CF0  addi r4, r10, 0x2cf0
	ctx.r[4].s64 = ctx.r[10].s64 + 11504;
	// 825EC080: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825EC084: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825EC088: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 825EC08C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825EC090: 386A9DA4  addi r3, r10, -0x625c
	ctx.r[3].s64 = ctx.r[10].s64 + -25180;
	// 825EC094: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 825EC098: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825EC09C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825EC0A0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825EC0A4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825EC0A8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825EC0AC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825EC0B0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825EC0B4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825EC0B8: 4BE7AD69  bl 0x82466e20
	ctx.lr = 0x825EC0BC;
	sub_82466E20(ctx, base);
	// 825EC0BC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825EC0C0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825EC0C4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825EC0C8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825EC0D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825EC0D0 size=108
    let mut pc: u32 = 0x825EC0D0;
    'dispatch: loop {
        match pc {
            0x825EC0D0 => {
    //   block [0x825EC0D0..0x825EC13C)
	// 825EC0D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825EC0D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825EC0D8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825EC0DC: 3D608273  lis r11, -0x7d8d
	ctx.r[11].s64 = -2106392576;
	// 825EC0E0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825EC0E4: 38EB5F20  addi r7, r11, 0x5f20
	ctx.r[7].s64 = ctx.r[11].s64 + 24352;
	// 825EC0E8: 39000006  li r8, 6
	ctx.r[8].s64 = 6;
	// 825EC0EC: 388A2D14  addi r4, r10, 0x2d14
	ctx.r[4].s64 = ctx.r[10].s64 + 11540;
	// 825EC0F0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825EC0F4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825EC0F8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 825EC0FC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825EC100: 386A9DD4  addi r3, r10, -0x622c
	ctx.r[3].s64 = ctx.r[10].s64 + -25132;
	// 825EC104: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 825EC108: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825EC10C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825EC110: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825EC114: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825EC118: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825EC11C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825EC120: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825EC124: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825EC128: 4BE7ACF9  bl 0x82466e20
	ctx.lr = 0x825EC12C;
	sub_82466E20(ctx, base);
	// 825EC12C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825EC130: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825EC134: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825EC138: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825EC140(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825EC140 size=100
    let mut pc: u32 = 0x825EC140;
    'dispatch: loop {
        match pc {
            0x825EC140 => {
    //   block [0x825EC140..0x825EC1A4)
	// 825EC140: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825EC144: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825EC148: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825EC14C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825EC150: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825EC154: 38AAAF74  addi r5, r10, -0x508c
	ctx.r[5].s64 = ctx.r[10].s64 + -20620;
	// 825EC158: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825EC15C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825EC160: 388A2D2C  addi r4, r10, 0x2d2c
	ctx.r[4].s64 = ctx.r[10].s64 + 11564;
	// 825EC164: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825EC168: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825EC16C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825EC170: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825EC174: 386A9E04  addi r3, r10, -0x61fc
	ctx.r[3].s64 = ctx.r[10].s64 + -25084;
	// 825EC178: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825EC17C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825EC180: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 825EC184: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825EC188: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 825EC18C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825EC190: 4BE7AC91  bl 0x82466e20
	ctx.lr = 0x825EC194;
	sub_82466E20(ctx, base);
	// 825EC194: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825EC198: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825EC19C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825EC1A0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825EC1A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825EC1A8 size=112
    let mut pc: u32 = 0x825EC1A8;
    'dispatch: loop {
        match pc {
            0x825EC1A8 => {
    //   block [0x825EC1A8..0x825EC218)
	// 825EC1A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825EC1AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825EC1B0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825EC1B4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825EC1B8: 3D608273  lis r11, -0x7d8d
	ctx.r[11].s64 = -2106392576;
	// 825EC1BC: 38AA9E04  addi r5, r10, -0x61fc
	ctx.r[5].s64 = ctx.r[10].s64 + -25084;
	// 825EC1C0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825EC1C4: 390B5FB0  addi r8, r11, 0x5fb0
	ctx.r[8].s64 = ctx.r[11].s64 + 24496;
	// 825EC1C8: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 825EC1CC: 388A2D48  addi r4, r10, 0x2d48
	ctx.r[4].s64 = ctx.r[10].s64 + 11592;
	// 825EC1D0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825EC1D4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825EC1D8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825EC1DC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825EC1E0: 386A9E34  addi r3, r10, -0x61cc
	ctx.r[3].s64 = ctx.r[10].s64 + -25036;
	// 825EC1E4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 825EC1E8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825EC1EC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825EC1F0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825EC1F4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825EC1F8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825EC1FC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825EC200: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825EC204: 4BE7AC1D  bl 0x82466e20
	ctx.lr = 0x825EC208;
	sub_82466E20(ctx, base);
	// 825EC208: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825EC20C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825EC210: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825EC214: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825EC218(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825EC218 size=108
    let mut pc: u32 = 0x825EC218;
    'dispatch: loop {
        match pc {
            0x825EC218 => {
    //   block [0x825EC218..0x825EC284)
	// 825EC218: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825EC21C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825EC220: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825EC224: 3D608273  lis r11, -0x7d8d
	ctx.r[11].s64 = -2106392576;
	// 825EC228: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825EC22C: 38EB6010  addi r7, r11, 0x6010
	ctx.r[7].s64 = ctx.r[11].s64 + 24592;
	// 825EC230: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 825EC234: 388A2D68  addi r4, r10, 0x2d68
	ctx.r[4].s64 = ctx.r[10].s64 + 11624;
	// 825EC238: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825EC23C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825EC240: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 825EC244: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825EC248: 386A9E64  addi r3, r10, -0x619c
	ctx.r[3].s64 = ctx.r[10].s64 + -24988;
	// 825EC24C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 825EC250: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825EC254: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825EC258: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825EC25C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825EC260: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825EC264: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825EC268: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825EC26C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825EC270: 4BE7ABB1  bl 0x82466e20
	ctx.lr = 0x825EC274;
	sub_82466E20(ctx, base);
	// 825EC274: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825EC278: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825EC27C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825EC280: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825EC288(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825EC288 size=108
    let mut pc: u32 = 0x825EC288;
    'dispatch: loop {
        match pc {
            0x825EC288 => {
    //   block [0x825EC288..0x825EC2F4)
	// 825EC288: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825EC28C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825EC290: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825EC294: 3D608273  lis r11, -0x7d8d
	ctx.r[11].s64 = -2106392576;
	// 825EC298: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825EC29C: 38EB6040  addi r7, r11, 0x6040
	ctx.r[7].s64 = ctx.r[11].s64 + 24640;
	// 825EC2A0: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 825EC2A4: 388A2D70  addi r4, r10, 0x2d70
	ctx.r[4].s64 = ctx.r[10].s64 + 11632;
	// 825EC2A8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825EC2AC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825EC2B0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 825EC2B4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825EC2B8: 386A9E94  addi r3, r10, -0x616c
	ctx.r[3].s64 = ctx.r[10].s64 + -24940;
	// 825EC2BC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 825EC2C0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825EC2C4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825EC2C8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825EC2CC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825EC2D0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825EC2D4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825EC2D8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825EC2DC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825EC2E0: 4BE7AB41  bl 0x82466e20
	ctx.lr = 0x825EC2E4;
	sub_82466E20(ctx, base);
	// 825EC2E4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825EC2E8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825EC2EC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825EC2F0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825EC2F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825EC2F8 size=108
    let mut pc: u32 = 0x825EC2F8;
    'dispatch: loop {
        match pc {
            0x825EC2F8 => {
    //   block [0x825EC2F8..0x825EC364)
	// 825EC2F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825EC2FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825EC300: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825EC304: 3D608273  lis r11, -0x7d8d
	ctx.r[11].s64 = -2106392576;
	// 825EC308: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825EC30C: 38EB60A0  addi r7, r11, 0x60a0
	ctx.r[7].s64 = ctx.r[11].s64 + 24736;
	// 825EC310: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 825EC314: 388A2D84  addi r4, r10, 0x2d84
	ctx.r[4].s64 = ctx.r[10].s64 + 11652;
	// 825EC318: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825EC31C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825EC320: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 825EC324: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825EC328: 386A9EC4  addi r3, r10, -0x613c
	ctx.r[3].s64 = ctx.r[10].s64 + -24892;
	// 825EC32C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 825EC330: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825EC334: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825EC338: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825EC33C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825EC340: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825EC344: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825EC348: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825EC34C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825EC350: 4BE7AAD1  bl 0x82466e20
	ctx.lr = 0x825EC354;
	sub_82466E20(ctx, base);
	// 825EC354: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825EC358: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825EC35C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825EC360: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825EC368(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825EC368 size=24
    let mut pc: u32 = 0x825EC368;
    'dispatch: loop {
        match pc {
            0x825EC368 => {
    //   block [0x825EC368..0x825EC380)
	// 825EC368: 3D608273  lis r11, -0x7d8d
	ctx.r[11].s64 = -2106392576;
	// 825EC36C: 3D408274  lis r10, -0x7d8c
	ctx.r[10].s64 = -2106327040;
	// 825EC370: 394ADC88  addi r10, r10, -0x2378
	ctx.r[10].s64 = ctx.r[10].s64 + -9080;
	// 825EC374: 816B5E44  lwz r11, 0x5e44(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24132 as u32) ) } as u64;
	// 825EC378: 916A0128  stw r11, 0x128(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(296 as u32), ctx.r[11].u32 ) };
	// 825EC37C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825EC380(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825EC380 size=116
    let mut pc: u32 = 0x825EC380;
    'dispatch: loop {
        match pc {
            0x825EC380 => {
    //   block [0x825EC380..0x825EC3F4)
	// 825EC380: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825EC384: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825EC388: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825EC38C: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825EC390: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825EC394: 390BDC88  addi r8, r11, -0x2378
	ctx.r[8].s64 = ctx.r[11].s64 + -9080;
	// 825EC398: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825EC39C: 392A0E60  addi r9, r10, 0xe60
	ctx.r[9].s64 = ctx.r[10].s64 + 3680;
	// 825EC3A0: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825EC3A4: 38E0000D  li r7, 0xd
	ctx.r[7].s64 = 13;
	// 825EC3A8: 38AAAF74  addi r5, r10, -0x508c
	ctx.r[5].s64 = ctx.r[10].s64 + -20620;
	// 825EC3AC: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825EC3B0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825EC3B4: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825EC3B8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825EC3BC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825EC3C0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825EC3C4: 3D608293  lis r11, -0x7d6d
	ctx.r[11].s64 = -2104295424;
	// 825EC3C8: 388A2D90  addi r4, r10, 0x2d90
	ctx.r[4].s64 = ctx.r[10].s64 + 11664;
	// 825EC3CC: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 825EC3D0: 386B9EF4  addi r3, r11, -0x610c
	ctx.r[3].s64 = ctx.r[11].s64 + -24844;
	// 825EC3D4: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 825EC3D8: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825EC3DC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825EC3E0: 4BE7AA41  bl 0x82466e20
	ctx.lr = 0x825EC3E4;
	sub_82466E20(ctx, base);
	// 825EC3E4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825EC3E8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825EC3EC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825EC3F0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825EC3F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825EC3F8 size=112
    let mut pc: u32 = 0x825EC3F8;
    'dispatch: loop {
        match pc {
            0x825EC3F8 => {
    //   block [0x825EC3F8..0x825EC468)
	// 825EC3F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825EC3FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825EC400: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825EC404: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825EC408: 3D608273  lis r11, -0x7d8d
	ctx.r[11].s64 = -2106392576;
	// 825EC40C: 38AAAF74  addi r5, r10, -0x508c
	ctx.r[5].s64 = ctx.r[10].s64 + -20620;
	// 825EC410: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825EC414: 390B6100  addi r8, r11, 0x6100
	ctx.r[8].s64 = ctx.r[11].s64 + 24832;
	// 825EC418: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 825EC41C: 388A2DA4  addi r4, r10, 0x2da4
	ctx.r[4].s64 = ctx.r[10].s64 + 11684;
	// 825EC420: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825EC424: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825EC428: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825EC42C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825EC430: 386A9F24  addi r3, r10, -0x60dc
	ctx.r[3].s64 = ctx.r[10].s64 + -24796;
	// 825EC434: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 825EC438: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825EC43C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825EC440: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825EC444: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825EC448: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825EC44C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825EC450: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825EC454: 4BE7A9CD  bl 0x82466e20
	ctx.lr = 0x825EC458;
	sub_82466E20(ctx, base);
	// 825EC458: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825EC45C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825EC460: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825EC464: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825EC468(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825EC468 size=112
    let mut pc: u32 = 0x825EC468;
    'dispatch: loop {
        match pc {
            0x825EC468 => {
    //   block [0x825EC468..0x825EC4D8)
	// 825EC468: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825EC46C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825EC470: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825EC474: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825EC478: 3D608273  lis r11, -0x7d8d
	ctx.r[11].s64 = -2106392576;
	// 825EC47C: 38AAAF74  addi r5, r10, -0x508c
	ctx.r[5].s64 = ctx.r[10].s64 + -20620;
	// 825EC480: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825EC484: 390B6148  addi r8, r11, 0x6148
	ctx.r[8].s64 = ctx.r[11].s64 + 24904;
	// 825EC488: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 825EC48C: 388A2DBC  addi r4, r10, 0x2dbc
	ctx.r[4].s64 = ctx.r[10].s64 + 11708;
	// 825EC490: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825EC494: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825EC498: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825EC49C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825EC4A0: 386A9F54  addi r3, r10, -0x60ac
	ctx.r[3].s64 = ctx.r[10].s64 + -24748;
	// 825EC4A4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 825EC4A8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825EC4AC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825EC4B0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825EC4B4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825EC4B8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825EC4BC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825EC4C0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825EC4C4: 4BE7A95D  bl 0x82466e20
	ctx.lr = 0x825EC4C8;
	sub_82466E20(ctx, base);
	// 825EC4C8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825EC4CC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825EC4D0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825EC4D4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825EC4D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825EC4D8 size=112
    let mut pc: u32 = 0x825EC4D8;
    'dispatch: loop {
        match pc {
            0x825EC4D8 => {
    //   block [0x825EC4D8..0x825EC548)
	// 825EC4D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825EC4DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825EC4E0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825EC4E4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825EC4E8: 3D608273  lis r11, -0x7d8d
	ctx.r[11].s64 = -2106392576;
	// 825EC4EC: 38AAA134  addi r5, r10, -0x5ecc
	ctx.r[5].s64 = ctx.r[10].s64 + -24268;
	// 825EC4F0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825EC4F4: 390B6190  addi r8, r11, 0x6190
	ctx.r[8].s64 = ctx.r[11].s64 + 24976;
	// 825EC4F8: 39200009  li r9, 9
	ctx.r[9].s64 = 9;
	// 825EC4FC: 388A2DCC  addi r4, r10, 0x2dcc
	ctx.r[4].s64 = ctx.r[10].s64 + 11724;
	// 825EC500: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825EC504: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825EC508: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825EC50C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825EC510: 386A9F84  addi r3, r10, -0x607c
	ctx.r[3].s64 = ctx.r[10].s64 + -24700;
	// 825EC514: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 825EC518: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825EC51C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825EC520: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825EC524: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825EC528: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825EC52C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825EC530: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825EC534: 4BE7A8ED  bl 0x82466e20
	ctx.lr = 0x825EC538;
	sub_82466E20(ctx, base);
	// 825EC538: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825EC53C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825EC540: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825EC544: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825EC548(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825EC548 size=108
    let mut pc: u32 = 0x825EC548;
    'dispatch: loop {
        match pc {
            0x825EC548 => {
    //   block [0x825EC548..0x825EC5B4)
	// 825EC548: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825EC54C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825EC550: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825EC554: 3D608273  lis r11, -0x7d8d
	ctx.r[11].s64 = -2106392576;
	// 825EC558: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825EC55C: 38EB6268  addi r7, r11, 0x6268
	ctx.r[7].s64 = ctx.r[11].s64 + 25192;
	// 825EC560: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 825EC564: 388A2DD8  addi r4, r10, 0x2dd8
	ctx.r[4].s64 = ctx.r[10].s64 + 11736;
	// 825EC568: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825EC56C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825EC570: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 825EC574: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825EC578: 386A9FB4  addi r3, r10, -0x604c
	ctx.r[3].s64 = ctx.r[10].s64 + -24652;
	// 825EC57C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 825EC580: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825EC584: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825EC588: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825EC58C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825EC590: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825EC594: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825EC598: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825EC59C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825EC5A0: 4BE7A881  bl 0x82466e20
	ctx.lr = 0x825EC5A4;
	sub_82466E20(ctx, base);
	// 825EC5A4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825EC5A8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825EC5AC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825EC5B0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825EC5B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825EC5B8 size=108
    let mut pc: u32 = 0x825EC5B8;
    'dispatch: loop {
        match pc {
            0x825EC5B8 => {
    //   block [0x825EC5B8..0x825EC624)
	// 825EC5B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825EC5BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825EC5C0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825EC5C4: 3D608273  lis r11, -0x7d8d
	ctx.r[11].s64 = -2106392576;
	// 825EC5C8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825EC5CC: 38EB6298  addi r7, r11, 0x6298
	ctx.r[7].s64 = ctx.r[11].s64 + 25240;
	// 825EC5D0: 39000007  li r8, 7
	ctx.r[8].s64 = 7;
	// 825EC5D4: 388A2DF0  addi r4, r10, 0x2df0
	ctx.r[4].s64 = ctx.r[10].s64 + 11760;
	// 825EC5D8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825EC5DC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825EC5E0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 825EC5E4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825EC5E8: 386A9FE4  addi r3, r10, -0x601c
	ctx.r[3].s64 = ctx.r[10].s64 + -24604;
	// 825EC5EC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 825EC5F0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825EC5F4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825EC5F8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825EC5FC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825EC600: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825EC604: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825EC608: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825EC60C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825EC610: 4BE7A811  bl 0x82466e20
	ctx.lr = 0x825EC614;
	sub_82466E20(ctx, base);
	// 825EC614: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825EC618: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825EC61C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825EC620: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825EC628(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825EC628 size=112
    let mut pc: u32 = 0x825EC628;
    'dispatch: loop {
        match pc {
            0x825EC628 => {
    //   block [0x825EC628..0x825EC698)
	// 825EC628: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825EC62C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825EC630: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825EC634: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 825EC638: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825EC63C: 392B0E94  addi r9, r11, 0xe94
	ctx.r[9].s64 = ctx.r[11].s64 + 3732;
	// 825EC640: 38C0000E  li r6, 0xe
	ctx.r[6].s64 = 14;
	// 825EC644: 38E90014  addi r7, r9, 0x14
	ctx.r[7].s64 = ctx.r[9].s64 + 20;
	// 825EC648: 3D608273  lis r11, -0x7d8d
	ctx.r[11].s64 = -2106392576;
	// 825EC64C: 388A2E00  addi r4, r10, 0x2e00
	ctx.r[4].s64 = ctx.r[10].s64 + 11776;
	// 825EC650: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825EC654: 396B6348  addi r11, r11, 0x6348
	ctx.r[11].s64 = ctx.r[11].s64 + 25416;
	// 825EC658: 90C1005C  stw r6, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[6].u32 ) };
	// 825EC65C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825EC660: 90E10064  stw r7, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[7].u32 ) };
	// 825EC664: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825EC668: 386AA014  addi r3, r10, -0x5fec
	ctx.r[3].s64 = ctx.r[10].s64 + -24556;
	// 825EC66C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 825EC670: 91010074  stw r8, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[8].u32 ) };
	// 825EC674: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825EC678: 9101006C  stw r8, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[8].u32 ) };
	// 825EC67C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825EC680: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 825EC684: 4BE7A79D  bl 0x82466e20
	ctx.lr = 0x825EC688;
	sub_82466E20(ctx, base);
	// 825EC688: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825EC68C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825EC690: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825EC694: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825EC698(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825EC698 size=112
    let mut pc: u32 = 0x825EC698;
    'dispatch: loop {
        match pc {
            0x825EC698 => {
    //   block [0x825EC698..0x825EC708)
	// 825EC698: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825EC69C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825EC6A0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825EC6A4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825EC6A8: 3D608273  lis r11, -0x7d8d
	ctx.r[11].s64 = -2106392576;
	// 825EC6AC: 38AAAF74  addi r5, r10, -0x508c
	ctx.r[5].s64 = ctx.r[10].s64 + -20620;
	// 825EC6B0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825EC6B4: 390B6498  addi r8, r11, 0x6498
	ctx.r[8].s64 = ctx.r[11].s64 + 25752;
	// 825EC6B8: 39200007  li r9, 7
	ctx.r[9].s64 = 7;
	// 825EC6BC: 388A2E18  addi r4, r10, 0x2e18
	ctx.r[4].s64 = ctx.r[10].s64 + 11800;
	// 825EC6C0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825EC6C4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825EC6C8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825EC6CC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825EC6D0: 386AA044  addi r3, r10, -0x5fbc
	ctx.r[3].s64 = ctx.r[10].s64 + -24508;
	// 825EC6D4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 825EC6D8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825EC6DC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825EC6E0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825EC6E4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825EC6E8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825EC6EC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825EC6F0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825EC6F4: 4BE7A72D  bl 0x82466e20
	ctx.lr = 0x825EC6F8;
	sub_82466E20(ctx, base);
	// 825EC6F8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825EC6FC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825EC700: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825EC704: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825EC708(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825EC708 size=112
    let mut pc: u32 = 0x825EC708;
    'dispatch: loop {
        match pc {
            0x825EC708 => {
    //   block [0x825EC708..0x825EC778)
	// 825EC708: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825EC70C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825EC710: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825EC714: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825EC718: 3D608273  lis r11, -0x7d8d
	ctx.r[11].s64 = -2106392576;
	// 825EC71C: 38AAAF74  addi r5, r10, -0x508c
	ctx.r[5].s64 = ctx.r[10].s64 + -20620;
	// 825EC720: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825EC724: 390B6540  addi r8, r11, 0x6540
	ctx.r[8].s64 = ctx.r[11].s64 + 25920;
	// 825EC728: 39200006  li r9, 6
	ctx.r[9].s64 = 6;
	// 825EC72C: 388A2E30  addi r4, r10, 0x2e30
	ctx.r[4].s64 = ctx.r[10].s64 + 11824;
	// 825EC730: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825EC734: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825EC738: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825EC73C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825EC740: 386AA074  addi r3, r10, -0x5f8c
	ctx.r[3].s64 = ctx.r[10].s64 + -24460;
	// 825EC744: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 825EC748: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825EC74C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825EC750: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825EC754: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825EC758: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825EC75C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825EC760: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825EC764: 4BE7A6BD  bl 0x82466e20
	ctx.lr = 0x825EC768;
	sub_82466E20(ctx, base);
	// 825EC768: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825EC76C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825EC770: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825EC774: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825EC778(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825EC778 size=112
    let mut pc: u32 = 0x825EC778;
    'dispatch: loop {
        match pc {
            0x825EC778 => {
    //   block [0x825EC778..0x825EC7E8)
	// 825EC778: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825EC77C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825EC780: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825EC784: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825EC788: 3D608273  lis r11, -0x7d8d
	ctx.r[11].s64 = -2106392576;
	// 825EC78C: 38AAAF74  addi r5, r10, -0x508c
	ctx.r[5].s64 = ctx.r[10].s64 + -20620;
	// 825EC790: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825EC794: 390B65D0  addi r8, r11, 0x65d0
	ctx.r[8].s64 = ctx.r[11].s64 + 26064;
	// 825EC798: 39200005  li r9, 5
	ctx.r[9].s64 = 5;
	// 825EC79C: 388A2E44  addi r4, r10, 0x2e44
	ctx.r[4].s64 = ctx.r[10].s64 + 11844;
	// 825EC7A0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825EC7A4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825EC7A8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825EC7AC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825EC7B0: 386AA0A4  addi r3, r10, -0x5f5c
	ctx.r[3].s64 = ctx.r[10].s64 + -24412;
	// 825EC7B4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 825EC7B8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825EC7BC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825EC7C0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825EC7C4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825EC7C8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825EC7CC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825EC7D0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825EC7D4: 4BE7A64D  bl 0x82466e20
	ctx.lr = 0x825EC7D8;
	sub_82466E20(ctx, base);
	// 825EC7D8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825EC7DC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825EC7E0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825EC7E4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825EC7E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825EC7E8 size=108
    let mut pc: u32 = 0x825EC7E8;
    'dispatch: loop {
        match pc {
            0x825EC7E8 => {
    //   block [0x825EC7E8..0x825EC854)
	// 825EC7E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825EC7EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825EC7F0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825EC7F4: 3D608273  lis r11, -0x7d8d
	ctx.r[11].s64 = -2106392576;
	// 825EC7F8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825EC7FC: 38EB6648  addi r7, r11, 0x6648
	ctx.r[7].s64 = ctx.r[11].s64 + 26184;
	// 825EC800: 39000005  li r8, 5
	ctx.r[8].s64 = 5;
	// 825EC804: 388A2E58  addi r4, r10, 0x2e58
	ctx.r[4].s64 = ctx.r[10].s64 + 11864;
	// 825EC808: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825EC80C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825EC810: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 825EC814: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825EC818: 386AA0D4  addi r3, r10, -0x5f2c
	ctx.r[3].s64 = ctx.r[10].s64 + -24364;
	// 825EC81C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 825EC820: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825EC824: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825EC828: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825EC82C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825EC830: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825EC834: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825EC838: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825EC83C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825EC840: 4BE7A5E1  bl 0x82466e20
	ctx.lr = 0x825EC844;
	sub_82466E20(ctx, base);
	// 825EC844: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825EC848: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825EC84C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825EC850: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825EC858(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825EC858 size=112
    let mut pc: u32 = 0x825EC858;
    'dispatch: loop {
        match pc {
            0x825EC858 => {
    //   block [0x825EC858..0x825EC8C8)
	// 825EC858: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825EC85C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825EC860: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825EC864: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825EC868: 3D608273  lis r11, -0x7d8d
	ctx.r[11].s64 = -2106392576;
	// 825EC86C: 392A0EF0  addi r9, r10, 0xef0
	ctx.r[9].s64 = ctx.r[10].s64 + 3824;
	// 825EC870: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825EC874: 390B66C0  addi r8, r11, 0x66c0
	ctx.r[8].s64 = ctx.r[11].s64 + 26304;
	// 825EC878: 38E00002  li r7, 2
	ctx.r[7].s64 = 2;
	// 825EC87C: 388A2E64  addi r4, r10, 0x2e64
	ctx.r[4].s64 = ctx.r[10].s64 + 11876;
	// 825EC880: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825EC884: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825EC888: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825EC88C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825EC890: 386AA104  addi r3, r10, -0x5efc
	ctx.r[3].s64 = ctx.r[10].s64 + -24316;
	// 825EC894: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 825EC898: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 825EC89C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825EC8A0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825EC8A4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825EC8A8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825EC8AC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825EC8B0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825EC8B4: 4BE7A56D  bl 0x82466e20
	ctx.lr = 0x825EC8B8;
	sub_82466E20(ctx, base);
	// 825EC8B8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825EC8BC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825EC8C0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825EC8C4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825EC8C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825EC8C8 size=100
    let mut pc: u32 = 0x825EC8C8;
    'dispatch: loop {
        match pc {
            0x825EC8C8 => {
    //   block [0x825EC8C8..0x825EC92C)
	// 825EC8C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825EC8CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825EC8D0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825EC8D4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825EC8D8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825EC8DC: 38AAA8E4  addi r5, r10, -0x571c
	ctx.r[5].s64 = ctx.r[10].s64 + -22300;
	// 825EC8E0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825EC8E4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825EC8E8: 388A2E70  addi r4, r10, 0x2e70
	ctx.r[4].s64 = ctx.r[10].s64 + 11888;
	// 825EC8EC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825EC8F0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825EC8F4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825EC8F8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825EC8FC: 386AA134  addi r3, r10, -0x5ecc
	ctx.r[3].s64 = ctx.r[10].s64 + -24268;
	// 825EC900: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825EC904: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825EC908: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 825EC90C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825EC910: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 825EC914: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825EC918: 4BE7A509  bl 0x82466e20
	ctx.lr = 0x825EC91C;
	sub_82466E20(ctx, base);
	// 825EC91C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825EC920: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825EC924: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825EC928: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825EC930(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825EC930 size=108
    let mut pc: u32 = 0x825EC930;
    'dispatch: loop {
        match pc {
            0x825EC930 => {
    //   block [0x825EC930..0x825EC99C)
	// 825EC930: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825EC934: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825EC938: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825EC93C: 3D608273  lis r11, -0x7d8d
	ctx.r[11].s64 = -2106392576;
	// 825EC940: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825EC944: 38EB66F4  addi r7, r11, 0x66f4
	ctx.r[7].s64 = ctx.r[11].s64 + 26356;
	// 825EC948: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 825EC94C: 388A2E80  addi r4, r10, 0x2e80
	ctx.r[4].s64 = ctx.r[10].s64 + 11904;
	// 825EC950: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825EC954: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825EC958: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 825EC95C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825EC960: 386AA164  addi r3, r10, -0x5e9c
	ctx.r[3].s64 = ctx.r[10].s64 + -24220;
	// 825EC964: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 825EC968: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825EC96C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825EC970: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825EC974: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825EC978: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825EC97C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825EC980: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825EC984: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825EC988: 4BE7A499  bl 0x82466e20
	ctx.lr = 0x825EC98C;
	sub_82466E20(ctx, base);
	// 825EC98C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825EC990: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825EC994: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825EC998: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825EC9A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825EC9A0 size=112
    let mut pc: u32 = 0x825EC9A0;
    'dispatch: loop {
        match pc {
            0x825EC9A0 => {
    //   block [0x825EC9A0..0x825ECA10)
	// 825EC9A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825EC9A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825EC9A8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825EC9AC: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825EC9B0: 3D608273  lis r11, -0x7d8d
	ctx.r[11].s64 = -2106392576;
	// 825EC9B4: 392A0F68  addi r9, r10, 0xf68
	ctx.r[9].s64 = ctx.r[10].s64 + 3944;
	// 825EC9B8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825EC9BC: 390B6728  addi r8, r11, 0x6728
	ctx.r[8].s64 = ctx.r[11].s64 + 26408;
	// 825EC9C0: 38E00005  li r7, 5
	ctx.r[7].s64 = 5;
	// 825EC9C4: 388A2EA8  addi r4, r10, 0x2ea8
	ctx.r[4].s64 = ctx.r[10].s64 + 11944;
	// 825EC9C8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825EC9CC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825EC9D0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825EC9D4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825EC9D8: 386AA194  addi r3, r10, -0x5e6c
	ctx.r[3].s64 = ctx.r[10].s64 + -24172;
	// 825EC9DC: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 825EC9E0: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 825EC9E4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825EC9E8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825EC9EC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825EC9F0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825EC9F4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825EC9F8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825EC9FC: 4BE7A425  bl 0x82466e20
	ctx.lr = 0x825ECA00;
	sub_82466E20(ctx, base);
	// 825ECA00: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825ECA04: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825ECA08: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825ECA0C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825ECA10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825ECA10 size=112
    let mut pc: u32 = 0x825ECA10;
    'dispatch: loop {
        match pc {
            0x825ECA10 => {
    //   block [0x825ECA10..0x825ECA80)
	// 825ECA10: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825ECA14: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825ECA18: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825ECA1C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825ECA20: 3D608273  lis r11, -0x7d8d
	ctx.r[11].s64 = -2106392576;
	// 825ECA24: 38AAA134  addi r5, r10, -0x5ecc
	ctx.r[5].s64 = ctx.r[10].s64 + -24268;
	// 825ECA28: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825ECA2C: 390B67A0  addi r8, r11, 0x67a0
	ctx.r[8].s64 = ctx.r[11].s64 + 26528;
	// 825ECA30: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 825ECA34: 388A2EBC  addi r4, r10, 0x2ebc
	ctx.r[4].s64 = ctx.r[10].s64 + 11964;
	// 825ECA38: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825ECA3C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825ECA40: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825ECA44: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825ECA48: 386AA1C4  addi r3, r10, -0x5e3c
	ctx.r[3].s64 = ctx.r[10].s64 + -24124;
	// 825ECA4C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 825ECA50: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825ECA54: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825ECA58: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825ECA5C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825ECA60: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825ECA64: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825ECA68: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825ECA6C: 4BE7A3B5  bl 0x82466e20
	ctx.lr = 0x825ECA70;
	sub_82466E20(ctx, base);
	// 825ECA70: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825ECA74: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825ECA78: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825ECA7C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825ECA80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825ECA80 size=116
    let mut pc: u32 = 0x825ECA80;
    'dispatch: loop {
        match pc {
            0x825ECA80 => {
    //   block [0x825ECA80..0x825ECAF4)
	// 825ECA80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825ECA84: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825ECA88: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825ECA8C: 3D408273  lis r10, -0x7d8d
	ctx.r[10].s64 = -2106392576;
	// 825ECA90: 38E00007  li r7, 7
	ctx.r[7].s64 = 7;
	// 825ECA94: 390A67D0  addi r8, r10, 0x67d0
	ctx.r[8].s64 = ctx.r[10].s64 + 26576;
	// 825ECA98: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825ECA9C: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 825ECAA0: 38AAA134  addi r5, r10, -0x5ecc
	ctx.r[5].s64 = ctx.r[10].s64 + -24268;
	// 825ECAA4: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825ECAA8: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 825ECAAC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825ECAB0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825ECAB4: 388A2EDC  addi r4, r10, 0x2edc
	ctx.r[4].s64 = ctx.r[10].s64 + 11996;
	// 825ECAB8: 396B0F7C  addi r11, r11, 0xf7c
	ctx.r[11].s64 = ctx.r[11].s64 + 3964;
	// 825ECABC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825ECAC0: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825ECAC4: 386AA1F4  addi r3, r10, -0x5e0c
	ctx.r[3].s64 = ctx.r[10].s64 + -24076;
	// 825ECAC8: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 825ECACC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825ECAD0: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 825ECAD4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825ECAD8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825ECADC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825ECAE0: 4BE7A341  bl 0x82466e20
	ctx.lr = 0x825ECAE4;
	sub_82466E20(ctx, base);
	// 825ECAE4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825ECAE8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825ECAEC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825ECAF0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825ECAF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825ECAF8 size=100
    let mut pc: u32 = 0x825ECAF8;
    'dispatch: loop {
        match pc {
            0x825ECAF8 => {
    //   block [0x825ECAF8..0x825ECB5C)
	// 825ECAF8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825ECAFC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825ECB00: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825ECB04: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825ECB08: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825ECB0C: 38AAA1F4  addi r5, r10, -0x5e0c
	ctx.r[5].s64 = ctx.r[10].s64 + -24076;
	// 825ECB10: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825ECB14: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825ECB18: 388A2EF8  addi r4, r10, 0x2ef8
	ctx.r[4].s64 = ctx.r[10].s64 + 12024;
	// 825ECB1C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825ECB20: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825ECB24: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825ECB28: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825ECB2C: 386AA224  addi r3, r10, -0x5ddc
	ctx.r[3].s64 = ctx.r[10].s64 + -24028;
	// 825ECB30: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825ECB34: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825ECB38: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 825ECB3C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825ECB40: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 825ECB44: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825ECB48: 4BE7A2D9  bl 0x82466e20
	ctx.lr = 0x825ECB4C;
	sub_82466E20(ctx, base);
	// 825ECB4C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825ECB50: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825ECB54: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825ECB58: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825ECB60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825ECB60 size=24
    let mut pc: u32 = 0x825ECB60;
    'dispatch: loop {
        match pc {
            0x825ECB60 => {
    //   block [0x825ECB60..0x825ECB78)
	// 825ECB60: 3D608273  lis r11, -0x7d8d
	ctx.r[11].s64 = -2106392576;
	// 825ECB64: 3D408274  lis r10, -0x7d8c
	ctx.r[10].s64 = -2106327040;
	// 825ECB68: 394ADDC0  addi r10, r10, -0x2240
	ctx.r[10].s64 = ctx.r[10].s64 + -8768;
	// 825ECB6C: 816B6724  lwz r11, 0x6724(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(26404 as u32) ) } as u64;
	// 825ECB70: 916A0068  stw r11, 0x68(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(104 as u32), ctx.r[11].u32 ) };
	// 825ECB74: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825ECB78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825ECB78 size=116
    let mut pc: u32 = 0x825ECB78;
    'dispatch: loop {
        match pc {
            0x825ECB78 => {
    //   block [0x825ECB78..0x825ECBEC)
	// 825ECB78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825ECB7C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825ECB80: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825ECB84: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 825ECB88: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825ECB8C: 392B0FB8  addi r9, r11, 0xfb8
	ctx.r[9].s64 = ctx.r[11].s64 + 4024;
	// 825ECB90: 38AAAF74  addi r5, r10, -0x508c
	ctx.r[5].s64 = ctx.r[10].s64 + -20620;
	// 825ECB94: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825ECB98: 38E90014  addi r7, r9, 0x14
	ctx.r[7].s64 = ctx.r[9].s64 + 20;
	// 825ECB9C: 38C00005  li r6, 5
	ctx.r[6].s64 = 5;
	// 825ECBA0: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825ECBA4: 388A2F1C  addi r4, r10, 0x2f1c
	ctx.r[4].s64 = ctx.r[10].s64 + 12060;
	// 825ECBA8: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825ECBAC: 396BDDC0  addi r11, r11, -0x2240
	ctx.r[11].s64 = ctx.r[11].s64 + -8768;
	// 825ECBB0: 90E10064  stw r7, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[7].u32 ) };
	// 825ECBB4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825ECBB8: 90C1005C  stw r6, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[6].u32 ) };
	// 825ECBBC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825ECBC0: 386AA254  addi r3, r10, -0x5dac
	ctx.r[3].s64 = ctx.r[10].s64 + -23980;
	// 825ECBC4: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 825ECBC8: 91010074  stw r8, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[8].u32 ) };
	// 825ECBCC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825ECBD0: 9101006C  stw r8, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[8].u32 ) };
	// 825ECBD4: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 825ECBD8: 4BE7A249  bl 0x82466e20
	ctx.lr = 0x825ECBDC;
	sub_82466E20(ctx, base);
	// 825ECBDC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825ECBE0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825ECBE4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825ECBE8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825ECBF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825ECBF0 size=116
    let mut pc: u32 = 0x825ECBF0;
    'dispatch: loop {
        match pc {
            0x825ECBF0 => {
    //   block [0x825ECBF0..0x825ECC64)
	// 825ECBF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825ECBF4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825ECBF8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825ECBFC: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 825ECC00: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825ECC04: 392B0FFC  addi r9, r11, 0xffc
	ctx.r[9].s64 = ctx.r[11].s64 + 4092;
	// 825ECC08: 38AAA134  addi r5, r10, -0x5ecc
	ctx.r[5].s64 = ctx.r[10].s64 + -24268;
	// 825ECC0C: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825ECC10: 38E90014  addi r7, r9, 0x14
	ctx.r[7].s64 = ctx.r[9].s64 + 20;
	// 825ECC14: 38C00007  li r6, 7
	ctx.r[6].s64 = 7;
	// 825ECC18: 3D608273  lis r11, -0x7d8d
	ctx.r[11].s64 = -2106392576;
	// 825ECC1C: 388A2F38  addi r4, r10, 0x2f38
	ctx.r[4].s64 = ctx.r[10].s64 + 12088;
	// 825ECC20: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825ECC24: 396B6880  addi r11, r11, 0x6880
	ctx.r[11].s64 = ctx.r[11].s64 + 26752;
	// 825ECC28: 90E10064  stw r7, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[7].u32 ) };
	// 825ECC2C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825ECC30: 90C1005C  stw r6, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[6].u32 ) };
	// 825ECC34: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825ECC38: 386AA284  addi r3, r10, -0x5d7c
	ctx.r[3].s64 = ctx.r[10].s64 + -23932;
	// 825ECC3C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 825ECC40: 91010074  stw r8, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[8].u32 ) };
	// 825ECC44: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825ECC48: 9101006C  stw r8, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[8].u32 ) };
	// 825ECC4C: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 825ECC50: 4BE7A1D1  bl 0x82466e20
	ctx.lr = 0x825ECC54;
	sub_82466E20(ctx, base);
	// 825ECC54: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825ECC58: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825ECC5C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825ECC60: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825ECC68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825ECC68 size=108
    let mut pc: u32 = 0x825ECC68;
    'dispatch: loop {
        match pc {
            0x825ECC68 => {
    //   block [0x825ECC68..0x825ECCD4)
	// 825ECC68: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825ECC6C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825ECC70: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825ECC74: 3D608273  lis r11, -0x7d8d
	ctx.r[11].s64 = -2106392576;
	// 825ECC78: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825ECC7C: 38EB6928  addi r7, r11, 0x6928
	ctx.r[7].s64 = ctx.r[11].s64 + 26920;
	// 825ECC80: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 825ECC84: 388A2F4C  addi r4, r10, 0x2f4c
	ctx.r[4].s64 = ctx.r[10].s64 + 12108;
	// 825ECC88: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825ECC8C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825ECC90: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 825ECC94: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825ECC98: 386AA2B4  addi r3, r10, -0x5d4c
	ctx.r[3].s64 = ctx.r[10].s64 + -23884;
	// 825ECC9C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 825ECCA0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825ECCA4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825ECCA8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825ECCAC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825ECCB0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825ECCB4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825ECCB8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825ECCBC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825ECCC0: 4BE7A161  bl 0x82466e20
	ctx.lr = 0x825ECCC4;
	sub_82466E20(ctx, base);
	// 825ECCC4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825ECCC8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825ECCCC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825ECCD0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825ECCD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825ECCD8 size=24
    let mut pc: u32 = 0x825ECCD8;
    'dispatch: loop {
        match pc {
            0x825ECCD8 => {
    //   block [0x825ECCD8..0x825ECCF0)
	// 825ECCD8: 3D608273  lis r11, -0x7d8d
	ctx.r[11].s64 = -2106392576;
	// 825ECCDC: 3D408274  lis r10, -0x7d8c
	ctx.r[10].s64 = -2106327040;
	// 825ECCE0: 394ADE38  addi r10, r10, -0x21c8
	ctx.r[10].s64 = ctx.r[10].s64 + -8648;
	// 825ECCE4: 816B687C  lwz r11, 0x687c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(26748 as u32) ) } as u64;
	// 825ECCE8: 916A0008  stw r11, 8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 825ECCEC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825ECCF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825ECCF0 size=116
    let mut pc: u32 = 0x825ECCF0;
    'dispatch: loop {
        match pc {
            0x825ECCF0 => {
    //   block [0x825ECCF0..0x825ECD64)
	// 825ECCF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825ECCF4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825ECCF8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825ECCFC: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825ECD00: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825ECD04: 390BDE38  addi r8, r11, -0x21c8
	ctx.r[8].s64 = ctx.r[11].s64 + -8648;
	// 825ECD08: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825ECD0C: 392A1068  addi r9, r10, 0x1068
	ctx.r[9].s64 = ctx.r[10].s64 + 4200;
	// 825ECD10: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825ECD14: 38E00010  li r7, 0x10
	ctx.r[7].s64 = 16;
	// 825ECD18: 38AAA134  addi r5, r10, -0x5ecc
	ctx.r[5].s64 = ctx.r[10].s64 + -24268;
	// 825ECD1C: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825ECD20: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825ECD24: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825ECD28: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825ECD2C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825ECD30: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825ECD34: 3D608293  lis r11, -0x7d6d
	ctx.r[11].s64 = -2104295424;
	// 825ECD38: 388A3040  addi r4, r10, 0x3040
	ctx.r[4].s64 = ctx.r[10].s64 + 12352;
	// 825ECD3C: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 825ECD40: 386BA2E4  addi r3, r11, -0x5d1c
	ctx.r[3].s64 = ctx.r[11].s64 + -23836;
	// 825ECD44: 39400002  li r10, 2
	ctx.r[10].s64 = 2;
	// 825ECD48: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825ECD4C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825ECD50: 4BE7A0D1  bl 0x82466e20
	ctx.lr = 0x825ECD54;
	sub_82466E20(ctx, base);
	// 825ECD54: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825ECD58: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825ECD5C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825ECD60: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825ECD68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825ECD68 size=112
    let mut pc: u32 = 0x825ECD68;
    'dispatch: loop {
        match pc {
            0x825ECD68 => {
    //   block [0x825ECD68..0x825ECDD8)
	// 825ECD68: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825ECD6C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825ECD70: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825ECD74: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825ECD78: 3D608273  lis r11, -0x7d8d
	ctx.r[11].s64 = -2106392576;
	// 825ECD7C: 38AAA134  addi r5, r10, -0x5ecc
	ctx.r[5].s64 = ctx.r[10].s64 + -24268;
	// 825ECD80: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825ECD84: 390B6974  addi r8, r11, 0x6974
	ctx.r[8].s64 = ctx.r[11].s64 + 26996;
	// 825ECD88: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 825ECD8C: 388A3054  addi r4, r10, 0x3054
	ctx.r[4].s64 = ctx.r[10].s64 + 12372;
	// 825ECD90: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825ECD94: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825ECD98: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825ECD9C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825ECDA0: 386AA314  addi r3, r10, -0x5cec
	ctx.r[3].s64 = ctx.r[10].s64 + -23788;
	// 825ECDA4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 825ECDA8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825ECDAC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825ECDB0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825ECDB4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825ECDB8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825ECDBC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825ECDC0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825ECDC4: 4BE7A05D  bl 0x82466e20
	ctx.lr = 0x825ECDC8;
	sub_82466E20(ctx, base);
	// 825ECDC8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825ECDCC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825ECDD0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825ECDD4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825ECDD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825ECDD8 size=24
    let mut pc: u32 = 0x825ECDD8;
    'dispatch: loop {
        match pc {
            0x825ECDD8 => {
    //   block [0x825ECDD8..0x825ECDF0)
	// 825ECDD8: 3D608273  lis r11, -0x7d8d
	ctx.r[11].s64 = -2106392576;
	// 825ECDDC: 3D408274  lis r10, -0x7d8c
	ctx.r[10].s64 = -2106327040;
	// 825ECDE0: 394ADFB8  addi r10, r10, -0x2048
	ctx.r[10].s64 = ctx.r[10].s64 + -8264;
	// 825ECDE4: 816B69A4  lwz r11, 0x69a4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(27044 as u32) ) } as u64;
	// 825ECDE8: 916A0068  stw r11, 0x68(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(104 as u32), ctx.r[11].u32 ) };
	// 825ECDEC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825ECDF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825ECDF0 size=116
    let mut pc: u32 = 0x825ECDF0;
    'dispatch: loop {
        match pc {
            0x825ECDF0 => {
    //   block [0x825ECDF0..0x825ECE64)
	// 825ECDF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825ECDF4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825ECDF8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825ECDFC: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825ECE00: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825ECE04: 390BDFB8  addi r8, r11, -0x2048
	ctx.r[8].s64 = ctx.r[11].s64 + -8264;
	// 825ECE08: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825ECE0C: 392A10A0  addi r9, r10, 0x10a0
	ctx.r[9].s64 = ctx.r[10].s64 + 4256;
	// 825ECE10: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825ECE14: 38E0000A  li r7, 0xa
	ctx.r[7].s64 = 10;
	// 825ECE18: 38AAA284  addi r5, r10, -0x5d7c
	ctx.r[5].s64 = ctx.r[10].s64 + -23932;
	// 825ECE1C: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825ECE20: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825ECE24: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825ECE28: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825ECE2C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825ECE30: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825ECE34: 3D608293  lis r11, -0x7d6d
	ctx.r[11].s64 = -2104295424;
	// 825ECE38: 388A30BC  addi r4, r10, 0x30bc
	ctx.r[4].s64 = ctx.r[10].s64 + 12476;
	// 825ECE3C: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 825ECE40: 386BA344  addi r3, r11, -0x5cbc
	ctx.r[3].s64 = ctx.r[11].s64 + -23740;
	// 825ECE44: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 825ECE48: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825ECE4C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825ECE50: 4BE79FD1  bl 0x82466e20
	ctx.lr = 0x825ECE54;
	sub_82466E20(ctx, base);
	// 825ECE54: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825ECE58: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825ECE5C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825ECE60: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825ECE68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825ECE68 size=112
    let mut pc: u32 = 0x825ECE68;
    'dispatch: loop {
        match pc {
            0x825ECE68 => {
    //   block [0x825ECE68..0x825ECED8)
	// 825ECE68: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825ECE6C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825ECE70: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825ECE74: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825ECE78: 3D608273  lis r11, -0x7d8d
	ctx.r[11].s64 = -2106392576;
	// 825ECE7C: 38AAA134  addi r5, r10, -0x5ecc
	ctx.r[5].s64 = ctx.r[10].s64 + -24268;
	// 825ECE80: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825ECE84: 390B69A8  addi r8, r11, 0x69a8
	ctx.r[8].s64 = ctx.r[11].s64 + 27048;
	// 825ECE88: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 825ECE8C: 388A30D8  addi r4, r10, 0x30d8
	ctx.r[4].s64 = ctx.r[10].s64 + 12504;
	// 825ECE90: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825ECE94: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825ECE98: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825ECE9C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825ECEA0: 386AA374  addi r3, r10, -0x5c8c
	ctx.r[3].s64 = ctx.r[10].s64 + -23692;
	// 825ECEA4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 825ECEA8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825ECEAC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825ECEB0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825ECEB4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825ECEB8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825ECEBC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825ECEC0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825ECEC4: 4BE79F5D  bl 0x82466e20
	ctx.lr = 0x825ECEC8;
	sub_82466E20(ctx, base);
	// 825ECEC8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825ECECC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825ECED0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825ECED4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825ECED8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825ECED8 size=100
    let mut pc: u32 = 0x825ECED8;
    'dispatch: loop {
        match pc {
            0x825ECED8 => {
    //   block [0x825ECED8..0x825ECF3C)
	// 825ECED8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825ECEDC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825ECEE0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825ECEE4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825ECEE8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825ECEEC: 38AAA8E4  addi r5, r10, -0x571c
	ctx.r[5].s64 = ctx.r[10].s64 + -22300;
	// 825ECEF0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825ECEF4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825ECEF8: 388A30F4  addi r4, r10, 0x30f4
	ctx.r[4].s64 = ctx.r[10].s64 + 12532;
	// 825ECEFC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825ECF00: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825ECF04: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825ECF08: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825ECF0C: 386AA3A4  addi r3, r10, -0x5c5c
	ctx.r[3].s64 = ctx.r[10].s64 + -23644;
	// 825ECF10: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825ECF14: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825ECF18: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 825ECF1C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825ECF20: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 825ECF24: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825ECF28: 4BE79EF9  bl 0x82466e20
	ctx.lr = 0x825ECF2C;
	sub_82466E20(ctx, base);
	// 825ECF2C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825ECF30: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825ECF34: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825ECF38: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825ECF40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825ECF40 size=108
    let mut pc: u32 = 0x825ECF40;
    'dispatch: loop {
        match pc {
            0x825ECF40 => {
    //   block [0x825ECF40..0x825ECFAC)
	// 825ECF40: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825ECF44: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825ECF48: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825ECF4C: 3D608273  lis r11, -0x7d8d
	ctx.r[11].s64 = -2106392576;
	// 825ECF50: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825ECF54: 38EB69C0  addi r7, r11, 0x69c0
	ctx.r[7].s64 = ctx.r[11].s64 + 27072;
	// 825ECF58: 39000009  li r8, 9
	ctx.r[8].s64 = 9;
	// 825ECF5C: 388A3100  addi r4, r10, 0x3100
	ctx.r[4].s64 = ctx.r[10].s64 + 12544;
	// 825ECF60: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825ECF64: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825ECF68: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 825ECF6C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825ECF70: 386AA3D4  addi r3, r10, -0x5c2c
	ctx.r[3].s64 = ctx.r[10].s64 + -23596;
	// 825ECF74: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 825ECF78: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825ECF7C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825ECF80: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825ECF84: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825ECF88: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825ECF8C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825ECF90: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825ECF94: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825ECF98: 4BE79E89  bl 0x82466e20
	ctx.lr = 0x825ECF9C;
	sub_82466E20(ctx, base);
	// 825ECF9C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825ECFA0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825ECFA4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825ECFA8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825ECFB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825ECFB0 size=112
    let mut pc: u32 = 0x825ECFB0;
    'dispatch: loop {
        match pc {
            0x825ECFB0 => {
    //   block [0x825ECFB0..0x825ED020)
	// 825ECFB0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825ECFB4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825ECFB8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825ECFBC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825ECFC0: 3D608273  lis r11, -0x7d8d
	ctx.r[11].s64 = -2106392576;
	// 825ECFC4: 38AAA3A4  addi r5, r10, -0x5c5c
	ctx.r[5].s64 = ctx.r[10].s64 + -23644;
	// 825ECFC8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825ECFCC: 390B6A98  addi r8, r11, 0x6a98
	ctx.r[8].s64 = ctx.r[11].s64 + 27288;
	// 825ECFD0: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 825ECFD4: 388A312C  addi r4, r10, 0x312c
	ctx.r[4].s64 = ctx.r[10].s64 + 12588;
	// 825ECFD8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825ECFDC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825ECFE0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825ECFE4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825ECFE8: 386AA404  addi r3, r10, -0x5bfc
	ctx.r[3].s64 = ctx.r[10].s64 + -23548;
	// 825ECFEC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 825ECFF0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825ECFF4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825ECFF8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825ECFFC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825ED000: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825ED004: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825ED008: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825ED00C: 4BE79E15  bl 0x82466e20
	ctx.lr = 0x825ED010;
	sub_82466E20(ctx, base);
	// 825ED010: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825ED014: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825ED018: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825ED01C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825ED020(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825ED020 size=108
    let mut pc: u32 = 0x825ED020;
    'dispatch: loop {
        match pc {
            0x825ED020 => {
    //   block [0x825ED020..0x825ED08C)
	// 825ED020: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825ED024: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825ED028: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825ED02C: 3D608273  lis r11, -0x7d8d
	ctx.r[11].s64 = -2106392576;
	// 825ED030: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825ED034: 38EB6AC8  addi r7, r11, 0x6ac8
	ctx.r[7].s64 = ctx.r[11].s64 + 27336;
	// 825ED038: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 825ED03C: 388A3144  addi r4, r10, 0x3144
	ctx.r[4].s64 = ctx.r[10].s64 + 12612;
	// 825ED040: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825ED044: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825ED048: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 825ED04C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825ED050: 386AA434  addi r3, r10, -0x5bcc
	ctx.r[3].s64 = ctx.r[10].s64 + -23500;
	// 825ED054: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 825ED058: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825ED05C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825ED060: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825ED064: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825ED068: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825ED06C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825ED070: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825ED074: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825ED078: 4BE79DA9  bl 0x82466e20
	ctx.lr = 0x825ED07C;
	sub_82466E20(ctx, base);
	// 825ED07C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825ED080: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825ED084: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825ED088: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825ED090(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825ED090 size=112
    let mut pc: u32 = 0x825ED090;
    'dispatch: loop {
        match pc {
            0x825ED090 => {
    //   block [0x825ED090..0x825ED100)
	// 825ED090: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825ED094: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825ED098: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825ED09C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825ED0A0: 3D608273  lis r11, -0x7d8d
	ctx.r[11].s64 = -2106392576;
	// 825ED0A4: 38AAA3A4  addi r5, r10, -0x5c5c
	ctx.r[5].s64 = ctx.r[10].s64 + -23644;
	// 825ED0A8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825ED0AC: 390B6AF8  addi r8, r11, 0x6af8
	ctx.r[8].s64 = ctx.r[11].s64 + 27384;
	// 825ED0B0: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 825ED0B4: 388A3164  addi r4, r10, 0x3164
	ctx.r[4].s64 = ctx.r[10].s64 + 12644;
	// 825ED0B8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825ED0BC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825ED0C0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825ED0C4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825ED0C8: 386AA464  addi r3, r10, -0x5b9c
	ctx.r[3].s64 = ctx.r[10].s64 + -23452;
	// 825ED0CC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 825ED0D0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825ED0D4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825ED0D8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825ED0DC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825ED0E0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825ED0E4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825ED0E8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825ED0EC: 4BE79D35  bl 0x82466e20
	ctx.lr = 0x825ED0F0;
	sub_82466E20(ctx, base);
	// 825ED0F0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825ED0F4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825ED0F8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825ED0FC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825ED100(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825ED100 size=112
    let mut pc: u32 = 0x825ED100;
    'dispatch: loop {
        match pc {
            0x825ED100 => {
    //   block [0x825ED100..0x825ED170)
	// 825ED100: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825ED104: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825ED108: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825ED10C: 3D408273  lis r10, -0x7d8d
	ctx.r[10].s64 = -2106392576;
	// 825ED110: 39000009  li r8, 9
	ctx.r[8].s64 = 9;
	// 825ED114: 38EA6B10  addi r7, r10, 0x6b10
	ctx.r[7].s64 = ctx.r[10].s64 + 27408;
	// 825ED118: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825ED11C: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 825ED120: 388A317C  addi r4, r10, 0x317c
	ctx.r[4].s64 = ctx.r[10].s64 + 12668;
	// 825ED124: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825ED128: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 825ED12C: 396B10B4  addi r11, r11, 0x10b4
	ctx.r[11].s64 = ctx.r[11].s64 + 4276;
	// 825ED130: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 825ED134: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825ED138: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825ED13C: 386AA494  addi r3, r10, -0x5b6c
	ctx.r[3].s64 = ctx.r[10].s64 + -23404;
	// 825ED140: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825ED144: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 825ED148: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825ED14C: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 825ED150: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825ED154: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825ED158: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825ED15C: 4BE79CC5  bl 0x82466e20
	ctx.lr = 0x825ED160;
	sub_82466E20(ctx, base);
	// 825ED160: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825ED164: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825ED168: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825ED16C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825ED170(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825ED170 size=108
    let mut pc: u32 = 0x825ED170;
    'dispatch: loop {
        match pc {
            0x825ED170 => {
    //   block [0x825ED170..0x825ED1DC)
	// 825ED170: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825ED174: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825ED178: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825ED17C: 3D608273  lis r11, -0x7d8d
	ctx.r[11].s64 = -2106392576;
	// 825ED180: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825ED184: 38EB6BE8  addi r7, r11, 0x6be8
	ctx.r[7].s64 = ctx.r[11].s64 + 27624;
	// 825ED188: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 825ED18C: 388A318C  addi r4, r10, 0x318c
	ctx.r[4].s64 = ctx.r[10].s64 + 12684;
	// 825ED190: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825ED194: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825ED198: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 825ED19C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825ED1A0: 386AA4C4  addi r3, r10, -0x5b3c
	ctx.r[3].s64 = ctx.r[10].s64 + -23356;
	// 825ED1A4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 825ED1A8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825ED1AC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825ED1B0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825ED1B4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825ED1B8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825ED1BC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825ED1C0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825ED1C4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825ED1C8: 4BE79C59  bl 0x82466e20
	ctx.lr = 0x825ED1CC;
	sub_82466E20(ctx, base);
	// 825ED1CC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825ED1D0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825ED1D4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825ED1D8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825ED1E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825ED1E0 size=108
    let mut pc: u32 = 0x825ED1E0;
    'dispatch: loop {
        match pc {
            0x825ED1E0 => {
    //   block [0x825ED1E0..0x825ED24C)
	// 825ED1E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825ED1E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825ED1E8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825ED1EC: 3D608273  lis r11, -0x7d8d
	ctx.r[11].s64 = -2106392576;
	// 825ED1F0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825ED1F4: 38EB6C00  addi r7, r11, 0x6c00
	ctx.r[7].s64 = ctx.r[11].s64 + 27648;
	// 825ED1F8: 3900000B  li r8, 0xb
	ctx.r[8].s64 = 11;
	// 825ED1FC: 388A31A4  addi r4, r10, 0x31a4
	ctx.r[4].s64 = ctx.r[10].s64 + 12708;
	// 825ED200: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825ED204: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825ED208: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 825ED20C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825ED210: 386AA4F4  addi r3, r10, -0x5b0c
	ctx.r[3].s64 = ctx.r[10].s64 + -23308;
	// 825ED214: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 825ED218: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825ED21C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825ED220: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825ED224: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825ED228: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825ED22C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825ED230: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825ED234: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825ED238: 4BE79BE9  bl 0x82466e20
	ctx.lr = 0x825ED23C;
	sub_82466E20(ctx, base);
	// 825ED23C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825ED240: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825ED244: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825ED248: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825ED250(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825ED250 size=108
    let mut pc: u32 = 0x825ED250;
    'dispatch: loop {
        match pc {
            0x825ED250 => {
    //   block [0x825ED250..0x825ED2BC)
	// 825ED250: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825ED254: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825ED258: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825ED25C: 3D608273  lis r11, -0x7d8d
	ctx.r[11].s64 = -2106392576;
	// 825ED260: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825ED264: 38EB6D08  addi r7, r11, 0x6d08
	ctx.r[7].s64 = ctx.r[11].s64 + 27912;
	// 825ED268: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 825ED26C: 388A31BC  addi r4, r10, 0x31bc
	ctx.r[4].s64 = ctx.r[10].s64 + 12732;
	// 825ED270: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825ED274: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825ED278: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 825ED27C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825ED280: 386AA524  addi r3, r10, -0x5adc
	ctx.r[3].s64 = ctx.r[10].s64 + -23260;
	// 825ED284: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 825ED288: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825ED28C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825ED290: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825ED294: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825ED298: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825ED29C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825ED2A0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825ED2A4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825ED2A8: 4BE79B79  bl 0x82466e20
	ctx.lr = 0x825ED2AC;
	sub_82466E20(ctx, base);
	// 825ED2AC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825ED2B0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825ED2B4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825ED2B8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825ED2C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825ED2C0 size=112
    let mut pc: u32 = 0x825ED2C0;
    'dispatch: loop {
        match pc {
            0x825ED2C0 => {
    //   block [0x825ED2C0..0x825ED330)
	// 825ED2C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825ED2C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825ED2C8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825ED2CC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825ED2D0: 3D608273  lis r11, -0x7d8d
	ctx.r[11].s64 = -2106392576;
	// 825ED2D4: 38AAA3A4  addi r5, r10, -0x5c5c
	ctx.r[5].s64 = ctx.r[10].s64 + -23644;
	// 825ED2D8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825ED2DC: 390B6D68  addi r8, r11, 0x6d68
	ctx.r[8].s64 = ctx.r[11].s64 + 28008;
	// 825ED2E0: 3920000C  li r9, 0xc
	ctx.r[9].s64 = 12;
	// 825ED2E4: 388A31E0  addi r4, r10, 0x31e0
	ctx.r[4].s64 = ctx.r[10].s64 + 12768;
	// 825ED2E8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825ED2EC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825ED2F0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825ED2F4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825ED2F8: 386AA554  addi r3, r10, -0x5aac
	ctx.r[3].s64 = ctx.r[10].s64 + -23212;
	// 825ED2FC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 825ED300: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825ED304: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825ED308: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825ED30C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825ED310: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825ED314: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825ED318: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825ED31C: 4BE79B05  bl 0x82466e20
	ctx.lr = 0x825ED320;
	sub_82466E20(ctx, base);
	// 825ED320: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825ED324: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825ED328: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825ED32C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825ED330(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825ED330 size=112
    let mut pc: u32 = 0x825ED330;
    'dispatch: loop {
        match pc {
            0x825ED330 => {
    //   block [0x825ED330..0x825ED3A0)
	// 825ED330: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825ED334: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825ED338: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825ED33C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825ED340: 3D608273  lis r11, -0x7d8d
	ctx.r[11].s64 = -2106392576;
	// 825ED344: 38AAA3A4  addi r5, r10, -0x5c5c
	ctx.r[5].s64 = ctx.r[10].s64 + -23644;
	// 825ED348: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825ED34C: 390B6E88  addi r8, r11, 0x6e88
	ctx.r[8].s64 = ctx.r[11].s64 + 28296;
	// 825ED350: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 825ED354: 388A31F4  addi r4, r10, 0x31f4
	ctx.r[4].s64 = ctx.r[10].s64 + 12788;
	// 825ED358: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825ED35C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825ED360: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825ED364: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825ED368: 386AA584  addi r3, r10, -0x5a7c
	ctx.r[3].s64 = ctx.r[10].s64 + -23164;
	// 825ED36C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 825ED370: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825ED374: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825ED378: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825ED37C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825ED380: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825ED384: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825ED388: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825ED38C: 4BE79A95  bl 0x82466e20
	ctx.lr = 0x825ED390;
	sub_82466E20(ctx, base);
	// 825ED390: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825ED394: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825ED398: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825ED39C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825ED3A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825ED3A0 size=116
    let mut pc: u32 = 0x825ED3A0;
    'dispatch: loop {
        match pc {
            0x825ED3A0 => {
    //   block [0x825ED3A0..0x825ED414)
	// 825ED3A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825ED3A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825ED3A8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825ED3AC: 3D408273  lis r10, -0x7d8d
	ctx.r[10].s64 = -2106392576;
	// 825ED3B0: 38E00004  li r7, 4
	ctx.r[7].s64 = 4;
	// 825ED3B4: 390A6EA0  addi r8, r10, 0x6ea0
	ctx.r[8].s64 = ctx.r[10].s64 + 28320;
	// 825ED3B8: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825ED3BC: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 825ED3C0: 38AAA3A4  addi r5, r10, -0x5c5c
	ctx.r[5].s64 = ctx.r[10].s64 + -23644;
	// 825ED3C4: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825ED3C8: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 825ED3CC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825ED3D0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825ED3D4: 388A3210  addi r4, r10, 0x3210
	ctx.r[4].s64 = ctx.r[10].s64 + 12816;
	// 825ED3D8: 396B10E4  addi r11, r11, 0x10e4
	ctx.r[11].s64 = ctx.r[11].s64 + 4324;
	// 825ED3DC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825ED3E0: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825ED3E4: 386AA5B4  addi r3, r10, -0x5a4c
	ctx.r[3].s64 = ctx.r[10].s64 + -23116;
	// 825ED3E8: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 825ED3EC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825ED3F0: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 825ED3F4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825ED3F8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825ED3FC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825ED400: 4BE79A21  bl 0x82466e20
	ctx.lr = 0x825ED404;
	sub_82466E20(ctx, base);
	// 825ED404: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825ED408: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825ED40C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825ED410: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825ED418(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825ED418 size=108
    let mut pc: u32 = 0x825ED418;
    'dispatch: loop {
        match pc {
            0x825ED418 => {
    //   block [0x825ED418..0x825ED484)
	// 825ED418: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825ED41C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825ED420: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825ED424: 3D608273  lis r11, -0x7d8d
	ctx.r[11].s64 = -2106392576;
	// 825ED428: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825ED42C: 38EB6F00  addi r7, r11, 0x6f00
	ctx.r[7].s64 = ctx.r[11].s64 + 28416;
	// 825ED430: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 825ED434: 388A3224  addi r4, r10, 0x3224
	ctx.r[4].s64 = ctx.r[10].s64 + 12836;
	// 825ED438: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825ED43C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825ED440: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 825ED444: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825ED448: 386AA5E4  addi r3, r10, -0x5a1c
	ctx.r[3].s64 = ctx.r[10].s64 + -23068;
	// 825ED44C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 825ED450: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825ED454: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825ED458: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825ED45C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825ED460: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825ED464: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825ED468: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825ED46C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825ED470: 4BE799B1  bl 0x82466e20
	ctx.lr = 0x825ED474;
	sub_82466E20(ctx, base);
	// 825ED474: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825ED478: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825ED47C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825ED480: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825ED488(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825ED488 size=108
    let mut pc: u32 = 0x825ED488;
    'dispatch: loop {
        match pc {
            0x825ED488 => {
    //   block [0x825ED488..0x825ED4F4)
	// 825ED488: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825ED48C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825ED490: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825ED494: 3D608273  lis r11, -0x7d8d
	ctx.r[11].s64 = -2106392576;
	// 825ED498: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825ED49C: 38EB6F48  addi r7, r11, 0x6f48
	ctx.r[7].s64 = ctx.r[11].s64 + 28488;
	// 825ED4A0: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 825ED4A4: 388A3244  addi r4, r10, 0x3244
	ctx.r[4].s64 = ctx.r[10].s64 + 12868;
	// 825ED4A8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825ED4AC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825ED4B0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 825ED4B4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825ED4B8: 386AA614  addi r3, r10, -0x59ec
	ctx.r[3].s64 = ctx.r[10].s64 + -23020;
	// 825ED4BC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 825ED4C0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825ED4C4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825ED4C8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825ED4CC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825ED4D0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825ED4D4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825ED4D8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825ED4DC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825ED4E0: 4BE79941  bl 0x82466e20
	ctx.lr = 0x825ED4E4;
	sub_82466E20(ctx, base);
	// 825ED4E4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825ED4E8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825ED4EC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825ED4F0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825ED4F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825ED4F8 size=112
    let mut pc: u32 = 0x825ED4F8;
    'dispatch: loop {
        match pc {
            0x825ED4F8 => {
    //   block [0x825ED4F8..0x825ED568)
	// 825ED4F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825ED4FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825ED500: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825ED504: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825ED508: 3D608273  lis r11, -0x7d8d
	ctx.r[11].s64 = -2106392576;
	// 825ED50C: 38AAA3A4  addi r5, r10, -0x5c5c
	ctx.r[5].s64 = ctx.r[10].s64 + -23644;
	// 825ED510: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825ED514: 390B6F90  addi r8, r11, 0x6f90
	ctx.r[8].s64 = ctx.r[11].s64 + 28560;
	// 825ED518: 3920000B  li r9, 0xb
	ctx.r[9].s64 = 11;
	// 825ED51C: 388A3260  addi r4, r10, 0x3260
	ctx.r[4].s64 = ctx.r[10].s64 + 12896;
	// 825ED520: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825ED524: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825ED528: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825ED52C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825ED530: 386AA644  addi r3, r10, -0x59bc
	ctx.r[3].s64 = ctx.r[10].s64 + -22972;
	// 825ED534: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 825ED538: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825ED53C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825ED540: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825ED544: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825ED548: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825ED54C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825ED550: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825ED554: 4BE798CD  bl 0x82466e20
	ctx.lr = 0x825ED558;
	sub_82466E20(ctx, base);
	// 825ED558: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825ED55C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825ED560: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825ED564: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825ED568(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825ED568 size=112
    let mut pc: u32 = 0x825ED568;
    'dispatch: loop {
        match pc {
            0x825ED568 => {
    //   block [0x825ED568..0x825ED5D8)
	// 825ED568: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825ED56C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825ED570: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825ED574: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825ED578: 3D608273  lis r11, -0x7d8d
	ctx.r[11].s64 = -2106392576;
	// 825ED57C: 38AAA3A4  addi r5, r10, -0x5c5c
	ctx.r[5].s64 = ctx.r[10].s64 + -23644;
	// 825ED580: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825ED584: 390B7098  addi r8, r11, 0x7098
	ctx.r[8].s64 = ctx.r[11].s64 + 28824;
	// 825ED588: 3920000B  li r9, 0xb
	ctx.r[9].s64 = 11;
	// 825ED58C: 388A3274  addi r4, r10, 0x3274
	ctx.r[4].s64 = ctx.r[10].s64 + 12916;
	// 825ED590: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825ED594: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825ED598: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825ED59C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825ED5A0: 386AA674  addi r3, r10, -0x598c
	ctx.r[3].s64 = ctx.r[10].s64 + -22924;
	// 825ED5A4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 825ED5A8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825ED5AC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825ED5B0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825ED5B4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825ED5B8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825ED5BC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825ED5C0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825ED5C4: 4BE7985D  bl 0x82466e20
	ctx.lr = 0x825ED5C8;
	sub_82466E20(ctx, base);
	// 825ED5C8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825ED5CC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825ED5D0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825ED5D4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825ED5D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825ED5D8 size=112
    let mut pc: u32 = 0x825ED5D8;
    'dispatch: loop {
        match pc {
            0x825ED5D8 => {
    //   block [0x825ED5D8..0x825ED648)
	// 825ED5D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825ED5DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825ED5E0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825ED5E4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825ED5E8: 3D608273  lis r11, -0x7d8d
	ctx.r[11].s64 = -2106392576;
	// 825ED5EC: 38AAA3A4  addi r5, r10, -0x5c5c
	ctx.r[5].s64 = ctx.r[10].s64 + -23644;
	// 825ED5F0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825ED5F4: 390B71A0  addi r8, r11, 0x71a0
	ctx.r[8].s64 = ctx.r[11].s64 + 29088;
	// 825ED5F8: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 825ED5FC: 388A3288  addi r4, r10, 0x3288
	ctx.r[4].s64 = ctx.r[10].s64 + 12936;
	// 825ED600: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825ED604: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825ED608: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825ED60C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825ED610: 386AA6A4  addi r3, r10, -0x595c
	ctx.r[3].s64 = ctx.r[10].s64 + -22876;
	// 825ED614: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 825ED618: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825ED61C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825ED620: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825ED624: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825ED628: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825ED62C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825ED630: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825ED634: 4BE797ED  bl 0x82466e20
	ctx.lr = 0x825ED638;
	sub_82466E20(ctx, base);
	// 825ED638: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825ED63C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825ED640: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825ED644: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825ED648(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825ED648 size=112
    let mut pc: u32 = 0x825ED648;
    'dispatch: loop {
        match pc {
            0x825ED648 => {
    //   block [0x825ED648..0x825ED6B8)
	// 825ED648: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825ED64C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825ED650: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825ED654: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825ED658: 3D608273  lis r11, -0x7d8d
	ctx.r[11].s64 = -2106392576;
	// 825ED65C: 38AAAF74  addi r5, r10, -0x508c
	ctx.r[5].s64 = ctx.r[10].s64 + -20620;
	// 825ED660: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825ED664: 390B71B8  addi r8, r11, 0x71b8
	ctx.r[8].s64 = ctx.r[11].s64 + 29112;
	// 825ED668: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 825ED66C: 388A32A4  addi r4, r10, 0x32a4
	ctx.r[4].s64 = ctx.r[10].s64 + 12964;
	// 825ED670: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825ED674: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825ED678: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825ED67C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825ED680: 386AA6D4  addi r3, r10, -0x592c
	ctx.r[3].s64 = ctx.r[10].s64 + -22828;
	// 825ED684: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 825ED688: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825ED68C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825ED690: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825ED694: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825ED698: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825ED69C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825ED6A0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825ED6A4: 4BE7977D  bl 0x82466e20
	ctx.lr = 0x825ED6A8;
	sub_82466E20(ctx, base);
	// 825ED6A8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825ED6AC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825ED6B0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825ED6B4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825ED6B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825ED6B8 size=108
    let mut pc: u32 = 0x825ED6B8;
    'dispatch: loop {
        match pc {
            0x825ED6B8 => {
    //   block [0x825ED6B8..0x825ED724)
	// 825ED6B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825ED6BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825ED6C0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825ED6C4: 3D608273  lis r11, -0x7d8d
	ctx.r[11].s64 = -2106392576;
	// 825ED6C8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825ED6CC: 38EB71E8  addi r7, r11, 0x71e8
	ctx.r[7].s64 = ctx.r[11].s64 + 29160;
	// 825ED6D0: 39000005  li r8, 5
	ctx.r[8].s64 = 5;
	// 825ED6D4: 388A32B4  addi r4, r10, 0x32b4
	ctx.r[4].s64 = ctx.r[10].s64 + 12980;
	// 825ED6D8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825ED6DC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825ED6E0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 825ED6E4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825ED6E8: 386AA704  addi r3, r10, -0x58fc
	ctx.r[3].s64 = ctx.r[10].s64 + -22780;
	// 825ED6EC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 825ED6F0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825ED6F4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825ED6F8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825ED6FC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825ED700: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825ED704: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825ED708: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825ED70C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825ED710: 4BE79711  bl 0x82466e20
	ctx.lr = 0x825ED714;
	sub_82466E20(ctx, base);
	// 825ED714: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825ED718: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825ED71C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825ED720: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825ED728(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825ED728 size=24
    let mut pc: u32 = 0x825ED728;
    'dispatch: loop {
        match pc {
            0x825ED728 => {
    //   block [0x825ED728..0x825ED740)
	// 825ED728: 3D608273  lis r11, -0x7d8d
	ctx.r[11].s64 = -2106392576;
	// 825ED72C: 3D408274  lis r10, -0x7d8c
	ctx.r[10].s64 = -2106327040;
	// 825ED730: 394AE0A8  addi r10, r10, -0x1f58
	ctx.r[10].s64 = ctx.r[10].s64 + -8024;
	// 825ED734: 816B7260  lwz r11, 0x7260(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(29280 as u32) ) } as u64;
	// 825ED738: 916A0050  stw r11, 0x50(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 825ED73C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825ED740(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825ED740 size=116
    let mut pc: u32 = 0x825ED740;
    'dispatch: loop {
        match pc {
            0x825ED740 => {
    //   block [0x825ED740..0x825ED7B4)
	// 825ED740: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825ED744: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825ED748: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825ED74C: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825ED750: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825ED754: 390BE0A8  addi r8, r11, -0x1f58
	ctx.r[8].s64 = ctx.r[11].s64 + -8024;
	// 825ED758: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825ED75C: 392A1110  addi r9, r10, 0x1110
	ctx.r[9].s64 = ctx.r[10].s64 + 4368;
	// 825ED760: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825ED764: 38E0000B  li r7, 0xb
	ctx.r[7].s64 = 11;
	// 825ED768: 38AAA3A4  addi r5, r10, -0x5c5c
	ctx.r[5].s64 = ctx.r[10].s64 + -23644;
	// 825ED76C: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825ED770: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825ED774: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825ED778: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825ED77C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825ED780: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825ED784: 3D608293  lis r11, -0x7d6d
	ctx.r[11].s64 = -2104295424;
	// 825ED788: 388A3300  addi r4, r10, 0x3300
	ctx.r[4].s64 = ctx.r[10].s64 + 13056;
	// 825ED78C: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 825ED790: 386BA734  addi r3, r11, -0x58cc
	ctx.r[3].s64 = ctx.r[11].s64 + -22732;
	// 825ED794: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 825ED798: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825ED79C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825ED7A0: 4BE79681  bl 0x82466e20
	ctx.lr = 0x825ED7A4;
	sub_82466E20(ctx, base);
	// 825ED7A4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825ED7A8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825ED7AC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825ED7B0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825ED7B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825ED7B8 size=112
    let mut pc: u32 = 0x825ED7B8;
    'dispatch: loop {
        match pc {
            0x825ED7B8 => {
    //   block [0x825ED7B8..0x825ED828)
	// 825ED7B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825ED7BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825ED7C0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825ED7C4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825ED7C8: 3D608273  lis r11, -0x7d8d
	ctx.r[11].s64 = -2106392576;
	// 825ED7CC: 38AAA3A4  addi r5, r10, -0x5c5c
	ctx.r[5].s64 = ctx.r[10].s64 + -23644;
	// 825ED7D0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825ED7D4: 390B7264  addi r8, r11, 0x7264
	ctx.r[8].s64 = ctx.r[11].s64 + 29284;
	// 825ED7D8: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 825ED7DC: 388A331C  addi r4, r10, 0x331c
	ctx.r[4].s64 = ctx.r[10].s64 + 13084;
	// 825ED7E0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825ED7E4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825ED7E8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825ED7EC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825ED7F0: 386AA764  addi r3, r10, -0x589c
	ctx.r[3].s64 = ctx.r[10].s64 + -22684;
	// 825ED7F4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 825ED7F8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825ED7FC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825ED800: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825ED804: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825ED808: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825ED80C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825ED810: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825ED814: 4BE7960D  bl 0x82466e20
	ctx.lr = 0x825ED818;
	sub_82466E20(ctx, base);
	// 825ED818: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825ED81C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825ED820: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825ED824: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825ED828(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825ED828 size=116
    let mut pc: u32 = 0x825ED828;
    'dispatch: loop {
        match pc {
            0x825ED828 => {
    //   block [0x825ED828..0x825ED89C)
	// 825ED828: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825ED82C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825ED830: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825ED834: 3D408273  lis r10, -0x7d8d
	ctx.r[10].s64 = -2106392576;
	// 825ED838: 38E00008  li r7, 8
	ctx.r[7].s64 = 8;
	// 825ED83C: 390A7298  addi r8, r10, 0x7298
	ctx.r[8].s64 = ctx.r[10].s64 + 29336;
	// 825ED840: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825ED844: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 825ED848: 38AAA3A4  addi r5, r10, -0x5c5c
	ctx.r[5].s64 = ctx.r[10].s64 + -23644;
	// 825ED84C: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825ED850: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 825ED854: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825ED858: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825ED85C: 388A3340  addi r4, r10, 0x3340
	ctx.r[4].s64 = ctx.r[10].s64 + 13120;
	// 825ED860: 396B1124  addi r11, r11, 0x1124
	ctx.r[11].s64 = ctx.r[11].s64 + 4388;
	// 825ED864: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825ED868: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825ED86C: 386AA794  addi r3, r10, -0x586c
	ctx.r[3].s64 = ctx.r[10].s64 + -22636;
	// 825ED870: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 825ED874: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825ED878: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 825ED87C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825ED880: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825ED884: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825ED888: 4BE79599  bl 0x82466e20
	ctx.lr = 0x825ED88C;
	sub_82466E20(ctx, base);
	// 825ED88C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825ED890: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825ED894: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825ED898: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825ED8A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825ED8A0 size=112
    let mut pc: u32 = 0x825ED8A0;
    'dispatch: loop {
        match pc {
            0x825ED8A0 => {
    //   block [0x825ED8A0..0x825ED910)
	// 825ED8A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825ED8A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825ED8A8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825ED8AC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825ED8B0: 3D608273  lis r11, -0x7d8d
	ctx.r[11].s64 = -2106392576;
	// 825ED8B4: 38AAA3A4  addi r5, r10, -0x5c5c
	ctx.r[5].s64 = ctx.r[10].s64 + -23644;
	// 825ED8B8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825ED8BC: 390B7358  addi r8, r11, 0x7358
	ctx.r[8].s64 = ctx.r[11].s64 + 29528;
	// 825ED8C0: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 825ED8C4: 388A335C  addi r4, r10, 0x335c
	ctx.r[4].s64 = ctx.r[10].s64 + 13148;
	// 825ED8C8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825ED8CC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825ED8D0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825ED8D4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825ED8D8: 386AA7C4  addi r3, r10, -0x583c
	ctx.r[3].s64 = ctx.r[10].s64 + -22588;
	// 825ED8DC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 825ED8E0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825ED8E4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825ED8E8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825ED8EC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825ED8F0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825ED8F4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825ED8F8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825ED8FC: 4BE79525  bl 0x82466e20
	ctx.lr = 0x825ED900;
	sub_82466E20(ctx, base);
	// 825ED900: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825ED904: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825ED908: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825ED90C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825ED910(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825ED910 size=108
    let mut pc: u32 = 0x825ED910;
    'dispatch: loop {
        match pc {
            0x825ED910 => {
    //   block [0x825ED910..0x825ED97C)
	// 825ED910: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825ED914: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825ED918: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825ED91C: 3D608273  lis r11, -0x7d8d
	ctx.r[11].s64 = -2106392576;
	// 825ED920: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825ED924: 38EB7370  addi r7, r11, 0x7370
	ctx.r[7].s64 = ctx.r[11].s64 + 29552;
	// 825ED928: 3900000D  li r8, 0xd
	ctx.r[8].s64 = 13;
	// 825ED92C: 388A3374  addi r4, r10, 0x3374
	ctx.r[4].s64 = ctx.r[10].s64 + 13172;
	// 825ED930: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825ED934: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825ED938: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 825ED93C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825ED940: 386AA7F4  addi r3, r10, -0x580c
	ctx.r[3].s64 = ctx.r[10].s64 + -22540;
	// 825ED944: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 825ED948: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825ED94C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825ED950: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825ED954: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825ED958: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825ED95C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825ED960: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825ED964: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825ED968: 4BE794B9  bl 0x82466e20
	ctx.lr = 0x825ED96C;
	sub_82466E20(ctx, base);
	// 825ED96C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825ED970: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825ED974: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825ED978: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825ED980(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825ED980 size=116
    let mut pc: u32 = 0x825ED980;
    'dispatch: loop {
        match pc {
            0x825ED980 => {
    //   block [0x825ED980..0x825ED9F4)
	// 825ED980: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825ED984: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825ED988: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825ED98C: 3D408273  lis r10, -0x7d8d
	ctx.r[10].s64 = -2106392576;
	// 825ED990: 38E00007  li r7, 7
	ctx.r[7].s64 = 7;
	// 825ED994: 390A74A8  addi r8, r10, 0x74a8
	ctx.r[8].s64 = ctx.r[10].s64 + 29864;
	// 825ED998: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825ED99C: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 825ED9A0: 38AAA3A4  addi r5, r10, -0x5c5c
	ctx.r[5].s64 = ctx.r[10].s64 + -23644;
	// 825ED9A4: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825ED9A8: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 825ED9AC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825ED9B0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825ED9B4: 388A3394  addi r4, r10, 0x3394
	ctx.r[4].s64 = ctx.r[10].s64 + 13204;
	// 825ED9B8: 396B1148  addi r11, r11, 0x1148
	ctx.r[11].s64 = ctx.r[11].s64 + 4424;
	// 825ED9BC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825ED9C0: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825ED9C4: 386AA824  addi r3, r10, -0x57dc
	ctx.r[3].s64 = ctx.r[10].s64 + -22492;
	// 825ED9C8: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 825ED9CC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825ED9D0: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 825ED9D4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825ED9D8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825ED9DC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825ED9E0: 4BE79441  bl 0x82466e20
	ctx.lr = 0x825ED9E4;
	sub_82466E20(ctx, base);
	// 825ED9E4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825ED9E8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825ED9EC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825ED9F0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825ED9F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825ED9F8 size=112
    let mut pc: u32 = 0x825ED9F8;
    'dispatch: loop {
        match pc {
            0x825ED9F8 => {
    //   block [0x825ED9F8..0x825EDA68)
	// 825ED9F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825ED9FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825EDA00: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825EDA04: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825EDA08: 3D608273  lis r11, -0x7d8d
	ctx.r[11].s64 = -2106392576;
	// 825EDA0C: 38AAA3A4  addi r5, r10, -0x5c5c
	ctx.r[5].s64 = ctx.r[10].s64 + -23644;
	// 825EDA10: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825EDA14: 390B7550  addi r8, r11, 0x7550
	ctx.r[8].s64 = ctx.r[11].s64 + 30032;
	// 825EDA18: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 825EDA1C: 388A33B0  addi r4, r10, 0x33b0
	ctx.r[4].s64 = ctx.r[10].s64 + 13232;
	// 825EDA20: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825EDA24: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825EDA28: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825EDA2C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825EDA30: 386AA854  addi r3, r10, -0x57ac
	ctx.r[3].s64 = ctx.r[10].s64 + -22444;
	// 825EDA34: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 825EDA38: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825EDA3C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825EDA40: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825EDA44: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825EDA48: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825EDA4C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825EDA50: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825EDA54: 4BE793CD  bl 0x82466e20
	ctx.lr = 0x825EDA58;
	sub_82466E20(ctx, base);
	// 825EDA58: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825EDA5C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825EDA60: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825EDA64: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825EDA68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825EDA68 size=112
    let mut pc: u32 = 0x825EDA68;
    'dispatch: loop {
        match pc {
            0x825EDA68 => {
    //   block [0x825EDA68..0x825EDAD8)
	// 825EDA68: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825EDA6C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825EDA70: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825EDA74: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825EDA78: 3D608273  lis r11, -0x7d8d
	ctx.r[11].s64 = -2106392576;
	// 825EDA7C: 38AAA3A4  addi r5, r10, -0x5c5c
	ctx.r[5].s64 = ctx.r[10].s64 + -23644;
	// 825EDA80: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825EDA84: 390B7568  addi r8, r11, 0x7568
	ctx.r[8].s64 = ctx.r[11].s64 + 30056;
	// 825EDA88: 3920000C  li r9, 0xc
	ctx.r[9].s64 = 12;
	// 825EDA8C: 388A33D4  addi r4, r10, 0x33d4
	ctx.r[4].s64 = ctx.r[10].s64 + 13268;
	// 825EDA90: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825EDA94: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825EDA98: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825EDA9C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825EDAA0: 386AA884  addi r3, r10, -0x577c
	ctx.r[3].s64 = ctx.r[10].s64 + -22396;
	// 825EDAA4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 825EDAA8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825EDAAC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825EDAB0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825EDAB4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825EDAB8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825EDABC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825EDAC0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825EDAC4: 4BE7935D  bl 0x82466e20
	ctx.lr = 0x825EDAC8;
	sub_82466E20(ctx, base);
	// 825EDAC8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825EDACC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825EDAD0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825EDAD4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825EDAD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825EDAD8 size=112
    let mut pc: u32 = 0x825EDAD8;
    'dispatch: loop {
        match pc {
            0x825EDAD8 => {
    //   block [0x825EDAD8..0x825EDB48)
	// 825EDAD8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825EDADC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825EDAE0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825EDAE4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825EDAE8: 3D608273  lis r11, -0x7d8d
	ctx.r[11].s64 = -2106392576;
	// 825EDAEC: 38AAA3A4  addi r5, r10, -0x5c5c
	ctx.r[5].s64 = ctx.r[10].s64 + -23644;
	// 825EDAF0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825EDAF4: 390B7688  addi r8, r11, 0x7688
	ctx.r[8].s64 = ctx.r[11].s64 + 30344;
	// 825EDAF8: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 825EDAFC: 388A33E8  addi r4, r10, 0x33e8
	ctx.r[4].s64 = ctx.r[10].s64 + 13288;
	// 825EDB00: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825EDB04: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825EDB08: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825EDB0C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825EDB10: 386AA8B4  addi r3, r10, -0x574c
	ctx.r[3].s64 = ctx.r[10].s64 + -22348;
	// 825EDB14: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 825EDB18: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825EDB1C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825EDB20: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825EDB24: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825EDB28: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825EDB2C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825EDB30: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825EDB34: 4BE792ED  bl 0x82466e20
	ctx.lr = 0x825EDB38;
	sub_82466E20(ctx, base);
	// 825EDB38: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825EDB3C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825EDB40: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825EDB44: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825EDB48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825EDB48 size=116
    let mut pc: u32 = 0x825EDB48;
    'dispatch: loop {
        match pc {
            0x825EDB48 => {
    //   block [0x825EDB48..0x825EDBBC)
	// 825EDB48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825EDB4C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825EDB50: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825EDB54: 3D608273  lis r11, -0x7d8d
	ctx.r[11].s64 = -2106392576;
	// 825EDB58: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825EDB5C: 390B76A0  addi r8, r11, 0x76a0
	ctx.r[8].s64 = ctx.r[11].s64 + 30368;
	// 825EDB60: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825EDB64: 392A1180  addi r9, r10, 0x1180
	ctx.r[9].s64 = ctx.r[10].s64 + 4480;
	// 825EDB68: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825EDB6C: 38E00002  li r7, 2
	ctx.r[7].s64 = 2;
	// 825EDB70: 38AAAF74  addi r5, r10, -0x508c
	ctx.r[5].s64 = ctx.r[10].s64 + -20620;
	// 825EDB74: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825EDB78: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825EDB7C: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825EDB80: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825EDB84: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825EDB88: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825EDB8C: 3D608293  lis r11, -0x7d6d
	ctx.r[11].s64 = -2104295424;
	// 825EDB90: 388A33FC  addi r4, r10, 0x33fc
	ctx.r[4].s64 = ctx.r[10].s64 + 13308;
	// 825EDB94: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 825EDB98: 386BA8E4  addi r3, r11, -0x571c
	ctx.r[3].s64 = ctx.r[11].s64 + -22300;
	// 825EDB9C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 825EDBA0: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825EDBA4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825EDBA8: 4BE79279  bl 0x82466e20
	ctx.lr = 0x825EDBAC;
	sub_82466E20(ctx, base);
	// 825EDBAC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825EDBB0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825EDBB4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825EDBB8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825EDBC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825EDBC0 size=100
    let mut pc: u32 = 0x825EDBC0;
    'dispatch: loop {
        match pc {
            0x825EDBC0 => {
    //   block [0x825EDBC0..0x825EDC24)
	// 825EDBC0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825EDBC4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825EDBC8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825EDBCC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825EDBD0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825EDBD4: 38AAAF74  addi r5, r10, -0x508c
	ctx.r[5].s64 = ctx.r[10].s64 + -20620;
	// 825EDBD8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825EDBDC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825EDBE0: 388A3404  addi r4, r10, 0x3404
	ctx.r[4].s64 = ctx.r[10].s64 + 13316;
	// 825EDBE4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825EDBE8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825EDBEC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825EDBF0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825EDBF4: 386AA914  addi r3, r10, -0x56ec
	ctx.r[3].s64 = ctx.r[10].s64 + -22252;
	// 825EDBF8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825EDBFC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825EDC00: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 825EDC04: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825EDC08: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 825EDC0C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825EDC10: 4BE79211  bl 0x82466e20
	ctx.lr = 0x825EDC14;
	sub_82466E20(ctx, base);
	// 825EDC14: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825EDC18: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825EDC1C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825EDC20: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825EDC28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825EDC28 size=112
    let mut pc: u32 = 0x825EDC28;
    'dispatch: loop {
        match pc {
            0x825EDC28 => {
    //   block [0x825EDC28..0x825EDC98)
	// 825EDC28: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825EDC2C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825EDC30: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825EDC34: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825EDC38: 3D608273  lis r11, -0x7d8d
	ctx.r[11].s64 = -2106392576;
	// 825EDC3C: 38AAA914  addi r5, r10, -0x56ec
	ctx.r[5].s64 = ctx.r[10].s64 + -22252;
	// 825EDC40: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825EDC44: 390B76D0  addi r8, r11, 0x76d0
	ctx.r[8].s64 = ctx.r[11].s64 + 30416;
	// 825EDC48: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 825EDC4C: 388A3414  addi r4, r10, 0x3414
	ctx.r[4].s64 = ctx.r[10].s64 + 13332;
	// 825EDC50: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825EDC54: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825EDC58: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825EDC5C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825EDC60: 386AA944  addi r3, r10, -0x56bc
	ctx.r[3].s64 = ctx.r[10].s64 + -22204;
	// 825EDC64: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 825EDC68: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825EDC6C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825EDC70: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825EDC74: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825EDC78: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825EDC7C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825EDC80: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825EDC84: 4BE7919D  bl 0x82466e20
	ctx.lr = 0x825EDC88;
	sub_82466E20(ctx, base);
	// 825EDC88: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825EDC8C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825EDC90: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825EDC94: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825EDC98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825EDC98 size=112
    let mut pc: u32 = 0x825EDC98;
    'dispatch: loop {
        match pc {
            0x825EDC98 => {
    //   block [0x825EDC98..0x825EDD08)
	// 825EDC98: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825EDC9C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825EDCA0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825EDCA4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825EDCA8: 3D608273  lis r11, -0x7d8d
	ctx.r[11].s64 = -2106392576;
	// 825EDCAC: 38AAAF74  addi r5, r10, -0x508c
	ctx.r[5].s64 = ctx.r[10].s64 + -20620;
	// 825EDCB0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825EDCB4: 390B76E8  addi r8, r11, 0x76e8
	ctx.r[8].s64 = ctx.r[11].s64 + 30440;
	// 825EDCB8: 39200007  li r9, 7
	ctx.r[9].s64 = 7;
	// 825EDCBC: 388A3428  addi r4, r10, 0x3428
	ctx.r[4].s64 = ctx.r[10].s64 + 13352;
	// 825EDCC0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825EDCC4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825EDCC8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825EDCCC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825EDCD0: 386AA974  addi r3, r10, -0x568c
	ctx.r[3].s64 = ctx.r[10].s64 + -22156;
	// 825EDCD4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 825EDCD8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825EDCDC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825EDCE0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825EDCE4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825EDCE8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825EDCEC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825EDCF0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825EDCF4: 4BE7912D  bl 0x82466e20
	ctx.lr = 0x825EDCF8;
	sub_82466E20(ctx, base);
	// 825EDCF8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825EDCFC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825EDD00: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825EDD04: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825EDD08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825EDD08 size=112
    let mut pc: u32 = 0x825EDD08;
    'dispatch: loop {
        match pc {
            0x825EDD08 => {
    //   block [0x825EDD08..0x825EDD78)
	// 825EDD08: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825EDD0C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825EDD10: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825EDD14: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825EDD18: 3D608273  lis r11, -0x7d8d
	ctx.r[11].s64 = -2106392576;
	// 825EDD1C: 38AAAF74  addi r5, r10, -0x508c
	ctx.r[5].s64 = ctx.r[10].s64 + -20620;
	// 825EDD20: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825EDD24: 390B7790  addi r8, r11, 0x7790
	ctx.r[8].s64 = ctx.r[11].s64 + 30608;
	// 825EDD28: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 825EDD2C: 388A3440  addi r4, r10, 0x3440
	ctx.r[4].s64 = ctx.r[10].s64 + 13376;
	// 825EDD30: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825EDD34: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825EDD38: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825EDD3C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825EDD40: 386AA9A4  addi r3, r10, -0x565c
	ctx.r[3].s64 = ctx.r[10].s64 + -22108;
	// 825EDD44: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 825EDD48: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825EDD4C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825EDD50: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825EDD54: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825EDD58: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825EDD5C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825EDD60: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825EDD64: 4BE790BD  bl 0x82466e20
	ctx.lr = 0x825EDD68;
	sub_82466E20(ctx, base);
	// 825EDD68: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825EDD6C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825EDD70: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825EDD74: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825EDD78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825EDD78 size=112
    let mut pc: u32 = 0x825EDD78;
    'dispatch: loop {
        match pc {
            0x825EDD78 => {
    //   block [0x825EDD78..0x825EDDE8)
	// 825EDD78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825EDD7C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825EDD80: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825EDD84: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825EDD88: 3D608273  lis r11, -0x7d8d
	ctx.r[11].s64 = -2106392576;
	// 825EDD8C: 38AAAF74  addi r5, r10, -0x508c
	ctx.r[5].s64 = ctx.r[10].s64 + -20620;
	// 825EDD90: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825EDD94: 390B77D8  addi r8, r11, 0x77d8
	ctx.r[8].s64 = ctx.r[11].s64 + 30680;
	// 825EDD98: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 825EDD9C: 388A3450  addi r4, r10, 0x3450
	ctx.r[4].s64 = ctx.r[10].s64 + 13392;
	// 825EDDA0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825EDDA4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825EDDA8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825EDDAC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825EDDB0: 386AA9D4  addi r3, r10, -0x562c
	ctx.r[3].s64 = ctx.r[10].s64 + -22060;
	// 825EDDB4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 825EDDB8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825EDDBC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825EDDC0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825EDDC4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825EDDC8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825EDDCC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825EDDD0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825EDDD4: 4BE7904D  bl 0x82466e20
	ctx.lr = 0x825EDDD8;
	sub_82466E20(ctx, base);
	// 825EDDD8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825EDDDC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825EDDE0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825EDDE4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825EDDE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825EDDE8 size=112
    let mut pc: u32 = 0x825EDDE8;
    'dispatch: loop {
        match pc {
            0x825EDDE8 => {
    //   block [0x825EDDE8..0x825EDE58)
	// 825EDDE8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825EDDEC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825EDDF0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825EDDF4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825EDDF8: 3D608273  lis r11, -0x7d8d
	ctx.r[11].s64 = -2106392576;
	// 825EDDFC: 38AAA3A4  addi r5, r10, -0x5c5c
	ctx.r[5].s64 = ctx.r[10].s64 + -23644;
	// 825EDE00: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825EDE04: 390B7808  addi r8, r11, 0x7808
	ctx.r[8].s64 = ctx.r[11].s64 + 30728;
	// 825EDE08: 39200005  li r9, 5
	ctx.r[9].s64 = 5;
	// 825EDE0C: 388A3468  addi r4, r10, 0x3468
	ctx.r[4].s64 = ctx.r[10].s64 + 13416;
	// 825EDE10: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825EDE14: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825EDE18: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825EDE1C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825EDE20: 386AAA04  addi r3, r10, -0x55fc
	ctx.r[3].s64 = ctx.r[10].s64 + -22012;
	// 825EDE24: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 825EDE28: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825EDE2C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825EDE30: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825EDE34: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825EDE38: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825EDE3C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825EDE40: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825EDE44: 4BE78FDD  bl 0x82466e20
	ctx.lr = 0x825EDE48;
	sub_82466E20(ctx, base);
	// 825EDE48: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825EDE4C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825EDE50: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825EDE54: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825EDE58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825EDE58 size=100
    let mut pc: u32 = 0x825EDE58;
    'dispatch: loop {
        match pc {
            0x825EDE58 => {
    //   block [0x825EDE58..0x825EDEBC)
	// 825EDE58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825EDE5C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825EDE60: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825EDE64: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825EDE68: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825EDE6C: 38AAAF74  addi r5, r10, -0x508c
	ctx.r[5].s64 = ctx.r[10].s64 + -20620;
	// 825EDE70: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825EDE74: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825EDE78: 388A3474  addi r4, r10, 0x3474
	ctx.r[4].s64 = ctx.r[10].s64 + 13428;
	// 825EDE7C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825EDE80: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825EDE84: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825EDE88: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825EDE8C: 386AAA34  addi r3, r10, -0x55cc
	ctx.r[3].s64 = ctx.r[10].s64 + -21964;
	// 825EDE90: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825EDE94: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825EDE98: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 825EDE9C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825EDEA0: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 825EDEA4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825EDEA8: 4BE78F79  bl 0x82466e20
	ctx.lr = 0x825EDEAC;
	sub_82466E20(ctx, base);
	// 825EDEAC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825EDEB0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825EDEB4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825EDEB8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825EDEC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825EDEC0 size=108
    let mut pc: u32 = 0x825EDEC0;
    'dispatch: loop {
        match pc {
            0x825EDEC0 => {
    //   block [0x825EDEC0..0x825EDF2C)
	// 825EDEC0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825EDEC4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825EDEC8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825EDECC: 3D608273  lis r11, -0x7d8d
	ctx.r[11].s64 = -2106392576;
	// 825EDED0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825EDED4: 38EB7880  addi r7, r11, 0x7880
	ctx.r[7].s64 = ctx.r[11].s64 + 30848;
	// 825EDED8: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 825EDEDC: 388A3488  addi r4, r10, 0x3488
	ctx.r[4].s64 = ctx.r[10].s64 + 13448;
	// 825EDEE0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825EDEE4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825EDEE8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 825EDEEC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825EDEF0: 386AAA64  addi r3, r10, -0x559c
	ctx.r[3].s64 = ctx.r[10].s64 + -21916;
	// 825EDEF4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 825EDEF8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825EDEFC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825EDF00: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825EDF04: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825EDF08: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825EDF0C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825EDF10: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825EDF14: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825EDF18: 4BE78F09  bl 0x82466e20
	ctx.lr = 0x825EDF1C;
	sub_82466E20(ctx, base);
	// 825EDF1C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825EDF20: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825EDF24: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825EDF28: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825EDF30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825EDF30 size=112
    let mut pc: u32 = 0x825EDF30;
    'dispatch: loop {
        match pc {
            0x825EDF30 => {
    //   block [0x825EDF30..0x825EDFA0)
	// 825EDF30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825EDF34: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825EDF38: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825EDF3C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825EDF40: 3D608273  lis r11, -0x7d8d
	ctx.r[11].s64 = -2106392576;
	// 825EDF44: 38AAAA34  addi r5, r10, -0x55cc
	ctx.r[5].s64 = ctx.r[10].s64 + -21964;
	// 825EDF48: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825EDF4C: 390B78B0  addi r8, r11, 0x78b0
	ctx.r[8].s64 = ctx.r[11].s64 + 30896;
	// 825EDF50: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 825EDF54: 388A34AC  addi r4, r10, 0x34ac
	ctx.r[4].s64 = ctx.r[10].s64 + 13484;
	// 825EDF58: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825EDF5C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825EDF60: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825EDF64: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825EDF68: 386AAA94  addi r3, r10, -0x556c
	ctx.r[3].s64 = ctx.r[10].s64 + -21868;
	// 825EDF6C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 825EDF70: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825EDF74: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825EDF78: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825EDF7C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825EDF80: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825EDF84: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825EDF88: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825EDF8C: 4BE78E95  bl 0x82466e20
	ctx.lr = 0x825EDF90;
	sub_82466E20(ctx, base);
	// 825EDF90: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825EDF94: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825EDF98: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825EDF9C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825EDFA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825EDFA0 size=108
    let mut pc: u32 = 0x825EDFA0;
    'dispatch: loop {
        match pc {
            0x825EDFA0 => {
    //   block [0x825EDFA0..0x825EE00C)
	// 825EDFA0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825EDFA4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825EDFA8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825EDFAC: 3D608273  lis r11, -0x7d8d
	ctx.r[11].s64 = -2106392576;
	// 825EDFB0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825EDFB4: 38EB78E0  addi r7, r11, 0x78e0
	ctx.r[7].s64 = ctx.r[11].s64 + 30944;
	// 825EDFB8: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 825EDFBC: 388A34C4  addi r4, r10, 0x34c4
	ctx.r[4].s64 = ctx.r[10].s64 + 13508;
	// 825EDFC0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825EDFC4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825EDFC8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 825EDFCC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825EDFD0: 386AAAC4  addi r3, r10, -0x553c
	ctx.r[3].s64 = ctx.r[10].s64 + -21820;
	// 825EDFD4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 825EDFD8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825EDFDC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825EDFE0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825EDFE4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825EDFE8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825EDFEC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825EDFF0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825EDFF4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825EDFF8: 4BE78E29  bl 0x82466e20
	ctx.lr = 0x825EDFFC;
	sub_82466E20(ctx, base);
	// 825EDFFC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825EE000: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825EE004: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825EE008: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825EE010(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825EE010 size=112
    let mut pc: u32 = 0x825EE010;
    'dispatch: loop {
        match pc {
            0x825EE010 => {
    //   block [0x825EE010..0x825EE080)
	// 825EE010: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825EE014: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825EE018: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825EE01C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825EE020: 3D608273  lis r11, -0x7d8d
	ctx.r[11].s64 = -2106392576;
	// 825EE024: 38AAAA34  addi r5, r10, -0x55cc
	ctx.r[5].s64 = ctx.r[10].s64 + -21964;
	// 825EE028: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825EE02C: 390B7910  addi r8, r11, 0x7910
	ctx.r[8].s64 = ctx.r[11].s64 + 30992;
	// 825EE030: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 825EE034: 388A34F4  addi r4, r10, 0x34f4
	ctx.r[4].s64 = ctx.r[10].s64 + 13556;
	// 825EE038: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825EE03C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825EE040: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825EE044: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825EE048: 386AAAF4  addi r3, r10, -0x550c
	ctx.r[3].s64 = ctx.r[10].s64 + -21772;
	// 825EE04C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 825EE050: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825EE054: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825EE058: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825EE05C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825EE060: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825EE064: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825EE068: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825EE06C: 4BE78DB5  bl 0x82466e20
	ctx.lr = 0x825EE070;
	sub_82466E20(ctx, base);
	// 825EE070: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825EE074: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825EE078: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825EE07C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825EE080(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825EE080 size=108
    let mut pc: u32 = 0x825EE080;
    'dispatch: loop {
        match pc {
            0x825EE080 => {
    //   block [0x825EE080..0x825EE0EC)
	// 825EE080: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825EE084: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825EE088: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825EE08C: 3D608273  lis r11, -0x7d8d
	ctx.r[11].s64 = -2106392576;
	// 825EE090: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825EE094: 38EB7958  addi r7, r11, 0x7958
	ctx.r[7].s64 = ctx.r[11].s64 + 31064;
	// 825EE098: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 825EE09C: 388A3514  addi r4, r10, 0x3514
	ctx.r[4].s64 = ctx.r[10].s64 + 13588;
	// 825EE0A0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825EE0A4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825EE0A8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 825EE0AC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825EE0B0: 386AAB24  addi r3, r10, -0x54dc
	ctx.r[3].s64 = ctx.r[10].s64 + -21724;
	// 825EE0B4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 825EE0B8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825EE0BC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825EE0C0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825EE0C4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825EE0C8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825EE0CC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825EE0D0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825EE0D4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825EE0D8: 4BE78D49  bl 0x82466e20
	ctx.lr = 0x825EE0DC;
	sub_82466E20(ctx, base);
	// 825EE0DC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825EE0E0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825EE0E4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825EE0E8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825EE0F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825EE0F0 size=112
    let mut pc: u32 = 0x825EE0F0;
    'dispatch: loop {
        match pc {
            0x825EE0F0 => {
    //   block [0x825EE0F0..0x825EE160)
	// 825EE0F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825EE0F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825EE0F8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825EE0FC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825EE100: 3D608273  lis r11, -0x7d8d
	ctx.r[11].s64 = -2106392576;
	// 825EE104: 38AAAA34  addi r5, r10, -0x55cc
	ctx.r[5].s64 = ctx.r[10].s64 + -21964;
	// 825EE108: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825EE10C: 390B7988  addi r8, r11, 0x7988
	ctx.r[8].s64 = ctx.r[11].s64 + 31112;
	// 825EE110: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 825EE114: 388A3544  addi r4, r10, 0x3544
	ctx.r[4].s64 = ctx.r[10].s64 + 13636;
	// 825EE118: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825EE11C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825EE120: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825EE124: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825EE128: 386AAB54  addi r3, r10, -0x54ac
	ctx.r[3].s64 = ctx.r[10].s64 + -21676;
	// 825EE12C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 825EE130: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825EE134: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825EE138: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825EE13C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825EE140: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825EE144: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825EE148: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825EE14C: 4BE78CD5  bl 0x82466e20
	ctx.lr = 0x825EE150;
	sub_82466E20(ctx, base);
	// 825EE150: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825EE154: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825EE158: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825EE15C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825EE160(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825EE160 size=108
    let mut pc: u32 = 0x825EE160;
    'dispatch: loop {
        match pc {
            0x825EE160 => {
    //   block [0x825EE160..0x825EE1CC)
	// 825EE160: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825EE164: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825EE168: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825EE16C: 3D608273  lis r11, -0x7d8d
	ctx.r[11].s64 = -2106392576;
	// 825EE170: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825EE174: 38EB79D0  addi r7, r11, 0x79d0
	ctx.r[7].s64 = ctx.r[11].s64 + 31184;
	// 825EE178: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 825EE17C: 388A3564  addi r4, r10, 0x3564
	ctx.r[4].s64 = ctx.r[10].s64 + 13668;
	// 825EE180: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825EE184: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825EE188: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 825EE18C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825EE190: 386AAB84  addi r3, r10, -0x547c
	ctx.r[3].s64 = ctx.r[10].s64 + -21628;
	// 825EE194: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 825EE198: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825EE19C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825EE1A0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825EE1A4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825EE1A8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825EE1AC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825EE1B0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825EE1B4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825EE1B8: 4BE78C69  bl 0x82466e20
	ctx.lr = 0x825EE1BC;
	sub_82466E20(ctx, base);
	// 825EE1BC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825EE1C0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825EE1C4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825EE1C8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825EE1D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825EE1D0 size=112
    let mut pc: u32 = 0x825EE1D0;
    'dispatch: loop {
        match pc {
            0x825EE1D0 => {
    //   block [0x825EE1D0..0x825EE240)
	// 825EE1D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825EE1D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825EE1D8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825EE1DC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825EE1E0: 3D608273  lis r11, -0x7d8d
	ctx.r[11].s64 = -2106392576;
	// 825EE1E4: 38AAAA34  addi r5, r10, -0x55cc
	ctx.r[5].s64 = ctx.r[10].s64 + -21964;
	// 825EE1E8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825EE1EC: 390B7A00  addi r8, r11, 0x7a00
	ctx.r[8].s64 = ctx.r[11].s64 + 31232;
	// 825EE1F0: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 825EE1F4: 388A3594  addi r4, r10, 0x3594
	ctx.r[4].s64 = ctx.r[10].s64 + 13716;
	// 825EE1F8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825EE1FC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825EE200: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825EE204: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825EE208: 386AABB4  addi r3, r10, -0x544c
	ctx.r[3].s64 = ctx.r[10].s64 + -21580;
	// 825EE20C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 825EE210: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825EE214: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825EE218: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825EE21C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825EE220: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825EE224: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825EE228: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825EE22C: 4BE78BF5  bl 0x82466e20
	ctx.lr = 0x825EE230;
	sub_82466E20(ctx, base);
	// 825EE230: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825EE234: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825EE238: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825EE23C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825EE240(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825EE240 size=108
    let mut pc: u32 = 0x825EE240;
    'dispatch: loop {
        match pc {
            0x825EE240 => {
    //   block [0x825EE240..0x825EE2AC)
	// 825EE240: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825EE244: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825EE248: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825EE24C: 3D608273  lis r11, -0x7d8d
	ctx.r[11].s64 = -2106392576;
	// 825EE250: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825EE254: 38EB7A48  addi r7, r11, 0x7a48
	ctx.r[7].s64 = ctx.r[11].s64 + 31304;
	// 825EE258: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 825EE25C: 388A35B0  addi r4, r10, 0x35b0
	ctx.r[4].s64 = ctx.r[10].s64 + 13744;
	// 825EE260: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825EE264: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825EE268: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 825EE26C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825EE270: 386AABE4  addi r3, r10, -0x541c
	ctx.r[3].s64 = ctx.r[10].s64 + -21532;
	// 825EE274: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 825EE278: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825EE27C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825EE280: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825EE284: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825EE288: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825EE28C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825EE290: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825EE294: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825EE298: 4BE78B89  bl 0x82466e20
	ctx.lr = 0x825EE29C;
	sub_82466E20(ctx, base);
	// 825EE29C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825EE2A0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825EE2A4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825EE2A8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825EE2B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825EE2B0 size=112
    let mut pc: u32 = 0x825EE2B0;
    'dispatch: loop {
        match pc {
            0x825EE2B0 => {
    //   block [0x825EE2B0..0x825EE320)
	// 825EE2B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825EE2B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825EE2B8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825EE2BC: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825EE2C0: 3D608273  lis r11, -0x7d8d
	ctx.r[11].s64 = -2106392576;
	// 825EE2C4: 392A11F0  addi r9, r10, 0x11f0
	ctx.r[9].s64 = ctx.r[10].s64 + 4592;
	// 825EE2C8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825EE2CC: 390B7AB0  addi r8, r11, 0x7ab0
	ctx.r[8].s64 = ctx.r[11].s64 + 31408;
	// 825EE2D0: 38E00008  li r7, 8
	ctx.r[7].s64 = 8;
	// 825EE2D4: 388A35CC  addi r4, r10, 0x35cc
	ctx.r[4].s64 = ctx.r[10].s64 + 13772;
	// 825EE2D8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825EE2DC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825EE2E0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825EE2E4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825EE2E8: 386AAC14  addi r3, r10, -0x53ec
	ctx.r[3].s64 = ctx.r[10].s64 + -21484;
	// 825EE2EC: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 825EE2F0: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 825EE2F4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825EE2F8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825EE2FC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825EE300: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825EE304: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825EE308: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825EE30C: 4BE78B15  bl 0x82466e20
	ctx.lr = 0x825EE310;
	sub_82466E20(ctx, base);
	// 825EE310: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825EE314: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825EE318: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825EE31C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825EE320(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825EE320 size=108
    let mut pc: u32 = 0x825EE320;
    'dispatch: loop {
        match pc {
            0x825EE320 => {
    //   block [0x825EE320..0x825EE38C)
	// 825EE320: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825EE324: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825EE328: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825EE32C: 3D608273  lis r11, -0x7d8d
	ctx.r[11].s64 = -2106392576;
	// 825EE330: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825EE334: 38EB7B70  addi r7, r11, 0x7b70
	ctx.r[7].s64 = ctx.r[11].s64 + 31600;
	// 825EE338: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 825EE33C: 388A35EC  addi r4, r10, 0x35ec
	ctx.r[4].s64 = ctx.r[10].s64 + 13804;
	// 825EE340: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825EE344: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825EE348: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 825EE34C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825EE350: 386AAC44  addi r3, r10, -0x53bc
	ctx.r[3].s64 = ctx.r[10].s64 + -21436;
	// 825EE354: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 825EE358: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825EE35C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825EE360: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825EE364: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825EE368: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825EE36C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825EE370: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825EE374: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825EE378: 4BE78AA9  bl 0x82466e20
	ctx.lr = 0x825EE37C;
	sub_82466E20(ctx, base);
	// 825EE37C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825EE380: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825EE384: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825EE388: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825EE390(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825EE390 size=108
    let mut pc: u32 = 0x825EE390;
    'dispatch: loop {
        match pc {
            0x825EE390 => {
    //   block [0x825EE390..0x825EE3FC)
	// 825EE390: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825EE394: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825EE398: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825EE39C: 3D608273  lis r11, -0x7d8d
	ctx.r[11].s64 = -2106392576;
	// 825EE3A0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825EE3A4: 38EB7BB8  addi r7, r11, 0x7bb8
	ctx.r[7].s64 = ctx.r[11].s64 + 31672;
	// 825EE3A8: 39000005  li r8, 5
	ctx.r[8].s64 = 5;
	// 825EE3AC: 388A3610  addi r4, r10, 0x3610
	ctx.r[4].s64 = ctx.r[10].s64 + 13840;
	// 825EE3B0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825EE3B4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825EE3B8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 825EE3BC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825EE3C0: 386AAC74  addi r3, r10, -0x538c
	ctx.r[3].s64 = ctx.r[10].s64 + -21388;
	// 825EE3C4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 825EE3C8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825EE3CC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825EE3D0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825EE3D4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825EE3D8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825EE3DC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825EE3E0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825EE3E4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825EE3E8: 4BE78A39  bl 0x82466e20
	ctx.lr = 0x825EE3EC;
	sub_82466E20(ctx, base);
	// 825EE3EC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825EE3F0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825EE3F4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825EE3F8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825EE400(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825EE400 size=116
    let mut pc: u32 = 0x825EE400;
    'dispatch: loop {
        match pc {
            0x825EE400 => {
    //   block [0x825EE400..0x825EE474)
	// 825EE400: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825EE404: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825EE408: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825EE40C: 3D408273  lis r10, -0x7d8d
	ctx.r[10].s64 = -2106392576;
	// 825EE410: 38E00011  li r7, 0x11
	ctx.r[7].s64 = 17;
	// 825EE414: 390A7C30  addi r8, r10, 0x7c30
	ctx.r[8].s64 = ctx.r[10].s64 + 31792;
	// 825EE418: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825EE41C: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 825EE420: 38AAA134  addi r5, r10, -0x5ecc
	ctx.r[5].s64 = ctx.r[10].s64 + -24268;
	// 825EE424: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825EE428: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 825EE42C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825EE430: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825EE434: 388A362C  addi r4, r10, 0x362c
	ctx.r[4].s64 = ctx.r[10].s64 + 13868;
	// 825EE438: 396B1208  addi r11, r11, 0x1208
	ctx.r[11].s64 = ctx.r[11].s64 + 4616;
	// 825EE43C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825EE440: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825EE444: 386AACA4  addi r3, r10, -0x535c
	ctx.r[3].s64 = ctx.r[10].s64 + -21340;
	// 825EE448: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 825EE44C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825EE450: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 825EE454: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825EE458: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825EE45C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825EE460: 4BE789C1  bl 0x82466e20
	ctx.lr = 0x825EE464;
	sub_82466E20(ctx, base);
	// 825EE464: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825EE468: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825EE46C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825EE470: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825EE478(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825EE478 size=100
    let mut pc: u32 = 0x825EE478;
    'dispatch: loop {
        match pc {
            0x825EE478 => {
    //   block [0x825EE478..0x825EE4DC)
	// 825EE478: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825EE47C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825EE480: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825EE484: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825EE488: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825EE48C: 38AAA134  addi r5, r10, -0x5ecc
	ctx.r[5].s64 = ctx.r[10].s64 + -24268;
	// 825EE490: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825EE494: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825EE498: 388A363C  addi r4, r10, 0x363c
	ctx.r[4].s64 = ctx.r[10].s64 + 13884;
	// 825EE49C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825EE4A0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825EE4A4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825EE4A8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825EE4AC: 386AACD4  addi r3, r10, -0x532c
	ctx.r[3].s64 = ctx.r[10].s64 + -21292;
	// 825EE4B0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825EE4B4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825EE4B8: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 825EE4BC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825EE4C0: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 825EE4C4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825EE4C8: 4BE78959  bl 0x82466e20
	ctx.lr = 0x825EE4CC;
	sub_82466E20(ctx, base);
	// 825EE4CC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825EE4D0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825EE4D4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825EE4D8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825EE4E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825EE4E0 size=24
    let mut pc: u32 = 0x825EE4E0;
    'dispatch: loop {
        match pc {
            0x825EE4E0 => {
    //   block [0x825EE4E0..0x825EE4F8)
	// 825EE4E0: 3D608273  lis r11, -0x7d8d
	ctx.r[11].s64 = -2106392576;
	// 825EE4E4: 3D408274  lis r10, -0x7d8c
	ctx.r[10].s64 = -2106327040;
	// 825EE4E8: 394AE1B0  addi r10, r10, -0x1e50
	ctx.r[10].s64 = ctx.r[10].s64 + -7760;
	// 825EE4EC: 816B7DC8  lwz r11, 0x7dc8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(32200 as u32) ) } as u64;
	// 825EE4F0: 916A0050  stw r11, 0x50(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 825EE4F4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825EE4F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825EE4F8 size=116
    let mut pc: u32 = 0x825EE4F8;
    'dispatch: loop {
        match pc {
            0x825EE4F8 => {
    //   block [0x825EE4F8..0x825EE56C)
	// 825EE4F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825EE4FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825EE500: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825EE504: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825EE508: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825EE50C: 390BE1B0  addi r8, r11, -0x1e50
	ctx.r[8].s64 = ctx.r[11].s64 + -7760;
	// 825EE510: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825EE514: 392A1280  addi r9, r10, 0x1280
	ctx.r[9].s64 = ctx.r[10].s64 + 4736;
	// 825EE518: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825EE51C: 38E0000C  li r7, 0xc
	ctx.r[7].s64 = 12;
	// 825EE520: 38AAACD4  addi r5, r10, -0x532c
	ctx.r[5].s64 = ctx.r[10].s64 + -21292;
	// 825EE524: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825EE528: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825EE52C: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825EE530: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825EE534: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825EE538: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825EE53C: 3D608293  lis r11, -0x7d6d
	ctx.r[11].s64 = -2104295424;
	// 825EE540: 388A3690  addi r4, r10, 0x3690
	ctx.r[4].s64 = ctx.r[10].s64 + 13968;
	// 825EE544: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 825EE548: 386BAD04  addi r3, r11, -0x52fc
	ctx.r[3].s64 = ctx.r[11].s64 + -21244;
	// 825EE54C: 39400002  li r10, 2
	ctx.r[10].s64 = 2;
	// 825EE550: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825EE554: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825EE558: 4BE788C9  bl 0x82466e20
	ctx.lr = 0x825EE55C;
	sub_82466E20(ctx, base);
	// 825EE55C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825EE560: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825EE564: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825EE568: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825EE570(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825EE570 size=112
    let mut pc: u32 = 0x825EE570;
    'dispatch: loop {
        match pc {
            0x825EE570 => {
    //   block [0x825EE570..0x825EE5E0)
	// 825EE570: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825EE574: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825EE578: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825EE57C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825EE580: 3D608273  lis r11, -0x7d8d
	ctx.r[11].s64 = -2106392576;
	// 825EE584: 38AAAF74  addi r5, r10, -0x508c
	ctx.r[5].s64 = ctx.r[10].s64 + -20620;
	// 825EE588: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825EE58C: 390B7DD0  addi r8, r11, 0x7dd0
	ctx.r[8].s64 = ctx.r[11].s64 + 32208;
	// 825EE590: 39200009  li r9, 9
	ctx.r[9].s64 = 9;
	// 825EE594: 388A36AC  addi r4, r10, 0x36ac
	ctx.r[4].s64 = ctx.r[10].s64 + 13996;
	// 825EE598: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825EE59C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825EE5A0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825EE5A4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825EE5A8: 386AAD34  addi r3, r10, -0x52cc
	ctx.r[3].s64 = ctx.r[10].s64 + -21196;
	// 825EE5AC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 825EE5B0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825EE5B4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825EE5B8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825EE5BC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825EE5C0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825EE5C4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825EE5C8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825EE5CC: 4BE78855  bl 0x82466e20
	ctx.lr = 0x825EE5D0;
	sub_82466E20(ctx, base);
	// 825EE5D0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825EE5D4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825EE5D8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825EE5DC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825EE5E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825EE5E0 size=108
    let mut pc: u32 = 0x825EE5E0;
    'dispatch: loop {
        match pc {
            0x825EE5E0 => {
    //   block [0x825EE5E0..0x825EE64C)
	// 825EE5E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825EE5E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825EE5E8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825EE5EC: 3D608273  lis r11, -0x7d8d
	ctx.r[11].s64 = -2106392576;
	// 825EE5F0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825EE5F4: 38EB7EA8  addi r7, r11, 0x7ea8
	ctx.r[7].s64 = ctx.r[11].s64 + 32424;
	// 825EE5F8: 39000005  li r8, 5
	ctx.r[8].s64 = 5;
	// 825EE5FC: 388A36C8  addi r4, r10, 0x36c8
	ctx.r[4].s64 = ctx.r[10].s64 + 14024;
	// 825EE600: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825EE604: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825EE608: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 825EE60C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825EE610: 386AAD64  addi r3, r10, -0x529c
	ctx.r[3].s64 = ctx.r[10].s64 + -21148;
	// 825EE614: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 825EE618: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825EE61C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825EE620: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825EE624: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825EE628: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825EE62C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825EE630: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825EE634: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825EE638: 4BE787E9  bl 0x82466e20
	ctx.lr = 0x825EE63C;
	sub_82466E20(ctx, base);
	// 825EE63C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825EE640: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825EE644: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825EE648: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825EE650(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825EE650 size=108
    let mut pc: u32 = 0x825EE650;
    'dispatch: loop {
        match pc {
            0x825EE650 => {
    //   block [0x825EE650..0x825EE6BC)
	// 825EE650: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825EE654: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825EE658: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825EE65C: 3D608273  lis r11, -0x7d8d
	ctx.r[11].s64 = -2106392576;
	// 825EE660: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825EE664: 38EB7F20  addi r7, r11, 0x7f20
	ctx.r[7].s64 = ctx.r[11].s64 + 32544;
	// 825EE668: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 825EE66C: 388A36E4  addi r4, r10, 0x36e4
	ctx.r[4].s64 = ctx.r[10].s64 + 14052;
	// 825EE670: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825EE674: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825EE678: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 825EE67C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825EE680: 386AAD94  addi r3, r10, -0x526c
	ctx.r[3].s64 = ctx.r[10].s64 + -21100;
	// 825EE684: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 825EE688: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825EE68C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825EE690: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825EE694: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825EE698: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825EE69C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825EE6A0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825EE6A4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825EE6A8: 4BE78779  bl 0x82466e20
	ctx.lr = 0x825EE6AC;
	sub_82466E20(ctx, base);
	// 825EE6AC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825EE6B0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825EE6B4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825EE6B8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825EE6C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825EE6C0 size=112
    let mut pc: u32 = 0x825EE6C0;
    'dispatch: loop {
        match pc {
            0x825EE6C0 => {
    //   block [0x825EE6C0..0x825EE730)
	// 825EE6C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825EE6C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825EE6C8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825EE6CC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825EE6D0: 3D608273  lis r11, -0x7d8d
	ctx.r[11].s64 = -2106392576;
	// 825EE6D4: 38AAAF74  addi r5, r10, -0x508c
	ctx.r[5].s64 = ctx.r[10].s64 + -20620;
	// 825EE6D8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825EE6DC: 390B7F68  addi r8, r11, 0x7f68
	ctx.r[8].s64 = ctx.r[11].s64 + 32616;
	// 825EE6E0: 39200013  li r9, 0x13
	ctx.r[9].s64 = 19;
	// 825EE6E4: 388A3704  addi r4, r10, 0x3704
	ctx.r[4].s64 = ctx.r[10].s64 + 14084;
	// 825EE6E8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825EE6EC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825EE6F0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825EE6F4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825EE6F8: 386AADC4  addi r3, r10, -0x523c
	ctx.r[3].s64 = ctx.r[10].s64 + -21052;
	// 825EE6FC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 825EE700: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825EE704: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825EE708: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825EE70C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825EE710: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825EE714: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825EE718: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825EE71C: 4BE78705  bl 0x82466e20
	ctx.lr = 0x825EE720;
	sub_82466E20(ctx, base);
	// 825EE720: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825EE724: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825EE728: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825EE72C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825EE730(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825EE730 size=108
    let mut pc: u32 = 0x825EE730;
    'dispatch: loop {
        match pc {
            0x825EE730 => {
    //   block [0x825EE730..0x825EE79C)
	// 825EE730: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825EE734: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825EE738: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825EE73C: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825EE740: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825EE744: 38EB8130  addi r7, r11, -0x7ed0
	ctx.r[7].s64 = ctx.r[11].s64 + -32464;
	// 825EE748: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 825EE74C: 388A3714  addi r4, r10, 0x3714
	ctx.r[4].s64 = ctx.r[10].s64 + 14100;
	// 825EE750: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825EE754: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825EE758: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 825EE75C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825EE760: 386AADF4  addi r3, r10, -0x520c
	ctx.r[3].s64 = ctx.r[10].s64 + -21004;
	// 825EE764: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 825EE768: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825EE76C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825EE770: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825EE774: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825EE778: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825EE77C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825EE780: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825EE784: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825EE788: 4BE78699  bl 0x82466e20
	ctx.lr = 0x825EE78C;
	sub_82466E20(ctx, base);
	// 825EE78C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825EE790: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825EE794: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825EE798: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825EE7A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825EE7A0 size=24
    let mut pc: u32 = 0x825EE7A0;
    'dispatch: loop {
        match pc {
            0x825EE7A0 => {
    //   block [0x825EE7A0..0x825EE7B8)
	// 825EE7A0: 3D608273  lis r11, -0x7d8d
	ctx.r[11].s64 = -2106392576;
	// 825EE7A4: 3D408274  lis r10, -0x7d8c
	ctx.r[10].s64 = -2106327040;
	// 825EE7A8: 394AE2D0  addi r10, r10, -0x1d30
	ctx.r[10].s64 = ctx.r[10].s64 + -7472;
	// 825EE7AC: 816B7DCC  lwz r11, 0x7dcc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(32204 as u32) ) } as u64;
	// 825EE7B0: 916A0020  stw r11, 0x20(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 825EE7B4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825EE7B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825EE7B8 size=112
    let mut pc: u32 = 0x825EE7B8;
    'dispatch: loop {
        match pc {
            0x825EE7B8 => {
    //   block [0x825EE7B8..0x825EE828)
	// 825EE7B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825EE7BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825EE7C0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825EE7C4: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825EE7C8: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825EE7CC: 392A12D8  addi r9, r10, 0x12d8
	ctx.r[9].s64 = ctx.r[10].s64 + 4824;
	// 825EE7D0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825EE7D4: 390BE2D0  addi r8, r11, -0x1d30
	ctx.r[8].s64 = ctx.r[11].s64 + -7472;
	// 825EE7D8: 38E00002  li r7, 2
	ctx.r[7].s64 = 2;
	// 825EE7DC: 388A3728  addi r4, r10, 0x3728
	ctx.r[4].s64 = ctx.r[10].s64 + 14120;
	// 825EE7E0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825EE7E4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825EE7E8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825EE7EC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825EE7F0: 386AAE24  addi r3, r10, -0x51dc
	ctx.r[3].s64 = ctx.r[10].s64 + -20956;
	// 825EE7F4: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 825EE7F8: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 825EE7FC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825EE800: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825EE804: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825EE808: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825EE80C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825EE810: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825EE814: 4BE7860D  bl 0x82466e20
	ctx.lr = 0x825EE818;
	sub_82466E20(ctx, base);
	// 825EE818: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825EE81C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825EE820: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825EE824: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825EE828(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825EE828 size=108
    let mut pc: u32 = 0x825EE828;
    'dispatch: loop {
        match pc {
            0x825EE828 => {
    //   block [0x825EE828..0x825EE894)
	// 825EE828: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825EE82C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825EE830: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825EE834: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825EE838: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825EE83C: 38EB8148  addi r7, r11, -0x7eb8
	ctx.r[7].s64 = ctx.r[11].s64 + -32440;
	// 825EE840: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 825EE844: 388A3738  addi r4, r10, 0x3738
	ctx.r[4].s64 = ctx.r[10].s64 + 14136;
	// 825EE848: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825EE84C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825EE850: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 825EE854: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825EE858: 386AAE54  addi r3, r10, -0x51ac
	ctx.r[3].s64 = ctx.r[10].s64 + -20908;
	// 825EE85C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 825EE860: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825EE864: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825EE868: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825EE86C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825EE870: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825EE874: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825EE878: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825EE87C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825EE880: 4BE785A1  bl 0x82466e20
	ctx.lr = 0x825EE884;
	sub_82466E20(ctx, base);
	// 825EE884: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825EE888: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825EE88C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825EE890: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825EE898(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825EE898 size=112
    let mut pc: u32 = 0x825EE898;
    'dispatch: loop {
        match pc {
            0x825EE898 => {
    //   block [0x825EE898..0x825EE908)
	// 825EE898: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825EE89C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825EE8A0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825EE8A4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825EE8A8: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825EE8AC: 38AAAF74  addi r5, r10, -0x508c
	ctx.r[5].s64 = ctx.r[10].s64 + -20620;
	// 825EE8B0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825EE8B4: 390B81A8  addi r8, r11, -0x7e58
	ctx.r[8].s64 = ctx.r[11].s64 + -32344;
	// 825EE8B8: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 825EE8BC: 388A3758  addi r4, r10, 0x3758
	ctx.r[4].s64 = ctx.r[10].s64 + 14168;
	// 825EE8C0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825EE8C4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825EE8C8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825EE8CC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825EE8D0: 386AAE84  addi r3, r10, -0x517c
	ctx.r[3].s64 = ctx.r[10].s64 + -20860;
	// 825EE8D4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 825EE8D8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825EE8DC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825EE8E0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825EE8E4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825EE8E8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825EE8EC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825EE8F0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825EE8F4: 4BE7852D  bl 0x82466e20
	ctx.lr = 0x825EE8F8;
	sub_82466E20(ctx, base);
	// 825EE8F8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825EE8FC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825EE900: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825EE904: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825EE908(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825EE908 size=108
    let mut pc: u32 = 0x825EE908;
    'dispatch: loop {
        match pc {
            0x825EE908 => {
    //   block [0x825EE908..0x825EE974)
	// 825EE908: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825EE90C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825EE910: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825EE914: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825EE918: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825EE91C: 38EB81C8  addi r7, r11, -0x7e38
	ctx.r[7].s64 = ctx.r[11].s64 + -32312;
	// 825EE920: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 825EE924: 388A3770  addi r4, r10, 0x3770
	ctx.r[4].s64 = ctx.r[10].s64 + 14192;
	// 825EE928: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825EE92C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825EE930: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 825EE934: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825EE938: 386AAEB4  addi r3, r10, -0x514c
	ctx.r[3].s64 = ctx.r[10].s64 + -20812;
	// 825EE93C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 825EE940: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825EE944: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825EE948: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825EE94C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825EE950: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825EE954: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825EE958: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825EE95C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825EE960: 4BE784C1  bl 0x82466e20
	ctx.lr = 0x825EE964;
	sub_82466E20(ctx, base);
	// 825EE964: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825EE968: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825EE96C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825EE970: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825EE978(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825EE978 size=108
    let mut pc: u32 = 0x825EE978;
    'dispatch: loop {
        match pc {
            0x825EE978 => {
    //   block [0x825EE978..0x825EE9E4)
	// 825EE978: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825EE97C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825EE980: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825EE984: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825EE988: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825EE98C: 38EB8228  addi r7, r11, -0x7dd8
	ctx.r[7].s64 = ctx.r[11].s64 + -32216;
	// 825EE990: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 825EE994: 388A3788  addi r4, r10, 0x3788
	ctx.r[4].s64 = ctx.r[10].s64 + 14216;
	// 825EE998: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825EE99C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825EE9A0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 825EE9A4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825EE9A8: 386AAEE4  addi r3, r10, -0x511c
	ctx.r[3].s64 = ctx.r[10].s64 + -20764;
	// 825EE9AC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 825EE9B0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825EE9B4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825EE9B8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825EE9BC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825EE9C0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825EE9C4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825EE9C8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825EE9CC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825EE9D0: 4BE78451  bl 0x82466e20
	ctx.lr = 0x825EE9D4;
	sub_82466E20(ctx, base);
	// 825EE9D4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825EE9D8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825EE9DC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825EE9E0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825EE9E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825EE9E8 size=116
    let mut pc: u32 = 0x825EE9E8;
    'dispatch: loop {
        match pc {
            0x825EE9E8 => {
    //   block [0x825EE9E8..0x825EEA5C)
	// 825EE9E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825EE9EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825EE9F0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825EE9F4: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825EE9F8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825EE9FC: 390B8258  addi r8, r11, -0x7da8
	ctx.r[8].s64 = ctx.r[11].s64 + -32168;
	// 825EEA00: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825EEA04: 392A1304  addi r9, r10, 0x1304
	ctx.r[9].s64 = ctx.r[10].s64 + 4868;
	// 825EEA08: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825EEA0C: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
	// 825EEA10: 38AAAF74  addi r5, r10, -0x508c
	ctx.r[5].s64 = ctx.r[10].s64 + -20620;
	// 825EEA14: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825EEA18: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825EEA1C: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825EEA20: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825EEA24: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825EEA28: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825EEA2C: 3D608293  lis r11, -0x7d6d
	ctx.r[11].s64 = -2104295424;
	// 825EEA30: 388A37A0  addi r4, r10, 0x37a0
	ctx.r[4].s64 = ctx.r[10].s64 + 14240;
	// 825EEA34: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 825EEA38: 386BAF14  addi r3, r11, -0x50ec
	ctx.r[3].s64 = ctx.r[11].s64 + -20716;
	// 825EEA3C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 825EEA40: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825EEA44: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825EEA48: 4BE783D9  bl 0x82466e20
	ctx.lr = 0x825EEA4C;
	sub_82466E20(ctx, base);
	// 825EEA4C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825EEA50: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825EEA54: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825EEA58: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825EEA60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825EEA60 size=96
    let mut pc: u32 = 0x825EEA60;
    'dispatch: loop {
        match pc {
            0x825EEA60 => {
    //   block [0x825EEA60..0x825EEAC0)
	// 825EEA60: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825EEA64: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825EEA68: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825EEA6C: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 825EEA70: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825EEA74: 388A6DD4  addi r4, r10, 0x6dd4
	ctx.r[4].s64 = ctx.r[10].s64 + 28116;
	// 825EEA78: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825EEA7C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825EEA80: 386AAF44  addi r3, r10, -0x50bc
	ctx.r[3].s64 = ctx.r[10].s64 + -20668;
	// 825EEA84: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825EEA88: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825EEA8C: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 825EEA90: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825EEA94: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825EEA98: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825EEA9C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825EEAA0: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 825EEAA4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825EEAA8: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 825EEAAC: 4BE78375  bl 0x82466e20
	ctx.lr = 0x825EEAB0;
	sub_82466E20(ctx, base);
	// 825EEAB0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825EEAB4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825EEAB8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825EEABC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825EEAC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825EEAC0 size=112
    let mut pc: u32 = 0x825EEAC0;
    'dispatch: loop {
        match pc {
            0x825EEAC0 => {
    //   block [0x825EEAC0..0x825EEB30)
	// 825EEAC0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825EEAC4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825EEAC8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825EEACC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825EEAD0: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825EEAD4: 38AAAF44  addi r5, r10, -0x50bc
	ctx.r[5].s64 = ctx.r[10].s64 + -20668;
	// 825EEAD8: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 825EEADC: 390B8270  addi r8, r11, -0x7d90
	ctx.r[8].s64 = ctx.r[11].s64 + -32144;
	// 825EEAE0: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 825EEAE4: 388A7380  addi r4, r10, 0x7380
	ctx.r[4].s64 = ctx.r[10].s64 + 29568;
	// 825EEAE8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825EEAEC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825EEAF0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825EEAF4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825EEAF8: 386AAF74  addi r3, r10, -0x508c
	ctx.r[3].s64 = ctx.r[10].s64 + -20620;
	// 825EEAFC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 825EEB00: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825EEB04: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825EEB08: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825EEB0C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825EEB10: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825EEB14: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825EEB18: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825EEB1C: 4BE78305  bl 0x82466e20
	ctx.lr = 0x825EEB20;
	sub_82466E20(ctx, base);
	// 825EEB20: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825EEB24: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825EEB28: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825EEB2C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825EEB30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825EEB30 size=112
    let mut pc: u32 = 0x825EEB30;
    'dispatch: loop {
        match pc {
            0x825EEB30 => {
    //   block [0x825EEB30..0x825EEBA0)
	// 825EEB30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825EEB34: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825EEB38: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825EEB3C: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825EEB40: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825EEB44: 392A1320  addi r9, r10, 0x1320
	ctx.r[9].s64 = ctx.r[10].s64 + 4896;
	// 825EEB48: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 825EEB4C: 390B82A0  addi r8, r11, -0x7d60
	ctx.r[8].s64 = ctx.r[11].s64 + -32096;
	// 825EEB50: 38E00007  li r7, 7
	ctx.r[7].s64 = 7;
	// 825EEB54: 388A7468  addi r4, r10, 0x7468
	ctx.r[4].s64 = ctx.r[10].s64 + 29800;
	// 825EEB58: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825EEB5C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825EEB60: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825EEB64: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825EEB68: 386AAFA4  addi r3, r10, -0x505c
	ctx.r[3].s64 = ctx.r[10].s64 + -20572;
	// 825EEB6C: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 825EEB70: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 825EEB74: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825EEB78: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825EEB7C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825EEB80: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825EEB84: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825EEB88: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825EEB8C: 4BE78295  bl 0x82466e20
	ctx.lr = 0x825EEB90;
	sub_82466E20(ctx, base);
	// 825EEB90: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825EEB94: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825EEB98: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825EEB9C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825EEBA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825EEBA0 size=108
    let mut pc: u32 = 0x825EEBA0;
    'dispatch: loop {
        match pc {
            0x825EEBA0 => {
    //   block [0x825EEBA0..0x825EEC0C)
	// 825EEBA0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825EEBA4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825EEBA8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825EEBAC: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825EEBB0: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 825EEBB4: 38EB8348  addi r7, r11, -0x7cb8
	ctx.r[7].s64 = ctx.r[11].s64 + -31928;
	// 825EEBB8: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 825EEBBC: 388A7888  addi r4, r10, 0x7888
	ctx.r[4].s64 = ctx.r[10].s64 + 30856;
	// 825EEBC0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825EEBC4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825EEBC8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 825EEBCC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825EEBD0: 386AAFD4  addi r3, r10, -0x502c
	ctx.r[3].s64 = ctx.r[10].s64 + -20524;
	// 825EEBD4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 825EEBD8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825EEBDC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825EEBE0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825EEBE4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825EEBE8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825EEBEC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825EEBF0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825EEBF4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825EEBF8: 4BE78229  bl 0x82466e20
	ctx.lr = 0x825EEBFC;
	sub_82466E20(ctx, base);
	// 825EEBFC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825EEC00: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825EEC04: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825EEC08: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825EEC10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825EEC10 size=108
    let mut pc: u32 = 0x825EEC10;
    'dispatch: loop {
        match pc {
            0x825EEC10 => {
    //   block [0x825EEC10..0x825EEC7C)
	// 825EEC10: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825EEC14: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825EEC18: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825EEC1C: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825EEC20: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 825EEC24: 38EB8378  addi r7, r11, -0x7c88
	ctx.r[7].s64 = ctx.r[11].s64 + -31880;
	// 825EEC28: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 825EEC2C: 388A7898  addi r4, r10, 0x7898
	ctx.r[4].s64 = ctx.r[10].s64 + 30872;
	// 825EEC30: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825EEC34: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825EEC38: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 825EEC3C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825EEC40: 386AB004  addi r3, r10, -0x4ffc
	ctx.r[3].s64 = ctx.r[10].s64 + -20476;
	// 825EEC44: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 825EEC48: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825EEC4C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825EEC50: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825EEC54: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825EEC58: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825EEC5C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825EEC60: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825EEC64: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825EEC68: 4BE781B9  bl 0x82466e20
	ctx.lr = 0x825EEC6C;
	sub_82466E20(ctx, base);
	// 825EEC6C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825EEC70: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825EEC74: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825EEC78: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825EEC80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825EEC80 size=28
    let mut pc: u32 = 0x825EEC80;
    'dispatch: loop {
        match pc {
            0x825EEC80 => {
    //   block [0x825EEC80..0x825EEC9C)
	// 825EEC80: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825EEC84: 3D408274  lis r10, -0x7d8c
	ctx.r[10].s64 = -2106327040;
	// 825EEC88: 394AE300  addi r10, r10, -0x1d00
	ctx.r[10].s64 = ctx.r[10].s64 + -7424;
	// 825EEC8C: 816B83A8  lwz r11, -0x7c58(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-31832 as u32) ) } as u64;
	// 825EEC90: 916A0050  stw r11, 0x50(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 825EEC94: 916A0068  stw r11, 0x68(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(104 as u32), ctx.r[11].u32 ) };
	// 825EEC98: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825EECA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825EECA0 size=112
    let mut pc: u32 = 0x825EECA0;
    'dispatch: loop {
        match pc {
            0x825EECA0 => {
    //   block [0x825EECA0..0x825EED10)
	// 825EECA0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825EECA4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825EECA8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825EECAC: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825EECB0: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825EECB4: 392A14B0  addi r9, r10, 0x14b0
	ctx.r[9].s64 = ctx.r[10].s64 + 5296;
	// 825EECB8: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 825EECBC: 390BE300  addi r8, r11, -0x1d00
	ctx.r[8].s64 = ctx.r[11].s64 + -7424;
	// 825EECC0: 38E00008  li r7, 8
	ctx.r[7].s64 = 8;
	// 825EECC4: 388A72F4  addi r4, r10, 0x72f4
	ctx.r[4].s64 = ctx.r[10].s64 + 29428;
	// 825EECC8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825EECCC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825EECD0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825EECD4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825EECD8: 386AB034  addi r3, r10, -0x4fcc
	ctx.r[3].s64 = ctx.r[10].s64 + -20428;
	// 825EECDC: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 825EECE0: 39400003  li r10, 3
	ctx.r[10].s64 = 3;
	// 825EECE4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825EECE8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825EECEC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825EECF0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825EECF4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825EECF8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825EECFC: 4BE78125  bl 0x82466e20
	ctx.lr = 0x825EED00;
	sub_82466E20(ctx, base);
	// 825EED00: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825EED04: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825EED08: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825EED0C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825EED10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825EED10 size=108
    let mut pc: u32 = 0x825EED10;
    'dispatch: loop {
        match pc {
            0x825EED10 => {
    //   block [0x825EED10..0x825EED7C)
	// 825EED10: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825EED14: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825EED18: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825EED1C: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825EED20: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 825EED24: 38EB83B4  addi r7, r11, -0x7c4c
	ctx.r[7].s64 = ctx.r[11].s64 + -31820;
	// 825EED28: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 825EED2C: 388A7914  addi r4, r10, 0x7914
	ctx.r[4].s64 = ctx.r[10].s64 + 30996;
	// 825EED30: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825EED34: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825EED38: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 825EED3C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825EED40: 386AB064  addi r3, r10, -0x4f9c
	ctx.r[3].s64 = ctx.r[10].s64 + -20380;
	// 825EED44: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 825EED48: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825EED4C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825EED50: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825EED54: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825EED58: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825EED5C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825EED60: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825EED64: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825EED68: 4BE780B9  bl 0x82466e20
	ctx.lr = 0x825EED6C;
	sub_82466E20(ctx, base);
	// 825EED6C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825EED70: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825EED74: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825EED78: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825EED80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825EED80 size=108
    let mut pc: u32 = 0x825EED80;
    'dispatch: loop {
        match pc {
            0x825EED80 => {
    //   block [0x825EED80..0x825EEDEC)
	// 825EED80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825EED84: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825EED88: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825EED8C: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825EED90: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 825EED94: 38EB83E4  addi r7, r11, -0x7c1c
	ctx.r[7].s64 = ctx.r[11].s64 + -31772;
	// 825EED98: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 825EED9C: 388A7788  addi r4, r10, 0x7788
	ctx.r[4].s64 = ctx.r[10].s64 + 30600;
	// 825EEDA0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825EEDA4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825EEDA8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 825EEDAC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825EEDB0: 386AB094  addi r3, r10, -0x4f6c
	ctx.r[3].s64 = ctx.r[10].s64 + -20332;
	// 825EEDB4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 825EEDB8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825EEDBC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825EEDC0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825EEDC4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825EEDC8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825EEDCC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825EEDD0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825EEDD4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825EEDD8: 4BE78049  bl 0x82466e20
	ctx.lr = 0x825EEDDC;
	sub_82466E20(ctx, base);
	// 825EEDDC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825EEDE0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825EEDE4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825EEDE8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825EEDF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825EEDF0 size=108
    let mut pc: u32 = 0x825EEDF0;
    'dispatch: loop {
        match pc {
            0x825EEDF0 => {
    //   block [0x825EEDF0..0x825EEE5C)
	// 825EEDF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825EEDF4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825EEDF8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825EEDFC: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825EEE00: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 825EEE04: 38EB8414  addi r7, r11, -0x7bec
	ctx.r[7].s64 = ctx.r[11].s64 + -31724;
	// 825EEE08: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 825EEE0C: 388A77AC  addi r4, r10, 0x77ac
	ctx.r[4].s64 = ctx.r[10].s64 + 30636;
	// 825EEE10: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825EEE14: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825EEE18: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 825EEE1C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825EEE20: 386AB0C4  addi r3, r10, -0x4f3c
	ctx.r[3].s64 = ctx.r[10].s64 + -20284;
	// 825EEE24: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 825EEE28: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825EEE2C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825EEE30: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825EEE34: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825EEE38: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825EEE3C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825EEE40: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825EEE44: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825EEE48: 4BE77FD9  bl 0x82466e20
	ctx.lr = 0x825EEE4C;
	sub_82466E20(ctx, base);
	// 825EEE4C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825EEE50: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825EEE54: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825EEE58: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825EEE60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825EEE60 size=24
    let mut pc: u32 = 0x825EEE60;
    'dispatch: loop {
        match pc {
            0x825EEE60 => {
    //   block [0x825EEE60..0x825EEE78)
	// 825EEE60: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825EEE64: 3D408274  lis r10, -0x7d8c
	ctx.r[10].s64 = -2106327040;
	// 825EEE68: 394AE3C0  addi r10, r10, -0x1c40
	ctx.r[10].s64 = ctx.r[10].s64 + -7232;
	// 825EEE6C: 816B842C  lwz r11, -0x7bd4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-31700 as u32) ) } as u64;
	// 825EEE70: 916A0050  stw r11, 0x50(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 825EEE74: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825EEE78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825EEE78 size=112
    let mut pc: u32 = 0x825EEE78;
    'dispatch: loop {
        match pc {
            0x825EEE78 => {
    //   block [0x825EEE78..0x825EEEE8)
	// 825EEE78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825EEE7C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825EEE80: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825EEE84: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825EEE88: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825EEE8C: 392A1504  addi r9, r10, 0x1504
	ctx.r[9].s64 = ctx.r[10].s64 + 5380;
	// 825EEE90: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 825EEE94: 390BE3C0  addi r8, r11, -0x1c40
	ctx.r[8].s64 = ctx.r[11].s64 + -7232;
	// 825EEE98: 38E00009  li r7, 9
	ctx.r[7].s64 = 9;
	// 825EEE9C: 388A77C8  addi r4, r10, 0x77c8
	ctx.r[4].s64 = ctx.r[10].s64 + 30664;
	// 825EEEA0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825EEEA4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825EEEA8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825EEEAC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825EEEB0: 386AB0F4  addi r3, r10, -0x4f0c
	ctx.r[3].s64 = ctx.r[10].s64 + -20236;
	// 825EEEB4: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 825EEEB8: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 825EEEBC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825EEEC0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825EEEC4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825EEEC8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825EEECC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825EEED0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825EEED4: 4BE77F4D  bl 0x82466e20
	ctx.lr = 0x825EEED8;
	sub_82466E20(ctx, base);
	// 825EEED8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825EEEDC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825EEEE0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825EEEE4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825EEEE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825EEEE8 size=112
    let mut pc: u32 = 0x825EEEE8;
    'dispatch: loop {
        match pc {
            0x825EEEE8 => {
    //   block [0x825EEEE8..0x825EEF58)
	// 825EEEE8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825EEEEC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825EEEF0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825EEEF4: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825EEEF8: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825EEEFC: 392A1540  addi r9, r10, 0x1540
	ctx.r[9].s64 = ctx.r[10].s64 + 5440;
	// 825EEF00: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825EEF04: 390B8438  addi r8, r11, -0x7bc8
	ctx.r[8].s64 = ctx.r[11].s64 + -31688;
	// 825EEF08: 38E00003  li r7, 3
	ctx.r[7].s64 = 3;
	// 825EEF0C: 388A37B0  addi r4, r10, 0x37b0
	ctx.r[4].s64 = ctx.r[10].s64 + 14256;
	// 825EEF10: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825EEF14: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825EEF18: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825EEF1C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825EEF20: 386AB124  addi r3, r10, -0x4edc
	ctx.r[3].s64 = ctx.r[10].s64 + -20188;
	// 825EEF24: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 825EEF28: 39400002  li r10, 2
	ctx.r[10].s64 = 2;
	// 825EEF2C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825EEF30: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825EEF34: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825EEF38: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825EEF3C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825EEF40: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825EEF44: 4BE77EDD  bl 0x82466e20
	ctx.lr = 0x825EEF48;
	sub_82466E20(ctx, base);
	// 825EEF48: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825EEF4C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825EEF50: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825EEF54: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825EEF58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825EEF58 size=108
    let mut pc: u32 = 0x825EEF58;
    'dispatch: loop {
        match pc {
            0x825EEF58 => {
    //   block [0x825EEF58..0x825EEFC4)
	// 825EEF58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825EEF5C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825EEF60: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825EEF64: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825EEF68: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 825EEF6C: 38EB8480  addi r7, r11, -0x7b80
	ctx.r[7].s64 = ctx.r[11].s64 + -31616;
	// 825EEF70: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 825EEF74: 388A7BB0  addi r4, r10, 0x7bb0
	ctx.r[4].s64 = ctx.r[10].s64 + 31664;
	// 825EEF78: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825EEF7C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825EEF80: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 825EEF84: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825EEF88: 386AB154  addi r3, r10, -0x4eac
	ctx.r[3].s64 = ctx.r[10].s64 + -20140;
	// 825EEF8C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 825EEF90: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825EEF94: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825EEF98: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825EEF9C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825EEFA0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825EEFA4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825EEFA8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825EEFAC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825EEFB0: 4BE77E71  bl 0x82466e20
	ctx.lr = 0x825EEFB4;
	sub_82466E20(ctx, base);
	// 825EEFB4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825EEFB8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825EEFBC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825EEFC0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825EEFC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825EEFC8 size=108
    let mut pc: u32 = 0x825EEFC8;
    'dispatch: loop {
        match pc {
            0x825EEFC8 => {
    //   block [0x825EEFC8..0x825EF034)
	// 825EEFC8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825EEFCC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825EEFD0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825EEFD4: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825EEFD8: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 825EEFDC: 38EB84B0  addi r7, r11, -0x7b50
	ctx.r[7].s64 = ctx.r[11].s64 + -31568;
	// 825EEFE0: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 825EEFE4: 388A7F88  addi r4, r10, 0x7f88
	ctx.r[4].s64 = ctx.r[10].s64 + 32648;
	// 825EEFE8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825EEFEC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825EEFF0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 825EEFF4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825EEFF8: 386AB184  addi r3, r10, -0x4e7c
	ctx.r[3].s64 = ctx.r[10].s64 + -20092;
	// 825EEFFC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 825EF000: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825EF004: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825EF008: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825EF00C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825EF010: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825EF014: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825EF018: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825EF01C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825EF020: 4BE77E01  bl 0x82466e20
	ctx.lr = 0x825EF024;
	sub_82466E20(ctx, base);
	// 825EF024: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825EF028: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825EF02C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825EF030: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825EF038(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825EF038 size=112
    let mut pc: u32 = 0x825EF038;
    'dispatch: loop {
        match pc {
            0x825EF038 => {
    //   block [0x825EF038..0x825EF0A8)
	// 825EF038: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825EF03C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825EF040: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825EF044: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825EF048: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825EF04C: 392A1578  addi r9, r10, 0x1578
	ctx.r[9].s64 = ctx.r[10].s64 + 5496;
	// 825EF050: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825EF054: 390B84E8  addi r8, r11, -0x7b18
	ctx.r[8].s64 = ctx.r[11].s64 + -31512;
	// 825EF058: 38E00004  li r7, 4
	ctx.r[7].s64 = 4;
	// 825EF05C: 388A82E8  addi r4, r10, -0x7d18
	ctx.r[4].s64 = ctx.r[10].s64 + -32024;
	// 825EF060: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825EF064: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825EF068: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825EF06C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825EF070: 386AB1B4  addi r3, r10, -0x4e4c
	ctx.r[3].s64 = ctx.r[10].s64 + -20044;
	// 825EF074: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 825EF078: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 825EF07C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825EF080: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825EF084: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825EF088: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825EF08C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825EF090: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825EF094: 4BE77D8D  bl 0x82466e20
	ctx.lr = 0x825EF098;
	sub_82466E20(ctx, base);
	// 825EF098: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825EF09C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825EF0A0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825EF0A4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825EF0A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825EF0A8 size=108
    let mut pc: u32 = 0x825EF0A8;
    'dispatch: loop {
        match pc {
            0x825EF0A8 => {
    //   block [0x825EF0A8..0x825EF114)
	// 825EF0A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825EF0AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825EF0B0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825EF0B4: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825EF0B8: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 825EF0BC: 38EB8548  addi r7, r11, -0x7ab8
	ctx.r[7].s64 = ctx.r[11].s64 + -31416;
	// 825EF0C0: 3900000B  li r8, 0xb
	ctx.r[8].s64 = 11;
	// 825EF0C4: 388A7F28  addi r4, r10, 0x7f28
	ctx.r[4].s64 = ctx.r[10].s64 + 32552;
	// 825EF0C8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825EF0CC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825EF0D0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 825EF0D4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825EF0D8: 386AB1E4  addi r3, r10, -0x4e1c
	ctx.r[3].s64 = ctx.r[10].s64 + -19996;
	// 825EF0DC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 825EF0E0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825EF0E4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825EF0E8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825EF0EC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825EF0F0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825EF0F4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825EF0F8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825EF0FC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825EF100: 4BE77D21  bl 0x82466e20
	ctx.lr = 0x825EF104;
	sub_82466E20(ctx, base);
	// 825EF104: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825EF108: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825EF10C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825EF110: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825EF118(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825EF118 size=108
    let mut pc: u32 = 0x825EF118;
    'dispatch: loop {
        match pc {
            0x825EF118 => {
    //   block [0x825EF118..0x825EF184)
	// 825EF118: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825EF11C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825EF120: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825EF124: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825EF128: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 825EF12C: 38EB8650  addi r7, r11, -0x79b0
	ctx.r[7].s64 = ctx.r[11].s64 + -31152;
	// 825EF130: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 825EF134: 388A78C0  addi r4, r10, 0x78c0
	ctx.r[4].s64 = ctx.r[10].s64 + 30912;
	// 825EF138: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825EF13C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825EF140: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 825EF144: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825EF148: 386AB214  addi r3, r10, -0x4dec
	ctx.r[3].s64 = ctx.r[10].s64 + -19948;
	// 825EF14C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 825EF150: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825EF154: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825EF158: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825EF15C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825EF160: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825EF164: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825EF168: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825EF16C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825EF170: 4BE77CB1  bl 0x82466e20
	ctx.lr = 0x825EF174;
	sub_82466E20(ctx, base);
	// 825EF174: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825EF178: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825EF17C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825EF180: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825EF188(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825EF188 size=108
    let mut pc: u32 = 0x825EF188;
    'dispatch: loop {
        match pc {
            0x825EF188 => {
    //   block [0x825EF188..0x825EF1F4)
	// 825EF188: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825EF18C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825EF190: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825EF194: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825EF198: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825EF19C: 38EB8668  addi r7, r11, -0x7998
	ctx.r[7].s64 = ctx.r[11].s64 + -31128;
	// 825EF1A0: 39000005  li r8, 5
	ctx.r[8].s64 = 5;
	// 825EF1A4: 388A8060  addi r4, r10, -0x7fa0
	ctx.r[4].s64 = ctx.r[10].s64 + -32672;
	// 825EF1A8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825EF1AC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825EF1B0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 825EF1B4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825EF1B8: 386AB244  addi r3, r10, -0x4dbc
	ctx.r[3].s64 = ctx.r[10].s64 + -19900;
	// 825EF1BC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 825EF1C0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825EF1C4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825EF1C8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825EF1CC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825EF1D0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825EF1D4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825EF1D8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825EF1DC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825EF1E0: 4BE77C41  bl 0x82466e20
	ctx.lr = 0x825EF1E4;
	sub_82466E20(ctx, base);
	// 825EF1E4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825EF1E8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825EF1EC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825EF1F0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825EF1F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825EF1F8 size=24
    let mut pc: u32 = 0x825EF1F8;
    'dispatch: loop {
        match pc {
            0x825EF1F8 => {
    //   block [0x825EF1F8..0x825EF210)
	// 825EF1F8: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825EF1FC: 3D408274  lis r10, -0x7d8c
	ctx.r[10].s64 = -2106327040;
	// 825EF200: 394AE498  addi r10, r10, -0x1b68
	ctx.r[10].s64 = ctx.r[10].s64 + -7016;
	// 825EF204: 816B84E4  lwz r11, -0x7b1c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-31516 as u32) ) } as u64;
	// 825EF208: 916A0020  stw r11, 0x20(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 825EF20C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825EF210(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825EF210 size=108
    let mut pc: u32 = 0x825EF210;
    'dispatch: loop {
        match pc {
            0x825EF210 => {
    //   block [0x825EF210..0x825EF27C)
	// 825EF210: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825EF214: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825EF218: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825EF21C: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825EF220: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825EF224: 38EBE498  addi r7, r11, -0x1b68
	ctx.r[7].s64 = ctx.r[11].s64 + -7016;
	// 825EF228: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 825EF22C: 388AB020  addi r4, r10, -0x4fe0
	ctx.r[4].s64 = ctx.r[10].s64 + -20448;
	// 825EF230: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825EF234: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825EF238: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 825EF23C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825EF240: 386AB274  addi r3, r10, -0x4d8c
	ctx.r[3].s64 = ctx.r[10].s64 + -19852;
	// 825EF244: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 825EF248: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825EF24C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825EF250: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825EF254: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825EF258: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825EF25C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825EF260: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825EF264: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825EF268: 4BE77BB9  bl 0x82466e20
	ctx.lr = 0x825EF26C;
	sub_82466E20(ctx, base);
	// 825EF26C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825EF270: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825EF274: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825EF278: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825EF280(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825EF280 size=24
    let mut pc: u32 = 0x825EF280;
    'dispatch: loop {
        match pc {
            0x825EF280 => {
    //   block [0x825EF280..0x825EF298)
	// 825EF280: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825EF284: 3D408274  lis r10, -0x7d8c
	ctx.r[10].s64 = -2106327040;
	// 825EF288: 394AE4C8  addi r10, r10, -0x1b38
	ctx.r[10].s64 = ctx.r[10].s64 + -6968;
	// 825EF28C: 816B84E4  lwz r11, -0x7b1c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-31516 as u32) ) } as u64;
	// 825EF290: 916A0020  stw r11, 0x20(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 825EF294: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825EF298(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825EF298 size=108
    let mut pc: u32 = 0x825EF298;
    'dispatch: loop {
        match pc {
            0x825EF298 => {
    //   block [0x825EF298..0x825EF304)
	// 825EF298: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825EF29C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825EF2A0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825EF2A4: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825EF2A8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825EF2AC: 38EBE4C8  addi r7, r11, -0x1b38
	ctx.r[7].s64 = ctx.r[11].s64 + -6968;
	// 825EF2B0: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 825EF2B4: 388A9C48  addi r4, r10, -0x63b8
	ctx.r[4].s64 = ctx.r[10].s64 + -25528;
	// 825EF2B8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825EF2BC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825EF2C0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 825EF2C4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825EF2C8: 386AB2A4  addi r3, r10, -0x4d5c
	ctx.r[3].s64 = ctx.r[10].s64 + -19804;
	// 825EF2CC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 825EF2D0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825EF2D4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825EF2D8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825EF2DC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825EF2E0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825EF2E4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825EF2E8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825EF2EC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825EF2F0: 4BE77B31  bl 0x82466e20
	ctx.lr = 0x825EF2F4;
	sub_82466E20(ctx, base);
	// 825EF2F4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825EF2F8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825EF2FC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825EF300: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825EF308(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825EF308 size=108
    let mut pc: u32 = 0x825EF308;
    'dispatch: loop {
        match pc {
            0x825EF308 => {
    //   block [0x825EF308..0x825EF374)
	// 825EF308: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825EF30C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825EF310: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825EF314: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825EF318: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825EF31C: 38EB86E0  addi r7, r11, -0x7920
	ctx.r[7].s64 = ctx.r[11].s64 + -31008;
	// 825EF320: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 825EF324: 388AA080  addi r4, r10, -0x5f80
	ctx.r[4].s64 = ctx.r[10].s64 + -24448;
	// 825EF328: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825EF32C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825EF330: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 825EF334: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825EF338: 386AB2D4  addi r3, r10, -0x4d2c
	ctx.r[3].s64 = ctx.r[10].s64 + -19756;
	// 825EF33C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 825EF340: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825EF344: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825EF348: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825EF34C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825EF350: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825EF354: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825EF358: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825EF35C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825EF360: 4BE77AC1  bl 0x82466e20
	ctx.lr = 0x825EF364;
	sub_82466E20(ctx, base);
	// 825EF364: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825EF368: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825EF36C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825EF370: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825EF378(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825EF378 size=24
    let mut pc: u32 = 0x825EF378;
    'dispatch: loop {
        match pc {
            0x825EF378 => {
    //   block [0x825EF378..0x825EF390)
	// 825EF378: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825EF37C: 3D408274  lis r10, -0x7d8c
	ctx.r[10].s64 = -2106327040;
	// 825EF380: 394AE4F8  addi r10, r10, -0x1b08
	ctx.r[10].s64 = ctx.r[10].s64 + -6920;
	// 825EF384: 816B84E4  lwz r11, -0x7b1c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-31516 as u32) ) } as u64;
	// 825EF388: 916A0020  stw r11, 0x20(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 825EF38C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825EF390(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825EF390 size=108
    let mut pc: u32 = 0x825EF390;
    'dispatch: loop {
        match pc {
            0x825EF390 => {
    //   block [0x825EF390..0x825EF3FC)
	// 825EF390: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825EF394: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825EF398: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825EF39C: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825EF3A0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825EF3A4: 38EBE4F8  addi r7, r11, -0x1b08
	ctx.r[7].s64 = ctx.r[11].s64 + -6920;
	// 825EF3A8: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 825EF3AC: 388A9BE8  addi r4, r10, -0x6418
	ctx.r[4].s64 = ctx.r[10].s64 + -25624;
	// 825EF3B0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825EF3B4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825EF3B8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 825EF3BC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825EF3C0: 386AB304  addi r3, r10, -0x4cfc
	ctx.r[3].s64 = ctx.r[10].s64 + -19708;
	// 825EF3C4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 825EF3C8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825EF3CC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825EF3D0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825EF3D4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825EF3D8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825EF3DC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825EF3E0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825EF3E4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825EF3E8: 4BE77A39  bl 0x82466e20
	ctx.lr = 0x825EF3EC;
	sub_82466E20(ctx, base);
	// 825EF3EC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825EF3F0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825EF3F4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825EF3F8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825EF400(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825EF400 size=112
    let mut pc: u32 = 0x825EF400;
    'dispatch: loop {
        match pc {
            0x825EF400 => {
    //   block [0x825EF400..0x825EF470)
	// 825EF400: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825EF404: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825EF408: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825EF40C: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825EF410: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825EF414: 392A15BC  addi r9, r10, 0x15bc
	ctx.r[9].s64 = ctx.r[10].s64 + 5564;
	// 825EF418: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825EF41C: 390B86F8  addi r8, r11, -0x7908
	ctx.r[8].s64 = ctx.r[11].s64 + -30984;
	// 825EF420: 38E00002  li r7, 2
	ctx.r[7].s64 = 2;
	// 825EF424: 388AA778  addi r4, r10, -0x5888
	ctx.r[4].s64 = ctx.r[10].s64 + -22664;
	// 825EF428: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825EF42C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825EF430: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825EF434: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825EF438: 386AB334  addi r3, r10, -0x4ccc
	ctx.r[3].s64 = ctx.r[10].s64 + -19660;
	// 825EF43C: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 825EF440: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 825EF444: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825EF448: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825EF44C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825EF450: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825EF454: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825EF458: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825EF45C: 4BE779C5  bl 0x82466e20
	ctx.lr = 0x825EF460;
	sub_82466E20(ctx, base);
	// 825EF460: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825EF464: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825EF468: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825EF46C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825EF470(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825EF470 size=108
    let mut pc: u32 = 0x825EF470;
    'dispatch: loop {
        match pc {
            0x825EF470 => {
    //   block [0x825EF470..0x825EF4DC)
	// 825EF470: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825EF474: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825EF478: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825EF47C: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825EF480: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825EF484: 38EB8728  addi r7, r11, -0x78d8
	ctx.r[7].s64 = ctx.r[11].s64 + -30936;
	// 825EF488: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 825EF48C: 388AA350  addi r4, r10, -0x5cb0
	ctx.r[4].s64 = ctx.r[10].s64 + -23728;
	// 825EF490: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825EF494: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825EF498: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 825EF49C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825EF4A0: 386AB364  addi r3, r10, -0x4c9c
	ctx.r[3].s64 = ctx.r[10].s64 + -19612;
	// 825EF4A4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 825EF4A8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825EF4AC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825EF4B0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825EF4B4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825EF4B8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825EF4BC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825EF4C0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825EF4C4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825EF4C8: 4BE77959  bl 0x82466e20
	ctx.lr = 0x825EF4CC;
	sub_82466E20(ctx, base);
	// 825EF4CC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825EF4D0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825EF4D4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825EF4D8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825EF4E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825EF4E0 size=108
    let mut pc: u32 = 0x825EF4E0;
    'dispatch: loop {
        match pc {
            0x825EF4E0 => {
    //   block [0x825EF4E0..0x825EF54C)
	// 825EF4E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825EF4E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825EF4E8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825EF4EC: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825EF4F0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825EF4F4: 38EB8758  addi r7, r11, -0x78a8
	ctx.r[7].s64 = ctx.r[11].s64 + -30888;
	// 825EF4F8: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 825EF4FC: 388AB2D8  addi r4, r10, -0x4d28
	ctx.r[4].s64 = ctx.r[10].s64 + -19752;
	// 825EF500: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825EF504: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825EF508: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 825EF50C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825EF510: 386AB394  addi r3, r10, -0x4c6c
	ctx.r[3].s64 = ctx.r[10].s64 + -19564;
	// 825EF514: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 825EF518: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825EF51C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825EF520: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825EF524: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825EF528: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825EF52C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825EF530: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825EF534: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825EF538: 4BE778E9  bl 0x82466e20
	ctx.lr = 0x825EF53C;
	sub_82466E20(ctx, base);
	// 825EF53C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825EF540: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825EF544: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825EF548: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825EF550(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825EF550 size=108
    let mut pc: u32 = 0x825EF550;
    'dispatch: loop {
        match pc {
            0x825EF550 => {
    //   block [0x825EF550..0x825EF5BC)
	// 825EF550: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825EF554: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825EF558: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825EF55C: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825EF560: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825EF564: 38EB8770  addi r7, r11, -0x7890
	ctx.r[7].s64 = ctx.r[11].s64 + -30864;
	// 825EF568: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 825EF56C: 388AAA44  addi r4, r10, -0x55bc
	ctx.r[4].s64 = ctx.r[10].s64 + -21948;
	// 825EF570: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825EF574: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825EF578: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 825EF57C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825EF580: 386AB3C4  addi r3, r10, -0x4c3c
	ctx.r[3].s64 = ctx.r[10].s64 + -19516;
	// 825EF584: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 825EF588: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825EF58C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825EF590: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825EF594: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825EF598: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825EF59C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825EF5A0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825EF5A4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825EF5A8: 4BE77879  bl 0x82466e20
	ctx.lr = 0x825EF5AC;
	sub_82466E20(ctx, base);
	// 825EF5AC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825EF5B0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825EF5B4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825EF5B8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825EF5C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825EF5C0 size=112
    let mut pc: u32 = 0x825EF5C0;
    'dispatch: loop {
        match pc {
            0x825EF5C0 => {
    //   block [0x825EF5C0..0x825EF630)
	// 825EF5C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825EF5C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825EF5C8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825EF5CC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825EF5D0: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825EF5D4: 38AAB424  addi r5, r10, -0x4bdc
	ctx.r[5].s64 = ctx.r[10].s64 + -19420;
	// 825EF5D8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825EF5DC: 390B87A0  addi r8, r11, -0x7860
	ctx.r[8].s64 = ctx.r[11].s64 + -30816;
	// 825EF5E0: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 825EF5E4: 388AA9A0  addi r4, r10, -0x5660
	ctx.r[4].s64 = ctx.r[10].s64 + -22112;
	// 825EF5E8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825EF5EC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825EF5F0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825EF5F4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825EF5F8: 386AB3F4  addi r3, r10, -0x4c0c
	ctx.r[3].s64 = ctx.r[10].s64 + -19468;
	// 825EF5FC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 825EF600: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825EF604: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825EF608: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825EF60C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825EF610: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825EF614: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825EF618: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825EF61C: 4BE77805  bl 0x82466e20
	ctx.lr = 0x825EF620;
	sub_82466E20(ctx, base);
	// 825EF620: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825EF624: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825EF628: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825EF62C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825EF630(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825EF630 size=108
    let mut pc: u32 = 0x825EF630;
    'dispatch: loop {
        match pc {
            0x825EF630 => {
    //   block [0x825EF630..0x825EF69C)
	// 825EF630: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825EF634: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825EF638: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825EF63C: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825EF640: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825EF644: 38EB87B8  addi r7, r11, -0x7848
	ctx.r[7].s64 = ctx.r[11].s64 + -30792;
	// 825EF648: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 825EF64C: 388AA640  addi r4, r10, -0x59c0
	ctx.r[4].s64 = ctx.r[10].s64 + -22976;
	// 825EF650: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825EF654: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825EF658: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 825EF65C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825EF660: 386AB424  addi r3, r10, -0x4bdc
	ctx.r[3].s64 = ctx.r[10].s64 + -19420;
	// 825EF664: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 825EF668: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825EF66C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825EF670: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825EF674: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825EF678: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825EF67C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825EF680: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825EF684: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825EF688: 4BE77799  bl 0x82466e20
	ctx.lr = 0x825EF68C;
	sub_82466E20(ctx, base);
	// 825EF68C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825EF690: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825EF694: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825EF698: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825EF6A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825EF6A0 size=108
    let mut pc: u32 = 0x825EF6A0;
    'dispatch: loop {
        match pc {
            0x825EF6A0 => {
    //   block [0x825EF6A0..0x825EF70C)
	// 825EF6A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825EF6A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825EF6A8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825EF6AC: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825EF6B0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825EF6B4: 38EB87E8  addi r7, r11, -0x7818
	ctx.r[7].s64 = ctx.r[11].s64 + -30744;
	// 825EF6B8: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 825EF6BC: 388AA2E0  addi r4, r10, -0x5d20
	ctx.r[4].s64 = ctx.r[10].s64 + -23840;
	// 825EF6C0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825EF6C4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825EF6C8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 825EF6CC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825EF6D0: 386AB454  addi r3, r10, -0x4bac
	ctx.r[3].s64 = ctx.r[10].s64 + -19372;
	// 825EF6D4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 825EF6D8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825EF6DC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825EF6E0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825EF6E4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825EF6E8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825EF6EC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825EF6F0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825EF6F4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825EF6F8: 4BE77729  bl 0x82466e20
	ctx.lr = 0x825EF6FC;
	sub_82466E20(ctx, base);
	// 825EF6FC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825EF700: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825EF704: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825EF708: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825EF710(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825EF710 size=108
    let mut pc: u32 = 0x825EF710;
    'dispatch: loop {
        match pc {
            0x825EF710 => {
    //   block [0x825EF710..0x825EF77C)
	// 825EF710: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825EF714: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825EF718: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825EF71C: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825EF720: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825EF724: 38EB8800  addi r7, r11, -0x7800
	ctx.r[7].s64 = ctx.r[11].s64 + -30720;
	// 825EF728: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 825EF72C: 388AA304  addi r4, r10, -0x5cfc
	ctx.r[4].s64 = ctx.r[10].s64 + -23804;
	// 825EF730: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825EF734: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825EF738: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 825EF73C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825EF740: 386AB484  addi r3, r10, -0x4b7c
	ctx.r[3].s64 = ctx.r[10].s64 + -19324;
	// 825EF744: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 825EF748: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825EF74C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825EF750: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825EF754: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825EF758: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825EF75C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825EF760: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825EF764: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825EF768: 4BE776B9  bl 0x82466e20
	ctx.lr = 0x825EF76C;
	sub_82466E20(ctx, base);
	// 825EF76C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825EF770: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825EF774: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825EF778: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825EF780(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825EF780 size=108
    let mut pc: u32 = 0x825EF780;
    'dispatch: loop {
        match pc {
            0x825EF780 => {
    //   block [0x825EF780..0x825EF7EC)
	// 825EF780: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825EF784: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825EF788: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825EF78C: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825EF790: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825EF794: 38EB8830  addi r7, r11, -0x77d0
	ctx.r[7].s64 = ctx.r[11].s64 + -30672;
	// 825EF798: 39000007  li r8, 7
	ctx.r[8].s64 = 7;
	// 825EF79C: 388AAED0  addi r4, r10, -0x5130
	ctx.r[4].s64 = ctx.r[10].s64 + -20784;
	// 825EF7A0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825EF7A4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825EF7A8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 825EF7AC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825EF7B0: 386AB4B4  addi r3, r10, -0x4b4c
	ctx.r[3].s64 = ctx.r[10].s64 + -19276;
	// 825EF7B4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 825EF7B8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825EF7BC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825EF7C0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825EF7C4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825EF7C8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825EF7CC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825EF7D0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825EF7D4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825EF7D8: 4BE77649  bl 0x82466e20
	ctx.lr = 0x825EF7DC;
	sub_82466E20(ctx, base);
	// 825EF7DC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825EF7E0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825EF7E4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825EF7E8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825EF7F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825EF7F0 size=108
    let mut pc: u32 = 0x825EF7F0;
    'dispatch: loop {
        match pc {
            0x825EF7F0 => {
    //   block [0x825EF7F0..0x825EF85C)
	// 825EF7F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825EF7F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825EF7F8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825EF7FC: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825EF800: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825EF804: 38EB88D8  addi r7, r11, -0x7728
	ctx.r[7].s64 = ctx.r[11].s64 + -30504;
	// 825EF808: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 825EF80C: 388AA130  addi r4, r10, -0x5ed0
	ctx.r[4].s64 = ctx.r[10].s64 + -24272;
	// 825EF810: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825EF814: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825EF818: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 825EF81C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825EF820: 386AB4E4  addi r3, r10, -0x4b1c
	ctx.r[3].s64 = ctx.r[10].s64 + -19228;
	// 825EF824: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 825EF828: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825EF82C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825EF830: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825EF834: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825EF838: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825EF83C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825EF840: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825EF844: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825EF848: 4BE775D9  bl 0x82466e20
	ctx.lr = 0x825EF84C;
	sub_82466E20(ctx, base);
	// 825EF84C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825EF850: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825EF854: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825EF858: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825EF860(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825EF860 size=108
    let mut pc: u32 = 0x825EF860;
    'dispatch: loop {
        match pc {
            0x825EF860 => {
    //   block [0x825EF860..0x825EF8CC)
	// 825EF860: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825EF864: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825EF868: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825EF86C: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825EF870: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825EF874: 38EB8908  addi r7, r11, -0x76f8
	ctx.r[7].s64 = ctx.r[11].s64 + -30456;
	// 825EF878: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 825EF87C: 388AA148  addi r4, r10, -0x5eb8
	ctx.r[4].s64 = ctx.r[10].s64 + -24248;
	// 825EF880: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825EF884: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825EF888: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 825EF88C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825EF890: 386AB514  addi r3, r10, -0x4aec
	ctx.r[3].s64 = ctx.r[10].s64 + -19180;
	// 825EF894: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 825EF898: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825EF89C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825EF8A0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825EF8A4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825EF8A8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825EF8AC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825EF8B0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825EF8B4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825EF8B8: 4BE77569  bl 0x82466e20
	ctx.lr = 0x825EF8BC;
	sub_82466E20(ctx, base);
	// 825EF8BC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825EF8C0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825EF8C4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825EF8C8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825EF8D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825EF8D0 size=108
    let mut pc: u32 = 0x825EF8D0;
    'dispatch: loop {
        match pc {
            0x825EF8D0 => {
    //   block [0x825EF8D0..0x825EF93C)
	// 825EF8D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825EF8D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825EF8D8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825EF8DC: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825EF8E0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825EF8E4: 38EB8920  addi r7, r11, -0x76e0
	ctx.r[7].s64 = ctx.r[11].s64 + -30432;
	// 825EF8E8: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 825EF8EC: 388AB290  addi r4, r10, -0x4d70
	ctx.r[4].s64 = ctx.r[10].s64 + -19824;
	// 825EF8F0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825EF8F4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825EF8F8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 825EF8FC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825EF900: 386AB544  addi r3, r10, -0x4abc
	ctx.r[3].s64 = ctx.r[10].s64 + -19132;
	// 825EF904: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 825EF908: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825EF90C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825EF910: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825EF914: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825EF918: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825EF91C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825EF920: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825EF924: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825EF928: 4BE774F9  bl 0x82466e20
	ctx.lr = 0x825EF92C;
	sub_82466E20(ctx, base);
	// 825EF92C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825EF930: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825EF934: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825EF938: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825EF940(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825EF940 size=112
    let mut pc: u32 = 0x825EF940;
    'dispatch: loop {
        match pc {
            0x825EF940 => {
    //   block [0x825EF940..0x825EF9B0)
	// 825EF940: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825EF944: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825EF948: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825EF94C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825EF950: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825EF954: 38AAB394  addi r5, r10, -0x4c6c
	ctx.r[5].s64 = ctx.r[10].s64 + -19564;
	// 825EF958: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825EF95C: 390B8950  addi r8, r11, -0x76b0
	ctx.r[8].s64 = ctx.r[11].s64 + -30384;
	// 825EF960: 39200007  li r9, 7
	ctx.r[9].s64 = 7;
	// 825EF964: 388AB2A8  addi r4, r10, -0x4d58
	ctx.r[4].s64 = ctx.r[10].s64 + -19800;
	// 825EF968: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825EF96C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825EF970: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825EF974: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825EF978: 386AB574  addi r3, r10, -0x4a8c
	ctx.r[3].s64 = ctx.r[10].s64 + -19084;
	// 825EF97C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 825EF980: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825EF984: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825EF988: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825EF98C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825EF990: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825EF994: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825EF998: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825EF99C: 4BE77485  bl 0x82466e20
	ctx.lr = 0x825EF9A0;
	sub_82466E20(ctx, base);
	// 825EF9A0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825EF9A4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825EF9A8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825EF9AC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825EF9B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825EF9B0 size=24
    let mut pc: u32 = 0x825EF9B0;
    'dispatch: loop {
        match pc {
            0x825EF9B0 => {
    //   block [0x825EF9B0..0x825EF9C8)
	// 825EF9B0: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825EF9B4: 3D408274  lis r10, -0x7d8c
	ctx.r[10].s64 = -2106327040;
	// 825EF9B8: 394AE528  addi r10, r10, -0x1ad8
	ctx.r[10].s64 = ctx.r[10].s64 + -6872;
	// 825EF9BC: 816B89F8  lwz r11, -0x7608(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-30216 as u32) ) } as u64;
	// 825EF9C0: 916A0008  stw r11, 8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 825EF9C4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825EF9C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825EF9C8 size=112
    let mut pc: u32 = 0x825EF9C8;
    'dispatch: loop {
        match pc {
            0x825EF9C8 => {
    //   block [0x825EF9C8..0x825EFA38)
	// 825EF9C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825EF9CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825EF9D0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825EF9D4: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825EF9D8: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825EF9DC: 392A15E8  addi r9, r10, 0x15e8
	ctx.r[9].s64 = ctx.r[10].s64 + 5608;
	// 825EF9E0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825EF9E4: 390BE528  addi r8, r11, -0x1ad8
	ctx.r[8].s64 = ctx.r[11].s64 + -6872;
	// 825EF9E8: 38E00005  li r7, 5
	ctx.r[7].s64 = 5;
	// 825EF9EC: 388AB098  addi r4, r10, -0x4f68
	ctx.r[4].s64 = ctx.r[10].s64 + -20328;
	// 825EF9F0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825EF9F4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825EF9F8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825EF9FC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825EFA00: 386AB5A4  addi r3, r10, -0x4a5c
	ctx.r[3].s64 = ctx.r[10].s64 + -19036;
	// 825EFA04: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 825EFA08: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 825EFA0C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825EFA10: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825EFA14: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825EFA18: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825EFA1C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825EFA20: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825EFA24: 4BE773FD  bl 0x82466e20
	ctx.lr = 0x825EFA28;
	sub_82466E20(ctx, base);
	// 825EFA28: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825EFA2C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825EFA30: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825EFA34: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825EFA38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825EFA38 size=108
    let mut pc: u32 = 0x825EFA38;
    'dispatch: loop {
        match pc {
            0x825EFA38 => {
    //   block [0x825EFA38..0x825EFAA4)
	// 825EFA38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825EFA3C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825EFA40: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825EFA44: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825EFA48: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825EFA4C: 38EB8A00  addi r7, r11, -0x7600
	ctx.r[7].s64 = ctx.r[11].s64 + -30208;
	// 825EFA50: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 825EFA54: 388AAC70  addi r4, r10, -0x5390
	ctx.r[4].s64 = ctx.r[10].s64 + -21392;
	// 825EFA58: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825EFA5C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825EFA60: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 825EFA64: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825EFA68: 386AB5D4  addi r3, r10, -0x4a2c
	ctx.r[3].s64 = ctx.r[10].s64 + -18988;
	// 825EFA6C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 825EFA70: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825EFA74: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825EFA78: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825EFA7C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825EFA80: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825EFA84: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825EFA88: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825EFA8C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825EFA90: 4BE77391  bl 0x82466e20
	ctx.lr = 0x825EFA94;
	sub_82466E20(ctx, base);
	// 825EFA94: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825EFA98: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825EFA9C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825EFAA0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825EFAA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825EFAA8 size=116
    let mut pc: u32 = 0x825EFAA8;
    'dispatch: loop {
        match pc {
            0x825EFAA8 => {
    //   block [0x825EFAA8..0x825EFB1C)
	// 825EFAA8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825EFAAC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825EFAB0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825EFAB4: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825EFAB8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825EFABC: 390B8A30  addi r8, r11, -0x75d0
	ctx.r[8].s64 = ctx.r[11].s64 + -30160;
	// 825EFAC0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825EFAC4: 392A162C  addi r9, r10, 0x162c
	ctx.r[9].s64 = ctx.r[10].s64 + 5676;
	// 825EFAC8: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825EFACC: 38E00008  li r7, 8
	ctx.r[7].s64 = 8;
	// 825EFAD0: 38AAB394  addi r5, r10, -0x4c6c
	ctx.r[5].s64 = ctx.r[10].s64 + -19564;
	// 825EFAD4: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825EFAD8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825EFADC: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825EFAE0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825EFAE4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825EFAE8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825EFAEC: 3D608293  lis r11, -0x7d6d
	ctx.r[11].s64 = -2104295424;
	// 825EFAF0: 388AAC88  addi r4, r10, -0x5378
	ctx.r[4].s64 = ctx.r[10].s64 + -21368;
	// 825EFAF4: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 825EFAF8: 386BB604  addi r3, r11, -0x49fc
	ctx.r[3].s64 = ctx.r[11].s64 + -18940;
	// 825EFAFC: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 825EFB00: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825EFB04: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825EFB08: 4BE77319  bl 0x82466e20
	ctx.lr = 0x825EFB0C;
	sub_82466E20(ctx, base);
	// 825EFB0C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825EFB10: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825EFB14: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825EFB18: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825EFB20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825EFB20 size=24
    let mut pc: u32 = 0x825EFB20;
    'dispatch: loop {
        match pc {
            0x825EFB20 => {
    //   block [0x825EFB20..0x825EFB38)
	// 825EFB20: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825EFB24: 3D408274  lis r10, -0x7d8c
	ctx.r[10].s64 = -2106327040;
	// 825EFB28: 394AE5A0  addi r10, r10, -0x1a60
	ctx.r[10].s64 = ctx.r[10].s64 + -6752;
	// 825EFB2C: 816B8AF0  lwz r11, -0x7510(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-29968 as u32) ) } as u64;
	// 825EFB30: 916A0020  stw r11, 0x20(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 825EFB34: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825EFB38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825EFB38 size=112
    let mut pc: u32 = 0x825EFB38;
    'dispatch: loop {
        match pc {
            0x825EFB38 => {
    //   block [0x825EFB38..0x825EFBA8)
	// 825EFB38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825EFB3C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825EFB40: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825EFB44: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825EFB48: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825EFB4C: 392A1668  addi r9, r10, 0x1668
	ctx.r[9].s64 = ctx.r[10].s64 + 5736;
	// 825EFB50: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825EFB54: 390BE5A0  addi r8, r11, -0x1a60
	ctx.r[8].s64 = ctx.r[11].s64 + -6752;
	// 825EFB58: 38E00003  li r7, 3
	ctx.r[7].s64 = 3;
	// 825EFB5C: 388AB004  addi r4, r10, -0x4ffc
	ctx.r[4].s64 = ctx.r[10].s64 + -20476;
	// 825EFB60: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825EFB64: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825EFB68: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825EFB6C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825EFB70: 386AB634  addi r3, r10, -0x49cc
	ctx.r[3].s64 = ctx.r[10].s64 + -18892;
	// 825EFB74: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 825EFB78: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 825EFB7C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825EFB80: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825EFB84: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825EFB88: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825EFB8C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825EFB90: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825EFB94: 4BE7728D  bl 0x82466e20
	ctx.lr = 0x825EFB98;
	sub_82466E20(ctx, base);
	// 825EFB98: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825EFB9C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825EFBA0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825EFBA4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825EFBA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825EFBA8 size=108
    let mut pc: u32 = 0x825EFBA8;
    'dispatch: loop {
        match pc {
            0x825EFBA8 => {
    //   block [0x825EFBA8..0x825EFC14)
	// 825EFBA8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825EFBAC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825EFBB0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825EFBB4: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825EFBB8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825EFBBC: 38EB8AF4  addi r7, r11, -0x750c
	ctx.r[7].s64 = ctx.r[11].s64 + -29964;
	// 825EFBC0: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 825EFBC4: 388AAF00  addi r4, r10, -0x5100
	ctx.r[4].s64 = ctx.r[10].s64 + -20736;
	// 825EFBC8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825EFBCC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825EFBD0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 825EFBD4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825EFBD8: 386AB664  addi r3, r10, -0x499c
	ctx.r[3].s64 = ctx.r[10].s64 + -18844;
	// 825EFBDC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 825EFBE0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825EFBE4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825EFBE8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825EFBEC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825EFBF0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825EFBF4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825EFBF8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825EFBFC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825EFC00: 4BE77221  bl 0x82466e20
	ctx.lr = 0x825EFC04;
	sub_82466E20(ctx, base);
	// 825EFC04: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825EFC08: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825EFC0C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825EFC10: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825EFC18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825EFC18 size=108
    let mut pc: u32 = 0x825EFC18;
    'dispatch: loop {
        match pc {
            0x825EFC18 => {
    //   block [0x825EFC18..0x825EFC84)
	// 825EFC18: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825EFC1C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825EFC20: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825EFC24: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825EFC28: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825EFC2C: 38EB8B0C  addi r7, r11, -0x74f4
	ctx.r[7].s64 = ctx.r[11].s64 + -29940;
	// 825EFC30: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 825EFC34: 388A9D38  addi r4, r10, -0x62c8
	ctx.r[4].s64 = ctx.r[10].s64 + -25288;
	// 825EFC38: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825EFC3C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825EFC40: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 825EFC44: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825EFC48: 386AB694  addi r3, r10, -0x496c
	ctx.r[3].s64 = ctx.r[10].s64 + -18796;
	// 825EFC4C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 825EFC50: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825EFC54: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825EFC58: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825EFC5C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825EFC60: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825EFC64: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825EFC68: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825EFC6C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825EFC70: 4BE771B1  bl 0x82466e20
	ctx.lr = 0x825EFC74;
	sub_82466E20(ctx, base);
	// 825EFC74: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825EFC78: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825EFC7C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825EFC80: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825EFC88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825EFC88 size=24
    let mut pc: u32 = 0x825EFC88;
    'dispatch: loop {
        match pc {
            0x825EFC88 => {
    //   block [0x825EFC88..0x825EFCA0)
	// 825EFC88: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825EFC8C: 3D408274  lis r10, -0x7d8c
	ctx.r[10].s64 = -2106327040;
	// 825EFC90: 394AE5E8  addi r10, r10, -0x1a18
	ctx.r[10].s64 = ctx.r[10].s64 + -6680;
	// 825EFC94: 816B8B3C  lwz r11, -0x74c4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-29892 as u32) ) } as u64;
	// 825EFC98: 916A0008  stw r11, 8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 825EFC9C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825EFCA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825EFCA0 size=112
    let mut pc: u32 = 0x825EFCA0;
    'dispatch: loop {
        match pc {
            0x825EFCA0 => {
    //   block [0x825EFCA0..0x825EFD10)
	// 825EFCA0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825EFCA4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825EFCA8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825EFCAC: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825EFCB0: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825EFCB4: 392A16A4  addi r9, r10, 0x16a4
	ctx.r[9].s64 = ctx.r[10].s64 + 5796;
	// 825EFCB8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825EFCBC: 390BE5E8  addi r8, r11, -0x1a18
	ctx.r[8].s64 = ctx.r[11].s64 + -6680;
	// 825EFCC0: 38E00005  li r7, 5
	ctx.r[7].s64 = 5;
	// 825EFCC4: 388AB450  addi r4, r10, -0x4bb0
	ctx.r[4].s64 = ctx.r[10].s64 + -19376;
	// 825EFCC8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825EFCCC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825EFCD0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825EFCD4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825EFCD8: 386AB6C4  addi r3, r10, -0x493c
	ctx.r[3].s64 = ctx.r[10].s64 + -18748;
	// 825EFCDC: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 825EFCE0: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 825EFCE4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825EFCE8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825EFCEC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825EFCF0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825EFCF4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825EFCF8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825EFCFC: 4BE77125  bl 0x82466e20
	ctx.lr = 0x825EFD00;
	sub_82466E20(ctx, base);
	// 825EFD00: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825EFD04: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825EFD08: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825EFD0C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825EFD10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825EFD10 size=112
    let mut pc: u32 = 0x825EFD10;
    'dispatch: loop {
        match pc {
            0x825EFD10 => {
    //   block [0x825EFD10..0x825EFD80)
	// 825EFD10: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825EFD14: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825EFD18: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825EFD1C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825EFD20: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825EFD24: 38AAB394  addi r5, r10, -0x4c6c
	ctx.r[5].s64 = ctx.r[10].s64 + -19564;
	// 825EFD28: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825EFD2C: 390B8B40  addi r8, r11, -0x74c0
	ctx.r[8].s64 = ctx.r[11].s64 + -29888;
	// 825EFD30: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 825EFD34: 388A9CDC  addi r4, r10, -0x6324
	ctx.r[4].s64 = ctx.r[10].s64 + -25380;
	// 825EFD38: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825EFD3C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825EFD40: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825EFD44: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825EFD48: 386AB6F4  addi r3, r10, -0x490c
	ctx.r[3].s64 = ctx.r[10].s64 + -18700;
	// 825EFD4C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 825EFD50: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825EFD54: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825EFD58: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825EFD5C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825EFD60: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825EFD64: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825EFD68: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825EFD6C: 4BE770B5  bl 0x82466e20
	ctx.lr = 0x825EFD70;
	sub_82466E20(ctx, base);
	// 825EFD70: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825EFD74: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825EFD78: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825EFD7C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825EFD80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825EFD80 size=108
    let mut pc: u32 = 0x825EFD80;
    'dispatch: loop {
        match pc {
            0x825EFD80 => {
    //   block [0x825EFD80..0x825EFDEC)
	// 825EFD80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825EFD84: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825EFD88: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825EFD8C: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825EFD90: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825EFD94: 38EB8B70  addi r7, r11, -0x7490
	ctx.r[7].s64 = ctx.r[11].s64 + -29840;
	// 825EFD98: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 825EFD9C: 388A9CF4  addi r4, r10, -0x630c
	ctx.r[4].s64 = ctx.r[10].s64 + -25356;
	// 825EFDA0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825EFDA4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825EFDA8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 825EFDAC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825EFDB0: 386AB724  addi r3, r10, -0x48dc
	ctx.r[3].s64 = ctx.r[10].s64 + -18652;
	// 825EFDB4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 825EFDB8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825EFDBC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825EFDC0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825EFDC4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825EFDC8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825EFDCC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825EFDD0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825EFDD4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825EFDD8: 4BE77049  bl 0x82466e20
	ctx.lr = 0x825EFDDC;
	sub_82466E20(ctx, base);
	// 825EFDDC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825EFDE0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825EFDE4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825EFDE8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825EFDF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825EFDF0 size=108
    let mut pc: u32 = 0x825EFDF0;
    'dispatch: loop {
        match pc {
            0x825EFDF0 => {
    //   block [0x825EFDF0..0x825EFE5C)
	// 825EFDF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825EFDF4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825EFDF8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825EFDFC: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825EFE00: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825EFE04: 38EB8BA0  addi r7, r11, -0x7460
	ctx.r[7].s64 = ctx.r[11].s64 + -29792;
	// 825EFE08: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 825EFE0C: 388AB148  addi r4, r10, -0x4eb8
	ctx.r[4].s64 = ctx.r[10].s64 + -20152;
	// 825EFE10: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825EFE14: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825EFE18: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 825EFE1C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825EFE20: 386AB754  addi r3, r10, -0x48ac
	ctx.r[3].s64 = ctx.r[10].s64 + -18604;
	// 825EFE24: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 825EFE28: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825EFE2C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825EFE30: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825EFE34: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825EFE38: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825EFE3C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825EFE40: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825EFE44: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825EFE48: 4BE76FD9  bl 0x82466e20
	ctx.lr = 0x825EFE4C;
	sub_82466E20(ctx, base);
	// 825EFE4C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825EFE50: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825EFE54: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825EFE58: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825EFE60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825EFE60 size=108
    let mut pc: u32 = 0x825EFE60;
    'dispatch: loop {
        match pc {
            0x825EFE60 => {
    //   block [0x825EFE60..0x825EFECC)
	// 825EFE60: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825EFE64: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825EFE68: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825EFE6C: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825EFE70: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825EFE74: 38EB8C00  addi r7, r11, -0x7400
	ctx.r[7].s64 = ctx.r[11].s64 + -29696;
	// 825EFE78: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 825EFE7C: 388AA9FC  addi r4, r10, -0x5604
	ctx.r[4].s64 = ctx.r[10].s64 + -22020;
	// 825EFE80: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825EFE84: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825EFE88: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 825EFE8C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825EFE90: 386AB784  addi r3, r10, -0x487c
	ctx.r[3].s64 = ctx.r[10].s64 + -18556;
	// 825EFE94: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 825EFE98: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825EFE9C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825EFEA0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825EFEA4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825EFEA8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825EFEAC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825EFEB0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825EFEB4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825EFEB8: 4BE76F69  bl 0x82466e20
	ctx.lr = 0x825EFEBC;
	sub_82466E20(ctx, base);
	// 825EFEBC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825EFEC0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825EFEC4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825EFEC8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825EFED0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825EFED0 size=108
    let mut pc: u32 = 0x825EFED0;
    'dispatch: loop {
        match pc {
            0x825EFED0 => {
    //   block [0x825EFED0..0x825EFF3C)
	// 825EFED0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825EFED4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825EFED8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825EFEDC: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825EFEE0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825EFEE4: 38EB8C30  addi r7, r11, -0x73d0
	ctx.r[7].s64 = ctx.r[11].s64 + -29648;
	// 825EFEE8: 3900000C  li r8, 0xc
	ctx.r[8].s64 = 12;
	// 825EFEEC: 388AA978  addi r4, r10, -0x5688
	ctx.r[4].s64 = ctx.r[10].s64 + -22152;
	// 825EFEF0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825EFEF4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825EFEF8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 825EFEFC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825EFF00: 386AB7B4  addi r3, r10, -0x484c
	ctx.r[3].s64 = ctx.r[10].s64 + -18508;
	// 825EFF04: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 825EFF08: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825EFF0C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825EFF10: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825EFF14: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825EFF18: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825EFF1C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825EFF20: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825EFF24: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825EFF28: 4BE76EF9  bl 0x82466e20
	ctx.lr = 0x825EFF2C;
	sub_82466E20(ctx, base);
	// 825EFF2C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825EFF30: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825EFF34: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825EFF38: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825EFF40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825EFF40 size=108
    let mut pc: u32 = 0x825EFF40;
    'dispatch: loop {
        match pc {
            0x825EFF40 => {
    //   block [0x825EFF40..0x825EFFAC)
	// 825EFF40: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825EFF44: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825EFF48: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825EFF4C: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825EFF50: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825EFF54: 38EB8D50  addi r7, r11, -0x72b0
	ctx.r[7].s64 = ctx.r[11].s64 + -29360;
	// 825EFF58: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 825EFF5C: 388AA044  addi r4, r10, -0x5fbc
	ctx.r[4].s64 = ctx.r[10].s64 + -24508;
	// 825EFF60: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825EFF64: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825EFF68: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 825EFF6C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825EFF70: 386AB7E4  addi r3, r10, -0x481c
	ctx.r[3].s64 = ctx.r[10].s64 + -18460;
	// 825EFF74: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 825EFF78: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825EFF7C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825EFF80: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825EFF84: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825EFF88: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825EFF8C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825EFF90: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825EFF94: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825EFF98: 4BE76E89  bl 0x82466e20
	ctx.lr = 0x825EFF9C;
	sub_82466E20(ctx, base);
	// 825EFF9C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825EFFA0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825EFFA4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825EFFA8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825EFFB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825EFFB0 size=108
    let mut pc: u32 = 0x825EFFB0;
    'dispatch: loop {
        match pc {
            0x825EFFB0 => {
    //   block [0x825EFFB0..0x825F001C)
	// 825EFFB0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825EFFB4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825EFFB8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825EFFBC: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825EFFC0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825EFFC4: 38EB8D68  addi r7, r11, -0x7298
	ctx.r[7].s64 = ctx.r[11].s64 + -29336;
	// 825EFFC8: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 825EFFCC: 388AAF3C  addi r4, r10, -0x50c4
	ctx.r[4].s64 = ctx.r[10].s64 + -20676;
	// 825EFFD0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825EFFD4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825EFFD8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 825EFFDC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825EFFE0: 386AB814  addi r3, r10, -0x47ec
	ctx.r[3].s64 = ctx.r[10].s64 + -18412;
	// 825EFFE4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 825EFFE8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825EFFEC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825EFFF0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825EFFF4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825EFFF8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825EFFFC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F0000: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F0004: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825F0008: 4BE76E19  bl 0x82466e20
	ctx.lr = 0x825F000C;
	sub_82466E20(ctx, base);
	// 825F000C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F0010: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F0014: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F0018: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F0020(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F0020 size=108
    let mut pc: u32 = 0x825F0020;
    'dispatch: loop {
        match pc {
            0x825F0020 => {
    //   block [0x825F0020..0x825F008C)
	// 825F0020: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F0024: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F0028: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F002C: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825F0030: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F0034: 38EB8D80  addi r7, r11, -0x7280
	ctx.r[7].s64 = ctx.r[11].s64 + -29312;
	// 825F0038: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 825F003C: 388AA0C0  addi r4, r10, -0x5f40
	ctx.r[4].s64 = ctx.r[10].s64 + -24384;
	// 825F0040: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825F0044: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F0048: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 825F004C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825F0050: 386AB844  addi r3, r10, -0x47bc
	ctx.r[3].s64 = ctx.r[10].s64 + -18364;
	// 825F0054: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 825F0058: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825F005C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F0060: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825F0064: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F0068: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825F006C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F0070: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F0074: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825F0078: 4BE76DA9  bl 0x82466e20
	ctx.lr = 0x825F007C;
	sub_82466E20(ctx, base);
	// 825F007C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F0080: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F0084: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F0088: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F0090(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F0090 size=108
    let mut pc: u32 = 0x825F0090;
    'dispatch: loop {
        match pc {
            0x825F0090 => {
    //   block [0x825F0090..0x825F00FC)
	// 825F0090: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F0094: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F0098: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F009C: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825F00A0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F00A4: 38EB8D98  addi r7, r11, -0x7268
	ctx.r[7].s64 = ctx.r[11].s64 + -29288;
	// 825F00A8: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 825F00AC: 388A9C24  addi r4, r10, -0x63dc
	ctx.r[4].s64 = ctx.r[10].s64 + -25564;
	// 825F00B0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825F00B4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F00B8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 825F00BC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825F00C0: 386AB874  addi r3, r10, -0x478c
	ctx.r[3].s64 = ctx.r[10].s64 + -18316;
	// 825F00C4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 825F00C8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825F00CC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F00D0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825F00D4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F00D8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825F00DC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F00E0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F00E4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825F00E8: 4BE76D39  bl 0x82466e20
	ctx.lr = 0x825F00EC;
	sub_82466E20(ctx, base);
	// 825F00EC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F00F0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F00F4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F00F8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F0100(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F0100 size=108
    let mut pc: u32 = 0x825F0100;
    'dispatch: loop {
        match pc {
            0x825F0100 => {
    //   block [0x825F0100..0x825F016C)
	// 825F0100: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F0104: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F0108: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F010C: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825F0110: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F0114: 38EB8DB0  addi r7, r11, -0x7250
	ctx.r[7].s64 = ctx.r[11].s64 + -29264;
	// 825F0118: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 825F011C: 388AA684  addi r4, r10, -0x597c
	ctx.r[4].s64 = ctx.r[10].s64 + -22908;
	// 825F0120: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825F0124: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F0128: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 825F012C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825F0130: 386AB8A4  addi r3, r10, -0x475c
	ctx.r[3].s64 = ctx.r[10].s64 + -18268;
	// 825F0134: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 825F0138: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825F013C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F0140: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825F0144: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F0148: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825F014C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F0150: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F0154: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825F0158: 4BE76CC9  bl 0x82466e20
	ctx.lr = 0x825F015C;
	sub_82466E20(ctx, base);
	// 825F015C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F0160: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F0164: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F0168: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F0170(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F0170 size=108
    let mut pc: u32 = 0x825F0170;
    'dispatch: loop {
        match pc {
            0x825F0170 => {
    //   block [0x825F0170..0x825F01DC)
	// 825F0170: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F0174: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F0178: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F017C: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825F0180: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F0184: 38EB8DC8  addi r7, r11, -0x7238
	ctx.r[7].s64 = ctx.r[11].s64 + -29240;
	// 825F0188: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 825F018C: 388AA26C  addi r4, r10, -0x5d94
	ctx.r[4].s64 = ctx.r[10].s64 + -23956;
	// 825F0190: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825F0194: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F0198: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 825F019C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825F01A0: 386AB8D4  addi r3, r10, -0x472c
	ctx.r[3].s64 = ctx.r[10].s64 + -18220;
	// 825F01A4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 825F01A8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825F01AC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F01B0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825F01B4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F01B8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825F01BC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F01C0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F01C4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825F01C8: 4BE76C59  bl 0x82466e20
	ctx.lr = 0x825F01CC;
	sub_82466E20(ctx, base);
	// 825F01CC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F01D0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F01D4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F01D8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F01E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F01E0 size=108
    let mut pc: u32 = 0x825F01E0;
    'dispatch: loop {
        match pc {
            0x825F01E0 => {
    //   block [0x825F01E0..0x825F024C)
	// 825F01E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F01E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F01E8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F01EC: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825F01F0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F01F4: 38EB8DE0  addi r7, r11, -0x7220
	ctx.r[7].s64 = ctx.r[11].s64 + -29216;
	// 825F01F8: 39000006  li r8, 6
	ctx.r[8].s64 = 6;
	// 825F01FC: 388AB380  addi r4, r10, -0x4c80
	ctx.r[4].s64 = ctx.r[10].s64 + -19584;
	// 825F0200: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825F0204: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F0208: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 825F020C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825F0210: 386AB904  addi r3, r10, -0x46fc
	ctx.r[3].s64 = ctx.r[10].s64 + -18172;
	// 825F0214: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 825F0218: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825F021C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F0220: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825F0224: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F0228: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825F022C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F0230: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F0234: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825F0238: 4BE76BE9  bl 0x82466e20
	ctx.lr = 0x825F023C;
	sub_82466E20(ctx, base);
	// 825F023C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F0240: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F0244: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F0248: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F0250(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F0250 size=108
    let mut pc: u32 = 0x825F0250;
    'dispatch: loop {
        match pc {
            0x825F0250 => {
    //   block [0x825F0250..0x825F02BC)
	// 825F0250: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F0254: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F0258: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F025C: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825F0260: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F0264: 38EB8E70  addi r7, r11, -0x7190
	ctx.r[7].s64 = ctx.r[11].s64 + -29072;
	// 825F0268: 39000008  li r8, 8
	ctx.r[8].s64 = 8;
	// 825F026C: 388AA228  addi r4, r10, -0x5dd8
	ctx.r[4].s64 = ctx.r[10].s64 + -24024;
	// 825F0270: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825F0274: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F0278: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 825F027C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825F0280: 386AB934  addi r3, r10, -0x46cc
	ctx.r[3].s64 = ctx.r[10].s64 + -18124;
	// 825F0284: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 825F0288: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825F028C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F0290: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825F0294: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F0298: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825F029C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F02A0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F02A4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825F02A8: 4BE76B79  bl 0x82466e20
	ctx.lr = 0x825F02AC;
	sub_82466E20(ctx, base);
	// 825F02AC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F02B0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F02B4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F02B8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F02C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F02C0 size=108
    let mut pc: u32 = 0x825F02C0;
    'dispatch: loop {
        match pc {
            0x825F02C0 => {
    //   block [0x825F02C0..0x825F032C)
	// 825F02C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F02C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F02C8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F02CC: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825F02D0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F02D4: 38EB8F30  addi r7, r11, -0x70d0
	ctx.r[7].s64 = ctx.r[11].s64 + -28880;
	// 825F02D8: 39000009  li r8, 9
	ctx.r[8].s64 = 9;
	// 825F02DC: 388AA000  addi r4, r10, -0x6000
	ctx.r[4].s64 = ctx.r[10].s64 + -24576;
	// 825F02E0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825F02E4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F02E8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 825F02EC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825F02F0: 386AB964  addi r3, r10, -0x469c
	ctx.r[3].s64 = ctx.r[10].s64 + -18076;
	// 825F02F4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 825F02F8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825F02FC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F0300: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825F0304: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F0308: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825F030C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F0310: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F0314: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825F0318: 4BE76B09  bl 0x82466e20
	ctx.lr = 0x825F031C;
	sub_82466E20(ctx, base);
	// 825F031C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F0320: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F0324: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F0328: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F0330(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F0330 size=108
    let mut pc: u32 = 0x825F0330;
    'dispatch: loop {
        match pc {
            0x825F0330 => {
    //   block [0x825F0330..0x825F039C)
	// 825F0330: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F0334: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F0338: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F033C: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825F0340: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F0344: 38EB9008  addi r7, r11, -0x6ff8
	ctx.r[7].s64 = ctx.r[11].s64 + -28664;
	// 825F0348: 39000008  li r8, 8
	ctx.r[8].s64 = 8;
	// 825F034C: 388A9F00  addi r4, r10, -0x6100
	ctx.r[4].s64 = ctx.r[10].s64 + -24832;
	// 825F0350: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825F0354: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F0358: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 825F035C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825F0360: 386AB994  addi r3, r10, -0x466c
	ctx.r[3].s64 = ctx.r[10].s64 + -18028;
	// 825F0364: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 825F0368: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825F036C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F0370: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825F0374: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F0378: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825F037C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F0380: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F0384: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825F0388: 4BE76A99  bl 0x82466e20
	ctx.lr = 0x825F038C;
	sub_82466E20(ctx, base);
	// 825F038C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F0390: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F0394: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F0398: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F03A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F03A0 size=108
    let mut pc: u32 = 0x825F03A0;
    'dispatch: loop {
        match pc {
            0x825F03A0 => {
    //   block [0x825F03A0..0x825F040C)
	// 825F03A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F03A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F03A8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F03AC: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825F03B0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F03B4: 38EB90C8  addi r7, r11, -0x6f38
	ctx.r[7].s64 = ctx.r[11].s64 + -28472;
	// 825F03B8: 39000007  li r8, 7
	ctx.r[8].s64 = 7;
	// 825F03BC: 388AADE0  addi r4, r10, -0x5220
	ctx.r[4].s64 = ctx.r[10].s64 + -21024;
	// 825F03C0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825F03C4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F03C8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 825F03CC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825F03D0: 386AB9C4  addi r3, r10, -0x463c
	ctx.r[3].s64 = ctx.r[10].s64 + -17980;
	// 825F03D4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 825F03D8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825F03DC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F03E0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825F03E4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F03E8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825F03EC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F03F0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F03F4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825F03F8: 4BE76A29  bl 0x82466e20
	ctx.lr = 0x825F03FC;
	sub_82466E20(ctx, base);
	// 825F03FC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F0400: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F0404: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F0408: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F0410(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F0410 size=112
    let mut pc: u32 = 0x825F0410;
    'dispatch: loop {
        match pc {
            0x825F0410 => {
    //   block [0x825F0410..0x825F0480)
	// 825F0410: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F0414: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F0418: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F041C: 3D408274  lis r10, -0x7d8c
	ctx.r[10].s64 = -2106327040;
	// 825F0420: 3900000C  li r8, 0xc
	ctx.r[8].s64 = 12;
	// 825F0424: 38EA9170  addi r7, r10, -0x6e90
	ctx.r[7].s64 = ctx.r[10].s64 + -28304;
	// 825F0428: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F042C: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 825F0430: 388AA5FC  addi r4, r10, -0x5a04
	ctx.r[4].s64 = ctx.r[10].s64 + -23044;
	// 825F0434: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825F0438: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 825F043C: 396B16B8  addi r11, r11, 0x16b8
	ctx.r[11].s64 = ctx.r[11].s64 + 5816;
	// 825F0440: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 825F0444: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F0448: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F044C: 386AB9F4  addi r3, r10, -0x460c
	ctx.r[3].s64 = ctx.r[10].s64 + -17932;
	// 825F0450: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825F0454: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 825F0458: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F045C: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 825F0460: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F0464: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F0468: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825F046C: 4BE769B5  bl 0x82466e20
	ctx.lr = 0x825F0470;
	sub_82466E20(ctx, base);
	// 825F0470: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F0474: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F0478: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F047C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F0480(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F0480 size=108
    let mut pc: u32 = 0x825F0480;
    'dispatch: loop {
        match pc {
            0x825F0480 => {
    //   block [0x825F0480..0x825F04EC)
	// 825F0480: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F0484: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F0488: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F048C: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825F0490: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F0494: 38EB9290  addi r7, r11, -0x6d70
	ctx.r[7].s64 = ctx.r[11].s64 + -28016;
	// 825F0498: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 825F049C: 388AAD28  addi r4, r10, -0x52d8
	ctx.r[4].s64 = ctx.r[10].s64 + -21208;
	// 825F04A0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825F04A4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F04A8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 825F04AC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825F04B0: 386ABA24  addi r3, r10, -0x45dc
	ctx.r[3].s64 = ctx.r[10].s64 + -17884;
	// 825F04B4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 825F04B8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825F04BC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F04C0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825F04C4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F04C8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825F04CC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F04D0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F04D4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825F04D8: 4BE76949  bl 0x82466e20
	ctx.lr = 0x825F04DC;
	sub_82466E20(ctx, base);
	// 825F04DC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F04E0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F04E4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F04E8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F04F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F04F0 size=108
    let mut pc: u32 = 0x825F04F0;
    'dispatch: loop {
        match pc {
            0x825F04F0 => {
    //   block [0x825F04F0..0x825F055C)
	// 825F04F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F04F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F04F8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F04FC: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825F0500: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F0504: 38EB92F0  addi r7, r11, -0x6d10
	ctx.r[7].s64 = ctx.r[11].s64 + -27920;
	// 825F0508: 3900000B  li r8, 0xb
	ctx.r[8].s64 = 11;
	// 825F050C: 388A9070  addi r4, r10, -0x6f90
	ctx.r[4].s64 = ctx.r[10].s64 + -28560;
	// 825F0510: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825F0514: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F0518: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 825F051C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825F0520: 386ABA54  addi r3, r10, -0x45ac
	ctx.r[3].s64 = ctx.r[10].s64 + -17836;
	// 825F0524: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 825F0528: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825F052C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F0530: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825F0534: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F0538: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825F053C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F0540: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F0544: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825F0548: 4BE768D9  bl 0x82466e20
	ctx.lr = 0x825F054C;
	sub_82466E20(ctx, base);
	// 825F054C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F0550: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F0554: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F0558: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F0560(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F0560 size=108
    let mut pc: u32 = 0x825F0560;
    'dispatch: loop {
        match pc {
            0x825F0560 => {
    //   block [0x825F0560..0x825F05CC)
	// 825F0560: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F0564: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F0568: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F056C: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825F0570: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F0574: 38EB93F8  addi r7, r11, -0x6c08
	ctx.r[7].s64 = ctx.r[11].s64 + -27656;
	// 825F0578: 39000009  li r8, 9
	ctx.r[8].s64 = 9;
	// 825F057C: 388A8E98  addi r4, r10, -0x7168
	ctx.r[4].s64 = ctx.r[10].s64 + -29032;
	// 825F0580: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825F0584: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F0588: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 825F058C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825F0590: 386ABA84  addi r3, r10, -0x457c
	ctx.r[3].s64 = ctx.r[10].s64 + -17788;
	// 825F0594: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 825F0598: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825F059C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F05A0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825F05A4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F05A8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825F05AC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F05B0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F05B4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825F05B8: 4BE76869  bl 0x82466e20
	ctx.lr = 0x825F05BC;
	sub_82466E20(ctx, base);
	// 825F05BC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F05C0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F05C4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F05C8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F05D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F05D0 size=108
    let mut pc: u32 = 0x825F05D0;
    'dispatch: loop {
        match pc {
            0x825F05D0 => {
    //   block [0x825F05D0..0x825F063C)
	// 825F05D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F05D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F05D8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F05DC: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825F05E0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F05E4: 38EB94D0  addi r7, r11, -0x6b30
	ctx.r[7].s64 = ctx.r[11].s64 + -27440;
	// 825F05E8: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 825F05EC: 388A8CF0  addi r4, r10, -0x7310
	ctx.r[4].s64 = ctx.r[10].s64 + -29456;
	// 825F05F0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825F05F4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F05F8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 825F05FC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825F0600: 386ABAB4  addi r3, r10, -0x454c
	ctx.r[3].s64 = ctx.r[10].s64 + -17740;
	// 825F0604: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 825F0608: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825F060C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F0610: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825F0614: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F0618: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825F061C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F0620: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F0624: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825F0628: 4BE767F9  bl 0x82466e20
	ctx.lr = 0x825F062C;
	sub_82466E20(ctx, base);
	// 825F062C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F0630: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F0634: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F0638: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F0640(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F0640 size=108
    let mut pc: u32 = 0x825F0640;
    'dispatch: loop {
        match pc {
            0x825F0640 => {
    //   block [0x825F0640..0x825F06AC)
	// 825F0640: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F0644: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F0648: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F064C: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825F0650: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F0654: 38EB9518  addi r7, r11, -0x6ae8
	ctx.r[7].s64 = ctx.r[11].s64 + -27368;
	// 825F0658: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 825F065C: 388A8D14  addi r4, r10, -0x72ec
	ctx.r[4].s64 = ctx.r[10].s64 + -29420;
	// 825F0660: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825F0664: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F0668: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 825F066C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825F0670: 386ABAE4  addi r3, r10, -0x451c
	ctx.r[3].s64 = ctx.r[10].s64 + -17692;
	// 825F0674: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 825F0678: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825F067C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F0680: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825F0684: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F0688: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825F068C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F0690: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F0694: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825F0698: 4BE76789  bl 0x82466e20
	ctx.lr = 0x825F069C;
	sub_82466E20(ctx, base);
	// 825F069C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F06A0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F06A4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F06A8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F06B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F06B0 size=108
    let mut pc: u32 = 0x825F06B0;
    'dispatch: loop {
        match pc {
            0x825F06B0 => {
    //   block [0x825F06B0..0x825F071C)
	// 825F06B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F06B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F06B8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F06BC: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825F06C0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F06C4: 38EB9530  addi r7, r11, -0x6ad0
	ctx.r[7].s64 = ctx.r[11].s64 + -27344;
	// 825F06C8: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 825F06CC: 388A37C4  addi r4, r10, 0x37c4
	ctx.r[4].s64 = ctx.r[10].s64 + 14276;
	// 825F06D0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825F06D4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F06D8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 825F06DC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825F06E0: 386ABB14  addi r3, r10, -0x44ec
	ctx.r[3].s64 = ctx.r[10].s64 + -17644;
	// 825F06E4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 825F06E8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825F06EC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F06F0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825F06F4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F06F8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825F06FC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F0700: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F0704: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825F0708: 4BE76719  bl 0x82466e20
	ctx.lr = 0x825F070C;
	sub_82466E20(ctx, base);
	// 825F070C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F0710: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F0714: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F0718: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F0720(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F0720 size=108
    let mut pc: u32 = 0x825F0720;
    'dispatch: loop {
        match pc {
            0x825F0720 => {
    //   block [0x825F0720..0x825F078C)
	// 825F0720: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F0724: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F0728: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F072C: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825F0730: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F0734: 38EB9578  addi r7, r11, -0x6a88
	ctx.r[7].s64 = ctx.r[11].s64 + -27272;
	// 825F0738: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 825F073C: 388A37F4  addi r4, r10, 0x37f4
	ctx.r[4].s64 = ctx.r[10].s64 + 14324;
	// 825F0740: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825F0744: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F0748: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 825F074C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825F0750: 386ABB44  addi r3, r10, -0x44bc
	ctx.r[3].s64 = ctx.r[10].s64 + -17596;
	// 825F0754: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 825F0758: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825F075C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F0760: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825F0764: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F0768: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825F076C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F0770: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F0774: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825F0778: 4BE766A9  bl 0x82466e20
	ctx.lr = 0x825F077C;
	sub_82466E20(ctx, base);
	// 825F077C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F0780: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F0784: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F0788: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F0790(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F0790 size=112
    let mut pc: u32 = 0x825F0790;
    'dispatch: loop {
        match pc {
            0x825F0790 => {
    //   block [0x825F0790..0x825F0800)
	// 825F0790: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F0794: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F0798: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F079C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F07A0: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825F07A4: 38AAAF74  addi r5, r10, -0x508c
	ctx.r[5].s64 = ctx.r[10].s64 + -20620;
	// 825F07A8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F07AC: 390B9590  addi r8, r11, -0x6a70
	ctx.r[8].s64 = ctx.r[11].s64 + -27248;
	// 825F07B0: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 825F07B4: 388A3810  addi r4, r10, 0x3810
	ctx.r[4].s64 = ctx.r[10].s64 + 14352;
	// 825F07B8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825F07BC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F07C0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825F07C4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F07C8: 386ABB74  addi r3, r10, -0x448c
	ctx.r[3].s64 = ctx.r[10].s64 + -17548;
	// 825F07CC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 825F07D0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825F07D4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825F07D8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825F07DC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F07E0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825F07E4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F07E8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F07EC: 4BE76635  bl 0x82466e20
	ctx.lr = 0x825F07F0;
	sub_82466E20(ctx, base);
	// 825F07F0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F07F4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F07F8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F07FC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F0800(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F0800 size=112
    let mut pc: u32 = 0x825F0800;
    'dispatch: loop {
        match pc {
            0x825F0800 => {
    //   block [0x825F0800..0x825F0870)
	// 825F0800: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F0804: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F0808: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F080C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F0810: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825F0814: 38AAAF74  addi r5, r10, -0x508c
	ctx.r[5].s64 = ctx.r[10].s64 + -20620;
	// 825F0818: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F081C: 390B95D8  addi r8, r11, -0x6a28
	ctx.r[8].s64 = ctx.r[11].s64 + -27176;
	// 825F0820: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 825F0824: 388A3824  addi r4, r10, 0x3824
	ctx.r[4].s64 = ctx.r[10].s64 + 14372;
	// 825F0828: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825F082C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F0830: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825F0834: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F0838: 386ABBA4  addi r3, r10, -0x445c
	ctx.r[3].s64 = ctx.r[10].s64 + -17500;
	// 825F083C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 825F0840: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825F0844: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825F0848: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825F084C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F0850: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825F0854: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F0858: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F085C: 4BE765C5  bl 0x82466e20
	ctx.lr = 0x825F0860;
	sub_82466E20(ctx, base);
	// 825F0860: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F0864: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F0868: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F086C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F0870(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F0870 size=108
    let mut pc: u32 = 0x825F0870;
    'dispatch: loop {
        match pc {
            0x825F0870 => {
    //   block [0x825F0870..0x825F08DC)
	// 825F0870: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F0874: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F0878: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F087C: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825F0880: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F0884: 38EB9620  addi r7, r11, -0x69e0
	ctx.r[7].s64 = ctx.r[11].s64 + -27104;
	// 825F0888: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 825F088C: 388A3838  addi r4, r10, 0x3838
	ctx.r[4].s64 = ctx.r[10].s64 + 14392;
	// 825F0890: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825F0894: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F0898: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 825F089C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825F08A0: 386ABBD4  addi r3, r10, -0x442c
	ctx.r[3].s64 = ctx.r[10].s64 + -17452;
	// 825F08A4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 825F08A8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825F08AC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F08B0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825F08B4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F08B8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825F08BC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F08C0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F08C4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825F08C8: 4BE76559  bl 0x82466e20
	ctx.lr = 0x825F08CC;
	sub_82466E20(ctx, base);
	// 825F08CC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F08D0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F08D4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F08D8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F08E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825F08E0 size=24
    let mut pc: u32 = 0x825F08E0;
    'dispatch: loop {
        match pc {
            0x825F08E0 => {
    //   block [0x825F08E0..0x825F08F8)
	// 825F08E0: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825F08E4: 3D408274  lis r10, -0x7d8c
	ctx.r[10].s64 = -2106327040;
	// 825F08E8: 394AE660  addi r10, r10, -0x19a0
	ctx.r[10].s64 = ctx.r[10].s64 + -6560;
	// 825F08EC: 816B9B00  lwz r11, -0x6500(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-25856 as u32) ) } as u64;
	// 825F08F0: 916A0068  stw r11, 0x68(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(104 as u32), ctx.r[11].u32 ) };
	// 825F08F4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F08F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F08F8 size=112
    let mut pc: u32 = 0x825F08F8;
    'dispatch: loop {
        match pc {
            0x825F08F8 => {
    //   block [0x825F08F8..0x825F0968)
	// 825F08F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F08FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F0900: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F0904: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F0908: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825F090C: 38AABDE4  addi r5, r10, -0x421c
	ctx.r[5].s64 = ctx.r[10].s64 + -16924;
	// 825F0910: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F0914: 390BE660  addi r8, r11, -0x19a0
	ctx.r[8].s64 = ctx.r[11].s64 + -6560;
	// 825F0918: 39200008  li r9, 8
	ctx.r[9].s64 = 8;
	// 825F091C: 388A3888  addi r4, r10, 0x3888
	ctx.r[4].s64 = ctx.r[10].s64 + 14472;
	// 825F0920: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825F0924: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F0928: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825F092C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F0930: 386ABC04  addi r3, r10, -0x43fc
	ctx.r[3].s64 = ctx.r[10].s64 + -17404;
	// 825F0934: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 825F0938: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825F093C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825F0940: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825F0944: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F0948: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825F094C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F0950: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F0954: 4BE764CD  bl 0x82466e20
	ctx.lr = 0x825F0958;
	sub_82466E20(ctx, base);
	// 825F0958: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F095C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F0960: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F0964: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F0968(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F0968 size=108
    let mut pc: u32 = 0x825F0968;
    'dispatch: loop {
        match pc {
            0x825F0968 => {
    //   block [0x825F0968..0x825F09D4)
	// 825F0968: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F096C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F0970: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F0974: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825F0978: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F097C: 38EB9638  addi r7, r11, -0x69c8
	ctx.r[7].s64 = ctx.r[11].s64 + -27080;
	// 825F0980: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 825F0984: 388A38A0  addi r4, r10, 0x38a0
	ctx.r[4].s64 = ctx.r[10].s64 + 14496;
	// 825F0988: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825F098C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F0990: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 825F0994: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825F0998: 386ABC34  addi r3, r10, -0x43cc
	ctx.r[3].s64 = ctx.r[10].s64 + -17356;
	// 825F099C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 825F09A0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825F09A4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F09A8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825F09AC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F09B0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825F09B4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F09B8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F09BC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825F09C0: 4BE76461  bl 0x82466e20
	ctx.lr = 0x825F09C4;
	sub_82466E20(ctx, base);
	// 825F09C4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F09C8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F09CC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F09D0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F09D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F09D8 size=112
    let mut pc: u32 = 0x825F09D8;
    'dispatch: loop {
        match pc {
            0x825F09D8 => {
    //   block [0x825F09D8..0x825F0A48)
	// 825F09D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F09DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F09E0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F09E4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F09E8: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825F09EC: 38AABDE4  addi r5, r10, -0x421c
	ctx.r[5].s64 = ctx.r[10].s64 + -16924;
	// 825F09F0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F09F4: 390B9698  addi r8, r11, -0x6968
	ctx.r[8].s64 = ctx.r[11].s64 + -26984;
	// 825F09F8: 39200008  li r9, 8
	ctx.r[9].s64 = 8;
	// 825F09FC: 388A38BC  addi r4, r10, 0x38bc
	ctx.r[4].s64 = ctx.r[10].s64 + 14524;
	// 825F0A00: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825F0A04: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F0A08: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825F0A0C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F0A10: 386ABC64  addi r3, r10, -0x439c
	ctx.r[3].s64 = ctx.r[10].s64 + -17308;
	// 825F0A14: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 825F0A18: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825F0A1C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825F0A20: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825F0A24: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F0A28: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825F0A2C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F0A30: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F0A34: 4BE763ED  bl 0x82466e20
	ctx.lr = 0x825F0A38;
	sub_82466E20(ctx, base);
	// 825F0A38: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F0A3C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F0A40: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F0A44: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F0A48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F0A48 size=108
    let mut pc: u32 = 0x825F0A48;
    'dispatch: loop {
        match pc {
            0x825F0A48 => {
    //   block [0x825F0A48..0x825F0AB4)
	// 825F0A48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F0A4C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F0A50: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F0A54: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825F0A58: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F0A5C: 38EB9758  addi r7, r11, -0x68a8
	ctx.r[7].s64 = ctx.r[11].s64 + -26792;
	// 825F0A60: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 825F0A64: 388A38D0  addi r4, r10, 0x38d0
	ctx.r[4].s64 = ctx.r[10].s64 + 14544;
	// 825F0A68: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825F0A6C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F0A70: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 825F0A74: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825F0A78: 386ABC94  addi r3, r10, -0x436c
	ctx.r[3].s64 = ctx.r[10].s64 + -17260;
	// 825F0A7C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 825F0A80: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825F0A84: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F0A88: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825F0A8C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F0A90: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825F0A94: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F0A98: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F0A9C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825F0AA0: 4BE76381  bl 0x82466e20
	ctx.lr = 0x825F0AA4;
	sub_82466E20(ctx, base);
	// 825F0AA4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F0AA8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F0AAC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F0AB0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F0AB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F0AB8 size=108
    let mut pc: u32 = 0x825F0AB8;
    'dispatch: loop {
        match pc {
            0x825F0AB8 => {
    //   block [0x825F0AB8..0x825F0B24)
	// 825F0AB8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F0ABC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F0AC0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F0AC4: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825F0AC8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F0ACC: 38EB97B8  addi r7, r11, -0x6848
	ctx.r[7].s64 = ctx.r[11].s64 + -26696;
	// 825F0AD0: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 825F0AD4: 388A38E0  addi r4, r10, 0x38e0
	ctx.r[4].s64 = ctx.r[10].s64 + 14560;
	// 825F0AD8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825F0ADC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F0AE0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 825F0AE4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825F0AE8: 386ABCC4  addi r3, r10, -0x433c
	ctx.r[3].s64 = ctx.r[10].s64 + -17212;
	// 825F0AEC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 825F0AF0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825F0AF4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F0AF8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825F0AFC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F0B00: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825F0B04: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F0B08: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F0B0C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825F0B10: 4BE76311  bl 0x82466e20
	ctx.lr = 0x825F0B14;
	sub_82466E20(ctx, base);
	// 825F0B14: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F0B18: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F0B1C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F0B20: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F0B28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F0B28 size=108
    let mut pc: u32 = 0x825F0B28;
    'dispatch: loop {
        match pc {
            0x825F0B28 => {
    //   block [0x825F0B28..0x825F0B94)
	// 825F0B28: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F0B2C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F0B30: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F0B34: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825F0B38: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F0B3C: 38EB9800  addi r7, r11, -0x6800
	ctx.r[7].s64 = ctx.r[11].s64 + -26624;
	// 825F0B40: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 825F0B44: 388A3904  addi r4, r10, 0x3904
	ctx.r[4].s64 = ctx.r[10].s64 + 14596;
	// 825F0B48: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825F0B4C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F0B50: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 825F0B54: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825F0B58: 386ABCF4  addi r3, r10, -0x430c
	ctx.r[3].s64 = ctx.r[10].s64 + -17164;
	// 825F0B5C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 825F0B60: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825F0B64: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F0B68: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825F0B6C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F0B70: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825F0B74: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F0B78: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F0B7C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825F0B80: 4BE762A1  bl 0x82466e20
	ctx.lr = 0x825F0B84;
	sub_82466E20(ctx, base);
	// 825F0B84: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F0B88: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F0B8C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F0B90: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F0B98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F0B98 size=112
    let mut pc: u32 = 0x825F0B98;
    'dispatch: loop {
        match pc {
            0x825F0B98 => {
    //   block [0x825F0B98..0x825F0C08)
	// 825F0B98: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F0B9C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F0BA0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F0BA4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F0BA8: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825F0BAC: 38AABCF4  addi r5, r10, -0x430c
	ctx.r[5].s64 = ctx.r[10].s64 + -17164;
	// 825F0BB0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F0BB4: 390B9848  addi r8, r11, -0x67b8
	ctx.r[8].s64 = ctx.r[11].s64 + -26552;
	// 825F0BB8: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 825F0BBC: 388A3928  addi r4, r10, 0x3928
	ctx.r[4].s64 = ctx.r[10].s64 + 14632;
	// 825F0BC0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825F0BC4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F0BC8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825F0BCC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F0BD0: 386ABD24  addi r3, r10, -0x42dc
	ctx.r[3].s64 = ctx.r[10].s64 + -17116;
	// 825F0BD4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 825F0BD8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825F0BDC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825F0BE0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825F0BE4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F0BE8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825F0BEC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F0BF0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F0BF4: 4BE7622D  bl 0x82466e20
	ctx.lr = 0x825F0BF8;
	sub_82466E20(ctx, base);
	// 825F0BF8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F0BFC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F0C00: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F0C04: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F0C08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F0C08 size=108
    let mut pc: u32 = 0x825F0C08;
    'dispatch: loop {
        match pc {
            0x825F0C08 => {
    //   block [0x825F0C08..0x825F0C74)
	// 825F0C08: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F0C0C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F0C10: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F0C14: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825F0C18: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F0C1C: 38EB9878  addi r7, r11, -0x6788
	ctx.r[7].s64 = ctx.r[11].s64 + -26504;
	// 825F0C20: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 825F0C24: 388A394C  addi r4, r10, 0x394c
	ctx.r[4].s64 = ctx.r[10].s64 + 14668;
	// 825F0C28: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825F0C2C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F0C30: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 825F0C34: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825F0C38: 386ABD54  addi r3, r10, -0x42ac
	ctx.r[3].s64 = ctx.r[10].s64 + -17068;
	// 825F0C3C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 825F0C40: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825F0C44: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F0C48: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825F0C4C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F0C50: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825F0C54: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F0C58: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F0C5C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825F0C60: 4BE761C1  bl 0x82466e20
	ctx.lr = 0x825F0C64;
	sub_82466E20(ctx, base);
	// 825F0C64: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F0C68: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F0C6C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F0C70: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F0C78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F0C78 size=108
    let mut pc: u32 = 0x825F0C78;
    'dispatch: loop {
        match pc {
            0x825F0C78 => {
    //   block [0x825F0C78..0x825F0CE4)
	// 825F0C78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F0C7C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F0C80: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F0C84: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825F0C88: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F0C8C: 38EB98C0  addi r7, r11, -0x6740
	ctx.r[7].s64 = ctx.r[11].s64 + -26432;
	// 825F0C90: 39000008  li r8, 8
	ctx.r[8].s64 = 8;
	// 825F0C94: 388A3968  addi r4, r10, 0x3968
	ctx.r[4].s64 = ctx.r[10].s64 + 14696;
	// 825F0C98: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825F0C9C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F0CA0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 825F0CA4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825F0CA8: 386ABD84  addi r3, r10, -0x427c
	ctx.r[3].s64 = ctx.r[10].s64 + -17020;
	// 825F0CAC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 825F0CB0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825F0CB4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F0CB8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825F0CBC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F0CC0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825F0CC4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F0CC8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F0CCC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825F0CD0: 4BE76151  bl 0x82466e20
	ctx.lr = 0x825F0CD4;
	sub_82466E20(ctx, base);
	// 825F0CD4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F0CD8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F0CDC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F0CE0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F0CE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F0CE8 size=108
    let mut pc: u32 = 0x825F0CE8;
    'dispatch: loop {
        match pc {
            0x825F0CE8 => {
    //   block [0x825F0CE8..0x825F0D54)
	// 825F0CE8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F0CEC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F0CF0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F0CF4: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825F0CF8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F0CFC: 38EB9980  addi r7, r11, -0x6680
	ctx.r[7].s64 = ctx.r[11].s64 + -26240;
	// 825F0D00: 39000010  li r8, 0x10
	ctx.r[8].s64 = 16;
	// 825F0D04: 388A3980  addi r4, r10, 0x3980
	ctx.r[4].s64 = ctx.r[10].s64 + 14720;
	// 825F0D08: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825F0D0C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F0D10: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 825F0D14: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825F0D18: 386ABDB4  addi r3, r10, -0x424c
	ctx.r[3].s64 = ctx.r[10].s64 + -16972;
	// 825F0D1C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 825F0D20: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825F0D24: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F0D28: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825F0D2C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F0D30: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825F0D34: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F0D38: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F0D3C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825F0D40: 4BE760E1  bl 0x82466e20
	ctx.lr = 0x825F0D44;
	sub_82466E20(ctx, base);
	// 825F0D44: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F0D48: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F0D4C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F0D50: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F0D58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F0D58 size=112
    let mut pc: u32 = 0x825F0D58;
    'dispatch: loop {
        match pc {
            0x825F0D58 => {
    //   block [0x825F0D58..0x825F0DC8)
	// 825F0D58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F0D5C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F0D60: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F0D64: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F0D68: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825F0D6C: 38AAAF74  addi r5, r10, -0x508c
	ctx.r[5].s64 = ctx.r[10].s64 + -20620;
	// 825F0D70: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F0D74: 390B9B08  addi r8, r11, -0x64f8
	ctx.r[8].s64 = ctx.r[11].s64 + -25848;
	// 825F0D78: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 825F0D7C: 388A3990  addi r4, r10, 0x3990
	ctx.r[4].s64 = ctx.r[10].s64 + 14736;
	// 825F0D80: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825F0D84: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F0D88: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825F0D8C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F0D90: 386ABDE4  addi r3, r10, -0x421c
	ctx.r[3].s64 = ctx.r[10].s64 + -16924;
	// 825F0D94: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 825F0D98: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825F0D9C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825F0DA0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825F0DA4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F0DA8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825F0DAC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F0DB0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F0DB4: 4BE7606D  bl 0x82466e20
	ctx.lr = 0x825F0DB8;
	sub_82466E20(ctx, base);
	// 825F0DB8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F0DBC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F0DC0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F0DC4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F0DC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F0DC8 size=108
    let mut pc: u32 = 0x825F0DC8;
    'dispatch: loop {
        match pc {
            0x825F0DC8 => {
    //   block [0x825F0DC8..0x825F0E34)
	// 825F0DC8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F0DCC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F0DD0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F0DD4: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825F0DD8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F0DDC: 38EB9B68  addi r7, r11, -0x6498
	ctx.r[7].s64 = ctx.r[11].s64 + -25752;
	// 825F0DE0: 39000005  li r8, 5
	ctx.r[8].s64 = 5;
	// 825F0DE4: 388A39A0  addi r4, r10, 0x39a0
	ctx.r[4].s64 = ctx.r[10].s64 + 14752;
	// 825F0DE8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825F0DEC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F0DF0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 825F0DF4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825F0DF8: 386ABE14  addi r3, r10, -0x41ec
	ctx.r[3].s64 = ctx.r[10].s64 + -16876;
	// 825F0DFC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 825F0E00: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825F0E04: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F0E08: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825F0E0C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F0E10: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825F0E14: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F0E18: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F0E1C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825F0E20: 4BE76001  bl 0x82466e20
	ctx.lr = 0x825F0E24;
	sub_82466E20(ctx, base);
	// 825F0E24: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F0E28: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F0E2C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F0E30: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F0E38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F0E38 size=112
    let mut pc: u32 = 0x825F0E38;
    'dispatch: loop {
        match pc {
            0x825F0E38 => {
    //   block [0x825F0E38..0x825F0EA8)
	// 825F0E38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F0E3C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F0E40: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F0E44: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F0E48: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825F0E4C: 38AAAF74  addi r5, r10, -0x508c
	ctx.r[5].s64 = ctx.r[10].s64 + -20620;
	// 825F0E50: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F0E54: 390B9BE0  addi r8, r11, -0x6420
	ctx.r[8].s64 = ctx.r[11].s64 + -25632;
	// 825F0E58: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 825F0E5C: 388A39B4  addi r4, r10, 0x39b4
	ctx.r[4].s64 = ctx.r[10].s64 + 14772;
	// 825F0E60: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825F0E64: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F0E68: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825F0E6C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F0E70: 386ABE44  addi r3, r10, -0x41bc
	ctx.r[3].s64 = ctx.r[10].s64 + -16828;
	// 825F0E74: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 825F0E78: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825F0E7C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825F0E80: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825F0E84: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F0E88: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825F0E8C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F0E90: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F0E94: 4BE75F8D  bl 0x82466e20
	ctx.lr = 0x825F0E98;
	sub_82466E20(ctx, base);
	// 825F0E98: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F0E9C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F0EA0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F0EA4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F0EA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F0EA8 size=108
    let mut pc: u32 = 0x825F0EA8;
    'dispatch: loop {
        match pc {
            0x825F0EA8 => {
    //   block [0x825F0EA8..0x825F0F14)
	// 825F0EA8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F0EAC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F0EB0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F0EB4: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825F0EB8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F0EBC: 38EB9C28  addi r7, r11, -0x63d8
	ctx.r[7].s64 = ctx.r[11].s64 + -25560;
	// 825F0EC0: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 825F0EC4: 388A39C0  addi r4, r10, 0x39c0
	ctx.r[4].s64 = ctx.r[10].s64 + 14784;
	// 825F0EC8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825F0ECC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F0ED0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 825F0ED4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825F0ED8: 386ABE74  addi r3, r10, -0x418c
	ctx.r[3].s64 = ctx.r[10].s64 + -16780;
	// 825F0EDC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 825F0EE0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825F0EE4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F0EE8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825F0EEC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F0EF0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825F0EF4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F0EF8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F0EFC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825F0F00: 4BE75F21  bl 0x82466e20
	ctx.lr = 0x825F0F04;
	sub_82466E20(ctx, base);
	// 825F0F04: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F0F08: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F0F0C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F0F10: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F0F18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F0F18 size=108
    let mut pc: u32 = 0x825F0F18;
    'dispatch: loop {
        match pc {
            0x825F0F18 => {
    //   block [0x825F0F18..0x825F0F84)
	// 825F0F18: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F0F1C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F0F20: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F0F24: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825F0F28: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F0F2C: 38EB9C88  addi r7, r11, -0x6378
	ctx.r[7].s64 = ctx.r[11].s64 + -25464;
	// 825F0F30: 39000008  li r8, 8
	ctx.r[8].s64 = 8;
	// 825F0F34: 388A39D8  addi r4, r10, 0x39d8
	ctx.r[4].s64 = ctx.r[10].s64 + 14808;
	// 825F0F38: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825F0F3C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F0F40: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 825F0F44: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825F0F48: 386ABEA4  addi r3, r10, -0x415c
	ctx.r[3].s64 = ctx.r[10].s64 + -16732;
	// 825F0F4C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 825F0F50: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825F0F54: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F0F58: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825F0F5C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F0F60: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825F0F64: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F0F68: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F0F6C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825F0F70: 4BE75EB1  bl 0x82466e20
	ctx.lr = 0x825F0F74;
	sub_82466E20(ctx, base);
	// 825F0F74: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F0F78: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F0F7C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F0F80: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F0F88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F0F88 size=108
    let mut pc: u32 = 0x825F0F88;
    'dispatch: loop {
        match pc {
            0x825F0F88 => {
    //   block [0x825F0F88..0x825F0FF4)
	// 825F0F88: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F0F8C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F0F90: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F0F94: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825F0F98: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F0F9C: 38EB9D48  addi r7, r11, -0x62b8
	ctx.r[7].s64 = ctx.r[11].s64 + -25272;
	// 825F0FA0: 39000006  li r8, 6
	ctx.r[8].s64 = 6;
	// 825F0FA4: 388A39F4  addi r4, r10, 0x39f4
	ctx.r[4].s64 = ctx.r[10].s64 + 14836;
	// 825F0FA8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825F0FAC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F0FB0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 825F0FB4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825F0FB8: 386ABED4  addi r3, r10, -0x412c
	ctx.r[3].s64 = ctx.r[10].s64 + -16684;
	// 825F0FBC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 825F0FC0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825F0FC4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F0FC8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825F0FCC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F0FD0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825F0FD4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F0FD8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F0FDC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825F0FE0: 4BE75E41  bl 0x82466e20
	ctx.lr = 0x825F0FE4;
	sub_82466E20(ctx, base);
	// 825F0FE4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F0FE8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F0FEC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F0FF0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F0FF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F0FF8 size=108
    let mut pc: u32 = 0x825F0FF8;
    'dispatch: loop {
        match pc {
            0x825F0FF8 => {
    //   block [0x825F0FF8..0x825F1064)
	// 825F0FF8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F0FFC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F1000: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F1004: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825F1008: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F100C: 38EB9DD8  addi r7, r11, -0x6228
	ctx.r[7].s64 = ctx.r[11].s64 + -25128;
	// 825F1010: 3900000D  li r8, 0xd
	ctx.r[8].s64 = 13;
	// 825F1014: 388A3A14  addi r4, r10, 0x3a14
	ctx.r[4].s64 = ctx.r[10].s64 + 14868;
	// 825F1018: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825F101C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F1020: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 825F1024: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825F1028: 386ABF04  addi r3, r10, -0x40fc
	ctx.r[3].s64 = ctx.r[10].s64 + -16636;
	// 825F102C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 825F1030: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825F1034: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F1038: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825F103C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F1040: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825F1044: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F1048: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F104C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825F1050: 4BE75DD1  bl 0x82466e20
	ctx.lr = 0x825F1054;
	sub_82466E20(ctx, base);
	// 825F1054: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F1058: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F105C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F1060: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F1068(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F1068 size=108
    let mut pc: u32 = 0x825F1068;
    'dispatch: loop {
        match pc {
            0x825F1068 => {
    //   block [0x825F1068..0x825F10D4)
	// 825F1068: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F106C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F1070: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F1074: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825F1078: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F107C: 38EB9F10  addi r7, r11, -0x60f0
	ctx.r[7].s64 = ctx.r[11].s64 + -24816;
	// 825F1080: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 825F1084: 388A3A30  addi r4, r10, 0x3a30
	ctx.r[4].s64 = ctx.r[10].s64 + 14896;
	// 825F1088: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825F108C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F1090: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 825F1094: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825F1098: 386ABF34  addi r3, r10, -0x40cc
	ctx.r[3].s64 = ctx.r[10].s64 + -16588;
	// 825F109C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 825F10A0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825F10A4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F10A8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825F10AC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F10B0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825F10B4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F10B8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F10BC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825F10C0: 4BE75D61  bl 0x82466e20
	ctx.lr = 0x825F10C4;
	sub_82466E20(ctx, base);
	// 825F10C4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F10C8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F10CC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F10D0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F10D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F10D8 size=116
    let mut pc: u32 = 0x825F10D8;
    'dispatch: loop {
        match pc {
            0x825F10D8 => {
    //   block [0x825F10D8..0x825F114C)
	// 825F10D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F10DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F10E0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F10E4: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825F10E8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F10EC: 390B9F70  addi r8, r11, -0x6090
	ctx.r[8].s64 = ctx.r[11].s64 + -24720;
	// 825F10F0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825F10F4: 392A176C  addi r9, r10, 0x176c
	ctx.r[9].s64 = ctx.r[10].s64 + 5996;
	// 825F10F8: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F10FC: 38E00005  li r7, 5
	ctx.r[7].s64 = 5;
	// 825F1100: 38AABF34  addi r5, r10, -0x40cc
	ctx.r[5].s64 = ctx.r[10].s64 + -16588;
	// 825F1104: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825F1108: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825F110C: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F1110: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825F1114: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F1118: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F111C: 3D608293  lis r11, -0x7d6d
	ctx.r[11].s64 = -2104295424;
	// 825F1120: 388A3A3C  addi r4, r10, 0x3a3c
	ctx.r[4].s64 = ctx.r[10].s64 + 14908;
	// 825F1124: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 825F1128: 386BBF64  addi r3, r11, -0x409c
	ctx.r[3].s64 = ctx.r[11].s64 + -16540;
	// 825F112C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 825F1130: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F1134: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F1138: 4BE75CE9  bl 0x82466e20
	ctx.lr = 0x825F113C;
	sub_82466E20(ctx, base);
	// 825F113C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F1140: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F1144: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F1148: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F1150(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F1150 size=96
    let mut pc: u32 = 0x825F1150;
    'dispatch: loop {
        match pc {
            0x825F1150 => {
    //   block [0x825F1150..0x825F11B0)
	// 825F1150: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F1154: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F1158: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F115C: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F1160: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825F1164: 388A3A4C  addi r4, r10, 0x3a4c
	ctx.r[4].s64 = ctx.r[10].s64 + 14924;
	// 825F1168: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F116C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825F1170: 386ABF94  addi r3, r10, -0x406c
	ctx.r[3].s64 = ctx.r[10].s64 + -16492;
	// 825F1174: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825F1178: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825F117C: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 825F1180: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825F1184: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F1188: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F118C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F1190: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 825F1194: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825F1198: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 825F119C: 4BE75C85  bl 0x82466e20
	ctx.lr = 0x825F11A0;
	sub_82466E20(ctx, base);
	// 825F11A0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F11A4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F11A8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F11AC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F11B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F11B0 size=112
    let mut pc: u32 = 0x825F11B0;
    'dispatch: loop {
        match pc {
            0x825F11B0 => {
    //   block [0x825F11B0..0x825F1220)
	// 825F11B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F11B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F11B8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F11BC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F11C0: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825F11C4: 38AADFA4  addi r5, r10, -0x205c
	ctx.r[5].s64 = ctx.r[10].s64 + -8284;
	// 825F11C8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F11CC: 390B9FE8  addi r8, r11, -0x6018
	ctx.r[8].s64 = ctx.r[11].s64 + -24600;
	// 825F11D0: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 825F11D4: 388A3A64  addi r4, r10, 0x3a64
	ctx.r[4].s64 = ctx.r[10].s64 + 14948;
	// 825F11D8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825F11DC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F11E0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825F11E4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F11E8: 386ABFC4  addi r3, r10, -0x403c
	ctx.r[3].s64 = ctx.r[10].s64 + -16444;
	// 825F11EC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 825F11F0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825F11F4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825F11F8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825F11FC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F1200: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825F1204: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F1208: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F120C: 4BE75C15  bl 0x82466e20
	ctx.lr = 0x825F1210;
	sub_82466E20(ctx, base);
	// 825F1210: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F1214: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F1218: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F121C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F1220(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F1220 size=96
    let mut pc: u32 = 0x825F1220;
    'dispatch: loop {
        match pc {
            0x825F1220 => {
    //   block [0x825F1220..0x825F1280)
	// 825F1220: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F1224: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F1228: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F122C: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F1230: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825F1234: 388A3A7C  addi r4, r10, 0x3a7c
	ctx.r[4].s64 = ctx.r[10].s64 + 14972;
	// 825F1238: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F123C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825F1240: 386ABFF4  addi r3, r10, -0x400c
	ctx.r[3].s64 = ctx.r[10].s64 + -16396;
	// 825F1244: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825F1248: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825F124C: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 825F1250: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825F1254: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F1258: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F125C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F1260: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 825F1264: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825F1268: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 825F126C: 4BE75BB5  bl 0x82466e20
	ctx.lr = 0x825F1270;
	sub_82466E20(ctx, base);
	// 825F1270: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F1274: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F1278: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F127C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F1280(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825F1280 size=24
    let mut pc: u32 = 0x825F1280;
    'dispatch: loop {
        match pc {
            0x825F1280 => {
    //   block [0x825F1280..0x825F1298)
	// 825F1280: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825F1284: 3D408274  lis r10, -0x7d8c
	ctx.r[10].s64 = -2106327040;
	// 825F1288: 394AE720  addi r10, r10, -0x18e0
	ctx.r[10].s64 = ctx.r[10].s64 + -6368;
	// 825F128C: 816BA048  lwz r11, -0x5fb8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-24504 as u32) ) } as u64;
	// 825F1290: 916A0008  stw r11, 8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 825F1294: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F1298(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F1298 size=116
    let mut pc: u32 = 0x825F1298;
    'dispatch: loop {
        match pc {
            0x825F1298 => {
    //   block [0x825F1298..0x825F130C)
	// 825F1298: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F129C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F12A0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F12A4: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825F12A8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F12AC: 390BE720  addi r8, r11, -0x18e0
	ctx.r[8].s64 = ctx.r[11].s64 + -6368;
	// 825F12B0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825F12B4: 392A17A8  addi r9, r10, 0x17a8
	ctx.r[9].s64 = ctx.r[10].s64 + 6056;
	// 825F12B8: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F12BC: 38E00002  li r7, 2
	ctx.r[7].s64 = 2;
	// 825F12C0: 38AAAF74  addi r5, r10, -0x508c
	ctx.r[5].s64 = ctx.r[10].s64 + -20620;
	// 825F12C4: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825F12C8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825F12CC: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F12D0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825F12D4: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 825F12D8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F12DC: 3D608293  lis r11, -0x7d6d
	ctx.r[11].s64 = -2104295424;
	// 825F12E0: 388A3A9C  addi r4, r10, 0x3a9c
	ctx.r[4].s64 = ctx.r[10].s64 + 15004;
	// 825F12E4: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 825F12E8: 386BC024  addi r3, r11, -0x3fdc
	ctx.r[3].s64 = ctx.r[11].s64 + -16348;
	// 825F12EC: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 825F12F0: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F12F4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F12F8: 4BE75B29  bl 0x82466e20
	ctx.lr = 0x825F12FC;
	sub_82466E20(ctx, base);
	// 825F12FC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F1300: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F1304: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F1308: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F1310(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F1310 size=104
    let mut pc: u32 = 0x825F1310;
    'dispatch: loop {
        match pc {
            0x825F1310 => {
    //   block [0x825F1310..0x825F1378)
	// 825F1310: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F1314: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F1318: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F131C: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F1320: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825F1324: 392A17D4  addi r9, r10, 0x17d4
	ctx.r[9].s64 = ctx.r[10].s64 + 6100;
	// 825F1328: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F132C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F1330: 38AAAF74  addi r5, r10, -0x508c
	ctx.r[5].s64 = ctx.r[10].s64 + -20620;
	// 825F1334: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F1338: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825F133C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F1340: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825F1344: 388A3AB0  addi r4, r10, 0x3ab0
	ctx.r[4].s64 = ctx.r[10].s64 + 15024;
	// 825F1348: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F134C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F1350: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 825F1354: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F1358: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 825F135C: 386AC054  addi r3, r10, -0x3fac
	ctx.r[3].s64 = ctx.r[10].s64 + -16300;
	// 825F1360: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 825F1364: 4BE75ABD  bl 0x82466e20
	ctx.lr = 0x825F1368;
	sub_82466E20(ctx, base);
	// 825F1368: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F136C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F1370: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F1374: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F1378(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F1378 size=96
    let mut pc: u32 = 0x825F1378;
    'dispatch: loop {
        match pc {
            0x825F1378 => {
    //   block [0x825F1378..0x825F13D8)
	// 825F1378: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F137C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F1380: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F1384: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F1388: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825F138C: 388A3AC4  addi r4, r10, 0x3ac4
	ctx.r[4].s64 = ctx.r[10].s64 + 15044;
	// 825F1390: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F1394: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825F1398: 386AC084  addi r3, r10, -0x3f7c
	ctx.r[3].s64 = ctx.r[10].s64 + -16252;
	// 825F139C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825F13A0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825F13A4: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 825F13A8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825F13AC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F13B0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F13B4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F13B8: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 825F13BC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825F13C0: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 825F13C4: 4BE75A5D  bl 0x82466e20
	ctx.lr = 0x825F13C8;
	sub_82466E20(ctx, base);
	// 825F13C8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F13CC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F13D0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F13D4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F13D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F13D8 size=100
    let mut pc: u32 = 0x825F13D8;
    'dispatch: loop {
        match pc {
            0x825F13D8 => {
    //   block [0x825F13D8..0x825F143C)
	// 825F13D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F13DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F13E0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F13E4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F13E8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825F13EC: 38AAC054  addi r5, r10, -0x3fac
	ctx.r[5].s64 = ctx.r[10].s64 + -16300;
	// 825F13F0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F13F4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825F13F8: 388A3ADC  addi r4, r10, 0x3adc
	ctx.r[4].s64 = ctx.r[10].s64 + 15068;
	// 825F13FC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F1400: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825F1404: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F1408: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825F140C: 386AC0B4  addi r3, r10, -0x3f4c
	ctx.r[3].s64 = ctx.r[10].s64 + -16204;
	// 825F1410: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F1414: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825F1418: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 825F141C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F1420: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 825F1424: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F1428: 4BE759F9  bl 0x82466e20
	ctx.lr = 0x825F142C;
	sub_82466E20(ctx, base);
	// 825F142C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F1430: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F1434: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F1438: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F1440(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F1440 size=112
    let mut pc: u32 = 0x825F1440;
    'dispatch: loop {
        match pc {
            0x825F1440 => {
    //   block [0x825F1440..0x825F14B0)
	// 825F1440: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F1444: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F1448: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F144C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F1450: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825F1454: 38AAC024  addi r5, r10, -0x3fdc
	ctx.r[5].s64 = ctx.r[10].s64 + -16348;
	// 825F1458: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F145C: 390BA050  addi r8, r11, -0x5fb0
	ctx.r[8].s64 = ctx.r[11].s64 + -24496;
	// 825F1460: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 825F1464: 388A3AF8  addi r4, r10, 0x3af8
	ctx.r[4].s64 = ctx.r[10].s64 + 15096;
	// 825F1468: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825F146C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F1470: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825F1474: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F1478: 386AC0E4  addi r3, r10, -0x3f1c
	ctx.r[3].s64 = ctx.r[10].s64 + -16156;
	// 825F147C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 825F1480: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825F1484: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825F1488: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825F148C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F1490: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825F1494: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F1498: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F149C: 4BE75985  bl 0x82466e20
	ctx.lr = 0x825F14A0;
	sub_82466E20(ctx, base);
	// 825F14A0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F14A4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F14A8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F14AC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F14B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F14B0 size=112
    let mut pc: u32 = 0x825F14B0;
    'dispatch: loop {
        match pc {
            0x825F14B0 => {
    //   block [0x825F14B0..0x825F1520)
	// 825F14B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F14B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F14B8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F14BC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F14C0: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825F14C4: 38AAC024  addi r5, r10, -0x3fdc
	ctx.r[5].s64 = ctx.r[10].s64 + -16348;
	// 825F14C8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F14CC: 390BA098  addi r8, r11, -0x5f68
	ctx.r[8].s64 = ctx.r[11].s64 + -24424;
	// 825F14D0: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 825F14D4: 388A3B08  addi r4, r10, 0x3b08
	ctx.r[4].s64 = ctx.r[10].s64 + 15112;
	// 825F14D8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825F14DC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F14E0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825F14E4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F14E8: 386AC114  addi r3, r10, -0x3eec
	ctx.r[3].s64 = ctx.r[10].s64 + -16108;
	// 825F14EC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 825F14F0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825F14F4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825F14F8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825F14FC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F1500: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825F1504: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F1508: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F150C: 4BE75915  bl 0x82466e20
	ctx.lr = 0x825F1510;
	sub_82466E20(ctx, base);
	// 825F1510: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F1514: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F1518: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F151C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F1520(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F1520 size=100
    let mut pc: u32 = 0x825F1520;
    'dispatch: loop {
        match pc {
            0x825F1520 => {
    //   block [0x825F1520..0x825F1584)
	// 825F1520: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F1524: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F1528: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F152C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F1530: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825F1534: 38AAC024  addi r5, r10, -0x3fdc
	ctx.r[5].s64 = ctx.r[10].s64 + -16348;
	// 825F1538: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F153C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825F1540: 388A3B20  addi r4, r10, 0x3b20
	ctx.r[4].s64 = ctx.r[10].s64 + 15136;
	// 825F1544: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F1548: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825F154C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F1550: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825F1554: 386AC144  addi r3, r10, -0x3ebc
	ctx.r[3].s64 = ctx.r[10].s64 + -16060;
	// 825F1558: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F155C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825F1560: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 825F1564: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F1568: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 825F156C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F1570: 4BE758B1  bl 0x82466e20
	ctx.lr = 0x825F1574;
	sub_82466E20(ctx, base);
	// 825F1574: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F1578: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F157C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F1580: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F1588(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F1588 size=96
    let mut pc: u32 = 0x825F1588;
    'dispatch: loop {
        match pc {
            0x825F1588 => {
    //   block [0x825F1588..0x825F15E8)
	// 825F1588: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F158C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F1590: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F1594: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F1598: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825F159C: 388A3B38  addi r4, r10, 0x3b38
	ctx.r[4].s64 = ctx.r[10].s64 + 15160;
	// 825F15A0: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F15A4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825F15A8: 386AC174  addi r3, r10, -0x3e8c
	ctx.r[3].s64 = ctx.r[10].s64 + -16012;
	// 825F15AC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825F15B0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825F15B4: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 825F15B8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825F15BC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F15C0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F15C4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F15C8: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 825F15CC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825F15D0: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 825F15D4: 4BE7584D  bl 0x82466e20
	ctx.lr = 0x825F15D8;
	sub_82466E20(ctx, base);
	// 825F15D8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F15DC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F15E0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F15E4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F15E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F15E8 size=112
    let mut pc: u32 = 0x825F15E8;
    'dispatch: loop {
        match pc {
            0x825F15E8 => {
    //   block [0x825F15E8..0x825F1658)
	// 825F15E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F15EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F15F0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F15F4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F15F8: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825F15FC: 38AAAF74  addi r5, r10, -0x508c
	ctx.r[5].s64 = ctx.r[10].s64 + -20620;
	// 825F1600: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F1604: 390BA0B0  addi r8, r11, -0x5f50
	ctx.r[8].s64 = ctx.r[11].s64 + -24400;
	// 825F1608: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 825F160C: 388A3B54  addi r4, r10, 0x3b54
	ctx.r[4].s64 = ctx.r[10].s64 + 15188;
	// 825F1610: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825F1614: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F1618: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825F161C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F1620: 386AC1A4  addi r3, r10, -0x3e5c
	ctx.r[3].s64 = ctx.r[10].s64 + -15964;
	// 825F1624: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 825F1628: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825F162C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825F1630: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825F1634: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F1638: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825F163C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F1640: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F1644: 4BE757DD  bl 0x82466e20
	ctx.lr = 0x825F1648;
	sub_82466E20(ctx, base);
	// 825F1648: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F164C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F1650: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F1654: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F1658(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F1658 size=96
    let mut pc: u32 = 0x825F1658;
    'dispatch: loop {
        match pc {
            0x825F1658 => {
    //   block [0x825F1658..0x825F16B8)
	// 825F1658: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F165C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F1660: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F1664: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F1668: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825F166C: 388A3B5C  addi r4, r10, 0x3b5c
	ctx.r[4].s64 = ctx.r[10].s64 + 15196;
	// 825F1670: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F1674: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825F1678: 386AC1D4  addi r3, r10, -0x3e2c
	ctx.r[3].s64 = ctx.r[10].s64 + -15916;
	// 825F167C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825F1680: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825F1684: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 825F1688: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825F168C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F1690: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F1694: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F1698: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 825F169C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825F16A0: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 825F16A4: 4BE7577D  bl 0x82466e20
	ctx.lr = 0x825F16A8;
	sub_82466E20(ctx, base);
	// 825F16A8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F16AC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F16B0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F16B4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F16B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F16B8 size=112
    let mut pc: u32 = 0x825F16B8;
    'dispatch: loop {
        match pc {
            0x825F16B8 => {
    //   block [0x825F16B8..0x825F1728)
	// 825F16B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F16BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F16C0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F16C4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F16C8: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825F16CC: 38AAC1D4  addi r5, r10, -0x3e2c
	ctx.r[5].s64 = ctx.r[10].s64 + -15916;
	// 825F16D0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F16D4: 390BA0E0  addi r8, r11, -0x5f20
	ctx.r[8].s64 = ctx.r[11].s64 + -24352;
	// 825F16D8: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 825F16DC: 388A3B70  addi r4, r10, 0x3b70
	ctx.r[4].s64 = ctx.r[10].s64 + 15216;
	// 825F16E0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825F16E4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F16E8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825F16EC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F16F0: 386AC204  addi r3, r10, -0x3dfc
	ctx.r[3].s64 = ctx.r[10].s64 + -15868;
	// 825F16F4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 825F16F8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825F16FC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825F1700: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825F1704: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F1708: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825F170C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F1710: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F1714: 4BE7570D  bl 0x82466e20
	ctx.lr = 0x825F1718;
	sub_82466E20(ctx, base);
	// 825F1718: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F171C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F1720: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F1724: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F1728(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F1728 size=108
    let mut pc: u32 = 0x825F1728;
    'dispatch: loop {
        match pc {
            0x825F1728 => {
    //   block [0x825F1728..0x825F1794)
	// 825F1728: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F172C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F1730: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F1734: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825F1738: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F173C: 38EBA0F8  addi r7, r11, -0x5f08
	ctx.r[7].s64 = ctx.r[11].s64 + -24328;
	// 825F1740: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 825F1744: 388A3B88  addi r4, r10, 0x3b88
	ctx.r[4].s64 = ctx.r[10].s64 + 15240;
	// 825F1748: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825F174C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F1750: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 825F1754: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825F1758: 386AC234  addi r3, r10, -0x3dcc
	ctx.r[3].s64 = ctx.r[10].s64 + -15820;
	// 825F175C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 825F1760: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825F1764: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F1768: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825F176C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F1770: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825F1774: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F1778: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F177C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825F1780: 4BE756A1  bl 0x82466e20
	ctx.lr = 0x825F1784;
	sub_82466E20(ctx, base);
	// 825F1784: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F1788: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F178C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F1790: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F1798(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F1798 size=112
    let mut pc: u32 = 0x825F1798;
    'dispatch: loop {
        match pc {
            0x825F1798 => {
    //   block [0x825F1798..0x825F1808)
	// 825F1798: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F179C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F17A0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F17A4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F17A8: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825F17AC: 38AAC354  addi r5, r10, -0x3cac
	ctx.r[5].s64 = ctx.r[10].s64 + -15532;
	// 825F17B0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F17B4: 390BA158  addi r8, r11, -0x5ea8
	ctx.r[8].s64 = ctx.r[11].s64 + -24232;
	// 825F17B8: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 825F17BC: 388A3B9C  addi r4, r10, 0x3b9c
	ctx.r[4].s64 = ctx.r[10].s64 + 15260;
	// 825F17C0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825F17C4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F17C8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825F17CC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F17D0: 386AC264  addi r3, r10, -0x3d9c
	ctx.r[3].s64 = ctx.r[10].s64 + -15772;
	// 825F17D4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 825F17D8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825F17DC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825F17E0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825F17E4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F17E8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825F17EC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F17F0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F17F4: 4BE7562D  bl 0x82466e20
	ctx.lr = 0x825F17F8;
	sub_82466E20(ctx, base);
	// 825F17F8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F17FC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F1800: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F1804: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F1808(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F1808 size=112
    let mut pc: u32 = 0x825F1808;
    'dispatch: loop {
        match pc {
            0x825F1808 => {
    //   block [0x825F1808..0x825F1878)
	// 825F1808: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F180C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F1810: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F1814: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F1818: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825F181C: 38AAC1A4  addi r5, r10, -0x3e5c
	ctx.r[5].s64 = ctx.r[10].s64 + -15964;
	// 825F1820: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F1824: 390BA170  addi r8, r11, -0x5e90
	ctx.r[8].s64 = ctx.r[11].s64 + -24208;
	// 825F1828: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 825F182C: 388A3BA8  addi r4, r10, 0x3ba8
	ctx.r[4].s64 = ctx.r[10].s64 + 15272;
	// 825F1830: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825F1834: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F1838: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825F183C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F1840: 386AC294  addi r3, r10, -0x3d6c
	ctx.r[3].s64 = ctx.r[10].s64 + -15724;
	// 825F1844: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 825F1848: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825F184C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825F1850: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825F1854: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F1858: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825F185C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F1860: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F1864: 4BE755BD  bl 0x82466e20
	ctx.lr = 0x825F1868;
	sub_82466E20(ctx, base);
	// 825F1868: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F186C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F1870: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F1874: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F1878(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F1878 size=100
    let mut pc: u32 = 0x825F1878;
    'dispatch: loop {
        match pc {
            0x825F1878 => {
    //   block [0x825F1878..0x825F18DC)
	// 825F1878: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F187C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F1880: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F1884: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F1888: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825F188C: 38AAC1A4  addi r5, r10, -0x3e5c
	ctx.r[5].s64 = ctx.r[10].s64 + -15964;
	// 825F1890: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F1894: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825F1898: 388A3BB4  addi r4, r10, 0x3bb4
	ctx.r[4].s64 = ctx.r[10].s64 + 15284;
	// 825F189C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F18A0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825F18A4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F18A8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825F18AC: 386AC2C4  addi r3, r10, -0x3d3c
	ctx.r[3].s64 = ctx.r[10].s64 + -15676;
	// 825F18B0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F18B4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825F18B8: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 825F18BC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F18C0: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 825F18C4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F18C8: 4BE75559  bl 0x82466e20
	ctx.lr = 0x825F18CC;
	sub_82466E20(ctx, base);
	// 825F18CC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F18D0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F18D4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F18D8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F18E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F18E0 size=116
    let mut pc: u32 = 0x825F18E0;
    'dispatch: loop {
        match pc {
            0x825F18E0 => {
    //   block [0x825F18E0..0x825F1954)
	// 825F18E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F18E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F18E8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F18EC: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825F18F0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F18F4: 390BA1A4  addi r8, r11, -0x5e5c
	ctx.r[8].s64 = ctx.r[11].s64 + -24156;
	// 825F18F8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825F18FC: 392A1800  addi r9, r10, 0x1800
	ctx.r[9].s64 = ctx.r[10].s64 + 6144;
	// 825F1900: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F1904: 38E00002  li r7, 2
	ctx.r[7].s64 = 2;
	// 825F1908: 38AAC354  addi r5, r10, -0x3cac
	ctx.r[5].s64 = ctx.r[10].s64 + -15532;
	// 825F190C: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825F1910: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825F1914: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F1918: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825F191C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F1920: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F1924: 3D608293  lis r11, -0x7d6d
	ctx.r[11].s64 = -2104295424;
	// 825F1928: 388A3BC4  addi r4, r10, 0x3bc4
	ctx.r[4].s64 = ctx.r[10].s64 + 15300;
	// 825F192C: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 825F1930: 386BC2F4  addi r3, r11, -0x3d0c
	ctx.r[3].s64 = ctx.r[11].s64 + -15628;
	// 825F1934: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 825F1938: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F193C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F1940: 4BE754E1  bl 0x82466e20
	ctx.lr = 0x825F1944;
	sub_82466E20(ctx, base);
	// 825F1944: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F1948: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F194C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F1950: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F1958(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F1958 size=112
    let mut pc: u32 = 0x825F1958;
    'dispatch: loop {
        match pc {
            0x825F1958 => {
    //   block [0x825F1958..0x825F19C8)
	// 825F1958: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F195C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F1960: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F1964: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F1968: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825F196C: 38AAC1A4  addi r5, r10, -0x3e5c
	ctx.r[5].s64 = ctx.r[10].s64 + -15964;
	// 825F1970: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F1974: 390BA1D4  addi r8, r11, -0x5e2c
	ctx.r[8].s64 = ctx.r[11].s64 + -24108;
	// 825F1978: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 825F197C: 388A3BD4  addi r4, r10, 0x3bd4
	ctx.r[4].s64 = ctx.r[10].s64 + 15316;
	// 825F1980: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825F1984: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F1988: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825F198C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F1990: 386AC324  addi r3, r10, -0x3cdc
	ctx.r[3].s64 = ctx.r[10].s64 + -15580;
	// 825F1994: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 825F1998: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825F199C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825F19A0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825F19A4: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 825F19A8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825F19AC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F19B0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F19B4: 4BE7546D  bl 0x82466e20
	ctx.lr = 0x825F19B8;
	sub_82466E20(ctx, base);
	// 825F19B8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F19BC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F19C0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F19C4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F19C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F19C8 size=116
    let mut pc: u32 = 0x825F19C8;
    'dispatch: loop {
        match pc {
            0x825F19C8 => {
    //   block [0x825F19C8..0x825F1A3C)
	// 825F19C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F19CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F19D0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F19D4: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825F19D8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F19DC: 390BA1F0  addi r8, r11, -0x5e10
	ctx.r[8].s64 = ctx.r[11].s64 + -24080;
	// 825F19E0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825F19E4: 392A182C  addi r9, r10, 0x182c
	ctx.r[9].s64 = ctx.r[10].s64 + 6188;
	// 825F19E8: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F19EC: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
	// 825F19F0: 38AAC984  addi r5, r10, -0x367c
	ctx.r[5].s64 = ctx.r[10].s64 + -13948;
	// 825F19F4: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825F19F8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825F19FC: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F1A00: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825F1A04: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F1A08: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F1A0C: 3D608293  lis r11, -0x7d6d
	ctx.r[11].s64 = -2104295424;
	// 825F1A10: 388A3BE8  addi r4, r10, 0x3be8
	ctx.r[4].s64 = ctx.r[10].s64 + 15336;
	// 825F1A14: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 825F1A18: 386BC354  addi r3, r11, -0x3cac
	ctx.r[3].s64 = ctx.r[11].s64 + -15532;
	// 825F1A1C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 825F1A20: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F1A24: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F1A28: 4BE753F9  bl 0x82466e20
	ctx.lr = 0x825F1A2C;
	sub_82466E20(ctx, base);
	// 825F1A2C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F1A30: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F1A34: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F1A38: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F1A40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F1A40 size=112
    let mut pc: u32 = 0x825F1A40;
    'dispatch: loop {
        match pc {
            0x825F1A40 => {
    //   block [0x825F1A40..0x825F1AB0)
	// 825F1A40: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F1A44: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F1A48: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F1A4C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F1A50: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825F1A54: 38AAC354  addi r5, r10, -0x3cac
	ctx.r[5].s64 = ctx.r[10].s64 + -15532;
	// 825F1A58: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F1A5C: 390BA208  addi r8, r11, -0x5df8
	ctx.r[8].s64 = ctx.r[11].s64 + -24056;
	// 825F1A60: 39200005  li r9, 5
	ctx.r[9].s64 = 5;
	// 825F1A64: 388A3BF8  addi r4, r10, 0x3bf8
	ctx.r[4].s64 = ctx.r[10].s64 + 15352;
	// 825F1A68: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825F1A6C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F1A70: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825F1A74: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F1A78: 386AC384  addi r3, r10, -0x3c7c
	ctx.r[3].s64 = ctx.r[10].s64 + -15484;
	// 825F1A7C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 825F1A80: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825F1A84: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825F1A88: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825F1A8C: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 825F1A90: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825F1A94: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F1A98: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F1A9C: 4BE75385  bl 0x82466e20
	ctx.lr = 0x825F1AA0;
	sub_82466E20(ctx, base);
	// 825F1AA0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F1AA4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F1AA8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F1AAC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F1AB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F1AB0 size=112
    let mut pc: u32 = 0x825F1AB0;
    'dispatch: loop {
        match pc {
            0x825F1AB0 => {
    //   block [0x825F1AB0..0x825F1B20)
	// 825F1AB0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F1AB4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F1AB8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F1ABC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F1AC0: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825F1AC4: 38AAC324  addi r5, r10, -0x3cdc
	ctx.r[5].s64 = ctx.r[10].s64 + -15580;
	// 825F1AC8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F1ACC: 390BA280  addi r8, r11, -0x5d80
	ctx.r[8].s64 = ctx.r[11].s64 + -23936;
	// 825F1AD0: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 825F1AD4: 388A3C0C  addi r4, r10, 0x3c0c
	ctx.r[4].s64 = ctx.r[10].s64 + 15372;
	// 825F1AD8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825F1ADC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F1AE0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825F1AE4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F1AE8: 386AC3B4  addi r3, r10, -0x3c4c
	ctx.r[3].s64 = ctx.r[10].s64 + -15436;
	// 825F1AEC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 825F1AF0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825F1AF4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825F1AF8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825F1AFC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F1B00: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825F1B04: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F1B08: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F1B0C: 4BE75315  bl 0x82466e20
	ctx.lr = 0x825F1B10;
	sub_82466E20(ctx, base);
	// 825F1B10: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F1B14: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F1B18: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F1B1C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F1B20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F1B20 size=112
    let mut pc: u32 = 0x825F1B20;
    'dispatch: loop {
        match pc {
            0x825F1B20 => {
    //   block [0x825F1B20..0x825F1B90)
	// 825F1B20: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F1B24: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F1B28: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F1B2C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F1B30: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825F1B34: 38AAC354  addi r5, r10, -0x3cac
	ctx.r[5].s64 = ctx.r[10].s64 + -15532;
	// 825F1B38: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F1B3C: 390BA2C8  addi r8, r11, -0x5d38
	ctx.r[8].s64 = ctx.r[11].s64 + -23864;
	// 825F1B40: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 825F1B44: 388A3C24  addi r4, r10, 0x3c24
	ctx.r[4].s64 = ctx.r[10].s64 + 15396;
	// 825F1B48: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825F1B4C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F1B50: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825F1B54: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F1B58: 386AC3E4  addi r3, r10, -0x3c1c
	ctx.r[3].s64 = ctx.r[10].s64 + -15388;
	// 825F1B5C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 825F1B60: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825F1B64: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825F1B68: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825F1B6C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F1B70: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825F1B74: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F1B78: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F1B7C: 4BE752A5  bl 0x82466e20
	ctx.lr = 0x825F1B80;
	sub_82466E20(ctx, base);
	// 825F1B80: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F1B84: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F1B88: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F1B8C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F1B90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F1B90 size=112
    let mut pc: u32 = 0x825F1B90;
    'dispatch: loop {
        match pc {
            0x825F1B90 => {
    //   block [0x825F1B90..0x825F1C00)
	// 825F1B90: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F1B94: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F1B98: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F1B9C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F1BA0: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825F1BA4: 38AAC354  addi r5, r10, -0x3cac
	ctx.r[5].s64 = ctx.r[10].s64 + -15532;
	// 825F1BA8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F1BAC: 390BA310  addi r8, r11, -0x5cf0
	ctx.r[8].s64 = ctx.r[11].s64 + -23792;
	// 825F1BB0: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 825F1BB4: 388A3C3C  addi r4, r10, 0x3c3c
	ctx.r[4].s64 = ctx.r[10].s64 + 15420;
	// 825F1BB8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825F1BBC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F1BC0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825F1BC4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F1BC8: 386AC414  addi r3, r10, -0x3bec
	ctx.r[3].s64 = ctx.r[10].s64 + -15340;
	// 825F1BCC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 825F1BD0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825F1BD4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825F1BD8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825F1BDC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F1BE0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825F1BE4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F1BE8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F1BEC: 4BE75235  bl 0x82466e20
	ctx.lr = 0x825F1BF0;
	sub_82466E20(ctx, base);
	// 825F1BF0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F1BF4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F1BF8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F1BFC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F1C00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F1C00 size=108
    let mut pc: u32 = 0x825F1C00;
    'dispatch: loop {
        match pc {
            0x825F1C00 => {
    //   block [0x825F1C00..0x825F1C6C)
	// 825F1C00: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F1C04: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F1C08: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F1C0C: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825F1C10: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F1C14: 38EBA358  addi r7, r11, -0x5ca8
	ctx.r[7].s64 = ctx.r[11].s64 + -23720;
	// 825F1C18: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 825F1C1C: 388A3C54  addi r4, r10, 0x3c54
	ctx.r[4].s64 = ctx.r[10].s64 + 15444;
	// 825F1C20: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825F1C24: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F1C28: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 825F1C2C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825F1C30: 386AC444  addi r3, r10, -0x3bbc
	ctx.r[3].s64 = ctx.r[10].s64 + -15292;
	// 825F1C34: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 825F1C38: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825F1C3C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F1C40: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825F1C44: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F1C48: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825F1C4C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F1C50: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F1C54: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825F1C58: 4BE751C9  bl 0x82466e20
	ctx.lr = 0x825F1C5C;
	sub_82466E20(ctx, base);
	// 825F1C5C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F1C60: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F1C64: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F1C68: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F1C70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F1C70 size=112
    let mut pc: u32 = 0x825F1C70;
    'dispatch: loop {
        match pc {
            0x825F1C70 => {
    //   block [0x825F1C70..0x825F1CE0)
	// 825F1C70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F1C74: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F1C78: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F1C7C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F1C80: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825F1C84: 38AAC354  addi r5, r10, -0x3cac
	ctx.r[5].s64 = ctx.r[10].s64 + -15532;
	// 825F1C88: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F1C8C: 390BA3A0  addi r8, r11, -0x5c60
	ctx.r[8].s64 = ctx.r[11].s64 + -23648;
	// 825F1C90: 39200005  li r9, 5
	ctx.r[9].s64 = 5;
	// 825F1C94: 388A3C78  addi r4, r10, 0x3c78
	ctx.r[4].s64 = ctx.r[10].s64 + 15480;
	// 825F1C98: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825F1C9C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F1CA0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825F1CA4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F1CA8: 386AC474  addi r3, r10, -0x3b8c
	ctx.r[3].s64 = ctx.r[10].s64 + -15244;
	// 825F1CAC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 825F1CB0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825F1CB4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825F1CB8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825F1CBC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F1CC0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825F1CC4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F1CC8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F1CCC: 4BE75155  bl 0x82466e20
	ctx.lr = 0x825F1CD0;
	sub_82466E20(ctx, base);
	// 825F1CD0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F1CD4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F1CD8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F1CDC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F1CE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F1CE0 size=116
    let mut pc: u32 = 0x825F1CE0;
    'dispatch: loop {
        match pc {
            0x825F1CE0 => {
    //   block [0x825F1CE0..0x825F1D54)
	// 825F1CE0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F1CE4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F1CE8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F1CEC: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 825F1CF0: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F1CF4: 392B1868  addi r9, r11, 0x1868
	ctx.r[9].s64 = ctx.r[11].s64 + 6248;
	// 825F1CF8: 38AAC354  addi r5, r10, -0x3cac
	ctx.r[5].s64 = ctx.r[10].s64 + -15532;
	// 825F1CFC: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F1D00: 38E90014  addi r7, r9, 0x14
	ctx.r[7].s64 = ctx.r[9].s64 + 20;
	// 825F1D04: 38C00006  li r6, 6
	ctx.r[6].s64 = 6;
	// 825F1D08: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825F1D0C: 388A3C90  addi r4, r10, 0x3c90
	ctx.r[4].s64 = ctx.r[10].s64 + 15504;
	// 825F1D10: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F1D14: 396BA420  addi r11, r11, -0x5be0
	ctx.r[11].s64 = ctx.r[11].s64 + -23520;
	// 825F1D18: 90E10064  stw r7, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[7].u32 ) };
	// 825F1D1C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F1D20: 90C1005C  stw r6, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[6].u32 ) };
	// 825F1D24: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F1D28: 386AC4A4  addi r3, r10, -0x3b5c
	ctx.r[3].s64 = ctx.r[10].s64 + -15196;
	// 825F1D2C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 825F1D30: 91010074  stw r8, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[8].u32 ) };
	// 825F1D34: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F1D38: 9101006C  stw r8, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[8].u32 ) };
	// 825F1D3C: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 825F1D40: 4BE750E1  bl 0x82466e20
	ctx.lr = 0x825F1D44;
	sub_82466E20(ctx, base);
	// 825F1D44: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F1D48: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F1D4C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F1D50: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F1D58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825F1D58 size=36
    let mut pc: u32 = 0x825F1D58;
    'dispatch: loop {
        match pc {
            0x825F1D58 => {
    //   block [0x825F1D58..0x825F1D7C)
	// 825F1D58: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825F1D5C: 814BA4B4  lwz r10, -0x5b4c(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-23372 as u32) ) } as u64;
	// 825F1D60: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825F1D64: 396BE750  addi r11, r11, -0x18b0
	ctx.r[11].s64 = ctx.r[11].s64 + -6320;
	// 825F1D68: 914B0008  stw r10, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 825F1D6C: 3D408274  lis r10, -0x7d8c
	ctx.r[10].s64 = -2106327040;
	// 825F1D70: 814AA41C  lwz r10, -0x5be4(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-23524 as u32) ) } as u64;
	// 825F1D74: 914B0020  stw r10, 0x20(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(32 as u32), ctx.r[10].u32 ) };
	// 825F1D78: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F1D80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F1D80 size=108
    let mut pc: u32 = 0x825F1D80;
    'dispatch: loop {
        match pc {
            0x825F1D80 => {
    //   block [0x825F1D80..0x825F1DEC)
	// 825F1D80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F1D84: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F1D88: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F1D8C: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825F1D90: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F1D94: 38EBE750  addi r7, r11, -0x18b0
	ctx.r[7].s64 = ctx.r[11].s64 + -6320;
	// 825F1D98: 39000007  li r8, 7
	ctx.r[8].s64 = 7;
	// 825F1D9C: 388A3D1C  addi r4, r10, 0x3d1c
	ctx.r[4].s64 = ctx.r[10].s64 + 15644;
	// 825F1DA0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825F1DA4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F1DA8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 825F1DAC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825F1DB0: 386AC4D4  addi r3, r10, -0x3b2c
	ctx.r[3].s64 = ctx.r[10].s64 + -15148;
	// 825F1DB4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 825F1DB8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825F1DBC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F1DC0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825F1DC4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F1DC8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825F1DCC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F1DD0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F1DD4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825F1DD8: 4BE75049  bl 0x82466e20
	ctx.lr = 0x825F1DDC;
	sub_82466E20(ctx, base);
	// 825F1DDC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F1DE0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F1DE4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F1DE8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F1DF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825F1DF0 size=24
    let mut pc: u32 = 0x825F1DF0;
    'dispatch: loop {
        match pc {
            0x825F1DF0 => {
    //   block [0x825F1DF0..0x825F1E08)
	// 825F1DF0: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825F1DF4: 3D408274  lis r10, -0x7d8c
	ctx.r[10].s64 = -2106327040;
	// 825F1DF8: 394AE7F8  addi r10, r10, -0x1808
	ctx.r[10].s64 = ctx.r[10].s64 + -6152;
	// 825F1DFC: 816BA41C  lwz r11, -0x5be4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-23524 as u32) ) } as u64;
	// 825F1E00: 916A0080  stw r11, 0x80(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(128 as u32), ctx.r[11].u32 ) };
	// 825F1E04: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F1E08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F1E08 size=116
    let mut pc: u32 = 0x825F1E08;
    'dispatch: loop {
        match pc {
            0x825F1E08 => {
    //   block [0x825F1E08..0x825F1E7C)
	// 825F1E08: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F1E0C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F1E10: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F1E14: 3D408274  lis r10, -0x7d8c
	ctx.r[10].s64 = -2106327040;
	// 825F1E18: 38E00008  li r7, 8
	ctx.r[7].s64 = 8;
	// 825F1E1C: 390AE7F8  addi r8, r10, -0x1808
	ctx.r[8].s64 = ctx.r[10].s64 + -6152;
	// 825F1E20: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F1E24: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 825F1E28: 38AAC4D4  addi r5, r10, -0x3b2c
	ctx.r[5].s64 = ctx.r[10].s64 + -15148;
	// 825F1E2C: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F1E30: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 825F1E34: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825F1E38: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825F1E3C: 388A3D58  addi r4, r10, 0x3d58
	ctx.r[4].s64 = ctx.r[10].s64 + 15704;
	// 825F1E40: 396B1924  addi r11, r11, 0x1924
	ctx.r[11].s64 = ctx.r[11].s64 + 6436;
	// 825F1E44: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F1E48: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F1E4C: 386AC504  addi r3, r10, -0x3afc
	ctx.r[3].s64 = ctx.r[10].s64 + -15100;
	// 825F1E50: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 825F1E54: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825F1E58: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 825F1E5C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F1E60: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F1E64: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F1E68: 4BE74FB9  bl 0x82466e20
	ctx.lr = 0x825F1E6C;
	sub_82466E20(ctx, base);
	// 825F1E6C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F1E70: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F1E74: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F1E78: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F1E80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F1E80 size=112
    let mut pc: u32 = 0x825F1E80;
    'dispatch: loop {
        match pc {
            0x825F1E80 => {
    //   block [0x825F1E80..0x825F1EF0)
	// 825F1E80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F1E84: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F1E88: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F1E8C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F1E90: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825F1E94: 38AAC4D4  addi r5, r10, -0x3b2c
	ctx.r[5].s64 = ctx.r[10].s64 + -15148;
	// 825F1E98: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F1E9C: 390BA4B8  addi r8, r11, -0x5b48
	ctx.r[8].s64 = ctx.r[11].s64 + -23368;
	// 825F1EA0: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 825F1EA4: 388A3D7C  addi r4, r10, 0x3d7c
	ctx.r[4].s64 = ctx.r[10].s64 + 15740;
	// 825F1EA8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825F1EAC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F1EB0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825F1EB4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F1EB8: 386AC534  addi r3, r10, -0x3acc
	ctx.r[3].s64 = ctx.r[10].s64 + -15052;
	// 825F1EBC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 825F1EC0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825F1EC4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825F1EC8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825F1ECC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F1ED0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825F1ED4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F1ED8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F1EDC: 4BE74F45  bl 0x82466e20
	ctx.lr = 0x825F1EE0;
	sub_82466E20(ctx, base);
	// 825F1EE0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F1EE4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F1EE8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F1EEC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F1EF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825F1EF0 size=24
    let mut pc: u32 = 0x825F1EF0;
    'dispatch: loop {
        match pc {
            0x825F1EF0 => {
    //   block [0x825F1EF0..0x825F1F08)
	// 825F1EF0: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825F1EF4: 3D408274  lis r10, -0x7d8c
	ctx.r[10].s64 = -2106327040;
	// 825F1EF8: 394AE8B8  addi r10, r10, -0x1748
	ctx.r[10].s64 = ctx.r[10].s64 + -5960;
	// 825F1EFC: 816BAB68  lwz r11, -0x5498(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-21656 as u32) ) } as u64;
	// 825F1F00: 916A0080  stw r11, 0x80(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(128 as u32), ctx.r[11].u32 ) };
	// 825F1F04: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F1F08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F1F08 size=116
    let mut pc: u32 = 0x825F1F08;
    'dispatch: loop {
        match pc {
            0x825F1F08 => {
    //   block [0x825F1F08..0x825F1F7C)
	// 825F1F08: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F1F0C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F1F10: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F1F14: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 825F1F18: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F1F1C: 392B18E8  addi r9, r11, 0x18e8
	ctx.r[9].s64 = ctx.r[11].s64 + 6376;
	// 825F1F20: 38AAC324  addi r5, r10, -0x3cdc
	ctx.r[5].s64 = ctx.r[10].s64 + -15580;
	// 825F1F24: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F1F28: 38E90060  addi r7, r9, 0x60
	ctx.r[7].s64 = ctx.r[9].s64 + 96;
	// 825F1F2C: 38C00007  li r6, 7
	ctx.r[6].s64 = 7;
	// 825F1F30: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825F1F34: 388A3DA0  addi r4, r10, 0x3da0
	ctx.r[4].s64 = ctx.r[10].s64 + 15776;
	// 825F1F38: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F1F3C: 396BE8B8  addi r11, r11, -0x1748
	ctx.r[11].s64 = ctx.r[11].s64 + -5960;
	// 825F1F40: 90E10064  stw r7, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[7].u32 ) };
	// 825F1F44: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F1F48: 90C1005C  stw r6, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[6].u32 ) };
	// 825F1F4C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F1F50: 386AC564  addi r3, r10, -0x3a9c
	ctx.r[3].s64 = ctx.r[10].s64 + -15004;
	// 825F1F54: 39400003  li r10, 3
	ctx.r[10].s64 = 3;
	// 825F1F58: 91010074  stw r8, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[8].u32 ) };
	// 825F1F5C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F1F60: 9101006C  stw r8, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[8].u32 ) };
	// 825F1F64: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 825F1F68: 4BE74EB9  bl 0x82466e20
	ctx.lr = 0x825F1F6C;
	sub_82466E20(ctx, base);
	// 825F1F6C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F1F70: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F1F74: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F1F78: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F1F80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F1F80 size=100
    let mut pc: u32 = 0x825F1F80;
    'dispatch: loop {
        match pc {
            0x825F1F80 => {
    //   block [0x825F1F80..0x825F1FE4)
	// 825F1F80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F1F84: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F1F88: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F1F8C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F1F90: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825F1F94: 38AAC6B4  addi r5, r10, -0x394c
	ctx.r[5].s64 = ctx.r[10].s64 + -14668;
	// 825F1F98: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F1F9C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825F1FA0: 388A3DB4  addi r4, r10, 0x3db4
	ctx.r[4].s64 = ctx.r[10].s64 + 15796;
	// 825F1FA4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F1FA8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825F1FAC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F1FB0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825F1FB4: 386AC594  addi r3, r10, -0x3a6c
	ctx.r[3].s64 = ctx.r[10].s64 + -14956;
	// 825F1FB8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F1FBC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825F1FC0: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 825F1FC4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F1FC8: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 825F1FCC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F1FD0: 4BE74E51  bl 0x82466e20
	ctx.lr = 0x825F1FD4;
	sub_82466E20(ctx, base);
	// 825F1FD4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F1FD8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F1FDC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F1FE0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F1FE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F1FE8 size=100
    let mut pc: u32 = 0x825F1FE8;
    'dispatch: loop {
        match pc {
            0x825F1FE8 => {
    //   block [0x825F1FE8..0x825F204C)
	// 825F1FE8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F1FEC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F1FF0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F1FF4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F1FF8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825F1FFC: 38AAC1A4  addi r5, r10, -0x3e5c
	ctx.r[5].s64 = ctx.r[10].s64 + -15964;
	// 825F2000: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F2004: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825F2008: 388A3DC4  addi r4, r10, 0x3dc4
	ctx.r[4].s64 = ctx.r[10].s64 + 15812;
	// 825F200C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F2010: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825F2014: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F2018: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825F201C: 386AC5C4  addi r3, r10, -0x3a3c
	ctx.r[3].s64 = ctx.r[10].s64 + -14908;
	// 825F2020: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F2024: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825F2028: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 825F202C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F2030: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 825F2034: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F2038: 4BE74DE9  bl 0x82466e20
	ctx.lr = 0x825F203C;
	sub_82466E20(ctx, base);
	// 825F203C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F2040: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F2044: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F2048: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F2050(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F2050 size=108
    let mut pc: u32 = 0x825F2050;
    'dispatch: loop {
        match pc {
            0x825F2050 => {
    //   block [0x825F2050..0x825F20BC)
	// 825F2050: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F2054: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F2058: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F205C: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825F2060: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F2064: 38EBA518  addi r7, r11, -0x5ae8
	ctx.r[7].s64 = ctx.r[11].s64 + -23272;
	// 825F2068: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 825F206C: 388A3DD8  addi r4, r10, 0x3dd8
	ctx.r[4].s64 = ctx.r[10].s64 + 15832;
	// 825F2070: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825F2074: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F2078: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 825F207C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825F2080: 386AC5F4  addi r3, r10, -0x3a0c
	ctx.r[3].s64 = ctx.r[10].s64 + -14860;
	// 825F2084: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 825F2088: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825F208C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F2090: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825F2094: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F2098: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825F209C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F20A0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F20A4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825F20A8: 4BE74D79  bl 0x82466e20
	ctx.lr = 0x825F20AC;
	sub_82466E20(ctx, base);
	// 825F20AC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F20B0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F20B4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F20B8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F20C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F20C0 size=112
    let mut pc: u32 = 0x825F20C0;
    'dispatch: loop {
        match pc {
            0x825F20C0 => {
    //   block [0x825F20C0..0x825F2130)
	// 825F20C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F20C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F20C8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F20CC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F20D0: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825F20D4: 38AAC324  addi r5, r10, -0x3cdc
	ctx.r[5].s64 = ctx.r[10].s64 + -15580;
	// 825F20D8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F20DC: 390BA578  addi r8, r11, -0x5a88
	ctx.r[8].s64 = ctx.r[11].s64 + -23176;
	// 825F20E0: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 825F20E4: 388A3DF0  addi r4, r10, 0x3df0
	ctx.r[4].s64 = ctx.r[10].s64 + 15856;
	// 825F20E8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825F20EC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F20F0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825F20F4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F20F8: 386AC624  addi r3, r10, -0x39dc
	ctx.r[3].s64 = ctx.r[10].s64 + -14812;
	// 825F20FC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 825F2100: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825F2104: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825F2108: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825F210C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F2110: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825F2114: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F2118: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F211C: 4BE74D05  bl 0x82466e20
	ctx.lr = 0x825F2120;
	sub_82466E20(ctx, base);
	// 825F2120: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F2124: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F2128: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F212C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F2130(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F2130 size=108
    let mut pc: u32 = 0x825F2130;
    'dispatch: loop {
        match pc {
            0x825F2130 => {
    //   block [0x825F2130..0x825F219C)
	// 825F2130: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F2134: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F2138: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F213C: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825F2140: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F2144: 38EBA5C0  addi r7, r11, -0x5a40
	ctx.r[7].s64 = ctx.r[11].s64 + -23104;
	// 825F2148: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 825F214C: 388A3DFC  addi r4, r10, 0x3dfc
	ctx.r[4].s64 = ctx.r[10].s64 + 15868;
	// 825F2150: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825F2154: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F2158: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 825F215C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825F2160: 386AC654  addi r3, r10, -0x39ac
	ctx.r[3].s64 = ctx.r[10].s64 + -14764;
	// 825F2164: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 825F2168: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825F216C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F2170: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825F2174: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F2178: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825F217C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F2180: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F2184: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825F2188: 4BE74C99  bl 0x82466e20
	ctx.lr = 0x825F218C;
	sub_82466E20(ctx, base);
	// 825F218C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F2190: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F2194: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F2198: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F21A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825F21A0 size=28
    let mut pc: u32 = 0x825F21A0;
    'dispatch: loop {
        match pc {
            0x825F21A0 => {
    //   block [0x825F21A0..0x825F21BC)
	// 825F21A0: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825F21A4: 3D408274  lis r10, -0x7d8c
	ctx.r[10].s64 = -2106327040;
	// 825F21A8: 394AE960  addi r10, r10, -0x16a0
	ctx.r[10].s64 = ctx.r[10].s64 + -5792;
	// 825F21AC: 816BA5D8  lwz r11, -0x5a28(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-23080 as u32) ) } as u64;
	// 825F21B0: 916A0068  stw r11, 0x68(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(104 as u32), ctx.r[11].u32 ) };
	// 825F21B4: 916A0080  stw r11, 0x80(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(128 as u32), ctx.r[11].u32 ) };
	// 825F21B8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F21C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F21C0 size=112
    let mut pc: u32 = 0x825F21C0;
    'dispatch: loop {
        match pc {
            0x825F21C0 => {
    //   block [0x825F21C0..0x825F2230)
	// 825F21C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F21C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F21C8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F21CC: 3D408274  lis r10, -0x7d8c
	ctx.r[10].s64 = -2106327040;
	// 825F21D0: 3900000E  li r8, 0xe
	ctx.r[8].s64 = 14;
	// 825F21D4: 38EAE960  addi r7, r10, -0x16a0
	ctx.r[7].s64 = ctx.r[10].s64 + -5792;
	// 825F21D8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F21DC: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 825F21E0: 388A3E1C  addi r4, r10, 0x3e1c
	ctx.r[4].s64 = ctx.r[10].s64 + 15900;
	// 825F21E4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825F21E8: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 825F21EC: 396B19D0  addi r11, r11, 0x19d0
	ctx.r[11].s64 = ctx.r[11].s64 + 6608;
	// 825F21F0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 825F21F4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F21F8: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F21FC: 386AC684  addi r3, r10, -0x397c
	ctx.r[3].s64 = ctx.r[10].s64 + -14716;
	// 825F2200: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825F2204: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 825F2208: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F220C: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 825F2210: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F2214: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F2218: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825F221C: 4BE74C05  bl 0x82466e20
	ctx.lr = 0x825F2220;
	sub_82466E20(ctx, base);
	// 825F2220: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F2224: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F2228: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F222C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F2230(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825F2230 size=24
    let mut pc: u32 = 0x825F2230;
    'dispatch: loop {
        match pc {
            0x825F2230 => {
    //   block [0x825F2230..0x825F2248)
	// 825F2230: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825F2234: 3D408274  lis r10, -0x7d8c
	ctx.r[10].s64 = -2106327040;
	// 825F2238: 394AEAB0  addi r10, r10, -0x1550
	ctx.r[10].s64 = ctx.r[10].s64 + -5456;
	// 825F223C: 816BAB68  lwz r11, -0x5498(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-21656 as u32) ) } as u64;
	// 825F2240: 916A0068  stw r11, 0x68(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(104 as u32), ctx.r[11].u32 ) };
	// 825F2244: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F2248(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F2248 size=116
    let mut pc: u32 = 0x825F2248;
    'dispatch: loop {
        match pc {
            0x825F2248 => {
    //   block [0x825F2248..0x825F22BC)
	// 825F2248: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F224C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F2250: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F2254: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 825F2258: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F225C: 392B19A8  addi r9, r11, 0x19a8
	ctx.r[9].s64 = ctx.r[11].s64 + 6568;
	// 825F2260: 38AAC324  addi r5, r10, -0x3cdc
	ctx.r[5].s64 = ctx.r[10].s64 + -15580;
	// 825F2264: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F2268: 38E90064  addi r7, r9, 0x64
	ctx.r[7].s64 = ctx.r[9].s64 + 100;
	// 825F226C: 38C00007  li r6, 7
	ctx.r[6].s64 = 7;
	// 825F2270: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825F2274: 388A3E30  addi r4, r10, 0x3e30
	ctx.r[4].s64 = ctx.r[10].s64 + 15920;
	// 825F2278: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F227C: 396BEAB0  addi r11, r11, -0x1550
	ctx.r[11].s64 = ctx.r[11].s64 + -5456;
	// 825F2280: 90E10064  stw r7, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[7].u32 ) };
	// 825F2284: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F2288: 90C1005C  stw r6, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[6].u32 ) };
	// 825F228C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F2290: 386AC6B4  addi r3, r10, -0x394c
	ctx.r[3].s64 = ctx.r[10].s64 + -14668;
	// 825F2294: 39400002  li r10, 2
	ctx.r[10].s64 = 2;
	// 825F2298: 91010074  stw r8, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[8].u32 ) };
	// 825F229C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F22A0: 9101006C  stw r8, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[8].u32 ) };
	// 825F22A4: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 825F22A8: 4BE74B79  bl 0x82466e20
	ctx.lr = 0x825F22AC;
	sub_82466E20(ctx, base);
	// 825F22AC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F22B0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F22B4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F22B8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F22C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F22C0 size=112
    let mut pc: u32 = 0x825F22C0;
    'dispatch: loop {
        match pc {
            0x825F22C0 => {
    //   block [0x825F22C0..0x825F2330)
	// 825F22C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F22C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F22C8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F22CC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F22D0: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825F22D4: 38AAC2C4  addi r5, r10, -0x3d3c
	ctx.r[5].s64 = ctx.r[10].s64 + -15676;
	// 825F22D8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F22DC: 390BA5E0  addi r8, r11, -0x5a20
	ctx.r[8].s64 = ctx.r[11].s64 + -23072;
	// 825F22E0: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 825F22E4: 388A3E3C  addi r4, r10, 0x3e3c
	ctx.r[4].s64 = ctx.r[10].s64 + 15932;
	// 825F22E8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825F22EC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F22F0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825F22F4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F22F8: 386AC6E4  addi r3, r10, -0x391c
	ctx.r[3].s64 = ctx.r[10].s64 + -14620;
	// 825F22FC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 825F2300: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825F2304: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825F2308: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825F230C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F2310: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825F2314: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F2318: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F231C: 4BE74B05  bl 0x82466e20
	ctx.lr = 0x825F2320;
	sub_82466E20(ctx, base);
	// 825F2320: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F2324: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F2328: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F232C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F2330(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825F2330 size=24
    let mut pc: u32 = 0x825F2330;
    'dispatch: loop {
        match pc {
            0x825F2330 => {
    //   block [0x825F2330..0x825F2348)
	// 825F2330: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825F2334: 3D408274  lis r10, -0x7d8c
	ctx.r[10].s64 = -2106327040;
	// 825F2338: 394AEB58  addi r10, r10, -0x14a8
	ctx.r[10].s64 = ctx.r[10].s64 + -5288;
	// 825F233C: 816BAB68  lwz r11, -0x5498(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-21656 as u32) ) } as u64;
	// 825F2340: 916A00C8  stw r11, 0xc8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(200 as u32), ctx.r[11].u32 ) };
	// 825F2344: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


