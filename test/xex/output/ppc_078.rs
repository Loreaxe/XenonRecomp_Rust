pub fn sub_825FEE20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825FEE20 size=100
    let mut pc: u32 = 0x825FEE20;
    'dispatch: loop {
        match pc {
            0x825FEE20 => {
    //   block [0x825FEE20..0x825FEE84)
	// 825FEE20: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825FEE24: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825FEE28: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825FEE2C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FEE30: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825FEE34: 38AA176C  addi r5, r10, 0x176c
	ctx.r[5].s64 = ctx.r[10].s64 + 5996;
	// 825FEE38: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 825FEE3C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825FEE40: 388AB750  addi r4, r10, -0x48b0
	ctx.r[4].s64 = ctx.r[10].s64 + -18608;
	// 825FEE44: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FEE48: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825FEE4C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825FEE50: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825FEE54: 386A1B8C  addi r3, r10, 0x1b8c
	ctx.r[3].s64 = ctx.r[10].s64 + 7052;
	// 825FEE58: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825FEE5C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825FEE60: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 825FEE64: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825FEE68: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 825FEE6C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825FEE70: 4BE67FB1  bl 0x82466e20
	ctx.lr = 0x825FEE74;
	sub_82466E20(ctx, base);
	// 825FEE74: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825FEE78: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825FEE7C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825FEE80: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825FEE88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825FEE88 size=112
    let mut pc: u32 = 0x825FEE88;
    'dispatch: loop {
        match pc {
            0x825FEE88 => {
    //   block [0x825FEE88..0x825FEEF8)
	// 825FEE88: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825FEE8C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825FEE90: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825FEE94: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FEE98: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825FEE9C: 38AA1B8C  addi r5, r10, 0x1b8c
	ctx.r[5].s64 = ctx.r[10].s64 + 7052;
	// 825FEEA0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 825FEEA4: 390B4A48  addi r8, r11, 0x4a48
	ctx.r[8].s64 = ctx.r[11].s64 + 19016;
	// 825FEEA8: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 825FEEAC: 388AB760  addi r4, r10, -0x48a0
	ctx.r[4].s64 = ctx.r[10].s64 + -18592;
	// 825FEEB0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825FEEB4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FEEB8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825FEEBC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825FEEC0: 386A1BBC  addi r3, r10, 0x1bbc
	ctx.r[3].s64 = ctx.r[10].s64 + 7100;
	// 825FEEC4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 825FEEC8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825FEECC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825FEED0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825FEED4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825FEED8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825FEEDC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825FEEE0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825FEEE4: 4BE67F3D  bl 0x82466e20
	ctx.lr = 0x825FEEE8;
	sub_82466E20(ctx, base);
	// 825FEEE8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825FEEEC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825FEEF0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825FEEF4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825FEEF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825FEEF8 size=112
    let mut pc: u32 = 0x825FEEF8;
    'dispatch: loop {
        match pc {
            0x825FEEF8 => {
    //   block [0x825FEEF8..0x825FEF68)
	// 825FEEF8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825FEEFC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825FEF00: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825FEF04: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FEF08: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825FEF0C: 38AA1BBC  addi r5, r10, 0x1bbc
	ctx.r[5].s64 = ctx.r[10].s64 + 7100;
	// 825FEF10: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 825FEF14: 390B4AA8  addi r8, r11, 0x4aa8
	ctx.r[8].s64 = ctx.r[11].s64 + 19112;
	// 825FEF18: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 825FEF1C: 388AB778  addi r4, r10, -0x4888
	ctx.r[4].s64 = ctx.r[10].s64 + -18568;
	// 825FEF20: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825FEF24: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FEF28: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825FEF2C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825FEF30: 386A1BEC  addi r3, r10, 0x1bec
	ctx.r[3].s64 = ctx.r[10].s64 + 7148;
	// 825FEF34: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 825FEF38: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825FEF3C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825FEF40: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825FEF44: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825FEF48: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825FEF4C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825FEF50: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825FEF54: 4BE67ECD  bl 0x82466e20
	ctx.lr = 0x825FEF58;
	sub_82466E20(ctx, base);
	// 825FEF58: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825FEF5C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825FEF60: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825FEF64: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825FEF68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825FEF68 size=24
    let mut pc: u32 = 0x825FEF68;
    'dispatch: loop {
        match pc {
            0x825FEF68 => {
    //   block [0x825FEF68..0x825FEF80)
	// 825FEF68: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825FEF6C: 3D408275  lis r10, -0x7d8b
	ctx.r[10].s64 = -2106261504;
	// 825FEF70: 394AAF68  addi r10, r10, -0x5098
	ctx.r[10].s64 = ctx.r[10].s64 + -20632;
	// 825FEF74: 816B51E8  lwz r11, 0x51e8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20968 as u32) ) } as u64;
	// 825FEF78: 916A0068  stw r11, 0x68(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(104 as u32), ctx.r[11].u32 ) };
	// 825FEF7C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825FEF80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825FEF80 size=116
    let mut pc: u32 = 0x825FEF80;
    'dispatch: loop {
        match pc {
            0x825FEF80 => {
    //   block [0x825FEF80..0x825FEFF4)
	// 825FEF80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825FEF84: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825FEF88: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825FEF8C: 3D408275  lis r10, -0x7d8b
	ctx.r[10].s64 = -2106261504;
	// 825FEF90: 38E00009  li r7, 9
	ctx.r[7].s64 = 9;
	// 825FEF94: 390AAF68  addi r8, r10, -0x5098
	ctx.r[8].s64 = ctx.r[10].s64 + -20632;
	// 825FEF98: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FEF9C: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 825FEFA0: 38AA1BBC  addi r5, r10, 0x1bbc
	ctx.r[5].s64 = ctx.r[10].s64 + 7100;
	// 825FEFA4: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 825FEFA8: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 825FEFAC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825FEFB0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825FEFB4: 388AB78C  addi r4, r10, -0x4874
	ctx.r[4].s64 = ctx.r[10].s64 + -18548;
	// 825FEFB8: 396B90A8  addi r11, r11, -0x6f58
	ctx.r[11].s64 = ctx.r[11].s64 + -28504;
	// 825FEFBC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FEFC0: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 825FEFC4: 386A1C1C  addi r3, r10, 0x1c1c
	ctx.r[3].s64 = ctx.r[10].s64 + 7196;
	// 825FEFC8: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 825FEFCC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825FEFD0: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 825FEFD4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825FEFD8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825FEFDC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825FEFE0: 4BE67E41  bl 0x82466e20
	ctx.lr = 0x825FEFE4;
	sub_82466E20(ctx, base);
	// 825FEFE4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825FEFE8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825FEFEC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825FEFF0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825FEFF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825FEFF8 size=112
    let mut pc: u32 = 0x825FEFF8;
    'dispatch: loop {
        match pc {
            0x825FEFF8 => {
    //   block [0x825FEFF8..0x825FF068)
	// 825FEFF8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825FEFFC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825FF000: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825FF004: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FF008: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825FF00C: 38AA09BC  addi r5, r10, 0x9bc
	ctx.r[5].s64 = ctx.r[10].s64 + 2492;
	// 825FF010: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 825FF014: 390B4AD8  addi r8, r11, 0x4ad8
	ctx.r[8].s64 = ctx.r[11].s64 + 19160;
	// 825FF018: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 825FF01C: 388AB7A4  addi r4, r10, -0x485c
	ctx.r[4].s64 = ctx.r[10].s64 + -18524;
	// 825FF020: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825FF024: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FF028: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825FF02C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825FF030: 386A1C4C  addi r3, r10, 0x1c4c
	ctx.r[3].s64 = ctx.r[10].s64 + 7244;
	// 825FF034: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 825FF038: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825FF03C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825FF040: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825FF044: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 825FF048: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825FF04C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825FF050: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825FF054: 4BE67DCD  bl 0x82466e20
	ctx.lr = 0x825FF058;
	sub_82466E20(ctx, base);
	// 825FF058: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825FF05C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825FF060: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825FF064: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825FF068(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825FF068 size=116
    let mut pc: u32 = 0x825FF068;
    'dispatch: loop {
        match pc {
            0x825FF068 => {
    //   block [0x825FF068..0x825FF0DC)
	// 825FF068: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825FF06C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825FF070: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825FF074: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825FF078: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 825FF07C: 390B4B08  addi r8, r11, 0x4b08
	ctx.r[8].s64 = ctx.r[11].s64 + 19208;
	// 825FF080: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825FF084: 392A90E8  addi r9, r10, -0x6f18
	ctx.r[9].s64 = ctx.r[10].s64 + -28440;
	// 825FF088: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FF08C: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
	// 825FF090: 38AA1F1C  addi r5, r10, 0x1f1c
	ctx.r[5].s64 = ctx.r[10].s64 + 7964;
	// 825FF094: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825FF098: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825FF09C: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 825FF0A0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825FF0A4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825FF0A8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825FF0AC: 3D608293  lis r11, -0x7d6d
	ctx.r[11].s64 = -2104295424;
	// 825FF0B0: 388AB7C4  addi r4, r10, -0x483c
	ctx.r[4].s64 = ctx.r[10].s64 + -18492;
	// 825FF0B4: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 825FF0B8: 386B1C7C  addi r3, r11, 0x1c7c
	ctx.r[3].s64 = ctx.r[11].s64 + 7292;
	// 825FF0BC: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 825FF0C0: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825FF0C4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825FF0C8: 4BE67D59  bl 0x82466e20
	ctx.lr = 0x825FF0CC;
	sub_82466E20(ctx, base);
	// 825FF0CC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825FF0D0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825FF0D4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825FF0D8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825FF0E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825FF0E0 size=112
    let mut pc: u32 = 0x825FF0E0;
    'dispatch: loop {
        match pc {
            0x825FF0E0 => {
    //   block [0x825FF0E0..0x825FF150)
	// 825FF0E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825FF0E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825FF0E8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825FF0EC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FF0F0: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825FF0F4: 38AA1C7C  addi r5, r10, 0x1c7c
	ctx.r[5].s64 = ctx.r[10].s64 + 7292;
	// 825FF0F8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 825FF0FC: 390B4B20  addi r8, r11, 0x4b20
	ctx.r[8].s64 = ctx.r[11].s64 + 19232;
	// 825FF100: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 825FF104: 388AB7D4  addi r4, r10, -0x482c
	ctx.r[4].s64 = ctx.r[10].s64 + -18476;
	// 825FF108: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825FF10C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FF110: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825FF114: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825FF118: 386A1CAC  addi r3, r10, 0x1cac
	ctx.r[3].s64 = ctx.r[10].s64 + 7340;
	// 825FF11C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 825FF120: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825FF124: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825FF128: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825FF12C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825FF130: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825FF134: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825FF138: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825FF13C: 4BE67CE5  bl 0x82466e20
	ctx.lr = 0x825FF140;
	sub_82466E20(ctx, base);
	// 825FF140: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825FF144: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825FF148: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825FF14C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825FF150(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825FF150 size=116
    let mut pc: u32 = 0x825FF150;
    'dispatch: loop {
        match pc {
            0x825FF150 => {
    //   block [0x825FF150..0x825FF1C4)
	// 825FF150: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825FF154: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825FF158: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825FF15C: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825FF160: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 825FF164: 390B4B3C  addi r8, r11, 0x4b3c
	ctx.r[8].s64 = ctx.r[11].s64 + 19260;
	// 825FF168: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825FF16C: 392A9114  addi r9, r10, -0x6eec
	ctx.r[9].s64 = ctx.r[10].s64 + -28396;
	// 825FF170: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FF174: 38E00002  li r7, 2
	ctx.r[7].s64 = 2;
	// 825FF178: 38AA1C7C  addi r5, r10, 0x1c7c
	ctx.r[5].s64 = ctx.r[10].s64 + 7292;
	// 825FF17C: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825FF180: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825FF184: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 825FF188: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825FF18C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825FF190: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825FF194: 3D608293  lis r11, -0x7d6d
	ctx.r[11].s64 = -2104295424;
	// 825FF198: 388AB7E0  addi r4, r10, -0x4820
	ctx.r[4].s64 = ctx.r[10].s64 + -18464;
	// 825FF19C: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 825FF1A0: 386B1CDC  addi r3, r11, 0x1cdc
	ctx.r[3].s64 = ctx.r[11].s64 + 7388;
	// 825FF1A4: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 825FF1A8: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825FF1AC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825FF1B0: 4BE67C71  bl 0x82466e20
	ctx.lr = 0x825FF1B4;
	sub_82466E20(ctx, base);
	// 825FF1B4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825FF1B8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825FF1BC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825FF1C0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825FF1C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825FF1C8 size=112
    let mut pc: u32 = 0x825FF1C8;
    'dispatch: loop {
        match pc {
            0x825FF1C8 => {
    //   block [0x825FF1C8..0x825FF238)
	// 825FF1C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825FF1CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825FF1D0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825FF1D4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FF1D8: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825FF1DC: 38AA1C7C  addi r5, r10, 0x1c7c
	ctx.r[5].s64 = ctx.r[10].s64 + 7292;
	// 825FF1E0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 825FF1E4: 390B4B70  addi r8, r11, 0x4b70
	ctx.r[8].s64 = ctx.r[11].s64 + 19312;
	// 825FF1E8: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 825FF1EC: 388AB7F0  addi r4, r10, -0x4810
	ctx.r[4].s64 = ctx.r[10].s64 + -18448;
	// 825FF1F0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825FF1F4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FF1F8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825FF1FC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825FF200: 386A1D0C  addi r3, r10, 0x1d0c
	ctx.r[3].s64 = ctx.r[10].s64 + 7436;
	// 825FF204: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 825FF208: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825FF20C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825FF210: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825FF214: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825FF218: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825FF21C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825FF220: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825FF224: 4BE67BFD  bl 0x82466e20
	ctx.lr = 0x825FF228;
	sub_82466E20(ctx, base);
	// 825FF228: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825FF22C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825FF230: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825FF234: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825FF238(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825FF238 size=112
    let mut pc: u32 = 0x825FF238;
    'dispatch: loop {
        match pc {
            0x825FF238 => {
    //   block [0x825FF238..0x825FF2A8)
	// 825FF238: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825FF23C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825FF240: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825FF244: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FF248: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825FF24C: 38AA1C7C  addi r5, r10, 0x1c7c
	ctx.r[5].s64 = ctx.r[10].s64 + 7292;
	// 825FF250: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 825FF254: 390B4BB8  addi r8, r11, 0x4bb8
	ctx.r[8].s64 = ctx.r[11].s64 + 19384;
	// 825FF258: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 825FF25C: 388AB808  addi r4, r10, -0x47f8
	ctx.r[4].s64 = ctx.r[10].s64 + -18424;
	// 825FF260: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825FF264: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FF268: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825FF26C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825FF270: 386A1D3C  addi r3, r10, 0x1d3c
	ctx.r[3].s64 = ctx.r[10].s64 + 7484;
	// 825FF274: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 825FF278: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825FF27C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825FF280: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825FF284: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825FF288: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825FF28C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825FF290: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825FF294: 4BE67B8D  bl 0x82466e20
	ctx.lr = 0x825FF298;
	sub_82466E20(ctx, base);
	// 825FF298: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825FF29C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825FF2A0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825FF2A4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825FF2A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825FF2A8 size=108
    let mut pc: u32 = 0x825FF2A8;
    'dispatch: loop {
        match pc {
            0x825FF2A8 => {
    //   block [0x825FF2A8..0x825FF314)
	// 825FF2A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825FF2AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825FF2B0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825FF2B4: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825FF2B8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 825FF2BC: 38EB4C00  addi r7, r11, 0x4c00
	ctx.r[7].s64 = ctx.r[11].s64 + 19456;
	// 825FF2C0: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 825FF2C4: 388AB820  addi r4, r10, -0x47e0
	ctx.r[4].s64 = ctx.r[10].s64 + -18400;
	// 825FF2C8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825FF2CC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FF2D0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 825FF2D4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825FF2D8: 386A1D6C  addi r3, r10, 0x1d6c
	ctx.r[3].s64 = ctx.r[10].s64 + 7532;
	// 825FF2DC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 825FF2E0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825FF2E4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825FF2E8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825FF2EC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825FF2F0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825FF2F4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825FF2F8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825FF2FC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825FF300: 4BE67B21  bl 0x82466e20
	ctx.lr = 0x825FF304;
	sub_82466E20(ctx, base);
	// 825FF304: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825FF308: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825FF30C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825FF310: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825FF318(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825FF318 size=112
    let mut pc: u32 = 0x825FF318;
    'dispatch: loop {
        match pc {
            0x825FF318 => {
    //   block [0x825FF318..0x825FF388)
	// 825FF318: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825FF31C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825FF320: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825FF324: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FF328: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825FF32C: 38AA1C7C  addi r5, r10, 0x1c7c
	ctx.r[5].s64 = ctx.r[10].s64 + 7292;
	// 825FF330: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 825FF334: 390B4C48  addi r8, r11, 0x4c48
	ctx.r[8].s64 = ctx.r[11].s64 + 19528;
	// 825FF338: 39200005  li r9, 5
	ctx.r[9].s64 = 5;
	// 825FF33C: 388AB844  addi r4, r10, -0x47bc
	ctx.r[4].s64 = ctx.r[10].s64 + -18364;
	// 825FF340: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825FF344: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FF348: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825FF34C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825FF350: 386A1D9C  addi r3, r10, 0x1d9c
	ctx.r[3].s64 = ctx.r[10].s64 + 7580;
	// 825FF354: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 825FF358: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825FF35C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825FF360: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825FF364: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825FF368: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825FF36C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825FF370: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825FF374: 4BE67AAD  bl 0x82466e20
	ctx.lr = 0x825FF378;
	sub_82466E20(ctx, base);
	// 825FF378: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825FF37C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825FF380: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825FF384: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825FF388(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825FF388 size=116
    let mut pc: u32 = 0x825FF388;
    'dispatch: loop {
        match pc {
            0x825FF388 => {
    //   block [0x825FF388..0x825FF3FC)
	// 825FF388: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825FF38C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825FF390: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825FF394: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 825FF398: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FF39C: 392B9150  addi r9, r11, -0x6eb0
	ctx.r[9].s64 = ctx.r[11].s64 + -28336;
	// 825FF3A0: 38AA1C7C  addi r5, r10, 0x1c7c
	ctx.r[5].s64 = ctx.r[10].s64 + 7292;
	// 825FF3A4: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 825FF3A8: 38E90014  addi r7, r9, 0x14
	ctx.r[7].s64 = ctx.r[9].s64 + 20;
	// 825FF3AC: 38C00006  li r6, 6
	ctx.r[6].s64 = 6;
	// 825FF3B0: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825FF3B4: 388AB85C  addi r4, r10, -0x47a4
	ctx.r[4].s64 = ctx.r[10].s64 + -18340;
	// 825FF3B8: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825FF3BC: 396B4CC0  addi r11, r11, 0x4cc0
	ctx.r[11].s64 = ctx.r[11].s64 + 19648;
	// 825FF3C0: 90E10064  stw r7, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[7].u32 ) };
	// 825FF3C4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FF3C8: 90C1005C  stw r6, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[6].u32 ) };
	// 825FF3CC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825FF3D0: 386A1DCC  addi r3, r10, 0x1dcc
	ctx.r[3].s64 = ctx.r[10].s64 + 7628;
	// 825FF3D4: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 825FF3D8: 91010074  stw r8, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[8].u32 ) };
	// 825FF3DC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825FF3E0: 9101006C  stw r8, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[8].u32 ) };
	// 825FF3E4: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 825FF3E8: 4BE67A39  bl 0x82466e20
	ctx.lr = 0x825FF3EC;
	sub_82466E20(ctx, base);
	// 825FF3EC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825FF3F0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825FF3F4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825FF3F8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825FF400(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825FF400 size=108
    let mut pc: u32 = 0x825FF400;
    'dispatch: loop {
        match pc {
            0x825FF400 => {
    //   block [0x825FF400..0x825FF46C)
	// 825FF400: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825FF404: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825FF408: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825FF40C: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825FF410: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 825FF414: 38EB4D50  addi r7, r11, 0x4d50
	ctx.r[7].s64 = ctx.r[11].s64 + 19792;
	// 825FF418: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 825FF41C: 388AB870  addi r4, r10, -0x4790
	ctx.r[4].s64 = ctx.r[10].s64 + -18320;
	// 825FF420: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825FF424: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FF428: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 825FF42C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825FF430: 386A1DFC  addi r3, r10, 0x1dfc
	ctx.r[3].s64 = ctx.r[10].s64 + 7676;
	// 825FF434: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 825FF438: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825FF43C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825FF440: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825FF444: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825FF448: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825FF44C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825FF450: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825FF454: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825FF458: 4BE679C9  bl 0x82466e20
	ctx.lr = 0x825FF45C;
	sub_82466E20(ctx, base);
	// 825FF45C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825FF460: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825FF464: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825FF468: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825FF470(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825FF470 size=108
    let mut pc: u32 = 0x825FF470;
    'dispatch: loop {
        match pc {
            0x825FF470 => {
    //   block [0x825FF470..0x825FF4DC)
	// 825FF470: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825FF474: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825FF478: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825FF47C: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825FF480: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 825FF484: 38EB4D98  addi r7, r11, 0x4d98
	ctx.r[7].s64 = ctx.r[11].s64 + 19864;
	// 825FF488: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 825FF48C: 388AB898  addi r4, r10, -0x4768
	ctx.r[4].s64 = ctx.r[10].s64 + -18280;
	// 825FF490: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825FF494: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FF498: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 825FF49C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825FF4A0: 386A1E2C  addi r3, r10, 0x1e2c
	ctx.r[3].s64 = ctx.r[10].s64 + 7724;
	// 825FF4A4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 825FF4A8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825FF4AC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825FF4B0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825FF4B4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825FF4B8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825FF4BC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825FF4C0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825FF4C4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825FF4C8: 4BE67959  bl 0x82466e20
	ctx.lr = 0x825FF4CC;
	sub_82466E20(ctx, base);
	// 825FF4CC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825FF4D0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825FF4D4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825FF4D8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825FF4E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825FF4E0 size=112
    let mut pc: u32 = 0x825FF4E0;
    'dispatch: loop {
        match pc {
            0x825FF4E0 => {
    //   block [0x825FF4E0..0x825FF550)
	// 825FF4E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825FF4E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825FF4E8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825FF4EC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FF4F0: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825FF4F4: 38AA1C7C  addi r5, r10, 0x1c7c
	ctx.r[5].s64 = ctx.r[10].s64 + 7292;
	// 825FF4F8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 825FF4FC: 390B4DF8  addi r8, r11, 0x4df8
	ctx.r[8].s64 = ctx.r[11].s64 + 19960;
	// 825FF500: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 825FF504: 388AB8C0  addi r4, r10, -0x4740
	ctx.r[4].s64 = ctx.r[10].s64 + -18240;
	// 825FF508: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825FF50C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FF510: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825FF514: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825FF518: 386A1E5C  addi r3, r10, 0x1e5c
	ctx.r[3].s64 = ctx.r[10].s64 + 7772;
	// 825FF51C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 825FF520: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825FF524: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825FF528: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825FF52C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825FF530: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825FF534: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825FF538: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825FF53C: 4BE678E5  bl 0x82466e20
	ctx.lr = 0x825FF540;
	sub_82466E20(ctx, base);
	// 825FF540: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825FF544: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825FF548: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825FF54C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825FF550(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825FF550 size=112
    let mut pc: u32 = 0x825FF550;
    'dispatch: loop {
        match pc {
            0x825FF550 => {
    //   block [0x825FF550..0x825FF5C0)
	// 825FF550: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825FF554: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825FF558: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825FF55C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FF560: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825FF564: 38AA1C7C  addi r5, r10, 0x1c7c
	ctx.r[5].s64 = ctx.r[10].s64 + 7292;
	// 825FF568: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 825FF56C: 390B4E58  addi r8, r11, 0x4e58
	ctx.r[8].s64 = ctx.r[11].s64 + 20056;
	// 825FF570: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 825FF574: 388AB8E0  addi r4, r10, -0x4720
	ctx.r[4].s64 = ctx.r[10].s64 + -18208;
	// 825FF578: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825FF57C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FF580: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825FF584: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825FF588: 386A1E8C  addi r3, r10, 0x1e8c
	ctx.r[3].s64 = ctx.r[10].s64 + 7820;
	// 825FF58C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 825FF590: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825FF594: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825FF598: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825FF59C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825FF5A0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825FF5A4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825FF5A8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825FF5AC: 4BE67875  bl 0x82466e20
	ctx.lr = 0x825FF5B0;
	sub_82466E20(ctx, base);
	// 825FF5B0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825FF5B4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825FF5B8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825FF5BC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825FF5C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825FF5C0 size=24
    let mut pc: u32 = 0x825FF5C0;
    'dispatch: loop {
        match pc {
            0x825FF5C0 => {
    //   block [0x825FF5C0..0x825FF5D8)
	// 825FF5C0: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825FF5C4: 3D408275  lis r10, -0x7d8b
	ctx.r[10].s64 = -2106261504;
	// 825FF5C8: 394AB040  addi r10, r10, -0x4fc0
	ctx.r[10].s64 = ctx.r[10].s64 + -20416;
	// 825FF5CC: 816B51E8  lwz r11, 0x51e8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20968 as u32) ) } as u64;
	// 825FF5D0: 916A0020  stw r11, 0x20(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 825FF5D4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825FF5D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825FF5D8 size=116
    let mut pc: u32 = 0x825FF5D8;
    'dispatch: loop {
        match pc {
            0x825FF5D8 => {
    //   block [0x825FF5D8..0x825FF64C)
	// 825FF5D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825FF5DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825FF5E0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825FF5E4: 3D408275  lis r10, -0x7d8b
	ctx.r[10].s64 = -2106261504;
	// 825FF5E8: 38E00007  li r7, 7
	ctx.r[7].s64 = 7;
	// 825FF5EC: 390AB040  addi r8, r10, -0x4fc0
	ctx.r[8].s64 = ctx.r[10].s64 + -20416;
	// 825FF5F0: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FF5F4: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 825FF5F8: 38AA1C7C  addi r5, r10, 0x1c7c
	ctx.r[5].s64 = ctx.r[10].s64 + 7292;
	// 825FF5FC: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 825FF600: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 825FF604: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825FF608: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825FF60C: 388AB8F0  addi r4, r10, -0x4710
	ctx.r[4].s64 = ctx.r[10].s64 + -18192;
	// 825FF610: 396B9180  addi r11, r11, -0x6e80
	ctx.r[11].s64 = ctx.r[11].s64 + -28288;
	// 825FF614: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FF618: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825FF61C: 386A1EBC  addi r3, r10, 0x1ebc
	ctx.r[3].s64 = ctx.r[10].s64 + 7868;
	// 825FF620: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 825FF624: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825FF628: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 825FF62C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825FF630: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825FF634: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825FF638: 4BE677E9  bl 0x82466e20
	ctx.lr = 0x825FF63C;
	sub_82466E20(ctx, base);
	// 825FF63C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825FF640: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825FF644: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825FF648: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825FF650(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825FF650 size=100
    let mut pc: u32 = 0x825FF650;
    'dispatch: loop {
        match pc {
            0x825FF650 => {
    //   block [0x825FF650..0x825FF6B4)
	// 825FF650: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825FF654: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825FF658: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825FF65C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FF660: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825FF664: 38AA176C  addi r5, r10, 0x176c
	ctx.r[5].s64 = ctx.r[10].s64 + 5996;
	// 825FF668: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 825FF66C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825FF670: 388AB904  addi r4, r10, -0x46fc
	ctx.r[4].s64 = ctx.r[10].s64 + -18172;
	// 825FF674: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FF678: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825FF67C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825FF680: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825FF684: 386A1EEC  addi r3, r10, 0x1eec
	ctx.r[3].s64 = ctx.r[10].s64 + 7916;
	// 825FF688: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825FF68C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825FF690: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 825FF694: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825FF698: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 825FF69C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825FF6A0: 4BE67781  bl 0x82466e20
	ctx.lr = 0x825FF6A4;
	sub_82466E20(ctx, base);
	// 825FF6A4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825FF6A8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825FF6AC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825FF6B0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825FF6B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825FF6B8 size=100
    let mut pc: u32 = 0x825FF6B8;
    'dispatch: loop {
        match pc {
            0x825FF6B8 => {
    //   block [0x825FF6B8..0x825FF71C)
	// 825FF6B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825FF6BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825FF6C0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825FF6C4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FF6C8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825FF6CC: 38AA176C  addi r5, r10, 0x176c
	ctx.r[5].s64 = ctx.r[10].s64 + 5996;
	// 825FF6D0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 825FF6D4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825FF6D8: 388AB918  addi r4, r10, -0x46e8
	ctx.r[4].s64 = ctx.r[10].s64 + -18152;
	// 825FF6DC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FF6E0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825FF6E4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825FF6E8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825FF6EC: 386A1F1C  addi r3, r10, 0x1f1c
	ctx.r[3].s64 = ctx.r[10].s64 + 7964;
	// 825FF6F0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825FF6F4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825FF6F8: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 825FF6FC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825FF700: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 825FF704: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825FF708: 4BE67719  bl 0x82466e20
	ctx.lr = 0x825FF70C;
	sub_82466E20(ctx, base);
	// 825FF70C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825FF710: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825FF714: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825FF718: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825FF720(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825FF720 size=112
    let mut pc: u32 = 0x825FF720;
    'dispatch: loop {
        match pc {
            0x825FF720 => {
    //   block [0x825FF720..0x825FF790)
	// 825FF720: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825FF724: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825FF728: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825FF72C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FF730: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825FF734: 38AA1EEC  addi r5, r10, 0x1eec
	ctx.r[5].s64 = ctx.r[10].s64 + 7916;
	// 825FF738: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 825FF73C: 390B4E70  addi r8, r11, 0x4e70
	ctx.r[8].s64 = ctx.r[11].s64 + 20080;
	// 825FF740: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 825FF744: 388AB92C  addi r4, r10, -0x46d4
	ctx.r[4].s64 = ctx.r[10].s64 + -18132;
	// 825FF748: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825FF74C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FF750: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825FF754: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825FF758: 386A1F4C  addi r3, r10, 0x1f4c
	ctx.r[3].s64 = ctx.r[10].s64 + 8012;
	// 825FF75C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 825FF760: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825FF764: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825FF768: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825FF76C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825FF770: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825FF774: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825FF778: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825FF77C: 4BE676A5  bl 0x82466e20
	ctx.lr = 0x825FF780;
	sub_82466E20(ctx, base);
	// 825FF780: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825FF784: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825FF788: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825FF78C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825FF790(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825FF790 size=112
    let mut pc: u32 = 0x825FF790;
    'dispatch: loop {
        match pc {
            0x825FF790 => {
    //   block [0x825FF790..0x825FF800)
	// 825FF790: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825FF794: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825FF798: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825FF79C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FF7A0: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825FF7A4: 38AA1EEC  addi r5, r10, 0x1eec
	ctx.r[5].s64 = ctx.r[10].s64 + 7916;
	// 825FF7A8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 825FF7AC: 390B4EB8  addi r8, r11, 0x4eb8
	ctx.r[8].s64 = ctx.r[11].s64 + 20152;
	// 825FF7B0: 39200008  li r9, 8
	ctx.r[9].s64 = 8;
	// 825FF7B4: 388AB93C  addi r4, r10, -0x46c4
	ctx.r[4].s64 = ctx.r[10].s64 + -18116;
	// 825FF7B8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825FF7BC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FF7C0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825FF7C4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825FF7C8: 386A1F7C  addi r3, r10, 0x1f7c
	ctx.r[3].s64 = ctx.r[10].s64 + 8060;
	// 825FF7CC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 825FF7D0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825FF7D4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825FF7D8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825FF7DC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825FF7E0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825FF7E4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825FF7E8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825FF7EC: 4BE67635  bl 0x82466e20
	ctx.lr = 0x825FF7F0;
	sub_82466E20(ctx, base);
	// 825FF7F0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825FF7F4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825FF7F8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825FF7FC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825FF800(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825FF800 size=112
    let mut pc: u32 = 0x825FF800;
    'dispatch: loop {
        match pc {
            0x825FF800 => {
    //   block [0x825FF800..0x825FF870)
	// 825FF800: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825FF804: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825FF808: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825FF80C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FF810: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825FF814: 38AA1F7C  addi r5, r10, 0x1f7c
	ctx.r[5].s64 = ctx.r[10].s64 + 8060;
	// 825FF818: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 825FF81C: 390B4F78  addi r8, r11, 0x4f78
	ctx.r[8].s64 = ctx.r[11].s64 + 20344;
	// 825FF820: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 825FF824: 388AB958  addi r4, r10, -0x46a8
	ctx.r[4].s64 = ctx.r[10].s64 + -18088;
	// 825FF828: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825FF82C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FF830: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825FF834: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825FF838: 386A1FAC  addi r3, r10, 0x1fac
	ctx.r[3].s64 = ctx.r[10].s64 + 8108;
	// 825FF83C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 825FF840: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825FF844: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825FF848: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825FF84C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825FF850: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825FF854: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825FF858: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825FF85C: 4BE675C5  bl 0x82466e20
	ctx.lr = 0x825FF860;
	sub_82466E20(ctx, base);
	// 825FF860: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825FF864: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825FF868: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825FF86C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825FF870(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825FF870 size=112
    let mut pc: u32 = 0x825FF870;
    'dispatch: loop {
        match pc {
            0x825FF870 => {
    //   block [0x825FF870..0x825FF8E0)
	// 825FF870: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825FF874: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825FF878: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825FF87C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FF880: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825FF884: 38AA1B8C  addi r5, r10, 0x1b8c
	ctx.r[5].s64 = ctx.r[10].s64 + 7052;
	// 825FF888: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 825FF88C: 390B4FA8  addi r8, r11, 0x4fa8
	ctx.r[8].s64 = ctx.r[11].s64 + 20392;
	// 825FF890: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 825FF894: 388AB97C  addi r4, r10, -0x4684
	ctx.r[4].s64 = ctx.r[10].s64 + -18052;
	// 825FF898: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825FF89C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FF8A0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825FF8A4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825FF8A8: 386A1FDC  addi r3, r10, 0x1fdc
	ctx.r[3].s64 = ctx.r[10].s64 + 8156;
	// 825FF8AC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 825FF8B0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825FF8B4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825FF8B8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825FF8BC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825FF8C0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825FF8C4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825FF8C8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825FF8CC: 4BE67555  bl 0x82466e20
	ctx.lr = 0x825FF8D0;
	sub_82466E20(ctx, base);
	// 825FF8D0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825FF8D4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825FF8D8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825FF8DC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825FF8E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825FF8E0 size=112
    let mut pc: u32 = 0x825FF8E0;
    'dispatch: loop {
        match pc {
            0x825FF8E0 => {
    //   block [0x825FF8E0..0x825FF950)
	// 825FF8E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825FF8E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825FF8E8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825FF8EC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FF8F0: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825FF8F4: 38AA17FC  addi r5, r10, 0x17fc
	ctx.r[5].s64 = ctx.r[10].s64 + 6140;
	// 825FF8F8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 825FF8FC: 390B4FD8  addi r8, r11, 0x4fd8
	ctx.r[8].s64 = ctx.r[11].s64 + 20440;
	// 825FF900: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 825FF904: 388AB9A0  addi r4, r10, -0x4660
	ctx.r[4].s64 = ctx.r[10].s64 + -18016;
	// 825FF908: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825FF90C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FF910: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825FF914: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825FF918: 386A200C  addi r3, r10, 0x200c
	ctx.r[3].s64 = ctx.r[10].s64 + 8204;
	// 825FF91C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 825FF920: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825FF924: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825FF928: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825FF92C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825FF930: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825FF934: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825FF938: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825FF93C: 4BE674E5  bl 0x82466e20
	ctx.lr = 0x825FF940;
	sub_82466E20(ctx, base);
	// 825FF940: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825FF944: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825FF948: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825FF94C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825FF950(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825FF950 size=112
    let mut pc: u32 = 0x825FF950;
    'dispatch: loop {
        match pc {
            0x825FF950 => {
    //   block [0x825FF950..0x825FF9C0)
	// 825FF950: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825FF954: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825FF958: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825FF95C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FF960: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825FF964: 38AA176C  addi r5, r10, 0x176c
	ctx.r[5].s64 = ctx.r[10].s64 + 5996;
	// 825FF968: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 825FF96C: 390B5008  addi r8, r11, 0x5008
	ctx.r[8].s64 = ctx.r[11].s64 + 20488;
	// 825FF970: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 825FF974: 388AB9C4  addi r4, r10, -0x463c
	ctx.r[4].s64 = ctx.r[10].s64 + -17980;
	// 825FF978: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825FF97C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FF980: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825FF984: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825FF988: 386A203C  addi r3, r10, 0x203c
	ctx.r[3].s64 = ctx.r[10].s64 + 8252;
	// 825FF98C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 825FF990: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825FF994: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825FF998: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825FF99C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825FF9A0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825FF9A4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825FF9A8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825FF9AC: 4BE67475  bl 0x82466e20
	ctx.lr = 0x825FF9B0;
	sub_82466E20(ctx, base);
	// 825FF9B0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825FF9B4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825FF9B8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825FF9BC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825FF9C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825FF9C0 size=112
    let mut pc: u32 = 0x825FF9C0;
    'dispatch: loop {
        match pc {
            0x825FF9C0 => {
    //   block [0x825FF9C0..0x825FFA30)
	// 825FF9C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825FF9C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825FF9C8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825FF9CC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FF9D0: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825FF9D4: 38AA1C7C  addi r5, r10, 0x1c7c
	ctx.r[5].s64 = ctx.r[10].s64 + 7292;
	// 825FF9D8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 825FF9DC: 390B5038  addi r8, r11, 0x5038
	ctx.r[8].s64 = ctx.r[11].s64 + 20536;
	// 825FF9E0: 39200005  li r9, 5
	ctx.r[9].s64 = 5;
	// 825FF9E4: 388AB9D0  addi r4, r10, -0x4630
	ctx.r[4].s64 = ctx.r[10].s64 + -17968;
	// 825FF9E8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825FF9EC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FF9F0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825FF9F4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825FF9F8: 386A206C  addi r3, r10, 0x206c
	ctx.r[3].s64 = ctx.r[10].s64 + 8300;
	// 825FF9FC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 825FFA00: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825FFA04: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825FFA08: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825FFA0C: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 825FFA10: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825FFA14: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825FFA18: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825FFA1C: 4BE67405  bl 0x82466e20
	ctx.lr = 0x825FFA20;
	sub_82466E20(ctx, base);
	// 825FFA20: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825FFA24: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825FFA28: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825FFA2C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825FFA30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825FFA30 size=108
    let mut pc: u32 = 0x825FFA30;
    'dispatch: loop {
        match pc {
            0x825FFA30 => {
    //   block [0x825FFA30..0x825FFA9C)
	// 825FFA30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825FFA34: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825FFA38: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825FFA3C: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825FFA40: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 825FFA44: 38EB50B0  addi r7, r11, 0x50b0
	ctx.r[7].s64 = ctx.r[11].s64 + 20656;
	// 825FFA48: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 825FFA4C: 388AB9E4  addi r4, r10, -0x461c
	ctx.r[4].s64 = ctx.r[10].s64 + -17948;
	// 825FFA50: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825FFA54: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FFA58: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 825FFA5C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825FFA60: 386A209C  addi r3, r10, 0x209c
	ctx.r[3].s64 = ctx.r[10].s64 + 8348;
	// 825FFA64: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 825FFA68: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825FFA6C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825FFA70: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825FFA74: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825FFA78: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825FFA7C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825FFA80: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825FFA84: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825FFA88: 4BE67399  bl 0x82466e20
	ctx.lr = 0x825FFA8C;
	sub_82466E20(ctx, base);
	// 825FFA8C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825FFA90: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825FFA94: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825FFA98: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825FFAA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825FFAA0 size=112
    let mut pc: u32 = 0x825FFAA0;
    'dispatch: loop {
        match pc {
            0x825FFAA0 => {
    //   block [0x825FFAA0..0x825FFB10)
	// 825FFAA0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825FFAA4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825FFAA8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825FFAAC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FFAB0: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825FFAB4: 38AA176C  addi r5, r10, 0x176c
	ctx.r[5].s64 = ctx.r[10].s64 + 5996;
	// 825FFAB8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 825FFABC: 390B50E0  addi r8, r11, 0x50e0
	ctx.r[8].s64 = ctx.r[11].s64 + 20704;
	// 825FFAC0: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 825FFAC4: 388AB9F8  addi r4, r10, -0x4608
	ctx.r[4].s64 = ctx.r[10].s64 + -17928;
	// 825FFAC8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825FFACC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FFAD0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825FFAD4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825FFAD8: 386A20CC  addi r3, r10, 0x20cc
	ctx.r[3].s64 = ctx.r[10].s64 + 8396;
	// 825FFADC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 825FFAE0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825FFAE4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825FFAE8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825FFAEC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825FFAF0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825FFAF4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825FFAF8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825FFAFC: 4BE67325  bl 0x82466e20
	ctx.lr = 0x825FFB00;
	sub_82466E20(ctx, base);
	// 825FFB00: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825FFB04: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825FFB08: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825FFB0C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825FFB10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825FFB10 size=112
    let mut pc: u32 = 0x825FFB10;
    'dispatch: loop {
        match pc {
            0x825FFB10 => {
    //   block [0x825FFB10..0x825FFB80)
	// 825FFB10: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825FFB14: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825FFB18: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825FFB1C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FFB20: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825FFB24: 38AA1F1C  addi r5, r10, 0x1f1c
	ctx.r[5].s64 = ctx.r[10].s64 + 7964;
	// 825FFB28: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 825FFB2C: 390B5110  addi r8, r11, 0x5110
	ctx.r[8].s64 = ctx.r[11].s64 + 20752;
	// 825FFB30: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 825FFB34: 388ABA0C  addi r4, r10, -0x45f4
	ctx.r[4].s64 = ctx.r[10].s64 + -17908;
	// 825FFB38: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825FFB3C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FFB40: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825FFB44: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825FFB48: 386A20FC  addi r3, r10, 0x20fc
	ctx.r[3].s64 = ctx.r[10].s64 + 8444;
	// 825FFB4C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 825FFB50: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825FFB54: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825FFB58: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825FFB5C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825FFB60: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825FFB64: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825FFB68: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825FFB6C: 4BE672B5  bl 0x82466e20
	ctx.lr = 0x825FFB70;
	sub_82466E20(ctx, base);
	// 825FFB70: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825FFB74: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825FFB78: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825FFB7C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825FFB80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825FFB80 size=100
    let mut pc: u32 = 0x825FFB80;
    'dispatch: loop {
        match pc {
            0x825FFB80 => {
    //   block [0x825FFB80..0x825FFBE4)
	// 825FFB80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825FFB84: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825FFB88: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825FFB8C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FFB90: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825FFB94: 38AA176C  addi r5, r10, 0x176c
	ctx.r[5].s64 = ctx.r[10].s64 + 5996;
	// 825FFB98: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 825FFB9C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825FFBA0: 388ABA20  addi r4, r10, -0x45e0
	ctx.r[4].s64 = ctx.r[10].s64 + -17888;
	// 825FFBA4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FFBA8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825FFBAC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825FFBB0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825FFBB4: 386A212C  addi r3, r10, 0x212c
	ctx.r[3].s64 = ctx.r[10].s64 + 8492;
	// 825FFBB8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825FFBBC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825FFBC0: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 825FFBC4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825FFBC8: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 825FFBCC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825FFBD0: 4BE67251  bl 0x82466e20
	ctx.lr = 0x825FFBD4;
	sub_82466E20(ctx, base);
	// 825FFBD4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825FFBD8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825FFBDC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825FFBE0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825FFBE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825FFBE8 size=112
    let mut pc: u32 = 0x825FFBE8;
    'dispatch: loop {
        match pc {
            0x825FFBE8 => {
    //   block [0x825FFBE8..0x825FFC58)
	// 825FFBE8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825FFBEC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825FFBF0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825FFBF4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FFBF8: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825FFBFC: 38AA176C  addi r5, r10, 0x176c
	ctx.r[5].s64 = ctx.r[10].s64 + 5996;
	// 825FFC00: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 825FFC04: 390B5140  addi r8, r11, 0x5140
	ctx.r[8].s64 = ctx.r[11].s64 + 20800;
	// 825FFC08: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 825FFC0C: 388ABA38  addi r4, r10, -0x45c8
	ctx.r[4].s64 = ctx.r[10].s64 + -17864;
	// 825FFC10: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825FFC14: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FFC18: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825FFC1C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825FFC20: 386A215C  addi r3, r10, 0x215c
	ctx.r[3].s64 = ctx.r[10].s64 + 8540;
	// 825FFC24: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 825FFC28: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825FFC2C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825FFC30: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825FFC34: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825FFC38: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825FFC3C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825FFC40: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825FFC44: 4BE671DD  bl 0x82466e20
	ctx.lr = 0x825FFC48;
	sub_82466E20(ctx, base);
	// 825FFC48: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825FFC4C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825FFC50: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825FFC54: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825FFC58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825FFC58 size=96
    let mut pc: u32 = 0x825FFC58;
    'dispatch: loop {
        match pc {
            0x825FFC58 => {
    //   block [0x825FFC58..0x825FFCB8)
	// 825FFC58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825FFC5C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825FFC60: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825FFC64: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 825FFC68: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825FFC6C: 388ABA4C  addi r4, r10, -0x45b4
	ctx.r[4].s64 = ctx.r[10].s64 + -17844;
	// 825FFC70: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FFC74: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825FFC78: 386A218C  addi r3, r10, 0x218c
	ctx.r[3].s64 = ctx.r[10].s64 + 8588;
	// 825FFC7C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825FFC80: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825FFC84: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 825FFC88: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825FFC8C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825FFC90: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825FFC94: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825FFC98: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 825FFC9C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825FFCA0: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 825FFCA4: 4BE6717D  bl 0x82466e20
	ctx.lr = 0x825FFCA8;
	sub_82466E20(ctx, base);
	// 825FFCA8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825FFCAC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825FFCB0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825FFCB4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825FFCB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825FFCB8 size=108
    let mut pc: u32 = 0x825FFCB8;
    'dispatch: loop {
        match pc {
            0x825FFCB8 => {
    //   block [0x825FFCB8..0x825FFD24)
	// 825FFCB8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825FFCBC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825FFCC0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825FFCC4: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825FFCC8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 825FFCCC: 38EB5188  addi r7, r11, 0x5188
	ctx.r[7].s64 = ctx.r[11].s64 + 20872;
	// 825FFCD0: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 825FFCD4: 388ABA68  addi r4, r10, -0x4598
	ctx.r[4].s64 = ctx.r[10].s64 + -17816;
	// 825FFCD8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825FFCDC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FFCE0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 825FFCE4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825FFCE8: 386A21BC  addi r3, r10, 0x21bc
	ctx.r[3].s64 = ctx.r[10].s64 + 8636;
	// 825FFCEC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 825FFCF0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825FFCF4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825FFCF8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825FFCFC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825FFD00: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825FFD04: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825FFD08: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825FFD0C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825FFD10: 4BE67111  bl 0x82466e20
	ctx.lr = 0x825FFD14;
	sub_82466E20(ctx, base);
	// 825FFD14: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825FFD18: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825FFD1C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825FFD20: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825FFD28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825FFD28 size=100
    let mut pc: u32 = 0x825FFD28;
    'dispatch: loop {
        match pc {
            0x825FFD28 => {
    //   block [0x825FFD28..0x825FFD8C)
	// 825FFD28: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825FFD2C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825FFD30: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825FFD34: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 825FFD38: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825FFD3C: 392A91F0  addi r9, r10, -0x6e10
	ctx.r[9].s64 = ctx.r[10].s64 + -28176;
	// 825FFD40: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 825FFD44: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825FFD48: 388ABA80  addi r4, r10, -0x4580
	ctx.r[4].s64 = ctx.r[10].s64 + -17792;
	// 825FFD4C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FFD50: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825FFD54: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825FFD58: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825FFD5C: 386A21EC  addi r3, r10, 0x21ec
	ctx.r[3].s64 = ctx.r[10].s64 + 8684;
	// 825FFD60: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825FFD64: 39400003  li r10, 3
	ctx.r[10].s64 = 3;
	// 825FFD68: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 825FFD6C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825FFD70: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 825FFD74: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825FFD78: 4BE670A9  bl 0x82466e20
	ctx.lr = 0x825FFD7C;
	sub_82466E20(ctx, base);
	// 825FFD7C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825FFD80: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825FFD84: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825FFD88: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825FFD90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825FFD90 size=24
    let mut pc: u32 = 0x825FFD90;
    'dispatch: loop {
        match pc {
            0x825FFD90 => {
    //   block [0x825FFD90..0x825FFDA8)
	// 825FFD90: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825FFD94: 3D408275  lis r10, -0x7d8b
	ctx.r[10].s64 = -2106261504;
	// 825FFD98: 394AB0E8  addi r10, r10, -0x4f18
	ctx.r[10].s64 = ctx.r[10].s64 + -20248;
	// 825FFD9C: 816B51F4  lwz r11, 0x51f4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20980 as u32) ) } as u64;
	// 825FFDA0: 916A0008  stw r11, 8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 825FFDA4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825FFDA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825FFDA8 size=112
    let mut pc: u32 = 0x825FFDA8;
    'dispatch: loop {
        match pc {
            0x825FFDA8 => {
    //   block [0x825FFDA8..0x825FFE18)
	// 825FFDA8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825FFDAC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825FFDB0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825FFDB4: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 825FFDB8: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 825FFDBC: 392A9330  addi r9, r10, -0x6cd0
	ctx.r[9].s64 = ctx.r[10].s64 + -27856;
	// 825FFDC0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 825FFDC4: 390BB0E8  addi r8, r11, -0x4f18
	ctx.r[8].s64 = ctx.r[11].s64 + -20248;
	// 825FFDC8: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
	// 825FFDCC: 388ABA94  addi r4, r10, -0x456c
	ctx.r[4].s64 = ctx.r[10].s64 + -17772;
	// 825FFDD0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825FFDD4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FFDD8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825FFDDC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825FFDE0: 386A221C  addi r3, r10, 0x221c
	ctx.r[3].s64 = ctx.r[10].s64 + 8732;
	// 825FFDE4: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 825FFDE8: 39400002  li r10, 2
	ctx.r[10].s64 = 2;
	// 825FFDEC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825FFDF0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825FFDF4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825FFDF8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825FFDFC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825FFE00: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825FFE04: 4BE6701D  bl 0x82466e20
	ctx.lr = 0x825FFE08;
	sub_82466E20(ctx, base);
	// 825FFE08: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825FFE0C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825FFE10: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825FFE14: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825FFE18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825FFE18 size=112
    let mut pc: u32 = 0x825FFE18;
    'dispatch: loop {
        match pc {
            0x825FFE18 => {
    //   block [0x825FFE18..0x825FFE88)
	// 825FFE18: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825FFE1C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825FFE20: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825FFE24: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FFE28: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825FFE2C: 38AA221C  addi r5, r10, 0x221c
	ctx.r[5].s64 = ctx.r[10].s64 + 8732;
	// 825FFE30: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 825FFE34: 390B51FC  addi r8, r11, 0x51fc
	ctx.r[8].s64 = ctx.r[11].s64 + 20988;
	// 825FFE38: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 825FFE3C: 388ABAA8  addi r4, r10, -0x4558
	ctx.r[4].s64 = ctx.r[10].s64 + -17752;
	// 825FFE40: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825FFE44: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FFE48: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825FFE4C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825FFE50: 386A224C  addi r3, r10, 0x224c
	ctx.r[3].s64 = ctx.r[10].s64 + 8780;
	// 825FFE54: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 825FFE58: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825FFE5C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825FFE60: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825FFE64: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825FFE68: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825FFE6C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825FFE70: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825FFE74: 4BE66FAD  bl 0x82466e20
	ctx.lr = 0x825FFE78;
	sub_82466E20(ctx, base);
	// 825FFE78: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825FFE7C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825FFE80: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825FFE84: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825FFE88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825FFE88 size=108
    let mut pc: u32 = 0x825FFE88;
    'dispatch: loop {
        match pc {
            0x825FFE88 => {
    //   block [0x825FFE88..0x825FFEF4)
	// 825FFE88: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825FFE8C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825FFE90: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825FFE94: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825FFE98: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 825FFE9C: 38EB522C  addi r7, r11, 0x522c
	ctx.r[7].s64 = ctx.r[11].s64 + 21036;
	// 825FFEA0: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 825FFEA4: 388ABAC0  addi r4, r10, -0x4540
	ctx.r[4].s64 = ctx.r[10].s64 + -17728;
	// 825FFEA8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825FFEAC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FFEB0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 825FFEB4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825FFEB8: 386A227C  addi r3, r10, 0x227c
	ctx.r[3].s64 = ctx.r[10].s64 + 8828;
	// 825FFEBC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 825FFEC0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825FFEC4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825FFEC8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825FFECC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825FFED0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825FFED4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825FFED8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825FFEDC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825FFEE0: 4BE66F41  bl 0x82466e20
	ctx.lr = 0x825FFEE4;
	sub_82466E20(ctx, base);
	// 825FFEE4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825FFEE8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825FFEEC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825FFEF0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825FFEF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825FFEF8 size=112
    let mut pc: u32 = 0x825FFEF8;
    'dispatch: loop {
        match pc {
            0x825FFEF8 => {
    //   block [0x825FFEF8..0x825FFF68)
	// 825FFEF8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825FFEFC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825FFF00: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825FFF04: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FFF08: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825FFF0C: 38AA221C  addi r5, r10, 0x221c
	ctx.r[5].s64 = ctx.r[10].s64 + 8732;
	// 825FFF10: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 825FFF14: 390B5248  addi r8, r11, 0x5248
	ctx.r[8].s64 = ctx.r[11].s64 + 21064;
	// 825FFF18: 39200007  li r9, 7
	ctx.r[9].s64 = 7;
	// 825FFF1C: 388ABAD0  addi r4, r10, -0x4530
	ctx.r[4].s64 = ctx.r[10].s64 + -17712;
	// 825FFF20: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825FFF24: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FFF28: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825FFF2C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825FFF30: 386A22AC  addi r3, r10, 0x22ac
	ctx.r[3].s64 = ctx.r[10].s64 + 8876;
	// 825FFF34: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 825FFF38: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825FFF3C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825FFF40: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825FFF44: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825FFF48: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825FFF4C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825FFF50: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825FFF54: 4BE66ECD  bl 0x82466e20
	ctx.lr = 0x825FFF58;
	sub_82466E20(ctx, base);
	// 825FFF58: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825FFF5C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825FFF60: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825FFF64: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825FFF68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825FFF68 size=100
    let mut pc: u32 = 0x825FFF68;
    'dispatch: loop {
        match pc {
            0x825FFF68 => {
    //   block [0x825FFF68..0x825FFFCC)
	// 825FFF68: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825FFF6C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825FFF70: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825FFF74: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FFF78: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825FFF7C: 38AA221C  addi r5, r10, 0x221c
	ctx.r[5].s64 = ctx.r[10].s64 + 8732;
	// 825FFF80: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 825FFF84: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825FFF88: 388ABAF0  addi r4, r10, -0x4510
	ctx.r[4].s64 = ctx.r[10].s64 + -17680;
	// 825FFF8C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FFF90: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825FFF94: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825FFF98: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825FFF9C: 386A22DC  addi r3, r10, 0x22dc
	ctx.r[3].s64 = ctx.r[10].s64 + 8924;
	// 825FFFA0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825FFFA4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825FFFA8: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 825FFFAC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825FFFB0: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 825FFFB4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825FFFB8: 4BE66E69  bl 0x82466e20
	ctx.lr = 0x825FFFBC;
	sub_82466E20(ctx, base);
	// 825FFFBC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825FFFC0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825FFFC4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825FFFC8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825FFFD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825FFFD0 size=112
    let mut pc: u32 = 0x825FFFD0;
    'dispatch: loop {
        match pc {
            0x825FFFD0 => {
    //   block [0x825FFFD0..0x82600040)
	// 825FFFD0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825FFFD4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825FFFD8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825FFFDC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825FFFE0: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825FFFE4: 38AA221C  addi r5, r10, 0x221c
	ctx.r[5].s64 = ctx.r[10].s64 + 8732;
	// 825FFFE8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 825FFFEC: 390B52F0  addi r8, r11, 0x52f0
	ctx.r[8].s64 = ctx.r[11].s64 + 21232;
	// 825FFFF0: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 825FFFF4: 388ABB0C  addi r4, r10, -0x44f4
	ctx.r[4].s64 = ctx.r[10].s64 + -17652;
	// 825FFFF8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825FFFFC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82600000: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82600004: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82600008: 386A230C  addi r3, r10, 0x230c
	ctx.r[3].s64 = ctx.r[10].s64 + 8972;
	// 8260000C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82600010: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82600014: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82600018: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8260001C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82600020: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82600024: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82600028: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8260002C: 4BE66DF5  bl 0x82466e20
	ctx.lr = 0x82600030;
	sub_82466E20(ctx, base);
	// 82600030: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82600034: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82600038: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8260003C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82600040(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82600040 size=112
    let mut pc: u32 = 0x82600040;
    'dispatch: loop {
        match pc {
            0x82600040 => {
    //   block [0x82600040..0x826000B0)
	// 82600040: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82600044: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82600048: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8260004C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82600050: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 82600054: 38AA221C  addi r5, r10, 0x221c
	ctx.r[5].s64 = ctx.r[10].s64 + 8732;
	// 82600058: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8260005C: 390B5308  addi r8, r11, 0x5308
	ctx.r[8].s64 = ctx.r[11].s64 + 21256;
	// 82600060: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 82600064: 388ABB2C  addi r4, r10, -0x44d4
	ctx.r[4].s64 = ctx.r[10].s64 + -17620;
	// 82600068: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8260006C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82600070: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82600074: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82600078: 386A233C  addi r3, r10, 0x233c
	ctx.r[3].s64 = ctx.r[10].s64 + 9020;
	// 8260007C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82600080: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82600084: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82600088: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8260008C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82600090: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82600094: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82600098: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8260009C: 4BE66D85  bl 0x82466e20
	ctx.lr = 0x826000A0;
	sub_82466E20(ctx, base);
	// 826000A0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826000A4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826000A8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826000AC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826000B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826000B0 size=112
    let mut pc: u32 = 0x826000B0;
    'dispatch: loop {
        match pc {
            0x826000B0 => {
    //   block [0x826000B0..0x82600120)
	// 826000B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826000B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826000B8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826000BC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 826000C0: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 826000C4: 38AA221C  addi r5, r10, 0x221c
	ctx.r[5].s64 = ctx.r[10].s64 + 8732;
	// 826000C8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826000CC: 390B5338  addi r8, r11, 0x5338
	ctx.r[8].s64 = ctx.r[11].s64 + 21304;
	// 826000D0: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826000D4: 388ABB50  addi r4, r10, -0x44b0
	ctx.r[4].s64 = ctx.r[10].s64 + -17584;
	// 826000D8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826000DC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 826000E0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826000E4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826000E8: 386A236C  addi r3, r10, 0x236c
	ctx.r[3].s64 = ctx.r[10].s64 + 9068;
	// 826000EC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826000F0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826000F4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826000F8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826000FC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82600100: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82600104: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82600108: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8260010C: 4BE66D15  bl 0x82466e20
	ctx.lr = 0x82600110;
	sub_82466E20(ctx, base);
	// 82600110: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82600114: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82600118: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8260011C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82600120(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82600120 size=112
    let mut pc: u32 = 0x82600120;
    'dispatch: loop {
        match pc {
            0x82600120 => {
    //   block [0x82600120..0x82600190)
	// 82600120: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82600124: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82600128: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8260012C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82600130: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 82600134: 38AA221C  addi r5, r10, 0x221c
	ctx.r[5].s64 = ctx.r[10].s64 + 8732;
	// 82600138: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8260013C: 390B5368  addi r8, r11, 0x5368
	ctx.r[8].s64 = ctx.r[11].s64 + 21352;
	// 82600140: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 82600144: 388ABB78  addi r4, r10, -0x4488
	ctx.r[4].s64 = ctx.r[10].s64 + -17544;
	// 82600148: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8260014C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82600150: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82600154: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82600158: 386A239C  addi r3, r10, 0x239c
	ctx.r[3].s64 = ctx.r[10].s64 + 9116;
	// 8260015C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82600160: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82600164: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82600168: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8260016C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82600170: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82600174: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82600178: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8260017C: 4BE66CA5  bl 0x82466e20
	ctx.lr = 0x82600180;
	sub_82466E20(ctx, base);
	// 82600180: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82600184: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82600188: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8260018C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82600190(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82600190 size=112
    let mut pc: u32 = 0x82600190;
    'dispatch: loop {
        match pc {
            0x82600190 => {
    //   block [0x82600190..0x82600200)
	// 82600190: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82600194: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82600198: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8260019C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 826001A0: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 826001A4: 38AA221C  addi r5, r10, 0x221c
	ctx.r[5].s64 = ctx.r[10].s64 + 8732;
	// 826001A8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826001AC: 390B5398  addi r8, r11, 0x5398
	ctx.r[8].s64 = ctx.r[11].s64 + 21400;
	// 826001B0: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826001B4: 388ABB9C  addi r4, r10, -0x4464
	ctx.r[4].s64 = ctx.r[10].s64 + -17508;
	// 826001B8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826001BC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 826001C0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826001C4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826001C8: 386A23CC  addi r3, r10, 0x23cc
	ctx.r[3].s64 = ctx.r[10].s64 + 9164;
	// 826001CC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826001D0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826001D4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826001D8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826001DC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826001E0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826001E4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826001E8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826001EC: 4BE66C35  bl 0x82466e20
	ctx.lr = 0x826001F0;
	sub_82466E20(ctx, base);
	// 826001F0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826001F4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826001F8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826001FC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82600200(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82600200 size=112
    let mut pc: u32 = 0x82600200;
    'dispatch: loop {
        match pc {
            0x82600200 => {
    //   block [0x82600200..0x82600270)
	// 82600200: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82600204: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82600208: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8260020C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82600210: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 82600214: 38AA221C  addi r5, r10, 0x221c
	ctx.r[5].s64 = ctx.r[10].s64 + 8732;
	// 82600218: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8260021C: 390B53B0  addi r8, r11, 0x53b0
	ctx.r[8].s64 = ctx.r[11].s64 + 21424;
	// 82600220: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82600224: 388ABBBC  addi r4, r10, -0x4444
	ctx.r[4].s64 = ctx.r[10].s64 + -17476;
	// 82600228: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8260022C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82600230: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82600234: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82600238: 386A23FC  addi r3, r10, 0x23fc
	ctx.r[3].s64 = ctx.r[10].s64 + 9212;
	// 8260023C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82600240: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82600244: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82600248: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8260024C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82600250: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82600254: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82600258: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8260025C: 4BE66BC5  bl 0x82466e20
	ctx.lr = 0x82600260;
	sub_82466E20(ctx, base);
	// 82600260: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82600264: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82600268: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8260026C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82600270(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82600270 size=112
    let mut pc: u32 = 0x82600270;
    'dispatch: loop {
        match pc {
            0x82600270 => {
    //   block [0x82600270..0x826002E0)
	// 82600270: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82600274: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82600278: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8260027C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82600280: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 82600284: 38AA221C  addi r5, r10, 0x221c
	ctx.r[5].s64 = ctx.r[10].s64 + 8732;
	// 82600288: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8260028C: 390B53C8  addi r8, r11, 0x53c8
	ctx.r[8].s64 = ctx.r[11].s64 + 21448;
	// 82600290: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 82600294: 388ABBD4  addi r4, r10, -0x442c
	ctx.r[4].s64 = ctx.r[10].s64 + -17452;
	// 82600298: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8260029C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 826002A0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826002A4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826002A8: 386A242C  addi r3, r10, 0x242c
	ctx.r[3].s64 = ctx.r[10].s64 + 9260;
	// 826002AC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826002B0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826002B4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826002B8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826002BC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826002C0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826002C4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826002C8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826002CC: 4BE66B55  bl 0x82466e20
	ctx.lr = 0x826002D0;
	sub_82466E20(ctx, base);
	// 826002D0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826002D4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826002D8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826002DC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826002E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826002E0 size=112
    let mut pc: u32 = 0x826002E0;
    'dispatch: loop {
        match pc {
            0x826002E0 => {
    //   block [0x826002E0..0x82600350)
	// 826002E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826002E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826002E8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826002EC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 826002F0: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 826002F4: 38AA221C  addi r5, r10, 0x221c
	ctx.r[5].s64 = ctx.r[10].s64 + 8732;
	// 826002F8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826002FC: 390B5410  addi r8, r11, 0x5410
	ctx.r[8].s64 = ctx.r[11].s64 + 21520;
	// 82600300: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 82600304: 388ABBF0  addi r4, r10, -0x4410
	ctx.r[4].s64 = ctx.r[10].s64 + -17424;
	// 82600308: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8260030C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82600310: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82600314: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82600318: 386A245C  addi r3, r10, 0x245c
	ctx.r[3].s64 = ctx.r[10].s64 + 9308;
	// 8260031C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82600320: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82600324: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82600328: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8260032C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82600330: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82600334: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82600338: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8260033C: 4BE66AE5  bl 0x82466e20
	ctx.lr = 0x82600340;
	sub_82466E20(ctx, base);
	// 82600340: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82600344: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82600348: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8260034C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82600350(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82600350 size=112
    let mut pc: u32 = 0x82600350;
    'dispatch: loop {
        match pc {
            0x82600350 => {
    //   block [0x82600350..0x826003C0)
	// 82600350: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82600354: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82600358: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8260035C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82600360: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 82600364: 38AA221C  addi r5, r10, 0x221c
	ctx.r[5].s64 = ctx.r[10].s64 + 8732;
	// 82600368: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8260036C: 390B5458  addi r8, r11, 0x5458
	ctx.r[8].s64 = ctx.r[11].s64 + 21592;
	// 82600370: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82600374: 388ABC0C  addi r4, r10, -0x43f4
	ctx.r[4].s64 = ctx.r[10].s64 + -17396;
	// 82600378: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8260037C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82600380: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82600384: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82600388: 386A248C  addi r3, r10, 0x248c
	ctx.r[3].s64 = ctx.r[10].s64 + 9356;
	// 8260038C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82600390: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82600394: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82600398: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8260039C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826003A0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826003A4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826003A8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826003AC: 4BE66A75  bl 0x82466e20
	ctx.lr = 0x826003B0;
	sub_82466E20(ctx, base);
	// 826003B0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826003B4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826003B8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826003BC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826003C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826003C0 size=112
    let mut pc: u32 = 0x826003C0;
    'dispatch: loop {
        match pc {
            0x826003C0 => {
    //   block [0x826003C0..0x82600430)
	// 826003C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826003C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826003C8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826003CC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 826003D0: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 826003D4: 38AA221C  addi r5, r10, 0x221c
	ctx.r[5].s64 = ctx.r[10].s64 + 8732;
	// 826003D8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826003DC: 390B5470  addi r8, r11, 0x5470
	ctx.r[8].s64 = ctx.r[11].s64 + 21616;
	// 826003E0: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826003E4: 388ABC24  addi r4, r10, -0x43dc
	ctx.r[4].s64 = ctx.r[10].s64 + -17372;
	// 826003E8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826003EC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 826003F0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826003F4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826003F8: 386A24BC  addi r3, r10, 0x24bc
	ctx.r[3].s64 = ctx.r[10].s64 + 9404;
	// 826003FC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82600400: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82600404: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82600408: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8260040C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82600410: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82600414: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82600418: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8260041C: 4BE66A05  bl 0x82466e20
	ctx.lr = 0x82600420;
	sub_82466E20(ctx, base);
	// 82600420: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82600424: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82600428: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8260042C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82600430(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82600430 size=116
    let mut pc: u32 = 0x82600430;
    'dispatch: loop {
        match pc {
            0x82600430 => {
    //   block [0x82600430..0x826004A4)
	// 82600430: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82600434: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82600438: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8260043C: 3D408274  lis r10, -0x7d8c
	ctx.r[10].s64 = -2106327040;
	// 82600440: 38E00005  li r7, 5
	ctx.r[7].s64 = 5;
	// 82600444: 390A54A0  addi r8, r10, 0x54a0
	ctx.r[8].s64 = ctx.r[10].s64 + 21664;
	// 82600448: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 8260044C: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82600450: 38AA221C  addi r5, r10, 0x221c
	ctx.r[5].s64 = ctx.r[10].s64 + 8732;
	// 82600454: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82600458: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8260045C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82600460: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82600464: 388ABC3C  addi r4, r10, -0x43c4
	ctx.r[4].s64 = ctx.r[10].s64 + -17348;
	// 82600468: 396B9358  addi r11, r11, -0x6ca8
	ctx.r[11].s64 = ctx.r[11].s64 + -27816;
	// 8260046C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82600470: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82600474: 386A24EC  addi r3, r10, 0x24ec
	ctx.r[3].s64 = ctx.r[10].s64 + 9452;
	// 82600478: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 8260047C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82600480: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 82600484: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82600488: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8260048C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82600490: 4BE66991  bl 0x82466e20
	ctx.lr = 0x82600494;
	sub_82466E20(ctx, base);
	// 82600494: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82600498: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8260049C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826004A0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826004A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826004A8 size=116
    let mut pc: u32 = 0x826004A8;
    'dispatch: loop {
        match pc {
            0x826004A8 => {
    //   block [0x826004A8..0x8260051C)
	// 826004A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826004AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826004B0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826004B4: 3D408274  lis r10, -0x7d8c
	ctx.r[10].s64 = -2106327040;
	// 826004B8: 38E00006  li r7, 6
	ctx.r[7].s64 = 6;
	// 826004BC: 390A5518  addi r8, r10, 0x5518
	ctx.r[8].s64 = ctx.r[10].s64 + 21784;
	// 826004C0: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 826004C4: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 826004C8: 38AA221C  addi r5, r10, 0x221c
	ctx.r[5].s64 = ctx.r[10].s64 + 8732;
	// 826004CC: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826004D0: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826004D4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826004D8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826004DC: 388ABC58  addi r4, r10, -0x43a8
	ctx.r[4].s64 = ctx.r[10].s64 + -17320;
	// 826004E0: 396B9370  addi r11, r11, -0x6c90
	ctx.r[11].s64 = ctx.r[11].s64 + -27792;
	// 826004E4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 826004E8: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826004EC: 386A251C  addi r3, r10, 0x251c
	ctx.r[3].s64 = ctx.r[10].s64 + 9500;
	// 826004F0: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 826004F4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826004F8: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 826004FC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82600500: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82600504: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82600508: 4BE66919  bl 0x82466e20
	ctx.lr = 0x8260050C;
	sub_82466E20(ctx, base);
	// 8260050C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82600510: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82600514: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82600518: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82600520(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82600520 size=24
    let mut pc: u32 = 0x82600520;
    'dispatch: loop {
        match pc {
            0x82600520 => {
    //   block [0x82600520..0x82600538)
	// 82600520: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 82600524: 3D408275  lis r10, -0x7d8b
	ctx.r[10].s64 = -2106261504;
	// 82600528: 394AB100  addi r10, r10, -0x4f00
	ctx.r[10].s64 = ctx.r[10].s64 + -20224;
	// 8260052C: 816B5244  lwz r11, 0x5244(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(21060 as u32) ) } as u64;
	// 82600530: 916A0050  stw r11, 0x50(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 82600534: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82600538(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82600538 size=116
    let mut pc: u32 = 0x82600538;
    'dispatch: loop {
        match pc {
            0x82600538 => {
    //   block [0x82600538..0x826005AC)
	// 82600538: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260053C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82600540: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82600544: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82600548: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 8260054C: 392B939C  addi r9, r11, -0x6c64
	ctx.r[9].s64 = ctx.r[11].s64 + -27748;
	// 82600550: 38AA221C  addi r5, r10, 0x221c
	ctx.r[5].s64 = ctx.r[10].s64 + 8732;
	// 82600554: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82600558: 38E90014  addi r7, r9, 0x14
	ctx.r[7].s64 = ctx.r[9].s64 + 20;
	// 8260055C: 38C00008  li r6, 8
	ctx.r[6].s64 = 8;
	// 82600560: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 82600564: 388ABC8C  addi r4, r10, -0x4374
	ctx.r[4].s64 = ctx.r[10].s64 + -17268;
	// 82600568: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8260056C: 396BB100  addi r11, r11, -0x4f00
	ctx.r[11].s64 = ctx.r[11].s64 + -20224;
	// 82600570: 90E10064  stw r7, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[7].u32 ) };
	// 82600574: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82600578: 90C1005C  stw r6, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[6].u32 ) };
	// 8260057C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82600580: 386A254C  addi r3, r10, 0x254c
	ctx.r[3].s64 = ctx.r[10].s64 + 9548;
	// 82600584: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82600588: 91010074  stw r8, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[8].u32 ) };
	// 8260058C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82600590: 9101006C  stw r8, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[8].u32 ) };
	// 82600594: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82600598: 4BE66889  bl 0x82466e20
	ctx.lr = 0x8260059C;
	sub_82466E20(ctx, base);
	// 8260059C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826005A0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826005A4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826005A8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826005B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826005B0 size=112
    let mut pc: u32 = 0x826005B0;
    'dispatch: loop {
        match pc {
            0x826005B0 => {
    //   block [0x826005B0..0x82600620)
	// 826005B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826005B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826005B8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826005BC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 826005C0: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 826005C4: 38AA221C  addi r5, r10, 0x221c
	ctx.r[5].s64 = ctx.r[10].s64 + 8732;
	// 826005C8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826005CC: 390B55A8  addi r8, r11, 0x55a8
	ctx.r[8].s64 = ctx.r[11].s64 + 21928;
	// 826005D0: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 826005D4: 388ABCA8  addi r4, r10, -0x4358
	ctx.r[4].s64 = ctx.r[10].s64 + -17240;
	// 826005D8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826005DC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 826005E0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826005E4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826005E8: 386A257C  addi r3, r10, 0x257c
	ctx.r[3].s64 = ctx.r[10].s64 + 9596;
	// 826005EC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826005F0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826005F4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826005F8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826005FC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82600600: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82600604: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82600608: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8260060C: 4BE66815  bl 0x82466e20
	ctx.lr = 0x82600610;
	sub_82466E20(ctx, base);
	// 82600610: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82600614: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82600618: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8260061C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82600620(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82600620 size=112
    let mut pc: u32 = 0x82600620;
    'dispatch: loop {
        match pc {
            0x82600620 => {
    //   block [0x82600620..0x82600690)
	// 82600620: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82600624: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82600628: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8260062C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82600630: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 82600634: 38AA221C  addi r5, r10, 0x221c
	ctx.r[5].s64 = ctx.r[10].s64 + 8732;
	// 82600638: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8260063C: 390B5608  addi r8, r11, 0x5608
	ctx.r[8].s64 = ctx.r[11].s64 + 22024;
	// 82600640: 39200007  li r9, 7
	ctx.r[9].s64 = 7;
	// 82600644: 388ABCC8  addi r4, r10, -0x4338
	ctx.r[4].s64 = ctx.r[10].s64 + -17208;
	// 82600648: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8260064C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82600650: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82600654: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82600658: 386A25AC  addi r3, r10, 0x25ac
	ctx.r[3].s64 = ctx.r[10].s64 + 9644;
	// 8260065C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82600660: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82600664: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82600668: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8260066C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82600670: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82600674: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82600678: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8260067C: 4BE667A5  bl 0x82466e20
	ctx.lr = 0x82600680;
	sub_82466E20(ctx, base);
	// 82600680: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82600684: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82600688: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8260068C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82600690(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82600690 size=112
    let mut pc: u32 = 0x82600690;
    'dispatch: loop {
        match pc {
            0x82600690 => {
    //   block [0x82600690..0x82600700)
	// 82600690: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82600694: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82600698: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8260069C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 826006A0: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 826006A4: 38AA221C  addi r5, r10, 0x221c
	ctx.r[5].s64 = ctx.r[10].s64 + 8732;
	// 826006A8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826006AC: 390B56B0  addi r8, r11, 0x56b0
	ctx.r[8].s64 = ctx.r[11].s64 + 22192;
	// 826006B0: 39200005  li r9, 5
	ctx.r[9].s64 = 5;
	// 826006B4: 388ABCE4  addi r4, r10, -0x431c
	ctx.r[4].s64 = ctx.r[10].s64 + -17180;
	// 826006B8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826006BC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 826006C0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826006C4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826006C8: 386A25DC  addi r3, r10, 0x25dc
	ctx.r[3].s64 = ctx.r[10].s64 + 9692;
	// 826006CC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826006D0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826006D4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826006D8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826006DC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826006E0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826006E4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826006E8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826006EC: 4BE66735  bl 0x82466e20
	ctx.lr = 0x826006F0;
	sub_82466E20(ctx, base);
	// 826006F0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826006F4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826006F8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826006FC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82600700(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82600700 size=112
    let mut pc: u32 = 0x82600700;
    'dispatch: loop {
        match pc {
            0x82600700 => {
    //   block [0x82600700..0x82600770)
	// 82600700: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82600704: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82600708: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8260070C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82600710: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 82600714: 38AA221C  addi r5, r10, 0x221c
	ctx.r[5].s64 = ctx.r[10].s64 + 8732;
	// 82600718: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8260071C: 390B5728  addi r8, r11, 0x5728
	ctx.r[8].s64 = ctx.r[11].s64 + 22312;
	// 82600720: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 82600724: 388ABD04  addi r4, r10, -0x42fc
	ctx.r[4].s64 = ctx.r[10].s64 + -17148;
	// 82600728: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8260072C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82600730: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82600734: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82600738: 386A260C  addi r3, r10, 0x260c
	ctx.r[3].s64 = ctx.r[10].s64 + 9740;
	// 8260073C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82600740: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82600744: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82600748: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8260074C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82600750: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82600754: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82600758: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8260075C: 4BE666C5  bl 0x82466e20
	ctx.lr = 0x82600760;
	sub_82466E20(ctx, base);
	// 82600760: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82600764: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82600768: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8260076C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82600770(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82600770 size=112
    let mut pc: u32 = 0x82600770;
    'dispatch: loop {
        match pc {
            0x82600770 => {
    //   block [0x82600770..0x826007E0)
	// 82600770: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82600774: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82600778: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8260077C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82600780: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 82600784: 38AA221C  addi r5, r10, 0x221c
	ctx.r[5].s64 = ctx.r[10].s64 + 8732;
	// 82600788: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8260078C: 390B5770  addi r8, r11, 0x5770
	ctx.r[8].s64 = ctx.r[11].s64 + 22384;
	// 82600790: 39200006  li r9, 6
	ctx.r[9].s64 = 6;
	// 82600794: 388ABD24  addi r4, r10, -0x42dc
	ctx.r[4].s64 = ctx.r[10].s64 + -17116;
	// 82600798: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8260079C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 826007A0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826007A4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826007A8: 386A263C  addi r3, r10, 0x263c
	ctx.r[3].s64 = ctx.r[10].s64 + 9788;
	// 826007AC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826007B0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826007B4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826007B8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826007BC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826007C0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826007C4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826007C8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826007CC: 4BE66655  bl 0x82466e20
	ctx.lr = 0x826007D0;
	sub_82466E20(ctx, base);
	// 826007D0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826007D4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826007D8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826007DC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826007E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826007E0 size=112
    let mut pc: u32 = 0x826007E0;
    'dispatch: loop {
        match pc {
            0x826007E0 => {
    //   block [0x826007E0..0x82600850)
	// 826007E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826007E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826007E8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826007EC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 826007F0: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 826007F4: 38AA221C  addi r5, r10, 0x221c
	ctx.r[5].s64 = ctx.r[10].s64 + 8732;
	// 826007F8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826007FC: 390B5800  addi r8, r11, 0x5800
	ctx.r[8].s64 = ctx.r[11].s64 + 22528;
	// 82600800: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 82600804: 388ABD40  addi r4, r10, -0x42c0
	ctx.r[4].s64 = ctx.r[10].s64 + -17088;
	// 82600808: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8260080C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82600810: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82600814: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82600818: 386A266C  addi r3, r10, 0x266c
	ctx.r[3].s64 = ctx.r[10].s64 + 9836;
	// 8260081C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82600820: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82600824: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82600828: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8260082C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82600830: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82600834: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82600838: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8260083C: 4BE665E5  bl 0x82466e20
	ctx.lr = 0x82600840;
	sub_82466E20(ctx, base);
	// 82600840: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82600844: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82600848: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8260084C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82600850(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82600850 size=112
    let mut pc: u32 = 0x82600850;
    'dispatch: loop {
        match pc {
            0x82600850 => {
    //   block [0x82600850..0x826008C0)
	// 82600850: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82600854: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82600858: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8260085C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82600860: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 82600864: 38AA221C  addi r5, r10, 0x221c
	ctx.r[5].s64 = ctx.r[10].s64 + 8732;
	// 82600868: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8260086C: 390B5860  addi r8, r11, 0x5860
	ctx.r[8].s64 = ctx.r[11].s64 + 22624;
	// 82600870: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 82600874: 388ABD58  addi r4, r10, -0x42a8
	ctx.r[4].s64 = ctx.r[10].s64 + -17064;
	// 82600878: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8260087C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82600880: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82600884: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82600888: 386A269C  addi r3, r10, 0x269c
	ctx.r[3].s64 = ctx.r[10].s64 + 9884;
	// 8260088C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82600890: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82600894: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82600898: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8260089C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826008A0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826008A4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826008A8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826008AC: 4BE66575  bl 0x82466e20
	ctx.lr = 0x826008B0;
	sub_82466E20(ctx, base);
	// 826008B0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826008B4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826008B8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826008BC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826008C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826008C0 size=112
    let mut pc: u32 = 0x826008C0;
    'dispatch: loop {
        match pc {
            0x826008C0 => {
    //   block [0x826008C0..0x82600930)
	// 826008C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826008C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826008C8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826008CC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 826008D0: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 826008D4: 38AA269C  addi r5, r10, 0x269c
	ctx.r[5].s64 = ctx.r[10].s64 + 9884;
	// 826008D8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826008DC: 390B58C0  addi r8, r11, 0x58c0
	ctx.r[8].s64 = ctx.r[11].s64 + 22720;
	// 826008E0: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826008E4: 388ABD74  addi r4, r10, -0x428c
	ctx.r[4].s64 = ctx.r[10].s64 + -17036;
	// 826008E8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826008EC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 826008F0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826008F4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826008F8: 386A26CC  addi r3, r10, 0x26cc
	ctx.r[3].s64 = ctx.r[10].s64 + 9932;
	// 826008FC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82600900: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82600904: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82600908: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8260090C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82600910: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82600914: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82600918: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8260091C: 4BE66505  bl 0x82466e20
	ctx.lr = 0x82600920;
	sub_82466E20(ctx, base);
	// 82600920: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82600924: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82600928: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8260092C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82600930(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82600930 size=112
    let mut pc: u32 = 0x82600930;
    'dispatch: loop {
        match pc {
            0x82600930 => {
    //   block [0x82600930..0x826009A0)
	// 82600930: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82600934: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82600938: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8260093C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82600940: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 82600944: 38AA269C  addi r5, r10, 0x269c
	ctx.r[5].s64 = ctx.r[10].s64 + 9884;
	// 82600948: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8260094C: 390B58F0  addi r8, r11, 0x58f0
	ctx.r[8].s64 = ctx.r[11].s64 + 22768;
	// 82600950: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 82600954: 388ABD9C  addi r4, r10, -0x4264
	ctx.r[4].s64 = ctx.r[10].s64 + -16996;
	// 82600958: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8260095C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82600960: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82600964: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82600968: 386A26FC  addi r3, r10, 0x26fc
	ctx.r[3].s64 = ctx.r[10].s64 + 9980;
	// 8260096C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82600970: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82600974: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82600978: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8260097C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82600980: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82600984: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82600988: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8260098C: 4BE66495  bl 0x82466e20
	ctx.lr = 0x82600990;
	sub_82466E20(ctx, base);
	// 82600990: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82600994: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82600998: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8260099C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826009A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826009A0 size=100
    let mut pc: u32 = 0x826009A0;
    'dispatch: loop {
        match pc {
            0x826009A0 => {
    //   block [0x826009A0..0x82600A04)
	// 826009A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826009A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826009A8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826009AC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 826009B0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826009B4: 38AA269C  addi r5, r10, 0x269c
	ctx.r[5].s64 = ctx.r[10].s64 + 9884;
	// 826009B8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826009BC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826009C0: 388ABDC4  addi r4, r10, -0x423c
	ctx.r[4].s64 = ctx.r[10].s64 + -16956;
	// 826009C4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 826009C8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826009CC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826009D0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826009D4: 386A272C  addi r3, r10, 0x272c
	ctx.r[3].s64 = ctx.r[10].s64 + 10028;
	// 826009D8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826009DC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826009E0: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826009E4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826009E8: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826009EC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826009F0: 4BE66431  bl 0x82466e20
	ctx.lr = 0x826009F4;
	sub_82466E20(ctx, base);
	// 826009F4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826009F8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826009FC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82600A00: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82600A08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82600A08 size=112
    let mut pc: u32 = 0x82600A08;
    'dispatch: loop {
        match pc {
            0x82600A08 => {
    //   block [0x82600A08..0x82600A78)
	// 82600A08: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82600A0C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82600A10: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82600A14: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82600A18: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 82600A1C: 38AA269C  addi r5, r10, 0x269c
	ctx.r[5].s64 = ctx.r[10].s64 + 9884;
	// 82600A20: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82600A24: 390B5938  addi r8, r11, 0x5938
	ctx.r[8].s64 = ctx.r[11].s64 + 22840;
	// 82600A28: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82600A2C: 388ABDEC  addi r4, r10, -0x4214
	ctx.r[4].s64 = ctx.r[10].s64 + -16916;
	// 82600A30: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82600A34: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82600A38: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82600A3C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82600A40: 386A275C  addi r3, r10, 0x275c
	ctx.r[3].s64 = ctx.r[10].s64 + 10076;
	// 82600A44: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82600A48: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82600A4C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82600A50: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82600A54: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82600A58: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82600A5C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82600A60: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82600A64: 4BE663BD  bl 0x82466e20
	ctx.lr = 0x82600A68;
	sub_82466E20(ctx, base);
	// 82600A68: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82600A6C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82600A70: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82600A74: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82600A78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82600A78 size=108
    let mut pc: u32 = 0x82600A78;
    'dispatch: loop {
        match pc {
            0x82600A78 => {
    //   block [0x82600A78..0x82600AE4)
	// 82600A78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82600A7C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82600A80: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82600A84: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 82600A88: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82600A8C: 38EB5950  addi r7, r11, 0x5950
	ctx.r[7].s64 = ctx.r[11].s64 + 22864;
	// 82600A90: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 82600A94: 388ABE14  addi r4, r10, -0x41ec
	ctx.r[4].s64 = ctx.r[10].s64 + -16876;
	// 82600A98: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82600A9C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82600AA0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82600AA4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82600AA8: 386A278C  addi r3, r10, 0x278c
	ctx.r[3].s64 = ctx.r[10].s64 + 10124;
	// 82600AAC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82600AB0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82600AB4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82600AB8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82600ABC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82600AC0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82600AC4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82600AC8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82600ACC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82600AD0: 4BE66351  bl 0x82466e20
	ctx.lr = 0x82600AD4;
	sub_82466E20(ctx, base);
	// 82600AD4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82600AD8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82600ADC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82600AE0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82600AE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82600AE8 size=112
    let mut pc: u32 = 0x82600AE8;
    'dispatch: loop {
        match pc {
            0x82600AE8 => {
    //   block [0x82600AE8..0x82600B58)
	// 82600AE8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82600AEC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82600AF0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82600AF4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82600AF8: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 82600AFC: 38AA09BC  addi r5, r10, 0x9bc
	ctx.r[5].s64 = ctx.r[10].s64 + 2492;
	// 82600B00: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82600B04: 390B5998  addi r8, r11, 0x5998
	ctx.r[8].s64 = ctx.r[11].s64 + 22936;
	// 82600B08: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 82600B0C: 388ABE38  addi r4, r10, -0x41c8
	ctx.r[4].s64 = ctx.r[10].s64 + -16840;
	// 82600B10: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82600B14: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82600B18: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82600B1C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82600B20: 386A27BC  addi r3, r10, 0x27bc
	ctx.r[3].s64 = ctx.r[10].s64 + 10172;
	// 82600B24: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82600B28: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82600B2C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82600B30: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82600B34: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82600B38: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82600B3C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82600B40: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82600B44: 4BE662DD  bl 0x82466e20
	ctx.lr = 0x82600B48;
	sub_82466E20(ctx, base);
	// 82600B48: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82600B4C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82600B50: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82600B54: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82600B58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82600B58 size=112
    let mut pc: u32 = 0x82600B58;
    'dispatch: loop {
        match pc {
            0x82600B58 => {
    //   block [0x82600B58..0x82600BC8)
	// 82600B58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82600B5C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82600B60: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82600B64: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82600B68: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 82600B6C: 38AA27BC  addi r5, r10, 0x27bc
	ctx.r[5].s64 = ctx.r[10].s64 + 10172;
	// 82600B70: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82600B74: 390B59F8  addi r8, r11, 0x59f8
	ctx.r[8].s64 = ctx.r[11].s64 + 23032;
	// 82600B78: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82600B7C: 388ABE44  addi r4, r10, -0x41bc
	ctx.r[4].s64 = ctx.r[10].s64 + -16828;
	// 82600B80: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82600B84: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82600B88: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82600B8C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82600B90: 386A27EC  addi r3, r10, 0x27ec
	ctx.r[3].s64 = ctx.r[10].s64 + 10220;
	// 82600B94: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82600B98: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82600B9C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82600BA0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82600BA4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82600BA8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82600BAC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82600BB0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82600BB4: 4BE6626D  bl 0x82466e20
	ctx.lr = 0x82600BB8;
	sub_82466E20(ctx, base);
	// 82600BB8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82600BBC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82600BC0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82600BC4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82600BC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82600BC8 size=112
    let mut pc: u32 = 0x82600BC8;
    'dispatch: loop {
        match pc {
            0x82600BC8 => {
    //   block [0x82600BC8..0x82600C38)
	// 82600BC8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82600BCC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82600BD0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82600BD4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82600BD8: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 82600BDC: 38AA27BC  addi r5, r10, 0x27bc
	ctx.r[5].s64 = ctx.r[10].s64 + 10172;
	// 82600BE0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82600BE4: 390B5A10  addi r8, r11, 0x5a10
	ctx.r[8].s64 = ctx.r[11].s64 + 23056;
	// 82600BE8: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 82600BEC: 388ABE54  addi r4, r10, -0x41ac
	ctx.r[4].s64 = ctx.r[10].s64 + -16812;
	// 82600BF0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82600BF4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82600BF8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82600BFC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82600C00: 386A281C  addi r3, r10, 0x281c
	ctx.r[3].s64 = ctx.r[10].s64 + 10268;
	// 82600C04: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82600C08: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82600C0C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82600C10: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82600C14: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82600C18: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82600C1C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82600C20: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82600C24: 4BE661FD  bl 0x82466e20
	ctx.lr = 0x82600C28;
	sub_82466E20(ctx, base);
	// 82600C28: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82600C2C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82600C30: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82600C34: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82600C38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82600C38 size=112
    let mut pc: u32 = 0x82600C38;
    'dispatch: loop {
        match pc {
            0x82600C38 => {
    //   block [0x82600C38..0x82600CA8)
	// 82600C38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82600C3C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82600C40: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82600C44: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82600C48: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 82600C4C: 38AA27BC  addi r5, r10, 0x27bc
	ctx.r[5].s64 = ctx.r[10].s64 + 10172;
	// 82600C50: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82600C54: 390B5A40  addi r8, r11, 0x5a40
	ctx.r[8].s64 = ctx.r[11].s64 + 23104;
	// 82600C58: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82600C5C: 388ABE64  addi r4, r10, -0x419c
	ctx.r[4].s64 = ctx.r[10].s64 + -16796;
	// 82600C60: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82600C64: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82600C68: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82600C6C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82600C70: 386A284C  addi r3, r10, 0x284c
	ctx.r[3].s64 = ctx.r[10].s64 + 10316;
	// 82600C74: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82600C78: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82600C7C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82600C80: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82600C84: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82600C88: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82600C8C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82600C90: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82600C94: 4BE6618D  bl 0x82466e20
	ctx.lr = 0x82600C98;
	sub_82466E20(ctx, base);
	// 82600C98: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82600C9C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82600CA0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82600CA4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82600CA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82600CA8 size=24
    let mut pc: u32 = 0x82600CA8;
    'dispatch: loop {
        match pc {
            0x82600CA8 => {
    //   block [0x82600CA8..0x82600CC0)
	// 82600CA8: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 82600CAC: 3D408275  lis r10, -0x7d8b
	ctx.r[10].s64 = -2106261504;
	// 82600CB0: 394AB1C0  addi r10, r10, -0x4e40
	ctx.r[10].s64 = ctx.r[10].s64 + -20032;
	// 82600CB4: 816B5A58  lwz r11, 0x5a58(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(23128 as u32) ) } as u64;
	// 82600CB8: 916A0008  stw r11, 8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82600CBC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82600CC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82600CC0 size=112
    let mut pc: u32 = 0x82600CC0;
    'dispatch: loop {
        match pc {
            0x82600CC0 => {
    //   block [0x82600CC0..0x82600D30)
	// 82600CC0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82600CC4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82600CC8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82600CCC: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82600CD0: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 82600CD4: 392A9400  addi r9, r10, -0x6c00
	ctx.r[9].s64 = ctx.r[10].s64 + -27648;
	// 82600CD8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82600CDC: 390BB1C0  addi r8, r11, -0x4e40
	ctx.r[8].s64 = ctx.r[11].s64 + -20032;
	// 82600CE0: 38E00003  li r7, 3
	ctx.r[7].s64 = 3;
	// 82600CE4: 388ABE74  addi r4, r10, -0x418c
	ctx.r[4].s64 = ctx.r[10].s64 + -16780;
	// 82600CE8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82600CEC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82600CF0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82600CF4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82600CF8: 386A287C  addi r3, r10, 0x287c
	ctx.r[3].s64 = ctx.r[10].s64 + 10364;
	// 82600CFC: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82600D00: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82600D04: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82600D08: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82600D0C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82600D10: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82600D14: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82600D18: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82600D1C: 4BE66105  bl 0x82466e20
	ctx.lr = 0x82600D20;
	sub_82466E20(ctx, base);
	// 82600D20: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82600D24: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82600D28: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82600D2C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82600D30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82600D30 size=108
    let mut pc: u32 = 0x82600D30;
    'dispatch: loop {
        match pc {
            0x82600D30 => {
    //   block [0x82600D30..0x82600D9C)
	// 82600D30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82600D34: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82600D38: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82600D3C: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 82600D40: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82600D44: 38EB5A5C  addi r7, r11, 0x5a5c
	ctx.r[7].s64 = ctx.r[11].s64 + 23132;
	// 82600D48: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 82600D4C: 388ABE80  addi r4, r10, -0x4180
	ctx.r[4].s64 = ctx.r[10].s64 + -16768;
	// 82600D50: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82600D54: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82600D58: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82600D5C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82600D60: 386A28AC  addi r3, r10, 0x28ac
	ctx.r[3].s64 = ctx.r[10].s64 + 10412;
	// 82600D64: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82600D68: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82600D6C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82600D70: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82600D74: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82600D78: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82600D7C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82600D80: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82600D84: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82600D88: 4BE66099  bl 0x82466e20
	ctx.lr = 0x82600D8C;
	sub_82466E20(ctx, base);
	// 82600D8C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82600D90: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82600D94: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82600D98: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82600DA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82600DA0 size=108
    let mut pc: u32 = 0x82600DA0;
    'dispatch: loop {
        match pc {
            0x82600DA0 => {
    //   block [0x82600DA0..0x82600E0C)
	// 82600DA0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82600DA4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82600DA8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82600DAC: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 82600DB0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82600DB4: 38EB5A78  addi r7, r11, 0x5a78
	ctx.r[7].s64 = ctx.r[11].s64 + 23160;
	// 82600DB8: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 82600DBC: 388ABE94  addi r4, r10, -0x416c
	ctx.r[4].s64 = ctx.r[10].s64 + -16748;
	// 82600DC0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82600DC4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82600DC8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82600DCC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82600DD0: 386A28DC  addi r3, r10, 0x28dc
	ctx.r[3].s64 = ctx.r[10].s64 + 10460;
	// 82600DD4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82600DD8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82600DDC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82600DE0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82600DE4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82600DE8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82600DEC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82600DF0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82600DF4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82600DF8: 4BE66029  bl 0x82466e20
	ctx.lr = 0x82600DFC;
	sub_82466E20(ctx, base);
	// 82600DFC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82600E00: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82600E04: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82600E08: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82600E10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82600E10 size=116
    let mut pc: u32 = 0x82600E10;
    'dispatch: loop {
        match pc {
            0x82600E10 => {
    //   block [0x82600E10..0x82600E84)
	// 82600E10: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82600E14: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82600E18: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82600E1C: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 82600E20: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82600E24: 390B5AC0  addi r8, r11, 0x5ac0
	ctx.r[8].s64 = ctx.r[11].s64 + 23232;
	// 82600E28: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82600E2C: 392A94C0  addi r9, r10, -0x6b40
	ctx.r[9].s64 = ctx.r[10].s64 + -27456;
	// 82600E30: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82600E34: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
	// 82600E38: 38AA09BC  addi r5, r10, 0x9bc
	ctx.r[5].s64 = ctx.r[10].s64 + 2492;
	// 82600E3C: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82600E40: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82600E44: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82600E48: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82600E4C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82600E50: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82600E54: 3D608293  lis r11, -0x7d6d
	ctx.r[11].s64 = -2104295424;
	// 82600E58: 388ABEA0  addi r4, r10, -0x4160
	ctx.r[4].s64 = ctx.r[10].s64 + -16736;
	// 82600E5C: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82600E60: 386B290C  addi r3, r11, 0x290c
	ctx.r[3].s64 = ctx.r[11].s64 + 10508;
	// 82600E64: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82600E68: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82600E6C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82600E70: 4BE65FB1  bl 0x82466e20
	ctx.lr = 0x82600E74;
	sub_82466E20(ctx, base);
	// 82600E74: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82600E78: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82600E7C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82600E80: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82600E88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82600E88 size=24
    let mut pc: u32 = 0x82600E88;
    'dispatch: loop {
        match pc {
            0x82600E88 => {
    //   block [0x82600E88..0x82600EA0)
	// 82600E88: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 82600E8C: 3D408275  lis r10, -0x7d8b
	ctx.r[10].s64 = -2106261504;
	// 82600E90: 394AB208  addi r10, r10, -0x4df8
	ctx.r[10].s64 = ctx.r[10].s64 + -19960;
	// 82600E94: 816B5AD8  lwz r11, 0x5ad8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(23256 as u32) ) } as u64;
	// 82600E98: 916A0068  stw r11, 0x68(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(104 as u32), ctx.r[11].u32 ) };
	// 82600E9C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82600EA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82600EA0 size=116
    let mut pc: u32 = 0x82600EA0;
    'dispatch: loop {
        match pc {
            0x82600EA0 => {
    //   block [0x82600EA0..0x82600F14)
	// 82600EA0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82600EA4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82600EA8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82600EAC: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 82600EB0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82600EB4: 390BB208  addi r8, r11, -0x4df8
	ctx.r[8].s64 = ctx.r[11].s64 + -19960;
	// 82600EB8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82600EBC: 392A9530  addi r9, r10, -0x6ad0
	ctx.r[9].s64 = ctx.r[10].s64 + -27344;
	// 82600EC0: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82600EC4: 38E00009  li r7, 9
	ctx.r[7].s64 = 9;
	// 82600EC8: 38AA09BC  addi r5, r10, 0x9bc
	ctx.r[5].s64 = ctx.r[10].s64 + 2492;
	// 82600ECC: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82600ED0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82600ED4: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82600ED8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82600EDC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82600EE0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82600EE4: 3D608293  lis r11, -0x7d6d
	ctx.r[11].s64 = -2104295424;
	// 82600EE8: 388ABEB4  addi r4, r10, -0x414c
	ctx.r[4].s64 = ctx.r[10].s64 + -16716;
	// 82600EEC: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82600EF0: 386B293C  addi r3, r11, 0x293c
	ctx.r[3].s64 = ctx.r[11].s64 + 10556;
	// 82600EF4: 39400004  li r10, 4
	ctx.r[10].s64 = 4;
	// 82600EF8: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82600EFC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82600F00: 4BE65F21  bl 0x82466e20
	ctx.lr = 0x82600F04;
	sub_82466E20(ctx, base);
	// 82600F04: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82600F08: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82600F0C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82600F10: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82600F18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82600F18 size=108
    let mut pc: u32 = 0x82600F18;
    'dispatch: loop {
        match pc {
            0x82600F18 => {
    //   block [0x82600F18..0x82600F84)
	// 82600F18: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82600F1C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82600F20: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82600F24: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 82600F28: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82600F2C: 38EB5AE8  addi r7, r11, 0x5ae8
	ctx.r[7].s64 = ctx.r[11].s64 + 23272;
	// 82600F30: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 82600F34: 388ABECC  addi r4, r10, -0x4134
	ctx.r[4].s64 = ctx.r[10].s64 + -16692;
	// 82600F38: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82600F3C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82600F40: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82600F44: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82600F48: 386A296C  addi r3, r10, 0x296c
	ctx.r[3].s64 = ctx.r[10].s64 + 10604;
	// 82600F4C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82600F50: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82600F54: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82600F58: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82600F5C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82600F60: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82600F64: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82600F68: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82600F6C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82600F70: 4BE65EB1  bl 0x82466e20
	ctx.lr = 0x82600F74;
	sub_82466E20(ctx, base);
	// 82600F74: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82600F78: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82600F7C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82600F80: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82600F88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82600F88 size=112
    let mut pc: u32 = 0x82600F88;
    'dispatch: loop {
        match pc {
            0x82600F88 => {
    //   block [0x82600F88..0x82600FF8)
	// 82600F88: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82600F8C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82600F90: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82600F94: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82600F98: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 82600F9C: 38AA290C  addi r5, r10, 0x290c
	ctx.r[5].s64 = ctx.r[10].s64 + 10508;
	// 82600FA0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82600FA4: 390B5B18  addi r8, r11, 0x5b18
	ctx.r[8].s64 = ctx.r[11].s64 + 23320;
	// 82600FA8: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82600FAC: 388ABEF0  addi r4, r10, -0x4110
	ctx.r[4].s64 = ctx.r[10].s64 + -16656;
	// 82600FB0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82600FB4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82600FB8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82600FBC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82600FC0: 386A299C  addi r3, r10, 0x299c
	ctx.r[3].s64 = ctx.r[10].s64 + 10652;
	// 82600FC4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82600FC8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82600FCC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82600FD0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82600FD4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82600FD8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82600FDC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82600FE0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82600FE4: 4BE65E3D  bl 0x82466e20
	ctx.lr = 0x82600FE8;
	sub_82466E20(ctx, base);
	// 82600FE8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82600FEC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82600FF0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82600FF4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82600FF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82600FF8 size=112
    let mut pc: u32 = 0x82600FF8;
    'dispatch: loop {
        match pc {
            0x82600FF8 => {
    //   block [0x82600FF8..0x82601068)
	// 82600FF8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82600FFC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82601000: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82601004: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82601008: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 8260100C: 392A9588  addi r9, r10, -0x6a78
	ctx.r[9].s64 = ctx.r[10].s64 + -27256;
	// 82601010: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82601014: 390B5B38  addi r8, r11, 0x5b38
	ctx.r[8].s64 = ctx.r[11].s64 + 23352;
	// 82601018: 38E00003  li r7, 3
	ctx.r[7].s64 = 3;
	// 8260101C: 388ABF10  addi r4, r10, -0x40f0
	ctx.r[4].s64 = ctx.r[10].s64 + -16624;
	// 82601020: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82601024: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82601028: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8260102C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82601030: 386A29CC  addi r3, r10, 0x29cc
	ctx.r[3].s64 = ctx.r[10].s64 + 10700;
	// 82601034: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82601038: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8260103C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82601040: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82601044: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82601048: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8260104C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82601050: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82601054: 4BE65DCD  bl 0x82466e20
	ctx.lr = 0x82601058;
	sub_82466E20(ctx, base);
	// 82601058: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8260105C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82601060: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82601064: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82601068(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82601068 size=112
    let mut pc: u32 = 0x82601068;
    'dispatch: loop {
        match pc {
            0x82601068 => {
    //   block [0x82601068..0x826010D8)
	// 82601068: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260106C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82601070: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82601074: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82601078: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 8260107C: 38AA290C  addi r5, r10, 0x290c
	ctx.r[5].s64 = ctx.r[10].s64 + 10508;
	// 82601080: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82601084: 390B5B80  addi r8, r11, 0x5b80
	ctx.r[8].s64 = ctx.r[11].s64 + 23424;
	// 82601088: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8260108C: 388ABF2C  addi r4, r10, -0x40d4
	ctx.r[4].s64 = ctx.r[10].s64 + -16596;
	// 82601090: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82601094: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82601098: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8260109C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826010A0: 386A29FC  addi r3, r10, 0x29fc
	ctx.r[3].s64 = ctx.r[10].s64 + 10748;
	// 826010A4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826010A8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826010AC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826010B0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826010B4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826010B8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826010BC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826010C0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826010C4: 4BE65D5D  bl 0x82466e20
	ctx.lr = 0x826010C8;
	sub_82466E20(ctx, base);
	// 826010C8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826010CC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826010D0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826010D4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826010D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826010D8 size=112
    let mut pc: u32 = 0x826010D8;
    'dispatch: loop {
        match pc {
            0x826010D8 => {
    //   block [0x826010D8..0x82601148)
	// 826010D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826010DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826010E0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826010E4: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826010E8: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 826010EC: 392A95B4  addi r9, r10, -0x6a4c
	ctx.r[9].s64 = ctx.r[10].s64 + -27212;
	// 826010F0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826010F4: 390B5B98  addi r8, r11, 0x5b98
	ctx.r[8].s64 = ctx.r[11].s64 + 23448;
	// 826010F8: 38E00006  li r7, 6
	ctx.r[7].s64 = 6;
	// 826010FC: 388ABF44  addi r4, r10, -0x40bc
	ctx.r[4].s64 = ctx.r[10].s64 + -16572;
	// 82601100: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82601104: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82601108: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8260110C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82601110: 386A2A2C  addi r3, r10, 0x2a2c
	ctx.r[3].s64 = ctx.r[10].s64 + 10796;
	// 82601114: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82601118: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8260111C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82601120: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82601124: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82601128: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8260112C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82601130: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82601134: 4BE65CED  bl 0x82466e20
	ctx.lr = 0x82601138;
	sub_82466E20(ctx, base);
	// 82601138: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8260113C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82601140: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82601144: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82601148(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82601148 size=112
    let mut pc: u32 = 0x82601148;
    'dispatch: loop {
        match pc {
            0x82601148 => {
    //   block [0x82601148..0x826011B8)
	// 82601148: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260114C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82601150: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82601154: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82601158: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 8260115C: 38AA290C  addi r5, r10, 0x290c
	ctx.r[5].s64 = ctx.r[10].s64 + 10508;
	// 82601160: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82601164: 390B5C28  addi r8, r11, 0x5c28
	ctx.r[8].s64 = ctx.r[11].s64 + 23592;
	// 82601168: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8260116C: 388ABF68  addi r4, r10, -0x4098
	ctx.r[4].s64 = ctx.r[10].s64 + -16536;
	// 82601170: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82601174: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82601178: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8260117C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82601180: 386A2A5C  addi r3, r10, 0x2a5c
	ctx.r[3].s64 = ctx.r[10].s64 + 10844;
	// 82601184: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82601188: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8260118C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82601190: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82601194: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82601198: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8260119C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826011A0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826011A4: 4BE65C7D  bl 0x82466e20
	ctx.lr = 0x826011A8;
	sub_82466E20(ctx, base);
	// 826011A8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826011AC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826011B0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826011B4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826011B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826011B8 size=112
    let mut pc: u32 = 0x826011B8;
    'dispatch: loop {
        match pc {
            0x826011B8 => {
    //   block [0x826011B8..0x82601228)
	// 826011B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826011BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826011C0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826011C4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 826011C8: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 826011CC: 38AA2ABC  addi r5, r10, 0x2abc
	ctx.r[5].s64 = ctx.r[10].s64 + 10940;
	// 826011D0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826011D4: 390B5C40  addi r8, r11, 0x5c40
	ctx.r[8].s64 = ctx.r[11].s64 + 23616;
	// 826011D8: 39200005  li r9, 5
	ctx.r[9].s64 = 5;
	// 826011DC: 388ABF88  addi r4, r10, -0x4078
	ctx.r[4].s64 = ctx.r[10].s64 + -16504;
	// 826011E0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826011E4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 826011E8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826011EC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826011F0: 386A2A8C  addi r3, r10, 0x2a8c
	ctx.r[3].s64 = ctx.r[10].s64 + 10892;
	// 826011F4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826011F8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826011FC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82601200: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82601204: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82601208: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8260120C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82601210: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82601214: 4BE65C0D  bl 0x82466e20
	ctx.lr = 0x82601218;
	sub_82466E20(ctx, base);
	// 82601218: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8260121C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82601220: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82601224: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82601228(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82601228 size=100
    let mut pc: u32 = 0x82601228;
    'dispatch: loop {
        match pc {
            0x82601228 => {
    //   block [0x82601228..0x8260128C)
	// 82601228: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260122C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82601230: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82601234: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82601238: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8260123C: 38AA09BC  addi r5, r10, 0x9bc
	ctx.r[5].s64 = ctx.r[10].s64 + 2492;
	// 82601240: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82601244: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82601248: 388ABFA4  addi r4, r10, -0x405c
	ctx.r[4].s64 = ctx.r[10].s64 + -16476;
	// 8260124C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82601250: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82601254: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82601258: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8260125C: 386A2ABC  addi r3, r10, 0x2abc
	ctx.r[3].s64 = ctx.r[10].s64 + 10940;
	// 82601260: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82601264: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82601268: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8260126C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82601270: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82601274: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82601278: 4BE65BA9  bl 0x82466e20
	ctx.lr = 0x8260127C;
	sub_82466E20(ctx, base);
	// 8260127C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82601280: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82601284: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82601288: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82601290(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82601290 size=24
    let mut pc: u32 = 0x82601290;
    'dispatch: loop {
        match pc {
            0x82601290 => {
    //   block [0x82601290..0x826012A8)
	// 82601290: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 82601294: 3D408275  lis r10, -0x7d8b
	ctx.r[10].s64 = -2106261504;
	// 82601298: 394AB2E0  addi r10, r10, -0x4d20
	ctx.r[10].s64 = ctx.r[10].s64 + -19744;
	// 8260129C: 816B5CB8  lwz r11, 0x5cb8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(23736 as u32) ) } as u64;
	// 826012A0: 916A0050  stw r11, 0x50(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 826012A4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826012A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826012A8 size=116
    let mut pc: u32 = 0x826012A8;
    'dispatch: loop {
        match pc {
            0x826012A8 => {
    //   block [0x826012A8..0x8260131C)
	// 826012A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826012AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826012B0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826012B4: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 826012B8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826012BC: 390BB2E0  addi r8, r11, -0x4d20
	ctx.r[8].s64 = ctx.r[11].s64 + -19744;
	// 826012C0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826012C4: 392A95F0  addi r9, r10, -0x6a10
	ctx.r[9].s64 = ctx.r[10].s64 + -27152;
	// 826012C8: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 826012CC: 38E00005  li r7, 5
	ctx.r[7].s64 = 5;
	// 826012D0: 38AA290C  addi r5, r10, 0x290c
	ctx.r[5].s64 = ctx.r[10].s64 + 10508;
	// 826012D4: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826012D8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826012DC: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826012E0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826012E4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826012E8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826012EC: 3D608293  lis r11, -0x7d6d
	ctx.r[11].s64 = -2104295424;
	// 826012F0: 388ABFB8  addi r4, r10, -0x4048
	ctx.r[4].s64 = ctx.r[10].s64 + -16456;
	// 826012F4: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826012F8: 386B2AEC  addi r3, r11, 0x2aec
	ctx.r[3].s64 = ctx.r[11].s64 + 10988;
	// 826012FC: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82601300: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82601304: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82601308: 4BE65B19  bl 0x82466e20
	ctx.lr = 0x8260130C;
	sub_82466E20(ctx, base);
	// 8260130C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82601310: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82601314: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82601318: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82601320(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82601320 size=108
    let mut pc: u32 = 0x82601320;
    'dispatch: loop {
        match pc {
            0x82601320 => {
    //   block [0x82601320..0x8260138C)
	// 82601320: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82601324: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82601328: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8260132C: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 82601330: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82601334: 38EB5CBC  addi r7, r11, 0x5cbc
	ctx.r[7].s64 = ctx.r[11].s64 + 23740;
	// 82601338: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 8260133C: 388ABFD8  addi r4, r10, -0x4028
	ctx.r[4].s64 = ctx.r[10].s64 + -16424;
	// 82601340: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82601344: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82601348: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8260134C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82601350: 386A2B1C  addi r3, r10, 0x2b1c
	ctx.r[3].s64 = ctx.r[10].s64 + 11036;
	// 82601354: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82601358: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8260135C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82601360: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82601364: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82601368: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8260136C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82601370: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82601374: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82601378: 4BE65AA9  bl 0x82466e20
	ctx.lr = 0x8260137C;
	sub_82466E20(ctx, base);
	// 8260137C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82601380: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82601384: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82601388: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82601390(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82601390 size=112
    let mut pc: u32 = 0x82601390;
    'dispatch: loop {
        match pc {
            0x82601390 => {
    //   block [0x82601390..0x82601400)
	// 82601390: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82601394: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82601398: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8260139C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 826013A0: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 826013A4: 38AA290C  addi r5, r10, 0x290c
	ctx.r[5].s64 = ctx.r[10].s64 + 10508;
	// 826013A8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826013AC: 390B5CEC  addi r8, r11, 0x5cec
	ctx.r[8].s64 = ctx.r[11].s64 + 23788;
	// 826013B0: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826013B4: 388ABFFC  addi r4, r10, -0x4004
	ctx.r[4].s64 = ctx.r[10].s64 + -16388;
	// 826013B8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826013BC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 826013C0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826013C4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826013C8: 386A2B4C  addi r3, r10, 0x2b4c
	ctx.r[3].s64 = ctx.r[10].s64 + 11084;
	// 826013CC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826013D0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826013D4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826013D8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826013DC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826013E0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826013E4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826013E8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826013EC: 4BE65A35  bl 0x82466e20
	ctx.lr = 0x826013F0;
	sub_82466E20(ctx, base);
	// 826013F0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826013F4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826013F8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826013FC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82601400(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82601400 size=112
    let mut pc: u32 = 0x82601400;
    'dispatch: loop {
        match pc {
            0x82601400 => {
    //   block [0x82601400..0x82601470)
	// 82601400: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82601404: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82601408: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8260140C: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82601410: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 82601414: 392A9614  addi r9, r10, -0x69ec
	ctx.r[9].s64 = ctx.r[10].s64 + -27116;
	// 82601418: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8260141C: 390B5D08  addi r8, r11, 0x5d08
	ctx.r[8].s64 = ctx.r[11].s64 + 23816;
	// 82601420: 38E00007  li r7, 7
	ctx.r[7].s64 = 7;
	// 82601424: 388AC01C  addi r4, r10, -0x3fe4
	ctx.r[4].s64 = ctx.r[10].s64 + -16356;
	// 82601428: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8260142C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82601430: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82601434: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82601438: 386A2B7C  addi r3, r10, 0x2b7c
	ctx.r[3].s64 = ctx.r[10].s64 + 11132;
	// 8260143C: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82601440: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82601444: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82601448: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8260144C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82601450: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82601454: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82601458: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8260145C: 4BE659C5  bl 0x82466e20
	ctx.lr = 0x82601460;
	sub_82466E20(ctx, base);
	// 82601460: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82601464: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82601468: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8260146C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82601470(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82601470 size=112
    let mut pc: u32 = 0x82601470;
    'dispatch: loop {
        match pc {
            0x82601470 => {
    //   block [0x82601470..0x826014E0)
	// 82601470: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82601474: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82601478: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8260147C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82601480: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 82601484: 38AA290C  addi r5, r10, 0x290c
	ctx.r[5].s64 = ctx.r[10].s64 + 10508;
	// 82601488: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8260148C: 390B5DB0  addi r8, r11, 0x5db0
	ctx.r[8].s64 = ctx.r[11].s64 + 23984;
	// 82601490: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82601494: 388AC03C  addi r4, r10, -0x3fc4
	ctx.r[4].s64 = ctx.r[10].s64 + -16324;
	// 82601498: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8260149C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 826014A0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826014A4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826014A8: 386A2BAC  addi r3, r10, 0x2bac
	ctx.r[3].s64 = ctx.r[10].s64 + 11180;
	// 826014AC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826014B0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826014B4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826014B8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826014BC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826014C0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826014C4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826014C8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826014CC: 4BE65955  bl 0x82466e20
	ctx.lr = 0x826014D0;
	sub_82466E20(ctx, base);
	// 826014D0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826014D4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826014D8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826014DC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826014E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826014E0 size=112
    let mut pc: u32 = 0x826014E0;
    'dispatch: loop {
        match pc {
            0x826014E0 => {
    //   block [0x826014E0..0x82601550)
	// 826014E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826014E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826014E8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826014EC: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826014F0: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 826014F4: 392A966C  addi r9, r10, -0x6994
	ctx.r[9].s64 = ctx.r[10].s64 + -27028;
	// 826014F8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826014FC: 390B5DD0  addi r8, r11, 0x5dd0
	ctx.r[8].s64 = ctx.r[11].s64 + 24016;
	// 82601500: 38E00007  li r7, 7
	ctx.r[7].s64 = 7;
	// 82601504: 388AC058  addi r4, r10, -0x3fa8
	ctx.r[4].s64 = ctx.r[10].s64 + -16296;
	// 82601508: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8260150C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82601510: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82601514: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82601518: 386A2BDC  addi r3, r10, 0x2bdc
	ctx.r[3].s64 = ctx.r[10].s64 + 11228;
	// 8260151C: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82601520: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82601524: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82601528: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8260152C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82601530: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82601534: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82601538: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8260153C: 4BE658E5  bl 0x82466e20
	ctx.lr = 0x82601540;
	sub_82466E20(ctx, base);
	// 82601540: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82601544: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82601548: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8260154C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82601550(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82601550 size=116
    let mut pc: u32 = 0x82601550;
    'dispatch: loop {
        match pc {
            0x82601550 => {
    //   block [0x82601550..0x826015C4)
	// 82601550: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82601554: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82601558: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8260155C: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 82601560: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82601564: 390B5E78  addi r8, r11, 0x5e78
	ctx.r[8].s64 = ctx.r[11].s64 + 24184;
	// 82601568: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8260156C: 392A9640  addi r9, r10, -0x69c0
	ctx.r[9].s64 = ctx.r[10].s64 + -27072;
	// 82601570: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82601574: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
	// 82601578: 38AA290C  addi r5, r10, 0x290c
	ctx.r[5].s64 = ctx.r[10].s64 + 10508;
	// 8260157C: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82601580: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82601584: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82601588: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8260158C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82601590: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82601594: 3D608293  lis r11, -0x7d6d
	ctx.r[11].s64 = -2104295424;
	// 82601598: 388AC078  addi r4, r10, -0x3f88
	ctx.r[4].s64 = ctx.r[10].s64 + -16264;
	// 8260159C: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826015A0: 386B2C0C  addi r3, r11, 0x2c0c
	ctx.r[3].s64 = ctx.r[11].s64 + 11276;
	// 826015A4: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826015A8: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826015AC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826015B0: 4BE65871  bl 0x82466e20
	ctx.lr = 0x826015B4;
	sub_82466E20(ctx, base);
	// 826015B4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826015B8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826015BC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826015C0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826015C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826015C8 size=108
    let mut pc: u32 = 0x826015C8;
    'dispatch: loop {
        match pc {
            0x826015C8 => {
    //   block [0x826015C8..0x82601634)
	// 826015C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826015CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826015D0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826015D4: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 826015D8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826015DC: 38EB5E90  addi r7, r11, 0x5e90
	ctx.r[7].s64 = ctx.r[11].s64 + 24208;
	// 826015E0: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826015E4: 388AC094  addi r4, r10, -0x3f6c
	ctx.r[4].s64 = ctx.r[10].s64 + -16236;
	// 826015E8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826015EC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 826015F0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826015F4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826015F8: 386A2C3C  addi r3, r10, 0x2c3c
	ctx.r[3].s64 = ctx.r[10].s64 + 11324;
	// 826015FC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82601600: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82601604: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82601608: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8260160C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82601610: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82601614: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82601618: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8260161C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82601620: 4BE65801  bl 0x82466e20
	ctx.lr = 0x82601624;
	sub_82466E20(ctx, base);
	// 82601624: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82601628: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8260162C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82601630: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82601638(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82601638 size=112
    let mut pc: u32 = 0x82601638;
    'dispatch: loop {
        match pc {
            0x82601638 => {
    //   block [0x82601638..0x826016A8)
	// 82601638: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260163C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82601640: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82601644: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82601648: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 8260164C: 38AA290C  addi r5, r10, 0x290c
	ctx.r[5].s64 = ctx.r[10].s64 + 10508;
	// 82601650: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82601654: 390B5EC0  addi r8, r11, 0x5ec0
	ctx.r[8].s64 = ctx.r[11].s64 + 24256;
	// 82601658: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8260165C: 388AC0B8  addi r4, r10, -0x3f48
	ctx.r[4].s64 = ctx.r[10].s64 + -16200;
	// 82601660: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82601664: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82601668: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8260166C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82601670: 386A2C6C  addi r3, r10, 0x2c6c
	ctx.r[3].s64 = ctx.r[10].s64 + 11372;
	// 82601674: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82601678: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8260167C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82601680: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82601684: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82601688: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8260168C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82601690: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82601694: 4BE6578D  bl 0x82466e20
	ctx.lr = 0x82601698;
	sub_82466E20(ctx, base);
	// 82601698: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8260169C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826016A0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826016A4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826016A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826016A8 size=112
    let mut pc: u32 = 0x826016A8;
    'dispatch: loop {
        match pc {
            0x826016A8 => {
    //   block [0x826016A8..0x82601718)
	// 826016A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826016AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826016B0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826016B4: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826016B8: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 826016BC: 392A96A0  addi r9, r10, -0x6960
	ctx.r[9].s64 = ctx.r[10].s64 + -26976;
	// 826016C0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826016C4: 390B5EE0  addi r8, r11, 0x5ee0
	ctx.r[8].s64 = ctx.r[11].s64 + 24288;
	// 826016C8: 38E00007  li r7, 7
	ctx.r[7].s64 = 7;
	// 826016CC: 388AC0D8  addi r4, r10, -0x3f28
	ctx.r[4].s64 = ctx.r[10].s64 + -16168;
	// 826016D0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826016D4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 826016D8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826016DC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826016E0: 386A2C9C  addi r3, r10, 0x2c9c
	ctx.r[3].s64 = ctx.r[10].s64 + 11420;
	// 826016E4: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826016E8: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826016EC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826016F0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826016F4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826016F8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826016FC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82601700: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82601704: 4BE6571D  bl 0x82466e20
	ctx.lr = 0x82601708;
	sub_82466E20(ctx, base);
	// 82601708: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8260170C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82601710: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82601714: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82601718(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82601718 size=112
    let mut pc: u32 = 0x82601718;
    'dispatch: loop {
        match pc {
            0x82601718 => {
    //   block [0x82601718..0x82601788)
	// 82601718: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260171C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82601720: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82601724: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82601728: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 8260172C: 38AA290C  addi r5, r10, 0x290c
	ctx.r[5].s64 = ctx.r[10].s64 + 10508;
	// 82601730: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82601734: 390B5F88  addi r8, r11, 0x5f88
	ctx.r[8].s64 = ctx.r[11].s64 + 24456;
	// 82601738: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 8260173C: 388AC0F4  addi r4, r10, -0x3f0c
	ctx.r[4].s64 = ctx.r[10].s64 + -16140;
	// 82601740: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82601744: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82601748: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8260174C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82601750: 386A2CCC  addi r3, r10, 0x2ccc
	ctx.r[3].s64 = ctx.r[10].s64 + 11468;
	// 82601754: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82601758: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8260175C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82601760: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82601764: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82601768: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8260176C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82601770: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82601774: 4BE656AD  bl 0x82466e20
	ctx.lr = 0x82601778;
	sub_82466E20(ctx, base);
	// 82601778: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8260177C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82601780: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82601784: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82601788(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82601788 size=108
    let mut pc: u32 = 0x82601788;
    'dispatch: loop {
        match pc {
            0x82601788 => {
    //   block [0x82601788..0x826017F4)
	// 82601788: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260178C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82601790: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82601794: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 82601798: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8260179C: 38EB5FD0  addi r7, r11, 0x5fd0
	ctx.r[7].s64 = ctx.r[11].s64 + 24528;
	// 826017A0: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826017A4: 388AC10C  addi r4, r10, -0x3ef4
	ctx.r[4].s64 = ctx.r[10].s64 + -16116;
	// 826017A8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826017AC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 826017B0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826017B4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826017B8: 386A2CFC  addi r3, r10, 0x2cfc
	ctx.r[3].s64 = ctx.r[10].s64 + 11516;
	// 826017BC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826017C0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826017C4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826017C8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826017CC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826017D0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826017D4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826017D8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826017DC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826017E0: 4BE65641  bl 0x82466e20
	ctx.lr = 0x826017E4;
	sub_82466E20(ctx, base);
	// 826017E4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826017E8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826017EC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826017F0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826017F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826017F8 size=112
    let mut pc: u32 = 0x826017F8;
    'dispatch: loop {
        match pc {
            0x826017F8 => {
    //   block [0x826017F8..0x82601868)
	// 826017F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826017FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82601800: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82601804: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82601808: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 8260180C: 38AA290C  addi r5, r10, 0x290c
	ctx.r[5].s64 = ctx.r[10].s64 + 10508;
	// 82601810: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82601814: 390B6000  addi r8, r11, 0x6000
	ctx.r[8].s64 = ctx.r[11].s64 + 24576;
	// 82601818: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8260181C: 388AC130  addi r4, r10, -0x3ed0
	ctx.r[4].s64 = ctx.r[10].s64 + -16080;
	// 82601820: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82601824: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82601828: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8260182C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82601830: 386A2D2C  addi r3, r10, 0x2d2c
	ctx.r[3].s64 = ctx.r[10].s64 + 11564;
	// 82601834: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82601838: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8260183C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82601840: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82601844: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82601848: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8260184C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82601850: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82601854: 4BE655CD  bl 0x82466e20
	ctx.lr = 0x82601858;
	sub_82466E20(ctx, base);
	// 82601858: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8260185C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82601860: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82601864: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82601868(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82601868 size=112
    let mut pc: u32 = 0x82601868;
    'dispatch: loop {
        match pc {
            0x82601868 => {
    //   block [0x82601868..0x826018D8)
	// 82601868: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260186C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82601870: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82601874: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82601878: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 8260187C: 38AA290C  addi r5, r10, 0x290c
	ctx.r[5].s64 = ctx.r[10].s64 + 10508;
	// 82601880: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82601884: 390B6018  addi r8, r11, 0x6018
	ctx.r[8].s64 = ctx.r[11].s64 + 24600;
	// 82601888: 39200008  li r9, 8
	ctx.r[9].s64 = 8;
	// 8260188C: 388AC14C  addi r4, r10, -0x3eb4
	ctx.r[4].s64 = ctx.r[10].s64 + -16052;
	// 82601890: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82601894: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82601898: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8260189C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826018A0: 386A2D5C  addi r3, r10, 0x2d5c
	ctx.r[3].s64 = ctx.r[10].s64 + 11612;
	// 826018A4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826018A8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826018AC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826018B0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826018B4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826018B8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826018BC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826018C0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826018C4: 4BE6555D  bl 0x82466e20
	ctx.lr = 0x826018C8;
	sub_82466E20(ctx, base);
	// 826018C8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826018CC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826018D0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826018D4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826018D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826018D8 size=100
    let mut pc: u32 = 0x826018D8;
    'dispatch: loop {
        match pc {
            0x826018D8 => {
    //   block [0x826018D8..0x8260193C)
	// 826018D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826018DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826018E0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826018E4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 826018E8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826018EC: 38AA290C  addi r5, r10, 0x290c
	ctx.r[5].s64 = ctx.r[10].s64 + 10508;
	// 826018F0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826018F4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826018F8: 388AC168  addi r4, r10, -0x3e98
	ctx.r[4].s64 = ctx.r[10].s64 + -16024;
	// 826018FC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82601900: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82601904: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82601908: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8260190C: 386A2D8C  addi r3, r10, 0x2d8c
	ctx.r[3].s64 = ctx.r[10].s64 + 11660;
	// 82601910: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82601914: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82601918: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8260191C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82601920: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82601924: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82601928: 4BE654F9  bl 0x82466e20
	ctx.lr = 0x8260192C;
	sub_82466E20(ctx, base);
	// 8260192C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82601930: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82601934: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82601938: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82601940(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82601940 size=112
    let mut pc: u32 = 0x82601940;
    'dispatch: loop {
        match pc {
            0x82601940 => {
    //   block [0x82601940..0x826019B0)
	// 82601940: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82601944: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82601948: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8260194C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82601950: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 82601954: 38AA293C  addi r5, r10, 0x293c
	ctx.r[5].s64 = ctx.r[10].s64 + 10556;
	// 82601958: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8260195C: 390B60D8  addi r8, r11, 0x60d8
	ctx.r[8].s64 = ctx.r[11].s64 + 24792;
	// 82601960: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 82601964: 388AC180  addi r4, r10, -0x3e80
	ctx.r[4].s64 = ctx.r[10].s64 + -16000;
	// 82601968: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8260196C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82601970: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82601974: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82601978: 386A2DBC  addi r3, r10, 0x2dbc
	ctx.r[3].s64 = ctx.r[10].s64 + 11708;
	// 8260197C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82601980: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82601984: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82601988: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8260198C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82601990: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82601994: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82601998: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8260199C: 4BE65485  bl 0x82466e20
	ctx.lr = 0x826019A0;
	sub_82466E20(ctx, base);
	// 826019A0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826019A4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826019A8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826019AC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826019B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826019B0 size=112
    let mut pc: u32 = 0x826019B0;
    'dispatch: loop {
        match pc {
            0x826019B0 => {
    //   block [0x826019B0..0x82601A20)
	// 826019B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826019B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826019B8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826019BC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 826019C0: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 826019C4: 38AA27BC  addi r5, r10, 0x27bc
	ctx.r[5].s64 = ctx.r[10].s64 + 10172;
	// 826019C8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826019CC: 390B6108  addi r8, r11, 0x6108
	ctx.r[8].s64 = ctx.r[11].s64 + 24840;
	// 826019D0: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826019D4: 388AC19C  addi r4, r10, -0x3e64
	ctx.r[4].s64 = ctx.r[10].s64 + -15972;
	// 826019D8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826019DC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 826019E0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826019E4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826019E8: 386A2DEC  addi r3, r10, 0x2dec
	ctx.r[3].s64 = ctx.r[10].s64 + 11756;
	// 826019EC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826019F0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826019F4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826019F8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826019FC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82601A00: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82601A04: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82601A08: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82601A0C: 4BE65415  bl 0x82466e20
	ctx.lr = 0x82601A10;
	sub_82466E20(ctx, base);
	// 82601A10: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82601A14: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82601A18: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82601A1C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82601A20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82601A20 size=108
    let mut pc: u32 = 0x82601A20;
    'dispatch: loop {
        match pc {
            0x82601A20 => {
    //   block [0x82601A20..0x82601A8C)
	// 82601A20: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82601A24: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82601A28: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82601A2C: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 82601A30: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82601A34: 38EB6120  addi r7, r11, 0x6120
	ctx.r[7].s64 = ctx.r[11].s64 + 24864;
	// 82601A38: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 82601A3C: 388AC1C0  addi r4, r10, -0x3e40
	ctx.r[4].s64 = ctx.r[10].s64 + -15936;
	// 82601A40: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82601A44: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82601A48: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82601A4C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82601A50: 386A2E1C  addi r3, r10, 0x2e1c
	ctx.r[3].s64 = ctx.r[10].s64 + 11804;
	// 82601A54: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82601A58: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82601A5C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82601A60: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82601A64: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82601A68: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82601A6C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82601A70: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82601A74: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82601A78: 4BE653A9  bl 0x82466e20
	ctx.lr = 0x82601A7C;
	sub_82466E20(ctx, base);
	// 82601A7C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82601A80: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82601A84: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82601A88: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82601A90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82601A90 size=112
    let mut pc: u32 = 0x82601A90;
    'dispatch: loop {
        match pc {
            0x82601A90 => {
    //   block [0x82601A90..0x82601B00)
	// 82601A90: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82601A94: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82601A98: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82601A9C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82601AA0: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 82601AA4: 38AA2D8C  addi r5, r10, 0x2d8c
	ctx.r[5].s64 = ctx.r[10].s64 + 11660;
	// 82601AA8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82601AAC: 390B6150  addi r8, r11, 0x6150
	ctx.r[8].s64 = ctx.r[11].s64 + 24912;
	// 82601AB0: 39200006  li r9, 6
	ctx.r[9].s64 = 6;
	// 82601AB4: 388AC1E8  addi r4, r10, -0x3e18
	ctx.r[4].s64 = ctx.r[10].s64 + -15896;
	// 82601AB8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82601ABC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82601AC0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82601AC4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82601AC8: 386A2E4C  addi r3, r10, 0x2e4c
	ctx.r[3].s64 = ctx.r[10].s64 + 11852;
	// 82601ACC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82601AD0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82601AD4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82601AD8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82601ADC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82601AE0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82601AE4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82601AE8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82601AEC: 4BE65335  bl 0x82466e20
	ctx.lr = 0x82601AF0;
	sub_82466E20(ctx, base);
	// 82601AF0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82601AF4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82601AF8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82601AFC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82601B00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82601B00 size=112
    let mut pc: u32 = 0x82601B00;
    'dispatch: loop {
        match pc {
            0x82601B00 => {
    //   block [0x82601B00..0x82601B70)
	// 82601B00: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82601B04: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82601B08: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82601B0C: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82601B10: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 82601B14: 392A96CC  addi r9, r10, -0x6934
	ctx.r[9].s64 = ctx.r[10].s64 + -26932;
	// 82601B18: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82601B1C: 390B61E0  addi r8, r11, 0x61e0
	ctx.r[8].s64 = ctx.r[11].s64 + 25056;
	// 82601B20: 38E00003  li r7, 3
	ctx.r[7].s64 = 3;
	// 82601B24: 388AC200  addi r4, r10, -0x3e00
	ctx.r[4].s64 = ctx.r[10].s64 + -15872;
	// 82601B28: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82601B2C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82601B30: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82601B34: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82601B38: 386A2E7C  addi r3, r10, 0x2e7c
	ctx.r[3].s64 = ctx.r[10].s64 + 11900;
	// 82601B3C: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82601B40: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82601B44: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82601B48: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82601B4C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82601B50: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82601B54: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82601B58: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82601B5C: 4BE652C5  bl 0x82466e20
	ctx.lr = 0x82601B60;
	sub_82466E20(ctx, base);
	// 82601B60: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82601B64: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82601B68: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82601B6C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82601B70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82601B70 size=112
    let mut pc: u32 = 0x82601B70;
    'dispatch: loop {
        match pc {
            0x82601B70 => {
    //   block [0x82601B70..0x82601BE0)
	// 82601B70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82601B74: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82601B78: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82601B7C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82601B80: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 82601B84: 38AA290C  addi r5, r10, 0x290c
	ctx.r[5].s64 = ctx.r[10].s64 + 10508;
	// 82601B88: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82601B8C: 390B6228  addi r8, r11, 0x6228
	ctx.r[8].s64 = ctx.r[11].s64 + 25128;
	// 82601B90: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82601B94: 388AC218  addi r4, r10, -0x3de8
	ctx.r[4].s64 = ctx.r[10].s64 + -15848;
	// 82601B98: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82601B9C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82601BA0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82601BA4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82601BA8: 386A2EAC  addi r3, r10, 0x2eac
	ctx.r[3].s64 = ctx.r[10].s64 + 11948;
	// 82601BAC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82601BB0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82601BB4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82601BB8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82601BBC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82601BC0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82601BC4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82601BC8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82601BCC: 4BE65255  bl 0x82466e20
	ctx.lr = 0x82601BD0;
	sub_82466E20(ctx, base);
	// 82601BD0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82601BD4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82601BD8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82601BDC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82601BE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82601BE0 size=108
    let mut pc: u32 = 0x82601BE0;
    'dispatch: loop {
        match pc {
            0x82601BE0 => {
    //   block [0x82601BE0..0x82601C4C)
	// 82601BE0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82601BE4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82601BE8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82601BEC: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 82601BF0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82601BF4: 38EB6240  addi r7, r11, 0x6240
	ctx.r[7].s64 = ctx.r[11].s64 + 25152;
	// 82601BF8: 39000006  li r8, 6
	ctx.r[8].s64 = 6;
	// 82601BFC: 388AC22C  addi r4, r10, -0x3dd4
	ctx.r[4].s64 = ctx.r[10].s64 + -15828;
	// 82601C00: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82601C04: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82601C08: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82601C0C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82601C10: 386A2EDC  addi r3, r10, 0x2edc
	ctx.r[3].s64 = ctx.r[10].s64 + 11996;
	// 82601C14: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82601C18: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82601C1C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82601C20: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82601C24: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82601C28: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82601C2C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82601C30: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82601C34: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82601C38: 4BE651E9  bl 0x82466e20
	ctx.lr = 0x82601C3C;
	sub_82466E20(ctx, base);
	// 82601C3C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82601C40: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82601C44: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82601C48: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82601C50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82601C50 size=116
    let mut pc: u32 = 0x82601C50;
    'dispatch: loop {
        match pc {
            0x82601C50 => {
    //   block [0x82601C50..0x82601CC4)
	// 82601C50: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82601C54: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82601C58: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82601C5C: 3D408274  lis r10, -0x7d8c
	ctx.r[10].s64 = -2106327040;
	// 82601C60: 38E00009  li r7, 9
	ctx.r[7].s64 = 9;
	// 82601C64: 390A62D0  addi r8, r10, 0x62d0
	ctx.r[8].s64 = ctx.r[10].s64 + 25296;
	// 82601C68: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82601C6C: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82601C70: 38AA2D8C  addi r5, r10, 0x2d8c
	ctx.r[5].s64 = ctx.r[10].s64 + 11660;
	// 82601C74: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82601C78: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82601C7C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82601C80: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82601C84: 388AC250  addi r4, r10, -0x3db0
	ctx.r[4].s64 = ctx.r[10].s64 + -15792;
	// 82601C88: 396B96E0  addi r11, r11, -0x6920
	ctx.r[11].s64 = ctx.r[11].s64 + -26912;
	// 82601C8C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82601C90: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82601C94: 386A2F0C  addi r3, r10, 0x2f0c
	ctx.r[3].s64 = ctx.r[10].s64 + 12044;
	// 82601C98: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 82601C9C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82601CA0: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 82601CA4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82601CA8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82601CAC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82601CB0: 4BE65171  bl 0x82466e20
	ctx.lr = 0x82601CB4;
	sub_82466E20(ctx, base);
	// 82601CB4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82601CB8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82601CBC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82601CC0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82601CC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82601CC8 size=112
    let mut pc: u32 = 0x82601CC8;
    'dispatch: loop {
        match pc {
            0x82601CC8 => {
    //   block [0x82601CC8..0x82601D38)
	// 82601CC8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82601CCC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82601CD0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82601CD4: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82601CD8: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 82601CDC: 392A972C  addi r9, r10, -0x68d4
	ctx.r[9].s64 = ctx.r[10].s64 + -26836;
	// 82601CE0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82601CE4: 390B63B0  addi r8, r11, 0x63b0
	ctx.r[8].s64 = ctx.r[11].s64 + 25520;
	// 82601CE8: 38E00004  li r7, 4
	ctx.r[7].s64 = 4;
	// 82601CEC: 388AC264  addi r4, r10, -0x3d9c
	ctx.r[4].s64 = ctx.r[10].s64 + -15772;
	// 82601CF0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82601CF4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82601CF8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82601CFC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82601D00: 386A2F3C  addi r3, r10, 0x2f3c
	ctx.r[3].s64 = ctx.r[10].s64 + 12092;
	// 82601D04: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82601D08: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82601D0C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82601D10: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82601D14: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82601D18: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82601D1C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82601D20: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82601D24: 4BE650FD  bl 0x82466e20
	ctx.lr = 0x82601D28;
	sub_82466E20(ctx, base);
	// 82601D28: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82601D2C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82601D30: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82601D34: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82601D38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82601D38 size=112
    let mut pc: u32 = 0x82601D38;
    'dispatch: loop {
        match pc {
            0x82601D38 => {
    //   block [0x82601D38..0x82601DA8)
	// 82601D38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82601D3C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82601D40: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82601D44: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82601D48: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 82601D4C: 38AA290C  addi r5, r10, 0x290c
	ctx.r[5].s64 = ctx.r[10].s64 + 10508;
	// 82601D50: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82601D54: 390B6410  addi r8, r11, 0x6410
	ctx.r[8].s64 = ctx.r[11].s64 + 25616;
	// 82601D58: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82601D5C: 388AC280  addi r4, r10, -0x3d80
	ctx.r[4].s64 = ctx.r[10].s64 + -15744;
	// 82601D60: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82601D64: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82601D68: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82601D6C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82601D70: 386A2F6C  addi r3, r10, 0x2f6c
	ctx.r[3].s64 = ctx.r[10].s64 + 12140;
	// 82601D74: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82601D78: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82601D7C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82601D80: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82601D84: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82601D88: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82601D8C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82601D90: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82601D94: 4BE6508D  bl 0x82466e20
	ctx.lr = 0x82601D98;
	sub_82466E20(ctx, base);
	// 82601D98: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82601D9C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82601DA0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82601DA4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82601DA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82601DA8 size=108
    let mut pc: u32 = 0x82601DA8;
    'dispatch: loop {
        match pc {
            0x82601DA8 => {
    //   block [0x82601DA8..0x82601E14)
	// 82601DA8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82601DAC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82601DB0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82601DB4: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 82601DB8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82601DBC: 38EB6428  addi r7, r11, 0x6428
	ctx.r[7].s64 = ctx.r[11].s64 + 25640;
	// 82601DC0: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 82601DC4: 388AC298  addi r4, r10, -0x3d68
	ctx.r[4].s64 = ctx.r[10].s64 + -15720;
	// 82601DC8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82601DCC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82601DD0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82601DD4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82601DD8: 386A2F9C  addi r3, r10, 0x2f9c
	ctx.r[3].s64 = ctx.r[10].s64 + 12188;
	// 82601DDC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82601DE0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82601DE4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82601DE8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82601DEC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82601DF0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82601DF4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82601DF8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82601DFC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82601E00: 4BE65021  bl 0x82466e20
	ctx.lr = 0x82601E04;
	sub_82466E20(ctx, base);
	// 82601E04: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82601E08: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82601E0C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82601E10: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82601E18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82601E18 size=112
    let mut pc: u32 = 0x82601E18;
    'dispatch: loop {
        match pc {
            0x82601E18 => {
    //   block [0x82601E18..0x82601E88)
	// 82601E18: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82601E1C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82601E20: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82601E24: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82601E28: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 82601E2C: 38AA2D8C  addi r5, r10, 0x2d8c
	ctx.r[5].s64 = ctx.r[10].s64 + 11660;
	// 82601E30: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82601E34: 390B6470  addi r8, r11, 0x6470
	ctx.r[8].s64 = ctx.r[11].s64 + 25712;
	// 82601E38: 39200005  li r9, 5
	ctx.r[9].s64 = 5;
	// 82601E3C: 388AC2C0  addi r4, r10, -0x3d40
	ctx.r[4].s64 = ctx.r[10].s64 + -15680;
	// 82601E40: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82601E44: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82601E48: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82601E4C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82601E50: 386A2FCC  addi r3, r10, 0x2fcc
	ctx.r[3].s64 = ctx.r[10].s64 + 12236;
	// 82601E54: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82601E58: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82601E5C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82601E60: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82601E64: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82601E68: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82601E6C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82601E70: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82601E74: 4BE64FAD  bl 0x82466e20
	ctx.lr = 0x82601E78;
	sub_82466E20(ctx, base);
	// 82601E78: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82601E7C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82601E80: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82601E84: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82601E88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82601E88 size=112
    let mut pc: u32 = 0x82601E88;
    'dispatch: loop {
        match pc {
            0x82601E88 => {
    //   block [0x82601E88..0x82601EF8)
	// 82601E88: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82601E8C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82601E90: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82601E94: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82601E98: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 82601E9C: 38AA290C  addi r5, r10, 0x290c
	ctx.r[5].s64 = ctx.r[10].s64 + 10508;
	// 82601EA0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82601EA4: 390B64E8  addi r8, r11, 0x64e8
	ctx.r[8].s64 = ctx.r[11].s64 + 25832;
	// 82601EA8: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 82601EAC: 388AC2D8  addi r4, r10, -0x3d28
	ctx.r[4].s64 = ctx.r[10].s64 + -15656;
	// 82601EB0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82601EB4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82601EB8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82601EBC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82601EC0: 386A2FFC  addi r3, r10, 0x2ffc
	ctx.r[3].s64 = ctx.r[10].s64 + 12284;
	// 82601EC4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82601EC8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82601ECC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82601ED0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82601ED4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82601ED8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82601EDC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82601EE0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82601EE4: 4BE64F3D  bl 0x82466e20
	ctx.lr = 0x82601EE8;
	sub_82466E20(ctx, base);
	// 82601EE8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82601EEC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82601EF0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82601EF4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82601EF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82601EF8 size=108
    let mut pc: u32 = 0x82601EF8;
    'dispatch: loop {
        match pc {
            0x82601EF8 => {
    //   block [0x82601EF8..0x82601F64)
	// 82601EF8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82601EFC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82601F00: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82601F04: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 82601F08: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82601F0C: 38EB6518  addi r7, r11, 0x6518
	ctx.r[7].s64 = ctx.r[11].s64 + 25880;
	// 82601F10: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 82601F14: 388AC2F4  addi r4, r10, -0x3d0c
	ctx.r[4].s64 = ctx.r[10].s64 + -15628;
	// 82601F18: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82601F1C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82601F20: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82601F24: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82601F28: 386A302C  addi r3, r10, 0x302c
	ctx.r[3].s64 = ctx.r[10].s64 + 12332;
	// 82601F2C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82601F30: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82601F34: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82601F38: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82601F3C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82601F40: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82601F44: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82601F48: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82601F4C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82601F50: 4BE64ED1  bl 0x82466e20
	ctx.lr = 0x82601F54;
	sub_82466E20(ctx, base);
	// 82601F54: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82601F58: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82601F5C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82601F60: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82601F68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82601F68 size=108
    let mut pc: u32 = 0x82601F68;
    'dispatch: loop {
        match pc {
            0x82601F68 => {
    //   block [0x82601F68..0x82601FD4)
	// 82601F68: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82601F6C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82601F70: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82601F74: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 82601F78: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82601F7C: 38EB6578  addi r7, r11, 0x6578
	ctx.r[7].s64 = ctx.r[11].s64 + 25976;
	// 82601F80: 39000005  li r8, 5
	ctx.r[8].s64 = 5;
	// 82601F84: 388AC324  addi r4, r10, -0x3cdc
	ctx.r[4].s64 = ctx.r[10].s64 + -15580;
	// 82601F88: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82601F8C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82601F90: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82601F94: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82601F98: 386A305C  addi r3, r10, 0x305c
	ctx.r[3].s64 = ctx.r[10].s64 + 12380;
	// 82601F9C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82601FA0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82601FA4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82601FA8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82601FAC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82601FB0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82601FB4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82601FB8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82601FBC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82601FC0: 4BE64E61  bl 0x82466e20
	ctx.lr = 0x82601FC4;
	sub_82466E20(ctx, base);
	// 82601FC4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82601FC8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82601FCC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82601FD0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82601FD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82601FD8 size=112
    let mut pc: u32 = 0x82601FD8;
    'dispatch: loop {
        match pc {
            0x82601FD8 => {
    //   block [0x82601FD8..0x82602048)
	// 82601FD8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82601FDC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82601FE0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82601FE4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82601FE8: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 82601FEC: 38AA290C  addi r5, r10, 0x290c
	ctx.r[5].s64 = ctx.r[10].s64 + 10508;
	// 82601FF0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82601FF4: 390B65F0  addi r8, r11, 0x65f0
	ctx.r[8].s64 = ctx.r[11].s64 + 26096;
	// 82601FF8: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 82601FFC: 388AC344  addi r4, r10, -0x3cbc
	ctx.r[4].s64 = ctx.r[10].s64 + -15548;
	// 82602000: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82602004: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82602008: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8260200C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82602010: 386A308C  addi r3, r10, 0x308c
	ctx.r[3].s64 = ctx.r[10].s64 + 12428;
	// 82602014: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82602018: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8260201C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82602020: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82602024: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82602028: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8260202C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82602030: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82602034: 4BE64DED  bl 0x82466e20
	ctx.lr = 0x82602038;
	sub_82466E20(ctx, base);
	// 82602038: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8260203C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82602040: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82602044: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82602048(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82602048 size=24
    let mut pc: u32 = 0x82602048;
    'dispatch: loop {
        match pc {
            0x82602048 => {
    //   block [0x82602048..0x82602060)
	// 82602048: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 8260204C: 3D408275  lis r10, -0x7d8b
	ctx.r[10].s64 = -2106261504;
	// 82602050: 394AB358  addi r10, r10, -0x4ca8
	ctx.r[10].s64 = ctx.r[10].s64 + -19624;
	// 82602054: 816B63AC  lwz r11, 0x63ac(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(25516 as u32) ) } as u64;
	// 82602058: 916A0008  stw r11, 8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 8260205C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82602060(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82602060 size=116
    let mut pc: u32 = 0x82602060;
    'dispatch: loop {
        match pc {
            0x82602060 => {
    //   block [0x82602060..0x826020D4)
	// 82602060: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82602064: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82602068: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8260206C: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 82602070: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82602074: 390BB358  addi r8, r11, -0x4ca8
	ctx.r[8].s64 = ctx.r[11].s64 + -19624;
	// 82602078: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8260207C: 392A9770  addi r9, r10, -0x6890
	ctx.r[9].s64 = ctx.r[10].s64 + -26768;
	// 82602080: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82602084: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
	// 82602088: 38AA09BC  addi r5, r10, 0x9bc
	ctx.r[5].s64 = ctx.r[10].s64 + 2492;
	// 8260208C: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82602090: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82602094: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82602098: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8260209C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826020A0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826020A4: 3D608293  lis r11, -0x7d6d
	ctx.r[11].s64 = -2104295424;
	// 826020A8: 388AC360  addi r4, r10, -0x3ca0
	ctx.r[4].s64 = ctx.r[10].s64 + -15520;
	// 826020AC: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826020B0: 386B30BC  addi r3, r11, 0x30bc
	ctx.r[3].s64 = ctx.r[11].s64 + 12476;
	// 826020B4: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826020B8: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826020BC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826020C0: 4BE64D61  bl 0x82466e20
	ctx.lr = 0x826020C4;
	sub_82466E20(ctx, base);
	// 826020C4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826020C8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826020CC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826020D0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826020D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826020D8 size=112
    let mut pc: u32 = 0x826020D8;
    'dispatch: loop {
        match pc {
            0x826020D8 => {
    //   block [0x826020D8..0x82602148)
	// 826020D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826020DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826020E0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826020E4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 826020E8: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 826020EC: 38AA30BC  addi r5, r10, 0x30bc
	ctx.r[5].s64 = ctx.r[10].s64 + 12476;
	// 826020F0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826020F4: 390B6638  addi r8, r11, 0x6638
	ctx.r[8].s64 = ctx.r[11].s64 + 26168;
	// 826020F8: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826020FC: 388AC374  addi r4, r10, -0x3c8c
	ctx.r[4].s64 = ctx.r[10].s64 + -15500;
	// 82602100: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82602104: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82602108: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8260210C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82602110: 386A30EC  addi r3, r10, 0x30ec
	ctx.r[3].s64 = ctx.r[10].s64 + 12524;
	// 82602114: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82602118: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8260211C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82602120: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82602124: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82602128: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8260212C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82602130: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82602134: 4BE64CED  bl 0x82466e20
	ctx.lr = 0x82602138;
	sub_82466E20(ctx, base);
	// 82602138: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8260213C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82602140: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82602144: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82602148(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82602148 size=24
    let mut pc: u32 = 0x82602148;
    'dispatch: loop {
        match pc {
            0x82602148 => {
    //   block [0x82602148..0x82602160)
	// 82602148: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 8260214C: 3D408275  lis r10, -0x7d8b
	ctx.r[10].s64 = -2106261504;
	// 82602150: 394AB370  addi r10, r10, -0x4c90
	ctx.r[10].s64 = ctx.r[10].s64 + -19600;
	// 82602154: 816B6668  lwz r11, 0x6668(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(26216 as u32) ) } as u64;
	// 82602158: 916A0020  stw r11, 0x20(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 8260215C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82602160(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82602160 size=116
    let mut pc: u32 = 0x82602160;
    'dispatch: loop {
        match pc {
            0x82602160 => {
    //   block [0x82602160..0x826021D4)
	// 82602160: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82602164: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82602168: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8260216C: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 82602170: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82602174: 390BB370  addi r8, r11, -0x4c90
	ctx.r[8].s64 = ctx.r[11].s64 + -19600;
	// 82602178: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8260217C: 392A97AC  addi r9, r10, -0x6854
	ctx.r[9].s64 = ctx.r[10].s64 + -26708;
	// 82602180: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82602184: 38E00005  li r7, 5
	ctx.r[7].s64 = 5;
	// 82602188: 38AA30EC  addi r5, r10, 0x30ec
	ctx.r[5].s64 = ctx.r[10].s64 + 12524;
	// 8260218C: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82602190: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82602194: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82602198: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8260219C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826021A0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826021A4: 3D608293  lis r11, -0x7d6d
	ctx.r[11].s64 = -2104295424;
	// 826021A8: 388AC394  addi r4, r10, -0x3c6c
	ctx.r[4].s64 = ctx.r[10].s64 + -15468;
	// 826021AC: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826021B0: 386B311C  addi r3, r11, 0x311c
	ctx.r[3].s64 = ctx.r[11].s64 + 12572;
	// 826021B4: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826021B8: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826021BC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826021C0: 4BE64C61  bl 0x82466e20
	ctx.lr = 0x826021C4;
	sub_82466E20(ctx, base);
	// 826021C4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826021C8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826021CC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826021D0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826021D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826021D8 size=112
    let mut pc: u32 = 0x826021D8;
    'dispatch: loop {
        match pc {
            0x826021D8 => {
    //   block [0x826021D8..0x82602248)
	// 826021D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826021DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826021E0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826021E4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 826021E8: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 826021EC: 38AA30EC  addi r5, r10, 0x30ec
	ctx.r[5].s64 = ctx.r[10].s64 + 12524;
	// 826021F0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826021F4: 390B6670  addi r8, r11, 0x6670
	ctx.r[8].s64 = ctx.r[11].s64 + 26224;
	// 826021F8: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 826021FC: 388AC3B0  addi r4, r10, -0x3c50
	ctx.r[4].s64 = ctx.r[10].s64 + -15440;
	// 82602200: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82602204: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82602208: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8260220C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82602210: 386A314C  addi r3, r10, 0x314c
	ctx.r[3].s64 = ctx.r[10].s64 + 12620;
	// 82602214: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82602218: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8260221C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82602220: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82602224: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82602228: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8260222C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82602230: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82602234: 4BE64BED  bl 0x82466e20
	ctx.lr = 0x82602238;
	sub_82466E20(ctx, base);
	// 82602238: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8260223C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82602240: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82602244: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82602248(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82602248 size=112
    let mut pc: u32 = 0x82602248;
    'dispatch: loop {
        match pc {
            0x82602248 => {
    //   block [0x82602248..0x826022B8)
	// 82602248: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260224C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82602250: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82602254: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82602258: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 8260225C: 38AA30EC  addi r5, r10, 0x30ec
	ctx.r[5].s64 = ctx.r[10].s64 + 12524;
	// 82602260: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82602264: 390B66D0  addi r8, r11, 0x66d0
	ctx.r[8].s64 = ctx.r[11].s64 + 26320;
	// 82602268: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 8260226C: 388AC3CC  addi r4, r10, -0x3c34
	ctx.r[4].s64 = ctx.r[10].s64 + -15412;
	// 82602270: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82602274: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82602278: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8260227C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82602280: 386A317C  addi r3, r10, 0x317c
	ctx.r[3].s64 = ctx.r[10].s64 + 12668;
	// 82602284: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82602288: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8260228C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82602290: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82602294: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82602298: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8260229C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826022A0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826022A4: 4BE64B7D  bl 0x82466e20
	ctx.lr = 0x826022A8;
	sub_82466E20(ctx, base);
	// 826022A8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826022AC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826022B0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826022B4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826022B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826022B8 size=112
    let mut pc: u32 = 0x826022B8;
    'dispatch: loop {
        match pc {
            0x826022B8 => {
    //   block [0x826022B8..0x82602328)
	// 826022B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826022BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826022C0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826022C4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 826022C8: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 826022CC: 38AA30EC  addi r5, r10, 0x30ec
	ctx.r[5].s64 = ctx.r[10].s64 + 12524;
	// 826022D0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826022D4: 390B6700  addi r8, r11, 0x6700
	ctx.r[8].s64 = ctx.r[11].s64 + 26368;
	// 826022D8: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 826022DC: 388AC3EC  addi r4, r10, -0x3c14
	ctx.r[4].s64 = ctx.r[10].s64 + -15380;
	// 826022E0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826022E4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 826022E8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826022EC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826022F0: 386A31AC  addi r3, r10, 0x31ac
	ctx.r[3].s64 = ctx.r[10].s64 + 12716;
	// 826022F4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826022F8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826022FC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82602300: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82602304: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82602308: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8260230C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82602310: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82602314: 4BE64B0D  bl 0x82466e20
	ctx.lr = 0x82602318;
	sub_82466E20(ctx, base);
	// 82602318: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8260231C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82602320: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82602324: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82602328(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82602328 size=108
    let mut pc: u32 = 0x82602328;
    'dispatch: loop {
        match pc {
            0x82602328 => {
    //   block [0x82602328..0x82602394)
	// 82602328: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260232C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82602330: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82602334: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 82602338: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8260233C: 38EB6748  addi r7, r11, 0x6748
	ctx.r[7].s64 = ctx.r[11].s64 + 26440;
	// 82602340: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 82602344: 388AC408  addi r4, r10, -0x3bf8
	ctx.r[4].s64 = ctx.r[10].s64 + -15352;
	// 82602348: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8260234C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82602350: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82602354: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82602358: 386A31DC  addi r3, r10, 0x31dc
	ctx.r[3].s64 = ctx.r[10].s64 + 12764;
	// 8260235C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82602360: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82602364: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82602368: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8260236C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82602370: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82602374: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82602378: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8260237C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82602380: 4BE64AA1  bl 0x82466e20
	ctx.lr = 0x82602384;
	sub_82466E20(ctx, base);
	// 82602384: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82602388: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8260238C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82602390: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82602398(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82602398 size=112
    let mut pc: u32 = 0x82602398;
    'dispatch: loop {
        match pc {
            0x82602398 => {
    //   block [0x82602398..0x82602408)
	// 82602398: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260239C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826023A0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826023A4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 826023A8: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 826023AC: 38AA290C  addi r5, r10, 0x290c
	ctx.r[5].s64 = ctx.r[10].s64 + 10508;
	// 826023B0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826023B4: 390B6778  addi r8, r11, 0x6778
	ctx.r[8].s64 = ctx.r[11].s64 + 26488;
	// 826023B8: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826023BC: 388AC428  addi r4, r10, -0x3bd8
	ctx.r[4].s64 = ctx.r[10].s64 + -15320;
	// 826023C0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826023C4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 826023C8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826023CC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826023D0: 386A320C  addi r3, r10, 0x320c
	ctx.r[3].s64 = ctx.r[10].s64 + 12812;
	// 826023D4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826023D8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826023DC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826023E0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826023E4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826023E8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826023EC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826023F0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826023F4: 4BE64A2D  bl 0x82466e20
	ctx.lr = 0x826023F8;
	sub_82466E20(ctx, base);
	// 826023F8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826023FC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82602400: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82602404: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82602408(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82602408 size=108
    let mut pc: u32 = 0x82602408;
    'dispatch: loop {
        match pc {
            0x82602408 => {
    //   block [0x82602408..0x82602474)
	// 82602408: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260240C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82602410: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82602414: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 82602418: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8260241C: 38EB6790  addi r7, r11, 0x6790
	ctx.r[7].s64 = ctx.r[11].s64 + 26512;
	// 82602420: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 82602424: 388AC440  addi r4, r10, -0x3bc0
	ctx.r[4].s64 = ctx.r[10].s64 + -15296;
	// 82602428: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8260242C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82602430: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82602434: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82602438: 386A323C  addi r3, r10, 0x323c
	ctx.r[3].s64 = ctx.r[10].s64 + 12860;
	// 8260243C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82602440: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82602444: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82602448: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8260244C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82602450: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82602454: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82602458: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8260245C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82602460: 4BE649C1  bl 0x82466e20
	ctx.lr = 0x82602464;
	sub_82466E20(ctx, base);
	// 82602464: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82602468: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8260246C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82602470: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82602478(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82602478 size=108
    let mut pc: u32 = 0x82602478;
    'dispatch: loop {
        match pc {
            0x82602478 => {
    //   block [0x82602478..0x826024E4)
	// 82602478: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260247C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82602480: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82602484: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 82602488: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8260248C: 38EB67D8  addi r7, r11, 0x67d8
	ctx.r[7].s64 = ctx.r[11].s64 + 26584;
	// 82602490: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 82602494: 388AC46C  addi r4, r10, -0x3b94
	ctx.r[4].s64 = ctx.r[10].s64 + -15252;
	// 82602498: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8260249C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 826024A0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826024A4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826024A8: 386A326C  addi r3, r10, 0x326c
	ctx.r[3].s64 = ctx.r[10].s64 + 12908;
	// 826024AC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826024B0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826024B4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826024B8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826024BC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826024C0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826024C4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826024C8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826024CC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826024D0: 4BE64951  bl 0x82466e20
	ctx.lr = 0x826024D4;
	sub_82466E20(ctx, base);
	// 826024D4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826024D8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826024DC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826024E0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826024E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826024E8 size=108
    let mut pc: u32 = 0x826024E8;
    'dispatch: loop {
        match pc {
            0x826024E8 => {
    //   block [0x826024E8..0x82602554)
	// 826024E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826024EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826024F0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826024F4: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 826024F8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826024FC: 38EB6838  addi r7, r11, 0x6838
	ctx.r[7].s64 = ctx.r[11].s64 + 26680;
	// 82602500: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 82602504: 388AC48C  addi r4, r10, -0x3b74
	ctx.r[4].s64 = ctx.r[10].s64 + -15220;
	// 82602508: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8260250C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82602510: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82602514: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82602518: 386A329C  addi r3, r10, 0x329c
	ctx.r[3].s64 = ctx.r[10].s64 + 12956;
	// 8260251C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82602520: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82602524: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82602528: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8260252C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82602530: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82602534: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82602538: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8260253C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82602540: 4BE648E1  bl 0x82466e20
	ctx.lr = 0x82602544;
	sub_82466E20(ctx, base);
	// 82602544: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82602548: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8260254C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82602550: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82602558(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82602558 size=116
    let mut pc: u32 = 0x82602558;
    'dispatch: loop {
        match pc {
            0x82602558 => {
    //   block [0x82602558..0x826025CC)
	// 82602558: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260255C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82602560: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82602564: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82602568: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 8260256C: 392B97E8  addi r9, r11, -0x6818
	ctx.r[9].s64 = ctx.r[11].s64 + -26648;
	// 82602570: 38AA37AC  addi r5, r10, 0x37ac
	ctx.r[5].s64 = ctx.r[10].s64 + 14252;
	// 82602574: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82602578: 38E90018  addi r7, r9, 0x18
	ctx.r[7].s64 = ctx.r[9].s64 + 24;
	// 8260257C: 38C00011  li r6, 0x11
	ctx.r[6].s64 = 17;
	// 82602580: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 82602584: 388AC4A8  addi r4, r10, -0x3b58
	ctx.r[4].s64 = ctx.r[10].s64 + -15192;
	// 82602588: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8260258C: 396B6868  addi r11, r11, 0x6868
	ctx.r[11].s64 = ctx.r[11].s64 + 26728;
	// 82602590: 90E10064  stw r7, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[7].u32 ) };
	// 82602594: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82602598: 90C1005C  stw r6, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[6].u32 ) };
	// 8260259C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826025A0: 386A32CC  addi r3, r10, 0x32cc
	ctx.r[3].s64 = ctx.r[10].s64 + 13004;
	// 826025A4: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826025A8: 91010074  stw r8, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[8].u32 ) };
	// 826025AC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826025B0: 9101006C  stw r8, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[8].u32 ) };
	// 826025B4: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826025B8: 4BE64869  bl 0x82466e20
	ctx.lr = 0x826025BC;
	sub_82466E20(ctx, base);
	// 826025BC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826025C0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826025C4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826025C8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826025D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826025D0 size=100
    let mut pc: u32 = 0x826025D0;
    'dispatch: loop {
        match pc {
            0x826025D0 => {
    //   block [0x826025D0..0x82602634)
	// 826025D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826025D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826025D8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826025DC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 826025E0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826025E4: 38AA09BC  addi r5, r10, 0x9bc
	ctx.r[5].s64 = ctx.r[10].s64 + 2492;
	// 826025E8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826025EC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826025F0: 388AC4B4  addi r4, r10, -0x3b4c
	ctx.r[4].s64 = ctx.r[10].s64 + -15180;
	// 826025F4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 826025F8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826025FC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82602600: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82602604: 386A32FC  addi r3, r10, 0x32fc
	ctx.r[3].s64 = ctx.r[10].s64 + 13052;
	// 82602608: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8260260C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82602610: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82602614: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82602618: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8260261C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82602620: 4BE64801  bl 0x82466e20
	ctx.lr = 0x82602624;
	sub_82466E20(ctx, base);
	// 82602624: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82602628: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8260262C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82602630: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82602638(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82602638 size=100
    let mut pc: u32 = 0x82602638;
    'dispatch: loop {
        match pc {
            0x82602638 => {
    //   block [0x82602638..0x8260269C)
	// 82602638: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260263C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82602640: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82602644: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82602648: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8260264C: 38AA338C  addi r5, r10, 0x338c
	ctx.r[5].s64 = ctx.r[10].s64 + 13196;
	// 82602650: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82602654: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82602658: 388AC4CC  addi r4, r10, -0x3b34
	ctx.r[4].s64 = ctx.r[10].s64 + -15156;
	// 8260265C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82602660: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82602664: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82602668: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8260266C: 386A332C  addi r3, r10, 0x332c
	ctx.r[3].s64 = ctx.r[10].s64 + 13100;
	// 82602670: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82602674: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82602678: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8260267C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82602680: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82602684: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82602688: 4BE64799  bl 0x82466e20
	ctx.lr = 0x8260268C;
	sub_82466E20(ctx, base);
	// 8260268C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82602690: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82602694: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82602698: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826026A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826026A0 size=100
    let mut pc: u32 = 0x826026A0;
    'dispatch: loop {
        match pc {
            0x826026A0 => {
    //   block [0x826026A0..0x82602704)
	// 826026A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826026A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826026A8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826026AC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 826026B0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826026B4: 38AA32CC  addi r5, r10, 0x32cc
	ctx.r[5].s64 = ctx.r[10].s64 + 13004;
	// 826026B8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826026BC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826026C0: 388AC4E8  addi r4, r10, -0x3b18
	ctx.r[4].s64 = ctx.r[10].s64 + -15128;
	// 826026C4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 826026C8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826026CC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826026D0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826026D4: 386A335C  addi r3, r10, 0x335c
	ctx.r[3].s64 = ctx.r[10].s64 + 13148;
	// 826026D8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826026DC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826026E0: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826026E4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826026E8: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826026EC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826026F0: 4BE64731  bl 0x82466e20
	ctx.lr = 0x826026F4;
	sub_82466E20(ctx, base);
	// 826026F4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826026F8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826026FC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82602700: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82602708(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82602708 size=104
    let mut pc: u32 = 0x82602708;
    'dispatch: loop {
        match pc {
            0x82602708 => {
    //   block [0x82602708..0x82602770)
	// 82602708: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260270C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82602710: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82602714: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82602718: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8260271C: 392A9868  addi r9, r10, -0x6798
	ctx.r[9].s64 = ctx.r[10].s64 + -26520;
	// 82602720: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82602724: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82602728: 38AA32FC  addi r5, r10, 0x32fc
	ctx.r[5].s64 = ctx.r[10].s64 + 13052;
	// 8260272C: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82602730: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82602734: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82602738: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8260273C: 388AC4F8  addi r4, r10, -0x3b08
	ctx.r[4].s64 = ctx.r[10].s64 + -15112;
	// 82602740: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82602744: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82602748: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8260274C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82602750: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82602754: 386A338C  addi r3, r10, 0x338c
	ctx.r[3].s64 = ctx.r[10].s64 + 13196;
	// 82602758: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8260275C: 4BE646C5  bl 0x82466e20
	ctx.lr = 0x82602760;
	sub_82466E20(ctx, base);
	// 82602760: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82602764: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82602768: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8260276C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82602770(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82602770 size=108
    let mut pc: u32 = 0x82602770;
    'dispatch: loop {
        match pc {
            0x82602770 => {
    //   block [0x82602770..0x826027DC)
	// 82602770: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82602774: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82602778: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8260277C: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 82602780: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82602784: 38EB6A04  addi r7, r11, 0x6a04
	ctx.r[7].s64 = ctx.r[11].s64 + 27140;
	// 82602788: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 8260278C: 388AC510  addi r4, r10, -0x3af0
	ctx.r[4].s64 = ctx.r[10].s64 + -15088;
	// 82602790: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82602794: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82602798: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8260279C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826027A0: 386A33BC  addi r3, r10, 0x33bc
	ctx.r[3].s64 = ctx.r[10].s64 + 13244;
	// 826027A4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826027A8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826027AC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826027B0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826027B4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826027B8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826027BC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826027C0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826027C4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826027C8: 4BE64659  bl 0x82466e20
	ctx.lr = 0x826027CC;
	sub_82466E20(ctx, base);
	// 826027CC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826027D0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826027D4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826027D8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826027E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826027E0 size=112
    let mut pc: u32 = 0x826027E0;
    'dispatch: loop {
        match pc {
            0x826027E0 => {
    //   block [0x826027E0..0x82602850)
	// 826027E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826027E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826027E8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826027EC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 826027F0: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 826027F4: 38AA338C  addi r5, r10, 0x338c
	ctx.r[5].s64 = ctx.r[10].s64 + 13196;
	// 826027F8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826027FC: 390B6A38  addi r8, r11, 0x6a38
	ctx.r[8].s64 = ctx.r[11].s64 + 27192;
	// 82602800: 39200007  li r9, 7
	ctx.r[9].s64 = 7;
	// 82602804: 388AC538  addi r4, r10, -0x3ac8
	ctx.r[4].s64 = ctx.r[10].s64 + -15048;
	// 82602808: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8260280C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82602810: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82602814: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82602818: 386A33EC  addi r3, r10, 0x33ec
	ctx.r[3].s64 = ctx.r[10].s64 + 13292;
	// 8260281C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82602820: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82602824: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82602828: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8260282C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82602830: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82602834: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82602838: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8260283C: 4BE645E5  bl 0x82466e20
	ctx.lr = 0x82602840;
	sub_82466E20(ctx, base);
	// 82602840: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82602844: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82602848: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8260284C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82602850(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82602850 size=24
    let mut pc: u32 = 0x82602850;
    'dispatch: loop {
        match pc {
            0x82602850 => {
    //   block [0x82602850..0x82602868)
	// 82602850: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 82602854: 3D408275  lis r10, -0x7d8b
	ctx.r[10].s64 = -2106261504;
	// 82602858: 394AB3E8  addi r10, r10, -0x4c18
	ctx.r[10].s64 = ctx.r[10].s64 + -19480;
	// 8260285C: 816B6A34  lwz r11, 0x6a34(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(27188 as u32) ) } as u64;
	// 82602860: 916A0008  stw r11, 8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82602864: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82602868(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82602868 size=116
    let mut pc: u32 = 0x82602868;
    'dispatch: loop {
        match pc {
            0x82602868 => {
    //   block [0x82602868..0x826028DC)
	// 82602868: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260286C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82602870: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82602874: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 82602878: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8260287C: 390BB3E8  addi r8, r11, -0x4c18
	ctx.r[8].s64 = ctx.r[11].s64 + -19480;
	// 82602880: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82602884: 392A98D8  addi r9, r10, -0x6728
	ctx.r[9].s64 = ctx.r[10].s64 + -26408;
	// 82602888: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 8260288C: 38E0000B  li r7, 0xb
	ctx.r[7].s64 = 11;
	// 82602890: 38AA09BC  addi r5, r10, 0x9bc
	ctx.r[5].s64 = ctx.r[10].s64 + 2492;
	// 82602894: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82602898: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8260289C: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826028A0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826028A4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826028A8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826028AC: 3D608293  lis r11, -0x7d6d
	ctx.r[11].s64 = -2104295424;
	// 826028B0: 388AC558  addi r4, r10, -0x3aa8
	ctx.r[4].s64 = ctx.r[10].s64 + -15016;
	// 826028B4: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826028B8: 386B341C  addi r3, r11, 0x341c
	ctx.r[3].s64 = ctx.r[11].s64 + 13340;
	// 826028BC: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826028C0: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826028C4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826028C8: 4BE64559  bl 0x82466e20
	ctx.lr = 0x826028CC;
	sub_82466E20(ctx, base);
	// 826028CC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826028D0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826028D4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826028D8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826028E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826028E0 size=100
    let mut pc: u32 = 0x826028E0;
    'dispatch: loop {
        match pc {
            0x826028E0 => {
    //   block [0x826028E0..0x82602944)
	// 826028E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826028E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826028E8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826028EC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 826028F0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826028F4: 38AA341C  addi r5, r10, 0x341c
	ctx.r[5].s64 = ctx.r[10].s64 + 13340;
	// 826028F8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826028FC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82602900: 388AC564  addi r4, r10, -0x3a9c
	ctx.r[4].s64 = ctx.r[10].s64 + -15004;
	// 82602904: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82602908: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8260290C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82602910: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82602914: 386A344C  addi r3, r10, 0x344c
	ctx.r[3].s64 = ctx.r[10].s64 + 13388;
	// 82602918: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8260291C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82602920: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82602924: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82602928: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8260292C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82602930: 4BE644F1  bl 0x82466e20
	ctx.lr = 0x82602934;
	sub_82466E20(ctx, base);
	// 82602934: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82602938: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8260293C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82602940: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82602948(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82602948 size=100
    let mut pc: u32 = 0x82602948;
    'dispatch: loop {
        match pc {
            0x82602948 => {
    //   block [0x82602948..0x826029AC)
	// 82602948: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260294C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82602950: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82602954: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82602958: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8260295C: 38AA341C  addi r5, r10, 0x341c
	ctx.r[5].s64 = ctx.r[10].s64 + 13340;
	// 82602960: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82602964: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82602968: 388AC574  addi r4, r10, -0x3a8c
	ctx.r[4].s64 = ctx.r[10].s64 + -14988;
	// 8260296C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82602970: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82602974: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82602978: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8260297C: 386A347C  addi r3, r10, 0x347c
	ctx.r[3].s64 = ctx.r[10].s64 + 13436;
	// 82602980: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82602984: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82602988: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8260298C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82602990: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82602994: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82602998: 4BE64489  bl 0x82466e20
	ctx.lr = 0x8260299C;
	sub_82466E20(ctx, base);
	// 8260299C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826029A0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826029A4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826029A8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826029B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826029B0 size=100
    let mut pc: u32 = 0x826029B0;
    'dispatch: loop {
        match pc {
            0x826029B0 => {
    //   block [0x826029B0..0x82602A14)
	// 826029B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826029B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826029B8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826029BC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 826029C0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826029C4: 38AA34DC  addi r5, r10, 0x34dc
	ctx.r[5].s64 = ctx.r[10].s64 + 13532;
	// 826029C8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826029CC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826029D0: 388AC588  addi r4, r10, -0x3a78
	ctx.r[4].s64 = ctx.r[10].s64 + -14968;
	// 826029D4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 826029D8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826029DC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826029E0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826029E4: 386A34AC  addi r3, r10, 0x34ac
	ctx.r[3].s64 = ctx.r[10].s64 + 13484;
	// 826029E8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826029EC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826029F0: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826029F4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826029F8: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826029FC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82602A00: 4BE64421  bl 0x82466e20
	ctx.lr = 0x82602A04;
	sub_82466E20(ctx, base);
	// 82602A04: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82602A08: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82602A0C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82602A10: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82602A18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82602A18 size=100
    let mut pc: u32 = 0x82602A18;
    'dispatch: loop {
        match pc {
            0x82602A18 => {
    //   block [0x82602A18..0x82602A7C)
	// 82602A18: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82602A1C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82602A20: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82602A24: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82602A28: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82602A2C: 38AA341C  addi r5, r10, 0x341c
	ctx.r[5].s64 = ctx.r[10].s64 + 13340;
	// 82602A30: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82602A34: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82602A38: 388AC59C  addi r4, r10, -0x3a64
	ctx.r[4].s64 = ctx.r[10].s64 + -14948;
	// 82602A3C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82602A40: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82602A44: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82602A48: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82602A4C: 386A34DC  addi r3, r10, 0x34dc
	ctx.r[3].s64 = ctx.r[10].s64 + 13532;
	// 82602A50: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82602A54: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82602A58: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82602A5C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82602A60: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82602A64: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82602A68: 4BE643B9  bl 0x82466e20
	ctx.lr = 0x82602A6C;
	sub_82466E20(ctx, base);
	// 82602A6C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82602A70: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82602A74: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82602A78: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82602A80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82602A80 size=100
    let mut pc: u32 = 0x82602A80;
    'dispatch: loop {
        match pc {
            0x82602A80 => {
    //   block [0x82602A80..0x82602AE4)
	// 82602A80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82602A84: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82602A88: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82602A8C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82602A90: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82602A94: 38AA34DC  addi r5, r10, 0x34dc
	ctx.r[5].s64 = ctx.r[10].s64 + 13532;
	// 82602A98: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82602A9C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82602AA0: 388AC5B4  addi r4, r10, -0x3a4c
	ctx.r[4].s64 = ctx.r[10].s64 + -14924;
	// 82602AA4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82602AA8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82602AAC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82602AB0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82602AB4: 386A350C  addi r3, r10, 0x350c
	ctx.r[3].s64 = ctx.r[10].s64 + 13580;
	// 82602AB8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82602ABC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82602AC0: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82602AC4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82602AC8: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82602ACC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82602AD0: 4BE64351  bl 0x82466e20
	ctx.lr = 0x82602AD4;
	sub_82466E20(ctx, base);
	// 82602AD4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82602AD8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82602ADC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82602AE0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82602AE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82602AE8 size=100
    let mut pc: u32 = 0x82602AE8;
    'dispatch: loop {
        match pc {
            0x82602AE8 => {
    //   block [0x82602AE8..0x82602B4C)
	// 82602AE8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82602AEC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82602AF0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82602AF4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82602AF8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82602AFC: 38AA341C  addi r5, r10, 0x341c
	ctx.r[5].s64 = ctx.r[10].s64 + 13340;
	// 82602B00: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82602B04: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82602B08: 388AC5C8  addi r4, r10, -0x3a38
	ctx.r[4].s64 = ctx.r[10].s64 + -14904;
	// 82602B0C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82602B10: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82602B14: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82602B18: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82602B1C: 386A353C  addi r3, r10, 0x353c
	ctx.r[3].s64 = ctx.r[10].s64 + 13628;
	// 82602B20: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82602B24: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82602B28: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82602B2C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82602B30: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82602B34: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82602B38: 4BE642E9  bl 0x82466e20
	ctx.lr = 0x82602B3C;
	sub_82466E20(ctx, base);
	// 82602B3C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82602B40: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82602B44: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82602B48: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82602B50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82602B50 size=100
    let mut pc: u32 = 0x82602B50;
    'dispatch: loop {
        match pc {
            0x82602B50 => {
    //   block [0x82602B50..0x82602BB4)
	// 82602B50: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82602B54: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82602B58: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82602B5C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82602B60: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82602B64: 38AA344C  addi r5, r10, 0x344c
	ctx.r[5].s64 = ctx.r[10].s64 + 13388;
	// 82602B68: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82602B6C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82602B70: 388AC5D8  addi r4, r10, -0x3a28
	ctx.r[4].s64 = ctx.r[10].s64 + -14888;
	// 82602B74: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82602B78: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82602B7C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82602B80: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82602B84: 386A356C  addi r3, r10, 0x356c
	ctx.r[3].s64 = ctx.r[10].s64 + 13676;
	// 82602B88: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82602B8C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82602B90: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82602B94: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82602B98: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82602B9C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82602BA0: 4BE64281  bl 0x82466e20
	ctx.lr = 0x82602BA4;
	sub_82466E20(ctx, base);
	// 82602BA4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82602BA8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82602BAC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82602BB0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82602BB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82602BB8 size=100
    let mut pc: u32 = 0x82602BB8;
    'dispatch: loop {
        match pc {
            0x82602BB8 => {
    //   block [0x82602BB8..0x82602C1C)
	// 82602BB8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82602BBC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82602BC0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82602BC4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82602BC8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82602BCC: 38AA353C  addi r5, r10, 0x353c
	ctx.r[5].s64 = ctx.r[10].s64 + 13628;
	// 82602BD0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82602BD4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82602BD8: 388AC5F0  addi r4, r10, -0x3a10
	ctx.r[4].s64 = ctx.r[10].s64 + -14864;
	// 82602BDC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82602BE0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82602BE4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82602BE8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82602BEC: 386A359C  addi r3, r10, 0x359c
	ctx.r[3].s64 = ctx.r[10].s64 + 13724;
	// 82602BF0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82602BF4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82602BF8: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82602BFC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82602C00: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82602C04: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82602C08: 4BE64219  bl 0x82466e20
	ctx.lr = 0x82602C0C;
	sub_82466E20(ctx, base);
	// 82602C0C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82602C10: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82602C14: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82602C18: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82602C20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82602C20 size=100
    let mut pc: u32 = 0x82602C20;
    'dispatch: loop {
        match pc {
            0x82602C20 => {
    //   block [0x82602C20..0x82602C84)
	// 82602C20: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82602C24: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82602C28: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82602C2C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82602C30: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82602C34: 38AA344C  addi r5, r10, 0x344c
	ctx.r[5].s64 = ctx.r[10].s64 + 13388;
	// 82602C38: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82602C3C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82602C40: 388AC60C  addi r4, r10, -0x39f4
	ctx.r[4].s64 = ctx.r[10].s64 + -14836;
	// 82602C44: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82602C48: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82602C4C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82602C50: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82602C54: 386A35CC  addi r3, r10, 0x35cc
	ctx.r[3].s64 = ctx.r[10].s64 + 13772;
	// 82602C58: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82602C5C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82602C60: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82602C64: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82602C68: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82602C6C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82602C70: 4BE641B1  bl 0x82466e20
	ctx.lr = 0x82602C74;
	sub_82466E20(ctx, base);
	// 82602C74: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82602C78: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82602C7C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82602C80: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82602C88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82602C88 size=112
    let mut pc: u32 = 0x82602C88;
    'dispatch: loop {
        match pc {
            0x82602C88 => {
    //   block [0x82602C88..0x82602CF8)
	// 82602C88: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82602C8C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82602C90: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82602C94: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82602C98: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 82602C9C: 38AA365C  addi r5, r10, 0x365c
	ctx.r[5].s64 = ctx.r[10].s64 + 13916;
	// 82602CA0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82602CA4: 390B6AE0  addi r8, r11, 0x6ae0
	ctx.r[8].s64 = ctx.r[11].s64 + 27360;
	// 82602CA8: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 82602CAC: 388AC620  addi r4, r10, -0x39e0
	ctx.r[4].s64 = ctx.r[10].s64 + -14816;
	// 82602CB0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82602CB4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82602CB8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82602CBC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82602CC0: 386A35FC  addi r3, r10, 0x35fc
	ctx.r[3].s64 = ctx.r[10].s64 + 13820;
	// 82602CC4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82602CC8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82602CCC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82602CD0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82602CD4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82602CD8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82602CDC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82602CE0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82602CE4: 4BE6413D  bl 0x82466e20
	ctx.lr = 0x82602CE8;
	sub_82466E20(ctx, base);
	// 82602CE8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82602CEC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82602CF0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82602CF4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82602CF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82602CF8 size=112
    let mut pc: u32 = 0x82602CF8;
    'dispatch: loop {
        match pc {
            0x82602CF8 => {
    //   block [0x82602CF8..0x82602D68)
	// 82602CF8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82602CFC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82602D00: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82602D04: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82602D08: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 82602D0C: 38AA368C  addi r5, r10, 0x368c
	ctx.r[5].s64 = ctx.r[10].s64 + 13964;
	// 82602D10: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82602D14: 390B6B10  addi r8, r11, 0x6b10
	ctx.r[8].s64 = ctx.r[11].s64 + 27408;
	// 82602D18: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82602D1C: 388AC630  addi r4, r10, -0x39d0
	ctx.r[4].s64 = ctx.r[10].s64 + -14800;
	// 82602D20: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82602D24: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82602D28: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82602D2C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82602D30: 386A362C  addi r3, r10, 0x362c
	ctx.r[3].s64 = ctx.r[10].s64 + 13868;
	// 82602D34: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82602D38: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82602D3C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82602D40: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82602D44: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82602D48: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82602D4C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82602D50: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82602D54: 4BE640CD  bl 0x82466e20
	ctx.lr = 0x82602D58;
	sub_82466E20(ctx, base);
	// 82602D58: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82602D5C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82602D60: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82602D64: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82602D68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82602D68 size=112
    let mut pc: u32 = 0x82602D68;
    'dispatch: loop {
        match pc {
            0x82602D68 => {
    //   block [0x82602D68..0x82602DD8)
	// 82602D68: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82602D6C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82602D70: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82602D74: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82602D78: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 82602D7C: 38AA37AC  addi r5, r10, 0x37ac
	ctx.r[5].s64 = ctx.r[10].s64 + 14252;
	// 82602D80: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82602D84: 390B6B28  addi r8, r11, 0x6b28
	ctx.r[8].s64 = ctx.r[11].s64 + 27432;
	// 82602D88: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 82602D8C: 388AC648  addi r4, r10, -0x39b8
	ctx.r[4].s64 = ctx.r[10].s64 + -14776;
	// 82602D90: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82602D94: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82602D98: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82602D9C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82602DA0: 386A365C  addi r3, r10, 0x365c
	ctx.r[3].s64 = ctx.r[10].s64 + 13916;
	// 82602DA4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82602DA8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82602DAC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82602DB0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82602DB4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82602DB8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82602DBC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82602DC0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82602DC4: 4BE6405D  bl 0x82466e20
	ctx.lr = 0x82602DC8;
	sub_82466E20(ctx, base);
	// 82602DC8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82602DCC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82602DD0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82602DD4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82602DD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82602DD8 size=112
    let mut pc: u32 = 0x82602DD8;
    'dispatch: loop {
        match pc {
            0x82602DD8 => {
    //   block [0x82602DD8..0x82602E48)
	// 82602DD8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82602DDC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82602DE0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82602DE4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82602DE8: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 82602DEC: 38AA365C  addi r5, r10, 0x365c
	ctx.r[5].s64 = ctx.r[10].s64 + 13916;
	// 82602DF0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82602DF4: 390B6B58  addi r8, r11, 0x6b58
	ctx.r[8].s64 = ctx.r[11].s64 + 27480;
	// 82602DF8: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82602DFC: 388AC654  addi r4, r10, -0x39ac
	ctx.r[4].s64 = ctx.r[10].s64 + -14764;
	// 82602E00: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82602E04: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82602E08: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82602E0C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82602E10: 386A368C  addi r3, r10, 0x368c
	ctx.r[3].s64 = ctx.r[10].s64 + 13964;
	// 82602E14: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82602E18: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82602E1C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82602E20: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82602E24: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82602E28: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82602E2C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82602E30: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82602E34: 4BE63FED  bl 0x82466e20
	ctx.lr = 0x82602E38;
	sub_82466E20(ctx, base);
	// 82602E38: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82602E3C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82602E40: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82602E44: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82602E48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82602E48 size=108
    let mut pc: u32 = 0x82602E48;
    'dispatch: loop {
        match pc {
            0x82602E48 => {
    //   block [0x82602E48..0x82602EB4)
	// 82602E48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82602E4C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82602E50: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82602E54: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 82602E58: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82602E5C: 38EB6B70  addi r7, r11, 0x6b70
	ctx.r[7].s64 = ctx.r[11].s64 + 27504;
	// 82602E60: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 82602E64: 388AC664  addi r4, r10, -0x399c
	ctx.r[4].s64 = ctx.r[10].s64 + -14748;
	// 82602E68: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82602E6C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82602E70: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82602E74: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82602E78: 386A36BC  addi r3, r10, 0x36bc
	ctx.r[3].s64 = ctx.r[10].s64 + 14012;
	// 82602E7C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82602E80: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82602E84: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82602E88: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82602E8C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82602E90: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82602E94: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82602E98: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82602E9C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82602EA0: 4BE63F81  bl 0x82466e20
	ctx.lr = 0x82602EA4;
	sub_82466E20(ctx, base);
	// 82602EA4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82602EA8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82602EAC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82602EB0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82602EB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82602EB8 size=112
    let mut pc: u32 = 0x82602EB8;
    'dispatch: loop {
        match pc {
            0x82602EB8 => {
    //   block [0x82602EB8..0x82602F28)
	// 82602EB8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82602EBC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82602EC0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82602EC4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82602EC8: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 82602ECC: 38AA368C  addi r5, r10, 0x368c
	ctx.r[5].s64 = ctx.r[10].s64 + 13964;
	// 82602ED0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82602ED4: 390B6B88  addi r8, r11, 0x6b88
	ctx.r[8].s64 = ctx.r[11].s64 + 27528;
	// 82602ED8: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82602EDC: 388AC68C  addi r4, r10, -0x3974
	ctx.r[4].s64 = ctx.r[10].s64 + -14708;
	// 82602EE0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82602EE4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82602EE8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82602EEC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82602EF0: 386A36EC  addi r3, r10, 0x36ec
	ctx.r[3].s64 = ctx.r[10].s64 + 14060;
	// 82602EF4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82602EF8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82602EFC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82602F00: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82602F04: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82602F08: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82602F0C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82602F10: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82602F14: 4BE63F0D  bl 0x82466e20
	ctx.lr = 0x82602F18;
	sub_82466E20(ctx, base);
	// 82602F18: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82602F1C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82602F20: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82602F24: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82602F28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82602F28 size=116
    let mut pc: u32 = 0x82602F28;
    'dispatch: loop {
        match pc {
            0x82602F28 => {
    //   block [0x82602F28..0x82602F9C)
	// 82602F28: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82602F2C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82602F30: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82602F34: 3D408274  lis r10, -0x7d8c
	ctx.r[10].s64 = -2106327040;
	// 82602F38: 38E00007  li r7, 7
	ctx.r[7].s64 = 7;
	// 82602F3C: 390A6BA0  addi r8, r10, 0x6ba0
	ctx.r[8].s64 = ctx.r[10].s64 + 27552;
	// 82602F40: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82602F44: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82602F48: 38AA09BC  addi r5, r10, 0x9bc
	ctx.r[5].s64 = ctx.r[10].s64 + 2492;
	// 82602F4C: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82602F50: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82602F54: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82602F58: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82602F5C: 388AC6A4  addi r4, r10, -0x395c
	ctx.r[4].s64 = ctx.r[10].s64 + -14684;
	// 82602F60: 396B98EC  addi r11, r11, -0x6714
	ctx.r[11].s64 = ctx.r[11].s64 + -26388;
	// 82602F64: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82602F68: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82602F6C: 386A371C  addi r3, r10, 0x371c
	ctx.r[3].s64 = ctx.r[10].s64 + 14108;
	// 82602F70: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 82602F74: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82602F78: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 82602F7C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82602F80: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82602F84: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82602F88: 4BE63E99  bl 0x82466e20
	ctx.lr = 0x82602F8C;
	sub_82466E20(ctx, base);
	// 82602F8C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82602F90: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82602F94: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82602F98: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82602FA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82602FA0 size=116
    let mut pc: u32 = 0x82602FA0;
    'dispatch: loop {
        match pc {
            0x82602FA0 => {
    //   block [0x82602FA0..0x82603014)
	// 82602FA0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82602FA4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82602FA8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82602FAC: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82602FB0: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
	// 82602FB4: 392A9A14  addi r9, r10, -0x65ec
	ctx.r[9].s64 = ctx.r[10].s64 + -26092;
	// 82602FB8: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82602FBC: 38C00043  li r6, 0x43
	ctx.r[6].s64 = 67;
	// 82602FC0: 38AA09BC  addi r5, r10, 0x9bc
	ctx.r[5].s64 = ctx.r[10].s64 + 2492;
	// 82602FC4: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82602FC8: 90E10074  stw r7, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[7].u32 ) };
	// 82602FCC: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 82602FD0: 388AC6B8  addi r4, r10, -0x3948
	ctx.r[4].s64 = ctx.r[10].s64 + -14664;
	// 82602FD4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82602FD8: 90C1005C  stw r6, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[6].u32 ) };
	// 82602FDC: 396B6C68  addi r11, r11, 0x6c68
	ctx.r[11].s64 = ctx.r[11].s64 + 27752;
	// 82602FE0: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82602FE4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82602FE8: 386A374C  addi r3, r10, 0x374c
	ctx.r[3].s64 = ctx.r[10].s64 + 14156;
	// 82602FEC: 39400002  li r10, 2
	ctx.r[10].s64 = 2;
	// 82602FF0: 9101006C  stw r8, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[8].u32 ) };
	// 82602FF4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82602FF8: 91010064  stw r8, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[8].u32 ) };
	// 82602FFC: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82603000: 4BE63E21  bl 0x82466e20
	ctx.lr = 0x82603004;
	sub_82466E20(ctx, base);
	// 82603004: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82603008: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8260300C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82603010: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82603018(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82603018 size=48
    let mut pc: u32 = 0x82603018;
    'dispatch: loop {
        match pc {
            0x82603018 => {
    //   block [0x82603018..0x82603048)
	// 82603018: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 8260301C: 814B72B8  lwz r10, 0x72b8(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(29368 as u32) ) } as u64;
	// 82603020: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 82603024: 396BB4F0  addi r11, r11, -0x4b10
	ctx.r[11].s64 = ctx.r[11].s64 + -19216;
	// 82603028: 914B0050  stw r10, 0x50(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 8260302C: 3D408274  lis r10, -0x7d8c
	ctx.r[10].s64 = -2106327040;
	// 82603030: 814A72B4  lwz r10, 0x72b4(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(29364 as u32) ) } as u64;
	// 82603034: 914B0140  stw r10, 0x140(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(320 as u32), ctx.r[10].u32 ) };
	// 82603038: 3D408274  lis r10, -0x7d8c
	ctx.r[10].s64 = -2106327040;
	// 8260303C: 814A72B0  lwz r10, 0x72b0(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(29360 as u32) ) } as u64;
	// 82603040: 914B0338  stw r10, 0x338(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(824 as u32), ctx.r[10].u32 ) };
	// 82603044: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82603048(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82603048 size=116
    let mut pc: u32 = 0x82603048;
    'dispatch: loop {
        match pc {
            0x82603048 => {
    //   block [0x82603048..0x826030BC)
	// 82603048: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260304C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82603050: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82603054: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82603058: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 8260305C: 392B9AF0  addi r9, r11, -0x6510
	ctx.r[9].s64 = ctx.r[11].s64 + -25872;
	// 82603060: 38AA09BC  addi r5, r10, 0x9bc
	ctx.r[5].s64 = ctx.r[10].s64 + 2492;
	// 82603064: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82603068: 38E90050  addi r7, r9, 0x50
	ctx.r[7].s64 = ctx.r[9].s64 + 80;
	// 8260306C: 38C00027  li r6, 0x27
	ctx.r[6].s64 = 39;
	// 82603070: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 82603074: 388AC6C4  addi r4, r10, -0x393c
	ctx.r[4].s64 = ctx.r[10].s64 + -14652;
	// 82603078: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8260307C: 396BB4F0  addi r11, r11, -0x4b10
	ctx.r[11].s64 = ctx.r[11].s64 + -19216;
	// 82603080: 90E10064  stw r7, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[7].u32 ) };
	// 82603084: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82603088: 90C1005C  stw r6, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[6].u32 ) };
	// 8260308C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82603090: 386A377C  addi r3, r10, 0x377c
	ctx.r[3].s64 = ctx.r[10].s64 + 14204;
	// 82603094: 39400004  li r10, 4
	ctx.r[10].s64 = 4;
	// 82603098: 91010074  stw r8, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[8].u32 ) };
	// 8260309C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826030A0: 9101006C  stw r8, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[8].u32 ) };
	// 826030A4: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826030A8: 4BE63D79  bl 0x82466e20
	ctx.lr = 0x826030AC;
	sub_82466E20(ctx, base);
	// 826030AC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826030B0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826030B4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826030B8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826030C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826030C0 size=116
    let mut pc: u32 = 0x826030C0;
    'dispatch: loop {
        match pc {
            0x826030C0 => {
    //   block [0x826030C0..0x82603134)
	// 826030C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826030C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826030C8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826030CC: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 826030D0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826030D4: 390B72C8  addi r8, r11, 0x72c8
	ctx.r[8].s64 = ctx.r[11].s64 + 29384;
	// 826030D8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826030DC: 392A9C8C  addi r9, r10, -0x6374
	ctx.r[9].s64 = ctx.r[10].s64 + -25460;
	// 826030E0: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 826030E4: 38E00006  li r7, 6
	ctx.r[7].s64 = 6;
	// 826030E8: 38AA09BC  addi r5, r10, 0x9bc
	ctx.r[5].s64 = ctx.r[10].s64 + 2492;
	// 826030EC: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826030F0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826030F4: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826030F8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826030FC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82603100: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82603104: 3D608293  lis r11, -0x7d6d
	ctx.r[11].s64 = -2104295424;
	// 82603108: 388AC6D4  addi r4, r10, -0x392c
	ctx.r[4].s64 = ctx.r[10].s64 + -14636;
	// 8260310C: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82603110: 386B37AC  addi r3, r11, 0x37ac
	ctx.r[3].s64 = ctx.r[11].s64 + 14252;
	// 82603114: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82603118: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8260311C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82603120: 4BE63D01  bl 0x82466e20
	ctx.lr = 0x82603124;
	sub_82466E20(ctx, base);
	// 82603124: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82603128: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8260312C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82603130: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82603138(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82603138 size=112
    let mut pc: u32 = 0x82603138;
    'dispatch: loop {
        match pc {
            0x82603138 => {
    //   block [0x82603138..0x826031A8)
	// 82603138: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260313C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82603140: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82603144: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82603148: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 8260314C: 38AA09BC  addi r5, r10, 0x9bc
	ctx.r[5].s64 = ctx.r[10].s64 + 2492;
	// 82603150: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82603154: 390B7358  addi r8, r11, 0x7358
	ctx.r[8].s64 = ctx.r[11].s64 + 29528;
	// 82603158: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8260315C: 388AC6E4  addi r4, r10, -0x391c
	ctx.r[4].s64 = ctx.r[10].s64 + -14620;
	// 82603160: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82603164: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82603168: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8260316C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82603170: 386A37DC  addi r3, r10, 0x37dc
	ctx.r[3].s64 = ctx.r[10].s64 + 14300;
	// 82603174: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82603178: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8260317C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82603180: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82603184: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82603188: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8260318C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82603190: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82603194: 4BE63C8D  bl 0x82466e20
	ctx.lr = 0x82603198;
	sub_82466E20(ctx, base);
	// 82603198: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8260319C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826031A0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826031A4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826031A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826031A8 size=36
    let mut pc: u32 = 0x826031A8;
    'dispatch: loop {
        match pc {
            0x826031A8 => {
    //   block [0x826031A8..0x826031CC)
	// 826031A8: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 826031AC: 814B7374  lwz r10, 0x7374(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(29556 as u32) ) } as u64;
	// 826031B0: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 826031B4: 396BB898  addi r11, r11, -0x4768
	ctx.r[11].s64 = ctx.r[11].s64 + -18280;
	// 826031B8: 914B0020  stw r10, 0x20(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(32 as u32), ctx.r[10].u32 ) };
	// 826031BC: 3D408274  lis r10, -0x7d8c
	ctx.r[10].s64 = -2106327040;
	// 826031C0: 814A72C4  lwz r10, 0x72c4(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(29380 as u32) ) } as u64;
	// 826031C4: 914B00B0  stw r10, 0xb0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(176 as u32), ctx.r[10].u32 ) };
	// 826031C8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826031D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826031D0 size=116
    let mut pc: u32 = 0x826031D0;
    'dispatch: loop {
        match pc {
            0x826031D0 => {
    //   block [0x826031D0..0x82603244)
	// 826031D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826031D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826031D8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826031DC: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826031E0: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
	// 826031E4: 392A9CF4  addi r9, r10, -0x630c
	ctx.r[9].s64 = ctx.r[10].s64 + -25356;
	// 826031E8: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 826031EC: 38C00008  li r6, 8
	ctx.r[6].s64 = 8;
	// 826031F0: 38AA09BC  addi r5, r10, 0x9bc
	ctx.r[5].s64 = ctx.r[10].s64 + 2492;
	// 826031F4: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826031F8: 90E10074  stw r7, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[7].u32 ) };
	// 826031FC: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 82603200: 388AC75C  addi r4, r10, -0x38a4
	ctx.r[4].s64 = ctx.r[10].s64 + -14500;
	// 82603204: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82603208: 90C1005C  stw r6, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[6].u32 ) };
	// 8260320C: 396BB898  addi r11, r11, -0x4768
	ctx.r[11].s64 = ctx.r[11].s64 + -18280;
	// 82603210: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82603214: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82603218: 386A380C  addi r3, r10, 0x380c
	ctx.r[3].s64 = ctx.r[10].s64 + 14348;
	// 8260321C: 39400002  li r10, 2
	ctx.r[10].s64 = 2;
	// 82603220: 9101006C  stw r8, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[8].u32 ) };
	// 82603224: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82603228: 91010064  stw r8, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[8].u32 ) };
	// 8260322C: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82603230: 4BE63BF1  bl 0x82466e20
	ctx.lr = 0x82603234;
	sub_82466E20(ctx, base);
	// 82603234: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82603238: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8260323C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82603240: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82603248(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82603248 size=108
    let mut pc: u32 = 0x82603248;
    'dispatch: loop {
        match pc {
            0x82603248 => {
    //   block [0x82603248..0x826032B4)
	// 82603248: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260324C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82603250: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82603254: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 82603258: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8260325C: 38EB7378  addi r7, r11, 0x7378
	ctx.r[7].s64 = ctx.r[11].s64 + 29560;
	// 82603260: 39000005  li r8, 5
	ctx.r[8].s64 = 5;
	// 82603264: 388AC76C  addi r4, r10, -0x3894
	ctx.r[4].s64 = ctx.r[10].s64 + -14484;
	// 82603268: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8260326C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82603270: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82603274: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82603278: 386A383C  addi r3, r10, 0x383c
	ctx.r[3].s64 = ctx.r[10].s64 + 14396;
	// 8260327C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82603280: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82603284: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82603288: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8260328C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82603290: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82603294: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82603298: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8260329C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826032A0: 4BE63B81  bl 0x82466e20
	ctx.lr = 0x826032A4;
	sub_82466E20(ctx, base);
	// 826032A4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826032A8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826032AC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826032B0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826032B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826032B8 size=112
    let mut pc: u32 = 0x826032B8;
    'dispatch: loop {
        match pc {
            0x826032B8 => {
    //   block [0x826032B8..0x82603328)
	// 826032B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826032BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826032C0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826032C4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 826032C8: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 826032CC: 38AA155C  addi r5, r10, 0x155c
	ctx.r[5].s64 = ctx.r[10].s64 + 5468;
	// 826032D0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826032D4: 390B73F0  addi r8, r11, 0x73f0
	ctx.r[8].s64 = ctx.r[11].s64 + 29680;
	// 826032D8: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826032DC: 388AC780  addi r4, r10, -0x3880
	ctx.r[4].s64 = ctx.r[10].s64 + -14464;
	// 826032E0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826032E4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 826032E8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826032EC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826032F0: 386A386C  addi r3, r10, 0x386c
	ctx.r[3].s64 = ctx.r[10].s64 + 14444;
	// 826032F4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826032F8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826032FC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82603300: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82603304: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82603308: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8260330C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82603310: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82603314: 4BE63B0D  bl 0x82466e20
	ctx.lr = 0x82603318;
	sub_82466E20(ctx, base);
	// 82603318: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8260331C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82603320: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82603324: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82603328(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82603328 size=108
    let mut pc: u32 = 0x82603328;
    'dispatch: loop {
        match pc {
            0x82603328 => {
    //   block [0x82603328..0x82603394)
	// 82603328: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260332C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82603330: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82603334: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 82603338: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8260333C: 38EB7408  addi r7, r11, 0x7408
	ctx.r[7].s64 = ctx.r[11].s64 + 29704;
	// 82603340: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 82603344: 388AC794  addi r4, r10, -0x386c
	ctx.r[4].s64 = ctx.r[10].s64 + -14444;
	// 82603348: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8260334C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82603350: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82603354: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82603358: 386A389C  addi r3, r10, 0x389c
	ctx.r[3].s64 = ctx.r[10].s64 + 14492;
	// 8260335C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82603360: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82603364: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82603368: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8260336C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82603370: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82603374: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82603378: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8260337C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82603380: 4BE63AA1  bl 0x82466e20
	ctx.lr = 0x82603384;
	sub_82466E20(ctx, base);
	// 82603384: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82603388: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8260338C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82603390: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82603398(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82603398 size=112
    let mut pc: u32 = 0x82603398;
    'dispatch: loop {
        match pc {
            0x82603398 => {
    //   block [0x82603398..0x82603408)
	// 82603398: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260339C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826033A0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826033A4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 826033A8: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 826033AC: 38AA09BC  addi r5, r10, 0x9bc
	ctx.r[5].s64 = ctx.r[10].s64 + 2492;
	// 826033B0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826033B4: 390B7420  addi r8, r11, 0x7420
	ctx.r[8].s64 = ctx.r[11].s64 + 29728;
	// 826033B8: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 826033BC: 388AC7A8  addi r4, r10, -0x3858
	ctx.r[4].s64 = ctx.r[10].s64 + -14424;
	// 826033C0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826033C4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 826033C8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826033CC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826033D0: 386A38CC  addi r3, r10, 0x38cc
	ctx.r[3].s64 = ctx.r[10].s64 + 14540;
	// 826033D4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826033D8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826033DC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826033E0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826033E4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826033E8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826033EC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826033F0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826033F4: 4BE63A2D  bl 0x82466e20
	ctx.lr = 0x826033F8;
	sub_82466E20(ctx, base);
	// 826033F8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826033FC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82603400: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82603404: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82603408(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82603408 size=108
    let mut pc: u32 = 0x82603408;
    'dispatch: loop {
        match pc {
            0x82603408 => {
    //   block [0x82603408..0x82603474)
	// 82603408: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260340C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82603410: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82603414: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 82603418: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8260341C: 38EB7468  addi r7, r11, 0x7468
	ctx.r[7].s64 = ctx.r[11].s64 + 29800;
	// 82603420: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 82603424: 388AC7C4  addi r4, r10, -0x383c
	ctx.r[4].s64 = ctx.r[10].s64 + -14396;
	// 82603428: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8260342C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82603430: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82603434: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82603438: 386A38FC  addi r3, r10, 0x38fc
	ctx.r[3].s64 = ctx.r[10].s64 + 14588;
	// 8260343C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82603440: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82603444: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82603448: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8260344C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82603450: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82603454: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82603458: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8260345C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82603460: 4BE639C1  bl 0x82466e20
	ctx.lr = 0x82603464;
	sub_82466E20(ctx, base);
	// 82603464: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82603468: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8260346C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82603470: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82603478(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82603478 size=108
    let mut pc: u32 = 0x82603478;
    'dispatch: loop {
        match pc {
            0x82603478 => {
    //   block [0x82603478..0x826034E4)
	// 82603478: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260347C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82603480: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82603484: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 82603488: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8260348C: 38EB7498  addi r7, r11, 0x7498
	ctx.r[7].s64 = ctx.r[11].s64 + 29848;
	// 82603490: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 82603494: 388AC7E4  addi r4, r10, -0x381c
	ctx.r[4].s64 = ctx.r[10].s64 + -14364;
	// 82603498: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8260349C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 826034A0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826034A4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826034A8: 386A392C  addi r3, r10, 0x392c
	ctx.r[3].s64 = ctx.r[10].s64 + 14636;
	// 826034AC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826034B0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826034B4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826034B8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826034BC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826034C0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826034C4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826034C8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826034CC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826034D0: 4BE63951  bl 0x82466e20
	ctx.lr = 0x826034D4;
	sub_82466E20(ctx, base);
	// 826034D4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826034D8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826034DC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826034E0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826034E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826034E8 size=112
    let mut pc: u32 = 0x826034E8;
    'dispatch: loop {
        match pc {
            0x826034E8 => {
    //   block [0x826034E8..0x82603558)
	// 826034E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826034EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826034F0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826034F4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 826034F8: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 826034FC: 38AA09BC  addi r5, r10, 0x9bc
	ctx.r[5].s64 = ctx.r[10].s64 + 2492;
	// 82603500: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82603504: 390B74B0  addi r8, r11, 0x74b0
	ctx.r[8].s64 = ctx.r[11].s64 + 29872;
	// 82603508: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 8260350C: 388AC7F8  addi r4, r10, -0x3808
	ctx.r[4].s64 = ctx.r[10].s64 + -14344;
	// 82603510: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82603514: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82603518: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8260351C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82603520: 386A395C  addi r3, r10, 0x395c
	ctx.r[3].s64 = ctx.r[10].s64 + 14684;
	// 82603524: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82603528: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8260352C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82603530: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82603534: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82603538: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8260353C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82603540: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82603544: 4BE638DD  bl 0x82466e20
	ctx.lr = 0x82603548;
	sub_82466E20(ctx, base);
	// 82603548: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8260354C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82603550: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82603554: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82603558(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82603558 size=112
    let mut pc: u32 = 0x82603558;
    'dispatch: loop {
        match pc {
            0x82603558 => {
    //   block [0x82603558..0x826035C8)
	// 82603558: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260355C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82603560: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82603564: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82603568: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 8260356C: 38AA281C  addi r5, r10, 0x281c
	ctx.r[5].s64 = ctx.r[10].s64 + 10268;
	// 82603570: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82603574: 390B74E0  addi r8, r11, 0x74e0
	ctx.r[8].s64 = ctx.r[11].s64 + 29920;
	// 82603578: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 8260357C: 388AC804  addi r4, r10, -0x37fc
	ctx.r[4].s64 = ctx.r[10].s64 + -14332;
	// 82603580: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82603584: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82603588: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8260358C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82603590: 386A398C  addi r3, r10, 0x398c
	ctx.r[3].s64 = ctx.r[10].s64 + 14732;
	// 82603594: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82603598: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8260359C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826035A0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826035A4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826035A8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826035AC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826035B0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826035B4: 4BE6386D  bl 0x82466e20
	ctx.lr = 0x826035B8;
	sub_82466E20(ctx, base);
	// 826035B8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826035BC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826035C0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826035C4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826035C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826035C8 size=112
    let mut pc: u32 = 0x826035C8;
    'dispatch: loop {
        match pc {
            0x826035C8 => {
    //   block [0x826035C8..0x82603638)
	// 826035C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826035CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826035D0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826035D4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 826035D8: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 826035DC: 38AA281C  addi r5, r10, 0x281c
	ctx.r[5].s64 = ctx.r[10].s64 + 10268;
	// 826035E0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826035E4: 390B7528  addi r8, r11, 0x7528
	ctx.r[8].s64 = ctx.r[11].s64 + 29992;
	// 826035E8: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 826035EC: 388AC81C  addi r4, r10, -0x37e4
	ctx.r[4].s64 = ctx.r[10].s64 + -14308;
	// 826035F0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826035F4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 826035F8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826035FC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82603600: 386A39BC  addi r3, r10, 0x39bc
	ctx.r[3].s64 = ctx.r[10].s64 + 14780;
	// 82603604: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82603608: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8260360C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82603610: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82603614: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82603618: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8260361C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82603620: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82603624: 4BE637FD  bl 0x82466e20
	ctx.lr = 0x82603628;
	sub_82466E20(ctx, base);
	// 82603628: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8260362C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82603630: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82603634: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82603638(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82603638 size=112
    let mut pc: u32 = 0x82603638;
    'dispatch: loop {
        match pc {
            0x82603638 => {
    //   block [0x82603638..0x826036A8)
	// 82603638: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260363C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82603640: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82603644: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82603648: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 8260364C: 38AA284C  addi r5, r10, 0x284c
	ctx.r[5].s64 = ctx.r[10].s64 + 10316;
	// 82603650: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82603654: 390B7588  addi r8, r11, 0x7588
	ctx.r[8].s64 = ctx.r[11].s64 + 30088;
	// 82603658: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 8260365C: 388AC830  addi r4, r10, -0x37d0
	ctx.r[4].s64 = ctx.r[10].s64 + -14288;
	// 82603660: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82603664: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82603668: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8260366C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82603670: 386A39EC  addi r3, r10, 0x39ec
	ctx.r[3].s64 = ctx.r[10].s64 + 14828;
	// 82603674: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82603678: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8260367C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82603680: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82603684: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82603688: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8260368C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82603690: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82603694: 4BE6378D  bl 0x82466e20
	ctx.lr = 0x82603698;
	sub_82466E20(ctx, base);
	// 82603698: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8260369C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826036A0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826036A4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826036A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826036A8 size=112
    let mut pc: u32 = 0x826036A8;
    'dispatch: loop {
        match pc {
            0x826036A8 => {
    //   block [0x826036A8..0x82603718)
	// 826036A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826036AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826036B0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826036B4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 826036B8: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 826036BC: 38AA284C  addi r5, r10, 0x284c
	ctx.r[5].s64 = ctx.r[10].s64 + 10316;
	// 826036C0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826036C4: 390B75E8  addi r8, r11, 0x75e8
	ctx.r[8].s64 = ctx.r[11].s64 + 30184;
	// 826036C8: 39200008  li r9, 8
	ctx.r[9].s64 = 8;
	// 826036CC: 388AC840  addi r4, r10, -0x37c0
	ctx.r[4].s64 = ctx.r[10].s64 + -14272;
	// 826036D0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826036D4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 826036D8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826036DC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826036E0: 386A3A1C  addi r3, r10, 0x3a1c
	ctx.r[3].s64 = ctx.r[10].s64 + 14876;
	// 826036E4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826036E8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826036EC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826036F0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826036F4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826036F8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826036FC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82603700: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82603704: 4BE6371D  bl 0x82466e20
	ctx.lr = 0x82603708;
	sub_82466E20(ctx, base);
	// 82603708: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8260370C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82603710: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82603714: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82603718(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82603718 size=112
    let mut pc: u32 = 0x82603718;
    'dispatch: loop {
        match pc {
            0x82603718 => {
    //   block [0x82603718..0x82603788)
	// 82603718: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260371C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82603720: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82603724: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82603728: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 8260372C: 38AA284C  addi r5, r10, 0x284c
	ctx.r[5].s64 = ctx.r[10].s64 + 10316;
	// 82603730: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82603734: 390B76A8  addi r8, r11, 0x76a8
	ctx.r[8].s64 = ctx.r[11].s64 + 30376;
	// 82603738: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 8260373C: 388AC858  addi r4, r10, -0x37a8
	ctx.r[4].s64 = ctx.r[10].s64 + -14248;
	// 82603740: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82603744: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82603748: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8260374C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82603750: 386A3A4C  addi r3, r10, 0x3a4c
	ctx.r[3].s64 = ctx.r[10].s64 + 14924;
	// 82603754: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82603758: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8260375C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82603760: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82603764: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82603768: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8260376C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82603770: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82603774: 4BE636AD  bl 0x82466e20
	ctx.lr = 0x82603778;
	sub_82466E20(ctx, base);
	// 82603778: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8260377C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82603780: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82603784: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82603788(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82603788 size=112
    let mut pc: u32 = 0x82603788;
    'dispatch: loop {
        match pc {
            0x82603788 => {
    //   block [0x82603788..0x826037F8)
	// 82603788: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260378C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82603790: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82603794: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82603798: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 8260379C: 38AA281C  addi r5, r10, 0x281c
	ctx.r[5].s64 = ctx.r[10].s64 + 10268;
	// 826037A0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826037A4: 390B7708  addi r8, r11, 0x7708
	ctx.r[8].s64 = ctx.r[11].s64 + 30472;
	// 826037A8: 39200008  li r9, 8
	ctx.r[9].s64 = 8;
	// 826037AC: 388AC86C  addi r4, r10, -0x3794
	ctx.r[4].s64 = ctx.r[10].s64 + -14228;
	// 826037B0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826037B4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 826037B8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826037BC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826037C0: 386A3A7C  addi r3, r10, 0x3a7c
	ctx.r[3].s64 = ctx.r[10].s64 + 14972;
	// 826037C4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826037C8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826037CC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826037D0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826037D4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826037D8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826037DC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826037E0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826037E4: 4BE6363D  bl 0x82466e20
	ctx.lr = 0x826037E8;
	sub_82466E20(ctx, base);
	// 826037E8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826037EC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826037F0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826037F4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826037F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826037F8 size=112
    let mut pc: u32 = 0x826037F8;
    'dispatch: loop {
        match pc {
            0x826037F8 => {
    //   block [0x826037F8..0x82603868)
	// 826037F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826037FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82603800: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82603804: 3D408274  lis r10, -0x7d8c
	ctx.r[10].s64 = -2106327040;
	// 82603808: 39000013  li r8, 0x13
	ctx.r[8].s64 = 19;
	// 8260380C: 38EA77C8  addi r7, r10, 0x77c8
	ctx.r[7].s64 = ctx.r[10].s64 + 30664;
	// 82603810: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82603814: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82603818: 388AC87C  addi r4, r10, -0x3784
	ctx.r[4].s64 = ctx.r[10].s64 + -14212;
	// 8260381C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82603820: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82603824: 396B9D20  addi r11, r11, -0x62e0
	ctx.r[11].s64 = ctx.r[11].s64 + -25312;
	// 82603828: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8260382C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82603830: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82603834: 386A3AAC  addi r3, r10, 0x3aac
	ctx.r[3].s64 = ctx.r[10].s64 + 15020;
	// 82603838: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8260383C: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 82603840: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82603844: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 82603848: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8260384C: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82603850: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82603854: 4BE635CD  bl 0x82466e20
	ctx.lr = 0x82603858;
	sub_82466E20(ctx, base);
	// 82603858: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8260385C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82603860: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82603864: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82603868(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82603868 size=112
    let mut pc: u32 = 0x82603868;
    'dispatch: loop {
        match pc {
            0x82603868 => {
    //   block [0x82603868..0x826038D8)
	// 82603868: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260386C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82603870: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82603874: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82603878: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 8260387C: 38AA15EC  addi r5, r10, 0x15ec
	ctx.r[5].s64 = ctx.r[10].s64 + 5612;
	// 82603880: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82603884: 390B7990  addi r8, r11, 0x7990
	ctx.r[8].s64 = ctx.r[11].s64 + 31120;
	// 82603888: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8260388C: 388AC894  addi r4, r10, -0x376c
	ctx.r[4].s64 = ctx.r[10].s64 + -14188;
	// 82603890: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82603894: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82603898: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8260389C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826038A0: 386A3ADC  addi r3, r10, 0x3adc
	ctx.r[3].s64 = ctx.r[10].s64 + 15068;
	// 826038A4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826038A8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826038AC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826038B0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826038B4: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826038B8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826038BC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826038C0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826038C4: 4BE6355D  bl 0x82466e20
	ctx.lr = 0x826038C8;
	sub_82466E20(ctx, base);
	// 826038C8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826038CC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826038D0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826038D4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826038D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826038D8 size=108
    let mut pc: u32 = 0x826038D8;
    'dispatch: loop {
        match pc {
            0x826038D8 => {
    //   block [0x826038D8..0x82603944)
	// 826038D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826038DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826038E0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826038E4: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 826038E8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826038EC: 38EB79A8  addi r7, r11, 0x79a8
	ctx.r[7].s64 = ctx.r[11].s64 + 31144;
	// 826038F0: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826038F4: 388AC8B0  addi r4, r10, -0x3750
	ctx.r[4].s64 = ctx.r[10].s64 + -14160;
	// 826038F8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826038FC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82603900: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82603904: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82603908: 386A3B0C  addi r3, r10, 0x3b0c
	ctx.r[3].s64 = ctx.r[10].s64 + 15116;
	// 8260390C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82603910: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82603914: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82603918: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8260391C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82603920: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82603924: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82603928: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8260392C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82603930: 4BE634F1  bl 0x82466e20
	ctx.lr = 0x82603934;
	sub_82466E20(ctx, base);
	// 82603934: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82603938: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8260393C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82603940: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82603948(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82603948 size=112
    let mut pc: u32 = 0x82603948;
    'dispatch: loop {
        match pc {
            0x82603948 => {
    //   block [0x82603948..0x826039B8)
	// 82603948: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260394C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82603950: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82603954: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82603958: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 8260395C: 38AA15EC  addi r5, r10, 0x15ec
	ctx.r[5].s64 = ctx.r[10].s64 + 5612;
	// 82603960: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82603964: 390B79D8  addi r8, r11, 0x79d8
	ctx.r[8].s64 = ctx.r[11].s64 + 31192;
	// 82603968: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8260396C: 388AC8D8  addi r4, r10, -0x3728
	ctx.r[4].s64 = ctx.r[10].s64 + -14120;
	// 82603970: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82603974: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82603978: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8260397C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82603980: 386A3B3C  addi r3, r10, 0x3b3c
	ctx.r[3].s64 = ctx.r[10].s64 + 15164;
	// 82603984: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82603988: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8260398C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82603990: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82603994: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 82603998: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8260399C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826039A0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826039A4: 4BE6347D  bl 0x82466e20
	ctx.lr = 0x826039A8;
	sub_82466E20(ctx, base);
	// 826039A8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826039AC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826039B0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826039B4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826039B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826039B8 size=108
    let mut pc: u32 = 0x826039B8;
    'dispatch: loop {
        match pc {
            0x826039B8 => {
    //   block [0x826039B8..0x82603A24)
	// 826039B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826039BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826039C0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826039C4: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 826039C8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826039CC: 38EB79F0  addi r7, r11, 0x79f0
	ctx.r[7].s64 = ctx.r[11].s64 + 31216;
	// 826039D0: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826039D4: 388AC8F4  addi r4, r10, -0x370c
	ctx.r[4].s64 = ctx.r[10].s64 + -14092;
	// 826039D8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826039DC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 826039E0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826039E4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826039E8: 386A3B6C  addi r3, r10, 0x3b6c
	ctx.r[3].s64 = ctx.r[10].s64 + 15212;
	// 826039EC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826039F0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826039F4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826039F8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826039FC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82603A00: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82603A04: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82603A08: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82603A0C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82603A10: 4BE63411  bl 0x82466e20
	ctx.lr = 0x82603A14;
	sub_82466E20(ctx, base);
	// 82603A14: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82603A18: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82603A1C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82603A20: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82603A28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82603A28 size=108
    let mut pc: u32 = 0x82603A28;
    'dispatch: loop {
        match pc {
            0x82603A28 => {
    //   block [0x82603A28..0x82603A94)
	// 82603A28: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82603A2C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82603A30: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82603A34: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 82603A38: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82603A3C: 38EB7A20  addi r7, r11, 0x7a20
	ctx.r[7].s64 = ctx.r[11].s64 + 31264;
	// 82603A40: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 82603A44: 388AC910  addi r4, r10, -0x36f0
	ctx.r[4].s64 = ctx.r[10].s64 + -14064;
	// 82603A48: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82603A4C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82603A50: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82603A54: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82603A58: 386A3B9C  addi r3, r10, 0x3b9c
	ctx.r[3].s64 = ctx.r[10].s64 + 15260;
	// 82603A5C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82603A60: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82603A64: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82603A68: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82603A6C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82603A70: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82603A74: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82603A78: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82603A7C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82603A80: 4BE633A1  bl 0x82466e20
	ctx.lr = 0x82603A84;
	sub_82466E20(ctx, base);
	// 82603A84: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82603A88: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82603A8C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82603A90: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82603A98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82603A98 size=112
    let mut pc: u32 = 0x82603A98;
    'dispatch: loop {
        match pc {
            0x82603A98 => {
    //   block [0x82603A98..0x82603B08)
	// 82603A98: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82603A9C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82603AA0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82603AA4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82603AA8: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 82603AAC: 38AA09BC  addi r5, r10, 0x9bc
	ctx.r[5].s64 = ctx.r[10].s64 + 2492;
	// 82603AB0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82603AB4: 390B7A68  addi r8, r11, 0x7a68
	ctx.r[8].s64 = ctx.r[11].s64 + 31336;
	// 82603AB8: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 82603ABC: 388AC930  addi r4, r10, -0x36d0
	ctx.r[4].s64 = ctx.r[10].s64 + -14032;
	// 82603AC0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82603AC4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82603AC8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82603ACC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82603AD0: 386A3BCC  addi r3, r10, 0x3bcc
	ctx.r[3].s64 = ctx.r[10].s64 + 15308;
	// 82603AD4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82603AD8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82603ADC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82603AE0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82603AE4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82603AE8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82603AEC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82603AF0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82603AF4: 4BE6332D  bl 0x82466e20
	ctx.lr = 0x82603AF8;
	sub_82466E20(ctx, base);
	// 82603AF8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82603AFC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82603B00: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82603B04: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82603B08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82603B08 size=112
    let mut pc: u32 = 0x82603B08;
    'dispatch: loop {
        match pc {
            0x82603B08 => {
    //   block [0x82603B08..0x82603B78)
	// 82603B08: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82603B0C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82603B10: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82603B14: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82603B18: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 82603B1C: 38AA15EC  addi r5, r10, 0x15ec
	ctx.r[5].s64 = ctx.r[10].s64 + 5612;
	// 82603B20: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82603B24: 390B7AB0  addi r8, r11, 0x7ab0
	ctx.r[8].s64 = ctx.r[11].s64 + 31408;
	// 82603B28: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82603B2C: 388AC948  addi r4, r10, -0x36b8
	ctx.r[4].s64 = ctx.r[10].s64 + -14008;
	// 82603B30: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82603B34: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82603B38: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82603B3C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82603B40: 386A3BFC  addi r3, r10, 0x3bfc
	ctx.r[3].s64 = ctx.r[10].s64 + 15356;
	// 82603B44: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82603B48: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82603B4C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82603B50: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82603B54: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 82603B58: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82603B5C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82603B60: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82603B64: 4BE632BD  bl 0x82466e20
	ctx.lr = 0x82603B68;
	sub_82466E20(ctx, base);
	// 82603B68: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82603B6C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82603B70: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82603B74: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82603B78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82603B78 size=112
    let mut pc: u32 = 0x82603B78;
    'dispatch: loop {
        match pc {
            0x82603B78 => {
    //   block [0x82603B78..0x82603BE8)
	// 82603B78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82603B7C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82603B80: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82603B84: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82603B88: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 82603B8C: 38AA15EC  addi r5, r10, 0x15ec
	ctx.r[5].s64 = ctx.r[10].s64 + 5612;
	// 82603B90: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82603B94: 390B7AC8  addi r8, r11, 0x7ac8
	ctx.r[8].s64 = ctx.r[11].s64 + 31432;
	// 82603B98: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 82603B9C: 388AC968  addi r4, r10, -0x3698
	ctx.r[4].s64 = ctx.r[10].s64 + -13976;
	// 82603BA0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82603BA4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82603BA8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82603BAC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82603BB0: 386A3C2C  addi r3, r10, 0x3c2c
	ctx.r[3].s64 = ctx.r[10].s64 + 15404;
	// 82603BB4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82603BB8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82603BBC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82603BC0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82603BC4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82603BC8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82603BCC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82603BD0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82603BD4: 4BE6324D  bl 0x82466e20
	ctx.lr = 0x82603BD8;
	sub_82466E20(ctx, base);
	// 82603BD8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82603BDC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82603BE0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82603BE4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82603BE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82603BE8 size=112
    let mut pc: u32 = 0x82603BE8;
    'dispatch: loop {
        match pc {
            0x82603BE8 => {
    //   block [0x82603BE8..0x82603C58)
	// 82603BE8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82603BEC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82603BF0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82603BF4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82603BF8: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 82603BFC: 38AA371C  addi r5, r10, 0x371c
	ctx.r[5].s64 = ctx.r[10].s64 + 14108;
	// 82603C00: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82603C04: 390B7AF8  addi r8, r11, 0x7af8
	ctx.r[8].s64 = ctx.r[11].s64 + 31480;
	// 82603C08: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82603C0C: 388AC980  addi r4, r10, -0x3680
	ctx.r[4].s64 = ctx.r[10].s64 + -13952;
	// 82603C10: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82603C14: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82603C18: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82603C1C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82603C20: 386A3C5C  addi r3, r10, 0x3c5c
	ctx.r[3].s64 = ctx.r[10].s64 + 15452;
	// 82603C24: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82603C28: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82603C2C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82603C30: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82603C34: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82603C38: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82603C3C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82603C40: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82603C44: 4BE631DD  bl 0x82466e20
	ctx.lr = 0x82603C48;
	sub_82466E20(ctx, base);
	// 82603C48: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82603C4C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82603C50: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82603C54: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82603C58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82603C58 size=108
    let mut pc: u32 = 0x82603C58;
    'dispatch: loop {
        match pc {
            0x82603C58 => {
    //   block [0x82603C58..0x82603CC4)
	// 82603C58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82603C5C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82603C60: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82603C64: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 82603C68: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82603C6C: 38EB7B10  addi r7, r11, 0x7b10
	ctx.r[7].s64 = ctx.r[11].s64 + 31504;
	// 82603C70: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 82603C74: 388AC9A0  addi r4, r10, -0x3660
	ctx.r[4].s64 = ctx.r[10].s64 + -13920;
	// 82603C78: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82603C7C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82603C80: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82603C84: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82603C88: 386A3C8C  addi r3, r10, 0x3c8c
	ctx.r[3].s64 = ctx.r[10].s64 + 15500;
	// 82603C8C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82603C90: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82603C94: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82603C98: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82603C9C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82603CA0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82603CA4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82603CA8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82603CAC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82603CB0: 4BE63171  bl 0x82466e20
	ctx.lr = 0x82603CB4;
	sub_82466E20(ctx, base);
	// 82603CB4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82603CB8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82603CBC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82603CC0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82603CC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82603CC8 size=112
    let mut pc: u32 = 0x82603CC8;
    'dispatch: loop {
        match pc {
            0x82603CC8 => {
    //   block [0x82603CC8..0x82603D38)
	// 82603CC8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82603CCC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82603CD0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82603CD4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82603CD8: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 82603CDC: 38AA3C8C  addi r5, r10, 0x3c8c
	ctx.r[5].s64 = ctx.r[10].s64 + 15500;
	// 82603CE0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82603CE4: 390B7B40  addi r8, r11, 0x7b40
	ctx.r[8].s64 = ctx.r[11].s64 + 31552;
	// 82603CE8: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 82603CEC: 388AC9BC  addi r4, r10, -0x3644
	ctx.r[4].s64 = ctx.r[10].s64 + -13892;
	// 82603CF0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82603CF4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82603CF8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82603CFC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82603D00: 386A3CBC  addi r3, r10, 0x3cbc
	ctx.r[3].s64 = ctx.r[10].s64 + 15548;
	// 82603D04: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82603D08: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82603D0C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82603D10: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82603D14: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82603D18: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82603D1C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82603D20: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82603D24: 4BE630FD  bl 0x82466e20
	ctx.lr = 0x82603D28;
	sub_82466E20(ctx, base);
	// 82603D28: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82603D2C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82603D30: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82603D34: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82603D38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82603D38 size=24
    let mut pc: u32 = 0x82603D38;
    'dispatch: loop {
        match pc {
            0x82603D38 => {
    //   block [0x82603D38..0x82603D50)
	// 82603D38: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 82603D3C: 3D408275  lis r10, -0x7d8b
	ctx.r[10].s64 = -2106261504;
	// 82603D40: 394AB958  addi r10, r10, -0x46a8
	ctx.r[10].s64 = ctx.r[10].s64 + -18088;
	// 82603D44: 816B7B70  lwz r11, 0x7b70(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(31600 as u32) ) } as u64;
	// 82603D48: 916A0080  stw r11, 0x80(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(128 as u32), ctx.r[11].u32 ) };
	// 82603D4C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82603D50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82603D50 size=116
    let mut pc: u32 = 0x82603D50;
    'dispatch: loop {
        match pc {
            0x82603D50 => {
    //   block [0x82603D50..0x82603DC4)
	// 82603D50: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82603D54: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82603D58: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82603D5C: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 82603D60: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82603D64: 390BB958  addi r8, r11, -0x46a8
	ctx.r[8].s64 = ctx.r[11].s64 + -18088;
	// 82603D68: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82603D6C: 392A9DC0  addi r9, r10, -0x6240
	ctx.r[9].s64 = ctx.r[10].s64 + -25152;
	// 82603D70: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82603D74: 38E0000E  li r7, 0xe
	ctx.r[7].s64 = 14;
	// 82603D78: 38AA09BC  addi r5, r10, 0x9bc
	ctx.r[5].s64 = ctx.r[10].s64 + 2492;
	// 82603D7C: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82603D80: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82603D84: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82603D88: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82603D8C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82603D90: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82603D94: 3D608293  lis r11, -0x7d6d
	ctx.r[11].s64 = -2104295424;
	// 82603D98: 388ACA40  addi r4, r10, -0x35c0
	ctx.r[4].s64 = ctx.r[10].s64 + -13760;
	// 82603D9C: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82603DA0: 386B3CEC  addi r3, r11, 0x3cec
	ctx.r[3].s64 = ctx.r[11].s64 + 15596;
	// 82603DA4: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82603DA8: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82603DAC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82603DB0: 4BE63071  bl 0x82466e20
	ctx.lr = 0x82603DB4;
	sub_82466E20(ctx, base);
	// 82603DB4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82603DB8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82603DBC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82603DC0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82603DC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82603DC8 size=108
    let mut pc: u32 = 0x82603DC8;
    'dispatch: loop {
        match pc {
            0x82603DC8 => {
    //   block [0x82603DC8..0x82603E34)
	// 82603DC8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82603DCC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82603DD0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82603DD4: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 82603DD8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82603DDC: 38EB7B78  addi r7, r11, 0x7b78
	ctx.r[7].s64 = ctx.r[11].s64 + 31608;
	// 82603DE0: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 82603DE4: 388ACA5C  addi r4, r10, -0x35a4
	ctx.r[4].s64 = ctx.r[10].s64 + -13732;
	// 82603DE8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82603DEC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82603DF0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82603DF4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82603DF8: 386A3D1C  addi r3, r10, 0x3d1c
	ctx.r[3].s64 = ctx.r[10].s64 + 15644;
	// 82603DFC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82603E00: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82603E04: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82603E08: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82603E0C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82603E10: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82603E14: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82603E18: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82603E1C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82603E20: 4BE63001  bl 0x82466e20
	ctx.lr = 0x82603E24;
	sub_82466E20(ctx, base);
	// 82603E24: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82603E28: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82603E2C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82603E30: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82603E38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82603E38 size=108
    let mut pc: u32 = 0x82603E38;
    'dispatch: loop {
        match pc {
            0x82603E38 => {
    //   block [0x82603E38..0x82603EA4)
	// 82603E38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82603E3C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82603E40: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82603E44: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 82603E48: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82603E4C: 38EB7BC0  addi r7, r11, 0x7bc0
	ctx.r[7].s64 = ctx.r[11].s64 + 31680;
	// 82603E50: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 82603E54: 388ACA78  addi r4, r10, -0x3588
	ctx.r[4].s64 = ctx.r[10].s64 + -13704;
	// 82603E58: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82603E5C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82603E60: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82603E64: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82603E68: 386A3D4C  addi r3, r10, 0x3d4c
	ctx.r[3].s64 = ctx.r[10].s64 + 15692;
	// 82603E6C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82603E70: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82603E74: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82603E78: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82603E7C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82603E80: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82603E84: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82603E88: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82603E8C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82603E90: 4BE62F91  bl 0x82466e20
	ctx.lr = 0x82603E94;
	sub_82466E20(ctx, base);
	// 82603E94: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82603E98: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82603E9C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82603EA0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82603EA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82603EA8 size=108
    let mut pc: u32 = 0x82603EA8;
    'dispatch: loop {
        match pc {
            0x82603EA8 => {
    //   block [0x82603EA8..0x82603F14)
	// 82603EA8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82603EAC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82603EB0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82603EB4: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 82603EB8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82603EBC: 38EB7BF0  addi r7, r11, 0x7bf0
	ctx.r[7].s64 = ctx.r[11].s64 + 31728;
	// 82603EC0: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 82603EC4: 388ACA98  addi r4, r10, -0x3568
	ctx.r[4].s64 = ctx.r[10].s64 + -13672;
	// 82603EC8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82603ECC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82603ED0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82603ED4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82603ED8: 386A3D7C  addi r3, r10, 0x3d7c
	ctx.r[3].s64 = ctx.r[10].s64 + 15740;
	// 82603EDC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82603EE0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82603EE4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82603EE8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82603EEC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82603EF0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82603EF4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82603EF8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82603EFC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82603F00: 4BE62F21  bl 0x82466e20
	ctx.lr = 0x82603F04;
	sub_82466E20(ctx, base);
	// 82603F04: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82603F08: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82603F0C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82603F10: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82603F18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82603F18 size=112
    let mut pc: u32 = 0x82603F18;
    'dispatch: loop {
        match pc {
            0x82603F18 => {
    //   block [0x82603F18..0x82603F88)
	// 82603F18: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82603F1C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82603F20: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82603F24: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82603F28: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 82603F2C: 38AA09BC  addi r5, r10, 0x9bc
	ctx.r[5].s64 = ctx.r[10].s64 + 2492;
	// 82603F30: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82603F34: 390B7C20  addi r8, r11, 0x7c20
	ctx.r[8].s64 = ctx.r[11].s64 + 31776;
	// 82603F38: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 82603F3C: 388ACAB0  addi r4, r10, -0x3550
	ctx.r[4].s64 = ctx.r[10].s64 + -13648;
	// 82603F40: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82603F44: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82603F48: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82603F4C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82603F50: 386A3DAC  addi r3, r10, 0x3dac
	ctx.r[3].s64 = ctx.r[10].s64 + 15788;
	// 82603F54: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82603F58: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82603F5C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82603F60: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82603F64: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82603F68: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82603F6C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82603F70: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82603F74: 4BE62EAD  bl 0x82466e20
	ctx.lr = 0x82603F78;
	sub_82466E20(ctx, base);
	// 82603F78: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82603F7C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82603F80: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82603F84: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82603F88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82603F88 size=112
    let mut pc: u32 = 0x82603F88;
    'dispatch: loop {
        match pc {
            0x82603F88 => {
    //   block [0x82603F88..0x82603FF8)
	// 82603F88: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82603F8C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82603F90: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82603F94: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82603F98: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 82603F9C: 38AA09BC  addi r5, r10, 0x9bc
	ctx.r[5].s64 = ctx.r[10].s64 + 2492;
	// 82603FA0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82603FA4: 390B7C50  addi r8, r11, 0x7c50
	ctx.r[8].s64 = ctx.r[11].s64 + 31824;
	// 82603FA8: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82603FAC: 388ACAC0  addi r4, r10, -0x3540
	ctx.r[4].s64 = ctx.r[10].s64 + -13632;
	// 82603FB0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82603FB4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82603FB8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82603FBC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82603FC0: 386A3DDC  addi r3, r10, 0x3ddc
	ctx.r[3].s64 = ctx.r[10].s64 + 15836;
	// 82603FC4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82603FC8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82603FCC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82603FD0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82603FD4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82603FD8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82603FDC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82603FE0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82603FE4: 4BE62E3D  bl 0x82466e20
	ctx.lr = 0x82603FE8;
	sub_82466E20(ctx, base);
	// 82603FE8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82603FEC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82603FF0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82603FF4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82603FF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82603FF8 size=112
    let mut pc: u32 = 0x82603FF8;
    'dispatch: loop {
        match pc {
            0x82603FF8 => {
    //   block [0x82603FF8..0x82604068)
	// 82603FF8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82603FFC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82604000: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82604004: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82604008: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 8260400C: 38AA09BC  addi r5, r10, 0x9bc
	ctx.r[5].s64 = ctx.r[10].s64 + 2492;
	// 82604010: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82604014: 390B7C68  addi r8, r11, 0x7c68
	ctx.r[8].s64 = ctx.r[11].s64 + 31848;
	// 82604018: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8260401C: 388ACADC  addi r4, r10, -0x3524
	ctx.r[4].s64 = ctx.r[10].s64 + -13604;
	// 82604020: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82604024: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82604028: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8260402C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82604030: 386A3E0C  addi r3, r10, 0x3e0c
	ctx.r[3].s64 = ctx.r[10].s64 + 15884;
	// 82604034: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82604038: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8260403C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82604040: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82604044: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82604048: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8260404C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82604050: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82604054: 4BE62DCD  bl 0x82466e20
	ctx.lr = 0x82604058;
	sub_82466E20(ctx, base);
	// 82604058: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8260405C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82604060: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82604064: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82604068(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82604068 size=108
    let mut pc: u32 = 0x82604068;
    'dispatch: loop {
        match pc {
            0x82604068 => {
    //   block [0x82604068..0x826040D4)
	// 82604068: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260406C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82604070: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82604074: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 82604078: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8260407C: 38EB7C80  addi r7, r11, 0x7c80
	ctx.r[7].s64 = ctx.r[11].s64 + 31872;
	// 82604080: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 82604084: 388ACAFC  addi r4, r10, -0x3504
	ctx.r[4].s64 = ctx.r[10].s64 + -13572;
	// 82604088: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8260408C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82604090: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82604094: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82604098: 386A3E3C  addi r3, r10, 0x3e3c
	ctx.r[3].s64 = ctx.r[10].s64 + 15932;
	// 8260409C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826040A0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826040A4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826040A8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826040AC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826040B0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826040B4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826040B8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826040BC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826040C0: 4BE62D61  bl 0x82466e20
	ctx.lr = 0x826040C4;
	sub_82466E20(ctx, base);
	// 826040C4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826040C8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826040CC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826040D0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826040D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826040D8 size=112
    let mut pc: u32 = 0x826040D8;
    'dispatch: loop {
        match pc {
            0x826040D8 => {
    //   block [0x826040D8..0x82604148)
	// 826040D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826040DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826040E0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826040E4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 826040E8: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 826040EC: 38AA09BC  addi r5, r10, 0x9bc
	ctx.r[5].s64 = ctx.r[10].s64 + 2492;
	// 826040F0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826040F4: 390B7CB0  addi r8, r11, 0x7cb0
	ctx.r[8].s64 = ctx.r[11].s64 + 31920;
	// 826040F8: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826040FC: 388ACB34  addi r4, r10, -0x34cc
	ctx.r[4].s64 = ctx.r[10].s64 + -13516;
	// 82604100: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82604104: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82604108: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8260410C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82604110: 386A3E6C  addi r3, r10, 0x3e6c
	ctx.r[3].s64 = ctx.r[10].s64 + 15980;
	// 82604114: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82604118: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8260411C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82604120: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82604124: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82604128: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8260412C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82604130: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82604134: 4BE62CED  bl 0x82466e20
	ctx.lr = 0x82604138;
	sub_82466E20(ctx, base);
	// 82604138: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8260413C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82604140: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82604144: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82604148(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82604148 size=108
    let mut pc: u32 = 0x82604148;
    'dispatch: loop {
        match pc {
            0x82604148 => {
    //   block [0x82604148..0x826041B4)
	// 82604148: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260414C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82604150: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82604154: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 82604158: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8260415C: 38EB7CC8  addi r7, r11, 0x7cc8
	ctx.r[7].s64 = ctx.r[11].s64 + 31944;
	// 82604160: 3900000A  li r8, 0xa
	ctx.r[8].s64 = 10;
	// 82604164: 388ACB58  addi r4, r10, -0x34a8
	ctx.r[4].s64 = ctx.r[10].s64 + -13480;
	// 82604168: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8260416C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82604170: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82604174: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82604178: 386A3E9C  addi r3, r10, 0x3e9c
	ctx.r[3].s64 = ctx.r[10].s64 + 16028;
	// 8260417C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82604180: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82604184: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82604188: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8260418C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82604190: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82604194: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82604198: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8260419C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826041A0: 4BE62C81  bl 0x82466e20
	ctx.lr = 0x826041A4;
	sub_82466E20(ctx, base);
	// 826041A4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826041A8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826041AC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826041B0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826041B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826041B8 size=116
    let mut pc: u32 = 0x826041B8;
    'dispatch: loop {
        match pc {
            0x826041B8 => {
    //   block [0x826041B8..0x8260422C)
	// 826041B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826041BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826041C0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826041C4: 3D408274  lis r10, -0x7d8c
	ctx.r[10].s64 = -2106327040;
	// 826041C8: 38E00013  li r7, 0x13
	ctx.r[7].s64 = 19;
	// 826041CC: 390A7DB8  addi r8, r10, 0x7db8
	ctx.r[8].s64 = ctx.r[10].s64 + 32184;
	// 826041D0: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 826041D4: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 826041D8: 38AA09BC  addi r5, r10, 0x9bc
	ctx.r[5].s64 = ctx.r[10].s64 + 2492;
	// 826041DC: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826041E0: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826041E4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826041E8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826041EC: 388ACB7C  addi r4, r10, -0x3484
	ctx.r[4].s64 = ctx.r[10].s64 + -13444;
	// 826041F0: 396B9DD8  addi r11, r11, -0x6228
	ctx.r[11].s64 = ctx.r[11].s64 + -25128;
	// 826041F4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 826041F8: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826041FC: 386A3ECC  addi r3, r10, 0x3ecc
	ctx.r[3].s64 = ctx.r[10].s64 + 16076;
	// 82604200: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 82604204: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82604208: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 8260420C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82604210: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82604214: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82604218: 4BE62C09  bl 0x82466e20
	ctx.lr = 0x8260421C;
	sub_82466E20(ctx, base);
	// 8260421C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82604220: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82604224: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82604228: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82604230(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82604230 size=108
    let mut pc: u32 = 0x82604230;
    'dispatch: loop {
        match pc {
            0x82604230 => {
    //   block [0x82604230..0x8260429C)
	// 82604230: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82604234: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82604238: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8260423C: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 82604240: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82604244: 38EB7F80  addi r7, r11, 0x7f80
	ctx.r[7].s64 = ctx.r[11].s64 + 32640;
	// 82604248: 39000011  li r8, 0x11
	ctx.r[8].s64 = 17;
	// 8260424C: 388ACB8C  addi r4, r10, -0x3474
	ctx.r[4].s64 = ctx.r[10].s64 + -13428;
	// 82604250: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82604254: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82604258: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8260425C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82604260: 386A3EFC  addi r3, r10, 0x3efc
	ctx.r[3].s64 = ctx.r[10].s64 + 16124;
	// 82604264: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82604268: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8260426C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82604270: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82604274: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82604278: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8260427C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82604280: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82604284: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82604288: 4BE62B99  bl 0x82466e20
	ctx.lr = 0x8260428C;
	sub_82466E20(ctx, base);
	// 8260428C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82604290: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82604294: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82604298: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826042A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826042A0 size=112
    let mut pc: u32 = 0x826042A0;
    'dispatch: loop {
        match pc {
            0x826042A0 => {
    //   block [0x826042A0..0x82604310)
	// 826042A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826042A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826042A8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826042AC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 826042B0: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 826042B4: 38AA284C  addi r5, r10, 0x284c
	ctx.r[5].s64 = ctx.r[10].s64 + 10316;
	// 826042B8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826042BC: 390B8118  addi r8, r11, -0x7ee8
	ctx.r[8].s64 = ctx.r[11].s64 + -32488;
	// 826042C0: 39200019  li r9, 0x19
	ctx.r[9].s64 = 25;
	// 826042C4: 388ACBA8  addi r4, r10, -0x3458
	ctx.r[4].s64 = ctx.r[10].s64 + -13400;
	// 826042C8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826042CC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 826042D0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826042D4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826042D8: 386A3F2C  addi r3, r10, 0x3f2c
	ctx.r[3].s64 = ctx.r[10].s64 + 16172;
	// 826042DC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826042E0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826042E4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826042E8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826042EC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826042F0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826042F4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826042F8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826042FC: 4BE62B25  bl 0x82466e20
	ctx.lr = 0x82604300;
	sub_82466E20(ctx, base);
	// 82604300: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82604304: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82604308: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8260430C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82604310(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82604310 size=100
    let mut pc: u32 = 0x82604310;
    'dispatch: loop {
        match pc {
            0x82604310 => {
    //   block [0x82604310..0x82604374)
	// 82604310: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82604314: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82604318: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8260431C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82604320: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82604324: 38AA09BC  addi r5, r10, 0x9bc
	ctx.r[5].s64 = ctx.r[10].s64 + 2492;
	// 82604328: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8260432C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82604330: 388ACBBC  addi r4, r10, -0x3444
	ctx.r[4].s64 = ctx.r[10].s64 + -13380;
	// 82604334: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82604338: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8260433C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82604340: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82604344: 386A3F5C  addi r3, r10, 0x3f5c
	ctx.r[3].s64 = ctx.r[10].s64 + 16220;
	// 82604348: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8260434C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82604350: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82604354: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82604358: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8260435C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82604360: 4BE62AC1  bl 0x82466e20
	ctx.lr = 0x82604364;
	sub_82466E20(ctx, base);
	// 82604364: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82604368: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8260436C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82604370: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82604378(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82604378 size=112
    let mut pc: u32 = 0x82604378;
    'dispatch: loop {
        match pc {
            0x82604378 => {
    //   block [0x82604378..0x826043E8)
	// 82604378: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260437C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82604380: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82604384: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82604388: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 8260438C: 38AA3F5C  addi r5, r10, 0x3f5c
	ctx.r[5].s64 = ctx.r[10].s64 + 16220;
	// 82604390: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82604394: 390B8370  addi r8, r11, -0x7c90
	ctx.r[8].s64 = ctx.r[11].s64 + -31888;
	// 82604398: 39200005  li r9, 5
	ctx.r[9].s64 = 5;
	// 8260439C: 388ACBD4  addi r4, r10, -0x342c
	ctx.r[4].s64 = ctx.r[10].s64 + -13356;
	// 826043A0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826043A4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 826043A8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826043AC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826043B0: 386A3F8C  addi r3, r10, 0x3f8c
	ctx.r[3].s64 = ctx.r[10].s64 + 16268;
	// 826043B4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826043B8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826043BC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826043C0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826043C4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826043C8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826043CC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826043D0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826043D4: 4BE62A4D  bl 0x82466e20
	ctx.lr = 0x826043D8;
	sub_82466E20(ctx, base);
	// 826043D8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826043DC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826043E0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826043E4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826043E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826043E8 size=100
    let mut pc: u32 = 0x826043E8;
    'dispatch: loop {
        match pc {
            0x826043E8 => {
    //   block [0x826043E8..0x8260444C)
	// 826043E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826043EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826043F0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826043F4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 826043F8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826043FC: 38AA09BC  addi r5, r10, 0x9bc
	ctx.r[5].s64 = ctx.r[10].s64 + 2492;
	// 82604400: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82604404: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82604408: 388ACBF4  addi r4, r10, -0x340c
	ctx.r[4].s64 = ctx.r[10].s64 + -13324;
	// 8260440C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82604410: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82604414: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82604418: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8260441C: 386A3FBC  addi r3, r10, 0x3fbc
	ctx.r[3].s64 = ctx.r[10].s64 + 16316;
	// 82604420: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82604424: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82604428: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8260442C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82604430: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82604434: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82604438: 4BE629E9  bl 0x82466e20
	ctx.lr = 0x8260443C;
	sub_82466E20(ctx, base);
	// 8260443C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82604440: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82604444: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82604448: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82604450(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82604450 size=108
    let mut pc: u32 = 0x82604450;
    'dispatch: loop {
        match pc {
            0x82604450 => {
    //   block [0x82604450..0x826044BC)
	// 82604450: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82604454: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82604458: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8260445C: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 82604460: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82604464: 38EB83E8  addi r7, r11, -0x7c18
	ctx.r[7].s64 = ctx.r[11].s64 + -31768;
	// 82604468: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 8260446C: 388ACC04  addi r4, r10, -0x33fc
	ctx.r[4].s64 = ctx.r[10].s64 + -13308;
	// 82604470: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82604474: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82604478: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8260447C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82604480: 386A3FEC  addi r3, r10, 0x3fec
	ctx.r[3].s64 = ctx.r[10].s64 + 16364;
	// 82604484: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82604488: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8260448C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82604490: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82604494: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82604498: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8260449C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826044A0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826044A4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826044A8: 4BE62979  bl 0x82466e20
	ctx.lr = 0x826044AC;
	sub_82466E20(ctx, base);
	// 826044AC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826044B0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826044B4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826044B8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826044C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826044C0 size=112
    let mut pc: u32 = 0x826044C0;
    'dispatch: loop {
        match pc {
            0x826044C0 => {
    //   block [0x826044C0..0x82604530)
	// 826044C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826044C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826044C8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826044CC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 826044D0: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 826044D4: 38AA3FBC  addi r5, r10, 0x3fbc
	ctx.r[5].s64 = ctx.r[10].s64 + 16316;
	// 826044D8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826044DC: 390B8430  addi r8, r11, -0x7bd0
	ctx.r[8].s64 = ctx.r[11].s64 + -31696;
	// 826044E0: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826044E4: 388ACC34  addi r4, r10, -0x33cc
	ctx.r[4].s64 = ctx.r[10].s64 + -13260;
	// 826044E8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826044EC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 826044F0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826044F4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826044F8: 386A401C  addi r3, r10, 0x401c
	ctx.r[3].s64 = ctx.r[10].s64 + 16412;
	// 826044FC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82604500: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82604504: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82604508: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8260450C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82604510: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82604514: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82604518: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8260451C: 4BE62905  bl 0x82466e20
	ctx.lr = 0x82604520;
	sub_82466E20(ctx, base);
	// 82604520: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82604524: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82604528: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8260452C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82604530(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82604530 size=100
    let mut pc: u32 = 0x82604530;
    'dispatch: loop {
        match pc {
            0x82604530 => {
    //   block [0x82604530..0x82604594)
	// 82604530: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82604534: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82604538: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8260453C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82604540: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82604544: 38AA09BC  addi r5, r10, 0x9bc
	ctx.r[5].s64 = ctx.r[10].s64 + 2492;
	// 82604548: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8260454C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82604550: 388ACC4C  addi r4, r10, -0x33b4
	ctx.r[4].s64 = ctx.r[10].s64 + -13236;
	// 82604554: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82604558: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8260455C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82604560: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82604564: 386A404C  addi r3, r10, 0x404c
	ctx.r[3].s64 = ctx.r[10].s64 + 16460;
	// 82604568: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8260456C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82604570: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82604574: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82604578: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8260457C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82604580: 4BE628A1  bl 0x82466e20
	ctx.lr = 0x82604584;
	sub_82466E20(ctx, base);
	// 82604584: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82604588: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8260458C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82604590: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82604598(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82604598 size=100
    let mut pc: u32 = 0x82604598;
    'dispatch: loop {
        match pc {
            0x82604598 => {
    //   block [0x82604598..0x826045FC)
	// 82604598: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260459C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826045A0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826045A4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 826045A8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826045AC: 38AA09BC  addi r5, r10, 0x9bc
	ctx.r[5].s64 = ctx.r[10].s64 + 2492;
	// 826045B0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826045B4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826045B8: 388ACC68  addi r4, r10, -0x3398
	ctx.r[4].s64 = ctx.r[10].s64 + -13208;
	// 826045BC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 826045C0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826045C4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826045C8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826045CC: 386A407C  addi r3, r10, 0x407c
	ctx.r[3].s64 = ctx.r[10].s64 + 16508;
	// 826045D0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826045D4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826045D8: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826045DC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826045E0: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826045E4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826045E8: 4BE62839  bl 0x82466e20
	ctx.lr = 0x826045EC;
	sub_82466E20(ctx, base);
	// 826045EC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826045F0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826045F4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826045F8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82604600(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82604600 size=112
    let mut pc: u32 = 0x82604600;
    'dispatch: loop {
        match pc {
            0x82604600 => {
    //   block [0x82604600..0x82604670)
	// 82604600: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82604604: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82604608: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8260460C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82604610: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 82604614: 38AA404C  addi r5, r10, 0x404c
	ctx.r[5].s64 = ctx.r[10].s64 + 16460;
	// 82604618: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8260461C: 390B8460  addi r8, r11, -0x7ba0
	ctx.r[8].s64 = ctx.r[11].s64 + -31648;
	// 82604620: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 82604624: 388ACC80  addi r4, r10, -0x3380
	ctx.r[4].s64 = ctx.r[10].s64 + -13184;
	// 82604628: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8260462C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82604630: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82604634: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82604638: 386A40AC  addi r3, r10, 0x40ac
	ctx.r[3].s64 = ctx.r[10].s64 + 16556;
	// 8260463C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82604640: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82604644: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82604648: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8260464C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82604650: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82604654: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82604658: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8260465C: 4BE627C5  bl 0x82466e20
	ctx.lr = 0x82604660;
	sub_82466E20(ctx, base);
	// 82604660: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82604664: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82604668: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8260466C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82604670(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82604670 size=112
    let mut pc: u32 = 0x82604670;
    'dispatch: loop {
        match pc {
            0x82604670 => {
    //   block [0x82604670..0x826046E0)
	// 82604670: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82604674: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82604678: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8260467C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82604680: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 82604684: 38AA407C  addi r5, r10, 0x407c
	ctx.r[5].s64 = ctx.r[10].s64 + 16508;
	// 82604688: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8260468C: 390B84C0  addi r8, r11, -0x7b40
	ctx.r[8].s64 = ctx.r[11].s64 + -31552;
	// 82604690: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 82604694: 388ACCA4  addi r4, r10, -0x335c
	ctx.r[4].s64 = ctx.r[10].s64 + -13148;
	// 82604698: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8260469C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 826046A0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826046A4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826046A8: 386A40DC  addi r3, r10, 0x40dc
	ctx.r[3].s64 = ctx.r[10].s64 + 16604;
	// 826046AC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826046B0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826046B4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826046B8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826046BC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826046C0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826046C4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826046C8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826046CC: 4BE62755  bl 0x82466e20
	ctx.lr = 0x826046D0;
	sub_82466E20(ctx, base);
	// 826046D0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826046D4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826046D8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826046DC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826046E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826046E0 size=100
    let mut pc: u32 = 0x826046E0;
    'dispatch: loop {
        match pc {
            0x826046E0 => {
    //   block [0x826046E0..0x82604744)
	// 826046E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826046E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826046E8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826046EC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 826046F0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826046F4: 38AA09BC  addi r5, r10, 0x9bc
	ctx.r[5].s64 = ctx.r[10].s64 + 2492;
	// 826046F8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826046FC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82604700: 388ACCC8  addi r4, r10, -0x3338
	ctx.r[4].s64 = ctx.r[10].s64 + -13112;
	// 82604704: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82604708: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8260470C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82604710: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82604714: 386A410C  addi r3, r10, 0x410c
	ctx.r[3].s64 = ctx.r[10].s64 + 16652;
	// 82604718: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8260471C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82604720: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82604724: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82604728: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8260472C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82604730: 4BE626F1  bl 0x82466e20
	ctx.lr = 0x82604734;
	sub_82466E20(ctx, base);
	// 82604734: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82604738: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8260473C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82604740: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82604748(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82604748 size=112
    let mut pc: u32 = 0x82604748;
    'dispatch: loop {
        match pc {
            0x82604748 => {
    //   block [0x82604748..0x826047B8)
	// 82604748: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260474C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82604750: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82604754: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82604758: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 8260475C: 38AA410C  addi r5, r10, 0x410c
	ctx.r[5].s64 = ctx.r[10].s64 + 16652;
	// 82604760: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82604764: 390B8520  addi r8, r11, -0x7ae0
	ctx.r[8].s64 = ctx.r[11].s64 + -31456;
	// 82604768: 3920000A  li r9, 0xa
	ctx.r[9].s64 = 10;
	// 8260476C: 388ACCDC  addi r4, r10, -0x3324
	ctx.r[4].s64 = ctx.r[10].s64 + -13092;
	// 82604770: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82604774: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82604778: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8260477C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82604780: 386A413C  addi r3, r10, 0x413c
	ctx.r[3].s64 = ctx.r[10].s64 + 16700;
	// 82604784: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82604788: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8260478C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82604790: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82604794: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82604798: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8260479C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826047A0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826047A4: 4BE6267D  bl 0x82466e20
	ctx.lr = 0x826047A8;
	sub_82466E20(ctx, base);
	// 826047A8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826047AC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826047B0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826047B4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826047B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826047B8 size=108
    let mut pc: u32 = 0x826047B8;
    'dispatch: loop {
        match pc {
            0x826047B8 => {
    //   block [0x826047B8..0x82604824)
	// 826047B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826047BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826047C0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826047C4: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 826047C8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826047CC: 38EB8610  addi r7, r11, -0x79f0
	ctx.r[7].s64 = ctx.r[11].s64 + -31216;
	// 826047D0: 3900000A  li r8, 0xa
	ctx.r[8].s64 = 10;
	// 826047D4: 388ACCF4  addi r4, r10, -0x330c
	ctx.r[4].s64 = ctx.r[10].s64 + -13068;
	// 826047D8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826047DC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 826047E0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826047E4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826047E8: 386A416C  addi r3, r10, 0x416c
	ctx.r[3].s64 = ctx.r[10].s64 + 16748;
	// 826047EC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826047F0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826047F4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826047F8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826047FC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82604800: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82604804: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82604808: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8260480C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82604810: 4BE62611  bl 0x82466e20
	ctx.lr = 0x82604814;
	sub_82466E20(ctx, base);
	// 82604814: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82604818: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8260481C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82604820: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82604828(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82604828 size=108
    let mut pc: u32 = 0x82604828;
    'dispatch: loop {
        match pc {
            0x82604828 => {
    //   block [0x82604828..0x82604894)
	// 82604828: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260482C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82604830: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82604834: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 82604838: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8260483C: 38EB8700  addi r7, r11, -0x7900
	ctx.r[7].s64 = ctx.r[11].s64 + -30976;
	// 82604840: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 82604844: 388ACD24  addi r4, r10, -0x32dc
	ctx.r[4].s64 = ctx.r[10].s64 + -13020;
	// 82604848: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8260484C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82604850: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82604854: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82604858: 386A419C  addi r3, r10, 0x419c
	ctx.r[3].s64 = ctx.r[10].s64 + 16796;
	// 8260485C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82604860: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82604864: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82604868: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8260486C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82604870: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82604874: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82604878: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8260487C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82604880: 4BE625A1  bl 0x82466e20
	ctx.lr = 0x82604884;
	sub_82466E20(ctx, base);
	// 82604884: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82604888: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8260488C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82604890: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82604898(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82604898 size=108
    let mut pc: u32 = 0x82604898;
    'dispatch: loop {
        match pc {
            0x82604898 => {
    //   block [0x82604898..0x82604904)
	// 82604898: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260489C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826048A0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826048A4: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 826048A8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826048AC: 38EB8748  addi r7, r11, -0x78b8
	ctx.r[7].s64 = ctx.r[11].s64 + -30904;
	// 826048B0: 39000009  li r8, 9
	ctx.r[8].s64 = 9;
	// 826048B4: 388ACD44  addi r4, r10, -0x32bc
	ctx.r[4].s64 = ctx.r[10].s64 + -12988;
	// 826048B8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826048BC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 826048C0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826048C4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826048C8: 386A41CC  addi r3, r10, 0x41cc
	ctx.r[3].s64 = ctx.r[10].s64 + 16844;
	// 826048CC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826048D0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826048D4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826048D8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826048DC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826048E0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826048E4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826048E8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826048EC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826048F0: 4BE62531  bl 0x82466e20
	ctx.lr = 0x826048F4;
	sub_82466E20(ctx, base);
	// 826048F4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826048F8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826048FC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82604900: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82604908(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82604908 size=108
    let mut pc: u32 = 0x82604908;
    'dispatch: loop {
        match pc {
            0x82604908 => {
    //   block [0x82604908..0x82604974)
	// 82604908: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260490C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82604910: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82604914: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 82604918: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8260491C: 38EB8820  addi r7, r11, -0x77e0
	ctx.r[7].s64 = ctx.r[11].s64 + -30688;
	// 82604920: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 82604924: 388ACD68  addi r4, r10, -0x3298
	ctx.r[4].s64 = ctx.r[10].s64 + -12952;
	// 82604928: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8260492C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82604930: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82604934: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82604938: 386A41FC  addi r3, r10, 0x41fc
	ctx.r[3].s64 = ctx.r[10].s64 + 16892;
	// 8260493C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82604940: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82604944: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82604948: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8260494C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82604950: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82604954: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82604958: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8260495C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82604960: 4BE624C1  bl 0x82466e20
	ctx.lr = 0x82604964;
	sub_82466E20(ctx, base);
	// 82604964: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82604968: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8260496C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82604970: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82604978(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82604978 size=100
    let mut pc: u32 = 0x82604978;
    'dispatch: loop {
        match pc {
            0x82604978 => {
    //   block [0x82604978..0x826049DC)
	// 82604978: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260497C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82604980: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82604984: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82604988: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8260498C: 38AA09BC  addi r5, r10, 0x9bc
	ctx.r[5].s64 = ctx.r[10].s64 + 2492;
	// 82604990: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82604994: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82604998: 388ACD84  addi r4, r10, -0x327c
	ctx.r[4].s64 = ctx.r[10].s64 + -12924;
	// 8260499C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 826049A0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826049A4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826049A8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826049AC: 386A422C  addi r3, r10, 0x422c
	ctx.r[3].s64 = ctx.r[10].s64 + 16940;
	// 826049B0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826049B4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826049B8: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826049BC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826049C0: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826049C4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826049C8: 4BE62459  bl 0x82466e20
	ctx.lr = 0x826049CC;
	sub_82466E20(ctx, base);
	// 826049CC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826049D0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826049D4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826049D8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826049E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826049E0 size=112
    let mut pc: u32 = 0x826049E0;
    'dispatch: loop {
        match pc {
            0x826049E0 => {
    //   block [0x826049E0..0x82604A50)
	// 826049E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826049E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826049E8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826049EC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 826049F0: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 826049F4: 38AA422C  addi r5, r10, 0x422c
	ctx.r[5].s64 = ctx.r[10].s64 + 16940;
	// 826049F8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826049FC: 390B8838  addi r8, r11, -0x77c8
	ctx.r[8].s64 = ctx.r[11].s64 + -30664;
	// 82604A00: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 82604A04: 388ACD98  addi r4, r10, -0x3268
	ctx.r[4].s64 = ctx.r[10].s64 + -12904;
	// 82604A08: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82604A0C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82604A10: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82604A14: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82604A18: 386A425C  addi r3, r10, 0x425c
	ctx.r[3].s64 = ctx.r[10].s64 + 16988;
	// 82604A1C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82604A20: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82604A24: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82604A28: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82604A2C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82604A30: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82604A34: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82604A38: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82604A3C: 4BE623E5  bl 0x82466e20
	ctx.lr = 0x82604A40;
	sub_82466E20(ctx, base);
	// 82604A40: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82604A44: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82604A48: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82604A4C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82604A50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82604A50 size=108
    let mut pc: u32 = 0x82604A50;
    'dispatch: loop {
        match pc {
            0x82604A50 => {
    //   block [0x82604A50..0x82604ABC)
	// 82604A50: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82604A54: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82604A58: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82604A5C: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 82604A60: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82604A64: 38EB8880  addi r7, r11, -0x7780
	ctx.r[7].s64 = ctx.r[11].s64 + -30592;
	// 82604A68: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 82604A6C: 388ACDB4  addi r4, r10, -0x324c
	ctx.r[4].s64 = ctx.r[10].s64 + -12876;
	// 82604A70: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82604A74: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82604A78: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82604A7C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82604A80: 386A428C  addi r3, r10, 0x428c
	ctx.r[3].s64 = ctx.r[10].s64 + 17036;
	// 82604A84: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82604A88: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82604A8C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82604A90: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82604A94: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82604A98: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82604A9C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82604AA0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82604AA4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82604AA8: 4BE62379  bl 0x82466e20
	ctx.lr = 0x82604AAC;
	sub_82466E20(ctx, base);
	// 82604AAC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82604AB0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82604AB4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82604AB8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82604AC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82604AC0 size=112
    let mut pc: u32 = 0x82604AC0;
    'dispatch: loop {
        match pc {
            0x82604AC0 => {
    //   block [0x82604AC0..0x82604B30)
	// 82604AC0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82604AC4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82604AC8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82604ACC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82604AD0: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 82604AD4: 38AA09BC  addi r5, r10, 0x9bc
	ctx.r[5].s64 = ctx.r[10].s64 + 2492;
	// 82604AD8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82604ADC: 390B88C8  addi r8, r11, -0x7738
	ctx.r[8].s64 = ctx.r[11].s64 + -30520;
	// 82604AE0: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82604AE4: 388ACDE4  addi r4, r10, -0x321c
	ctx.r[4].s64 = ctx.r[10].s64 + -12828;
	// 82604AE8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82604AEC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82604AF0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82604AF4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82604AF8: 386A42BC  addi r3, r10, 0x42bc
	ctx.r[3].s64 = ctx.r[10].s64 + 17084;
	// 82604AFC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82604B00: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82604B04: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82604B08: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82604B0C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82604B10: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82604B14: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82604B18: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82604B1C: 4BE62305  bl 0x82466e20
	ctx.lr = 0x82604B20;
	sub_82466E20(ctx, base);
	// 82604B20: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82604B24: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82604B28: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82604B2C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82604B30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82604B30 size=108
    let mut pc: u32 = 0x82604B30;
    'dispatch: loop {
        match pc {
            0x82604B30 => {
    //   block [0x82604B30..0x82604B9C)
	// 82604B30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82604B34: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82604B38: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82604B3C: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 82604B40: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82604B44: 38EB88E0  addi r7, r11, -0x7720
	ctx.r[7].s64 = ctx.r[11].s64 + -30496;
	// 82604B48: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 82604B4C: 388ACDFC  addi r4, r10, -0x3204
	ctx.r[4].s64 = ctx.r[10].s64 + -12804;
	// 82604B50: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82604B54: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82604B58: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82604B5C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82604B60: 386A42EC  addi r3, r10, 0x42ec
	ctx.r[3].s64 = ctx.r[10].s64 + 17132;
	// 82604B64: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82604B68: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82604B6C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82604B70: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82604B74: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82604B78: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82604B7C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82604B80: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82604B84: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82604B88: 4BE62299  bl 0x82466e20
	ctx.lr = 0x82604B8C;
	sub_82466E20(ctx, base);
	// 82604B8C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82604B90: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82604B94: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82604B98: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82604BA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82604BA0 size=112
    let mut pc: u32 = 0x82604BA0;
    'dispatch: loop {
        match pc {
            0x82604BA0 => {
    //   block [0x82604BA0..0x82604C10)
	// 82604BA0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82604BA4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82604BA8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82604BAC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82604BB0: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 82604BB4: 38AA42BC  addi r5, r10, 0x42bc
	ctx.r[5].s64 = ctx.r[10].s64 + 17084;
	// 82604BB8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82604BBC: 390B8928  addi r8, r11, -0x76d8
	ctx.r[8].s64 = ctx.r[11].s64 + -30424;
	// 82604BC0: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82604BC4: 388ACE38  addi r4, r10, -0x31c8
	ctx.r[4].s64 = ctx.r[10].s64 + -12744;
	// 82604BC8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82604BCC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82604BD0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82604BD4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82604BD8: 386A431C  addi r3, r10, 0x431c
	ctx.r[3].s64 = ctx.r[10].s64 + 17180;
	// 82604BDC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82604BE0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82604BE4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82604BE8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82604BEC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82604BF0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82604BF4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82604BF8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82604BFC: 4BE62225  bl 0x82466e20
	ctx.lr = 0x82604C00;
	sub_82466E20(ctx, base);
	// 82604C00: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82604C04: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82604C08: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82604C0C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82604C10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82604C10 size=100
    let mut pc: u32 = 0x82604C10;
    'dispatch: loop {
        match pc {
            0x82604C10 => {
    //   block [0x82604C10..0x82604C74)
	// 82604C10: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82604C14: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82604C18: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82604C1C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82604C20: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82604C24: 38AA09BC  addi r5, r10, 0x9bc
	ctx.r[5].s64 = ctx.r[10].s64 + 2492;
	// 82604C28: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82604C2C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82604C30: 388ACE54  addi r4, r10, -0x31ac
	ctx.r[4].s64 = ctx.r[10].s64 + -12716;
	// 82604C34: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82604C38: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82604C3C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82604C40: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82604C44: 386A434C  addi r3, r10, 0x434c
	ctx.r[3].s64 = ctx.r[10].s64 + 17228;
	// 82604C48: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82604C4C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82604C50: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82604C54: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82604C58: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82604C5C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82604C60: 4BE621C1  bl 0x82466e20
	ctx.lr = 0x82604C64;
	sub_82466E20(ctx, base);
	// 82604C64: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82604C68: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82604C6C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82604C70: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82604C78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82604C78 size=112
    let mut pc: u32 = 0x82604C78;
    'dispatch: loop {
        match pc {
            0x82604C78 => {
    //   block [0x82604C78..0x82604CE8)
	// 82604C78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82604C7C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82604C80: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82604C84: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82604C88: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 82604C8C: 38AA434C  addi r5, r10, 0x434c
	ctx.r[5].s64 = ctx.r[10].s64 + 17228;
	// 82604C90: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82604C94: 390B8940  addi r8, r11, -0x76c0
	ctx.r[8].s64 = ctx.r[11].s64 + -30400;
	// 82604C98: 39200007  li r9, 7
	ctx.r[9].s64 = 7;
	// 82604C9C: 388ACE6C  addi r4, r10, -0x3194
	ctx.r[4].s64 = ctx.r[10].s64 + -12692;
	// 82604CA0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82604CA4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82604CA8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82604CAC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82604CB0: 386A437C  addi r3, r10, 0x437c
	ctx.r[3].s64 = ctx.r[10].s64 + 17276;
	// 82604CB4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82604CB8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82604CBC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82604CC0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82604CC4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82604CC8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82604CCC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82604CD0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82604CD4: 4BE6214D  bl 0x82466e20
	ctx.lr = 0x82604CD8;
	sub_82466E20(ctx, base);
	// 82604CD8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82604CDC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82604CE0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82604CE4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82604CE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82604CE8 size=108
    let mut pc: u32 = 0x82604CE8;
    'dispatch: loop {
        match pc {
            0x82604CE8 => {
    //   block [0x82604CE8..0x82604D54)
	// 82604CE8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82604CEC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82604CF0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82604CF4: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 82604CF8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82604CFC: 38EB89E8  addi r7, r11, -0x7618
	ctx.r[7].s64 = ctx.r[11].s64 + -30232;
	// 82604D00: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 82604D04: 388ACE8C  addi r4, r10, -0x3174
	ctx.r[4].s64 = ctx.r[10].s64 + -12660;
	// 82604D08: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82604D0C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82604D10: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82604D14: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82604D18: 386A43AC  addi r3, r10, 0x43ac
	ctx.r[3].s64 = ctx.r[10].s64 + 17324;
	// 82604D1C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82604D20: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82604D24: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82604D28: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82604D2C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82604D30: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82604D34: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82604D38: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82604D3C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82604D40: 4BE620E1  bl 0x82466e20
	ctx.lr = 0x82604D44;
	sub_82466E20(ctx, base);
	// 82604D44: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82604D48: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82604D4C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82604D50: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82604D58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82604D58 size=112
    let mut pc: u32 = 0x82604D58;
    'dispatch: loop {
        match pc {
            0x82604D58 => {
    //   block [0x82604D58..0x82604DC8)
	// 82604D58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82604D5C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82604D60: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82604D64: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82604D68: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 82604D6C: 38AA09BC  addi r5, r10, 0x9bc
	ctx.r[5].s64 = ctx.r[10].s64 + 2492;
	// 82604D70: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82604D74: 390B8A18  addi r8, r11, -0x75e8
	ctx.r[8].s64 = ctx.r[11].s64 + -30184;
	// 82604D78: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 82604D7C: 388ACEA0  addi r4, r10, -0x3160
	ctx.r[4].s64 = ctx.r[10].s64 + -12640;
	// 82604D80: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82604D84: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82604D88: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82604D8C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82604D90: 386A43DC  addi r3, r10, 0x43dc
	ctx.r[3].s64 = ctx.r[10].s64 + 17372;
	// 82604D94: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82604D98: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82604D9C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82604DA0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82604DA4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82604DA8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82604DAC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82604DB0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82604DB4: 4BE6206D  bl 0x82466e20
	ctx.lr = 0x82604DB8;
	sub_82466E20(ctx, base);
	// 82604DB8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82604DBC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82604DC0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82604DC4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82604DC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82604DC8 size=112
    let mut pc: u32 = 0x82604DC8;
    'dispatch: loop {
        match pc {
            0x82604DC8 => {
    //   block [0x82604DC8..0x82604E38)
	// 82604DC8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82604DCC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82604DD0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82604DD4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82604DD8: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 82604DDC: 38AA09BC  addi r5, r10, 0x9bc
	ctx.r[5].s64 = ctx.r[10].s64 + 2492;
	// 82604DE0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82604DE4: 390B8A60  addi r8, r11, -0x75a0
	ctx.r[8].s64 = ctx.r[11].s64 + -30112;
	// 82604DE8: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 82604DEC: 388A88D0  addi r4, r10, -0x7730
	ctx.r[4].s64 = ctx.r[10].s64 + -30512;
	// 82604DF0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82604DF4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82604DF8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82604DFC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82604E00: 386A440C  addi r3, r10, 0x440c
	ctx.r[3].s64 = ctx.r[10].s64 + 17420;
	// 82604E04: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82604E08: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82604E0C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82604E10: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82604E14: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82604E18: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82604E1C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82604E20: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82604E24: 4BE61FFD  bl 0x82466e20
	ctx.lr = 0x82604E28;
	sub_82466E20(ctx, base);
	// 82604E28: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82604E2C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82604E30: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82604E34: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82604E38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82604E38 size=100
    let mut pc: u32 = 0x82604E38;
    'dispatch: loop {
        match pc {
            0x82604E38 => {
    //   block [0x82604E38..0x82604E9C)
	// 82604E38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82604E3C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82604E40: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82604E44: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82604E48: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82604E4C: 38AA09BC  addi r5, r10, 0x9bc
	ctx.r[5].s64 = ctx.r[10].s64 + 2492;
	// 82604E50: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82604E54: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82604E58: 388ACEB4  addi r4, r10, -0x314c
	ctx.r[4].s64 = ctx.r[10].s64 + -12620;
	// 82604E5C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82604E60: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82604E64: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82604E68: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82604E6C: 386A443C  addi r3, r10, 0x443c
	ctx.r[3].s64 = ctx.r[10].s64 + 17468;
	// 82604E70: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82604E74: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82604E78: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82604E7C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82604E80: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82604E84: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82604E88: 4BE61F99  bl 0x82466e20
	ctx.lr = 0x82604E8C;
	sub_82466E20(ctx, base);
	// 82604E8C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82604E90: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82604E94: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82604E98: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82604EA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82604EA0 size=112
    let mut pc: u32 = 0x82604EA0;
    'dispatch: loop {
        match pc {
            0x82604EA0 => {
    //   block [0x82604EA0..0x82604F10)
	// 82604EA0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82604EA4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82604EA8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82604EAC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82604EB0: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 82604EB4: 38AA443C  addi r5, r10, 0x443c
	ctx.r[5].s64 = ctx.r[10].s64 + 17468;
	// 82604EB8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82604EBC: 390B8AA8  addi r8, r11, -0x7558
	ctx.r[8].s64 = ctx.r[11].s64 + -30040;
	// 82604EC0: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 82604EC4: 388ACED0  addi r4, r10, -0x3130
	ctx.r[4].s64 = ctx.r[10].s64 + -12592;
	// 82604EC8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82604ECC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82604ED0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82604ED4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82604ED8: 386A446C  addi r3, r10, 0x446c
	ctx.r[3].s64 = ctx.r[10].s64 + 17516;
	// 82604EDC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82604EE0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82604EE4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82604EE8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82604EEC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82604EF0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82604EF4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82604EF8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82604EFC: 4BE61F25  bl 0x82466e20
	ctx.lr = 0x82604F00;
	sub_82466E20(ctx, base);
	// 82604F00: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82604F04: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82604F08: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82604F0C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82604F10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82604F10 size=112
    let mut pc: u32 = 0x82604F10;
    'dispatch: loop {
        match pc {
            0x82604F10 => {
    //   block [0x82604F10..0x82604F80)
	// 82604F10: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82604F14: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82604F18: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82604F1C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82604F20: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 82604F24: 38AA09BC  addi r5, r10, 0x9bc
	ctx.r[5].s64 = ctx.r[10].s64 + 2492;
	// 82604F28: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82604F2C: 390B8AF0  addi r8, r11, -0x7510
	ctx.r[8].s64 = ctx.r[11].s64 + -29968;
	// 82604F30: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82604F34: 388ACEF0  addi r4, r10, -0x3110
	ctx.r[4].s64 = ctx.r[10].s64 + -12560;
	// 82604F38: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82604F3C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82604F40: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82604F44: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82604F48: 386A449C  addi r3, r10, 0x449c
	ctx.r[3].s64 = ctx.r[10].s64 + 17564;
	// 82604F4C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82604F50: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82604F54: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82604F58: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82604F5C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82604F60: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82604F64: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82604F68: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82604F6C: 4BE61EB5  bl 0x82466e20
	ctx.lr = 0x82604F70;
	sub_82466E20(ctx, base);
	// 82604F70: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82604F74: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82604F78: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82604F7C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82604F80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82604F80 size=112
    let mut pc: u32 = 0x82604F80;
    'dispatch: loop {
        match pc {
            0x82604F80 => {
    //   block [0x82604F80..0x82604FF0)
	// 82604F80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82604F84: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82604F88: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82604F8C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82604F90: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 82604F94: 38AA09BC  addi r5, r10, 0x9bc
	ctx.r[5].s64 = ctx.r[10].s64 + 2492;
	// 82604F98: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82604F9C: 390B8B08  addi r8, r11, -0x74f8
	ctx.r[8].s64 = ctx.r[11].s64 + -29944;
	// 82604FA0: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82604FA4: 388ACF08  addi r4, r10, -0x30f8
	ctx.r[4].s64 = ctx.r[10].s64 + -12536;
	// 82604FA8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82604FAC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82604FB0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82604FB4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82604FB8: 386A44CC  addi r3, r10, 0x44cc
	ctx.r[3].s64 = ctx.r[10].s64 + 17612;
	// 82604FBC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82604FC0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82604FC4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82604FC8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82604FCC: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 82604FD0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82604FD4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82604FD8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82604FDC: 4BE61E45  bl 0x82466e20
	ctx.lr = 0x82604FE0;
	sub_82466E20(ctx, base);
	// 82604FE0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82604FE4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82604FE8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82604FEC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82604FF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82604FF0 size=112
    let mut pc: u32 = 0x82604FF0;
    'dispatch: loop {
        match pc {
            0x82604FF0 => {
    //   block [0x82604FF0..0x82605060)
	// 82604FF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82604FF4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82604FF8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82604FFC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82605000: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 82605004: 38AA449C  addi r5, r10, 0x449c
	ctx.r[5].s64 = ctx.r[10].s64 + 17564;
	// 82605008: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8260500C: 390B8B20  addi r8, r11, -0x74e0
	ctx.r[8].s64 = ctx.r[11].s64 + -29920;
	// 82605010: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 82605014: 388ACF24  addi r4, r10, -0x30dc
	ctx.r[4].s64 = ctx.r[10].s64 + -12508;
	// 82605018: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8260501C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82605020: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82605024: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82605028: 386A44FC  addi r3, r10, 0x44fc
	ctx.r[3].s64 = ctx.r[10].s64 + 17660;
	// 8260502C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82605030: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82605034: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82605038: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8260503C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82605040: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82605044: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82605048: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8260504C: 4BE61DD5  bl 0x82466e20
	ctx.lr = 0x82605050;
	sub_82466E20(ctx, base);
	// 82605050: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82605054: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82605058: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8260505C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82605060(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82605060 size=72
    let mut pc: u32 = 0x82605060;
    'dispatch: loop {
        match pc {
            0x82605060 => {
    //   block [0x82605060..0x826050A8)
	// 82605060: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82605064: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82605068: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8260506C: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 82605070: 38A00005  li r5, 5
	ctx.r[5].s64 = 5;
	// 82605074: 38CB7BE0  addi r6, r11, 0x7be0
	ctx.r[6].s64 = ctx.r[11].s64 + 31712;
	// 82605078: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 8260507C: 388B9E28  addi r4, r11, -0x61d8
	ctx.r[4].s64 = ctx.r[11].s64 + -25048;
	// 82605080: 3D608293  lis r11, -0x7d6d
	ctx.r[11].s64 = -2104295424;
	// 82605084: 386B452C  addi r3, r11, 0x452c
	ctx.r[3].s64 = ctx.r[11].s64 + 17708;
	// 82605088: 4BE76A01  bl 0x8247ba88
	ctx.lr = 0x8260508C;
	sub_8247BA88(ctx, base);
	// 8260508C: 3D608271  lis r11, -0x7d8f
	ctx.r[11].s64 = -2106523648;
	// 82605090: 386BCD08  addi r3, r11, -0x32f8
	ctx.r[3].s64 = ctx.r[11].s64 + -13048;
	// 82605094: 4BF2DAA5  bl 0x82532b38
	ctx.lr = 0x82605098;
	sub_82532B38(ctx, base);
	// 82605098: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8260509C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826050A0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826050A4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826050A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826050A8 size=108
    let mut pc: u32 = 0x826050A8;
    'dispatch: loop {
        match pc {
            0x826050A8 => {
    //   block [0x826050A8..0x82605114)
	// 826050A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826050AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826050B0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826050B4: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 826050B8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826050BC: 38EBBAA8  addi r7, r11, -0x4558
	ctx.r[7].s64 = ctx.r[11].s64 + -17752;
	// 826050C0: 39000005  li r8, 5
	ctx.r[8].s64 = 5;
	// 826050C4: 388A2B24  addi r4, r10, 0x2b24
	ctx.r[4].s64 = ctx.r[10].s64 + 11044;
	// 826050C8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826050CC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 826050D0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826050D4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826050D8: 386A4544  addi r3, r10, 0x4544
	ctx.r[3].s64 = ctx.r[10].s64 + 17732;
	// 826050DC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826050E0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826050E4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826050E8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826050EC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826050F0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826050F4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826050F8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826050FC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82605100: 4BE61D21  bl 0x82466e20
	ctx.lr = 0x82605104;
	sub_82466E20(ctx, base);
	// 82605104: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82605108: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8260510C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82605110: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82605118(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82605118 size=24
    let mut pc: u32 = 0x82605118;
    'dispatch: loop {
        match pc {
            0x82605118 => {
    //   block [0x82605118..0x82605130)
	// 82605118: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 8260511C: 3D408275  lis r10, -0x7d8b
	ctx.r[10].s64 = -2106261504;
	// 82605120: 394A2C78  addi r10, r10, 0x2c78
	ctx.r[10].s64 = ctx.r[10].s64 + 11384;
	// 82605124: 816BBB20  lwz r11, -0x44e0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-17632 as u32) ) } as u64;
	// 82605128: 916A0038  stw r11, 0x38(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(56 as u32), ctx.r[11].u32 ) };
	// 8260512C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82605130(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82605130 size=112
    let mut pc: u32 = 0x82605130;
    'dispatch: loop {
        match pc {
            0x82605130 => {
    //   block [0x82605130..0x826051A0)
	// 82605130: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82605134: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82605138: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8260513C: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82605140: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 82605144: 392ACF9C  addi r9, r10, -0x3064
	ctx.r[9].s64 = ctx.r[10].s64 + -12388;
	// 82605148: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8260514C: 390B2C78  addi r8, r11, 0x2c78
	ctx.r[8].s64 = ctx.r[11].s64 + 11384;
	// 82605150: 38E00003  li r7, 3
	ctx.r[7].s64 = 3;
	// 82605154: 388A2B3C  addi r4, r10, 0x2b3c
	ctx.r[4].s64 = ctx.r[10].s64 + 11068;
	// 82605158: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8260515C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82605160: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82605164: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82605168: 386A4574  addi r3, r10, 0x4574
	ctx.r[3].s64 = ctx.r[10].s64 + 17780;
	// 8260516C: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82605170: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82605174: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82605178: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8260517C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82605180: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82605184: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82605188: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8260518C: 4BE61C95  bl 0x82466e20
	ctx.lr = 0x82605190;
	sub_82466E20(ctx, base);
	// 82605190: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82605194: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82605198: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8260519C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826051A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826051A0 size=108
    let mut pc: u32 = 0x826051A0;
    'dispatch: loop {
        match pc {
            0x826051A0 => {
    //   block [0x826051A0..0x8260520C)
	// 826051A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826051A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826051A8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826051AC: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 826051B0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826051B4: 38EBBB24  addi r7, r11, -0x44dc
	ctx.r[7].s64 = ctx.r[11].s64 + -17628;
	// 826051B8: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826051BC: 388A2B50  addi r4, r10, 0x2b50
	ctx.r[4].s64 = ctx.r[10].s64 + 11088;
	// 826051C0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826051C4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 826051C8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826051CC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826051D0: 386A45A4  addi r3, r10, 0x45a4
	ctx.r[3].s64 = ctx.r[10].s64 + 17828;
	// 826051D4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826051D8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826051DC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826051E0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826051E4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826051E8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826051EC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826051F0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826051F4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826051F8: 4BE61C29  bl 0x82466e20
	ctx.lr = 0x826051FC;
	sub_82466E20(ctx, base);
	// 826051FC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82605200: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82605204: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82605208: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82605210(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82605210 size=108
    let mut pc: u32 = 0x82605210;
    'dispatch: loop {
        match pc {
            0x82605210 => {
    //   block [0x82605210..0x8260527C)
	// 82605210: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82605214: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82605218: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8260521C: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 82605220: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82605224: 38EBBB54  addi r7, r11, -0x44ac
	ctx.r[7].s64 = ctx.r[11].s64 + -17580;
	// 82605228: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 8260522C: 388A2B6C  addi r4, r10, 0x2b6c
	ctx.r[4].s64 = ctx.r[10].s64 + 11116;
	// 82605230: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82605234: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82605238: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8260523C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82605240: 386A45D4  addi r3, r10, 0x45d4
	ctx.r[3].s64 = ctx.r[10].s64 + 17876;
	// 82605244: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82605248: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8260524C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82605250: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82605254: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82605258: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8260525C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82605260: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82605264: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82605268: 4BE61BB9  bl 0x82466e20
	ctx.lr = 0x8260526C;
	sub_82466E20(ctx, base);
	// 8260526C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82605270: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82605274: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82605278: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82605280(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82605280 size=24
    let mut pc: u32 = 0x82605280;
    'dispatch: loop {
        match pc {
            0x82605280 => {
    //   block [0x82605280..0x82605298)
	// 82605280: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 82605284: 3D408275  lis r10, -0x7d8b
	ctx.r[10].s64 = -2106261504;
	// 82605288: 394A2CC0  addi r10, r10, 0x2cc0
	ctx.r[10].s64 = ctx.r[10].s64 + 11456;
	// 8260528C: 816BBB84  lwz r11, -0x447c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-17532 as u32) ) } as u64;
	// 82605290: 916A0008  stw r11, 8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82605294: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82605298(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82605298 size=116
    let mut pc: u32 = 0x82605298;
    'dispatch: loop {
        match pc {
            0x82605298 => {
    //   block [0x82605298..0x8260530C)
	// 82605298: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260529C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826052A0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826052A4: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 826052A8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826052AC: 390B2CC0  addi r8, r11, 0x2cc0
	ctx.r[8].s64 = ctx.r[11].s64 + 11456;
	// 826052B0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826052B4: 392ACFD0  addi r9, r10, -0x3030
	ctx.r[9].s64 = ctx.r[10].s64 + -12336;
	// 826052B8: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 826052BC: 38E00005  li r7, 5
	ctx.r[7].s64 = 5;
	// 826052C0: 38AA5774  addi r5, r10, 0x5774
	ctx.r[5].s64 = ctx.r[10].s64 + 22388;
	// 826052C4: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826052C8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826052CC: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826052D0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826052D4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826052D8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826052DC: 3D608293  lis r11, -0x7d6d
	ctx.r[11].s64 = -2104295424;
	// 826052E0: 388A2BB4  addi r4, r10, 0x2bb4
	ctx.r[4].s64 = ctx.r[10].s64 + 11188;
	// 826052E4: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826052E8: 386B4604  addi r3, r11, 0x4604
	ctx.r[3].s64 = ctx.r[11].s64 + 17924;
	// 826052EC: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826052F0: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826052F4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826052F8: 4BE61B29  bl 0x82466e20
	ctx.lr = 0x826052FC;
	sub_82466E20(ctx, base);
	// 826052FC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82605300: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82605304: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82605308: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82605310(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82605310 size=108
    let mut pc: u32 = 0x82605310;
    'dispatch: loop {
        match pc {
            0x82605310 => {
    //   block [0x82605310..0x8260537C)
	// 82605310: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82605314: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82605318: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8260531C: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 82605320: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82605324: 38EBBB88  addi r7, r11, -0x4478
	ctx.r[7].s64 = ctx.r[11].s64 + -17528;
	// 82605328: 39000006  li r8, 6
	ctx.r[8].s64 = 6;
	// 8260532C: 388A2BC8  addi r4, r10, 0x2bc8
	ctx.r[4].s64 = ctx.r[10].s64 + 11208;
	// 82605330: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82605334: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82605338: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8260533C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82605340: 386A4634  addi r3, r10, 0x4634
	ctx.r[3].s64 = ctx.r[10].s64 + 17972;
	// 82605344: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82605348: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8260534C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82605350: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82605354: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82605358: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8260535C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82605360: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82605364: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82605368: 4BE61AB9  bl 0x82466e20
	ctx.lr = 0x8260536C;
	sub_82466E20(ctx, base);
	// 8260536C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82605370: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82605374: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82605378: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82605380(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82605380 size=112
    let mut pc: u32 = 0x82605380;
    'dispatch: loop {
        match pc {
            0x82605380 => {
    //   block [0x82605380..0x826053F0)
	// 82605380: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82605384: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82605388: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8260538C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82605390: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 82605394: 38AA4604  addi r5, r10, 0x4604
	ctx.r[5].s64 = ctx.r[10].s64 + 17924;
	// 82605398: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8260539C: 390BBC18  addi r8, r11, -0x43e8
	ctx.r[8].s64 = ctx.r[11].s64 + -17384;
	// 826053A0: 3920000C  li r9, 0xc
	ctx.r[9].s64 = 12;
	// 826053A4: 388A2C00  addi r4, r10, 0x2c00
	ctx.r[4].s64 = ctx.r[10].s64 + 11264;
	// 826053A8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826053AC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 826053B0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826053B4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826053B8: 386A4664  addi r3, r10, 0x4664
	ctx.r[3].s64 = ctx.r[10].s64 + 18020;
	// 826053BC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826053C0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826053C4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826053C8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826053CC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826053D0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826053D4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826053D8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826053DC: 4BE61A45  bl 0x82466e20
	ctx.lr = 0x826053E0;
	sub_82466E20(ctx, base);
	// 826053E0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826053E4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826053E8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826053EC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826053F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826053F0 size=112
    let mut pc: u32 = 0x826053F0;
    'dispatch: loop {
        match pc {
            0x826053F0 => {
    //   block [0x826053F0..0x82605460)
	// 826053F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826053F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826053F8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826053FC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82605400: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 82605404: 38AA4604  addi r5, r10, 0x4604
	ctx.r[5].s64 = ctx.r[10].s64 + 17924;
	// 82605408: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8260540C: 390BBD38  addi r8, r11, -0x42c8
	ctx.r[8].s64 = ctx.r[11].s64 + -17096;
	// 82605410: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82605414: 388A2C24  addi r4, r10, 0x2c24
	ctx.r[4].s64 = ctx.r[10].s64 + 11300;
	// 82605418: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8260541C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82605420: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82605424: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82605428: 386A4694  addi r3, r10, 0x4694
	ctx.r[3].s64 = ctx.r[10].s64 + 18068;
	// 8260542C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82605430: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82605434: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82605438: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8260543C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82605440: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82605444: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82605448: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8260544C: 4BE619D5  bl 0x82466e20
	ctx.lr = 0x82605450;
	sub_82466E20(ctx, base);
	// 82605450: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82605454: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82605458: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8260545C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82605460(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82605460 size=108
    let mut pc: u32 = 0x82605460;
    'dispatch: loop {
        match pc {
            0x82605460 => {
    //   block [0x82605460..0x826054CC)
	// 82605460: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82605464: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82605468: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8260546C: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 82605470: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82605474: 38EBBD50  addi r7, r11, -0x42b0
	ctx.r[7].s64 = ctx.r[11].s64 + -17072;
	// 82605478: 39000006  li r8, 6
	ctx.r[8].s64 = 6;
	// 8260547C: 388A2C44  addi r4, r10, 0x2c44
	ctx.r[4].s64 = ctx.r[10].s64 + 11332;
	// 82605480: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82605484: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82605488: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8260548C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82605490: 386A46C4  addi r3, r10, 0x46c4
	ctx.r[3].s64 = ctx.r[10].s64 + 18116;
	// 82605494: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82605498: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8260549C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826054A0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826054A4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826054A8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826054AC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826054B0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826054B4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826054B8: 4BE61969  bl 0x82466e20
	ctx.lr = 0x826054BC;
	sub_82466E20(ctx, base);
	// 826054BC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826054C0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826054C4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826054C8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826054D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826054D0 size=112
    let mut pc: u32 = 0x826054D0;
    'dispatch: loop {
        match pc {
            0x826054D0 => {
    //   block [0x826054D0..0x82605540)
	// 826054D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826054D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826054D8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826054DC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 826054E0: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 826054E4: 38AA4604  addi r5, r10, 0x4604
	ctx.r[5].s64 = ctx.r[10].s64 + 17924;
	// 826054E8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826054EC: 390BBDE0  addi r8, r11, -0x4220
	ctx.r[8].s64 = ctx.r[11].s64 + -16928;
	// 826054F0: 3920000A  li r9, 0xa
	ctx.r[9].s64 = 10;
	// 826054F4: 388A2C74  addi r4, r10, 0x2c74
	ctx.r[4].s64 = ctx.r[10].s64 + 11380;
	// 826054F8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826054FC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82605500: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82605504: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82605508: 386A46F4  addi r3, r10, 0x46f4
	ctx.r[3].s64 = ctx.r[10].s64 + 18164;
	// 8260550C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82605510: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82605514: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82605518: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8260551C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82605520: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82605524: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82605528: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8260552C: 4BE618F5  bl 0x82466e20
	ctx.lr = 0x82605530;
	sub_82466E20(ctx, base);
	// 82605530: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82605534: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82605538: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8260553C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82605540(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82605540 size=108
    let mut pc: u32 = 0x82605540;
    'dispatch: loop {
        match pc {
            0x82605540 => {
    //   block [0x82605540..0x826055AC)
	// 82605540: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82605544: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82605548: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8260554C: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 82605550: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82605554: 38EBBED0  addi r7, r11, -0x4130
	ctx.r[7].s64 = ctx.r[11].s64 + -16688;
	// 82605558: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 8260555C: 388A2C90  addi r4, r10, 0x2c90
	ctx.r[4].s64 = ctx.r[10].s64 + 11408;
	// 82605560: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82605564: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82605568: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8260556C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82605570: 386A4724  addi r3, r10, 0x4724
	ctx.r[3].s64 = ctx.r[10].s64 + 18212;
	// 82605574: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82605578: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8260557C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82605580: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82605584: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82605588: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8260558C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82605590: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82605594: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82605598: 4BE61889  bl 0x82466e20
	ctx.lr = 0x8260559C;
	sub_82466E20(ctx, base);
	// 8260559C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826055A0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826055A4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826055A8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826055B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826055B0 size=108
    let mut pc: u32 = 0x826055B0;
    'dispatch: loop {
        match pc {
            0x826055B0 => {
    //   block [0x826055B0..0x8260561C)
	// 826055B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826055B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826055B8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826055BC: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 826055C0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826055C4: 38EBBEE8  addi r7, r11, -0x4118
	ctx.r[7].s64 = ctx.r[11].s64 + -16664;
	// 826055C8: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 826055CC: 388A2CA8  addi r4, r10, 0x2ca8
	ctx.r[4].s64 = ctx.r[10].s64 + 11432;
	// 826055D0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826055D4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 826055D8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826055DC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826055E0: 386A4754  addi r3, r10, 0x4754
	ctx.r[3].s64 = ctx.r[10].s64 + 18260;
	// 826055E4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826055E8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826055EC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826055F0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826055F4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826055F8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826055FC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82605600: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82605604: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82605608: 4BE61819  bl 0x82466e20
	ctx.lr = 0x8260560C;
	sub_82466E20(ctx, base);
	// 8260560C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82605610: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82605614: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82605618: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


