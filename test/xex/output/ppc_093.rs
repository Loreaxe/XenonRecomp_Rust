pub fn sub_8265FAC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8265FAC0 size=100
    let mut pc: u32 = 0x8265FAC0;
    'dispatch: loop {
        match pc {
            0x8265FAC0 => {
    //   block [0x8265FAC0..0x8265FB24)
	// 8265FAC0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8265FAC4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8265FAC8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8265FACC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265FAD0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8265FAD4: 38AABB60  addi r5, r10, -0x44a0
	ctx.r[5].s64 = ctx.r[10].s64 + -17568;
	// 8265FAD8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8265FADC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8265FAE0: 388A2D2C  addi r4, r10, 0x2d2c
	ctx.r[4].s64 = ctx.r[10].s64 + 11564;
	// 8265FAE4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265FAE8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8265FAEC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8265FAF0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8265FAF4: 386AA9C0  addi r3, r10, -0x5640
	ctx.r[3].s64 = ctx.r[10].s64 + -22080;
	// 8265FAF8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8265FAFC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8265FB00: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8265FB04: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8265FB08: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8265FB0C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8265FB10: 4BE07311  bl 0x82466e20
	ctx.lr = 0x8265FB14;
	sub_82466E20(ctx, base);
	// 8265FB14: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8265FB18: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8265FB1C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8265FB20: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8265FB28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8265FB28 size=112
    let mut pc: u32 = 0x8265FB28;
    'dispatch: loop {
        match pc {
            0x8265FB28 => {
    //   block [0x8265FB28..0x8265FB98)
	// 8265FB28: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8265FB2C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8265FB30: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8265FB34: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265FB38: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 8265FB3C: 38AAA9C0  addi r5, r10, -0x5640
	ctx.r[5].s64 = ctx.r[10].s64 + -22080;
	// 8265FB40: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8265FB44: 390B3208  addi r8, r11, 0x3208
	ctx.r[8].s64 = ctx.r[11].s64 + 12808;
	// 8265FB48: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 8265FB4C: 388A2D48  addi r4, r10, 0x2d48
	ctx.r[4].s64 = ctx.r[10].s64 + 11592;
	// 8265FB50: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8265FB54: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265FB58: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8265FB5C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8265FB60: 386AA9F0  addi r3, r10, -0x5610
	ctx.r[3].s64 = ctx.r[10].s64 + -22032;
	// 8265FB64: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8265FB68: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8265FB6C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8265FB70: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8265FB74: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8265FB78: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8265FB7C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8265FB80: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8265FB84: 4BE0729D  bl 0x82466e20
	ctx.lr = 0x8265FB88;
	sub_82466E20(ctx, base);
	// 8265FB88: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8265FB8C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8265FB90: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8265FB94: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8265FB98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8265FB98 size=108
    let mut pc: u32 = 0x8265FB98;
    'dispatch: loop {
        match pc {
            0x8265FB98 => {
    //   block [0x8265FB98..0x8265FC04)
	// 8265FB98: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8265FB9C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8265FBA0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8265FBA4: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 8265FBA8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8265FBAC: 38EB3268  addi r7, r11, 0x3268
	ctx.r[7].s64 = ctx.r[11].s64 + 12904;
	// 8265FBB0: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 8265FBB4: 388A2D68  addi r4, r10, 0x2d68
	ctx.r[4].s64 = ctx.r[10].s64 + 11624;
	// 8265FBB8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8265FBBC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265FBC0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8265FBC4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8265FBC8: 386AAA20  addi r3, r10, -0x55e0
	ctx.r[3].s64 = ctx.r[10].s64 + -21984;
	// 8265FBCC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8265FBD0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8265FBD4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8265FBD8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8265FBDC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8265FBE0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8265FBE4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8265FBE8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8265FBEC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8265FBF0: 4BE07231  bl 0x82466e20
	ctx.lr = 0x8265FBF4;
	sub_82466E20(ctx, base);
	// 8265FBF4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8265FBF8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8265FBFC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8265FC00: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8265FC08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8265FC08 size=108
    let mut pc: u32 = 0x8265FC08;
    'dispatch: loop {
        match pc {
            0x8265FC08 => {
    //   block [0x8265FC08..0x8265FC74)
	// 8265FC08: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8265FC0C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8265FC10: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8265FC14: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 8265FC18: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8265FC1C: 38EB3298  addi r7, r11, 0x3298
	ctx.r[7].s64 = ctx.r[11].s64 + 12952;
	// 8265FC20: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 8265FC24: 388A2D70  addi r4, r10, 0x2d70
	ctx.r[4].s64 = ctx.r[10].s64 + 11632;
	// 8265FC28: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8265FC2C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265FC30: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8265FC34: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8265FC38: 386AAA50  addi r3, r10, -0x55b0
	ctx.r[3].s64 = ctx.r[10].s64 + -21936;
	// 8265FC3C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8265FC40: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8265FC44: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8265FC48: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8265FC4C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8265FC50: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8265FC54: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8265FC58: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8265FC5C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8265FC60: 4BE071C1  bl 0x82466e20
	ctx.lr = 0x8265FC64;
	sub_82466E20(ctx, base);
	// 8265FC64: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8265FC68: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8265FC6C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8265FC70: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8265FC78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8265FC78 size=108
    let mut pc: u32 = 0x8265FC78;
    'dispatch: loop {
        match pc {
            0x8265FC78 => {
    //   block [0x8265FC78..0x8265FCE4)
	// 8265FC78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8265FC7C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8265FC80: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8265FC84: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 8265FC88: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8265FC8C: 38EB32F8  addi r7, r11, 0x32f8
	ctx.r[7].s64 = ctx.r[11].s64 + 13048;
	// 8265FC90: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 8265FC94: 388A2D84  addi r4, r10, 0x2d84
	ctx.r[4].s64 = ctx.r[10].s64 + 11652;
	// 8265FC98: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8265FC9C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265FCA0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8265FCA4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8265FCA8: 386AAA80  addi r3, r10, -0x5580
	ctx.r[3].s64 = ctx.r[10].s64 + -21888;
	// 8265FCAC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8265FCB0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8265FCB4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8265FCB8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8265FCBC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8265FCC0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8265FCC4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8265FCC8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8265FCCC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8265FCD0: 4BE07151  bl 0x82466e20
	ctx.lr = 0x8265FCD4;
	sub_82466E20(ctx, base);
	// 8265FCD4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8265FCD8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8265FCDC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8265FCE0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8265FCE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8265FCE8 size=24
    let mut pc: u32 = 0x8265FCE8;
    'dispatch: loop {
        match pc {
            0x8265FCE8 => {
    //   block [0x8265FCE8..0x8265FD00)
	// 8265FCE8: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 8265FCEC: 3D40827A  lis r10, -0x7d86
	ctx.r[10].s64 = -2105933824;
	// 8265FCF0: 394AB270  addi r10, r10, -0x4d90
	ctx.r[10].s64 = ctx.r[10].s64 + -19856;
	// 8265FCF4: 816B309C  lwz r11, 0x309c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12444 as u32) ) } as u64;
	// 8265FCF8: 916A0128  stw r11, 0x128(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(296 as u32), ctx.r[11].u32 ) };
	// 8265FCFC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8265FD00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8265FD00 size=116
    let mut pc: u32 = 0x8265FD00;
    'dispatch: loop {
        match pc {
            0x8265FD00 => {
    //   block [0x8265FD00..0x8265FD74)
	// 8265FD00: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8265FD04: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8265FD08: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8265FD0C: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 8265FD10: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 8265FD14: 390BB270  addi r8, r11, -0x4d90
	ctx.r[8].s64 = ctx.r[11].s64 + -19856;
	// 8265FD18: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8265FD1C: 392AE808  addi r9, r10, -0x17f8
	ctx.r[9].s64 = ctx.r[10].s64 + -6136;
	// 8265FD20: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265FD24: 38E0000D  li r7, 0xd
	ctx.r[7].s64 = 13;
	// 8265FD28: 38AABB60  addi r5, r10, -0x44a0
	ctx.r[5].s64 = ctx.r[10].s64 + -17568;
	// 8265FD2C: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8265FD30: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8265FD34: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8265FD38: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8265FD3C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8265FD40: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8265FD44: 3D608296  lis r11, -0x7d6a
	ctx.r[11].s64 = -2104098816;
	// 8265FD48: 388A2D90  addi r4, r10, 0x2d90
	ctx.r[4].s64 = ctx.r[10].s64 + 11664;
	// 8265FD4C: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8265FD50: 386BAAB0  addi r3, r11, -0x5550
	ctx.r[3].s64 = ctx.r[11].s64 + -21840;
	// 8265FD54: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8265FD58: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8265FD5C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8265FD60: 4BE070C1  bl 0x82466e20
	ctx.lr = 0x8265FD64;
	sub_82466E20(ctx, base);
	// 8265FD64: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8265FD68: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8265FD6C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8265FD70: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8265FD78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8265FD78 size=112
    let mut pc: u32 = 0x8265FD78;
    'dispatch: loop {
        match pc {
            0x8265FD78 => {
    //   block [0x8265FD78..0x8265FDE8)
	// 8265FD78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8265FD7C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8265FD80: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8265FD84: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265FD88: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 8265FD8C: 38AABB60  addi r5, r10, -0x44a0
	ctx.r[5].s64 = ctx.r[10].s64 + -17568;
	// 8265FD90: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8265FD94: 390B3358  addi r8, r11, 0x3358
	ctx.r[8].s64 = ctx.r[11].s64 + 13144;
	// 8265FD98: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 8265FD9C: 388A2DA4  addi r4, r10, 0x2da4
	ctx.r[4].s64 = ctx.r[10].s64 + 11684;
	// 8265FDA0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8265FDA4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265FDA8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8265FDAC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8265FDB0: 386AAAE0  addi r3, r10, -0x5520
	ctx.r[3].s64 = ctx.r[10].s64 + -21792;
	// 8265FDB4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8265FDB8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8265FDBC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8265FDC0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8265FDC4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8265FDC8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8265FDCC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8265FDD0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8265FDD4: 4BE0704D  bl 0x82466e20
	ctx.lr = 0x8265FDD8;
	sub_82466E20(ctx, base);
	// 8265FDD8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8265FDDC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8265FDE0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8265FDE4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8265FDE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8265FDE8 size=112
    let mut pc: u32 = 0x8265FDE8;
    'dispatch: loop {
        match pc {
            0x8265FDE8 => {
    //   block [0x8265FDE8..0x8265FE58)
	// 8265FDE8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8265FDEC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8265FDF0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8265FDF4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265FDF8: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 8265FDFC: 38AABB60  addi r5, r10, -0x44a0
	ctx.r[5].s64 = ctx.r[10].s64 + -17568;
	// 8265FE00: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8265FE04: 390B33A0  addi r8, r11, 0x33a0
	ctx.r[8].s64 = ctx.r[11].s64 + 13216;
	// 8265FE08: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 8265FE0C: 388A2DBC  addi r4, r10, 0x2dbc
	ctx.r[4].s64 = ctx.r[10].s64 + 11708;
	// 8265FE10: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8265FE14: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265FE18: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8265FE1C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8265FE20: 386AAB10  addi r3, r10, -0x54f0
	ctx.r[3].s64 = ctx.r[10].s64 + -21744;
	// 8265FE24: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8265FE28: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8265FE2C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8265FE30: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8265FE34: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8265FE38: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8265FE3C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8265FE40: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8265FE44: 4BE06FDD  bl 0x82466e20
	ctx.lr = 0x8265FE48;
	sub_82466E20(ctx, base);
	// 8265FE48: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8265FE4C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8265FE50: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8265FE54: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8265FE58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8265FE58 size=24
    let mut pc: u32 = 0x8265FE58;
    'dispatch: loop {
        match pc {
            0x8265FE58 => {
    //   block [0x8265FE58..0x8265FE70)
	// 8265FE58: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 8265FE5C: 3D40827A  lis r10, -0x7d86
	ctx.r[10].s64 = -2105933824;
	// 8265FE60: 394AB3A8  addi r10, r10, -0x4c58
	ctx.r[10].s64 = ctx.r[10].s64 + -19544;
	// 8265FE64: 816B33E8  lwz r11, 0x33e8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(13288 as u32) ) } as u64;
	// 8265FE68: 916A0008  stw r11, 8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 8265FE6C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8265FE70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8265FE70 size=116
    let mut pc: u32 = 0x8265FE70;
    'dispatch: loop {
        match pc {
            0x8265FE70 => {
    //   block [0x8265FE70..0x8265FEE4)
	// 8265FE70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8265FE74: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8265FE78: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8265FE7C: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 8265FE80: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 8265FE84: 390BB3A8  addi r8, r11, -0x4c58
	ctx.r[8].s64 = ctx.r[11].s64 + -19544;
	// 8265FE88: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8265FE8C: 392AE834  addi r9, r10, -0x17cc
	ctx.r[9].s64 = ctx.r[10].s64 + -6092;
	// 8265FE90: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265FE94: 38E0000B  li r7, 0xb
	ctx.r[7].s64 = 11;
	// 8265FE98: 38AAACF0  addi r5, r10, -0x5310
	ctx.r[5].s64 = ctx.r[10].s64 + -21264;
	// 8265FE9C: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8265FEA0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8265FEA4: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8265FEA8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8265FEAC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8265FEB0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8265FEB4: 3D608296  lis r11, -0x7d6a
	ctx.r[11].s64 = -2104098816;
	// 8265FEB8: 388A2DCC  addi r4, r10, 0x2dcc
	ctx.r[4].s64 = ctx.r[10].s64 + 11724;
	// 8265FEBC: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8265FEC0: 386BAB40  addi r3, r11, -0x54c0
	ctx.r[3].s64 = ctx.r[11].s64 + -21696;
	// 8265FEC4: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8265FEC8: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8265FECC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8265FED0: 4BE06F51  bl 0x82466e20
	ctx.lr = 0x8265FED4;
	sub_82466E20(ctx, base);
	// 8265FED4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8265FED8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8265FEDC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8265FEE0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8265FEE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8265FEE8 size=108
    let mut pc: u32 = 0x8265FEE8;
    'dispatch: loop {
        match pc {
            0x8265FEE8 => {
    //   block [0x8265FEE8..0x8265FF54)
	// 8265FEE8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8265FEEC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8265FEF0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8265FEF4: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 8265FEF8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8265FEFC: 38EB33F0  addi r7, r11, 0x33f0
	ctx.r[7].s64 = ctx.r[11].s64 + 13296;
	// 8265FF00: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 8265FF04: 388A2DD8  addi r4, r10, 0x2dd8
	ctx.r[4].s64 = ctx.r[10].s64 + 11736;
	// 8265FF08: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8265FF0C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265FF10: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8265FF14: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8265FF18: 386AAB70  addi r3, r10, -0x5490
	ctx.r[3].s64 = ctx.r[10].s64 + -21648;
	// 8265FF1C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8265FF20: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8265FF24: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8265FF28: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8265FF2C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8265FF30: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8265FF34: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8265FF38: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8265FF3C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8265FF40: 4BE06EE1  bl 0x82466e20
	ctx.lr = 0x8265FF44;
	sub_82466E20(ctx, base);
	// 8265FF44: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8265FF48: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8265FF4C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8265FF50: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8265FF58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8265FF58 size=108
    let mut pc: u32 = 0x8265FF58;
    'dispatch: loop {
        match pc {
            0x8265FF58 => {
    //   block [0x8265FF58..0x8265FFC4)
	// 8265FF58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8265FF5C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8265FF60: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8265FF64: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 8265FF68: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8265FF6C: 38EB3450  addi r7, r11, 0x3450
	ctx.r[7].s64 = ctx.r[11].s64 + 13392;
	// 8265FF70: 39000007  li r8, 7
	ctx.r[8].s64 = 7;
	// 8265FF74: 388A2DF0  addi r4, r10, 0x2df0
	ctx.r[4].s64 = ctx.r[10].s64 + 11760;
	// 8265FF78: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8265FF7C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265FF80: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8265FF84: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8265FF88: 386AABA0  addi r3, r10, -0x5460
	ctx.r[3].s64 = ctx.r[10].s64 + -21600;
	// 8265FF8C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8265FF90: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8265FF94: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8265FF98: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8265FF9C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8265FFA0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8265FFA4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8265FFA8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8265FFAC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8265FFB0: 4BE06E71  bl 0x82466e20
	ctx.lr = 0x8265FFB4;
	sub_82466E20(ctx, base);
	// 8265FFB4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8265FFB8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8265FFBC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8265FFC0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8265FFC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8265FFC8 size=112
    let mut pc: u32 = 0x8265FFC8;
    'dispatch: loop {
        match pc {
            0x8265FFC8 => {
    //   block [0x8265FFC8..0x82660038)
	// 8265FFC8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8265FFCC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8265FFD0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8265FFD4: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 8265FFD8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8265FFDC: 392BE868  addi r9, r11, -0x1798
	ctx.r[9].s64 = ctx.r[11].s64 + -6040;
	// 8265FFE0: 38C0000E  li r6, 0xe
	ctx.r[6].s64 = 14;
	// 8265FFE4: 38E90018  addi r7, r9, 0x18
	ctx.r[7].s64 = ctx.r[9].s64 + 24;
	// 8265FFE8: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 8265FFEC: 388A2E00  addi r4, r10, 0x2e00
	ctx.r[4].s64 = ctx.r[10].s64 + 11776;
	// 8265FFF0: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8265FFF4: 396B34F8  addi r11, r11, 0x34f8
	ctx.r[11].s64 = ctx.r[11].s64 + 13560;
	// 8265FFF8: 90C1005C  stw r6, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[6].u32 ) };
	// 8265FFFC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82660000: 90E10064  stw r7, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[7].u32 ) };
	// 82660004: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82660008: 386AABD0  addi r3, r10, -0x5430
	ctx.r[3].s64 = ctx.r[10].s64 + -21552;
	// 8266000C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82660010: 91010074  stw r8, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[8].u32 ) };
	// 82660014: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82660018: 9101006C  stw r8, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[8].u32 ) };
	// 8266001C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82660020: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82660024: 4BE06DFD  bl 0x82466e20
	ctx.lr = 0x82660028;
	sub_82466E20(ctx, base);
	// 82660028: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8266002C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82660030: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82660034: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82660038(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82660038 size=112
    let mut pc: u32 = 0x82660038;
    'dispatch: loop {
        match pc {
            0x82660038 => {
    //   block [0x82660038..0x826600A8)
	// 82660038: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8266003C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82660040: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82660044: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82660048: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 8266004C: 38AABB60  addi r5, r10, -0x44a0
	ctx.r[5].s64 = ctx.r[10].s64 + -17568;
	// 82660050: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82660054: 390B3648  addi r8, r11, 0x3648
	ctx.r[8].s64 = ctx.r[11].s64 + 13896;
	// 82660058: 39200007  li r9, 7
	ctx.r[9].s64 = 7;
	// 8266005C: 388A2E18  addi r4, r10, 0x2e18
	ctx.r[4].s64 = ctx.r[10].s64 + 11800;
	// 82660060: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82660064: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82660068: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8266006C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82660070: 386AAC00  addi r3, r10, -0x5400
	ctx.r[3].s64 = ctx.r[10].s64 + -21504;
	// 82660074: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82660078: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8266007C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82660080: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82660084: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82660088: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8266008C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82660090: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82660094: 4BE06D8D  bl 0x82466e20
	ctx.lr = 0x82660098;
	sub_82466E20(ctx, base);
	// 82660098: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8266009C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826600A0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826600A4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826600A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826600A8 size=112
    let mut pc: u32 = 0x826600A8;
    'dispatch: loop {
        match pc {
            0x826600A8 => {
    //   block [0x826600A8..0x82660118)
	// 826600A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826600AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826600B0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826600B4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 826600B8: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 826600BC: 38AABB60  addi r5, r10, -0x44a0
	ctx.r[5].s64 = ctx.r[10].s64 + -17568;
	// 826600C0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826600C4: 390B36F0  addi r8, r11, 0x36f0
	ctx.r[8].s64 = ctx.r[11].s64 + 14064;
	// 826600C8: 39200006  li r9, 6
	ctx.r[9].s64 = 6;
	// 826600CC: 388A2E30  addi r4, r10, 0x2e30
	ctx.r[4].s64 = ctx.r[10].s64 + 11824;
	// 826600D0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826600D4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 826600D8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826600DC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826600E0: 386AAC30  addi r3, r10, -0x53d0
	ctx.r[3].s64 = ctx.r[10].s64 + -21456;
	// 826600E4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826600E8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826600EC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826600F0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826600F4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826600F8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826600FC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82660100: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82660104: 4BE06D1D  bl 0x82466e20
	ctx.lr = 0x82660108;
	sub_82466E20(ctx, base);
	// 82660108: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8266010C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82660110: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82660114: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82660118(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82660118 size=112
    let mut pc: u32 = 0x82660118;
    'dispatch: loop {
        match pc {
            0x82660118 => {
    //   block [0x82660118..0x82660188)
	// 82660118: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8266011C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82660120: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82660124: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82660128: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 8266012C: 38AABB60  addi r5, r10, -0x44a0
	ctx.r[5].s64 = ctx.r[10].s64 + -17568;
	// 82660130: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82660134: 390B3780  addi r8, r11, 0x3780
	ctx.r[8].s64 = ctx.r[11].s64 + 14208;
	// 82660138: 39200005  li r9, 5
	ctx.r[9].s64 = 5;
	// 8266013C: 388A2E44  addi r4, r10, 0x2e44
	ctx.r[4].s64 = ctx.r[10].s64 + 11844;
	// 82660140: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82660144: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82660148: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8266014C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82660150: 386AAC60  addi r3, r10, -0x53a0
	ctx.r[3].s64 = ctx.r[10].s64 + -21408;
	// 82660154: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82660158: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8266015C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82660160: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82660164: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82660168: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8266016C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82660170: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82660174: 4BE06CAD  bl 0x82466e20
	ctx.lr = 0x82660178;
	sub_82466E20(ctx, base);
	// 82660178: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8266017C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82660180: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82660184: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82660188(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82660188 size=108
    let mut pc: u32 = 0x82660188;
    'dispatch: loop {
        match pc {
            0x82660188 => {
    //   block [0x82660188..0x826601F4)
	// 82660188: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8266018C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82660190: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82660194: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 82660198: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8266019C: 38EB37F8  addi r7, r11, 0x37f8
	ctx.r[7].s64 = ctx.r[11].s64 + 14328;
	// 826601A0: 39000007  li r8, 7
	ctx.r[8].s64 = 7;
	// 826601A4: 388A2E58  addi r4, r10, 0x2e58
	ctx.r[4].s64 = ctx.r[10].s64 + 11864;
	// 826601A8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826601AC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 826601B0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826601B4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826601B8: 386AAC90  addi r3, r10, -0x5370
	ctx.r[3].s64 = ctx.r[10].s64 + -21360;
	// 826601BC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826601C0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826601C4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826601C8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826601CC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826601D0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826601D4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826601D8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826601DC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826601E0: 4BE06C41  bl 0x82466e20
	ctx.lr = 0x826601E4;
	sub_82466E20(ctx, base);
	// 826601E4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826601E8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826601EC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826601F0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826601F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826601F8 size=112
    let mut pc: u32 = 0x826601F8;
    'dispatch: loop {
        match pc {
            0x826601F8 => {
    //   block [0x826601F8..0x82660268)
	// 826601F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826601FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82660200: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82660204: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 82660208: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 8266020C: 392AE8C8  addi r9, r10, -0x1738
	ctx.r[9].s64 = ctx.r[10].s64 + -5944;
	// 82660210: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82660214: 390B38A4  addi r8, r11, 0x38a4
	ctx.r[8].s64 = ctx.r[11].s64 + 14500;
	// 82660218: 38E00002  li r7, 2
	ctx.r[7].s64 = 2;
	// 8266021C: 388A2E64  addi r4, r10, 0x2e64
	ctx.r[4].s64 = ctx.r[10].s64 + 11876;
	// 82660220: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82660224: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82660228: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8266022C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82660230: 386AACC0  addi r3, r10, -0x5340
	ctx.r[3].s64 = ctx.r[10].s64 + -21312;
	// 82660234: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82660238: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8266023C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82660240: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82660244: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82660248: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8266024C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82660250: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82660254: 4BE06BCD  bl 0x82466e20
	ctx.lr = 0x82660258;
	sub_82466E20(ctx, base);
	// 82660258: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8266025C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82660260: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82660264: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82660268(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82660268 size=100
    let mut pc: u32 = 0x82660268;
    'dispatch: loop {
        match pc {
            0x82660268 => {
    //   block [0x82660268..0x826602CC)
	// 82660268: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8266026C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82660270: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82660274: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82660278: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8266027C: 38AAB4A0  addi r5, r10, -0x4b60
	ctx.r[5].s64 = ctx.r[10].s64 + -19296;
	// 82660280: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82660284: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82660288: 388A2E70  addi r4, r10, 0x2e70
	ctx.r[4].s64 = ctx.r[10].s64 + 11888;
	// 8266028C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82660290: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82660294: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82660298: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8266029C: 386AACF0  addi r3, r10, -0x5310
	ctx.r[3].s64 = ctx.r[10].s64 + -21264;
	// 826602A0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826602A4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826602A8: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826602AC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826602B0: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826602B4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826602B8: 4BE06B69  bl 0x82466e20
	ctx.lr = 0x826602BC;
	sub_82466E20(ctx, base);
	// 826602BC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826602C0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826602C4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826602C8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826602D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826602D0 size=108
    let mut pc: u32 = 0x826602D0;
    'dispatch: loop {
        match pc {
            0x826602D0 => {
    //   block [0x826602D0..0x8266033C)
	// 826602D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826602D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826602D8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826602DC: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 826602E0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826602E4: 38EB38D8  addi r7, r11, 0x38d8
	ctx.r[7].s64 = ctx.r[11].s64 + 14552;
	// 826602E8: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826602EC: 388A2E80  addi r4, r10, 0x2e80
	ctx.r[4].s64 = ctx.r[10].s64 + 11904;
	// 826602F0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826602F4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 826602F8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826602FC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82660300: 386AAD20  addi r3, r10, -0x52e0
	ctx.r[3].s64 = ctx.r[10].s64 + -21216;
	// 82660304: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82660308: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8266030C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82660310: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82660314: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82660318: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8266031C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82660320: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82660324: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82660328: 4BE06AF9  bl 0x82466e20
	ctx.lr = 0x8266032C;
	sub_82466E20(ctx, base);
	// 8266032C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82660330: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82660334: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82660338: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82660340(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82660340 size=112
    let mut pc: u32 = 0x82660340;
    'dispatch: loop {
        match pc {
            0x82660340 => {
    //   block [0x82660340..0x826603B0)
	// 82660340: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82660344: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82660348: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8266034C: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 82660350: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 82660354: 392AE928  addi r9, r10, -0x16d8
	ctx.r[9].s64 = ctx.r[10].s64 + -5848;
	// 82660358: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8266035C: 390B3908  addi r8, r11, 0x3908
	ctx.r[8].s64 = ctx.r[11].s64 + 14600;
	// 82660360: 38E00005  li r7, 5
	ctx.r[7].s64 = 5;
	// 82660364: 388A2EA8  addi r4, r10, 0x2ea8
	ctx.r[4].s64 = ctx.r[10].s64 + 11944;
	// 82660368: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8266036C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82660370: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82660374: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82660378: 386AAD50  addi r3, r10, -0x52b0
	ctx.r[3].s64 = ctx.r[10].s64 + -21168;
	// 8266037C: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82660380: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82660384: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82660388: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8266038C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82660390: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82660394: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82660398: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8266039C: 4BE06A85  bl 0x82466e20
	ctx.lr = 0x826603A0;
	sub_82466E20(ctx, base);
	// 826603A0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826603A4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826603A8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826603AC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826603B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826603B0 size=112
    let mut pc: u32 = 0x826603B0;
    'dispatch: loop {
        match pc {
            0x826603B0 => {
    //   block [0x826603B0..0x82660420)
	// 826603B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826603B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826603B8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826603BC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 826603C0: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 826603C4: 38AAACF0  addi r5, r10, -0x5310
	ctx.r[5].s64 = ctx.r[10].s64 + -21264;
	// 826603C8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826603CC: 390B3980  addi r8, r11, 0x3980
	ctx.r[8].s64 = ctx.r[11].s64 + 14720;
	// 826603D0: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826603D4: 388A2EBC  addi r4, r10, 0x2ebc
	ctx.r[4].s64 = ctx.r[10].s64 + 11964;
	// 826603D8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826603DC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 826603E0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826603E4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826603E8: 386AAD80  addi r3, r10, -0x5280
	ctx.r[3].s64 = ctx.r[10].s64 + -21120;
	// 826603EC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826603F0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826603F4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826603F8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826603FC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82660400: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82660404: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82660408: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8266040C: 4BE06A15  bl 0x82466e20
	ctx.lr = 0x82660410;
	sub_82466E20(ctx, base);
	// 82660410: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82660414: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82660418: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8266041C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82660420(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82660420 size=116
    let mut pc: u32 = 0x82660420;
    'dispatch: loop {
        match pc {
            0x82660420 => {
    //   block [0x82660420..0x82660494)
	// 82660420: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82660424: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82660428: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8266042C: 3D408279  lis r10, -0x7d87
	ctx.r[10].s64 = -2105999360;
	// 82660430: 38E00007  li r7, 7
	ctx.r[7].s64 = 7;
	// 82660434: 390A39B0  addi r8, r10, 0x39b0
	ctx.r[8].s64 = ctx.r[10].s64 + 14768;
	// 82660438: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8266043C: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 82660440: 38AAACF0  addi r5, r10, -0x5310
	ctx.r[5].s64 = ctx.r[10].s64 + -21264;
	// 82660444: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82660448: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8266044C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82660450: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82660454: 388A2EDC  addi r4, r10, 0x2edc
	ctx.r[4].s64 = ctx.r[10].s64 + 11996;
	// 82660458: 396BE93C  addi r11, r11, -0x16c4
	ctx.r[11].s64 = ctx.r[11].s64 + -5828;
	// 8266045C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82660460: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82660464: 386AADB0  addi r3, r10, -0x5250
	ctx.r[3].s64 = ctx.r[10].s64 + -21072;
	// 82660468: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 8266046C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82660470: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 82660474: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82660478: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8266047C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82660480: 4BE069A1  bl 0x82466e20
	ctx.lr = 0x82660484;
	sub_82466E20(ctx, base);
	// 82660484: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82660488: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8266048C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82660490: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82660498(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82660498 size=100
    let mut pc: u32 = 0x82660498;
    'dispatch: loop {
        match pc {
            0x82660498 => {
    //   block [0x82660498..0x826604FC)
	// 82660498: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8266049C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826604A0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826604A4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 826604A8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826604AC: 38AAADB0  addi r5, r10, -0x5250
	ctx.r[5].s64 = ctx.r[10].s64 + -21072;
	// 826604B0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826604B4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826604B8: 388A2EF8  addi r4, r10, 0x2ef8
	ctx.r[4].s64 = ctx.r[10].s64 + 12024;
	// 826604BC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 826604C0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826604C4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826604C8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826604CC: 386AADE0  addi r3, r10, -0x5220
	ctx.r[3].s64 = ctx.r[10].s64 + -21024;
	// 826604D0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826604D4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826604D8: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826604DC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826604E0: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826604E4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826604E8: 4BE06939  bl 0x82466e20
	ctx.lr = 0x826604EC;
	sub_82466E20(ctx, base);
	// 826604EC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826604F0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826604F4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826604F8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82660500(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82660500 size=24
    let mut pc: u32 = 0x82660500;
    'dispatch: loop {
        match pc {
            0x82660500 => {
    //   block [0x82660500..0x82660518)
	// 82660500: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 82660504: 3D40827A  lis r10, -0x7d86
	ctx.r[10].s64 = -2105933824;
	// 82660508: 394AB4B0  addi r10, r10, -0x4b50
	ctx.r[10].s64 = ctx.r[10].s64 + -19280;
	// 8266050C: 816B3A58  lwz r11, 0x3a58(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(14936 as u32) ) } as u64;
	// 82660510: 916A0068  stw r11, 0x68(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(104 as u32), ctx.r[11].u32 ) };
	// 82660514: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82660518(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82660518 size=116
    let mut pc: u32 = 0x82660518;
    'dispatch: loop {
        match pc {
            0x82660518 => {
    //   block [0x82660518..0x8266058C)
	// 82660518: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8266051C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82660520: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82660524: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 82660528: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8266052C: 392BE978  addi r9, r11, -0x1688
	ctx.r[9].s64 = ctx.r[11].s64 + -5768;
	// 82660530: 38AABB60  addi r5, r10, -0x44a0
	ctx.r[5].s64 = ctx.r[10].s64 + -17568;
	// 82660534: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82660538: 38E90014  addi r7, r9, 0x14
	ctx.r[7].s64 = ctx.r[9].s64 + 20;
	// 8266053C: 38C00007  li r6, 7
	ctx.r[6].s64 = 7;
	// 82660540: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 82660544: 388A2F1C  addi r4, r10, 0x2f1c
	ctx.r[4].s64 = ctx.r[10].s64 + 12060;
	// 82660548: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8266054C: 396BB4B0  addi r11, r11, -0x4b50
	ctx.r[11].s64 = ctx.r[11].s64 + -19280;
	// 82660550: 90E10064  stw r7, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[7].u32 ) };
	// 82660554: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82660558: 90C1005C  stw r6, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[6].u32 ) };
	// 8266055C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82660560: 386AAE10  addi r3, r10, -0x51f0
	ctx.r[3].s64 = ctx.r[10].s64 + -20976;
	// 82660564: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82660568: 91010074  stw r8, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[8].u32 ) };
	// 8266056C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82660570: 9101006C  stw r8, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[8].u32 ) };
	// 82660574: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82660578: 4BE068A9  bl 0x82466e20
	ctx.lr = 0x8266057C;
	sub_82466E20(ctx, base);
	// 8266057C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82660580: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82660584: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82660588: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82660590(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82660590 size=116
    let mut pc: u32 = 0x82660590;
    'dispatch: loop {
        match pc {
            0x82660590 => {
    //   block [0x82660590..0x82660604)
	// 82660590: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82660594: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82660598: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8266059C: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 826605A0: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 826605A4: 392BE9CC  addi r9, r11, -0x1634
	ctx.r[9].s64 = ctx.r[11].s64 + -5684;
	// 826605A8: 38AAACF0  addi r5, r10, -0x5310
	ctx.r[5].s64 = ctx.r[10].s64 + -21264;
	// 826605AC: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826605B0: 38E90014  addi r7, r9, 0x14
	ctx.r[7].s64 = ctx.r[9].s64 + 20;
	// 826605B4: 38C00009  li r6, 9
	ctx.r[6].s64 = 9;
	// 826605B8: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 826605BC: 388A2F38  addi r4, r10, 0x2f38
	ctx.r[4].s64 = ctx.r[10].s64 + 12088;
	// 826605C0: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826605C4: 396B3A60  addi r11, r11, 0x3a60
	ctx.r[11].s64 = ctx.r[11].s64 + 14944;
	// 826605C8: 90E10064  stw r7, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[7].u32 ) };
	// 826605CC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 826605D0: 90C1005C  stw r6, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[6].u32 ) };
	// 826605D4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826605D8: 386AAE40  addi r3, r10, -0x51c0
	ctx.r[3].s64 = ctx.r[10].s64 + -20928;
	// 826605DC: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826605E0: 91010074  stw r8, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[8].u32 ) };
	// 826605E4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826605E8: 9101006C  stw r8, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[8].u32 ) };
	// 826605EC: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826605F0: 4BE06831  bl 0x82466e20
	ctx.lr = 0x826605F4;
	sub_82466E20(ctx, base);
	// 826605F4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826605F8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826605FC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82660600: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82660608(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82660608 size=108
    let mut pc: u32 = 0x82660608;
    'dispatch: loop {
        match pc {
            0x82660608 => {
    //   block [0x82660608..0x82660674)
	// 82660608: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8266060C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82660610: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82660614: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 82660618: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8266061C: 38EB3B38  addi r7, r11, 0x3b38
	ctx.r[7].s64 = ctx.r[11].s64 + 15160;
	// 82660620: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 82660624: 388A2F4C  addi r4, r10, 0x2f4c
	ctx.r[4].s64 = ctx.r[10].s64 + 12108;
	// 82660628: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8266062C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82660630: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82660634: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82660638: 386AAE70  addi r3, r10, -0x5190
	ctx.r[3].s64 = ctx.r[10].s64 + -20880;
	// 8266063C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82660640: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82660644: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82660648: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8266064C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82660650: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82660654: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82660658: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8266065C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82660660: 4BE067C1  bl 0x82466e20
	ctx.lr = 0x82660664;
	sub_82466E20(ctx, base);
	// 82660664: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82660668: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8266066C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82660670: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82660678(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82660678 size=24
    let mut pc: u32 = 0x82660678;
    'dispatch: loop {
        match pc {
            0x82660678 => {
    //   block [0x82660678..0x82660690)
	// 82660678: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 8266067C: 3D40827A  lis r10, -0x7d86
	ctx.r[10].s64 = -2105933824;
	// 82660680: 394AB558  addi r10, r10, -0x4aa8
	ctx.r[10].s64 = ctx.r[10].s64 + -19112;
	// 82660684: 816B3B98  lwz r11, 0x3b98(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(15256 as u32) ) } as u64;
	// 82660688: 916A0008  stw r11, 8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 8266068C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82660690(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82660690 size=116
    let mut pc: u32 = 0x82660690;
    'dispatch: loop {
        match pc {
            0x82660690 => {
    //   block [0x82660690..0x82660704)
	// 82660690: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82660694: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82660698: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8266069C: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 826606A0: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 826606A4: 390BB558  addi r8, r11, -0x4aa8
	ctx.r[8].s64 = ctx.r[11].s64 + -19112;
	// 826606A8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826606AC: 392AEA40  addi r9, r10, -0x15c0
	ctx.r[9].s64 = ctx.r[10].s64 + -5568;
	// 826606B0: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 826606B4: 38E00011  li r7, 0x11
	ctx.r[7].s64 = 17;
	// 826606B8: 38AAACF0  addi r5, r10, -0x5310
	ctx.r[5].s64 = ctx.r[10].s64 + -21264;
	// 826606BC: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826606C0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826606C4: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826606C8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826606CC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826606D0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826606D4: 3D608296  lis r11, -0x7d6a
	ctx.r[11].s64 = -2104098816;
	// 826606D8: 388A3040  addi r4, r10, 0x3040
	ctx.r[4].s64 = ctx.r[10].s64 + 12352;
	// 826606DC: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826606E0: 386BAEA0  addi r3, r11, -0x5160
	ctx.r[3].s64 = ctx.r[11].s64 + -20832;
	// 826606E4: 39400002  li r10, 2
	ctx.r[10].s64 = 2;
	// 826606E8: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826606EC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826606F0: 4BE06731  bl 0x82466e20
	ctx.lr = 0x826606F4;
	sub_82466E20(ctx, base);
	// 826606F4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826606F8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826606FC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82660700: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82660708(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82660708 size=112
    let mut pc: u32 = 0x82660708;
    'dispatch: loop {
        match pc {
            0x82660708 => {
    //   block [0x82660708..0x82660778)
	// 82660708: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8266070C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82660710: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82660714: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82660718: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 8266071C: 38AAACF0  addi r5, r10, -0x5310
	ctx.r[5].s64 = ctx.r[10].s64 + -21264;
	// 82660720: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82660724: 390B3BA0  addi r8, r11, 0x3ba0
	ctx.r[8].s64 = ctx.r[11].s64 + 15264;
	// 82660728: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 8266072C: 388A3054  addi r4, r10, 0x3054
	ctx.r[4].s64 = ctx.r[10].s64 + 12372;
	// 82660730: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82660734: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82660738: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8266073C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82660740: 386AAED0  addi r3, r10, -0x5130
	ctx.r[3].s64 = ctx.r[10].s64 + -20784;
	// 82660744: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82660748: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8266074C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82660750: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82660754: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82660758: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8266075C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82660760: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82660764: 4BE066BD  bl 0x82466e20
	ctx.lr = 0x82660768;
	sub_82466E20(ctx, base);
	// 82660768: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8266076C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82660770: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82660774: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82660778(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82660778 size=24
    let mut pc: u32 = 0x82660778;
    'dispatch: loop {
        match pc {
            0x82660778 => {
    //   block [0x82660778..0x82660790)
	// 82660778: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 8266077C: 3D40827A  lis r10, -0x7d86
	ctx.r[10].s64 = -2105933824;
	// 82660780: 394AB6F0  addi r10, r10, -0x4910
	ctx.r[10].s64 = ctx.r[10].s64 + -18704;
	// 82660784: 816B3BD0  lwz r11, 0x3bd0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(15312 as u32) ) } as u64;
	// 82660788: 916A0068  stw r11, 0x68(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(104 as u32), ctx.r[11].u32 ) };
	// 8266078C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82660790(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82660790 size=116
    let mut pc: u32 = 0x82660790;
    'dispatch: loop {
        match pc {
            0x82660790 => {
    //   block [0x82660790..0x82660804)
	// 82660790: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82660794: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82660798: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8266079C: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 826607A0: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 826607A4: 390BB6F0  addi r8, r11, -0x4910
	ctx.r[8].s64 = ctx.r[11].s64 + -18704;
	// 826607A8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826607AC: 392AEA78  addi r9, r10, -0x1588
	ctx.r[9].s64 = ctx.r[10].s64 + -5512;
	// 826607B0: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 826607B4: 38E0000A  li r7, 0xa
	ctx.r[7].s64 = 10;
	// 826607B8: 38AAAE40  addi r5, r10, -0x51c0
	ctx.r[5].s64 = ctx.r[10].s64 + -20928;
	// 826607BC: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826607C0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826607C4: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826607C8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826607CC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826607D0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826607D4: 3D608296  lis r11, -0x7d6a
	ctx.r[11].s64 = -2104098816;
	// 826607D8: 388A30BC  addi r4, r10, 0x30bc
	ctx.r[4].s64 = ctx.r[10].s64 + 12476;
	// 826607DC: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826607E0: 386BAF00  addi r3, r11, -0x5100
	ctx.r[3].s64 = ctx.r[11].s64 + -20736;
	// 826607E4: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826607E8: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826607EC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826607F0: 4BE06631  bl 0x82466e20
	ctx.lr = 0x826607F4;
	sub_82466E20(ctx, base);
	// 826607F4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826607F8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826607FC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82660800: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82660808(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82660808 size=112
    let mut pc: u32 = 0x82660808;
    'dispatch: loop {
        match pc {
            0x82660808 => {
    //   block [0x82660808..0x82660878)
	// 82660808: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8266080C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82660810: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82660814: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82660818: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 8266081C: 38AAACF0  addi r5, r10, -0x5310
	ctx.r[5].s64 = ctx.r[10].s64 + -21264;
	// 82660820: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82660824: 390B3BD4  addi r8, r11, 0x3bd4
	ctx.r[8].s64 = ctx.r[11].s64 + 15316;
	// 82660828: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8266082C: 388A30D8  addi r4, r10, 0x30d8
	ctx.r[4].s64 = ctx.r[10].s64 + 12504;
	// 82660830: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82660834: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82660838: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8266083C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82660840: 386AAF30  addi r3, r10, -0x50d0
	ctx.r[3].s64 = ctx.r[10].s64 + -20688;
	// 82660844: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82660848: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8266084C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82660850: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82660854: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82660858: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8266085C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82660860: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82660864: 4BE065BD  bl 0x82466e20
	ctx.lr = 0x82660868;
	sub_82466E20(ctx, base);
	// 82660868: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8266086C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82660870: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82660874: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82660878(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82660878 size=100
    let mut pc: u32 = 0x82660878;
    'dispatch: loop {
        match pc {
            0x82660878 => {
    //   block [0x82660878..0x826608DC)
	// 82660878: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8266087C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82660880: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82660884: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82660888: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8266088C: 38AAB4A0  addi r5, r10, -0x4b60
	ctx.r[5].s64 = ctx.r[10].s64 + -19296;
	// 82660890: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82660894: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82660898: 388A30F4  addi r4, r10, 0x30f4
	ctx.r[4].s64 = ctx.r[10].s64 + 12532;
	// 8266089C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 826608A0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826608A4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826608A8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826608AC: 386AAF60  addi r3, r10, -0x50a0
	ctx.r[3].s64 = ctx.r[10].s64 + -20640;
	// 826608B0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826608B4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826608B8: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826608BC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826608C0: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826608C4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826608C8: 4BE06559  bl 0x82466e20
	ctx.lr = 0x826608CC;
	sub_82466E20(ctx, base);
	// 826608CC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826608D0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826608D4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826608D8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826608E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826608E0 size=108
    let mut pc: u32 = 0x826608E0;
    'dispatch: loop {
        match pc {
            0x826608E0 => {
    //   block [0x826608E0..0x8266094C)
	// 826608E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826608E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826608E8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826608EC: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 826608F0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826608F4: 38EB3BF0  addi r7, r11, 0x3bf0
	ctx.r[7].s64 = ctx.r[11].s64 + 15344;
	// 826608F8: 39000009  li r8, 9
	ctx.r[8].s64 = 9;
	// 826608FC: 388A3100  addi r4, r10, 0x3100
	ctx.r[4].s64 = ctx.r[10].s64 + 12544;
	// 82660900: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82660904: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82660908: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8266090C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82660910: 386AAF90  addi r3, r10, -0x5070
	ctx.r[3].s64 = ctx.r[10].s64 + -20592;
	// 82660914: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82660918: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8266091C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82660920: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82660924: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82660928: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8266092C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82660930: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82660934: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82660938: 4BE064E9  bl 0x82466e20
	ctx.lr = 0x8266093C;
	sub_82466E20(ctx, base);
	// 8266093C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82660940: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82660944: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82660948: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82660950(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82660950 size=112
    let mut pc: u32 = 0x82660950;
    'dispatch: loop {
        match pc {
            0x82660950 => {
    //   block [0x82660950..0x826609C0)
	// 82660950: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82660954: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82660958: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8266095C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82660960: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 82660964: 38AAAF60  addi r5, r10, -0x50a0
	ctx.r[5].s64 = ctx.r[10].s64 + -20640;
	// 82660968: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8266096C: 390B3CC8  addi r8, r11, 0x3cc8
	ctx.r[8].s64 = ctx.r[11].s64 + 15560;
	// 82660970: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 82660974: 388A312C  addi r4, r10, 0x312c
	ctx.r[4].s64 = ctx.r[10].s64 + 12588;
	// 82660978: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8266097C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82660980: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82660984: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82660988: 386AAFC0  addi r3, r10, -0x5040
	ctx.r[3].s64 = ctx.r[10].s64 + -20544;
	// 8266098C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82660990: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82660994: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82660998: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8266099C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826609A0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826609A4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826609A8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826609AC: 4BE06475  bl 0x82466e20
	ctx.lr = 0x826609B0;
	sub_82466E20(ctx, base);
	// 826609B0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826609B4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826609B8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826609BC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826609C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826609C0 size=108
    let mut pc: u32 = 0x826609C0;
    'dispatch: loop {
        match pc {
            0x826609C0 => {
    //   block [0x826609C0..0x82660A2C)
	// 826609C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826609C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826609C8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826609CC: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 826609D0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826609D4: 38EB3CF8  addi r7, r11, 0x3cf8
	ctx.r[7].s64 = ctx.r[11].s64 + 15608;
	// 826609D8: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826609DC: 388A3144  addi r4, r10, 0x3144
	ctx.r[4].s64 = ctx.r[10].s64 + 12612;
	// 826609E0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826609E4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 826609E8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826609EC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826609F0: 386AAFF0  addi r3, r10, -0x5010
	ctx.r[3].s64 = ctx.r[10].s64 + -20496;
	// 826609F4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826609F8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826609FC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82660A00: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82660A04: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82660A08: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82660A0C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82660A10: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82660A14: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82660A18: 4BE06409  bl 0x82466e20
	ctx.lr = 0x82660A1C;
	sub_82466E20(ctx, base);
	// 82660A1C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82660A20: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82660A24: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82660A28: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82660A30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82660A30 size=112
    let mut pc: u32 = 0x82660A30;
    'dispatch: loop {
        match pc {
            0x82660A30 => {
    //   block [0x82660A30..0x82660AA0)
	// 82660A30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82660A34: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82660A38: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82660A3C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82660A40: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 82660A44: 38AAAF60  addi r5, r10, -0x50a0
	ctx.r[5].s64 = ctx.r[10].s64 + -20640;
	// 82660A48: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82660A4C: 390B3D28  addi r8, r11, 0x3d28
	ctx.r[8].s64 = ctx.r[11].s64 + 15656;
	// 82660A50: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82660A54: 388A3164  addi r4, r10, 0x3164
	ctx.r[4].s64 = ctx.r[10].s64 + 12644;
	// 82660A58: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82660A5C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82660A60: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82660A64: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82660A68: 386AB020  addi r3, r10, -0x4fe0
	ctx.r[3].s64 = ctx.r[10].s64 + -20448;
	// 82660A6C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82660A70: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82660A74: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82660A78: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82660A7C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82660A80: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82660A84: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82660A88: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82660A8C: 4BE06395  bl 0x82466e20
	ctx.lr = 0x82660A90;
	sub_82466E20(ctx, base);
	// 82660A90: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82660A94: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82660A98: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82660A9C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82660AA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82660AA0 size=112
    let mut pc: u32 = 0x82660AA0;
    'dispatch: loop {
        match pc {
            0x82660AA0 => {
    //   block [0x82660AA0..0x82660B10)
	// 82660AA0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82660AA4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82660AA8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82660AAC: 3D408279  lis r10, -0x7d87
	ctx.r[10].s64 = -2105999360;
	// 82660AB0: 39000009  li r8, 9
	ctx.r[8].s64 = 9;
	// 82660AB4: 38EA3D40  addi r7, r10, 0x3d40
	ctx.r[7].s64 = ctx.r[10].s64 + 15680;
	// 82660AB8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82660ABC: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 82660AC0: 388A317C  addi r4, r10, 0x317c
	ctx.r[4].s64 = ctx.r[10].s64 + 12668;
	// 82660AC4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82660AC8: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82660ACC: 396BEA8C  addi r11, r11, -0x1574
	ctx.r[11].s64 = ctx.r[11].s64 + -5492;
	// 82660AD0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82660AD4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82660AD8: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82660ADC: 386AB050  addi r3, r10, -0x4fb0
	ctx.r[3].s64 = ctx.r[10].s64 + -20400;
	// 82660AE0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82660AE4: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 82660AE8: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82660AEC: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 82660AF0: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82660AF4: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82660AF8: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82660AFC: 4BE06325  bl 0x82466e20
	ctx.lr = 0x82660B00;
	sub_82466E20(ctx, base);
	// 82660B00: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82660B04: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82660B08: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82660B0C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82660B10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82660B10 size=108
    let mut pc: u32 = 0x82660B10;
    'dispatch: loop {
        match pc {
            0x82660B10 => {
    //   block [0x82660B10..0x82660B7C)
	// 82660B10: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82660B14: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82660B18: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82660B1C: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 82660B20: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82660B24: 38EB3E18  addi r7, r11, 0x3e18
	ctx.r[7].s64 = ctx.r[11].s64 + 15896;
	// 82660B28: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 82660B2C: 388A318C  addi r4, r10, 0x318c
	ctx.r[4].s64 = ctx.r[10].s64 + 12684;
	// 82660B30: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82660B34: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82660B38: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82660B3C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82660B40: 386AB080  addi r3, r10, -0x4f80
	ctx.r[3].s64 = ctx.r[10].s64 + -20352;
	// 82660B44: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82660B48: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82660B4C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82660B50: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82660B54: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82660B58: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82660B5C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82660B60: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82660B64: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82660B68: 4BE062B9  bl 0x82466e20
	ctx.lr = 0x82660B6C;
	sub_82466E20(ctx, base);
	// 82660B6C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82660B70: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82660B74: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82660B78: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82660B80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82660B80 size=108
    let mut pc: u32 = 0x82660B80;
    'dispatch: loop {
        match pc {
            0x82660B80 => {
    //   block [0x82660B80..0x82660BEC)
	// 82660B80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82660B84: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82660B88: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82660B8C: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 82660B90: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82660B94: 38EB3E30  addi r7, r11, 0x3e30
	ctx.r[7].s64 = ctx.r[11].s64 + 15920;
	// 82660B98: 3900000B  li r8, 0xb
	ctx.r[8].s64 = 11;
	// 82660B9C: 388A31A4  addi r4, r10, 0x31a4
	ctx.r[4].s64 = ctx.r[10].s64 + 12708;
	// 82660BA0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82660BA4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82660BA8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82660BAC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82660BB0: 386AB0B0  addi r3, r10, -0x4f50
	ctx.r[3].s64 = ctx.r[10].s64 + -20304;
	// 82660BB4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82660BB8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82660BBC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82660BC0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82660BC4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82660BC8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82660BCC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82660BD0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82660BD4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82660BD8: 4BE06249  bl 0x82466e20
	ctx.lr = 0x82660BDC;
	sub_82466E20(ctx, base);
	// 82660BDC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82660BE0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82660BE4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82660BE8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82660BF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82660BF0 size=108
    let mut pc: u32 = 0x82660BF0;
    'dispatch: loop {
        match pc {
            0x82660BF0 => {
    //   block [0x82660BF0..0x82660C5C)
	// 82660BF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82660BF4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82660BF8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82660BFC: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 82660C00: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82660C04: 38EB3F38  addi r7, r11, 0x3f38
	ctx.r[7].s64 = ctx.r[11].s64 + 16184;
	// 82660C08: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 82660C0C: 388A31BC  addi r4, r10, 0x31bc
	ctx.r[4].s64 = ctx.r[10].s64 + 12732;
	// 82660C10: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82660C14: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82660C18: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82660C1C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82660C20: 386AB0E0  addi r3, r10, -0x4f20
	ctx.r[3].s64 = ctx.r[10].s64 + -20256;
	// 82660C24: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82660C28: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82660C2C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82660C30: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82660C34: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82660C38: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82660C3C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82660C40: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82660C44: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82660C48: 4BE061D9  bl 0x82466e20
	ctx.lr = 0x82660C4C;
	sub_82466E20(ctx, base);
	// 82660C4C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82660C50: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82660C54: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82660C58: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82660C60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82660C60 size=112
    let mut pc: u32 = 0x82660C60;
    'dispatch: loop {
        match pc {
            0x82660C60 => {
    //   block [0x82660C60..0x82660CD0)
	// 82660C60: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82660C64: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82660C68: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82660C6C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82660C70: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 82660C74: 38AAAF60  addi r5, r10, -0x50a0
	ctx.r[5].s64 = ctx.r[10].s64 + -20640;
	// 82660C78: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82660C7C: 390B3F98  addi r8, r11, 0x3f98
	ctx.r[8].s64 = ctx.r[11].s64 + 16280;
	// 82660C80: 3920000C  li r9, 0xc
	ctx.r[9].s64 = 12;
	// 82660C84: 388A31E0  addi r4, r10, 0x31e0
	ctx.r[4].s64 = ctx.r[10].s64 + 12768;
	// 82660C88: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82660C8C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82660C90: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82660C94: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82660C98: 386AB110  addi r3, r10, -0x4ef0
	ctx.r[3].s64 = ctx.r[10].s64 + -20208;
	// 82660C9C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82660CA0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82660CA4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82660CA8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82660CAC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82660CB0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82660CB4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82660CB8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82660CBC: 4BE06165  bl 0x82466e20
	ctx.lr = 0x82660CC0;
	sub_82466E20(ctx, base);
	// 82660CC0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82660CC4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82660CC8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82660CCC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82660CD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82660CD0 size=112
    let mut pc: u32 = 0x82660CD0;
    'dispatch: loop {
        match pc {
            0x82660CD0 => {
    //   block [0x82660CD0..0x82660D40)
	// 82660CD0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82660CD4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82660CD8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82660CDC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82660CE0: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 82660CE4: 38AAAF60  addi r5, r10, -0x50a0
	ctx.r[5].s64 = ctx.r[10].s64 + -20640;
	// 82660CE8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82660CEC: 390B40B8  addi r8, r11, 0x40b8
	ctx.r[8].s64 = ctx.r[11].s64 + 16568;
	// 82660CF0: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82660CF4: 388A31F4  addi r4, r10, 0x31f4
	ctx.r[4].s64 = ctx.r[10].s64 + 12788;
	// 82660CF8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82660CFC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82660D00: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82660D04: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82660D08: 386AB140  addi r3, r10, -0x4ec0
	ctx.r[3].s64 = ctx.r[10].s64 + -20160;
	// 82660D0C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82660D10: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82660D14: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82660D18: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82660D1C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82660D20: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82660D24: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82660D28: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82660D2C: 4BE060F5  bl 0x82466e20
	ctx.lr = 0x82660D30;
	sub_82466E20(ctx, base);
	// 82660D30: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82660D34: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82660D38: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82660D3C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82660D40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82660D40 size=116
    let mut pc: u32 = 0x82660D40;
    'dispatch: loop {
        match pc {
            0x82660D40 => {
    //   block [0x82660D40..0x82660DB4)
	// 82660D40: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82660D44: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82660D48: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82660D4C: 3D408279  lis r10, -0x7d87
	ctx.r[10].s64 = -2105999360;
	// 82660D50: 38E00004  li r7, 4
	ctx.r[7].s64 = 4;
	// 82660D54: 390A40D0  addi r8, r10, 0x40d0
	ctx.r[8].s64 = ctx.r[10].s64 + 16592;
	// 82660D58: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82660D5C: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 82660D60: 38AAAF60  addi r5, r10, -0x50a0
	ctx.r[5].s64 = ctx.r[10].s64 + -20640;
	// 82660D64: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82660D68: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82660D6C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82660D70: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82660D74: 388A3210  addi r4, r10, 0x3210
	ctx.r[4].s64 = ctx.r[10].s64 + 12816;
	// 82660D78: 396BEABC  addi r11, r11, -0x1544
	ctx.r[11].s64 = ctx.r[11].s64 + -5444;
	// 82660D7C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82660D80: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82660D84: 386AB170  addi r3, r10, -0x4e90
	ctx.r[3].s64 = ctx.r[10].s64 + -20112;
	// 82660D88: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 82660D8C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82660D90: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 82660D94: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82660D98: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82660D9C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82660DA0: 4BE06081  bl 0x82466e20
	ctx.lr = 0x82660DA4;
	sub_82466E20(ctx, base);
	// 82660DA4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82660DA8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82660DAC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82660DB0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82660DB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82660DB8 size=108
    let mut pc: u32 = 0x82660DB8;
    'dispatch: loop {
        match pc {
            0x82660DB8 => {
    //   block [0x82660DB8..0x82660E24)
	// 82660DB8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82660DBC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82660DC0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82660DC4: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 82660DC8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82660DCC: 38EB4130  addi r7, r11, 0x4130
	ctx.r[7].s64 = ctx.r[11].s64 + 16688;
	// 82660DD0: 39000007  li r8, 7
	ctx.r[8].s64 = 7;
	// 82660DD4: 388AA8F8  addi r4, r10, -0x5708
	ctx.r[4].s64 = ctx.r[10].s64 + -22280;
	// 82660DD8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82660DDC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82660DE0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82660DE4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82660DE8: 386AB1A0  addi r3, r10, -0x4e60
	ctx.r[3].s64 = ctx.r[10].s64 + -20064;
	// 82660DEC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82660DF0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82660DF4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82660DF8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82660DFC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82660E00: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82660E04: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82660E08: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82660E0C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82660E10: 4BE06011  bl 0x82466e20
	ctx.lr = 0x82660E14;
	sub_82466E20(ctx, base);
	// 82660E14: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82660E18: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82660E1C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82660E20: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82660E28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82660E28 size=112
    let mut pc: u32 = 0x82660E28;
    'dispatch: loop {
        match pc {
            0x82660E28 => {
    //   block [0x82660E28..0x82660E98)
	// 82660E28: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82660E2C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82660E30: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82660E34: 3D408279  lis r10, -0x7d87
	ctx.r[10].s64 = -2105999360;
	// 82660E38: 39000005  li r8, 5
	ctx.r[8].s64 = 5;
	// 82660E3C: 38EA41D8  addi r7, r10, 0x41d8
	ctx.r[7].s64 = ctx.r[10].s64 + 16856;
	// 82660E40: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82660E44: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 82660E48: 388AA910  addi r4, r10, -0x56f0
	ctx.r[4].s64 = ctx.r[10].s64 + -22256;
	// 82660E4C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82660E50: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82660E54: 396BEAD0  addi r11, r11, -0x1530
	ctx.r[11].s64 = ctx.r[11].s64 + -5424;
	// 82660E58: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82660E5C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82660E60: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82660E64: 386AB1D0  addi r3, r10, -0x4e30
	ctx.r[3].s64 = ctx.r[10].s64 + -20016;
	// 82660E68: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82660E6C: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 82660E70: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82660E74: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 82660E78: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82660E7C: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82660E80: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82660E84: 4BE05F9D  bl 0x82466e20
	ctx.lr = 0x82660E88;
	sub_82466E20(ctx, base);
	// 82660E88: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82660E8C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82660E90: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82660E94: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82660E98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82660E98 size=112
    let mut pc: u32 = 0x82660E98;
    'dispatch: loop {
        match pc {
            0x82660E98 => {
    //   block [0x82660E98..0x82660F08)
	// 82660E98: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82660E9C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82660EA0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82660EA4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82660EA8: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 82660EAC: 38AAAF60  addi r5, r10, -0x50a0
	ctx.r[5].s64 = ctx.r[10].s64 + -20640;
	// 82660EB0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82660EB4: 390B4250  addi r8, r11, 0x4250
	ctx.r[8].s64 = ctx.r[11].s64 + 16976;
	// 82660EB8: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 82660EBC: 388A3260  addi r4, r10, 0x3260
	ctx.r[4].s64 = ctx.r[10].s64 + 12896;
	// 82660EC0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82660EC4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82660EC8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82660ECC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82660ED0: 386AB200  addi r3, r10, -0x4e00
	ctx.r[3].s64 = ctx.r[10].s64 + -19968;
	// 82660ED4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82660ED8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82660EDC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82660EE0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82660EE4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82660EE8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82660EEC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82660EF0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82660EF4: 4BE05F2D  bl 0x82466e20
	ctx.lr = 0x82660EF8;
	sub_82466E20(ctx, base);
	// 82660EF8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82660EFC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82660F00: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82660F04: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82660F08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82660F08 size=112
    let mut pc: u32 = 0x82660F08;
    'dispatch: loop {
        match pc {
            0x82660F08 => {
    //   block [0x82660F08..0x82660F78)
	// 82660F08: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82660F0C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82660F10: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82660F14: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82660F18: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 82660F1C: 38AAAF60  addi r5, r10, -0x50a0
	ctx.r[5].s64 = ctx.r[10].s64 + -20640;
	// 82660F20: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82660F24: 390B4298  addi r8, r11, 0x4298
	ctx.r[8].s64 = ctx.r[11].s64 + 17048;
	// 82660F28: 3920000B  li r9, 0xb
	ctx.r[9].s64 = 11;
	// 82660F2C: 388A3274  addi r4, r10, 0x3274
	ctx.r[4].s64 = ctx.r[10].s64 + 12916;
	// 82660F30: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82660F34: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82660F38: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82660F3C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82660F40: 386AB230  addi r3, r10, -0x4dd0
	ctx.r[3].s64 = ctx.r[10].s64 + -19920;
	// 82660F44: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82660F48: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82660F4C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82660F50: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82660F54: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82660F58: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82660F5C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82660F60: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82660F64: 4BE05EBD  bl 0x82466e20
	ctx.lr = 0x82660F68;
	sub_82466E20(ctx, base);
	// 82660F68: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82660F6C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82660F70: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82660F74: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82660F78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82660F78 size=112
    let mut pc: u32 = 0x82660F78;
    'dispatch: loop {
        match pc {
            0x82660F78 => {
    //   block [0x82660F78..0x82660FE8)
	// 82660F78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82660F7C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82660F80: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82660F84: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82660F88: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 82660F8C: 38AAAF60  addi r5, r10, -0x50a0
	ctx.r[5].s64 = ctx.r[10].s64 + -20640;
	// 82660F90: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82660F94: 390B43A0  addi r8, r11, 0x43a0
	ctx.r[8].s64 = ctx.r[11].s64 + 17312;
	// 82660F98: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82660F9C: 388A3288  addi r4, r10, 0x3288
	ctx.r[4].s64 = ctx.r[10].s64 + 12936;
	// 82660FA0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82660FA4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82660FA8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82660FAC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82660FB0: 386AB260  addi r3, r10, -0x4da0
	ctx.r[3].s64 = ctx.r[10].s64 + -19872;
	// 82660FB4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82660FB8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82660FBC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82660FC0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82660FC4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82660FC8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82660FCC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82660FD0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82660FD4: 4BE05E4D  bl 0x82466e20
	ctx.lr = 0x82660FD8;
	sub_82466E20(ctx, base);
	// 82660FD8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82660FDC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82660FE0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82660FE4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82660FE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82660FE8 size=112
    let mut pc: u32 = 0x82660FE8;
    'dispatch: loop {
        match pc {
            0x82660FE8 => {
    //   block [0x82660FE8..0x82661058)
	// 82660FE8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82660FEC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82660FF0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82660FF4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82660FF8: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 82660FFC: 38AABB60  addi r5, r10, -0x44a0
	ctx.r[5].s64 = ctx.r[10].s64 + -17568;
	// 82661000: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82661004: 390B43B8  addi r8, r11, 0x43b8
	ctx.r[8].s64 = ctx.r[11].s64 + 17336;
	// 82661008: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 8266100C: 388A32A4  addi r4, r10, 0x32a4
	ctx.r[4].s64 = ctx.r[10].s64 + 12964;
	// 82661010: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82661014: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82661018: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8266101C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82661020: 386AB290  addi r3, r10, -0x4d70
	ctx.r[3].s64 = ctx.r[10].s64 + -19824;
	// 82661024: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82661028: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8266102C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82661030: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82661034: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82661038: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8266103C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82661040: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82661044: 4BE05DDD  bl 0x82466e20
	ctx.lr = 0x82661048;
	sub_82466E20(ctx, base);
	// 82661048: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8266104C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82661050: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82661054: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82661058(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82661058 size=108
    let mut pc: u32 = 0x82661058;
    'dispatch: loop {
        match pc {
            0x82661058 => {
    //   block [0x82661058..0x826610C4)
	// 82661058: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8266105C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82661060: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82661064: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 82661068: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8266106C: 38EB43E8  addi r7, r11, 0x43e8
	ctx.r[7].s64 = ctx.r[11].s64 + 17384;
	// 82661070: 39000005  li r8, 5
	ctx.r[8].s64 = 5;
	// 82661074: 388A32B4  addi r4, r10, 0x32b4
	ctx.r[4].s64 = ctx.r[10].s64 + 12980;
	// 82661078: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8266107C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82661080: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82661084: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82661088: 386AB2C0  addi r3, r10, -0x4d40
	ctx.r[3].s64 = ctx.r[10].s64 + -19776;
	// 8266108C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82661090: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82661094: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82661098: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8266109C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826610A0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826610A4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826610A8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826610AC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826610B0: 4BE05D71  bl 0x82466e20
	ctx.lr = 0x826610B4;
	sub_82466E20(ctx, base);
	// 826610B4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826610B8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826610BC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826610C0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826610C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826610C8 size=24
    let mut pc: u32 = 0x826610C8;
    'dispatch: loop {
        match pc {
            0x826610C8 => {
    //   block [0x826610C8..0x826610E0)
	// 826610C8: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 826610CC: 3D40827A  lis r10, -0x7d86
	ctx.r[10].s64 = -2105933824;
	// 826610D0: 394AB7E0  addi r10, r10, -0x4820
	ctx.r[10].s64 = ctx.r[10].s64 + -18464;
	// 826610D4: 816B3BEC  lwz r11, 0x3bec(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(15340 as u32) ) } as u64;
	// 826610D8: 916A0050  stw r11, 0x50(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 826610DC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826610E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826610E0 size=116
    let mut pc: u32 = 0x826610E0;
    'dispatch: loop {
        match pc {
            0x826610E0 => {
    //   block [0x826610E0..0x82661154)
	// 826610E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826610E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826610E8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826610EC: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 826610F0: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 826610F4: 390BB7E0  addi r8, r11, -0x4820
	ctx.r[8].s64 = ctx.r[11].s64 + -18464;
	// 826610F8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826610FC: 392AEB08  addi r9, r10, -0x14f8
	ctx.r[9].s64 = ctx.r[10].s64 + -5368;
	// 82661100: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82661104: 38E0000B  li r7, 0xb
	ctx.r[7].s64 = 11;
	// 82661108: 38AAAF60  addi r5, r10, -0x50a0
	ctx.r[5].s64 = ctx.r[10].s64 + -20640;
	// 8266110C: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82661110: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82661114: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82661118: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8266111C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82661120: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82661124: 3D608296  lis r11, -0x7d6a
	ctx.r[11].s64 = -2104098816;
	// 82661128: 388A3300  addi r4, r10, 0x3300
	ctx.r[4].s64 = ctx.r[10].s64 + 13056;
	// 8266112C: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82661130: 386BB2F0  addi r3, r11, -0x4d10
	ctx.r[3].s64 = ctx.r[11].s64 + -19728;
	// 82661134: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82661138: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8266113C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82661140: 4BE05CE1  bl 0x82466e20
	ctx.lr = 0x82661144;
	sub_82466E20(ctx, base);
	// 82661144: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82661148: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8266114C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82661150: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82661158(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82661158 size=112
    let mut pc: u32 = 0x82661158;
    'dispatch: loop {
        match pc {
            0x82661158 => {
    //   block [0x82661158..0x826611C8)
	// 82661158: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8266115C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82661160: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82661164: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82661168: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 8266116C: 38AAAF60  addi r5, r10, -0x50a0
	ctx.r[5].s64 = ctx.r[10].s64 + -20640;
	// 82661170: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82661174: 390B4460  addi r8, r11, 0x4460
	ctx.r[8].s64 = ctx.r[11].s64 + 17504;
	// 82661178: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 8266117C: 388A331C  addi r4, r10, 0x331c
	ctx.r[4].s64 = ctx.r[10].s64 + 13084;
	// 82661180: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82661184: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82661188: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8266118C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82661190: 386AB320  addi r3, r10, -0x4ce0
	ctx.r[3].s64 = ctx.r[10].s64 + -19680;
	// 82661194: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82661198: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8266119C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826611A0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826611A4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826611A8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826611AC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826611B0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826611B4: 4BE05C6D  bl 0x82466e20
	ctx.lr = 0x826611B8;
	sub_82466E20(ctx, base);
	// 826611B8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826611BC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826611C0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826611C4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826611C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826611C8 size=116
    let mut pc: u32 = 0x826611C8;
    'dispatch: loop {
        match pc {
            0x826611C8 => {
    //   block [0x826611C8..0x8266123C)
	// 826611C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826611CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826611D0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826611D4: 3D408279  lis r10, -0x7d87
	ctx.r[10].s64 = -2105999360;
	// 826611D8: 38E00008  li r7, 8
	ctx.r[7].s64 = 8;
	// 826611DC: 390A4490  addi r8, r10, 0x4490
	ctx.r[8].s64 = ctx.r[10].s64 + 17552;
	// 826611E0: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 826611E4: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 826611E8: 38AAAF60  addi r5, r10, -0x50a0
	ctx.r[5].s64 = ctx.r[10].s64 + -20640;
	// 826611EC: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826611F0: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826611F4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826611F8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826611FC: 388A3340  addi r4, r10, 0x3340
	ctx.r[4].s64 = ctx.r[10].s64 + 13120;
	// 82661200: 396BEB1C  addi r11, r11, -0x14e4
	ctx.r[11].s64 = ctx.r[11].s64 + -5348;
	// 82661204: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82661208: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8266120C: 386AB350  addi r3, r10, -0x4cb0
	ctx.r[3].s64 = ctx.r[10].s64 + -19632;
	// 82661210: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 82661214: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82661218: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 8266121C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82661220: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82661224: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82661228: 4BE05BF9  bl 0x82466e20
	ctx.lr = 0x8266122C;
	sub_82466E20(ctx, base);
	// 8266122C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82661230: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82661234: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82661238: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82661240(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82661240 size=112
    let mut pc: u32 = 0x82661240;
    'dispatch: loop {
        match pc {
            0x82661240 => {
    //   block [0x82661240..0x826612B0)
	// 82661240: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82661244: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82661248: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8266124C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82661250: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 82661254: 38AAAF60  addi r5, r10, -0x50a0
	ctx.r[5].s64 = ctx.r[10].s64 + -20640;
	// 82661258: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8266125C: 390B4550  addi r8, r11, 0x4550
	ctx.r[8].s64 = ctx.r[11].s64 + 17744;
	// 82661260: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82661264: 388A335C  addi r4, r10, 0x335c
	ctx.r[4].s64 = ctx.r[10].s64 + 13148;
	// 82661268: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8266126C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82661270: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82661274: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82661278: 386AB380  addi r3, r10, -0x4c80
	ctx.r[3].s64 = ctx.r[10].s64 + -19584;
	// 8266127C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82661280: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82661284: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82661288: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8266128C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82661290: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82661294: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82661298: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8266129C: 4BE05B85  bl 0x82466e20
	ctx.lr = 0x826612A0;
	sub_82466E20(ctx, base);
	// 826612A0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826612A4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826612A8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826612AC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826612B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826612B0 size=108
    let mut pc: u32 = 0x826612B0;
    'dispatch: loop {
        match pc {
            0x826612B0 => {
    //   block [0x826612B0..0x8266131C)
	// 826612B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826612B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826612B8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826612BC: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 826612C0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826612C4: 38EB4568  addi r7, r11, 0x4568
	ctx.r[7].s64 = ctx.r[11].s64 + 17768;
	// 826612C8: 3900000D  li r8, 0xd
	ctx.r[8].s64 = 13;
	// 826612CC: 388A3374  addi r4, r10, 0x3374
	ctx.r[4].s64 = ctx.r[10].s64 + 13172;
	// 826612D0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826612D4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 826612D8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826612DC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826612E0: 386AB3B0  addi r3, r10, -0x4c50
	ctx.r[3].s64 = ctx.r[10].s64 + -19536;
	// 826612E4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826612E8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826612EC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826612F0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826612F4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826612F8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826612FC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82661300: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82661304: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82661308: 4BE05B19  bl 0x82466e20
	ctx.lr = 0x8266130C;
	sub_82466E20(ctx, base);
	// 8266130C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82661310: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82661314: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82661318: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82661320(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82661320 size=116
    let mut pc: u32 = 0x82661320;
    'dispatch: loop {
        match pc {
            0x82661320 => {
    //   block [0x82661320..0x82661394)
	// 82661320: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82661324: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82661328: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8266132C: 3D408279  lis r10, -0x7d87
	ctx.r[10].s64 = -2105999360;
	// 82661330: 38E00007  li r7, 7
	ctx.r[7].s64 = 7;
	// 82661334: 390A46A0  addi r8, r10, 0x46a0
	ctx.r[8].s64 = ctx.r[10].s64 + 18080;
	// 82661338: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8266133C: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 82661340: 38AAAF60  addi r5, r10, -0x50a0
	ctx.r[5].s64 = ctx.r[10].s64 + -20640;
	// 82661344: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82661348: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8266134C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82661350: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82661354: 388A3394  addi r4, r10, 0x3394
	ctx.r[4].s64 = ctx.r[10].s64 + 13204;
	// 82661358: 396BEB40  addi r11, r11, -0x14c0
	ctx.r[11].s64 = ctx.r[11].s64 + -5312;
	// 8266135C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82661360: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82661364: 386AB3E0  addi r3, r10, -0x4c20
	ctx.r[3].s64 = ctx.r[10].s64 + -19488;
	// 82661368: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 8266136C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82661370: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 82661374: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82661378: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8266137C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82661380: 4BE05AA1  bl 0x82466e20
	ctx.lr = 0x82661384;
	sub_82466E20(ctx, base);
	// 82661384: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82661388: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8266138C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82661390: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82661398(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82661398 size=112
    let mut pc: u32 = 0x82661398;
    'dispatch: loop {
        match pc {
            0x82661398 => {
    //   block [0x82661398..0x82661408)
	// 82661398: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8266139C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826613A0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826613A4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 826613A8: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 826613AC: 38AAAF60  addi r5, r10, -0x50a0
	ctx.r[5].s64 = ctx.r[10].s64 + -20640;
	// 826613B0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826613B4: 390B4748  addi r8, r11, 0x4748
	ctx.r[8].s64 = ctx.r[11].s64 + 18248;
	// 826613B8: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826613BC: 388A33B0  addi r4, r10, 0x33b0
	ctx.r[4].s64 = ctx.r[10].s64 + 13232;
	// 826613C0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826613C4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 826613C8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826613CC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826613D0: 386AB410  addi r3, r10, -0x4bf0
	ctx.r[3].s64 = ctx.r[10].s64 + -19440;
	// 826613D4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826613D8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826613DC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826613E0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826613E4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826613E8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826613EC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826613F0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826613F4: 4BE05A2D  bl 0x82466e20
	ctx.lr = 0x826613F8;
	sub_82466E20(ctx, base);
	// 826613F8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826613FC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82661400: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82661404: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82661408(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82661408 size=112
    let mut pc: u32 = 0x82661408;
    'dispatch: loop {
        match pc {
            0x82661408 => {
    //   block [0x82661408..0x82661478)
	// 82661408: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8266140C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82661410: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82661414: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82661418: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 8266141C: 38AAAF60  addi r5, r10, -0x50a0
	ctx.r[5].s64 = ctx.r[10].s64 + -20640;
	// 82661420: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82661424: 390B4760  addi r8, r11, 0x4760
	ctx.r[8].s64 = ctx.r[11].s64 + 18272;
	// 82661428: 3920000D  li r9, 0xd
	ctx.r[9].s64 = 13;
	// 8266142C: 388A33D4  addi r4, r10, 0x33d4
	ctx.r[4].s64 = ctx.r[10].s64 + 13268;
	// 82661430: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82661434: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82661438: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8266143C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82661440: 386AB440  addi r3, r10, -0x4bc0
	ctx.r[3].s64 = ctx.r[10].s64 + -19392;
	// 82661444: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82661448: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8266144C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82661450: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82661454: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82661458: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8266145C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82661460: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82661464: 4BE059BD  bl 0x82466e20
	ctx.lr = 0x82661468;
	sub_82466E20(ctx, base);
	// 82661468: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8266146C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82661470: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82661474: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82661478(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82661478 size=112
    let mut pc: u32 = 0x82661478;
    'dispatch: loop {
        match pc {
            0x82661478 => {
    //   block [0x82661478..0x826614E8)
	// 82661478: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8266147C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82661480: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82661484: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82661488: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 8266148C: 38AAAF60  addi r5, r10, -0x50a0
	ctx.r[5].s64 = ctx.r[10].s64 + -20640;
	// 82661490: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82661494: 390B4898  addi r8, r11, 0x4898
	ctx.r[8].s64 = ctx.r[11].s64 + 18584;
	// 82661498: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8266149C: 388A33E8  addi r4, r10, 0x33e8
	ctx.r[4].s64 = ctx.r[10].s64 + 13288;
	// 826614A0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826614A4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 826614A8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826614AC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826614B0: 386AB470  addi r3, r10, -0x4b90
	ctx.r[3].s64 = ctx.r[10].s64 + -19344;
	// 826614B4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826614B8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826614BC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826614C0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826614C4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826614C8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826614CC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826614D0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826614D4: 4BE0594D  bl 0x82466e20
	ctx.lr = 0x826614D8;
	sub_82466E20(ctx, base);
	// 826614D8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826614DC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826614E0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826614E4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826614E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826614E8 size=116
    let mut pc: u32 = 0x826614E8;
    'dispatch: loop {
        match pc {
            0x826614E8 => {
    //   block [0x826614E8..0x8266155C)
	// 826614E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826614EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826614F0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826614F4: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 826614F8: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 826614FC: 390B48B4  addi r8, r11, 0x48b4
	ctx.r[8].s64 = ctx.r[11].s64 + 18612;
	// 82661500: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82661504: 392AEB78  addi r9, r10, -0x1488
	ctx.r[9].s64 = ctx.r[10].s64 + -5256;
	// 82661508: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8266150C: 38E00002  li r7, 2
	ctx.r[7].s64 = 2;
	// 82661510: 38AABB60  addi r5, r10, -0x44a0
	ctx.r[5].s64 = ctx.r[10].s64 + -17568;
	// 82661514: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82661518: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8266151C: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82661520: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82661524: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82661528: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8266152C: 3D608296  lis r11, -0x7d6a
	ctx.r[11].s64 = -2104098816;
	// 82661530: 388A33FC  addi r4, r10, 0x33fc
	ctx.r[4].s64 = ctx.r[10].s64 + 13308;
	// 82661534: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82661538: 386BB4A0  addi r3, r11, -0x4b60
	ctx.r[3].s64 = ctx.r[11].s64 + -19296;
	// 8266153C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82661540: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82661544: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82661548: 4BE058D9  bl 0x82466e20
	ctx.lr = 0x8266154C;
	sub_82466E20(ctx, base);
	// 8266154C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82661550: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82661554: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82661558: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82661560(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82661560 size=100
    let mut pc: u32 = 0x82661560;
    'dispatch: loop {
        match pc {
            0x82661560 => {
    //   block [0x82661560..0x826615C4)
	// 82661560: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82661564: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82661568: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8266156C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82661570: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82661574: 38AABB60  addi r5, r10, -0x44a0
	ctx.r[5].s64 = ctx.r[10].s64 + -17568;
	// 82661578: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8266157C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82661580: 388A3404  addi r4, r10, 0x3404
	ctx.r[4].s64 = ctx.r[10].s64 + 13316;
	// 82661584: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82661588: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8266158C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82661590: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82661594: 386AB4D0  addi r3, r10, -0x4b30
	ctx.r[3].s64 = ctx.r[10].s64 + -19248;
	// 82661598: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8266159C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826615A0: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826615A4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826615A8: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826615AC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826615B0: 4BE05871  bl 0x82466e20
	ctx.lr = 0x826615B4;
	sub_82466E20(ctx, base);
	// 826615B4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826615B8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826615BC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826615C0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826615C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826615C8 size=112
    let mut pc: u32 = 0x826615C8;
    'dispatch: loop {
        match pc {
            0x826615C8 => {
    //   block [0x826615C8..0x82661638)
	// 826615C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826615CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826615D0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826615D4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 826615D8: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 826615DC: 38AAB4D0  addi r5, r10, -0x4b30
	ctx.r[5].s64 = ctx.r[10].s64 + -19248;
	// 826615E0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826615E4: 390B48E4  addi r8, r11, 0x48e4
	ctx.r[8].s64 = ctx.r[11].s64 + 18660;
	// 826615E8: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826615EC: 388A3414  addi r4, r10, 0x3414
	ctx.r[4].s64 = ctx.r[10].s64 + 13332;
	// 826615F0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826615F4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 826615F8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826615FC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82661600: 386AB500  addi r3, r10, -0x4b00
	ctx.r[3].s64 = ctx.r[10].s64 + -19200;
	// 82661604: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82661608: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8266160C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82661610: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82661614: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82661618: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8266161C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82661620: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82661624: 4BE057FD  bl 0x82466e20
	ctx.lr = 0x82661628;
	sub_82466E20(ctx, base);
	// 82661628: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8266162C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82661630: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82661634: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82661638(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82661638 size=112
    let mut pc: u32 = 0x82661638;
    'dispatch: loop {
        match pc {
            0x82661638 => {
    //   block [0x82661638..0x826616A8)
	// 82661638: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8266163C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82661640: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82661644: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82661648: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 8266164C: 38AABB60  addi r5, r10, -0x44a0
	ctx.r[5].s64 = ctx.r[10].s64 + -17568;
	// 82661650: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82661654: 390B4900  addi r8, r11, 0x4900
	ctx.r[8].s64 = ctx.r[11].s64 + 18688;
	// 82661658: 39200007  li r9, 7
	ctx.r[9].s64 = 7;
	// 8266165C: 388A3428  addi r4, r10, 0x3428
	ctx.r[4].s64 = ctx.r[10].s64 + 13352;
	// 82661660: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82661664: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82661668: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8266166C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82661670: 386AB530  addi r3, r10, -0x4ad0
	ctx.r[3].s64 = ctx.r[10].s64 + -19152;
	// 82661674: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82661678: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8266167C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82661680: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82661684: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82661688: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8266168C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82661690: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82661694: 4BE0578D  bl 0x82466e20
	ctx.lr = 0x82661698;
	sub_82466E20(ctx, base);
	// 82661698: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8266169C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826616A0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826616A4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826616A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826616A8 size=112
    let mut pc: u32 = 0x826616A8;
    'dispatch: loop {
        match pc {
            0x826616A8 => {
    //   block [0x826616A8..0x82661718)
	// 826616A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826616AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826616B0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826616B4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 826616B8: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 826616BC: 38AABB60  addi r5, r10, -0x44a0
	ctx.r[5].s64 = ctx.r[10].s64 + -17568;
	// 826616C0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826616C4: 390B49A8  addi r8, r11, 0x49a8
	ctx.r[8].s64 = ctx.r[11].s64 + 18856;
	// 826616C8: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 826616CC: 388A3440  addi r4, r10, 0x3440
	ctx.r[4].s64 = ctx.r[10].s64 + 13376;
	// 826616D0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826616D4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 826616D8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826616DC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826616E0: 386AB560  addi r3, r10, -0x4aa0
	ctx.r[3].s64 = ctx.r[10].s64 + -19104;
	// 826616E4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826616E8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826616EC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826616F0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826616F4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826616F8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826616FC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82661700: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82661704: 4BE0571D  bl 0x82466e20
	ctx.lr = 0x82661708;
	sub_82466E20(ctx, base);
	// 82661708: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8266170C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82661710: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82661714: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82661718(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82661718 size=112
    let mut pc: u32 = 0x82661718;
    'dispatch: loop {
        match pc {
            0x82661718 => {
    //   block [0x82661718..0x82661788)
	// 82661718: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8266171C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82661720: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82661724: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82661728: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 8266172C: 38AABB60  addi r5, r10, -0x44a0
	ctx.r[5].s64 = ctx.r[10].s64 + -17568;
	// 82661730: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82661734: 390B49F0  addi r8, r11, 0x49f0
	ctx.r[8].s64 = ctx.r[11].s64 + 18928;
	// 82661738: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 8266173C: 388A3450  addi r4, r10, 0x3450
	ctx.r[4].s64 = ctx.r[10].s64 + 13392;
	// 82661740: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82661744: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82661748: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8266174C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82661750: 386AB590  addi r3, r10, -0x4a70
	ctx.r[3].s64 = ctx.r[10].s64 + -19056;
	// 82661754: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82661758: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8266175C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82661760: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82661764: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82661768: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8266176C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82661770: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82661774: 4BE056AD  bl 0x82466e20
	ctx.lr = 0x82661778;
	sub_82466E20(ctx, base);
	// 82661778: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8266177C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82661780: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82661784: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82661788(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82661788 size=116
    let mut pc: u32 = 0x82661788;
    'dispatch: loop {
        match pc {
            0x82661788 => {
    //   block [0x82661788..0x826617FC)
	// 82661788: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8266178C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82661790: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82661794: 3D408279  lis r10, -0x7d87
	ctx.r[10].s64 = -2105999360;
	// 82661798: 38E00008  li r7, 8
	ctx.r[7].s64 = 8;
	// 8266179C: 390A4A20  addi r8, r10, 0x4a20
	ctx.r[8].s64 = ctx.r[10].s64 + 18976;
	// 826617A0: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 826617A4: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 826617A8: 38AAAF60  addi r5, r10, -0x50a0
	ctx.r[5].s64 = ctx.r[10].s64 + -20640;
	// 826617AC: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826617B0: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826617B4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826617B8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826617BC: 388A3468  addi r4, r10, 0x3468
	ctx.r[4].s64 = ctx.r[10].s64 + 13416;
	// 826617C0: 396BEB8C  addi r11, r11, -0x1474
	ctx.r[11].s64 = ctx.r[11].s64 + -5236;
	// 826617C4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 826617C8: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826617CC: 386AB5C0  addi r3, r10, -0x4a40
	ctx.r[3].s64 = ctx.r[10].s64 + -19008;
	// 826617D0: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 826617D4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826617D8: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 826617DC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826617E0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826617E4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826617E8: 4BE05639  bl 0x82466e20
	ctx.lr = 0x826617EC;
	sub_82466E20(ctx, base);
	// 826617EC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826617F0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826617F4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826617F8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82661800(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82661800 size=100
    let mut pc: u32 = 0x82661800;
    'dispatch: loop {
        match pc {
            0x82661800 => {
    //   block [0x82661800..0x82661864)
	// 82661800: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82661804: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82661808: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8266180C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82661810: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82661814: 38AABB60  addi r5, r10, -0x44a0
	ctx.r[5].s64 = ctx.r[10].s64 + -17568;
	// 82661818: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8266181C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82661820: 388A3474  addi r4, r10, 0x3474
	ctx.r[4].s64 = ctx.r[10].s64 + 13428;
	// 82661824: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82661828: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8266182C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82661830: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82661834: 386AB5F0  addi r3, r10, -0x4a10
	ctx.r[3].s64 = ctx.r[10].s64 + -18960;
	// 82661838: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8266183C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82661840: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82661844: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82661848: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8266184C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82661850: 4BE055D1  bl 0x82466e20
	ctx.lr = 0x82661854;
	sub_82466E20(ctx, base);
	// 82661854: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82661858: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8266185C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82661860: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82661868(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82661868 size=108
    let mut pc: u32 = 0x82661868;
    'dispatch: loop {
        match pc {
            0x82661868 => {
    //   block [0x82661868..0x826618D4)
	// 82661868: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8266186C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82661870: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82661874: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 82661878: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8266187C: 38EB4AE0  addi r7, r11, 0x4ae0
	ctx.r[7].s64 = ctx.r[11].s64 + 19168;
	// 82661880: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 82661884: 388A3488  addi r4, r10, 0x3488
	ctx.r[4].s64 = ctx.r[10].s64 + 13448;
	// 82661888: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8266188C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82661890: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82661894: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82661898: 386AB620  addi r3, r10, -0x49e0
	ctx.r[3].s64 = ctx.r[10].s64 + -18912;
	// 8266189C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826618A0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826618A4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826618A8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826618AC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826618B0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826618B4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826618B8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826618BC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826618C0: 4BE05561  bl 0x82466e20
	ctx.lr = 0x826618C4;
	sub_82466E20(ctx, base);
	// 826618C4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826618C8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826618CC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826618D0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826618D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826618D8 size=112
    let mut pc: u32 = 0x826618D8;
    'dispatch: loop {
        match pc {
            0x826618D8 => {
    //   block [0x826618D8..0x82661948)
	// 826618D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826618DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826618E0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826618E4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 826618E8: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 826618EC: 38AAB5F0  addi r5, r10, -0x4a10
	ctx.r[5].s64 = ctx.r[10].s64 + -18960;
	// 826618F0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826618F4: 390B4B10  addi r8, r11, 0x4b10
	ctx.r[8].s64 = ctx.r[11].s64 + 19216;
	// 826618F8: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826618FC: 388A34AC  addi r4, r10, 0x34ac
	ctx.r[4].s64 = ctx.r[10].s64 + 13484;
	// 82661900: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82661904: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82661908: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8266190C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82661910: 386AB650  addi r3, r10, -0x49b0
	ctx.r[3].s64 = ctx.r[10].s64 + -18864;
	// 82661914: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82661918: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8266191C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82661920: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82661924: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82661928: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8266192C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82661930: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82661934: 4BE054ED  bl 0x82466e20
	ctx.lr = 0x82661938;
	sub_82466E20(ctx, base);
	// 82661938: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8266193C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82661940: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82661944: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82661948(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82661948 size=108
    let mut pc: u32 = 0x82661948;
    'dispatch: loop {
        match pc {
            0x82661948 => {
    //   block [0x82661948..0x826619B4)
	// 82661948: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8266194C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82661950: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82661954: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 82661958: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8266195C: 38EB4B40  addi r7, r11, 0x4b40
	ctx.r[7].s64 = ctx.r[11].s64 + 19264;
	// 82661960: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 82661964: 388A34C4  addi r4, r10, 0x34c4
	ctx.r[4].s64 = ctx.r[10].s64 + 13508;
	// 82661968: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8266196C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82661970: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82661974: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82661978: 386AB680  addi r3, r10, -0x4980
	ctx.r[3].s64 = ctx.r[10].s64 + -18816;
	// 8266197C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82661980: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82661984: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82661988: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8266198C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82661990: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82661994: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82661998: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8266199C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826619A0: 4BE05481  bl 0x82466e20
	ctx.lr = 0x826619A4;
	sub_82466E20(ctx, base);
	// 826619A4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826619A8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826619AC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826619B0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826619B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826619B8 size=112
    let mut pc: u32 = 0x826619B8;
    'dispatch: loop {
        match pc {
            0x826619B8 => {
    //   block [0x826619B8..0x82661A28)
	// 826619B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826619BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826619C0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826619C4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 826619C8: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 826619CC: 38AAB5F0  addi r5, r10, -0x4a10
	ctx.r[5].s64 = ctx.r[10].s64 + -18960;
	// 826619D0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826619D4: 390B4B70  addi r8, r11, 0x4b70
	ctx.r[8].s64 = ctx.r[11].s64 + 19312;
	// 826619D8: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 826619DC: 388A34F4  addi r4, r10, 0x34f4
	ctx.r[4].s64 = ctx.r[10].s64 + 13556;
	// 826619E0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826619E4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 826619E8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826619EC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826619F0: 386AB6B0  addi r3, r10, -0x4950
	ctx.r[3].s64 = ctx.r[10].s64 + -18768;
	// 826619F4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826619F8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826619FC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82661A00: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82661A04: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82661A08: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82661A0C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82661A10: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82661A14: 4BE0540D  bl 0x82466e20
	ctx.lr = 0x82661A18;
	sub_82466E20(ctx, base);
	// 82661A18: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82661A1C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82661A20: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82661A24: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82661A28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82661A28 size=108
    let mut pc: u32 = 0x82661A28;
    'dispatch: loop {
        match pc {
            0x82661A28 => {
    //   block [0x82661A28..0x82661A94)
	// 82661A28: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82661A2C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82661A30: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82661A34: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 82661A38: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82661A3C: 38EB4BB8  addi r7, r11, 0x4bb8
	ctx.r[7].s64 = ctx.r[11].s64 + 19384;
	// 82661A40: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 82661A44: 388A3514  addi r4, r10, 0x3514
	ctx.r[4].s64 = ctx.r[10].s64 + 13588;
	// 82661A48: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82661A4C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82661A50: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82661A54: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82661A58: 386AB6E0  addi r3, r10, -0x4920
	ctx.r[3].s64 = ctx.r[10].s64 + -18720;
	// 82661A5C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82661A60: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82661A64: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82661A68: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82661A6C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82661A70: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82661A74: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82661A78: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82661A7C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82661A80: 4BE053A1  bl 0x82466e20
	ctx.lr = 0x82661A84;
	sub_82466E20(ctx, base);
	// 82661A84: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82661A88: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82661A8C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82661A90: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82661A98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82661A98 size=112
    let mut pc: u32 = 0x82661A98;
    'dispatch: loop {
        match pc {
            0x82661A98 => {
    //   block [0x82661A98..0x82661B08)
	// 82661A98: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82661A9C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82661AA0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82661AA4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82661AA8: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 82661AAC: 38AAB5F0  addi r5, r10, -0x4a10
	ctx.r[5].s64 = ctx.r[10].s64 + -18960;
	// 82661AB0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82661AB4: 390B4BE8  addi r8, r11, 0x4be8
	ctx.r[8].s64 = ctx.r[11].s64 + 19432;
	// 82661AB8: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 82661ABC: 388A3544  addi r4, r10, 0x3544
	ctx.r[4].s64 = ctx.r[10].s64 + 13636;
	// 82661AC0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82661AC4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82661AC8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82661ACC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82661AD0: 386AB710  addi r3, r10, -0x48f0
	ctx.r[3].s64 = ctx.r[10].s64 + -18672;
	// 82661AD4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82661AD8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82661ADC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82661AE0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82661AE4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82661AE8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82661AEC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82661AF0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82661AF4: 4BE0532D  bl 0x82466e20
	ctx.lr = 0x82661AF8;
	sub_82466E20(ctx, base);
	// 82661AF8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82661AFC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82661B00: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82661B04: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82661B08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82661B08 size=108
    let mut pc: u32 = 0x82661B08;
    'dispatch: loop {
        match pc {
            0x82661B08 => {
    //   block [0x82661B08..0x82661B74)
	// 82661B08: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82661B0C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82661B10: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82661B14: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 82661B18: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82661B1C: 38EB4C30  addi r7, r11, 0x4c30
	ctx.r[7].s64 = ctx.r[11].s64 + 19504;
	// 82661B20: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 82661B24: 388A3564  addi r4, r10, 0x3564
	ctx.r[4].s64 = ctx.r[10].s64 + 13668;
	// 82661B28: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82661B2C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82661B30: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82661B34: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82661B38: 386AB740  addi r3, r10, -0x48c0
	ctx.r[3].s64 = ctx.r[10].s64 + -18624;
	// 82661B3C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82661B40: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82661B44: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82661B48: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82661B4C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82661B50: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82661B54: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82661B58: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82661B5C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82661B60: 4BE052C1  bl 0x82466e20
	ctx.lr = 0x82661B64;
	sub_82466E20(ctx, base);
	// 82661B64: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82661B68: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82661B6C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82661B70: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82661B78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82661B78 size=112
    let mut pc: u32 = 0x82661B78;
    'dispatch: loop {
        match pc {
            0x82661B78 => {
    //   block [0x82661B78..0x82661BE8)
	// 82661B78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82661B7C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82661B80: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82661B84: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82661B88: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 82661B8C: 38AAB5F0  addi r5, r10, -0x4a10
	ctx.r[5].s64 = ctx.r[10].s64 + -18960;
	// 82661B90: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82661B94: 390B4C60  addi r8, r11, 0x4c60
	ctx.r[8].s64 = ctx.r[11].s64 + 19552;
	// 82661B98: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 82661B9C: 388A3594  addi r4, r10, 0x3594
	ctx.r[4].s64 = ctx.r[10].s64 + 13716;
	// 82661BA0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82661BA4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82661BA8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82661BAC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82661BB0: 386AB770  addi r3, r10, -0x4890
	ctx.r[3].s64 = ctx.r[10].s64 + -18576;
	// 82661BB4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82661BB8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82661BBC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82661BC0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82661BC4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82661BC8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82661BCC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82661BD0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82661BD4: 4BE0524D  bl 0x82466e20
	ctx.lr = 0x82661BD8;
	sub_82466E20(ctx, base);
	// 82661BD8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82661BDC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82661BE0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82661BE4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82661BE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82661BE8 size=108
    let mut pc: u32 = 0x82661BE8;
    'dispatch: loop {
        match pc {
            0x82661BE8 => {
    //   block [0x82661BE8..0x82661C54)
	// 82661BE8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82661BEC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82661BF0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82661BF4: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 82661BF8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82661BFC: 38EB4CA8  addi r7, r11, 0x4ca8
	ctx.r[7].s64 = ctx.r[11].s64 + 19624;
	// 82661C00: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 82661C04: 388A35B0  addi r4, r10, 0x35b0
	ctx.r[4].s64 = ctx.r[10].s64 + 13744;
	// 82661C08: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82661C0C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82661C10: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82661C14: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82661C18: 386AB7A0  addi r3, r10, -0x4860
	ctx.r[3].s64 = ctx.r[10].s64 + -18528;
	// 82661C1C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82661C20: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82661C24: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82661C28: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82661C2C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82661C30: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82661C34: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82661C38: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82661C3C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82661C40: 4BE051E1  bl 0x82466e20
	ctx.lr = 0x82661C44;
	sub_82466E20(ctx, base);
	// 82661C44: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82661C48: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82661C4C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82661C50: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82661C58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82661C58 size=112
    let mut pc: u32 = 0x82661C58;
    'dispatch: loop {
        match pc {
            0x82661C58 => {
    //   block [0x82661C58..0x82661CC8)
	// 82661C58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82661C5C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82661C60: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82661C64: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 82661C68: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 82661C6C: 392AEC48  addi r9, r10, -0x13b8
	ctx.r[9].s64 = ctx.r[10].s64 + -5048;
	// 82661C70: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82661C74: 390B4D10  addi r8, r11, 0x4d10
	ctx.r[8].s64 = ctx.r[11].s64 + 19728;
	// 82661C78: 38E00008  li r7, 8
	ctx.r[7].s64 = 8;
	// 82661C7C: 388A35CC  addi r4, r10, 0x35cc
	ctx.r[4].s64 = ctx.r[10].s64 + 13772;
	// 82661C80: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82661C84: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82661C88: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82661C8C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82661C90: 386AB7D0  addi r3, r10, -0x4830
	ctx.r[3].s64 = ctx.r[10].s64 + -18480;
	// 82661C94: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82661C98: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82661C9C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82661CA0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82661CA4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82661CA8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82661CAC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82661CB0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82661CB4: 4BE0516D  bl 0x82466e20
	ctx.lr = 0x82661CB8;
	sub_82466E20(ctx, base);
	// 82661CB8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82661CBC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82661CC0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82661CC4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82661CC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82661CC8 size=108
    let mut pc: u32 = 0x82661CC8;
    'dispatch: loop {
        match pc {
            0x82661CC8 => {
    //   block [0x82661CC8..0x82661D34)
	// 82661CC8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82661CCC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82661CD0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82661CD4: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 82661CD8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82661CDC: 38EB4DD0  addi r7, r11, 0x4dd0
	ctx.r[7].s64 = ctx.r[11].s64 + 19920;
	// 82661CE0: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 82661CE4: 388A35EC  addi r4, r10, 0x35ec
	ctx.r[4].s64 = ctx.r[10].s64 + 13804;
	// 82661CE8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82661CEC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82661CF0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82661CF4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82661CF8: 386AB800  addi r3, r10, -0x4800
	ctx.r[3].s64 = ctx.r[10].s64 + -18432;
	// 82661CFC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82661D00: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82661D04: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82661D08: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82661D0C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82661D10: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82661D14: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82661D18: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82661D1C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82661D20: 4BE05101  bl 0x82466e20
	ctx.lr = 0x82661D24;
	sub_82466E20(ctx, base);
	// 82661D24: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82661D28: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82661D2C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82661D30: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82661D38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82661D38 size=108
    let mut pc: u32 = 0x82661D38;
    'dispatch: loop {
        match pc {
            0x82661D38 => {
    //   block [0x82661D38..0x82661DA4)
	// 82661D38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82661D3C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82661D40: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82661D44: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 82661D48: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82661D4C: 38EB4E30  addi r7, r11, 0x4e30
	ctx.r[7].s64 = ctx.r[11].s64 + 20016;
	// 82661D50: 39000007  li r8, 7
	ctx.r[8].s64 = 7;
	// 82661D54: 388A3610  addi r4, r10, 0x3610
	ctx.r[4].s64 = ctx.r[10].s64 + 13840;
	// 82661D58: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82661D5C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82661D60: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82661D64: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82661D68: 386AB830  addi r3, r10, -0x47d0
	ctx.r[3].s64 = ctx.r[10].s64 + -18384;
	// 82661D6C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82661D70: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82661D74: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82661D78: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82661D7C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82661D80: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82661D84: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82661D88: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82661D8C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82661D90: 4BE05091  bl 0x82466e20
	ctx.lr = 0x82661D94;
	sub_82466E20(ctx, base);
	// 82661D94: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82661D98: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82661D9C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82661DA0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82661DA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82661DA8 size=24
    let mut pc: u32 = 0x82661DA8;
    'dispatch: loop {
        match pc {
            0x82661DA8 => {
    //   block [0x82661DA8..0x82661DC0)
	// 82661DA8: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 82661DAC: 3D40827A  lis r10, -0x7d86
	ctx.r[10].s64 = -2105933824;
	// 82661DB0: 394AB8E8  addi r10, r10, -0x4718
	ctx.r[10].s64 = ctx.r[10].s64 + -18200;
	// 82661DB4: 816B48FC  lwz r11, 0x48fc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(18684 as u32) ) } as u64;
	// 82661DB8: 916A0098  stw r11, 0x98(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(152 as u32), ctx.r[11].u32 ) };
	// 82661DBC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82661DC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82661DC0 size=116
    let mut pc: u32 = 0x82661DC0;
    'dispatch: loop {
        match pc {
            0x82661DC0 => {
    //   block [0x82661DC0..0x82661E34)
	// 82661DC0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82661DC4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82661DC8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82661DCC: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 82661DD0: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82661DD4: 392BEBD4  addi r9, r11, -0x142c
	ctx.r[9].s64 = ctx.r[11].s64 + -5164;
	// 82661DD8: 38AAACF0  addi r5, r10, -0x5310
	ctx.r[5].s64 = ctx.r[10].s64 + -21264;
	// 82661DDC: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82661DE0: 38E9008C  addi r7, r9, 0x8c
	ctx.r[7].s64 = ctx.r[9].s64 + 140;
	// 82661DE4: 38C00013  li r6, 0x13
	ctx.r[6].s64 = 19;
	// 82661DE8: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 82661DEC: 388A362C  addi r4, r10, 0x362c
	ctx.r[4].s64 = ctx.r[10].s64 + 13868;
	// 82661DF0: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82661DF4: 396BB8E8  addi r11, r11, -0x4718
	ctx.r[11].s64 = ctx.r[11].s64 + -18200;
	// 82661DF8: 90E10064  stw r7, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[7].u32 ) };
	// 82661DFC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82661E00: 90C1005C  stw r6, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[6].u32 ) };
	// 82661E04: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82661E08: 386AB860  addi r3, r10, -0x47a0
	ctx.r[3].s64 = ctx.r[10].s64 + -18336;
	// 82661E0C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82661E10: 91010074  stw r8, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[8].u32 ) };
	// 82661E14: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82661E18: 9101006C  stw r8, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[8].u32 ) };
	// 82661E1C: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82661E20: 4BE05001  bl 0x82466e20
	ctx.lr = 0x82661E24;
	sub_82466E20(ctx, base);
	// 82661E24: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82661E28: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82661E2C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82661E30: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82661E38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82661E38 size=100
    let mut pc: u32 = 0x82661E38;
    'dispatch: loop {
        match pc {
            0x82661E38 => {
    //   block [0x82661E38..0x82661E9C)
	// 82661E38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82661E3C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82661E40: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82661E44: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82661E48: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82661E4C: 38AAACF0  addi r5, r10, -0x5310
	ctx.r[5].s64 = ctx.r[10].s64 + -21264;
	// 82661E50: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82661E54: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82661E58: 388A363C  addi r4, r10, 0x363c
	ctx.r[4].s64 = ctx.r[10].s64 + 13884;
	// 82661E5C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82661E60: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82661E64: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82661E68: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82661E6C: 386AB890  addi r3, r10, -0x4770
	ctx.r[3].s64 = ctx.r[10].s64 + -18288;
	// 82661E70: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82661E74: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82661E78: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82661E7C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82661E80: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82661E84: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82661E88: 4BE04F99  bl 0x82466e20
	ctx.lr = 0x82661E8C;
	sub_82466E20(ctx, base);
	// 82661E8C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82661E90: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82661E94: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82661E98: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82661EA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82661EA0 size=24
    let mut pc: u32 = 0x82661EA0;
    'dispatch: loop {
        match pc {
            0x82661EA0 => {
    //   block [0x82661EA0..0x82661EB8)
	// 82661EA0: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 82661EA4: 3D40827A  lis r10, -0x7d86
	ctx.r[10].s64 = -2105933824;
	// 82661EA8: 394ABAB0  addi r10, r10, -0x4550
	ctx.r[10].s64 = ctx.r[10].s64 + -17744;
	// 82661EAC: 816B4ED8  lwz r11, 0x4ed8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20184 as u32) ) } as u64;
	// 82661EB0: 916A0050  stw r11, 0x50(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 82661EB4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82661EB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82661EB8 size=116
    let mut pc: u32 = 0x82661EB8;
    'dispatch: loop {
        match pc {
            0x82661EB8 => {
    //   block [0x82661EB8..0x82661F2C)
	// 82661EB8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82661EBC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82661EC0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82661EC4: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 82661EC8: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 82661ECC: 390BBAB0  addi r8, r11, -0x4550
	ctx.r[8].s64 = ctx.r[11].s64 + -17744;
	// 82661ED0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82661ED4: 392AECE4  addi r9, r10, -0x131c
	ctx.r[9].s64 = ctx.r[10].s64 + -4892;
	// 82661ED8: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82661EDC: 38E0000C  li r7, 0xc
	ctx.r[7].s64 = 12;
	// 82661EE0: 38AAB890  addi r5, r10, -0x4770
	ctx.r[5].s64 = ctx.r[10].s64 + -18288;
	// 82661EE4: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82661EE8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82661EEC: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82661EF0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82661EF4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82661EF8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82661EFC: 3D608296  lis r11, -0x7d6a
	ctx.r[11].s64 = -2104098816;
	// 82661F00: 388A3690  addi r4, r10, 0x3690
	ctx.r[4].s64 = ctx.r[10].s64 + 13968;
	// 82661F04: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82661F08: 386BB8C0  addi r3, r11, -0x4740
	ctx.r[3].s64 = ctx.r[11].s64 + -18240;
	// 82661F0C: 39400002  li r10, 2
	ctx.r[10].s64 = 2;
	// 82661F10: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82661F14: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82661F18: 4BE04F09  bl 0x82466e20
	ctx.lr = 0x82661F1C;
	sub_82466E20(ctx, base);
	// 82661F1C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82661F20: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82661F24: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82661F28: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82661F30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82661F30 size=112
    let mut pc: u32 = 0x82661F30;
    'dispatch: loop {
        match pc {
            0x82661F30 => {
    //   block [0x82661F30..0x82661FA0)
	// 82661F30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82661F34: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82661F38: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82661F3C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82661F40: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 82661F44: 38AAB890  addi r5, r10, -0x4770
	ctx.r[5].s64 = ctx.r[10].s64 + -18288;
	// 82661F48: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82661F4C: 390B4EE0  addi r8, r11, 0x4ee0
	ctx.r[8].s64 = ctx.r[11].s64 + 20192;
	// 82661F50: 39200009  li r9, 9
	ctx.r[9].s64 = 9;
	// 82661F54: 388AAAE8  addi r4, r10, -0x5518
	ctx.r[4].s64 = ctx.r[10].s64 + -21784;
	// 82661F58: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82661F5C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82661F60: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82661F64: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82661F68: 386AB8F0  addi r3, r10, -0x4710
	ctx.r[3].s64 = ctx.r[10].s64 + -18192;
	// 82661F6C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82661F70: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82661F74: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82661F78: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82661F7C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82661F80: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82661F84: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82661F88: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82661F8C: 4BE04E95  bl 0x82466e20
	ctx.lr = 0x82661F90;
	sub_82466E20(ctx, base);
	// 82661F90: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82661F94: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82661F98: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82661F9C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82661FA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82661FA0 size=112
    let mut pc: u32 = 0x82661FA0;
    'dispatch: loop {
        match pc {
            0x82661FA0 => {
    //   block [0x82661FA0..0x82662010)
	// 82661FA0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82661FA4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82661FA8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82661FAC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82661FB0: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 82661FB4: 38AABB60  addi r5, r10, -0x44a0
	ctx.r[5].s64 = ctx.r[10].s64 + -17568;
	// 82661FB8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82661FBC: 390B4FB8  addi r8, r11, 0x4fb8
	ctx.r[8].s64 = ctx.r[11].s64 + 20408;
	// 82661FC0: 39200009  li r9, 9
	ctx.r[9].s64 = 9;
	// 82661FC4: 388A36AC  addi r4, r10, 0x36ac
	ctx.r[4].s64 = ctx.r[10].s64 + 13996;
	// 82661FC8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82661FCC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82661FD0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82661FD4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82661FD8: 386AB920  addi r3, r10, -0x46e0
	ctx.r[3].s64 = ctx.r[10].s64 + -18144;
	// 82661FDC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82661FE0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82661FE4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82661FE8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82661FEC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82661FF0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82661FF4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82661FF8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82661FFC: 4BE04E25  bl 0x82466e20
	ctx.lr = 0x82662000;
	sub_82466E20(ctx, base);
	// 82662000: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82662004: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82662008: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8266200C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82662010(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82662010 size=108
    let mut pc: u32 = 0x82662010;
    'dispatch: loop {
        match pc {
            0x82662010 => {
    //   block [0x82662010..0x8266207C)
	// 82662010: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82662014: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82662018: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8266201C: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 82662020: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82662024: 38EB5090  addi r7, r11, 0x5090
	ctx.r[7].s64 = ctx.r[11].s64 + 20624;
	// 82662028: 39000005  li r8, 5
	ctx.r[8].s64 = 5;
	// 8266202C: 388A36C8  addi r4, r10, 0x36c8
	ctx.r[4].s64 = ctx.r[10].s64 + 14024;
	// 82662030: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82662034: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82662038: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8266203C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82662040: 386AB950  addi r3, r10, -0x46b0
	ctx.r[3].s64 = ctx.r[10].s64 + -18096;
	// 82662044: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82662048: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8266204C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82662050: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82662054: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82662058: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8266205C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82662060: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82662064: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82662068: 4BE04DB9  bl 0x82466e20
	ctx.lr = 0x8266206C;
	sub_82466E20(ctx, base);
	// 8266206C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82662070: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82662074: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82662078: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82662080(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82662080 size=108
    let mut pc: u32 = 0x82662080;
    'dispatch: loop {
        match pc {
            0x82662080 => {
    //   block [0x82662080..0x826620EC)
	// 82662080: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82662084: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82662088: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8266208C: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 82662090: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82662094: 38EB5108  addi r7, r11, 0x5108
	ctx.r[7].s64 = ctx.r[11].s64 + 20744;
	// 82662098: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 8266209C: 388A36E4  addi r4, r10, 0x36e4
	ctx.r[4].s64 = ctx.r[10].s64 + 14052;
	// 826620A0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826620A4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 826620A8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826620AC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826620B0: 386AB980  addi r3, r10, -0x4680
	ctx.r[3].s64 = ctx.r[10].s64 + -18048;
	// 826620B4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826620B8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826620BC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826620C0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826620C4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826620C8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826620CC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826620D0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826620D4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826620D8: 4BE04D49  bl 0x82466e20
	ctx.lr = 0x826620DC;
	sub_82466E20(ctx, base);
	// 826620DC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826620E0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826620E4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826620E8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826620F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826620F0 size=112
    let mut pc: u32 = 0x826620F0;
    'dispatch: loop {
        match pc {
            0x826620F0 => {
    //   block [0x826620F0..0x82662160)
	// 826620F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826620F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826620F8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826620FC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82662100: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 82662104: 38AABB60  addi r5, r10, -0x44a0
	ctx.r[5].s64 = ctx.r[10].s64 + -17568;
	// 82662108: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8266210C: 390B5150  addi r8, r11, 0x5150
	ctx.r[8].s64 = ctx.r[11].s64 + 20816;
	// 82662110: 39200014  li r9, 0x14
	ctx.r[9].s64 = 20;
	// 82662114: 388A3704  addi r4, r10, 0x3704
	ctx.r[4].s64 = ctx.r[10].s64 + 14084;
	// 82662118: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8266211C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82662120: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82662124: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82662128: 386AB9B0  addi r3, r10, -0x4650
	ctx.r[3].s64 = ctx.r[10].s64 + -18000;
	// 8266212C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82662130: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82662134: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82662138: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8266213C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82662140: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82662144: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82662148: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8266214C: 4BE04CD5  bl 0x82466e20
	ctx.lr = 0x82662150;
	sub_82466E20(ctx, base);
	// 82662150: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82662154: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82662158: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8266215C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82662160(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82662160 size=108
    let mut pc: u32 = 0x82662160;
    'dispatch: loop {
        match pc {
            0x82662160 => {
    //   block [0x82662160..0x826621CC)
	// 82662160: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82662164: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82662168: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8266216C: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 82662170: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82662174: 38EB5330  addi r7, r11, 0x5330
	ctx.r[7].s64 = ctx.r[11].s64 + 21296;
	// 82662178: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 8266217C: 388A3714  addi r4, r10, 0x3714
	ctx.r[4].s64 = ctx.r[10].s64 + 14100;
	// 82662180: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82662184: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82662188: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8266218C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82662190: 386AB9E0  addi r3, r10, -0x4620
	ctx.r[3].s64 = ctx.r[10].s64 + -17952;
	// 82662194: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82662198: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8266219C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826621A0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826621A4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826621A8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826621AC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826621B0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826621B4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826621B8: 4BE04C69  bl 0x82466e20
	ctx.lr = 0x826621BC;
	sub_82466E20(ctx, base);
	// 826621BC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826621C0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826621C4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826621C8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826621D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826621D0 size=24
    let mut pc: u32 = 0x826621D0;
    'dispatch: loop {
        match pc {
            0x826621D0 => {
    //   block [0x826621D0..0x826621E8)
	// 826621D0: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 826621D4: 3D40827A  lis r10, -0x7d86
	ctx.r[10].s64 = -2105933824;
	// 826621D8: 394ABBD0  addi r10, r10, -0x4430
	ctx.r[10].s64 = ctx.r[10].s64 + -17456;
	// 826621DC: 816B4EDC  lwz r11, 0x4edc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20188 as u32) ) } as u64;
	// 826621E0: 916A0020  stw r11, 0x20(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 826621E4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826621E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826621E8 size=112
    let mut pc: u32 = 0x826621E8;
    'dispatch: loop {
        match pc {
            0x826621E8 => {
    //   block [0x826621E8..0x82662258)
	// 826621E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826621EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826621F0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826621F4: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 826621F8: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 826621FC: 392AED3C  addi r9, r10, -0x12c4
	ctx.r[9].s64 = ctx.r[10].s64 + -4804;
	// 82662200: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82662204: 390BBBD0  addi r8, r11, -0x4430
	ctx.r[8].s64 = ctx.r[11].s64 + -17456;
	// 82662208: 38E00002  li r7, 2
	ctx.r[7].s64 = 2;
	// 8266220C: 388A3728  addi r4, r10, 0x3728
	ctx.r[4].s64 = ctx.r[10].s64 + 14120;
	// 82662210: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82662214: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82662218: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8266221C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82662220: 386ABA10  addi r3, r10, -0x45f0
	ctx.r[3].s64 = ctx.r[10].s64 + -17904;
	// 82662224: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82662228: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8266222C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82662230: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82662234: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82662238: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8266223C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82662240: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82662244: 4BE04BDD  bl 0x82466e20
	ctx.lr = 0x82662248;
	sub_82466E20(ctx, base);
	// 82662248: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8266224C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82662250: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82662254: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82662258(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82662258 size=112
    let mut pc: u32 = 0x82662258;
    'dispatch: loop {
        match pc {
            0x82662258 => {
    //   block [0x82662258..0x826622C8)
	// 82662258: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8266225C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82662260: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82662264: 3D408279  lis r10, -0x7d87
	ctx.r[10].s64 = -2105999360;
	// 82662268: 39000006  li r8, 6
	ctx.r[8].s64 = 6;
	// 8266226C: 38EA5348  addi r7, r10, 0x5348
	ctx.r[7].s64 = ctx.r[10].s64 + 21320;
	// 82662270: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82662274: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 82662278: 388A3738  addi r4, r10, 0x3738
	ctx.r[4].s64 = ctx.r[10].s64 + 14136;
	// 8266227C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82662280: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82662284: 396BED50  addi r11, r11, -0x12b0
	ctx.r[11].s64 = ctx.r[11].s64 + -4784;
	// 82662288: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8266228C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82662290: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82662294: 386ABA40  addi r3, r10, -0x45c0
	ctx.r[3].s64 = ctx.r[10].s64 + -17856;
	// 82662298: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8266229C: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 826622A0: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826622A4: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 826622A8: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826622AC: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826622B0: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826622B4: 4BE04B6D  bl 0x82466e20
	ctx.lr = 0x826622B8;
	sub_82466E20(ctx, base);
	// 826622B8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826622BC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826622C0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826622C4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826622C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826622C8 size=112
    let mut pc: u32 = 0x826622C8;
    'dispatch: loop {
        match pc {
            0x826622C8 => {
    //   block [0x826622C8..0x82662338)
	// 826622C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826622CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826622D0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826622D4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 826622D8: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 826622DC: 38AABB60  addi r5, r10, -0x44a0
	ctx.r[5].s64 = ctx.r[10].s64 + -17568;
	// 826622E0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826622E4: 390B53D8  addi r8, r11, 0x53d8
	ctx.r[8].s64 = ctx.r[11].s64 + 21464;
	// 826622E8: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826622EC: 388A3758  addi r4, r10, 0x3758
	ctx.r[4].s64 = ctx.r[10].s64 + 14168;
	// 826622F0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826622F4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 826622F8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826622FC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82662300: 386ABA70  addi r3, r10, -0x4590
	ctx.r[3].s64 = ctx.r[10].s64 + -17808;
	// 82662304: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82662308: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8266230C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82662310: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82662314: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82662318: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8266231C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82662320: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82662324: 4BE04AFD  bl 0x82466e20
	ctx.lr = 0x82662328;
	sub_82466E20(ctx, base);
	// 82662328: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8266232C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82662330: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82662334: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82662338(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82662338 size=108
    let mut pc: u32 = 0x82662338;
    'dispatch: loop {
        match pc {
            0x82662338 => {
    //   block [0x82662338..0x826623A4)
	// 82662338: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8266233C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82662340: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82662344: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 82662348: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8266234C: 38EB53F8  addi r7, r11, 0x53f8
	ctx.r[7].s64 = ctx.r[11].s64 + 21496;
	// 82662350: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 82662354: 388A3770  addi r4, r10, 0x3770
	ctx.r[4].s64 = ctx.r[10].s64 + 14192;
	// 82662358: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8266235C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82662360: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82662364: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82662368: 386ABAA0  addi r3, r10, -0x4560
	ctx.r[3].s64 = ctx.r[10].s64 + -17760;
	// 8266236C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82662370: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82662374: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82662378: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8266237C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82662380: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82662384: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82662388: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8266238C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82662390: 4BE04A91  bl 0x82466e20
	ctx.lr = 0x82662394;
	sub_82466E20(ctx, base);
	// 82662394: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82662398: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8266239C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826623A0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826623A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826623A8 size=108
    let mut pc: u32 = 0x826623A8;
    'dispatch: loop {
        match pc {
            0x826623A8 => {
    //   block [0x826623A8..0x82662414)
	// 826623A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826623AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826623B0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826623B4: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 826623B8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826623BC: 38EB5458  addi r7, r11, 0x5458
	ctx.r[7].s64 = ctx.r[11].s64 + 21592;
	// 826623C0: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826623C4: 388A3788  addi r4, r10, 0x3788
	ctx.r[4].s64 = ctx.r[10].s64 + 14216;
	// 826623C8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826623CC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 826623D0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826623D4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826623D8: 386ABAD0  addi r3, r10, -0x4530
	ctx.r[3].s64 = ctx.r[10].s64 + -17712;
	// 826623DC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826623E0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826623E4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826623E8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826623EC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826623F0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826623F4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826623F8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826623FC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82662400: 4BE04A21  bl 0x82466e20
	ctx.lr = 0x82662404;
	sub_82466E20(ctx, base);
	// 82662404: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82662408: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8266240C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82662410: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82662418(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82662418 size=116
    let mut pc: u32 = 0x82662418;
    'dispatch: loop {
        match pc {
            0x82662418 => {
    //   block [0x82662418..0x8266248C)
	// 82662418: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8266241C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82662420: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82662424: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 82662428: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 8266242C: 390B5488  addi r8, r11, 0x5488
	ctx.r[8].s64 = ctx.r[11].s64 + 21640;
	// 82662430: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82662434: 392AED84  addi r9, r10, -0x127c
	ctx.r[9].s64 = ctx.r[10].s64 + -4732;
	// 82662438: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8266243C: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
	// 82662440: 38AABB60  addi r5, r10, -0x44a0
	ctx.r[5].s64 = ctx.r[10].s64 + -17568;
	// 82662444: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82662448: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8266244C: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82662450: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82662454: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82662458: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8266245C: 3D608296  lis r11, -0x7d6a
	ctx.r[11].s64 = -2104098816;
	// 82662460: 388A37A0  addi r4, r10, 0x37a0
	ctx.r[4].s64 = ctx.r[10].s64 + 14240;
	// 82662464: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82662468: 386BBB00  addi r3, r11, -0x4500
	ctx.r[3].s64 = ctx.r[11].s64 + -17664;
	// 8266246C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82662470: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82662474: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82662478: 4BE049A9  bl 0x82466e20
	ctx.lr = 0x8266247C;
	sub_82466E20(ctx, base);
	// 8266247C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82662480: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82662484: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82662488: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82662490(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82662490 size=96
    let mut pc: u32 = 0x82662490;
    'dispatch: loop {
        match pc {
            0x82662490 => {
    //   block [0x82662490..0x826624F0)
	// 82662490: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82662494: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82662498: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8266249C: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 826624A0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826624A4: 388A6DD4  addi r4, r10, 0x6dd4
	ctx.r[4].s64 = ctx.r[10].s64 + 28116;
	// 826624A8: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 826624AC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826624B0: 386ABB30  addi r3, r10, -0x44d0
	ctx.r[3].s64 = ctx.r[10].s64 + -17616;
	// 826624B4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826624B8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826624BC: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826624C0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826624C4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826624C8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826624CC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826624D0: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826624D4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826624D8: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826624DC: 4BE04945  bl 0x82466e20
	ctx.lr = 0x826624E0;
	sub_82466E20(ctx, base);
	// 826624E0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826624E4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826624E8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826624EC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826624F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826624F0 size=112
    let mut pc: u32 = 0x826624F0;
    'dispatch: loop {
        match pc {
            0x826624F0 => {
    //   block [0x826624F0..0x82662560)
	// 826624F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826624F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826624F8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826624FC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82662500: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 82662504: 38AABB30  addi r5, r10, -0x44d0
	ctx.r[5].s64 = ctx.r[10].s64 + -17616;
	// 82662508: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 8266250C: 390B54A0  addi r8, r11, 0x54a0
	ctx.r[8].s64 = ctx.r[11].s64 + 21664;
	// 82662510: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 82662514: 388A7380  addi r4, r10, 0x7380
	ctx.r[4].s64 = ctx.r[10].s64 + 29568;
	// 82662518: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8266251C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82662520: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82662524: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82662528: 386ABB60  addi r3, r10, -0x44a0
	ctx.r[3].s64 = ctx.r[10].s64 + -17568;
	// 8266252C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82662530: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82662534: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82662538: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8266253C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82662540: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82662544: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82662548: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8266254C: 4BE048D5  bl 0x82466e20
	ctx.lr = 0x82662550;
	sub_82466E20(ctx, base);
	// 82662550: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82662554: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82662558: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8266255C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82662560(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82662560 size=112
    let mut pc: u32 = 0x82662560;
    'dispatch: loop {
        match pc {
            0x82662560 => {
    //   block [0x82662560..0x826625D0)
	// 82662560: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82662564: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82662568: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8266256C: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 82662570: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 82662574: 392AEDA0  addi r9, r10, -0x1260
	ctx.r[9].s64 = ctx.r[10].s64 + -4704;
	// 82662578: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 8266257C: 390B54D0  addi r8, r11, 0x54d0
	ctx.r[8].s64 = ctx.r[11].s64 + 21712;
	// 82662580: 38E00007  li r7, 7
	ctx.r[7].s64 = 7;
	// 82662584: 388A7468  addi r4, r10, 0x7468
	ctx.r[4].s64 = ctx.r[10].s64 + 29800;
	// 82662588: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8266258C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82662590: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82662594: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82662598: 386ABB90  addi r3, r10, -0x4470
	ctx.r[3].s64 = ctx.r[10].s64 + -17520;
	// 8266259C: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826625A0: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826625A4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826625A8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826625AC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826625B0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826625B4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826625B8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826625BC: 4BE04865  bl 0x82466e20
	ctx.lr = 0x826625C0;
	sub_82466E20(ctx, base);
	// 826625C0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826625C4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826625C8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826625CC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826625D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826625D0 size=108
    let mut pc: u32 = 0x826625D0;
    'dispatch: loop {
        match pc {
            0x826625D0 => {
    //   block [0x826625D0..0x8266263C)
	// 826625D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826625D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826625D8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826625DC: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 826625E0: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 826625E4: 38EB5578  addi r7, r11, 0x5578
	ctx.r[7].s64 = ctx.r[11].s64 + 21880;
	// 826625E8: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826625EC: 388A7888  addi r4, r10, 0x7888
	ctx.r[4].s64 = ctx.r[10].s64 + 30856;
	// 826625F0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826625F4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 826625F8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826625FC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82662600: 386ABBC0  addi r3, r10, -0x4440
	ctx.r[3].s64 = ctx.r[10].s64 + -17472;
	// 82662604: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82662608: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8266260C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82662610: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82662614: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82662618: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8266261C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82662620: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82662624: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82662628: 4BE047F9  bl 0x82466e20
	ctx.lr = 0x8266262C;
	sub_82466E20(ctx, base);
	// 8266262C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82662630: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82662634: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82662638: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82662640(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82662640 size=108
    let mut pc: u32 = 0x82662640;
    'dispatch: loop {
        match pc {
            0x82662640 => {
    //   block [0x82662640..0x826626AC)
	// 82662640: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82662644: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82662648: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8266264C: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 82662650: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 82662654: 38EB55A8  addi r7, r11, 0x55a8
	ctx.r[7].s64 = ctx.r[11].s64 + 21928;
	// 82662658: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 8266265C: 388A7898  addi r4, r10, 0x7898
	ctx.r[4].s64 = ctx.r[10].s64 + 30872;
	// 82662660: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82662664: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82662668: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8266266C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82662670: 386ABBF0  addi r3, r10, -0x4410
	ctx.r[3].s64 = ctx.r[10].s64 + -17424;
	// 82662674: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82662678: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8266267C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82662680: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82662684: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82662688: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8266268C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82662690: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82662694: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82662698: 4BE04789  bl 0x82466e20
	ctx.lr = 0x8266269C;
	sub_82466E20(ctx, base);
	// 8266269C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826626A0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826626A4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826626A8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826626B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826626B0 size=28
    let mut pc: u32 = 0x826626B0;
    'dispatch: loop {
        match pc {
            0x826626B0 => {
    //   block [0x826626B0..0x826626CC)
	// 826626B0: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 826626B4: 3D40827A  lis r10, -0x7d86
	ctx.r[10].s64 = -2105933824;
	// 826626B8: 394ABC00  addi r10, r10, -0x4400
	ctx.r[10].s64 = ctx.r[10].s64 + -17408;
	// 826626BC: 816B55D8  lwz r11, 0x55d8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(21976 as u32) ) } as u64;
	// 826626C0: 916A0050  stw r11, 0x50(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 826626C4: 916A0068  stw r11, 0x68(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(104 as u32), ctx.r[11].u32 ) };
	// 826626C8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826626D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826626D0 size=112
    let mut pc: u32 = 0x826626D0;
    'dispatch: loop {
        match pc {
            0x826626D0 => {
    //   block [0x826626D0..0x82662740)
	// 826626D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826626D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826626D8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826626DC: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 826626E0: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 826626E4: 392AEF50  addi r9, r10, -0x10b0
	ctx.r[9].s64 = ctx.r[10].s64 + -4272;
	// 826626E8: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 826626EC: 390BBC00  addi r8, r11, -0x4400
	ctx.r[8].s64 = ctx.r[11].s64 + -17408;
	// 826626F0: 38E00008  li r7, 8
	ctx.r[7].s64 = 8;
	// 826626F4: 388A72F4  addi r4, r10, 0x72f4
	ctx.r[4].s64 = ctx.r[10].s64 + 29428;
	// 826626F8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826626FC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82662700: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82662704: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82662708: 386ABC20  addi r3, r10, -0x43e0
	ctx.r[3].s64 = ctx.r[10].s64 + -17376;
	// 8266270C: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82662710: 39400003  li r10, 3
	ctx.r[10].s64 = 3;
	// 82662714: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82662718: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8266271C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82662720: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82662724: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82662728: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8266272C: 4BE046F5  bl 0x82466e20
	ctx.lr = 0x82662730;
	sub_82466E20(ctx, base);
	// 82662730: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82662734: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82662738: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8266273C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82662740(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82662740 size=108
    let mut pc: u32 = 0x82662740;
    'dispatch: loop {
        match pc {
            0x82662740 => {
    //   block [0x82662740..0x826627AC)
	// 82662740: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82662744: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82662748: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8266274C: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 82662750: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 82662754: 38EB55E4  addi r7, r11, 0x55e4
	ctx.r[7].s64 = ctx.r[11].s64 + 21988;
	// 82662758: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 8266275C: 388A7914  addi r4, r10, 0x7914
	ctx.r[4].s64 = ctx.r[10].s64 + 30996;
	// 82662760: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82662764: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82662768: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8266276C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82662770: 386ABC50  addi r3, r10, -0x43b0
	ctx.r[3].s64 = ctx.r[10].s64 + -17328;
	// 82662774: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82662778: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8266277C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82662780: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82662784: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82662788: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8266278C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82662790: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82662794: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82662798: 4BE04689  bl 0x82466e20
	ctx.lr = 0x8266279C;
	sub_82466E20(ctx, base);
	// 8266279C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826627A0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826627A4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826627A8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826627B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826627B0 size=108
    let mut pc: u32 = 0x826627B0;
    'dispatch: loop {
        match pc {
            0x826627B0 => {
    //   block [0x826627B0..0x8266281C)
	// 826627B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826627B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826627B8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826627BC: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 826627C0: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 826627C4: 38EB5614  addi r7, r11, 0x5614
	ctx.r[7].s64 = ctx.r[11].s64 + 22036;
	// 826627C8: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826627CC: 388A7788  addi r4, r10, 0x7788
	ctx.r[4].s64 = ctx.r[10].s64 + 30600;
	// 826627D0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826627D4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 826627D8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826627DC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826627E0: 386ABC80  addi r3, r10, -0x4380
	ctx.r[3].s64 = ctx.r[10].s64 + -17280;
	// 826627E4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826627E8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826627EC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826627F0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826627F4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826627F8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826627FC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82662800: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82662804: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82662808: 4BE04619  bl 0x82466e20
	ctx.lr = 0x8266280C;
	sub_82466E20(ctx, base);
	// 8266280C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82662810: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82662814: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82662818: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82662820(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82662820 size=108
    let mut pc: u32 = 0x82662820;
    'dispatch: loop {
        match pc {
            0x82662820 => {
    //   block [0x82662820..0x8266288C)
	// 82662820: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82662824: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82662828: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8266282C: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 82662830: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 82662834: 38EB5644  addi r7, r11, 0x5644
	ctx.r[7].s64 = ctx.r[11].s64 + 22084;
	// 82662838: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 8266283C: 388A77AC  addi r4, r10, 0x77ac
	ctx.r[4].s64 = ctx.r[10].s64 + 30636;
	// 82662840: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82662844: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82662848: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8266284C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82662850: 386ABCB0  addi r3, r10, -0x4350
	ctx.r[3].s64 = ctx.r[10].s64 + -17232;
	// 82662854: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82662858: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8266285C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82662860: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82662864: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82662868: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8266286C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82662870: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82662874: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82662878: 4BE045A9  bl 0x82466e20
	ctx.lr = 0x8266287C;
	sub_82466E20(ctx, base);
	// 8266287C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82662880: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82662884: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82662888: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82662890(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82662890 size=24
    let mut pc: u32 = 0x82662890;
    'dispatch: loop {
        match pc {
            0x82662890 => {
    //   block [0x82662890..0x826628A8)
	// 82662890: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 82662894: 3D40827A  lis r10, -0x7d86
	ctx.r[10].s64 = -2105933824;
	// 82662898: 394ABCC0  addi r10, r10, -0x4340
	ctx.r[10].s64 = ctx.r[10].s64 + -17216;
	// 8266289C: 816B565C  lwz r11, 0x565c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(22108 as u32) ) } as u64;
	// 826628A0: 916A0050  stw r11, 0x50(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 826628A4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826628A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826628A8 size=112
    let mut pc: u32 = 0x826628A8;
    'dispatch: loop {
        match pc {
            0x826628A8 => {
    //   block [0x826628A8..0x82662918)
	// 826628A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826628AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826628B0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826628B4: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 826628B8: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 826628BC: 392AEFA4  addi r9, r10, -0x105c
	ctx.r[9].s64 = ctx.r[10].s64 + -4188;
	// 826628C0: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 826628C4: 390BBCC0  addi r8, r11, -0x4340
	ctx.r[8].s64 = ctx.r[11].s64 + -17216;
	// 826628C8: 38E00009  li r7, 9
	ctx.r[7].s64 = 9;
	// 826628CC: 388A77C8  addi r4, r10, 0x77c8
	ctx.r[4].s64 = ctx.r[10].s64 + 30664;
	// 826628D0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826628D4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 826628D8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826628DC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826628E0: 386ABCE0  addi r3, r10, -0x4320
	ctx.r[3].s64 = ctx.r[10].s64 + -17184;
	// 826628E4: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826628E8: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826628EC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826628F0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826628F4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826628F8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826628FC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82662900: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82662904: 4BE0451D  bl 0x82466e20
	ctx.lr = 0x82662908;
	sub_82466E20(ctx, base);
	// 82662908: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8266290C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82662910: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82662914: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82662918(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82662918 size=112
    let mut pc: u32 = 0x82662918;
    'dispatch: loop {
        match pc {
            0x82662918 => {
    //   block [0x82662918..0x82662988)
	// 82662918: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8266291C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82662920: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82662924: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 82662928: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 8266292C: 392AEFE0  addi r9, r10, -0x1020
	ctx.r[9].s64 = ctx.r[10].s64 + -4128;
	// 82662930: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82662934: 390B5668  addi r8, r11, 0x5668
	ctx.r[8].s64 = ctx.r[11].s64 + 22120;
	// 82662938: 38E00003  li r7, 3
	ctx.r[7].s64 = 3;
	// 8266293C: 388A8198  addi r4, r10, -0x7e68
	ctx.r[4].s64 = ctx.r[10].s64 + -32360;
	// 82662940: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82662944: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82662948: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8266294C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82662950: 386ABD10  addi r3, r10, -0x42f0
	ctx.r[3].s64 = ctx.r[10].s64 + -17136;
	// 82662954: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82662958: 39400002  li r10, 2
	ctx.r[10].s64 = 2;
	// 8266295C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82662960: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82662964: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82662968: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8266296C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82662970: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82662974: 4BE044AD  bl 0x82466e20
	ctx.lr = 0x82662978;
	sub_82466E20(ctx, base);
	// 82662978: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8266297C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82662980: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82662984: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82662988(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82662988 size=108
    let mut pc: u32 = 0x82662988;
    'dispatch: loop {
        match pc {
            0x82662988 => {
    //   block [0x82662988..0x826629F4)
	// 82662988: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8266298C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82662990: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82662994: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 82662998: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 8266299C: 38EB56B0  addi r7, r11, 0x56b0
	ctx.r[7].s64 = ctx.r[11].s64 + 22192;
	// 826629A0: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826629A4: 388A7BB0  addi r4, r10, 0x7bb0
	ctx.r[4].s64 = ctx.r[10].s64 + 31664;
	// 826629A8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826629AC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 826629B0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826629B4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826629B8: 386ABD40  addi r3, r10, -0x42c0
	ctx.r[3].s64 = ctx.r[10].s64 + -17088;
	// 826629BC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826629C0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826629C4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826629C8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826629CC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826629D0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826629D4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826629D8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826629DC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826629E0: 4BE04441  bl 0x82466e20
	ctx.lr = 0x826629E4;
	sub_82466E20(ctx, base);
	// 826629E4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826629E8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826629EC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826629F0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826629F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826629F8 size=108
    let mut pc: u32 = 0x826629F8;
    'dispatch: loop {
        match pc {
            0x826629F8 => {
    //   block [0x826629F8..0x82662A64)
	// 826629F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826629FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82662A00: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82662A04: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 82662A08: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 82662A0C: 38EB56E0  addi r7, r11, 0x56e0
	ctx.r[7].s64 = ctx.r[11].s64 + 22240;
	// 82662A10: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 82662A14: 388A7F88  addi r4, r10, 0x7f88
	ctx.r[4].s64 = ctx.r[10].s64 + 32648;
	// 82662A18: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82662A1C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82662A20: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82662A24: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82662A28: 386ABD70  addi r3, r10, -0x4290
	ctx.r[3].s64 = ctx.r[10].s64 + -17040;
	// 82662A2C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82662A30: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82662A34: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82662A38: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82662A3C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82662A40: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82662A44: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82662A48: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82662A4C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82662A50: 4BE043D1  bl 0x82466e20
	ctx.lr = 0x82662A54;
	sub_82466E20(ctx, base);
	// 82662A54: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82662A58: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82662A5C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82662A60: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82662A68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82662A68 size=112
    let mut pc: u32 = 0x82662A68;
    'dispatch: loop {
        match pc {
            0x82662A68 => {
    //   block [0x82662A68..0x82662AD8)
	// 82662A68: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82662A6C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82662A70: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82662A74: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 82662A78: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 82662A7C: 392AF020  addi r9, r10, -0xfe0
	ctx.r[9].s64 = ctx.r[10].s64 + -4064;
	// 82662A80: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82662A84: 390B5718  addi r8, r11, 0x5718
	ctx.r[8].s64 = ctx.r[11].s64 + 22296;
	// 82662A88: 38E00005  li r7, 5
	ctx.r[7].s64 = 5;
	// 82662A8C: 388A82E8  addi r4, r10, -0x7d18
	ctx.r[4].s64 = ctx.r[10].s64 + -32024;
	// 82662A90: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82662A94: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82662A98: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82662A9C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82662AA0: 386ABDA0  addi r3, r10, -0x4260
	ctx.r[3].s64 = ctx.r[10].s64 + -16992;
	// 82662AA4: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82662AA8: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82662AAC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82662AB0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82662AB4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82662AB8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82662ABC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82662AC0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82662AC4: 4BE0435D  bl 0x82466e20
	ctx.lr = 0x82662AC8;
	sub_82466E20(ctx, base);
	// 82662AC8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82662ACC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82662AD0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82662AD4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82662AD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82662AD8 size=108
    let mut pc: u32 = 0x82662AD8;
    'dispatch: loop {
        match pc {
            0x82662AD8 => {
    //   block [0x82662AD8..0x82662B44)
	// 82662AD8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82662ADC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82662AE0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82662AE4: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 82662AE8: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 82662AEC: 38EB5790  addi r7, r11, 0x5790
	ctx.r[7].s64 = ctx.r[11].s64 + 22416;
	// 82662AF0: 3900000B  li r8, 0xb
	ctx.r[8].s64 = 11;
	// 82662AF4: 388A7F28  addi r4, r10, 0x7f28
	ctx.r[4].s64 = ctx.r[10].s64 + 32552;
	// 82662AF8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82662AFC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82662B00: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82662B04: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82662B08: 386ABDD0  addi r3, r10, -0x4230
	ctx.r[3].s64 = ctx.r[10].s64 + -16944;
	// 82662B0C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82662B10: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82662B14: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82662B18: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82662B1C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82662B20: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82662B24: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82662B28: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82662B2C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82662B30: 4BE042F1  bl 0x82466e20
	ctx.lr = 0x82662B34;
	sub_82466E20(ctx, base);
	// 82662B34: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82662B38: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82662B3C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82662B40: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82662B48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82662B48 size=108
    let mut pc: u32 = 0x82662B48;
    'dispatch: loop {
        match pc {
            0x82662B48 => {
    //   block [0x82662B48..0x82662BB4)
	// 82662B48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82662B4C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82662B50: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82662B54: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 82662B58: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 82662B5C: 38EB5898  addi r7, r11, 0x5898
	ctx.r[7].s64 = ctx.r[11].s64 + 22680;
	// 82662B60: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 82662B64: 388A78C0  addi r4, r10, 0x78c0
	ctx.r[4].s64 = ctx.r[10].s64 + 30912;
	// 82662B68: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82662B6C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82662B70: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82662B74: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82662B78: 386ABE00  addi r3, r10, -0x4200
	ctx.r[3].s64 = ctx.r[10].s64 + -16896;
	// 82662B7C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82662B80: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82662B84: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82662B88: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82662B8C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82662B90: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82662B94: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82662B98: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82662B9C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82662BA0: 4BE04281  bl 0x82466e20
	ctx.lr = 0x82662BA4;
	sub_82466E20(ctx, base);
	// 82662BA4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82662BA8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82662BAC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82662BB0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82662BB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82662BB8 size=108
    let mut pc: u32 = 0x82662BB8;
    'dispatch: loop {
        match pc {
            0x82662BB8 => {
    //   block [0x82662BB8..0x82662C24)
	// 82662BB8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82662BBC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82662BC0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82662BC4: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 82662BC8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82662BCC: 38EB58B0  addi r7, r11, 0x58b0
	ctx.r[7].s64 = ctx.r[11].s64 + 22704;
	// 82662BD0: 39000005  li r8, 5
	ctx.r[8].s64 = 5;
	// 82662BD4: 388A8060  addi r4, r10, -0x7fa0
	ctx.r[4].s64 = ctx.r[10].s64 + -32672;
	// 82662BD8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82662BDC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82662BE0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82662BE4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82662BE8: 386ABE30  addi r3, r10, -0x41d0
	ctx.r[3].s64 = ctx.r[10].s64 + -16848;
	// 82662BEC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82662BF0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82662BF4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82662BF8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82662BFC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82662C00: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82662C04: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82662C08: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82662C0C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82662C10: 4BE04211  bl 0x82466e20
	ctx.lr = 0x82662C14;
	sub_82466E20(ctx, base);
	// 82662C14: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82662C18: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82662C1C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82662C20: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82662C28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82662C28 size=24
    let mut pc: u32 = 0x82662C28;
    'dispatch: loop {
        match pc {
            0x82662C28 => {
    //   block [0x82662C28..0x82662C40)
	// 82662C28: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 82662C2C: 3D40827A  lis r10, -0x7d86
	ctx.r[10].s64 = -2105933824;
	// 82662C30: 394ABD98  addi r10, r10, -0x4268
	ctx.r[10].s64 = ctx.r[10].s64 + -17000;
	// 82662C34: 816B5714  lwz r11, 0x5714(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(22292 as u32) ) } as u64;
	// 82662C38: 916A0020  stw r11, 0x20(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 82662C3C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82662C40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82662C40 size=108
    let mut pc: u32 = 0x82662C40;
    'dispatch: loop {
        match pc {
            0x82662C40 => {
    //   block [0x82662C40..0x82662CAC)
	// 82662C40: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82662C44: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82662C48: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82662C4C: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 82662C50: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82662C54: 38EBBD98  addi r7, r11, -0x4268
	ctx.r[7].s64 = ctx.r[11].s64 + -17000;
	// 82662C58: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 82662C5C: 388AB020  addi r4, r10, -0x4fe0
	ctx.r[4].s64 = ctx.r[10].s64 + -20448;
	// 82662C60: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82662C64: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82662C68: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82662C6C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82662C70: 386ABE60  addi r3, r10, -0x41a0
	ctx.r[3].s64 = ctx.r[10].s64 + -16800;
	// 82662C74: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82662C78: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82662C7C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82662C80: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82662C84: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82662C88: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82662C8C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82662C90: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82662C94: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82662C98: 4BE04189  bl 0x82466e20
	ctx.lr = 0x82662C9C;
	sub_82466E20(ctx, base);
	// 82662C9C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82662CA0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82662CA4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82662CA8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82662CB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82662CB0 size=24
    let mut pc: u32 = 0x82662CB0;
    'dispatch: loop {
        match pc {
            0x82662CB0 => {
    //   block [0x82662CB0..0x82662CC8)
	// 82662CB0: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 82662CB4: 3D40827A  lis r10, -0x7d86
	ctx.r[10].s64 = -2105933824;
	// 82662CB8: 394ABDC8  addi r10, r10, -0x4238
	ctx.r[10].s64 = ctx.r[10].s64 + -16952;
	// 82662CBC: 816B5714  lwz r11, 0x5714(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(22292 as u32) ) } as u64;
	// 82662CC0: 916A0020  stw r11, 0x20(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 82662CC4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82662CC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82662CC8 size=108
    let mut pc: u32 = 0x82662CC8;
    'dispatch: loop {
        match pc {
            0x82662CC8 => {
    //   block [0x82662CC8..0x82662D34)
	// 82662CC8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82662CCC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82662CD0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82662CD4: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 82662CD8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82662CDC: 38EBBDC8  addi r7, r11, -0x4238
	ctx.r[7].s64 = ctx.r[11].s64 + -16952;
	// 82662CE0: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 82662CE4: 388A9C48  addi r4, r10, -0x63b8
	ctx.r[4].s64 = ctx.r[10].s64 + -25528;
	// 82662CE8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82662CEC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82662CF0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82662CF4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82662CF8: 386ABE90  addi r3, r10, -0x4170
	ctx.r[3].s64 = ctx.r[10].s64 + -16752;
	// 82662CFC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82662D00: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82662D04: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82662D08: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82662D0C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82662D10: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82662D14: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82662D18: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82662D1C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82662D20: 4BE04101  bl 0x82466e20
	ctx.lr = 0x82662D24;
	sub_82466E20(ctx, base);
	// 82662D24: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82662D28: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82662D2C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82662D30: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82662D38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82662D38 size=108
    let mut pc: u32 = 0x82662D38;
    'dispatch: loop {
        match pc {
            0x82662D38 => {
    //   block [0x82662D38..0x82662DA4)
	// 82662D38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82662D3C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82662D40: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82662D44: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 82662D48: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82662D4C: 38EB5928  addi r7, r11, 0x5928
	ctx.r[7].s64 = ctx.r[11].s64 + 22824;
	// 82662D50: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 82662D54: 388AA080  addi r4, r10, -0x5f80
	ctx.r[4].s64 = ctx.r[10].s64 + -24448;
	// 82662D58: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82662D5C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82662D60: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82662D64: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82662D68: 386ABEC0  addi r3, r10, -0x4140
	ctx.r[3].s64 = ctx.r[10].s64 + -16704;
	// 82662D6C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82662D70: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82662D74: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82662D78: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82662D7C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82662D80: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82662D84: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82662D88: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82662D8C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82662D90: 4BE04091  bl 0x82466e20
	ctx.lr = 0x82662D94;
	sub_82466E20(ctx, base);
	// 82662D94: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82662D98: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82662D9C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82662DA0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82662DA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82662DA8 size=24
    let mut pc: u32 = 0x82662DA8;
    'dispatch: loop {
        match pc {
            0x82662DA8 => {
    //   block [0x82662DA8..0x82662DC0)
	// 82662DA8: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 82662DAC: 3D40827A  lis r10, -0x7d86
	ctx.r[10].s64 = -2105933824;
	// 82662DB0: 394ABDF8  addi r10, r10, -0x4208
	ctx.r[10].s64 = ctx.r[10].s64 + -16904;
	// 82662DB4: 816B5714  lwz r11, 0x5714(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(22292 as u32) ) } as u64;
	// 82662DB8: 916A0020  stw r11, 0x20(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 82662DBC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82662DC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82662DC0 size=108
    let mut pc: u32 = 0x82662DC0;
    'dispatch: loop {
        match pc {
            0x82662DC0 => {
    //   block [0x82662DC0..0x82662E2C)
	// 82662DC0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82662DC4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82662DC8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82662DCC: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 82662DD0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82662DD4: 38EBBDF8  addi r7, r11, -0x4208
	ctx.r[7].s64 = ctx.r[11].s64 + -16904;
	// 82662DD8: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 82662DDC: 388A9BE8  addi r4, r10, -0x6418
	ctx.r[4].s64 = ctx.r[10].s64 + -25624;
	// 82662DE0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82662DE4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82662DE8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82662DEC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82662DF0: 386ABEF0  addi r3, r10, -0x4110
	ctx.r[3].s64 = ctx.r[10].s64 + -16656;
	// 82662DF4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82662DF8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82662DFC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82662E00: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82662E04: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82662E08: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82662E0C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82662E10: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82662E14: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82662E18: 4BE04009  bl 0x82466e20
	ctx.lr = 0x82662E1C;
	sub_82466E20(ctx, base);
	// 82662E1C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82662E20: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82662E24: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82662E28: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82662E30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82662E30 size=112
    let mut pc: u32 = 0x82662E30;
    'dispatch: loop {
        match pc {
            0x82662E30 => {
    //   block [0x82662E30..0x82662EA0)
	// 82662E30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82662E34: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82662E38: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82662E3C: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 82662E40: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 82662E44: 392AF064  addi r9, r10, -0xf9c
	ctx.r[9].s64 = ctx.r[10].s64 + -3996;
	// 82662E48: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82662E4C: 390B5940  addi r8, r11, 0x5940
	ctx.r[8].s64 = ctx.r[11].s64 + 22848;
	// 82662E50: 38E00002  li r7, 2
	ctx.r[7].s64 = 2;
	// 82662E54: 388AA778  addi r4, r10, -0x5888
	ctx.r[4].s64 = ctx.r[10].s64 + -22664;
	// 82662E58: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82662E5C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82662E60: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82662E64: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82662E68: 386ABF20  addi r3, r10, -0x40e0
	ctx.r[3].s64 = ctx.r[10].s64 + -16608;
	// 82662E6C: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82662E70: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82662E74: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82662E78: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82662E7C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82662E80: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82662E84: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82662E88: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82662E8C: 4BE03F95  bl 0x82466e20
	ctx.lr = 0x82662E90;
	sub_82466E20(ctx, base);
	// 82662E90: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82662E94: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82662E98: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82662E9C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82662EA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82662EA0 size=108
    let mut pc: u32 = 0x82662EA0;
    'dispatch: loop {
        match pc {
            0x82662EA0 => {
    //   block [0x82662EA0..0x82662F0C)
	// 82662EA0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82662EA4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82662EA8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82662EAC: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 82662EB0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82662EB4: 38EB5970  addi r7, r11, 0x5970
	ctx.r[7].s64 = ctx.r[11].s64 + 22896;
	// 82662EB8: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 82662EBC: 388AA350  addi r4, r10, -0x5cb0
	ctx.r[4].s64 = ctx.r[10].s64 + -23728;
	// 82662EC0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82662EC4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82662EC8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82662ECC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82662ED0: 386ABF50  addi r3, r10, -0x40b0
	ctx.r[3].s64 = ctx.r[10].s64 + -16560;
	// 82662ED4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82662ED8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82662EDC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82662EE0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82662EE4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82662EE8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82662EEC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82662EF0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82662EF4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82662EF8: 4BE03F29  bl 0x82466e20
	ctx.lr = 0x82662EFC;
	sub_82466E20(ctx, base);
	// 82662EFC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82662F00: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82662F04: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82662F08: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82662F10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82662F10 size=108
    let mut pc: u32 = 0x82662F10;
    'dispatch: loop {
        match pc {
            0x82662F10 => {
    //   block [0x82662F10..0x82662F7C)
	// 82662F10: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82662F14: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82662F18: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82662F1C: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 82662F20: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82662F24: 38EB59A0  addi r7, r11, 0x59a0
	ctx.r[7].s64 = ctx.r[11].s64 + 22944;
	// 82662F28: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 82662F2C: 388AB2D8  addi r4, r10, -0x4d28
	ctx.r[4].s64 = ctx.r[10].s64 + -19752;
	// 82662F30: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82662F34: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82662F38: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82662F3C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82662F40: 386ABF80  addi r3, r10, -0x4080
	ctx.r[3].s64 = ctx.r[10].s64 + -16512;
	// 82662F44: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82662F48: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82662F4C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82662F50: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82662F54: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82662F58: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82662F5C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82662F60: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82662F64: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82662F68: 4BE03EB9  bl 0x82466e20
	ctx.lr = 0x82662F6C;
	sub_82466E20(ctx, base);
	// 82662F6C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82662F70: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82662F74: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82662F78: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82662F80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82662F80 size=108
    let mut pc: u32 = 0x82662F80;
    'dispatch: loop {
        match pc {
            0x82662F80 => {
    //   block [0x82662F80..0x82662FEC)
	// 82662F80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82662F84: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82662F88: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82662F8C: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 82662F90: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82662F94: 38EB59B8  addi r7, r11, 0x59b8
	ctx.r[7].s64 = ctx.r[11].s64 + 22968;
	// 82662F98: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 82662F9C: 388AAA44  addi r4, r10, -0x55bc
	ctx.r[4].s64 = ctx.r[10].s64 + -21948;
	// 82662FA0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82662FA4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82662FA8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82662FAC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82662FB0: 386ABFB0  addi r3, r10, -0x4050
	ctx.r[3].s64 = ctx.r[10].s64 + -16464;
	// 82662FB4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82662FB8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82662FBC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82662FC0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82662FC4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82662FC8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82662FCC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82662FD0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82662FD4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82662FD8: 4BE03E49  bl 0x82466e20
	ctx.lr = 0x82662FDC;
	sub_82466E20(ctx, base);
	// 82662FDC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82662FE0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82662FE4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82662FE8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82662FF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82662FF0 size=112
    let mut pc: u32 = 0x82662FF0;
    'dispatch: loop {
        match pc {
            0x82662FF0 => {
    //   block [0x82662FF0..0x82663060)
	// 82662FF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82662FF4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82662FF8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82662FFC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82663000: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 82663004: 38AAC010  addi r5, r10, -0x3ff0
	ctx.r[5].s64 = ctx.r[10].s64 + -16368;
	// 82663008: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8266300C: 390B59E8  addi r8, r11, 0x59e8
	ctx.r[8].s64 = ctx.r[11].s64 + 23016;
	// 82663010: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82663014: 388AA9A0  addi r4, r10, -0x5660
	ctx.r[4].s64 = ctx.r[10].s64 + -22112;
	// 82663018: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8266301C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82663020: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82663024: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82663028: 386ABFE0  addi r3, r10, -0x4020
	ctx.r[3].s64 = ctx.r[10].s64 + -16416;
	// 8266302C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82663030: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82663034: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82663038: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8266303C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82663040: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82663044: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82663048: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8266304C: 4BE03DD5  bl 0x82466e20
	ctx.lr = 0x82663050;
	sub_82466E20(ctx, base);
	// 82663050: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82663054: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82663058: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8266305C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82663060(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82663060 size=108
    let mut pc: u32 = 0x82663060;
    'dispatch: loop {
        match pc {
            0x82663060 => {
    //   block [0x82663060..0x826630CC)
	// 82663060: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82663064: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82663068: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8266306C: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 82663070: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82663074: 38EB5A00  addi r7, r11, 0x5a00
	ctx.r[7].s64 = ctx.r[11].s64 + 23040;
	// 82663078: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 8266307C: 388AA640  addi r4, r10, -0x59c0
	ctx.r[4].s64 = ctx.r[10].s64 + -22976;
	// 82663080: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82663084: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82663088: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8266308C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82663090: 386AC010  addi r3, r10, -0x3ff0
	ctx.r[3].s64 = ctx.r[10].s64 + -16368;
	// 82663094: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82663098: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8266309C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826630A0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826630A4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826630A8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826630AC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826630B0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826630B4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826630B8: 4BE03D69  bl 0x82466e20
	ctx.lr = 0x826630BC;
	sub_82466E20(ctx, base);
	// 826630BC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826630C0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826630C4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826630C8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826630D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826630D0 size=108
    let mut pc: u32 = 0x826630D0;
    'dispatch: loop {
        match pc {
            0x826630D0 => {
    //   block [0x826630D0..0x8266313C)
	// 826630D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826630D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826630D8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826630DC: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 826630E0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826630E4: 38EB5A30  addi r7, r11, 0x5a30
	ctx.r[7].s64 = ctx.r[11].s64 + 23088;
	// 826630E8: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826630EC: 388AA2E0  addi r4, r10, -0x5d20
	ctx.r[4].s64 = ctx.r[10].s64 + -23840;
	// 826630F0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826630F4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 826630F8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826630FC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82663100: 386AC040  addi r3, r10, -0x3fc0
	ctx.r[3].s64 = ctx.r[10].s64 + -16320;
	// 82663104: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82663108: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8266310C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82663110: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82663114: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82663118: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8266311C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82663120: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82663124: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82663128: 4BE03CF9  bl 0x82466e20
	ctx.lr = 0x8266312C;
	sub_82466E20(ctx, base);
	// 8266312C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82663130: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82663134: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82663138: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82663140(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82663140 size=108
    let mut pc: u32 = 0x82663140;
    'dispatch: loop {
        match pc {
            0x82663140 => {
    //   block [0x82663140..0x826631AC)
	// 82663140: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82663144: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82663148: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8266314C: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 82663150: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82663154: 38EB5A48  addi r7, r11, 0x5a48
	ctx.r[7].s64 = ctx.r[11].s64 + 23112;
	// 82663158: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 8266315C: 388AA304  addi r4, r10, -0x5cfc
	ctx.r[4].s64 = ctx.r[10].s64 + -23804;
	// 82663160: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82663164: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82663168: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8266316C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82663170: 386AC070  addi r3, r10, -0x3f90
	ctx.r[3].s64 = ctx.r[10].s64 + -16272;
	// 82663174: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82663178: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8266317C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82663180: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82663184: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82663188: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8266318C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82663190: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82663194: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82663198: 4BE03C89  bl 0x82466e20
	ctx.lr = 0x8266319C;
	sub_82466E20(ctx, base);
	// 8266319C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826631A0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826631A4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826631A8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826631B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826631B0 size=108
    let mut pc: u32 = 0x826631B0;
    'dispatch: loop {
        match pc {
            0x826631B0 => {
    //   block [0x826631B0..0x8266321C)
	// 826631B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826631B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826631B8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826631BC: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 826631C0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826631C4: 38EB5A78  addi r7, r11, 0x5a78
	ctx.r[7].s64 = ctx.r[11].s64 + 23160;
	// 826631C8: 39000007  li r8, 7
	ctx.r[8].s64 = 7;
	// 826631CC: 388AAED0  addi r4, r10, -0x5130
	ctx.r[4].s64 = ctx.r[10].s64 + -20784;
	// 826631D0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826631D4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 826631D8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826631DC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826631E0: 386AC0A0  addi r3, r10, -0x3f60
	ctx.r[3].s64 = ctx.r[10].s64 + -16224;
	// 826631E4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826631E8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826631EC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826631F0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826631F4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826631F8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826631FC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82663200: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82663204: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82663208: 4BE03C19  bl 0x82466e20
	ctx.lr = 0x8266320C;
	sub_82466E20(ctx, base);
	// 8266320C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82663210: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82663214: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82663218: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82663220(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82663220 size=108
    let mut pc: u32 = 0x82663220;
    'dispatch: loop {
        match pc {
            0x82663220 => {
    //   block [0x82663220..0x8266328C)
	// 82663220: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82663224: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82663228: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8266322C: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 82663230: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82663234: 38EB5B20  addi r7, r11, 0x5b20
	ctx.r[7].s64 = ctx.r[11].s64 + 23328;
	// 82663238: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 8266323C: 388AA130  addi r4, r10, -0x5ed0
	ctx.r[4].s64 = ctx.r[10].s64 + -24272;
	// 82663240: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82663244: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82663248: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8266324C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82663250: 386AC0D0  addi r3, r10, -0x3f30
	ctx.r[3].s64 = ctx.r[10].s64 + -16176;
	// 82663254: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82663258: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8266325C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82663260: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82663264: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82663268: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8266326C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82663270: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82663274: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82663278: 4BE03BA9  bl 0x82466e20
	ctx.lr = 0x8266327C;
	sub_82466E20(ctx, base);
	// 8266327C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82663280: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82663284: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82663288: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82663290(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82663290 size=108
    let mut pc: u32 = 0x82663290;
    'dispatch: loop {
        match pc {
            0x82663290 => {
    //   block [0x82663290..0x826632FC)
	// 82663290: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82663294: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82663298: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8266329C: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 826632A0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826632A4: 38EB5B50  addi r7, r11, 0x5b50
	ctx.r[7].s64 = ctx.r[11].s64 + 23376;
	// 826632A8: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826632AC: 388AA148  addi r4, r10, -0x5eb8
	ctx.r[4].s64 = ctx.r[10].s64 + -24248;
	// 826632B0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826632B4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 826632B8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826632BC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826632C0: 386AC100  addi r3, r10, -0x3f00
	ctx.r[3].s64 = ctx.r[10].s64 + -16128;
	// 826632C4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826632C8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826632CC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826632D0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826632D4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826632D8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826632DC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826632E0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826632E4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826632E8: 4BE03B39  bl 0x82466e20
	ctx.lr = 0x826632EC;
	sub_82466E20(ctx, base);
	// 826632EC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826632F0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826632F4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826632F8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82663300(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82663300 size=108
    let mut pc: u32 = 0x82663300;
    'dispatch: loop {
        match pc {
            0x82663300 => {
    //   block [0x82663300..0x8266336C)
	// 82663300: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82663304: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82663308: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8266330C: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 82663310: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82663314: 38EB5B68  addi r7, r11, 0x5b68
	ctx.r[7].s64 = ctx.r[11].s64 + 23400;
	// 82663318: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 8266331C: 388AB290  addi r4, r10, -0x4d70
	ctx.r[4].s64 = ctx.r[10].s64 + -19824;
	// 82663320: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82663324: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82663328: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8266332C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82663330: 386AC130  addi r3, r10, -0x3ed0
	ctx.r[3].s64 = ctx.r[10].s64 + -16080;
	// 82663334: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82663338: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8266333C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82663340: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82663344: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82663348: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8266334C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82663350: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82663354: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82663358: 4BE03AC9  bl 0x82466e20
	ctx.lr = 0x8266335C;
	sub_82466E20(ctx, base);
	// 8266335C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82663360: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82663364: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82663368: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82663370(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82663370 size=112
    let mut pc: u32 = 0x82663370;
    'dispatch: loop {
        match pc {
            0x82663370 => {
    //   block [0x82663370..0x826633E0)
	// 82663370: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82663374: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82663378: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8266337C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82663380: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 82663384: 38AABF80  addi r5, r10, -0x4080
	ctx.r[5].s64 = ctx.r[10].s64 + -16512;
	// 82663388: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8266338C: 390B5B98  addi r8, r11, 0x5b98
	ctx.r[8].s64 = ctx.r[11].s64 + 23448;
	// 82663390: 39200007  li r9, 7
	ctx.r[9].s64 = 7;
	// 82663394: 388AB2A8  addi r4, r10, -0x4d58
	ctx.r[4].s64 = ctx.r[10].s64 + -19800;
	// 82663398: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8266339C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 826633A0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826633A4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826633A8: 386AC160  addi r3, r10, -0x3ea0
	ctx.r[3].s64 = ctx.r[10].s64 + -16032;
	// 826633AC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826633B0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826633B4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826633B8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826633BC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826633C0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826633C4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826633C8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826633CC: 4BE03A55  bl 0x82466e20
	ctx.lr = 0x826633D0;
	sub_82466E20(ctx, base);
	// 826633D0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826633D4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826633D8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826633DC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826633E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826633E0 size=24
    let mut pc: u32 = 0x826633E0;
    'dispatch: loop {
        match pc {
            0x826633E0 => {
    //   block [0x826633E0..0x826633F8)
	// 826633E0: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 826633E4: 3D40827A  lis r10, -0x7d86
	ctx.r[10].s64 = -2105933824;
	// 826633E8: 394ABE28  addi r10, r10, -0x41d8
	ctx.r[10].s64 = ctx.r[10].s64 + -16856;
	// 826633EC: 816B5C40  lwz r11, 0x5c40(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(23616 as u32) ) } as u64;
	// 826633F0: 916A0008  stw r11, 8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 826633F4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826633F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826633F8 size=112
    let mut pc: u32 = 0x826633F8;
    'dispatch: loop {
        match pc {
            0x826633F8 => {
    //   block [0x826633F8..0x82663468)
	// 826633F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826633FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82663400: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82663404: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 82663408: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 8266340C: 392AF090  addi r9, r10, -0xf70
	ctx.r[9].s64 = ctx.r[10].s64 + -3952;
	// 82663410: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82663414: 390BBE28  addi r8, r11, -0x41d8
	ctx.r[8].s64 = ctx.r[11].s64 + -16856;
	// 82663418: 38E00005  li r7, 5
	ctx.r[7].s64 = 5;
	// 8266341C: 388AB098  addi r4, r10, -0x4f68
	ctx.r[4].s64 = ctx.r[10].s64 + -20328;
	// 82663420: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82663424: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82663428: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8266342C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82663430: 386AC190  addi r3, r10, -0x3e70
	ctx.r[3].s64 = ctx.r[10].s64 + -15984;
	// 82663434: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82663438: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8266343C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82663440: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82663444: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82663448: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8266344C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82663450: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82663454: 4BE039CD  bl 0x82466e20
	ctx.lr = 0x82663458;
	sub_82466E20(ctx, base);
	// 82663458: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8266345C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82663460: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82663464: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82663468(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82663468 size=108
    let mut pc: u32 = 0x82663468;
    'dispatch: loop {
        match pc {
            0x82663468 => {
    //   block [0x82663468..0x826634D4)
	// 82663468: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8266346C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82663470: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82663474: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 82663478: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8266347C: 38EB5C48  addi r7, r11, 0x5c48
	ctx.r[7].s64 = ctx.r[11].s64 + 23624;
	// 82663480: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 82663484: 388AAC70  addi r4, r10, -0x5390
	ctx.r[4].s64 = ctx.r[10].s64 + -21392;
	// 82663488: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8266348C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82663490: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82663494: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82663498: 386AC1C0  addi r3, r10, -0x3e40
	ctx.r[3].s64 = ctx.r[10].s64 + -15936;
	// 8266349C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826634A0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826634A4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826634A8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826634AC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826634B0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826634B4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826634B8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826634BC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826634C0: 4BE03961  bl 0x82466e20
	ctx.lr = 0x826634C4;
	sub_82466E20(ctx, base);
	// 826634C4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826634C8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826634CC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826634D0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826634D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826634D8 size=116
    let mut pc: u32 = 0x826634D8;
    'dispatch: loop {
        match pc {
            0x826634D8 => {
    //   block [0x826634D8..0x8266354C)
	// 826634D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826634DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826634E0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826634E4: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 826634E8: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 826634EC: 390B5C78  addi r8, r11, 0x5c78
	ctx.r[8].s64 = ctx.r[11].s64 + 23672;
	// 826634F0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826634F4: 392AF0D4  addi r9, r10, -0xf2c
	ctx.r[9].s64 = ctx.r[10].s64 + -3884;
	// 826634F8: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 826634FC: 38E00008  li r7, 8
	ctx.r[7].s64 = 8;
	// 82663500: 38AABF80  addi r5, r10, -0x4080
	ctx.r[5].s64 = ctx.r[10].s64 + -16512;
	// 82663504: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82663508: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8266350C: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82663510: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82663514: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82663518: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8266351C: 3D608296  lis r11, -0x7d6a
	ctx.r[11].s64 = -2104098816;
	// 82663520: 388AAC88  addi r4, r10, -0x5378
	ctx.r[4].s64 = ctx.r[10].s64 + -21368;
	// 82663524: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82663528: 386BC1F0  addi r3, r11, -0x3e10
	ctx.r[3].s64 = ctx.r[11].s64 + -15888;
	// 8266352C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82663530: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82663534: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82663538: 4BE038E9  bl 0x82466e20
	ctx.lr = 0x8266353C;
	sub_82466E20(ctx, base);
	// 8266353C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82663540: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82663544: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82663548: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82663550(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82663550 size=24
    let mut pc: u32 = 0x82663550;
    'dispatch: loop {
        match pc {
            0x82663550 => {
    //   block [0x82663550..0x82663568)
	// 82663550: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 82663554: 3D40827A  lis r10, -0x7d86
	ctx.r[10].s64 = -2105933824;
	// 82663558: 394ABEA0  addi r10, r10, -0x4160
	ctx.r[10].s64 = ctx.r[10].s64 + -16736;
	// 8266355C: 816B5D38  lwz r11, 0x5d38(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(23864 as u32) ) } as u64;
	// 82663560: 916A0020  stw r11, 0x20(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 82663564: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82663568(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82663568 size=112
    let mut pc: u32 = 0x82663568;
    'dispatch: loop {
        match pc {
            0x82663568 => {
    //   block [0x82663568..0x826635D8)
	// 82663568: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8266356C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82663570: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82663574: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 82663578: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 8266357C: 392AF110  addi r9, r10, -0xef0
	ctx.r[9].s64 = ctx.r[10].s64 + -3824;
	// 82663580: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82663584: 390BBEA0  addi r8, r11, -0x4160
	ctx.r[8].s64 = ctx.r[11].s64 + -16736;
	// 82663588: 38E00003  li r7, 3
	ctx.r[7].s64 = 3;
	// 8266358C: 388AB004  addi r4, r10, -0x4ffc
	ctx.r[4].s64 = ctx.r[10].s64 + -20476;
	// 82663590: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82663594: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82663598: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8266359C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826635A0: 386AC220  addi r3, r10, -0x3de0
	ctx.r[3].s64 = ctx.r[10].s64 + -15840;
	// 826635A4: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826635A8: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826635AC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826635B0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826635B4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826635B8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826635BC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826635C0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826635C4: 4BE0385D  bl 0x82466e20
	ctx.lr = 0x826635C8;
	sub_82466E20(ctx, base);
	// 826635C8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826635CC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826635D0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826635D4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826635D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826635D8 size=108
    let mut pc: u32 = 0x826635D8;
    'dispatch: loop {
        match pc {
            0x826635D8 => {
    //   block [0x826635D8..0x82663644)
	// 826635D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826635DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826635E0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826635E4: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 826635E8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826635EC: 38EB5D3C  addi r7, r11, 0x5d3c
	ctx.r[7].s64 = ctx.r[11].s64 + 23868;
	// 826635F0: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826635F4: 388AAF00  addi r4, r10, -0x5100
	ctx.r[4].s64 = ctx.r[10].s64 + -20736;
	// 826635F8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826635FC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82663600: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82663604: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82663608: 386AC250  addi r3, r10, -0x3db0
	ctx.r[3].s64 = ctx.r[10].s64 + -15792;
	// 8266360C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82663610: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82663614: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82663618: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8266361C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82663620: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82663624: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82663628: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8266362C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82663630: 4BE037F1  bl 0x82466e20
	ctx.lr = 0x82663634;
	sub_82466E20(ctx, base);
	// 82663634: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82663638: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8266363C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82663640: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82663648(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82663648 size=108
    let mut pc: u32 = 0x82663648;
    'dispatch: loop {
        match pc {
            0x82663648 => {
    //   block [0x82663648..0x826636B4)
	// 82663648: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8266364C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82663650: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82663654: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 82663658: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8266365C: 38EB5D54  addi r7, r11, 0x5d54
	ctx.r[7].s64 = ctx.r[11].s64 + 23892;
	// 82663660: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 82663664: 388A9D38  addi r4, r10, -0x62c8
	ctx.r[4].s64 = ctx.r[10].s64 + -25288;
	// 82663668: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8266366C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82663670: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82663674: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82663678: 386AC280  addi r3, r10, -0x3d80
	ctx.r[3].s64 = ctx.r[10].s64 + -15744;
	// 8266367C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82663680: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82663684: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82663688: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8266368C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82663690: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82663694: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82663698: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8266369C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826636A0: 4BE03781  bl 0x82466e20
	ctx.lr = 0x826636A4;
	sub_82466E20(ctx, base);
	// 826636A4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826636A8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826636AC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826636B0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826636B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826636B8 size=24
    let mut pc: u32 = 0x826636B8;
    'dispatch: loop {
        match pc {
            0x826636B8 => {
    //   block [0x826636B8..0x826636D0)
	// 826636B8: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 826636BC: 3D40827A  lis r10, -0x7d86
	ctx.r[10].s64 = -2105933824;
	// 826636C0: 394ABEE8  addi r10, r10, -0x4118
	ctx.r[10].s64 = ctx.r[10].s64 + -16664;
	// 826636C4: 816B5D84  lwz r11, 0x5d84(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(23940 as u32) ) } as u64;
	// 826636C8: 916A0008  stw r11, 8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 826636CC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826636D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826636D0 size=112
    let mut pc: u32 = 0x826636D0;
    'dispatch: loop {
        match pc {
            0x826636D0 => {
    //   block [0x826636D0..0x82663740)
	// 826636D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826636D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826636D8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826636DC: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 826636E0: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 826636E4: 392AF14C  addi r9, r10, -0xeb4
	ctx.r[9].s64 = ctx.r[10].s64 + -3764;
	// 826636E8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826636EC: 390BBEE8  addi r8, r11, -0x4118
	ctx.r[8].s64 = ctx.r[11].s64 + -16664;
	// 826636F0: 38E00005  li r7, 5
	ctx.r[7].s64 = 5;
	// 826636F4: 388AB450  addi r4, r10, -0x4bb0
	ctx.r[4].s64 = ctx.r[10].s64 + -19376;
	// 826636F8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826636FC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82663700: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82663704: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82663708: 386AC2B0  addi r3, r10, -0x3d50
	ctx.r[3].s64 = ctx.r[10].s64 + -15696;
	// 8266370C: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82663710: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82663714: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82663718: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8266371C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82663720: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82663724: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82663728: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8266372C: 4BE036F5  bl 0x82466e20
	ctx.lr = 0x82663730;
	sub_82466E20(ctx, base);
	// 82663730: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82663734: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82663738: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8266373C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82663740(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82663740 size=112
    let mut pc: u32 = 0x82663740;
    'dispatch: loop {
        match pc {
            0x82663740 => {
    //   block [0x82663740..0x826637B0)
	// 82663740: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82663744: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82663748: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8266374C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82663750: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 82663754: 38AABF80  addi r5, r10, -0x4080
	ctx.r[5].s64 = ctx.r[10].s64 + -16512;
	// 82663758: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8266375C: 390B5D88  addi r8, r11, 0x5d88
	ctx.r[8].s64 = ctx.r[11].s64 + 23944;
	// 82663760: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 82663764: 388A9CDC  addi r4, r10, -0x6324
	ctx.r[4].s64 = ctx.r[10].s64 + -25380;
	// 82663768: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8266376C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82663770: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82663774: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82663778: 386AC2E0  addi r3, r10, -0x3d20
	ctx.r[3].s64 = ctx.r[10].s64 + -15648;
	// 8266377C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82663780: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82663784: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82663788: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8266378C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82663790: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82663794: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82663798: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8266379C: 4BE03685  bl 0x82466e20
	ctx.lr = 0x826637A0;
	sub_82466E20(ctx, base);
	// 826637A0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826637A4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826637A8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826637AC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826637B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826637B0 size=108
    let mut pc: u32 = 0x826637B0;
    'dispatch: loop {
        match pc {
            0x826637B0 => {
    //   block [0x826637B0..0x8266381C)
	// 826637B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826637B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826637B8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826637BC: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 826637C0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826637C4: 38EB5DB8  addi r7, r11, 0x5db8
	ctx.r[7].s64 = ctx.r[11].s64 + 23992;
	// 826637C8: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826637CC: 388A9CF4  addi r4, r10, -0x630c
	ctx.r[4].s64 = ctx.r[10].s64 + -25356;
	// 826637D0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826637D4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 826637D8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826637DC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826637E0: 386AC310  addi r3, r10, -0x3cf0
	ctx.r[3].s64 = ctx.r[10].s64 + -15600;
	// 826637E4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826637E8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826637EC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826637F0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826637F4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826637F8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826637FC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82663800: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82663804: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82663808: 4BE03619  bl 0x82466e20
	ctx.lr = 0x8266380C;
	sub_82466E20(ctx, base);
	// 8266380C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82663810: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82663814: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82663818: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82663820(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82663820 size=108
    let mut pc: u32 = 0x82663820;
    'dispatch: loop {
        match pc {
            0x82663820 => {
    //   block [0x82663820..0x8266388C)
	// 82663820: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82663824: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82663828: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8266382C: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 82663830: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82663834: 38EB5DE8  addi r7, r11, 0x5de8
	ctx.r[7].s64 = ctx.r[11].s64 + 24040;
	// 82663838: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 8266383C: 388AB148  addi r4, r10, -0x4eb8
	ctx.r[4].s64 = ctx.r[10].s64 + -20152;
	// 82663840: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82663844: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82663848: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8266384C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82663850: 386AC340  addi r3, r10, -0x3cc0
	ctx.r[3].s64 = ctx.r[10].s64 + -15552;
	// 82663854: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82663858: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8266385C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82663860: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82663864: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82663868: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8266386C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82663870: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82663874: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82663878: 4BE035A9  bl 0x82466e20
	ctx.lr = 0x8266387C;
	sub_82466E20(ctx, base);
	// 8266387C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82663880: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82663884: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82663888: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82663890(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82663890 size=108
    let mut pc: u32 = 0x82663890;
    'dispatch: loop {
        match pc {
            0x82663890 => {
    //   block [0x82663890..0x826638FC)
	// 82663890: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82663894: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82663898: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8266389C: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 826638A0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826638A4: 38EB5E48  addi r7, r11, 0x5e48
	ctx.r[7].s64 = ctx.r[11].s64 + 24136;
	// 826638A8: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826638AC: 388AA9FC  addi r4, r10, -0x5604
	ctx.r[4].s64 = ctx.r[10].s64 + -22020;
	// 826638B0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826638B4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 826638B8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826638BC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826638C0: 386AC370  addi r3, r10, -0x3c90
	ctx.r[3].s64 = ctx.r[10].s64 + -15504;
	// 826638C4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826638C8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826638CC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826638D0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826638D4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826638D8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826638DC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826638E0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826638E4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826638E8: 4BE03539  bl 0x82466e20
	ctx.lr = 0x826638EC;
	sub_82466E20(ctx, base);
	// 826638EC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826638F0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826638F4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826638F8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82663900(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82663900 size=108
    let mut pc: u32 = 0x82663900;
    'dispatch: loop {
        match pc {
            0x82663900 => {
    //   block [0x82663900..0x8266396C)
	// 82663900: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82663904: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82663908: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8266390C: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 82663910: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82663914: 38EB5E78  addi r7, r11, 0x5e78
	ctx.r[7].s64 = ctx.r[11].s64 + 24184;
	// 82663918: 3900000C  li r8, 0xc
	ctx.r[8].s64 = 12;
	// 8266391C: 388AA978  addi r4, r10, -0x5688
	ctx.r[4].s64 = ctx.r[10].s64 + -22152;
	// 82663920: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82663924: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82663928: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8266392C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82663930: 386AC3A0  addi r3, r10, -0x3c60
	ctx.r[3].s64 = ctx.r[10].s64 + -15456;
	// 82663934: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82663938: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8266393C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82663940: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82663944: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82663948: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8266394C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82663950: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82663954: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82663958: 4BE034C9  bl 0x82466e20
	ctx.lr = 0x8266395C;
	sub_82466E20(ctx, base);
	// 8266395C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82663960: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82663964: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82663968: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82663970(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82663970 size=108
    let mut pc: u32 = 0x82663970;
    'dispatch: loop {
        match pc {
            0x82663970 => {
    //   block [0x82663970..0x826639DC)
	// 82663970: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82663974: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82663978: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8266397C: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 82663980: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82663984: 38EB5F98  addi r7, r11, 0x5f98
	ctx.r[7].s64 = ctx.r[11].s64 + 24472;
	// 82663988: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 8266398C: 388AA044  addi r4, r10, -0x5fbc
	ctx.r[4].s64 = ctx.r[10].s64 + -24508;
	// 82663990: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82663994: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82663998: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8266399C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826639A0: 386AC3D0  addi r3, r10, -0x3c30
	ctx.r[3].s64 = ctx.r[10].s64 + -15408;
	// 826639A4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826639A8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826639AC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826639B0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826639B4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826639B8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826639BC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826639C0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826639C4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826639C8: 4BE03459  bl 0x82466e20
	ctx.lr = 0x826639CC;
	sub_82466E20(ctx, base);
	// 826639CC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826639D0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826639D4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826639D8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826639E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826639E0 size=108
    let mut pc: u32 = 0x826639E0;
    'dispatch: loop {
        match pc {
            0x826639E0 => {
    //   block [0x826639E0..0x82663A4C)
	// 826639E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826639E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826639E8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826639EC: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 826639F0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826639F4: 38EB5FB0  addi r7, r11, 0x5fb0
	ctx.r[7].s64 = ctx.r[11].s64 + 24496;
	// 826639F8: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826639FC: 388AAF3C  addi r4, r10, -0x50c4
	ctx.r[4].s64 = ctx.r[10].s64 + -20676;
	// 82663A00: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82663A04: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82663A08: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82663A0C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82663A10: 386AC400  addi r3, r10, -0x3c00
	ctx.r[3].s64 = ctx.r[10].s64 + -15360;
	// 82663A14: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82663A18: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82663A1C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82663A20: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82663A24: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82663A28: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82663A2C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82663A30: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82663A34: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82663A38: 4BE033E9  bl 0x82466e20
	ctx.lr = 0x82663A3C;
	sub_82466E20(ctx, base);
	// 82663A3C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82663A40: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82663A44: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82663A48: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82663A50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82663A50 size=108
    let mut pc: u32 = 0x82663A50;
    'dispatch: loop {
        match pc {
            0x82663A50 => {
    //   block [0x82663A50..0x82663ABC)
	// 82663A50: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82663A54: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82663A58: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82663A5C: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 82663A60: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82663A64: 38EB5FC8  addi r7, r11, 0x5fc8
	ctx.r[7].s64 = ctx.r[11].s64 + 24520;
	// 82663A68: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 82663A6C: 388AA0C0  addi r4, r10, -0x5f40
	ctx.r[4].s64 = ctx.r[10].s64 + -24384;
	// 82663A70: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82663A74: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82663A78: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82663A7C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82663A80: 386AC430  addi r3, r10, -0x3bd0
	ctx.r[3].s64 = ctx.r[10].s64 + -15312;
	// 82663A84: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82663A88: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82663A8C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82663A90: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82663A94: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82663A98: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82663A9C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82663AA0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82663AA4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82663AA8: 4BE03379  bl 0x82466e20
	ctx.lr = 0x82663AAC;
	sub_82466E20(ctx, base);
	// 82663AAC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82663AB0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82663AB4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82663AB8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82663AC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82663AC0 size=108
    let mut pc: u32 = 0x82663AC0;
    'dispatch: loop {
        match pc {
            0x82663AC0 => {
    //   block [0x82663AC0..0x82663B2C)
	// 82663AC0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82663AC4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82663AC8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82663ACC: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 82663AD0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82663AD4: 38EB5FE0  addi r7, r11, 0x5fe0
	ctx.r[7].s64 = ctx.r[11].s64 + 24544;
	// 82663AD8: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 82663ADC: 388A9C24  addi r4, r10, -0x63dc
	ctx.r[4].s64 = ctx.r[10].s64 + -25564;
	// 82663AE0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82663AE4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82663AE8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82663AEC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82663AF0: 386AC460  addi r3, r10, -0x3ba0
	ctx.r[3].s64 = ctx.r[10].s64 + -15264;
	// 82663AF4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82663AF8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82663AFC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82663B00: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82663B04: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82663B08: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82663B0C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82663B10: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82663B14: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82663B18: 4BE03309  bl 0x82466e20
	ctx.lr = 0x82663B1C;
	sub_82466E20(ctx, base);
	// 82663B1C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82663B20: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82663B24: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82663B28: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82663B30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82663B30 size=108
    let mut pc: u32 = 0x82663B30;
    'dispatch: loop {
        match pc {
            0x82663B30 => {
    //   block [0x82663B30..0x82663B9C)
	// 82663B30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82663B34: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82663B38: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82663B3C: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 82663B40: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82663B44: 38EB5FF8  addi r7, r11, 0x5ff8
	ctx.r[7].s64 = ctx.r[11].s64 + 24568;
	// 82663B48: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 82663B4C: 388AA684  addi r4, r10, -0x597c
	ctx.r[4].s64 = ctx.r[10].s64 + -22908;
	// 82663B50: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82663B54: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82663B58: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82663B5C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82663B60: 386AC490  addi r3, r10, -0x3b70
	ctx.r[3].s64 = ctx.r[10].s64 + -15216;
	// 82663B64: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82663B68: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82663B6C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82663B70: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82663B74: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82663B78: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82663B7C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82663B80: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82663B84: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82663B88: 4BE03299  bl 0x82466e20
	ctx.lr = 0x82663B8C;
	sub_82466E20(ctx, base);
	// 82663B8C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82663B90: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82663B94: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82663B98: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82663BA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82663BA0 size=108
    let mut pc: u32 = 0x82663BA0;
    'dispatch: loop {
        match pc {
            0x82663BA0 => {
    //   block [0x82663BA0..0x82663C0C)
	// 82663BA0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82663BA4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82663BA8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82663BAC: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 82663BB0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82663BB4: 38EB6010  addi r7, r11, 0x6010
	ctx.r[7].s64 = ctx.r[11].s64 + 24592;
	// 82663BB8: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 82663BBC: 388AA26C  addi r4, r10, -0x5d94
	ctx.r[4].s64 = ctx.r[10].s64 + -23956;
	// 82663BC0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82663BC4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82663BC8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82663BCC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82663BD0: 386AC4C0  addi r3, r10, -0x3b40
	ctx.r[3].s64 = ctx.r[10].s64 + -15168;
	// 82663BD4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82663BD8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82663BDC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82663BE0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82663BE4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82663BE8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82663BEC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82663BF0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82663BF4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82663BF8: 4BE03229  bl 0x82466e20
	ctx.lr = 0x82663BFC;
	sub_82466E20(ctx, base);
	// 82663BFC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82663C00: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82663C04: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82663C08: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82663C10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82663C10 size=108
    let mut pc: u32 = 0x82663C10;
    'dispatch: loop {
        match pc {
            0x82663C10 => {
    //   block [0x82663C10..0x82663C7C)
	// 82663C10: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82663C14: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82663C18: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82663C1C: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 82663C20: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82663C24: 38EB6028  addi r7, r11, 0x6028
	ctx.r[7].s64 = ctx.r[11].s64 + 24616;
	// 82663C28: 39000006  li r8, 6
	ctx.r[8].s64 = 6;
	// 82663C2C: 388AB380  addi r4, r10, -0x4c80
	ctx.r[4].s64 = ctx.r[10].s64 + -19584;
	// 82663C30: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82663C34: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82663C38: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82663C3C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82663C40: 386AC4F0  addi r3, r10, -0x3b10
	ctx.r[3].s64 = ctx.r[10].s64 + -15120;
	// 82663C44: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82663C48: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82663C4C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82663C50: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82663C54: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82663C58: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82663C5C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82663C60: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82663C64: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82663C68: 4BE031B9  bl 0x82466e20
	ctx.lr = 0x82663C6C;
	sub_82466E20(ctx, base);
	// 82663C6C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82663C70: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82663C74: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82663C78: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82663C80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82663C80 size=108
    let mut pc: u32 = 0x82663C80;
    'dispatch: loop {
        match pc {
            0x82663C80 => {
    //   block [0x82663C80..0x82663CEC)
	// 82663C80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82663C84: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82663C88: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82663C8C: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 82663C90: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82663C94: 38EB60B8  addi r7, r11, 0x60b8
	ctx.r[7].s64 = ctx.r[11].s64 + 24760;
	// 82663C98: 39000008  li r8, 8
	ctx.r[8].s64 = 8;
	// 82663C9C: 388AA228  addi r4, r10, -0x5dd8
	ctx.r[4].s64 = ctx.r[10].s64 + -24024;
	// 82663CA0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82663CA4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82663CA8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82663CAC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82663CB0: 386AC520  addi r3, r10, -0x3ae0
	ctx.r[3].s64 = ctx.r[10].s64 + -15072;
	// 82663CB4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82663CB8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82663CBC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82663CC0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82663CC4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82663CC8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82663CCC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82663CD0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82663CD4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82663CD8: 4BE03149  bl 0x82466e20
	ctx.lr = 0x82663CDC;
	sub_82466E20(ctx, base);
	// 82663CDC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82663CE0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82663CE4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82663CE8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82663CF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82663CF0 size=108
    let mut pc: u32 = 0x82663CF0;
    'dispatch: loop {
        match pc {
            0x82663CF0 => {
    //   block [0x82663CF0..0x82663D5C)
	// 82663CF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82663CF4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82663CF8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82663CFC: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 82663D00: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82663D04: 38EB6178  addi r7, r11, 0x6178
	ctx.r[7].s64 = ctx.r[11].s64 + 24952;
	// 82663D08: 39000009  li r8, 9
	ctx.r[8].s64 = 9;
	// 82663D0C: 388AA000  addi r4, r10, -0x6000
	ctx.r[4].s64 = ctx.r[10].s64 + -24576;
	// 82663D10: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82663D14: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82663D18: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82663D1C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82663D20: 386AC550  addi r3, r10, -0x3ab0
	ctx.r[3].s64 = ctx.r[10].s64 + -15024;
	// 82663D24: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82663D28: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82663D2C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82663D30: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82663D34: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82663D38: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82663D3C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82663D40: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82663D44: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82663D48: 4BE030D9  bl 0x82466e20
	ctx.lr = 0x82663D4C;
	sub_82466E20(ctx, base);
	// 82663D4C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82663D50: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82663D54: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82663D58: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82663D60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82663D60 size=108
    let mut pc: u32 = 0x82663D60;
    'dispatch: loop {
        match pc {
            0x82663D60 => {
    //   block [0x82663D60..0x82663DCC)
	// 82663D60: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82663D64: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82663D68: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82663D6C: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 82663D70: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82663D74: 38EB6250  addi r7, r11, 0x6250
	ctx.r[7].s64 = ctx.r[11].s64 + 25168;
	// 82663D78: 39000008  li r8, 8
	ctx.r[8].s64 = 8;
	// 82663D7C: 388A9F00  addi r4, r10, -0x6100
	ctx.r[4].s64 = ctx.r[10].s64 + -24832;
	// 82663D80: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82663D84: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82663D88: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82663D8C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82663D90: 386AC580  addi r3, r10, -0x3a80
	ctx.r[3].s64 = ctx.r[10].s64 + -14976;
	// 82663D94: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82663D98: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82663D9C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82663DA0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82663DA4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82663DA8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82663DAC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82663DB0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82663DB4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82663DB8: 4BE03069  bl 0x82466e20
	ctx.lr = 0x82663DBC;
	sub_82466E20(ctx, base);
	// 82663DBC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82663DC0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82663DC4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82663DC8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82663DD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82663DD0 size=108
    let mut pc: u32 = 0x82663DD0;
    'dispatch: loop {
        match pc {
            0x82663DD0 => {
    //   block [0x82663DD0..0x82663E3C)
	// 82663DD0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82663DD4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82663DD8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82663DDC: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 82663DE0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82663DE4: 38EB6310  addi r7, r11, 0x6310
	ctx.r[7].s64 = ctx.r[11].s64 + 25360;
	// 82663DE8: 39000007  li r8, 7
	ctx.r[8].s64 = 7;
	// 82663DEC: 388AADE0  addi r4, r10, -0x5220
	ctx.r[4].s64 = ctx.r[10].s64 + -21024;
	// 82663DF0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82663DF4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82663DF8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82663DFC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82663E00: 386AC5B0  addi r3, r10, -0x3a50
	ctx.r[3].s64 = ctx.r[10].s64 + -14928;
	// 82663E04: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82663E08: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82663E0C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82663E10: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82663E14: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82663E18: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82663E1C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82663E20: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82663E24: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82663E28: 4BE02FF9  bl 0x82466e20
	ctx.lr = 0x82663E2C;
	sub_82466E20(ctx, base);
	// 82663E2C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82663E30: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82663E34: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82663E38: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82663E40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82663E40 size=112
    let mut pc: u32 = 0x82663E40;
    'dispatch: loop {
        match pc {
            0x82663E40 => {
    //   block [0x82663E40..0x82663EB0)
	// 82663E40: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82663E44: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82663E48: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82663E4C: 3D408279  lis r10, -0x7d87
	ctx.r[10].s64 = -2105999360;
	// 82663E50: 3900000C  li r8, 0xc
	ctx.r[8].s64 = 12;
	// 82663E54: 38EA63B8  addi r7, r10, 0x63b8
	ctx.r[7].s64 = ctx.r[10].s64 + 25528;
	// 82663E58: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82663E5C: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 82663E60: 388AA5FC  addi r4, r10, -0x5a04
	ctx.r[4].s64 = ctx.r[10].s64 + -23044;
	// 82663E64: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82663E68: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82663E6C: 396BF160  addi r11, r11, -0xea0
	ctx.r[11].s64 = ctx.r[11].s64 + -3744;
	// 82663E70: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82663E74: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82663E78: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82663E7C: 386AC5E0  addi r3, r10, -0x3a20
	ctx.r[3].s64 = ctx.r[10].s64 + -14880;
	// 82663E80: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82663E84: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 82663E88: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82663E8C: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 82663E90: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82663E94: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82663E98: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82663E9C: 4BE02F85  bl 0x82466e20
	ctx.lr = 0x82663EA0;
	sub_82466E20(ctx, base);
	// 82663EA0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82663EA4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82663EA8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82663EAC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82663EB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82663EB0 size=108
    let mut pc: u32 = 0x82663EB0;
    'dispatch: loop {
        match pc {
            0x82663EB0 => {
    //   block [0x82663EB0..0x82663F1C)
	// 82663EB0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82663EB4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82663EB8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82663EBC: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 82663EC0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82663EC4: 38EB64D8  addi r7, r11, 0x64d8
	ctx.r[7].s64 = ctx.r[11].s64 + 25816;
	// 82663EC8: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 82663ECC: 388AAD28  addi r4, r10, -0x52d8
	ctx.r[4].s64 = ctx.r[10].s64 + -21208;
	// 82663ED0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82663ED4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82663ED8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82663EDC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82663EE0: 386AC610  addi r3, r10, -0x39f0
	ctx.r[3].s64 = ctx.r[10].s64 + -14832;
	// 82663EE4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82663EE8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82663EEC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82663EF0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82663EF4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82663EF8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82663EFC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82663F00: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82663F04: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82663F08: 4BE02F19  bl 0x82466e20
	ctx.lr = 0x82663F0C;
	sub_82466E20(ctx, base);
	// 82663F0C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82663F10: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82663F14: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82663F18: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82663F20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82663F20 size=108
    let mut pc: u32 = 0x82663F20;
    'dispatch: loop {
        match pc {
            0x82663F20 => {
    //   block [0x82663F20..0x82663F8C)
	// 82663F20: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82663F24: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82663F28: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82663F2C: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 82663F30: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82663F34: 38EB6538  addi r7, r11, 0x6538
	ctx.r[7].s64 = ctx.r[11].s64 + 25912;
	// 82663F38: 3900000B  li r8, 0xb
	ctx.r[8].s64 = 11;
	// 82663F3C: 388A9070  addi r4, r10, -0x6f90
	ctx.r[4].s64 = ctx.r[10].s64 + -28560;
	// 82663F40: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82663F44: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82663F48: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82663F4C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82663F50: 386AC640  addi r3, r10, -0x39c0
	ctx.r[3].s64 = ctx.r[10].s64 + -14784;
	// 82663F54: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82663F58: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82663F5C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82663F60: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82663F64: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82663F68: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82663F6C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82663F70: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82663F74: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82663F78: 4BE02EA9  bl 0x82466e20
	ctx.lr = 0x82663F7C;
	sub_82466E20(ctx, base);
	// 82663F7C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82663F80: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82663F84: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82663F88: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82663F90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82663F90 size=108
    let mut pc: u32 = 0x82663F90;
    'dispatch: loop {
        match pc {
            0x82663F90 => {
    //   block [0x82663F90..0x82663FFC)
	// 82663F90: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82663F94: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82663F98: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82663F9C: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 82663FA0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82663FA4: 38EB6640  addi r7, r11, 0x6640
	ctx.r[7].s64 = ctx.r[11].s64 + 26176;
	// 82663FA8: 39000009  li r8, 9
	ctx.r[8].s64 = 9;
	// 82663FAC: 388A8E98  addi r4, r10, -0x7168
	ctx.r[4].s64 = ctx.r[10].s64 + -29032;
	// 82663FB0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82663FB4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82663FB8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82663FBC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82663FC0: 386AC670  addi r3, r10, -0x3990
	ctx.r[3].s64 = ctx.r[10].s64 + -14736;
	// 82663FC4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82663FC8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82663FCC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82663FD0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82663FD4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82663FD8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82663FDC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82663FE0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82663FE4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82663FE8: 4BE02E39  bl 0x82466e20
	ctx.lr = 0x82663FEC;
	sub_82466E20(ctx, base);
	// 82663FEC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82663FF0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82663FF4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82663FF8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82664000(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82664000 size=108
    let mut pc: u32 = 0x82664000;
    'dispatch: loop {
        match pc {
            0x82664000 => {
    //   block [0x82664000..0x8266406C)
	// 82664000: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82664004: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82664008: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8266400C: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 82664010: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82664014: 38EB6718  addi r7, r11, 0x6718
	ctx.r[7].s64 = ctx.r[11].s64 + 26392;
	// 82664018: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 8266401C: 388A8CF0  addi r4, r10, -0x7310
	ctx.r[4].s64 = ctx.r[10].s64 + -29456;
	// 82664020: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82664024: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82664028: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8266402C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82664030: 386AC6A0  addi r3, r10, -0x3960
	ctx.r[3].s64 = ctx.r[10].s64 + -14688;
	// 82664034: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82664038: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8266403C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82664040: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82664044: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82664048: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8266404C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82664050: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82664054: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82664058: 4BE02DC9  bl 0x82466e20
	ctx.lr = 0x8266405C;
	sub_82466E20(ctx, base);
	// 8266405C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82664060: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82664064: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82664068: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82664070(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82664070 size=108
    let mut pc: u32 = 0x82664070;
    'dispatch: loop {
        match pc {
            0x82664070 => {
    //   block [0x82664070..0x826640DC)
	// 82664070: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82664074: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82664078: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8266407C: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 82664080: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82664084: 38EB6760  addi r7, r11, 0x6760
	ctx.r[7].s64 = ctx.r[11].s64 + 26464;
	// 82664088: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 8266408C: 388A8D14  addi r4, r10, -0x72ec
	ctx.r[4].s64 = ctx.r[10].s64 + -29420;
	// 82664090: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82664094: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82664098: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8266409C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826640A0: 386AC6D0  addi r3, r10, -0x3930
	ctx.r[3].s64 = ctx.r[10].s64 + -14640;
	// 826640A4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826640A8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826640AC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826640B0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826640B4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826640B8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826640BC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826640C0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826640C4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826640C8: 4BE02D59  bl 0x82466e20
	ctx.lr = 0x826640CC;
	sub_82466E20(ctx, base);
	// 826640CC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826640D0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826640D4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826640D8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826640E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826640E0 size=112
    let mut pc: u32 = 0x826640E0;
    'dispatch: loop {
        match pc {
            0x826640E0 => {
    //   block [0x826640E0..0x82664150)
	// 826640E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826640E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826640E8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826640EC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 826640F0: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 826640F4: 38AABB60  addi r5, r10, -0x44a0
	ctx.r[5].s64 = ctx.r[10].s64 + -17568;
	// 826640F8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826640FC: 390B6778  addi r8, r11, 0x6778
	ctx.r[8].s64 = ctx.r[11].s64 + 26488;
	// 82664100: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 82664104: 388A3810  addi r4, r10, 0x3810
	ctx.r[4].s64 = ctx.r[10].s64 + 14352;
	// 82664108: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8266410C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82664110: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82664114: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82664118: 386AC700  addi r3, r10, -0x3900
	ctx.r[3].s64 = ctx.r[10].s64 + -14592;
	// 8266411C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82664120: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82664124: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82664128: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8266412C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82664130: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82664134: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82664138: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8266413C: 4BE02CE5  bl 0x82466e20
	ctx.lr = 0x82664140;
	sub_82466E20(ctx, base);
	// 82664140: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82664144: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82664148: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8266414C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82664150(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82664150 size=112
    let mut pc: u32 = 0x82664150;
    'dispatch: loop {
        match pc {
            0x82664150 => {
    //   block [0x82664150..0x826641C0)
	// 82664150: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82664154: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82664158: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8266415C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82664160: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 82664164: 38AABB60  addi r5, r10, -0x44a0
	ctx.r[5].s64 = ctx.r[10].s64 + -17568;
	// 82664168: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8266416C: 390B67D8  addi r8, r11, 0x67d8
	ctx.r[8].s64 = ctx.r[11].s64 + 26584;
	// 82664170: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 82664174: 388A3824  addi r4, r10, 0x3824
	ctx.r[4].s64 = ctx.r[10].s64 + 14372;
	// 82664178: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8266417C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82664180: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82664184: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82664188: 386AC730  addi r3, r10, -0x38d0
	ctx.r[3].s64 = ctx.r[10].s64 + -14544;
	// 8266418C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82664190: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82664194: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82664198: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8266419C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826641A0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826641A4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826641A8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826641AC: 4BE02C75  bl 0x82466e20
	ctx.lr = 0x826641B0;
	sub_82466E20(ctx, base);
	// 826641B0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826641B4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826641B8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826641BC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826641C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826641C0 size=108
    let mut pc: u32 = 0x826641C0;
    'dispatch: loop {
        match pc {
            0x826641C0 => {
    //   block [0x826641C0..0x8266422C)
	// 826641C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826641C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826641C8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826641CC: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 826641D0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826641D4: 38EB6820  addi r7, r11, 0x6820
	ctx.r[7].s64 = ctx.r[11].s64 + 26656;
	// 826641D8: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826641DC: 388A3838  addi r4, r10, 0x3838
	ctx.r[4].s64 = ctx.r[10].s64 + 14392;
	// 826641E0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826641E4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 826641E8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826641EC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826641F0: 386AC760  addi r3, r10, -0x38a0
	ctx.r[3].s64 = ctx.r[10].s64 + -14496;
	// 826641F4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826641F8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826641FC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82664200: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82664204: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82664208: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8266420C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82664210: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82664214: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82664218: 4BE02C09  bl 0x82466e20
	ctx.lr = 0x8266421C;
	sub_82466E20(ctx, base);
	// 8266421C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82664220: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82664224: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82664228: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82664230(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82664230 size=24
    let mut pc: u32 = 0x82664230;
    'dispatch: loop {
        match pc {
            0x82664230 => {
    //   block [0x82664230..0x82664248)
	// 82664230: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 82664234: 3D40827A  lis r10, -0x7d86
	ctx.r[10].s64 = -2105933824;
	// 82664238: 394ABF60  addi r10, r10, -0x40a0
	ctx.r[10].s64 = ctx.r[10].s64 + -16544;
	// 8266423C: 816B6D60  lwz r11, 0x6d60(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28000 as u32) ) } as u64;
	// 82664240: 916A0068  stw r11, 0x68(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(104 as u32), ctx.r[11].u32 ) };
	// 82664244: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82664248(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82664248 size=112
    let mut pc: u32 = 0x82664248;
    'dispatch: loop {
        match pc {
            0x82664248 => {
    //   block [0x82664248..0x826642B8)
	// 82664248: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8266424C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82664250: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82664254: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82664258: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 8266425C: 38AAC970  addi r5, r10, -0x3690
	ctx.r[5].s64 = ctx.r[10].s64 + -13968;
	// 82664260: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82664264: 390BBF60  addi r8, r11, -0x40a0
	ctx.r[8].s64 = ctx.r[11].s64 + -16544;
	// 82664268: 39200008  li r9, 8
	ctx.r[9].s64 = 8;
	// 8266426C: 388A3888  addi r4, r10, 0x3888
	ctx.r[4].s64 = ctx.r[10].s64 + 14472;
	// 82664270: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82664274: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82664278: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8266427C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82664280: 386AC790  addi r3, r10, -0x3870
	ctx.r[3].s64 = ctx.r[10].s64 + -14448;
	// 82664284: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82664288: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8266428C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82664290: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82664294: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82664298: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8266429C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826642A0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826642A4: 4BE02B7D  bl 0x82466e20
	ctx.lr = 0x826642A8;
	sub_82466E20(ctx, base);
	// 826642A8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826642AC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826642B0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826642B4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826642B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826642B8 size=108
    let mut pc: u32 = 0x826642B8;
    'dispatch: loop {
        match pc {
            0x826642B8 => {
    //   block [0x826642B8..0x82664324)
	// 826642B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826642BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826642C0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826642C4: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 826642C8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826642CC: 38EB6838  addi r7, r11, 0x6838
	ctx.r[7].s64 = ctx.r[11].s64 + 26680;
	// 826642D0: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 826642D4: 388A38A0  addi r4, r10, 0x38a0
	ctx.r[4].s64 = ctx.r[10].s64 + 14496;
	// 826642D8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826642DC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 826642E0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826642E4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826642E8: 386AC7C0  addi r3, r10, -0x3840
	ctx.r[3].s64 = ctx.r[10].s64 + -14400;
	// 826642EC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826642F0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826642F4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826642F8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826642FC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82664300: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82664304: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82664308: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8266430C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82664310: 4BE02B11  bl 0x82466e20
	ctx.lr = 0x82664314;
	sub_82466E20(ctx, base);
	// 82664314: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82664318: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8266431C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82664320: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82664328(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82664328 size=112
    let mut pc: u32 = 0x82664328;
    'dispatch: loop {
        match pc {
            0x82664328 => {
    //   block [0x82664328..0x82664398)
	// 82664328: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8266432C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82664330: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82664334: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82664338: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 8266433C: 38AAC970  addi r5, r10, -0x3690
	ctx.r[5].s64 = ctx.r[10].s64 + -13968;
	// 82664340: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82664344: 390B6898  addi r8, r11, 0x6898
	ctx.r[8].s64 = ctx.r[11].s64 + 26776;
	// 82664348: 39200008  li r9, 8
	ctx.r[9].s64 = 8;
	// 8266434C: 388A38BC  addi r4, r10, 0x38bc
	ctx.r[4].s64 = ctx.r[10].s64 + 14524;
	// 82664350: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82664354: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82664358: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8266435C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82664360: 386AC7F0  addi r3, r10, -0x3810
	ctx.r[3].s64 = ctx.r[10].s64 + -14352;
	// 82664364: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82664368: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8266436C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82664370: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82664374: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82664378: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8266437C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82664380: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82664384: 4BE02A9D  bl 0x82466e20
	ctx.lr = 0x82664388;
	sub_82466E20(ctx, base);
	// 82664388: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8266438C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82664390: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82664394: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82664398(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82664398 size=108
    let mut pc: u32 = 0x82664398;
    'dispatch: loop {
        match pc {
            0x82664398 => {
    //   block [0x82664398..0x82664404)
	// 82664398: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8266439C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826643A0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826643A4: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 826643A8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826643AC: 38EB6958  addi r7, r11, 0x6958
	ctx.r[7].s64 = ctx.r[11].s64 + 26968;
	// 826643B0: 39000005  li r8, 5
	ctx.r[8].s64 = 5;
	// 826643B4: 388A38D0  addi r4, r10, 0x38d0
	ctx.r[4].s64 = ctx.r[10].s64 + 14544;
	// 826643B8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826643BC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 826643C0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826643C4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826643C8: 386AC820  addi r3, r10, -0x37e0
	ctx.r[3].s64 = ctx.r[10].s64 + -14304;
	// 826643CC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826643D0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826643D4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826643D8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826643DC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826643E0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826643E4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826643E8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826643EC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826643F0: 4BE02A31  bl 0x82466e20
	ctx.lr = 0x826643F4;
	sub_82466E20(ctx, base);
	// 826643F4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826643F8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826643FC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82664400: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82664408(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82664408 size=108
    let mut pc: u32 = 0x82664408;
    'dispatch: loop {
        match pc {
            0x82664408 => {
    //   block [0x82664408..0x82664474)
	// 82664408: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8266440C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82664410: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82664414: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 82664418: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8266441C: 38EB69D0  addi r7, r11, 0x69d0
	ctx.r[7].s64 = ctx.r[11].s64 + 27088;
	// 82664420: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 82664424: 388A38E0  addi r4, r10, 0x38e0
	ctx.r[4].s64 = ctx.r[10].s64 + 14560;
	// 82664428: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8266442C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82664430: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82664434: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82664438: 386AC850  addi r3, r10, -0x37b0
	ctx.r[3].s64 = ctx.r[10].s64 + -14256;
	// 8266443C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82664440: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82664444: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82664448: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8266444C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82664450: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82664454: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82664458: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8266445C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82664460: 4BE029C1  bl 0x82466e20
	ctx.lr = 0x82664464;
	sub_82466E20(ctx, base);
	// 82664464: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82664468: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8266446C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82664470: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82664478(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82664478 size=108
    let mut pc: u32 = 0x82664478;
    'dispatch: loop {
        match pc {
            0x82664478 => {
    //   block [0x82664478..0x826644E4)
	// 82664478: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8266447C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82664480: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82664484: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 82664488: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8266448C: 38EB6A18  addi r7, r11, 0x6a18
	ctx.r[7].s64 = ctx.r[11].s64 + 27160;
	// 82664490: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 82664494: 388A3904  addi r4, r10, 0x3904
	ctx.r[4].s64 = ctx.r[10].s64 + 14596;
	// 82664498: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8266449C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 826644A0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826644A4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826644A8: 386AC880  addi r3, r10, -0x3780
	ctx.r[3].s64 = ctx.r[10].s64 + -14208;
	// 826644AC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826644B0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826644B4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826644B8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826644BC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826644C0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826644C4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826644C8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826644CC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826644D0: 4BE02951  bl 0x82466e20
	ctx.lr = 0x826644D4;
	sub_82466E20(ctx, base);
	// 826644D4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826644D8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826644DC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826644E0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826644E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826644E8 size=112
    let mut pc: u32 = 0x826644E8;
    'dispatch: loop {
        match pc {
            0x826644E8 => {
    //   block [0x826644E8..0x82664558)
	// 826644E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826644EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826644F0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826644F4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 826644F8: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 826644FC: 38AAC880  addi r5, r10, -0x3780
	ctx.r[5].s64 = ctx.r[10].s64 + -14208;
	// 82664500: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82664504: 390B6A60  addi r8, r11, 0x6a60
	ctx.r[8].s64 = ctx.r[11].s64 + 27232;
	// 82664508: 39200005  li r9, 5
	ctx.r[9].s64 = 5;
	// 8266450C: 388A3928  addi r4, r10, 0x3928
	ctx.r[4].s64 = ctx.r[10].s64 + 14632;
	// 82664510: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82664514: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82664518: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8266451C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82664520: 386AC8B0  addi r3, r10, -0x3750
	ctx.r[3].s64 = ctx.r[10].s64 + -14160;
	// 82664524: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82664528: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8266452C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82664530: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82664534: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82664538: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8266453C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82664540: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82664544: 4BE028DD  bl 0x82466e20
	ctx.lr = 0x82664548;
	sub_82466E20(ctx, base);
	// 82664548: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8266454C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82664550: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82664554: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82664558(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82664558 size=108
    let mut pc: u32 = 0x82664558;
    'dispatch: loop {
        match pc {
            0x82664558 => {
    //   block [0x82664558..0x826645C4)
	// 82664558: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8266455C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82664560: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82664564: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 82664568: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8266456C: 38EB6AD8  addi r7, r11, 0x6ad8
	ctx.r[7].s64 = ctx.r[11].s64 + 27352;
	// 82664570: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 82664574: 388A394C  addi r4, r10, 0x394c
	ctx.r[4].s64 = ctx.r[10].s64 + 14668;
	// 82664578: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8266457C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82664580: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82664584: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82664588: 386AC8E0  addi r3, r10, -0x3720
	ctx.r[3].s64 = ctx.r[10].s64 + -14112;
	// 8266458C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82664590: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82664594: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82664598: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8266459C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826645A0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826645A4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826645A8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826645AC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826645B0: 4BE02871  bl 0x82466e20
	ctx.lr = 0x826645B4;
	sub_82466E20(ctx, base);
	// 826645B4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826645B8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826645BC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826645C0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826645C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826645C8 size=108
    let mut pc: u32 = 0x826645C8;
    'dispatch: loop {
        match pc {
            0x826645C8 => {
    //   block [0x826645C8..0x82664634)
	// 826645C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826645CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826645D0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826645D4: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 826645D8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826645DC: 38EB6B20  addi r7, r11, 0x6b20
	ctx.r[7].s64 = ctx.r[11].s64 + 27424;
	// 826645E0: 39000008  li r8, 8
	ctx.r[8].s64 = 8;
	// 826645E4: 388A3968  addi r4, r10, 0x3968
	ctx.r[4].s64 = ctx.r[10].s64 + 14696;
	// 826645E8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826645EC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 826645F0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826645F4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826645F8: 386AC910  addi r3, r10, -0x36f0
	ctx.r[3].s64 = ctx.r[10].s64 + -14064;
	// 826645FC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82664600: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82664604: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82664608: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8266460C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82664610: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82664614: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82664618: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8266461C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82664620: 4BE02801  bl 0x82466e20
	ctx.lr = 0x82664624;
	sub_82466E20(ctx, base);
	// 82664624: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82664628: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8266462C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82664630: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82664638(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82664638 size=108
    let mut pc: u32 = 0x82664638;
    'dispatch: loop {
        match pc {
            0x82664638 => {
    //   block [0x82664638..0x826646A4)
	// 82664638: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8266463C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82664640: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82664644: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 82664648: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8266464C: 38EB6BE0  addi r7, r11, 0x6be0
	ctx.r[7].s64 = ctx.r[11].s64 + 27616;
	// 82664650: 39000010  li r8, 0x10
	ctx.r[8].s64 = 16;
	// 82664654: 388A3980  addi r4, r10, 0x3980
	ctx.r[4].s64 = ctx.r[10].s64 + 14720;
	// 82664658: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8266465C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82664660: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82664664: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82664668: 386AC940  addi r3, r10, -0x36c0
	ctx.r[3].s64 = ctx.r[10].s64 + -14016;
	// 8266466C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82664670: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82664674: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82664678: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8266467C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82664680: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82664684: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82664688: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8266468C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82664690: 4BE02791  bl 0x82466e20
	ctx.lr = 0x82664694;
	sub_82466E20(ctx, base);
	// 82664694: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82664698: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8266469C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826646A0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826646A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826646A8 size=112
    let mut pc: u32 = 0x826646A8;
    'dispatch: loop {
        match pc {
            0x826646A8 => {
    //   block [0x826646A8..0x82664718)
	// 826646A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826646AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826646B0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826646B4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 826646B8: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 826646BC: 38AABB60  addi r5, r10, -0x44a0
	ctx.r[5].s64 = ctx.r[10].s64 + -17568;
	// 826646C0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826646C4: 390B6D68  addi r8, r11, 0x6d68
	ctx.r[8].s64 = ctx.r[11].s64 + 28008;
	// 826646C8: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 826646CC: 388A3990  addi r4, r10, 0x3990
	ctx.r[4].s64 = ctx.r[10].s64 + 14736;
	// 826646D0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826646D4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 826646D8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826646DC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826646E0: 386AC970  addi r3, r10, -0x3690
	ctx.r[3].s64 = ctx.r[10].s64 + -13968;
	// 826646E4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826646E8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826646EC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826646F0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826646F4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826646F8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826646FC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82664700: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82664704: 4BE0271D  bl 0x82466e20
	ctx.lr = 0x82664708;
	sub_82466E20(ctx, base);
	// 82664708: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8266470C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82664710: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82664714: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82664718(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82664718 size=108
    let mut pc: u32 = 0x82664718;
    'dispatch: loop {
        match pc {
            0x82664718 => {
    //   block [0x82664718..0x82664784)
	// 82664718: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8266471C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82664720: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82664724: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 82664728: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8266472C: 38EB6DC8  addi r7, r11, 0x6dc8
	ctx.r[7].s64 = ctx.r[11].s64 + 28104;
	// 82664730: 39000005  li r8, 5
	ctx.r[8].s64 = 5;
	// 82664734: 388A39A0  addi r4, r10, 0x39a0
	ctx.r[4].s64 = ctx.r[10].s64 + 14752;
	// 82664738: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8266473C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82664740: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82664744: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82664748: 386AC9A0  addi r3, r10, -0x3660
	ctx.r[3].s64 = ctx.r[10].s64 + -13920;
	// 8266474C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82664750: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82664754: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82664758: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8266475C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82664760: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82664764: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82664768: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8266476C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82664770: 4BE026B1  bl 0x82466e20
	ctx.lr = 0x82664774;
	sub_82466E20(ctx, base);
	// 82664774: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82664778: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8266477C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82664780: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82664788(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82664788 size=112
    let mut pc: u32 = 0x82664788;
    'dispatch: loop {
        match pc {
            0x82664788 => {
    //   block [0x82664788..0x826647F8)
	// 82664788: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8266478C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82664790: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82664794: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82664798: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 8266479C: 38AABB60  addi r5, r10, -0x44a0
	ctx.r[5].s64 = ctx.r[10].s64 + -17568;
	// 826647A0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826647A4: 390B6E40  addi r8, r11, 0x6e40
	ctx.r[8].s64 = ctx.r[11].s64 + 28224;
	// 826647A8: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 826647AC: 388A39B4  addi r4, r10, 0x39b4
	ctx.r[4].s64 = ctx.r[10].s64 + 14772;
	// 826647B0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826647B4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 826647B8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826647BC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826647C0: 386AC9D0  addi r3, r10, -0x3630
	ctx.r[3].s64 = ctx.r[10].s64 + -13872;
	// 826647C4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826647C8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826647CC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826647D0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826647D4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826647D8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826647DC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826647E0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826647E4: 4BE0263D  bl 0x82466e20
	ctx.lr = 0x826647E8;
	sub_82466E20(ctx, base);
	// 826647E8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826647EC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826647F0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826647F4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826647F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826647F8 size=108
    let mut pc: u32 = 0x826647F8;
    'dispatch: loop {
        match pc {
            0x826647F8 => {
    //   block [0x826647F8..0x82664864)
	// 826647F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826647FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82664800: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82664804: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 82664808: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8266480C: 38EB6E88  addi r7, r11, 0x6e88
	ctx.r[7].s64 = ctx.r[11].s64 + 28296;
	// 82664810: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 82664814: 388A39C0  addi r4, r10, 0x39c0
	ctx.r[4].s64 = ctx.r[10].s64 + 14784;
	// 82664818: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8266481C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82664820: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82664824: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82664828: 386ACA00  addi r3, r10, -0x3600
	ctx.r[3].s64 = ctx.r[10].s64 + -13824;
	// 8266482C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82664830: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82664834: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82664838: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8266483C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82664840: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82664844: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82664848: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8266484C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82664850: 4BE025D1  bl 0x82466e20
	ctx.lr = 0x82664854;
	sub_82466E20(ctx, base);
	// 82664854: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82664858: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8266485C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82664860: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82664868(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82664868 size=108
    let mut pc: u32 = 0x82664868;
    'dispatch: loop {
        match pc {
            0x82664868 => {
    //   block [0x82664868..0x826648D4)
	// 82664868: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8266486C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82664870: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82664874: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 82664878: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8266487C: 38EB6EE8  addi r7, r11, 0x6ee8
	ctx.r[7].s64 = ctx.r[11].s64 + 28392;
	// 82664880: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 82664884: 388A0DBC  addi r4, r10, 0xdbc
	ctx.r[4].s64 = ctx.r[10].s64 + 3516;
	// 82664888: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8266488C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82664890: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82664894: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82664898: 386ACA30  addi r3, r10, -0x35d0
	ctx.r[3].s64 = ctx.r[10].s64 + -13776;
	// 8266489C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826648A0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826648A4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826648A8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826648AC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826648B0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826648B4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826648B8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826648BC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826648C0: 4BE02561  bl 0x82466e20
	ctx.lr = 0x826648C4;
	sub_82466E20(ctx, base);
	// 826648C4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826648C8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826648CC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826648D0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826648D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826648D8 size=108
    let mut pc: u32 = 0x826648D8;
    'dispatch: loop {
        match pc {
            0x826648D8 => {
    //   block [0x826648D8..0x82664944)
	// 826648D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826648DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826648E0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826648E4: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 826648E8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826648EC: 38EB6F30  addi r7, r11, 0x6f30
	ctx.r[7].s64 = ctx.r[11].s64 + 28464;
	// 826648F0: 39000008  li r8, 8
	ctx.r[8].s64 = 8;
	// 826648F4: 388A39D8  addi r4, r10, 0x39d8
	ctx.r[4].s64 = ctx.r[10].s64 + 14808;
	// 826648F8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826648FC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82664900: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82664904: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82664908: 386ACA60  addi r3, r10, -0x35a0
	ctx.r[3].s64 = ctx.r[10].s64 + -13728;
	// 8266490C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82664910: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82664914: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82664918: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8266491C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82664920: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82664924: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82664928: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8266492C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82664930: 4BE024F1  bl 0x82466e20
	ctx.lr = 0x82664934;
	sub_82466E20(ctx, base);
	// 82664934: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82664938: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8266493C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82664940: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82664948(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82664948 size=108
    let mut pc: u32 = 0x82664948;
    'dispatch: loop {
        match pc {
            0x82664948 => {
    //   block [0x82664948..0x826649B4)
	// 82664948: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8266494C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82664950: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82664954: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 82664958: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8266495C: 38EB6FF0  addi r7, r11, 0x6ff0
	ctx.r[7].s64 = ctx.r[11].s64 + 28656;
	// 82664960: 39000006  li r8, 6
	ctx.r[8].s64 = 6;
	// 82664964: 388A39F4  addi r4, r10, 0x39f4
	ctx.r[4].s64 = ctx.r[10].s64 + 14836;
	// 82664968: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8266496C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82664970: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82664974: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82664978: 386ACA90  addi r3, r10, -0x3570
	ctx.r[3].s64 = ctx.r[10].s64 + -13680;
	// 8266497C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82664980: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82664984: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82664988: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8266498C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82664990: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82664994: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82664998: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8266499C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826649A0: 4BE02481  bl 0x82466e20
	ctx.lr = 0x826649A4;
	sub_82466E20(ctx, base);
	// 826649A4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826649A8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826649AC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826649B0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826649B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826649B8 size=108
    let mut pc: u32 = 0x826649B8;
    'dispatch: loop {
        match pc {
            0x826649B8 => {
    //   block [0x826649B8..0x82664A24)
	// 826649B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826649BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826649C0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826649C4: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 826649C8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826649CC: 38EB7080  addi r7, r11, 0x7080
	ctx.r[7].s64 = ctx.r[11].s64 + 28800;
	// 826649D0: 3900000D  li r8, 0xd
	ctx.r[8].s64 = 13;
	// 826649D4: 388A3A14  addi r4, r10, 0x3a14
	ctx.r[4].s64 = ctx.r[10].s64 + 14868;
	// 826649D8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826649DC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 826649E0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826649E4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826649E8: 386ACAC0  addi r3, r10, -0x3540
	ctx.r[3].s64 = ctx.r[10].s64 + -13632;
	// 826649EC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826649F0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826649F4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826649F8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826649FC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82664A00: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82664A04: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82664A08: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82664A0C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82664A10: 4BE02411  bl 0x82466e20
	ctx.lr = 0x82664A14;
	sub_82466E20(ctx, base);
	// 82664A14: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82664A18: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82664A1C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82664A20: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82664A28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82664A28 size=108
    let mut pc: u32 = 0x82664A28;
    'dispatch: loop {
        match pc {
            0x82664A28 => {
    //   block [0x82664A28..0x82664A94)
	// 82664A28: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82664A2C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82664A30: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82664A34: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 82664A38: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82664A3C: 38EB71B8  addi r7, r11, 0x71b8
	ctx.r[7].s64 = ctx.r[11].s64 + 29112;
	// 82664A40: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 82664A44: 388A3A30  addi r4, r10, 0x3a30
	ctx.r[4].s64 = ctx.r[10].s64 + 14896;
	// 82664A48: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82664A4C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82664A50: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82664A54: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82664A58: 386ACAF0  addi r3, r10, -0x3510
	ctx.r[3].s64 = ctx.r[10].s64 + -13584;
	// 82664A5C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82664A60: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82664A64: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82664A68: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82664A6C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82664A70: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82664A74: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82664A78: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82664A7C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82664A80: 4BE023A1  bl 0x82466e20
	ctx.lr = 0x82664A84;
	sub_82466E20(ctx, base);
	// 82664A84: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82664A88: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82664A8C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82664A90: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82664A98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82664A98 size=116
    let mut pc: u32 = 0x82664A98;
    'dispatch: loop {
        match pc {
            0x82664A98 => {
    //   block [0x82664A98..0x82664B0C)
	// 82664A98: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82664A9C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82664AA0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82664AA4: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 82664AA8: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 82664AAC: 390B7218  addi r8, r11, 0x7218
	ctx.r[8].s64 = ctx.r[11].s64 + 29208;
	// 82664AB0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82664AB4: 392AF214  addi r9, r10, -0xdec
	ctx.r[9].s64 = ctx.r[10].s64 + -3564;
	// 82664AB8: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82664ABC: 38E00005  li r7, 5
	ctx.r[7].s64 = 5;
	// 82664AC0: 38AACAF0  addi r5, r10, -0x3510
	ctx.r[5].s64 = ctx.r[10].s64 + -13584;
	// 82664AC4: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82664AC8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82664ACC: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82664AD0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82664AD4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82664AD8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82664ADC: 3D608296  lis r11, -0x7d6a
	ctx.r[11].s64 = -2104098816;
	// 82664AE0: 388A3A3C  addi r4, r10, 0x3a3c
	ctx.r[4].s64 = ctx.r[10].s64 + 14908;
	// 82664AE4: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82664AE8: 386BCB20  addi r3, r11, -0x34e0
	ctx.r[3].s64 = ctx.r[11].s64 + -13536;
	// 82664AEC: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82664AF0: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82664AF4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82664AF8: 4BE02329  bl 0x82466e20
	ctx.lr = 0x82664AFC;
	sub_82466E20(ctx, base);
	// 82664AFC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82664B00: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82664B04: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82664B08: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82664B10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82664B10 size=96
    let mut pc: u32 = 0x82664B10;
    'dispatch: loop {
        match pc {
            0x82664B10 => {
    //   block [0x82664B10..0x82664B70)
	// 82664B10: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82664B14: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82664B18: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82664B1C: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82664B20: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82664B24: 388A3A4C  addi r4, r10, 0x3a4c
	ctx.r[4].s64 = ctx.r[10].s64 + 14924;
	// 82664B28: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82664B2C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82664B30: 386ACB50  addi r3, r10, -0x34b0
	ctx.r[3].s64 = ctx.r[10].s64 + -13488;
	// 82664B34: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82664B38: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82664B3C: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 82664B40: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82664B44: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82664B48: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82664B4C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82664B50: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82664B54: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82664B58: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82664B5C: 4BE022C5  bl 0x82466e20
	ctx.lr = 0x82664B60;
	sub_82466E20(ctx, base);
	// 82664B60: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82664B64: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82664B68: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82664B6C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82664B70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82664B70 size=112
    let mut pc: u32 = 0x82664B70;
    'dispatch: loop {
        match pc {
            0x82664B70 => {
    //   block [0x82664B70..0x82664BE0)
	// 82664B70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82664B74: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82664B78: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82664B7C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82664B80: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 82664B84: 38AAECE0  addi r5, r10, -0x1320
	ctx.r[5].s64 = ctx.r[10].s64 + -4896;
	// 82664B88: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82664B8C: 390B7290  addi r8, r11, 0x7290
	ctx.r[8].s64 = ctx.r[11].s64 + 29328;
	// 82664B90: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 82664B94: 388A3A64  addi r4, r10, 0x3a64
	ctx.r[4].s64 = ctx.r[10].s64 + 14948;
	// 82664B98: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82664B9C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82664BA0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82664BA4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82664BA8: 386ACB80  addi r3, r10, -0x3480
	ctx.r[3].s64 = ctx.r[10].s64 + -13440;
	// 82664BAC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82664BB0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82664BB4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82664BB8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82664BBC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82664BC0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82664BC4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82664BC8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82664BCC: 4BE02255  bl 0x82466e20
	ctx.lr = 0x82664BD0;
	sub_82466E20(ctx, base);
	// 82664BD0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82664BD4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82664BD8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82664BDC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82664BE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82664BE0 size=96
    let mut pc: u32 = 0x82664BE0;
    'dispatch: loop {
        match pc {
            0x82664BE0 => {
    //   block [0x82664BE0..0x82664C40)
	// 82664BE0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82664BE4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82664BE8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82664BEC: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82664BF0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82664BF4: 388A3A7C  addi r4, r10, 0x3a7c
	ctx.r[4].s64 = ctx.r[10].s64 + 14972;
	// 82664BF8: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82664BFC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82664C00: 386ACBB0  addi r3, r10, -0x3450
	ctx.r[3].s64 = ctx.r[10].s64 + -13392;
	// 82664C04: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82664C08: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82664C0C: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 82664C10: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82664C14: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82664C18: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82664C1C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82664C20: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82664C24: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82664C28: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82664C2C: 4BE021F5  bl 0x82466e20
	ctx.lr = 0x82664C30;
	sub_82466E20(ctx, base);
	// 82664C30: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82664C34: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82664C38: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82664C3C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82664C40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82664C40 size=24
    let mut pc: u32 = 0x82664C40;
    'dispatch: loop {
        match pc {
            0x82664C40 => {
    //   block [0x82664C40..0x82664C58)
	// 82664C40: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 82664C44: 3D40827A  lis r10, -0x7d86
	ctx.r[10].s64 = -2105933824;
	// 82664C48: 394AC020  addi r10, r10, -0x3fe0
	ctx.r[10].s64 = ctx.r[10].s64 + -16352;
	// 82664C4C: 816B72F0  lwz r11, 0x72f0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(29424 as u32) ) } as u64;
	// 82664C50: 916A0008  stw r11, 8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82664C54: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82664C58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82664C58 size=116
    let mut pc: u32 = 0x82664C58;
    'dispatch: loop {
        match pc {
            0x82664C58 => {
    //   block [0x82664C58..0x82664CCC)
	// 82664C58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82664C5C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82664C60: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82664C64: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 82664C68: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 82664C6C: 390BC020  addi r8, r11, -0x3fe0
	ctx.r[8].s64 = ctx.r[11].s64 + -16352;
	// 82664C70: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82664C74: 392AF250  addi r9, r10, -0xdb0
	ctx.r[9].s64 = ctx.r[10].s64 + -3504;
	// 82664C78: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82664C7C: 38E00002  li r7, 2
	ctx.r[7].s64 = 2;
	// 82664C80: 38AABB60  addi r5, r10, -0x44a0
	ctx.r[5].s64 = ctx.r[10].s64 + -17568;
	// 82664C84: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82664C88: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82664C8C: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82664C90: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82664C94: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 82664C98: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82664C9C: 3D608296  lis r11, -0x7d6a
	ctx.r[11].s64 = -2104098816;
	// 82664CA0: 388A3A9C  addi r4, r10, 0x3a9c
	ctx.r[4].s64 = ctx.r[10].s64 + 15004;
	// 82664CA4: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82664CA8: 386BCBE0  addi r3, r11, -0x3420
	ctx.r[3].s64 = ctx.r[11].s64 + -13344;
	// 82664CAC: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82664CB0: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82664CB4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82664CB8: 4BE02169  bl 0x82466e20
	ctx.lr = 0x82664CBC;
	sub_82466E20(ctx, base);
	// 82664CBC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82664CC0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82664CC4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82664CC8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82664CD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82664CD0 size=104
    let mut pc: u32 = 0x82664CD0;
    'dispatch: loop {
        match pc {
            0x82664CD0 => {
    //   block [0x82664CD0..0x82664D38)
	// 82664CD0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82664CD4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82664CD8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82664CDC: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 82664CE0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82664CE4: 392AF27C  addi r9, r10, -0xd84
	ctx.r[9].s64 = ctx.r[10].s64 + -3460;
	// 82664CE8: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82664CEC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82664CF0: 38AABB60  addi r5, r10, -0x44a0
	ctx.r[5].s64 = ctx.r[10].s64 + -17568;
	// 82664CF4: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82664CF8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82664CFC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82664D00: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82664D04: 388A3AB0  addi r4, r10, 0x3ab0
	ctx.r[4].s64 = ctx.r[10].s64 + 15024;
	// 82664D08: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82664D0C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82664D10: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82664D14: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82664D18: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82664D1C: 386ACC10  addi r3, r10, -0x33f0
	ctx.r[3].s64 = ctx.r[10].s64 + -13296;
	// 82664D20: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82664D24: 4BE020FD  bl 0x82466e20
	ctx.lr = 0x82664D28;
	sub_82466E20(ctx, base);
	// 82664D28: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82664D2C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82664D30: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82664D34: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82664D38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82664D38 size=96
    let mut pc: u32 = 0x82664D38;
    'dispatch: loop {
        match pc {
            0x82664D38 => {
    //   block [0x82664D38..0x82664D98)
	// 82664D38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82664D3C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82664D40: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82664D44: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82664D48: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82664D4C: 388A3AC4  addi r4, r10, 0x3ac4
	ctx.r[4].s64 = ctx.r[10].s64 + 15044;
	// 82664D50: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82664D54: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82664D58: 386ACC40  addi r3, r10, -0x33c0
	ctx.r[3].s64 = ctx.r[10].s64 + -13248;
	// 82664D5C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82664D60: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82664D64: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 82664D68: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82664D6C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82664D70: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82664D74: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82664D78: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82664D7C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82664D80: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82664D84: 4BE0209D  bl 0x82466e20
	ctx.lr = 0x82664D88;
	sub_82466E20(ctx, base);
	// 82664D88: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82664D8C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82664D90: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82664D94: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82664D98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82664D98 size=100
    let mut pc: u32 = 0x82664D98;
    'dispatch: loop {
        match pc {
            0x82664D98 => {
    //   block [0x82664D98..0x82664DFC)
	// 82664D98: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82664D9C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82664DA0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82664DA4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82664DA8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82664DAC: 38AACC10  addi r5, r10, -0x33f0
	ctx.r[5].s64 = ctx.r[10].s64 + -13296;
	// 82664DB0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82664DB4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82664DB8: 388A3ADC  addi r4, r10, 0x3adc
	ctx.r[4].s64 = ctx.r[10].s64 + 15068;
	// 82664DBC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82664DC0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82664DC4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82664DC8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82664DCC: 386ACC70  addi r3, r10, -0x3390
	ctx.r[3].s64 = ctx.r[10].s64 + -13200;
	// 82664DD0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82664DD4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82664DD8: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82664DDC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82664DE0: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82664DE4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82664DE8: 4BE02039  bl 0x82466e20
	ctx.lr = 0x82664DEC;
	sub_82466E20(ctx, base);
	// 82664DEC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82664DF0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82664DF4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82664DF8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82664E00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82664E00 size=112
    let mut pc: u32 = 0x82664E00;
    'dispatch: loop {
        match pc {
            0x82664E00 => {
    //   block [0x82664E00..0x82664E70)
	// 82664E00: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82664E04: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82664E08: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82664E0C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82664E10: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 82664E14: 38AACBE0  addi r5, r10, -0x3420
	ctx.r[5].s64 = ctx.r[10].s64 + -13344;
	// 82664E18: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82664E1C: 390B72F8  addi r8, r11, 0x72f8
	ctx.r[8].s64 = ctx.r[11].s64 + 29432;
	// 82664E20: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 82664E24: 388A3AF8  addi r4, r10, 0x3af8
	ctx.r[4].s64 = ctx.r[10].s64 + 15096;
	// 82664E28: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82664E2C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82664E30: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82664E34: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82664E38: 386ACCA0  addi r3, r10, -0x3360
	ctx.r[3].s64 = ctx.r[10].s64 + -13152;
	// 82664E3C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82664E40: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82664E44: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82664E48: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82664E4C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82664E50: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82664E54: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82664E58: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82664E5C: 4BE01FC5  bl 0x82466e20
	ctx.lr = 0x82664E60;
	sub_82466E20(ctx, base);
	// 82664E60: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82664E64: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82664E68: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82664E6C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82664E70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82664E70 size=112
    let mut pc: u32 = 0x82664E70;
    'dispatch: loop {
        match pc {
            0x82664E70 => {
    //   block [0x82664E70..0x82664EE0)
	// 82664E70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82664E74: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82664E78: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82664E7C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82664E80: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 82664E84: 38AACBE0  addi r5, r10, -0x3420
	ctx.r[5].s64 = ctx.r[10].s64 + -13344;
	// 82664E88: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82664E8C: 390B7340  addi r8, r11, 0x7340
	ctx.r[8].s64 = ctx.r[11].s64 + 29504;
	// 82664E90: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82664E94: 388A3B08  addi r4, r10, 0x3b08
	ctx.r[4].s64 = ctx.r[10].s64 + 15112;
	// 82664E98: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82664E9C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82664EA0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82664EA4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82664EA8: 386ACCD0  addi r3, r10, -0x3330
	ctx.r[3].s64 = ctx.r[10].s64 + -13104;
	// 82664EAC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82664EB0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82664EB4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82664EB8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82664EBC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82664EC0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82664EC4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82664EC8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82664ECC: 4BE01F55  bl 0x82466e20
	ctx.lr = 0x82664ED0;
	sub_82466E20(ctx, base);
	// 82664ED0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82664ED4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82664ED8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82664EDC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82664EE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82664EE0 size=100
    let mut pc: u32 = 0x82664EE0;
    'dispatch: loop {
        match pc {
            0x82664EE0 => {
    //   block [0x82664EE0..0x82664F44)
	// 82664EE0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82664EE4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82664EE8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82664EEC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82664EF0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82664EF4: 38AACBE0  addi r5, r10, -0x3420
	ctx.r[5].s64 = ctx.r[10].s64 + -13344;
	// 82664EF8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82664EFC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82664F00: 388A3B20  addi r4, r10, 0x3b20
	ctx.r[4].s64 = ctx.r[10].s64 + 15136;
	// 82664F04: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82664F08: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82664F0C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82664F10: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82664F14: 386ACD00  addi r3, r10, -0x3300
	ctx.r[3].s64 = ctx.r[10].s64 + -13056;
	// 82664F18: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82664F1C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82664F20: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82664F24: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82664F28: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82664F2C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82664F30: 4BE01EF1  bl 0x82466e20
	ctx.lr = 0x82664F34;
	sub_82466E20(ctx, base);
	// 82664F34: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82664F38: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82664F3C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82664F40: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82664F48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82664F48 size=96
    let mut pc: u32 = 0x82664F48;
    'dispatch: loop {
        match pc {
            0x82664F48 => {
    //   block [0x82664F48..0x82664FA8)
	// 82664F48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82664F4C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82664F50: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82664F54: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82664F58: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82664F5C: 388A3B38  addi r4, r10, 0x3b38
	ctx.r[4].s64 = ctx.r[10].s64 + 15160;
	// 82664F60: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82664F64: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82664F68: 386ACD30  addi r3, r10, -0x32d0
	ctx.r[3].s64 = ctx.r[10].s64 + -13008;
	// 82664F6C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82664F70: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82664F74: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 82664F78: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82664F7C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82664F80: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82664F84: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82664F88: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82664F8C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82664F90: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82664F94: 4BE01E8D  bl 0x82466e20
	ctx.lr = 0x82664F98;
	sub_82466E20(ctx, base);
	// 82664F98: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82664F9C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82664FA0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82664FA4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82664FA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82664FA8 size=112
    let mut pc: u32 = 0x82664FA8;
    'dispatch: loop {
        match pc {
            0x82664FA8 => {
    //   block [0x82664FA8..0x82665018)
	// 82664FA8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82664FAC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82664FB0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82664FB4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82664FB8: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 82664FBC: 38AABB60  addi r5, r10, -0x44a0
	ctx.r[5].s64 = ctx.r[10].s64 + -17568;
	// 82664FC0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82664FC4: 390B7358  addi r8, r11, 0x7358
	ctx.r[8].s64 = ctx.r[11].s64 + 29528;
	// 82664FC8: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 82664FCC: 388A3B54  addi r4, r10, 0x3b54
	ctx.r[4].s64 = ctx.r[10].s64 + 15188;
	// 82664FD0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82664FD4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82664FD8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82664FDC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82664FE0: 386ACD60  addi r3, r10, -0x32a0
	ctx.r[3].s64 = ctx.r[10].s64 + -12960;
	// 82664FE4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82664FE8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82664FEC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82664FF0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82664FF4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82664FF8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82664FFC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82665000: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82665004: 4BE01E1D  bl 0x82466e20
	ctx.lr = 0x82665008;
	sub_82466E20(ctx, base);
	// 82665008: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8266500C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82665010: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82665014: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82665018(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82665018 size=96
    let mut pc: u32 = 0x82665018;
    'dispatch: loop {
        match pc {
            0x82665018 => {
    //   block [0x82665018..0x82665078)
	// 82665018: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8266501C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82665020: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82665024: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82665028: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8266502C: 388A3B5C  addi r4, r10, 0x3b5c
	ctx.r[4].s64 = ctx.r[10].s64 + 15196;
	// 82665030: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82665034: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82665038: 386ACD90  addi r3, r10, -0x3270
	ctx.r[3].s64 = ctx.r[10].s64 + -12912;
	// 8266503C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82665040: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82665044: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 82665048: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8266504C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82665050: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82665054: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82665058: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8266505C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82665060: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82665064: 4BE01DBD  bl 0x82466e20
	ctx.lr = 0x82665068;
	sub_82466E20(ctx, base);
	// 82665068: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8266506C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82665070: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82665074: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82665078(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82665078 size=112
    let mut pc: u32 = 0x82665078;
    'dispatch: loop {
        match pc {
            0x82665078 => {
    //   block [0x82665078..0x826650E8)
	// 82665078: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8266507C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82665080: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82665084: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82665088: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 8266508C: 38AACD90  addi r5, r10, -0x3270
	ctx.r[5].s64 = ctx.r[10].s64 + -12912;
	// 82665090: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82665094: 390B7388  addi r8, r11, 0x7388
	ctx.r[8].s64 = ctx.r[11].s64 + 29576;
	// 82665098: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8266509C: 388A3B70  addi r4, r10, 0x3b70
	ctx.r[4].s64 = ctx.r[10].s64 + 15216;
	// 826650A0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826650A4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 826650A8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826650AC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826650B0: 386ACDC0  addi r3, r10, -0x3240
	ctx.r[3].s64 = ctx.r[10].s64 + -12864;
	// 826650B4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826650B8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826650BC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826650C0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826650C4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826650C8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826650CC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826650D0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826650D4: 4BE01D4D  bl 0x82466e20
	ctx.lr = 0x826650D8;
	sub_82466E20(ctx, base);
	// 826650D8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826650DC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826650E0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826650E4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826650E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826650E8 size=108
    let mut pc: u32 = 0x826650E8;
    'dispatch: loop {
        match pc {
            0x826650E8 => {
    //   block [0x826650E8..0x82665154)
	// 826650E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826650EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826650F0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826650F4: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 826650F8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826650FC: 38EB73A0  addi r7, r11, 0x73a0
	ctx.r[7].s64 = ctx.r[11].s64 + 29600;
	// 82665100: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 82665104: 388A3B88  addi r4, r10, 0x3b88
	ctx.r[4].s64 = ctx.r[10].s64 + 15240;
	// 82665108: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8266510C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82665110: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82665114: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82665118: 386ACDF0  addi r3, r10, -0x3210
	ctx.r[3].s64 = ctx.r[10].s64 + -12816;
	// 8266511C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82665120: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82665124: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82665128: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8266512C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82665130: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82665134: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82665138: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8266513C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82665140: 4BE01CE1  bl 0x82466e20
	ctx.lr = 0x82665144;
	sub_82466E20(ctx, base);
	// 82665144: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82665148: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8266514C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82665150: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82665158(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82665158 size=112
    let mut pc: u32 = 0x82665158;
    'dispatch: loop {
        match pc {
            0x82665158 => {
    //   block [0x82665158..0x826651C8)
	// 82665158: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8266515C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82665160: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82665164: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82665168: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 8266516C: 38AACF10  addi r5, r10, -0x30f0
	ctx.r[5].s64 = ctx.r[10].s64 + -12528;
	// 82665170: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82665174: 390B7400  addi r8, r11, 0x7400
	ctx.r[8].s64 = ctx.r[11].s64 + 29696;
	// 82665178: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8266517C: 388A3B9C  addi r4, r10, 0x3b9c
	ctx.r[4].s64 = ctx.r[10].s64 + 15260;
	// 82665180: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82665184: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82665188: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8266518C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82665190: 386ACE20  addi r3, r10, -0x31e0
	ctx.r[3].s64 = ctx.r[10].s64 + -12768;
	// 82665194: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82665198: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8266519C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826651A0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826651A4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826651A8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826651AC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826651B0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826651B4: 4BE01C6D  bl 0x82466e20
	ctx.lr = 0x826651B8;
	sub_82466E20(ctx, base);
	// 826651B8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826651BC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826651C0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826651C4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826651C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826651C8 size=112
    let mut pc: u32 = 0x826651C8;
    'dispatch: loop {
        match pc {
            0x826651C8 => {
    //   block [0x826651C8..0x82665238)
	// 826651C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826651CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826651D0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826651D4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 826651D8: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 826651DC: 38AACD60  addi r5, r10, -0x32a0
	ctx.r[5].s64 = ctx.r[10].s64 + -12960;
	// 826651E0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826651E4: 390B7418  addi r8, r11, 0x7418
	ctx.r[8].s64 = ctx.r[11].s64 + 29720;
	// 826651E8: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826651EC: 388A3BA8  addi r4, r10, 0x3ba8
	ctx.r[4].s64 = ctx.r[10].s64 + 15272;
	// 826651F0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826651F4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 826651F8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826651FC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82665200: 386ACE50  addi r3, r10, -0x31b0
	ctx.r[3].s64 = ctx.r[10].s64 + -12720;
	// 82665204: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82665208: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8266520C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82665210: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82665214: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82665218: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8266521C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82665220: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82665224: 4BE01BFD  bl 0x82466e20
	ctx.lr = 0x82665228;
	sub_82466E20(ctx, base);
	// 82665228: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8266522C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82665230: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82665234: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82665238(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82665238 size=100
    let mut pc: u32 = 0x82665238;
    'dispatch: loop {
        match pc {
            0x82665238 => {
    //   block [0x82665238..0x8266529C)
	// 82665238: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8266523C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82665240: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82665244: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82665248: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8266524C: 38AACD60  addi r5, r10, -0x32a0
	ctx.r[5].s64 = ctx.r[10].s64 + -12960;
	// 82665250: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82665254: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82665258: 388A3BB4  addi r4, r10, 0x3bb4
	ctx.r[4].s64 = ctx.r[10].s64 + 15284;
	// 8266525C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82665260: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82665264: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82665268: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8266526C: 386ACE80  addi r3, r10, -0x3180
	ctx.r[3].s64 = ctx.r[10].s64 + -12672;
	// 82665270: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82665274: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82665278: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8266527C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82665280: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82665284: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82665288: 4BE01B99  bl 0x82466e20
	ctx.lr = 0x8266528C;
	sub_82466E20(ctx, base);
	// 8266528C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82665290: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82665294: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82665298: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826652A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826652A0 size=116
    let mut pc: u32 = 0x826652A0;
    'dispatch: loop {
        match pc {
            0x826652A0 => {
    //   block [0x826652A0..0x82665314)
	// 826652A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826652A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826652A8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826652AC: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 826652B0: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 826652B4: 390B744C  addi r8, r11, 0x744c
	ctx.r[8].s64 = ctx.r[11].s64 + 29772;
	// 826652B8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826652BC: 392AF2A8  addi r9, r10, -0xd58
	ctx.r[9].s64 = ctx.r[10].s64 + -3416;
	// 826652C0: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 826652C4: 38E00002  li r7, 2
	ctx.r[7].s64 = 2;
	// 826652C8: 38AACF10  addi r5, r10, -0x30f0
	ctx.r[5].s64 = ctx.r[10].s64 + -12528;
	// 826652CC: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826652D0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826652D4: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826652D8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826652DC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826652E0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826652E4: 3D608296  lis r11, -0x7d6a
	ctx.r[11].s64 = -2104098816;
	// 826652E8: 388A3BC4  addi r4, r10, 0x3bc4
	ctx.r[4].s64 = ctx.r[10].s64 + 15300;
	// 826652EC: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826652F0: 386BCEB0  addi r3, r11, -0x3150
	ctx.r[3].s64 = ctx.r[11].s64 + -12624;
	// 826652F4: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826652F8: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826652FC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82665300: 4BE01B21  bl 0x82466e20
	ctx.lr = 0x82665304;
	sub_82466E20(ctx, base);
	// 82665304: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82665308: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8266530C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82665310: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82665318(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82665318 size=112
    let mut pc: u32 = 0x82665318;
    'dispatch: loop {
        match pc {
            0x82665318 => {
    //   block [0x82665318..0x82665388)
	// 82665318: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8266531C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82665320: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82665324: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82665328: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 8266532C: 38AACD60  addi r5, r10, -0x32a0
	ctx.r[5].s64 = ctx.r[10].s64 + -12960;
	// 82665330: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82665334: 390B747C  addi r8, r11, 0x747c
	ctx.r[8].s64 = ctx.r[11].s64 + 29820;
	// 82665338: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8266533C: 388A3BD4  addi r4, r10, 0x3bd4
	ctx.r[4].s64 = ctx.r[10].s64 + 15316;
	// 82665340: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82665344: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82665348: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8266534C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82665350: 386ACEE0  addi r3, r10, -0x3120
	ctx.r[3].s64 = ctx.r[10].s64 + -12576;
	// 82665354: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82665358: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8266535C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82665360: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82665364: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 82665368: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8266536C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82665370: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82665374: 4BE01AAD  bl 0x82466e20
	ctx.lr = 0x82665378;
	sub_82466E20(ctx, base);
	// 82665378: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8266537C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82665380: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82665384: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82665388(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82665388 size=116
    let mut pc: u32 = 0x82665388;
    'dispatch: loop {
        match pc {
            0x82665388 => {
    //   block [0x82665388..0x826653FC)
	// 82665388: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8266538C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82665390: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82665394: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 82665398: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 8266539C: 390B7498  addi r8, r11, 0x7498
	ctx.r[8].s64 = ctx.r[11].s64 + 29848;
	// 826653A0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826653A4: 392AF2D4  addi r9, r10, -0xd2c
	ctx.r[9].s64 = ctx.r[10].s64 + -3372;
	// 826653A8: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 826653AC: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
	// 826653B0: 38AAD570  addi r5, r10, -0x2a90
	ctx.r[5].s64 = ctx.r[10].s64 + -10896;
	// 826653B4: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826653B8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826653BC: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826653C0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826653C4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826653C8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826653CC: 3D608296  lis r11, -0x7d6a
	ctx.r[11].s64 = -2104098816;
	// 826653D0: 388A3BE8  addi r4, r10, 0x3be8
	ctx.r[4].s64 = ctx.r[10].s64 + 15336;
	// 826653D4: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826653D8: 386BCF10  addi r3, r11, -0x30f0
	ctx.r[3].s64 = ctx.r[11].s64 + -12528;
	// 826653DC: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826653E0: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826653E4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826653E8: 4BE01A39  bl 0x82466e20
	ctx.lr = 0x826653EC;
	sub_82466E20(ctx, base);
	// 826653EC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826653F0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826653F4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826653F8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82665400(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82665400 size=112
    let mut pc: u32 = 0x82665400;
    'dispatch: loop {
        match pc {
            0x82665400 => {
    //   block [0x82665400..0x82665470)
	// 82665400: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82665404: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82665408: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8266540C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82665410: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 82665414: 38AACF10  addi r5, r10, -0x30f0
	ctx.r[5].s64 = ctx.r[10].s64 + -12528;
	// 82665418: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8266541C: 390B74B0  addi r8, r11, 0x74b0
	ctx.r[8].s64 = ctx.r[11].s64 + 29872;
	// 82665420: 39200005  li r9, 5
	ctx.r[9].s64 = 5;
	// 82665424: 388A3BF8  addi r4, r10, 0x3bf8
	ctx.r[4].s64 = ctx.r[10].s64 + 15352;
	// 82665428: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8266542C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82665430: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82665434: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82665438: 386ACF40  addi r3, r10, -0x30c0
	ctx.r[3].s64 = ctx.r[10].s64 + -12480;
	// 8266543C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82665440: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82665444: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82665448: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8266544C: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 82665450: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82665454: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82665458: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8266545C: 4BE019C5  bl 0x82466e20
	ctx.lr = 0x82665460;
	sub_82466E20(ctx, base);
	// 82665460: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82665464: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82665468: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8266546C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82665470(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82665470 size=112
    let mut pc: u32 = 0x82665470;
    'dispatch: loop {
        match pc {
            0x82665470 => {
    //   block [0x82665470..0x826654E0)
	// 82665470: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82665474: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82665478: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8266547C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82665480: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 82665484: 38AACEE0  addi r5, r10, -0x3120
	ctx.r[5].s64 = ctx.r[10].s64 + -12576;
	// 82665488: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8266548C: 390B7528  addi r8, r11, 0x7528
	ctx.r[8].s64 = ctx.r[11].s64 + 29992;
	// 82665490: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 82665494: 388A3C0C  addi r4, r10, 0x3c0c
	ctx.r[4].s64 = ctx.r[10].s64 + 15372;
	// 82665498: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8266549C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 826654A0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826654A4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826654A8: 386ACF70  addi r3, r10, -0x3090
	ctx.r[3].s64 = ctx.r[10].s64 + -12432;
	// 826654AC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826654B0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826654B4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826654B8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826654BC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826654C0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826654C4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826654C8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826654CC: 4BE01955  bl 0x82466e20
	ctx.lr = 0x826654D0;
	sub_82466E20(ctx, base);
	// 826654D0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826654D4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826654D8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826654DC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826654E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826654E0 size=112
    let mut pc: u32 = 0x826654E0;
    'dispatch: loop {
        match pc {
            0x826654E0 => {
    //   block [0x826654E0..0x82665550)
	// 826654E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826654E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826654E8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826654EC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 826654F0: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 826654F4: 38AACF10  addi r5, r10, -0x30f0
	ctx.r[5].s64 = ctx.r[10].s64 + -12528;
	// 826654F8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826654FC: 390B7570  addi r8, r11, 0x7570
	ctx.r[8].s64 = ctx.r[11].s64 + 30064;
	// 82665500: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 82665504: 388A3C24  addi r4, r10, 0x3c24
	ctx.r[4].s64 = ctx.r[10].s64 + 15396;
	// 82665508: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8266550C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82665510: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82665514: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82665518: 386ACFA0  addi r3, r10, -0x3060
	ctx.r[3].s64 = ctx.r[10].s64 + -12384;
	// 8266551C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82665520: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82665524: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82665528: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8266552C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82665530: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82665534: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82665538: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8266553C: 4BE018E5  bl 0x82466e20
	ctx.lr = 0x82665540;
	sub_82466E20(ctx, base);
	// 82665540: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82665544: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82665548: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8266554C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82665550(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82665550 size=112
    let mut pc: u32 = 0x82665550;
    'dispatch: loop {
        match pc {
            0x82665550 => {
    //   block [0x82665550..0x826655C0)
	// 82665550: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82665554: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82665558: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8266555C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82665560: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 82665564: 38AACF10  addi r5, r10, -0x30f0
	ctx.r[5].s64 = ctx.r[10].s64 + -12528;
	// 82665568: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8266556C: 390B75B8  addi r8, r11, 0x75b8
	ctx.r[8].s64 = ctx.r[11].s64 + 30136;
	// 82665570: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 82665574: 388A3C3C  addi r4, r10, 0x3c3c
	ctx.r[4].s64 = ctx.r[10].s64 + 15420;
	// 82665578: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8266557C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82665580: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82665584: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82665588: 386ACFD0  addi r3, r10, -0x3030
	ctx.r[3].s64 = ctx.r[10].s64 + -12336;
	// 8266558C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82665590: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82665594: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82665598: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8266559C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826655A0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826655A4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826655A8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826655AC: 4BE01875  bl 0x82466e20
	ctx.lr = 0x826655B0;
	sub_82466E20(ctx, base);
	// 826655B0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826655B4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826655B8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826655BC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826655C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826655C0 size=108
    let mut pc: u32 = 0x826655C0;
    'dispatch: loop {
        match pc {
            0x826655C0 => {
    //   block [0x826655C0..0x8266562C)
	// 826655C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826655C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826655C8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826655CC: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 826655D0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826655D4: 38EB7600  addi r7, r11, 0x7600
	ctx.r[7].s64 = ctx.r[11].s64 + 30208;
	// 826655D8: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 826655DC: 388A3C54  addi r4, r10, 0x3c54
	ctx.r[4].s64 = ctx.r[10].s64 + 15444;
	// 826655E0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826655E4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 826655E8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826655EC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826655F0: 386AD000  addi r3, r10, -0x3000
	ctx.r[3].s64 = ctx.r[10].s64 + -12288;
	// 826655F4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826655F8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826655FC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82665600: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82665604: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82665608: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8266560C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82665610: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82665614: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82665618: 4BE01809  bl 0x82466e20
	ctx.lr = 0x8266561C;
	sub_82466E20(ctx, base);
	// 8266561C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82665620: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82665624: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82665628: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82665630(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82665630 size=112
    let mut pc: u32 = 0x82665630;
    'dispatch: loop {
        match pc {
            0x82665630 => {
    //   block [0x82665630..0x826656A0)
	// 82665630: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82665634: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82665638: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8266563C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82665640: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 82665644: 38AACF10  addi r5, r10, -0x30f0
	ctx.r[5].s64 = ctx.r[10].s64 + -12528;
	// 82665648: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8266564C: 390B7648  addi r8, r11, 0x7648
	ctx.r[8].s64 = ctx.r[11].s64 + 30280;
	// 82665650: 39200005  li r9, 5
	ctx.r[9].s64 = 5;
	// 82665654: 388A3C78  addi r4, r10, 0x3c78
	ctx.r[4].s64 = ctx.r[10].s64 + 15480;
	// 82665658: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8266565C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82665660: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82665664: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82665668: 386AD030  addi r3, r10, -0x2fd0
	ctx.r[3].s64 = ctx.r[10].s64 + -12240;
	// 8266566C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82665670: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82665674: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82665678: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8266567C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82665680: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82665684: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82665688: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8266568C: 4BE01795  bl 0x82466e20
	ctx.lr = 0x82665690;
	sub_82466E20(ctx, base);
	// 82665690: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82665694: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82665698: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8266569C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826656A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826656A0 size=116
    let mut pc: u32 = 0x826656A0;
    'dispatch: loop {
        match pc {
            0x826656A0 => {
    //   block [0x826656A0..0x82665714)
	// 826656A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826656A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826656A8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826656AC: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 826656B0: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 826656B4: 392BF310  addi r9, r11, -0xcf0
	ctx.r[9].s64 = ctx.r[11].s64 + -3312;
	// 826656B8: 38AACF10  addi r5, r10, -0x30f0
	ctx.r[5].s64 = ctx.r[10].s64 + -12528;
	// 826656BC: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826656C0: 38E90014  addi r7, r9, 0x14
	ctx.r[7].s64 = ctx.r[9].s64 + 20;
	// 826656C4: 38C00006  li r6, 6
	ctx.r[6].s64 = 6;
	// 826656C8: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 826656CC: 388A3C90  addi r4, r10, 0x3c90
	ctx.r[4].s64 = ctx.r[10].s64 + 15504;
	// 826656D0: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826656D4: 396B76C8  addi r11, r11, 0x76c8
	ctx.r[11].s64 = ctx.r[11].s64 + 30408;
	// 826656D8: 90E10064  stw r7, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[7].u32 ) };
	// 826656DC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 826656E0: 90C1005C  stw r6, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[6].u32 ) };
	// 826656E4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826656E8: 386AD060  addi r3, r10, -0x2fa0
	ctx.r[3].s64 = ctx.r[10].s64 + -12192;
	// 826656EC: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826656F0: 91010074  stw r8, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[8].u32 ) };
	// 826656F4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826656F8: 9101006C  stw r8, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[8].u32 ) };
	// 826656FC: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82665700: 4BE01721  bl 0x82466e20
	ctx.lr = 0x82665704;
	sub_82466E20(ctx, base);
	// 82665704: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82665708: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8266570C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82665710: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82665718(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82665718 size=36
    let mut pc: u32 = 0x82665718;
    'dispatch: loop {
        match pc {
            0x82665718 => {
    //   block [0x82665718..0x8266573C)
	// 82665718: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 8266571C: 814B775C  lwz r10, 0x775c(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(30556 as u32) ) } as u64;
	// 82665720: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 82665724: 396BC050  addi r11, r11, -0x3fb0
	ctx.r[11].s64 = ctx.r[11].s64 + -16304;
	// 82665728: 914B0008  stw r10, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 8266572C: 3D408279  lis r10, -0x7d87
	ctx.r[10].s64 = -2105999360;
	// 82665730: 814A76C4  lwz r10, 0x76c4(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(30404 as u32) ) } as u64;
	// 82665734: 914B0020  stw r10, 0x20(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(32 as u32), ctx.r[10].u32 ) };
	// 82665738: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82665740(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82665740 size=108
    let mut pc: u32 = 0x82665740;
    'dispatch: loop {
        match pc {
            0x82665740 => {
    //   block [0x82665740..0x826657AC)
	// 82665740: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82665744: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82665748: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8266574C: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 82665750: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82665754: 38EBC050  addi r7, r11, -0x3fb0
	ctx.r[7].s64 = ctx.r[11].s64 + -16304;
	// 82665758: 39000007  li r8, 7
	ctx.r[8].s64 = 7;
	// 8266575C: 388A3D1C  addi r4, r10, 0x3d1c
	ctx.r[4].s64 = ctx.r[10].s64 + 15644;
	// 82665760: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82665764: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82665768: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8266576C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82665770: 386AD090  addi r3, r10, -0x2f70
	ctx.r[3].s64 = ctx.r[10].s64 + -12144;
	// 82665774: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82665778: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8266577C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82665780: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82665784: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82665788: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8266578C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82665790: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82665794: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82665798: 4BE01689  bl 0x82466e20
	ctx.lr = 0x8266579C;
	sub_82466E20(ctx, base);
	// 8266579C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826657A0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826657A4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826657A8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826657B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826657B0 size=24
    let mut pc: u32 = 0x826657B0;
    'dispatch: loop {
        match pc {
            0x826657B0 => {
    //   block [0x826657B0..0x826657C8)
	// 826657B0: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 826657B4: 3D40827A  lis r10, -0x7d86
	ctx.r[10].s64 = -2105933824;
	// 826657B8: 394AC0F8  addi r10, r10, -0x3f08
	ctx.r[10].s64 = ctx.r[10].s64 + -16136;
	// 826657BC: 816B76C4  lwz r11, 0x76c4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(30404 as u32) ) } as u64;
	// 826657C0: 916A00B0  stw r11, 0xb0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(176 as u32), ctx.r[11].u32 ) };
	// 826657C4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826657C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826657C8 size=116
    let mut pc: u32 = 0x826657C8;
    'dispatch: loop {
        match pc {
            0x826657C8 => {
    //   block [0x826657C8..0x8266583C)
	// 826657C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826657CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826657D0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826657D4: 3D40827A  lis r10, -0x7d86
	ctx.r[10].s64 = -2105933824;
	// 826657D8: 38E0000A  li r7, 0xa
	ctx.r[7].s64 = 10;
	// 826657DC: 390AC0F8  addi r8, r10, -0x3f08
	ctx.r[8].s64 = ctx.r[10].s64 + -16136;
	// 826657E0: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 826657E4: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 826657E8: 38AAD090  addi r5, r10, -0x2f70
	ctx.r[5].s64 = ctx.r[10].s64 + -12144;
	// 826657EC: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826657F0: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826657F4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826657F8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826657FC: 388A3D58  addi r4, r10, 0x3d58
	ctx.r[4].s64 = ctx.r[10].s64 + 15704;
	// 82665800: 396BF3CC  addi r11, r11, -0xc34
	ctx.r[11].s64 = ctx.r[11].s64 + -3124;
	// 82665804: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82665808: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8266580C: 386AD0C0  addi r3, r10, -0x2f40
	ctx.r[3].s64 = ctx.r[10].s64 + -12096;
	// 82665810: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 82665814: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82665818: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 8266581C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82665820: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82665824: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82665828: 4BE015F9  bl 0x82466e20
	ctx.lr = 0x8266582C;
	sub_82466E20(ctx, base);
	// 8266582C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82665830: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82665834: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82665838: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82665840(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82665840 size=112
    let mut pc: u32 = 0x82665840;
    'dispatch: loop {
        match pc {
            0x82665840 => {
    //   block [0x82665840..0x826658B0)
	// 82665840: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82665844: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82665848: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8266584C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82665850: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 82665854: 38AAD090  addi r5, r10, -0x2f70
	ctx.r[5].s64 = ctx.r[10].s64 + -12144;
	// 82665858: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8266585C: 390B7760  addi r8, r11, 0x7760
	ctx.r[8].s64 = ctx.r[11].s64 + 30560;
	// 82665860: 39200005  li r9, 5
	ctx.r[9].s64 = 5;
	// 82665864: 388A3D7C  addi r4, r10, 0x3d7c
	ctx.r[4].s64 = ctx.r[10].s64 + 15740;
	// 82665868: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8266586C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82665870: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82665874: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82665878: 386AD0F0  addi r3, r10, -0x2f10
	ctx.r[3].s64 = ctx.r[10].s64 + -12048;
	// 8266587C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82665880: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82665884: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82665888: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8266588C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82665890: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82665894: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82665898: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8266589C: 4BE01585  bl 0x82466e20
	ctx.lr = 0x826658A0;
	sub_82466E20(ctx, base);
	// 826658A0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826658A4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826658A8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826658AC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826658B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826658B0 size=24
    let mut pc: u32 = 0x826658B0;
    'dispatch: loop {
        match pc {
            0x826658B0 => {
    //   block [0x826658B0..0x826658C8)
	// 826658B0: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 826658B4: 3D40827A  lis r10, -0x7d86
	ctx.r[10].s64 = -2105933824;
	// 826658B8: 394AC1E8  addi r10, r10, -0x3e18
	ctx.r[10].s64 = ctx.r[10].s64 + -15896;
	// 826658BC: 816B7F18  lwz r11, 0x7f18(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(32536 as u32) ) } as u64;
	// 826658C0: 916A00B0  stw r11, 0xb0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(176 as u32), ctx.r[11].u32 ) };
	// 826658C4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826658C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826658C8 size=116
    let mut pc: u32 = 0x826658C8;
    'dispatch: loop {
        match pc {
            0x826658C8 => {
    //   block [0x826658C8..0x8266593C)
	// 826658C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826658CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826658D0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826658D4: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 826658D8: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 826658DC: 392BF390  addi r9, r11, -0xc70
	ctx.r[9].s64 = ctx.r[11].s64 + -3184;
	// 826658E0: 38AACEE0  addi r5, r10, -0x3120
	ctx.r[5].s64 = ctx.r[10].s64 + -12576;
	// 826658E4: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826658E8: 38E90068  addi r7, r9, 0x68
	ctx.r[7].s64 = ctx.r[9].s64 + 104;
	// 826658EC: 38C0000A  li r6, 0xa
	ctx.r[6].s64 = 10;
	// 826658F0: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 826658F4: 388A3DA0  addi r4, r10, 0x3da0
	ctx.r[4].s64 = ctx.r[10].s64 + 15776;
	// 826658F8: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826658FC: 396BC1E8  addi r11, r11, -0x3e18
	ctx.r[11].s64 = ctx.r[11].s64 + -15896;
	// 82665900: 90E10064  stw r7, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[7].u32 ) };
	// 82665904: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82665908: 90C1005C  stw r6, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[6].u32 ) };
	// 8266590C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82665910: 386AD120  addi r3, r10, -0x2ee0
	ctx.r[3].s64 = ctx.r[10].s64 + -12000;
	// 82665914: 39400003  li r10, 3
	ctx.r[10].s64 = 3;
	// 82665918: 91010074  stw r8, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[8].u32 ) };
	// 8266591C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82665920: 9101006C  stw r8, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[8].u32 ) };
	// 82665924: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82665928: 4BE014F9  bl 0x82466e20
	ctx.lr = 0x8266592C;
	sub_82466E20(ctx, base);
	// 8266592C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82665930: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82665934: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82665938: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82665940(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82665940 size=100
    let mut pc: u32 = 0x82665940;
    'dispatch: loop {
        match pc {
            0x82665940 => {
    //   block [0x82665940..0x826659A4)
	// 82665940: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82665944: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82665948: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8266594C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82665950: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82665954: 38AAD270  addi r5, r10, -0x2d90
	ctx.r[5].s64 = ctx.r[10].s64 + -11664;
	// 82665958: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8266595C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82665960: 388A3DB4  addi r4, r10, 0x3db4
	ctx.r[4].s64 = ctx.r[10].s64 + 15796;
	// 82665964: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82665968: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8266596C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82665970: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82665974: 386AD150  addi r3, r10, -0x2eb0
	ctx.r[3].s64 = ctx.r[10].s64 + -11952;
	// 82665978: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8266597C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82665980: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82665984: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82665988: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8266598C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82665990: 4BE01491  bl 0x82466e20
	ctx.lr = 0x82665994;
	sub_82466E20(ctx, base);
	// 82665994: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82665998: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8266599C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826659A0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826659A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826659A8 size=100
    let mut pc: u32 = 0x826659A8;
    'dispatch: loop {
        match pc {
            0x826659A8 => {
    //   block [0x826659A8..0x82665A0C)
	// 826659A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826659AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826659B0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826659B4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 826659B8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826659BC: 38AACD60  addi r5, r10, -0x32a0
	ctx.r[5].s64 = ctx.r[10].s64 + -12960;
	// 826659C0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826659C4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826659C8: 388A3DC4  addi r4, r10, 0x3dc4
	ctx.r[4].s64 = ctx.r[10].s64 + 15812;
	// 826659CC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 826659D0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826659D4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826659D8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826659DC: 386AD180  addi r3, r10, -0x2e80
	ctx.r[3].s64 = ctx.r[10].s64 + -11904;
	// 826659E0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826659E4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826659E8: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826659EC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826659F0: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826659F4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826659F8: 4BE01429  bl 0x82466e20
	ctx.lr = 0x826659FC;
	sub_82466E20(ctx, base);
	// 826659FC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82665A00: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82665A04: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82665A08: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82665A10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82665A10 size=108
    let mut pc: u32 = 0x82665A10;
    'dispatch: loop {
        match pc {
            0x82665A10 => {
    //   block [0x82665A10..0x82665A7C)
	// 82665A10: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82665A14: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82665A18: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82665A1C: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 82665A20: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82665A24: 38EB77D8  addi r7, r11, 0x77d8
	ctx.r[7].s64 = ctx.r[11].s64 + 30680;
	// 82665A28: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 82665A2C: 388A3DD8  addi r4, r10, 0x3dd8
	ctx.r[4].s64 = ctx.r[10].s64 + 15832;
	// 82665A30: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82665A34: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82665A38: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82665A3C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82665A40: 386AD1B0  addi r3, r10, -0x2e50
	ctx.r[3].s64 = ctx.r[10].s64 + -11856;
	// 82665A44: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82665A48: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82665A4C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82665A50: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82665A54: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82665A58: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82665A5C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82665A60: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82665A64: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82665A68: 4BE013B9  bl 0x82466e20
	ctx.lr = 0x82665A6C;
	sub_82466E20(ctx, base);
	// 82665A6C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82665A70: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82665A74: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82665A78: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82665A80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82665A80 size=112
    let mut pc: u32 = 0x82665A80;
    'dispatch: loop {
        match pc {
            0x82665A80 => {
    //   block [0x82665A80..0x82665AF0)
	// 82665A80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82665A84: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82665A88: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82665A8C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82665A90: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 82665A94: 38AACEE0  addi r5, r10, -0x3120
	ctx.r[5].s64 = ctx.r[10].s64 + -12576;
	// 82665A98: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82665A9C: 390B7838  addi r8, r11, 0x7838
	ctx.r[8].s64 = ctx.r[11].s64 + 30776;
	// 82665AA0: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 82665AA4: 388A3DF0  addi r4, r10, 0x3df0
	ctx.r[4].s64 = ctx.r[10].s64 + 15856;
	// 82665AA8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82665AAC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82665AB0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82665AB4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82665AB8: 386AD1E0  addi r3, r10, -0x2e20
	ctx.r[3].s64 = ctx.r[10].s64 + -11808;
	// 82665ABC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82665AC0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82665AC4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82665AC8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82665ACC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82665AD0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82665AD4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82665AD8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82665ADC: 4BE01345  bl 0x82466e20
	ctx.lr = 0x82665AE0;
	sub_82466E20(ctx, base);
	// 82665AE0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82665AE4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82665AE8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82665AEC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82665AF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82665AF0 size=108
    let mut pc: u32 = 0x82665AF0;
    'dispatch: loop {
        match pc {
            0x82665AF0 => {
    //   block [0x82665AF0..0x82665B5C)
	// 82665AF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82665AF4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82665AF8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82665AFC: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 82665B00: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82665B04: 38EB7898  addi r7, r11, 0x7898
	ctx.r[7].s64 = ctx.r[11].s64 + 30872;
	// 82665B08: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 82665B0C: 388A3DFC  addi r4, r10, 0x3dfc
	ctx.r[4].s64 = ctx.r[10].s64 + 15868;
	// 82665B10: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82665B14: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82665B18: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82665B1C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82665B20: 386AD210  addi r3, r10, -0x2df0
	ctx.r[3].s64 = ctx.r[10].s64 + -11760;
	// 82665B24: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82665B28: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82665B2C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82665B30: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82665B34: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82665B38: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82665B3C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82665B40: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82665B44: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82665B48: 4BE012D9  bl 0x82466e20
	ctx.lr = 0x82665B4C;
	sub_82466E20(ctx, base);
	// 82665B4C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82665B50: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82665B54: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82665B58: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82665B60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82665B60 size=28
    let mut pc: u32 = 0x82665B60;
    'dispatch: loop {
        match pc {
            0x82665B60 => {
    //   block [0x82665B60..0x82665B7C)
	// 82665B60: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 82665B64: 3D40827A  lis r10, -0x7d86
	ctx.r[10].s64 = -2105933824;
	// 82665B68: 394AC2D8  addi r10, r10, -0x3d28
	ctx.r[10].s64 = ctx.r[10].s64 + -15656;
	// 82665B6C: 816B78B0  lwz r11, 0x78b0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(30896 as u32) ) } as u64;
	// 82665B70: 916A0068  stw r11, 0x68(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(104 as u32), ctx.r[11].u32 ) };
	// 82665B74: 916A0080  stw r11, 0x80(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(128 as u32), ctx.r[11].u32 ) };
	// 82665B78: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82665B80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82665B80 size=112
    let mut pc: u32 = 0x82665B80;
    'dispatch: loop {
        match pc {
            0x82665B80 => {
    //   block [0x82665B80..0x82665BF0)
	// 82665B80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82665B84: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82665B88: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82665B8C: 3D40827A  lis r10, -0x7d86
	ctx.r[10].s64 = -2105933824;
	// 82665B90: 3900000F  li r8, 0xf
	ctx.r[8].s64 = 15;
	// 82665B94: 38EAC2D8  addi r7, r10, -0x3d28
	ctx.r[7].s64 = ctx.r[10].s64 + -15656;
	// 82665B98: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82665B9C: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 82665BA0: 388A3E1C  addi r4, r10, 0x3e1c
	ctx.r[4].s64 = ctx.r[10].s64 + 15900;
	// 82665BA4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82665BA8: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82665BAC: 396BF490  addi r11, r11, -0xb70
	ctx.r[11].s64 = ctx.r[11].s64 + -2928;
	// 82665BB0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82665BB4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82665BB8: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82665BBC: 386AD240  addi r3, r10, -0x2dc0
	ctx.r[3].s64 = ctx.r[10].s64 + -11712;
	// 82665BC0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82665BC4: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 82665BC8: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82665BCC: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 82665BD0: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82665BD4: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82665BD8: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82665BDC: 4BE01245  bl 0x82466e20
	ctx.lr = 0x82665BE0;
	sub_82466E20(ctx, base);
	// 82665BE0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82665BE4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82665BE8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82665BEC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82665BF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82665BF0 size=24
    let mut pc: u32 = 0x82665BF0;
    'dispatch: loop {
        match pc {
            0x82665BF0 => {
    //   block [0x82665BF0..0x82665C08)
	// 82665BF0: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 82665BF4: 3D40827A  lis r10, -0x7d86
	ctx.r[10].s64 = -2105933824;
	// 82665BF8: 394AC440  addi r10, r10, -0x3bc0
	ctx.r[10].s64 = ctx.r[10].s64 + -15296;
	// 82665BFC: 816B7F18  lwz r11, 0x7f18(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(32536 as u32) ) } as u64;
	// 82665C00: 916A0068  stw r11, 0x68(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(104 as u32), ctx.r[11].u32 ) };
	// 82665C04: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82665C08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82665C08 size=116
    let mut pc: u32 = 0x82665C08;
    'dispatch: loop {
        match pc {
            0x82665C08 => {
    //   block [0x82665C08..0x82665C7C)
	// 82665C08: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82665C0C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82665C10: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82665C14: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 82665C18: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82665C1C: 392BF464  addi r9, r11, -0xb9c
	ctx.r[9].s64 = ctx.r[11].s64 + -2972;
	// 82665C20: 38AACEE0  addi r5, r10, -0x3120
	ctx.r[5].s64 = ctx.r[10].s64 + -12576;
	// 82665C24: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82665C28: 38E9006C  addi r7, r9, 0x6c
	ctx.r[7].s64 = ctx.r[9].s64 + 108;
	// 82665C2C: 38C00007  li r6, 7
	ctx.r[6].s64 = 7;
	// 82665C30: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 82665C34: 388A3E30  addi r4, r10, 0x3e30
	ctx.r[4].s64 = ctx.r[10].s64 + 15920;
	// 82665C38: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82665C3C: 396BC440  addi r11, r11, -0x3bc0
	ctx.r[11].s64 = ctx.r[11].s64 + -15296;
	// 82665C40: 90E10064  stw r7, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[7].u32 ) };
	// 82665C44: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82665C48: 90C1005C  stw r6, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[6].u32 ) };
	// 82665C4C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82665C50: 386AD270  addi r3, r10, -0x2d90
	ctx.r[3].s64 = ctx.r[10].s64 + -11664;
	// 82665C54: 39400002  li r10, 2
	ctx.r[10].s64 = 2;
	// 82665C58: 91010074  stw r8, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[8].u32 ) };
	// 82665C5C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82665C60: 9101006C  stw r8, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[8].u32 ) };
	// 82665C64: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82665C68: 4BE011B9  bl 0x82466e20
	ctx.lr = 0x82665C6C;
	sub_82466E20(ctx, base);
	// 82665C6C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82665C70: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82665C74: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82665C78: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82665C80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82665C80 size=112
    let mut pc: u32 = 0x82665C80;
    'dispatch: loop {
        match pc {
            0x82665C80 => {
    //   block [0x82665C80..0x82665CF0)
	// 82665C80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82665C84: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82665C88: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82665C8C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82665C90: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 82665C94: 38AACE80  addi r5, r10, -0x3180
	ctx.r[5].s64 = ctx.r[10].s64 + -12672;
	// 82665C98: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82665C9C: 390B78B8  addi r8, r11, 0x78b8
	ctx.r[8].s64 = ctx.r[11].s64 + 30904;
	// 82665CA0: 39200006  li r9, 6
	ctx.r[9].s64 = 6;
	// 82665CA4: 388A3E3C  addi r4, r10, 0x3e3c
	ctx.r[4].s64 = ctx.r[10].s64 + 15932;
	// 82665CA8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82665CAC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82665CB0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82665CB4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82665CB8: 386AD2A0  addi r3, r10, -0x2d60
	ctx.r[3].s64 = ctx.r[10].s64 + -11616;
	// 82665CBC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82665CC0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82665CC4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82665CC8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82665CCC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82665CD0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82665CD4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82665CD8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82665CDC: 4BE01145  bl 0x82466e20
	ctx.lr = 0x82665CE0;
	sub_82466E20(ctx, base);
	// 82665CE0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82665CE4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82665CE8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82665CEC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82665CF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82665CF0 size=24
    let mut pc: u32 = 0x82665CF0;
    'dispatch: loop {
        match pc {
            0x82665CF0 => {
    //   block [0x82665CF0..0x82665D08)
	// 82665CF0: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 82665CF4: 3D40827A  lis r10, -0x7d86
	ctx.r[10].s64 = -2105933824;
	// 82665CF8: 394AC4E8  addi r10, r10, -0x3b18
	ctx.r[10].s64 = ctx.r[10].s64 + -15128;
	// 82665CFC: 816B7F18  lwz r11, 0x7f18(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(32536 as u32) ) } as u64;
	// 82665D00: 916A00C8  stw r11, 0xc8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(200 as u32), ctx.r[11].u32 ) };
	// 82665D04: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82665D08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82665D08 size=116
    let mut pc: u32 = 0x82665D08;
    'dispatch: loop {
        match pc {
            0x82665D08 => {
    //   block [0x82665D08..0x82665D7C)
	// 82665D08: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82665D0C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82665D10: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82665D14: 3D40827A  lis r10, -0x7d86
	ctx.r[10].s64 = -2105933824;
	// 82665D18: 38E0000D  li r7, 0xd
	ctx.r[7].s64 = 13;
	// 82665D1C: 390AC4E8  addi r8, r10, -0x3b18
	ctx.r[8].s64 = ctx.r[10].s64 + -15128;
	// 82665D20: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82665D24: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 82665D28: 38AACE80  addi r5, r10, -0x3180
	ctx.r[5].s64 = ctx.r[10].s64 + -12672;
	// 82665D2C: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82665D30: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82665D34: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82665D38: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82665D3C: 388A3E64  addi r4, r10, 0x3e64
	ctx.r[4].s64 = ctx.r[10].s64 + 15972;
	// 82665D40: 396BF4F0  addi r11, r11, -0xb10
	ctx.r[11].s64 = ctx.r[11].s64 + -2832;
	// 82665D44: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82665D48: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 82665D4C: 386AD2D0  addi r3, r10, -0x2d30
	ctx.r[3].s64 = ctx.r[10].s64 + -11568;
	// 82665D50: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 82665D54: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82665D58: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 82665D5C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82665D60: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82665D64: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82665D68: 4BE010B9  bl 0x82466e20
	ctx.lr = 0x82665D6C;
	sub_82466E20(ctx, base);
	// 82665D6C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82665D70: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82665D74: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82665D78: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82665D80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82665D80 size=112
    let mut pc: u32 = 0x82665D80;
    'dispatch: loop {
        match pc {
            0x82665D80 => {
    //   block [0x82665D80..0x82665DF0)
	// 82665D80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82665D84: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82665D88: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82665D8C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82665D90: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 82665D94: 38AABB60  addi r5, r10, -0x44a0
	ctx.r[5].s64 = ctx.r[10].s64 + -17568;
	// 82665D98: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82665D9C: 390B7948  addi r8, r11, 0x7948
	ctx.r[8].s64 = ctx.r[11].s64 + 31048;
	// 82665DA0: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 82665DA4: 388A3E78  addi r4, r10, 0x3e78
	ctx.r[4].s64 = ctx.r[10].s64 + 15992;
	// 82665DA8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82665DAC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82665DB0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82665DB4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82665DB8: 386AD300  addi r3, r10, -0x2d00
	ctx.r[3].s64 = ctx.r[10].s64 + -11520;
	// 82665DBC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82665DC0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82665DC4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82665DC8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82665DCC: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 82665DD0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82665DD4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82665DD8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82665DDC: 4BE01045  bl 0x82466e20
	ctx.lr = 0x82665DE0;
	sub_82466E20(ctx, base);
	// 82665DE0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82665DE4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82665DE8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82665DEC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82665DF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82665DF0 size=108
    let mut pc: u32 = 0x82665DF0;
    'dispatch: loop {
        match pc {
            0x82665DF0 => {
    //   block [0x82665DF0..0x82665E5C)
	// 82665DF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82665DF4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82665DF8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82665DFC: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 82665E00: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82665E04: 38EB7978  addi r7, r11, 0x7978
	ctx.r[7].s64 = ctx.r[11].s64 + 31096;
	// 82665E08: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 82665E0C: 388A3E98  addi r4, r10, 0x3e98
	ctx.r[4].s64 = ctx.r[10].s64 + 16024;
	// 82665E10: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82665E14: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82665E18: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82665E1C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82665E20: 386AD330  addi r3, r10, -0x2cd0
	ctx.r[3].s64 = ctx.r[10].s64 + -11472;
	// 82665E24: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82665E28: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82665E2C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82665E30: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82665E34: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82665E38: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82665E3C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82665E40: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82665E44: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82665E48: 4BE00FD9  bl 0x82466e20
	ctx.lr = 0x82665E4C;
	sub_82466E20(ctx, base);
	// 82665E4C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82665E50: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82665E54: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82665E58: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82665E60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82665E60 size=112
    let mut pc: u32 = 0x82665E60;
    'dispatch: loop {
        match pc {
            0x82665E60 => {
    //   block [0x82665E60..0x82665ED0)
	// 82665E60: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82665E64: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82665E68: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82665E6C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82665E70: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 82665E74: 38AACD60  addi r5, r10, -0x32a0
	ctx.r[5].s64 = ctx.r[10].s64 + -12960;
	// 82665E78: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82665E7C: 390B79A8  addi r8, r11, 0x79a8
	ctx.r[8].s64 = ctx.r[11].s64 + 31144;
	// 82665E80: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 82665E84: 388A3EAC  addi r4, r10, 0x3eac
	ctx.r[4].s64 = ctx.r[10].s64 + 16044;
	// 82665E88: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82665E8C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82665E90: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82665E94: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82665E98: 386AD360  addi r3, r10, -0x2ca0
	ctx.r[3].s64 = ctx.r[10].s64 + -11424;
	// 82665E9C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82665EA0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82665EA4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82665EA8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82665EAC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82665EB0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82665EB4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82665EB8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82665EBC: 4BE00F65  bl 0x82466e20
	ctx.lr = 0x82665EC0;
	sub_82466E20(ctx, base);
	// 82665EC0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82665EC4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82665EC8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82665ECC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82665ED0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82665ED0 size=112
    let mut pc: u32 = 0x82665ED0;
    'dispatch: loop {
        match pc {
            0x82665ED0 => {
    //   block [0x82665ED0..0x82665F40)
	// 82665ED0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82665ED4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82665ED8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82665EDC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82665EE0: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 82665EE4: 38AAD570  addi r5, r10, -0x2a90
	ctx.r[5].s64 = ctx.r[10].s64 + -10896;
	// 82665EE8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82665EEC: 390B79D8  addi r8, r11, 0x79d8
	ctx.r[8].s64 = ctx.r[11].s64 + 31192;
	// 82665EF0: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 82665EF4: 388A3EBC  addi r4, r10, 0x3ebc
	ctx.r[4].s64 = ctx.r[10].s64 + 16060;
	// 82665EF8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82665EFC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82665F00: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82665F04: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82665F08: 386AD390  addi r3, r10, -0x2c70
	ctx.r[3].s64 = ctx.r[10].s64 + -11376;
	// 82665F0C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82665F10: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82665F14: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82665F18: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82665F1C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82665F20: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82665F24: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82665F28: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82665F2C: 4BE00EF5  bl 0x82466e20
	ctx.lr = 0x82665F30;
	sub_82466E20(ctx, base);
	// 82665F30: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82665F34: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82665F38: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82665F3C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82665F40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82665F40 size=108
    let mut pc: u32 = 0x82665F40;
    'dispatch: loop {
        match pc {
            0x82665F40 => {
    //   block [0x82665F40..0x82665FAC)
	// 82665F40: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82665F44: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82665F48: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82665F4C: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 82665F50: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82665F54: 38EB7A08  addi r7, r11, 0x7a08
	ctx.r[7].s64 = ctx.r[11].s64 + 31240;
	// 82665F58: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 82665F5C: 388A3ED0  addi r4, r10, 0x3ed0
	ctx.r[4].s64 = ctx.r[10].s64 + 16080;
	// 82665F60: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82665F64: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82665F68: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82665F6C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82665F70: 386AD3C0  addi r3, r10, -0x2c40
	ctx.r[3].s64 = ctx.r[10].s64 + -11328;
	// 82665F74: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82665F78: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82665F7C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82665F80: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82665F84: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82665F88: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82665F8C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82665F90: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82665F94: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82665F98: 4BE00E89  bl 0x82466e20
	ctx.lr = 0x82665F9C;
	sub_82466E20(ctx, base);
	// 82665F9C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82665FA0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82665FA4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82665FA8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


