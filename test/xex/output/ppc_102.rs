pub fn sub_82699AE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82699AE8 size=108
    let mut pc: u32 = 0x82699AE8;
    'dispatch: loop {
        match pc {
            0x82699AE8 => {
    //   block [0x82699AE8..0x82699B54)
	// 82699AE8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82699AEC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82699AF0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82699AF4: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 82699AF8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82699AFC: 38EBF668  addi r7, r11, -0x998
	ctx.r[7].s64 = ctx.r[11].s64 + -2456;
	// 82699B00: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 82699B04: 388AAF00  addi r4, r10, -0x5100
	ctx.r[4].s64 = ctx.r[10].s64 + -20736;
	// 82699B08: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82699B0C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82699B10: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82699B14: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82699B18: 386A3170  addi r3, r10, 0x3170
	ctx.r[3].s64 = ctx.r[10].s64 + 12656;
	// 82699B1C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82699B20: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82699B24: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82699B28: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82699B2C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82699B30: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82699B34: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82699B38: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82699B3C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82699B40: 4BDCD2E1  bl 0x82466e20
	ctx.lr = 0x82699B44;
	sub_82466E20(ctx, base);
	// 82699B44: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82699B48: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82699B4C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82699B50: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82699B58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82699B58 size=108
    let mut pc: u32 = 0x82699B58;
    'dispatch: loop {
        match pc {
            0x82699B58 => {
    //   block [0x82699B58..0x82699BC4)
	// 82699B58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82699B5C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82699B60: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82699B64: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 82699B68: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82699B6C: 38EBF680  addi r7, r11, -0x980
	ctx.r[7].s64 = ctx.r[11].s64 + -2432;
	// 82699B70: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 82699B74: 388A9D38  addi r4, r10, -0x62c8
	ctx.r[4].s64 = ctx.r[10].s64 + -25288;
	// 82699B78: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82699B7C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82699B80: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82699B84: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82699B88: 386A31A0  addi r3, r10, 0x31a0
	ctx.r[3].s64 = ctx.r[10].s64 + 12704;
	// 82699B8C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82699B90: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82699B94: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82699B98: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82699B9C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82699BA0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82699BA4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82699BA8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82699BAC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82699BB0: 4BDCD271  bl 0x82466e20
	ctx.lr = 0x82699BB4;
	sub_82466E20(ctx, base);
	// 82699BB4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82699BB8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82699BBC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82699BC0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82699BC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82699BC8 size=24
    let mut pc: u32 = 0x82699BC8;
    'dispatch: loop {
        match pc {
            0x82699BC8 => {
    //   block [0x82699BC8..0x82699BE0)
	// 82699BC8: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 82699BCC: 3D40827C  lis r10, -0x7d84
	ctx.r[10].s64 = -2105802752;
	// 82699BD0: 394A6AE0  addi r10, r10, 0x6ae0
	ctx.r[10].s64 = ctx.r[10].s64 + 27360;
	// 82699BD4: 816BF6B0  lwz r11, -0x950(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-2384 as u32) ) } as u64;
	// 82699BD8: 916A0008  stw r11, 8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82699BDC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82699BE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82699BE0 size=112
    let mut pc: u32 = 0x82699BE0;
    'dispatch: loop {
        match pc {
            0x82699BE0 => {
    //   block [0x82699BE0..0x82699C50)
	// 82699BE0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82699BE4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82699BE8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82699BEC: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 82699BF0: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 82699BF4: 392A98E0  addi r9, r10, -0x6720
	ctx.r[9].s64 = ctx.r[10].s64 + -26400;
	// 82699BF8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82699BFC: 390B6AE0  addi r8, r11, 0x6ae0
	ctx.r[8].s64 = ctx.r[11].s64 + 27360;
	// 82699C00: 38E00005  li r7, 5
	ctx.r[7].s64 = 5;
	// 82699C04: 388AB450  addi r4, r10, -0x4bb0
	ctx.r[4].s64 = ctx.r[10].s64 + -19376;
	// 82699C08: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82699C0C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82699C10: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82699C14: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82699C18: 386A31D0  addi r3, r10, 0x31d0
	ctx.r[3].s64 = ctx.r[10].s64 + 12752;
	// 82699C1C: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82699C20: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82699C24: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82699C28: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82699C2C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82699C30: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82699C34: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82699C38: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82699C3C: 4BDCD1E5  bl 0x82466e20
	ctx.lr = 0x82699C40;
	sub_82466E20(ctx, base);
	// 82699C40: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82699C44: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82699C48: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82699C4C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82699C50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82699C50 size=112
    let mut pc: u32 = 0x82699C50;
    'dispatch: loop {
        match pc {
            0x82699C50 => {
    //   block [0x82699C50..0x82699CC0)
	// 82699C50: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82699C54: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82699C58: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82699C5C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82699C60: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 82699C64: 38AA2EA0  addi r5, r10, 0x2ea0
	ctx.r[5].s64 = ctx.r[10].s64 + 11936;
	// 82699C68: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82699C6C: 390BF6B4  addi r8, r11, -0x94c
	ctx.r[8].s64 = ctx.r[11].s64 + -2380;
	// 82699C70: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 82699C74: 388A9CDC  addi r4, r10, -0x6324
	ctx.r[4].s64 = ctx.r[10].s64 + -25380;
	// 82699C78: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82699C7C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82699C80: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82699C84: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82699C88: 386A3200  addi r3, r10, 0x3200
	ctx.r[3].s64 = ctx.r[10].s64 + 12800;
	// 82699C8C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82699C90: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82699C94: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82699C98: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82699C9C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82699CA0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82699CA4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82699CA8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82699CAC: 4BDCD175  bl 0x82466e20
	ctx.lr = 0x82699CB0;
	sub_82466E20(ctx, base);
	// 82699CB0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82699CB4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82699CB8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82699CBC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82699CC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82699CC0 size=108
    let mut pc: u32 = 0x82699CC0;
    'dispatch: loop {
        match pc {
            0x82699CC0 => {
    //   block [0x82699CC0..0x82699D2C)
	// 82699CC0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82699CC4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82699CC8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82699CCC: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 82699CD0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82699CD4: 38EBF6E4  addi r7, r11, -0x91c
	ctx.r[7].s64 = ctx.r[11].s64 + -2332;
	// 82699CD8: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 82699CDC: 388A9CF4  addi r4, r10, -0x630c
	ctx.r[4].s64 = ctx.r[10].s64 + -25356;
	// 82699CE0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82699CE4: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82699CE8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82699CEC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82699CF0: 386A3230  addi r3, r10, 0x3230
	ctx.r[3].s64 = ctx.r[10].s64 + 12848;
	// 82699CF4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82699CF8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82699CFC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82699D00: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82699D04: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82699D08: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82699D0C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82699D10: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82699D14: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82699D18: 4BDCD109  bl 0x82466e20
	ctx.lr = 0x82699D1C;
	sub_82466E20(ctx, base);
	// 82699D1C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82699D20: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82699D24: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82699D28: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82699D30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82699D30 size=108
    let mut pc: u32 = 0x82699D30;
    'dispatch: loop {
        match pc {
            0x82699D30 => {
    //   block [0x82699D30..0x82699D9C)
	// 82699D30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82699D34: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82699D38: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82699D3C: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 82699D40: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82699D44: 38EBF718  addi r7, r11, -0x8e8
	ctx.r[7].s64 = ctx.r[11].s64 + -2280;
	// 82699D48: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 82699D4C: 388AB148  addi r4, r10, -0x4eb8
	ctx.r[4].s64 = ctx.r[10].s64 + -20152;
	// 82699D50: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82699D54: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82699D58: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82699D5C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82699D60: 386A3260  addi r3, r10, 0x3260
	ctx.r[3].s64 = ctx.r[10].s64 + 12896;
	// 82699D64: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82699D68: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82699D6C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82699D70: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82699D74: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82699D78: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82699D7C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82699D80: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82699D84: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82699D88: 4BDCD099  bl 0x82466e20
	ctx.lr = 0x82699D8C;
	sub_82466E20(ctx, base);
	// 82699D8C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82699D90: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82699D94: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82699D98: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82699DA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82699DA0 size=108
    let mut pc: u32 = 0x82699DA0;
    'dispatch: loop {
        match pc {
            0x82699DA0 => {
    //   block [0x82699DA0..0x82699E0C)
	// 82699DA0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82699DA4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82699DA8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82699DAC: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 82699DB0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82699DB4: 38EBF778  addi r7, r11, -0x888
	ctx.r[7].s64 = ctx.r[11].s64 + -2184;
	// 82699DB8: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 82699DBC: 388AA9FC  addi r4, r10, -0x5604
	ctx.r[4].s64 = ctx.r[10].s64 + -22020;
	// 82699DC0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82699DC4: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82699DC8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82699DCC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82699DD0: 386A3290  addi r3, r10, 0x3290
	ctx.r[3].s64 = ctx.r[10].s64 + 12944;
	// 82699DD4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82699DD8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82699DDC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82699DE0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82699DE4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82699DE8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82699DEC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82699DF0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82699DF4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82699DF8: 4BDCD029  bl 0x82466e20
	ctx.lr = 0x82699DFC;
	sub_82466E20(ctx, base);
	// 82699DFC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82699E00: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82699E04: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82699E08: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82699E10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82699E10 size=108
    let mut pc: u32 = 0x82699E10;
    'dispatch: loop {
        match pc {
            0x82699E10 => {
    //   block [0x82699E10..0x82699E7C)
	// 82699E10: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82699E14: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82699E18: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82699E1C: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 82699E20: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82699E24: 38EBF7A8  addi r7, r11, -0x858
	ctx.r[7].s64 = ctx.r[11].s64 + -2136;
	// 82699E28: 3900000C  li r8, 0xc
	ctx.r[8].s64 = 12;
	// 82699E2C: 388AA978  addi r4, r10, -0x5688
	ctx.r[4].s64 = ctx.r[10].s64 + -22152;
	// 82699E30: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82699E34: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82699E38: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82699E3C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82699E40: 386A32C0  addi r3, r10, 0x32c0
	ctx.r[3].s64 = ctx.r[10].s64 + 12992;
	// 82699E44: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82699E48: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82699E4C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82699E50: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82699E54: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82699E58: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82699E5C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82699E60: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82699E64: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82699E68: 4BDCCFB9  bl 0x82466e20
	ctx.lr = 0x82699E6C;
	sub_82466E20(ctx, base);
	// 82699E6C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82699E70: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82699E74: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82699E78: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82699E80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82699E80 size=108
    let mut pc: u32 = 0x82699E80;
    'dispatch: loop {
        match pc {
            0x82699E80 => {
    //   block [0x82699E80..0x82699EEC)
	// 82699E80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82699E84: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82699E88: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82699E8C: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 82699E90: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82699E94: 38EBF8C8  addi r7, r11, -0x738
	ctx.r[7].s64 = ctx.r[11].s64 + -1848;
	// 82699E98: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 82699E9C: 388AA044  addi r4, r10, -0x5fbc
	ctx.r[4].s64 = ctx.r[10].s64 + -24508;
	// 82699EA0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82699EA4: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82699EA8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82699EAC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82699EB0: 386A32F0  addi r3, r10, 0x32f0
	ctx.r[3].s64 = ctx.r[10].s64 + 13040;
	// 82699EB4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82699EB8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82699EBC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82699EC0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82699EC4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82699EC8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82699ECC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82699ED0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82699ED4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82699ED8: 4BDCCF49  bl 0x82466e20
	ctx.lr = 0x82699EDC;
	sub_82466E20(ctx, base);
	// 82699EDC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82699EE0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82699EE4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82699EE8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82699EF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82699EF0 size=108
    let mut pc: u32 = 0x82699EF0;
    'dispatch: loop {
        match pc {
            0x82699EF0 => {
    //   block [0x82699EF0..0x82699F5C)
	// 82699EF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82699EF4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82699EF8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82699EFC: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 82699F00: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82699F04: 38EBF8E0  addi r7, r11, -0x720
	ctx.r[7].s64 = ctx.r[11].s64 + -1824;
	// 82699F08: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 82699F0C: 388AAF3C  addi r4, r10, -0x50c4
	ctx.r[4].s64 = ctx.r[10].s64 + -20676;
	// 82699F10: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82699F14: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82699F18: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82699F1C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82699F20: 386A3320  addi r3, r10, 0x3320
	ctx.r[3].s64 = ctx.r[10].s64 + 13088;
	// 82699F24: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82699F28: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82699F2C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82699F30: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82699F34: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82699F38: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82699F3C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82699F40: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82699F44: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82699F48: 4BDCCED9  bl 0x82466e20
	ctx.lr = 0x82699F4C;
	sub_82466E20(ctx, base);
	// 82699F4C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82699F50: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82699F54: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82699F58: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82699F60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82699F60 size=108
    let mut pc: u32 = 0x82699F60;
    'dispatch: loop {
        match pc {
            0x82699F60 => {
    //   block [0x82699F60..0x82699FCC)
	// 82699F60: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82699F64: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82699F68: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82699F6C: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 82699F70: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82699F74: 38EBF8F8  addi r7, r11, -0x708
	ctx.r[7].s64 = ctx.r[11].s64 + -1800;
	// 82699F78: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 82699F7C: 388AA0C0  addi r4, r10, -0x5f40
	ctx.r[4].s64 = ctx.r[10].s64 + -24384;
	// 82699F80: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82699F84: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82699F88: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82699F8C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82699F90: 386A3350  addi r3, r10, 0x3350
	ctx.r[3].s64 = ctx.r[10].s64 + 13136;
	// 82699F94: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82699F98: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82699F9C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82699FA0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82699FA4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82699FA8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82699FAC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82699FB0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82699FB4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82699FB8: 4BDCCE69  bl 0x82466e20
	ctx.lr = 0x82699FBC;
	sub_82466E20(ctx, base);
	// 82699FBC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82699FC0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82699FC4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82699FC8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82699FD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82699FD0 size=108
    let mut pc: u32 = 0x82699FD0;
    'dispatch: loop {
        match pc {
            0x82699FD0 => {
    //   block [0x82699FD0..0x8269A03C)
	// 82699FD0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82699FD4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82699FD8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82699FDC: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 82699FE0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82699FE4: 38EBF910  addi r7, r11, -0x6f0
	ctx.r[7].s64 = ctx.r[11].s64 + -1776;
	// 82699FE8: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 82699FEC: 388A9C24  addi r4, r10, -0x63dc
	ctx.r[4].s64 = ctx.r[10].s64 + -25564;
	// 82699FF0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82699FF4: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82699FF8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82699FFC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8269A000: 386A3380  addi r3, r10, 0x3380
	ctx.r[3].s64 = ctx.r[10].s64 + 13184;
	// 8269A004: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8269A008: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8269A00C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8269A010: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8269A014: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8269A018: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8269A01C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8269A020: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8269A024: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8269A028: 4BDCCDF9  bl 0x82466e20
	ctx.lr = 0x8269A02C;
	sub_82466E20(ctx, base);
	// 8269A02C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8269A030: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8269A034: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8269A038: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8269A040(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8269A040 size=108
    let mut pc: u32 = 0x8269A040;
    'dispatch: loop {
        match pc {
            0x8269A040 => {
    //   block [0x8269A040..0x8269A0AC)
	// 8269A040: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8269A044: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8269A048: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8269A04C: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 8269A050: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8269A054: 38EBF928  addi r7, r11, -0x6d8
	ctx.r[7].s64 = ctx.r[11].s64 + -1752;
	// 8269A058: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 8269A05C: 388AA684  addi r4, r10, -0x597c
	ctx.r[4].s64 = ctx.r[10].s64 + -22908;
	// 8269A060: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8269A064: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269A068: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8269A06C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8269A070: 386A33B0  addi r3, r10, 0x33b0
	ctx.r[3].s64 = ctx.r[10].s64 + 13232;
	// 8269A074: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8269A078: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8269A07C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8269A080: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8269A084: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8269A088: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8269A08C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8269A090: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8269A094: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8269A098: 4BDCCD89  bl 0x82466e20
	ctx.lr = 0x8269A09C;
	sub_82466E20(ctx, base);
	// 8269A09C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8269A0A0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8269A0A4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8269A0A8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8269A0B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8269A0B0 size=108
    let mut pc: u32 = 0x8269A0B0;
    'dispatch: loop {
        match pc {
            0x8269A0B0 => {
    //   block [0x8269A0B0..0x8269A11C)
	// 8269A0B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8269A0B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8269A0B8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8269A0BC: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 8269A0C0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8269A0C4: 38EBF940  addi r7, r11, -0x6c0
	ctx.r[7].s64 = ctx.r[11].s64 + -1728;
	// 8269A0C8: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 8269A0CC: 388AA26C  addi r4, r10, -0x5d94
	ctx.r[4].s64 = ctx.r[10].s64 + -23956;
	// 8269A0D0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8269A0D4: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269A0D8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8269A0DC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8269A0E0: 386A33E0  addi r3, r10, 0x33e0
	ctx.r[3].s64 = ctx.r[10].s64 + 13280;
	// 8269A0E4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8269A0E8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8269A0EC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8269A0F0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8269A0F4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8269A0F8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8269A0FC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8269A100: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8269A104: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8269A108: 4BDCCD19  bl 0x82466e20
	ctx.lr = 0x8269A10C;
	sub_82466E20(ctx, base);
	// 8269A10C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8269A110: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8269A114: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8269A118: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8269A120(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8269A120 size=108
    let mut pc: u32 = 0x8269A120;
    'dispatch: loop {
        match pc {
            0x8269A120 => {
    //   block [0x8269A120..0x8269A18C)
	// 8269A120: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8269A124: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8269A128: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8269A12C: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 8269A130: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8269A134: 38EBF958  addi r7, r11, -0x6a8
	ctx.r[7].s64 = ctx.r[11].s64 + -1704;
	// 8269A138: 39000006  li r8, 6
	ctx.r[8].s64 = 6;
	// 8269A13C: 388AB380  addi r4, r10, -0x4c80
	ctx.r[4].s64 = ctx.r[10].s64 + -19584;
	// 8269A140: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8269A144: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269A148: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8269A14C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8269A150: 386A3410  addi r3, r10, 0x3410
	ctx.r[3].s64 = ctx.r[10].s64 + 13328;
	// 8269A154: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8269A158: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8269A15C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8269A160: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8269A164: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8269A168: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8269A16C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8269A170: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8269A174: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8269A178: 4BDCCCA9  bl 0x82466e20
	ctx.lr = 0x8269A17C;
	sub_82466E20(ctx, base);
	// 8269A17C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8269A180: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8269A184: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8269A188: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8269A190(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8269A190 size=108
    let mut pc: u32 = 0x8269A190;
    'dispatch: loop {
        match pc {
            0x8269A190 => {
    //   block [0x8269A190..0x8269A1FC)
	// 8269A190: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8269A194: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8269A198: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8269A19C: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 8269A1A0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8269A1A4: 38EBF9E8  addi r7, r11, -0x618
	ctx.r[7].s64 = ctx.r[11].s64 + -1560;
	// 8269A1A8: 39000008  li r8, 8
	ctx.r[8].s64 = 8;
	// 8269A1AC: 388AA228  addi r4, r10, -0x5dd8
	ctx.r[4].s64 = ctx.r[10].s64 + -24024;
	// 8269A1B0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8269A1B4: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269A1B8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8269A1BC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8269A1C0: 386A3440  addi r3, r10, 0x3440
	ctx.r[3].s64 = ctx.r[10].s64 + 13376;
	// 8269A1C4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8269A1C8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8269A1CC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8269A1D0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8269A1D4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8269A1D8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8269A1DC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8269A1E0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8269A1E4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8269A1E8: 4BDCCC39  bl 0x82466e20
	ctx.lr = 0x8269A1EC;
	sub_82466E20(ctx, base);
	// 8269A1EC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8269A1F0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8269A1F4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8269A1F8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8269A200(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8269A200 size=108
    let mut pc: u32 = 0x8269A200;
    'dispatch: loop {
        match pc {
            0x8269A200 => {
    //   block [0x8269A200..0x8269A26C)
	// 8269A200: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8269A204: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8269A208: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8269A20C: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 8269A210: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8269A214: 38EBFAA8  addi r7, r11, -0x558
	ctx.r[7].s64 = ctx.r[11].s64 + -1368;
	// 8269A218: 39000009  li r8, 9
	ctx.r[8].s64 = 9;
	// 8269A21C: 388AA000  addi r4, r10, -0x6000
	ctx.r[4].s64 = ctx.r[10].s64 + -24576;
	// 8269A220: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8269A224: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269A228: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8269A22C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8269A230: 386A3470  addi r3, r10, 0x3470
	ctx.r[3].s64 = ctx.r[10].s64 + 13424;
	// 8269A234: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8269A238: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8269A23C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8269A240: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8269A244: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8269A248: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8269A24C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8269A250: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8269A254: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8269A258: 4BDCCBC9  bl 0x82466e20
	ctx.lr = 0x8269A25C;
	sub_82466E20(ctx, base);
	// 8269A25C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8269A260: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8269A264: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8269A268: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8269A270(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8269A270 size=108
    let mut pc: u32 = 0x8269A270;
    'dispatch: loop {
        match pc {
            0x8269A270 => {
    //   block [0x8269A270..0x8269A2DC)
	// 8269A270: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8269A274: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8269A278: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8269A27C: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 8269A280: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8269A284: 38EBFB80  addi r7, r11, -0x480
	ctx.r[7].s64 = ctx.r[11].s64 + -1152;
	// 8269A288: 39000008  li r8, 8
	ctx.r[8].s64 = 8;
	// 8269A28C: 388A9F00  addi r4, r10, -0x6100
	ctx.r[4].s64 = ctx.r[10].s64 + -24832;
	// 8269A290: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8269A294: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269A298: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8269A29C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8269A2A0: 386A34A0  addi r3, r10, 0x34a0
	ctx.r[3].s64 = ctx.r[10].s64 + 13472;
	// 8269A2A4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8269A2A8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8269A2AC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8269A2B0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8269A2B4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8269A2B8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8269A2BC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8269A2C0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8269A2C4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8269A2C8: 4BDCCB59  bl 0x82466e20
	ctx.lr = 0x8269A2CC;
	sub_82466E20(ctx, base);
	// 8269A2CC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8269A2D0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8269A2D4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8269A2D8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8269A2E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8269A2E0 size=108
    let mut pc: u32 = 0x8269A2E0;
    'dispatch: loop {
        match pc {
            0x8269A2E0 => {
    //   block [0x8269A2E0..0x8269A34C)
	// 8269A2E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8269A2E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8269A2E8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8269A2EC: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 8269A2F0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8269A2F4: 38EBFC40  addi r7, r11, -0x3c0
	ctx.r[7].s64 = ctx.r[11].s64 + -960;
	// 8269A2F8: 39000007  li r8, 7
	ctx.r[8].s64 = 7;
	// 8269A2FC: 388AADE0  addi r4, r10, -0x5220
	ctx.r[4].s64 = ctx.r[10].s64 + -21024;
	// 8269A300: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8269A304: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269A308: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8269A30C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8269A310: 386A34D0  addi r3, r10, 0x34d0
	ctx.r[3].s64 = ctx.r[10].s64 + 13520;
	// 8269A314: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8269A318: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8269A31C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8269A320: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8269A324: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8269A328: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8269A32C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8269A330: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8269A334: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8269A338: 4BDCCAE9  bl 0x82466e20
	ctx.lr = 0x8269A33C;
	sub_82466E20(ctx, base);
	// 8269A33C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8269A340: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8269A344: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8269A348: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8269A350(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8269A350 size=112
    let mut pc: u32 = 0x8269A350;
    'dispatch: loop {
        match pc {
            0x8269A350 => {
    //   block [0x8269A350..0x8269A3C0)
	// 8269A350: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8269A354: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8269A358: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8269A35C: 3D40827C  lis r10, -0x7d84
	ctx.r[10].s64 = -2105802752;
	// 8269A360: 3900000D  li r8, 0xd
	ctx.r[8].s64 = 13;
	// 8269A364: 38EAFCE8  addi r7, r10, -0x318
	ctx.r[7].s64 = ctx.r[10].s64 + -792;
	// 8269A368: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8269A36C: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 8269A370: 388AA5FC  addi r4, r10, -0x5a04
	ctx.r[4].s64 = ctx.r[10].s64 + -23044;
	// 8269A374: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8269A378: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8269A37C: 396B98F8  addi r11, r11, -0x6708
	ctx.r[11].s64 = ctx.r[11].s64 + -26376;
	// 8269A380: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8269A384: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269A388: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8269A38C: 386A3500  addi r3, r10, 0x3500
	ctx.r[3].s64 = ctx.r[10].s64 + 13568;
	// 8269A390: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8269A394: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 8269A398: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8269A39C: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 8269A3A0: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8269A3A4: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8269A3A8: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8269A3AC: 4BDCCA75  bl 0x82466e20
	ctx.lr = 0x8269A3B0;
	sub_82466E20(ctx, base);
	// 8269A3B0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8269A3B4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8269A3B8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8269A3BC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8269A3C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8269A3C0 size=112
    let mut pc: u32 = 0x8269A3C0;
    'dispatch: loop {
        match pc {
            0x8269A3C0 => {
    //   block [0x8269A3C0..0x8269A430)
	// 8269A3C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8269A3C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8269A3C8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8269A3CC: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269A3D0: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 8269A3D4: 38AA2EA0  addi r5, r10, 0x2ea0
	ctx.r[5].s64 = ctx.r[10].s64 + 11936;
	// 8269A3D8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8269A3DC: 390BFE20  addi r8, r11, -0x1e0
	ctx.r[8].s64 = ctx.r[11].s64 + -480;
	// 8269A3E0: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 8269A3E4: 388AA3A4  addi r4, r10, -0x5c5c
	ctx.r[4].s64 = ctx.r[10].s64 + -23644;
	// 8269A3E8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8269A3EC: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269A3F0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8269A3F4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8269A3F8: 386A3530  addi r3, r10, 0x3530
	ctx.r[3].s64 = ctx.r[10].s64 + 13616;
	// 8269A3FC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8269A400: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8269A404: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8269A408: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8269A40C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8269A410: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8269A414: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8269A418: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8269A41C: 4BDCCA05  bl 0x82466e20
	ctx.lr = 0x8269A420;
	sub_82466E20(ctx, base);
	// 8269A420: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8269A424: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8269A428: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8269A42C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8269A430(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8269A430 size=108
    let mut pc: u32 = 0x8269A430;
    'dispatch: loop {
        match pc {
            0x8269A430 => {
    //   block [0x8269A430..0x8269A49C)
	// 8269A430: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8269A434: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8269A438: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8269A43C: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 8269A440: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8269A444: 38EBFE50  addi r7, r11, -0x1b0
	ctx.r[7].s64 = ctx.r[11].s64 + -432;
	// 8269A448: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 8269A44C: 388AAD28  addi r4, r10, -0x52d8
	ctx.r[4].s64 = ctx.r[10].s64 + -21208;
	// 8269A450: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8269A454: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269A458: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8269A45C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8269A460: 386A3560  addi r3, r10, 0x3560
	ctx.r[3].s64 = ctx.r[10].s64 + 13664;
	// 8269A464: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8269A468: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8269A46C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8269A470: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8269A474: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8269A478: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8269A47C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8269A480: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8269A484: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8269A488: 4BDCC999  bl 0x82466e20
	ctx.lr = 0x8269A48C;
	sub_82466E20(ctx, base);
	// 8269A48C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8269A490: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8269A494: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8269A498: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8269A4A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8269A4A0 size=108
    let mut pc: u32 = 0x8269A4A0;
    'dispatch: loop {
        match pc {
            0x8269A4A0 => {
    //   block [0x8269A4A0..0x8269A50C)
	// 8269A4A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8269A4A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8269A4A8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8269A4AC: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 8269A4B0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8269A4B4: 38EBFEB0  addi r7, r11, -0x150
	ctx.r[7].s64 = ctx.r[11].s64 + -336;
	// 8269A4B8: 3900000B  li r8, 0xb
	ctx.r[8].s64 = 11;
	// 8269A4BC: 388A9070  addi r4, r10, -0x6f90
	ctx.r[4].s64 = ctx.r[10].s64 + -28560;
	// 8269A4C0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8269A4C4: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269A4C8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8269A4CC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8269A4D0: 386A3590  addi r3, r10, 0x3590
	ctx.r[3].s64 = ctx.r[10].s64 + 13712;
	// 8269A4D4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8269A4D8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8269A4DC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8269A4E0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8269A4E4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8269A4E8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8269A4EC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8269A4F0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8269A4F4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8269A4F8: 4BDCC929  bl 0x82466e20
	ctx.lr = 0x8269A4FC;
	sub_82466E20(ctx, base);
	// 8269A4FC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8269A500: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8269A504: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8269A508: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8269A510(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8269A510 size=108
    let mut pc: u32 = 0x8269A510;
    'dispatch: loop {
        match pc {
            0x8269A510 => {
    //   block [0x8269A510..0x8269A57C)
	// 8269A510: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8269A514: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8269A518: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8269A51C: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 8269A520: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8269A524: 38EBFFB8  addi r7, r11, -0x48
	ctx.r[7].s64 = ctx.r[11].s64 + -72;
	// 8269A528: 39000009  li r8, 9
	ctx.r[8].s64 = 9;
	// 8269A52C: 388A8E98  addi r4, r10, -0x7168
	ctx.r[4].s64 = ctx.r[10].s64 + -29032;
	// 8269A530: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8269A534: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269A538: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8269A53C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8269A540: 386A35C0  addi r3, r10, 0x35c0
	ctx.r[3].s64 = ctx.r[10].s64 + 13760;
	// 8269A544: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8269A548: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8269A54C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8269A550: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8269A554: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8269A558: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8269A55C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8269A560: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8269A564: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8269A568: 4BDCC8B9  bl 0x82466e20
	ctx.lr = 0x8269A56C;
	sub_82466E20(ctx, base);
	// 8269A56C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8269A570: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8269A574: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8269A578: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8269A580(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8269A580 size=108
    let mut pc: u32 = 0x8269A580;
    'dispatch: loop {
        match pc {
            0x8269A580 => {
    //   block [0x8269A580..0x8269A5EC)
	// 8269A580: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8269A584: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8269A588: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8269A58C: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 8269A590: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8269A594: 38EB0090  addi r7, r11, 0x90
	ctx.r[7].s64 = ctx.r[11].s64 + 144;
	// 8269A598: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 8269A59C: 388A8CF0  addi r4, r10, -0x7310
	ctx.r[4].s64 = ctx.r[10].s64 + -29456;
	// 8269A5A0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8269A5A4: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269A5A8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8269A5AC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8269A5B0: 386A35F0  addi r3, r10, 0x35f0
	ctx.r[3].s64 = ctx.r[10].s64 + 13808;
	// 8269A5B4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8269A5B8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8269A5BC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8269A5C0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8269A5C4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8269A5C8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8269A5CC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8269A5D0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8269A5D4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8269A5D8: 4BDCC849  bl 0x82466e20
	ctx.lr = 0x8269A5DC;
	sub_82466E20(ctx, base);
	// 8269A5DC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8269A5E0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8269A5E4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8269A5E8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8269A5F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8269A5F0 size=108
    let mut pc: u32 = 0x8269A5F0;
    'dispatch: loop {
        match pc {
            0x8269A5F0 => {
    //   block [0x8269A5F0..0x8269A65C)
	// 8269A5F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8269A5F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8269A5F8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8269A5FC: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 8269A600: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8269A604: 38EB00D8  addi r7, r11, 0xd8
	ctx.r[7].s64 = ctx.r[11].s64 + 216;
	// 8269A608: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 8269A60C: 388A8D14  addi r4, r10, -0x72ec
	ctx.r[4].s64 = ctx.r[10].s64 + -29420;
	// 8269A610: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8269A614: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269A618: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8269A61C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8269A620: 386A3620  addi r3, r10, 0x3620
	ctx.r[3].s64 = ctx.r[10].s64 + 13856;
	// 8269A624: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8269A628: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8269A62C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8269A630: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8269A634: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8269A638: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8269A63C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8269A640: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8269A644: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8269A648: 4BDCC7D9  bl 0x82466e20
	ctx.lr = 0x8269A64C;
	sub_82466E20(ctx, base);
	// 8269A64C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8269A650: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8269A654: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8269A658: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8269A660(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8269A660 size=108
    let mut pc: u32 = 0x8269A660;
    'dispatch: loop {
        match pc {
            0x8269A660 => {
    //   block [0x8269A660..0x8269A6CC)
	// 8269A660: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8269A664: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8269A668: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8269A66C: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 8269A670: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8269A674: 38EB00F0  addi r7, r11, 0xf0
	ctx.r[7].s64 = ctx.r[11].s64 + 240;
	// 8269A678: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 8269A67C: 388AB3B4  addi r4, r10, -0x4c4c
	ctx.r[4].s64 = ctx.r[10].s64 + -19532;
	// 8269A680: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8269A684: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269A688: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8269A68C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8269A690: 386A3650  addi r3, r10, 0x3650
	ctx.r[3].s64 = ctx.r[10].s64 + 13904;
	// 8269A694: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8269A698: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8269A69C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8269A6A0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8269A6A4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8269A6A8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8269A6AC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8269A6B0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8269A6B4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8269A6B8: 4BDCC769  bl 0x82466e20
	ctx.lr = 0x8269A6BC;
	sub_82466E20(ctx, base);
	// 8269A6BC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8269A6C0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8269A6C4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8269A6C8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8269A6D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8269A6D0 size=108
    let mut pc: u32 = 0x8269A6D0;
    'dispatch: loop {
        match pc {
            0x8269A6D0 => {
    //   block [0x8269A6D0..0x8269A73C)
	// 8269A6D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8269A6D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8269A6D8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8269A6DC: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 8269A6E0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8269A6E4: 38EB0150  addi r7, r11, 0x150
	ctx.r[7].s64 = ctx.r[11].s64 + 336;
	// 8269A6E8: 39000008  li r8, 8
	ctx.r[8].s64 = 8;
	// 8269A6EC: 388AB3C0  addi r4, r10, -0x4c40
	ctx.r[4].s64 = ctx.r[10].s64 + -19520;
	// 8269A6F0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8269A6F4: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269A6F8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8269A6FC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8269A700: 386A3680  addi r3, r10, 0x3680
	ctx.r[3].s64 = ctx.r[10].s64 + 13952;
	// 8269A704: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8269A708: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8269A70C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8269A710: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8269A714: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8269A718: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8269A71C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8269A720: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8269A724: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8269A728: 4BDCC6F9  bl 0x82466e20
	ctx.lr = 0x8269A72C;
	sub_82466E20(ctx, base);
	// 8269A72C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8269A730: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8269A734: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8269A738: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8269A740(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8269A740 size=116
    let mut pc: u32 = 0x8269A740;
    'dispatch: loop {
        match pc {
            0x8269A740 => {
    //   block [0x8269A740..0x8269A7B4)
	// 8269A740: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8269A744: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8269A748: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8269A74C: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 8269A750: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 8269A754: 390B0210  addi r8, r11, 0x210
	ctx.r[8].s64 = ctx.r[11].s64 + 528;
	// 8269A758: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8269A75C: 392A9974  addi r9, r10, -0x668c
	ctx.r[9].s64 = ctx.r[10].s64 + -26252;
	// 8269A760: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269A764: 38E00006  li r7, 6
	ctx.r[7].s64 = 6;
	// 8269A768: 38AA3650  addi r5, r10, 0x3650
	ctx.r[5].s64 = ctx.r[10].s64 + 13904;
	// 8269A76C: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8269A770: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8269A774: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8269A778: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8269A77C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8269A780: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8269A784: 3D608297  lis r11, -0x7d69
	ctx.r[11].s64 = -2104033280;
	// 8269A788: 388AB3E0  addi r4, r10, -0x4c20
	ctx.r[4].s64 = ctx.r[10].s64 + -19488;
	// 8269A78C: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8269A790: 386B36B0  addi r3, r11, 0x36b0
	ctx.r[3].s64 = ctx.r[11].s64 + 14000;
	// 8269A794: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8269A798: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8269A79C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8269A7A0: 4BDCC681  bl 0x82466e20
	ctx.lr = 0x8269A7A4;
	sub_82466E20(ctx, base);
	// 8269A7A4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8269A7A8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8269A7AC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8269A7B0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8269A7B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8269A7B8 size=112
    let mut pc: u32 = 0x8269A7B8;
    'dispatch: loop {
        match pc {
            0x8269A7B8 => {
    //   block [0x8269A7B8..0x8269A828)
	// 8269A7B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8269A7BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8269A7C0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8269A7C4: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269A7C8: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 8269A7CC: 38AA5990  addi r5, r10, 0x5990
	ctx.r[5].s64 = ctx.r[10].s64 + 22928;
	// 8269A7D0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8269A7D4: 390B02A0  addi r8, r11, 0x2a0
	ctx.r[8].s64 = ctx.r[11].s64 + 672;
	// 8269A7D8: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 8269A7DC: 388AB3F0  addi r4, r10, -0x4c10
	ctx.r[4].s64 = ctx.r[10].s64 + -19472;
	// 8269A7E0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8269A7E4: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269A7E8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8269A7EC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8269A7F0: 386A36E0  addi r3, r10, 0x36e0
	ctx.r[3].s64 = ctx.r[10].s64 + 14048;
	// 8269A7F4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8269A7F8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8269A7FC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8269A800: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8269A804: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8269A808: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8269A80C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8269A810: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8269A814: 4BDCC60D  bl 0x82466e20
	ctx.lr = 0x8269A818;
	sub_82466E20(ctx, base);
	// 8269A818: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8269A81C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8269A820: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8269A824: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8269A828(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8269A828 size=96
    let mut pc: u32 = 0x8269A828;
    'dispatch: loop {
        match pc {
            0x8269A828 => {
    //   block [0x8269A828..0x8269A888)
	// 8269A828: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8269A82C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8269A830: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8269A834: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8269A838: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8269A83C: 388AB40C  addi r4, r10, -0x4bf4
	ctx.r[4].s64 = ctx.r[10].s64 + -19444;
	// 8269A840: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269A844: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8269A848: 386A3710  addi r3, r10, 0x3710
	ctx.r[3].s64 = ctx.r[10].s64 + 14096;
	// 8269A84C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8269A850: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8269A854: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 8269A858: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8269A85C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8269A860: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8269A864: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8269A868: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8269A86C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8269A870: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8269A874: 4BDCC5AD  bl 0x82466e20
	ctx.lr = 0x8269A878;
	sub_82466E20(ctx, base);
	// 8269A878: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8269A87C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8269A880: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8269A884: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8269A888(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8269A888 size=24
    let mut pc: u32 = 0x8269A888;
    'dispatch: loop {
        match pc {
            0x8269A888 => {
    //   block [0x8269A888..0x8269A8A0)
	// 8269A888: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 8269A88C: 3D40827C  lis r10, -0x7d84
	ctx.r[10].s64 = -2105802752;
	// 8269A890: 394A6B58  addi r10, r10, 0x6b58
	ctx.r[10].s64 = ctx.r[10].s64 + 27480;
	// 8269A894: 816B0300  lwz r11, 0x300(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(768 as u32) ) } as u64;
	// 8269A898: 916A0020  stw r11, 0x20(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 8269A89C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8269A8A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8269A8A0 size=116
    let mut pc: u32 = 0x8269A8A0;
    'dispatch: loop {
        match pc {
            0x8269A8A0 => {
    //   block [0x8269A8A0..0x8269A914)
	// 8269A8A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8269A8A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8269A8A8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8269A8AC: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 8269A8B0: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 8269A8B4: 390B6B58  addi r8, r11, 0x6b58
	ctx.r[8].s64 = ctx.r[11].s64 + 27480;
	// 8269A8B8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8269A8BC: 392A99C0  addi r9, r10, -0x6640
	ctx.r[9].s64 = ctx.r[10].s64 + -26176;
	// 8269A8C0: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269A8C4: 38E00003  li r7, 3
	ctx.r[7].s64 = 3;
	// 8269A8C8: 38AA2B10  addi r5, r10, 0x2b10
	ctx.r[5].s64 = ctx.r[10].s64 + 11024;
	// 8269A8CC: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8269A8D0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8269A8D4: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8269A8D8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8269A8DC: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 8269A8E0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8269A8E4: 3D608297  lis r11, -0x7d69
	ctx.r[11].s64 = -2104033280;
	// 8269A8E8: 388AB42C  addi r4, r10, -0x4bd4
	ctx.r[4].s64 = ctx.r[10].s64 + -19412;
	// 8269A8EC: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8269A8F0: 386B3740  addi r3, r11, 0x3740
	ctx.r[3].s64 = ctx.r[11].s64 + 14144;
	// 8269A8F4: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8269A8F8: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8269A8FC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8269A900: 4BDCC521  bl 0x82466e20
	ctx.lr = 0x8269A904;
	sub_82466E20(ctx, base);
	// 8269A904: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8269A908: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8269A90C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8269A910: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8269A918(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8269A918 size=104
    let mut pc: u32 = 0x8269A918;
    'dispatch: loop {
        match pc {
            0x8269A918 => {
    //   block [0x8269A918..0x8269A980)
	// 8269A918: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8269A91C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8269A920: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8269A924: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 8269A928: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8269A92C: 392A99EC  addi r9, r10, -0x6614
	ctx.r[9].s64 = ctx.r[10].s64 + -26132;
	// 8269A930: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269A934: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8269A938: 38AA2B10  addi r5, r10, 0x2b10
	ctx.r[5].s64 = ctx.r[10].s64 + 11024;
	// 8269A93C: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8269A940: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8269A944: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8269A948: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8269A94C: 388AB440  addi r4, r10, -0x4bc0
	ctx.r[4].s64 = ctx.r[10].s64 + -19392;
	// 8269A950: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8269A954: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269A958: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8269A95C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8269A960: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8269A964: 386A3770  addi r3, r10, 0x3770
	ctx.r[3].s64 = ctx.r[10].s64 + 14192;
	// 8269A968: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8269A96C: 4BDCC4B5  bl 0x82466e20
	ctx.lr = 0x8269A970;
	sub_82466E20(ctx, base);
	// 8269A970: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8269A974: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8269A978: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8269A97C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8269A980(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8269A980 size=96
    let mut pc: u32 = 0x8269A980;
    'dispatch: loop {
        match pc {
            0x8269A980 => {
    //   block [0x8269A980..0x8269A9E0)
	// 8269A980: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8269A984: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8269A988: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8269A98C: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8269A990: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8269A994: 388AB454  addi r4, r10, -0x4bac
	ctx.r[4].s64 = ctx.r[10].s64 + -19372;
	// 8269A998: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269A99C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8269A9A0: 386A37A0  addi r3, r10, 0x37a0
	ctx.r[3].s64 = ctx.r[10].s64 + 14240;
	// 8269A9A4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8269A9A8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8269A9AC: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 8269A9B0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8269A9B4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8269A9B8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8269A9BC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8269A9C0: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8269A9C4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8269A9C8: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8269A9CC: 4BDCC455  bl 0x82466e20
	ctx.lr = 0x8269A9D0;
	sub_82466E20(ctx, base);
	// 8269A9D0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8269A9D4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8269A9D8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8269A9DC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8269A9E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8269A9E0 size=96
    let mut pc: u32 = 0x8269A9E0;
    'dispatch: loop {
        match pc {
            0x8269A9E0 => {
    //   block [0x8269A9E0..0x8269AA40)
	// 8269A9E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8269A9E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8269A9E8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8269A9EC: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8269A9F0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8269A9F4: 388AB46C  addi r4, r10, -0x4b94
	ctx.r[4].s64 = ctx.r[10].s64 + -19348;
	// 8269A9F8: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269A9FC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8269AA00: 386A37D0  addi r3, r10, 0x37d0
	ctx.r[3].s64 = ctx.r[10].s64 + 14288;
	// 8269AA04: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8269AA08: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8269AA0C: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 8269AA10: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8269AA14: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8269AA18: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8269AA1C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8269AA20: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8269AA24: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8269AA28: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8269AA2C: 4BDCC3F5  bl 0x82466e20
	ctx.lr = 0x8269AA30;
	sub_82466E20(ctx, base);
	// 8269AA30: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8269AA34: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8269AA38: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8269AA3C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8269AA40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8269AA40 size=100
    let mut pc: u32 = 0x8269AA40;
    'dispatch: loop {
        match pc {
            0x8269AA40 => {
    //   block [0x8269AA40..0x8269AAA4)
	// 8269AA40: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8269AA44: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8269AA48: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8269AA4C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269AA50: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8269AA54: 38AA3770  addi r5, r10, 0x3770
	ctx.r[5].s64 = ctx.r[10].s64 + 14192;
	// 8269AA58: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8269AA5C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8269AA60: 388AB488  addi r4, r10, -0x4b78
	ctx.r[4].s64 = ctx.r[10].s64 + -19320;
	// 8269AA64: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269AA68: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8269AA6C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8269AA70: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8269AA74: 386A3800  addi r3, r10, 0x3800
	ctx.r[3].s64 = ctx.r[10].s64 + 14336;
	// 8269AA78: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8269AA7C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8269AA80: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8269AA84: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8269AA88: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8269AA8C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8269AA90: 4BDCC391  bl 0x82466e20
	ctx.lr = 0x8269AA94;
	sub_82466E20(ctx, base);
	// 8269AA94: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8269AA98: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8269AA9C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8269AAA0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8269AAA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8269AAA8 size=112
    let mut pc: u32 = 0x8269AAA8;
    'dispatch: loop {
        match pc {
            0x8269AAA8 => {
    //   block [0x8269AAA8..0x8269AB18)
	// 8269AAA8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8269AAAC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8269AAB0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8269AAB4: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269AAB8: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 8269AABC: 38AA3740  addi r5, r10, 0x3740
	ctx.r[5].s64 = ctx.r[10].s64 + 14144;
	// 8269AAC0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8269AAC4: 390B0308  addi r8, r11, 0x308
	ctx.r[8].s64 = ctx.r[11].s64 + 776;
	// 8269AAC8: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 8269AACC: 388AB4A4  addi r4, r10, -0x4b5c
	ctx.r[4].s64 = ctx.r[10].s64 + -19292;
	// 8269AAD0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8269AAD4: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269AAD8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8269AADC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8269AAE0: 386A3830  addi r3, r10, 0x3830
	ctx.r[3].s64 = ctx.r[10].s64 + 14384;
	// 8269AAE4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8269AAE8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8269AAEC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8269AAF0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8269AAF4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8269AAF8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8269AAFC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8269AB00: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8269AB04: 4BDCC31D  bl 0x82466e20
	ctx.lr = 0x8269AB08;
	sub_82466E20(ctx, base);
	// 8269AB08: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8269AB0C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8269AB10: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8269AB14: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8269AB18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8269AB18 size=112
    let mut pc: u32 = 0x8269AB18;
    'dispatch: loop {
        match pc {
            0x8269AB18 => {
    //   block [0x8269AB18..0x8269AB88)
	// 8269AB18: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8269AB1C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8269AB20: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8269AB24: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269AB28: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 8269AB2C: 38AA3740  addi r5, r10, 0x3740
	ctx.r[5].s64 = ctx.r[10].s64 + 14144;
	// 8269AB30: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8269AB34: 390B0350  addi r8, r11, 0x350
	ctx.r[8].s64 = ctx.r[11].s64 + 848;
	// 8269AB38: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8269AB3C: 388AB4B4  addi r4, r10, -0x4b4c
	ctx.r[4].s64 = ctx.r[10].s64 + -19276;
	// 8269AB40: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8269AB44: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269AB48: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8269AB4C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8269AB50: 386A3860  addi r3, r10, 0x3860
	ctx.r[3].s64 = ctx.r[10].s64 + 14432;
	// 8269AB54: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8269AB58: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8269AB5C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8269AB60: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8269AB64: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8269AB68: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8269AB6C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8269AB70: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8269AB74: 4BDCC2AD  bl 0x82466e20
	ctx.lr = 0x8269AB78;
	sub_82466E20(ctx, base);
	// 8269AB78: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8269AB7C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8269AB80: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8269AB84: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8269AB88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8269AB88 size=100
    let mut pc: u32 = 0x8269AB88;
    'dispatch: loop {
        match pc {
            0x8269AB88 => {
    //   block [0x8269AB88..0x8269ABEC)
	// 8269AB88: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8269AB8C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8269AB90: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8269AB94: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269AB98: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8269AB9C: 38AA3740  addi r5, r10, 0x3740
	ctx.r[5].s64 = ctx.r[10].s64 + 14144;
	// 8269ABA0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8269ABA4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8269ABA8: 388AB4CC  addi r4, r10, -0x4b34
	ctx.r[4].s64 = ctx.r[10].s64 + -19252;
	// 8269ABAC: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269ABB0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8269ABB4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8269ABB8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8269ABBC: 386A3890  addi r3, r10, 0x3890
	ctx.r[3].s64 = ctx.r[10].s64 + 14480;
	// 8269ABC0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8269ABC4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8269ABC8: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8269ABCC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8269ABD0: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8269ABD4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8269ABD8: 4BDCC249  bl 0x82466e20
	ctx.lr = 0x8269ABDC;
	sub_82466E20(ctx, base);
	// 8269ABDC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8269ABE0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8269ABE4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8269ABE8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8269ABF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8269ABF0 size=112
    let mut pc: u32 = 0x8269ABF0;
    'dispatch: loop {
        match pc {
            0x8269ABF0 => {
    //   block [0x8269ABF0..0x8269AC60)
	// 8269ABF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8269ABF4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8269ABF8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8269ABFC: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269AC00: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 8269AC04: 38AA2B10  addi r5, r10, 0x2b10
	ctx.r[5].s64 = ctx.r[10].s64 + 11024;
	// 8269AC08: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8269AC0C: 390B0368  addi r8, r11, 0x368
	ctx.r[8].s64 = ctx.r[11].s64 + 872;
	// 8269AC10: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 8269AC14: 388AB4E4  addi r4, r10, -0x4b1c
	ctx.r[4].s64 = ctx.r[10].s64 + -19228;
	// 8269AC18: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8269AC1C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269AC20: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8269AC24: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8269AC28: 386A38C0  addi r3, r10, 0x38c0
	ctx.r[3].s64 = ctx.r[10].s64 + 14528;
	// 8269AC2C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8269AC30: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8269AC34: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8269AC38: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8269AC3C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8269AC40: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8269AC44: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8269AC48: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8269AC4C: 4BDCC1D5  bl 0x82466e20
	ctx.lr = 0x8269AC50;
	sub_82466E20(ctx, base);
	// 8269AC50: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8269AC54: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8269AC58: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8269AC5C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8269AC60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8269AC60 size=96
    let mut pc: u32 = 0x8269AC60;
    'dispatch: loop {
        match pc {
            0x8269AC60 => {
    //   block [0x8269AC60..0x8269ACC0)
	// 8269AC60: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8269AC64: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8269AC68: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8269AC6C: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8269AC70: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8269AC74: 388AB4F0  addi r4, r10, -0x4b10
	ctx.r[4].s64 = ctx.r[10].s64 + -19216;
	// 8269AC78: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269AC7C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8269AC80: 386A38F0  addi r3, r10, 0x38f0
	ctx.r[3].s64 = ctx.r[10].s64 + 14576;
	// 8269AC84: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8269AC88: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8269AC8C: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 8269AC90: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8269AC94: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8269AC98: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8269AC9C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8269ACA0: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8269ACA4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8269ACA8: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8269ACAC: 4BDCC175  bl 0x82466e20
	ctx.lr = 0x8269ACB0;
	sub_82466E20(ctx, base);
	// 8269ACB0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8269ACB4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8269ACB8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8269ACBC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8269ACC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8269ACC0 size=112
    let mut pc: u32 = 0x8269ACC0;
    'dispatch: loop {
        match pc {
            0x8269ACC0 => {
    //   block [0x8269ACC0..0x8269AD30)
	// 8269ACC0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8269ACC4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8269ACC8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8269ACCC: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269ACD0: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 8269ACD4: 38AA38F0  addi r5, r10, 0x38f0
	ctx.r[5].s64 = ctx.r[10].s64 + 14576;
	// 8269ACD8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8269ACDC: 390B0398  addi r8, r11, 0x398
	ctx.r[8].s64 = ctx.r[11].s64 + 920;
	// 8269ACE0: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8269ACE4: 388AB504  addi r4, r10, -0x4afc
	ctx.r[4].s64 = ctx.r[10].s64 + -19196;
	// 8269ACE8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8269ACEC: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269ACF0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8269ACF4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8269ACF8: 386A3920  addi r3, r10, 0x3920
	ctx.r[3].s64 = ctx.r[10].s64 + 14624;
	// 8269ACFC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8269AD00: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8269AD04: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8269AD08: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8269AD0C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8269AD10: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8269AD14: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8269AD18: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8269AD1C: 4BDCC105  bl 0x82466e20
	ctx.lr = 0x8269AD20;
	sub_82466E20(ctx, base);
	// 8269AD20: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8269AD24: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8269AD28: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8269AD2C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8269AD30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8269AD30 size=112
    let mut pc: u32 = 0x8269AD30;
    'dispatch: loop {
        match pc {
            0x8269AD30 => {
    //   block [0x8269AD30..0x8269ADA0)
	// 8269AD30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8269AD34: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8269AD38: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8269AD3C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269AD40: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 8269AD44: 38AA38C0  addi r5, r10, 0x38c0
	ctx.r[5].s64 = ctx.r[10].s64 + 14528;
	// 8269AD48: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8269AD4C: 390B03B0  addi r8, r11, 0x3b0
	ctx.r[8].s64 = ctx.r[11].s64 + 944;
	// 8269AD50: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8269AD54: 388AB51C  addi r4, r10, -0x4ae4
	ctx.r[4].s64 = ctx.r[10].s64 + -19172;
	// 8269AD58: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8269AD5C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269AD60: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8269AD64: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8269AD68: 386A3950  addi r3, r10, 0x3950
	ctx.r[3].s64 = ctx.r[10].s64 + 14672;
	// 8269AD6C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8269AD70: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8269AD74: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8269AD78: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8269AD7C: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 8269AD80: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8269AD84: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8269AD88: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8269AD8C: 4BDCC095  bl 0x82466e20
	ctx.lr = 0x8269AD90;
	sub_82466E20(ctx, base);
	// 8269AD90: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8269AD94: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8269AD98: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8269AD9C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8269ADA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8269ADA0 size=36
    let mut pc: u32 = 0x8269ADA0;
    'dispatch: loop {
        match pc {
            0x8269ADA0 => {
    //   block [0x8269ADA0..0x8269ADC4)
	// 8269ADA0: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 8269ADA4: 814B03D0  lwz r10, 0x3d0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(976 as u32) ) } as u64;
	// 8269ADA8: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 8269ADAC: 396B6BA0  addi r11, r11, 0x6ba0
	ctx.r[11].s64 = ctx.r[11].s64 + 27552;
	// 8269ADB0: 914B0008  stw r10, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 8269ADB4: 3D40827C  lis r10, -0x7d84
	ctx.r[10].s64 = -2105802752;
	// 8269ADB8: 814A03C8  lwz r10, 0x3c8(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(968 as u32) ) } as u64;
	// 8269ADBC: 914B0020  stw r10, 0x20(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(32 as u32), ctx.r[10].u32 ) };
	// 8269ADC0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8269ADC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8269ADC8 size=108
    let mut pc: u32 = 0x8269ADC8;
    'dispatch: loop {
        match pc {
            0x8269ADC8 => {
    //   block [0x8269ADC8..0x8269AE34)
	// 8269ADC8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8269ADCC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8269ADD0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8269ADD4: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 8269ADD8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8269ADDC: 38EB6BA0  addi r7, r11, 0x6ba0
	ctx.r[7].s64 = ctx.r[11].s64 + 27552;
	// 8269ADE0: 39000007  li r8, 7
	ctx.r[8].s64 = 7;
	// 8269ADE4: 388AB548  addi r4, r10, -0x4ab8
	ctx.r[4].s64 = ctx.r[10].s64 + -19128;
	// 8269ADE8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8269ADEC: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269ADF0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8269ADF4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8269ADF8: 386A3980  addi r3, r10, 0x3980
	ctx.r[3].s64 = ctx.r[10].s64 + 14720;
	// 8269ADFC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8269AE00: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8269AE04: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8269AE08: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8269AE0C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8269AE10: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8269AE14: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8269AE18: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8269AE1C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8269AE20: 4BDCC001  bl 0x82466e20
	ctx.lr = 0x8269AE24;
	sub_82466E20(ctx, base);
	// 8269AE24: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8269AE28: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8269AE2C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8269AE30: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8269AE38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8269AE38 size=24
    let mut pc: u32 = 0x8269AE38;
    'dispatch: loop {
        match pc {
            0x8269AE38 => {
    //   block [0x8269AE38..0x8269AE50)
	// 8269AE38: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 8269AE3C: 3D40827C  lis r10, -0x7d84
	ctx.r[10].s64 = -2105802752;
	// 8269AE40: 394A6C48  addi r10, r10, 0x6c48
	ctx.r[10].s64 = ctx.r[10].s64 + 27720;
	// 8269AE44: 816B03C8  lwz r11, 0x3c8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(968 as u32) ) } as u64;
	// 8269AE48: 916A00B0  stw r11, 0xb0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(176 as u32), ctx.r[11].u32 ) };
	// 8269AE4C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8269AE50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8269AE50 size=116
    let mut pc: u32 = 0x8269AE50;
    'dispatch: loop {
        match pc {
            0x8269AE50 => {
    //   block [0x8269AE50..0x8269AEC4)
	// 8269AE50: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8269AE54: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8269AE58: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8269AE5C: 3D40827C  lis r10, -0x7d84
	ctx.r[10].s64 = -2105802752;
	// 8269AE60: 38E0000A  li r7, 0xa
	ctx.r[7].s64 = 10;
	// 8269AE64: 390A6C48  addi r8, r10, 0x6c48
	ctx.r[8].s64 = ctx.r[10].s64 + 27720;
	// 8269AE68: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269AE6C: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 8269AE70: 38AA3980  addi r5, r10, 0x3980
	ctx.r[5].s64 = ctx.r[10].s64 + 14720;
	// 8269AE74: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8269AE78: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8269AE7C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8269AE80: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8269AE84: 388AB57C  addi r4, r10, -0x4a84
	ctx.r[4].s64 = ctx.r[10].s64 + -19076;
	// 8269AE88: 396B9A8C  addi r11, r11, -0x6574
	ctx.r[11].s64 = ctx.r[11].s64 + -25972;
	// 8269AE8C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269AE90: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8269AE94: 386A39B0  addi r3, r10, 0x39b0
	ctx.r[3].s64 = ctx.r[10].s64 + 14768;
	// 8269AE98: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 8269AE9C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8269AEA0: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 8269AEA4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8269AEA8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8269AEAC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8269AEB0: 4BDCBF71  bl 0x82466e20
	ctx.lr = 0x8269AEB4;
	sub_82466E20(ctx, base);
	// 8269AEB4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8269AEB8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8269AEBC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8269AEC0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8269AEC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8269AEC8 size=112
    let mut pc: u32 = 0x8269AEC8;
    'dispatch: loop {
        match pc {
            0x8269AEC8 => {
    //   block [0x8269AEC8..0x8269AF38)
	// 8269AEC8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8269AECC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8269AED0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8269AED4: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269AED8: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 8269AEDC: 38AA3980  addi r5, r10, 0x3980
	ctx.r[5].s64 = ctx.r[10].s64 + 14720;
	// 8269AEE0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8269AEE4: 390B03D8  addi r8, r11, 0x3d8
	ctx.r[8].s64 = ctx.r[11].s64 + 984;
	// 8269AEE8: 39200005  li r9, 5
	ctx.r[9].s64 = 5;
	// 8269AEEC: 388AB5A4  addi r4, r10, -0x4a5c
	ctx.r[4].s64 = ctx.r[10].s64 + -19036;
	// 8269AEF0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8269AEF4: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269AEF8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8269AEFC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8269AF00: 386A39E0  addi r3, r10, 0x39e0
	ctx.r[3].s64 = ctx.r[10].s64 + 14816;
	// 8269AF04: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8269AF08: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8269AF0C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8269AF10: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8269AF14: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8269AF18: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8269AF1C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8269AF20: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8269AF24: 4BDCBEFD  bl 0x82466e20
	ctx.lr = 0x8269AF28;
	sub_82466E20(ctx, base);
	// 8269AF28: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8269AF2C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8269AF30: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8269AF34: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8269AF38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8269AF38 size=24
    let mut pc: u32 = 0x8269AF38;
    'dispatch: loop {
        match pc {
            0x8269AF38 => {
    //   block [0x8269AF38..0x8269AF50)
	// 8269AF38: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 8269AF3C: 3D40827C  lis r10, -0x7d84
	ctx.r[10].s64 = -2105802752;
	// 8269AF40: 394A6D38  addi r10, r10, 0x6d38
	ctx.r[10].s64 = ctx.r[10].s64 + 27960;
	// 8269AF44: 816B0EB0  lwz r11, 0xeb0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(3760 as u32) ) } as u64;
	// 8269AF48: 916A00B0  stw r11, 0xb0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(176 as u32), ctx.r[11].u32 ) };
	// 8269AF4C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8269AF50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8269AF50 size=116
    let mut pc: u32 = 0x8269AF50;
    'dispatch: loop {
        match pc {
            0x8269AF50 => {
    //   block [0x8269AF50..0x8269AFC4)
	// 8269AF50: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8269AF54: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8269AF58: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8269AF5C: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 8269AF60: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269AF64: 392B9A50  addi r9, r11, -0x65b0
	ctx.r[9].s64 = ctx.r[11].s64 + -26032;
	// 8269AF68: 38AA3950  addi r5, r10, 0x3950
	ctx.r[5].s64 = ctx.r[10].s64 + 14672;
	// 8269AF6C: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8269AF70: 38E90068  addi r7, r9, 0x68
	ctx.r[7].s64 = ctx.r[9].s64 + 104;
	// 8269AF74: 38C0000B  li r6, 0xb
	ctx.r[6].s64 = 11;
	// 8269AF78: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 8269AF7C: 388AB5E4  addi r4, r10, -0x4a1c
	ctx.r[4].s64 = ctx.r[10].s64 + -18972;
	// 8269AF80: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8269AF84: 396B6D38  addi r11, r11, 0x6d38
	ctx.r[11].s64 = ctx.r[11].s64 + 27960;
	// 8269AF88: 90E10064  stw r7, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[7].u32 ) };
	// 8269AF8C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269AF90: 90C1005C  stw r6, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[6].u32 ) };
	// 8269AF94: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8269AF98: 386A3A10  addi r3, r10, 0x3a10
	ctx.r[3].s64 = ctx.r[10].s64 + 14864;
	// 8269AF9C: 39400003  li r10, 3
	ctx.r[10].s64 = 3;
	// 8269AFA0: 91010074  stw r8, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[8].u32 ) };
	// 8269AFA4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8269AFA8: 9101006C  stw r8, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[8].u32 ) };
	// 8269AFAC: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8269AFB0: 4BDCBE71  bl 0x82466e20
	ctx.lr = 0x8269AFB4;
	sub_82466E20(ctx, base);
	// 8269AFB4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8269AFB8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8269AFBC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8269AFC0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8269AFC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8269AFC8 size=108
    let mut pc: u32 = 0x8269AFC8;
    'dispatch: loop {
        match pc {
            0x8269AFC8 => {
    //   block [0x8269AFC8..0x8269B034)
	// 8269AFC8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8269AFCC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8269AFD0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8269AFD4: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 8269AFD8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8269AFDC: 38EB0450  addi r7, r11, 0x450
	ctx.r[7].s64 = ctx.r[11].s64 + 1104;
	// 8269AFE0: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 8269AFE4: 388AB610  addi r4, r10, -0x49f0
	ctx.r[4].s64 = ctx.r[10].s64 + -18928;
	// 8269AFE8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8269AFEC: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269AFF0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8269AFF4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8269AFF8: 386A3A40  addi r3, r10, 0x3a40
	ctx.r[3].s64 = ctx.r[10].s64 + 14912;
	// 8269AFFC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8269B000: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8269B004: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8269B008: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8269B00C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8269B010: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8269B014: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8269B018: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8269B01C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8269B020: 4BDCBE01  bl 0x82466e20
	ctx.lr = 0x8269B024;
	sub_82466E20(ctx, base);
	// 8269B024: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8269B028: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8269B02C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8269B030: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8269B038(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8269B038 size=112
    let mut pc: u32 = 0x8269B038;
    'dispatch: loop {
        match pc {
            0x8269B038 => {
    //   block [0x8269B038..0x8269B0A8)
	// 8269B038: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8269B03C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8269B040: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8269B044: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269B048: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 8269B04C: 38AA3950  addi r5, r10, 0x3950
	ctx.r[5].s64 = ctx.r[10].s64 + 14672;
	// 8269B050: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8269B054: 390B04B0  addi r8, r11, 0x4b0
	ctx.r[8].s64 = ctx.r[11].s64 + 1200;
	// 8269B058: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 8269B05C: 388AB628  addi r4, r10, -0x49d8
	ctx.r[4].s64 = ctx.r[10].s64 + -18904;
	// 8269B060: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8269B064: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269B068: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8269B06C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8269B070: 386A3A70  addi r3, r10, 0x3a70
	ctx.r[3].s64 = ctx.r[10].s64 + 14960;
	// 8269B074: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8269B078: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8269B07C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8269B080: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8269B084: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8269B088: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8269B08C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8269B090: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8269B094: 4BDCBD8D  bl 0x82466e20
	ctx.lr = 0x8269B098;
	sub_82466E20(ctx, base);
	// 8269B098: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8269B09C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8269B0A0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8269B0A4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8269B0A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8269B0A8 size=108
    let mut pc: u32 = 0x8269B0A8;
    'dispatch: loop {
        match pc {
            0x8269B0A8 => {
    //   block [0x8269B0A8..0x8269B114)
	// 8269B0A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8269B0AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8269B0B0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8269B0B4: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 8269B0B8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8269B0BC: 38EB0510  addi r7, r11, 0x510
	ctx.r[7].s64 = ctx.r[11].s64 + 1296;
	// 8269B0C0: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 8269B0C4: 388AB638  addi r4, r10, -0x49c8
	ctx.r[4].s64 = ctx.r[10].s64 + -18888;
	// 8269B0C8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8269B0CC: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269B0D0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8269B0D4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8269B0D8: 386A3AA0  addi r3, r10, 0x3aa0
	ctx.r[3].s64 = ctx.r[10].s64 + 15008;
	// 8269B0DC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8269B0E0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8269B0E4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8269B0E8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8269B0EC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8269B0F0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8269B0F4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8269B0F8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8269B0FC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8269B100: 4BDCBD21  bl 0x82466e20
	ctx.lr = 0x8269B104;
	sub_82466E20(ctx, base);
	// 8269B104: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8269B108: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8269B10C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8269B110: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8269B118(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8269B118 size=108
    let mut pc: u32 = 0x8269B118;
    'dispatch: loop {
        match pc {
            0x8269B118 => {
    //   block [0x8269B118..0x8269B184)
	// 8269B118: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8269B11C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8269B120: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8269B124: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 8269B128: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8269B12C: 38EB0528  addi r7, r11, 0x528
	ctx.r[7].s64 = ctx.r[11].s64 + 1320;
	// 8269B130: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 8269B134: 388AB66C  addi r4, r10, -0x4994
	ctx.r[4].s64 = ctx.r[10].s64 + -18836;
	// 8269B138: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8269B13C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269B140: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8269B144: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8269B148: 386A3AD0  addi r3, r10, 0x3ad0
	ctx.r[3].s64 = ctx.r[10].s64 + 15056;
	// 8269B14C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8269B150: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8269B154: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8269B158: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8269B15C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8269B160: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8269B164: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8269B168: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8269B16C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8269B170: 4BDCBCB1  bl 0x82466e20
	ctx.lr = 0x8269B174;
	sub_82466E20(ctx, base);
	// 8269B174: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8269B178: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8269B17C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8269B180: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8269B188(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8269B188 size=24
    let mut pc: u32 = 0x8269B188;
    'dispatch: loop {
        match pc {
            0x8269B188 => {
    //   block [0x8269B188..0x8269B1A0)
	// 8269B188: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 8269B18C: 3D40827C  lis r10, -0x7d84
	ctx.r[10].s64 = -2105802752;
	// 8269B190: 394A6E40  addi r10, r10, 0x6e40
	ctx.r[10].s64 = ctx.r[10].s64 + 28224;
	// 8269B194: 816B0EB0  lwz r11, 0xeb0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(3760 as u32) ) } as u64;
	// 8269B198: 916A0068  stw r11, 0x68(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(104 as u32), ctx.r[11].u32 ) };
	// 8269B19C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8269B1A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8269B1A0 size=116
    let mut pc: u32 = 0x8269B1A0;
    'dispatch: loop {
        match pc {
            0x8269B1A0 => {
    //   block [0x8269B1A0..0x8269B214)
	// 8269B1A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8269B1A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8269B1A8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8269B1AC: 3D40827C  lis r10, -0x7d84
	ctx.r[10].s64 = -2105802752;
	// 8269B1B0: 38E00005  li r7, 5
	ctx.r[7].s64 = 5;
	// 8269B1B4: 390A6E40  addi r8, r10, 0x6e40
	ctx.r[8].s64 = ctx.r[10].s64 + 28224;
	// 8269B1B8: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269B1BC: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 8269B1C0: 38AA3950  addi r5, r10, 0x3950
	ctx.r[5].s64 = ctx.r[10].s64 + 14672;
	// 8269B1C4: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8269B1C8: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8269B1CC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8269B1D0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8269B1D4: 388AB688  addi r4, r10, -0x4978
	ctx.r[4].s64 = ctx.r[10].s64 + -18808;
	// 8269B1D8: 396B9AE8  addi r11, r11, -0x6518
	ctx.r[11].s64 = ctx.r[11].s64 + -25880;
	// 8269B1DC: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269B1E0: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8269B1E4: 386A3B00  addi r3, r10, 0x3b00
	ctx.r[3].s64 = ctx.r[10].s64 + 15104;
	// 8269B1E8: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 8269B1EC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8269B1F0: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 8269B1F4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8269B1F8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8269B1FC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8269B200: 4BDCBC21  bl 0x82466e20
	ctx.lr = 0x8269B204;
	sub_82466E20(ctx, base);
	// 8269B204: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8269B208: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8269B20C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8269B210: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8269B218(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8269B218 size=112
    let mut pc: u32 = 0x8269B218;
    'dispatch: loop {
        match pc {
            0x8269B218 => {
    //   block [0x8269B218..0x8269B288)
	// 8269B218: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8269B21C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8269B220: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8269B224: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269B228: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 8269B22C: 38AA2B10  addi r5, r10, 0x2b10
	ctx.r[5].s64 = ctx.r[10].s64 + 11024;
	// 8269B230: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8269B234: 390B0588  addi r8, r11, 0x588
	ctx.r[8].s64 = ctx.r[11].s64 + 1416;
	// 8269B238: 39200006  li r9, 6
	ctx.r[9].s64 = 6;
	// 8269B23C: 388AB69C  addi r4, r10, -0x4964
	ctx.r[4].s64 = ctx.r[10].s64 + -18788;
	// 8269B240: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8269B244: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269B248: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8269B24C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8269B250: 386A3B30  addi r3, r10, 0x3b30
	ctx.r[3].s64 = ctx.r[10].s64 + 15152;
	// 8269B254: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8269B258: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8269B25C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8269B260: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8269B264: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8269B268: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8269B26C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8269B270: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8269B274: 4BDCBBAD  bl 0x82466e20
	ctx.lr = 0x8269B278;
	sub_82466E20(ctx, base);
	// 8269B278: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8269B27C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8269B280: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8269B284: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8269B288(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8269B288 size=112
    let mut pc: u32 = 0x8269B288;
    'dispatch: loop {
        match pc {
            0x8269B288 => {
    //   block [0x8269B288..0x8269B2F8)
	// 8269B288: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8269B28C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8269B290: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8269B294: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269B298: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 8269B29C: 38AA2B10  addi r5, r10, 0x2b10
	ctx.r[5].s64 = ctx.r[10].s64 + 11024;
	// 8269B2A0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8269B2A4: 390B0618  addi r8, r11, 0x618
	ctx.r[8].s64 = ctx.r[11].s64 + 1560;
	// 8269B2A8: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 8269B2AC: 388AB6CC  addi r4, r10, -0x4934
	ctx.r[4].s64 = ctx.r[10].s64 + -18740;
	// 8269B2B0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8269B2B4: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269B2B8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8269B2BC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8269B2C0: 386A3B60  addi r3, r10, 0x3b60
	ctx.r[3].s64 = ctx.r[10].s64 + 15200;
	// 8269B2C4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8269B2C8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8269B2CC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8269B2D0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8269B2D4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8269B2D8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8269B2DC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8269B2E0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8269B2E4: 4BDCBB3D  bl 0x82466e20
	ctx.lr = 0x8269B2E8;
	sub_82466E20(ctx, base);
	// 8269B2E8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8269B2EC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8269B2F0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8269B2F4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8269B2F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8269B2F8 size=112
    let mut pc: u32 = 0x8269B2F8;
    'dispatch: loop {
        match pc {
            0x8269B2F8 => {
    //   block [0x8269B2F8..0x8269B368)
	// 8269B2F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8269B2FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8269B300: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8269B304: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269B308: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 8269B30C: 38AA3A10  addi r5, r10, 0x3a10
	ctx.r[5].s64 = ctx.r[10].s64 + 14864;
	// 8269B310: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8269B314: 390B0678  addi r8, r11, 0x678
	ctx.r[8].s64 = ctx.r[11].s64 + 1656;
	// 8269B318: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 8269B31C: 388AB6FC  addi r4, r10, -0x4904
	ctx.r[4].s64 = ctx.r[10].s64 + -18692;
	// 8269B320: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8269B324: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269B328: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8269B32C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8269B330: 386A3B90  addi r3, r10, 0x3b90
	ctx.r[3].s64 = ctx.r[10].s64 + 15248;
	// 8269B334: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8269B338: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8269B33C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8269B340: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8269B344: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8269B348: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8269B34C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8269B350: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8269B354: 4BDCBACD  bl 0x82466e20
	ctx.lr = 0x8269B358;
	sub_82466E20(ctx, base);
	// 8269B358: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8269B35C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8269B360: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8269B364: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8269B368(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8269B368 size=100
    let mut pc: u32 = 0x8269B368;
    'dispatch: loop {
        match pc {
            0x8269B368 => {
    //   block [0x8269B368..0x8269B3CC)
	// 8269B368: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8269B36C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8269B370: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8269B374: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269B378: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8269B37C: 38AA38C0  addi r5, r10, 0x38c0
	ctx.r[5].s64 = ctx.r[10].s64 + 14528;
	// 8269B380: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8269B384: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8269B388: 388AB750  addi r4, r10, -0x48b0
	ctx.r[4].s64 = ctx.r[10].s64 + -18608;
	// 8269B38C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269B390: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8269B394: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8269B398: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8269B39C: 386A3BC0  addi r3, r10, 0x3bc0
	ctx.r[3].s64 = ctx.r[10].s64 + 15296;
	// 8269B3A0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8269B3A4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8269B3A8: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8269B3AC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8269B3B0: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8269B3B4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8269B3B8: 4BDCBA69  bl 0x82466e20
	ctx.lr = 0x8269B3BC;
	sub_82466E20(ctx, base);
	// 8269B3BC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8269B3C0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8269B3C4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8269B3C8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8269B3D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8269B3D0 size=112
    let mut pc: u32 = 0x8269B3D0;
    'dispatch: loop {
        match pc {
            0x8269B3D0 => {
    //   block [0x8269B3D0..0x8269B440)
	// 8269B3D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8269B3D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8269B3D8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8269B3DC: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269B3E0: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 8269B3E4: 38AA3BC0  addi r5, r10, 0x3bc0
	ctx.r[5].s64 = ctx.r[10].s64 + 15296;
	// 8269B3E8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8269B3EC: 390B06A8  addi r8, r11, 0x6a8
	ctx.r[8].s64 = ctx.r[11].s64 + 1704;
	// 8269B3F0: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 8269B3F4: 388AB760  addi r4, r10, -0x48a0
	ctx.r[4].s64 = ctx.r[10].s64 + -18592;
	// 8269B3F8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8269B3FC: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269B400: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8269B404: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8269B408: 386A3BF0  addi r3, r10, 0x3bf0
	ctx.r[3].s64 = ctx.r[10].s64 + 15344;
	// 8269B40C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8269B410: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8269B414: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8269B418: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8269B41C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8269B420: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8269B424: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8269B428: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8269B42C: 4BDCB9F5  bl 0x82466e20
	ctx.lr = 0x8269B430;
	sub_82466E20(ctx, base);
	// 8269B430: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8269B434: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8269B438: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8269B43C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8269B440(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8269B440 size=112
    let mut pc: u32 = 0x8269B440;
    'dispatch: loop {
        match pc {
            0x8269B440 => {
    //   block [0x8269B440..0x8269B4B0)
	// 8269B440: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8269B444: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8269B448: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8269B44C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269B450: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 8269B454: 38AA3BF0  addi r5, r10, 0x3bf0
	ctx.r[5].s64 = ctx.r[10].s64 + 15344;
	// 8269B458: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8269B45C: 390B0708  addi r8, r11, 0x708
	ctx.r[8].s64 = ctx.r[11].s64 + 1800;
	// 8269B460: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 8269B464: 388AB778  addi r4, r10, -0x4888
	ctx.r[4].s64 = ctx.r[10].s64 + -18568;
	// 8269B468: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8269B46C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269B470: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8269B474: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8269B478: 386A3C20  addi r3, r10, 0x3c20
	ctx.r[3].s64 = ctx.r[10].s64 + 15392;
	// 8269B47C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8269B480: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8269B484: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8269B488: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8269B48C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8269B490: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8269B494: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8269B498: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8269B49C: 4BDCB985  bl 0x82466e20
	ctx.lr = 0x8269B4A0;
	sub_82466E20(ctx, base);
	// 8269B4A0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8269B4A4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8269B4A8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8269B4AC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8269B4B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8269B4B0 size=112
    let mut pc: u32 = 0x8269B4B0;
    'dispatch: loop {
        match pc {
            0x8269B4B0 => {
    //   block [0x8269B4B0..0x8269B520)
	// 8269B4B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8269B4B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8269B4B8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8269B4BC: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269B4C0: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 8269B4C4: 38AA2B10  addi r5, r10, 0x2b10
	ctx.r[5].s64 = ctx.r[10].s64 + 11024;
	// 8269B4C8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8269B4CC: 390B0738  addi r8, r11, 0x738
	ctx.r[8].s64 = ctx.r[11].s64 + 1848;
	// 8269B4D0: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 8269B4D4: 388AB7A4  addi r4, r10, -0x485c
	ctx.r[4].s64 = ctx.r[10].s64 + -18524;
	// 8269B4D8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8269B4DC: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269B4E0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8269B4E4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8269B4E8: 386A3C50  addi r3, r10, 0x3c50
	ctx.r[3].s64 = ctx.r[10].s64 + 15440;
	// 8269B4EC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8269B4F0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8269B4F4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8269B4F8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8269B4FC: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 8269B500: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8269B504: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8269B508: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8269B50C: 4BDCB915  bl 0x82466e20
	ctx.lr = 0x8269B510;
	sub_82466E20(ctx, base);
	// 8269B510: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8269B514: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8269B518: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8269B51C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8269B520(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8269B520 size=116
    let mut pc: u32 = 0x8269B520;
    'dispatch: loop {
        match pc {
            0x8269B520 => {
    //   block [0x8269B520..0x8269B594)
	// 8269B520: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8269B524: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8269B528: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8269B52C: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 8269B530: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 8269B534: 390B0768  addi r8, r11, 0x768
	ctx.r[8].s64 = ctx.r[11].s64 + 1896;
	// 8269B538: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8269B53C: 392A9B18  addi r9, r10, -0x64e8
	ctx.r[9].s64 = ctx.r[10].s64 + -25832;
	// 8269B540: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269B544: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
	// 8269B548: 38AA4010  addi r5, r10, 0x4010
	ctx.r[5].s64 = ctx.r[10].s64 + 16400;
	// 8269B54C: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8269B550: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8269B554: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8269B558: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8269B55C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8269B560: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8269B564: 3D608297  lis r11, -0x7d69
	ctx.r[11].s64 = -2104033280;
	// 8269B568: 388AB7C4  addi r4, r10, -0x483c
	ctx.r[4].s64 = ctx.r[10].s64 + -18492;
	// 8269B56C: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8269B570: 386B3C80  addi r3, r11, 0x3c80
	ctx.r[3].s64 = ctx.r[11].s64 + 15488;
	// 8269B574: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8269B578: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8269B57C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8269B580: 4BDCB8A1  bl 0x82466e20
	ctx.lr = 0x8269B584;
	sub_82466E20(ctx, base);
	// 8269B584: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8269B588: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8269B58C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8269B590: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8269B598(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8269B598 size=112
    let mut pc: u32 = 0x8269B598;
    'dispatch: loop {
        match pc {
            0x8269B598 => {
    //   block [0x8269B598..0x8269B608)
	// 8269B598: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8269B59C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8269B5A0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8269B5A4: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269B5A8: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 8269B5AC: 38AA3C80  addi r5, r10, 0x3c80
	ctx.r[5].s64 = ctx.r[10].s64 + 15488;
	// 8269B5B0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8269B5B4: 390B0780  addi r8, r11, 0x780
	ctx.r[8].s64 = ctx.r[11].s64 + 1920;
	// 8269B5B8: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8269B5BC: 388AB7D4  addi r4, r10, -0x482c
	ctx.r[4].s64 = ctx.r[10].s64 + -18476;
	// 8269B5C0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8269B5C4: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269B5C8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8269B5CC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8269B5D0: 386A3CB0  addi r3, r10, 0x3cb0
	ctx.r[3].s64 = ctx.r[10].s64 + 15536;
	// 8269B5D4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8269B5D8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8269B5DC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8269B5E0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8269B5E4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8269B5E8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8269B5EC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8269B5F0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8269B5F4: 4BDCB82D  bl 0x82466e20
	ctx.lr = 0x8269B5F8;
	sub_82466E20(ctx, base);
	// 8269B5F8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8269B5FC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8269B600: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8269B604: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8269B608(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8269B608 size=116
    let mut pc: u32 = 0x8269B608;
    'dispatch: loop {
        match pc {
            0x8269B608 => {
    //   block [0x8269B608..0x8269B67C)
	// 8269B608: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8269B60C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8269B610: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8269B614: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 8269B618: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 8269B61C: 390B079C  addi r8, r11, 0x79c
	ctx.r[8].s64 = ctx.r[11].s64 + 1948;
	// 8269B620: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8269B624: 392A9B44  addi r9, r10, -0x64bc
	ctx.r[9].s64 = ctx.r[10].s64 + -25788;
	// 8269B628: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269B62C: 38E00002  li r7, 2
	ctx.r[7].s64 = 2;
	// 8269B630: 38AA3C80  addi r5, r10, 0x3c80
	ctx.r[5].s64 = ctx.r[10].s64 + 15488;
	// 8269B634: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8269B638: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8269B63C: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8269B640: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8269B644: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8269B648: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8269B64C: 3D608297  lis r11, -0x7d69
	ctx.r[11].s64 = -2104033280;
	// 8269B650: 388AB7E0  addi r4, r10, -0x4820
	ctx.r[4].s64 = ctx.r[10].s64 + -18464;
	// 8269B654: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8269B658: 386B3CE0  addi r3, r11, 0x3ce0
	ctx.r[3].s64 = ctx.r[11].s64 + 15584;
	// 8269B65C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8269B660: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8269B664: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8269B668: 4BDCB7B9  bl 0x82466e20
	ctx.lr = 0x8269B66C;
	sub_82466E20(ctx, base);
	// 8269B66C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8269B670: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8269B674: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8269B678: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8269B680(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8269B680 size=112
    let mut pc: u32 = 0x8269B680;
    'dispatch: loop {
        match pc {
            0x8269B680 => {
    //   block [0x8269B680..0x8269B6F0)
	// 8269B680: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8269B684: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8269B688: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8269B68C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269B690: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 8269B694: 38AA3C80  addi r5, r10, 0x3c80
	ctx.r[5].s64 = ctx.r[10].s64 + 15488;
	// 8269B698: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8269B69C: 390B07D0  addi r8, r11, 0x7d0
	ctx.r[8].s64 = ctx.r[11].s64 + 2000;
	// 8269B6A0: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 8269B6A4: 388AB7F0  addi r4, r10, -0x4810
	ctx.r[4].s64 = ctx.r[10].s64 + -18448;
	// 8269B6A8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8269B6AC: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269B6B0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8269B6B4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8269B6B8: 386A3D10  addi r3, r10, 0x3d10
	ctx.r[3].s64 = ctx.r[10].s64 + 15632;
	// 8269B6BC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8269B6C0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8269B6C4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8269B6C8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8269B6CC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8269B6D0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8269B6D4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8269B6D8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8269B6DC: 4BDCB745  bl 0x82466e20
	ctx.lr = 0x8269B6E0;
	sub_82466E20(ctx, base);
	// 8269B6E0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8269B6E4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8269B6E8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8269B6EC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8269B6F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8269B6F0 size=112
    let mut pc: u32 = 0x8269B6F0;
    'dispatch: loop {
        match pc {
            0x8269B6F0 => {
    //   block [0x8269B6F0..0x8269B760)
	// 8269B6F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8269B6F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8269B6F8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8269B6FC: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269B700: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 8269B704: 38AA3C80  addi r5, r10, 0x3c80
	ctx.r[5].s64 = ctx.r[10].s64 + 15488;
	// 8269B708: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8269B70C: 390B0818  addi r8, r11, 0x818
	ctx.r[8].s64 = ctx.r[11].s64 + 2072;
	// 8269B710: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 8269B714: 388AB808  addi r4, r10, -0x47f8
	ctx.r[4].s64 = ctx.r[10].s64 + -18424;
	// 8269B718: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8269B71C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269B720: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8269B724: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8269B728: 386A3D40  addi r3, r10, 0x3d40
	ctx.r[3].s64 = ctx.r[10].s64 + 15680;
	// 8269B72C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8269B730: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8269B734: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8269B738: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8269B73C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8269B740: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8269B744: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8269B748: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8269B74C: 4BDCB6D5  bl 0x82466e20
	ctx.lr = 0x8269B750;
	sub_82466E20(ctx, base);
	// 8269B750: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8269B754: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8269B758: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8269B75C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8269B760(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8269B760 size=112
    let mut pc: u32 = 0x8269B760;
    'dispatch: loop {
        match pc {
            0x8269B760 => {
    //   block [0x8269B760..0x8269B7D0)
	// 8269B760: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8269B764: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8269B768: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8269B76C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269B770: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 8269B774: 38AA2B10  addi r5, r10, 0x2b10
	ctx.r[5].s64 = ctx.r[10].s64 + 11024;
	// 8269B778: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 8269B77C: 390B0860  addi r8, r11, 0x860
	ctx.r[8].s64 = ctx.r[11].s64 + 2144;
	// 8269B780: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 8269B784: 388AB39C  addi r4, r10, -0x4c64
	ctx.r[4].s64 = ctx.r[10].s64 + -19556;
	// 8269B788: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8269B78C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269B790: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8269B794: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8269B798: 386A3D70  addi r3, r10, 0x3d70
	ctx.r[3].s64 = ctx.r[10].s64 + 15728;
	// 8269B79C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8269B7A0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8269B7A4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8269B7A8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8269B7AC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8269B7B0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8269B7B4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8269B7B8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8269B7BC: 4BDCB665  bl 0x82466e20
	ctx.lr = 0x8269B7C0;
	sub_82466E20(ctx, base);
	// 8269B7C0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8269B7C4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8269B7C8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8269B7CC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8269B7D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8269B7D0 size=108
    let mut pc: u32 = 0x8269B7D0;
    'dispatch: loop {
        match pc {
            0x8269B7D0 => {
    //   block [0x8269B7D0..0x8269B83C)
	// 8269B7D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8269B7D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8269B7D8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8269B7DC: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 8269B7E0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8269B7E4: 38EB0890  addi r7, r11, 0x890
	ctx.r[7].s64 = ctx.r[11].s64 + 2192;
	// 8269B7E8: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 8269B7EC: 388AB820  addi r4, r10, -0x47e0
	ctx.r[4].s64 = ctx.r[10].s64 + -18400;
	// 8269B7F0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8269B7F4: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269B7F8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8269B7FC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8269B800: 386A3DA0  addi r3, r10, 0x3da0
	ctx.r[3].s64 = ctx.r[10].s64 + 15776;
	// 8269B804: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8269B808: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8269B80C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8269B810: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8269B814: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8269B818: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8269B81C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8269B820: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8269B824: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8269B828: 4BDCB5F9  bl 0x82466e20
	ctx.lr = 0x8269B82C;
	sub_82466E20(ctx, base);
	// 8269B82C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8269B830: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8269B834: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8269B838: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8269B840(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8269B840 size=112
    let mut pc: u32 = 0x8269B840;
    'dispatch: loop {
        match pc {
            0x8269B840 => {
    //   block [0x8269B840..0x8269B8B0)
	// 8269B840: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8269B844: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8269B848: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8269B84C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269B850: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 8269B854: 38AA3C80  addi r5, r10, 0x3c80
	ctx.r[5].s64 = ctx.r[10].s64 + 15488;
	// 8269B858: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8269B85C: 390B08D8  addi r8, r11, 0x8d8
	ctx.r[8].s64 = ctx.r[11].s64 + 2264;
	// 8269B860: 39200006  li r9, 6
	ctx.r[9].s64 = 6;
	// 8269B864: 388AB844  addi r4, r10, -0x47bc
	ctx.r[4].s64 = ctx.r[10].s64 + -18364;
	// 8269B868: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8269B86C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269B870: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8269B874: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8269B878: 386A3DD0  addi r3, r10, 0x3dd0
	ctx.r[3].s64 = ctx.r[10].s64 + 15824;
	// 8269B87C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8269B880: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8269B884: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8269B888: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8269B88C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8269B890: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8269B894: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8269B898: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8269B89C: 4BDCB585  bl 0x82466e20
	ctx.lr = 0x8269B8A0;
	sub_82466E20(ctx, base);
	// 8269B8A0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8269B8A4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8269B8A8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8269B8AC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8269B8B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8269B8B0 size=116
    let mut pc: u32 = 0x8269B8B0;
    'dispatch: loop {
        match pc {
            0x8269B8B0 => {
    //   block [0x8269B8B0..0x8269B924)
	// 8269B8B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8269B8B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8269B8B8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8269B8BC: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 8269B8C0: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269B8C4: 392B9B80  addi r9, r11, -0x6480
	ctx.r[9].s64 = ctx.r[11].s64 + -25728;
	// 8269B8C8: 38AA3C80  addi r5, r10, 0x3c80
	ctx.r[5].s64 = ctx.r[10].s64 + 15488;
	// 8269B8CC: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8269B8D0: 38E90014  addi r7, r9, 0x14
	ctx.r[7].s64 = ctx.r[9].s64 + 20;
	// 8269B8D4: 38C00006  li r6, 6
	ctx.r[6].s64 = 6;
	// 8269B8D8: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 8269B8DC: 388AB85C  addi r4, r10, -0x47a4
	ctx.r[4].s64 = ctx.r[10].s64 + -18340;
	// 8269B8E0: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8269B8E4: 396B0968  addi r11, r11, 0x968
	ctx.r[11].s64 = ctx.r[11].s64 + 2408;
	// 8269B8E8: 90E10064  stw r7, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[7].u32 ) };
	// 8269B8EC: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269B8F0: 90C1005C  stw r6, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[6].u32 ) };
	// 8269B8F4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8269B8F8: 386A3E00  addi r3, r10, 0x3e00
	ctx.r[3].s64 = ctx.r[10].s64 + 15872;
	// 8269B8FC: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8269B900: 91010074  stw r8, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[8].u32 ) };
	// 8269B904: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8269B908: 9101006C  stw r8, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[8].u32 ) };
	// 8269B90C: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8269B910: 4BDCB511  bl 0x82466e20
	ctx.lr = 0x8269B914;
	sub_82466E20(ctx, base);
	// 8269B914: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8269B918: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8269B91C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8269B920: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8269B928(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8269B928 size=112
    let mut pc: u32 = 0x8269B928;
    'dispatch: loop {
        match pc {
            0x8269B928 => {
    //   block [0x8269B928..0x8269B998)
	// 8269B928: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8269B92C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8269B930: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8269B934: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269B938: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 8269B93C: 38AA3C80  addi r5, r10, 0x3c80
	ctx.r[5].s64 = ctx.r[10].s64 + 15488;
	// 8269B940: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8269B944: 390B09F8  addi r8, r11, 0x9f8
	ctx.r[8].s64 = ctx.r[11].s64 + 2552;
	// 8269B948: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8269B94C: 388AB8E0  addi r4, r10, -0x4720
	ctx.r[4].s64 = ctx.r[10].s64 + -18208;
	// 8269B950: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8269B954: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269B958: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8269B95C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8269B960: 386A3E30  addi r3, r10, 0x3e30
	ctx.r[3].s64 = ctx.r[10].s64 + 15920;
	// 8269B964: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8269B968: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8269B96C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8269B970: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8269B974: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8269B978: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8269B97C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8269B980: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8269B984: 4BDCB49D  bl 0x82466e20
	ctx.lr = 0x8269B988;
	sub_82466E20(ctx, base);
	// 8269B988: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8269B98C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8269B990: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8269B994: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8269B998(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8269B998 size=24
    let mut pc: u32 = 0x8269B998;
    'dispatch: loop {
        match pc {
            0x8269B998 => {
    //   block [0x8269B998..0x8269B9B0)
	// 8269B998: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 8269B99C: 3D40827C  lis r10, -0x7d84
	ctx.r[10].s64 = -2105802752;
	// 8269B9A0: 394A6EB8  addi r10, r10, 0x6eb8
	ctx.r[10].s64 = ctx.r[10].s64 + 28344;
	// 8269B9A4: 816B0EB0  lwz r11, 0xeb0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(3760 as u32) ) } as u64;
	// 8269B9A8: 916A0020  stw r11, 0x20(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 8269B9AC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8269B9B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8269B9B0 size=116
    let mut pc: u32 = 0x8269B9B0;
    'dispatch: loop {
        match pc {
            0x8269B9B0 => {
    //   block [0x8269B9B0..0x8269BA24)
	// 8269B9B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8269B9B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8269B9B8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8269B9BC: 3D40827C  lis r10, -0x7d84
	ctx.r[10].s64 = -2105802752;
	// 8269B9C0: 38E00007  li r7, 7
	ctx.r[7].s64 = 7;
	// 8269B9C4: 390A6EB8  addi r8, r10, 0x6eb8
	ctx.r[8].s64 = ctx.r[10].s64 + 28344;
	// 8269B9C8: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269B9CC: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 8269B9D0: 38AA3C80  addi r5, r10, 0x3c80
	ctx.r[5].s64 = ctx.r[10].s64 + 15488;
	// 8269B9D4: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8269B9D8: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8269B9DC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8269B9E0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8269B9E4: 388AB8F0  addi r4, r10, -0x4710
	ctx.r[4].s64 = ctx.r[10].s64 + -18192;
	// 8269B9E8: 396B9BB0  addi r11, r11, -0x6450
	ctx.r[11].s64 = ctx.r[11].s64 + -25680;
	// 8269B9EC: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269B9F0: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8269B9F4: 386A3E60  addi r3, r10, 0x3e60
	ctx.r[3].s64 = ctx.r[10].s64 + 15968;
	// 8269B9F8: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 8269B9FC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8269BA00: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 8269BA04: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8269BA08: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8269BA0C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8269BA10: 4BDCB411  bl 0x82466e20
	ctx.lr = 0x8269BA14;
	sub_82466E20(ctx, base);
	// 8269BA14: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8269BA18: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8269BA1C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8269BA20: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8269BA28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8269BA28 size=112
    let mut pc: u32 = 0x8269BA28;
    'dispatch: loop {
        match pc {
            0x8269BA28 => {
    //   block [0x8269BA28..0x8269BA98)
	// 8269BA28: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8269BA2C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8269BA30: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8269BA34: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269BA38: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 8269BA3C: 38AA3950  addi r5, r10, 0x3950
	ctx.r[5].s64 = ctx.r[10].s64 + 14672;
	// 8269BA40: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8269BA44: 390B0A10  addi r8, r11, 0xa10
	ctx.r[8].s64 = ctx.r[11].s64 + 2576;
	// 8269BA48: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 8269BA4C: 388AB530  addi r4, r10, -0x4ad0
	ctx.r[4].s64 = ctx.r[10].s64 + -19152;
	// 8269BA50: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8269BA54: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269BA58: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8269BA5C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8269BA60: 386A3E90  addi r3, r10, 0x3e90
	ctx.r[3].s64 = ctx.r[10].s64 + 16016;
	// 8269BA64: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8269BA68: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8269BA6C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8269BA70: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8269BA74: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8269BA78: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8269BA7C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8269BA80: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8269BA84: 4BDCB39D  bl 0x82466e20
	ctx.lr = 0x8269BA88;
	sub_82466E20(ctx, base);
	// 8269BA88: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8269BA8C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8269BA90: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8269BA94: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8269BA98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8269BA98 size=100
    let mut pc: u32 = 0x8269BA98;
    'dispatch: loop {
        match pc {
            0x8269BA98 => {
    //   block [0x8269BA98..0x8269BAFC)
	// 8269BA98: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8269BA9C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8269BAA0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8269BAA4: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269BAA8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8269BAAC: 38AA3F20  addi r5, r10, 0x3f20
	ctx.r[5].s64 = ctx.r[10].s64 + 16160;
	// 8269BAB0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8269BAB4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8269BAB8: 388AB5FC  addi r4, r10, -0x4a04
	ctx.r[4].s64 = ctx.r[10].s64 + -18948;
	// 8269BABC: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269BAC0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8269BAC4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8269BAC8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8269BACC: 386A3EC0  addi r3, r10, 0x3ec0
	ctx.r[3].s64 = ctx.r[10].s64 + 16064;
	// 8269BAD0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8269BAD4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8269BAD8: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8269BADC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8269BAE0: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8269BAE4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8269BAE8: 4BDCB339  bl 0x82466e20
	ctx.lr = 0x8269BAEC;
	sub_82466E20(ctx, base);
	// 8269BAEC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8269BAF0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8269BAF4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8269BAF8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8269BB00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8269BB00 size=28
    let mut pc: u32 = 0x8269BB00;
    'dispatch: loop {
        match pc {
            0x8269BB00 => {
    //   block [0x8269BB00..0x8269BB1C)
	// 8269BB00: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 8269BB04: 3D40827C  lis r10, -0x7d84
	ctx.r[10].s64 = -2105802752;
	// 8269BB08: 394A6F60  addi r10, r10, 0x6f60
	ctx.r[10].s64 = ctx.r[10].s64 + 28512;
	// 8269BB0C: 816B0A58  lwz r11, 0xa58(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2648 as u32) ) } as u64;
	// 8269BB10: 916A0068  stw r11, 0x68(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(104 as u32), ctx.r[11].u32 ) };
	// 8269BB14: 916A0080  stw r11, 0x80(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(128 as u32), ctx.r[11].u32 ) };
	// 8269BB18: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8269BB20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8269BB20 size=112
    let mut pc: u32 = 0x8269BB20;
    'dispatch: loop {
        match pc {
            0x8269BB20 => {
    //   block [0x8269BB20..0x8269BB90)
	// 8269BB20: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8269BB24: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8269BB28: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8269BB2C: 3D40827C  lis r10, -0x7d84
	ctx.r[10].s64 = -2105802752;
	// 8269BB30: 3900000F  li r8, 0xf
	ctx.r[8].s64 = 15;
	// 8269BB34: 38EA6F60  addi r7, r10, 0x6f60
	ctx.r[7].s64 = ctx.r[10].s64 + 28512;
	// 8269BB38: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8269BB3C: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 8269BB40: 388AB648  addi r4, r10, -0x49b8
	ctx.r[4].s64 = ctx.r[10].s64 + -18872;
	// 8269BB44: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8269BB48: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8269BB4C: 396B9C38  addi r11, r11, -0x63c8
	ctx.r[11].s64 = ctx.r[11].s64 + -25544;
	// 8269BB50: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8269BB54: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269BB58: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8269BB5C: 386A3EF0  addi r3, r10, 0x3ef0
	ctx.r[3].s64 = ctx.r[10].s64 + 16112;
	// 8269BB60: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8269BB64: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 8269BB68: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8269BB6C: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 8269BB70: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8269BB74: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8269BB78: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8269BB7C: 4BDCB2A5  bl 0x82466e20
	ctx.lr = 0x8269BB80;
	sub_82466E20(ctx, base);
	// 8269BB80: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8269BB84: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8269BB88: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8269BB8C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8269BB90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8269BB90 size=24
    let mut pc: u32 = 0x8269BB90;
    'dispatch: loop {
        match pc {
            0x8269BB90 => {
    //   block [0x8269BB90..0x8269BBA8)
	// 8269BB90: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 8269BB94: 3D40827C  lis r10, -0x7d84
	ctx.r[10].s64 = -2105802752;
	// 8269BB98: 394A70C8  addi r10, r10, 0x70c8
	ctx.r[10].s64 = ctx.r[10].s64 + 28872;
	// 8269BB9C: 816B0EB0  lwz r11, 0xeb0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(3760 as u32) ) } as u64;
	// 8269BBA0: 916A0068  stw r11, 0x68(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(104 as u32), ctx.r[11].u32 ) };
	// 8269BBA4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8269BBA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8269BBA8 size=116
    let mut pc: u32 = 0x8269BBA8;
    'dispatch: loop {
        match pc {
            0x8269BBA8 => {
    //   block [0x8269BBA8..0x8269BC1C)
	// 8269BBA8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8269BBAC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8269BBB0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8269BBB4: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 8269BBB8: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269BBBC: 392B9C10  addi r9, r11, -0x63f0
	ctx.r[9].s64 = ctx.r[11].s64 + -25584;
	// 8269BBC0: 38AA3950  addi r5, r10, 0x3950
	ctx.r[5].s64 = ctx.r[10].s64 + 14672;
	// 8269BBC4: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8269BBC8: 38E90068  addi r7, r9, 0x68
	ctx.r[7].s64 = ctx.r[9].s64 + 104;
	// 8269BBCC: 38C00007  li r6, 7
	ctx.r[6].s64 = 7;
	// 8269BBD0: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 8269BBD4: 388AB65C  addi r4, r10, -0x49a4
	ctx.r[4].s64 = ctx.r[10].s64 + -18852;
	// 8269BBD8: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8269BBDC: 396B70C8  addi r11, r11, 0x70c8
	ctx.r[11].s64 = ctx.r[11].s64 + 28872;
	// 8269BBE0: 90E10064  stw r7, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[7].u32 ) };
	// 8269BBE4: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269BBE8: 90C1005C  stw r6, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[6].u32 ) };
	// 8269BBEC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8269BBF0: 386A3F20  addi r3, r10, 0x3f20
	ctx.r[3].s64 = ctx.r[10].s64 + 16160;
	// 8269BBF4: 39400002  li r10, 2
	ctx.r[10].s64 = 2;
	// 8269BBF8: 91010074  stw r8, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[8].u32 ) };
	// 8269BBFC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8269BC00: 9101006C  stw r8, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[8].u32 ) };
	// 8269BC04: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8269BC08: 4BDCB219  bl 0x82466e20
	ctx.lr = 0x8269BC0C;
	sub_82466E20(ctx, base);
	// 8269BC0C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8269BC10: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8269BC14: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8269BC18: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8269BC20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8269BC20 size=112
    let mut pc: u32 = 0x8269BC20;
    'dispatch: loop {
        match pc {
            0x8269BC20 => {
    //   block [0x8269BC20..0x8269BC90)
	// 8269BC20: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8269BC24: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8269BC28: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8269BC2C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269BC30: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 8269BC34: 38AA4010  addi r5, r10, 0x4010
	ctx.r[5].s64 = ctx.r[10].s64 + 16400;
	// 8269BC38: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8269BC3C: 390B0A60  addi r8, r11, 0xa60
	ctx.r[8].s64 = ctx.r[11].s64 + 2656;
	// 8269BC40: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 8269BC44: 388ABA0C  addi r4, r10, -0x45f4
	ctx.r[4].s64 = ctx.r[10].s64 + -17908;
	// 8269BC48: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8269BC4C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269BC50: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8269BC54: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8269BC58: 386A3F50  addi r3, r10, 0x3f50
	ctx.r[3].s64 = ctx.r[10].s64 + 16208;
	// 8269BC5C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8269BC60: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8269BC64: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8269BC68: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8269BC6C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8269BC70: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8269BC74: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8269BC78: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8269BC7C: 4BDCB1A5  bl 0x82466e20
	ctx.lr = 0x8269BC80;
	sub_82466E20(ctx, base);
	// 8269BC80: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8269BC84: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8269BC88: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8269BC8C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8269BC90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8269BC90 size=112
    let mut pc: u32 = 0x8269BC90;
    'dispatch: loop {
        match pc {
            0x8269BC90 => {
    //   block [0x8269BC90..0x8269BD00)
	// 8269BC90: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8269BC94: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8269BC98: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8269BC9C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269BCA0: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 8269BCA4: 38AA2B10  addi r5, r10, 0x2b10
	ctx.r[5].s64 = ctx.r[10].s64 + 11024;
	// 8269BCA8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8269BCAC: 390B0A90  addi r8, r11, 0xa90
	ctx.r[8].s64 = ctx.r[11].s64 + 2704;
	// 8269BCB0: 39200006  li r9, 6
	ctx.r[9].s64 = 6;
	// 8269BCB4: 388AB718  addi r4, r10, -0x48e8
	ctx.r[4].s64 = ctx.r[10].s64 + -18664;
	// 8269BCB8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8269BCBC: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269BCC0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8269BCC4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8269BCC8: 386A3F80  addi r3, r10, 0x3f80
	ctx.r[3].s64 = ctx.r[10].s64 + 16256;
	// 8269BCCC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8269BCD0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8269BCD4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8269BCD8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8269BCDC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8269BCE0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8269BCE4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8269BCE8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8269BCEC: 4BDCB135  bl 0x82466e20
	ctx.lr = 0x8269BCF0;
	sub_82466E20(ctx, base);
	// 8269BCF0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8269BCF4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8269BCF8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8269BCFC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8269BD00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8269BD00 size=112
    let mut pc: u32 = 0x8269BD00;
    'dispatch: loop {
        match pc {
            0x8269BD00 => {
    //   block [0x8269BD00..0x8269BD70)
	// 8269BD00: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8269BD04: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8269BD08: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8269BD0C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269BD10: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 8269BD14: 38AA3F20  addi r5, r10, 0x3f20
	ctx.r[5].s64 = ctx.r[10].s64 + 16160;
	// 8269BD18: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8269BD1C: 390B0B20  addi r8, r11, 0xb20
	ctx.r[8].s64 = ctx.r[11].s64 + 2848;
	// 8269BD20: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8269BD24: 388AB73C  addi r4, r10, -0x48c4
	ctx.r[4].s64 = ctx.r[10].s64 + -18628;
	// 8269BD28: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8269BD2C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269BD30: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8269BD34: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8269BD38: 386A3FB0  addi r3, r10, 0x3fb0
	ctx.r[3].s64 = ctx.r[10].s64 + 16304;
	// 8269BD3C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8269BD40: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8269BD44: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8269BD48: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8269BD4C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8269BD50: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8269BD54: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8269BD58: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8269BD5C: 4BDCB0C5  bl 0x82466e20
	ctx.lr = 0x8269BD60;
	sub_82466E20(ctx, base);
	// 8269BD60: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8269BD64: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8269BD68: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8269BD6C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8269BD70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8269BD70 size=100
    let mut pc: u32 = 0x8269BD70;
    'dispatch: loop {
        match pc {
            0x8269BD70 => {
    //   block [0x8269BD70..0x8269BDD4)
	// 8269BD70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8269BD74: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8269BD78: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8269BD7C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269BD80: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8269BD84: 38AA38C0  addi r5, r10, 0x38c0
	ctx.r[5].s64 = ctx.r[10].s64 + 14528;
	// 8269BD88: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8269BD8C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8269BD90: 388AB904  addi r4, r10, -0x46fc
	ctx.r[4].s64 = ctx.r[10].s64 + -18172;
	// 8269BD94: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269BD98: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8269BD9C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8269BDA0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8269BDA4: 386A3FE0  addi r3, r10, 0x3fe0
	ctx.r[3].s64 = ctx.r[10].s64 + 16352;
	// 8269BDA8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8269BDAC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8269BDB0: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8269BDB4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8269BDB8: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8269BDBC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8269BDC0: 4BDCB061  bl 0x82466e20
	ctx.lr = 0x8269BDC4;
	sub_82466E20(ctx, base);
	// 8269BDC4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8269BDC8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8269BDCC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8269BDD0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8269BDD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8269BDD8 size=100
    let mut pc: u32 = 0x8269BDD8;
    'dispatch: loop {
        match pc {
            0x8269BDD8 => {
    //   block [0x8269BDD8..0x8269BE3C)
	// 8269BDD8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8269BDDC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8269BDE0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8269BDE4: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269BDE8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8269BDEC: 38AA38C0  addi r5, r10, 0x38c0
	ctx.r[5].s64 = ctx.r[10].s64 + 14528;
	// 8269BDF0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8269BDF4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8269BDF8: 388AB918  addi r4, r10, -0x46e8
	ctx.r[4].s64 = ctx.r[10].s64 + -18152;
	// 8269BDFC: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269BE00: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8269BE04: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8269BE08: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8269BE0C: 386A4010  addi r3, r10, 0x4010
	ctx.r[3].s64 = ctx.r[10].s64 + 16400;
	// 8269BE10: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8269BE14: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8269BE18: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8269BE1C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8269BE20: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8269BE24: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8269BE28: 4BDCAFF9  bl 0x82466e20
	ctx.lr = 0x8269BE2C;
	sub_82466E20(ctx, base);
	// 8269BE2C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8269BE30: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8269BE34: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8269BE38: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8269BE40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8269BE40 size=112
    let mut pc: u32 = 0x8269BE40;
    'dispatch: loop {
        match pc {
            0x8269BE40 => {
    //   block [0x8269BE40..0x8269BEB0)
	// 8269BE40: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8269BE44: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8269BE48: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8269BE4C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269BE50: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 8269BE54: 38AA3FE0  addi r5, r10, 0x3fe0
	ctx.r[5].s64 = ctx.r[10].s64 + 16352;
	// 8269BE58: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8269BE5C: 390B0B38  addi r8, r11, 0xb38
	ctx.r[8].s64 = ctx.r[11].s64 + 2872;
	// 8269BE60: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 8269BE64: 388AB92C  addi r4, r10, -0x46d4
	ctx.r[4].s64 = ctx.r[10].s64 + -18132;
	// 8269BE68: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8269BE6C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269BE70: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8269BE74: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8269BE78: 386A4040  addi r3, r10, 0x4040
	ctx.r[3].s64 = ctx.r[10].s64 + 16448;
	// 8269BE7C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8269BE80: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8269BE84: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8269BE88: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8269BE8C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8269BE90: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8269BE94: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8269BE98: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8269BE9C: 4BDCAF85  bl 0x82466e20
	ctx.lr = 0x8269BEA0;
	sub_82466E20(ctx, base);
	// 8269BEA0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8269BEA4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8269BEA8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8269BEAC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8269BEB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8269BEB0 size=112
    let mut pc: u32 = 0x8269BEB0;
    'dispatch: loop {
        match pc {
            0x8269BEB0 => {
    //   block [0x8269BEB0..0x8269BF20)
	// 8269BEB0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8269BEB4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8269BEB8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8269BEBC: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269BEC0: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 8269BEC4: 38AA3FE0  addi r5, r10, 0x3fe0
	ctx.r[5].s64 = ctx.r[10].s64 + 16352;
	// 8269BEC8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8269BECC: 390B0B80  addi r8, r11, 0xb80
	ctx.r[8].s64 = ctx.r[11].s64 + 2944;
	// 8269BED0: 39200008  li r9, 8
	ctx.r[9].s64 = 8;
	// 8269BED4: 388AB93C  addi r4, r10, -0x46c4
	ctx.r[4].s64 = ctx.r[10].s64 + -18116;
	// 8269BED8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8269BEDC: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269BEE0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8269BEE4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8269BEE8: 386A4070  addi r3, r10, 0x4070
	ctx.r[3].s64 = ctx.r[10].s64 + 16496;
	// 8269BEEC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8269BEF0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8269BEF4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8269BEF8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8269BEFC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8269BF00: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8269BF04: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8269BF08: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8269BF0C: 4BDCAF15  bl 0x82466e20
	ctx.lr = 0x8269BF10;
	sub_82466E20(ctx, base);
	// 8269BF10: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8269BF14: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8269BF18: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8269BF1C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8269BF20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8269BF20 size=112
    let mut pc: u32 = 0x8269BF20;
    'dispatch: loop {
        match pc {
            0x8269BF20 => {
    //   block [0x8269BF20..0x8269BF90)
	// 8269BF20: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8269BF24: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8269BF28: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8269BF2C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269BF30: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 8269BF34: 38AA4070  addi r5, r10, 0x4070
	ctx.r[5].s64 = ctx.r[10].s64 + 16496;
	// 8269BF38: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8269BF3C: 390B0C40  addi r8, r11, 0xc40
	ctx.r[8].s64 = ctx.r[11].s64 + 3136;
	// 8269BF40: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 8269BF44: 388AB958  addi r4, r10, -0x46a8
	ctx.r[4].s64 = ctx.r[10].s64 + -18088;
	// 8269BF48: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8269BF4C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269BF50: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8269BF54: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8269BF58: 386A40A0  addi r3, r10, 0x40a0
	ctx.r[3].s64 = ctx.r[10].s64 + 16544;
	// 8269BF5C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8269BF60: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8269BF64: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8269BF68: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8269BF6C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8269BF70: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8269BF74: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8269BF78: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8269BF7C: 4BDCAEA5  bl 0x82466e20
	ctx.lr = 0x8269BF80;
	sub_82466E20(ctx, base);
	// 8269BF80: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8269BF84: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8269BF88: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8269BF8C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8269BF90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8269BF90 size=112
    let mut pc: u32 = 0x8269BF90;
    'dispatch: loop {
        match pc {
            0x8269BF90 => {
    //   block [0x8269BF90..0x8269C000)
	// 8269BF90: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8269BF94: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8269BF98: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8269BF9C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269BFA0: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 8269BFA4: 38AA3BC0  addi r5, r10, 0x3bc0
	ctx.r[5].s64 = ctx.r[10].s64 + 15296;
	// 8269BFA8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8269BFAC: 390B0C70  addi r8, r11, 0xc70
	ctx.r[8].s64 = ctx.r[11].s64 + 3184;
	// 8269BFB0: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 8269BFB4: 388AB97C  addi r4, r10, -0x4684
	ctx.r[4].s64 = ctx.r[10].s64 + -18052;
	// 8269BFB8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8269BFBC: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269BFC0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8269BFC4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8269BFC8: 386A40D0  addi r3, r10, 0x40d0
	ctx.r[3].s64 = ctx.r[10].s64 + 16592;
	// 8269BFCC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8269BFD0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8269BFD4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8269BFD8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8269BFDC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8269BFE0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8269BFE4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8269BFE8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8269BFEC: 4BDCAE35  bl 0x82466e20
	ctx.lr = 0x8269BFF0;
	sub_82466E20(ctx, base);
	// 8269BFF0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8269BFF4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8269BFF8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8269BFFC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8269C000(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8269C000 size=112
    let mut pc: u32 = 0x8269C000;
    'dispatch: loop {
        match pc {
            0x8269C000 => {
    //   block [0x8269C000..0x8269C070)
	// 8269C000: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8269C004: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8269C008: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8269C00C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269C010: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 8269C014: 38AA3950  addi r5, r10, 0x3950
	ctx.r[5].s64 = ctx.r[10].s64 + 14672;
	// 8269C018: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8269C01C: 390B0CB8  addi r8, r11, 0xcb8
	ctx.r[8].s64 = ctx.r[11].s64 + 3256;
	// 8269C020: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 8269C024: 388AB9A0  addi r4, r10, -0x4660
	ctx.r[4].s64 = ctx.r[10].s64 + -18016;
	// 8269C028: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8269C02C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269C030: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8269C034: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8269C038: 386A4100  addi r3, r10, 0x4100
	ctx.r[3].s64 = ctx.r[10].s64 + 16640;
	// 8269C03C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8269C040: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8269C044: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8269C048: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8269C04C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8269C050: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8269C054: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8269C058: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8269C05C: 4BDCADC5  bl 0x82466e20
	ctx.lr = 0x8269C060;
	sub_82466E20(ctx, base);
	// 8269C060: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8269C064: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8269C068: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8269C06C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8269C070(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8269C070 size=112
    let mut pc: u32 = 0x8269C070;
    'dispatch: loop {
        match pc {
            0x8269C070 => {
    //   block [0x8269C070..0x8269C0E0)
	// 8269C070: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8269C074: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8269C078: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8269C07C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269C080: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 8269C084: 38AA38C0  addi r5, r10, 0x38c0
	ctx.r[5].s64 = ctx.r[10].s64 + 14528;
	// 8269C088: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8269C08C: 390B0D00  addi r8, r11, 0xd00
	ctx.r[8].s64 = ctx.r[11].s64 + 3328;
	// 8269C090: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 8269C094: 388AB9C4  addi r4, r10, -0x463c
	ctx.r[4].s64 = ctx.r[10].s64 + -17980;
	// 8269C098: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8269C09C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269C0A0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8269C0A4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8269C0A8: 386A4130  addi r3, r10, 0x4130
	ctx.r[3].s64 = ctx.r[10].s64 + 16688;
	// 8269C0AC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8269C0B0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8269C0B4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8269C0B8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8269C0BC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8269C0C0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8269C0C4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8269C0C8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8269C0CC: 4BDCAD55  bl 0x82466e20
	ctx.lr = 0x8269C0D0;
	sub_82466E20(ctx, base);
	// 8269C0D0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8269C0D4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8269C0D8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8269C0DC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8269C0E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8269C0E0 size=112
    let mut pc: u32 = 0x8269C0E0;
    'dispatch: loop {
        match pc {
            0x8269C0E0 => {
    //   block [0x8269C0E0..0x8269C150)
	// 8269C0E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8269C0E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8269C0E8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8269C0EC: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269C0F0: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 8269C0F4: 38AA3C80  addi r5, r10, 0x3c80
	ctx.r[5].s64 = ctx.r[10].s64 + 15488;
	// 8269C0F8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8269C0FC: 390B0D30  addi r8, r11, 0xd30
	ctx.r[8].s64 = ctx.r[11].s64 + 3376;
	// 8269C100: 39200005  li r9, 5
	ctx.r[9].s64 = 5;
	// 8269C104: 388AB9D0  addi r4, r10, -0x4630
	ctx.r[4].s64 = ctx.r[10].s64 + -17968;
	// 8269C108: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8269C10C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269C110: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8269C114: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8269C118: 386A4160  addi r3, r10, 0x4160
	ctx.r[3].s64 = ctx.r[10].s64 + 16736;
	// 8269C11C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8269C120: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8269C124: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8269C128: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8269C12C: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 8269C130: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8269C134: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8269C138: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8269C13C: 4BDCACE5  bl 0x82466e20
	ctx.lr = 0x8269C140;
	sub_82466E20(ctx, base);
	// 8269C140: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8269C144: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8269C148: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8269C14C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8269C150(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8269C150 size=108
    let mut pc: u32 = 0x8269C150;
    'dispatch: loop {
        match pc {
            0x8269C150 => {
    //   block [0x8269C150..0x8269C1BC)
	// 8269C150: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8269C154: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8269C158: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8269C15C: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 8269C160: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8269C164: 38EB0DA8  addi r7, r11, 0xda8
	ctx.r[7].s64 = ctx.r[11].s64 + 3496;
	// 8269C168: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 8269C16C: 388AB9E4  addi r4, r10, -0x461c
	ctx.r[4].s64 = ctx.r[10].s64 + -17948;
	// 8269C170: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8269C174: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269C178: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8269C17C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8269C180: 386A4190  addi r3, r10, 0x4190
	ctx.r[3].s64 = ctx.r[10].s64 + 16784;
	// 8269C184: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8269C188: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8269C18C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8269C190: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8269C194: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8269C198: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8269C19C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8269C1A0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8269C1A4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8269C1A8: 4BDCAC79  bl 0x82466e20
	ctx.lr = 0x8269C1AC;
	sub_82466E20(ctx, base);
	// 8269C1AC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8269C1B0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8269C1B4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8269C1B8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8269C1C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8269C1C0 size=112
    let mut pc: u32 = 0x8269C1C0;
    'dispatch: loop {
        match pc {
            0x8269C1C0 => {
    //   block [0x8269C1C0..0x8269C230)
	// 8269C1C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8269C1C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8269C1C8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8269C1CC: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269C1D0: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 8269C1D4: 38AA38C0  addi r5, r10, 0x38c0
	ctx.r[5].s64 = ctx.r[10].s64 + 14528;
	// 8269C1D8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8269C1DC: 390B0DD8  addi r8, r11, 0xdd8
	ctx.r[8].s64 = ctx.r[11].s64 + 3544;
	// 8269C1E0: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 8269C1E4: 388AB9F8  addi r4, r10, -0x4608
	ctx.r[4].s64 = ctx.r[10].s64 + -17928;
	// 8269C1E8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8269C1EC: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269C1F0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8269C1F4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8269C1F8: 386A41C0  addi r3, r10, 0x41c0
	ctx.r[3].s64 = ctx.r[10].s64 + 16832;
	// 8269C1FC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8269C200: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8269C204: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8269C208: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8269C20C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8269C210: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8269C214: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8269C218: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8269C21C: 4BDCAC05  bl 0x82466e20
	ctx.lr = 0x8269C220;
	sub_82466E20(ctx, base);
	// 8269C220: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8269C224: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8269C228: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8269C22C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8269C230(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8269C230 size=100
    let mut pc: u32 = 0x8269C230;
    'dispatch: loop {
        match pc {
            0x8269C230 => {
    //   block [0x8269C230..0x8269C294)
	// 8269C230: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8269C234: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8269C238: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8269C23C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269C240: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8269C244: 38AA38C0  addi r5, r10, 0x38c0
	ctx.r[5].s64 = ctx.r[10].s64 + 14528;
	// 8269C248: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8269C24C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8269C250: 388ABA20  addi r4, r10, -0x45e0
	ctx.r[4].s64 = ctx.r[10].s64 + -17888;
	// 8269C254: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269C258: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8269C25C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8269C260: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8269C264: 386A41F0  addi r3, r10, 0x41f0
	ctx.r[3].s64 = ctx.r[10].s64 + 16880;
	// 8269C268: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8269C26C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8269C270: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8269C274: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8269C278: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8269C27C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8269C280: 4BDCABA1  bl 0x82466e20
	ctx.lr = 0x8269C284;
	sub_82466E20(ctx, base);
	// 8269C284: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8269C288: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8269C28C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8269C290: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8269C298(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8269C298 size=112
    let mut pc: u32 = 0x8269C298;
    'dispatch: loop {
        match pc {
            0x8269C298 => {
    //   block [0x8269C298..0x8269C308)
	// 8269C298: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8269C29C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8269C2A0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8269C2A4: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269C2A8: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 8269C2AC: 38AA38C0  addi r5, r10, 0x38c0
	ctx.r[5].s64 = ctx.r[10].s64 + 14528;
	// 8269C2B0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8269C2B4: 390B0E08  addi r8, r11, 0xe08
	ctx.r[8].s64 = ctx.r[11].s64 + 3592;
	// 8269C2B8: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 8269C2BC: 388ABA38  addi r4, r10, -0x45c8
	ctx.r[4].s64 = ctx.r[10].s64 + -17864;
	// 8269C2C0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8269C2C4: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269C2C8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8269C2CC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8269C2D0: 386A4220  addi r3, r10, 0x4220
	ctx.r[3].s64 = ctx.r[10].s64 + 16928;
	// 8269C2D4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8269C2D8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8269C2DC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8269C2E0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8269C2E4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8269C2E8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8269C2EC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8269C2F0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8269C2F4: 4BDCAB2D  bl 0x82466e20
	ctx.lr = 0x8269C2F8;
	sub_82466E20(ctx, base);
	// 8269C2F8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8269C2FC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8269C300: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8269C304: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8269C308(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8269C308 size=96
    let mut pc: u32 = 0x8269C308;
    'dispatch: loop {
        match pc {
            0x8269C308 => {
    //   block [0x8269C308..0x8269C368)
	// 8269C308: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8269C30C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8269C310: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8269C314: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8269C318: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8269C31C: 388ABA4C  addi r4, r10, -0x45b4
	ctx.r[4].s64 = ctx.r[10].s64 + -17844;
	// 8269C320: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269C324: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8269C328: 386A4250  addi r3, r10, 0x4250
	ctx.r[3].s64 = ctx.r[10].s64 + 16976;
	// 8269C32C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8269C330: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8269C334: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 8269C338: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8269C33C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8269C340: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8269C344: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8269C348: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8269C34C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8269C350: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8269C354: 4BDCAACD  bl 0x82466e20
	ctx.lr = 0x8269C358;
	sub_82466E20(ctx, base);
	// 8269C358: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8269C35C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8269C360: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8269C364: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8269C368(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8269C368 size=108
    let mut pc: u32 = 0x8269C368;
    'dispatch: loop {
        match pc {
            0x8269C368 => {
    //   block [0x8269C368..0x8269C3D4)
	// 8269C368: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8269C36C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8269C370: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8269C374: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 8269C378: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8269C37C: 38EB0E50  addi r7, r11, 0xe50
	ctx.r[7].s64 = ctx.r[11].s64 + 3664;
	// 8269C380: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 8269C384: 388ABA68  addi r4, r10, -0x4598
	ctx.r[4].s64 = ctx.r[10].s64 + -17816;
	// 8269C388: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8269C38C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269C390: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8269C394: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8269C398: 386A4280  addi r3, r10, 0x4280
	ctx.r[3].s64 = ctx.r[10].s64 + 17024;
	// 8269C39C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8269C3A0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8269C3A4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8269C3A8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8269C3AC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8269C3B0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8269C3B4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8269C3B8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8269C3BC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8269C3C0: 4BDCAA61  bl 0x82466e20
	ctx.lr = 0x8269C3C4;
	sub_82466E20(ctx, base);
	// 8269C3C4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8269C3C8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8269C3CC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8269C3D0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8269C3D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8269C3D8 size=100
    let mut pc: u32 = 0x8269C3D8;
    'dispatch: loop {
        match pc {
            0x8269C3D8 => {
    //   block [0x8269C3D8..0x8269C43C)
	// 8269C3D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8269C3DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8269C3E0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8269C3E4: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 8269C3E8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8269C3EC: 392A9CE8  addi r9, r10, -0x6318
	ctx.r[9].s64 = ctx.r[10].s64 + -25368;
	// 8269C3F0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8269C3F4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8269C3F8: 388ABA80  addi r4, r10, -0x4580
	ctx.r[4].s64 = ctx.r[10].s64 + -17792;
	// 8269C3FC: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269C400: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8269C404: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8269C408: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8269C40C: 386A42B0  addi r3, r10, 0x42b0
	ctx.r[3].s64 = ctx.r[10].s64 + 17072;
	// 8269C410: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8269C414: 39400003  li r10, 3
	ctx.r[10].s64 = 3;
	// 8269C418: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8269C41C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8269C420: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8269C424: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8269C428: 4BDCA9F9  bl 0x82466e20
	ctx.lr = 0x8269C42C;
	sub_82466E20(ctx, base);
	// 8269C42C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8269C430: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8269C434: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8269C438: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8269C440(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8269C440 size=24
    let mut pc: u32 = 0x8269C440;
    'dispatch: loop {
        match pc {
            0x8269C440 => {
    //   block [0x8269C440..0x8269C458)
	// 8269C440: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 8269C444: 3D40827C  lis r10, -0x7d84
	ctx.r[10].s64 = -2105802752;
	// 8269C448: 394A7170  addi r10, r10, 0x7170
	ctx.r[10].s64 = ctx.r[10].s64 + 29040;
	// 8269C44C: 816B0EBC  lwz r11, 0xebc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(3772 as u32) ) } as u64;
	// 8269C450: 916A0008  stw r11, 8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 8269C454: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8269C458(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8269C458 size=112
    let mut pc: u32 = 0x8269C458;
    'dispatch: loop {
        match pc {
            0x8269C458 => {
    //   block [0x8269C458..0x8269C4C8)
	// 8269C458: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8269C45C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8269C460: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8269C464: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 8269C468: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 8269C46C: 392A9E30  addi r9, r10, -0x61d0
	ctx.r[9].s64 = ctx.r[10].s64 + -25040;
	// 8269C470: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8269C474: 390B7170  addi r8, r11, 0x7170
	ctx.r[8].s64 = ctx.r[11].s64 + 29040;
	// 8269C478: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
	// 8269C47C: 388ABA94  addi r4, r10, -0x456c
	ctx.r[4].s64 = ctx.r[10].s64 + -17772;
	// 8269C480: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8269C484: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269C488: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8269C48C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8269C490: 386A42E0  addi r3, r10, 0x42e0
	ctx.r[3].s64 = ctx.r[10].s64 + 17120;
	// 8269C494: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8269C498: 39400002  li r10, 2
	ctx.r[10].s64 = 2;
	// 8269C49C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8269C4A0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8269C4A4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8269C4A8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8269C4AC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8269C4B0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8269C4B4: 4BDCA96D  bl 0x82466e20
	ctx.lr = 0x8269C4B8;
	sub_82466E20(ctx, base);
	// 8269C4B8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8269C4BC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8269C4C0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8269C4C4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8269C4C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8269C4C8 size=112
    let mut pc: u32 = 0x8269C4C8;
    'dispatch: loop {
        match pc {
            0x8269C4C8 => {
    //   block [0x8269C4C8..0x8269C538)
	// 8269C4C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8269C4CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8269C4D0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8269C4D4: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269C4D8: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 8269C4DC: 38AA42E0  addi r5, r10, 0x42e0
	ctx.r[5].s64 = ctx.r[10].s64 + 17120;
	// 8269C4E0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8269C4E4: 390B0EC4  addi r8, r11, 0xec4
	ctx.r[8].s64 = ctx.r[11].s64 + 3780;
	// 8269C4E8: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 8269C4EC: 388ABAA8  addi r4, r10, -0x4558
	ctx.r[4].s64 = ctx.r[10].s64 + -17752;
	// 8269C4F0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8269C4F4: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269C4F8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8269C4FC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8269C500: 386A4310  addi r3, r10, 0x4310
	ctx.r[3].s64 = ctx.r[10].s64 + 17168;
	// 8269C504: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8269C508: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8269C50C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8269C510: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8269C514: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8269C518: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8269C51C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8269C520: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8269C524: 4BDCA8FD  bl 0x82466e20
	ctx.lr = 0x8269C528;
	sub_82466E20(ctx, base);
	// 8269C528: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8269C52C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8269C530: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8269C534: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8269C538(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8269C538 size=108
    let mut pc: u32 = 0x8269C538;
    'dispatch: loop {
        match pc {
            0x8269C538 => {
    //   block [0x8269C538..0x8269C5A4)
	// 8269C538: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8269C53C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8269C540: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8269C544: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 8269C548: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8269C54C: 38EB0EF4  addi r7, r11, 0xef4
	ctx.r[7].s64 = ctx.r[11].s64 + 3828;
	// 8269C550: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 8269C554: 388ABAC0  addi r4, r10, -0x4540
	ctx.r[4].s64 = ctx.r[10].s64 + -17728;
	// 8269C558: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8269C55C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269C560: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8269C564: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8269C568: 386A4340  addi r3, r10, 0x4340
	ctx.r[3].s64 = ctx.r[10].s64 + 17216;
	// 8269C56C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8269C570: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8269C574: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8269C578: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8269C57C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8269C580: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8269C584: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8269C588: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8269C58C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8269C590: 4BDCA891  bl 0x82466e20
	ctx.lr = 0x8269C594;
	sub_82466E20(ctx, base);
	// 8269C594: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8269C598: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8269C59C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8269C5A0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8269C5A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8269C5A8 size=112
    let mut pc: u32 = 0x8269C5A8;
    'dispatch: loop {
        match pc {
            0x8269C5A8 => {
    //   block [0x8269C5A8..0x8269C618)
	// 8269C5A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8269C5AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8269C5B0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8269C5B4: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269C5B8: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 8269C5BC: 38AA42E0  addi r5, r10, 0x42e0
	ctx.r[5].s64 = ctx.r[10].s64 + 17120;
	// 8269C5C0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8269C5C4: 390B0F10  addi r8, r11, 0xf10
	ctx.r[8].s64 = ctx.r[11].s64 + 3856;
	// 8269C5C8: 39200008  li r9, 8
	ctx.r[9].s64 = 8;
	// 8269C5CC: 388ABAD0  addi r4, r10, -0x4530
	ctx.r[4].s64 = ctx.r[10].s64 + -17712;
	// 8269C5D0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8269C5D4: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269C5D8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8269C5DC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8269C5E0: 386A4370  addi r3, r10, 0x4370
	ctx.r[3].s64 = ctx.r[10].s64 + 17264;
	// 8269C5E4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8269C5E8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8269C5EC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8269C5F0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8269C5F4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8269C5F8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8269C5FC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8269C600: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8269C604: 4BDCA81D  bl 0x82466e20
	ctx.lr = 0x8269C608;
	sub_82466E20(ctx, base);
	// 8269C608: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8269C60C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8269C610: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8269C614: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8269C618(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8269C618 size=100
    let mut pc: u32 = 0x8269C618;
    'dispatch: loop {
        match pc {
            0x8269C618 => {
    //   block [0x8269C618..0x8269C67C)
	// 8269C618: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8269C61C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8269C620: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8269C624: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269C628: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8269C62C: 38AA42E0  addi r5, r10, 0x42e0
	ctx.r[5].s64 = ctx.r[10].s64 + 17120;
	// 8269C630: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8269C634: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8269C638: 388ABAF0  addi r4, r10, -0x4510
	ctx.r[4].s64 = ctx.r[10].s64 + -17680;
	// 8269C63C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269C640: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8269C644: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8269C648: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8269C64C: 386A43A0  addi r3, r10, 0x43a0
	ctx.r[3].s64 = ctx.r[10].s64 + 17312;
	// 8269C650: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8269C654: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8269C658: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8269C65C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8269C660: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8269C664: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8269C668: 4BDCA7B9  bl 0x82466e20
	ctx.lr = 0x8269C66C;
	sub_82466E20(ctx, base);
	// 8269C66C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8269C670: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8269C674: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8269C678: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8269C680(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8269C680 size=112
    let mut pc: u32 = 0x8269C680;
    'dispatch: loop {
        match pc {
            0x8269C680 => {
    //   block [0x8269C680..0x8269C6F0)
	// 8269C680: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8269C684: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8269C688: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8269C68C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269C690: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 8269C694: 38AA42E0  addi r5, r10, 0x42e0
	ctx.r[5].s64 = ctx.r[10].s64 + 17120;
	// 8269C698: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8269C69C: 390B0FD0  addi r8, r11, 0xfd0
	ctx.r[8].s64 = ctx.r[11].s64 + 4048;
	// 8269C6A0: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8269C6A4: 388ABB0C  addi r4, r10, -0x44f4
	ctx.r[4].s64 = ctx.r[10].s64 + -17652;
	// 8269C6A8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8269C6AC: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269C6B0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8269C6B4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8269C6B8: 386A43D0  addi r3, r10, 0x43d0
	ctx.r[3].s64 = ctx.r[10].s64 + 17360;
	// 8269C6BC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8269C6C0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8269C6C4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8269C6C8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8269C6CC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8269C6D0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8269C6D4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8269C6D8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8269C6DC: 4BDCA745  bl 0x82466e20
	ctx.lr = 0x8269C6E0;
	sub_82466E20(ctx, base);
	// 8269C6E0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8269C6E4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8269C6E8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8269C6EC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8269C6F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8269C6F0 size=112
    let mut pc: u32 = 0x8269C6F0;
    'dispatch: loop {
        match pc {
            0x8269C6F0 => {
    //   block [0x8269C6F0..0x8269C760)
	// 8269C6F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8269C6F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8269C6F8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8269C6FC: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269C700: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 8269C704: 38AA42E0  addi r5, r10, 0x42e0
	ctx.r[5].s64 = ctx.r[10].s64 + 17120;
	// 8269C708: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8269C70C: 390B0FE8  addi r8, r11, 0xfe8
	ctx.r[8].s64 = ctx.r[11].s64 + 4072;
	// 8269C710: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 8269C714: 388ABB2C  addi r4, r10, -0x44d4
	ctx.r[4].s64 = ctx.r[10].s64 + -17620;
	// 8269C718: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8269C71C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269C720: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8269C724: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8269C728: 386A4400  addi r3, r10, 0x4400
	ctx.r[3].s64 = ctx.r[10].s64 + 17408;
	// 8269C72C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8269C730: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8269C734: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8269C738: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8269C73C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8269C740: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8269C744: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8269C748: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8269C74C: 4BDCA6D5  bl 0x82466e20
	ctx.lr = 0x8269C750;
	sub_82466E20(ctx, base);
	// 8269C750: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8269C754: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8269C758: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8269C75C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8269C760(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8269C760 size=112
    let mut pc: u32 = 0x8269C760;
    'dispatch: loop {
        match pc {
            0x8269C760 => {
    //   block [0x8269C760..0x8269C7D0)
	// 8269C760: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8269C764: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8269C768: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8269C76C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269C770: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 8269C774: 38AA42E0  addi r5, r10, 0x42e0
	ctx.r[5].s64 = ctx.r[10].s64 + 17120;
	// 8269C778: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8269C77C: 390B1018  addi r8, r11, 0x1018
	ctx.r[8].s64 = ctx.r[11].s64 + 4120;
	// 8269C780: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 8269C784: 388ABB50  addi r4, r10, -0x44b0
	ctx.r[4].s64 = ctx.r[10].s64 + -17584;
	// 8269C788: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8269C78C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269C790: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8269C794: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8269C798: 386A4430  addi r3, r10, 0x4430
	ctx.r[3].s64 = ctx.r[10].s64 + 17456;
	// 8269C79C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8269C7A0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8269C7A4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8269C7A8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8269C7AC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8269C7B0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8269C7B4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8269C7B8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8269C7BC: 4BDCA665  bl 0x82466e20
	ctx.lr = 0x8269C7C0;
	sub_82466E20(ctx, base);
	// 8269C7C0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8269C7C4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8269C7C8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8269C7CC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8269C7D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8269C7D0 size=112
    let mut pc: u32 = 0x8269C7D0;
    'dispatch: loop {
        match pc {
            0x8269C7D0 => {
    //   block [0x8269C7D0..0x8269C840)
	// 8269C7D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8269C7D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8269C7D8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8269C7DC: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269C7E0: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 8269C7E4: 38AA42E0  addi r5, r10, 0x42e0
	ctx.r[5].s64 = ctx.r[10].s64 + 17120;
	// 8269C7E8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8269C7EC: 390B1048  addi r8, r11, 0x1048
	ctx.r[8].s64 = ctx.r[11].s64 + 4168;
	// 8269C7F0: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 8269C7F4: 388ABB78  addi r4, r10, -0x4488
	ctx.r[4].s64 = ctx.r[10].s64 + -17544;
	// 8269C7F8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8269C7FC: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269C800: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8269C804: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8269C808: 386A4460  addi r3, r10, 0x4460
	ctx.r[3].s64 = ctx.r[10].s64 + 17504;
	// 8269C80C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8269C810: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8269C814: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8269C818: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8269C81C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8269C820: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8269C824: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8269C828: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8269C82C: 4BDCA5F5  bl 0x82466e20
	ctx.lr = 0x8269C830;
	sub_82466E20(ctx, base);
	// 8269C830: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8269C834: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8269C838: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8269C83C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8269C840(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8269C840 size=112
    let mut pc: u32 = 0x8269C840;
    'dispatch: loop {
        match pc {
            0x8269C840 => {
    //   block [0x8269C840..0x8269C8B0)
	// 8269C840: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8269C844: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8269C848: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8269C84C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269C850: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 8269C854: 38AA42E0  addi r5, r10, 0x42e0
	ctx.r[5].s64 = ctx.r[10].s64 + 17120;
	// 8269C858: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8269C85C: 390B1078  addi r8, r11, 0x1078
	ctx.r[8].s64 = ctx.r[11].s64 + 4216;
	// 8269C860: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8269C864: 388ABB9C  addi r4, r10, -0x4464
	ctx.r[4].s64 = ctx.r[10].s64 + -17508;
	// 8269C868: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8269C86C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269C870: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8269C874: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8269C878: 386A4490  addi r3, r10, 0x4490
	ctx.r[3].s64 = ctx.r[10].s64 + 17552;
	// 8269C87C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8269C880: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8269C884: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8269C888: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8269C88C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8269C890: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8269C894: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8269C898: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8269C89C: 4BDCA585  bl 0x82466e20
	ctx.lr = 0x8269C8A0;
	sub_82466E20(ctx, base);
	// 8269C8A0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8269C8A4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8269C8A8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8269C8AC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8269C8B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8269C8B0 size=112
    let mut pc: u32 = 0x8269C8B0;
    'dispatch: loop {
        match pc {
            0x8269C8B0 => {
    //   block [0x8269C8B0..0x8269C920)
	// 8269C8B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8269C8B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8269C8B8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8269C8BC: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269C8C0: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 8269C8C4: 38AA42E0  addi r5, r10, 0x42e0
	ctx.r[5].s64 = ctx.r[10].s64 + 17120;
	// 8269C8C8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8269C8CC: 390B1090  addi r8, r11, 0x1090
	ctx.r[8].s64 = ctx.r[11].s64 + 4240;
	// 8269C8D0: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8269C8D4: 388ABBBC  addi r4, r10, -0x4444
	ctx.r[4].s64 = ctx.r[10].s64 + -17476;
	// 8269C8D8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8269C8DC: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269C8E0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8269C8E4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8269C8E8: 386A44C0  addi r3, r10, 0x44c0
	ctx.r[3].s64 = ctx.r[10].s64 + 17600;
	// 8269C8EC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8269C8F0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8269C8F4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8269C8F8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8269C8FC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8269C900: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8269C904: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8269C908: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8269C90C: 4BDCA515  bl 0x82466e20
	ctx.lr = 0x8269C910;
	sub_82466E20(ctx, base);
	// 8269C910: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8269C914: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8269C918: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8269C91C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8269C920(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8269C920 size=112
    let mut pc: u32 = 0x8269C920;
    'dispatch: loop {
        match pc {
            0x8269C920 => {
    //   block [0x8269C920..0x8269C990)
	// 8269C920: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8269C924: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8269C928: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8269C92C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269C930: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 8269C934: 38AA42E0  addi r5, r10, 0x42e0
	ctx.r[5].s64 = ctx.r[10].s64 + 17120;
	// 8269C938: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8269C93C: 390B10A8  addi r8, r11, 0x10a8
	ctx.r[8].s64 = ctx.r[11].s64 + 4264;
	// 8269C940: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 8269C944: 388ABBD4  addi r4, r10, -0x442c
	ctx.r[4].s64 = ctx.r[10].s64 + -17452;
	// 8269C948: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8269C94C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269C950: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8269C954: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8269C958: 386A44F0  addi r3, r10, 0x44f0
	ctx.r[3].s64 = ctx.r[10].s64 + 17648;
	// 8269C95C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8269C960: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8269C964: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8269C968: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8269C96C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8269C970: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8269C974: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8269C978: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8269C97C: 4BDCA4A5  bl 0x82466e20
	ctx.lr = 0x8269C980;
	sub_82466E20(ctx, base);
	// 8269C980: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8269C984: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8269C988: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8269C98C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8269C990(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8269C990 size=112
    let mut pc: u32 = 0x8269C990;
    'dispatch: loop {
        match pc {
            0x8269C990 => {
    //   block [0x8269C990..0x8269CA00)
	// 8269C990: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8269C994: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8269C998: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8269C99C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269C9A0: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 8269C9A4: 38AA42E0  addi r5, r10, 0x42e0
	ctx.r[5].s64 = ctx.r[10].s64 + 17120;
	// 8269C9A8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8269C9AC: 390B10F0  addi r8, r11, 0x10f0
	ctx.r[8].s64 = ctx.r[11].s64 + 4336;
	// 8269C9B0: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 8269C9B4: 388ABBF0  addi r4, r10, -0x4410
	ctx.r[4].s64 = ctx.r[10].s64 + -17424;
	// 8269C9B8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8269C9BC: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269C9C0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8269C9C4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8269C9C8: 386A4520  addi r3, r10, 0x4520
	ctx.r[3].s64 = ctx.r[10].s64 + 17696;
	// 8269C9CC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8269C9D0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8269C9D4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8269C9D8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8269C9DC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8269C9E0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8269C9E4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8269C9E8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8269C9EC: 4BDCA435  bl 0x82466e20
	ctx.lr = 0x8269C9F0;
	sub_82466E20(ctx, base);
	// 8269C9F0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8269C9F4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8269C9F8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8269C9FC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8269CA00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8269CA00 size=112
    let mut pc: u32 = 0x8269CA00;
    'dispatch: loop {
        match pc {
            0x8269CA00 => {
    //   block [0x8269CA00..0x8269CA70)
	// 8269CA00: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8269CA04: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8269CA08: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8269CA0C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269CA10: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 8269CA14: 38AA42E0  addi r5, r10, 0x42e0
	ctx.r[5].s64 = ctx.r[10].s64 + 17120;
	// 8269CA18: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8269CA1C: 390B1138  addi r8, r11, 0x1138
	ctx.r[8].s64 = ctx.r[11].s64 + 4408;
	// 8269CA20: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8269CA24: 388ABC0C  addi r4, r10, -0x43f4
	ctx.r[4].s64 = ctx.r[10].s64 + -17396;
	// 8269CA28: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8269CA2C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269CA30: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8269CA34: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8269CA38: 386A4550  addi r3, r10, 0x4550
	ctx.r[3].s64 = ctx.r[10].s64 + 17744;
	// 8269CA3C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8269CA40: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8269CA44: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8269CA48: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8269CA4C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8269CA50: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8269CA54: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8269CA58: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8269CA5C: 4BDCA3C5  bl 0x82466e20
	ctx.lr = 0x8269CA60;
	sub_82466E20(ctx, base);
	// 8269CA60: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8269CA64: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8269CA68: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8269CA6C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8269CA70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8269CA70 size=112
    let mut pc: u32 = 0x8269CA70;
    'dispatch: loop {
        match pc {
            0x8269CA70 => {
    //   block [0x8269CA70..0x8269CAE0)
	// 8269CA70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8269CA74: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8269CA78: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8269CA7C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269CA80: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 8269CA84: 38AA42E0  addi r5, r10, 0x42e0
	ctx.r[5].s64 = ctx.r[10].s64 + 17120;
	// 8269CA88: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8269CA8C: 390B1150  addi r8, r11, 0x1150
	ctx.r[8].s64 = ctx.r[11].s64 + 4432;
	// 8269CA90: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 8269CA94: 388ABC24  addi r4, r10, -0x43dc
	ctx.r[4].s64 = ctx.r[10].s64 + -17372;
	// 8269CA98: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8269CA9C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269CAA0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8269CAA4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8269CAA8: 386A4580  addi r3, r10, 0x4580
	ctx.r[3].s64 = ctx.r[10].s64 + 17792;
	// 8269CAAC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8269CAB0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8269CAB4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8269CAB8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8269CABC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8269CAC0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8269CAC4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8269CAC8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8269CACC: 4BDCA355  bl 0x82466e20
	ctx.lr = 0x8269CAD0;
	sub_82466E20(ctx, base);
	// 8269CAD0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8269CAD4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8269CAD8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8269CADC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8269CAE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8269CAE0 size=116
    let mut pc: u32 = 0x8269CAE0;
    'dispatch: loop {
        match pc {
            0x8269CAE0 => {
    //   block [0x8269CAE0..0x8269CB54)
	// 8269CAE0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8269CAE4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8269CAE8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8269CAEC: 3D40827C  lis r10, -0x7d84
	ctx.r[10].s64 = -2105802752;
	// 8269CAF0: 38E00005  li r7, 5
	ctx.r[7].s64 = 5;
	// 8269CAF4: 390A1180  addi r8, r10, 0x1180
	ctx.r[8].s64 = ctx.r[10].s64 + 4480;
	// 8269CAF8: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269CAFC: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 8269CB00: 38AA42E0  addi r5, r10, 0x42e0
	ctx.r[5].s64 = ctx.r[10].s64 + 17120;
	// 8269CB04: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8269CB08: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8269CB0C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8269CB10: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8269CB14: 388ABC3C  addi r4, r10, -0x43c4
	ctx.r[4].s64 = ctx.r[10].s64 + -17348;
	// 8269CB18: 396B9E58  addi r11, r11, -0x61a8
	ctx.r[11].s64 = ctx.r[11].s64 + -25000;
	// 8269CB1C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269CB20: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8269CB24: 386A45B0  addi r3, r10, 0x45b0
	ctx.r[3].s64 = ctx.r[10].s64 + 17840;
	// 8269CB28: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 8269CB2C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8269CB30: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 8269CB34: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8269CB38: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8269CB3C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8269CB40: 4BDCA2E1  bl 0x82466e20
	ctx.lr = 0x8269CB44;
	sub_82466E20(ctx, base);
	// 8269CB44: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8269CB48: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8269CB4C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8269CB50: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8269CB58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8269CB58 size=116
    let mut pc: u32 = 0x8269CB58;
    'dispatch: loop {
        match pc {
            0x8269CB58 => {
    //   block [0x8269CB58..0x8269CBCC)
	// 8269CB58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8269CB5C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8269CB60: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8269CB64: 3D40827C  lis r10, -0x7d84
	ctx.r[10].s64 = -2105802752;
	// 8269CB68: 38E00006  li r7, 6
	ctx.r[7].s64 = 6;
	// 8269CB6C: 390A11F8  addi r8, r10, 0x11f8
	ctx.r[8].s64 = ctx.r[10].s64 + 4600;
	// 8269CB70: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269CB74: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 8269CB78: 38AA42E0  addi r5, r10, 0x42e0
	ctx.r[5].s64 = ctx.r[10].s64 + 17120;
	// 8269CB7C: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8269CB80: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8269CB84: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8269CB88: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8269CB8C: 388ABC58  addi r4, r10, -0x43a8
	ctx.r[4].s64 = ctx.r[10].s64 + -17320;
	// 8269CB90: 396B9E70  addi r11, r11, -0x6190
	ctx.r[11].s64 = ctx.r[11].s64 + -24976;
	// 8269CB94: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269CB98: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8269CB9C: 386A45E0  addi r3, r10, 0x45e0
	ctx.r[3].s64 = ctx.r[10].s64 + 17888;
	// 8269CBA0: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 8269CBA4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8269CBA8: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 8269CBAC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8269CBB0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8269CBB4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8269CBB8: 4BDCA269  bl 0x82466e20
	ctx.lr = 0x8269CBBC;
	sub_82466E20(ctx, base);
	// 8269CBBC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8269CBC0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8269CBC4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8269CBC8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8269CBD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8269CBD0 size=24
    let mut pc: u32 = 0x8269CBD0;
    'dispatch: loop {
        match pc {
            0x8269CBD0 => {
    //   block [0x8269CBD0..0x8269CBE8)
	// 8269CBD0: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 8269CBD4: 3D40827C  lis r10, -0x7d84
	ctx.r[10].s64 = -2105802752;
	// 8269CBD8: 394A7188  addi r10, r10, 0x7188
	ctx.r[10].s64 = ctx.r[10].s64 + 29064;
	// 8269CBDC: 816B0F0C  lwz r11, 0xf0c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(3852 as u32) ) } as u64;
	// 8269CBE0: 916A0050  stw r11, 0x50(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 8269CBE4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8269CBE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8269CBE8 size=116
    let mut pc: u32 = 0x8269CBE8;
    'dispatch: loop {
        match pc {
            0x8269CBE8 => {
    //   block [0x8269CBE8..0x8269CC5C)
	// 8269CBE8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8269CBEC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8269CBF0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8269CBF4: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 8269CBF8: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269CBFC: 392B9E9C  addi r9, r11, -0x6164
	ctx.r[9].s64 = ctx.r[11].s64 + -24932;
	// 8269CC00: 38AA42E0  addi r5, r10, 0x42e0
	ctx.r[5].s64 = ctx.r[10].s64 + 17120;
	// 8269CC04: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8269CC08: 38E90014  addi r7, r9, 0x14
	ctx.r[7].s64 = ctx.r[9].s64 + 20;
	// 8269CC0C: 38C00008  li r6, 8
	ctx.r[6].s64 = 8;
	// 8269CC10: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 8269CC14: 388ABC8C  addi r4, r10, -0x4374
	ctx.r[4].s64 = ctx.r[10].s64 + -17268;
	// 8269CC18: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8269CC1C: 396B7188  addi r11, r11, 0x7188
	ctx.r[11].s64 = ctx.r[11].s64 + 29064;
	// 8269CC20: 90E10064  stw r7, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[7].u32 ) };
	// 8269CC24: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269CC28: 90C1005C  stw r6, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[6].u32 ) };
	// 8269CC2C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8269CC30: 386A4610  addi r3, r10, 0x4610
	ctx.r[3].s64 = ctx.r[10].s64 + 17936;
	// 8269CC34: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8269CC38: 91010074  stw r8, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[8].u32 ) };
	// 8269CC3C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8269CC40: 9101006C  stw r8, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[8].u32 ) };
	// 8269CC44: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8269CC48: 4BDCA1D9  bl 0x82466e20
	ctx.lr = 0x8269CC4C;
	sub_82466E20(ctx, base);
	// 8269CC4C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8269CC50: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8269CC54: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8269CC58: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8269CC60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8269CC60 size=112
    let mut pc: u32 = 0x8269CC60;
    'dispatch: loop {
        match pc {
            0x8269CC60 => {
    //   block [0x8269CC60..0x8269CCD0)
	// 8269CC60: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8269CC64: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8269CC68: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8269CC6C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269CC70: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 8269CC74: 38AA42E0  addi r5, r10, 0x42e0
	ctx.r[5].s64 = ctx.r[10].s64 + 17120;
	// 8269CC78: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8269CC7C: 390B1288  addi r8, r11, 0x1288
	ctx.r[8].s64 = ctx.r[11].s64 + 4744;
	// 8269CC80: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 8269CC84: 388ABCA8  addi r4, r10, -0x4358
	ctx.r[4].s64 = ctx.r[10].s64 + -17240;
	// 8269CC88: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8269CC8C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269CC90: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8269CC94: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8269CC98: 386A4640  addi r3, r10, 0x4640
	ctx.r[3].s64 = ctx.r[10].s64 + 17984;
	// 8269CC9C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8269CCA0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8269CCA4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8269CCA8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8269CCAC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8269CCB0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8269CCB4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8269CCB8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8269CCBC: 4BDCA165  bl 0x82466e20
	ctx.lr = 0x8269CCC0;
	sub_82466E20(ctx, base);
	// 8269CCC0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8269CCC4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8269CCC8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8269CCCC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8269CCD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8269CCD0 size=112
    let mut pc: u32 = 0x8269CCD0;
    'dispatch: loop {
        match pc {
            0x8269CCD0 => {
    //   block [0x8269CCD0..0x8269CD40)
	// 8269CCD0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8269CCD4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8269CCD8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8269CCDC: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269CCE0: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 8269CCE4: 38AA42E0  addi r5, r10, 0x42e0
	ctx.r[5].s64 = ctx.r[10].s64 + 17120;
	// 8269CCE8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8269CCEC: 390B12E8  addi r8, r11, 0x12e8
	ctx.r[8].s64 = ctx.r[11].s64 + 4840;
	// 8269CCF0: 39200007  li r9, 7
	ctx.r[9].s64 = 7;
	// 8269CCF4: 388ABCC8  addi r4, r10, -0x4338
	ctx.r[4].s64 = ctx.r[10].s64 + -17208;
	// 8269CCF8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8269CCFC: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269CD00: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8269CD04: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8269CD08: 386A4670  addi r3, r10, 0x4670
	ctx.r[3].s64 = ctx.r[10].s64 + 18032;
	// 8269CD0C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8269CD10: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8269CD14: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8269CD18: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8269CD1C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8269CD20: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8269CD24: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8269CD28: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8269CD2C: 4BDCA0F5  bl 0x82466e20
	ctx.lr = 0x8269CD30;
	sub_82466E20(ctx, base);
	// 8269CD30: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8269CD34: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8269CD38: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8269CD3C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8269CD40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8269CD40 size=112
    let mut pc: u32 = 0x8269CD40;
    'dispatch: loop {
        match pc {
            0x8269CD40 => {
    //   block [0x8269CD40..0x8269CDB0)
	// 8269CD40: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8269CD44: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8269CD48: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8269CD4C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269CD50: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 8269CD54: 38AA42E0  addi r5, r10, 0x42e0
	ctx.r[5].s64 = ctx.r[10].s64 + 17120;
	// 8269CD58: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8269CD5C: 390B1390  addi r8, r11, 0x1390
	ctx.r[8].s64 = ctx.r[11].s64 + 5008;
	// 8269CD60: 39200005  li r9, 5
	ctx.r[9].s64 = 5;
	// 8269CD64: 388ABCE4  addi r4, r10, -0x431c
	ctx.r[4].s64 = ctx.r[10].s64 + -17180;
	// 8269CD68: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8269CD6C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269CD70: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8269CD74: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8269CD78: 386A46A0  addi r3, r10, 0x46a0
	ctx.r[3].s64 = ctx.r[10].s64 + 18080;
	// 8269CD7C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8269CD80: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8269CD84: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8269CD88: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8269CD8C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8269CD90: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8269CD94: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8269CD98: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8269CD9C: 4BDCA085  bl 0x82466e20
	ctx.lr = 0x8269CDA0;
	sub_82466E20(ctx, base);
	// 8269CDA0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8269CDA4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8269CDA8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8269CDAC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8269CDB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8269CDB0 size=112
    let mut pc: u32 = 0x8269CDB0;
    'dispatch: loop {
        match pc {
            0x8269CDB0 => {
    //   block [0x8269CDB0..0x8269CE20)
	// 8269CDB0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8269CDB4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8269CDB8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8269CDBC: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269CDC0: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 8269CDC4: 38AA42E0  addi r5, r10, 0x42e0
	ctx.r[5].s64 = ctx.r[10].s64 + 17120;
	// 8269CDC8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8269CDCC: 390B1408  addi r8, r11, 0x1408
	ctx.r[8].s64 = ctx.r[11].s64 + 5128;
	// 8269CDD0: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 8269CDD4: 388ABD04  addi r4, r10, -0x42fc
	ctx.r[4].s64 = ctx.r[10].s64 + -17148;
	// 8269CDD8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8269CDDC: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269CDE0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8269CDE4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8269CDE8: 386A46D0  addi r3, r10, 0x46d0
	ctx.r[3].s64 = ctx.r[10].s64 + 18128;
	// 8269CDEC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8269CDF0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8269CDF4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8269CDF8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8269CDFC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8269CE00: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8269CE04: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8269CE08: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8269CE0C: 4BDCA015  bl 0x82466e20
	ctx.lr = 0x8269CE10;
	sub_82466E20(ctx, base);
	// 8269CE10: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8269CE14: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8269CE18: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8269CE1C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8269CE20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8269CE20 size=112
    let mut pc: u32 = 0x8269CE20;
    'dispatch: loop {
        match pc {
            0x8269CE20 => {
    //   block [0x8269CE20..0x8269CE90)
	// 8269CE20: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8269CE24: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8269CE28: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8269CE2C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269CE30: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 8269CE34: 38AA42E0  addi r5, r10, 0x42e0
	ctx.r[5].s64 = ctx.r[10].s64 + 17120;
	// 8269CE38: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8269CE3C: 390B1450  addi r8, r11, 0x1450
	ctx.r[8].s64 = ctx.r[11].s64 + 5200;
	// 8269CE40: 39200006  li r9, 6
	ctx.r[9].s64 = 6;
	// 8269CE44: 388ABD24  addi r4, r10, -0x42dc
	ctx.r[4].s64 = ctx.r[10].s64 + -17116;
	// 8269CE48: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8269CE4C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269CE50: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8269CE54: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8269CE58: 386A4700  addi r3, r10, 0x4700
	ctx.r[3].s64 = ctx.r[10].s64 + 18176;
	// 8269CE5C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8269CE60: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8269CE64: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8269CE68: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8269CE6C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8269CE70: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8269CE74: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8269CE78: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8269CE7C: 4BDC9FA5  bl 0x82466e20
	ctx.lr = 0x8269CE80;
	sub_82466E20(ctx, base);
	// 8269CE80: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8269CE84: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8269CE88: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8269CE8C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8269CE90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8269CE90 size=112
    let mut pc: u32 = 0x8269CE90;
    'dispatch: loop {
        match pc {
            0x8269CE90 => {
    //   block [0x8269CE90..0x8269CF00)
	// 8269CE90: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8269CE94: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8269CE98: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8269CE9C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269CEA0: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 8269CEA4: 38AA42E0  addi r5, r10, 0x42e0
	ctx.r[5].s64 = ctx.r[10].s64 + 17120;
	// 8269CEA8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8269CEAC: 390B14E0  addi r8, r11, 0x14e0
	ctx.r[8].s64 = ctx.r[11].s64 + 5344;
	// 8269CEB0: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 8269CEB4: 388ABD40  addi r4, r10, -0x42c0
	ctx.r[4].s64 = ctx.r[10].s64 + -17088;
	// 8269CEB8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8269CEBC: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269CEC0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8269CEC4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8269CEC8: 386A4730  addi r3, r10, 0x4730
	ctx.r[3].s64 = ctx.r[10].s64 + 18224;
	// 8269CECC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8269CED0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8269CED4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8269CED8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8269CEDC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8269CEE0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8269CEE4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8269CEE8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8269CEEC: 4BDC9F35  bl 0x82466e20
	ctx.lr = 0x8269CEF0;
	sub_82466E20(ctx, base);
	// 8269CEF0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8269CEF4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8269CEF8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8269CEFC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8269CF00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8269CF00 size=112
    let mut pc: u32 = 0x8269CF00;
    'dispatch: loop {
        match pc {
            0x8269CF00 => {
    //   block [0x8269CF00..0x8269CF70)
	// 8269CF00: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8269CF04: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8269CF08: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8269CF0C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269CF10: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 8269CF14: 38AA42E0  addi r5, r10, 0x42e0
	ctx.r[5].s64 = ctx.r[10].s64 + 17120;
	// 8269CF18: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8269CF1C: 390B1540  addi r8, r11, 0x1540
	ctx.r[8].s64 = ctx.r[11].s64 + 5440;
	// 8269CF20: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 8269CF24: 388ABD58  addi r4, r10, -0x42a8
	ctx.r[4].s64 = ctx.r[10].s64 + -17064;
	// 8269CF28: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8269CF2C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269CF30: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8269CF34: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8269CF38: 386A4760  addi r3, r10, 0x4760
	ctx.r[3].s64 = ctx.r[10].s64 + 18272;
	// 8269CF3C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8269CF40: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8269CF44: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8269CF48: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8269CF4C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8269CF50: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8269CF54: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8269CF58: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8269CF5C: 4BDC9EC5  bl 0x82466e20
	ctx.lr = 0x8269CF60;
	sub_82466E20(ctx, base);
	// 8269CF60: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8269CF64: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8269CF68: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8269CF6C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8269CF70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8269CF70 size=112
    let mut pc: u32 = 0x8269CF70;
    'dispatch: loop {
        match pc {
            0x8269CF70 => {
    //   block [0x8269CF70..0x8269CFE0)
	// 8269CF70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8269CF74: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8269CF78: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8269CF7C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269CF80: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 8269CF84: 38AA4760  addi r5, r10, 0x4760
	ctx.r[5].s64 = ctx.r[10].s64 + 18272;
	// 8269CF88: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8269CF8C: 390B15A0  addi r8, r11, 0x15a0
	ctx.r[8].s64 = ctx.r[11].s64 + 5536;
	// 8269CF90: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 8269CF94: 388ABD74  addi r4, r10, -0x428c
	ctx.r[4].s64 = ctx.r[10].s64 + -17036;
	// 8269CF98: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8269CF9C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269CFA0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8269CFA4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8269CFA8: 386A4790  addi r3, r10, 0x4790
	ctx.r[3].s64 = ctx.r[10].s64 + 18320;
	// 8269CFAC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8269CFB0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8269CFB4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8269CFB8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8269CFBC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8269CFC0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8269CFC4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8269CFC8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8269CFCC: 4BDC9E55  bl 0x82466e20
	ctx.lr = 0x8269CFD0;
	sub_82466E20(ctx, base);
	// 8269CFD0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8269CFD4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8269CFD8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8269CFDC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8269CFE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8269CFE0 size=112
    let mut pc: u32 = 0x8269CFE0;
    'dispatch: loop {
        match pc {
            0x8269CFE0 => {
    //   block [0x8269CFE0..0x8269D050)
	// 8269CFE0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8269CFE4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8269CFE8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8269CFEC: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269CFF0: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 8269CFF4: 38AA4760  addi r5, r10, 0x4760
	ctx.r[5].s64 = ctx.r[10].s64 + 18272;
	// 8269CFF8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8269CFFC: 390B15D0  addi r8, r11, 0x15d0
	ctx.r[8].s64 = ctx.r[11].s64 + 5584;
	// 8269D000: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 8269D004: 388ABD9C  addi r4, r10, -0x4264
	ctx.r[4].s64 = ctx.r[10].s64 + -16996;
	// 8269D008: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8269D00C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269D010: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8269D014: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8269D018: 386A47C0  addi r3, r10, 0x47c0
	ctx.r[3].s64 = ctx.r[10].s64 + 18368;
	// 8269D01C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8269D020: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8269D024: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8269D028: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8269D02C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8269D030: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8269D034: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8269D038: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8269D03C: 4BDC9DE5  bl 0x82466e20
	ctx.lr = 0x8269D040;
	sub_82466E20(ctx, base);
	// 8269D040: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8269D044: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8269D048: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8269D04C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8269D050(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8269D050 size=100
    let mut pc: u32 = 0x8269D050;
    'dispatch: loop {
        match pc {
            0x8269D050 => {
    //   block [0x8269D050..0x8269D0B4)
	// 8269D050: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8269D054: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8269D058: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8269D05C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269D060: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8269D064: 38AA4760  addi r5, r10, 0x4760
	ctx.r[5].s64 = ctx.r[10].s64 + 18272;
	// 8269D068: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8269D06C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8269D070: 388ABDC4  addi r4, r10, -0x423c
	ctx.r[4].s64 = ctx.r[10].s64 + -16956;
	// 8269D074: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269D078: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8269D07C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8269D080: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8269D084: 386A47F0  addi r3, r10, 0x47f0
	ctx.r[3].s64 = ctx.r[10].s64 + 18416;
	// 8269D088: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8269D08C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8269D090: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8269D094: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8269D098: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8269D09C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8269D0A0: 4BDC9D81  bl 0x82466e20
	ctx.lr = 0x8269D0A4;
	sub_82466E20(ctx, base);
	// 8269D0A4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8269D0A8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8269D0AC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8269D0B0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8269D0B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8269D0B8 size=112
    let mut pc: u32 = 0x8269D0B8;
    'dispatch: loop {
        match pc {
            0x8269D0B8 => {
    //   block [0x8269D0B8..0x8269D128)
	// 8269D0B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8269D0BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8269D0C0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8269D0C4: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269D0C8: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 8269D0CC: 38AA4760  addi r5, r10, 0x4760
	ctx.r[5].s64 = ctx.r[10].s64 + 18272;
	// 8269D0D0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8269D0D4: 390B1618  addi r8, r11, 0x1618
	ctx.r[8].s64 = ctx.r[11].s64 + 5656;
	// 8269D0D8: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8269D0DC: 388ABDEC  addi r4, r10, -0x4214
	ctx.r[4].s64 = ctx.r[10].s64 + -16916;
	// 8269D0E0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8269D0E4: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269D0E8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8269D0EC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8269D0F0: 386A4820  addi r3, r10, 0x4820
	ctx.r[3].s64 = ctx.r[10].s64 + 18464;
	// 8269D0F4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8269D0F8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8269D0FC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8269D100: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8269D104: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8269D108: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8269D10C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8269D110: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8269D114: 4BDC9D0D  bl 0x82466e20
	ctx.lr = 0x8269D118;
	sub_82466E20(ctx, base);
	// 8269D118: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8269D11C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8269D120: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8269D124: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8269D128(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8269D128 size=100
    let mut pc: u32 = 0x8269D128;
    'dispatch: loop {
        match pc {
            0x8269D128 => {
    //   block [0x8269D128..0x8269D18C)
	// 8269D128: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8269D12C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8269D130: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8269D134: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269D138: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8269D13C: 38AA4760  addi r5, r10, 0x4760
	ctx.r[5].s64 = ctx.r[10].s64 + 18272;
	// 8269D140: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 8269D144: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8269D148: 388AB3BC  addi r4, r10, -0x4c44
	ctx.r[4].s64 = ctx.r[10].s64 + -19524;
	// 8269D14C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269D150: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8269D154: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8269D158: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8269D15C: 386A4850  addi r3, r10, 0x4850
	ctx.r[3].s64 = ctx.r[10].s64 + 18512;
	// 8269D160: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8269D164: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8269D168: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8269D16C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8269D170: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8269D174: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8269D178: 4BDC9CA9  bl 0x82466e20
	ctx.lr = 0x8269D17C;
	sub_82466E20(ctx, base);
	// 8269D17C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8269D180: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8269D184: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8269D188: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8269D190(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8269D190 size=108
    let mut pc: u32 = 0x8269D190;
    'dispatch: loop {
        match pc {
            0x8269D190 => {
    //   block [0x8269D190..0x8269D1FC)
	// 8269D190: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8269D194: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8269D198: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8269D19C: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 8269D1A0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8269D1A4: 38EB1630  addi r7, r11, 0x1630
	ctx.r[7].s64 = ctx.r[11].s64 + 5680;
	// 8269D1A8: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 8269D1AC: 388ABE14  addi r4, r10, -0x41ec
	ctx.r[4].s64 = ctx.r[10].s64 + -16876;
	// 8269D1B0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8269D1B4: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269D1B8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8269D1BC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8269D1C0: 386A4880  addi r3, r10, 0x4880
	ctx.r[3].s64 = ctx.r[10].s64 + 18560;
	// 8269D1C4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8269D1C8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8269D1CC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8269D1D0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8269D1D4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8269D1D8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8269D1DC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8269D1E0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8269D1E4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8269D1E8: 4BDC9C39  bl 0x82466e20
	ctx.lr = 0x8269D1EC;
	sub_82466E20(ctx, base);
	// 8269D1EC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8269D1F0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8269D1F4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8269D1F8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8269D200(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8269D200 size=112
    let mut pc: u32 = 0x8269D200;
    'dispatch: loop {
        match pc {
            0x8269D200 => {
    //   block [0x8269D200..0x8269D270)
	// 8269D200: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8269D204: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8269D208: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8269D20C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269D210: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 8269D214: 38AA2B10  addi r5, r10, 0x2b10
	ctx.r[5].s64 = ctx.r[10].s64 + 11024;
	// 8269D218: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8269D21C: 390B1678  addi r8, r11, 0x1678
	ctx.r[8].s64 = ctx.r[11].s64 + 5752;
	// 8269D220: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 8269D224: 388ABE38  addi r4, r10, -0x41c8
	ctx.r[4].s64 = ctx.r[10].s64 + -16840;
	// 8269D228: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8269D22C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269D230: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8269D234: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8269D238: 386A48B0  addi r3, r10, 0x48b0
	ctx.r[3].s64 = ctx.r[10].s64 + 18608;
	// 8269D23C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8269D240: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8269D244: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8269D248: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8269D24C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8269D250: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8269D254: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8269D258: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8269D25C: 4BDC9BC5  bl 0x82466e20
	ctx.lr = 0x8269D260;
	sub_82466E20(ctx, base);
	// 8269D260: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8269D264: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8269D268: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8269D26C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8269D270(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8269D270 size=112
    let mut pc: u32 = 0x8269D270;
    'dispatch: loop {
        match pc {
            0x8269D270 => {
    //   block [0x8269D270..0x8269D2E0)
	// 8269D270: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8269D274: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8269D278: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8269D27C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269D280: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 8269D284: 38AA48B0  addi r5, r10, 0x48b0
	ctx.r[5].s64 = ctx.r[10].s64 + 18608;
	// 8269D288: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8269D28C: 390B16D8  addi r8, r11, 0x16d8
	ctx.r[8].s64 = ctx.r[11].s64 + 5848;
	// 8269D290: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8269D294: 388ABE44  addi r4, r10, -0x41bc
	ctx.r[4].s64 = ctx.r[10].s64 + -16828;
	// 8269D298: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8269D29C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269D2A0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8269D2A4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8269D2A8: 386A48E0  addi r3, r10, 0x48e0
	ctx.r[3].s64 = ctx.r[10].s64 + 18656;
	// 8269D2AC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8269D2B0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8269D2B4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8269D2B8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8269D2BC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8269D2C0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8269D2C4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8269D2C8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8269D2CC: 4BDC9B55  bl 0x82466e20
	ctx.lr = 0x8269D2D0;
	sub_82466E20(ctx, base);
	// 8269D2D0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8269D2D4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8269D2D8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8269D2DC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8269D2E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8269D2E0 size=112
    let mut pc: u32 = 0x8269D2E0;
    'dispatch: loop {
        match pc {
            0x8269D2E0 => {
    //   block [0x8269D2E0..0x8269D350)
	// 8269D2E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8269D2E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8269D2E8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8269D2EC: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269D2F0: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 8269D2F4: 38AA48B0  addi r5, r10, 0x48b0
	ctx.r[5].s64 = ctx.r[10].s64 + 18608;
	// 8269D2F8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8269D2FC: 390B16F0  addi r8, r11, 0x16f0
	ctx.r[8].s64 = ctx.r[11].s64 + 5872;
	// 8269D300: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 8269D304: 388ABE54  addi r4, r10, -0x41ac
	ctx.r[4].s64 = ctx.r[10].s64 + -16812;
	// 8269D308: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8269D30C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269D310: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8269D314: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8269D318: 386A4910  addi r3, r10, 0x4910
	ctx.r[3].s64 = ctx.r[10].s64 + 18704;
	// 8269D31C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8269D320: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8269D324: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8269D328: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8269D32C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8269D330: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8269D334: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8269D338: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8269D33C: 4BDC9AE5  bl 0x82466e20
	ctx.lr = 0x8269D340;
	sub_82466E20(ctx, base);
	// 8269D340: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8269D344: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8269D348: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8269D34C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8269D350(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8269D350 size=112
    let mut pc: u32 = 0x8269D350;
    'dispatch: loop {
        match pc {
            0x8269D350 => {
    //   block [0x8269D350..0x8269D3C0)
	// 8269D350: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8269D354: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8269D358: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8269D35C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269D360: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 8269D364: 38AA48B0  addi r5, r10, 0x48b0
	ctx.r[5].s64 = ctx.r[10].s64 + 18608;
	// 8269D368: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8269D36C: 390B1720  addi r8, r11, 0x1720
	ctx.r[8].s64 = ctx.r[11].s64 + 5920;
	// 8269D370: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8269D374: 388ABE64  addi r4, r10, -0x419c
	ctx.r[4].s64 = ctx.r[10].s64 + -16796;
	// 8269D378: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8269D37C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269D380: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8269D384: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8269D388: 386A4940  addi r3, r10, 0x4940
	ctx.r[3].s64 = ctx.r[10].s64 + 18752;
	// 8269D38C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8269D390: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8269D394: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8269D398: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8269D39C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8269D3A0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8269D3A4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8269D3A8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8269D3AC: 4BDC9A75  bl 0x82466e20
	ctx.lr = 0x8269D3B0;
	sub_82466E20(ctx, base);
	// 8269D3B0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8269D3B4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8269D3B8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8269D3BC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8269D3C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8269D3C0 size=24
    let mut pc: u32 = 0x8269D3C0;
    'dispatch: loop {
        match pc {
            0x8269D3C0 => {
    //   block [0x8269D3C0..0x8269D3D8)
	// 8269D3C0: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 8269D3C4: 3D40827C  lis r10, -0x7d84
	ctx.r[10].s64 = -2105802752;
	// 8269D3C8: 394A7248  addi r10, r10, 0x7248
	ctx.r[10].s64 = ctx.r[10].s64 + 29256;
	// 8269D3CC: 816B1738  lwz r11, 0x1738(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(5944 as u32) ) } as u64;
	// 8269D3D0: 916A0008  stw r11, 8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 8269D3D4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8269D3D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8269D3D8 size=112
    let mut pc: u32 = 0x8269D3D8;
    'dispatch: loop {
        match pc {
            0x8269D3D8 => {
    //   block [0x8269D3D8..0x8269D448)
	// 8269D3D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8269D3DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8269D3E0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8269D3E4: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 8269D3E8: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 8269D3EC: 392A9F00  addi r9, r10, -0x6100
	ctx.r[9].s64 = ctx.r[10].s64 + -24832;
	// 8269D3F0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8269D3F4: 390B7248  addi r8, r11, 0x7248
	ctx.r[8].s64 = ctx.r[11].s64 + 29256;
	// 8269D3F8: 38E00003  li r7, 3
	ctx.r[7].s64 = 3;
	// 8269D3FC: 388ABE74  addi r4, r10, -0x418c
	ctx.r[4].s64 = ctx.r[10].s64 + -16780;
	// 8269D400: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8269D404: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269D408: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8269D40C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8269D410: 386A4970  addi r3, r10, 0x4970
	ctx.r[3].s64 = ctx.r[10].s64 + 18800;
	// 8269D414: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8269D418: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8269D41C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8269D420: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8269D424: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8269D428: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8269D42C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8269D430: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8269D434: 4BDC99ED  bl 0x82466e20
	ctx.lr = 0x8269D438;
	sub_82466E20(ctx, base);
	// 8269D438: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8269D43C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8269D440: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8269D444: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8269D448(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8269D448 size=108
    let mut pc: u32 = 0x8269D448;
    'dispatch: loop {
        match pc {
            0x8269D448 => {
    //   block [0x8269D448..0x8269D4B4)
	// 8269D448: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8269D44C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8269D450: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8269D454: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 8269D458: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8269D45C: 38EB173C  addi r7, r11, 0x173c
	ctx.r[7].s64 = ctx.r[11].s64 + 5948;
	// 8269D460: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 8269D464: 388ABE80  addi r4, r10, -0x4180
	ctx.r[4].s64 = ctx.r[10].s64 + -16768;
	// 8269D468: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8269D46C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269D470: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8269D474: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8269D478: 386A49A0  addi r3, r10, 0x49a0
	ctx.r[3].s64 = ctx.r[10].s64 + 18848;
	// 8269D47C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8269D480: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8269D484: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8269D488: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8269D48C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8269D490: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8269D494: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8269D498: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8269D49C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8269D4A0: 4BDC9981  bl 0x82466e20
	ctx.lr = 0x8269D4A4;
	sub_82466E20(ctx, base);
	// 8269D4A4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8269D4A8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8269D4AC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8269D4B0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8269D4B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8269D4B8 size=108
    let mut pc: u32 = 0x8269D4B8;
    'dispatch: loop {
        match pc {
            0x8269D4B8 => {
    //   block [0x8269D4B8..0x8269D524)
	// 8269D4B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8269D4BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8269D4C0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8269D4C4: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 8269D4C8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8269D4CC: 38EB1758  addi r7, r11, 0x1758
	ctx.r[7].s64 = ctx.r[11].s64 + 5976;
	// 8269D4D0: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 8269D4D4: 388ABE94  addi r4, r10, -0x416c
	ctx.r[4].s64 = ctx.r[10].s64 + -16748;
	// 8269D4D8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8269D4DC: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269D4E0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8269D4E4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8269D4E8: 386A49D0  addi r3, r10, 0x49d0
	ctx.r[3].s64 = ctx.r[10].s64 + 18896;
	// 8269D4EC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8269D4F0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8269D4F4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8269D4F8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8269D4FC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8269D500: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8269D504: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8269D508: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8269D50C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8269D510: 4BDC9911  bl 0x82466e20
	ctx.lr = 0x8269D514;
	sub_82466E20(ctx, base);
	// 8269D514: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8269D518: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8269D51C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8269D520: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8269D528(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8269D528 size=116
    let mut pc: u32 = 0x8269D528;
    'dispatch: loop {
        match pc {
            0x8269D528 => {
    //   block [0x8269D528..0x8269D59C)
	// 8269D528: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8269D52C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8269D530: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8269D534: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 8269D538: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 8269D53C: 390B17A0  addi r8, r11, 0x17a0
	ctx.r[8].s64 = ctx.r[11].s64 + 6048;
	// 8269D540: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8269D544: 392A9FC8  addi r9, r10, -0x6038
	ctx.r[9].s64 = ctx.r[10].s64 + -24632;
	// 8269D548: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269D54C: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
	// 8269D550: 38AA2B10  addi r5, r10, 0x2b10
	ctx.r[5].s64 = ctx.r[10].s64 + 11024;
	// 8269D554: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8269D558: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8269D55C: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8269D560: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8269D564: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8269D568: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8269D56C: 3D608297  lis r11, -0x7d69
	ctx.r[11].s64 = -2104033280;
	// 8269D570: 388ABEA0  addi r4, r10, -0x4160
	ctx.r[4].s64 = ctx.r[10].s64 + -16736;
	// 8269D574: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8269D578: 386B4A00  addi r3, r11, 0x4a00
	ctx.r[3].s64 = ctx.r[11].s64 + 18944;
	// 8269D57C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8269D580: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8269D584: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8269D588: 4BDC9899  bl 0x82466e20
	ctx.lr = 0x8269D58C;
	sub_82466E20(ctx, base);
	// 8269D58C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8269D590: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8269D594: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8269D598: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8269D5A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8269D5A0 size=24
    let mut pc: u32 = 0x8269D5A0;
    'dispatch: loop {
        match pc {
            0x8269D5A0 => {
    //   block [0x8269D5A0..0x8269D5B8)
	// 8269D5A0: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 8269D5A4: 3D40827C  lis r10, -0x7d84
	ctx.r[10].s64 = -2105802752;
	// 8269D5A8: 394A7290  addi r10, r10, 0x7290
	ctx.r[10].s64 = ctx.r[10].s64 + 29328;
	// 8269D5AC: 816B17B8  lwz r11, 0x17b8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(6072 as u32) ) } as u64;
	// 8269D5B0: 916A0068  stw r11, 0x68(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(104 as u32), ctx.r[11].u32 ) };
	// 8269D5B4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8269D5B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8269D5B8 size=116
    let mut pc: u32 = 0x8269D5B8;
    'dispatch: loop {
        match pc {
            0x8269D5B8 => {
    //   block [0x8269D5B8..0x8269D62C)
	// 8269D5B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8269D5BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8269D5C0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8269D5C4: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 8269D5C8: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 8269D5CC: 390B7290  addi r8, r11, 0x7290
	ctx.r[8].s64 = ctx.r[11].s64 + 29328;
	// 8269D5D0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8269D5D4: 392AA038  addi r9, r10, -0x5fc8
	ctx.r[9].s64 = ctx.r[10].s64 + -24520;
	// 8269D5D8: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269D5DC: 38E00009  li r7, 9
	ctx.r[7].s64 = 9;
	// 8269D5E0: 38AA2B10  addi r5, r10, 0x2b10
	ctx.r[5].s64 = ctx.r[10].s64 + 11024;
	// 8269D5E4: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8269D5E8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8269D5EC: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8269D5F0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8269D5F4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8269D5F8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8269D5FC: 3D608297  lis r11, -0x7d69
	ctx.r[11].s64 = -2104033280;
	// 8269D600: 388ABEB4  addi r4, r10, -0x414c
	ctx.r[4].s64 = ctx.r[10].s64 + -16716;
	// 8269D604: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8269D608: 386B4A30  addi r3, r11, 0x4a30
	ctx.r[3].s64 = ctx.r[11].s64 + 18992;
	// 8269D60C: 39400004  li r10, 4
	ctx.r[10].s64 = 4;
	// 8269D610: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8269D614: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8269D618: 4BDC9809  bl 0x82466e20
	ctx.lr = 0x8269D61C;
	sub_82466E20(ctx, base);
	// 8269D61C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8269D620: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8269D624: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8269D628: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8269D630(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8269D630 size=108
    let mut pc: u32 = 0x8269D630;
    'dispatch: loop {
        match pc {
            0x8269D630 => {
    //   block [0x8269D630..0x8269D69C)
	// 8269D630: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8269D634: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8269D638: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8269D63C: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 8269D640: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8269D644: 38EB17C8  addi r7, r11, 0x17c8
	ctx.r[7].s64 = ctx.r[11].s64 + 6088;
	// 8269D648: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 8269D64C: 388ABECC  addi r4, r10, -0x4134
	ctx.r[4].s64 = ctx.r[10].s64 + -16692;
	// 8269D650: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8269D654: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269D658: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8269D65C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8269D660: 386A4A60  addi r3, r10, 0x4a60
	ctx.r[3].s64 = ctx.r[10].s64 + 19040;
	// 8269D664: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8269D668: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8269D66C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8269D670: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8269D674: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8269D678: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8269D67C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8269D680: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8269D684: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8269D688: 4BDC9799  bl 0x82466e20
	ctx.lr = 0x8269D68C;
	sub_82466E20(ctx, base);
	// 8269D68C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8269D690: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8269D694: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8269D698: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8269D6A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8269D6A0 size=112
    let mut pc: u32 = 0x8269D6A0;
    'dispatch: loop {
        match pc {
            0x8269D6A0 => {
    //   block [0x8269D6A0..0x8269D710)
	// 8269D6A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8269D6A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8269D6A8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8269D6AC: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269D6B0: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 8269D6B4: 38AA4A00  addi r5, r10, 0x4a00
	ctx.r[5].s64 = ctx.r[10].s64 + 18944;
	// 8269D6B8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8269D6BC: 390B17F8  addi r8, r11, 0x17f8
	ctx.r[8].s64 = ctx.r[11].s64 + 6136;
	// 8269D6C0: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8269D6C4: 388ABEF0  addi r4, r10, -0x4110
	ctx.r[4].s64 = ctx.r[10].s64 + -16656;
	// 8269D6C8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8269D6CC: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269D6D0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8269D6D4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8269D6D8: 386A4A90  addi r3, r10, 0x4a90
	ctx.r[3].s64 = ctx.r[10].s64 + 19088;
	// 8269D6DC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8269D6E0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8269D6E4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8269D6E8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8269D6EC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8269D6F0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8269D6F4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8269D6F8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8269D6FC: 4BDC9725  bl 0x82466e20
	ctx.lr = 0x8269D700;
	sub_82466E20(ctx, base);
	// 8269D700: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8269D704: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8269D708: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8269D70C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8269D710(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8269D710 size=112
    let mut pc: u32 = 0x8269D710;
    'dispatch: loop {
        match pc {
            0x8269D710 => {
    //   block [0x8269D710..0x8269D780)
	// 8269D710: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8269D714: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8269D718: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8269D71C: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 8269D720: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 8269D724: 392AA090  addi r9, r10, -0x5f70
	ctx.r[9].s64 = ctx.r[10].s64 + -24432;
	// 8269D728: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8269D72C: 390B1818  addi r8, r11, 0x1818
	ctx.r[8].s64 = ctx.r[11].s64 + 6168;
	// 8269D730: 38E00003  li r7, 3
	ctx.r[7].s64 = 3;
	// 8269D734: 388ABF10  addi r4, r10, -0x40f0
	ctx.r[4].s64 = ctx.r[10].s64 + -16624;
	// 8269D738: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8269D73C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269D740: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8269D744: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8269D748: 386A4AC0  addi r3, r10, 0x4ac0
	ctx.r[3].s64 = ctx.r[10].s64 + 19136;
	// 8269D74C: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8269D750: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8269D754: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8269D758: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8269D75C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8269D760: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8269D764: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8269D768: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8269D76C: 4BDC96B5  bl 0x82466e20
	ctx.lr = 0x8269D770;
	sub_82466E20(ctx, base);
	// 8269D770: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8269D774: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8269D778: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8269D77C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8269D780(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8269D780 size=112
    let mut pc: u32 = 0x8269D780;
    'dispatch: loop {
        match pc {
            0x8269D780 => {
    //   block [0x8269D780..0x8269D7F0)
	// 8269D780: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8269D784: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8269D788: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8269D78C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269D790: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 8269D794: 38AA4A00  addi r5, r10, 0x4a00
	ctx.r[5].s64 = ctx.r[10].s64 + 18944;
	// 8269D798: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8269D79C: 390B1860  addi r8, r11, 0x1860
	ctx.r[8].s64 = ctx.r[11].s64 + 6240;
	// 8269D7A0: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8269D7A4: 388ABF2C  addi r4, r10, -0x40d4
	ctx.r[4].s64 = ctx.r[10].s64 + -16596;
	// 8269D7A8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8269D7AC: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269D7B0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8269D7B4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8269D7B8: 386A4AF0  addi r3, r10, 0x4af0
	ctx.r[3].s64 = ctx.r[10].s64 + 19184;
	// 8269D7BC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8269D7C0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8269D7C4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8269D7C8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8269D7CC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8269D7D0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8269D7D4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8269D7D8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8269D7DC: 4BDC9645  bl 0x82466e20
	ctx.lr = 0x8269D7E0;
	sub_82466E20(ctx, base);
	// 8269D7E0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8269D7E4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8269D7E8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8269D7EC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8269D7F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8269D7F0 size=112
    let mut pc: u32 = 0x8269D7F0;
    'dispatch: loop {
        match pc {
            0x8269D7F0 => {
    //   block [0x8269D7F0..0x8269D860)
	// 8269D7F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8269D7F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8269D7F8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8269D7FC: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 8269D800: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 8269D804: 392AA0BC  addi r9, r10, -0x5f44
	ctx.r[9].s64 = ctx.r[10].s64 + -24388;
	// 8269D808: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8269D80C: 390B1878  addi r8, r11, 0x1878
	ctx.r[8].s64 = ctx.r[11].s64 + 6264;
	// 8269D810: 38E00006  li r7, 6
	ctx.r[7].s64 = 6;
	// 8269D814: 388ABF44  addi r4, r10, -0x40bc
	ctx.r[4].s64 = ctx.r[10].s64 + -16572;
	// 8269D818: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8269D81C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269D820: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8269D824: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8269D828: 386A4B20  addi r3, r10, 0x4b20
	ctx.r[3].s64 = ctx.r[10].s64 + 19232;
	// 8269D82C: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8269D830: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8269D834: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8269D838: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8269D83C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8269D840: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8269D844: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8269D848: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8269D84C: 4BDC95D5  bl 0x82466e20
	ctx.lr = 0x8269D850;
	sub_82466E20(ctx, base);
	// 8269D850: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8269D854: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8269D858: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8269D85C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8269D860(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8269D860 size=112
    let mut pc: u32 = 0x8269D860;
    'dispatch: loop {
        match pc {
            0x8269D860 => {
    //   block [0x8269D860..0x8269D8D0)
	// 8269D860: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8269D864: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8269D868: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8269D86C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269D870: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 8269D874: 38AA4A00  addi r5, r10, 0x4a00
	ctx.r[5].s64 = ctx.r[10].s64 + 18944;
	// 8269D878: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8269D87C: 390B1908  addi r8, r11, 0x1908
	ctx.r[8].s64 = ctx.r[11].s64 + 6408;
	// 8269D880: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8269D884: 388ABF68  addi r4, r10, -0x4098
	ctx.r[4].s64 = ctx.r[10].s64 + -16536;
	// 8269D888: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8269D88C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269D890: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8269D894: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8269D898: 386A4B50  addi r3, r10, 0x4b50
	ctx.r[3].s64 = ctx.r[10].s64 + 19280;
	// 8269D89C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8269D8A0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8269D8A4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8269D8A8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8269D8AC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8269D8B0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8269D8B4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8269D8B8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8269D8BC: 4BDC9565  bl 0x82466e20
	ctx.lr = 0x8269D8C0;
	sub_82466E20(ctx, base);
	// 8269D8C0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8269D8C4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8269D8C8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8269D8CC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8269D8D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8269D8D0 size=112
    let mut pc: u32 = 0x8269D8D0;
    'dispatch: loop {
        match pc {
            0x8269D8D0 => {
    //   block [0x8269D8D0..0x8269D940)
	// 8269D8D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8269D8D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8269D8D8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8269D8DC: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269D8E0: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 8269D8E4: 38AA4BB0  addi r5, r10, 0x4bb0
	ctx.r[5].s64 = ctx.r[10].s64 + 19376;
	// 8269D8E8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8269D8EC: 390B1920  addi r8, r11, 0x1920
	ctx.r[8].s64 = ctx.r[11].s64 + 6432;
	// 8269D8F0: 39200005  li r9, 5
	ctx.r[9].s64 = 5;
	// 8269D8F4: 388ABF88  addi r4, r10, -0x4078
	ctx.r[4].s64 = ctx.r[10].s64 + -16504;
	// 8269D8F8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8269D8FC: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269D900: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8269D904: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8269D908: 386A4B80  addi r3, r10, 0x4b80
	ctx.r[3].s64 = ctx.r[10].s64 + 19328;
	// 8269D90C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8269D910: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8269D914: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8269D918: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8269D91C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8269D920: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8269D924: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8269D928: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8269D92C: 4BDC94F5  bl 0x82466e20
	ctx.lr = 0x8269D930;
	sub_82466E20(ctx, base);
	// 8269D930: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8269D934: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8269D938: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8269D93C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8269D940(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8269D940 size=100
    let mut pc: u32 = 0x8269D940;
    'dispatch: loop {
        match pc {
            0x8269D940 => {
    //   block [0x8269D940..0x8269D9A4)
	// 8269D940: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8269D944: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8269D948: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8269D94C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269D950: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8269D954: 38AA2B10  addi r5, r10, 0x2b10
	ctx.r[5].s64 = ctx.r[10].s64 + 11024;
	// 8269D958: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8269D95C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8269D960: 388ABFA4  addi r4, r10, -0x405c
	ctx.r[4].s64 = ctx.r[10].s64 + -16476;
	// 8269D964: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269D968: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8269D96C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8269D970: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8269D974: 386A4BB0  addi r3, r10, 0x4bb0
	ctx.r[3].s64 = ctx.r[10].s64 + 19376;
	// 8269D978: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8269D97C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8269D980: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8269D984: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8269D988: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8269D98C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8269D990: 4BDC9491  bl 0x82466e20
	ctx.lr = 0x8269D994;
	sub_82466E20(ctx, base);
	// 8269D994: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8269D998: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8269D99C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8269D9A0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8269D9A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8269D9A8 size=24
    let mut pc: u32 = 0x8269D9A8;
    'dispatch: loop {
        match pc {
            0x8269D9A8 => {
    //   block [0x8269D9A8..0x8269D9C0)
	// 8269D9A8: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 8269D9AC: 3D40827C  lis r10, -0x7d84
	ctx.r[10].s64 = -2105802752;
	// 8269D9B0: 394A7368  addi r10, r10, 0x7368
	ctx.r[10].s64 = ctx.r[10].s64 + 29544;
	// 8269D9B4: 816B1998  lwz r11, 0x1998(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(6552 as u32) ) } as u64;
	// 8269D9B8: 916A0050  stw r11, 0x50(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 8269D9BC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8269D9C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8269D9C0 size=116
    let mut pc: u32 = 0x8269D9C0;
    'dispatch: loop {
        match pc {
            0x8269D9C0 => {
    //   block [0x8269D9C0..0x8269DA34)
	// 8269D9C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8269D9C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8269D9C8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8269D9CC: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 8269D9D0: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 8269D9D4: 390B7368  addi r8, r11, 0x7368
	ctx.r[8].s64 = ctx.r[11].s64 + 29544;
	// 8269D9D8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8269D9DC: 392AA0F8  addi r9, r10, -0x5f08
	ctx.r[9].s64 = ctx.r[10].s64 + -24328;
	// 8269D9E0: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269D9E4: 38E00005  li r7, 5
	ctx.r[7].s64 = 5;
	// 8269D9E8: 38AA4A00  addi r5, r10, 0x4a00
	ctx.r[5].s64 = ctx.r[10].s64 + 18944;
	// 8269D9EC: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8269D9F0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8269D9F4: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8269D9F8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8269D9FC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8269DA00: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8269DA04: 3D608297  lis r11, -0x7d69
	ctx.r[11].s64 = -2104033280;
	// 8269DA08: 388ABFB8  addi r4, r10, -0x4048
	ctx.r[4].s64 = ctx.r[10].s64 + -16456;
	// 8269DA0C: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8269DA10: 386B4BE0  addi r3, r11, 0x4be0
	ctx.r[3].s64 = ctx.r[11].s64 + 19424;
	// 8269DA14: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8269DA18: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8269DA1C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8269DA20: 4BDC9401  bl 0x82466e20
	ctx.lr = 0x8269DA24;
	sub_82466E20(ctx, base);
	// 8269DA24: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8269DA28: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8269DA2C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8269DA30: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8269DA38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8269DA38 size=108
    let mut pc: u32 = 0x8269DA38;
    'dispatch: loop {
        match pc {
            0x8269DA38 => {
    //   block [0x8269DA38..0x8269DAA4)
	// 8269DA38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8269DA3C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8269DA40: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8269DA44: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 8269DA48: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8269DA4C: 38EB199C  addi r7, r11, 0x199c
	ctx.r[7].s64 = ctx.r[11].s64 + 6556;
	// 8269DA50: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 8269DA54: 388ABFD8  addi r4, r10, -0x4028
	ctx.r[4].s64 = ctx.r[10].s64 + -16424;
	// 8269DA58: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8269DA5C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269DA60: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8269DA64: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8269DA68: 386A4C10  addi r3, r10, 0x4c10
	ctx.r[3].s64 = ctx.r[10].s64 + 19472;
	// 8269DA6C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8269DA70: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8269DA74: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8269DA78: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8269DA7C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8269DA80: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8269DA84: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8269DA88: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8269DA8C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8269DA90: 4BDC9391  bl 0x82466e20
	ctx.lr = 0x8269DA94;
	sub_82466E20(ctx, base);
	// 8269DA94: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8269DA98: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8269DA9C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8269DAA0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8269DAA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8269DAA8 size=112
    let mut pc: u32 = 0x8269DAA8;
    'dispatch: loop {
        match pc {
            0x8269DAA8 => {
    //   block [0x8269DAA8..0x8269DB18)
	// 8269DAA8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8269DAAC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8269DAB0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8269DAB4: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269DAB8: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 8269DABC: 38AA4A00  addi r5, r10, 0x4a00
	ctx.r[5].s64 = ctx.r[10].s64 + 18944;
	// 8269DAC0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8269DAC4: 390B19CC  addi r8, r11, 0x19cc
	ctx.r[8].s64 = ctx.r[11].s64 + 6604;
	// 8269DAC8: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8269DACC: 388ABFFC  addi r4, r10, -0x4004
	ctx.r[4].s64 = ctx.r[10].s64 + -16388;
	// 8269DAD0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8269DAD4: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269DAD8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8269DADC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8269DAE0: 386A4C40  addi r3, r10, 0x4c40
	ctx.r[3].s64 = ctx.r[10].s64 + 19520;
	// 8269DAE4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8269DAE8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8269DAEC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8269DAF0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8269DAF4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8269DAF8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8269DAFC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8269DB00: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8269DB04: 4BDC931D  bl 0x82466e20
	ctx.lr = 0x8269DB08;
	sub_82466E20(ctx, base);
	// 8269DB08: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8269DB0C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8269DB10: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8269DB14: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8269DB18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8269DB18 size=112
    let mut pc: u32 = 0x8269DB18;
    'dispatch: loop {
        match pc {
            0x8269DB18 => {
    //   block [0x8269DB18..0x8269DB88)
	// 8269DB18: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8269DB1C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8269DB20: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8269DB24: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 8269DB28: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 8269DB2C: 392AA11C  addi r9, r10, -0x5ee4
	ctx.r[9].s64 = ctx.r[10].s64 + -24292;
	// 8269DB30: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8269DB34: 390B19E8  addi r8, r11, 0x19e8
	ctx.r[8].s64 = ctx.r[11].s64 + 6632;
	// 8269DB38: 38E00007  li r7, 7
	ctx.r[7].s64 = 7;
	// 8269DB3C: 388AC01C  addi r4, r10, -0x3fe4
	ctx.r[4].s64 = ctx.r[10].s64 + -16356;
	// 8269DB40: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8269DB44: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269DB48: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8269DB4C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8269DB50: 386A4C70  addi r3, r10, 0x4c70
	ctx.r[3].s64 = ctx.r[10].s64 + 19568;
	// 8269DB54: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8269DB58: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8269DB5C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8269DB60: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8269DB64: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8269DB68: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8269DB6C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8269DB70: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8269DB74: 4BDC92AD  bl 0x82466e20
	ctx.lr = 0x8269DB78;
	sub_82466E20(ctx, base);
	// 8269DB78: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8269DB7C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8269DB80: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8269DB84: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8269DB88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8269DB88 size=112
    let mut pc: u32 = 0x8269DB88;
    'dispatch: loop {
        match pc {
            0x8269DB88 => {
    //   block [0x8269DB88..0x8269DBF8)
	// 8269DB88: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8269DB8C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8269DB90: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8269DB94: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269DB98: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 8269DB9C: 38AA4A00  addi r5, r10, 0x4a00
	ctx.r[5].s64 = ctx.r[10].s64 + 18944;
	// 8269DBA0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8269DBA4: 390B1A90  addi r8, r11, 0x1a90
	ctx.r[8].s64 = ctx.r[11].s64 + 6800;
	// 8269DBA8: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8269DBAC: 388AC03C  addi r4, r10, -0x3fc4
	ctx.r[4].s64 = ctx.r[10].s64 + -16324;
	// 8269DBB0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8269DBB4: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269DBB8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8269DBBC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8269DBC0: 386A4CA0  addi r3, r10, 0x4ca0
	ctx.r[3].s64 = ctx.r[10].s64 + 19616;
	// 8269DBC4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8269DBC8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8269DBCC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8269DBD0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8269DBD4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8269DBD8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8269DBDC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8269DBE0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8269DBE4: 4BDC923D  bl 0x82466e20
	ctx.lr = 0x8269DBE8;
	sub_82466E20(ctx, base);
	// 8269DBE8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8269DBEC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8269DBF0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8269DBF4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8269DBF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8269DBF8 size=112
    let mut pc: u32 = 0x8269DBF8;
    'dispatch: loop {
        match pc {
            0x8269DBF8 => {
    //   block [0x8269DBF8..0x8269DC68)
	// 8269DBF8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8269DBFC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8269DC00: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8269DC04: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 8269DC08: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 8269DC0C: 392AA174  addi r9, r10, -0x5e8c
	ctx.r[9].s64 = ctx.r[10].s64 + -24204;
	// 8269DC10: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8269DC14: 390B1AB0  addi r8, r11, 0x1ab0
	ctx.r[8].s64 = ctx.r[11].s64 + 6832;
	// 8269DC18: 38E00007  li r7, 7
	ctx.r[7].s64 = 7;
	// 8269DC1C: 388AC058  addi r4, r10, -0x3fa8
	ctx.r[4].s64 = ctx.r[10].s64 + -16296;
	// 8269DC20: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8269DC24: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269DC28: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8269DC2C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8269DC30: 386A4CD0  addi r3, r10, 0x4cd0
	ctx.r[3].s64 = ctx.r[10].s64 + 19664;
	// 8269DC34: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8269DC38: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8269DC3C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8269DC40: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8269DC44: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8269DC48: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8269DC4C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8269DC50: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8269DC54: 4BDC91CD  bl 0x82466e20
	ctx.lr = 0x8269DC58;
	sub_82466E20(ctx, base);
	// 8269DC58: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8269DC5C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8269DC60: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8269DC64: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8269DC68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8269DC68 size=116
    let mut pc: u32 = 0x8269DC68;
    'dispatch: loop {
        match pc {
            0x8269DC68 => {
    //   block [0x8269DC68..0x8269DCDC)
	// 8269DC68: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8269DC6C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8269DC70: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8269DC74: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 8269DC78: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 8269DC7C: 390B1B58  addi r8, r11, 0x1b58
	ctx.r[8].s64 = ctx.r[11].s64 + 7000;
	// 8269DC80: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8269DC84: 392AA148  addi r9, r10, -0x5eb8
	ctx.r[9].s64 = ctx.r[10].s64 + -24248;
	// 8269DC88: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269DC8C: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
	// 8269DC90: 38AA4A00  addi r5, r10, 0x4a00
	ctx.r[5].s64 = ctx.r[10].s64 + 18944;
	// 8269DC94: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8269DC98: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8269DC9C: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8269DCA0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8269DCA4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8269DCA8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8269DCAC: 3D608297  lis r11, -0x7d69
	ctx.r[11].s64 = -2104033280;
	// 8269DCB0: 388AC078  addi r4, r10, -0x3f88
	ctx.r[4].s64 = ctx.r[10].s64 + -16264;
	// 8269DCB4: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8269DCB8: 386B4D00  addi r3, r11, 0x4d00
	ctx.r[3].s64 = ctx.r[11].s64 + 19712;
	// 8269DCBC: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8269DCC0: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8269DCC4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8269DCC8: 4BDC9159  bl 0x82466e20
	ctx.lr = 0x8269DCCC;
	sub_82466E20(ctx, base);
	// 8269DCCC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8269DCD0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8269DCD4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8269DCD8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8269DCE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8269DCE0 size=108
    let mut pc: u32 = 0x8269DCE0;
    'dispatch: loop {
        match pc {
            0x8269DCE0 => {
    //   block [0x8269DCE0..0x8269DD4C)
	// 8269DCE0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8269DCE4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8269DCE8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8269DCEC: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 8269DCF0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8269DCF4: 38EB1B70  addi r7, r11, 0x1b70
	ctx.r[7].s64 = ctx.r[11].s64 + 7024;
	// 8269DCF8: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 8269DCFC: 388AC094  addi r4, r10, -0x3f6c
	ctx.r[4].s64 = ctx.r[10].s64 + -16236;
	// 8269DD00: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8269DD04: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269DD08: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8269DD0C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8269DD10: 386A4D30  addi r3, r10, 0x4d30
	ctx.r[3].s64 = ctx.r[10].s64 + 19760;
	// 8269DD14: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8269DD18: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8269DD1C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8269DD20: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8269DD24: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8269DD28: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8269DD2C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8269DD30: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8269DD34: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8269DD38: 4BDC90E9  bl 0x82466e20
	ctx.lr = 0x8269DD3C;
	sub_82466E20(ctx, base);
	// 8269DD3C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8269DD40: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8269DD44: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8269DD48: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8269DD50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8269DD50 size=112
    let mut pc: u32 = 0x8269DD50;
    'dispatch: loop {
        match pc {
            0x8269DD50 => {
    //   block [0x8269DD50..0x8269DDC0)
	// 8269DD50: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8269DD54: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8269DD58: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8269DD5C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269DD60: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 8269DD64: 38AA4A00  addi r5, r10, 0x4a00
	ctx.r[5].s64 = ctx.r[10].s64 + 18944;
	// 8269DD68: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8269DD6C: 390B1BA0  addi r8, r11, 0x1ba0
	ctx.r[8].s64 = ctx.r[11].s64 + 7072;
	// 8269DD70: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8269DD74: 388AC0B8  addi r4, r10, -0x3f48
	ctx.r[4].s64 = ctx.r[10].s64 + -16200;
	// 8269DD78: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8269DD7C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269DD80: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8269DD84: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8269DD88: 386A4D60  addi r3, r10, 0x4d60
	ctx.r[3].s64 = ctx.r[10].s64 + 19808;
	// 8269DD8C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8269DD90: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8269DD94: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8269DD98: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8269DD9C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8269DDA0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8269DDA4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8269DDA8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8269DDAC: 4BDC9075  bl 0x82466e20
	ctx.lr = 0x8269DDB0;
	sub_82466E20(ctx, base);
	// 8269DDB0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8269DDB4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8269DDB8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8269DDBC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8269DDC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8269DDC0 size=112
    let mut pc: u32 = 0x8269DDC0;
    'dispatch: loop {
        match pc {
            0x8269DDC0 => {
    //   block [0x8269DDC0..0x8269DE30)
	// 8269DDC0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8269DDC4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8269DDC8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8269DDCC: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 8269DDD0: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 8269DDD4: 392AA1A8  addi r9, r10, -0x5e58
	ctx.r[9].s64 = ctx.r[10].s64 + -24152;
	// 8269DDD8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8269DDDC: 390B1BC0  addi r8, r11, 0x1bc0
	ctx.r[8].s64 = ctx.r[11].s64 + 7104;
	// 8269DDE0: 38E00007  li r7, 7
	ctx.r[7].s64 = 7;
	// 8269DDE4: 388AC0D8  addi r4, r10, -0x3f28
	ctx.r[4].s64 = ctx.r[10].s64 + -16168;
	// 8269DDE8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8269DDEC: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269DDF0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8269DDF4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8269DDF8: 386A4D90  addi r3, r10, 0x4d90
	ctx.r[3].s64 = ctx.r[10].s64 + 19856;
	// 8269DDFC: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8269DE00: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8269DE04: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8269DE08: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8269DE0C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8269DE10: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8269DE14: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8269DE18: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8269DE1C: 4BDC9005  bl 0x82466e20
	ctx.lr = 0x8269DE20;
	sub_82466E20(ctx, base);
	// 8269DE20: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8269DE24: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8269DE28: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8269DE2C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8269DE30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8269DE30 size=112
    let mut pc: u32 = 0x8269DE30;
    'dispatch: loop {
        match pc {
            0x8269DE30 => {
    //   block [0x8269DE30..0x8269DEA0)
	// 8269DE30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8269DE34: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8269DE38: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8269DE3C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269DE40: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 8269DE44: 38AA4A00  addi r5, r10, 0x4a00
	ctx.r[5].s64 = ctx.r[10].s64 + 18944;
	// 8269DE48: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8269DE4C: 390B1C68  addi r8, r11, 0x1c68
	ctx.r[8].s64 = ctx.r[11].s64 + 7272;
	// 8269DE50: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 8269DE54: 388AC0F4  addi r4, r10, -0x3f0c
	ctx.r[4].s64 = ctx.r[10].s64 + -16140;
	// 8269DE58: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8269DE5C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269DE60: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8269DE64: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8269DE68: 386A4DC0  addi r3, r10, 0x4dc0
	ctx.r[3].s64 = ctx.r[10].s64 + 19904;
	// 8269DE6C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8269DE70: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8269DE74: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8269DE78: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8269DE7C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8269DE80: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8269DE84: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8269DE88: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8269DE8C: 4BDC8F95  bl 0x82466e20
	ctx.lr = 0x8269DE90;
	sub_82466E20(ctx, base);
	// 8269DE90: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8269DE94: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8269DE98: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8269DE9C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8269DEA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8269DEA0 size=108
    let mut pc: u32 = 0x8269DEA0;
    'dispatch: loop {
        match pc {
            0x8269DEA0 => {
    //   block [0x8269DEA0..0x8269DF0C)
	// 8269DEA0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8269DEA4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8269DEA8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8269DEAC: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 8269DEB0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8269DEB4: 38EB1CB0  addi r7, r11, 0x1cb0
	ctx.r[7].s64 = ctx.r[11].s64 + 7344;
	// 8269DEB8: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 8269DEBC: 388AC10C  addi r4, r10, -0x3ef4
	ctx.r[4].s64 = ctx.r[10].s64 + -16116;
	// 8269DEC0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8269DEC4: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269DEC8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8269DECC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8269DED0: 386A4DF0  addi r3, r10, 0x4df0
	ctx.r[3].s64 = ctx.r[10].s64 + 19952;
	// 8269DED4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8269DED8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8269DEDC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8269DEE0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8269DEE4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8269DEE8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8269DEEC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8269DEF0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8269DEF4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8269DEF8: 4BDC8F29  bl 0x82466e20
	ctx.lr = 0x8269DEFC;
	sub_82466E20(ctx, base);
	// 8269DEFC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8269DF00: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8269DF04: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8269DF08: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8269DF10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8269DF10 size=112
    let mut pc: u32 = 0x8269DF10;
    'dispatch: loop {
        match pc {
            0x8269DF10 => {
    //   block [0x8269DF10..0x8269DF80)
	// 8269DF10: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8269DF14: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8269DF18: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8269DF1C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269DF20: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 8269DF24: 38AA4A00  addi r5, r10, 0x4a00
	ctx.r[5].s64 = ctx.r[10].s64 + 18944;
	// 8269DF28: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8269DF2C: 390B1CE0  addi r8, r11, 0x1ce0
	ctx.r[8].s64 = ctx.r[11].s64 + 7392;
	// 8269DF30: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8269DF34: 388AC130  addi r4, r10, -0x3ed0
	ctx.r[4].s64 = ctx.r[10].s64 + -16080;
	// 8269DF38: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8269DF3C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269DF40: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8269DF44: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8269DF48: 386A4E20  addi r3, r10, 0x4e20
	ctx.r[3].s64 = ctx.r[10].s64 + 20000;
	// 8269DF4C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8269DF50: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8269DF54: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8269DF58: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8269DF5C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8269DF60: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8269DF64: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8269DF68: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8269DF6C: 4BDC8EB5  bl 0x82466e20
	ctx.lr = 0x8269DF70;
	sub_82466E20(ctx, base);
	// 8269DF70: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8269DF74: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8269DF78: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8269DF7C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8269DF80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8269DF80 size=112
    let mut pc: u32 = 0x8269DF80;
    'dispatch: loop {
        match pc {
            0x8269DF80 => {
    //   block [0x8269DF80..0x8269DFF0)
	// 8269DF80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8269DF84: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8269DF88: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8269DF8C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269DF90: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 8269DF94: 38AA4A00  addi r5, r10, 0x4a00
	ctx.r[5].s64 = ctx.r[10].s64 + 18944;
	// 8269DF98: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8269DF9C: 390B1CF8  addi r8, r11, 0x1cf8
	ctx.r[8].s64 = ctx.r[11].s64 + 7416;
	// 8269DFA0: 39200008  li r9, 8
	ctx.r[9].s64 = 8;
	// 8269DFA4: 388AC14C  addi r4, r10, -0x3eb4
	ctx.r[4].s64 = ctx.r[10].s64 + -16052;
	// 8269DFA8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8269DFAC: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269DFB0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8269DFB4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8269DFB8: 386A4E50  addi r3, r10, 0x4e50
	ctx.r[3].s64 = ctx.r[10].s64 + 20048;
	// 8269DFBC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8269DFC0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8269DFC4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8269DFC8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8269DFCC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8269DFD0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8269DFD4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8269DFD8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8269DFDC: 4BDC8E45  bl 0x82466e20
	ctx.lr = 0x8269DFE0;
	sub_82466E20(ctx, base);
	// 8269DFE0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8269DFE4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8269DFE8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8269DFEC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8269DFF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8269DFF0 size=100
    let mut pc: u32 = 0x8269DFF0;
    'dispatch: loop {
        match pc {
            0x8269DFF0 => {
    //   block [0x8269DFF0..0x8269E054)
	// 8269DFF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8269DFF4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8269DFF8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8269DFFC: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269E000: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8269E004: 38AA4A00  addi r5, r10, 0x4a00
	ctx.r[5].s64 = ctx.r[10].s64 + 18944;
	// 8269E008: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8269E00C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8269E010: 388AC168  addi r4, r10, -0x3e98
	ctx.r[4].s64 = ctx.r[10].s64 + -16024;
	// 8269E014: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269E018: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8269E01C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8269E020: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8269E024: 386A4E80  addi r3, r10, 0x4e80
	ctx.r[3].s64 = ctx.r[10].s64 + 20096;
	// 8269E028: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8269E02C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8269E030: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8269E034: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8269E038: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8269E03C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8269E040: 4BDC8DE1  bl 0x82466e20
	ctx.lr = 0x8269E044;
	sub_82466E20(ctx, base);
	// 8269E044: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8269E048: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8269E04C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8269E050: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8269E058(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8269E058 size=112
    let mut pc: u32 = 0x8269E058;
    'dispatch: loop {
        match pc {
            0x8269E058 => {
    //   block [0x8269E058..0x8269E0C8)
	// 8269E058: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8269E05C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8269E060: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8269E064: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269E068: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 8269E06C: 38AA4A30  addi r5, r10, 0x4a30
	ctx.r[5].s64 = ctx.r[10].s64 + 18992;
	// 8269E070: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8269E074: 390B1DB8  addi r8, r11, 0x1db8
	ctx.r[8].s64 = ctx.r[11].s64 + 7608;
	// 8269E078: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 8269E07C: 388AC180  addi r4, r10, -0x3e80
	ctx.r[4].s64 = ctx.r[10].s64 + -16000;
	// 8269E080: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8269E084: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269E088: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8269E08C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8269E090: 386A4EB0  addi r3, r10, 0x4eb0
	ctx.r[3].s64 = ctx.r[10].s64 + 20144;
	// 8269E094: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8269E098: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8269E09C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8269E0A0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8269E0A4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8269E0A8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8269E0AC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8269E0B0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8269E0B4: 4BDC8D6D  bl 0x82466e20
	ctx.lr = 0x8269E0B8;
	sub_82466E20(ctx, base);
	// 8269E0B8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8269E0BC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8269E0C0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8269E0C4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8269E0C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8269E0C8 size=112
    let mut pc: u32 = 0x8269E0C8;
    'dispatch: loop {
        match pc {
            0x8269E0C8 => {
    //   block [0x8269E0C8..0x8269E138)
	// 8269E0C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8269E0CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8269E0D0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8269E0D4: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269E0D8: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 8269E0DC: 38AA48B0  addi r5, r10, 0x48b0
	ctx.r[5].s64 = ctx.r[10].s64 + 18608;
	// 8269E0E0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8269E0E4: 390B1DE8  addi r8, r11, 0x1de8
	ctx.r[8].s64 = ctx.r[11].s64 + 7656;
	// 8269E0E8: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8269E0EC: 388AC19C  addi r4, r10, -0x3e64
	ctx.r[4].s64 = ctx.r[10].s64 + -15972;
	// 8269E0F0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8269E0F4: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269E0F8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8269E0FC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8269E100: 386A4EE0  addi r3, r10, 0x4ee0
	ctx.r[3].s64 = ctx.r[10].s64 + 20192;
	// 8269E104: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8269E108: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8269E10C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8269E110: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8269E114: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8269E118: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8269E11C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8269E120: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8269E124: 4BDC8CFD  bl 0x82466e20
	ctx.lr = 0x8269E128;
	sub_82466E20(ctx, base);
	// 8269E128: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8269E12C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8269E130: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8269E134: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8269E138(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8269E138 size=108
    let mut pc: u32 = 0x8269E138;
    'dispatch: loop {
        match pc {
            0x8269E138 => {
    //   block [0x8269E138..0x8269E1A4)
	// 8269E138: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8269E13C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8269E140: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8269E144: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 8269E148: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8269E14C: 38EB1E00  addi r7, r11, 0x1e00
	ctx.r[7].s64 = ctx.r[11].s64 + 7680;
	// 8269E150: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 8269E154: 388AC1C0  addi r4, r10, -0x3e40
	ctx.r[4].s64 = ctx.r[10].s64 + -15936;
	// 8269E158: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8269E15C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269E160: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8269E164: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8269E168: 386A4F10  addi r3, r10, 0x4f10
	ctx.r[3].s64 = ctx.r[10].s64 + 20240;
	// 8269E16C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8269E170: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8269E174: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8269E178: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8269E17C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8269E180: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8269E184: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8269E188: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8269E18C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8269E190: 4BDC8C91  bl 0x82466e20
	ctx.lr = 0x8269E194;
	sub_82466E20(ctx, base);
	// 8269E194: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8269E198: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8269E19C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8269E1A0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8269E1A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8269E1A8 size=112
    let mut pc: u32 = 0x8269E1A8;
    'dispatch: loop {
        match pc {
            0x8269E1A8 => {
    //   block [0x8269E1A8..0x8269E218)
	// 8269E1A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8269E1AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8269E1B0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8269E1B4: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269E1B8: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 8269E1BC: 38AA4E80  addi r5, r10, 0x4e80
	ctx.r[5].s64 = ctx.r[10].s64 + 20096;
	// 8269E1C0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8269E1C4: 390B1E30  addi r8, r11, 0x1e30
	ctx.r[8].s64 = ctx.r[11].s64 + 7728;
	// 8269E1C8: 39200006  li r9, 6
	ctx.r[9].s64 = 6;
	// 8269E1CC: 388AC1E8  addi r4, r10, -0x3e18
	ctx.r[4].s64 = ctx.r[10].s64 + -15896;
	// 8269E1D0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8269E1D4: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269E1D8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8269E1DC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8269E1E0: 386A4F40  addi r3, r10, 0x4f40
	ctx.r[3].s64 = ctx.r[10].s64 + 20288;
	// 8269E1E4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8269E1E8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8269E1EC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8269E1F0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8269E1F4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8269E1F8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8269E1FC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8269E200: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8269E204: 4BDC8C1D  bl 0x82466e20
	ctx.lr = 0x8269E208;
	sub_82466E20(ctx, base);
	// 8269E208: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8269E20C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8269E210: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8269E214: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8269E218(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8269E218 size=112
    let mut pc: u32 = 0x8269E218;
    'dispatch: loop {
        match pc {
            0x8269E218 => {
    //   block [0x8269E218..0x8269E288)
	// 8269E218: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8269E21C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8269E220: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8269E224: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 8269E228: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 8269E22C: 392AA1D4  addi r9, r10, -0x5e2c
	ctx.r[9].s64 = ctx.r[10].s64 + -24108;
	// 8269E230: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8269E234: 390B1EC0  addi r8, r11, 0x1ec0
	ctx.r[8].s64 = ctx.r[11].s64 + 7872;
	// 8269E238: 38E00003  li r7, 3
	ctx.r[7].s64 = 3;
	// 8269E23C: 388AC200  addi r4, r10, -0x3e00
	ctx.r[4].s64 = ctx.r[10].s64 + -15872;
	// 8269E240: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8269E244: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269E248: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8269E24C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8269E250: 386A4F70  addi r3, r10, 0x4f70
	ctx.r[3].s64 = ctx.r[10].s64 + 20336;
	// 8269E254: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8269E258: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8269E25C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8269E260: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8269E264: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8269E268: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8269E26C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8269E270: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8269E274: 4BDC8BAD  bl 0x82466e20
	ctx.lr = 0x8269E278;
	sub_82466E20(ctx, base);
	// 8269E278: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8269E27C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8269E280: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8269E284: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8269E288(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8269E288 size=112
    let mut pc: u32 = 0x8269E288;
    'dispatch: loop {
        match pc {
            0x8269E288 => {
    //   block [0x8269E288..0x8269E2F8)
	// 8269E288: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8269E28C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8269E290: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8269E294: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269E298: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 8269E29C: 38AA4A00  addi r5, r10, 0x4a00
	ctx.r[5].s64 = ctx.r[10].s64 + 18944;
	// 8269E2A0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8269E2A4: 390B1F08  addi r8, r11, 0x1f08
	ctx.r[8].s64 = ctx.r[11].s64 + 7944;
	// 8269E2A8: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8269E2AC: 388AC218  addi r4, r10, -0x3de8
	ctx.r[4].s64 = ctx.r[10].s64 + -15848;
	// 8269E2B0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8269E2B4: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269E2B8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8269E2BC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8269E2C0: 386A4FA0  addi r3, r10, 0x4fa0
	ctx.r[3].s64 = ctx.r[10].s64 + 20384;
	// 8269E2C4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8269E2C8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8269E2CC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8269E2D0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8269E2D4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8269E2D8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8269E2DC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8269E2E0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8269E2E4: 4BDC8B3D  bl 0x82466e20
	ctx.lr = 0x8269E2E8;
	sub_82466E20(ctx, base);
	// 8269E2E8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8269E2EC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8269E2F0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8269E2F4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8269E2F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8269E2F8 size=108
    let mut pc: u32 = 0x8269E2F8;
    'dispatch: loop {
        match pc {
            0x8269E2F8 => {
    //   block [0x8269E2F8..0x8269E364)
	// 8269E2F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8269E2FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8269E300: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8269E304: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 8269E308: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8269E30C: 38EB1F20  addi r7, r11, 0x1f20
	ctx.r[7].s64 = ctx.r[11].s64 + 7968;
	// 8269E310: 39000006  li r8, 6
	ctx.r[8].s64 = 6;
	// 8269E314: 388AC22C  addi r4, r10, -0x3dd4
	ctx.r[4].s64 = ctx.r[10].s64 + -15828;
	// 8269E318: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8269E31C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269E320: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8269E324: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8269E328: 386A4FD0  addi r3, r10, 0x4fd0
	ctx.r[3].s64 = ctx.r[10].s64 + 20432;
	// 8269E32C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8269E330: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8269E334: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8269E338: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8269E33C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8269E340: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8269E344: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8269E348: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8269E34C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8269E350: 4BDC8AD1  bl 0x82466e20
	ctx.lr = 0x8269E354;
	sub_82466E20(ctx, base);
	// 8269E354: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8269E358: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8269E35C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8269E360: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8269E368(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8269E368 size=116
    let mut pc: u32 = 0x8269E368;
    'dispatch: loop {
        match pc {
            0x8269E368 => {
    //   block [0x8269E368..0x8269E3DC)
	// 8269E368: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8269E36C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8269E370: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8269E374: 3D40827C  lis r10, -0x7d84
	ctx.r[10].s64 = -2105802752;
	// 8269E378: 38E00009  li r7, 9
	ctx.r[7].s64 = 9;
	// 8269E37C: 390A1FB0  addi r8, r10, 0x1fb0
	ctx.r[8].s64 = ctx.r[10].s64 + 8112;
	// 8269E380: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269E384: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 8269E388: 38AA4E80  addi r5, r10, 0x4e80
	ctx.r[5].s64 = ctx.r[10].s64 + 20096;
	// 8269E38C: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8269E390: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8269E394: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8269E398: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8269E39C: 388AC250  addi r4, r10, -0x3db0
	ctx.r[4].s64 = ctx.r[10].s64 + -15792;
	// 8269E3A0: 396BA1E8  addi r11, r11, -0x5e18
	ctx.r[11].s64 = ctx.r[11].s64 + -24088;
	// 8269E3A4: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269E3A8: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8269E3AC: 386A5000  addi r3, r10, 0x5000
	ctx.r[3].s64 = ctx.r[10].s64 + 20480;
	// 8269E3B0: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 8269E3B4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8269E3B8: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 8269E3BC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8269E3C0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8269E3C4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8269E3C8: 4BDC8A59  bl 0x82466e20
	ctx.lr = 0x8269E3CC;
	sub_82466E20(ctx, base);
	// 8269E3CC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8269E3D0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8269E3D4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8269E3D8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8269E3E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8269E3E0 size=112
    let mut pc: u32 = 0x8269E3E0;
    'dispatch: loop {
        match pc {
            0x8269E3E0 => {
    //   block [0x8269E3E0..0x8269E450)
	// 8269E3E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8269E3E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8269E3E8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8269E3EC: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 8269E3F0: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 8269E3F4: 392AA234  addi r9, r10, -0x5dcc
	ctx.r[9].s64 = ctx.r[10].s64 + -24012;
	// 8269E3F8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8269E3FC: 390B2090  addi r8, r11, 0x2090
	ctx.r[8].s64 = ctx.r[11].s64 + 8336;
	// 8269E400: 38E00004  li r7, 4
	ctx.r[7].s64 = 4;
	// 8269E404: 388AC264  addi r4, r10, -0x3d9c
	ctx.r[4].s64 = ctx.r[10].s64 + -15772;
	// 8269E408: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8269E40C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269E410: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8269E414: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8269E418: 386A5030  addi r3, r10, 0x5030
	ctx.r[3].s64 = ctx.r[10].s64 + 20528;
	// 8269E41C: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8269E420: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8269E424: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8269E428: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8269E42C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8269E430: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8269E434: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8269E438: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8269E43C: 4BDC89E5  bl 0x82466e20
	ctx.lr = 0x8269E440;
	sub_82466E20(ctx, base);
	// 8269E440: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8269E444: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8269E448: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8269E44C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8269E450(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8269E450 size=112
    let mut pc: u32 = 0x8269E450;
    'dispatch: loop {
        match pc {
            0x8269E450 => {
    //   block [0x8269E450..0x8269E4C0)
	// 8269E450: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8269E454: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8269E458: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8269E45C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269E460: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 8269E464: 38AA4A00  addi r5, r10, 0x4a00
	ctx.r[5].s64 = ctx.r[10].s64 + 18944;
	// 8269E468: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8269E46C: 390B20F0  addi r8, r11, 0x20f0
	ctx.r[8].s64 = ctx.r[11].s64 + 8432;
	// 8269E470: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8269E474: 388AC280  addi r4, r10, -0x3d80
	ctx.r[4].s64 = ctx.r[10].s64 + -15744;
	// 8269E478: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8269E47C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269E480: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8269E484: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8269E488: 386A5060  addi r3, r10, 0x5060
	ctx.r[3].s64 = ctx.r[10].s64 + 20576;
	// 8269E48C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8269E490: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8269E494: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8269E498: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8269E49C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8269E4A0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8269E4A4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8269E4A8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8269E4AC: 4BDC8975  bl 0x82466e20
	ctx.lr = 0x8269E4B0;
	sub_82466E20(ctx, base);
	// 8269E4B0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8269E4B4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8269E4B8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8269E4BC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8269E4C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8269E4C0 size=108
    let mut pc: u32 = 0x8269E4C0;
    'dispatch: loop {
        match pc {
            0x8269E4C0 => {
    //   block [0x8269E4C0..0x8269E52C)
	// 8269E4C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8269E4C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8269E4C8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8269E4CC: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 8269E4D0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8269E4D4: 38EB2108  addi r7, r11, 0x2108
	ctx.r[7].s64 = ctx.r[11].s64 + 8456;
	// 8269E4D8: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 8269E4DC: 388AC298  addi r4, r10, -0x3d68
	ctx.r[4].s64 = ctx.r[10].s64 + -15720;
	// 8269E4E0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8269E4E4: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269E4E8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8269E4EC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8269E4F0: 386A5090  addi r3, r10, 0x5090
	ctx.r[3].s64 = ctx.r[10].s64 + 20624;
	// 8269E4F4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8269E4F8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8269E4FC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8269E500: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8269E504: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8269E508: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8269E50C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8269E510: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8269E514: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8269E518: 4BDC8909  bl 0x82466e20
	ctx.lr = 0x8269E51C;
	sub_82466E20(ctx, base);
	// 8269E51C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8269E520: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8269E524: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8269E528: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8269E530(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8269E530 size=112
    let mut pc: u32 = 0x8269E530;
    'dispatch: loop {
        match pc {
            0x8269E530 => {
    //   block [0x8269E530..0x8269E5A0)
	// 8269E530: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8269E534: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8269E538: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8269E53C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269E540: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 8269E544: 38AA4E80  addi r5, r10, 0x4e80
	ctx.r[5].s64 = ctx.r[10].s64 + 20096;
	// 8269E548: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8269E54C: 390B2150  addi r8, r11, 0x2150
	ctx.r[8].s64 = ctx.r[11].s64 + 8528;
	// 8269E550: 39200005  li r9, 5
	ctx.r[9].s64 = 5;
	// 8269E554: 388AC2C0  addi r4, r10, -0x3d40
	ctx.r[4].s64 = ctx.r[10].s64 + -15680;
	// 8269E558: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8269E55C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269E560: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8269E564: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8269E568: 386A50C0  addi r3, r10, 0x50c0
	ctx.r[3].s64 = ctx.r[10].s64 + 20672;
	// 8269E56C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8269E570: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8269E574: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8269E578: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8269E57C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8269E580: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8269E584: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8269E588: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8269E58C: 4BDC8895  bl 0x82466e20
	ctx.lr = 0x8269E590;
	sub_82466E20(ctx, base);
	// 8269E590: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8269E594: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8269E598: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8269E59C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8269E5A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8269E5A0 size=112
    let mut pc: u32 = 0x8269E5A0;
    'dispatch: loop {
        match pc {
            0x8269E5A0 => {
    //   block [0x8269E5A0..0x8269E610)
	// 8269E5A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8269E5A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8269E5A8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8269E5AC: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269E5B0: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 8269E5B4: 38AA4A00  addi r5, r10, 0x4a00
	ctx.r[5].s64 = ctx.r[10].s64 + 18944;
	// 8269E5B8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8269E5BC: 390B21C8  addi r8, r11, 0x21c8
	ctx.r[8].s64 = ctx.r[11].s64 + 8648;
	// 8269E5C0: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 8269E5C4: 388AC2D8  addi r4, r10, -0x3d28
	ctx.r[4].s64 = ctx.r[10].s64 + -15656;
	// 8269E5C8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8269E5CC: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269E5D0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8269E5D4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8269E5D8: 386A50F0  addi r3, r10, 0x50f0
	ctx.r[3].s64 = ctx.r[10].s64 + 20720;
	// 8269E5DC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8269E5E0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8269E5E4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8269E5E8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8269E5EC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8269E5F0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8269E5F4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8269E5F8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8269E5FC: 4BDC8825  bl 0x82466e20
	ctx.lr = 0x8269E600;
	sub_82466E20(ctx, base);
	// 8269E600: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8269E604: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8269E608: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8269E60C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8269E610(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8269E610 size=108
    let mut pc: u32 = 0x8269E610;
    'dispatch: loop {
        match pc {
            0x8269E610 => {
    //   block [0x8269E610..0x8269E67C)
	// 8269E610: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8269E614: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8269E618: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8269E61C: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 8269E620: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8269E624: 38EB21F8  addi r7, r11, 0x21f8
	ctx.r[7].s64 = ctx.r[11].s64 + 8696;
	// 8269E628: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 8269E62C: 388AC2F4  addi r4, r10, -0x3d0c
	ctx.r[4].s64 = ctx.r[10].s64 + -15628;
	// 8269E630: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8269E634: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269E638: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8269E63C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8269E640: 386A5120  addi r3, r10, 0x5120
	ctx.r[3].s64 = ctx.r[10].s64 + 20768;
	// 8269E644: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8269E648: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8269E64C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8269E650: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8269E654: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8269E658: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8269E65C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8269E660: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8269E664: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8269E668: 4BDC87B9  bl 0x82466e20
	ctx.lr = 0x8269E66C;
	sub_82466E20(ctx, base);
	// 8269E66C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8269E670: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8269E674: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8269E678: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8269E680(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8269E680 size=108
    let mut pc: u32 = 0x8269E680;
    'dispatch: loop {
        match pc {
            0x8269E680 => {
    //   block [0x8269E680..0x8269E6EC)
	// 8269E680: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8269E684: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8269E688: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8269E68C: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 8269E690: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8269E694: 38EB2258  addi r7, r11, 0x2258
	ctx.r[7].s64 = ctx.r[11].s64 + 8792;
	// 8269E698: 39000005  li r8, 5
	ctx.r[8].s64 = 5;
	// 8269E69C: 388AC324  addi r4, r10, -0x3cdc
	ctx.r[4].s64 = ctx.r[10].s64 + -15580;
	// 8269E6A0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8269E6A4: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269E6A8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8269E6AC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8269E6B0: 386A5150  addi r3, r10, 0x5150
	ctx.r[3].s64 = ctx.r[10].s64 + 20816;
	// 8269E6B4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8269E6B8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8269E6BC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8269E6C0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8269E6C4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8269E6C8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8269E6CC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8269E6D0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8269E6D4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8269E6D8: 4BDC8749  bl 0x82466e20
	ctx.lr = 0x8269E6DC;
	sub_82466E20(ctx, base);
	// 8269E6DC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8269E6E0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8269E6E4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8269E6E8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8269E6F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8269E6F0 size=112
    let mut pc: u32 = 0x8269E6F0;
    'dispatch: loop {
        match pc {
            0x8269E6F0 => {
    //   block [0x8269E6F0..0x8269E760)
	// 8269E6F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8269E6F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8269E6F8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8269E6FC: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269E700: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 8269E704: 38AA4A00  addi r5, r10, 0x4a00
	ctx.r[5].s64 = ctx.r[10].s64 + 18944;
	// 8269E708: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8269E70C: 390B22D0  addi r8, r11, 0x22d0
	ctx.r[8].s64 = ctx.r[11].s64 + 8912;
	// 8269E710: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 8269E714: 388AC344  addi r4, r10, -0x3cbc
	ctx.r[4].s64 = ctx.r[10].s64 + -15548;
	// 8269E718: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8269E71C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269E720: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8269E724: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8269E728: 386A5180  addi r3, r10, 0x5180
	ctx.r[3].s64 = ctx.r[10].s64 + 20864;
	// 8269E72C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8269E730: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8269E734: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8269E738: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8269E73C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8269E740: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8269E744: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8269E748: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8269E74C: 4BDC86D5  bl 0x82466e20
	ctx.lr = 0x8269E750;
	sub_82466E20(ctx, base);
	// 8269E750: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8269E754: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8269E758: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8269E75C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8269E760(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8269E760 size=24
    let mut pc: u32 = 0x8269E760;
    'dispatch: loop {
        match pc {
            0x8269E760 => {
    //   block [0x8269E760..0x8269E778)
	// 8269E760: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 8269E764: 3D40827C  lis r10, -0x7d84
	ctx.r[10].s64 = -2105802752;
	// 8269E768: 394A73E0  addi r10, r10, 0x73e0
	ctx.r[10].s64 = ctx.r[10].s64 + 29664;
	// 8269E76C: 816B208C  lwz r11, 0x208c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8332 as u32) ) } as u64;
	// 8269E770: 916A0008  stw r11, 8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 8269E774: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8269E778(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8269E778 size=116
    let mut pc: u32 = 0x8269E778;
    'dispatch: loop {
        match pc {
            0x8269E778 => {
    //   block [0x8269E778..0x8269E7EC)
	// 8269E778: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8269E77C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8269E780: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8269E784: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 8269E788: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 8269E78C: 390B73E0  addi r8, r11, 0x73e0
	ctx.r[8].s64 = ctx.r[11].s64 + 29664;
	// 8269E790: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8269E794: 392AA278  addi r9, r10, -0x5d88
	ctx.r[9].s64 = ctx.r[10].s64 + -23944;
	// 8269E798: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269E79C: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
	// 8269E7A0: 38AA2B10  addi r5, r10, 0x2b10
	ctx.r[5].s64 = ctx.r[10].s64 + 11024;
	// 8269E7A4: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8269E7A8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8269E7AC: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8269E7B0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8269E7B4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8269E7B8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8269E7BC: 3D608297  lis r11, -0x7d69
	ctx.r[11].s64 = -2104033280;
	// 8269E7C0: 388AC360  addi r4, r10, -0x3ca0
	ctx.r[4].s64 = ctx.r[10].s64 + -15520;
	// 8269E7C4: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8269E7C8: 386B51B0  addi r3, r11, 0x51b0
	ctx.r[3].s64 = ctx.r[11].s64 + 20912;
	// 8269E7CC: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8269E7D0: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8269E7D4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8269E7D8: 4BDC8649  bl 0x82466e20
	ctx.lr = 0x8269E7DC;
	sub_82466E20(ctx, base);
	// 8269E7DC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8269E7E0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8269E7E4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8269E7E8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8269E7F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8269E7F0 size=112
    let mut pc: u32 = 0x8269E7F0;
    'dispatch: loop {
        match pc {
            0x8269E7F0 => {
    //   block [0x8269E7F0..0x8269E860)
	// 8269E7F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8269E7F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8269E7F8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8269E7FC: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269E800: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 8269E804: 38AA51B0  addi r5, r10, 0x51b0
	ctx.r[5].s64 = ctx.r[10].s64 + 20912;
	// 8269E808: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8269E80C: 390B2318  addi r8, r11, 0x2318
	ctx.r[8].s64 = ctx.r[11].s64 + 8984;
	// 8269E810: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 8269E814: 388AC374  addi r4, r10, -0x3c8c
	ctx.r[4].s64 = ctx.r[10].s64 + -15500;
	// 8269E818: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8269E81C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269E820: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8269E824: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8269E828: 386A51E0  addi r3, r10, 0x51e0
	ctx.r[3].s64 = ctx.r[10].s64 + 20960;
	// 8269E82C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8269E830: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8269E834: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8269E838: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8269E83C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8269E840: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8269E844: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8269E848: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8269E84C: 4BDC85D5  bl 0x82466e20
	ctx.lr = 0x8269E850;
	sub_82466E20(ctx, base);
	// 8269E850: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8269E854: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8269E858: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8269E85C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8269E860(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8269E860 size=24
    let mut pc: u32 = 0x8269E860;
    'dispatch: loop {
        match pc {
            0x8269E860 => {
    //   block [0x8269E860..0x8269E878)
	// 8269E860: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 8269E864: 3D40827C  lis r10, -0x7d84
	ctx.r[10].s64 = -2105802752;
	// 8269E868: 394A73F8  addi r10, r10, 0x73f8
	ctx.r[10].s64 = ctx.r[10].s64 + 29688;
	// 8269E86C: 816B2348  lwz r11, 0x2348(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(9032 as u32) ) } as u64;
	// 8269E870: 916A0020  stw r11, 0x20(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 8269E874: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8269E878(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8269E878 size=116
    let mut pc: u32 = 0x8269E878;
    'dispatch: loop {
        match pc {
            0x8269E878 => {
    //   block [0x8269E878..0x8269E8EC)
	// 8269E878: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8269E87C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8269E880: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8269E884: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 8269E888: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 8269E88C: 390B73F8  addi r8, r11, 0x73f8
	ctx.r[8].s64 = ctx.r[11].s64 + 29688;
	// 8269E890: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8269E894: 392AA2B4  addi r9, r10, -0x5d4c
	ctx.r[9].s64 = ctx.r[10].s64 + -23884;
	// 8269E898: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269E89C: 38E00005  li r7, 5
	ctx.r[7].s64 = 5;
	// 8269E8A0: 38AA51E0  addi r5, r10, 0x51e0
	ctx.r[5].s64 = ctx.r[10].s64 + 20960;
	// 8269E8A4: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8269E8A8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8269E8AC: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8269E8B0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8269E8B4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8269E8B8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8269E8BC: 3D608297  lis r11, -0x7d69
	ctx.r[11].s64 = -2104033280;
	// 8269E8C0: 388AC394  addi r4, r10, -0x3c6c
	ctx.r[4].s64 = ctx.r[10].s64 + -15468;
	// 8269E8C4: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8269E8C8: 386B5210  addi r3, r11, 0x5210
	ctx.r[3].s64 = ctx.r[11].s64 + 21008;
	// 8269E8CC: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8269E8D0: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8269E8D4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8269E8D8: 4BDC8549  bl 0x82466e20
	ctx.lr = 0x8269E8DC;
	sub_82466E20(ctx, base);
	// 8269E8DC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8269E8E0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8269E8E4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8269E8E8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8269E8F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8269E8F0 size=112
    let mut pc: u32 = 0x8269E8F0;
    'dispatch: loop {
        match pc {
            0x8269E8F0 => {
    //   block [0x8269E8F0..0x8269E960)
	// 8269E8F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8269E8F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8269E8F8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8269E8FC: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269E900: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 8269E904: 38AA51E0  addi r5, r10, 0x51e0
	ctx.r[5].s64 = ctx.r[10].s64 + 20960;
	// 8269E908: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8269E90C: 390B2350  addi r8, r11, 0x2350
	ctx.r[8].s64 = ctx.r[11].s64 + 9040;
	// 8269E910: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 8269E914: 388AC3B0  addi r4, r10, -0x3c50
	ctx.r[4].s64 = ctx.r[10].s64 + -15440;
	// 8269E918: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8269E91C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269E920: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8269E924: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8269E928: 386A5240  addi r3, r10, 0x5240
	ctx.r[3].s64 = ctx.r[10].s64 + 21056;
	// 8269E92C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8269E930: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8269E934: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8269E938: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8269E93C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8269E940: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8269E944: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8269E948: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8269E94C: 4BDC84D5  bl 0x82466e20
	ctx.lr = 0x8269E950;
	sub_82466E20(ctx, base);
	// 8269E950: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8269E954: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8269E958: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8269E95C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8269E960(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8269E960 size=112
    let mut pc: u32 = 0x8269E960;
    'dispatch: loop {
        match pc {
            0x8269E960 => {
    //   block [0x8269E960..0x8269E9D0)
	// 8269E960: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8269E964: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8269E968: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8269E96C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269E970: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 8269E974: 38AA51E0  addi r5, r10, 0x51e0
	ctx.r[5].s64 = ctx.r[10].s64 + 20960;
	// 8269E978: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8269E97C: 390B23B0  addi r8, r11, 0x23b0
	ctx.r[8].s64 = ctx.r[11].s64 + 9136;
	// 8269E980: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 8269E984: 388AC3CC  addi r4, r10, -0x3c34
	ctx.r[4].s64 = ctx.r[10].s64 + -15412;
	// 8269E988: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8269E98C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269E990: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8269E994: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8269E998: 386A5270  addi r3, r10, 0x5270
	ctx.r[3].s64 = ctx.r[10].s64 + 21104;
	// 8269E99C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8269E9A0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8269E9A4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8269E9A8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8269E9AC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8269E9B0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8269E9B4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8269E9B8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8269E9BC: 4BDC8465  bl 0x82466e20
	ctx.lr = 0x8269E9C0;
	sub_82466E20(ctx, base);
	// 8269E9C0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8269E9C4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8269E9C8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8269E9CC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8269E9D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8269E9D0 size=112
    let mut pc: u32 = 0x8269E9D0;
    'dispatch: loop {
        match pc {
            0x8269E9D0 => {
    //   block [0x8269E9D0..0x8269EA40)
	// 8269E9D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8269E9D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8269E9D8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8269E9DC: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269E9E0: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 8269E9E4: 38AA51E0  addi r5, r10, 0x51e0
	ctx.r[5].s64 = ctx.r[10].s64 + 20960;
	// 8269E9E8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8269E9EC: 390B23E0  addi r8, r11, 0x23e0
	ctx.r[8].s64 = ctx.r[11].s64 + 9184;
	// 8269E9F0: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 8269E9F4: 388AC3EC  addi r4, r10, -0x3c14
	ctx.r[4].s64 = ctx.r[10].s64 + -15380;
	// 8269E9F8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8269E9FC: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269EA00: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8269EA04: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8269EA08: 386A52A0  addi r3, r10, 0x52a0
	ctx.r[3].s64 = ctx.r[10].s64 + 21152;
	// 8269EA0C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8269EA10: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8269EA14: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8269EA18: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8269EA1C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8269EA20: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8269EA24: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8269EA28: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8269EA2C: 4BDC83F5  bl 0x82466e20
	ctx.lr = 0x8269EA30;
	sub_82466E20(ctx, base);
	// 8269EA30: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8269EA34: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8269EA38: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8269EA3C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8269EA40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8269EA40 size=108
    let mut pc: u32 = 0x8269EA40;
    'dispatch: loop {
        match pc {
            0x8269EA40 => {
    //   block [0x8269EA40..0x8269EAAC)
	// 8269EA40: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8269EA44: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8269EA48: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8269EA4C: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 8269EA50: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8269EA54: 38EB2428  addi r7, r11, 0x2428
	ctx.r[7].s64 = ctx.r[11].s64 + 9256;
	// 8269EA58: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 8269EA5C: 388AC408  addi r4, r10, -0x3bf8
	ctx.r[4].s64 = ctx.r[10].s64 + -15352;
	// 8269EA60: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8269EA64: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269EA68: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8269EA6C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8269EA70: 386A52D0  addi r3, r10, 0x52d0
	ctx.r[3].s64 = ctx.r[10].s64 + 21200;
	// 8269EA74: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8269EA78: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8269EA7C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8269EA80: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8269EA84: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8269EA88: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8269EA8C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8269EA90: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8269EA94: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8269EA98: 4BDC8389  bl 0x82466e20
	ctx.lr = 0x8269EA9C;
	sub_82466E20(ctx, base);
	// 8269EA9C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8269EAA0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8269EAA4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8269EAA8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8269EAB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8269EAB0 size=112
    let mut pc: u32 = 0x8269EAB0;
    'dispatch: loop {
        match pc {
            0x8269EAB0 => {
    //   block [0x8269EAB0..0x8269EB20)
	// 8269EAB0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8269EAB4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8269EAB8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8269EABC: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269EAC0: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 8269EAC4: 38AA4A00  addi r5, r10, 0x4a00
	ctx.r[5].s64 = ctx.r[10].s64 + 18944;
	// 8269EAC8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8269EACC: 390B2458  addi r8, r11, 0x2458
	ctx.r[8].s64 = ctx.r[11].s64 + 9304;
	// 8269EAD0: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8269EAD4: 388AC428  addi r4, r10, -0x3bd8
	ctx.r[4].s64 = ctx.r[10].s64 + -15320;
	// 8269EAD8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8269EADC: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269EAE0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8269EAE4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8269EAE8: 386A5300  addi r3, r10, 0x5300
	ctx.r[3].s64 = ctx.r[10].s64 + 21248;
	// 8269EAEC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8269EAF0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8269EAF4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8269EAF8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8269EAFC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8269EB00: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8269EB04: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8269EB08: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8269EB0C: 4BDC8315  bl 0x82466e20
	ctx.lr = 0x8269EB10;
	sub_82466E20(ctx, base);
	// 8269EB10: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8269EB14: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8269EB18: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8269EB1C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8269EB20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8269EB20 size=108
    let mut pc: u32 = 0x8269EB20;
    'dispatch: loop {
        match pc {
            0x8269EB20 => {
    //   block [0x8269EB20..0x8269EB8C)
	// 8269EB20: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8269EB24: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8269EB28: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8269EB2C: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 8269EB30: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8269EB34: 38EB2470  addi r7, r11, 0x2470
	ctx.r[7].s64 = ctx.r[11].s64 + 9328;
	// 8269EB38: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 8269EB3C: 388AC440  addi r4, r10, -0x3bc0
	ctx.r[4].s64 = ctx.r[10].s64 + -15296;
	// 8269EB40: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8269EB44: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269EB48: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8269EB4C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8269EB50: 386A5330  addi r3, r10, 0x5330
	ctx.r[3].s64 = ctx.r[10].s64 + 21296;
	// 8269EB54: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8269EB58: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8269EB5C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8269EB60: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8269EB64: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8269EB68: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8269EB6C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8269EB70: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8269EB74: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8269EB78: 4BDC82A9  bl 0x82466e20
	ctx.lr = 0x8269EB7C;
	sub_82466E20(ctx, base);
	// 8269EB7C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8269EB80: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8269EB84: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8269EB88: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8269EB90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8269EB90 size=108
    let mut pc: u32 = 0x8269EB90;
    'dispatch: loop {
        match pc {
            0x8269EB90 => {
    //   block [0x8269EB90..0x8269EBFC)
	// 8269EB90: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8269EB94: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8269EB98: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8269EB9C: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 8269EBA0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8269EBA4: 38EB24B8  addi r7, r11, 0x24b8
	ctx.r[7].s64 = ctx.r[11].s64 + 9400;
	// 8269EBA8: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 8269EBAC: 388AC46C  addi r4, r10, -0x3b94
	ctx.r[4].s64 = ctx.r[10].s64 + -15252;
	// 8269EBB0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8269EBB4: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269EBB8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8269EBBC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8269EBC0: 386A5360  addi r3, r10, 0x5360
	ctx.r[3].s64 = ctx.r[10].s64 + 21344;
	// 8269EBC4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8269EBC8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8269EBCC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8269EBD0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8269EBD4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8269EBD8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8269EBDC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8269EBE0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8269EBE4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8269EBE8: 4BDC8239  bl 0x82466e20
	ctx.lr = 0x8269EBEC;
	sub_82466E20(ctx, base);
	// 8269EBEC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8269EBF0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8269EBF4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8269EBF8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8269EC00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8269EC00 size=108
    let mut pc: u32 = 0x8269EC00;
    'dispatch: loop {
        match pc {
            0x8269EC00 => {
    //   block [0x8269EC00..0x8269EC6C)
	// 8269EC00: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8269EC04: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8269EC08: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8269EC0C: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 8269EC10: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8269EC14: 38EB2518  addi r7, r11, 0x2518
	ctx.r[7].s64 = ctx.r[11].s64 + 9496;
	// 8269EC18: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 8269EC1C: 388AC48C  addi r4, r10, -0x3b74
	ctx.r[4].s64 = ctx.r[10].s64 + -15220;
	// 8269EC20: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8269EC24: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269EC28: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8269EC2C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8269EC30: 386A5390  addi r3, r10, 0x5390
	ctx.r[3].s64 = ctx.r[10].s64 + 21392;
	// 8269EC34: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8269EC38: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8269EC3C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8269EC40: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8269EC44: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8269EC48: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8269EC4C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8269EC50: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8269EC54: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8269EC58: 4BDC81C9  bl 0x82466e20
	ctx.lr = 0x8269EC5C;
	sub_82466E20(ctx, base);
	// 8269EC5C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8269EC60: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8269EC64: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8269EC68: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8269EC70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8269EC70 size=116
    let mut pc: u32 = 0x8269EC70;
    'dispatch: loop {
        match pc {
            0x8269EC70 => {
    //   block [0x8269EC70..0x8269ECE4)
	// 8269EC70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8269EC74: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8269EC78: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8269EC7C: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 8269EC80: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269EC84: 392BA2F0  addi r9, r11, -0x5d10
	ctx.r[9].s64 = ctx.r[11].s64 + -23824;
	// 8269EC88: 38AA58A0  addi r5, r10, 0x58a0
	ctx.r[5].s64 = ctx.r[10].s64 + 22688;
	// 8269EC8C: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8269EC90: 38E90018  addi r7, r9, 0x18
	ctx.r[7].s64 = ctx.r[9].s64 + 24;
	// 8269EC94: 38C00011  li r6, 0x11
	ctx.r[6].s64 = 17;
	// 8269EC98: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 8269EC9C: 388AC4A8  addi r4, r10, -0x3b58
	ctx.r[4].s64 = ctx.r[10].s64 + -15192;
	// 8269ECA0: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8269ECA4: 396B2548  addi r11, r11, 0x2548
	ctx.r[11].s64 = ctx.r[11].s64 + 9544;
	// 8269ECA8: 90E10064  stw r7, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[7].u32 ) };
	// 8269ECAC: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269ECB0: 90C1005C  stw r6, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[6].u32 ) };
	// 8269ECB4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8269ECB8: 386A53C0  addi r3, r10, 0x53c0
	ctx.r[3].s64 = ctx.r[10].s64 + 21440;
	// 8269ECBC: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8269ECC0: 91010074  stw r8, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[8].u32 ) };
	// 8269ECC4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8269ECC8: 9101006C  stw r8, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[8].u32 ) };
	// 8269ECCC: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8269ECD0: 4BDC8151  bl 0x82466e20
	ctx.lr = 0x8269ECD4;
	sub_82466E20(ctx, base);
	// 8269ECD4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8269ECD8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8269ECDC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8269ECE0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8269ECE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8269ECE8 size=100
    let mut pc: u32 = 0x8269ECE8;
    'dispatch: loop {
        match pc {
            0x8269ECE8 => {
    //   block [0x8269ECE8..0x8269ED4C)
	// 8269ECE8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8269ECEC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8269ECF0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8269ECF4: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269ECF8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8269ECFC: 38AA2B10  addi r5, r10, 0x2b10
	ctx.r[5].s64 = ctx.r[10].s64 + 11024;
	// 8269ED00: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8269ED04: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8269ED08: 388AC4B4  addi r4, r10, -0x3b4c
	ctx.r[4].s64 = ctx.r[10].s64 + -15180;
	// 8269ED0C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269ED10: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8269ED14: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8269ED18: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8269ED1C: 386A53F0  addi r3, r10, 0x53f0
	ctx.r[3].s64 = ctx.r[10].s64 + 21488;
	// 8269ED20: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8269ED24: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8269ED28: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8269ED2C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8269ED30: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8269ED34: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8269ED38: 4BDC80E9  bl 0x82466e20
	ctx.lr = 0x8269ED3C;
	sub_82466E20(ctx, base);
	// 8269ED3C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8269ED40: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8269ED44: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8269ED48: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8269ED50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8269ED50 size=100
    let mut pc: u32 = 0x8269ED50;
    'dispatch: loop {
        match pc {
            0x8269ED50 => {
    //   block [0x8269ED50..0x8269EDB4)
	// 8269ED50: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8269ED54: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8269ED58: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8269ED5C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269ED60: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8269ED64: 38AA5480  addi r5, r10, 0x5480
	ctx.r[5].s64 = ctx.r[10].s64 + 21632;
	// 8269ED68: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8269ED6C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8269ED70: 388AC4CC  addi r4, r10, -0x3b34
	ctx.r[4].s64 = ctx.r[10].s64 + -15156;
	// 8269ED74: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269ED78: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8269ED7C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8269ED80: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8269ED84: 386A5420  addi r3, r10, 0x5420
	ctx.r[3].s64 = ctx.r[10].s64 + 21536;
	// 8269ED88: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8269ED8C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8269ED90: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8269ED94: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8269ED98: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8269ED9C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8269EDA0: 4BDC8081  bl 0x82466e20
	ctx.lr = 0x8269EDA4;
	sub_82466E20(ctx, base);
	// 8269EDA4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8269EDA8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8269EDAC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8269EDB0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8269EDB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8269EDB8 size=100
    let mut pc: u32 = 0x8269EDB8;
    'dispatch: loop {
        match pc {
            0x8269EDB8 => {
    //   block [0x8269EDB8..0x8269EE1C)
	// 8269EDB8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8269EDBC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8269EDC0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8269EDC4: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269EDC8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8269EDCC: 38AA53C0  addi r5, r10, 0x53c0
	ctx.r[5].s64 = ctx.r[10].s64 + 21440;
	// 8269EDD0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8269EDD4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8269EDD8: 388AC4E8  addi r4, r10, -0x3b18
	ctx.r[4].s64 = ctx.r[10].s64 + -15128;
	// 8269EDDC: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269EDE0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8269EDE4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8269EDE8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8269EDEC: 386A5450  addi r3, r10, 0x5450
	ctx.r[3].s64 = ctx.r[10].s64 + 21584;
	// 8269EDF0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8269EDF4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8269EDF8: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8269EDFC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8269EE00: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8269EE04: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8269EE08: 4BDC8019  bl 0x82466e20
	ctx.lr = 0x8269EE0C;
	sub_82466E20(ctx, base);
	// 8269EE0C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8269EE10: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8269EE14: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8269EE18: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8269EE20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8269EE20 size=104
    let mut pc: u32 = 0x8269EE20;
    'dispatch: loop {
        match pc {
            0x8269EE20 => {
    //   block [0x8269EE20..0x8269EE88)
	// 8269EE20: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8269EE24: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8269EE28: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8269EE2C: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 8269EE30: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8269EE34: 392AA370  addi r9, r10, -0x5c90
	ctx.r[9].s64 = ctx.r[10].s64 + -23696;
	// 8269EE38: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269EE3C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8269EE40: 38AA53F0  addi r5, r10, 0x53f0
	ctx.r[5].s64 = ctx.r[10].s64 + 21488;
	// 8269EE44: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8269EE48: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8269EE4C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8269EE50: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8269EE54: 388AC4F8  addi r4, r10, -0x3b08
	ctx.r[4].s64 = ctx.r[10].s64 + -15112;
	// 8269EE58: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8269EE5C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269EE60: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8269EE64: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8269EE68: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8269EE6C: 386A5480  addi r3, r10, 0x5480
	ctx.r[3].s64 = ctx.r[10].s64 + 21632;
	// 8269EE70: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8269EE74: 4BDC7FAD  bl 0x82466e20
	ctx.lr = 0x8269EE78;
	sub_82466E20(ctx, base);
	// 8269EE78: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8269EE7C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8269EE80: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8269EE84: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8269EE88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8269EE88 size=108
    let mut pc: u32 = 0x8269EE88;
    'dispatch: loop {
        match pc {
            0x8269EE88 => {
    //   block [0x8269EE88..0x8269EEF4)
	// 8269EE88: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8269EE8C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8269EE90: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8269EE94: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 8269EE98: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8269EE9C: 38EB26E4  addi r7, r11, 0x26e4
	ctx.r[7].s64 = ctx.r[11].s64 + 9956;
	// 8269EEA0: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 8269EEA4: 388AC510  addi r4, r10, -0x3af0
	ctx.r[4].s64 = ctx.r[10].s64 + -15088;
	// 8269EEA8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8269EEAC: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269EEB0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8269EEB4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8269EEB8: 386A54B0  addi r3, r10, 0x54b0
	ctx.r[3].s64 = ctx.r[10].s64 + 21680;
	// 8269EEBC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8269EEC0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8269EEC4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8269EEC8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8269EECC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8269EED0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8269EED4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8269EED8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8269EEDC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8269EEE0: 4BDC7F41  bl 0x82466e20
	ctx.lr = 0x8269EEE4;
	sub_82466E20(ctx, base);
	// 8269EEE4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8269EEE8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8269EEEC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8269EEF0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8269EEF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8269EEF8 size=112
    let mut pc: u32 = 0x8269EEF8;
    'dispatch: loop {
        match pc {
            0x8269EEF8 => {
    //   block [0x8269EEF8..0x8269EF68)
	// 8269EEF8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8269EEFC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8269EF00: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8269EF04: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269EF08: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 8269EF0C: 38AA5480  addi r5, r10, 0x5480
	ctx.r[5].s64 = ctx.r[10].s64 + 21632;
	// 8269EF10: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8269EF14: 390B2718  addi r8, r11, 0x2718
	ctx.r[8].s64 = ctx.r[11].s64 + 10008;
	// 8269EF18: 39200007  li r9, 7
	ctx.r[9].s64 = 7;
	// 8269EF1C: 388AC538  addi r4, r10, -0x3ac8
	ctx.r[4].s64 = ctx.r[10].s64 + -15048;
	// 8269EF20: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8269EF24: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269EF28: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8269EF2C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8269EF30: 386A54E0  addi r3, r10, 0x54e0
	ctx.r[3].s64 = ctx.r[10].s64 + 21728;
	// 8269EF34: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8269EF38: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8269EF3C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8269EF40: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8269EF44: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8269EF48: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8269EF4C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8269EF50: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8269EF54: 4BDC7ECD  bl 0x82466e20
	ctx.lr = 0x8269EF58;
	sub_82466E20(ctx, base);
	// 8269EF58: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8269EF5C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8269EF60: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8269EF64: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8269EF68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8269EF68 size=24
    let mut pc: u32 = 0x8269EF68;
    'dispatch: loop {
        match pc {
            0x8269EF68 => {
    //   block [0x8269EF68..0x8269EF80)
	// 8269EF68: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 8269EF6C: 3D40827C  lis r10, -0x7d84
	ctx.r[10].s64 = -2105802752;
	// 8269EF70: 394A7470  addi r10, r10, 0x7470
	ctx.r[10].s64 = ctx.r[10].s64 + 29808;
	// 8269EF74: 816B2714  lwz r11, 0x2714(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(10004 as u32) ) } as u64;
	// 8269EF78: 916A0008  stw r11, 8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 8269EF7C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8269EF80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8269EF80 size=116
    let mut pc: u32 = 0x8269EF80;
    'dispatch: loop {
        match pc {
            0x8269EF80 => {
    //   block [0x8269EF80..0x8269EFF4)
	// 8269EF80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8269EF84: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8269EF88: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8269EF8C: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 8269EF90: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 8269EF94: 390B7470  addi r8, r11, 0x7470
	ctx.r[8].s64 = ctx.r[11].s64 + 29808;
	// 8269EF98: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8269EF9C: 392AA3E0  addi r9, r10, -0x5c20
	ctx.r[9].s64 = ctx.r[10].s64 + -23584;
	// 8269EFA0: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269EFA4: 38E0000B  li r7, 0xb
	ctx.r[7].s64 = 11;
	// 8269EFA8: 38AA2B10  addi r5, r10, 0x2b10
	ctx.r[5].s64 = ctx.r[10].s64 + 11024;
	// 8269EFAC: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8269EFB0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8269EFB4: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8269EFB8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8269EFBC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8269EFC0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8269EFC4: 3D608297  lis r11, -0x7d69
	ctx.r[11].s64 = -2104033280;
	// 8269EFC8: 388AC558  addi r4, r10, -0x3aa8
	ctx.r[4].s64 = ctx.r[10].s64 + -15016;
	// 8269EFCC: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8269EFD0: 386B5510  addi r3, r11, 0x5510
	ctx.r[3].s64 = ctx.r[11].s64 + 21776;
	// 8269EFD4: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8269EFD8: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8269EFDC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8269EFE0: 4BDC7E41  bl 0x82466e20
	ctx.lr = 0x8269EFE4;
	sub_82466E20(ctx, base);
	// 8269EFE4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8269EFE8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8269EFEC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8269EFF0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8269EFF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8269EFF8 size=100
    let mut pc: u32 = 0x8269EFF8;
    'dispatch: loop {
        match pc {
            0x8269EFF8 => {
    //   block [0x8269EFF8..0x8269F05C)
	// 8269EFF8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8269EFFC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8269F000: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8269F004: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269F008: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8269F00C: 38AA5510  addi r5, r10, 0x5510
	ctx.r[5].s64 = ctx.r[10].s64 + 21776;
	// 8269F010: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8269F014: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8269F018: 388AC564  addi r4, r10, -0x3a9c
	ctx.r[4].s64 = ctx.r[10].s64 + -15004;
	// 8269F01C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269F020: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8269F024: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8269F028: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8269F02C: 386A5540  addi r3, r10, 0x5540
	ctx.r[3].s64 = ctx.r[10].s64 + 21824;
	// 8269F030: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8269F034: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8269F038: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8269F03C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8269F040: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8269F044: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8269F048: 4BDC7DD9  bl 0x82466e20
	ctx.lr = 0x8269F04C;
	sub_82466E20(ctx, base);
	// 8269F04C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8269F050: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8269F054: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8269F058: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8269F060(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8269F060 size=100
    let mut pc: u32 = 0x8269F060;
    'dispatch: loop {
        match pc {
            0x8269F060 => {
    //   block [0x8269F060..0x8269F0C4)
	// 8269F060: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8269F064: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8269F068: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8269F06C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269F070: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8269F074: 38AA5510  addi r5, r10, 0x5510
	ctx.r[5].s64 = ctx.r[10].s64 + 21776;
	// 8269F078: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8269F07C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8269F080: 388AC574  addi r4, r10, -0x3a8c
	ctx.r[4].s64 = ctx.r[10].s64 + -14988;
	// 8269F084: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269F088: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8269F08C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8269F090: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8269F094: 386A5570  addi r3, r10, 0x5570
	ctx.r[3].s64 = ctx.r[10].s64 + 21872;
	// 8269F098: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8269F09C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8269F0A0: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8269F0A4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8269F0A8: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8269F0AC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8269F0B0: 4BDC7D71  bl 0x82466e20
	ctx.lr = 0x8269F0B4;
	sub_82466E20(ctx, base);
	// 8269F0B4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8269F0B8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8269F0BC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8269F0C0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8269F0C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8269F0C8 size=100
    let mut pc: u32 = 0x8269F0C8;
    'dispatch: loop {
        match pc {
            0x8269F0C8 => {
    //   block [0x8269F0C8..0x8269F12C)
	// 8269F0C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8269F0CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8269F0D0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8269F0D4: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269F0D8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8269F0DC: 38AA55D0  addi r5, r10, 0x55d0
	ctx.r[5].s64 = ctx.r[10].s64 + 21968;
	// 8269F0E0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8269F0E4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8269F0E8: 388AC588  addi r4, r10, -0x3a78
	ctx.r[4].s64 = ctx.r[10].s64 + -14968;
	// 8269F0EC: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269F0F0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8269F0F4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8269F0F8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8269F0FC: 386A55A0  addi r3, r10, 0x55a0
	ctx.r[3].s64 = ctx.r[10].s64 + 21920;
	// 8269F100: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8269F104: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8269F108: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8269F10C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8269F110: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8269F114: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8269F118: 4BDC7D09  bl 0x82466e20
	ctx.lr = 0x8269F11C;
	sub_82466E20(ctx, base);
	// 8269F11C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8269F120: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8269F124: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8269F128: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8269F130(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8269F130 size=100
    let mut pc: u32 = 0x8269F130;
    'dispatch: loop {
        match pc {
            0x8269F130 => {
    //   block [0x8269F130..0x8269F194)
	// 8269F130: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8269F134: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8269F138: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8269F13C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269F140: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8269F144: 38AA5510  addi r5, r10, 0x5510
	ctx.r[5].s64 = ctx.r[10].s64 + 21776;
	// 8269F148: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8269F14C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8269F150: 388AC59C  addi r4, r10, -0x3a64
	ctx.r[4].s64 = ctx.r[10].s64 + -14948;
	// 8269F154: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269F158: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8269F15C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8269F160: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8269F164: 386A55D0  addi r3, r10, 0x55d0
	ctx.r[3].s64 = ctx.r[10].s64 + 21968;
	// 8269F168: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8269F16C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8269F170: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8269F174: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8269F178: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8269F17C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8269F180: 4BDC7CA1  bl 0x82466e20
	ctx.lr = 0x8269F184;
	sub_82466E20(ctx, base);
	// 8269F184: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8269F188: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8269F18C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8269F190: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8269F198(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8269F198 size=100
    let mut pc: u32 = 0x8269F198;
    'dispatch: loop {
        match pc {
            0x8269F198 => {
    //   block [0x8269F198..0x8269F1FC)
	// 8269F198: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8269F19C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8269F1A0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8269F1A4: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269F1A8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8269F1AC: 38AA55D0  addi r5, r10, 0x55d0
	ctx.r[5].s64 = ctx.r[10].s64 + 21968;
	// 8269F1B0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8269F1B4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8269F1B8: 388AC5B4  addi r4, r10, -0x3a4c
	ctx.r[4].s64 = ctx.r[10].s64 + -14924;
	// 8269F1BC: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269F1C0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8269F1C4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8269F1C8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8269F1CC: 386A5600  addi r3, r10, 0x5600
	ctx.r[3].s64 = ctx.r[10].s64 + 22016;
	// 8269F1D0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8269F1D4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8269F1D8: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8269F1DC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8269F1E0: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8269F1E4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8269F1E8: 4BDC7C39  bl 0x82466e20
	ctx.lr = 0x8269F1EC;
	sub_82466E20(ctx, base);
	// 8269F1EC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8269F1F0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8269F1F4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8269F1F8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8269F200(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8269F200 size=100
    let mut pc: u32 = 0x8269F200;
    'dispatch: loop {
        match pc {
            0x8269F200 => {
    //   block [0x8269F200..0x8269F264)
	// 8269F200: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8269F204: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8269F208: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8269F20C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269F210: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8269F214: 38AA5510  addi r5, r10, 0x5510
	ctx.r[5].s64 = ctx.r[10].s64 + 21776;
	// 8269F218: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8269F21C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8269F220: 388AC5C8  addi r4, r10, -0x3a38
	ctx.r[4].s64 = ctx.r[10].s64 + -14904;
	// 8269F224: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269F228: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8269F22C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8269F230: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8269F234: 386A5630  addi r3, r10, 0x5630
	ctx.r[3].s64 = ctx.r[10].s64 + 22064;
	// 8269F238: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8269F23C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8269F240: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8269F244: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8269F248: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8269F24C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8269F250: 4BDC7BD1  bl 0x82466e20
	ctx.lr = 0x8269F254;
	sub_82466E20(ctx, base);
	// 8269F254: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8269F258: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8269F25C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8269F260: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8269F268(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8269F268 size=100
    let mut pc: u32 = 0x8269F268;
    'dispatch: loop {
        match pc {
            0x8269F268 => {
    //   block [0x8269F268..0x8269F2CC)
	// 8269F268: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8269F26C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8269F270: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8269F274: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269F278: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8269F27C: 38AA5540  addi r5, r10, 0x5540
	ctx.r[5].s64 = ctx.r[10].s64 + 21824;
	// 8269F280: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8269F284: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8269F288: 388AC5D8  addi r4, r10, -0x3a28
	ctx.r[4].s64 = ctx.r[10].s64 + -14888;
	// 8269F28C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269F290: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8269F294: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8269F298: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8269F29C: 386A5660  addi r3, r10, 0x5660
	ctx.r[3].s64 = ctx.r[10].s64 + 22112;
	// 8269F2A0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8269F2A4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8269F2A8: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8269F2AC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8269F2B0: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8269F2B4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8269F2B8: 4BDC7B69  bl 0x82466e20
	ctx.lr = 0x8269F2BC;
	sub_82466E20(ctx, base);
	// 8269F2BC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8269F2C0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8269F2C4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8269F2C8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8269F2D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8269F2D0 size=100
    let mut pc: u32 = 0x8269F2D0;
    'dispatch: loop {
        match pc {
            0x8269F2D0 => {
    //   block [0x8269F2D0..0x8269F334)
	// 8269F2D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8269F2D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8269F2D8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8269F2DC: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269F2E0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8269F2E4: 38AA5630  addi r5, r10, 0x5630
	ctx.r[5].s64 = ctx.r[10].s64 + 22064;
	// 8269F2E8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8269F2EC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8269F2F0: 388AC5F0  addi r4, r10, -0x3a10
	ctx.r[4].s64 = ctx.r[10].s64 + -14864;
	// 8269F2F4: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269F2F8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8269F2FC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8269F300: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8269F304: 386A5690  addi r3, r10, 0x5690
	ctx.r[3].s64 = ctx.r[10].s64 + 22160;
	// 8269F308: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8269F30C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8269F310: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8269F314: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8269F318: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8269F31C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8269F320: 4BDC7B01  bl 0x82466e20
	ctx.lr = 0x8269F324;
	sub_82466E20(ctx, base);
	// 8269F324: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8269F328: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8269F32C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8269F330: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8269F338(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8269F338 size=100
    let mut pc: u32 = 0x8269F338;
    'dispatch: loop {
        match pc {
            0x8269F338 => {
    //   block [0x8269F338..0x8269F39C)
	// 8269F338: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8269F33C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8269F340: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8269F344: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269F348: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8269F34C: 38AA5540  addi r5, r10, 0x5540
	ctx.r[5].s64 = ctx.r[10].s64 + 21824;
	// 8269F350: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8269F354: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8269F358: 388AC60C  addi r4, r10, -0x39f4
	ctx.r[4].s64 = ctx.r[10].s64 + -14836;
	// 8269F35C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269F360: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8269F364: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8269F368: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8269F36C: 386A56C0  addi r3, r10, 0x56c0
	ctx.r[3].s64 = ctx.r[10].s64 + 22208;
	// 8269F370: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8269F374: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8269F378: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8269F37C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8269F380: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8269F384: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8269F388: 4BDC7A99  bl 0x82466e20
	ctx.lr = 0x8269F38C;
	sub_82466E20(ctx, base);
	// 8269F38C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8269F390: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8269F394: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8269F398: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8269F3A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8269F3A0 size=112
    let mut pc: u32 = 0x8269F3A0;
    'dispatch: loop {
        match pc {
            0x8269F3A0 => {
    //   block [0x8269F3A0..0x8269F410)
	// 8269F3A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8269F3A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8269F3A8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8269F3AC: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269F3B0: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 8269F3B4: 38AA5750  addi r5, r10, 0x5750
	ctx.r[5].s64 = ctx.r[10].s64 + 22352;
	// 8269F3B8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8269F3BC: 390B27C0  addi r8, r11, 0x27c0
	ctx.r[8].s64 = ctx.r[11].s64 + 10176;
	// 8269F3C0: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 8269F3C4: 388AC620  addi r4, r10, -0x39e0
	ctx.r[4].s64 = ctx.r[10].s64 + -14816;
	// 8269F3C8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8269F3CC: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269F3D0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8269F3D4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8269F3D8: 386A56F0  addi r3, r10, 0x56f0
	ctx.r[3].s64 = ctx.r[10].s64 + 22256;
	// 8269F3DC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8269F3E0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8269F3E4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8269F3E8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8269F3EC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8269F3F0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8269F3F4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8269F3F8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8269F3FC: 4BDC7A25  bl 0x82466e20
	ctx.lr = 0x8269F400;
	sub_82466E20(ctx, base);
	// 8269F400: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8269F404: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8269F408: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8269F40C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8269F410(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8269F410 size=112
    let mut pc: u32 = 0x8269F410;
    'dispatch: loop {
        match pc {
            0x8269F410 => {
    //   block [0x8269F410..0x8269F480)
	// 8269F410: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8269F414: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8269F418: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8269F41C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269F420: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 8269F424: 38AA5780  addi r5, r10, 0x5780
	ctx.r[5].s64 = ctx.r[10].s64 + 22400;
	// 8269F428: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8269F42C: 390B27F0  addi r8, r11, 0x27f0
	ctx.r[8].s64 = ctx.r[11].s64 + 10224;
	// 8269F430: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8269F434: 388AC630  addi r4, r10, -0x39d0
	ctx.r[4].s64 = ctx.r[10].s64 + -14800;
	// 8269F438: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8269F43C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269F440: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8269F444: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8269F448: 386A5720  addi r3, r10, 0x5720
	ctx.r[3].s64 = ctx.r[10].s64 + 22304;
	// 8269F44C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8269F450: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8269F454: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8269F458: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8269F45C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8269F460: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8269F464: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8269F468: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8269F46C: 4BDC79B5  bl 0x82466e20
	ctx.lr = 0x8269F470;
	sub_82466E20(ctx, base);
	// 8269F470: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8269F474: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8269F478: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8269F47C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8269F480(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8269F480 size=112
    let mut pc: u32 = 0x8269F480;
    'dispatch: loop {
        match pc {
            0x8269F480 => {
    //   block [0x8269F480..0x8269F4F0)
	// 8269F480: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8269F484: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8269F488: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8269F48C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269F490: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 8269F494: 38AA58A0  addi r5, r10, 0x58a0
	ctx.r[5].s64 = ctx.r[10].s64 + 22688;
	// 8269F498: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8269F49C: 390B2808  addi r8, r11, 0x2808
	ctx.r[8].s64 = ctx.r[11].s64 + 10248;
	// 8269F4A0: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 8269F4A4: 388AC648  addi r4, r10, -0x39b8
	ctx.r[4].s64 = ctx.r[10].s64 + -14776;
	// 8269F4A8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8269F4AC: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269F4B0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8269F4B4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8269F4B8: 386A5750  addi r3, r10, 0x5750
	ctx.r[3].s64 = ctx.r[10].s64 + 22352;
	// 8269F4BC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8269F4C0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8269F4C4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8269F4C8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8269F4CC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8269F4D0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8269F4D4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8269F4D8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8269F4DC: 4BDC7945  bl 0x82466e20
	ctx.lr = 0x8269F4E0;
	sub_82466E20(ctx, base);
	// 8269F4E0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8269F4E4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8269F4E8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8269F4EC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8269F4F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8269F4F0 size=112
    let mut pc: u32 = 0x8269F4F0;
    'dispatch: loop {
        match pc {
            0x8269F4F0 => {
    //   block [0x8269F4F0..0x8269F560)
	// 8269F4F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8269F4F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8269F4F8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8269F4FC: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269F500: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 8269F504: 38AA5750  addi r5, r10, 0x5750
	ctx.r[5].s64 = ctx.r[10].s64 + 22352;
	// 8269F508: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8269F50C: 390B2838  addi r8, r11, 0x2838
	ctx.r[8].s64 = ctx.r[11].s64 + 10296;
	// 8269F510: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8269F514: 388AC654  addi r4, r10, -0x39ac
	ctx.r[4].s64 = ctx.r[10].s64 + -14764;
	// 8269F518: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8269F51C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269F520: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8269F524: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8269F528: 386A5780  addi r3, r10, 0x5780
	ctx.r[3].s64 = ctx.r[10].s64 + 22400;
	// 8269F52C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8269F530: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8269F534: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8269F538: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8269F53C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8269F540: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8269F544: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8269F548: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8269F54C: 4BDC78D5  bl 0x82466e20
	ctx.lr = 0x8269F550;
	sub_82466E20(ctx, base);
	// 8269F550: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8269F554: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8269F558: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8269F55C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8269F560(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8269F560 size=108
    let mut pc: u32 = 0x8269F560;
    'dispatch: loop {
        match pc {
            0x8269F560 => {
    //   block [0x8269F560..0x8269F5CC)
	// 8269F560: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8269F564: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8269F568: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8269F56C: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 8269F570: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8269F574: 38EB2850  addi r7, r11, 0x2850
	ctx.r[7].s64 = ctx.r[11].s64 + 10320;
	// 8269F578: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 8269F57C: 388AC664  addi r4, r10, -0x399c
	ctx.r[4].s64 = ctx.r[10].s64 + -14748;
	// 8269F580: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8269F584: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269F588: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8269F58C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8269F590: 386A57B0  addi r3, r10, 0x57b0
	ctx.r[3].s64 = ctx.r[10].s64 + 22448;
	// 8269F594: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8269F598: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8269F59C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8269F5A0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8269F5A4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8269F5A8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8269F5AC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8269F5B0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8269F5B4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8269F5B8: 4BDC7869  bl 0x82466e20
	ctx.lr = 0x8269F5BC;
	sub_82466E20(ctx, base);
	// 8269F5BC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8269F5C0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8269F5C4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8269F5C8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8269F5D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8269F5D0 size=112
    let mut pc: u32 = 0x8269F5D0;
    'dispatch: loop {
        match pc {
            0x8269F5D0 => {
    //   block [0x8269F5D0..0x8269F640)
	// 8269F5D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8269F5D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8269F5D8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8269F5DC: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269F5E0: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 8269F5E4: 38AA5780  addi r5, r10, 0x5780
	ctx.r[5].s64 = ctx.r[10].s64 + 22400;
	// 8269F5E8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8269F5EC: 390B2868  addi r8, r11, 0x2868
	ctx.r[8].s64 = ctx.r[11].s64 + 10344;
	// 8269F5F0: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8269F5F4: 388AC68C  addi r4, r10, -0x3974
	ctx.r[4].s64 = ctx.r[10].s64 + -14708;
	// 8269F5F8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8269F5FC: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269F600: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8269F604: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8269F608: 386A57E0  addi r3, r10, 0x57e0
	ctx.r[3].s64 = ctx.r[10].s64 + 22496;
	// 8269F60C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8269F610: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8269F614: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8269F618: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8269F61C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8269F620: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8269F624: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8269F628: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8269F62C: 4BDC77F5  bl 0x82466e20
	ctx.lr = 0x8269F630;
	sub_82466E20(ctx, base);
	// 8269F630: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8269F634: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8269F638: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8269F63C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8269F640(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8269F640 size=116
    let mut pc: u32 = 0x8269F640;
    'dispatch: loop {
        match pc {
            0x8269F640 => {
    //   block [0x8269F640..0x8269F6B4)
	// 8269F640: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8269F644: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8269F648: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8269F64C: 3D40827C  lis r10, -0x7d84
	ctx.r[10].s64 = -2105802752;
	// 8269F650: 38E00007  li r7, 7
	ctx.r[7].s64 = 7;
	// 8269F654: 390A2880  addi r8, r10, 0x2880
	ctx.r[8].s64 = ctx.r[10].s64 + 10368;
	// 8269F658: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269F65C: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 8269F660: 38AA2B10  addi r5, r10, 0x2b10
	ctx.r[5].s64 = ctx.r[10].s64 + 11024;
	// 8269F664: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8269F668: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8269F66C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8269F670: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8269F674: 388AC6A4  addi r4, r10, -0x395c
	ctx.r[4].s64 = ctx.r[10].s64 + -14684;
	// 8269F678: 396BA3F4  addi r11, r11, -0x5c0c
	ctx.r[11].s64 = ctx.r[11].s64 + -23564;
	// 8269F67C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269F680: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8269F684: 386A5810  addi r3, r10, 0x5810
	ctx.r[3].s64 = ctx.r[10].s64 + 22544;
	// 8269F688: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 8269F68C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8269F690: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 8269F694: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8269F698: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8269F69C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8269F6A0: 4BDC7781  bl 0x82466e20
	ctx.lr = 0x8269F6A4;
	sub_82466E20(ctx, base);
	// 8269F6A4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8269F6A8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8269F6AC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8269F6B0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8269F6B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8269F6B8 size=116
    let mut pc: u32 = 0x8269F6B8;
    'dispatch: loop {
        match pc {
            0x8269F6B8 => {
    //   block [0x8269F6B8..0x8269F72C)
	// 8269F6B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8269F6BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8269F6C0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8269F6C4: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 8269F6C8: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
	// 8269F6CC: 392AA51C  addi r9, r10, -0x5ae4
	ctx.r[9].s64 = ctx.r[10].s64 + -23268;
	// 8269F6D0: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269F6D4: 38C00044  li r6, 0x44
	ctx.r[6].s64 = 68;
	// 8269F6D8: 38AA2B10  addi r5, r10, 0x2b10
	ctx.r[5].s64 = ctx.r[10].s64 + 11024;
	// 8269F6DC: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8269F6E0: 90E10074  stw r7, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[7].u32 ) };
	// 8269F6E4: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 8269F6E8: 388AC6B8  addi r4, r10, -0x3948
	ctx.r[4].s64 = ctx.r[10].s64 + -14664;
	// 8269F6EC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8269F6F0: 90C1005C  stw r6, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[6].u32 ) };
	// 8269F6F4: 396B2948  addi r11, r11, 0x2948
	ctx.r[11].s64 = ctx.r[11].s64 + 10568;
	// 8269F6F8: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269F6FC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8269F700: 386A5840  addi r3, r10, 0x5840
	ctx.r[3].s64 = ctx.r[10].s64 + 22592;
	// 8269F704: 39400002  li r10, 2
	ctx.r[10].s64 = 2;
	// 8269F708: 9101006C  stw r8, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[8].u32 ) };
	// 8269F70C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8269F710: 91010064  stw r8, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[8].u32 ) };
	// 8269F714: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8269F718: 4BDC7709  bl 0x82466e20
	ctx.lr = 0x8269F71C;
	sub_82466E20(ctx, base);
	// 8269F71C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8269F720: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8269F724: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8269F728: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8269F730(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8269F730 size=48
    let mut pc: u32 = 0x8269F730;
    'dispatch: loop {
        match pc {
            0x8269F730 => {
    //   block [0x8269F730..0x8269F760)
	// 8269F730: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 8269F734: 814B2FB0  lwz r10, 0x2fb0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12208 as u32) ) } as u64;
	// 8269F738: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 8269F73C: 396B7578  addi r11, r11, 0x7578
	ctx.r[11].s64 = ctx.r[11].s64 + 30072;
	// 8269F740: 914B0050  stw r10, 0x50(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 8269F744: 3D40827C  lis r10, -0x7d84
	ctx.r[10].s64 = -2105802752;
	// 8269F748: 814A2FAC  lwz r10, 0x2fac(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12204 as u32) ) } as u64;
	// 8269F74C: 914B0140  stw r10, 0x140(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(320 as u32), ctx.r[10].u32 ) };
	// 8269F750: 3D40827C  lis r10, -0x7d84
	ctx.r[10].s64 = -2105802752;
	// 8269F754: 814A2FA8  lwz r10, 0x2fa8(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12200 as u32) ) } as u64;
	// 8269F758: 914B0380  stw r10, 0x380(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(896 as u32), ctx.r[10].u32 ) };
	// 8269F75C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8269F760(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8269F760 size=116
    let mut pc: u32 = 0x8269F760;
    'dispatch: loop {
        match pc {
            0x8269F760 => {
    //   block [0x8269F760..0x8269F7D4)
	// 8269F760: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8269F764: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8269F768: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8269F76C: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 8269F770: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269F774: 392BA5F8  addi r9, r11, -0x5a08
	ctx.r[9].s64 = ctx.r[11].s64 + -23048;
	// 8269F778: 38AA2B10  addi r5, r10, 0x2b10
	ctx.r[5].s64 = ctx.r[10].s64 + 11024;
	// 8269F77C: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8269F780: 38E90050  addi r7, r9, 0x50
	ctx.r[7].s64 = ctx.r[9].s64 + 80;
	// 8269F784: 38C0002A  li r6, 0x2a
	ctx.r[6].s64 = 42;
	// 8269F788: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 8269F78C: 388AC6C4  addi r4, r10, -0x393c
	ctx.r[4].s64 = ctx.r[10].s64 + -14652;
	// 8269F790: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8269F794: 396B7578  addi r11, r11, 0x7578
	ctx.r[11].s64 = ctx.r[11].s64 + 30072;
	// 8269F798: 90E10064  stw r7, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[7].u32 ) };
	// 8269F79C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269F7A0: 90C1005C  stw r6, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[6].u32 ) };
	// 8269F7A4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8269F7A8: 386A5870  addi r3, r10, 0x5870
	ctx.r[3].s64 = ctx.r[10].s64 + 22640;
	// 8269F7AC: 39400004  li r10, 4
	ctx.r[10].s64 = 4;
	// 8269F7B0: 91010074  stw r8, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[8].u32 ) };
	// 8269F7B4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8269F7B8: 9101006C  stw r8, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[8].u32 ) };
	// 8269F7BC: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8269F7C0: 4BDC7661  bl 0x82466e20
	ctx.lr = 0x8269F7C4;
	sub_82466E20(ctx, base);
	// 8269F7C4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8269F7C8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8269F7CC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8269F7D0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8269F7D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8269F7D8 size=116
    let mut pc: u32 = 0x8269F7D8;
    'dispatch: loop {
        match pc {
            0x8269F7D8 => {
    //   block [0x8269F7D8..0x8269F84C)
	// 8269F7D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8269F7DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8269F7E0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8269F7E4: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 8269F7E8: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 8269F7EC: 390B2FC0  addi r8, r11, 0x2fc0
	ctx.r[8].s64 = ctx.r[11].s64 + 12224;
	// 8269F7F0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8269F7F4: 392AA7A8  addi r9, r10, -0x5858
	ctx.r[9].s64 = ctx.r[10].s64 + -22616;
	// 8269F7F8: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269F7FC: 38E00006  li r7, 6
	ctx.r[7].s64 = 6;
	// 8269F800: 38AA2B10  addi r5, r10, 0x2b10
	ctx.r[5].s64 = ctx.r[10].s64 + 11024;
	// 8269F804: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8269F808: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8269F80C: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8269F810: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8269F814: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8269F818: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8269F81C: 3D608297  lis r11, -0x7d69
	ctx.r[11].s64 = -2104033280;
	// 8269F820: 388AC6D4  addi r4, r10, -0x392c
	ctx.r[4].s64 = ctx.r[10].s64 + -14636;
	// 8269F824: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8269F828: 386B58A0  addi r3, r11, 0x58a0
	ctx.r[3].s64 = ctx.r[11].s64 + 22688;
	// 8269F82C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8269F830: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8269F834: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8269F838: 4BDC75E9  bl 0x82466e20
	ctx.lr = 0x8269F83C;
	sub_82466E20(ctx, base);
	// 8269F83C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8269F840: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8269F844: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8269F848: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8269F850(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8269F850 size=112
    let mut pc: u32 = 0x8269F850;
    'dispatch: loop {
        match pc {
            0x8269F850 => {
    //   block [0x8269F850..0x8269F8C0)
	// 8269F850: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8269F854: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8269F858: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8269F85C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269F860: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 8269F864: 38AA2B10  addi r5, r10, 0x2b10
	ctx.r[5].s64 = ctx.r[10].s64 + 11024;
	// 8269F868: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8269F86C: 390B3050  addi r8, r11, 0x3050
	ctx.r[8].s64 = ctx.r[11].s64 + 12368;
	// 8269F870: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8269F874: 388AC6E4  addi r4, r10, -0x391c
	ctx.r[4].s64 = ctx.r[10].s64 + -14620;
	// 8269F878: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8269F87C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269F880: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8269F884: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8269F888: 386A58D0  addi r3, r10, 0x58d0
	ctx.r[3].s64 = ctx.r[10].s64 + 22736;
	// 8269F88C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8269F890: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8269F894: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8269F898: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8269F89C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8269F8A0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8269F8A4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8269F8A8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8269F8AC: 4BDC7575  bl 0x82466e20
	ctx.lr = 0x8269F8B0;
	sub_82466E20(ctx, base);
	// 8269F8B0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8269F8B4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8269F8B8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8269F8BC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8269F8C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8269F8C0 size=36
    let mut pc: u32 = 0x8269F8C0;
    'dispatch: loop {
        match pc {
            0x8269F8C0 => {
    //   block [0x8269F8C0..0x8269F8E4)
	// 8269F8C0: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 8269F8C4: 814B306C  lwz r10, 0x306c(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12396 as u32) ) } as u64;
	// 8269F8C8: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 8269F8CC: 396B7968  addi r11, r11, 0x7968
	ctx.r[11].s64 = ctx.r[11].s64 + 31080;
	// 8269F8D0: 914B0020  stw r10, 0x20(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(32 as u32), ctx.r[10].u32 ) };
	// 8269F8D4: 3D40827C  lis r10, -0x7d84
	ctx.r[10].s64 = -2105802752;
	// 8269F8D8: 814A2FBC  lwz r10, 0x2fbc(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12220 as u32) ) } as u64;
	// 8269F8DC: 914B00B0  stw r10, 0xb0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(176 as u32), ctx.r[10].u32 ) };
	// 8269F8E0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8269F8E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8269F8E8 size=116
    let mut pc: u32 = 0x8269F8E8;
    'dispatch: loop {
        match pc {
            0x8269F8E8 => {
    //   block [0x8269F8E8..0x8269F95C)
	// 8269F8E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8269F8EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8269F8F0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8269F8F4: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 8269F8F8: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
	// 8269F8FC: 392AA810  addi r9, r10, -0x57f0
	ctx.r[9].s64 = ctx.r[10].s64 + -22512;
	// 8269F900: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269F904: 38C00008  li r6, 8
	ctx.r[6].s64 = 8;
	// 8269F908: 38AA2B10  addi r5, r10, 0x2b10
	ctx.r[5].s64 = ctx.r[10].s64 + 11024;
	// 8269F90C: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8269F910: 90E10074  stw r7, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[7].u32 ) };
	// 8269F914: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 8269F918: 388AC75C  addi r4, r10, -0x38a4
	ctx.r[4].s64 = ctx.r[10].s64 + -14500;
	// 8269F91C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8269F920: 90C1005C  stw r6, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[6].u32 ) };
	// 8269F924: 396B7968  addi r11, r11, 0x7968
	ctx.r[11].s64 = ctx.r[11].s64 + 31080;
	// 8269F928: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269F92C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8269F930: 386A5900  addi r3, r10, 0x5900
	ctx.r[3].s64 = ctx.r[10].s64 + 22784;
	// 8269F934: 39400002  li r10, 2
	ctx.r[10].s64 = 2;
	// 8269F938: 9101006C  stw r8, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[8].u32 ) };
	// 8269F93C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8269F940: 91010064  stw r8, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[8].u32 ) };
	// 8269F944: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8269F948: 4BDC74D9  bl 0x82466e20
	ctx.lr = 0x8269F94C;
	sub_82466E20(ctx, base);
	// 8269F94C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8269F950: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8269F954: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8269F958: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8269F960(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8269F960 size=108
    let mut pc: u32 = 0x8269F960;
    'dispatch: loop {
        match pc {
            0x8269F960 => {
    //   block [0x8269F960..0x8269F9CC)
	// 8269F960: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8269F964: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8269F968: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8269F96C: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 8269F970: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8269F974: 38EB3070  addi r7, r11, 0x3070
	ctx.r[7].s64 = ctx.r[11].s64 + 12400;
	// 8269F978: 39000005  li r8, 5
	ctx.r[8].s64 = 5;
	// 8269F97C: 388AC76C  addi r4, r10, -0x3894
	ctx.r[4].s64 = ctx.r[10].s64 + -14484;
	// 8269F980: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8269F984: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269F988: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8269F98C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8269F990: 386A5930  addi r3, r10, 0x5930
	ctx.r[3].s64 = ctx.r[10].s64 + 22832;
	// 8269F994: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8269F998: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8269F99C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8269F9A0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8269F9A4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8269F9A8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8269F9AC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8269F9B0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8269F9B4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8269F9B8: 4BDC7469  bl 0x82466e20
	ctx.lr = 0x8269F9BC;
	sub_82466E20(ctx, base);
	// 8269F9BC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8269F9C0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8269F9C4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8269F9C8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8269F9D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8269F9D0 size=112
    let mut pc: u32 = 0x8269F9D0;
    'dispatch: loop {
        match pc {
            0x8269F9D0 => {
    //   block [0x8269F9D0..0x8269FA40)
	// 8269F9D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8269F9D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8269F9D8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8269F9DC: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269F9E0: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 8269F9E4: 38AA36B0  addi r5, r10, 0x36b0
	ctx.r[5].s64 = ctx.r[10].s64 + 14000;
	// 8269F9E8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8269F9EC: 390B30E8  addi r8, r11, 0x30e8
	ctx.r[8].s64 = ctx.r[11].s64 + 12520;
	// 8269F9F0: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8269F9F4: 388AC780  addi r4, r10, -0x3880
	ctx.r[4].s64 = ctx.r[10].s64 + -14464;
	// 8269F9F8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8269F9FC: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269FA00: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8269FA04: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8269FA08: 386A5960  addi r3, r10, 0x5960
	ctx.r[3].s64 = ctx.r[10].s64 + 22880;
	// 8269FA0C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8269FA10: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8269FA14: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8269FA18: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8269FA1C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8269FA20: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8269FA24: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8269FA28: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8269FA2C: 4BDC73F5  bl 0x82466e20
	ctx.lr = 0x8269FA30;
	sub_82466E20(ctx, base);
	// 8269FA30: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8269FA34: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8269FA38: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8269FA3C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8269FA40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8269FA40 size=108
    let mut pc: u32 = 0x8269FA40;
    'dispatch: loop {
        match pc {
            0x8269FA40 => {
    //   block [0x8269FA40..0x8269FAAC)
	// 8269FA40: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8269FA44: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8269FA48: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8269FA4C: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 8269FA50: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8269FA54: 38EB3100  addi r7, r11, 0x3100
	ctx.r[7].s64 = ctx.r[11].s64 + 12544;
	// 8269FA58: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 8269FA5C: 388AC794  addi r4, r10, -0x386c
	ctx.r[4].s64 = ctx.r[10].s64 + -14444;
	// 8269FA60: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8269FA64: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269FA68: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8269FA6C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8269FA70: 386A5990  addi r3, r10, 0x5990
	ctx.r[3].s64 = ctx.r[10].s64 + 22928;
	// 8269FA74: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8269FA78: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8269FA7C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8269FA80: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8269FA84: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8269FA88: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8269FA8C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8269FA90: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8269FA94: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8269FA98: 4BDC7389  bl 0x82466e20
	ctx.lr = 0x8269FA9C;
	sub_82466E20(ctx, base);
	// 8269FA9C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8269FAA0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8269FAA4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8269FAA8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8269FAB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8269FAB0 size=112
    let mut pc: u32 = 0x8269FAB0;
    'dispatch: loop {
        match pc {
            0x8269FAB0 => {
    //   block [0x8269FAB0..0x8269FB20)
	// 8269FAB0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8269FAB4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8269FAB8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8269FABC: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269FAC0: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 8269FAC4: 38AA2B10  addi r5, r10, 0x2b10
	ctx.r[5].s64 = ctx.r[10].s64 + 11024;
	// 8269FAC8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8269FACC: 390B3118  addi r8, r11, 0x3118
	ctx.r[8].s64 = ctx.r[11].s64 + 12568;
	// 8269FAD0: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 8269FAD4: 388AC7A8  addi r4, r10, -0x3858
	ctx.r[4].s64 = ctx.r[10].s64 + -14424;
	// 8269FAD8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8269FADC: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269FAE0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8269FAE4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8269FAE8: 386A59C0  addi r3, r10, 0x59c0
	ctx.r[3].s64 = ctx.r[10].s64 + 22976;
	// 8269FAEC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8269FAF0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8269FAF4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8269FAF8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8269FAFC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8269FB00: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8269FB04: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8269FB08: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8269FB0C: 4BDC7315  bl 0x82466e20
	ctx.lr = 0x8269FB10;
	sub_82466E20(ctx, base);
	// 8269FB10: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8269FB14: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8269FB18: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8269FB1C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8269FB20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8269FB20 size=108
    let mut pc: u32 = 0x8269FB20;
    'dispatch: loop {
        match pc {
            0x8269FB20 => {
    //   block [0x8269FB20..0x8269FB8C)
	// 8269FB20: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8269FB24: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8269FB28: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8269FB2C: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 8269FB30: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8269FB34: 38EB3160  addi r7, r11, 0x3160
	ctx.r[7].s64 = ctx.r[11].s64 + 12640;
	// 8269FB38: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 8269FB3C: 388AC7C4  addi r4, r10, -0x383c
	ctx.r[4].s64 = ctx.r[10].s64 + -14396;
	// 8269FB40: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8269FB44: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269FB48: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8269FB4C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8269FB50: 386A59F0  addi r3, r10, 0x59f0
	ctx.r[3].s64 = ctx.r[10].s64 + 23024;
	// 8269FB54: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8269FB58: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8269FB5C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8269FB60: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8269FB64: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8269FB68: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8269FB6C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8269FB70: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8269FB74: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8269FB78: 4BDC72A9  bl 0x82466e20
	ctx.lr = 0x8269FB7C;
	sub_82466E20(ctx, base);
	// 8269FB7C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8269FB80: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8269FB84: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8269FB88: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8269FB90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8269FB90 size=108
    let mut pc: u32 = 0x8269FB90;
    'dispatch: loop {
        match pc {
            0x8269FB90 => {
    //   block [0x8269FB90..0x8269FBFC)
	// 8269FB90: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8269FB94: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8269FB98: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8269FB9C: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 8269FBA0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8269FBA4: 38EB3194  addi r7, r11, 0x3194
	ctx.r[7].s64 = ctx.r[11].s64 + 12692;
	// 8269FBA8: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 8269FBAC: 388AC7E4  addi r4, r10, -0x381c
	ctx.r[4].s64 = ctx.r[10].s64 + -14364;
	// 8269FBB0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8269FBB4: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269FBB8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8269FBBC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8269FBC0: 386A5A20  addi r3, r10, 0x5a20
	ctx.r[3].s64 = ctx.r[10].s64 + 23072;
	// 8269FBC4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8269FBC8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8269FBCC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8269FBD0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8269FBD4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8269FBD8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8269FBDC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8269FBE0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8269FBE4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8269FBE8: 4BDC7239  bl 0x82466e20
	ctx.lr = 0x8269FBEC;
	sub_82466E20(ctx, base);
	// 8269FBEC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8269FBF0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8269FBF4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8269FBF8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8269FC00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8269FC00 size=24
    let mut pc: u32 = 0x8269FC00;
    'dispatch: loop {
        match pc {
            0x8269FC00 => {
    //   block [0x8269FC00..0x8269FC18)
	// 8269FC00: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 8269FC04: 3D40827C  lis r10, -0x7d84
	ctx.r[10].s64 = -2105802752;
	// 8269FC08: 394A7A28  addi r10, r10, 0x7a28
	ctx.r[10].s64 = ctx.r[10].s64 + 31272;
	// 8269FC0C: 816B3190  lwz r11, 0x3190(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12688 as u32) ) } as u64;
	// 8269FC10: 916A0038  stw r11, 0x38(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(56 as u32), ctx.r[11].u32 ) };
	// 8269FC14: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8269FC18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8269FC18 size=116
    let mut pc: u32 = 0x8269FC18;
    'dispatch: loop {
        match pc {
            0x8269FC18 => {
    //   block [0x8269FC18..0x8269FC8C)
	// 8269FC18: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8269FC1C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8269FC20: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8269FC24: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 8269FC28: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 8269FC2C: 390B7A28  addi r8, r11, 0x7a28
	ctx.r[8].s64 = ctx.r[11].s64 + 31272;
	// 8269FC30: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8269FC34: 392AA848  addi r9, r10, -0x57b8
	ctx.r[9].s64 = ctx.r[10].s64 + -22456;
	// 8269FC38: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269FC3C: 38E00003  li r7, 3
	ctx.r[7].s64 = 3;
	// 8269FC40: 38AA2B10  addi r5, r10, 0x2b10
	ctx.r[5].s64 = ctx.r[10].s64 + 11024;
	// 8269FC44: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8269FC48: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8269FC4C: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8269FC50: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8269FC54: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8269FC58: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8269FC5C: 3D608297  lis r11, -0x7d69
	ctx.r[11].s64 = -2104033280;
	// 8269FC60: 388AC7F8  addi r4, r10, -0x3808
	ctx.r[4].s64 = ctx.r[10].s64 + -14344;
	// 8269FC64: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8269FC68: 386B5A50  addi r3, r11, 0x5a50
	ctx.r[3].s64 = ctx.r[11].s64 + 23120;
	// 8269FC6C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8269FC70: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8269FC74: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8269FC78: 4BDC71A9  bl 0x82466e20
	ctx.lr = 0x8269FC7C;
	sub_82466E20(ctx, base);
	// 8269FC7C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8269FC80: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8269FC84: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8269FC88: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8269FC90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8269FC90 size=112
    let mut pc: u32 = 0x8269FC90;
    'dispatch: loop {
        match pc {
            0x8269FC90 => {
    //   block [0x8269FC90..0x8269FD00)
	// 8269FC90: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8269FC94: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8269FC98: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8269FC9C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269FCA0: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 8269FCA4: 38AA4910  addi r5, r10, 0x4910
	ctx.r[5].s64 = ctx.r[10].s64 + 18704;
	// 8269FCA8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8269FCAC: 390B31B0  addi r8, r11, 0x31b0
	ctx.r[8].s64 = ctx.r[11].s64 + 12720;
	// 8269FCB0: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 8269FCB4: 388AC804  addi r4, r10, -0x37fc
	ctx.r[4].s64 = ctx.r[10].s64 + -14332;
	// 8269FCB8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8269FCBC: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269FCC0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8269FCC4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8269FCC8: 386A5A80  addi r3, r10, 0x5a80
	ctx.r[3].s64 = ctx.r[10].s64 + 23168;
	// 8269FCCC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8269FCD0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8269FCD4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8269FCD8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8269FCDC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8269FCE0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8269FCE4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8269FCE8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8269FCEC: 4BDC7135  bl 0x82466e20
	ctx.lr = 0x8269FCF0;
	sub_82466E20(ctx, base);
	// 8269FCF0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8269FCF4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8269FCF8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8269FCFC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8269FD00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8269FD00 size=112
    let mut pc: u32 = 0x8269FD00;
    'dispatch: loop {
        match pc {
            0x8269FD00 => {
    //   block [0x8269FD00..0x8269FD70)
	// 8269FD00: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8269FD04: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8269FD08: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8269FD0C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269FD10: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 8269FD14: 38AA4910  addi r5, r10, 0x4910
	ctx.r[5].s64 = ctx.r[10].s64 + 18704;
	// 8269FD18: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8269FD1C: 390B31F8  addi r8, r11, 0x31f8
	ctx.r[8].s64 = ctx.r[11].s64 + 12792;
	// 8269FD20: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 8269FD24: 388AC81C  addi r4, r10, -0x37e4
	ctx.r[4].s64 = ctx.r[10].s64 + -14308;
	// 8269FD28: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8269FD2C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269FD30: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8269FD34: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8269FD38: 386A5AB0  addi r3, r10, 0x5ab0
	ctx.r[3].s64 = ctx.r[10].s64 + 23216;
	// 8269FD3C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8269FD40: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8269FD44: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8269FD48: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8269FD4C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8269FD50: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8269FD54: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8269FD58: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8269FD5C: 4BDC70C5  bl 0x82466e20
	ctx.lr = 0x8269FD60;
	sub_82466E20(ctx, base);
	// 8269FD60: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8269FD64: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8269FD68: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8269FD6C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8269FD70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8269FD70 size=112
    let mut pc: u32 = 0x8269FD70;
    'dispatch: loop {
        match pc {
            0x8269FD70 => {
    //   block [0x8269FD70..0x8269FDE0)
	// 8269FD70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8269FD74: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8269FD78: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8269FD7C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269FD80: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 8269FD84: 38AA4940  addi r5, r10, 0x4940
	ctx.r[5].s64 = ctx.r[10].s64 + 18752;
	// 8269FD88: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8269FD8C: 390B3258  addi r8, r11, 0x3258
	ctx.r[8].s64 = ctx.r[11].s64 + 12888;
	// 8269FD90: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 8269FD94: 388AC830  addi r4, r10, -0x37d0
	ctx.r[4].s64 = ctx.r[10].s64 + -14288;
	// 8269FD98: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8269FD9C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269FDA0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8269FDA4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8269FDA8: 386A5AE0  addi r3, r10, 0x5ae0
	ctx.r[3].s64 = ctx.r[10].s64 + 23264;
	// 8269FDAC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8269FDB0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8269FDB4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8269FDB8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8269FDBC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8269FDC0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8269FDC4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8269FDC8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8269FDCC: 4BDC7055  bl 0x82466e20
	ctx.lr = 0x8269FDD0;
	sub_82466E20(ctx, base);
	// 8269FDD0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8269FDD4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8269FDD8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8269FDDC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8269FDE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8269FDE0 size=112
    let mut pc: u32 = 0x8269FDE0;
    'dispatch: loop {
        match pc {
            0x8269FDE0 => {
    //   block [0x8269FDE0..0x8269FE50)
	// 8269FDE0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8269FDE4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8269FDE8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8269FDEC: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269FDF0: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 8269FDF4: 38AA4940  addi r5, r10, 0x4940
	ctx.r[5].s64 = ctx.r[10].s64 + 18752;
	// 8269FDF8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8269FDFC: 390B32B8  addi r8, r11, 0x32b8
	ctx.r[8].s64 = ctx.r[11].s64 + 12984;
	// 8269FE00: 39200008  li r9, 8
	ctx.r[9].s64 = 8;
	// 8269FE04: 388AC840  addi r4, r10, -0x37c0
	ctx.r[4].s64 = ctx.r[10].s64 + -14272;
	// 8269FE08: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8269FE0C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269FE10: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8269FE14: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8269FE18: 386A5B10  addi r3, r10, 0x5b10
	ctx.r[3].s64 = ctx.r[10].s64 + 23312;
	// 8269FE1C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8269FE20: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8269FE24: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8269FE28: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8269FE2C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8269FE30: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8269FE34: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8269FE38: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8269FE3C: 4BDC6FE5  bl 0x82466e20
	ctx.lr = 0x8269FE40;
	sub_82466E20(ctx, base);
	// 8269FE40: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8269FE44: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8269FE48: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8269FE4C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8269FE50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8269FE50 size=112
    let mut pc: u32 = 0x8269FE50;
    'dispatch: loop {
        match pc {
            0x8269FE50 => {
    //   block [0x8269FE50..0x8269FEC0)
	// 8269FE50: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8269FE54: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8269FE58: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8269FE5C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269FE60: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 8269FE64: 38AA4940  addi r5, r10, 0x4940
	ctx.r[5].s64 = ctx.r[10].s64 + 18752;
	// 8269FE68: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8269FE6C: 390B3378  addi r8, r11, 0x3378
	ctx.r[8].s64 = ctx.r[11].s64 + 13176;
	// 8269FE70: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 8269FE74: 388AC858  addi r4, r10, -0x37a8
	ctx.r[4].s64 = ctx.r[10].s64 + -14248;
	// 8269FE78: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8269FE7C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269FE80: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8269FE84: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8269FE88: 386A5B40  addi r3, r10, 0x5b40
	ctx.r[3].s64 = ctx.r[10].s64 + 23360;
	// 8269FE8C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8269FE90: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8269FE94: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8269FE98: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8269FE9C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8269FEA0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8269FEA4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8269FEA8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8269FEAC: 4BDC6F75  bl 0x82466e20
	ctx.lr = 0x8269FEB0;
	sub_82466E20(ctx, base);
	// 8269FEB0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8269FEB4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8269FEB8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8269FEBC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8269FEC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8269FEC0 size=112
    let mut pc: u32 = 0x8269FEC0;
    'dispatch: loop {
        match pc {
            0x8269FEC0 => {
    //   block [0x8269FEC0..0x8269FF30)
	// 8269FEC0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8269FEC4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8269FEC8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8269FECC: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269FED0: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 8269FED4: 38AA4910  addi r5, r10, 0x4910
	ctx.r[5].s64 = ctx.r[10].s64 + 18704;
	// 8269FED8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8269FEDC: 390B33D8  addi r8, r11, 0x33d8
	ctx.r[8].s64 = ctx.r[11].s64 + 13272;
	// 8269FEE0: 39200008  li r9, 8
	ctx.r[9].s64 = 8;
	// 8269FEE4: 388AC86C  addi r4, r10, -0x3794
	ctx.r[4].s64 = ctx.r[10].s64 + -14228;
	// 8269FEE8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8269FEEC: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269FEF0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8269FEF4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8269FEF8: 386A5B70  addi r3, r10, 0x5b70
	ctx.r[3].s64 = ctx.r[10].s64 + 23408;
	// 8269FEFC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8269FF00: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8269FF04: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8269FF08: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8269FF0C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8269FF10: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8269FF14: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8269FF18: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8269FF1C: 4BDC6F05  bl 0x82466e20
	ctx.lr = 0x8269FF20;
	sub_82466E20(ctx, base);
	// 8269FF20: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8269FF24: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8269FF28: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8269FF2C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8269FF30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8269FF30 size=112
    let mut pc: u32 = 0x8269FF30;
    'dispatch: loop {
        match pc {
            0x8269FF30 => {
    //   block [0x8269FF30..0x8269FFA0)
	// 8269FF30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8269FF34: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8269FF38: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8269FF3C: 3D40827C  lis r10, -0x7d84
	ctx.r[10].s64 = -2105802752;
	// 8269FF40: 39000013  li r8, 0x13
	ctx.r[8].s64 = 19;
	// 8269FF44: 38EA3498  addi r7, r10, 0x3498
	ctx.r[7].s64 = ctx.r[10].s64 + 13464;
	// 8269FF48: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8269FF4C: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 8269FF50: 388AC87C  addi r4, r10, -0x3784
	ctx.r[4].s64 = ctx.r[10].s64 + -14212;
	// 8269FF54: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8269FF58: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8269FF5C: 396BA860  addi r11, r11, -0x57a0
	ctx.r[11].s64 = ctx.r[11].s64 + -22432;
	// 8269FF60: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8269FF64: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269FF68: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8269FF6C: 386A5BA0  addi r3, r10, 0x5ba0
	ctx.r[3].s64 = ctx.r[10].s64 + 23456;
	// 8269FF70: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8269FF74: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 8269FF78: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8269FF7C: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 8269FF80: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8269FF84: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8269FF88: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8269FF8C: 4BDC6E95  bl 0x82466e20
	ctx.lr = 0x8269FF90;
	sub_82466E20(ctx, base);
	// 8269FF90: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8269FF94: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8269FF98: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8269FF9C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8269FFA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8269FFA0 size=112
    let mut pc: u32 = 0x8269FFA0;
    'dispatch: loop {
        match pc {
            0x8269FFA0 => {
    //   block [0x8269FFA0..0x826A0010)
	// 8269FFA0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8269FFA4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8269FFA8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8269FFAC: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269FFB0: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 8269FFB4: 38AA3740  addi r5, r10, 0x3740
	ctx.r[5].s64 = ctx.r[10].s64 + 14144;
	// 8269FFB8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8269FFBC: 390B3660  addi r8, r11, 0x3660
	ctx.r[8].s64 = ctx.r[11].s64 + 13920;
	// 8269FFC0: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8269FFC4: 388AC894  addi r4, r10, -0x376c
	ctx.r[4].s64 = ctx.r[10].s64 + -14188;
	// 8269FFC8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8269FFCC: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269FFD0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8269FFD4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8269FFD8: 386A5BD0  addi r3, r10, 0x5bd0
	ctx.r[3].s64 = ctx.r[10].s64 + 23504;
	// 8269FFDC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8269FFE0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8269FFE4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8269FFE8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8269FFEC: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 8269FFF0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8269FFF4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8269FFF8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8269FFFC: 4BDC6E25  bl 0x82466e20
	ctx.lr = 0x826A0000;
	sub_82466E20(ctx, base);
	// 826A0000: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A0004: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A0008: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A000C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A0010(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A0010 size=108
    let mut pc: u32 = 0x826A0010;
    'dispatch: loop {
        match pc {
            0x826A0010 => {
    //   block [0x826A0010..0x826A007C)
	// 826A0010: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A0014: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A0018: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A001C: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 826A0020: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826A0024: 38EB3678  addi r7, r11, 0x3678
	ctx.r[7].s64 = ctx.r[11].s64 + 13944;
	// 826A0028: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826A002C: 388AC8B0  addi r4, r10, -0x3750
	ctx.r[4].s64 = ctx.r[10].s64 + -14160;
	// 826A0030: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826A0034: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826A0038: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826A003C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826A0040: 386A5C00  addi r3, r10, 0x5c00
	ctx.r[3].s64 = ctx.r[10].s64 + 23552;
	// 826A0044: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826A0048: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826A004C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A0050: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826A0054: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A0058: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826A005C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A0060: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A0064: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826A0068: 4BDC6DB9  bl 0x82466e20
	ctx.lr = 0x826A006C;
	sub_82466E20(ctx, base);
	// 826A006C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A0070: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A0074: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A0078: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A0080(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A0080 size=112
    let mut pc: u32 = 0x826A0080;
    'dispatch: loop {
        match pc {
            0x826A0080 => {
    //   block [0x826A0080..0x826A00F0)
	// 826A0080: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A0084: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A0088: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A008C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826A0090: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 826A0094: 38AA3740  addi r5, r10, 0x3740
	ctx.r[5].s64 = ctx.r[10].s64 + 14144;
	// 826A0098: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826A009C: 390B36A8  addi r8, r11, 0x36a8
	ctx.r[8].s64 = ctx.r[11].s64 + 13992;
	// 826A00A0: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826A00A4: 388AC8D8  addi r4, r10, -0x3728
	ctx.r[4].s64 = ctx.r[10].s64 + -14120;
	// 826A00A8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826A00AC: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826A00B0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826A00B4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A00B8: 386A5C30  addi r3, r10, 0x5c30
	ctx.r[3].s64 = ctx.r[10].s64 + 23600;
	// 826A00BC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826A00C0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826A00C4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826A00C8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826A00CC: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826A00D0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826A00D4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A00D8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A00DC: 4BDC6D45  bl 0x82466e20
	ctx.lr = 0x826A00E0;
	sub_82466E20(ctx, base);
	// 826A00E0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A00E4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A00E8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A00EC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A00F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A00F0 size=108
    let mut pc: u32 = 0x826A00F0;
    'dispatch: loop {
        match pc {
            0x826A00F0 => {
    //   block [0x826A00F0..0x826A015C)
	// 826A00F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A00F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A00F8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A00FC: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 826A0100: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826A0104: 38EB36C0  addi r7, r11, 0x36c0
	ctx.r[7].s64 = ctx.r[11].s64 + 14016;
	// 826A0108: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826A010C: 388AC8F4  addi r4, r10, -0x370c
	ctx.r[4].s64 = ctx.r[10].s64 + -14092;
	// 826A0110: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826A0114: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826A0118: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826A011C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826A0120: 386A5C60  addi r3, r10, 0x5c60
	ctx.r[3].s64 = ctx.r[10].s64 + 23648;
	// 826A0124: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826A0128: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826A012C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A0130: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826A0134: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A0138: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826A013C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A0140: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A0144: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826A0148: 4BDC6CD9  bl 0x82466e20
	ctx.lr = 0x826A014C;
	sub_82466E20(ctx, base);
	// 826A014C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A0150: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A0154: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A0158: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


