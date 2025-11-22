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


pub fn sub_825F2348(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F2348 size=116
    let mut pc: u32 = 0x825F2348;
    'dispatch: loop {
        match pc {
            0x825F2348 => {
    //   block [0x825F2348..0x825F23BC)
	// 825F2348: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F234C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F2350: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F2354: 3D408274  lis r10, -0x7d8c
	ctx.r[10].s64 = -2106327040;
	// 825F2358: 38E0000D  li r7, 0xd
	ctx.r[7].s64 = 13;
	// 825F235C: 390AEB58  addi r8, r10, -0x14a8
	ctx.r[8].s64 = ctx.r[10].s64 + -5288;
	// 825F2360: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F2364: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 825F2368: 38AAC2C4  addi r5, r10, -0x3d3c
	ctx.r[5].s64 = ctx.r[10].s64 + -15676;
	// 825F236C: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F2370: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 825F2374: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825F2378: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825F237C: 388A3E64  addi r4, r10, 0x3e64
	ctx.r[4].s64 = ctx.r[10].s64 + 15972;
	// 825F2380: 396B1A2C  addi r11, r11, 0x1a2c
	ctx.r[11].s64 = ctx.r[11].s64 + 6700;
	// 825F2384: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F2388: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 825F238C: 386AC714  addi r3, r10, -0x38ec
	ctx.r[3].s64 = ctx.r[10].s64 + -14572;
	// 825F2390: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 825F2394: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825F2398: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 825F239C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F23A0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F23A4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F23A8: 4BE74A79  bl 0x82466e20
	ctx.lr = 0x825F23AC;
	sub_82466E20(ctx, base);
	// 825F23AC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F23B0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F23B4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F23B8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F23C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F23C0 size=112
    let mut pc: u32 = 0x825F23C0;
    'dispatch: loop {
        match pc {
            0x825F23C0 => {
    //   block [0x825F23C0..0x825F2430)
	// 825F23C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F23C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F23C8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F23CC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F23D0: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825F23D4: 38AAE094  addi r5, r10, -0x1f6c
	ctx.r[5].s64 = ctx.r[10].s64 + -8044;
	// 825F23D8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F23DC: 390BA610  addi r8, r11, -0x59f0
	ctx.r[8].s64 = ctx.r[11].s64 + -23024;
	// 825F23E0: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 825F23E4: 388A3E78  addi r4, r10, 0x3e78
	ctx.r[4].s64 = ctx.r[10].s64 + 15992;
	// 825F23E8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825F23EC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F23F0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825F23F4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F23F8: 386AC744  addi r3, r10, -0x38bc
	ctx.r[3].s64 = ctx.r[10].s64 + -14524;
	// 825F23FC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 825F2400: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825F2404: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825F2408: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825F240C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F2410: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825F2414: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F2418: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F241C: 4BE74A05  bl 0x82466e20
	ctx.lr = 0x825F2420;
	sub_82466E20(ctx, base);
	// 825F2420: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F2424: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F2428: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F242C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F2430(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F2430 size=108
    let mut pc: u32 = 0x825F2430;
    'dispatch: loop {
        match pc {
            0x825F2430 => {
    //   block [0x825F2430..0x825F249C)
	// 825F2430: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F2434: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F2438: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F243C: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825F2440: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F2444: 38EBA640  addi r7, r11, -0x59c0
	ctx.r[7].s64 = ctx.r[11].s64 + -22976;
	// 825F2448: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 825F244C: 388A3E98  addi r4, r10, 0x3e98
	ctx.r[4].s64 = ctx.r[10].s64 + 16024;
	// 825F2450: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825F2454: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F2458: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 825F245C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825F2460: 386AC774  addi r3, r10, -0x388c
	ctx.r[3].s64 = ctx.r[10].s64 + -14476;
	// 825F2464: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 825F2468: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825F246C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F2470: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825F2474: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F2478: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825F247C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F2480: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F2484: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825F2488: 4BE74999  bl 0x82466e20
	ctx.lr = 0x825F248C;
	sub_82466E20(ctx, base);
	// 825F248C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F2490: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F2494: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F2498: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F24A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F24A0 size=112
    let mut pc: u32 = 0x825F24A0;
    'dispatch: loop {
        match pc {
            0x825F24A0 => {
    //   block [0x825F24A0..0x825F2510)
	// 825F24A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F24A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F24A8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F24AC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F24B0: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825F24B4: 38AAC1A4  addi r5, r10, -0x3e5c
	ctx.r[5].s64 = ctx.r[10].s64 + -15964;
	// 825F24B8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F24BC: 390BA670  addi r8, r11, -0x5990
	ctx.r[8].s64 = ctx.r[11].s64 + -22928;
	// 825F24C0: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 825F24C4: 388A3EAC  addi r4, r10, 0x3eac
	ctx.r[4].s64 = ctx.r[10].s64 + 16044;
	// 825F24C8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825F24CC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F24D0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825F24D4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F24D8: 386AC7A4  addi r3, r10, -0x385c
	ctx.r[3].s64 = ctx.r[10].s64 + -14428;
	// 825F24DC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 825F24E0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825F24E4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825F24E8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825F24EC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F24F0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825F24F4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F24F8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F24FC: 4BE74925  bl 0x82466e20
	ctx.lr = 0x825F2500;
	sub_82466E20(ctx, base);
	// 825F2500: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F2504: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F2508: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F250C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F2510(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F2510 size=112
    let mut pc: u32 = 0x825F2510;
    'dispatch: loop {
        match pc {
            0x825F2510 => {
    //   block [0x825F2510..0x825F2580)
	// 825F2510: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F2514: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F2518: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F251C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F2520: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825F2524: 38AAC984  addi r5, r10, -0x367c
	ctx.r[5].s64 = ctx.r[10].s64 + -13948;
	// 825F2528: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F252C: 390BA6A0  addi r8, r11, -0x5960
	ctx.r[8].s64 = ctx.r[11].s64 + -22880;
	// 825F2530: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 825F2534: 388A3EBC  addi r4, r10, 0x3ebc
	ctx.r[4].s64 = ctx.r[10].s64 + 16060;
	// 825F2538: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825F253C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F2540: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825F2544: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F2548: 386AC7D4  addi r3, r10, -0x382c
	ctx.r[3].s64 = ctx.r[10].s64 + -14380;
	// 825F254C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 825F2550: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825F2554: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825F2558: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825F255C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F2560: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825F2564: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F2568: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F256C: 4BE748B5  bl 0x82466e20
	ctx.lr = 0x825F2570;
	sub_82466E20(ctx, base);
	// 825F2570: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F2574: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F2578: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F257C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F2580(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F2580 size=108
    let mut pc: u32 = 0x825F2580;
    'dispatch: loop {
        match pc {
            0x825F2580 => {
    //   block [0x825F2580..0x825F25EC)
	// 825F2580: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F2584: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F2588: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F258C: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825F2590: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F2594: 38EBA6D0  addi r7, r11, -0x5930
	ctx.r[7].s64 = ctx.r[11].s64 + -22832;
	// 825F2598: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 825F259C: 388A3ED0  addi r4, r10, 0x3ed0
	ctx.r[4].s64 = ctx.r[10].s64 + 16080;
	// 825F25A0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825F25A4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F25A8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 825F25AC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825F25B0: 386AC804  addi r3, r10, -0x37fc
	ctx.r[3].s64 = ctx.r[10].s64 + -14332;
	// 825F25B4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 825F25B8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825F25BC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F25C0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825F25C4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F25C8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825F25CC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F25D0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F25D4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825F25D8: 4BE74849  bl 0x82466e20
	ctx.lr = 0x825F25DC;
	sub_82466E20(ctx, base);
	// 825F25DC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F25E0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F25E4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F25E8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F25F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F25F0 size=112
    let mut pc: u32 = 0x825F25F0;
    'dispatch: loop {
        match pc {
            0x825F25F0 => {
    //   block [0x825F25F0..0x825F2660)
	// 825F25F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F25F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F25F8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F25FC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F2600: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825F2604: 38AAC354  addi r5, r10, -0x3cac
	ctx.r[5].s64 = ctx.r[10].s64 + -15532;
	// 825F2608: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F260C: 390BA718  addi r8, r11, -0x58e8
	ctx.r[8].s64 = ctx.r[11].s64 + -22760;
	// 825F2610: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 825F2614: 388A3EF8  addi r4, r10, 0x3ef8
	ctx.r[4].s64 = ctx.r[10].s64 + 16120;
	// 825F2618: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825F261C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F2620: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825F2624: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F2628: 386AC834  addi r3, r10, -0x37cc
	ctx.r[3].s64 = ctx.r[10].s64 + -14284;
	// 825F262C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 825F2630: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825F2634: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825F2638: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825F263C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F2640: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825F2644: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F2648: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F264C: 4BE747D5  bl 0x82466e20
	ctx.lr = 0x825F2650;
	sub_82466E20(ctx, base);
	// 825F2650: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F2654: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F2658: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F265C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F2660(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F2660 size=100
    let mut pc: u32 = 0x825F2660;
    'dispatch: loop {
        match pc {
            0x825F2660 => {
    //   block [0x825F2660..0x825F26C4)
	// 825F2660: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F2664: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F2668: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F266C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F2670: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825F2674: 38AAC1A4  addi r5, r10, -0x3e5c
	ctx.r[5].s64 = ctx.r[10].s64 + -15964;
	// 825F2678: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F267C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825F2680: 388A3F14  addi r4, r10, 0x3f14
	ctx.r[4].s64 = ctx.r[10].s64 + 16148;
	// 825F2684: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F2688: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825F268C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F2690: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825F2694: 386AC864  addi r3, r10, -0x379c
	ctx.r[3].s64 = ctx.r[10].s64 + -14236;
	// 825F2698: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F269C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825F26A0: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 825F26A4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F26A8: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 825F26AC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F26B0: 4BE74771  bl 0x82466e20
	ctx.lr = 0x825F26B4;
	sub_82466E20(ctx, base);
	// 825F26B4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F26B8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F26BC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F26C0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F26C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F26C8 size=112
    let mut pc: u32 = 0x825F26C8;
    'dispatch: loop {
        match pc {
            0x825F26C8 => {
    //   block [0x825F26C8..0x825F2738)
	// 825F26C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F26CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F26D0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F26D4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F26D8: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825F26DC: 38AAC5C4  addi r5, r10, -0x3a3c
	ctx.r[5].s64 = ctx.r[10].s64 + -14908;
	// 825F26E0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F26E4: 390BA778  addi r8, r11, -0x5888
	ctx.r[8].s64 = ctx.r[11].s64 + -22664;
	// 825F26E8: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 825F26EC: 388A3F2C  addi r4, r10, 0x3f2c
	ctx.r[4].s64 = ctx.r[10].s64 + 16172;
	// 825F26F0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825F26F4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F26F8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825F26FC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F2700: 386AC894  addi r3, r10, -0x376c
	ctx.r[3].s64 = ctx.r[10].s64 + -14188;
	// 825F2704: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 825F2708: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825F270C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825F2710: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825F2714: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F2718: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825F271C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F2720: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F2724: 4BE746FD  bl 0x82466e20
	ctx.lr = 0x825F2728;
	sub_82466E20(ctx, base);
	// 825F2728: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F272C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F2730: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F2734: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F2738(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F2738 size=112
    let mut pc: u32 = 0x825F2738;
    'dispatch: loop {
        match pc {
            0x825F2738 => {
    //   block [0x825F2738..0x825F27A8)
	// 825F2738: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F273C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F2740: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F2744: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F2748: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825F274C: 38AAC5C4  addi r5, r10, -0x3a3c
	ctx.r[5].s64 = ctx.r[10].s64 + -14908;
	// 825F2750: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F2754: 390BA7C0  addi r8, r11, -0x5840
	ctx.r[8].s64 = ctx.r[11].s64 + -22592;
	// 825F2758: 39200007  li r9, 7
	ctx.r[9].s64 = 7;
	// 825F275C: 388A3F3C  addi r4, r10, 0x3f3c
	ctx.r[4].s64 = ctx.r[10].s64 + 16188;
	// 825F2760: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825F2764: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F2768: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825F276C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F2770: 386AC8C4  addi r3, r10, -0x373c
	ctx.r[3].s64 = ctx.r[10].s64 + -14140;
	// 825F2774: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 825F2778: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825F277C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825F2780: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825F2784: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F2788: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825F278C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F2790: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F2794: 4BE7468D  bl 0x82466e20
	ctx.lr = 0x825F2798;
	sub_82466E20(ctx, base);
	// 825F2798: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F279C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F27A0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F27A4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F27A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F27A8 size=108
    let mut pc: u32 = 0x825F27A8;
    'dispatch: loop {
        match pc {
            0x825F27A8 => {
    //   block [0x825F27A8..0x825F2814)
	// 825F27A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F27AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F27B0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F27B4: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825F27B8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F27BC: 38EBA868  addi r7, r11, -0x5798
	ctx.r[7].s64 = ctx.r[11].s64 + -22424;
	// 825F27C0: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 825F27C4: 388A3F58  addi r4, r10, 0x3f58
	ctx.r[4].s64 = ctx.r[10].s64 + 16216;
	// 825F27C8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825F27CC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F27D0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 825F27D4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825F27D8: 386AC8F4  addi r3, r10, -0x370c
	ctx.r[3].s64 = ctx.r[10].s64 + -14092;
	// 825F27DC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 825F27E0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825F27E4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F27E8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825F27EC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F27F0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825F27F4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F27F8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F27FC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825F2800: 4BE74621  bl 0x82466e20
	ctx.lr = 0x825F2804;
	sub_82466E20(ctx, base);
	// 825F2804: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F2808: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F280C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F2810: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F2818(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825F2818 size=24
    let mut pc: u32 = 0x825F2818;
    'dispatch: loop {
        match pc {
            0x825F2818 => {
    //   block [0x825F2818..0x825F2830)
	// 825F2818: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825F281C: 3D408274  lis r10, -0x7d8c
	ctx.r[10].s64 = -2106327040;
	// 825F2820: 394AEC90  addi r10, r10, -0x1370
	ctx.r[10].s64 = ctx.r[10].s64 + -4976;
	// 825F2824: 816BAB68  lwz r11, -0x5498(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-21656 as u32) ) } as u64;
	// 825F2828: 916A0068  stw r11, 0x68(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(104 as u32), ctx.r[11].u32 ) };
	// 825F282C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F2830(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F2830 size=116
    let mut pc: u32 = 0x825F2830;
    'dispatch: loop {
        match pc {
            0x825F2830 => {
    //   block [0x825F2830..0x825F28A4)
	// 825F2830: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F2834: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F2838: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F283C: 3D408274  lis r10, -0x7d8c
	ctx.r[10].s64 = -2106327040;
	// 825F2840: 38E00005  li r7, 5
	ctx.r[7].s64 = 5;
	// 825F2844: 390AEC90  addi r8, r10, -0x1370
	ctx.r[8].s64 = ctx.r[10].s64 + -4976;
	// 825F2848: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F284C: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 825F2850: 38AAC324  addi r5, r10, -0x3cdc
	ctx.r[5].s64 = ctx.r[10].s64 + -15580;
	// 825F2854: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F2858: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 825F285C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825F2860: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825F2864: 388A3F74  addi r4, r10, 0x3f74
	ctx.r[4].s64 = ctx.r[10].s64 + 16244;
	// 825F2868: 396B1A64  addi r11, r11, 0x1a64
	ctx.r[11].s64 = ctx.r[11].s64 + 6756;
	// 825F286C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F2870: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F2874: 386AC924  addi r3, r10, -0x36dc
	ctx.r[3].s64 = ctx.r[10].s64 + -14044;
	// 825F2878: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 825F287C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825F2880: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 825F2884: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F2888: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F288C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F2890: 4BE74591  bl 0x82466e20
	ctx.lr = 0x825F2894;
	sub_82466E20(ctx, base);
	// 825F2894: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F2898: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F289C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F28A0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F28A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F28A8 size=100
    let mut pc: u32 = 0x825F28A8;
    'dispatch: loop {
        match pc {
            0x825F28A8 => {
    //   block [0x825F28A8..0x825F290C)
	// 825F28A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F28AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F28B0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F28B4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F28B8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825F28BC: 38AAC354  addi r5, r10, -0x3cac
	ctx.r[5].s64 = ctx.r[10].s64 + -15532;
	// 825F28C0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F28C4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825F28C8: 388A3F88  addi r4, r10, 0x3f88
	ctx.r[4].s64 = ctx.r[10].s64 + 16264;
	// 825F28CC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F28D0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825F28D4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F28D8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825F28DC: 386AC954  addi r3, r10, -0x36ac
	ctx.r[3].s64 = ctx.r[10].s64 + -13996;
	// 825F28E0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F28E4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825F28E8: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 825F28EC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F28F0: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 825F28F4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F28F8: 4BE74529  bl 0x82466e20
	ctx.lr = 0x825F28FC;
	sub_82466E20(ctx, base);
	// 825F28FC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F2900: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F2904: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F2908: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F2910(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F2910 size=100
    let mut pc: u32 = 0x825F2910;
    'dispatch: loop {
        match pc {
            0x825F2910 => {
    //   block [0x825F2910..0x825F2974)
	// 825F2910: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F2914: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F2918: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F291C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F2920: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825F2924: 38AAC1A4  addi r5, r10, -0x3e5c
	ctx.r[5].s64 = ctx.r[10].s64 + -15964;
	// 825F2928: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F292C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825F2930: 388A3F98  addi r4, r10, 0x3f98
	ctx.r[4].s64 = ctx.r[10].s64 + 16280;
	// 825F2934: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F2938: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825F293C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F2940: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825F2944: 386AC984  addi r3, r10, -0x367c
	ctx.r[3].s64 = ctx.r[10].s64 + -13948;
	// 825F2948: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F294C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825F2950: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 825F2954: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F2958: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 825F295C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F2960: 4BE744C1  bl 0x82466e20
	ctx.lr = 0x825F2964;
	sub_82466E20(ctx, base);
	// 825F2964: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F2968: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F296C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F2970: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F2978(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F2978 size=112
    let mut pc: u32 = 0x825F2978;
    'dispatch: loop {
        match pc {
            0x825F2978 => {
    //   block [0x825F2978..0x825F29E8)
	// 825F2978: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F297C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F2980: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F2984: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F2988: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825F298C: 38AAAF74  addi r5, r10, -0x508c
	ctx.r[5].s64 = ctx.r[10].s64 + -20620;
	// 825F2990: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F2994: 390BA8C8  addi r8, r11, -0x5738
	ctx.r[8].s64 = ctx.r[11].s64 + -22328;
	// 825F2998: 39200006  li r9, 6
	ctx.r[9].s64 = 6;
	// 825F299C: 388A3FAC  addi r4, r10, 0x3fac
	ctx.r[4].s64 = ctx.r[10].s64 + 16300;
	// 825F29A0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825F29A4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F29A8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825F29AC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F29B0: 386AC9B4  addi r3, r10, -0x364c
	ctx.r[3].s64 = ctx.r[10].s64 + -13900;
	// 825F29B4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 825F29B8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825F29BC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825F29C0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825F29C4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F29C8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825F29CC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F29D0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F29D4: 4BE7444D  bl 0x82466e20
	ctx.lr = 0x825F29D8;
	sub_82466E20(ctx, base);
	// 825F29D8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F29DC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F29E0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F29E4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F29E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F29E8 size=112
    let mut pc: u32 = 0x825F29E8;
    'dispatch: loop {
        match pc {
            0x825F29E8 => {
    //   block [0x825F29E8..0x825F2A58)
	// 825F29E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F29EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F29F0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F29F4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F29F8: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825F29FC: 38AAAF74  addi r5, r10, -0x508c
	ctx.r[5].s64 = ctx.r[10].s64 + -20620;
	// 825F2A00: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F2A04: 390BA958  addi r8, r11, -0x56a8
	ctx.r[8].s64 = ctx.r[11].s64 + -22184;
	// 825F2A08: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 825F2A0C: 388A3FDC  addi r4, r10, 0x3fdc
	ctx.r[4].s64 = ctx.r[10].s64 + 16348;
	// 825F2A10: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825F2A14: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F2A18: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825F2A1C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F2A20: 386AC9E4  addi r3, r10, -0x361c
	ctx.r[3].s64 = ctx.r[10].s64 + -13852;
	// 825F2A24: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 825F2A28: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825F2A2C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825F2A30: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825F2A34: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F2A38: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825F2A3C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F2A40: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F2A44: 4BE743DD  bl 0x82466e20
	ctx.lr = 0x825F2A48;
	sub_82466E20(ctx, base);
	// 825F2A48: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F2A4C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F2A50: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F2A54: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F2A58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F2A58 size=112
    let mut pc: u32 = 0x825F2A58;
    'dispatch: loop {
        match pc {
            0x825F2A58 => {
    //   block [0x825F2A58..0x825F2AC8)
	// 825F2A58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F2A5C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F2A60: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F2A64: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F2A68: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825F2A6C: 38AAC564  addi r5, r10, -0x3a9c
	ctx.r[5].s64 = ctx.r[10].s64 + -15004;
	// 825F2A70: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F2A74: 390BA9B8  addi r8, r11, -0x5648
	ctx.r[8].s64 = ctx.r[11].s64 + -22088;
	// 825F2A78: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 825F2A7C: 388A400C  addi r4, r10, 0x400c
	ctx.r[4].s64 = ctx.r[10].s64 + 16396;
	// 825F2A80: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825F2A84: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F2A88: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825F2A8C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F2A90: 386ACA14  addi r3, r10, -0x35ec
	ctx.r[3].s64 = ctx.r[10].s64 + -13804;
	// 825F2A94: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 825F2A98: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825F2A9C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825F2AA0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825F2AA4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F2AA8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825F2AAC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F2AB0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F2AB4: 4BE7436D  bl 0x82466e20
	ctx.lr = 0x825F2AB8;
	sub_82466E20(ctx, base);
	// 825F2AB8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F2ABC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F2AC0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F2AC4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F2AC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F2AC8 size=112
    let mut pc: u32 = 0x825F2AC8;
    'dispatch: loop {
        match pc {
            0x825F2AC8 => {
    //   block [0x825F2AC8..0x825F2B38)
	// 825F2AC8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F2ACC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F2AD0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F2AD4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F2AD8: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825F2ADC: 38AAAF74  addi r5, r10, -0x508c
	ctx.r[5].s64 = ctx.r[10].s64 + -20620;
	// 825F2AE0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F2AE4: 390BA9E8  addi r8, r11, -0x5618
	ctx.r[8].s64 = ctx.r[11].s64 + -22040;
	// 825F2AE8: 39200006  li r9, 6
	ctx.r[9].s64 = 6;
	// 825F2AEC: 388A4028  addi r4, r10, 0x4028
	ctx.r[4].s64 = ctx.r[10].s64 + 16424;
	// 825F2AF0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825F2AF4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F2AF8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825F2AFC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F2B00: 386ACA44  addi r3, r10, -0x35bc
	ctx.r[3].s64 = ctx.r[10].s64 + -13756;
	// 825F2B04: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 825F2B08: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825F2B0C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825F2B10: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825F2B14: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F2B18: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825F2B1C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F2B20: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F2B24: 4BE742FD  bl 0x82466e20
	ctx.lr = 0x825F2B28;
	sub_82466E20(ctx, base);
	// 825F2B28: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F2B2C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F2B30: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F2B34: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F2B38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F2B38 size=112
    let mut pc: u32 = 0x825F2B38;
    'dispatch: loop {
        match pc {
            0x825F2B38 => {
    //   block [0x825F2B38..0x825F2BA8)
	// 825F2B38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F2B3C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F2B40: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F2B44: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F2B48: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825F2B4C: 38AAC6B4  addi r5, r10, -0x394c
	ctx.r[5].s64 = ctx.r[10].s64 + -14668;
	// 825F2B50: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F2B54: 390BAA78  addi r8, r11, -0x5588
	ctx.r[8].s64 = ctx.r[11].s64 + -21896;
	// 825F2B58: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 825F2B5C: 388A404C  addi r4, r10, 0x404c
	ctx.r[4].s64 = ctx.r[10].s64 + 16460;
	// 825F2B60: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825F2B64: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F2B68: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825F2B6C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F2B70: 386ACA74  addi r3, r10, -0x358c
	ctx.r[3].s64 = ctx.r[10].s64 + -13708;
	// 825F2B74: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 825F2B78: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825F2B7C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825F2B80: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825F2B84: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F2B88: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825F2B8C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F2B90: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F2B94: 4BE7428D  bl 0x82466e20
	ctx.lr = 0x825F2B98;
	sub_82466E20(ctx, base);
	// 825F2B98: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F2B9C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F2BA0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F2BA4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F2BA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F2BA8 size=112
    let mut pc: u32 = 0x825F2BA8;
    'dispatch: loop {
        match pc {
            0x825F2BA8 => {
    //   block [0x825F2BA8..0x825F2C18)
	// 825F2BA8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F2BAC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F2BB0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F2BB4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F2BB8: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825F2BBC: 38AAC8C4  addi r5, r10, -0x373c
	ctx.r[5].s64 = ctx.r[10].s64 + -14140;
	// 825F2BC0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F2BC4: 390BAA90  addi r8, r11, -0x5570
	ctx.r[8].s64 = ctx.r[11].s64 + -21872;
	// 825F2BC8: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 825F2BCC: 388A4060  addi r4, r10, 0x4060
	ctx.r[4].s64 = ctx.r[10].s64 + 16480;
	// 825F2BD0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825F2BD4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F2BD8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825F2BDC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F2BE0: 386ACAA4  addi r3, r10, -0x355c
	ctx.r[3].s64 = ctx.r[10].s64 + -13660;
	// 825F2BE4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 825F2BE8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825F2BEC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825F2BF0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825F2BF4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F2BF8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825F2BFC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F2C00: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F2C04: 4BE7421D  bl 0x82466e20
	ctx.lr = 0x825F2C08;
	sub_82466E20(ctx, base);
	// 825F2C08: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F2C0C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F2C10: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F2C14: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F2C18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F2C18 size=112
    let mut pc: u32 = 0x825F2C18;
    'dispatch: loop {
        match pc {
            0x825F2C18 => {
    //   block [0x825F2C18..0x825F2C88)
	// 825F2C18: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F2C1C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F2C20: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F2C24: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F2C28: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825F2C2C: 38AAC1A4  addi r5, r10, -0x3e5c
	ctx.r[5].s64 = ctx.r[10].s64 + -15964;
	// 825F2C30: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F2C34: 390BAAC0  addi r8, r11, -0x5540
	ctx.r[8].s64 = ctx.r[11].s64 + -21824;
	// 825F2C38: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 825F2C3C: 388A4084  addi r4, r10, 0x4084
	ctx.r[4].s64 = ctx.r[10].s64 + 16516;
	// 825F2C40: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825F2C44: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F2C48: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825F2C4C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F2C50: 386ACAD4  addi r3, r10, -0x352c
	ctx.r[3].s64 = ctx.r[10].s64 + -13612;
	// 825F2C54: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 825F2C58: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825F2C5C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825F2C60: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825F2C64: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F2C68: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825F2C6C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F2C70: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F2C74: 4BE741AD  bl 0x82466e20
	ctx.lr = 0x825F2C78;
	sub_82466E20(ctx, base);
	// 825F2C78: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F2C7C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F2C80: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F2C84: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F2C88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825F2C88 size=24
    let mut pc: u32 = 0x825F2C88;
    'dispatch: loop {
        match pc {
            0x825F2C88 => {
    //   block [0x825F2C88..0x825F2CA0)
	// 825F2C88: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825F2C8C: 3D408274  lis r10, -0x7d8c
	ctx.r[10].s64 = -2106327040;
	// 825F2C90: 394AED08  addi r10, r10, -0x12f8
	ctx.r[10].s64 = ctx.r[10].s64 + -4856;
	// 825F2C94: 816BAB68  lwz r11, -0x5498(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-21656 as u32) ) } as u64;
	// 825F2C98: 916A0020  stw r11, 0x20(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 825F2C9C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F2CA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F2CA0 size=116
    let mut pc: u32 = 0x825F2CA0;
    'dispatch: loop {
        match pc {
            0x825F2CA0 => {
    //   block [0x825F2CA0..0x825F2D14)
	// 825F2CA0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F2CA4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F2CA8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F2CAC: 3D408274  lis r10, -0x7d8c
	ctx.r[10].s64 = -2106327040;
	// 825F2CB0: 38E00007  li r7, 7
	ctx.r[7].s64 = 7;
	// 825F2CB4: 390AED08  addi r8, r10, -0x12f8
	ctx.r[8].s64 = ctx.r[10].s64 + -4856;
	// 825F2CB8: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F2CBC: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 825F2CC0: 38AAC354  addi r5, r10, -0x3cac
	ctx.r[5].s64 = ctx.r[10].s64 + -15532;
	// 825F2CC4: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F2CC8: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 825F2CCC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825F2CD0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825F2CD4: 388A40B8  addi r4, r10, 0x40b8
	ctx.r[4].s64 = ctx.r[10].s64 + 16568;
	// 825F2CD8: 396B1A7C  addi r11, r11, 0x1a7c
	ctx.r[11].s64 = ctx.r[11].s64 + 6780;
	// 825F2CDC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F2CE0: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F2CE4: 386ACB04  addi r3, r10, -0x34fc
	ctx.r[3].s64 = ctx.r[10].s64 + -13564;
	// 825F2CE8: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 825F2CEC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825F2CF0: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 825F2CF4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F2CF8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F2CFC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F2D00: 4BE74121  bl 0x82466e20
	ctx.lr = 0x825F2D04;
	sub_82466E20(ctx, base);
	// 825F2D04: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F2D08: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F2D0C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F2D10: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F2D18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F2D18 size=112
    let mut pc: u32 = 0x825F2D18;
    'dispatch: loop {
        match pc {
            0x825F2D18 => {
    //   block [0x825F2D18..0x825F2D88)
	// 825F2D18: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F2D1C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F2D20: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F2D24: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F2D28: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825F2D2C: 38AAC2C4  addi r5, r10, -0x3d3c
	ctx.r[5].s64 = ctx.r[10].s64 + -15676;
	// 825F2D30: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F2D34: 390BAB08  addi r8, r11, -0x54f8
	ctx.r[8].s64 = ctx.r[11].s64 + -21752;
	// 825F2D38: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 825F2D3C: 388A40C8  addi r4, r10, 0x40c8
	ctx.r[4].s64 = ctx.r[10].s64 + 16584;
	// 825F2D40: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825F2D44: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F2D48: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825F2D4C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F2D50: 386ACB34  addi r3, r10, -0x34cc
	ctx.r[3].s64 = ctx.r[10].s64 + -13516;
	// 825F2D54: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 825F2D58: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825F2D5C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825F2D60: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825F2D64: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F2D68: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825F2D6C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F2D70: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F2D74: 4BE740AD  bl 0x82466e20
	ctx.lr = 0x825F2D78;
	sub_82466E20(ctx, base);
	// 825F2D78: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F2D7C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F2D80: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F2D84: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F2D88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F2D88 size=112
    let mut pc: u32 = 0x825F2D88;
    'dispatch: loop {
        match pc {
            0x825F2D88 => {
    //   block [0x825F2D88..0x825F2DF8)
	// 825F2D88: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F2D8C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F2D90: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F2D94: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F2D98: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825F2D9C: 38AAC324  addi r5, r10, -0x3cdc
	ctx.r[5].s64 = ctx.r[10].s64 + -15580;
	// 825F2DA0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F2DA4: 390BAB38  addi r8, r11, -0x54c8
	ctx.r[8].s64 = ctx.r[11].s64 + -21704;
	// 825F2DA8: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 825F2DAC: 388A40EC  addi r4, r10, 0x40ec
	ctx.r[4].s64 = ctx.r[10].s64 + 16620;
	// 825F2DB0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825F2DB4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F2DB8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825F2DBC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F2DC0: 386ACB64  addi r3, r10, -0x349c
	ctx.r[3].s64 = ctx.r[10].s64 + -13468;
	// 825F2DC4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 825F2DC8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825F2DCC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825F2DD0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825F2DD4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F2DD8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825F2DDC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F2DE0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F2DE4: 4BE7403D  bl 0x82466e20
	ctx.lr = 0x825F2DE8;
	sub_82466E20(ctx, base);
	// 825F2DE8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F2DEC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F2DF0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F2DF4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F2DF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F2DF8 size=100
    let mut pc: u32 = 0x825F2DF8;
    'dispatch: loop {
        match pc {
            0x825F2DF8 => {
    //   block [0x825F2DF8..0x825F2E5C)
	// 825F2DF8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F2DFC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F2E00: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F2E04: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F2E08: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825F2E0C: 392A1AEC  addi r9, r10, 0x1aec
	ctx.r[9].s64 = ctx.r[10].s64 + 6892;
	// 825F2E10: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F2E14: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F2E18: 388A4110  addi r4, r10, 0x4110
	ctx.r[4].s64 = ctx.r[10].s64 + 16656;
	// 825F2E1C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F2E20: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825F2E24: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F2E28: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825F2E2C: 386ACB94  addi r3, r10, -0x346c
	ctx.r[3].s64 = ctx.r[10].s64 + -13420;
	// 825F2E30: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F2E34: 39400003  li r10, 3
	ctx.r[10].s64 = 3;
	// 825F2E38: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 825F2E3C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F2E40: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 825F2E44: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825F2E48: 4BE73FD9  bl 0x82466e20
	ctx.lr = 0x825F2E4C;
	sub_82466E20(ctx, base);
	// 825F2E4C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F2E50: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F2E54: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F2E58: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F2E60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825F2E60 size=24
    let mut pc: u32 = 0x825F2E60;
    'dispatch: loop {
        match pc {
            0x825F2E60 => {
    //   block [0x825F2E60..0x825F2E78)
	// 825F2E60: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825F2E64: 3D408274  lis r10, -0x7d8c
	ctx.r[10].s64 = -2106327040;
	// 825F2E68: 394AEDB0  addi r10, r10, -0x1250
	ctx.r[10].s64 = ctx.r[10].s64 + -4688;
	// 825F2E6C: 816BAB74  lwz r11, -0x548c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-21644 as u32) ) } as u64;
	// 825F2E70: 916A0008  stw r11, 8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 825F2E74: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F2E78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F2E78 size=112
    let mut pc: u32 = 0x825F2E78;
    'dispatch: loop {
        match pc {
            0x825F2E78 => {
    //   block [0x825F2E78..0x825F2EE8)
	// 825F2E78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F2E7C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F2E80: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F2E84: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F2E88: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825F2E8C: 392A1C28  addi r9, r10, 0x1c28
	ctx.r[9].s64 = ctx.r[10].s64 + 7208;
	// 825F2E90: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F2E94: 390BEDB0  addi r8, r11, -0x1250
	ctx.r[8].s64 = ctx.r[11].s64 + -4688;
	// 825F2E98: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
	// 825F2E9C: 388A4124  addi r4, r10, 0x4124
	ctx.r[4].s64 = ctx.r[10].s64 + 16676;
	// 825F2EA0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825F2EA4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F2EA8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825F2EAC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F2EB0: 386ACBC4  addi r3, r10, -0x343c
	ctx.r[3].s64 = ctx.r[10].s64 + -13372;
	// 825F2EB4: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 825F2EB8: 39400002  li r10, 2
	ctx.r[10].s64 = 2;
	// 825F2EBC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F2EC0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825F2EC4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F2EC8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825F2ECC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825F2ED0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F2ED4: 4BE73F4D  bl 0x82466e20
	ctx.lr = 0x825F2ED8;
	sub_82466E20(ctx, base);
	// 825F2ED8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F2EDC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F2EE0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F2EE4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F2EE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F2EE8 size=112
    let mut pc: u32 = 0x825F2EE8;
    'dispatch: loop {
        match pc {
            0x825F2EE8 => {
    //   block [0x825F2EE8..0x825F2F58)
	// 825F2EE8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F2EEC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F2EF0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F2EF4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F2EF8: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825F2EFC: 38AACBC4  addi r5, r10, -0x343c
	ctx.r[5].s64 = ctx.r[10].s64 + -13372;
	// 825F2F00: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F2F04: 390BAB7C  addi r8, r11, -0x5484
	ctx.r[8].s64 = ctx.r[11].s64 + -21636;
	// 825F2F08: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 825F2F0C: 388A4138  addi r4, r10, 0x4138
	ctx.r[4].s64 = ctx.r[10].s64 + 16696;
	// 825F2F10: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825F2F14: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F2F18: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825F2F1C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F2F20: 386ACBF4  addi r3, r10, -0x340c
	ctx.r[3].s64 = ctx.r[10].s64 + -13324;
	// 825F2F24: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 825F2F28: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825F2F2C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825F2F30: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825F2F34: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F2F38: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825F2F3C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F2F40: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F2F44: 4BE73EDD  bl 0x82466e20
	ctx.lr = 0x825F2F48;
	sub_82466E20(ctx, base);
	// 825F2F48: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F2F4C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F2F50: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F2F54: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F2F58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F2F58 size=108
    let mut pc: u32 = 0x825F2F58;
    'dispatch: loop {
        match pc {
            0x825F2F58 => {
    //   block [0x825F2F58..0x825F2FC4)
	// 825F2F58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F2F5C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F2F60: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F2F64: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825F2F68: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F2F6C: 38EBABAC  addi r7, r11, -0x5454
	ctx.r[7].s64 = ctx.r[11].s64 + -21588;
	// 825F2F70: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 825F2F74: 388A4150  addi r4, r10, 0x4150
	ctx.r[4].s64 = ctx.r[10].s64 + 16720;
	// 825F2F78: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825F2F7C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F2F80: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 825F2F84: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825F2F88: 386ACC24  addi r3, r10, -0x33dc
	ctx.r[3].s64 = ctx.r[10].s64 + -13276;
	// 825F2F8C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 825F2F90: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825F2F94: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F2F98: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825F2F9C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F2FA0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825F2FA4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F2FA8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F2FAC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825F2FB0: 4BE73E71  bl 0x82466e20
	ctx.lr = 0x825F2FB4;
	sub_82466E20(ctx, base);
	// 825F2FB4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F2FB8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F2FBC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F2FC0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F2FC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F2FC8 size=100
    let mut pc: u32 = 0x825F2FC8;
    'dispatch: loop {
        match pc {
            0x825F2FC8 => {
    //   block [0x825F2FC8..0x825F302C)
	// 825F2FC8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F2FCC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F2FD0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F2FD4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F2FD8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825F2FDC: 38AACBC4  addi r5, r10, -0x343c
	ctx.r[5].s64 = ctx.r[10].s64 + -13372;
	// 825F2FE0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F2FE4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825F2FE8: 388A4160  addi r4, r10, 0x4160
	ctx.r[4].s64 = ctx.r[10].s64 + 16736;
	// 825F2FEC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F2FF0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825F2FF4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F2FF8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825F2FFC: 386ACC54  addi r3, r10, -0x33ac
	ctx.r[3].s64 = ctx.r[10].s64 + -13228;
	// 825F3000: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F3004: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825F3008: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 825F300C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F3010: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 825F3014: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F3018: 4BE73E09  bl 0x82466e20
	ctx.lr = 0x825F301C;
	sub_82466E20(ctx, base);
	// 825F301C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F3020: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F3024: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F3028: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F3030(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F3030 size=112
    let mut pc: u32 = 0x825F3030;
    'dispatch: loop {
        match pc {
            0x825F3030 => {
    //   block [0x825F3030..0x825F30A0)
	// 825F3030: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F3034: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F3038: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F303C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F3040: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825F3044: 38AACBC4  addi r5, r10, -0x343c
	ctx.r[5].s64 = ctx.r[10].s64 + -13372;
	// 825F3048: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F304C: 390BABC4  addi r8, r11, -0x543c
	ctx.r[8].s64 = ctx.r[11].s64 + -21564;
	// 825F3050: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 825F3054: 388A417C  addi r4, r10, 0x417c
	ctx.r[4].s64 = ctx.r[10].s64 + 16764;
	// 825F3058: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825F305C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F3060: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825F3064: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F3068: 386ACC84  addi r3, r10, -0x337c
	ctx.r[3].s64 = ctx.r[10].s64 + -13180;
	// 825F306C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 825F3070: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825F3074: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825F3078: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825F307C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F3080: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825F3084: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F3088: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F308C: 4BE73D95  bl 0x82466e20
	ctx.lr = 0x825F3090;
	sub_82466E20(ctx, base);
	// 825F3090: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F3094: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F3098: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F309C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F30A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F30A0 size=112
    let mut pc: u32 = 0x825F30A0;
    'dispatch: loop {
        match pc {
            0x825F30A0 => {
    //   block [0x825F30A0..0x825F3110)
	// 825F30A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F30A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F30A8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F30AC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F30B0: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825F30B4: 38AACBC4  addi r5, r10, -0x343c
	ctx.r[5].s64 = ctx.r[10].s64 + -13372;
	// 825F30B8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F30BC: 390BABDC  addi r8, r11, -0x5424
	ctx.r[8].s64 = ctx.r[11].s64 + -21540;
	// 825F30C0: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 825F30C4: 388A4198  addi r4, r10, 0x4198
	ctx.r[4].s64 = ctx.r[10].s64 + 16792;
	// 825F30C8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825F30CC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F30D0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825F30D4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F30D8: 386ACCB4  addi r3, r10, -0x334c
	ctx.r[3].s64 = ctx.r[10].s64 + -13132;
	// 825F30DC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 825F30E0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825F30E4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825F30E8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825F30EC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F30F0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825F30F4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F30F8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F30FC: 4BE73D25  bl 0x82466e20
	ctx.lr = 0x825F3100;
	sub_82466E20(ctx, base);
	// 825F3100: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F3104: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F3108: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F310C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F3110(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F3110 size=112
    let mut pc: u32 = 0x825F3110;
    'dispatch: loop {
        match pc {
            0x825F3110 => {
    //   block [0x825F3110..0x825F3180)
	// 825F3110: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F3114: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F3118: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F311C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F3120: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825F3124: 38AACBC4  addi r5, r10, -0x343c
	ctx.r[5].s64 = ctx.r[10].s64 + -13372;
	// 825F3128: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F312C: 390BAC0C  addi r8, r11, -0x53f4
	ctx.r[8].s64 = ctx.r[11].s64 + -21492;
	// 825F3130: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 825F3134: 388A41BC  addi r4, r10, 0x41bc
	ctx.r[4].s64 = ctx.r[10].s64 + 16828;
	// 825F3138: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825F313C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F3140: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825F3144: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F3148: 386ACCE4  addi r3, r10, -0x331c
	ctx.r[3].s64 = ctx.r[10].s64 + -13084;
	// 825F314C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 825F3150: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825F3154: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825F3158: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825F315C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F3160: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825F3164: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F3168: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F316C: 4BE73CB5  bl 0x82466e20
	ctx.lr = 0x825F3170;
	sub_82466E20(ctx, base);
	// 825F3170: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F3174: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F3178: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F317C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F3180(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F3180 size=112
    let mut pc: u32 = 0x825F3180;
    'dispatch: loop {
        match pc {
            0x825F3180 => {
    //   block [0x825F3180..0x825F31F0)
	// 825F3180: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F3184: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F3188: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F318C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F3190: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825F3194: 38AACBC4  addi r5, r10, -0x343c
	ctx.r[5].s64 = ctx.r[10].s64 + -13372;
	// 825F3198: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F319C: 390BAC3C  addi r8, r11, -0x53c4
	ctx.r[8].s64 = ctx.r[11].s64 + -21444;
	// 825F31A0: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 825F31A4: 388A41E4  addi r4, r10, 0x41e4
	ctx.r[4].s64 = ctx.r[10].s64 + 16868;
	// 825F31A8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825F31AC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F31B0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825F31B4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F31B8: 386ACD14  addi r3, r10, -0x32ec
	ctx.r[3].s64 = ctx.r[10].s64 + -13036;
	// 825F31BC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 825F31C0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825F31C4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825F31C8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825F31CC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F31D0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825F31D4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F31D8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F31DC: 4BE73C45  bl 0x82466e20
	ctx.lr = 0x825F31E0;
	sub_82466E20(ctx, base);
	// 825F31E0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F31E4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F31E8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F31EC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F31F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F31F0 size=112
    let mut pc: u32 = 0x825F31F0;
    'dispatch: loop {
        match pc {
            0x825F31F0 => {
    //   block [0x825F31F0..0x825F3260)
	// 825F31F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F31F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F31F8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F31FC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F3200: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825F3204: 38AACBC4  addi r5, r10, -0x343c
	ctx.r[5].s64 = ctx.r[10].s64 + -13372;
	// 825F3208: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F320C: 390BAC6C  addi r8, r11, -0x5394
	ctx.r[8].s64 = ctx.r[11].s64 + -21396;
	// 825F3210: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 825F3214: 388A4208  addi r4, r10, 0x4208
	ctx.r[4].s64 = ctx.r[10].s64 + 16904;
	// 825F3218: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825F321C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F3220: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825F3224: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F3228: 386ACD44  addi r3, r10, -0x32bc
	ctx.r[3].s64 = ctx.r[10].s64 + -12988;
	// 825F322C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 825F3230: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825F3234: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825F3238: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825F323C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F3240: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825F3244: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F3248: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F324C: 4BE73BD5  bl 0x82466e20
	ctx.lr = 0x825F3250;
	sub_82466E20(ctx, base);
	// 825F3250: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F3254: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F3258: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F325C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F3260(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F3260 size=112
    let mut pc: u32 = 0x825F3260;
    'dispatch: loop {
        match pc {
            0x825F3260 => {
    //   block [0x825F3260..0x825F32D0)
	// 825F3260: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F3264: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F3268: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F326C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F3270: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825F3274: 38AACBC4  addi r5, r10, -0x343c
	ctx.r[5].s64 = ctx.r[10].s64 + -13372;
	// 825F3278: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F327C: 390BAC84  addi r8, r11, -0x537c
	ctx.r[8].s64 = ctx.r[11].s64 + -21372;
	// 825F3280: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 825F3284: 388A4228  addi r4, r10, 0x4228
	ctx.r[4].s64 = ctx.r[10].s64 + 16936;
	// 825F3288: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825F328C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F3290: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825F3294: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F3298: 386ACD74  addi r3, r10, -0x328c
	ctx.r[3].s64 = ctx.r[10].s64 + -12940;
	// 825F329C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 825F32A0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825F32A4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825F32A8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825F32AC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F32B0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825F32B4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F32B8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F32BC: 4BE73B65  bl 0x82466e20
	ctx.lr = 0x825F32C0;
	sub_82466E20(ctx, base);
	// 825F32C0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F32C4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F32C8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F32CC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F32D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F32D0 size=112
    let mut pc: u32 = 0x825F32D0;
    'dispatch: loop {
        match pc {
            0x825F32D0 => {
    //   block [0x825F32D0..0x825F3340)
	// 825F32D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F32D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F32D8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F32DC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F32E0: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825F32E4: 38AACBC4  addi r5, r10, -0x343c
	ctx.r[5].s64 = ctx.r[10].s64 + -13372;
	// 825F32E8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F32EC: 390BACA0  addi r8, r11, -0x5360
	ctx.r[8].s64 = ctx.r[11].s64 + -21344;
	// 825F32F0: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 825F32F4: 388A423C  addi r4, r10, 0x423c
	ctx.r[4].s64 = ctx.r[10].s64 + 16956;
	// 825F32F8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825F32FC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F3300: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825F3304: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F3308: 386ACDA4  addi r3, r10, -0x325c
	ctx.r[3].s64 = ctx.r[10].s64 + -12892;
	// 825F330C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 825F3310: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825F3314: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825F3318: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825F331C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F3320: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825F3324: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F3328: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F332C: 4BE73AF5  bl 0x82466e20
	ctx.lr = 0x825F3330;
	sub_82466E20(ctx, base);
	// 825F3330: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F3334: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F3338: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F333C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F3340(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F3340 size=112
    let mut pc: u32 = 0x825F3340;
    'dispatch: loop {
        match pc {
            0x825F3340 => {
    //   block [0x825F3340..0x825F33B0)
	// 825F3340: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F3344: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F3348: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F334C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F3350: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825F3354: 38AACBC4  addi r5, r10, -0x343c
	ctx.r[5].s64 = ctx.r[10].s64 + -13372;
	// 825F3358: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F335C: 390BACE8  addi r8, r11, -0x5318
	ctx.r[8].s64 = ctx.r[11].s64 + -21272;
	// 825F3360: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 825F3364: 388A4254  addi r4, r10, 0x4254
	ctx.r[4].s64 = ctx.r[10].s64 + 16980;
	// 825F3368: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825F336C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F3370: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825F3374: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F3378: 386ACDD4  addi r3, r10, -0x322c
	ctx.r[3].s64 = ctx.r[10].s64 + -12844;
	// 825F337C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 825F3380: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825F3384: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825F3388: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825F338C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F3390: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825F3394: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F3398: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F339C: 4BE73A85  bl 0x82466e20
	ctx.lr = 0x825F33A0;
	sub_82466E20(ctx, base);
	// 825F33A0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F33A4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F33A8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F33AC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F33B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F33B0 size=112
    let mut pc: u32 = 0x825F33B0;
    'dispatch: loop {
        match pc {
            0x825F33B0 => {
    //   block [0x825F33B0..0x825F3420)
	// 825F33B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F33B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F33B8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F33BC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F33C0: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825F33C4: 38AACBC4  addi r5, r10, -0x343c
	ctx.r[5].s64 = ctx.r[10].s64 + -13372;
	// 825F33C8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F33CC: 390BAD30  addi r8, r11, -0x52d0
	ctx.r[8].s64 = ctx.r[11].s64 + -21200;
	// 825F33D0: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 825F33D4: 388A4270  addi r4, r10, 0x4270
	ctx.r[4].s64 = ctx.r[10].s64 + 17008;
	// 825F33D8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825F33DC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F33E0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825F33E4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F33E8: 386ACE04  addi r3, r10, -0x31fc
	ctx.r[3].s64 = ctx.r[10].s64 + -12796;
	// 825F33EC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 825F33F0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825F33F4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825F33F8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825F33FC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F3400: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825F3404: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F3408: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F340C: 4BE73A15  bl 0x82466e20
	ctx.lr = 0x825F3410;
	sub_82466E20(ctx, base);
	// 825F3410: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F3414: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F3418: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F341C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F3420(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F3420 size=112
    let mut pc: u32 = 0x825F3420;
    'dispatch: loop {
        match pc {
            0x825F3420 => {
    //   block [0x825F3420..0x825F3490)
	// 825F3420: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F3424: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F3428: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F342C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F3430: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825F3434: 38AACBC4  addi r5, r10, -0x343c
	ctx.r[5].s64 = ctx.r[10].s64 + -13372;
	// 825F3438: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F343C: 390BAD48  addi r8, r11, -0x52b8
	ctx.r[8].s64 = ctx.r[11].s64 + -21176;
	// 825F3440: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 825F3444: 388A4288  addi r4, r10, 0x4288
	ctx.r[4].s64 = ctx.r[10].s64 + 17032;
	// 825F3448: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825F344C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F3450: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825F3454: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F3458: 386ACE34  addi r3, r10, -0x31cc
	ctx.r[3].s64 = ctx.r[10].s64 + -12748;
	// 825F345C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 825F3460: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825F3464: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825F3468: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825F346C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F3470: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825F3474: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F3478: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F347C: 4BE739A5  bl 0x82466e20
	ctx.lr = 0x825F3480;
	sub_82466E20(ctx, base);
	// 825F3480: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F3484: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F3488: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F348C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F3490(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F3490 size=116
    let mut pc: u32 = 0x825F3490;
    'dispatch: loop {
        match pc {
            0x825F3490 => {
    //   block [0x825F3490..0x825F3504)
	// 825F3490: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F3494: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F3498: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F349C: 3D408274  lis r10, -0x7d8c
	ctx.r[10].s64 = -2106327040;
	// 825F34A0: 38E00005  li r7, 5
	ctx.r[7].s64 = 5;
	// 825F34A4: 390AAD78  addi r8, r10, -0x5288
	ctx.r[8].s64 = ctx.r[10].s64 + -21128;
	// 825F34A8: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F34AC: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 825F34B0: 38AACBC4  addi r5, r10, -0x343c
	ctx.r[5].s64 = ctx.r[10].s64 + -13372;
	// 825F34B4: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F34B8: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 825F34BC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825F34C0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825F34C4: 388A429C  addi r4, r10, 0x429c
	ctx.r[4].s64 = ctx.r[10].s64 + 17052;
	// 825F34C8: 396B1C50  addi r11, r11, 0x1c50
	ctx.r[11].s64 = ctx.r[11].s64 + 7248;
	// 825F34CC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F34D0: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F34D4: 386ACE64  addi r3, r10, -0x319c
	ctx.r[3].s64 = ctx.r[10].s64 + -12700;
	// 825F34D8: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 825F34DC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825F34E0: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 825F34E4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F34E8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F34EC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F34F0: 4BE73931  bl 0x82466e20
	ctx.lr = 0x825F34F4;
	sub_82466E20(ctx, base);
	// 825F34F4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F34F8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F34FC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F3500: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F3508(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F3508 size=116
    let mut pc: u32 = 0x825F3508;
    'dispatch: loop {
        match pc {
            0x825F3508 => {
    //   block [0x825F3508..0x825F357C)
	// 825F3508: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F350C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F3510: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F3514: 3D408274  lis r10, -0x7d8c
	ctx.r[10].s64 = -2106327040;
	// 825F3518: 38E00006  li r7, 6
	ctx.r[7].s64 = 6;
	// 825F351C: 390AADF0  addi r8, r10, -0x5210
	ctx.r[8].s64 = ctx.r[10].s64 + -21008;
	// 825F3520: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F3524: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 825F3528: 38AACBC4  addi r5, r10, -0x343c
	ctx.r[5].s64 = ctx.r[10].s64 + -13372;
	// 825F352C: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F3530: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 825F3534: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825F3538: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825F353C: 388A42B8  addi r4, r10, 0x42b8
	ctx.r[4].s64 = ctx.r[10].s64 + 17080;
	// 825F3540: 396B1C68  addi r11, r11, 0x1c68
	ctx.r[11].s64 = ctx.r[11].s64 + 7272;
	// 825F3544: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F3548: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F354C: 386ACE94  addi r3, r10, -0x316c
	ctx.r[3].s64 = ctx.r[10].s64 + -12652;
	// 825F3550: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 825F3554: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825F3558: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 825F355C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F3560: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F3564: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F3568: 4BE738B9  bl 0x82466e20
	ctx.lr = 0x825F356C;
	sub_82466E20(ctx, base);
	// 825F356C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F3570: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F3574: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F3578: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F3580(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825F3580 size=24
    let mut pc: u32 = 0x825F3580;
    'dispatch: loop {
        match pc {
            0x825F3580 => {
    //   block [0x825F3580..0x825F3598)
	// 825F3580: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825F3584: 3D408274  lis r10, -0x7d8c
	ctx.r[10].s64 = -2106327040;
	// 825F3588: 394AEDC8  addi r10, r10, -0x1238
	ctx.r[10].s64 = ctx.r[10].s64 + -4664;
	// 825F358C: 816BAC9C  lwz r11, -0x5364(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-21348 as u32) ) } as u64;
	// 825F3590: 916A0050  stw r11, 0x50(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 825F3594: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F3598(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F3598 size=116
    let mut pc: u32 = 0x825F3598;
    'dispatch: loop {
        match pc {
            0x825F3598 => {
    //   block [0x825F3598..0x825F360C)
	// 825F3598: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F359C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F35A0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F35A4: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 825F35A8: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F35AC: 392B1C94  addi r9, r11, 0x1c94
	ctx.r[9].s64 = ctx.r[11].s64 + 7316;
	// 825F35B0: 38AACBC4  addi r5, r10, -0x343c
	ctx.r[5].s64 = ctx.r[10].s64 + -13372;
	// 825F35B4: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F35B8: 38E90014  addi r7, r9, 0x14
	ctx.r[7].s64 = ctx.r[9].s64 + 20;
	// 825F35BC: 38C00007  li r6, 7
	ctx.r[6].s64 = 7;
	// 825F35C0: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825F35C4: 388A42D4  addi r4, r10, 0x42d4
	ctx.r[4].s64 = ctx.r[10].s64 + 17108;
	// 825F35C8: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F35CC: 396BEDC8  addi r11, r11, -0x1238
	ctx.r[11].s64 = ctx.r[11].s64 + -4664;
	// 825F35D0: 90E10064  stw r7, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[7].u32 ) };
	// 825F35D4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F35D8: 90C1005C  stw r6, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[6].u32 ) };
	// 825F35DC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F35E0: 386ACEC4  addi r3, r10, -0x313c
	ctx.r[3].s64 = ctx.r[10].s64 + -12604;
	// 825F35E4: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 825F35E8: 91010074  stw r8, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[8].u32 ) };
	// 825F35EC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F35F0: 9101006C  stw r8, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[8].u32 ) };
	// 825F35F4: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 825F35F8: 4BE73829  bl 0x82466e20
	ctx.lr = 0x825F35FC;
	sub_82466E20(ctx, base);
	// 825F35FC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F3600: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F3604: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F3608: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F3610(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F3610 size=112
    let mut pc: u32 = 0x825F3610;
    'dispatch: loop {
        match pc {
            0x825F3610 => {
    //   block [0x825F3610..0x825F3680)
	// 825F3610: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F3614: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F3618: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F361C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F3620: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825F3624: 38AACBC4  addi r5, r10, -0x343c
	ctx.r[5].s64 = ctx.r[10].s64 + -13372;
	// 825F3628: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F362C: 390BAE80  addi r8, r11, -0x5180
	ctx.r[8].s64 = ctx.r[11].s64 + -20864;
	// 825F3630: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 825F3634: 388A42F0  addi r4, r10, 0x42f0
	ctx.r[4].s64 = ctx.r[10].s64 + 17136;
	// 825F3638: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825F363C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F3640: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825F3644: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F3648: 386ACEF4  addi r3, r10, -0x310c
	ctx.r[3].s64 = ctx.r[10].s64 + -12556;
	// 825F364C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 825F3650: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825F3654: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825F3658: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825F365C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F3660: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825F3664: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F3668: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F366C: 4BE737B5  bl 0x82466e20
	ctx.lr = 0x825F3670;
	sub_82466E20(ctx, base);
	// 825F3670: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F3674: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F3678: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F367C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F3680(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F3680 size=112
    let mut pc: u32 = 0x825F3680;
    'dispatch: loop {
        match pc {
            0x825F3680 => {
    //   block [0x825F3680..0x825F36F0)
	// 825F3680: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F3684: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F3688: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F368C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F3690: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825F3694: 38AACBC4  addi r5, r10, -0x343c
	ctx.r[5].s64 = ctx.r[10].s64 + -13372;
	// 825F3698: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F369C: 390BAEE0  addi r8, r11, -0x5120
	ctx.r[8].s64 = ctx.r[11].s64 + -20768;
	// 825F36A0: 39200007  li r9, 7
	ctx.r[9].s64 = 7;
	// 825F36A4: 388A430C  addi r4, r10, 0x430c
	ctx.r[4].s64 = ctx.r[10].s64 + 17164;
	// 825F36A8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825F36AC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F36B0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825F36B4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F36B8: 386ACF24  addi r3, r10, -0x30dc
	ctx.r[3].s64 = ctx.r[10].s64 + -12508;
	// 825F36BC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 825F36C0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825F36C4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825F36C8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825F36CC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F36D0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825F36D4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F36D8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F36DC: 4BE73745  bl 0x82466e20
	ctx.lr = 0x825F36E0;
	sub_82466E20(ctx, base);
	// 825F36E0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F36E4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F36E8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F36EC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F36F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F36F0 size=112
    let mut pc: u32 = 0x825F36F0;
    'dispatch: loop {
        match pc {
            0x825F36F0 => {
    //   block [0x825F36F0..0x825F3760)
	// 825F36F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F36F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F36F8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F36FC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F3700: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825F3704: 38AACBC4  addi r5, r10, -0x343c
	ctx.r[5].s64 = ctx.r[10].s64 + -13372;
	// 825F3708: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F370C: 390BAF88  addi r8, r11, -0x5078
	ctx.r[8].s64 = ctx.r[11].s64 + -20600;
	// 825F3710: 39200005  li r9, 5
	ctx.r[9].s64 = 5;
	// 825F3714: 388A4328  addi r4, r10, 0x4328
	ctx.r[4].s64 = ctx.r[10].s64 + 17192;
	// 825F3718: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825F371C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F3720: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825F3724: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F3728: 386ACF54  addi r3, r10, -0x30ac
	ctx.r[3].s64 = ctx.r[10].s64 + -12460;
	// 825F372C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 825F3730: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825F3734: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825F3738: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825F373C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F3740: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825F3744: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F3748: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F374C: 4BE736D5  bl 0x82466e20
	ctx.lr = 0x825F3750;
	sub_82466E20(ctx, base);
	// 825F3750: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F3754: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F3758: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F375C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F3760(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F3760 size=112
    let mut pc: u32 = 0x825F3760;
    'dispatch: loop {
        match pc {
            0x825F3760 => {
    //   block [0x825F3760..0x825F37D0)
	// 825F3760: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F3764: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F3768: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F376C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F3770: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825F3774: 38AACBC4  addi r5, r10, -0x343c
	ctx.r[5].s64 = ctx.r[10].s64 + -13372;
	// 825F3778: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F377C: 390BB000  addi r8, r11, -0x5000
	ctx.r[8].s64 = ctx.r[11].s64 + -20480;
	// 825F3780: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 825F3784: 388A4348  addi r4, r10, 0x4348
	ctx.r[4].s64 = ctx.r[10].s64 + 17224;
	// 825F3788: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825F378C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F3790: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825F3794: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F3798: 386ACF84  addi r3, r10, -0x307c
	ctx.r[3].s64 = ctx.r[10].s64 + -12412;
	// 825F379C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 825F37A0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825F37A4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825F37A8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825F37AC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F37B0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825F37B4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F37B8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F37BC: 4BE73665  bl 0x82466e20
	ctx.lr = 0x825F37C0;
	sub_82466E20(ctx, base);
	// 825F37C0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F37C4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F37C8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F37CC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F37D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F37D0 size=112
    let mut pc: u32 = 0x825F37D0;
    'dispatch: loop {
        match pc {
            0x825F37D0 => {
    //   block [0x825F37D0..0x825F3840)
	// 825F37D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F37D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F37D8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F37DC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F37E0: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825F37E4: 38AACBC4  addi r5, r10, -0x343c
	ctx.r[5].s64 = ctx.r[10].s64 + -13372;
	// 825F37E8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F37EC: 390BB048  addi r8, r11, -0x4fb8
	ctx.r[8].s64 = ctx.r[11].s64 + -20408;
	// 825F37F0: 39200006  li r9, 6
	ctx.r[9].s64 = 6;
	// 825F37F4: 388A4364  addi r4, r10, 0x4364
	ctx.r[4].s64 = ctx.r[10].s64 + 17252;
	// 825F37F8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825F37FC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F3800: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825F3804: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F3808: 386ACFB4  addi r3, r10, -0x304c
	ctx.r[3].s64 = ctx.r[10].s64 + -12364;
	// 825F380C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 825F3810: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825F3814: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825F3818: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825F381C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F3820: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825F3824: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F3828: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F382C: 4BE735F5  bl 0x82466e20
	ctx.lr = 0x825F3830;
	sub_82466E20(ctx, base);
	// 825F3830: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F3834: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F3838: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F383C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F3840(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F3840 size=112
    let mut pc: u32 = 0x825F3840;
    'dispatch: loop {
        match pc {
            0x825F3840 => {
    //   block [0x825F3840..0x825F38B0)
	// 825F3840: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F3844: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F3848: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F384C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F3850: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825F3854: 38AACBC4  addi r5, r10, -0x343c
	ctx.r[5].s64 = ctx.r[10].s64 + -13372;
	// 825F3858: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F385C: 390BB0D8  addi r8, r11, -0x4f28
	ctx.r[8].s64 = ctx.r[11].s64 + -20264;
	// 825F3860: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 825F3864: 388A4380  addi r4, r10, 0x4380
	ctx.r[4].s64 = ctx.r[10].s64 + 17280;
	// 825F3868: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825F386C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F3870: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825F3874: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F3878: 386ACFE4  addi r3, r10, -0x301c
	ctx.r[3].s64 = ctx.r[10].s64 + -12316;
	// 825F387C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 825F3880: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825F3884: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825F3888: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825F388C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F3890: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825F3894: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F3898: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F389C: 4BE73585  bl 0x82466e20
	ctx.lr = 0x825F38A0;
	sub_82466E20(ctx, base);
	// 825F38A0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F38A4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F38A8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F38AC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F38B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F38B0 size=112
    let mut pc: u32 = 0x825F38B0;
    'dispatch: loop {
        match pc {
            0x825F38B0 => {
    //   block [0x825F38B0..0x825F3920)
	// 825F38B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F38B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F38B8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F38BC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F38C0: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825F38C4: 38AACBC4  addi r5, r10, -0x343c
	ctx.r[5].s64 = ctx.r[10].s64 + -13372;
	// 825F38C8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F38CC: 390BB138  addi r8, r11, -0x4ec8
	ctx.r[8].s64 = ctx.r[11].s64 + -20168;
	// 825F38D0: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 825F38D4: 388A4398  addi r4, r10, 0x4398
	ctx.r[4].s64 = ctx.r[10].s64 + 17304;
	// 825F38D8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825F38DC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F38E0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825F38E4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F38E8: 386AD014  addi r3, r10, -0x2fec
	ctx.r[3].s64 = ctx.r[10].s64 + -12268;
	// 825F38EC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 825F38F0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825F38F4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825F38F8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825F38FC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F3900: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825F3904: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F3908: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F390C: 4BE73515  bl 0x82466e20
	ctx.lr = 0x825F3910;
	sub_82466E20(ctx, base);
	// 825F3910: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F3914: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F3918: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F391C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F3920(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F3920 size=112
    let mut pc: u32 = 0x825F3920;
    'dispatch: loop {
        match pc {
            0x825F3920 => {
    //   block [0x825F3920..0x825F3990)
	// 825F3920: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F3924: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F3928: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F392C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F3930: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825F3934: 38AAD014  addi r5, r10, -0x2fec
	ctx.r[5].s64 = ctx.r[10].s64 + -12268;
	// 825F3938: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F393C: 390BB198  addi r8, r11, -0x4e68
	ctx.r[8].s64 = ctx.r[11].s64 + -20072;
	// 825F3940: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 825F3944: 388A43B4  addi r4, r10, 0x43b4
	ctx.r[4].s64 = ctx.r[10].s64 + 17332;
	// 825F3948: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825F394C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F3950: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825F3954: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F3958: 386AD044  addi r3, r10, -0x2fbc
	ctx.r[3].s64 = ctx.r[10].s64 + -12220;
	// 825F395C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 825F3960: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825F3964: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825F3968: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825F396C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F3970: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825F3974: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F3978: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F397C: 4BE734A5  bl 0x82466e20
	ctx.lr = 0x825F3980;
	sub_82466E20(ctx, base);
	// 825F3980: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F3984: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F3988: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F398C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F3990(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F3990 size=112
    let mut pc: u32 = 0x825F3990;
    'dispatch: loop {
        match pc {
            0x825F3990 => {
    //   block [0x825F3990..0x825F3A00)
	// 825F3990: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F3994: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F3998: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F399C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F39A0: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825F39A4: 38AAD014  addi r5, r10, -0x2fec
	ctx.r[5].s64 = ctx.r[10].s64 + -12268;
	// 825F39A8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F39AC: 390BB1C8  addi r8, r11, -0x4e38
	ctx.r[8].s64 = ctx.r[11].s64 + -20024;
	// 825F39B0: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 825F39B4: 388A43D8  addi r4, r10, 0x43d8
	ctx.r[4].s64 = ctx.r[10].s64 + 17368;
	// 825F39B8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825F39BC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F39C0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825F39C4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F39C8: 386AD074  addi r3, r10, -0x2f8c
	ctx.r[3].s64 = ctx.r[10].s64 + -12172;
	// 825F39CC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 825F39D0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825F39D4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825F39D8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825F39DC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F39E0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825F39E4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F39E8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F39EC: 4BE73435  bl 0x82466e20
	ctx.lr = 0x825F39F0;
	sub_82466E20(ctx, base);
	// 825F39F0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F39F4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F39F8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F39FC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F3A00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F3A00 size=100
    let mut pc: u32 = 0x825F3A00;
    'dispatch: loop {
        match pc {
            0x825F3A00 => {
    //   block [0x825F3A00..0x825F3A64)
	// 825F3A00: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F3A04: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F3A08: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F3A0C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F3A10: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825F3A14: 38AAD014  addi r5, r10, -0x2fec
	ctx.r[5].s64 = ctx.r[10].s64 + -12268;
	// 825F3A18: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F3A1C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825F3A20: 388A43FC  addi r4, r10, 0x43fc
	ctx.r[4].s64 = ctx.r[10].s64 + 17404;
	// 825F3A24: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F3A28: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825F3A2C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F3A30: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825F3A34: 386AD0A4  addi r3, r10, -0x2f5c
	ctx.r[3].s64 = ctx.r[10].s64 + -12124;
	// 825F3A38: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F3A3C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825F3A40: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 825F3A44: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F3A48: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 825F3A4C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F3A50: 4BE733D1  bl 0x82466e20
	ctx.lr = 0x825F3A54;
	sub_82466E20(ctx, base);
	// 825F3A54: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F3A58: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F3A5C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F3A60: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F3A68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F3A68 size=112
    let mut pc: u32 = 0x825F3A68;
    'dispatch: loop {
        match pc {
            0x825F3A68 => {
    //   block [0x825F3A68..0x825F3AD8)
	// 825F3A68: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F3A6C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F3A70: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F3A74: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F3A78: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825F3A7C: 38AAD014  addi r5, r10, -0x2fec
	ctx.r[5].s64 = ctx.r[10].s64 + -12268;
	// 825F3A80: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F3A84: 390BB1F8  addi r8, r11, -0x4e08
	ctx.r[8].s64 = ctx.r[11].s64 + -19976;
	// 825F3A88: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 825F3A8C: 388A4424  addi r4, r10, 0x4424
	ctx.r[4].s64 = ctx.r[10].s64 + 17444;
	// 825F3A90: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825F3A94: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F3A98: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825F3A9C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F3AA0: 386AD0D4  addi r3, r10, -0x2f2c
	ctx.r[3].s64 = ctx.r[10].s64 + -12076;
	// 825F3AA4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 825F3AA8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825F3AAC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825F3AB0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825F3AB4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F3AB8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825F3ABC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F3AC0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F3AC4: 4BE7335D  bl 0x82466e20
	ctx.lr = 0x825F3AC8;
	sub_82466E20(ctx, base);
	// 825F3AC8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F3ACC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F3AD0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F3AD4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F3AD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F3AD8 size=112
    let mut pc: u32 = 0x825F3AD8;
    'dispatch: loop {
        match pc {
            0x825F3AD8 => {
    //   block [0x825F3AD8..0x825F3B48)
	// 825F3AD8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F3ADC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F3AE0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F3AE4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F3AE8: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825F3AEC: 38AAAF74  addi r5, r10, -0x508c
	ctx.r[5].s64 = ctx.r[10].s64 + -20620;
	// 825F3AF0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F3AF4: 390BB210  addi r8, r11, -0x4df0
	ctx.r[8].s64 = ctx.r[11].s64 + -19952;
	// 825F3AF8: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 825F3AFC: 388A444C  addi r4, r10, 0x444c
	ctx.r[4].s64 = ctx.r[10].s64 + 17484;
	// 825F3B00: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825F3B04: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F3B08: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825F3B0C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F3B10: 386AD104  addi r3, r10, -0x2efc
	ctx.r[3].s64 = ctx.r[10].s64 + -12028;
	// 825F3B14: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 825F3B18: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825F3B1C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825F3B20: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825F3B24: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F3B28: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825F3B2C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F3B30: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F3B34: 4BE732ED  bl 0x82466e20
	ctx.lr = 0x825F3B38;
	sub_82466E20(ctx, base);
	// 825F3B38: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F3B3C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F3B40: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F3B44: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F3B48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F3B48 size=112
    let mut pc: u32 = 0x825F3B48;
    'dispatch: loop {
        match pc {
            0x825F3B48 => {
    //   block [0x825F3B48..0x825F3BB8)
	// 825F3B48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F3B4C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F3B50: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F3B54: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F3B58: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825F3B5C: 38AAD104  addi r5, r10, -0x2efc
	ctx.r[5].s64 = ctx.r[10].s64 + -12028;
	// 825F3B60: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F3B64: 390BB270  addi r8, r11, -0x4d90
	ctx.r[8].s64 = ctx.r[11].s64 + -19856;
	// 825F3B68: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 825F3B6C: 388A4458  addi r4, r10, 0x4458
	ctx.r[4].s64 = ctx.r[10].s64 + 17496;
	// 825F3B70: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825F3B74: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F3B78: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825F3B7C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F3B80: 386AD134  addi r3, r10, -0x2ecc
	ctx.r[3].s64 = ctx.r[10].s64 + -11980;
	// 825F3B84: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 825F3B88: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825F3B8C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825F3B90: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825F3B94: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F3B98: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825F3B9C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F3BA0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F3BA4: 4BE7327D  bl 0x82466e20
	ctx.lr = 0x825F3BA8;
	sub_82466E20(ctx, base);
	// 825F3BA8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F3BAC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F3BB0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F3BB4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F3BB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F3BB8 size=112
    let mut pc: u32 = 0x825F3BB8;
    'dispatch: loop {
        match pc {
            0x825F3BB8 => {
    //   block [0x825F3BB8..0x825F3C28)
	// 825F3BB8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F3BBC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F3BC0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F3BC4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F3BC8: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825F3BCC: 38AAD104  addi r5, r10, -0x2efc
	ctx.r[5].s64 = ctx.r[10].s64 + -12028;
	// 825F3BD0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F3BD4: 390BB288  addi r8, r11, -0x4d78
	ctx.r[8].s64 = ctx.r[11].s64 + -19832;
	// 825F3BD8: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 825F3BDC: 388A4468  addi r4, r10, 0x4468
	ctx.r[4].s64 = ctx.r[10].s64 + 17512;
	// 825F3BE0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825F3BE4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F3BE8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825F3BEC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F3BF0: 386AD164  addi r3, r10, -0x2e9c
	ctx.r[3].s64 = ctx.r[10].s64 + -11932;
	// 825F3BF4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 825F3BF8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825F3BFC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825F3C00: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825F3C04: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F3C08: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825F3C0C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F3C10: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F3C14: 4BE7320D  bl 0x82466e20
	ctx.lr = 0x825F3C18;
	sub_82466E20(ctx, base);
	// 825F3C18: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F3C1C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F3C20: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F3C24: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F3C28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F3C28 size=112
    let mut pc: u32 = 0x825F3C28;
    'dispatch: loop {
        match pc {
            0x825F3C28 => {
    //   block [0x825F3C28..0x825F3C98)
	// 825F3C28: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F3C2C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F3C30: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F3C34: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F3C38: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825F3C3C: 38AAD104  addi r5, r10, -0x2efc
	ctx.r[5].s64 = ctx.r[10].s64 + -12028;
	// 825F3C40: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F3C44: 390BB2B8  addi r8, r11, -0x4d48
	ctx.r[8].s64 = ctx.r[11].s64 + -19784;
	// 825F3C48: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 825F3C4C: 388A4478  addi r4, r10, 0x4478
	ctx.r[4].s64 = ctx.r[10].s64 + 17528;
	// 825F3C50: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825F3C54: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F3C58: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825F3C5C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F3C60: 386AD194  addi r3, r10, -0x2e6c
	ctx.r[3].s64 = ctx.r[10].s64 + -11884;
	// 825F3C64: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 825F3C68: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825F3C6C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825F3C70: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825F3C74: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F3C78: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825F3C7C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F3C80: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F3C84: 4BE7319D  bl 0x82466e20
	ctx.lr = 0x825F3C88;
	sub_82466E20(ctx, base);
	// 825F3C88: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F3C8C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F3C90: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F3C94: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F3C98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825F3C98 size=24
    let mut pc: u32 = 0x825F3C98;
    'dispatch: loop {
        match pc {
            0x825F3C98 => {
    //   block [0x825F3C98..0x825F3CB0)
	// 825F3C98: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825F3C9C: 3D408274  lis r10, -0x7d8c
	ctx.r[10].s64 = -2106327040;
	// 825F3CA0: 394AEE70  addi r10, r10, -0x1190
	ctx.r[10].s64 = ctx.r[10].s64 + -4496;
	// 825F3CA4: 816BB2D0  lwz r11, -0x4d30(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-19760 as u32) ) } as u64;
	// 825F3CA8: 916A0008  stw r11, 8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 825F3CAC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F3CB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F3CB0 size=112
    let mut pc: u32 = 0x825F3CB0;
    'dispatch: loop {
        match pc {
            0x825F3CB0 => {
    //   block [0x825F3CB0..0x825F3D20)
	// 825F3CB0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F3CB4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F3CB8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F3CBC: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F3CC0: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825F3CC4: 392A1CF0  addi r9, r10, 0x1cf0
	ctx.r[9].s64 = ctx.r[10].s64 + 7408;
	// 825F3CC8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F3CCC: 390BEE70  addi r8, r11, -0x1190
	ctx.r[8].s64 = ctx.r[11].s64 + -4496;
	// 825F3CD0: 38E00003  li r7, 3
	ctx.r[7].s64 = 3;
	// 825F3CD4: 388A4488  addi r4, r10, 0x4488
	ctx.r[4].s64 = ctx.r[10].s64 + 17544;
	// 825F3CD8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825F3CDC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F3CE0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825F3CE4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F3CE8: 386AD1C4  addi r3, r10, -0x2e3c
	ctx.r[3].s64 = ctx.r[10].s64 + -11836;
	// 825F3CEC: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 825F3CF0: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 825F3CF4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F3CF8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825F3CFC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F3D00: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825F3D04: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825F3D08: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F3D0C: 4BE73115  bl 0x82466e20
	ctx.lr = 0x825F3D10;
	sub_82466E20(ctx, base);
	// 825F3D10: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F3D14: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F3D18: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F3D1C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F3D20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F3D20 size=108
    let mut pc: u32 = 0x825F3D20;
    'dispatch: loop {
        match pc {
            0x825F3D20 => {
    //   block [0x825F3D20..0x825F3D8C)
	// 825F3D20: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F3D24: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F3D28: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F3D2C: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825F3D30: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F3D34: 38EBB2D4  addi r7, r11, -0x4d2c
	ctx.r[7].s64 = ctx.r[11].s64 + -19756;
	// 825F3D38: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 825F3D3C: 388A4494  addi r4, r10, 0x4494
	ctx.r[4].s64 = ctx.r[10].s64 + 17556;
	// 825F3D40: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825F3D44: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F3D48: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 825F3D4C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825F3D50: 386AD1F4  addi r3, r10, -0x2e0c
	ctx.r[3].s64 = ctx.r[10].s64 + -11788;
	// 825F3D54: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 825F3D58: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825F3D5C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F3D60: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825F3D64: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F3D68: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825F3D6C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F3D70: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F3D74: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825F3D78: 4BE730A9  bl 0x82466e20
	ctx.lr = 0x825F3D7C;
	sub_82466E20(ctx, base);
	// 825F3D7C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F3D80: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F3D84: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F3D88: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F3D90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F3D90 size=108
    let mut pc: u32 = 0x825F3D90;
    'dispatch: loop {
        match pc {
            0x825F3D90 => {
    //   block [0x825F3D90..0x825F3DFC)
	// 825F3D90: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F3D94: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F3D98: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F3D9C: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825F3DA0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F3DA4: 38EBB2F0  addi r7, r11, -0x4d10
	ctx.r[7].s64 = ctx.r[11].s64 + -19728;
	// 825F3DA8: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 825F3DAC: 388A44A4  addi r4, r10, 0x44a4
	ctx.r[4].s64 = ctx.r[10].s64 + 17572;
	// 825F3DB0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825F3DB4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F3DB8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 825F3DBC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825F3DC0: 386AD224  addi r3, r10, -0x2ddc
	ctx.r[3].s64 = ctx.r[10].s64 + -11740;
	// 825F3DC4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 825F3DC8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825F3DCC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F3DD0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825F3DD4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F3DD8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825F3DDC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F3DE0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F3DE4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825F3DE8: 4BE73039  bl 0x82466e20
	ctx.lr = 0x825F3DEC;
	sub_82466E20(ctx, base);
	// 825F3DEC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F3DF0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F3DF4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F3DF8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F3E00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F3E00 size=116
    let mut pc: u32 = 0x825F3E00;
    'dispatch: loop {
        match pc {
            0x825F3E00 => {
    //   block [0x825F3E00..0x825F3E74)
	// 825F3E00: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F3E04: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F3E08: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F3E0C: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825F3E10: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F3E14: 390BB338  addi r8, r11, -0x4cc8
	ctx.r[8].s64 = ctx.r[11].s64 + -19656;
	// 825F3E18: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825F3E1C: 392A1DA8  addi r9, r10, 0x1da8
	ctx.r[9].s64 = ctx.r[10].s64 + 7592;
	// 825F3E20: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F3E24: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
	// 825F3E28: 38AAAF74  addi r5, r10, -0x508c
	ctx.r[5].s64 = ctx.r[10].s64 + -20620;
	// 825F3E2C: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825F3E30: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825F3E34: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F3E38: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825F3E3C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F3E40: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F3E44: 3D608293  lis r11, -0x7d6d
	ctx.r[11].s64 = -2104295424;
	// 825F3E48: 388A44B0  addi r4, r10, 0x44b0
	ctx.r[4].s64 = ctx.r[10].s64 + 17584;
	// 825F3E4C: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 825F3E50: 386BD254  addi r3, r11, -0x2dac
	ctx.r[3].s64 = ctx.r[11].s64 + -11692;
	// 825F3E54: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 825F3E58: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F3E5C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F3E60: 4BE72FC1  bl 0x82466e20
	ctx.lr = 0x825F3E64;
	sub_82466E20(ctx, base);
	// 825F3E64: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F3E68: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F3E6C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F3E70: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F3E78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825F3E78 size=24
    let mut pc: u32 = 0x825F3E78;
    'dispatch: loop {
        match pc {
            0x825F3E78 => {
    //   block [0x825F3E78..0x825F3E90)
	// 825F3E78: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825F3E7C: 3D408274  lis r10, -0x7d8c
	ctx.r[10].s64 = -2106327040;
	// 825F3E80: 394AEEB8  addi r10, r10, -0x1148
	ctx.r[10].s64 = ctx.r[10].s64 + -4424;
	// 825F3E84: 816BB350  lwz r11, -0x4cb0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-19632 as u32) ) } as u64;
	// 825F3E88: 916A0068  stw r11, 0x68(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(104 as u32), ctx.r[11].u32 ) };
	// 825F3E8C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F3E90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F3E90 size=116
    let mut pc: u32 = 0x825F3E90;
    'dispatch: loop {
        match pc {
            0x825F3E90 => {
    //   block [0x825F3E90..0x825F3F04)
	// 825F3E90: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F3E94: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F3E98: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F3E9C: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825F3EA0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F3EA4: 390BEEB8  addi r8, r11, -0x1148
	ctx.r[8].s64 = ctx.r[11].s64 + -4424;
	// 825F3EA8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825F3EAC: 392A1E04  addi r9, r10, 0x1e04
	ctx.r[9].s64 = ctx.r[10].s64 + 7684;
	// 825F3EB0: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F3EB4: 38E00009  li r7, 9
	ctx.r[7].s64 = 9;
	// 825F3EB8: 38AAAF74  addi r5, r10, -0x508c
	ctx.r[5].s64 = ctx.r[10].s64 + -20620;
	// 825F3EBC: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825F3EC0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825F3EC4: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F3EC8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825F3ECC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F3ED0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F3ED4: 3D608293  lis r11, -0x7d6d
	ctx.r[11].s64 = -2104295424;
	// 825F3ED8: 388A44DC  addi r4, r10, 0x44dc
	ctx.r[4].s64 = ctx.r[10].s64 + 17628;
	// 825F3EDC: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 825F3EE0: 386BD284  addi r3, r11, -0x2d7c
	ctx.r[3].s64 = ctx.r[11].s64 + -11644;
	// 825F3EE4: 39400003  li r10, 3
	ctx.r[10].s64 = 3;
	// 825F3EE8: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F3EEC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F3EF0: 4BE72F31  bl 0x82466e20
	ctx.lr = 0x825F3EF4;
	sub_82466E20(ctx, base);
	// 825F3EF4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F3EF8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F3EFC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F3F00: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F3F08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F3F08 size=108
    let mut pc: u32 = 0x825F3F08;
    'dispatch: loop {
        match pc {
            0x825F3F08 => {
    //   block [0x825F3F08..0x825F3F74)
	// 825F3F08: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F3F0C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F3F10: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F3F14: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825F3F18: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F3F1C: 38EBB35C  addi r7, r11, -0x4ca4
	ctx.r[7].s64 = ctx.r[11].s64 + -19620;
	// 825F3F20: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 825F3F24: 388A44F4  addi r4, r10, 0x44f4
	ctx.r[4].s64 = ctx.r[10].s64 + 17652;
	// 825F3F28: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825F3F2C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F3F30: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 825F3F34: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825F3F38: 386AD2B4  addi r3, r10, -0x2d4c
	ctx.r[3].s64 = ctx.r[10].s64 + -11596;
	// 825F3F3C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 825F3F40: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825F3F44: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F3F48: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825F3F4C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F3F50: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825F3F54: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F3F58: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F3F5C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825F3F60: 4BE72EC1  bl 0x82466e20
	ctx.lr = 0x825F3F64;
	sub_82466E20(ctx, base);
	// 825F3F64: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F3F68: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F3F6C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F3F70: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F3F78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F3F78 size=112
    let mut pc: u32 = 0x825F3F78;
    'dispatch: loop {
        match pc {
            0x825F3F78 => {
    //   block [0x825F3F78..0x825F3FE8)
	// 825F3F78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F3F7C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F3F80: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F3F84: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F3F88: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825F3F8C: 38AAD254  addi r5, r10, -0x2dac
	ctx.r[5].s64 = ctx.r[10].s64 + -11692;
	// 825F3F90: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F3F94: 390BB38C  addi r8, r11, -0x4c74
	ctx.r[8].s64 = ctx.r[11].s64 + -19572;
	// 825F3F98: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 825F3F9C: 388A4518  addi r4, r10, 0x4518
	ctx.r[4].s64 = ctx.r[10].s64 + 17688;
	// 825F3FA0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825F3FA4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F3FA8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825F3FAC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F3FB0: 386AD2E4  addi r3, r10, -0x2d1c
	ctx.r[3].s64 = ctx.r[10].s64 + -11548;
	// 825F3FB4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 825F3FB8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825F3FBC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825F3FC0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825F3FC4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F3FC8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825F3FCC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F3FD0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F3FD4: 4BE72E4D  bl 0x82466e20
	ctx.lr = 0x825F3FD8;
	sub_82466E20(ctx, base);
	// 825F3FD8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F3FDC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F3FE0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F3FE4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F3FE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F3FE8 size=112
    let mut pc: u32 = 0x825F3FE8;
    'dispatch: loop {
        match pc {
            0x825F3FE8 => {
    //   block [0x825F3FE8..0x825F4058)
	// 825F3FE8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F3FEC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F3FF0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F3FF4: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F3FF8: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825F3FFC: 392A1E48  addi r9, r10, 0x1e48
	ctx.r[9].s64 = ctx.r[10].s64 + 7752;
	// 825F4000: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F4004: 390BB3A8  addi r8, r11, -0x4c58
	ctx.r[8].s64 = ctx.r[11].s64 + -19544;
	// 825F4008: 38E00003  li r7, 3
	ctx.r[7].s64 = 3;
	// 825F400C: 388A4538  addi r4, r10, 0x4538
	ctx.r[4].s64 = ctx.r[10].s64 + 17720;
	// 825F4010: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825F4014: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F4018: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825F401C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F4020: 386AD314  addi r3, r10, -0x2cec
	ctx.r[3].s64 = ctx.r[10].s64 + -11500;
	// 825F4024: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 825F4028: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 825F402C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F4030: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825F4034: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F4038: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825F403C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825F4040: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F4044: 4BE72DDD  bl 0x82466e20
	ctx.lr = 0x825F4048;
	sub_82466E20(ctx, base);
	// 825F4048: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F404C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F4050: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F4054: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F4058(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F4058 size=112
    let mut pc: u32 = 0x825F4058;
    'dispatch: loop {
        match pc {
            0x825F4058 => {
    //   block [0x825F4058..0x825F40C8)
	// 825F4058: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F405C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F4060: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F4064: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F4068: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825F406C: 38AAD254  addi r5, r10, -0x2dac
	ctx.r[5].s64 = ctx.r[10].s64 + -11692;
	// 825F4070: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F4074: 390BB3F0  addi r8, r11, -0x4c10
	ctx.r[8].s64 = ctx.r[11].s64 + -19472;
	// 825F4078: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 825F407C: 388A4554  addi r4, r10, 0x4554
	ctx.r[4].s64 = ctx.r[10].s64 + 17748;
	// 825F4080: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825F4084: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F4088: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825F408C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F4090: 386AD344  addi r3, r10, -0x2cbc
	ctx.r[3].s64 = ctx.r[10].s64 + -11452;
	// 825F4094: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 825F4098: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825F409C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825F40A0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825F40A4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F40A8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825F40AC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F40B0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F40B4: 4BE72D6D  bl 0x82466e20
	ctx.lr = 0x825F40B8;
	sub_82466E20(ctx, base);
	// 825F40B8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F40BC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F40C0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F40C4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F40C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F40C8 size=112
    let mut pc: u32 = 0x825F40C8;
    'dispatch: loop {
        match pc {
            0x825F40C8 => {
    //   block [0x825F40C8..0x825F4138)
	// 825F40C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F40CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F40D0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F40D4: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F40D8: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825F40DC: 392A1E74  addi r9, r10, 0x1e74
	ctx.r[9].s64 = ctx.r[10].s64 + 7796;
	// 825F40E0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F40E4: 390BB410  addi r8, r11, -0x4bf0
	ctx.r[8].s64 = ctx.r[11].s64 + -19440;
	// 825F40E8: 38E00006  li r7, 6
	ctx.r[7].s64 = 6;
	// 825F40EC: 388A456C  addi r4, r10, 0x456c
	ctx.r[4].s64 = ctx.r[10].s64 + 17772;
	// 825F40F0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825F40F4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F40F8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825F40FC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F4100: 386AD374  addi r3, r10, -0x2c8c
	ctx.r[3].s64 = ctx.r[10].s64 + -11404;
	// 825F4104: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 825F4108: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 825F410C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F4110: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825F4114: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F4118: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825F411C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825F4120: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F4124: 4BE72CFD  bl 0x82466e20
	ctx.lr = 0x825F4128;
	sub_82466E20(ctx, base);
	// 825F4128: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F412C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F4130: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F4134: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F4138(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F4138 size=112
    let mut pc: u32 = 0x825F4138;
    'dispatch: loop {
        match pc {
            0x825F4138 => {
    //   block [0x825F4138..0x825F41A8)
	// 825F4138: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F413C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F4140: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F4144: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F4148: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825F414C: 38AAD254  addi r5, r10, -0x2dac
	ctx.r[5].s64 = ctx.r[10].s64 + -11692;
	// 825F4150: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F4154: 390BB4A0  addi r8, r11, -0x4b60
	ctx.r[8].s64 = ctx.r[11].s64 + -19296;
	// 825F4158: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 825F415C: 388A4590  addi r4, r10, 0x4590
	ctx.r[4].s64 = ctx.r[10].s64 + 17808;
	// 825F4160: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825F4164: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F4168: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825F416C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F4170: 386AD3A4  addi r3, r10, -0x2c5c
	ctx.r[3].s64 = ctx.r[10].s64 + -11356;
	// 825F4174: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 825F4178: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825F417C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825F4180: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825F4184: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F4188: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825F418C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F4190: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F4194: 4BE72C8D  bl 0x82466e20
	ctx.lr = 0x825F4198;
	sub_82466E20(ctx, base);
	// 825F4198: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F419C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F41A0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F41A4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F41A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F41A8 size=112
    let mut pc: u32 = 0x825F41A8;
    'dispatch: loop {
        match pc {
            0x825F41A8 => {
    //   block [0x825F41A8..0x825F4218)
	// 825F41A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F41AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F41B0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F41B4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F41B8: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825F41BC: 38AAD404  addi r5, r10, -0x2bfc
	ctx.r[5].s64 = ctx.r[10].s64 + -11260;
	// 825F41C0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F41C4: 390BB4B8  addi r8, r11, -0x4b48
	ctx.r[8].s64 = ctx.r[11].s64 + -19272;
	// 825F41C8: 39200005  li r9, 5
	ctx.r[9].s64 = 5;
	// 825F41CC: 388A45B0  addi r4, r10, 0x45b0
	ctx.r[4].s64 = ctx.r[10].s64 + 17840;
	// 825F41D0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825F41D4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F41D8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825F41DC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F41E0: 386AD3D4  addi r3, r10, -0x2c2c
	ctx.r[3].s64 = ctx.r[10].s64 + -11308;
	// 825F41E4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 825F41E8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825F41EC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825F41F0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825F41F4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F41F8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825F41FC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F4200: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F4204: 4BE72C1D  bl 0x82466e20
	ctx.lr = 0x825F4208;
	sub_82466E20(ctx, base);
	// 825F4208: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F420C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F4210: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F4214: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F4218(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F4218 size=100
    let mut pc: u32 = 0x825F4218;
    'dispatch: loop {
        match pc {
            0x825F4218 => {
    //   block [0x825F4218..0x825F427C)
	// 825F4218: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F421C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F4220: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F4224: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F4228: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825F422C: 38AAAF74  addi r5, r10, -0x508c
	ctx.r[5].s64 = ctx.r[10].s64 + -20620;
	// 825F4230: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F4234: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825F4238: 388A45C8  addi r4, r10, 0x45c8
	ctx.r[4].s64 = ctx.r[10].s64 + 17864;
	// 825F423C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F4240: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825F4244: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F4248: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825F424C: 386AD404  addi r3, r10, -0x2bfc
	ctx.r[3].s64 = ctx.r[10].s64 + -11260;
	// 825F4250: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F4254: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825F4258: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 825F425C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F4260: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 825F4264: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F4268: 4BE72BB9  bl 0x82466e20
	ctx.lr = 0x825F426C;
	sub_82466E20(ctx, base);
	// 825F426C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F4270: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F4274: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F4278: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F4280(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825F4280 size=24
    let mut pc: u32 = 0x825F4280;
    'dispatch: loop {
        match pc {
            0x825F4280 => {
    //   block [0x825F4280..0x825F4298)
	// 825F4280: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825F4284: 3D408274  lis r10, -0x7d8c
	ctx.r[10].s64 = -2106327040;
	// 825F4288: 394AEF90  addi r10, r10, -0x1070
	ctx.r[10].s64 = ctx.r[10].s64 + -4208;
	// 825F428C: 816BB40C  lwz r11, -0x4bf4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-19444 as u32) ) } as u64;
	// 825F4290: 916A0050  stw r11, 0x50(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 825F4294: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F4298(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F4298 size=116
    let mut pc: u32 = 0x825F4298;
    'dispatch: loop {
        match pc {
            0x825F4298 => {
    //   block [0x825F4298..0x825F430C)
	// 825F4298: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F429C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F42A0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F42A4: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825F42A8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F42AC: 390BEF90  addi r8, r11, -0x1070
	ctx.r[8].s64 = ctx.r[11].s64 + -4208;
	// 825F42B0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825F42B4: 392A1EB0  addi r9, r10, 0x1eb0
	ctx.r[9].s64 = ctx.r[10].s64 + 7856;
	// 825F42B8: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F42BC: 38E00005  li r7, 5
	ctx.r[7].s64 = 5;
	// 825F42C0: 38AAD254  addi r5, r10, -0x2dac
	ctx.r[5].s64 = ctx.r[10].s64 + -11692;
	// 825F42C4: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825F42C8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825F42CC: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F42D0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825F42D4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F42D8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F42DC: 3D608293  lis r11, -0x7d6d
	ctx.r[11].s64 = -2104295424;
	// 825F42E0: 388A45EC  addi r4, r10, 0x45ec
	ctx.r[4].s64 = ctx.r[10].s64 + 17900;
	// 825F42E4: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 825F42E8: 386BD434  addi r3, r11, -0x2bcc
	ctx.r[3].s64 = ctx.r[11].s64 + -11212;
	// 825F42EC: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 825F42F0: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F42F4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F42F8: 4BE72B29  bl 0x82466e20
	ctx.lr = 0x825F42FC;
	sub_82466E20(ctx, base);
	// 825F42FC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F4300: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F4304: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F4308: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F4310(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F4310 size=108
    let mut pc: u32 = 0x825F4310;
    'dispatch: loop {
        match pc {
            0x825F4310 => {
    //   block [0x825F4310..0x825F437C)
	// 825F4310: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F4314: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F4318: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F431C: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825F4320: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F4324: 38EBB530  addi r7, r11, -0x4ad0
	ctx.r[7].s64 = ctx.r[11].s64 + -19152;
	// 825F4328: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 825F432C: 388A4608  addi r4, r10, 0x4608
	ctx.r[4].s64 = ctx.r[10].s64 + 17928;
	// 825F4330: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825F4334: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F4338: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 825F433C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825F4340: 386AD464  addi r3, r10, -0x2b9c
	ctx.r[3].s64 = ctx.r[10].s64 + -11164;
	// 825F4344: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 825F4348: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825F434C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F4350: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825F4354: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F4358: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825F435C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F4360: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F4364: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825F4368: 4BE72AB9  bl 0x82466e20
	ctx.lr = 0x825F436C;
	sub_82466E20(ctx, base);
	// 825F436C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F4370: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F4374: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F4378: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F4380(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F4380 size=112
    let mut pc: u32 = 0x825F4380;
    'dispatch: loop {
        match pc {
            0x825F4380 => {
    //   block [0x825F4380..0x825F43F0)
	// 825F4380: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F4384: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F4388: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F438C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F4390: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825F4394: 38AAD254  addi r5, r10, -0x2dac
	ctx.r[5].s64 = ctx.r[10].s64 + -11692;
	// 825F4398: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F439C: 390BB560  addi r8, r11, -0x4aa0
	ctx.r[8].s64 = ctx.r[11].s64 + -19104;
	// 825F43A0: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 825F43A4: 388A462C  addi r4, r10, 0x462c
	ctx.r[4].s64 = ctx.r[10].s64 + 17964;
	// 825F43A8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825F43AC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F43B0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825F43B4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F43B8: 386AD494  addi r3, r10, -0x2b6c
	ctx.r[3].s64 = ctx.r[10].s64 + -11116;
	// 825F43BC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 825F43C0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825F43C4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825F43C8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825F43CC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F43D0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825F43D4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F43D8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F43DC: 4BE72A45  bl 0x82466e20
	ctx.lr = 0x825F43E0;
	sub_82466E20(ctx, base);
	// 825F43E0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F43E4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F43E8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F43EC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F43F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F43F0 size=112
    let mut pc: u32 = 0x825F43F0;
    'dispatch: loop {
        match pc {
            0x825F43F0 => {
    //   block [0x825F43F0..0x825F4460)
	// 825F43F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F43F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F43F8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F43FC: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F4400: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825F4404: 392A1ED4  addi r9, r10, 0x1ed4
	ctx.r[9].s64 = ctx.r[10].s64 + 7892;
	// 825F4408: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F440C: 390BB580  addi r8, r11, -0x4a80
	ctx.r[8].s64 = ctx.r[11].s64 + -19072;
	// 825F4410: 38E00007  li r7, 7
	ctx.r[7].s64 = 7;
	// 825F4414: 388A464C  addi r4, r10, 0x464c
	ctx.r[4].s64 = ctx.r[10].s64 + 17996;
	// 825F4418: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825F441C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F4420: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825F4424: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F4428: 386AD4C4  addi r3, r10, -0x2b3c
	ctx.r[3].s64 = ctx.r[10].s64 + -11068;
	// 825F442C: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 825F4430: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 825F4434: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F4438: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825F443C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F4440: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825F4444: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825F4448: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F444C: 4BE729D5  bl 0x82466e20
	ctx.lr = 0x825F4450;
	sub_82466E20(ctx, base);
	// 825F4450: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F4454: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F4458: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F445C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F4460(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F4460 size=112
    let mut pc: u32 = 0x825F4460;
    'dispatch: loop {
        match pc {
            0x825F4460 => {
    //   block [0x825F4460..0x825F44D0)
	// 825F4460: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F4464: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F4468: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F446C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F4470: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825F4474: 38AAD254  addi r5, r10, -0x2dac
	ctx.r[5].s64 = ctx.r[10].s64 + -11692;
	// 825F4478: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F447C: 390BB628  addi r8, r11, -0x49d8
	ctx.r[8].s64 = ctx.r[11].s64 + -18904;
	// 825F4480: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 825F4484: 388A466C  addi r4, r10, 0x466c
	ctx.r[4].s64 = ctx.r[10].s64 + 18028;
	// 825F4488: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825F448C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F4490: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825F4494: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F4498: 386AD4F4  addi r3, r10, -0x2b0c
	ctx.r[3].s64 = ctx.r[10].s64 + -11020;
	// 825F449C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 825F44A0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825F44A4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825F44A8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825F44AC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F44B0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825F44B4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F44B8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F44BC: 4BE72965  bl 0x82466e20
	ctx.lr = 0x825F44C0;
	sub_82466E20(ctx, base);
	// 825F44C0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F44C4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F44C8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F44CC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F44D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F44D0 size=108
    let mut pc: u32 = 0x825F44D0;
    'dispatch: loop {
        match pc {
            0x825F44D0 => {
    //   block [0x825F44D0..0x825F453C)
	// 825F44D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F44D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F44D8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F44DC: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825F44E0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F44E4: 38EBB640  addi r7, r11, -0x49c0
	ctx.r[7].s64 = ctx.r[11].s64 + -18880;
	// 825F44E8: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 825F44EC: 388A4688  addi r4, r10, 0x4688
	ctx.r[4].s64 = ctx.r[10].s64 + 18056;
	// 825F44F0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825F44F4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F44F8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 825F44FC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825F4500: 386AD524  addi r3, r10, -0x2adc
	ctx.r[3].s64 = ctx.r[10].s64 + -10972;
	// 825F4504: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 825F4508: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825F450C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F4510: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825F4514: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F4518: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825F451C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F4520: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F4524: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825F4528: 4BE728F9  bl 0x82466e20
	ctx.lr = 0x825F452C;
	sub_82466E20(ctx, base);
	// 825F452C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F4530: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F4534: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F4538: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F4540(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F4540 size=112
    let mut pc: u32 = 0x825F4540;
    'dispatch: loop {
        match pc {
            0x825F4540 => {
    //   block [0x825F4540..0x825F45B0)
	// 825F4540: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F4544: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F4548: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F454C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F4550: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825F4554: 38AAD254  addi r5, r10, -0x2dac
	ctx.r[5].s64 = ctx.r[10].s64 + -11692;
	// 825F4558: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F455C: 390BB670  addi r8, r11, -0x4990
	ctx.r[8].s64 = ctx.r[11].s64 + -18832;
	// 825F4560: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 825F4564: 388A46AC  addi r4, r10, 0x46ac
	ctx.r[4].s64 = ctx.r[10].s64 + 18092;
	// 825F4568: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825F456C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F4570: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825F4574: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F4578: 386AD554  addi r3, r10, -0x2aac
	ctx.r[3].s64 = ctx.r[10].s64 + -10924;
	// 825F457C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 825F4580: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825F4584: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825F4588: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825F458C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F4590: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825F4594: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F4598: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F459C: 4BE72885  bl 0x82466e20
	ctx.lr = 0x825F45A0;
	sub_82466E20(ctx, base);
	// 825F45A0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F45A4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F45A8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F45AC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F45B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F45B0 size=112
    let mut pc: u32 = 0x825F45B0;
    'dispatch: loop {
        match pc {
            0x825F45B0 => {
    //   block [0x825F45B0..0x825F4620)
	// 825F45B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F45B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F45B8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F45BC: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F45C0: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825F45C4: 392A1F08  addi r9, r10, 0x1f08
	ctx.r[9].s64 = ctx.r[10].s64 + 7944;
	// 825F45C8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F45CC: 390BB688  addi r8, r11, -0x4978
	ctx.r[8].s64 = ctx.r[11].s64 + -18808;
	// 825F45D0: 38E00007  li r7, 7
	ctx.r[7].s64 = 7;
	// 825F45D4: 388A46C8  addi r4, r10, 0x46c8
	ctx.r[4].s64 = ctx.r[10].s64 + 18120;
	// 825F45D8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825F45DC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F45E0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825F45E4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F45E8: 386AD584  addi r3, r10, -0x2a7c
	ctx.r[3].s64 = ctx.r[10].s64 + -10876;
	// 825F45EC: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 825F45F0: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 825F45F4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F45F8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825F45FC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F4600: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825F4604: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825F4608: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F460C: 4BE72815  bl 0x82466e20
	ctx.lr = 0x825F4610;
	sub_82466E20(ctx, base);
	// 825F4610: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F4614: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F4618: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F461C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F4620(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F4620 size=112
    let mut pc: u32 = 0x825F4620;
    'dispatch: loop {
        match pc {
            0x825F4620 => {
    //   block [0x825F4620..0x825F4690)
	// 825F4620: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F4624: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F4628: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F462C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F4630: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825F4634: 38AAD254  addi r5, r10, -0x2dac
	ctx.r[5].s64 = ctx.r[10].s64 + -11692;
	// 825F4638: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F463C: 390BB730  addi r8, r11, -0x48d0
	ctx.r[8].s64 = ctx.r[11].s64 + -18640;
	// 825F4640: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 825F4644: 388A46E4  addi r4, r10, 0x46e4
	ctx.r[4].s64 = ctx.r[10].s64 + 18148;
	// 825F4648: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825F464C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F4650: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825F4654: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F4658: 386AD5B4  addi r3, r10, -0x2a4c
	ctx.r[3].s64 = ctx.r[10].s64 + -10828;
	// 825F465C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 825F4660: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825F4664: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825F4668: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825F466C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F4670: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825F4674: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F4678: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F467C: 4BE727A5  bl 0x82466e20
	ctx.lr = 0x825F4680;
	sub_82466E20(ctx, base);
	// 825F4680: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F4684: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F4688: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F468C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F4690(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F4690 size=112
    let mut pc: u32 = 0x825F4690;
    'dispatch: loop {
        match pc {
            0x825F4690 => {
    //   block [0x825F4690..0x825F4700)
	// 825F4690: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F4694: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F4698: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F469C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F46A0: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825F46A4: 38AAD254  addi r5, r10, -0x2dac
	ctx.r[5].s64 = ctx.r[10].s64 + -11692;
	// 825F46A8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F46AC: 390BB778  addi r8, r11, -0x4888
	ctx.r[8].s64 = ctx.r[11].s64 + -18568;
	// 825F46B0: 39200008  li r9, 8
	ctx.r[9].s64 = 8;
	// 825F46B4: 388A46FC  addi r4, r10, 0x46fc
	ctx.r[4].s64 = ctx.r[10].s64 + 18172;
	// 825F46B8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825F46BC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F46C0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825F46C4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F46C8: 386AD5E4  addi r3, r10, -0x2a1c
	ctx.r[3].s64 = ctx.r[10].s64 + -10780;
	// 825F46CC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 825F46D0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825F46D4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825F46D8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825F46DC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F46E0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825F46E4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F46E8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F46EC: 4BE72735  bl 0x82466e20
	ctx.lr = 0x825F46F0;
	sub_82466E20(ctx, base);
	// 825F46F0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F46F4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F46F8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F46FC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F4700(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F4700 size=100
    let mut pc: u32 = 0x825F4700;
    'dispatch: loop {
        match pc {
            0x825F4700 => {
    //   block [0x825F4700..0x825F4764)
	// 825F4700: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F4704: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F4708: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F470C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F4710: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825F4714: 38AAD254  addi r5, r10, -0x2dac
	ctx.r[5].s64 = ctx.r[10].s64 + -11692;
	// 825F4718: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F471C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825F4720: 388A4718  addi r4, r10, 0x4718
	ctx.r[4].s64 = ctx.r[10].s64 + 18200;
	// 825F4724: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F4728: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825F472C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F4730: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825F4734: 386AD614  addi r3, r10, -0x29ec
	ctx.r[3].s64 = ctx.r[10].s64 + -10732;
	// 825F4738: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F473C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825F4740: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 825F4744: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F4748: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 825F474C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F4750: 4BE726D1  bl 0x82466e20
	ctx.lr = 0x825F4754;
	sub_82466E20(ctx, base);
	// 825F4754: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F4758: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F475C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F4760: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F4768(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F4768 size=112
    let mut pc: u32 = 0x825F4768;
    'dispatch: loop {
        match pc {
            0x825F4768 => {
    //   block [0x825F4768..0x825F47D8)
	// 825F4768: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F476C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F4770: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F4774: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F4778: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825F477C: 38AAD284  addi r5, r10, -0x2d7c
	ctx.r[5].s64 = ctx.r[10].s64 + -11644;
	// 825F4780: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F4784: 390BB838  addi r8, r11, -0x47c8
	ctx.r[8].s64 = ctx.r[11].s64 + -18376;
	// 825F4788: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 825F478C: 388A4730  addi r4, r10, 0x4730
	ctx.r[4].s64 = ctx.r[10].s64 + 18224;
	// 825F4790: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825F4794: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F4798: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825F479C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F47A0: 386AD644  addi r3, r10, -0x29bc
	ctx.r[3].s64 = ctx.r[10].s64 + -10684;
	// 825F47A4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 825F47A8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825F47AC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825F47B0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825F47B4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F47B8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825F47BC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F47C0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F47C4: 4BE7265D  bl 0x82466e20
	ctx.lr = 0x825F47C8;
	sub_82466E20(ctx, base);
	// 825F47C8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F47CC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F47D0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F47D4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F47D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F47D8 size=112
    let mut pc: u32 = 0x825F47D8;
    'dispatch: loop {
        match pc {
            0x825F47D8 => {
    //   block [0x825F47D8..0x825F4848)
	// 825F47D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F47DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F47E0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F47E4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F47E8: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825F47EC: 38AAD104  addi r5, r10, -0x2efc
	ctx.r[5].s64 = ctx.r[10].s64 + -12028;
	// 825F47F0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F47F4: 390BB868  addi r8, r11, -0x4798
	ctx.r[8].s64 = ctx.r[11].s64 + -18328;
	// 825F47F8: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 825F47FC: 388A474C  addi r4, r10, 0x474c
	ctx.r[4].s64 = ctx.r[10].s64 + 18252;
	// 825F4800: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825F4804: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F4808: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825F480C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F4810: 386AD674  addi r3, r10, -0x298c
	ctx.r[3].s64 = ctx.r[10].s64 + -10636;
	// 825F4814: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 825F4818: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825F481C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825F4820: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825F4824: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F4828: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825F482C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F4830: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F4834: 4BE725ED  bl 0x82466e20
	ctx.lr = 0x825F4838;
	sub_82466E20(ctx, base);
	// 825F4838: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F483C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F4840: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F4844: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F4848(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F4848 size=108
    let mut pc: u32 = 0x825F4848;
    'dispatch: loop {
        match pc {
            0x825F4848 => {
    //   block [0x825F4848..0x825F48B4)
	// 825F4848: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F484C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F4850: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F4854: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825F4858: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F485C: 38EBB880  addi r7, r11, -0x4780
	ctx.r[7].s64 = ctx.r[11].s64 + -18304;
	// 825F4860: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 825F4864: 388A476C  addi r4, r10, 0x476c
	ctx.r[4].s64 = ctx.r[10].s64 + 18284;
	// 825F4868: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825F486C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F4870: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 825F4874: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825F4878: 386AD6A4  addi r3, r10, -0x295c
	ctx.r[3].s64 = ctx.r[10].s64 + -10588;
	// 825F487C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 825F4880: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825F4884: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F4888: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825F488C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F4890: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825F4894: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F4898: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F489C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825F48A0: 4BE72581  bl 0x82466e20
	ctx.lr = 0x825F48A4;
	sub_82466E20(ctx, base);
	// 825F48A4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F48A8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F48AC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F48B0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F48B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F48B8 size=112
    let mut pc: u32 = 0x825F48B8;
    'dispatch: loop {
        match pc {
            0x825F48B8 => {
    //   block [0x825F48B8..0x825F4928)
	// 825F48B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F48BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F48C0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F48C4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F48C8: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825F48CC: 38AAD614  addi r5, r10, -0x29ec
	ctx.r[5].s64 = ctx.r[10].s64 + -10732;
	// 825F48D0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F48D4: 390BB8B0  addi r8, r11, -0x4750
	ctx.r[8].s64 = ctx.r[11].s64 + -18256;
	// 825F48D8: 39200006  li r9, 6
	ctx.r[9].s64 = 6;
	// 825F48DC: 388A4790  addi r4, r10, 0x4790
	ctx.r[4].s64 = ctx.r[10].s64 + 18320;
	// 825F48E0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825F48E4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F48E8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825F48EC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F48F0: 386AD6D4  addi r3, r10, -0x292c
	ctx.r[3].s64 = ctx.r[10].s64 + -10540;
	// 825F48F4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 825F48F8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825F48FC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825F4900: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825F4904: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F4908: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825F490C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F4910: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F4914: 4BE7250D  bl 0x82466e20
	ctx.lr = 0x825F4918;
	sub_82466E20(ctx, base);
	// 825F4918: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F491C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F4920: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F4924: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F4928(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F4928 size=112
    let mut pc: u32 = 0x825F4928;
    'dispatch: loop {
        match pc {
            0x825F4928 => {
    //   block [0x825F4928..0x825F4998)
	// 825F4928: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F492C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F4930: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F4934: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F4938: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825F493C: 392A1F34  addi r9, r10, 0x1f34
	ctx.r[9].s64 = ctx.r[10].s64 + 7988;
	// 825F4940: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F4944: 390BB948  addi r8, r11, -0x46b8
	ctx.r[8].s64 = ctx.r[11].s64 + -18104;
	// 825F4948: 38E00003  li r7, 3
	ctx.r[7].s64 = 3;
	// 825F494C: 388A47A8  addi r4, r10, 0x47a8
	ctx.r[4].s64 = ctx.r[10].s64 + 18344;
	// 825F4950: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825F4954: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F4958: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825F495C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F4960: 386AD704  addi r3, r10, -0x28fc
	ctx.r[3].s64 = ctx.r[10].s64 + -10492;
	// 825F4964: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 825F4968: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 825F496C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F4970: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825F4974: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F4978: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825F497C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825F4980: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F4984: 4BE7249D  bl 0x82466e20
	ctx.lr = 0x825F4988;
	sub_82466E20(ctx, base);
	// 825F4988: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F498C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F4990: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F4994: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F4998(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F4998 size=112
    let mut pc: u32 = 0x825F4998;
    'dispatch: loop {
        match pc {
            0x825F4998 => {
    //   block [0x825F4998..0x825F4A08)
	// 825F4998: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F499C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F49A0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F49A4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F49A8: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825F49AC: 38AAD254  addi r5, r10, -0x2dac
	ctx.r[5].s64 = ctx.r[10].s64 + -11692;
	// 825F49B0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F49B4: 390BB990  addi r8, r11, -0x4670
	ctx.r[8].s64 = ctx.r[11].s64 + -18032;
	// 825F49B8: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 825F49BC: 388A47C0  addi r4, r10, 0x47c0
	ctx.r[4].s64 = ctx.r[10].s64 + 18368;
	// 825F49C0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825F49C4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F49C8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825F49CC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F49D0: 386AD734  addi r3, r10, -0x28cc
	ctx.r[3].s64 = ctx.r[10].s64 + -10444;
	// 825F49D4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 825F49D8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825F49DC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825F49E0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825F49E4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F49E8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825F49EC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F49F0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F49F4: 4BE7242D  bl 0x82466e20
	ctx.lr = 0x825F49F8;
	sub_82466E20(ctx, base);
	// 825F49F8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F49FC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F4A00: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F4A04: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F4A08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F4A08 size=108
    let mut pc: u32 = 0x825F4A08;
    'dispatch: loop {
        match pc {
            0x825F4A08 => {
    //   block [0x825F4A08..0x825F4A74)
	// 825F4A08: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F4A0C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F4A10: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F4A14: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825F4A18: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F4A1C: 38EBB9A8  addi r7, r11, -0x4658
	ctx.r[7].s64 = ctx.r[11].s64 + -18008;
	// 825F4A20: 39000006  li r8, 6
	ctx.r[8].s64 = 6;
	// 825F4A24: 388A47D4  addi r4, r10, 0x47d4
	ctx.r[4].s64 = ctx.r[10].s64 + 18388;
	// 825F4A28: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825F4A2C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F4A30: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 825F4A34: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825F4A38: 386AD764  addi r3, r10, -0x289c
	ctx.r[3].s64 = ctx.r[10].s64 + -10396;
	// 825F4A3C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 825F4A40: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825F4A44: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F4A48: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825F4A4C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F4A50: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825F4A54: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F4A58: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F4A5C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825F4A60: 4BE723C1  bl 0x82466e20
	ctx.lr = 0x825F4A64;
	sub_82466E20(ctx, base);
	// 825F4A64: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F4A68: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F4A6C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F4A70: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F4A78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F4A78 size=116
    let mut pc: u32 = 0x825F4A78;
    'dispatch: loop {
        match pc {
            0x825F4A78 => {
    //   block [0x825F4A78..0x825F4AEC)
	// 825F4A78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F4A7C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F4A80: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F4A84: 3D408274  lis r10, -0x7d8c
	ctx.r[10].s64 = -2106327040;
	// 825F4A88: 38E00009  li r7, 9
	ctx.r[7].s64 = 9;
	// 825F4A8C: 390ABA38  addi r8, r10, -0x45c8
	ctx.r[8].s64 = ctx.r[10].s64 + -17864;
	// 825F4A90: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F4A94: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 825F4A98: 38AAD614  addi r5, r10, -0x29ec
	ctx.r[5].s64 = ctx.r[10].s64 + -10732;
	// 825F4A9C: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F4AA0: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 825F4AA4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825F4AA8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825F4AAC: 388A47F8  addi r4, r10, 0x47f8
	ctx.r[4].s64 = ctx.r[10].s64 + 18424;
	// 825F4AB0: 396B1F48  addi r11, r11, 0x1f48
	ctx.r[11].s64 = ctx.r[11].s64 + 8008;
	// 825F4AB4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F4AB8: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F4ABC: 386AD794  addi r3, r10, -0x286c
	ctx.r[3].s64 = ctx.r[10].s64 + -10348;
	// 825F4AC0: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 825F4AC4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825F4AC8: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 825F4ACC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F4AD0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F4AD4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F4AD8: 4BE72349  bl 0x82466e20
	ctx.lr = 0x825F4ADC;
	sub_82466E20(ctx, base);
	// 825F4ADC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F4AE0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F4AE4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F4AE8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F4AF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F4AF0 size=108
    let mut pc: u32 = 0x825F4AF0;
    'dispatch: loop {
        match pc {
            0x825F4AF0 => {
    //   block [0x825F4AF0..0x825F4B5C)
	// 825F4AF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F4AF4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F4AF8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F4AFC: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825F4B00: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F4B04: 38EBBB10  addi r7, r11, -0x44f0
	ctx.r[7].s64 = ctx.r[11].s64 + -17648;
	// 825F4B08: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 825F4B0C: 388A480C  addi r4, r10, 0x480c
	ctx.r[4].s64 = ctx.r[10].s64 + 18444;
	// 825F4B10: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825F4B14: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F4B18: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 825F4B1C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825F4B20: 386AD7C4  addi r3, r10, -0x283c
	ctx.r[3].s64 = ctx.r[10].s64 + -10300;
	// 825F4B24: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 825F4B28: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825F4B2C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F4B30: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825F4B34: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F4B38: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825F4B3C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F4B40: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F4B44: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825F4B48: 4BE722D9  bl 0x82466e20
	ctx.lr = 0x825F4B4C;
	sub_82466E20(ctx, base);
	// 825F4B4C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F4B50: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F4B54: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F4B58: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F4B60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F4B60 size=112
    let mut pc: u32 = 0x825F4B60;
    'dispatch: loop {
        match pc {
            0x825F4B60 => {
    //   block [0x825F4B60..0x825F4BD0)
	// 825F4B60: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F4B64: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F4B68: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F4B6C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F4B70: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825F4B74: 38AAD614  addi r5, r10, -0x29ec
	ctx.r[5].s64 = ctx.r[10].s64 + -10732;
	// 825F4B78: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F4B7C: 390BBB58  addi r8, r11, -0x44a8
	ctx.r[8].s64 = ctx.r[11].s64 + -17576;
	// 825F4B80: 39200005  li r9, 5
	ctx.r[9].s64 = 5;
	// 825F4B84: 388A4834  addi r4, r10, 0x4834
	ctx.r[4].s64 = ctx.r[10].s64 + 18484;
	// 825F4B88: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825F4B8C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F4B90: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825F4B94: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F4B98: 386AD7F4  addi r3, r10, -0x280c
	ctx.r[3].s64 = ctx.r[10].s64 + -10252;
	// 825F4B9C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 825F4BA0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825F4BA4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825F4BA8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825F4BAC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F4BB0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825F4BB4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F4BB8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F4BBC: 4BE72265  bl 0x82466e20
	ctx.lr = 0x825F4BC0;
	sub_82466E20(ctx, base);
	// 825F4BC0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F4BC4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F4BC8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F4BCC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F4BD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F4BD0 size=112
    let mut pc: u32 = 0x825F4BD0;
    'dispatch: loop {
        match pc {
            0x825F4BD0 => {
    //   block [0x825F4BD0..0x825F4C40)
	// 825F4BD0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F4BD4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F4BD8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F4BDC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F4BE0: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825F4BE4: 38AAD254  addi r5, r10, -0x2dac
	ctx.r[5].s64 = ctx.r[10].s64 + -11692;
	// 825F4BE8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F4BEC: 390BBBD0  addi r8, r11, -0x4430
	ctx.r[8].s64 = ctx.r[11].s64 + -17456;
	// 825F4BF0: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 825F4BF4: 388A484C  addi r4, r10, 0x484c
	ctx.r[4].s64 = ctx.r[10].s64 + 18508;
	// 825F4BF8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825F4BFC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F4C00: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825F4C04: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F4C08: 386AD824  addi r3, r10, -0x27dc
	ctx.r[3].s64 = ctx.r[10].s64 + -10204;
	// 825F4C0C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 825F4C10: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825F4C14: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825F4C18: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825F4C1C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F4C20: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825F4C24: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F4C28: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F4C2C: 4BE721F5  bl 0x82466e20
	ctx.lr = 0x825F4C30;
	sub_82466E20(ctx, base);
	// 825F4C30: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F4C34: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F4C38: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F4C3C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F4C40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F4C40 size=108
    let mut pc: u32 = 0x825F4C40;
    'dispatch: loop {
        match pc {
            0x825F4C40 => {
    //   block [0x825F4C40..0x825F4CAC)
	// 825F4C40: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F4C44: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F4C48: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F4C4C: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825F4C50: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F4C54: 38EBBC00  addi r7, r11, -0x4400
	ctx.r[7].s64 = ctx.r[11].s64 + -17408;
	// 825F4C58: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 825F4C5C: 388A4864  addi r4, r10, 0x4864
	ctx.r[4].s64 = ctx.r[10].s64 + 18532;
	// 825F4C60: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825F4C64: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F4C68: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 825F4C6C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825F4C70: 386AD854  addi r3, r10, -0x27ac
	ctx.r[3].s64 = ctx.r[10].s64 + -10156;
	// 825F4C74: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 825F4C78: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825F4C7C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F4C80: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825F4C84: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F4C88: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825F4C8C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F4C90: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F4C94: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825F4C98: 4BE72189  bl 0x82466e20
	ctx.lr = 0x825F4C9C;
	sub_82466E20(ctx, base);
	// 825F4C9C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F4CA0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F4CA4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F4CA8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F4CB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F4CB0 size=108
    let mut pc: u32 = 0x825F4CB0;
    'dispatch: loop {
        match pc {
            0x825F4CB0 => {
    //   block [0x825F4CB0..0x825F4D1C)
	// 825F4CB0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F4CB4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F4CB8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F4CBC: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825F4CC0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F4CC4: 38EBBC60  addi r7, r11, -0x43a0
	ctx.r[7].s64 = ctx.r[11].s64 + -17312;
	// 825F4CC8: 39000005  li r8, 5
	ctx.r[8].s64 = 5;
	// 825F4CCC: 388A4890  addi r4, r10, 0x4890
	ctx.r[4].s64 = ctx.r[10].s64 + 18576;
	// 825F4CD0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825F4CD4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F4CD8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 825F4CDC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825F4CE0: 386AD884  addi r3, r10, -0x277c
	ctx.r[3].s64 = ctx.r[10].s64 + -10108;
	// 825F4CE4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 825F4CE8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825F4CEC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F4CF0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825F4CF4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F4CF8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825F4CFC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F4D00: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F4D04: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825F4D08: 4BE72119  bl 0x82466e20
	ctx.lr = 0x825F4D0C;
	sub_82466E20(ctx, base);
	// 825F4D0C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F4D10: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F4D14: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F4D18: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F4D20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F4D20 size=112
    let mut pc: u32 = 0x825F4D20;
    'dispatch: loop {
        match pc {
            0x825F4D20 => {
    //   block [0x825F4D20..0x825F4D90)
	// 825F4D20: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F4D24: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F4D28: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F4D2C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F4D30: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825F4D34: 38AAD254  addi r5, r10, -0x2dac
	ctx.r[5].s64 = ctx.r[10].s64 + -11692;
	// 825F4D38: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F4D3C: 390BBCD8  addi r8, r11, -0x4328
	ctx.r[8].s64 = ctx.r[11].s64 + -17192;
	// 825F4D40: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 825F4D44: 388A48B0  addi r4, r10, 0x48b0
	ctx.r[4].s64 = ctx.r[10].s64 + 18608;
	// 825F4D48: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825F4D4C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F4D50: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825F4D54: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F4D58: 386AD8B4  addi r3, r10, -0x274c
	ctx.r[3].s64 = ctx.r[10].s64 + -10060;
	// 825F4D5C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 825F4D60: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825F4D64: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825F4D68: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825F4D6C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F4D70: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825F4D74: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F4D78: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F4D7C: 4BE720A5  bl 0x82466e20
	ctx.lr = 0x825F4D80;
	sub_82466E20(ctx, base);
	// 825F4D80: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F4D84: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F4D88: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F4D8C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F4D90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825F4D90 size=24
    let mut pc: u32 = 0x825F4D90;
    'dispatch: loop {
        match pc {
            0x825F4D90 => {
    //   block [0x825F4D90..0x825F4DA8)
	// 825F4D90: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825F4D94: 3D408274  lis r10, -0x7d8c
	ctx.r[10].s64 = -2106327040;
	// 825F4D98: 394AF008  addi r10, r10, -0xff8
	ctx.r[10].s64 = ctx.r[10].s64 + -4088;
	// 825F4D9C: 816BB944  lwz r11, -0x46bc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-18108 as u32) ) } as u64;
	// 825F4DA0: 916A0008  stw r11, 8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 825F4DA4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F4DA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F4DA8 size=116
    let mut pc: u32 = 0x825F4DA8;
    'dispatch: loop {
        match pc {
            0x825F4DA8 => {
    //   block [0x825F4DA8..0x825F4E1C)
	// 825F4DA8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F4DAC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F4DB0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F4DB4: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825F4DB8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F4DBC: 390BF008  addi r8, r11, -0xff8
	ctx.r[8].s64 = ctx.r[11].s64 + -4088;
	// 825F4DC0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825F4DC4: 392A1FAC  addi r9, r10, 0x1fac
	ctx.r[9].s64 = ctx.r[10].s64 + 8108;
	// 825F4DC8: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F4DCC: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
	// 825F4DD0: 38AAAF74  addi r5, r10, -0x508c
	ctx.r[5].s64 = ctx.r[10].s64 + -20620;
	// 825F4DD4: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825F4DD8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825F4DDC: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F4DE0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825F4DE4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F4DE8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F4DEC: 3D608293  lis r11, -0x7d6d
	ctx.r[11].s64 = -2104295424;
	// 825F4DF0: 388A48CC  addi r4, r10, 0x48cc
	ctx.r[4].s64 = ctx.r[10].s64 + 18636;
	// 825F4DF4: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 825F4DF8: 386BD8E4  addi r3, r11, -0x271c
	ctx.r[3].s64 = ctx.r[11].s64 + -10012;
	// 825F4DFC: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 825F4E00: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F4E04: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F4E08: 4BE72019  bl 0x82466e20
	ctx.lr = 0x825F4E0C;
	sub_82466E20(ctx, base);
	// 825F4E0C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F4E10: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F4E14: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F4E18: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F4E20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F4E20 size=112
    let mut pc: u32 = 0x825F4E20;
    'dispatch: loop {
        match pc {
            0x825F4E20 => {
    //   block [0x825F4E20..0x825F4E90)
	// 825F4E20: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F4E24: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F4E28: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F4E2C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F4E30: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825F4E34: 38AAD8E4  addi r5, r10, -0x271c
	ctx.r[5].s64 = ctx.r[10].s64 + -10012;
	// 825F4E38: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F4E3C: 390BBD20  addi r8, r11, -0x42e0
	ctx.r[8].s64 = ctx.r[11].s64 + -17120;
	// 825F4E40: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 825F4E44: 388A48E0  addi r4, r10, 0x48e0
	ctx.r[4].s64 = ctx.r[10].s64 + 18656;
	// 825F4E48: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825F4E4C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F4E50: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825F4E54: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F4E58: 386AD914  addi r3, r10, -0x26ec
	ctx.r[3].s64 = ctx.r[10].s64 + -9964;
	// 825F4E5C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 825F4E60: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825F4E64: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825F4E68: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825F4E6C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F4E70: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825F4E74: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F4E78: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F4E7C: 4BE71FA5  bl 0x82466e20
	ctx.lr = 0x825F4E80;
	sub_82466E20(ctx, base);
	// 825F4E80: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F4E84: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F4E88: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F4E8C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F4E90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825F4E90 size=24
    let mut pc: u32 = 0x825F4E90;
    'dispatch: loop {
        match pc {
            0x825F4E90 => {
    //   block [0x825F4E90..0x825F4EA8)
	// 825F4E90: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825F4E94: 3D408274  lis r10, -0x7d8c
	ctx.r[10].s64 = -2106327040;
	// 825F4E98: 394AF020  addi r10, r10, -0xfe0
	ctx.r[10].s64 = ctx.r[10].s64 + -4064;
	// 825F4E9C: 816BBD50  lwz r11, -0x42b0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-17072 as u32) ) } as u64;
	// 825F4EA0: 916A0020  stw r11, 0x20(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 825F4EA4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F4EA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F4EA8 size=116
    let mut pc: u32 = 0x825F4EA8;
    'dispatch: loop {
        match pc {
            0x825F4EA8 => {
    //   block [0x825F4EA8..0x825F4F1C)
	// 825F4EA8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F4EAC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F4EB0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F4EB4: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825F4EB8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F4EBC: 390BF020  addi r8, r11, -0xfe0
	ctx.r[8].s64 = ctx.r[11].s64 + -4064;
	// 825F4EC0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825F4EC4: 392A1FE8  addi r9, r10, 0x1fe8
	ctx.r[9].s64 = ctx.r[10].s64 + 8168;
	// 825F4EC8: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F4ECC: 38E00005  li r7, 5
	ctx.r[7].s64 = 5;
	// 825F4ED0: 38AAD914  addi r5, r10, -0x26ec
	ctx.r[5].s64 = ctx.r[10].s64 + -9964;
	// 825F4ED4: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825F4ED8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825F4EDC: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F4EE0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825F4EE4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F4EE8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F4EEC: 3D608293  lis r11, -0x7d6d
	ctx.r[11].s64 = -2104295424;
	// 825F4EF0: 388A490C  addi r4, r10, 0x490c
	ctx.r[4].s64 = ctx.r[10].s64 + 18700;
	// 825F4EF4: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 825F4EF8: 386BD944  addi r3, r11, -0x26bc
	ctx.r[3].s64 = ctx.r[11].s64 + -9916;
	// 825F4EFC: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 825F4F00: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F4F04: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F4F08: 4BE71F19  bl 0x82466e20
	ctx.lr = 0x825F4F0C;
	sub_82466E20(ctx, base);
	// 825F4F0C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F4F10: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F4F14: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F4F18: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F4F20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F4F20 size=112
    let mut pc: u32 = 0x825F4F20;
    'dispatch: loop {
        match pc {
            0x825F4F20 => {
    //   block [0x825F4F20..0x825F4F90)
	// 825F4F20: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F4F24: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F4F28: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F4F2C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F4F30: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825F4F34: 38AAD914  addi r5, r10, -0x26ec
	ctx.r[5].s64 = ctx.r[10].s64 + -9964;
	// 825F4F38: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F4F3C: 390BBD58  addi r8, r11, -0x42a8
	ctx.r[8].s64 = ctx.r[11].s64 + -17064;
	// 825F4F40: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 825F4F44: 388A4928  addi r4, r10, 0x4928
	ctx.r[4].s64 = ctx.r[10].s64 + 18728;
	// 825F4F48: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825F4F4C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F4F50: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825F4F54: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F4F58: 386AD974  addi r3, r10, -0x268c
	ctx.r[3].s64 = ctx.r[10].s64 + -9868;
	// 825F4F5C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 825F4F60: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825F4F64: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825F4F68: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825F4F6C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F4F70: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825F4F74: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F4F78: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F4F7C: 4BE71EA5  bl 0x82466e20
	ctx.lr = 0x825F4F80;
	sub_82466E20(ctx, base);
	// 825F4F80: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F4F84: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F4F88: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F4F8C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F4F90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F4F90 size=112
    let mut pc: u32 = 0x825F4F90;
    'dispatch: loop {
        match pc {
            0x825F4F90 => {
    //   block [0x825F4F90..0x825F5000)
	// 825F4F90: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F4F94: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F4F98: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F4F9C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F4FA0: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825F4FA4: 38AAD914  addi r5, r10, -0x26ec
	ctx.r[5].s64 = ctx.r[10].s64 + -9964;
	// 825F4FA8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F4FAC: 390BBDB8  addi r8, r11, -0x4248
	ctx.r[8].s64 = ctx.r[11].s64 + -16968;
	// 825F4FB0: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 825F4FB4: 388A4944  addi r4, r10, 0x4944
	ctx.r[4].s64 = ctx.r[10].s64 + 18756;
	// 825F4FB8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825F4FBC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F4FC0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825F4FC4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F4FC8: 386AD9A4  addi r3, r10, -0x265c
	ctx.r[3].s64 = ctx.r[10].s64 + -9820;
	// 825F4FCC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 825F4FD0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825F4FD4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825F4FD8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825F4FDC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F4FE0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825F4FE4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F4FE8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F4FEC: 4BE71E35  bl 0x82466e20
	ctx.lr = 0x825F4FF0;
	sub_82466E20(ctx, base);
	// 825F4FF0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F4FF4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F4FF8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F4FFC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F5000(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F5000 size=112
    let mut pc: u32 = 0x825F5000;
    'dispatch: loop {
        match pc {
            0x825F5000 => {
    //   block [0x825F5000..0x825F5070)
	// 825F5000: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F5004: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F5008: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F500C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F5010: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825F5014: 38AAD914  addi r5, r10, -0x26ec
	ctx.r[5].s64 = ctx.r[10].s64 + -9964;
	// 825F5018: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F501C: 390BBDE8  addi r8, r11, -0x4218
	ctx.r[8].s64 = ctx.r[11].s64 + -16920;
	// 825F5020: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 825F5024: 388A4964  addi r4, r10, 0x4964
	ctx.r[4].s64 = ctx.r[10].s64 + 18788;
	// 825F5028: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825F502C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F5030: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825F5034: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F5038: 386AD9D4  addi r3, r10, -0x262c
	ctx.r[3].s64 = ctx.r[10].s64 + -9772;
	// 825F503C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 825F5040: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825F5044: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825F5048: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825F504C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F5050: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825F5054: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F5058: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F505C: 4BE71DC5  bl 0x82466e20
	ctx.lr = 0x825F5060;
	sub_82466E20(ctx, base);
	// 825F5060: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F5064: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F5068: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F506C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F5070(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F5070 size=108
    let mut pc: u32 = 0x825F5070;
    'dispatch: loop {
        match pc {
            0x825F5070 => {
    //   block [0x825F5070..0x825F50DC)
	// 825F5070: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F5074: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F5078: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F507C: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825F5080: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F5084: 38EBBE30  addi r7, r11, -0x41d0
	ctx.r[7].s64 = ctx.r[11].s64 + -16848;
	// 825F5088: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 825F508C: 388A4980  addi r4, r10, 0x4980
	ctx.r[4].s64 = ctx.r[10].s64 + 18816;
	// 825F5090: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825F5094: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F5098: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 825F509C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825F50A0: 386ADA04  addi r3, r10, -0x25fc
	ctx.r[3].s64 = ctx.r[10].s64 + -9724;
	// 825F50A4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 825F50A8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825F50AC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F50B0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825F50B4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F50B8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825F50BC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F50C0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F50C4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825F50C8: 4BE71D59  bl 0x82466e20
	ctx.lr = 0x825F50CC;
	sub_82466E20(ctx, base);
	// 825F50CC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F50D0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F50D4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F50D8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F50E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F50E0 size=112
    let mut pc: u32 = 0x825F50E0;
    'dispatch: loop {
        match pc {
            0x825F50E0 => {
    //   block [0x825F50E0..0x825F5150)
	// 825F50E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F50E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F50E8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F50EC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F50F0: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825F50F4: 38AAD254  addi r5, r10, -0x2dac
	ctx.r[5].s64 = ctx.r[10].s64 + -11692;
	// 825F50F8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F50FC: 390BBE60  addi r8, r11, -0x41a0
	ctx.r[8].s64 = ctx.r[11].s64 + -16800;
	// 825F5100: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 825F5104: 388A499C  addi r4, r10, 0x499c
	ctx.r[4].s64 = ctx.r[10].s64 + 18844;
	// 825F5108: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825F510C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F5110: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825F5114: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F5118: 386ADA34  addi r3, r10, -0x25cc
	ctx.r[3].s64 = ctx.r[10].s64 + -9676;
	// 825F511C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 825F5120: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825F5124: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825F5128: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825F512C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F5130: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825F5134: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F5138: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F513C: 4BE71CE5  bl 0x82466e20
	ctx.lr = 0x825F5140;
	sub_82466E20(ctx, base);
	// 825F5140: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F5144: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F5148: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F514C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F5150(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F5150 size=108
    let mut pc: u32 = 0x825F5150;
    'dispatch: loop {
        match pc {
            0x825F5150 => {
    //   block [0x825F5150..0x825F51BC)
	// 825F5150: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F5154: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F5158: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F515C: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825F5160: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F5164: 38EBBE78  addi r7, r11, -0x4188
	ctx.r[7].s64 = ctx.r[11].s64 + -16776;
	// 825F5168: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 825F516C: 388A49B4  addi r4, r10, 0x49b4
	ctx.r[4].s64 = ctx.r[10].s64 + 18868;
	// 825F5170: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825F5174: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F5178: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 825F517C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825F5180: 386ADA64  addi r3, r10, -0x259c
	ctx.r[3].s64 = ctx.r[10].s64 + -9628;
	// 825F5184: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 825F5188: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825F518C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F5190: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825F5194: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F5198: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825F519C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F51A0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F51A4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825F51A8: 4BE71C79  bl 0x82466e20
	ctx.lr = 0x825F51AC;
	sub_82466E20(ctx, base);
	// 825F51AC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F51B0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F51B4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F51B8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F51C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F51C0 size=108
    let mut pc: u32 = 0x825F51C0;
    'dispatch: loop {
        match pc {
            0x825F51C0 => {
    //   block [0x825F51C0..0x825F522C)
	// 825F51C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F51C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F51C8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F51CC: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825F51D0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F51D4: 38EBBEC0  addi r7, r11, -0x4140
	ctx.r[7].s64 = ctx.r[11].s64 + -16704;
	// 825F51D8: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 825F51DC: 388A49DC  addi r4, r10, 0x49dc
	ctx.r[4].s64 = ctx.r[10].s64 + 18908;
	// 825F51E0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825F51E4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F51E8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 825F51EC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825F51F0: 386ADA94  addi r3, r10, -0x256c
	ctx.r[3].s64 = ctx.r[10].s64 + -9580;
	// 825F51F4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 825F51F8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825F51FC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F5200: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825F5204: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F5208: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825F520C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F5210: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F5214: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825F5218: 4BE71C09  bl 0x82466e20
	ctx.lr = 0x825F521C;
	sub_82466E20(ctx, base);
	// 825F521C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F5220: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F5224: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F5228: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F5230(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F5230 size=116
    let mut pc: u32 = 0x825F5230;
    'dispatch: loop {
        match pc {
            0x825F5230 => {
    //   block [0x825F5230..0x825F52A4)
	// 825F5230: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F5234: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F5238: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F523C: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 825F5240: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F5244: 392B201C  addi r9, r11, 0x201c
	ctx.r[9].s64 = ctx.r[11].s64 + 8220;
	// 825F5248: 38AADF14  addi r5, r10, -0x20ec
	ctx.r[5].s64 = ctx.r[10].s64 + -8428;
	// 825F524C: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F5250: 38E90014  addi r7, r9, 0x14
	ctx.r[7].s64 = ctx.r[9].s64 + 20;
	// 825F5254: 38C00011  li r6, 0x11
	ctx.r[6].s64 = 17;
	// 825F5258: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825F525C: 388A49FC  addi r4, r10, 0x49fc
	ctx.r[4].s64 = ctx.r[10].s64 + 18940;
	// 825F5260: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F5264: 396BBF20  addi r11, r11, -0x40e0
	ctx.r[11].s64 = ctx.r[11].s64 + -16608;
	// 825F5268: 90E10064  stw r7, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[7].u32 ) };
	// 825F526C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F5270: 90C1005C  stw r6, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[6].u32 ) };
	// 825F5274: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F5278: 386ADAC4  addi r3, r10, -0x253c
	ctx.r[3].s64 = ctx.r[10].s64 + -9532;
	// 825F527C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 825F5280: 91010074  stw r8, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[8].u32 ) };
	// 825F5284: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F5288: 9101006C  stw r8, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[8].u32 ) };
	// 825F528C: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 825F5290: 4BE71B91  bl 0x82466e20
	ctx.lr = 0x825F5294;
	sub_82466E20(ctx, base);
	// 825F5294: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F5298: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F529C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F52A0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F52A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F52A8 size=100
    let mut pc: u32 = 0x825F52A8;
    'dispatch: loop {
        match pc {
            0x825F52A8 => {
    //   block [0x825F52A8..0x825F530C)
	// 825F52A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F52AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F52B0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F52B4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F52B8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825F52BC: 38AAAF74  addi r5, r10, -0x508c
	ctx.r[5].s64 = ctx.r[10].s64 + -20620;
	// 825F52C0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F52C4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825F52C8: 388A4A08  addi r4, r10, 0x4a08
	ctx.r[4].s64 = ctx.r[10].s64 + 18952;
	// 825F52CC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F52D0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825F52D4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F52D8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825F52DC: 386ADAF4  addi r3, r10, -0x250c
	ctx.r[3].s64 = ctx.r[10].s64 + -9484;
	// 825F52E0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F52E4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825F52E8: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 825F52EC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F52F0: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 825F52F4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F52F8: 4BE71B29  bl 0x82466e20
	ctx.lr = 0x825F52FC;
	sub_82466E20(ctx, base);
	// 825F52FC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F5300: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F5304: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F5308: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F5310(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F5310 size=100
    let mut pc: u32 = 0x825F5310;
    'dispatch: loop {
        match pc {
            0x825F5310 => {
    //   block [0x825F5310..0x825F5374)
	// 825F5310: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F5314: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F5318: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F531C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F5320: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825F5324: 38AADB84  addi r5, r10, -0x247c
	ctx.r[5].s64 = ctx.r[10].s64 + -9340;
	// 825F5328: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F532C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825F5330: 388A4A1C  addi r4, r10, 0x4a1c
	ctx.r[4].s64 = ctx.r[10].s64 + 18972;
	// 825F5334: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F5338: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825F533C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F5340: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825F5344: 386ADB24  addi r3, r10, -0x24dc
	ctx.r[3].s64 = ctx.r[10].s64 + -9436;
	// 825F5348: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F534C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825F5350: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 825F5354: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F5358: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 825F535C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F5360: 4BE71AC1  bl 0x82466e20
	ctx.lr = 0x825F5364;
	sub_82466E20(ctx, base);
	// 825F5364: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F5368: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F536C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F5370: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F5378(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F5378 size=100
    let mut pc: u32 = 0x825F5378;
    'dispatch: loop {
        match pc {
            0x825F5378 => {
    //   block [0x825F5378..0x825F53DC)
	// 825F5378: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F537C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F5380: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F5384: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F5388: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825F538C: 38AADAC4  addi r5, r10, -0x253c
	ctx.r[5].s64 = ctx.r[10].s64 + -9532;
	// 825F5390: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F5394: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825F5398: 388A4A38  addi r4, r10, 0x4a38
	ctx.r[4].s64 = ctx.r[10].s64 + 19000;
	// 825F539C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F53A0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825F53A4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F53A8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825F53AC: 386ADB54  addi r3, r10, -0x24ac
	ctx.r[3].s64 = ctx.r[10].s64 + -9388;
	// 825F53B0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F53B4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825F53B8: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 825F53BC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F53C0: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 825F53C4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F53C8: 4BE71A59  bl 0x82466e20
	ctx.lr = 0x825F53CC;
	sub_82466E20(ctx, base);
	// 825F53CC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F53D0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F53D4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F53D8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F53E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F53E0 size=104
    let mut pc: u32 = 0x825F53E0;
    'dispatch: loop {
        match pc {
            0x825F53E0 => {
    //   block [0x825F53E0..0x825F5448)
	// 825F53E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F53E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F53E8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F53EC: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F53F0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825F53F4: 392A2098  addi r9, r10, 0x2098
	ctx.r[9].s64 = ctx.r[10].s64 + 8344;
	// 825F53F8: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F53FC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F5400: 38AADAF4  addi r5, r10, -0x250c
	ctx.r[5].s64 = ctx.r[10].s64 + -9484;
	// 825F5404: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F5408: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825F540C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F5410: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825F5414: 388A4A44  addi r4, r10, 0x4a44
	ctx.r[4].s64 = ctx.r[10].s64 + 19012;
	// 825F5418: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F541C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F5420: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 825F5424: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F5428: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 825F542C: 386ADB84  addi r3, r10, -0x247c
	ctx.r[3].s64 = ctx.r[10].s64 + -9340;
	// 825F5430: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 825F5434: 4BE719ED  bl 0x82466e20
	ctx.lr = 0x825F5438;
	sub_82466E20(ctx, base);
	// 825F5438: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F543C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F5440: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F5444: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F5448(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F5448 size=108
    let mut pc: u32 = 0x825F5448;
    'dispatch: loop {
        match pc {
            0x825F5448 => {
    //   block [0x825F5448..0x825F54B4)
	// 825F5448: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F544C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F5450: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F5454: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825F5458: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F545C: 38EBC0BC  addi r7, r11, -0x3f44
	ctx.r[7].s64 = ctx.r[11].s64 + -16196;
	// 825F5460: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 825F5464: 388A4A5C  addi r4, r10, 0x4a5c
	ctx.r[4].s64 = ctx.r[10].s64 + 19036;
	// 825F5468: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825F546C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F5470: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 825F5474: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825F5478: 386ADBB4  addi r3, r10, -0x244c
	ctx.r[3].s64 = ctx.r[10].s64 + -9292;
	// 825F547C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 825F5480: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825F5484: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F5488: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825F548C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F5490: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825F5494: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F5498: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F549C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825F54A0: 4BE71981  bl 0x82466e20
	ctx.lr = 0x825F54A4;
	sub_82466E20(ctx, base);
	// 825F54A4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F54A8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F54AC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F54B0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F54B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F54B8 size=112
    let mut pc: u32 = 0x825F54B8;
    'dispatch: loop {
        match pc {
            0x825F54B8 => {
    //   block [0x825F54B8..0x825F5528)
	// 825F54B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F54BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F54C0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F54C4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F54C8: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825F54CC: 38AADB84  addi r5, r10, -0x247c
	ctx.r[5].s64 = ctx.r[10].s64 + -9340;
	// 825F54D0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F54D4: 390BC0F0  addi r8, r11, -0x3f10
	ctx.r[8].s64 = ctx.r[11].s64 + -16144;
	// 825F54D8: 39200007  li r9, 7
	ctx.r[9].s64 = 7;
	// 825F54DC: 388A4A80  addi r4, r10, 0x4a80
	ctx.r[4].s64 = ctx.r[10].s64 + 19072;
	// 825F54E0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825F54E4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F54E8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825F54EC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F54F0: 386ADBE4  addi r3, r10, -0x241c
	ctx.r[3].s64 = ctx.r[10].s64 + -9244;
	// 825F54F4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 825F54F8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825F54FC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825F5500: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825F5504: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F5508: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825F550C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F5510: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F5514: 4BE7190D  bl 0x82466e20
	ctx.lr = 0x825F5518;
	sub_82466E20(ctx, base);
	// 825F5518: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F551C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F5520: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F5524: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F5528(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825F5528 size=24
    let mut pc: u32 = 0x825F5528;
    'dispatch: loop {
        match pc {
            0x825F5528 => {
    //   block [0x825F5528..0x825F5540)
	// 825F5528: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825F552C: 3D408274  lis r10, -0x7d8c
	ctx.r[10].s64 = -2106327040;
	// 825F5530: 394AF098  addi r10, r10, -0xf68
	ctx.r[10].s64 = ctx.r[10].s64 + -3944;
	// 825F5534: 816BC0EC  lwz r11, -0x3f14(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-16148 as u32) ) } as u64;
	// 825F5538: 916A0008  stw r11, 8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 825F553C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F5540(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F5540 size=116
    let mut pc: u32 = 0x825F5540;
    'dispatch: loop {
        match pc {
            0x825F5540 => {
    //   block [0x825F5540..0x825F55B4)
	// 825F5540: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F5544: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F5548: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F554C: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825F5550: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F5554: 390BF098  addi r8, r11, -0xf68
	ctx.r[8].s64 = ctx.r[11].s64 + -3944;
	// 825F5558: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825F555C: 392A2100  addi r9, r10, 0x2100
	ctx.r[9].s64 = ctx.r[10].s64 + 8448;
	// 825F5560: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F5564: 38E00008  li r7, 8
	ctx.r[7].s64 = 8;
	// 825F5568: 38AAAF74  addi r5, r10, -0x508c
	ctx.r[5].s64 = ctx.r[10].s64 + -20620;
	// 825F556C: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825F5570: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825F5574: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F5578: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825F557C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F5580: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F5584: 3D608293  lis r11, -0x7d6d
	ctx.r[11].s64 = -2104295424;
	// 825F5588: 388A4B0C  addi r4, r10, 0x4b0c
	ctx.r[4].s64 = ctx.r[10].s64 + 19212;
	// 825F558C: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 825F5590: 386BDC14  addi r3, r11, -0x23ec
	ctx.r[3].s64 = ctx.r[11].s64 + -9196;
	// 825F5594: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 825F5598: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F559C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F55A0: 4BE71881  bl 0x82466e20
	ctx.lr = 0x825F55A4;
	sub_82466E20(ctx, base);
	// 825F55A4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F55A8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F55AC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F55B0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F55B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F55B8 size=100
    let mut pc: u32 = 0x825F55B8;
    'dispatch: loop {
        match pc {
            0x825F55B8 => {
    //   block [0x825F55B8..0x825F561C)
	// 825F55B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F55BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F55C0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F55C4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F55C8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825F55CC: 38AADC14  addi r5, r10, -0x23ec
	ctx.r[5].s64 = ctx.r[10].s64 + -9196;
	// 825F55D0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F55D4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825F55D8: 388A4B18  addi r4, r10, 0x4b18
	ctx.r[4].s64 = ctx.r[10].s64 + 19224;
	// 825F55DC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F55E0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825F55E4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F55E8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825F55EC: 386ADC44  addi r3, r10, -0x23bc
	ctx.r[3].s64 = ctx.r[10].s64 + -9148;
	// 825F55F0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F55F4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825F55F8: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 825F55FC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F5600: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 825F5604: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F5608: 4BE71819  bl 0x82466e20
	ctx.lr = 0x825F560C;
	sub_82466E20(ctx, base);
	// 825F560C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F5610: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F5614: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F5618: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F5620(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F5620 size=100
    let mut pc: u32 = 0x825F5620;
    'dispatch: loop {
        match pc {
            0x825F5620 => {
    //   block [0x825F5620..0x825F5684)
	// 825F5620: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F5624: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F5628: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F562C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F5630: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825F5634: 38AADCA4  addi r5, r10, -0x235c
	ctx.r[5].s64 = ctx.r[10].s64 + -9052;
	// 825F5638: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F563C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825F5640: 388A4B24  addi r4, r10, 0x4b24
	ctx.r[4].s64 = ctx.r[10].s64 + 19236;
	// 825F5644: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F5648: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825F564C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F5650: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825F5654: 386ADC74  addi r3, r10, -0x238c
	ctx.r[3].s64 = ctx.r[10].s64 + -9100;
	// 825F5658: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F565C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825F5660: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 825F5664: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F5668: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 825F566C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F5670: 4BE717B1  bl 0x82466e20
	ctx.lr = 0x825F5674;
	sub_82466E20(ctx, base);
	// 825F5674: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F5678: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F567C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F5680: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F5688(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F5688 size=112
    let mut pc: u32 = 0x825F5688;
    'dispatch: loop {
        match pc {
            0x825F5688 => {
    //   block [0x825F5688..0x825F56F8)
	// 825F5688: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F568C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F5690: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F5694: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F5698: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825F569C: 38AADC14  addi r5, r10, -0x23ec
	ctx.r[5].s64 = ctx.r[10].s64 + -9196;
	// 825F56A0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F56A4: 390BC198  addi r8, r11, -0x3e68
	ctx.r[8].s64 = ctx.r[11].s64 + -15976;
	// 825F56A8: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 825F56AC: 388A4B38  addi r4, r10, 0x4b38
	ctx.r[4].s64 = ctx.r[10].s64 + 19256;
	// 825F56B0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825F56B4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F56B8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825F56BC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F56C0: 386ADCA4  addi r3, r10, -0x235c
	ctx.r[3].s64 = ctx.r[10].s64 + -9052;
	// 825F56C4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 825F56C8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825F56CC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825F56D0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825F56D4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F56D8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825F56DC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F56E0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F56E4: 4BE7173D  bl 0x82466e20
	ctx.lr = 0x825F56E8;
	sub_82466E20(ctx, base);
	// 825F56E8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F56EC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F56F0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F56F4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F56F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F56F8 size=100
    let mut pc: u32 = 0x825F56F8;
    'dispatch: loop {
        match pc {
            0x825F56F8 => {
    //   block [0x825F56F8..0x825F575C)
	// 825F56F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F56FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F5700: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F5704: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F5708: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825F570C: 38AADCA4  addi r5, r10, -0x235c
	ctx.r[5].s64 = ctx.r[10].s64 + -9052;
	// 825F5710: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F5714: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825F5718: 388A4B50  addi r4, r10, 0x4b50
	ctx.r[4].s64 = ctx.r[10].s64 + 19280;
	// 825F571C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F5720: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825F5724: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F5728: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825F572C: 386ADCD4  addi r3, r10, -0x232c
	ctx.r[3].s64 = ctx.r[10].s64 + -9004;
	// 825F5730: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F5734: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825F5738: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 825F573C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F5740: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 825F5744: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F5748: 4BE716D9  bl 0x82466e20
	ctx.lr = 0x825F574C;
	sub_82466E20(ctx, base);
	// 825F574C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F5750: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F5754: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F5758: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F5760(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F5760 size=100
    let mut pc: u32 = 0x825F5760;
    'dispatch: loop {
        match pc {
            0x825F5760 => {
    //   block [0x825F5760..0x825F57C4)
	// 825F5760: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F5764: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F5768: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F576C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F5770: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825F5774: 38AADC14  addi r5, r10, -0x23ec
	ctx.r[5].s64 = ctx.r[10].s64 + -9196;
	// 825F5778: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F577C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825F5780: 388A4B60  addi r4, r10, 0x4b60
	ctx.r[4].s64 = ctx.r[10].s64 + 19296;
	// 825F5784: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F5788: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825F578C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F5790: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825F5794: 386ADD04  addi r3, r10, -0x22fc
	ctx.r[3].s64 = ctx.r[10].s64 + -8956;
	// 825F5798: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F579C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825F57A0: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 825F57A4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F57A8: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 825F57AC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F57B0: 4BE71671  bl 0x82466e20
	ctx.lr = 0x825F57B4;
	sub_82466E20(ctx, base);
	// 825F57B4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F57B8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F57BC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F57C0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F57C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F57C8 size=100
    let mut pc: u32 = 0x825F57C8;
    'dispatch: loop {
        match pc {
            0x825F57C8 => {
    //   block [0x825F57C8..0x825F582C)
	// 825F57C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F57CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F57D0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F57D4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F57D8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825F57DC: 38AADC44  addi r5, r10, -0x23bc
	ctx.r[5].s64 = ctx.r[10].s64 + -9148;
	// 825F57E0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F57E4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825F57E8: 388A4B70  addi r4, r10, 0x4b70
	ctx.r[4].s64 = ctx.r[10].s64 + 19312;
	// 825F57EC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F57F0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825F57F4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F57F8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825F57FC: 386ADD34  addi r3, r10, -0x22cc
	ctx.r[3].s64 = ctx.r[10].s64 + -8908;
	// 825F5800: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F5804: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825F5808: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 825F580C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F5810: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 825F5814: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F5818: 4BE71609  bl 0x82466e20
	ctx.lr = 0x825F581C;
	sub_82466E20(ctx, base);
	// 825F581C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F5820: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F5824: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F5828: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F5830(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F5830 size=100
    let mut pc: u32 = 0x825F5830;
    'dispatch: loop {
        match pc {
            0x825F5830 => {
    //   block [0x825F5830..0x825F5894)
	// 825F5830: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F5834: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F5838: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F583C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F5840: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825F5844: 38AADD04  addi r5, r10, -0x22fc
	ctx.r[5].s64 = ctx.r[10].s64 + -8956;
	// 825F5848: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F584C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825F5850: 388A4B88  addi r4, r10, 0x4b88
	ctx.r[4].s64 = ctx.r[10].s64 + 19336;
	// 825F5854: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F5858: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825F585C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F5860: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825F5864: 386ADD64  addi r3, r10, -0x229c
	ctx.r[3].s64 = ctx.r[10].s64 + -8860;
	// 825F5868: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F586C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825F5870: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 825F5874: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F5878: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 825F587C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F5880: 4BE715A1  bl 0x82466e20
	ctx.lr = 0x825F5884;
	sub_82466E20(ctx, base);
	// 825F5884: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F5888: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F588C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F5890: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F5898(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F5898 size=100
    let mut pc: u32 = 0x825F5898;
    'dispatch: loop {
        match pc {
            0x825F5898 => {
    //   block [0x825F5898..0x825F58FC)
	// 825F5898: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F589C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F58A0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F58A4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F58A8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825F58AC: 38AADC44  addi r5, r10, -0x23bc
	ctx.r[5].s64 = ctx.r[10].s64 + -9148;
	// 825F58B0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F58B4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825F58B8: 388A4BA4  addi r4, r10, 0x4ba4
	ctx.r[4].s64 = ctx.r[10].s64 + 19364;
	// 825F58BC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F58C0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825F58C4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F58C8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825F58CC: 386ADD94  addi r3, r10, -0x226c
	ctx.r[3].s64 = ctx.r[10].s64 + -8812;
	// 825F58D0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F58D4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825F58D8: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 825F58DC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F58E0: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 825F58E4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F58E8: 4BE71539  bl 0x82466e20
	ctx.lr = 0x825F58EC;
	sub_82466E20(ctx, base);
	// 825F58EC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F58F0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F58F4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F58F8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F5900(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F5900 size=112
    let mut pc: u32 = 0x825F5900;
    'dispatch: loop {
        match pc {
            0x825F5900 => {
    //   block [0x825F5900..0x825F5970)
	// 825F5900: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F5904: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F5908: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F590C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F5910: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825F5914: 38AADE24  addi r5, r10, -0x21dc
	ctx.r[5].s64 = ctx.r[10].s64 + -8668;
	// 825F5918: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F591C: 390BC1C8  addi r8, r11, -0x3e38
	ctx.r[8].s64 = ctx.r[11].s64 + -15928;
	// 825F5920: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 825F5924: 388A4BB4  addi r4, r10, 0x4bb4
	ctx.r[4].s64 = ctx.r[10].s64 + 19380;
	// 825F5928: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825F592C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F5930: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825F5934: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F5938: 386ADDC4  addi r3, r10, -0x223c
	ctx.r[3].s64 = ctx.r[10].s64 + -8764;
	// 825F593C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 825F5940: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825F5944: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825F5948: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825F594C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F5950: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825F5954: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F5958: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F595C: 4BE714C5  bl 0x82466e20
	ctx.lr = 0x825F5960;
	sub_82466E20(ctx, base);
	// 825F5960: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F5964: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F5968: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F596C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F5970(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F5970 size=112
    let mut pc: u32 = 0x825F5970;
    'dispatch: loop {
        match pc {
            0x825F5970 => {
    //   block [0x825F5970..0x825F59E0)
	// 825F5970: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F5974: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F5978: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F597C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F5980: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825F5984: 38AADE54  addi r5, r10, -0x21ac
	ctx.r[5].s64 = ctx.r[10].s64 + -8620;
	// 825F5988: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F598C: 390BC1F8  addi r8, r11, -0x3e08
	ctx.r[8].s64 = ctx.r[11].s64 + -15880;
	// 825F5990: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 825F5994: 388A4BC4  addi r4, r10, 0x4bc4
	ctx.r[4].s64 = ctx.r[10].s64 + 19396;
	// 825F5998: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825F599C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F59A0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825F59A4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F59A8: 386ADDF4  addi r3, r10, -0x220c
	ctx.r[3].s64 = ctx.r[10].s64 + -8716;
	// 825F59AC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 825F59B0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825F59B4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825F59B8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825F59BC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F59C0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825F59C4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F59C8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F59CC: 4BE71455  bl 0x82466e20
	ctx.lr = 0x825F59D0;
	sub_82466E20(ctx, base);
	// 825F59D0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F59D4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F59D8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F59DC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F59E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F59E0 size=112
    let mut pc: u32 = 0x825F59E0;
    'dispatch: loop {
        match pc {
            0x825F59E0 => {
    //   block [0x825F59E0..0x825F5A50)
	// 825F59E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F59E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F59E8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F59EC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F59F0: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825F59F4: 38AADF14  addi r5, r10, -0x20ec
	ctx.r[5].s64 = ctx.r[10].s64 + -8428;
	// 825F59F8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F59FC: 390BC210  addi r8, r11, -0x3df0
	ctx.r[8].s64 = ctx.r[11].s64 + -15856;
	// 825F5A00: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 825F5A04: 388A4BDC  addi r4, r10, 0x4bdc
	ctx.r[4].s64 = ctx.r[10].s64 + 19420;
	// 825F5A08: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825F5A0C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F5A10: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825F5A14: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F5A18: 386ADE24  addi r3, r10, -0x21dc
	ctx.r[3].s64 = ctx.r[10].s64 + -8668;
	// 825F5A1C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 825F5A20: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825F5A24: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825F5A28: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825F5A2C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F5A30: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825F5A34: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F5A38: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F5A3C: 4BE713E5  bl 0x82466e20
	ctx.lr = 0x825F5A40;
	sub_82466E20(ctx, base);
	// 825F5A40: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F5A44: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F5A48: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F5A4C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F5A50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F5A50 size=112
    let mut pc: u32 = 0x825F5A50;
    'dispatch: loop {
        match pc {
            0x825F5A50 => {
    //   block [0x825F5A50..0x825F5AC0)
	// 825F5A50: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F5A54: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F5A58: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F5A5C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F5A60: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825F5A64: 38AADE24  addi r5, r10, -0x21dc
	ctx.r[5].s64 = ctx.r[10].s64 + -8668;
	// 825F5A68: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F5A6C: 390BC240  addi r8, r11, -0x3dc0
	ctx.r[8].s64 = ctx.r[11].s64 + -15808;
	// 825F5A70: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 825F5A74: 388A4BE8  addi r4, r10, 0x4be8
	ctx.r[4].s64 = ctx.r[10].s64 + 19432;
	// 825F5A78: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825F5A7C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F5A80: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825F5A84: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F5A88: 386ADE54  addi r3, r10, -0x21ac
	ctx.r[3].s64 = ctx.r[10].s64 + -8620;
	// 825F5A8C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 825F5A90: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825F5A94: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825F5A98: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825F5A9C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F5AA0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825F5AA4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F5AA8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F5AAC: 4BE71375  bl 0x82466e20
	ctx.lr = 0x825F5AB0;
	sub_82466E20(ctx, base);
	// 825F5AB0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F5AB4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F5AB8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F5ABC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F5AC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F5AC0 size=112
    let mut pc: u32 = 0x825F5AC0;
    'dispatch: loop {
        match pc {
            0x825F5AC0 => {
    //   block [0x825F5AC0..0x825F5B30)
	// 825F5AC0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F5AC4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F5AC8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F5ACC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F5AD0: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825F5AD4: 38AADE54  addi r5, r10, -0x21ac
	ctx.r[5].s64 = ctx.r[10].s64 + -8620;
	// 825F5AD8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F5ADC: 390BC258  addi r8, r11, -0x3da8
	ctx.r[8].s64 = ctx.r[11].s64 + -15784;
	// 825F5AE0: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 825F5AE4: 388A4BF8  addi r4, r10, 0x4bf8
	ctx.r[4].s64 = ctx.r[10].s64 + 19448;
	// 825F5AE8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825F5AEC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F5AF0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825F5AF4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F5AF8: 386ADE84  addi r3, r10, -0x217c
	ctx.r[3].s64 = ctx.r[10].s64 + -8572;
	// 825F5AFC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 825F5B00: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825F5B04: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825F5B08: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825F5B0C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F5B10: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825F5B14: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F5B18: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F5B1C: 4BE71305  bl 0x82466e20
	ctx.lr = 0x825F5B20;
	sub_82466E20(ctx, base);
	// 825F5B20: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F5B24: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F5B28: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F5B2C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F5B30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F5B30 size=116
    let mut pc: u32 = 0x825F5B30;
    'dispatch: loop {
        match pc {
            0x825F5B30 => {
    //   block [0x825F5B30..0x825F5BA4)
	// 825F5B30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F5B34: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F5B38: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F5B3C: 3D408274  lis r10, -0x7d8c
	ctx.r[10].s64 = -2106327040;
	// 825F5B40: 38E00007  li r7, 7
	ctx.r[7].s64 = 7;
	// 825F5B44: 390AC270  addi r8, r10, -0x3d90
	ctx.r[8].s64 = ctx.r[10].s64 + -15760;
	// 825F5B48: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F5B4C: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 825F5B50: 38AAAF74  addi r5, r10, -0x508c
	ctx.r[5].s64 = ctx.r[10].s64 + -20620;
	// 825F5B54: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F5B58: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 825F5B5C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825F5B60: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825F5B64: 388A4C10  addi r4, r10, 0x4c10
	ctx.r[4].s64 = ctx.r[10].s64 + 19472;
	// 825F5B68: 396B2114  addi r11, r11, 0x2114
	ctx.r[11].s64 = ctx.r[11].s64 + 8468;
	// 825F5B6C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F5B70: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F5B74: 386ADEB4  addi r3, r10, -0x214c
	ctx.r[3].s64 = ctx.r[10].s64 + -8524;
	// 825F5B78: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 825F5B7C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825F5B80: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 825F5B84: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F5B88: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F5B8C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F5B90: 4BE71291  bl 0x82466e20
	ctx.lr = 0x825F5B94;
	sub_82466E20(ctx, base);
	// 825F5B94: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F5B98: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F5B9C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F5BA0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F5BA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825F5BA8 size=48
    let mut pc: u32 = 0x825F5BA8;
    'dispatch: loop {
        match pc {
            0x825F5BA8 => {
    //   block [0x825F5BA8..0x825F5BD8)
	// 825F5BA8: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825F5BAC: 814BC324  lwz r10, -0x3cdc(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-15580 as u32) ) } as u64;
	// 825F5BB0: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825F5BB4: 396BF158  addi r11, r11, -0xea8
	ctx.r[11].s64 = ctx.r[11].s64 + -3752;
	// 825F5BB8: 914B0050  stw r10, 0x50(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 825F5BBC: 3D408274  lis r10, -0x7d8c
	ctx.r[10].s64 = -2106327040;
	// 825F5BC0: 814AC320  lwz r10, -0x3ce0(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-15584 as u32) ) } as u64;
	// 825F5BC4: 914B0140  stw r10, 0x140(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(320 as u32), ctx.r[10].u32 ) };
	// 825F5BC8: 3D408274  lis r10, -0x7d8c
	ctx.r[10].s64 = -2106327040;
	// 825F5BCC: 814AC31C  lwz r10, -0x3ce4(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-15588 as u32) ) } as u64;
	// 825F5BD0: 914B0350  stw r10, 0x350(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(848 as u32), ctx.r[10].u32 ) };
	// 825F5BD4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F5BD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F5BD8 size=116
    let mut pc: u32 = 0x825F5BD8;
    'dispatch: loop {
        match pc {
            0x825F5BD8 => {
    //   block [0x825F5BD8..0x825F5C4C)
	// 825F5BD8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F5BDC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F5BE0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F5BE4: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 825F5BE8: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F5BEC: 392B21E8  addi r9, r11, 0x21e8
	ctx.r[9].s64 = ctx.r[11].s64 + 8680;
	// 825F5BF0: 38AAAF74  addi r5, r10, -0x508c
	ctx.r[5].s64 = ctx.r[10].s64 + -20620;
	// 825F5BF4: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F5BF8: 38E90050  addi r7, r9, 0x50
	ctx.r[7].s64 = ctx.r[9].s64 + 80;
	// 825F5BFC: 38C00028  li r6, 0x28
	ctx.r[6].s64 = 40;
	// 825F5C00: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825F5C04: 388A4FC8  addi r4, r10, 0x4fc8
	ctx.r[4].s64 = ctx.r[10].s64 + 20424;
	// 825F5C08: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F5C0C: 396BF158  addi r11, r11, -0xea8
	ctx.r[11].s64 = ctx.r[11].s64 + -3752;
	// 825F5C10: 90E10064  stw r7, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[7].u32 ) };
	// 825F5C14: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F5C18: 90C1005C  stw r6, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[6].u32 ) };
	// 825F5C1C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F5C20: 386ADEE4  addi r3, r10, -0x211c
	ctx.r[3].s64 = ctx.r[10].s64 + -8476;
	// 825F5C24: 39400004  li r10, 4
	ctx.r[10].s64 = 4;
	// 825F5C28: 91010074  stw r8, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[8].u32 ) };
	// 825F5C2C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F5C30: 9101006C  stw r8, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[8].u32 ) };
	// 825F5C34: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 825F5C38: 4BE711E9  bl 0x82466e20
	ctx.lr = 0x825F5C3C;
	sub_82466E20(ctx, base);
	// 825F5C3C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F5C40: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F5C44: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F5C48: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F5C50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F5C50 size=116
    let mut pc: u32 = 0x825F5C50;
    'dispatch: loop {
        match pc {
            0x825F5C50 => {
    //   block [0x825F5C50..0x825F5CC4)
	// 825F5C50: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F5C54: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F5C58: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F5C5C: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825F5C60: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F5C64: 390BC330  addi r8, r11, -0x3cd0
	ctx.r[8].s64 = ctx.r[11].s64 + -15568;
	// 825F5C68: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825F5C6C: 392A2364  addi r9, r10, 0x2364
	ctx.r[9].s64 = ctx.r[10].s64 + 9060;
	// 825F5C70: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F5C74: 38E00006  li r7, 6
	ctx.r[7].s64 = 6;
	// 825F5C78: 38AAAF74  addi r5, r10, -0x508c
	ctx.r[5].s64 = ctx.r[10].s64 + -20620;
	// 825F5C7C: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825F5C80: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825F5C84: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F5C88: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825F5C8C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F5C90: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F5C94: 3D608293  lis r11, -0x7d6d
	ctx.r[11].s64 = -2104295424;
	// 825F5C98: 388A4FD8  addi r4, r10, 0x4fd8
	ctx.r[4].s64 = ctx.r[10].s64 + 20440;
	// 825F5C9C: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 825F5CA0: 386BDF14  addi r3, r11, -0x20ec
	ctx.r[3].s64 = ctx.r[11].s64 + -8428;
	// 825F5CA4: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 825F5CA8: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F5CAC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F5CB0: 4BE71171  bl 0x82466e20
	ctx.lr = 0x825F5CB4;
	sub_82466E20(ctx, base);
	// 825F5CB4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F5CB8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F5CBC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F5CC0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F5CC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F5CC8 size=112
    let mut pc: u32 = 0x825F5CC8;
    'dispatch: loop {
        match pc {
            0x825F5CC8 => {
    //   block [0x825F5CC8..0x825F5D38)
	// 825F5CC8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F5CCC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F5CD0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F5CD4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F5CD8: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825F5CDC: 38AAAF74  addi r5, r10, -0x508c
	ctx.r[5].s64 = ctx.r[10].s64 + -20620;
	// 825F5CE0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F5CE4: 390BC3C0  addi r8, r11, -0x3c40
	ctx.r[8].s64 = ctx.r[11].s64 + -15424;
	// 825F5CE8: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 825F5CEC: 388A4FE8  addi r4, r10, 0x4fe8
	ctx.r[4].s64 = ctx.r[10].s64 + 20456;
	// 825F5CF0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825F5CF4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F5CF8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825F5CFC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F5D00: 386ADF44  addi r3, r10, -0x20bc
	ctx.r[3].s64 = ctx.r[10].s64 + -8380;
	// 825F5D04: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 825F5D08: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825F5D0C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825F5D10: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825F5D14: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F5D18: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825F5D1C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F5D20: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F5D24: 4BE710FD  bl 0x82466e20
	ctx.lr = 0x825F5D28;
	sub_82466E20(ctx, base);
	// 825F5D28: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F5D2C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F5D30: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F5D34: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F5D38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F5D38 size=112
    let mut pc: u32 = 0x825F5D38;
    'dispatch: loop {
        match pc {
            0x825F5D38 => {
    //   block [0x825F5D38..0x825F5DA8)
	// 825F5D38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F5D3C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F5D40: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F5D44: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F5D48: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825F5D4C: 38AABF64  addi r5, r10, -0x409c
	ctx.r[5].s64 = ctx.r[10].s64 + -16540;
	// 825F5D50: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F5D54: 390BC3D8  addi r8, r11, -0x3c28
	ctx.r[8].s64 = ctx.r[11].s64 + -15400;
	// 825F5D58: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 825F5D5C: 388A5000  addi r4, r10, 0x5000
	ctx.r[4].s64 = ctx.r[10].s64 + 20480;
	// 825F5D60: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825F5D64: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F5D68: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825F5D6C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F5D70: 386ADF74  addi r3, r10, -0x208c
	ctx.r[3].s64 = ctx.r[10].s64 + -8332;
	// 825F5D74: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 825F5D78: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825F5D7C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825F5D80: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825F5D84: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F5D88: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825F5D8C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F5D90: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F5D94: 4BE7108D  bl 0x82466e20
	ctx.lr = 0x825F5D98;
	sub_82466E20(ctx, base);
	// 825F5D98: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F5D9C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F5DA0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F5DA4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F5DA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F5DA8 size=108
    let mut pc: u32 = 0x825F5DA8;
    'dispatch: loop {
        match pc {
            0x825F5DA8 => {
    //   block [0x825F5DA8..0x825F5E14)
	// 825F5DA8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F5DAC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F5DB0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F5DB4: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825F5DB8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F5DBC: 38EBC3F0  addi r7, r11, -0x3c10
	ctx.r[7].s64 = ctx.r[11].s64 + -15376;
	// 825F5DC0: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 825F5DC4: 388A5014  addi r4, r10, 0x5014
	ctx.r[4].s64 = ctx.r[10].s64 + 20500;
	// 825F5DC8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825F5DCC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F5DD0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 825F5DD4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825F5DD8: 386ADFA4  addi r3, r10, -0x205c
	ctx.r[3].s64 = ctx.r[10].s64 + -8284;
	// 825F5DDC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 825F5DE0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825F5DE4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F5DE8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825F5DEC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F5DF0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825F5DF4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F5DF8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F5DFC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825F5E00: 4BE71021  bl 0x82466e20
	ctx.lr = 0x825F5E04;
	sub_82466E20(ctx, base);
	// 825F5E04: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F5E08: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F5E0C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F5E10: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F5E18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F5E18 size=112
    let mut pc: u32 = 0x825F5E18;
    'dispatch: loop {
        match pc {
            0x825F5E18 => {
    //   block [0x825F5E18..0x825F5E88)
	// 825F5E18: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F5E1C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F5E20: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F5E24: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F5E28: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825F5E2C: 38AAAF74  addi r5, r10, -0x508c
	ctx.r[5].s64 = ctx.r[10].s64 + -20620;
	// 825F5E30: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F5E34: 390BC408  addi r8, r11, -0x3bf8
	ctx.r[8].s64 = ctx.r[11].s64 + -15352;
	// 825F5E38: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 825F5E3C: 388A5028  addi r4, r10, 0x5028
	ctx.r[4].s64 = ctx.r[10].s64 + 20520;
	// 825F5E40: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825F5E44: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F5E48: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825F5E4C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F5E50: 386ADFD4  addi r3, r10, -0x202c
	ctx.r[3].s64 = ctx.r[10].s64 + -8236;
	// 825F5E54: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 825F5E58: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825F5E5C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825F5E60: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825F5E64: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F5E68: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825F5E6C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F5E70: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F5E74: 4BE70FAD  bl 0x82466e20
	ctx.lr = 0x825F5E78;
	sub_82466E20(ctx, base);
	// 825F5E78: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F5E7C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F5E80: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F5E84: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F5E88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F5E88 size=108
    let mut pc: u32 = 0x825F5E88;
    'dispatch: loop {
        match pc {
            0x825F5E88 => {
    //   block [0x825F5E88..0x825F5EF4)
	// 825F5E88: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F5E8C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F5E90: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F5E94: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825F5E98: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F5E9C: 38EBC450  addi r7, r11, -0x3bb0
	ctx.r[7].s64 = ctx.r[11].s64 + -15280;
	// 825F5EA0: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 825F5EA4: 388A5040  addi r4, r10, 0x5040
	ctx.r[4].s64 = ctx.r[10].s64 + 20544;
	// 825F5EA8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825F5EAC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F5EB0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 825F5EB4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825F5EB8: 386AE004  addi r3, r10, -0x1ffc
	ctx.r[3].s64 = ctx.r[10].s64 + -8188;
	// 825F5EBC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 825F5EC0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825F5EC4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F5EC8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825F5ECC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F5ED0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825F5ED4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F5ED8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F5EDC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825F5EE0: 4BE70F41  bl 0x82466e20
	ctx.lr = 0x825F5EE4;
	sub_82466E20(ctx, base);
	// 825F5EE4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F5EE8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F5EEC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F5EF0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F5EF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F5EF8 size=108
    let mut pc: u32 = 0x825F5EF8;
    'dispatch: loop {
        match pc {
            0x825F5EF8 => {
    //   block [0x825F5EF8..0x825F5F64)
	// 825F5EF8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F5EFC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F5F00: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F5F04: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825F5F08: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F5F0C: 38EBC480  addi r7, r11, -0x3b80
	ctx.r[7].s64 = ctx.r[11].s64 + -15232;
	// 825F5F10: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 825F5F14: 388A505C  addi r4, r10, 0x505c
	ctx.r[4].s64 = ctx.r[10].s64 + 20572;
	// 825F5F18: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825F5F1C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F5F20: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 825F5F24: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825F5F28: 386AE034  addi r3, r10, -0x1fcc
	ctx.r[3].s64 = ctx.r[10].s64 + -8140;
	// 825F5F2C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 825F5F30: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825F5F34: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F5F38: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825F5F3C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F5F40: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825F5F44: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F5F48: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F5F4C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825F5F50: 4BE70ED1  bl 0x82466e20
	ctx.lr = 0x825F5F54;
	sub_82466E20(ctx, base);
	// 825F5F54: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F5F58: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F5F5C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F5F60: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F5F68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F5F68 size=112
    let mut pc: u32 = 0x825F5F68;
    'dispatch: loop {
        match pc {
            0x825F5F68 => {
    //   block [0x825F5F68..0x825F5FD8)
	// 825F5F68: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F5F6C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F5F70: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F5F74: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F5F78: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825F5F7C: 38AAAF74  addi r5, r10, -0x508c
	ctx.r[5].s64 = ctx.r[10].s64 + -20620;
	// 825F5F80: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F5F84: 390BC498  addi r8, r11, -0x3b68
	ctx.r[8].s64 = ctx.r[11].s64 + -15208;
	// 825F5F88: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 825F5F8C: 388A5070  addi r4, r10, 0x5070
	ctx.r[4].s64 = ctx.r[10].s64 + 20592;
	// 825F5F90: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825F5F94: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F5F98: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825F5F9C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F5FA0: 386AE064  addi r3, r10, -0x1f9c
	ctx.r[3].s64 = ctx.r[10].s64 + -8092;
	// 825F5FA4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 825F5FA8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825F5FAC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825F5FB0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825F5FB4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F5FB8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825F5FBC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F5FC0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F5FC4: 4BE70E5D  bl 0x82466e20
	ctx.lr = 0x825F5FC8;
	sub_82466E20(ctx, base);
	// 825F5FC8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F5FCC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F5FD0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F5FD4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F5FD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F5FD8 size=96
    let mut pc: u32 = 0x825F5FD8;
    'dispatch: loop {
        match pc {
            0x825F5FD8 => {
    //   block [0x825F5FD8..0x825F6038)
	// 825F5FD8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F5FDC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F5FE0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F5FE4: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F5FE8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825F5FEC: 388A507C  addi r4, r10, 0x507c
	ctx.r[4].s64 = ctx.r[10].s64 + 20604;
	// 825F5FF0: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F5FF4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825F5FF8: 386AE094  addi r3, r10, -0x1f6c
	ctx.r[3].s64 = ctx.r[10].s64 + -8044;
	// 825F5FFC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825F6000: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825F6004: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 825F6008: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825F600C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F6010: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F6014: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F6018: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 825F601C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825F6020: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 825F6024: 4BE70DFD  bl 0x82466e20
	ctx.lr = 0x825F6028;
	sub_82466E20(ctx, base);
	// 825F6028: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F602C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F6030: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F6034: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F6038(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F6038 size=112
    let mut pc: u32 = 0x825F6038;
    'dispatch: loop {
        match pc {
            0x825F6038 => {
    //   block [0x825F6038..0x825F60A8)
	// 825F6038: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F603C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F6040: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F6044: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F6048: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825F604C: 392A23BC  addi r9, r10, 0x23bc
	ctx.r[9].s64 = ctx.r[10].s64 + 9148;
	// 825F6050: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F6054: 390BC4D0  addi r8, r11, -0x3b30
	ctx.r[8].s64 = ctx.r[11].s64 + -15152;
	// 825F6058: 38E00007  li r7, 7
	ctx.r[7].s64 = 7;
	// 825F605C: 388A508C  addi r4, r10, 0x508c
	ctx.r[4].s64 = ctx.r[10].s64 + 20620;
	// 825F6060: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825F6064: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F6068: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825F606C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F6070: 386AE0C4  addi r3, r10, -0x1f3c
	ctx.r[3].s64 = ctx.r[10].s64 + -7996;
	// 825F6074: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 825F6078: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 825F607C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F6080: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825F6084: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F6088: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825F608C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825F6090: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F6094: 4BE70D8D  bl 0x82466e20
	ctx.lr = 0x825F6098;
	sub_82466E20(ctx, base);
	// 825F6098: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F609C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F60A0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F60A4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F60A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F60A8 size=116
    let mut pc: u32 = 0x825F60A8;
    'dispatch: loop {
        match pc {
            0x825F60A8 => {
    //   block [0x825F60A8..0x825F611C)
	// 825F60A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F60AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F60B0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F60B4: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825F60B8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F60BC: 390BC578  addi r8, r11, -0x3a88
	ctx.r[8].s64 = ctx.r[11].s64 + -14984;
	// 825F60C0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825F60C4: 392A2390  addi r9, r10, 0x2390
	ctx.r[9].s64 = ctx.r[10].s64 + 9104;
	// 825F60C8: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F60CC: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
	// 825F60D0: 38AAD254  addi r5, r10, -0x2dac
	ctx.r[5].s64 = ctx.r[10].s64 + -11692;
	// 825F60D4: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825F60D8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825F60DC: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F60E0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825F60E4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F60E8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F60EC: 3D608293  lis r11, -0x7d6d
	ctx.r[11].s64 = -2104295424;
	// 825F60F0: 388A50AC  addi r4, r10, 0x50ac
	ctx.r[4].s64 = ctx.r[10].s64 + 20652;
	// 825F60F4: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 825F60F8: 386BE0F4  addi r3, r11, -0x1f0c
	ctx.r[3].s64 = ctx.r[11].s64 + -7948;
	// 825F60FC: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 825F6100: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F6104: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F6108: 4BE70D19  bl 0x82466e20
	ctx.lr = 0x825F610C;
	sub_82466E20(ctx, base);
	// 825F610C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F6110: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F6114: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F6118: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825F6120(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825F6120 size=112
    let mut pc: u32 = 0x825F6120;
    'dispatch: loop {
        match pc {
            0x825F6120 => {
    //   block [0x825F6120..0x825F6190)
	// 825F6120: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825F6124: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825F6128: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825F612C: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F6130: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825F6134: 392A23E8  addi r9, r10, 0x23e8
	ctx.r[9].s64 = ctx.r[10].s64 + 9192;
	// 825F6138: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825F613C: 390BC590  addi r8, r11, -0x3a70
	ctx.r[8].s64 = ctx.r[11].s64 + -14960;
	// 825F6140: 38E00004  li r7, 4
	ctx.r[7].s64 = 4;
	// 825F6144: 388A50C4  addi r4, r10, 0x50c4
	ctx.r[4].s64 = ctx.r[10].s64 + 20676;
	// 825F6148: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825F614C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825F6150: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825F6154: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825F6158: 386AE124  addi r3, r10, -0x1edc
	ctx.r[3].s64 = ctx.r[10].s64 + -7900;
	// 825F615C: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 825F6160: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 825F6164: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825F6168: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825F616C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825F6170: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825F6174: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825F6178: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825F617C: 4BE70CA5  bl 0x82466e20
	ctx.lr = 0x825F6180;
	sub_82466E20(ctx, base);
	// 825F6180: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825F6184: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825F6188: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825F618C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


