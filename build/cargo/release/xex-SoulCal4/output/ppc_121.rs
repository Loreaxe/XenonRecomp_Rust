pub fn sub_826BD9D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826BD9D8 size=112
    let mut pc: u32 = 0x826BD9D8;
    'dispatch: loop {
        match pc {
            0x826BD9D8 => {
    //   block [0x826BD9D8..0x826BDA48)
	// 826BD9D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826BD9DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826BD9E0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826BD9E4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826BD9E8: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826BD9EC: 38AA221C  addi r5, r10, 0x221c
	ctx.r[5].s64 = ctx.r[10].s64 + 8732;
	// 826BD9F0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826BD9F4: 390BF994  addi r8, r11, -0x66c
	ctx.r[8].s64 = ctx.r[11].s64 + -1644;
	// 826BD9F8: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826BD9FC: 388A417C  addi r4, r10, 0x417c
	ctx.r[4].s64 = ctx.r[10].s64 + 16764;
	// 826BDA00: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826BDA04: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826BDA08: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826BDA0C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826BDA10: 386A22DC  addi r3, r10, 0x22dc
	ctx.r[3].s64 = ctx.r[10].s64 + 8924;
	// 826BDA14: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826BDA18: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826BDA1C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826BDA20: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826BDA24: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826BDA28: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826BDA2C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826BDA30: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826BDA34: 4BDA93ED  bl 0x82466e20
	ctx.lr = 0x826BDA38;
	sub_82466E20(ctx, base);
	// 826BDA38: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826BDA3C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826BDA40: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826BDA44: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826BDA48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826BDA48 size=112
    let mut pc: u32 = 0x826BDA48;
    'dispatch: loop {
        match pc {
            0x826BDA48 => {
    //   block [0x826BDA48..0x826BDAB8)
	// 826BDA48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826BDA4C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826BDA50: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826BDA54: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826BDA58: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826BDA5C: 38AA221C  addi r5, r10, 0x221c
	ctx.r[5].s64 = ctx.r[10].s64 + 8732;
	// 826BDA60: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826BDA64: 390BF9AC  addi r8, r11, -0x654
	ctx.r[8].s64 = ctx.r[11].s64 + -1620;
	// 826BDA68: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826BDA6C: 388A4198  addi r4, r10, 0x4198
	ctx.r[4].s64 = ctx.r[10].s64 + 16792;
	// 826BDA70: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826BDA74: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826BDA78: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826BDA7C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826BDA80: 386A230C  addi r3, r10, 0x230c
	ctx.r[3].s64 = ctx.r[10].s64 + 8972;
	// 826BDA84: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826BDA88: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826BDA8C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826BDA90: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826BDA94: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826BDA98: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826BDA9C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826BDAA0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826BDAA4: 4BDA937D  bl 0x82466e20
	ctx.lr = 0x826BDAA8;
	sub_82466E20(ctx, base);
	// 826BDAA8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826BDAAC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826BDAB0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826BDAB4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826BDAB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826BDAB8 size=112
    let mut pc: u32 = 0x826BDAB8;
    'dispatch: loop {
        match pc {
            0x826BDAB8 => {
    //   block [0x826BDAB8..0x826BDB28)
	// 826BDAB8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826BDABC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826BDAC0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826BDAC4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826BDAC8: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826BDACC: 38AA221C  addi r5, r10, 0x221c
	ctx.r[5].s64 = ctx.r[10].s64 + 8732;
	// 826BDAD0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826BDAD4: 390BF9DC  addi r8, r11, -0x624
	ctx.r[8].s64 = ctx.r[11].s64 + -1572;
	// 826BDAD8: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826BDADC: 388A41BC  addi r4, r10, 0x41bc
	ctx.r[4].s64 = ctx.r[10].s64 + 16828;
	// 826BDAE0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826BDAE4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826BDAE8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826BDAEC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826BDAF0: 386A233C  addi r3, r10, 0x233c
	ctx.r[3].s64 = ctx.r[10].s64 + 9020;
	// 826BDAF4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826BDAF8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826BDAFC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826BDB00: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826BDB04: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826BDB08: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826BDB0C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826BDB10: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826BDB14: 4BDA930D  bl 0x82466e20
	ctx.lr = 0x826BDB18;
	sub_82466E20(ctx, base);
	// 826BDB18: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826BDB1C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826BDB20: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826BDB24: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826BDB28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826BDB28 size=112
    let mut pc: u32 = 0x826BDB28;
    'dispatch: loop {
        match pc {
            0x826BDB28 => {
    //   block [0x826BDB28..0x826BDB98)
	// 826BDB28: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826BDB2C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826BDB30: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826BDB34: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826BDB38: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826BDB3C: 38AA221C  addi r5, r10, 0x221c
	ctx.r[5].s64 = ctx.r[10].s64 + 8732;
	// 826BDB40: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826BDB44: 390BFA0C  addi r8, r11, -0x5f4
	ctx.r[8].s64 = ctx.r[11].s64 + -1524;
	// 826BDB48: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826BDB4C: 388A41E4  addi r4, r10, 0x41e4
	ctx.r[4].s64 = ctx.r[10].s64 + 16868;
	// 826BDB50: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826BDB54: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826BDB58: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826BDB5C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826BDB60: 386A236C  addi r3, r10, 0x236c
	ctx.r[3].s64 = ctx.r[10].s64 + 9068;
	// 826BDB64: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826BDB68: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826BDB6C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826BDB70: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826BDB74: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826BDB78: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826BDB7C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826BDB80: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826BDB84: 4BDA929D  bl 0x82466e20
	ctx.lr = 0x826BDB88;
	sub_82466E20(ctx, base);
	// 826BDB88: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826BDB8C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826BDB90: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826BDB94: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826BDB98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826BDB98 size=112
    let mut pc: u32 = 0x826BDB98;
    'dispatch: loop {
        match pc {
            0x826BDB98 => {
    //   block [0x826BDB98..0x826BDC08)
	// 826BDB98: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826BDB9C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826BDBA0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826BDBA4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826BDBA8: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826BDBAC: 38AA221C  addi r5, r10, 0x221c
	ctx.r[5].s64 = ctx.r[10].s64 + 8732;
	// 826BDBB0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826BDBB4: 390BFA3C  addi r8, r11, -0x5c4
	ctx.r[8].s64 = ctx.r[11].s64 + -1476;
	// 826BDBB8: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826BDBBC: 388A4208  addi r4, r10, 0x4208
	ctx.r[4].s64 = ctx.r[10].s64 + 16904;
	// 826BDBC0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826BDBC4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826BDBC8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826BDBCC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826BDBD0: 386A239C  addi r3, r10, 0x239c
	ctx.r[3].s64 = ctx.r[10].s64 + 9116;
	// 826BDBD4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826BDBD8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826BDBDC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826BDBE0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826BDBE4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826BDBE8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826BDBEC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826BDBF0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826BDBF4: 4BDA922D  bl 0x82466e20
	ctx.lr = 0x826BDBF8;
	sub_82466E20(ctx, base);
	// 826BDBF8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826BDBFC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826BDC00: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826BDC04: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826BDC08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826BDC08 size=112
    let mut pc: u32 = 0x826BDC08;
    'dispatch: loop {
        match pc {
            0x826BDC08 => {
    //   block [0x826BDC08..0x826BDC78)
	// 826BDC08: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826BDC0C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826BDC10: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826BDC14: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826BDC18: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826BDC1C: 38AA221C  addi r5, r10, 0x221c
	ctx.r[5].s64 = ctx.r[10].s64 + 8732;
	// 826BDC20: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826BDC24: 390BFA54  addi r8, r11, -0x5ac
	ctx.r[8].s64 = ctx.r[11].s64 + -1452;
	// 826BDC28: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826BDC2C: 388A4228  addi r4, r10, 0x4228
	ctx.r[4].s64 = ctx.r[10].s64 + 16936;
	// 826BDC30: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826BDC34: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826BDC38: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826BDC3C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826BDC40: 386A23CC  addi r3, r10, 0x23cc
	ctx.r[3].s64 = ctx.r[10].s64 + 9164;
	// 826BDC44: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826BDC48: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826BDC4C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826BDC50: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826BDC54: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826BDC58: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826BDC5C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826BDC60: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826BDC64: 4BDA91BD  bl 0x82466e20
	ctx.lr = 0x826BDC68;
	sub_82466E20(ctx, base);
	// 826BDC68: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826BDC6C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826BDC70: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826BDC74: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826BDC78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826BDC78 size=112
    let mut pc: u32 = 0x826BDC78;
    'dispatch: loop {
        match pc {
            0x826BDC78 => {
    //   block [0x826BDC78..0x826BDCE8)
	// 826BDC78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826BDC7C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826BDC80: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826BDC84: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826BDC88: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826BDC8C: 38AA221C  addi r5, r10, 0x221c
	ctx.r[5].s64 = ctx.r[10].s64 + 8732;
	// 826BDC90: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826BDC94: 390BFA70  addi r8, r11, -0x590
	ctx.r[8].s64 = ctx.r[11].s64 + -1424;
	// 826BDC98: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 826BDC9C: 388A423C  addi r4, r10, 0x423c
	ctx.r[4].s64 = ctx.r[10].s64 + 16956;
	// 826BDCA0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826BDCA4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826BDCA8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826BDCAC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826BDCB0: 386A23FC  addi r3, r10, 0x23fc
	ctx.r[3].s64 = ctx.r[10].s64 + 9212;
	// 826BDCB4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826BDCB8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826BDCBC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826BDCC0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826BDCC4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826BDCC8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826BDCCC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826BDCD0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826BDCD4: 4BDA914D  bl 0x82466e20
	ctx.lr = 0x826BDCD8;
	sub_82466E20(ctx, base);
	// 826BDCD8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826BDCDC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826BDCE0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826BDCE4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826BDCE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826BDCE8 size=112
    let mut pc: u32 = 0x826BDCE8;
    'dispatch: loop {
        match pc {
            0x826BDCE8 => {
    //   block [0x826BDCE8..0x826BDD58)
	// 826BDCE8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826BDCEC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826BDCF0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826BDCF4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826BDCF8: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826BDCFC: 38AA221C  addi r5, r10, 0x221c
	ctx.r[5].s64 = ctx.r[10].s64 + 8732;
	// 826BDD00: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826BDD04: 390BFAB8  addi r8, r11, -0x548
	ctx.r[8].s64 = ctx.r[11].s64 + -1352;
	// 826BDD08: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 826BDD0C: 388A4254  addi r4, r10, 0x4254
	ctx.r[4].s64 = ctx.r[10].s64 + 16980;
	// 826BDD10: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826BDD14: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826BDD18: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826BDD1C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826BDD20: 386A242C  addi r3, r10, 0x242c
	ctx.r[3].s64 = ctx.r[10].s64 + 9260;
	// 826BDD24: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826BDD28: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826BDD2C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826BDD30: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826BDD34: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826BDD38: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826BDD3C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826BDD40: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826BDD44: 4BDA90DD  bl 0x82466e20
	ctx.lr = 0x826BDD48;
	sub_82466E20(ctx, base);
	// 826BDD48: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826BDD4C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826BDD50: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826BDD54: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826BDD58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826BDD58 size=112
    let mut pc: u32 = 0x826BDD58;
    'dispatch: loop {
        match pc {
            0x826BDD58 => {
    //   block [0x826BDD58..0x826BDDC8)
	// 826BDD58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826BDD5C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826BDD60: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826BDD64: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826BDD68: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826BDD6C: 38AA221C  addi r5, r10, 0x221c
	ctx.r[5].s64 = ctx.r[10].s64 + 8732;
	// 826BDD70: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826BDD74: 390BFB00  addi r8, r11, -0x500
	ctx.r[8].s64 = ctx.r[11].s64 + -1280;
	// 826BDD78: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826BDD7C: 388A4270  addi r4, r10, 0x4270
	ctx.r[4].s64 = ctx.r[10].s64 + 17008;
	// 826BDD80: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826BDD84: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826BDD88: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826BDD8C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826BDD90: 386A245C  addi r3, r10, 0x245c
	ctx.r[3].s64 = ctx.r[10].s64 + 9308;
	// 826BDD94: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826BDD98: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826BDD9C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826BDDA0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826BDDA4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826BDDA8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826BDDAC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826BDDB0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826BDDB4: 4BDA906D  bl 0x82466e20
	ctx.lr = 0x826BDDB8;
	sub_82466E20(ctx, base);
	// 826BDDB8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826BDDBC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826BDDC0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826BDDC4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826BDDC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826BDDC8 size=112
    let mut pc: u32 = 0x826BDDC8;
    'dispatch: loop {
        match pc {
            0x826BDDC8 => {
    //   block [0x826BDDC8..0x826BDE38)
	// 826BDDC8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826BDDCC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826BDDD0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826BDDD4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826BDDD8: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826BDDDC: 38AA221C  addi r5, r10, 0x221c
	ctx.r[5].s64 = ctx.r[10].s64 + 8732;
	// 826BDDE0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826BDDE4: 390BFB18  addi r8, r11, -0x4e8
	ctx.r[8].s64 = ctx.r[11].s64 + -1256;
	// 826BDDE8: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826BDDEC: 388A4288  addi r4, r10, 0x4288
	ctx.r[4].s64 = ctx.r[10].s64 + 17032;
	// 826BDDF0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826BDDF4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826BDDF8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826BDDFC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826BDE00: 386A248C  addi r3, r10, 0x248c
	ctx.r[3].s64 = ctx.r[10].s64 + 9356;
	// 826BDE04: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826BDE08: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826BDE0C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826BDE10: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826BDE14: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826BDE18: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826BDE1C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826BDE20: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826BDE24: 4BDA8FFD  bl 0x82466e20
	ctx.lr = 0x826BDE28;
	sub_82466E20(ctx, base);
	// 826BDE28: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826BDE2C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826BDE30: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826BDE34: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826BDE38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826BDE38 size=116
    let mut pc: u32 = 0x826BDE38;
    'dispatch: loop {
        match pc {
            0x826BDE38 => {
    //   block [0x826BDE38..0x826BDEAC)
	// 826BDE38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826BDE3C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826BDE40: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826BDE44: 3D40827E  lis r10, -0x7d82
	ctx.r[10].s64 = -2105671680;
	// 826BDE48: 38E00005  li r7, 5
	ctx.r[7].s64 = 5;
	// 826BDE4C: 390AFB48  addi r8, r10, -0x4b8
	ctx.r[8].s64 = ctx.r[10].s64 + -1208;
	// 826BDE50: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826BDE54: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 826BDE58: 38AA221C  addi r5, r10, 0x221c
	ctx.r[5].s64 = ctx.r[10].s64 + 8732;
	// 826BDE5C: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826BDE60: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826BDE64: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826BDE68: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826BDE6C: 388A429C  addi r4, r10, 0x429c
	ctx.r[4].s64 = ctx.r[10].s64 + 17052;
	// 826BDE70: 396B0E00  addi r11, r11, 0xe00
	ctx.r[11].s64 = ctx.r[11].s64 + 3584;
	// 826BDE74: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826BDE78: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826BDE7C: 386A24BC  addi r3, r10, 0x24bc
	ctx.r[3].s64 = ctx.r[10].s64 + 9404;
	// 826BDE80: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 826BDE84: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826BDE88: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 826BDE8C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826BDE90: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826BDE94: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826BDE98: 4BDA8F89  bl 0x82466e20
	ctx.lr = 0x826BDE9C;
	sub_82466E20(ctx, base);
	// 826BDE9C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826BDEA0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826BDEA4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826BDEA8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826BDEB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826BDEB0 size=116
    let mut pc: u32 = 0x826BDEB0;
    'dispatch: loop {
        match pc {
            0x826BDEB0 => {
    //   block [0x826BDEB0..0x826BDF24)
	// 826BDEB0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826BDEB4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826BDEB8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826BDEBC: 3D40827E  lis r10, -0x7d82
	ctx.r[10].s64 = -2105671680;
	// 826BDEC0: 38E00006  li r7, 6
	ctx.r[7].s64 = 6;
	// 826BDEC4: 390AFBC0  addi r8, r10, -0x440
	ctx.r[8].s64 = ctx.r[10].s64 + -1088;
	// 826BDEC8: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826BDECC: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 826BDED0: 38AA221C  addi r5, r10, 0x221c
	ctx.r[5].s64 = ctx.r[10].s64 + 8732;
	// 826BDED4: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826BDED8: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826BDEDC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826BDEE0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826BDEE4: 388A42B8  addi r4, r10, 0x42b8
	ctx.r[4].s64 = ctx.r[10].s64 + 17080;
	// 826BDEE8: 396B0E18  addi r11, r11, 0xe18
	ctx.r[11].s64 = ctx.r[11].s64 + 3608;
	// 826BDEEC: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826BDEF0: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826BDEF4: 386A24EC  addi r3, r10, 0x24ec
	ctx.r[3].s64 = ctx.r[10].s64 + 9452;
	// 826BDEF8: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 826BDEFC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826BDF00: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 826BDF04: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826BDF08: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826BDF0C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826BDF10: 4BDA8F11  bl 0x82466e20
	ctx.lr = 0x826BDF14;
	sub_82466E20(ctx, base);
	// 826BDF14: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826BDF18: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826BDF1C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826BDF20: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826BDF28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826BDF28 size=24
    let mut pc: u32 = 0x826BDF28;
    'dispatch: loop {
        match pc {
            0x826BDF28 => {
    //   block [0x826BDF28..0x826BDF40)
	// 826BDF28: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826BDF2C: 3D40827E  lis r10, -0x7d82
	ctx.r[10].s64 = -2105671680;
	// 826BDF30: 394A3278  addi r10, r10, 0x3278
	ctx.r[10].s64 = ctx.r[10].s64 + 12920;
	// 826BDF34: 816BFA6C  lwz r11, -0x594(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-1428 as u32) ) } as u64;
	// 826BDF38: 916A0050  stw r11, 0x50(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 826BDF3C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826BDF40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826BDF40 size=116
    let mut pc: u32 = 0x826BDF40;
    'dispatch: loop {
        match pc {
            0x826BDF40 => {
    //   block [0x826BDF40..0x826BDFB4)
	// 826BDF40: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826BDF44: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826BDF48: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826BDF4C: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 826BDF50: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826BDF54: 392B0E44  addi r9, r11, 0xe44
	ctx.r[9].s64 = ctx.r[11].s64 + 3652;
	// 826BDF58: 38AA221C  addi r5, r10, 0x221c
	ctx.r[5].s64 = ctx.r[10].s64 + 8732;
	// 826BDF5C: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826BDF60: 38E90014  addi r7, r9, 0x14
	ctx.r[7].s64 = ctx.r[9].s64 + 20;
	// 826BDF64: 38C00007  li r6, 7
	ctx.r[6].s64 = 7;
	// 826BDF68: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826BDF6C: 388A42D4  addi r4, r10, 0x42d4
	ctx.r[4].s64 = ctx.r[10].s64 + 17108;
	// 826BDF70: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826BDF74: 396B3278  addi r11, r11, 0x3278
	ctx.r[11].s64 = ctx.r[11].s64 + 12920;
	// 826BDF78: 90E10064  stw r7, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[7].u32 ) };
	// 826BDF7C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826BDF80: 90C1005C  stw r6, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[6].u32 ) };
	// 826BDF84: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826BDF88: 386A251C  addi r3, r10, 0x251c
	ctx.r[3].s64 = ctx.r[10].s64 + 9500;
	// 826BDF8C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826BDF90: 91010074  stw r8, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[8].u32 ) };
	// 826BDF94: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826BDF98: 9101006C  stw r8, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[8].u32 ) };
	// 826BDF9C: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826BDFA0: 4BDA8E81  bl 0x82466e20
	ctx.lr = 0x826BDFA4;
	sub_82466E20(ctx, base);
	// 826BDFA4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826BDFA8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826BDFAC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826BDFB0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826BDFB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826BDFB8 size=112
    let mut pc: u32 = 0x826BDFB8;
    'dispatch: loop {
        match pc {
            0x826BDFB8 => {
    //   block [0x826BDFB8..0x826BE028)
	// 826BDFB8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826BDFBC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826BDFC0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826BDFC4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826BDFC8: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826BDFCC: 38AA221C  addi r5, r10, 0x221c
	ctx.r[5].s64 = ctx.r[10].s64 + 8732;
	// 826BDFD0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826BDFD4: 390BFC50  addi r8, r11, -0x3b0
	ctx.r[8].s64 = ctx.r[11].s64 + -944;
	// 826BDFD8: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 826BDFDC: 388A42F0  addi r4, r10, 0x42f0
	ctx.r[4].s64 = ctx.r[10].s64 + 17136;
	// 826BDFE0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826BDFE4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826BDFE8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826BDFEC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826BDFF0: 386A254C  addi r3, r10, 0x254c
	ctx.r[3].s64 = ctx.r[10].s64 + 9548;
	// 826BDFF4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826BDFF8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826BDFFC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826BE000: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826BE004: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826BE008: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826BE00C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826BE010: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826BE014: 4BDA8E0D  bl 0x82466e20
	ctx.lr = 0x826BE018;
	sub_82466E20(ctx, base);
	// 826BE018: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826BE01C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826BE020: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826BE024: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826BE028(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826BE028 size=112
    let mut pc: u32 = 0x826BE028;
    'dispatch: loop {
        match pc {
            0x826BE028 => {
    //   block [0x826BE028..0x826BE098)
	// 826BE028: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826BE02C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826BE030: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826BE034: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826BE038: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826BE03C: 38AA221C  addi r5, r10, 0x221c
	ctx.r[5].s64 = ctx.r[10].s64 + 8732;
	// 826BE040: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826BE044: 390BFCB0  addi r8, r11, -0x350
	ctx.r[8].s64 = ctx.r[11].s64 + -848;
	// 826BE048: 39200007  li r9, 7
	ctx.r[9].s64 = 7;
	// 826BE04C: 388A430C  addi r4, r10, 0x430c
	ctx.r[4].s64 = ctx.r[10].s64 + 17164;
	// 826BE050: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826BE054: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826BE058: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826BE05C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826BE060: 386A257C  addi r3, r10, 0x257c
	ctx.r[3].s64 = ctx.r[10].s64 + 9596;
	// 826BE064: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826BE068: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826BE06C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826BE070: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826BE074: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826BE078: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826BE07C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826BE080: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826BE084: 4BDA8D9D  bl 0x82466e20
	ctx.lr = 0x826BE088;
	sub_82466E20(ctx, base);
	// 826BE088: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826BE08C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826BE090: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826BE094: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826BE098(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826BE098 size=112
    let mut pc: u32 = 0x826BE098;
    'dispatch: loop {
        match pc {
            0x826BE098 => {
    //   block [0x826BE098..0x826BE108)
	// 826BE098: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826BE09C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826BE0A0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826BE0A4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826BE0A8: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826BE0AC: 38AA221C  addi r5, r10, 0x221c
	ctx.r[5].s64 = ctx.r[10].s64 + 8732;
	// 826BE0B0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826BE0B4: 390BFD58  addi r8, r11, -0x2a8
	ctx.r[8].s64 = ctx.r[11].s64 + -680;
	// 826BE0B8: 39200005  li r9, 5
	ctx.r[9].s64 = 5;
	// 826BE0BC: 388A4328  addi r4, r10, 0x4328
	ctx.r[4].s64 = ctx.r[10].s64 + 17192;
	// 826BE0C0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826BE0C4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826BE0C8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826BE0CC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826BE0D0: 386A25AC  addi r3, r10, 0x25ac
	ctx.r[3].s64 = ctx.r[10].s64 + 9644;
	// 826BE0D4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826BE0D8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826BE0DC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826BE0E0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826BE0E4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826BE0E8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826BE0EC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826BE0F0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826BE0F4: 4BDA8D2D  bl 0x82466e20
	ctx.lr = 0x826BE0F8;
	sub_82466E20(ctx, base);
	// 826BE0F8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826BE0FC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826BE100: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826BE104: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826BE108(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826BE108 size=112
    let mut pc: u32 = 0x826BE108;
    'dispatch: loop {
        match pc {
            0x826BE108 => {
    //   block [0x826BE108..0x826BE178)
	// 826BE108: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826BE10C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826BE110: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826BE114: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826BE118: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826BE11C: 38AA221C  addi r5, r10, 0x221c
	ctx.r[5].s64 = ctx.r[10].s64 + 8732;
	// 826BE120: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826BE124: 390BFDD0  addi r8, r11, -0x230
	ctx.r[8].s64 = ctx.r[11].s64 + -560;
	// 826BE128: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 826BE12C: 388A4348  addi r4, r10, 0x4348
	ctx.r[4].s64 = ctx.r[10].s64 + 17224;
	// 826BE130: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826BE134: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826BE138: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826BE13C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826BE140: 386A25DC  addi r3, r10, 0x25dc
	ctx.r[3].s64 = ctx.r[10].s64 + 9692;
	// 826BE144: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826BE148: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826BE14C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826BE150: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826BE154: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826BE158: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826BE15C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826BE160: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826BE164: 4BDA8CBD  bl 0x82466e20
	ctx.lr = 0x826BE168;
	sub_82466E20(ctx, base);
	// 826BE168: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826BE16C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826BE170: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826BE174: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826BE178(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826BE178 size=112
    let mut pc: u32 = 0x826BE178;
    'dispatch: loop {
        match pc {
            0x826BE178 => {
    //   block [0x826BE178..0x826BE1E8)
	// 826BE178: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826BE17C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826BE180: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826BE184: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826BE188: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826BE18C: 38AA221C  addi r5, r10, 0x221c
	ctx.r[5].s64 = ctx.r[10].s64 + 8732;
	// 826BE190: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826BE194: 390BFE18  addi r8, r11, -0x1e8
	ctx.r[8].s64 = ctx.r[11].s64 + -488;
	// 826BE198: 39200006  li r9, 6
	ctx.r[9].s64 = 6;
	// 826BE19C: 388A4364  addi r4, r10, 0x4364
	ctx.r[4].s64 = ctx.r[10].s64 + 17252;
	// 826BE1A0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826BE1A4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826BE1A8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826BE1AC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826BE1B0: 386A260C  addi r3, r10, 0x260c
	ctx.r[3].s64 = ctx.r[10].s64 + 9740;
	// 826BE1B4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826BE1B8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826BE1BC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826BE1C0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826BE1C4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826BE1C8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826BE1CC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826BE1D0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826BE1D4: 4BDA8C4D  bl 0x82466e20
	ctx.lr = 0x826BE1D8;
	sub_82466E20(ctx, base);
	// 826BE1D8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826BE1DC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826BE1E0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826BE1E4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826BE1E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826BE1E8 size=112
    let mut pc: u32 = 0x826BE1E8;
    'dispatch: loop {
        match pc {
            0x826BE1E8 => {
    //   block [0x826BE1E8..0x826BE258)
	// 826BE1E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826BE1EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826BE1F0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826BE1F4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826BE1F8: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826BE1FC: 38AA221C  addi r5, r10, 0x221c
	ctx.r[5].s64 = ctx.r[10].s64 + 8732;
	// 826BE200: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826BE204: 390BFEA8  addi r8, r11, -0x158
	ctx.r[8].s64 = ctx.r[11].s64 + -344;
	// 826BE208: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 826BE20C: 388A4380  addi r4, r10, 0x4380
	ctx.r[4].s64 = ctx.r[10].s64 + 17280;
	// 826BE210: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826BE214: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826BE218: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826BE21C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826BE220: 386A263C  addi r3, r10, 0x263c
	ctx.r[3].s64 = ctx.r[10].s64 + 9788;
	// 826BE224: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826BE228: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826BE22C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826BE230: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826BE234: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826BE238: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826BE23C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826BE240: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826BE244: 4BDA8BDD  bl 0x82466e20
	ctx.lr = 0x826BE248;
	sub_82466E20(ctx, base);
	// 826BE248: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826BE24C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826BE250: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826BE254: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826BE258(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826BE258 size=112
    let mut pc: u32 = 0x826BE258;
    'dispatch: loop {
        match pc {
            0x826BE258 => {
    //   block [0x826BE258..0x826BE2C8)
	// 826BE258: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826BE25C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826BE260: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826BE264: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826BE268: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826BE26C: 38AA221C  addi r5, r10, 0x221c
	ctx.r[5].s64 = ctx.r[10].s64 + 8732;
	// 826BE270: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826BE274: 390BFF08  addi r8, r11, -0xf8
	ctx.r[8].s64 = ctx.r[11].s64 + -248;
	// 826BE278: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 826BE27C: 388A4398  addi r4, r10, 0x4398
	ctx.r[4].s64 = ctx.r[10].s64 + 17304;
	// 826BE280: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826BE284: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826BE288: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826BE28C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826BE290: 386A266C  addi r3, r10, 0x266c
	ctx.r[3].s64 = ctx.r[10].s64 + 9836;
	// 826BE294: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826BE298: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826BE29C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826BE2A0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826BE2A4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826BE2A8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826BE2AC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826BE2B0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826BE2B4: 4BDA8B6D  bl 0x82466e20
	ctx.lr = 0x826BE2B8;
	sub_82466E20(ctx, base);
	// 826BE2B8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826BE2BC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826BE2C0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826BE2C4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826BE2C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826BE2C8 size=112
    let mut pc: u32 = 0x826BE2C8;
    'dispatch: loop {
        match pc {
            0x826BE2C8 => {
    //   block [0x826BE2C8..0x826BE338)
	// 826BE2C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826BE2CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826BE2D0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826BE2D4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826BE2D8: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826BE2DC: 38AA266C  addi r5, r10, 0x266c
	ctx.r[5].s64 = ctx.r[10].s64 + 9836;
	// 826BE2E0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826BE2E4: 390BFF50  addi r8, r11, -0xb0
	ctx.r[8].s64 = ctx.r[11].s64 + -176;
	// 826BE2E8: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826BE2EC: 388A43B4  addi r4, r10, 0x43b4
	ctx.r[4].s64 = ctx.r[10].s64 + 17332;
	// 826BE2F0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826BE2F4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826BE2F8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826BE2FC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826BE300: 386A269C  addi r3, r10, 0x269c
	ctx.r[3].s64 = ctx.r[10].s64 + 9884;
	// 826BE304: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826BE308: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826BE30C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826BE310: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826BE314: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826BE318: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826BE31C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826BE320: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826BE324: 4BDA8AFD  bl 0x82466e20
	ctx.lr = 0x826BE328;
	sub_82466E20(ctx, base);
	// 826BE328: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826BE32C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826BE330: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826BE334: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826BE338(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826BE338 size=112
    let mut pc: u32 = 0x826BE338;
    'dispatch: loop {
        match pc {
            0x826BE338 => {
    //   block [0x826BE338..0x826BE3A8)
	// 826BE338: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826BE33C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826BE340: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826BE344: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826BE348: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826BE34C: 38AA266C  addi r5, r10, 0x266c
	ctx.r[5].s64 = ctx.r[10].s64 + 9836;
	// 826BE350: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826BE354: 390BFF80  addi r8, r11, -0x80
	ctx.r[8].s64 = ctx.r[11].s64 + -128;
	// 826BE358: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826BE35C: 388A43D8  addi r4, r10, 0x43d8
	ctx.r[4].s64 = ctx.r[10].s64 + 17368;
	// 826BE360: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826BE364: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826BE368: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826BE36C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826BE370: 386A26CC  addi r3, r10, 0x26cc
	ctx.r[3].s64 = ctx.r[10].s64 + 9932;
	// 826BE374: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826BE378: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826BE37C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826BE380: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826BE384: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826BE388: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826BE38C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826BE390: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826BE394: 4BDA8A8D  bl 0x82466e20
	ctx.lr = 0x826BE398;
	sub_82466E20(ctx, base);
	// 826BE398: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826BE39C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826BE3A0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826BE3A4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826BE3A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826BE3A8 size=100
    let mut pc: u32 = 0x826BE3A8;
    'dispatch: loop {
        match pc {
            0x826BE3A8 => {
    //   block [0x826BE3A8..0x826BE40C)
	// 826BE3A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826BE3AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826BE3B0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826BE3B4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826BE3B8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826BE3BC: 38AA266C  addi r5, r10, 0x266c
	ctx.r[5].s64 = ctx.r[10].s64 + 9836;
	// 826BE3C0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826BE3C4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826BE3C8: 388A43FC  addi r4, r10, 0x43fc
	ctx.r[4].s64 = ctx.r[10].s64 + 17404;
	// 826BE3CC: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826BE3D0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826BE3D4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826BE3D8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826BE3DC: 386A26FC  addi r3, r10, 0x26fc
	ctx.r[3].s64 = ctx.r[10].s64 + 9980;
	// 826BE3E0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826BE3E4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826BE3E8: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826BE3EC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826BE3F0: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826BE3F4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826BE3F8: 4BDA8A29  bl 0x82466e20
	ctx.lr = 0x826BE3FC;
	sub_82466E20(ctx, base);
	// 826BE3FC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826BE400: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826BE404: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826BE408: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826BE410(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826BE410 size=112
    let mut pc: u32 = 0x826BE410;
    'dispatch: loop {
        match pc {
            0x826BE410 => {
    //   block [0x826BE410..0x826BE480)
	// 826BE410: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826BE414: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826BE418: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826BE41C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826BE420: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826BE424: 38AA266C  addi r5, r10, 0x266c
	ctx.r[5].s64 = ctx.r[10].s64 + 9836;
	// 826BE428: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826BE42C: 390BFFB0  addi r8, r11, -0x50
	ctx.r[8].s64 = ctx.r[11].s64 + -80;
	// 826BE430: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826BE434: 388A4424  addi r4, r10, 0x4424
	ctx.r[4].s64 = ctx.r[10].s64 + 17444;
	// 826BE438: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826BE43C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826BE440: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826BE444: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826BE448: 386A272C  addi r3, r10, 0x272c
	ctx.r[3].s64 = ctx.r[10].s64 + 10028;
	// 826BE44C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826BE450: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826BE454: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826BE458: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826BE45C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826BE460: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826BE464: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826BE468: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826BE46C: 4BDA89B5  bl 0x82466e20
	ctx.lr = 0x826BE470;
	sub_82466E20(ctx, base);
	// 826BE470: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826BE474: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826BE478: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826BE47C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826BE480(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826BE480 size=112
    let mut pc: u32 = 0x826BE480;
    'dispatch: loop {
        match pc {
            0x826BE480 => {
    //   block [0x826BE480..0x826BE4F0)
	// 826BE480: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826BE484: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826BE488: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826BE48C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826BE490: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826BE494: 38AA0DAC  addi r5, r10, 0xdac
	ctx.r[5].s64 = ctx.r[10].s64 + 3500;
	// 826BE498: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826BE49C: 390BFFC8  addi r8, r11, -0x38
	ctx.r[8].s64 = ctx.r[11].s64 + -56;
	// 826BE4A0: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 826BE4A4: 388A444C  addi r4, r10, 0x444c
	ctx.r[4].s64 = ctx.r[10].s64 + 17484;
	// 826BE4A8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826BE4AC: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826BE4B0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826BE4B4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826BE4B8: 386A275C  addi r3, r10, 0x275c
	ctx.r[3].s64 = ctx.r[10].s64 + 10076;
	// 826BE4BC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826BE4C0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826BE4C4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826BE4C8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826BE4CC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826BE4D0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826BE4D4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826BE4D8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826BE4DC: 4BDA8945  bl 0x82466e20
	ctx.lr = 0x826BE4E0;
	sub_82466E20(ctx, base);
	// 826BE4E0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826BE4E4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826BE4E8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826BE4EC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826BE4F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826BE4F0 size=112
    let mut pc: u32 = 0x826BE4F0;
    'dispatch: loop {
        match pc {
            0x826BE4F0 => {
    //   block [0x826BE4F0..0x826BE560)
	// 826BE4F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826BE4F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826BE4F8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826BE4FC: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826BE500: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826BE504: 38AA275C  addi r5, r10, 0x275c
	ctx.r[5].s64 = ctx.r[10].s64 + 10076;
	// 826BE508: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826BE50C: 390B0028  addi r8, r11, 0x28
	ctx.r[8].s64 = ctx.r[11].s64 + 40;
	// 826BE510: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826BE514: 388A4458  addi r4, r10, 0x4458
	ctx.r[4].s64 = ctx.r[10].s64 + 17496;
	// 826BE518: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826BE51C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826BE520: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826BE524: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826BE528: 386A278C  addi r3, r10, 0x278c
	ctx.r[3].s64 = ctx.r[10].s64 + 10124;
	// 826BE52C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826BE530: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826BE534: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826BE538: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826BE53C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826BE540: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826BE544: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826BE548: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826BE54C: 4BDA88D5  bl 0x82466e20
	ctx.lr = 0x826BE550;
	sub_82466E20(ctx, base);
	// 826BE550: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826BE554: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826BE558: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826BE55C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826BE560(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826BE560 size=112
    let mut pc: u32 = 0x826BE560;
    'dispatch: loop {
        match pc {
            0x826BE560 => {
    //   block [0x826BE560..0x826BE5D0)
	// 826BE560: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826BE564: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826BE568: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826BE56C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826BE570: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826BE574: 38AA275C  addi r5, r10, 0x275c
	ctx.r[5].s64 = ctx.r[10].s64 + 10076;
	// 826BE578: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826BE57C: 390B0040  addi r8, r11, 0x40
	ctx.r[8].s64 = ctx.r[11].s64 + 64;
	// 826BE580: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826BE584: 388A4468  addi r4, r10, 0x4468
	ctx.r[4].s64 = ctx.r[10].s64 + 17512;
	// 826BE588: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826BE58C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826BE590: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826BE594: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826BE598: 386A27BC  addi r3, r10, 0x27bc
	ctx.r[3].s64 = ctx.r[10].s64 + 10172;
	// 826BE59C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826BE5A0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826BE5A4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826BE5A8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826BE5AC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826BE5B0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826BE5B4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826BE5B8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826BE5BC: 4BDA8865  bl 0x82466e20
	ctx.lr = 0x826BE5C0;
	sub_82466E20(ctx, base);
	// 826BE5C0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826BE5C4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826BE5C8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826BE5CC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826BE5D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826BE5D0 size=112
    let mut pc: u32 = 0x826BE5D0;
    'dispatch: loop {
        match pc {
            0x826BE5D0 => {
    //   block [0x826BE5D0..0x826BE640)
	// 826BE5D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826BE5D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826BE5D8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826BE5DC: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826BE5E0: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826BE5E4: 38AA275C  addi r5, r10, 0x275c
	ctx.r[5].s64 = ctx.r[10].s64 + 10076;
	// 826BE5E8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826BE5EC: 390B0070  addi r8, r11, 0x70
	ctx.r[8].s64 = ctx.r[11].s64 + 112;
	// 826BE5F0: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826BE5F4: 388A4478  addi r4, r10, 0x4478
	ctx.r[4].s64 = ctx.r[10].s64 + 17528;
	// 826BE5F8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826BE5FC: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826BE600: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826BE604: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826BE608: 386A27EC  addi r3, r10, 0x27ec
	ctx.r[3].s64 = ctx.r[10].s64 + 10220;
	// 826BE60C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826BE610: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826BE614: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826BE618: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826BE61C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826BE620: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826BE624: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826BE628: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826BE62C: 4BDA87F5  bl 0x82466e20
	ctx.lr = 0x826BE630;
	sub_82466E20(ctx, base);
	// 826BE630: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826BE634: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826BE638: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826BE63C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826BE640(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826BE640 size=24
    let mut pc: u32 = 0x826BE640;
    'dispatch: loop {
        match pc {
            0x826BE640 => {
    //   block [0x826BE640..0x826BE658)
	// 826BE640: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826BE644: 3D40827E  lis r10, -0x7d82
	ctx.r[10].s64 = -2105671680;
	// 826BE648: 394A3320  addi r10, r10, 0x3320
	ctx.r[10].s64 = ctx.r[10].s64 + 13088;
	// 826BE64C: 816B0088  lwz r11, 0x88(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(136 as u32) ) } as u64;
	// 826BE650: 916A0008  stw r11, 8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 826BE654: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826BE658(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826BE658 size=112
    let mut pc: u32 = 0x826BE658;
    'dispatch: loop {
        match pc {
            0x826BE658 => {
    //   block [0x826BE658..0x826BE6C8)
	// 826BE658: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826BE65C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826BE660: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826BE664: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 826BE668: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826BE66C: 392A0EA0  addi r9, r10, 0xea0
	ctx.r[9].s64 = ctx.r[10].s64 + 3744;
	// 826BE670: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826BE674: 390B3320  addi r8, r11, 0x3320
	ctx.r[8].s64 = ctx.r[11].s64 + 13088;
	// 826BE678: 38E00003  li r7, 3
	ctx.r[7].s64 = 3;
	// 826BE67C: 388A4488  addi r4, r10, 0x4488
	ctx.r[4].s64 = ctx.r[10].s64 + 17544;
	// 826BE680: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826BE684: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826BE688: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826BE68C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826BE690: 386A281C  addi r3, r10, 0x281c
	ctx.r[3].s64 = ctx.r[10].s64 + 10268;
	// 826BE694: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826BE698: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826BE69C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826BE6A0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826BE6A4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826BE6A8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826BE6AC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826BE6B0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826BE6B4: 4BDA876D  bl 0x82466e20
	ctx.lr = 0x826BE6B8;
	sub_82466E20(ctx, base);
	// 826BE6B8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826BE6BC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826BE6C0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826BE6C4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826BE6C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826BE6C8 size=108
    let mut pc: u32 = 0x826BE6C8;
    'dispatch: loop {
        match pc {
            0x826BE6C8 => {
    //   block [0x826BE6C8..0x826BE734)
	// 826BE6C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826BE6CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826BE6D0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826BE6D4: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826BE6D8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826BE6DC: 38EB008C  addi r7, r11, 0x8c
	ctx.r[7].s64 = ctx.r[11].s64 + 140;
	// 826BE6E0: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826BE6E4: 388A4494  addi r4, r10, 0x4494
	ctx.r[4].s64 = ctx.r[10].s64 + 17556;
	// 826BE6E8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826BE6EC: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826BE6F0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826BE6F4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826BE6F8: 386A284C  addi r3, r10, 0x284c
	ctx.r[3].s64 = ctx.r[10].s64 + 10316;
	// 826BE6FC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826BE700: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826BE704: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826BE708: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826BE70C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826BE710: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826BE714: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826BE718: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826BE71C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826BE720: 4BDA8701  bl 0x82466e20
	ctx.lr = 0x826BE724;
	sub_82466E20(ctx, base);
	// 826BE724: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826BE728: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826BE72C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826BE730: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826BE738(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826BE738 size=108
    let mut pc: u32 = 0x826BE738;
    'dispatch: loop {
        match pc {
            0x826BE738 => {
    //   block [0x826BE738..0x826BE7A4)
	// 826BE738: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826BE73C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826BE740: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826BE744: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826BE748: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826BE74C: 38EB00A8  addi r7, r11, 0xa8
	ctx.r[7].s64 = ctx.r[11].s64 + 168;
	// 826BE750: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 826BE754: 388A44A4  addi r4, r10, 0x44a4
	ctx.r[4].s64 = ctx.r[10].s64 + 17572;
	// 826BE758: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826BE75C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826BE760: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826BE764: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826BE768: 386A287C  addi r3, r10, 0x287c
	ctx.r[3].s64 = ctx.r[10].s64 + 10364;
	// 826BE76C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826BE770: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826BE774: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826BE778: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826BE77C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826BE780: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826BE784: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826BE788: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826BE78C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826BE790: 4BDA8691  bl 0x82466e20
	ctx.lr = 0x826BE794;
	sub_82466E20(ctx, base);
	// 826BE794: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826BE798: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826BE79C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826BE7A0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826BE7A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826BE7A8 size=116
    let mut pc: u32 = 0x826BE7A8;
    'dispatch: loop {
        match pc {
            0x826BE7A8 => {
    //   block [0x826BE7A8..0x826BE81C)
	// 826BE7A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826BE7AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826BE7B0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826BE7B4: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826BE7B8: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 826BE7BC: 390B00F0  addi r8, r11, 0xf0
	ctx.r[8].s64 = ctx.r[11].s64 + 240;
	// 826BE7C0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826BE7C4: 392A0F58  addi r9, r10, 0xf58
	ctx.r[9].s64 = ctx.r[10].s64 + 3928;
	// 826BE7C8: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826BE7CC: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
	// 826BE7D0: 38AA0DAC  addi r5, r10, 0xdac
	ctx.r[5].s64 = ctx.r[10].s64 + 3500;
	// 826BE7D4: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826BE7D8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826BE7DC: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826BE7E0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826BE7E4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826BE7E8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826BE7EC: 3D608298  lis r11, -0x7d68
	ctx.r[11].s64 = -2103967744;
	// 826BE7F0: 388A44B0  addi r4, r10, 0x44b0
	ctx.r[4].s64 = ctx.r[10].s64 + 17584;
	// 826BE7F4: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826BE7F8: 386B28AC  addi r3, r11, 0x28ac
	ctx.r[3].s64 = ctx.r[11].s64 + 10412;
	// 826BE7FC: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826BE800: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826BE804: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826BE808: 4BDA8619  bl 0x82466e20
	ctx.lr = 0x826BE80C;
	sub_82466E20(ctx, base);
	// 826BE80C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826BE810: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826BE814: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826BE818: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826BE820(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826BE820 size=108
    let mut pc: u32 = 0x826BE820;
    'dispatch: loop {
        match pc {
            0x826BE820 => {
    //   block [0x826BE820..0x826BE88C)
	// 826BE820: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826BE824: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826BE828: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826BE82C: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826BE830: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826BE834: 38EB0108  addi r7, r11, 0x108
	ctx.r[7].s64 = ctx.r[11].s64 + 264;
	// 826BE838: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 826BE83C: 388A21EC  addi r4, r10, 0x21ec
	ctx.r[4].s64 = ctx.r[10].s64 + 8684;
	// 826BE840: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826BE844: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826BE848: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826BE84C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826BE850: 386A28DC  addi r3, r10, 0x28dc
	ctx.r[3].s64 = ctx.r[10].s64 + 10460;
	// 826BE854: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826BE858: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826BE85C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826BE860: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826BE864: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826BE868: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826BE86C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826BE870: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826BE874: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826BE878: 4BDA85A9  bl 0x82466e20
	ctx.lr = 0x826BE87C;
	sub_82466E20(ctx, base);
	// 826BE87C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826BE880: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826BE884: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826BE888: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826BE890(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826BE890 size=24
    let mut pc: u32 = 0x826BE890;
    'dispatch: loop {
        match pc {
            0x826BE890 => {
    //   block [0x826BE890..0x826BE8A8)
	// 826BE890: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826BE894: 3D40827E  lis r10, -0x7d82
	ctx.r[10].s64 = -2105671680;
	// 826BE898: 394A3368  addi r10, r10, 0x3368
	ctx.r[10].s64 = ctx.r[10].s64 + 13160;
	// 826BE89C: 816B0168  lwz r11, 0x168(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(360 as u32) ) } as u64;
	// 826BE8A0: 916A0068  stw r11, 0x68(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(104 as u32), ctx.r[11].u32 ) };
	// 826BE8A4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826BE8A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826BE8A8 size=116
    let mut pc: u32 = 0x826BE8A8;
    'dispatch: loop {
        match pc {
            0x826BE8A8 => {
    //   block [0x826BE8A8..0x826BE91C)
	// 826BE8A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826BE8AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826BE8B0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826BE8B4: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826BE8B8: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 826BE8BC: 390B3368  addi r8, r11, 0x3368
	ctx.r[8].s64 = ctx.r[11].s64 + 13160;
	// 826BE8C0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826BE8C4: 392A0FB4  addi r9, r10, 0xfb4
	ctx.r[9].s64 = ctx.r[10].s64 + 4020;
	// 826BE8C8: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826BE8CC: 38E00009  li r7, 9
	ctx.r[7].s64 = 9;
	// 826BE8D0: 38AA0DAC  addi r5, r10, 0xdac
	ctx.r[5].s64 = ctx.r[10].s64 + 3500;
	// 826BE8D4: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826BE8D8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826BE8DC: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826BE8E0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826BE8E4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826BE8E8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826BE8EC: 3D608298  lis r11, -0x7d68
	ctx.r[11].s64 = -2103967744;
	// 826BE8F0: 388A44DC  addi r4, r10, 0x44dc
	ctx.r[4].s64 = ctx.r[10].s64 + 17628;
	// 826BE8F4: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826BE8F8: 386B290C  addi r3, r11, 0x290c
	ctx.r[3].s64 = ctx.r[11].s64 + 10508;
	// 826BE8FC: 39400003  li r10, 3
	ctx.r[10].s64 = 3;
	// 826BE900: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826BE904: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826BE908: 4BDA8519  bl 0x82466e20
	ctx.lr = 0x826BE90C;
	sub_82466E20(ctx, base);
	// 826BE90C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826BE910: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826BE914: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826BE918: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826BE920(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826BE920 size=108
    let mut pc: u32 = 0x826BE920;
    'dispatch: loop {
        match pc {
            0x826BE920 => {
    //   block [0x826BE920..0x826BE98C)
	// 826BE920: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826BE924: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826BE928: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826BE92C: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826BE930: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826BE934: 38EB0174  addi r7, r11, 0x174
	ctx.r[7].s64 = ctx.r[11].s64 + 372;
	// 826BE938: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826BE93C: 388A44F4  addi r4, r10, 0x44f4
	ctx.r[4].s64 = ctx.r[10].s64 + 17652;
	// 826BE940: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826BE944: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826BE948: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826BE94C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826BE950: 386A293C  addi r3, r10, 0x293c
	ctx.r[3].s64 = ctx.r[10].s64 + 10556;
	// 826BE954: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826BE958: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826BE95C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826BE960: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826BE964: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826BE968: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826BE96C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826BE970: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826BE974: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826BE978: 4BDA84A9  bl 0x82466e20
	ctx.lr = 0x826BE97C;
	sub_82466E20(ctx, base);
	// 826BE97C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826BE980: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826BE984: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826BE988: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826BE990(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826BE990 size=112
    let mut pc: u32 = 0x826BE990;
    'dispatch: loop {
        match pc {
            0x826BE990 => {
    //   block [0x826BE990..0x826BEA00)
	// 826BE990: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826BE994: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826BE998: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826BE99C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826BE9A0: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826BE9A4: 38AA28AC  addi r5, r10, 0x28ac
	ctx.r[5].s64 = ctx.r[10].s64 + 10412;
	// 826BE9A8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826BE9AC: 390B01A4  addi r8, r11, 0x1a4
	ctx.r[8].s64 = ctx.r[11].s64 + 420;
	// 826BE9B0: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826BE9B4: 388A4518  addi r4, r10, 0x4518
	ctx.r[4].s64 = ctx.r[10].s64 + 17688;
	// 826BE9B8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826BE9BC: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826BE9C0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826BE9C4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826BE9C8: 386A296C  addi r3, r10, 0x296c
	ctx.r[3].s64 = ctx.r[10].s64 + 10604;
	// 826BE9CC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826BE9D0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826BE9D4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826BE9D8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826BE9DC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826BE9E0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826BE9E4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826BE9E8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826BE9EC: 4BDA8435  bl 0x82466e20
	ctx.lr = 0x826BE9F0;
	sub_82466E20(ctx, base);
	// 826BE9F0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826BE9F4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826BE9F8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826BE9FC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826BEA00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826BEA00 size=112
    let mut pc: u32 = 0x826BEA00;
    'dispatch: loop {
        match pc {
            0x826BEA00 => {
    //   block [0x826BEA00..0x826BEA70)
	// 826BEA00: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826BEA04: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826BEA08: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826BEA0C: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 826BEA10: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826BEA14: 392A0FF8  addi r9, r10, 0xff8
	ctx.r[9].s64 = ctx.r[10].s64 + 4088;
	// 826BEA18: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826BEA1C: 390B01C0  addi r8, r11, 0x1c0
	ctx.r[8].s64 = ctx.r[11].s64 + 448;
	// 826BEA20: 38E00003  li r7, 3
	ctx.r[7].s64 = 3;
	// 826BEA24: 388A4538  addi r4, r10, 0x4538
	ctx.r[4].s64 = ctx.r[10].s64 + 17720;
	// 826BEA28: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826BEA2C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826BEA30: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826BEA34: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826BEA38: 386A299C  addi r3, r10, 0x299c
	ctx.r[3].s64 = ctx.r[10].s64 + 10652;
	// 826BEA3C: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826BEA40: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826BEA44: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826BEA48: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826BEA4C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826BEA50: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826BEA54: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826BEA58: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826BEA5C: 4BDA83C5  bl 0x82466e20
	ctx.lr = 0x826BEA60;
	sub_82466E20(ctx, base);
	// 826BEA60: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826BEA64: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826BEA68: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826BEA6C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826BEA70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826BEA70 size=112
    let mut pc: u32 = 0x826BEA70;
    'dispatch: loop {
        match pc {
            0x826BEA70 => {
    //   block [0x826BEA70..0x826BEAE0)
	// 826BEA70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826BEA74: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826BEA78: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826BEA7C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826BEA80: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826BEA84: 38AA28AC  addi r5, r10, 0x28ac
	ctx.r[5].s64 = ctx.r[10].s64 + 10412;
	// 826BEA88: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826BEA8C: 390B0208  addi r8, r11, 0x208
	ctx.r[8].s64 = ctx.r[11].s64 + 520;
	// 826BEA90: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826BEA94: 388A4554  addi r4, r10, 0x4554
	ctx.r[4].s64 = ctx.r[10].s64 + 17748;
	// 826BEA98: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826BEA9C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826BEAA0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826BEAA4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826BEAA8: 386A29CC  addi r3, r10, 0x29cc
	ctx.r[3].s64 = ctx.r[10].s64 + 10700;
	// 826BEAAC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826BEAB0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826BEAB4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826BEAB8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826BEABC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826BEAC0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826BEAC4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826BEAC8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826BEACC: 4BDA8355  bl 0x82466e20
	ctx.lr = 0x826BEAD0;
	sub_82466E20(ctx, base);
	// 826BEAD0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826BEAD4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826BEAD8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826BEADC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826BEAE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826BEAE0 size=112
    let mut pc: u32 = 0x826BEAE0;
    'dispatch: loop {
        match pc {
            0x826BEAE0 => {
    //   block [0x826BEAE0..0x826BEB50)
	// 826BEAE0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826BEAE4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826BEAE8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826BEAEC: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 826BEAF0: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826BEAF4: 392A1024  addi r9, r10, 0x1024
	ctx.r[9].s64 = ctx.r[10].s64 + 4132;
	// 826BEAF8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826BEAFC: 390B0228  addi r8, r11, 0x228
	ctx.r[8].s64 = ctx.r[11].s64 + 552;
	// 826BEB00: 38E00006  li r7, 6
	ctx.r[7].s64 = 6;
	// 826BEB04: 388A456C  addi r4, r10, 0x456c
	ctx.r[4].s64 = ctx.r[10].s64 + 17772;
	// 826BEB08: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826BEB0C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826BEB10: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826BEB14: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826BEB18: 386A29FC  addi r3, r10, 0x29fc
	ctx.r[3].s64 = ctx.r[10].s64 + 10748;
	// 826BEB1C: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826BEB20: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826BEB24: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826BEB28: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826BEB2C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826BEB30: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826BEB34: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826BEB38: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826BEB3C: 4BDA82E5  bl 0x82466e20
	ctx.lr = 0x826BEB40;
	sub_82466E20(ctx, base);
	// 826BEB40: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826BEB44: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826BEB48: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826BEB4C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826BEB50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826BEB50 size=112
    let mut pc: u32 = 0x826BEB50;
    'dispatch: loop {
        match pc {
            0x826BEB50 => {
    //   block [0x826BEB50..0x826BEBC0)
	// 826BEB50: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826BEB54: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826BEB58: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826BEB5C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826BEB60: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826BEB64: 38AA28AC  addi r5, r10, 0x28ac
	ctx.r[5].s64 = ctx.r[10].s64 + 10412;
	// 826BEB68: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826BEB6C: 390B02B8  addi r8, r11, 0x2b8
	ctx.r[8].s64 = ctx.r[11].s64 + 696;
	// 826BEB70: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826BEB74: 388A4590  addi r4, r10, 0x4590
	ctx.r[4].s64 = ctx.r[10].s64 + 17808;
	// 826BEB78: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826BEB7C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826BEB80: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826BEB84: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826BEB88: 386A2A2C  addi r3, r10, 0x2a2c
	ctx.r[3].s64 = ctx.r[10].s64 + 10796;
	// 826BEB8C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826BEB90: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826BEB94: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826BEB98: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826BEB9C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826BEBA0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826BEBA4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826BEBA8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826BEBAC: 4BDA8275  bl 0x82466e20
	ctx.lr = 0x826BEBB0;
	sub_82466E20(ctx, base);
	// 826BEBB0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826BEBB4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826BEBB8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826BEBBC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826BEBC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826BEBC0 size=112
    let mut pc: u32 = 0x826BEBC0;
    'dispatch: loop {
        match pc {
            0x826BEBC0 => {
    //   block [0x826BEBC0..0x826BEC30)
	// 826BEBC0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826BEBC4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826BEBC8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826BEBCC: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826BEBD0: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826BEBD4: 38AA2A8C  addi r5, r10, 0x2a8c
	ctx.r[5].s64 = ctx.r[10].s64 + 10892;
	// 826BEBD8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826BEBDC: 390B02D0  addi r8, r11, 0x2d0
	ctx.r[8].s64 = ctx.r[11].s64 + 720;
	// 826BEBE0: 39200005  li r9, 5
	ctx.r[9].s64 = 5;
	// 826BEBE4: 388A45B0  addi r4, r10, 0x45b0
	ctx.r[4].s64 = ctx.r[10].s64 + 17840;
	// 826BEBE8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826BEBEC: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826BEBF0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826BEBF4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826BEBF8: 386A2A5C  addi r3, r10, 0x2a5c
	ctx.r[3].s64 = ctx.r[10].s64 + 10844;
	// 826BEBFC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826BEC00: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826BEC04: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826BEC08: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826BEC0C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826BEC10: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826BEC14: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826BEC18: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826BEC1C: 4BDA8205  bl 0x82466e20
	ctx.lr = 0x826BEC20;
	sub_82466E20(ctx, base);
	// 826BEC20: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826BEC24: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826BEC28: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826BEC2C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826BEC30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826BEC30 size=100
    let mut pc: u32 = 0x826BEC30;
    'dispatch: loop {
        match pc {
            0x826BEC30 => {
    //   block [0x826BEC30..0x826BEC94)
	// 826BEC30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826BEC34: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826BEC38: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826BEC3C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826BEC40: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826BEC44: 38AA0DAC  addi r5, r10, 0xdac
	ctx.r[5].s64 = ctx.r[10].s64 + 3500;
	// 826BEC48: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826BEC4C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826BEC50: 388A45C8  addi r4, r10, 0x45c8
	ctx.r[4].s64 = ctx.r[10].s64 + 17864;
	// 826BEC54: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826BEC58: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826BEC5C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826BEC60: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826BEC64: 386A2A8C  addi r3, r10, 0x2a8c
	ctx.r[3].s64 = ctx.r[10].s64 + 10892;
	// 826BEC68: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826BEC6C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826BEC70: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826BEC74: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826BEC78: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826BEC7C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826BEC80: 4BDA81A1  bl 0x82466e20
	ctx.lr = 0x826BEC84;
	sub_82466E20(ctx, base);
	// 826BEC84: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826BEC88: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826BEC8C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826BEC90: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826BEC98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826BEC98 size=24
    let mut pc: u32 = 0x826BEC98;
    'dispatch: loop {
        match pc {
            0x826BEC98 => {
    //   block [0x826BEC98..0x826BECB0)
	// 826BEC98: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826BEC9C: 3D40827E  lis r10, -0x7d82
	ctx.r[10].s64 = -2105671680;
	// 826BECA0: 394A3440  addi r10, r10, 0x3440
	ctx.r[10].s64 = ctx.r[10].s64 + 13376;
	// 826BECA4: 816B0224  lwz r11, 0x224(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(548 as u32) ) } as u64;
	// 826BECA8: 916A0050  stw r11, 0x50(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 826BECAC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826BECB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826BECB0 size=116
    let mut pc: u32 = 0x826BECB0;
    'dispatch: loop {
        match pc {
            0x826BECB0 => {
    //   block [0x826BECB0..0x826BED24)
	// 826BECB0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826BECB4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826BECB8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826BECBC: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826BECC0: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 826BECC4: 390B3440  addi r8, r11, 0x3440
	ctx.r[8].s64 = ctx.r[11].s64 + 13376;
	// 826BECC8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826BECCC: 392A1060  addi r9, r10, 0x1060
	ctx.r[9].s64 = ctx.r[10].s64 + 4192;
	// 826BECD0: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826BECD4: 38E00005  li r7, 5
	ctx.r[7].s64 = 5;
	// 826BECD8: 38AA28AC  addi r5, r10, 0x28ac
	ctx.r[5].s64 = ctx.r[10].s64 + 10412;
	// 826BECDC: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826BECE0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826BECE4: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826BECE8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826BECEC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826BECF0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826BECF4: 3D608298  lis r11, -0x7d68
	ctx.r[11].s64 = -2103967744;
	// 826BECF8: 388A45EC  addi r4, r10, 0x45ec
	ctx.r[4].s64 = ctx.r[10].s64 + 17900;
	// 826BECFC: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826BED00: 386B2ABC  addi r3, r11, 0x2abc
	ctx.r[3].s64 = ctx.r[11].s64 + 10940;
	// 826BED04: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826BED08: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826BED0C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826BED10: 4BDA8111  bl 0x82466e20
	ctx.lr = 0x826BED14;
	sub_82466E20(ctx, base);
	// 826BED14: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826BED18: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826BED1C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826BED20: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826BED28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826BED28 size=108
    let mut pc: u32 = 0x826BED28;
    'dispatch: loop {
        match pc {
            0x826BED28 => {
    //   block [0x826BED28..0x826BED94)
	// 826BED28: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826BED2C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826BED30: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826BED34: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826BED38: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826BED3C: 38EB0348  addi r7, r11, 0x348
	ctx.r[7].s64 = ctx.r[11].s64 + 840;
	// 826BED40: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826BED44: 388A4608  addi r4, r10, 0x4608
	ctx.r[4].s64 = ctx.r[10].s64 + 17928;
	// 826BED48: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826BED4C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826BED50: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826BED54: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826BED58: 386A2AEC  addi r3, r10, 0x2aec
	ctx.r[3].s64 = ctx.r[10].s64 + 10988;
	// 826BED5C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826BED60: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826BED64: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826BED68: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826BED6C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826BED70: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826BED74: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826BED78: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826BED7C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826BED80: 4BDA80A1  bl 0x82466e20
	ctx.lr = 0x826BED84;
	sub_82466E20(ctx, base);
	// 826BED84: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826BED88: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826BED8C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826BED90: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826BED98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826BED98 size=112
    let mut pc: u32 = 0x826BED98;
    'dispatch: loop {
        match pc {
            0x826BED98 => {
    //   block [0x826BED98..0x826BEE08)
	// 826BED98: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826BED9C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826BEDA0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826BEDA4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826BEDA8: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826BEDAC: 38AA28AC  addi r5, r10, 0x28ac
	ctx.r[5].s64 = ctx.r[10].s64 + 10412;
	// 826BEDB0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826BEDB4: 390B0378  addi r8, r11, 0x378
	ctx.r[8].s64 = ctx.r[11].s64 + 888;
	// 826BEDB8: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826BEDBC: 388A462C  addi r4, r10, 0x462c
	ctx.r[4].s64 = ctx.r[10].s64 + 17964;
	// 826BEDC0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826BEDC4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826BEDC8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826BEDCC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826BEDD0: 386A2B1C  addi r3, r10, 0x2b1c
	ctx.r[3].s64 = ctx.r[10].s64 + 11036;
	// 826BEDD4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826BEDD8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826BEDDC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826BEDE0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826BEDE4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826BEDE8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826BEDEC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826BEDF0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826BEDF4: 4BDA802D  bl 0x82466e20
	ctx.lr = 0x826BEDF8;
	sub_82466E20(ctx, base);
	// 826BEDF8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826BEDFC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826BEE00: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826BEE04: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826BEE08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826BEE08 size=112
    let mut pc: u32 = 0x826BEE08;
    'dispatch: loop {
        match pc {
            0x826BEE08 => {
    //   block [0x826BEE08..0x826BEE78)
	// 826BEE08: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826BEE0C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826BEE10: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826BEE14: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 826BEE18: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826BEE1C: 392A1084  addi r9, r10, 0x1084
	ctx.r[9].s64 = ctx.r[10].s64 + 4228;
	// 826BEE20: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826BEE24: 390B0398  addi r8, r11, 0x398
	ctx.r[8].s64 = ctx.r[11].s64 + 920;
	// 826BEE28: 38E00007  li r7, 7
	ctx.r[7].s64 = 7;
	// 826BEE2C: 388A464C  addi r4, r10, 0x464c
	ctx.r[4].s64 = ctx.r[10].s64 + 17996;
	// 826BEE30: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826BEE34: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826BEE38: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826BEE3C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826BEE40: 386A2B4C  addi r3, r10, 0x2b4c
	ctx.r[3].s64 = ctx.r[10].s64 + 11084;
	// 826BEE44: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826BEE48: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826BEE4C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826BEE50: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826BEE54: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826BEE58: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826BEE5C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826BEE60: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826BEE64: 4BDA7FBD  bl 0x82466e20
	ctx.lr = 0x826BEE68;
	sub_82466E20(ctx, base);
	// 826BEE68: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826BEE6C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826BEE70: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826BEE74: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826BEE78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826BEE78 size=112
    let mut pc: u32 = 0x826BEE78;
    'dispatch: loop {
        match pc {
            0x826BEE78 => {
    //   block [0x826BEE78..0x826BEEE8)
	// 826BEE78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826BEE7C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826BEE80: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826BEE84: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826BEE88: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826BEE8C: 38AA28AC  addi r5, r10, 0x28ac
	ctx.r[5].s64 = ctx.r[10].s64 + 10412;
	// 826BEE90: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826BEE94: 390B0440  addi r8, r11, 0x440
	ctx.r[8].s64 = ctx.r[11].s64 + 1088;
	// 826BEE98: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826BEE9C: 388A466C  addi r4, r10, 0x466c
	ctx.r[4].s64 = ctx.r[10].s64 + 18028;
	// 826BEEA0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826BEEA4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826BEEA8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826BEEAC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826BEEB0: 386A2B7C  addi r3, r10, 0x2b7c
	ctx.r[3].s64 = ctx.r[10].s64 + 11132;
	// 826BEEB4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826BEEB8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826BEEBC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826BEEC0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826BEEC4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826BEEC8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826BEECC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826BEED0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826BEED4: 4BDA7F4D  bl 0x82466e20
	ctx.lr = 0x826BEED8;
	sub_82466E20(ctx, base);
	// 826BEED8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826BEEDC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826BEEE0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826BEEE4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826BEEE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826BEEE8 size=108
    let mut pc: u32 = 0x826BEEE8;
    'dispatch: loop {
        match pc {
            0x826BEEE8 => {
    //   block [0x826BEEE8..0x826BEF54)
	// 826BEEE8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826BEEEC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826BEEF0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826BEEF4: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826BEEF8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826BEEFC: 38EB0458  addi r7, r11, 0x458
	ctx.r[7].s64 = ctx.r[11].s64 + 1112;
	// 826BEF00: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826BEF04: 388A4688  addi r4, r10, 0x4688
	ctx.r[4].s64 = ctx.r[10].s64 + 18056;
	// 826BEF08: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826BEF0C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826BEF10: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826BEF14: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826BEF18: 386A2BAC  addi r3, r10, 0x2bac
	ctx.r[3].s64 = ctx.r[10].s64 + 11180;
	// 826BEF1C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826BEF20: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826BEF24: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826BEF28: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826BEF2C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826BEF30: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826BEF34: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826BEF38: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826BEF3C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826BEF40: 4BDA7EE1  bl 0x82466e20
	ctx.lr = 0x826BEF44;
	sub_82466E20(ctx, base);
	// 826BEF44: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826BEF48: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826BEF4C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826BEF50: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826BEF58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826BEF58 size=112
    let mut pc: u32 = 0x826BEF58;
    'dispatch: loop {
        match pc {
            0x826BEF58 => {
    //   block [0x826BEF58..0x826BEFC8)
	// 826BEF58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826BEF5C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826BEF60: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826BEF64: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826BEF68: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826BEF6C: 38AA28AC  addi r5, r10, 0x28ac
	ctx.r[5].s64 = ctx.r[10].s64 + 10412;
	// 826BEF70: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826BEF74: 390B0488  addi r8, r11, 0x488
	ctx.r[8].s64 = ctx.r[11].s64 + 1160;
	// 826BEF78: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826BEF7C: 388A46AC  addi r4, r10, 0x46ac
	ctx.r[4].s64 = ctx.r[10].s64 + 18092;
	// 826BEF80: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826BEF84: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826BEF88: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826BEF8C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826BEF90: 386A2BDC  addi r3, r10, 0x2bdc
	ctx.r[3].s64 = ctx.r[10].s64 + 11228;
	// 826BEF94: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826BEF98: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826BEF9C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826BEFA0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826BEFA4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826BEFA8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826BEFAC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826BEFB0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826BEFB4: 4BDA7E6D  bl 0x82466e20
	ctx.lr = 0x826BEFB8;
	sub_82466E20(ctx, base);
	// 826BEFB8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826BEFBC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826BEFC0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826BEFC4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826BEFC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826BEFC8 size=112
    let mut pc: u32 = 0x826BEFC8;
    'dispatch: loop {
        match pc {
            0x826BEFC8 => {
    //   block [0x826BEFC8..0x826BF038)
	// 826BEFC8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826BEFCC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826BEFD0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826BEFD4: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 826BEFD8: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826BEFDC: 392A10B8  addi r9, r10, 0x10b8
	ctx.r[9].s64 = ctx.r[10].s64 + 4280;
	// 826BEFE0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826BEFE4: 390B04A0  addi r8, r11, 0x4a0
	ctx.r[8].s64 = ctx.r[11].s64 + 1184;
	// 826BEFE8: 38E00007  li r7, 7
	ctx.r[7].s64 = 7;
	// 826BEFEC: 388A46C8  addi r4, r10, 0x46c8
	ctx.r[4].s64 = ctx.r[10].s64 + 18120;
	// 826BEFF0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826BEFF4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826BEFF8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826BEFFC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826BF000: 386A2C0C  addi r3, r10, 0x2c0c
	ctx.r[3].s64 = ctx.r[10].s64 + 11276;
	// 826BF004: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826BF008: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826BF00C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826BF010: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826BF014: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826BF018: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826BF01C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826BF020: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826BF024: 4BDA7DFD  bl 0x82466e20
	ctx.lr = 0x826BF028;
	sub_82466E20(ctx, base);
	// 826BF028: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826BF02C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826BF030: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826BF034: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826BF038(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826BF038 size=112
    let mut pc: u32 = 0x826BF038;
    'dispatch: loop {
        match pc {
            0x826BF038 => {
    //   block [0x826BF038..0x826BF0A8)
	// 826BF038: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826BF03C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826BF040: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826BF044: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826BF048: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826BF04C: 38AA28AC  addi r5, r10, 0x28ac
	ctx.r[5].s64 = ctx.r[10].s64 + 10412;
	// 826BF050: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826BF054: 390B0548  addi r8, r11, 0x548
	ctx.r[8].s64 = ctx.r[11].s64 + 1352;
	// 826BF058: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 826BF05C: 388A46E4  addi r4, r10, 0x46e4
	ctx.r[4].s64 = ctx.r[10].s64 + 18148;
	// 826BF060: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826BF064: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826BF068: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826BF06C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826BF070: 386A2C3C  addi r3, r10, 0x2c3c
	ctx.r[3].s64 = ctx.r[10].s64 + 11324;
	// 826BF074: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826BF078: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826BF07C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826BF080: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826BF084: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826BF088: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826BF08C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826BF090: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826BF094: 4BDA7D8D  bl 0x82466e20
	ctx.lr = 0x826BF098;
	sub_82466E20(ctx, base);
	// 826BF098: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826BF09C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826BF0A0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826BF0A4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826BF0A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826BF0A8 size=112
    let mut pc: u32 = 0x826BF0A8;
    'dispatch: loop {
        match pc {
            0x826BF0A8 => {
    //   block [0x826BF0A8..0x826BF118)
	// 826BF0A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826BF0AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826BF0B0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826BF0B4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826BF0B8: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826BF0BC: 38AA28AC  addi r5, r10, 0x28ac
	ctx.r[5].s64 = ctx.r[10].s64 + 10412;
	// 826BF0C0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826BF0C4: 390B0590  addi r8, r11, 0x590
	ctx.r[8].s64 = ctx.r[11].s64 + 1424;
	// 826BF0C8: 39200009  li r9, 9
	ctx.r[9].s64 = 9;
	// 826BF0CC: 388A46FC  addi r4, r10, 0x46fc
	ctx.r[4].s64 = ctx.r[10].s64 + 18172;
	// 826BF0D0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826BF0D4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826BF0D8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826BF0DC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826BF0E0: 386A2C6C  addi r3, r10, 0x2c6c
	ctx.r[3].s64 = ctx.r[10].s64 + 11372;
	// 826BF0E4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826BF0E8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826BF0EC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826BF0F0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826BF0F4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826BF0F8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826BF0FC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826BF100: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826BF104: 4BDA7D1D  bl 0x82466e20
	ctx.lr = 0x826BF108;
	sub_82466E20(ctx, base);
	// 826BF108: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826BF10C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826BF110: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826BF114: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826BF118(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826BF118 size=100
    let mut pc: u32 = 0x826BF118;
    'dispatch: loop {
        match pc {
            0x826BF118 => {
    //   block [0x826BF118..0x826BF17C)
	// 826BF118: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826BF11C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826BF120: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826BF124: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826BF128: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826BF12C: 38AA28AC  addi r5, r10, 0x28ac
	ctx.r[5].s64 = ctx.r[10].s64 + 10412;
	// 826BF130: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826BF134: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826BF138: 388A4718  addi r4, r10, 0x4718
	ctx.r[4].s64 = ctx.r[10].s64 + 18200;
	// 826BF13C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826BF140: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826BF144: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826BF148: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826BF14C: 386A2C9C  addi r3, r10, 0x2c9c
	ctx.r[3].s64 = ctx.r[10].s64 + 11420;
	// 826BF150: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826BF154: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826BF158: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826BF15C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826BF160: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826BF164: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826BF168: 4BDA7CB9  bl 0x82466e20
	ctx.lr = 0x826BF16C;
	sub_82466E20(ctx, base);
	// 826BF16C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826BF170: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826BF174: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826BF178: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826BF180(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826BF180 size=112
    let mut pc: u32 = 0x826BF180;
    'dispatch: loop {
        match pc {
            0x826BF180 => {
    //   block [0x826BF180..0x826BF1F0)
	// 826BF180: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826BF184: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826BF188: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826BF18C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826BF190: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826BF194: 38AA290C  addi r5, r10, 0x290c
	ctx.r[5].s64 = ctx.r[10].s64 + 10508;
	// 826BF198: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826BF19C: 390B0668  addi r8, r11, 0x668
	ctx.r[8].s64 = ctx.r[11].s64 + 1640;
	// 826BF1A0: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826BF1A4: 388A4730  addi r4, r10, 0x4730
	ctx.r[4].s64 = ctx.r[10].s64 + 18224;
	// 826BF1A8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826BF1AC: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826BF1B0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826BF1B4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826BF1B8: 386A2CCC  addi r3, r10, 0x2ccc
	ctx.r[3].s64 = ctx.r[10].s64 + 11468;
	// 826BF1BC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826BF1C0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826BF1C4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826BF1C8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826BF1CC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826BF1D0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826BF1D4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826BF1D8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826BF1DC: 4BDA7C45  bl 0x82466e20
	ctx.lr = 0x826BF1E0;
	sub_82466E20(ctx, base);
	// 826BF1E0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826BF1E4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826BF1E8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826BF1EC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826BF1F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826BF1F0 size=112
    let mut pc: u32 = 0x826BF1F0;
    'dispatch: loop {
        match pc {
            0x826BF1F0 => {
    //   block [0x826BF1F0..0x826BF260)
	// 826BF1F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826BF1F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826BF1F8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826BF1FC: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826BF200: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826BF204: 38AA275C  addi r5, r10, 0x275c
	ctx.r[5].s64 = ctx.r[10].s64 + 10076;
	// 826BF208: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826BF20C: 390B0698  addi r8, r11, 0x698
	ctx.r[8].s64 = ctx.r[11].s64 + 1688;
	// 826BF210: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826BF214: 388A474C  addi r4, r10, 0x474c
	ctx.r[4].s64 = ctx.r[10].s64 + 18252;
	// 826BF218: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826BF21C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826BF220: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826BF224: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826BF228: 386A2CFC  addi r3, r10, 0x2cfc
	ctx.r[3].s64 = ctx.r[10].s64 + 11516;
	// 826BF22C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826BF230: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826BF234: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826BF238: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826BF23C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826BF240: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826BF244: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826BF248: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826BF24C: 4BDA7BD5  bl 0x82466e20
	ctx.lr = 0x826BF250;
	sub_82466E20(ctx, base);
	// 826BF250: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826BF254: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826BF258: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826BF25C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826BF260(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826BF260 size=108
    let mut pc: u32 = 0x826BF260;
    'dispatch: loop {
        match pc {
            0x826BF260 => {
    //   block [0x826BF260..0x826BF2CC)
	// 826BF260: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826BF264: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826BF268: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826BF26C: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826BF270: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826BF274: 38EB06B0  addi r7, r11, 0x6b0
	ctx.r[7].s64 = ctx.r[11].s64 + 1712;
	// 826BF278: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826BF27C: 388A476C  addi r4, r10, 0x476c
	ctx.r[4].s64 = ctx.r[10].s64 + 18284;
	// 826BF280: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826BF284: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826BF288: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826BF28C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826BF290: 386A2D2C  addi r3, r10, 0x2d2c
	ctx.r[3].s64 = ctx.r[10].s64 + 11564;
	// 826BF294: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826BF298: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826BF29C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826BF2A0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826BF2A4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826BF2A8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826BF2AC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826BF2B0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826BF2B4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826BF2B8: 4BDA7B69  bl 0x82466e20
	ctx.lr = 0x826BF2BC;
	sub_82466E20(ctx, base);
	// 826BF2BC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826BF2C0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826BF2C4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826BF2C8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826BF2D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826BF2D0 size=112
    let mut pc: u32 = 0x826BF2D0;
    'dispatch: loop {
        match pc {
            0x826BF2D0 => {
    //   block [0x826BF2D0..0x826BF340)
	// 826BF2D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826BF2D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826BF2D8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826BF2DC: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826BF2E0: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826BF2E4: 38AA2C9C  addi r5, r10, 0x2c9c
	ctx.r[5].s64 = ctx.r[10].s64 + 11420;
	// 826BF2E8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826BF2EC: 390B06E0  addi r8, r11, 0x6e0
	ctx.r[8].s64 = ctx.r[11].s64 + 1760;
	// 826BF2F0: 39200006  li r9, 6
	ctx.r[9].s64 = 6;
	// 826BF2F4: 388A4790  addi r4, r10, 0x4790
	ctx.r[4].s64 = ctx.r[10].s64 + 18320;
	// 826BF2F8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826BF2FC: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826BF300: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826BF304: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826BF308: 386A2D5C  addi r3, r10, 0x2d5c
	ctx.r[3].s64 = ctx.r[10].s64 + 11612;
	// 826BF30C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826BF310: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826BF314: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826BF318: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826BF31C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826BF320: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826BF324: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826BF328: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826BF32C: 4BDA7AF5  bl 0x82466e20
	ctx.lr = 0x826BF330;
	sub_82466E20(ctx, base);
	// 826BF330: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826BF334: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826BF338: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826BF33C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826BF340(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826BF340 size=112
    let mut pc: u32 = 0x826BF340;
    'dispatch: loop {
        match pc {
            0x826BF340 => {
    //   block [0x826BF340..0x826BF3B0)
	// 826BF340: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826BF344: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826BF348: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826BF34C: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 826BF350: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826BF354: 392A10E4  addi r9, r10, 0x10e4
	ctx.r[9].s64 = ctx.r[10].s64 + 4324;
	// 826BF358: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826BF35C: 390B0778  addi r8, r11, 0x778
	ctx.r[8].s64 = ctx.r[11].s64 + 1912;
	// 826BF360: 38E00003  li r7, 3
	ctx.r[7].s64 = 3;
	// 826BF364: 388A47A8  addi r4, r10, 0x47a8
	ctx.r[4].s64 = ctx.r[10].s64 + 18344;
	// 826BF368: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826BF36C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826BF370: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826BF374: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826BF378: 386A2D8C  addi r3, r10, 0x2d8c
	ctx.r[3].s64 = ctx.r[10].s64 + 11660;
	// 826BF37C: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826BF380: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826BF384: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826BF388: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826BF38C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826BF390: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826BF394: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826BF398: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826BF39C: 4BDA7A85  bl 0x82466e20
	ctx.lr = 0x826BF3A0;
	sub_82466E20(ctx, base);
	// 826BF3A0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826BF3A4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826BF3A8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826BF3AC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826BF3B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826BF3B0 size=112
    let mut pc: u32 = 0x826BF3B0;
    'dispatch: loop {
        match pc {
            0x826BF3B0 => {
    //   block [0x826BF3B0..0x826BF420)
	// 826BF3B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826BF3B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826BF3B8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826BF3BC: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826BF3C0: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826BF3C4: 38AA28AC  addi r5, r10, 0x28ac
	ctx.r[5].s64 = ctx.r[10].s64 + 10412;
	// 826BF3C8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826BF3CC: 390B07C0  addi r8, r11, 0x7c0
	ctx.r[8].s64 = ctx.r[11].s64 + 1984;
	// 826BF3D0: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826BF3D4: 388A47C0  addi r4, r10, 0x47c0
	ctx.r[4].s64 = ctx.r[10].s64 + 18368;
	// 826BF3D8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826BF3DC: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826BF3E0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826BF3E4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826BF3E8: 386A2DBC  addi r3, r10, 0x2dbc
	ctx.r[3].s64 = ctx.r[10].s64 + 11708;
	// 826BF3EC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826BF3F0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826BF3F4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826BF3F8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826BF3FC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826BF400: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826BF404: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826BF408: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826BF40C: 4BDA7A15  bl 0x82466e20
	ctx.lr = 0x826BF410;
	sub_82466E20(ctx, base);
	// 826BF410: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826BF414: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826BF418: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826BF41C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826BF420(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826BF420 size=108
    let mut pc: u32 = 0x826BF420;
    'dispatch: loop {
        match pc {
            0x826BF420 => {
    //   block [0x826BF420..0x826BF48C)
	// 826BF420: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826BF424: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826BF428: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826BF42C: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826BF430: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826BF434: 38EB07D8  addi r7, r11, 0x7d8
	ctx.r[7].s64 = ctx.r[11].s64 + 2008;
	// 826BF438: 39000006  li r8, 6
	ctx.r[8].s64 = 6;
	// 826BF43C: 388A47D4  addi r4, r10, 0x47d4
	ctx.r[4].s64 = ctx.r[10].s64 + 18388;
	// 826BF440: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826BF444: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826BF448: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826BF44C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826BF450: 386A2DEC  addi r3, r10, 0x2dec
	ctx.r[3].s64 = ctx.r[10].s64 + 11756;
	// 826BF454: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826BF458: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826BF45C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826BF460: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826BF464: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826BF468: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826BF46C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826BF470: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826BF474: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826BF478: 4BDA79A9  bl 0x82466e20
	ctx.lr = 0x826BF47C;
	sub_82466E20(ctx, base);
	// 826BF47C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826BF480: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826BF484: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826BF488: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826BF490(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826BF490 size=116
    let mut pc: u32 = 0x826BF490;
    'dispatch: loop {
        match pc {
            0x826BF490 => {
    //   block [0x826BF490..0x826BF504)
	// 826BF490: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826BF494: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826BF498: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826BF49C: 3D40827E  lis r10, -0x7d82
	ctx.r[10].s64 = -2105671680;
	// 826BF4A0: 38E00009  li r7, 9
	ctx.r[7].s64 = 9;
	// 826BF4A4: 390A0868  addi r8, r10, 0x868
	ctx.r[8].s64 = ctx.r[10].s64 + 2152;
	// 826BF4A8: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826BF4AC: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 826BF4B0: 38AA2C9C  addi r5, r10, 0x2c9c
	ctx.r[5].s64 = ctx.r[10].s64 + 11420;
	// 826BF4B4: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826BF4B8: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826BF4BC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826BF4C0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826BF4C4: 388A47F8  addi r4, r10, 0x47f8
	ctx.r[4].s64 = ctx.r[10].s64 + 18424;
	// 826BF4C8: 396B10F8  addi r11, r11, 0x10f8
	ctx.r[11].s64 = ctx.r[11].s64 + 4344;
	// 826BF4CC: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826BF4D0: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826BF4D4: 386A2E1C  addi r3, r10, 0x2e1c
	ctx.r[3].s64 = ctx.r[10].s64 + 11804;
	// 826BF4D8: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 826BF4DC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826BF4E0: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 826BF4E4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826BF4E8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826BF4EC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826BF4F0: 4BDA7931  bl 0x82466e20
	ctx.lr = 0x826BF4F4;
	sub_82466E20(ctx, base);
	// 826BF4F4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826BF4F8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826BF4FC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826BF500: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826BF508(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826BF508 size=108
    let mut pc: u32 = 0x826BF508;
    'dispatch: loop {
        match pc {
            0x826BF508 => {
    //   block [0x826BF508..0x826BF574)
	// 826BF508: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826BF50C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826BF510: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826BF514: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826BF518: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826BF51C: 38EB0940  addi r7, r11, 0x940
	ctx.r[7].s64 = ctx.r[11].s64 + 2368;
	// 826BF520: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 826BF524: 388A480C  addi r4, r10, 0x480c
	ctx.r[4].s64 = ctx.r[10].s64 + 18444;
	// 826BF528: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826BF52C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826BF530: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826BF534: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826BF538: 386A2E4C  addi r3, r10, 0x2e4c
	ctx.r[3].s64 = ctx.r[10].s64 + 11852;
	// 826BF53C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826BF540: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826BF544: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826BF548: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826BF54C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826BF550: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826BF554: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826BF558: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826BF55C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826BF560: 4BDA78C1  bl 0x82466e20
	ctx.lr = 0x826BF564;
	sub_82466E20(ctx, base);
	// 826BF564: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826BF568: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826BF56C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826BF570: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826BF578(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826BF578 size=112
    let mut pc: u32 = 0x826BF578;
    'dispatch: loop {
        match pc {
            0x826BF578 => {
    //   block [0x826BF578..0x826BF5E8)
	// 826BF578: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826BF57C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826BF580: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826BF584: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826BF588: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826BF58C: 38AA2C9C  addi r5, r10, 0x2c9c
	ctx.r[5].s64 = ctx.r[10].s64 + 11420;
	// 826BF590: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826BF594: 390B0988  addi r8, r11, 0x988
	ctx.r[8].s64 = ctx.r[11].s64 + 2440;
	// 826BF598: 39200005  li r9, 5
	ctx.r[9].s64 = 5;
	// 826BF59C: 388A4834  addi r4, r10, 0x4834
	ctx.r[4].s64 = ctx.r[10].s64 + 18484;
	// 826BF5A0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826BF5A4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826BF5A8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826BF5AC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826BF5B0: 386A2E7C  addi r3, r10, 0x2e7c
	ctx.r[3].s64 = ctx.r[10].s64 + 11900;
	// 826BF5B4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826BF5B8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826BF5BC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826BF5C0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826BF5C4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826BF5C8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826BF5CC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826BF5D0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826BF5D4: 4BDA784D  bl 0x82466e20
	ctx.lr = 0x826BF5D8;
	sub_82466E20(ctx, base);
	// 826BF5D8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826BF5DC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826BF5E0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826BF5E4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826BF5E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826BF5E8 size=112
    let mut pc: u32 = 0x826BF5E8;
    'dispatch: loop {
        match pc {
            0x826BF5E8 => {
    //   block [0x826BF5E8..0x826BF658)
	// 826BF5E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826BF5EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826BF5F0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826BF5F4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826BF5F8: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826BF5FC: 38AA28AC  addi r5, r10, 0x28ac
	ctx.r[5].s64 = ctx.r[10].s64 + 10412;
	// 826BF600: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826BF604: 390B0A00  addi r8, r11, 0xa00
	ctx.r[8].s64 = ctx.r[11].s64 + 2560;
	// 826BF608: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826BF60C: 388A484C  addi r4, r10, 0x484c
	ctx.r[4].s64 = ctx.r[10].s64 + 18508;
	// 826BF610: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826BF614: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826BF618: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826BF61C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826BF620: 386A2EAC  addi r3, r10, 0x2eac
	ctx.r[3].s64 = ctx.r[10].s64 + 11948;
	// 826BF624: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826BF628: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826BF62C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826BF630: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826BF634: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826BF638: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826BF63C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826BF640: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826BF644: 4BDA77DD  bl 0x82466e20
	ctx.lr = 0x826BF648;
	sub_82466E20(ctx, base);
	// 826BF648: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826BF64C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826BF650: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826BF654: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826BF658(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826BF658 size=108
    let mut pc: u32 = 0x826BF658;
    'dispatch: loop {
        match pc {
            0x826BF658 => {
    //   block [0x826BF658..0x826BF6C4)
	// 826BF658: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826BF65C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826BF660: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826BF664: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826BF668: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826BF66C: 38EB0A30  addi r7, r11, 0xa30
	ctx.r[7].s64 = ctx.r[11].s64 + 2608;
	// 826BF670: 39000005  li r8, 5
	ctx.r[8].s64 = 5;
	// 826BF674: 388A4890  addi r4, r10, 0x4890
	ctx.r[4].s64 = ctx.r[10].s64 + 18576;
	// 826BF678: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826BF67C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826BF680: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826BF684: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826BF688: 386A2EDC  addi r3, r10, 0x2edc
	ctx.r[3].s64 = ctx.r[10].s64 + 11996;
	// 826BF68C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826BF690: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826BF694: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826BF698: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826BF69C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826BF6A0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826BF6A4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826BF6A8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826BF6AC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826BF6B0: 4BDA7771  bl 0x82466e20
	ctx.lr = 0x826BF6B4;
	sub_82466E20(ctx, base);
	// 826BF6B4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826BF6B8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826BF6BC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826BF6C0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826BF6C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826BF6C8 size=112
    let mut pc: u32 = 0x826BF6C8;
    'dispatch: loop {
        match pc {
            0x826BF6C8 => {
    //   block [0x826BF6C8..0x826BF738)
	// 826BF6C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826BF6CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826BF6D0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826BF6D4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826BF6D8: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826BF6DC: 38AA28AC  addi r5, r10, 0x28ac
	ctx.r[5].s64 = ctx.r[10].s64 + 10412;
	// 826BF6E0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826BF6E4: 390B0AA8  addi r8, r11, 0xaa8
	ctx.r[8].s64 = ctx.r[11].s64 + 2728;
	// 826BF6E8: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 826BF6EC: 388A48B0  addi r4, r10, 0x48b0
	ctx.r[4].s64 = ctx.r[10].s64 + 18608;
	// 826BF6F0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826BF6F4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826BF6F8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826BF6FC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826BF700: 386A2F0C  addi r3, r10, 0x2f0c
	ctx.r[3].s64 = ctx.r[10].s64 + 12044;
	// 826BF704: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826BF708: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826BF70C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826BF710: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826BF714: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826BF718: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826BF71C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826BF720: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826BF724: 4BDA76FD  bl 0x82466e20
	ctx.lr = 0x826BF728;
	sub_82466E20(ctx, base);
	// 826BF728: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826BF72C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826BF730: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826BF734: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826BF738(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826BF738 size=24
    let mut pc: u32 = 0x826BF738;
    'dispatch: loop {
        match pc {
            0x826BF738 => {
    //   block [0x826BF738..0x826BF750)
	// 826BF738: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826BF73C: 3D40827E  lis r10, -0x7d82
	ctx.r[10].s64 = -2105671680;
	// 826BF740: 394A34B8  addi r10, r10, 0x34b8
	ctx.r[10].s64 = ctx.r[10].s64 + 13496;
	// 826BF744: 816B0774  lwz r11, 0x774(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(1908 as u32) ) } as u64;
	// 826BF748: 916A0008  stw r11, 8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 826BF74C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826BF750(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826BF750 size=116
    let mut pc: u32 = 0x826BF750;
    'dispatch: loop {
        match pc {
            0x826BF750 => {
    //   block [0x826BF750..0x826BF7C4)
	// 826BF750: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826BF754: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826BF758: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826BF75C: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826BF760: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 826BF764: 390B34B8  addi r8, r11, 0x34b8
	ctx.r[8].s64 = ctx.r[11].s64 + 13496;
	// 826BF768: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826BF76C: 392A1154  addi r9, r10, 0x1154
	ctx.r[9].s64 = ctx.r[10].s64 + 4436;
	// 826BF770: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826BF774: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
	// 826BF778: 38AA0DAC  addi r5, r10, 0xdac
	ctx.r[5].s64 = ctx.r[10].s64 + 3500;
	// 826BF77C: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826BF780: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826BF784: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826BF788: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826BF78C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826BF790: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826BF794: 3D608298  lis r11, -0x7d68
	ctx.r[11].s64 = -2103967744;
	// 826BF798: 388A48CC  addi r4, r10, 0x48cc
	ctx.r[4].s64 = ctx.r[10].s64 + 18636;
	// 826BF79C: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826BF7A0: 386B2F3C  addi r3, r11, 0x2f3c
	ctx.r[3].s64 = ctx.r[11].s64 + 12092;
	// 826BF7A4: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826BF7A8: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826BF7AC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826BF7B0: 4BDA7671  bl 0x82466e20
	ctx.lr = 0x826BF7B4;
	sub_82466E20(ctx, base);
	// 826BF7B4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826BF7B8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826BF7BC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826BF7C0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826BF7C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826BF7C8 size=112
    let mut pc: u32 = 0x826BF7C8;
    'dispatch: loop {
        match pc {
            0x826BF7C8 => {
    //   block [0x826BF7C8..0x826BF838)
	// 826BF7C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826BF7CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826BF7D0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826BF7D4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826BF7D8: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826BF7DC: 38AA2F3C  addi r5, r10, 0x2f3c
	ctx.r[5].s64 = ctx.r[10].s64 + 12092;
	// 826BF7E0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826BF7E4: 390B0AF0  addi r8, r11, 0xaf0
	ctx.r[8].s64 = ctx.r[11].s64 + 2800;
	// 826BF7E8: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826BF7EC: 388A48E0  addi r4, r10, 0x48e0
	ctx.r[4].s64 = ctx.r[10].s64 + 18656;
	// 826BF7F0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826BF7F4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826BF7F8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826BF7FC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826BF800: 386A2F6C  addi r3, r10, 0x2f6c
	ctx.r[3].s64 = ctx.r[10].s64 + 12140;
	// 826BF804: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826BF808: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826BF80C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826BF810: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826BF814: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826BF818: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826BF81C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826BF820: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826BF824: 4BDA75FD  bl 0x82466e20
	ctx.lr = 0x826BF828;
	sub_82466E20(ctx, base);
	// 826BF828: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826BF82C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826BF830: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826BF834: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826BF838(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826BF838 size=112
    let mut pc: u32 = 0x826BF838;
    'dispatch: loop {
        match pc {
            0x826BF838 => {
    //   block [0x826BF838..0x826BF8A8)
	// 826BF838: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826BF83C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826BF840: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826BF844: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826BF848: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826BF84C: 38AA2F6C  addi r5, r10, 0x2f6c
	ctx.r[5].s64 = ctx.r[10].s64 + 12140;
	// 826BF850: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826BF854: 390B0B20  addi r8, r11, 0xb20
	ctx.r[8].s64 = ctx.r[11].s64 + 2848;
	// 826BF858: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 826BF85C: 388A4928  addi r4, r10, 0x4928
	ctx.r[4].s64 = ctx.r[10].s64 + 18728;
	// 826BF860: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826BF864: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826BF868: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826BF86C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826BF870: 386A2F9C  addi r3, r10, 0x2f9c
	ctx.r[3].s64 = ctx.r[10].s64 + 12188;
	// 826BF874: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826BF878: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826BF87C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826BF880: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826BF884: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826BF888: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826BF88C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826BF890: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826BF894: 4BDA758D  bl 0x82466e20
	ctx.lr = 0x826BF898;
	sub_82466E20(ctx, base);
	// 826BF898: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826BF89C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826BF8A0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826BF8A4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826BF8A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826BF8A8 size=112
    let mut pc: u32 = 0x826BF8A8;
    'dispatch: loop {
        match pc {
            0x826BF8A8 => {
    //   block [0x826BF8A8..0x826BF918)
	// 826BF8A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826BF8AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826BF8B0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826BF8B4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826BF8B8: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826BF8BC: 38AA2F6C  addi r5, r10, 0x2f6c
	ctx.r[5].s64 = ctx.r[10].s64 + 12140;
	// 826BF8C0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826BF8C4: 390B0B80  addi r8, r11, 0xb80
	ctx.r[8].s64 = ctx.r[11].s64 + 2944;
	// 826BF8C8: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826BF8CC: 388A4944  addi r4, r10, 0x4944
	ctx.r[4].s64 = ctx.r[10].s64 + 18756;
	// 826BF8D0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826BF8D4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826BF8D8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826BF8DC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826BF8E0: 386A2FCC  addi r3, r10, 0x2fcc
	ctx.r[3].s64 = ctx.r[10].s64 + 12236;
	// 826BF8E4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826BF8E8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826BF8EC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826BF8F0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826BF8F4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826BF8F8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826BF8FC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826BF900: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826BF904: 4BDA751D  bl 0x82466e20
	ctx.lr = 0x826BF908;
	sub_82466E20(ctx, base);
	// 826BF908: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826BF90C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826BF910: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826BF914: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826BF918(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826BF918 size=112
    let mut pc: u32 = 0x826BF918;
    'dispatch: loop {
        match pc {
            0x826BF918 => {
    //   block [0x826BF918..0x826BF988)
	// 826BF918: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826BF91C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826BF920: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826BF924: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826BF928: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826BF92C: 38AA2F6C  addi r5, r10, 0x2f6c
	ctx.r[5].s64 = ctx.r[10].s64 + 12140;
	// 826BF930: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826BF934: 390B0BB0  addi r8, r11, 0xbb0
	ctx.r[8].s64 = ctx.r[11].s64 + 2992;
	// 826BF938: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 826BF93C: 388A4964  addi r4, r10, 0x4964
	ctx.r[4].s64 = ctx.r[10].s64 + 18788;
	// 826BF940: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826BF944: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826BF948: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826BF94C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826BF950: 386A2FFC  addi r3, r10, 0x2ffc
	ctx.r[3].s64 = ctx.r[10].s64 + 12284;
	// 826BF954: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826BF958: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826BF95C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826BF960: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826BF964: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826BF968: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826BF96C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826BF970: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826BF974: 4BDA74AD  bl 0x82466e20
	ctx.lr = 0x826BF978;
	sub_82466E20(ctx, base);
	// 826BF978: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826BF97C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826BF980: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826BF984: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826BF988(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826BF988 size=108
    let mut pc: u32 = 0x826BF988;
    'dispatch: loop {
        match pc {
            0x826BF988 => {
    //   block [0x826BF988..0x826BF9F4)
	// 826BF988: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826BF98C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826BF990: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826BF994: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826BF998: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826BF99C: 38EB0BF8  addi r7, r11, 0xbf8
	ctx.r[7].s64 = ctx.r[11].s64 + 3064;
	// 826BF9A0: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826BF9A4: 388A4980  addi r4, r10, 0x4980
	ctx.r[4].s64 = ctx.r[10].s64 + 18816;
	// 826BF9A8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826BF9AC: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826BF9B0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826BF9B4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826BF9B8: 386A302C  addi r3, r10, 0x302c
	ctx.r[3].s64 = ctx.r[10].s64 + 12332;
	// 826BF9BC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826BF9C0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826BF9C4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826BF9C8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826BF9CC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826BF9D0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826BF9D4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826BF9D8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826BF9DC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826BF9E0: 4BDA7441  bl 0x82466e20
	ctx.lr = 0x826BF9E4;
	sub_82466E20(ctx, base);
	// 826BF9E4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826BF9E8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826BF9EC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826BF9F0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826BF9F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826BF9F8 size=112
    let mut pc: u32 = 0x826BF9F8;
    'dispatch: loop {
        match pc {
            0x826BF9F8 => {
    //   block [0x826BF9F8..0x826BFA68)
	// 826BF9F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826BF9FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826BFA00: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826BFA04: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826BFA08: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826BFA0C: 38AA28AC  addi r5, r10, 0x28ac
	ctx.r[5].s64 = ctx.r[10].s64 + 10412;
	// 826BFA10: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826BFA14: 390B0C28  addi r8, r11, 0xc28
	ctx.r[8].s64 = ctx.r[11].s64 + 3112;
	// 826BFA18: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826BFA1C: 388A499C  addi r4, r10, 0x499c
	ctx.r[4].s64 = ctx.r[10].s64 + 18844;
	// 826BFA20: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826BFA24: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826BFA28: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826BFA2C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826BFA30: 386A305C  addi r3, r10, 0x305c
	ctx.r[3].s64 = ctx.r[10].s64 + 12380;
	// 826BFA34: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826BFA38: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826BFA3C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826BFA40: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826BFA44: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826BFA48: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826BFA4C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826BFA50: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826BFA54: 4BDA73CD  bl 0x82466e20
	ctx.lr = 0x826BFA58;
	sub_82466E20(ctx, base);
	// 826BFA58: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826BFA5C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826BFA60: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826BFA64: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826BFA68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826BFA68 size=116
    let mut pc: u32 = 0x826BFA68;
    'dispatch: loop {
        match pc {
            0x826BFA68 => {
    //   block [0x826BFA68..0x826BFADC)
	// 826BFA68: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826BFA6C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826BFA70: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826BFA74: 3D40827E  lis r10, -0x7d82
	ctx.r[10].s64 = -2105671680;
	// 826BFA78: 38E00010  li r7, 0x10
	ctx.r[7].s64 = 16;
	// 826BFA7C: 390A0C40  addi r8, r10, 0xc40
	ctx.r[8].s64 = ctx.r[10].s64 + 3136;
	// 826BFA80: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826BFA84: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 826BFA88: 38AA34DC  addi r5, r10, 0x34dc
	ctx.r[5].s64 = ctx.r[10].s64 + 13532;
	// 826BFA8C: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826BFA90: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826BFA94: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826BFA98: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826BFA9C: 388A49FC  addi r4, r10, 0x49fc
	ctx.r[4].s64 = ctx.r[10].s64 + 18940;
	// 826BFAA0: 396B1168  addi r11, r11, 0x1168
	ctx.r[11].s64 = ctx.r[11].s64 + 4456;
	// 826BFAA4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826BFAA8: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826BFAAC: 386A308C  addi r3, r10, 0x308c
	ctx.r[3].s64 = ctx.r[10].s64 + 12428;
	// 826BFAB0: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 826BFAB4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826BFAB8: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 826BFABC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826BFAC0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826BFAC4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826BFAC8: 4BDA7359  bl 0x82466e20
	ctx.lr = 0x826BFACC;
	sub_82466E20(ctx, base);
	// 826BFACC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826BFAD0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826BFAD4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826BFAD8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826BFAE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826BFAE0 size=100
    let mut pc: u32 = 0x826BFAE0;
    'dispatch: loop {
        match pc {
            0x826BFAE0 => {
    //   block [0x826BFAE0..0x826BFB44)
	// 826BFAE0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826BFAE4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826BFAE8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826BFAEC: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826BFAF0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826BFAF4: 38AA0DAC  addi r5, r10, 0xdac
	ctx.r[5].s64 = ctx.r[10].s64 + 3500;
	// 826BFAF8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826BFAFC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826BFB00: 388A4A08  addi r4, r10, 0x4a08
	ctx.r[4].s64 = ctx.r[10].s64 + 18952;
	// 826BFB04: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826BFB08: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826BFB0C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826BFB10: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826BFB14: 386A30BC  addi r3, r10, 0x30bc
	ctx.r[3].s64 = ctx.r[10].s64 + 12476;
	// 826BFB18: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826BFB1C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826BFB20: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826BFB24: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826BFB28: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826BFB2C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826BFB30: 4BDA72F1  bl 0x82466e20
	ctx.lr = 0x826BFB34;
	sub_82466E20(ctx, base);
	// 826BFB34: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826BFB38: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826BFB3C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826BFB40: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826BFB48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826BFB48 size=100
    let mut pc: u32 = 0x826BFB48;
    'dispatch: loop {
        match pc {
            0x826BFB48 => {
    //   block [0x826BFB48..0x826BFBAC)
	// 826BFB48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826BFB4C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826BFB50: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826BFB54: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826BFB58: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826BFB5C: 38AA314C  addi r5, r10, 0x314c
	ctx.r[5].s64 = ctx.r[10].s64 + 12620;
	// 826BFB60: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826BFB64: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826BFB68: 388A4A1C  addi r4, r10, 0x4a1c
	ctx.r[4].s64 = ctx.r[10].s64 + 18972;
	// 826BFB6C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826BFB70: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826BFB74: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826BFB78: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826BFB7C: 386A30EC  addi r3, r10, 0x30ec
	ctx.r[3].s64 = ctx.r[10].s64 + 12524;
	// 826BFB80: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826BFB84: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826BFB88: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826BFB8C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826BFB90: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826BFB94: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826BFB98: 4BDA7289  bl 0x82466e20
	ctx.lr = 0x826BFB9C;
	sub_82466E20(ctx, base);
	// 826BFB9C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826BFBA0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826BFBA4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826BFBA8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826BFBB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826BFBB0 size=100
    let mut pc: u32 = 0x826BFBB0;
    'dispatch: loop {
        match pc {
            0x826BFBB0 => {
    //   block [0x826BFBB0..0x826BFC14)
	// 826BFBB0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826BFBB4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826BFBB8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826BFBBC: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826BFBC0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826BFBC4: 38AA308C  addi r5, r10, 0x308c
	ctx.r[5].s64 = ctx.r[10].s64 + 12428;
	// 826BFBC8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826BFBCC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826BFBD0: 388A4A38  addi r4, r10, 0x4a38
	ctx.r[4].s64 = ctx.r[10].s64 + 19000;
	// 826BFBD4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826BFBD8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826BFBDC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826BFBE0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826BFBE4: 386A311C  addi r3, r10, 0x311c
	ctx.r[3].s64 = ctx.r[10].s64 + 12572;
	// 826BFBE8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826BFBEC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826BFBF0: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826BFBF4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826BFBF8: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826BFBFC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826BFC00: 4BDA7221  bl 0x82466e20
	ctx.lr = 0x826BFC04;
	sub_82466E20(ctx, base);
	// 826BFC04: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826BFC08: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826BFC0C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826BFC10: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826BFC18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826BFC18 size=104
    let mut pc: u32 = 0x826BFC18;
    'dispatch: loop {
        match pc {
            0x826BFC18 => {
    //   block [0x826BFC18..0x826BFC80)
	// 826BFC18: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826BFC1C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826BFC20: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826BFC24: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 826BFC28: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826BFC2C: 392A11CC  addi r9, r10, 0x11cc
	ctx.r[9].s64 = ctx.r[10].s64 + 4556;
	// 826BFC30: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826BFC34: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826BFC38: 38AA30BC  addi r5, r10, 0x30bc
	ctx.r[5].s64 = ctx.r[10].s64 + 12476;
	// 826BFC3C: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826BFC40: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826BFC44: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826BFC48: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826BFC4C: 388A4A44  addi r4, r10, 0x4a44
	ctx.r[4].s64 = ctx.r[10].s64 + 19012;
	// 826BFC50: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826BFC54: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826BFC58: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826BFC5C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826BFC60: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826BFC64: 386A314C  addi r3, r10, 0x314c
	ctx.r[3].s64 = ctx.r[10].s64 + 12620;
	// 826BFC68: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826BFC6C: 4BDA71B5  bl 0x82466e20
	ctx.lr = 0x826BFC70;
	sub_82466E20(ctx, base);
	// 826BFC70: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826BFC74: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826BFC78: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826BFC7C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826BFC80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826BFC80 size=108
    let mut pc: u32 = 0x826BFC80;
    'dispatch: loop {
        match pc {
            0x826BFC80 => {
    //   block [0x826BFC80..0x826BFCEC)
	// 826BFC80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826BFC84: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826BFC88: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826BFC8C: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826BFC90: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826BFC94: 38EB0DC4  addi r7, r11, 0xdc4
	ctx.r[7].s64 = ctx.r[11].s64 + 3524;
	// 826BFC98: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826BFC9C: 388A4A5C  addi r4, r10, 0x4a5c
	ctx.r[4].s64 = ctx.r[10].s64 + 19036;
	// 826BFCA0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826BFCA4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826BFCA8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826BFCAC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826BFCB0: 386A317C  addi r3, r10, 0x317c
	ctx.r[3].s64 = ctx.r[10].s64 + 12668;
	// 826BFCB4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826BFCB8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826BFCBC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826BFCC0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826BFCC4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826BFCC8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826BFCCC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826BFCD0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826BFCD4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826BFCD8: 4BDA7149  bl 0x82466e20
	ctx.lr = 0x826BFCDC;
	sub_82466E20(ctx, base);
	// 826BFCDC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826BFCE0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826BFCE4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826BFCE8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826BFCF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826BFCF0 size=112
    let mut pc: u32 = 0x826BFCF0;
    'dispatch: loop {
        match pc {
            0x826BFCF0 => {
    //   block [0x826BFCF0..0x826BFD60)
	// 826BFCF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826BFCF4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826BFCF8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826BFCFC: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826BFD00: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826BFD04: 38AA314C  addi r5, r10, 0x314c
	ctx.r[5].s64 = ctx.r[10].s64 + 12620;
	// 826BFD08: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826BFD0C: 390B0DF8  addi r8, r11, 0xdf8
	ctx.r[8].s64 = ctx.r[11].s64 + 3576;
	// 826BFD10: 39200007  li r9, 7
	ctx.r[9].s64 = 7;
	// 826BFD14: 388A4A80  addi r4, r10, 0x4a80
	ctx.r[4].s64 = ctx.r[10].s64 + 19072;
	// 826BFD18: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826BFD1C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826BFD20: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826BFD24: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826BFD28: 386A31AC  addi r3, r10, 0x31ac
	ctx.r[3].s64 = ctx.r[10].s64 + 12716;
	// 826BFD2C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826BFD30: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826BFD34: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826BFD38: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826BFD3C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826BFD40: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826BFD44: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826BFD48: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826BFD4C: 4BDA70D5  bl 0x82466e20
	ctx.lr = 0x826BFD50;
	sub_82466E20(ctx, base);
	// 826BFD50: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826BFD54: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826BFD58: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826BFD5C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826BFD60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826BFD60 size=24
    let mut pc: u32 = 0x826BFD60;
    'dispatch: loop {
        match pc {
            0x826BFD60 => {
    //   block [0x826BFD60..0x826BFD78)
	// 826BFD60: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826BFD64: 3D40827E  lis r10, -0x7d82
	ctx.r[10].s64 = -2105671680;
	// 826BFD68: 394A34D0  addi r10, r10, 0x34d0
	ctx.r[10].s64 = ctx.r[10].s64 + 13520;
	// 826BFD6C: 816B0DF4  lwz r11, 0xdf4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(3572 as u32) ) } as u64;
	// 826BFD70: 916A0008  stw r11, 8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 826BFD74: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826BFD78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826BFD78 size=116
    let mut pc: u32 = 0x826BFD78;
    'dispatch: loop {
        match pc {
            0x826BFD78 => {
    //   block [0x826BFD78..0x826BFDEC)
	// 826BFD78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826BFD7C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826BFD80: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826BFD84: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826BFD88: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 826BFD8C: 390B34D0  addi r8, r11, 0x34d0
	ctx.r[8].s64 = ctx.r[11].s64 + 13520;
	// 826BFD90: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826BFD94: 392A1230  addi r9, r10, 0x1230
	ctx.r[9].s64 = ctx.r[10].s64 + 4656;
	// 826BFD98: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826BFD9C: 38E00005  li r7, 5
	ctx.r[7].s64 = 5;
	// 826BFDA0: 38AA0DAC  addi r5, r10, 0xdac
	ctx.r[5].s64 = ctx.r[10].s64 + 3500;
	// 826BFDA4: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826BFDA8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826BFDAC: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826BFDB0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826BFDB4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826BFDB8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826BFDBC: 3D608298  lis r11, -0x7d68
	ctx.r[11].s64 = -2103967744;
	// 826BFDC0: 388A4B0C  addi r4, r10, 0x4b0c
	ctx.r[4].s64 = ctx.r[10].s64 + 19212;
	// 826BFDC4: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826BFDC8: 386B31DC  addi r3, r11, 0x31dc
	ctx.r[3].s64 = ctx.r[11].s64 + 12764;
	// 826BFDCC: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826BFDD0: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826BFDD4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826BFDD8: 4BDA7049  bl 0x82466e20
	ctx.lr = 0x826BFDDC;
	sub_82466E20(ctx, base);
	// 826BFDDC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826BFDE0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826BFDE4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826BFDE8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826BFDF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826BFDF0 size=100
    let mut pc: u32 = 0x826BFDF0;
    'dispatch: loop {
        match pc {
            0x826BFDF0 => {
    //   block [0x826BFDF0..0x826BFE54)
	// 826BFDF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826BFDF4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826BFDF8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826BFDFC: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826BFE00: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826BFE04: 38AA31DC  addi r5, r10, 0x31dc
	ctx.r[5].s64 = ctx.r[10].s64 + 12764;
	// 826BFE08: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826BFE0C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826BFE10: 388A4B18  addi r4, r10, 0x4b18
	ctx.r[4].s64 = ctx.r[10].s64 + 19224;
	// 826BFE14: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826BFE18: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826BFE1C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826BFE20: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826BFE24: 386A320C  addi r3, r10, 0x320c
	ctx.r[3].s64 = ctx.r[10].s64 + 12812;
	// 826BFE28: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826BFE2C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826BFE30: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826BFE34: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826BFE38: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826BFE3C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826BFE40: 4BDA6FE1  bl 0x82466e20
	ctx.lr = 0x826BFE44;
	sub_82466E20(ctx, base);
	// 826BFE44: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826BFE48: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826BFE4C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826BFE50: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826BFE58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826BFE58 size=100
    let mut pc: u32 = 0x826BFE58;
    'dispatch: loop {
        match pc {
            0x826BFE58 => {
    //   block [0x826BFE58..0x826BFEBC)
	// 826BFE58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826BFE5C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826BFE60: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826BFE64: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826BFE68: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826BFE6C: 38AA326C  addi r5, r10, 0x326c
	ctx.r[5].s64 = ctx.r[10].s64 + 12908;
	// 826BFE70: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826BFE74: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826BFE78: 388A4B24  addi r4, r10, 0x4b24
	ctx.r[4].s64 = ctx.r[10].s64 + 19236;
	// 826BFE7C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826BFE80: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826BFE84: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826BFE88: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826BFE8C: 386A323C  addi r3, r10, 0x323c
	ctx.r[3].s64 = ctx.r[10].s64 + 12860;
	// 826BFE90: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826BFE94: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826BFE98: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826BFE9C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826BFEA0: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826BFEA4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826BFEA8: 4BDA6F79  bl 0x82466e20
	ctx.lr = 0x826BFEAC;
	sub_82466E20(ctx, base);
	// 826BFEAC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826BFEB0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826BFEB4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826BFEB8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826BFEC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826BFEC0 size=112
    let mut pc: u32 = 0x826BFEC0;
    'dispatch: loop {
        match pc {
            0x826BFEC0 => {
    //   block [0x826BFEC0..0x826BFF30)
	// 826BFEC0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826BFEC4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826BFEC8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826BFECC: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826BFED0: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826BFED4: 38AA31DC  addi r5, r10, 0x31dc
	ctx.r[5].s64 = ctx.r[10].s64 + 12764;
	// 826BFED8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826BFEDC: 390B0EA0  addi r8, r11, 0xea0
	ctx.r[8].s64 = ctx.r[11].s64 + 3744;
	// 826BFEE0: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826BFEE4: 388A4B38  addi r4, r10, 0x4b38
	ctx.r[4].s64 = ctx.r[10].s64 + 19256;
	// 826BFEE8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826BFEEC: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826BFEF0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826BFEF4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826BFEF8: 386A326C  addi r3, r10, 0x326c
	ctx.r[3].s64 = ctx.r[10].s64 + 12908;
	// 826BFEFC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826BFF00: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826BFF04: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826BFF08: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826BFF0C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826BFF10: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826BFF14: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826BFF18: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826BFF1C: 4BDA6F05  bl 0x82466e20
	ctx.lr = 0x826BFF20;
	sub_82466E20(ctx, base);
	// 826BFF20: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826BFF24: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826BFF28: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826BFF2C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826BFF30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826BFF30 size=100
    let mut pc: u32 = 0x826BFF30;
    'dispatch: loop {
        match pc {
            0x826BFF30 => {
    //   block [0x826BFF30..0x826BFF94)
	// 826BFF30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826BFF34: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826BFF38: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826BFF3C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826BFF40: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826BFF44: 38AA326C  addi r5, r10, 0x326c
	ctx.r[5].s64 = ctx.r[10].s64 + 12908;
	// 826BFF48: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826BFF4C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826BFF50: 388A4B50  addi r4, r10, 0x4b50
	ctx.r[4].s64 = ctx.r[10].s64 + 19280;
	// 826BFF54: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826BFF58: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826BFF5C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826BFF60: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826BFF64: 386A329C  addi r3, r10, 0x329c
	ctx.r[3].s64 = ctx.r[10].s64 + 12956;
	// 826BFF68: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826BFF6C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826BFF70: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826BFF74: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826BFF78: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826BFF7C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826BFF80: 4BDA6EA1  bl 0x82466e20
	ctx.lr = 0x826BFF84;
	sub_82466E20(ctx, base);
	// 826BFF84: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826BFF88: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826BFF8C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826BFF90: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826BFF98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826BFF98 size=100
    let mut pc: u32 = 0x826BFF98;
    'dispatch: loop {
        match pc {
            0x826BFF98 => {
    //   block [0x826BFF98..0x826BFFFC)
	// 826BFF98: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826BFF9C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826BFFA0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826BFFA4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826BFFA8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826BFFAC: 38AA31DC  addi r5, r10, 0x31dc
	ctx.r[5].s64 = ctx.r[10].s64 + 12764;
	// 826BFFB0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826BFFB4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826BFFB8: 388A4B60  addi r4, r10, 0x4b60
	ctx.r[4].s64 = ctx.r[10].s64 + 19296;
	// 826BFFBC: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826BFFC0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826BFFC4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826BFFC8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826BFFCC: 386A32CC  addi r3, r10, 0x32cc
	ctx.r[3].s64 = ctx.r[10].s64 + 13004;
	// 826BFFD0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826BFFD4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826BFFD8: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826BFFDC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826BFFE0: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826BFFE4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826BFFE8: 4BDA6E39  bl 0x82466e20
	ctx.lr = 0x826BFFEC;
	sub_82466E20(ctx, base);
	// 826BFFEC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826BFFF0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826BFFF4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826BFFF8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C0000(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C0000 size=100
    let mut pc: u32 = 0x826C0000;
    'dispatch: loop {
        match pc {
            0x826C0000 => {
    //   block [0x826C0000..0x826C0064)
	// 826C0000: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C0004: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C0008: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C000C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C0010: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C0014: 38AA320C  addi r5, r10, 0x320c
	ctx.r[5].s64 = ctx.r[10].s64 + 12812;
	// 826C0018: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C001C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C0020: 388A4B70  addi r4, r10, 0x4b70
	ctx.r[4].s64 = ctx.r[10].s64 + 19312;
	// 826C0024: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C0028: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C002C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C0030: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C0034: 386A32FC  addi r3, r10, 0x32fc
	ctx.r[3].s64 = ctx.r[10].s64 + 13052;
	// 826C0038: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C003C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C0040: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826C0044: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C0048: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826C004C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C0050: 4BDA6DD1  bl 0x82466e20
	ctx.lr = 0x826C0054;
	sub_82466E20(ctx, base);
	// 826C0054: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C0058: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C005C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C0060: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C0068(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C0068 size=100
    let mut pc: u32 = 0x826C0068;
    'dispatch: loop {
        match pc {
            0x826C0068 => {
    //   block [0x826C0068..0x826C00CC)
	// 826C0068: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C006C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C0070: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C0074: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C0078: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C007C: 38AA32CC  addi r5, r10, 0x32cc
	ctx.r[5].s64 = ctx.r[10].s64 + 13004;
	// 826C0080: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C0084: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C0088: 388A4B88  addi r4, r10, 0x4b88
	ctx.r[4].s64 = ctx.r[10].s64 + 19336;
	// 826C008C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C0090: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C0094: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C0098: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C009C: 386A332C  addi r3, r10, 0x332c
	ctx.r[3].s64 = ctx.r[10].s64 + 13100;
	// 826C00A0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C00A4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C00A8: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826C00AC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C00B0: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826C00B4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C00B8: 4BDA6D69  bl 0x82466e20
	ctx.lr = 0x826C00BC;
	sub_82466E20(ctx, base);
	// 826C00BC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C00C0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C00C4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C00C8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C00D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C00D0 size=100
    let mut pc: u32 = 0x826C00D0;
    'dispatch: loop {
        match pc {
            0x826C00D0 => {
    //   block [0x826C00D0..0x826C0134)
	// 826C00D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C00D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C00D8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C00DC: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C00E0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C00E4: 38AA320C  addi r5, r10, 0x320c
	ctx.r[5].s64 = ctx.r[10].s64 + 12812;
	// 826C00E8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C00EC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C00F0: 388A4BA4  addi r4, r10, 0x4ba4
	ctx.r[4].s64 = ctx.r[10].s64 + 19364;
	// 826C00F4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C00F8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C00FC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C0100: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C0104: 386A335C  addi r3, r10, 0x335c
	ctx.r[3].s64 = ctx.r[10].s64 + 13148;
	// 826C0108: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C010C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C0110: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826C0114: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C0118: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826C011C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C0120: 4BDA6D01  bl 0x82466e20
	ctx.lr = 0x826C0124;
	sub_82466E20(ctx, base);
	// 826C0124: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C0128: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C012C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C0130: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C0138(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C0138 size=112
    let mut pc: u32 = 0x826C0138;
    'dispatch: loop {
        match pc {
            0x826C0138 => {
    //   block [0x826C0138..0x826C01A8)
	// 826C0138: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C013C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C0140: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C0144: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C0148: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826C014C: 38AA33EC  addi r5, r10, 0x33ec
	ctx.r[5].s64 = ctx.r[10].s64 + 13292;
	// 826C0150: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C0154: 390B0ED0  addi r8, r11, 0xed0
	ctx.r[8].s64 = ctx.r[11].s64 + 3792;
	// 826C0158: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826C015C: 388A4BB4  addi r4, r10, 0x4bb4
	ctx.r[4].s64 = ctx.r[10].s64 + 19380;
	// 826C0160: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C0164: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C0168: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826C016C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C0170: 386A338C  addi r3, r10, 0x338c
	ctx.r[3].s64 = ctx.r[10].s64 + 13196;
	// 826C0174: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826C0178: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C017C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C0180: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C0184: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C0188: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C018C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C0190: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C0194: 4BDA6C8D  bl 0x82466e20
	ctx.lr = 0x826C0198;
	sub_82466E20(ctx, base);
	// 826C0198: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C019C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C01A0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C01A4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C01A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C01A8 size=112
    let mut pc: u32 = 0x826C01A8;
    'dispatch: loop {
        match pc {
            0x826C01A8 => {
    //   block [0x826C01A8..0x826C0218)
	// 826C01A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C01AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C01B0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C01B4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C01B8: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826C01BC: 38AA341C  addi r5, r10, 0x341c
	ctx.r[5].s64 = ctx.r[10].s64 + 13340;
	// 826C01C0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C01C4: 390B0F00  addi r8, r11, 0xf00
	ctx.r[8].s64 = ctx.r[11].s64 + 3840;
	// 826C01C8: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826C01CC: 388A4BC4  addi r4, r10, 0x4bc4
	ctx.r[4].s64 = ctx.r[10].s64 + 19396;
	// 826C01D0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C01D4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C01D8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826C01DC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C01E0: 386A33BC  addi r3, r10, 0x33bc
	ctx.r[3].s64 = ctx.r[10].s64 + 13244;
	// 826C01E4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826C01E8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C01EC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C01F0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C01F4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C01F8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C01FC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C0200: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C0204: 4BDA6C1D  bl 0x82466e20
	ctx.lr = 0x826C0208;
	sub_82466E20(ctx, base);
	// 826C0208: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C020C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C0210: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C0214: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C0218(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C0218 size=112
    let mut pc: u32 = 0x826C0218;
    'dispatch: loop {
        match pc {
            0x826C0218 => {
    //   block [0x826C0218..0x826C0288)
	// 826C0218: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C021C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C0220: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C0224: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C0228: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826C022C: 38AA34DC  addi r5, r10, 0x34dc
	ctx.r[5].s64 = ctx.r[10].s64 + 13532;
	// 826C0230: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C0234: 390B0F18  addi r8, r11, 0xf18
	ctx.r[8].s64 = ctx.r[11].s64 + 3864;
	// 826C0238: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826C023C: 388A4BDC  addi r4, r10, 0x4bdc
	ctx.r[4].s64 = ctx.r[10].s64 + 19420;
	// 826C0240: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C0244: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C0248: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826C024C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C0250: 386A33EC  addi r3, r10, 0x33ec
	ctx.r[3].s64 = ctx.r[10].s64 + 13292;
	// 826C0254: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826C0258: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C025C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C0260: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C0264: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C0268: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C026C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C0270: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C0274: 4BDA6BAD  bl 0x82466e20
	ctx.lr = 0x826C0278;
	sub_82466E20(ctx, base);
	// 826C0278: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C027C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C0280: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C0284: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C0288(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C0288 size=112
    let mut pc: u32 = 0x826C0288;
    'dispatch: loop {
        match pc {
            0x826C0288 => {
    //   block [0x826C0288..0x826C02F8)
	// 826C0288: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C028C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C0290: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C0294: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C0298: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826C029C: 38AA33EC  addi r5, r10, 0x33ec
	ctx.r[5].s64 = ctx.r[10].s64 + 13292;
	// 826C02A0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C02A4: 390B0F48  addi r8, r11, 0xf48
	ctx.r[8].s64 = ctx.r[11].s64 + 3912;
	// 826C02A8: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826C02AC: 388A4BE8  addi r4, r10, 0x4be8
	ctx.r[4].s64 = ctx.r[10].s64 + 19432;
	// 826C02B0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C02B4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C02B8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826C02BC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C02C0: 386A341C  addi r3, r10, 0x341c
	ctx.r[3].s64 = ctx.r[10].s64 + 13340;
	// 826C02C4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826C02C8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C02CC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C02D0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C02D4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C02D8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C02DC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C02E0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C02E4: 4BDA6B3D  bl 0x82466e20
	ctx.lr = 0x826C02E8;
	sub_82466E20(ctx, base);
	// 826C02E8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C02EC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C02F0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C02F4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C02F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C02F8 size=112
    let mut pc: u32 = 0x826C02F8;
    'dispatch: loop {
        match pc {
            0x826C02F8 => {
    //   block [0x826C02F8..0x826C0368)
	// 826C02F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C02FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C0300: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C0304: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C0308: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826C030C: 38AA341C  addi r5, r10, 0x341c
	ctx.r[5].s64 = ctx.r[10].s64 + 13340;
	// 826C0310: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C0314: 390B0F60  addi r8, r11, 0xf60
	ctx.r[8].s64 = ctx.r[11].s64 + 3936;
	// 826C0318: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826C031C: 388A4BF8  addi r4, r10, 0x4bf8
	ctx.r[4].s64 = ctx.r[10].s64 + 19448;
	// 826C0320: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C0324: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C0328: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826C032C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C0330: 386A344C  addi r3, r10, 0x344c
	ctx.r[3].s64 = ctx.r[10].s64 + 13388;
	// 826C0334: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826C0338: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C033C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C0340: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C0344: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C0348: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C034C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C0350: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C0354: 4BDA6ACD  bl 0x82466e20
	ctx.lr = 0x826C0358;
	sub_82466E20(ctx, base);
	// 826C0358: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C035C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C0360: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C0364: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C0368(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C0368 size=116
    let mut pc: u32 = 0x826C0368;
    'dispatch: loop {
        match pc {
            0x826C0368 => {
    //   block [0x826C0368..0x826C03DC)
	// 826C0368: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C036C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C0370: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C0374: 3D40827E  lis r10, -0x7d82
	ctx.r[10].s64 = -2105671680;
	// 826C0378: 38E00007  li r7, 7
	ctx.r[7].s64 = 7;
	// 826C037C: 390A0F78  addi r8, r10, 0xf78
	ctx.r[8].s64 = ctx.r[10].s64 + 3960;
	// 826C0380: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C0384: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 826C0388: 38AA0DAC  addi r5, r10, 0xdac
	ctx.r[5].s64 = ctx.r[10].s64 + 3500;
	// 826C038C: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C0390: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826C0394: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C0398: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826C039C: 388A4C10  addi r4, r10, 0x4c10
	ctx.r[4].s64 = ctx.r[10].s64 + 19472;
	// 826C03A0: 396B1244  addi r11, r11, 0x1244
	ctx.r[11].s64 = ctx.r[11].s64 + 4676;
	// 826C03A4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C03A8: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C03AC: 386A347C  addi r3, r10, 0x347c
	ctx.r[3].s64 = ctx.r[10].s64 + 13436;
	// 826C03B0: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 826C03B4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C03B8: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 826C03BC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C03C0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C03C4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C03C8: 4BDA6A59  bl 0x82466e20
	ctx.lr = 0x826C03CC;
	sub_82466E20(ctx, base);
	// 826C03CC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C03D0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C03D4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C03D8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C03E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826C03E0 size=48
    let mut pc: u32 = 0x826C03E0;
    'dispatch: loop {
        match pc {
            0x826C03E0 => {
    //   block [0x826C03E0..0x826C0410)
	// 826C03E0: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826C03E4: 814B102C  lwz r10, 0x102c(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4140 as u32) ) } as u64;
	// 826C03E8: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826C03EC: 396B3548  addi r11, r11, 0x3548
	ctx.r[11].s64 = ctx.r[11].s64 + 13640;
	// 826C03F0: 914B0050  stw r10, 0x50(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 826C03F4: 3D40827E  lis r10, -0x7d82
	ctx.r[10].s64 = -2105671680;
	// 826C03F8: 814A1028  lwz r10, 0x1028(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4136 as u32) ) } as u64;
	// 826C03FC: 914B0110  stw r10, 0x110(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(272 as u32), ctx.r[10].u32 ) };
	// 826C0400: 3D40827E  lis r10, -0x7d82
	ctx.r[10].s64 = -2105671680;
	// 826C0404: 814A1024  lwz r10, 0x1024(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4132 as u32) ) } as u64;
	// 826C0408: 914B0230  stw r10, 0x230(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(560 as u32), ctx.r[10].u32 ) };
	// 826C040C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C0410(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C0410 size=116
    let mut pc: u32 = 0x826C0410;
    'dispatch: loop {
        match pc {
            0x826C0410 => {
    //   block [0x826C0410..0x826C0484)
	// 826C0410: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C0414: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C0418: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C041C: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 826C0420: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C0424: 392B1318  addi r9, r11, 0x1318
	ctx.r[9].s64 = ctx.r[11].s64 + 4888;
	// 826C0428: 38AA0DAC  addi r5, r10, 0xdac
	ctx.r[5].s64 = ctx.r[10].s64 + 3500;
	// 826C042C: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C0430: 38E90050  addi r7, r9, 0x50
	ctx.r[7].s64 = ctx.r[9].s64 + 80;
	// 826C0434: 38C0001B  li r6, 0x1b
	ctx.r[6].s64 = 27;
	// 826C0438: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826C043C: 388A4FC8  addi r4, r10, 0x4fc8
	ctx.r[4].s64 = ctx.r[10].s64 + 20424;
	// 826C0440: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C0444: 396B3548  addi r11, r11, 0x3548
	ctx.r[11].s64 = ctx.r[11].s64 + 13640;
	// 826C0448: 90E10064  stw r7, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[7].u32 ) };
	// 826C044C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C0450: 90C1005C  stw r6, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[6].u32 ) };
	// 826C0454: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C0458: 386A34AC  addi r3, r10, 0x34ac
	ctx.r[3].s64 = ctx.r[10].s64 + 13484;
	// 826C045C: 39400004  li r10, 4
	ctx.r[10].s64 = 4;
	// 826C0460: 91010074  stw r8, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[8].u32 ) };
	// 826C0464: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C0468: 9101006C  stw r8, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[8].u32 ) };
	// 826C046C: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826C0470: 4BDA69B1  bl 0x82466e20
	ctx.lr = 0x826C0474;
	sub_82466E20(ctx, base);
	// 826C0474: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C0478: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C047C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C0480: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C0488(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C0488 size=116
    let mut pc: u32 = 0x826C0488;
    'dispatch: loop {
        match pc {
            0x826C0488 => {
    //   block [0x826C0488..0x826C04FC)
	// 826C0488: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C048C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C0490: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C0494: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826C0498: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 826C049C: 390B1038  addi r8, r11, 0x1038
	ctx.r[8].s64 = ctx.r[11].s64 + 4152;
	// 826C04A0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C04A4: 392A1440  addi r9, r10, 0x1440
	ctx.r[9].s64 = ctx.r[10].s64 + 5184;
	// 826C04A8: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C04AC: 38E00006  li r7, 6
	ctx.r[7].s64 = 6;
	// 826C04B0: 38AA0DAC  addi r5, r10, 0xdac
	ctx.r[5].s64 = ctx.r[10].s64 + 3500;
	// 826C04B4: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826C04B8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C04BC: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C04C0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C04C4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C04C8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C04CC: 3D608298  lis r11, -0x7d68
	ctx.r[11].s64 = -2103967744;
	// 826C04D0: 388A4FD8  addi r4, r10, 0x4fd8
	ctx.r[4].s64 = ctx.r[10].s64 + 20440;
	// 826C04D4: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826C04D8: 386B34DC  addi r3, r11, 0x34dc
	ctx.r[3].s64 = ctx.r[11].s64 + 13532;
	// 826C04DC: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826C04E0: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C04E4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C04E8: 4BDA6939  bl 0x82466e20
	ctx.lr = 0x826C04EC;
	sub_82466E20(ctx, base);
	// 826C04EC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C04F0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C04F4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C04F8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C0500(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C0500 size=112
    let mut pc: u32 = 0x826C0500;
    'dispatch: loop {
        match pc {
            0x826C0500 => {
    //   block [0x826C0500..0x826C0570)
	// 826C0500: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C0504: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C0508: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C050C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C0510: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826C0514: 38AA0DAC  addi r5, r10, 0xdac
	ctx.r[5].s64 = ctx.r[10].s64 + 3500;
	// 826C0518: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C051C: 390B10C8  addi r8, r11, 0x10c8
	ctx.r[8].s64 = ctx.r[11].s64 + 4296;
	// 826C0520: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826C0524: 388A4FE8  addi r4, r10, 0x4fe8
	ctx.r[4].s64 = ctx.r[10].s64 + 20456;
	// 826C0528: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C052C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C0530: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826C0534: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C0538: 386A350C  addi r3, r10, 0x350c
	ctx.r[3].s64 = ctx.r[10].s64 + 13580;
	// 826C053C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826C0540: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C0544: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C0548: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C054C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C0550: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C0554: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C0558: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C055C: 4BDA68C5  bl 0x82466e20
	ctx.lr = 0x826C0560;
	sub_82466E20(ctx, base);
	// 826C0560: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C0564: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C0568: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C056C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C0570(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C0570 size=112
    let mut pc: u32 = 0x826C0570;
    'dispatch: loop {
        match pc {
            0x826C0570 => {
    //   block [0x826C0570..0x826C05E0)
	// 826C0570: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C0574: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C0578: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C057C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C0580: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826C0584: 38AA185C  addi r5, r10, 0x185c
	ctx.r[5].s64 = ctx.r[10].s64 + 6236;
	// 826C0588: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C058C: 390B10E0  addi r8, r11, 0x10e0
	ctx.r[8].s64 = ctx.r[11].s64 + 4320;
	// 826C0590: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826C0594: 388A5000  addi r4, r10, 0x5000
	ctx.r[4].s64 = ctx.r[10].s64 + 20480;
	// 826C0598: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C059C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C05A0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826C05A4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C05A8: 386A353C  addi r3, r10, 0x353c
	ctx.r[3].s64 = ctx.r[10].s64 + 13628;
	// 826C05AC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826C05B0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C05B4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C05B8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C05BC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C05C0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C05C4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C05C8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C05CC: 4BDA6855  bl 0x82466e20
	ctx.lr = 0x826C05D0;
	sub_82466E20(ctx, base);
	// 826C05D0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C05D4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C05D8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C05DC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C05E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C05E0 size=108
    let mut pc: u32 = 0x826C05E0;
    'dispatch: loop {
        match pc {
            0x826C05E0 => {
    //   block [0x826C05E0..0x826C064C)
	// 826C05E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C05E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C05E8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C05EC: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826C05F0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C05F4: 38EB10F8  addi r7, r11, 0x10f8
	ctx.r[7].s64 = ctx.r[11].s64 + 4344;
	// 826C05F8: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826C05FC: 388A5014  addi r4, r10, 0x5014
	ctx.r[4].s64 = ctx.r[10].s64 + 20500;
	// 826C0600: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C0604: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C0608: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826C060C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C0610: 386A356C  addi r3, r10, 0x356c
	ctx.r[3].s64 = ctx.r[10].s64 + 13676;
	// 826C0614: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826C0618: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C061C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C0620: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C0624: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C0628: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C062C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C0630: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C0634: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826C0638: 4BDA67E9  bl 0x82466e20
	ctx.lr = 0x826C063C;
	sub_82466E20(ctx, base);
	// 826C063C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C0640: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C0644: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C0648: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C0650(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C0650 size=112
    let mut pc: u32 = 0x826C0650;
    'dispatch: loop {
        match pc {
            0x826C0650 => {
    //   block [0x826C0650..0x826C06C0)
	// 826C0650: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C0654: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C0658: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C065C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C0660: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826C0664: 38AA0DAC  addi r5, r10, 0xdac
	ctx.r[5].s64 = ctx.r[10].s64 + 3500;
	// 826C0668: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C066C: 390B1110  addi r8, r11, 0x1110
	ctx.r[8].s64 = ctx.r[11].s64 + 4368;
	// 826C0670: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 826C0674: 388A5028  addi r4, r10, 0x5028
	ctx.r[4].s64 = ctx.r[10].s64 + 20520;
	// 826C0678: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C067C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C0680: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826C0684: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C0688: 386A359C  addi r3, r10, 0x359c
	ctx.r[3].s64 = ctx.r[10].s64 + 13724;
	// 826C068C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826C0690: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C0694: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C0698: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C069C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C06A0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C06A4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C06A8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C06AC: 4BDA6775  bl 0x82466e20
	ctx.lr = 0x826C06B0;
	sub_82466E20(ctx, base);
	// 826C06B0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C06B4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C06B8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C06BC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C06C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C06C0 size=108
    let mut pc: u32 = 0x826C06C0;
    'dispatch: loop {
        match pc {
            0x826C06C0 => {
    //   block [0x826C06C0..0x826C072C)
	// 826C06C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C06C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C06C8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C06CC: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826C06D0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C06D4: 38EB1158  addi r7, r11, 0x1158
	ctx.r[7].s64 = ctx.r[11].s64 + 4440;
	// 826C06D8: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826C06DC: 388A505C  addi r4, r10, 0x505c
	ctx.r[4].s64 = ctx.r[10].s64 + 20572;
	// 826C06E0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C06E4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C06E8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826C06EC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C06F0: 386A35CC  addi r3, r10, 0x35cc
	ctx.r[3].s64 = ctx.r[10].s64 + 13772;
	// 826C06F4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826C06F8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C06FC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C0700: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C0704: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C0708: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C070C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C0710: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C0714: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826C0718: 4BDA6709  bl 0x82466e20
	ctx.lr = 0x826C071C;
	sub_82466E20(ctx, base);
	// 826C071C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C0720: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C0724: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C0728: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C0730(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C0730 size=112
    let mut pc: u32 = 0x826C0730;
    'dispatch: loop {
        match pc {
            0x826C0730 => {
    //   block [0x826C0730..0x826C07A0)
	// 826C0730: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C0734: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C0738: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C073C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C0740: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826C0744: 38AA0DAC  addi r5, r10, 0xdac
	ctx.r[5].s64 = ctx.r[10].s64 + 3500;
	// 826C0748: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C074C: 390B1170  addi r8, r11, 0x1170
	ctx.r[8].s64 = ctx.r[11].s64 + 4464;
	// 826C0750: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826C0754: 388A5070  addi r4, r10, 0x5070
	ctx.r[4].s64 = ctx.r[10].s64 + 20592;
	// 826C0758: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C075C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C0760: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826C0764: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C0768: 386A35FC  addi r3, r10, 0x35fc
	ctx.r[3].s64 = ctx.r[10].s64 + 13820;
	// 826C076C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826C0770: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C0774: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C0778: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C077C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C0780: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C0784: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C0788: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C078C: 4BDA6695  bl 0x82466e20
	ctx.lr = 0x826C0790;
	sub_82466E20(ctx, base);
	// 826C0790: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C0794: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C0798: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C079C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C07A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C07A0 size=112
    let mut pc: u32 = 0x826C07A0;
    'dispatch: loop {
        match pc {
            0x826C07A0 => {
    //   block [0x826C07A0..0x826C0810)
	// 826C07A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C07A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C07A8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C07AC: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 826C07B0: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826C07B4: 392A1498  addi r9, r10, 0x1498
	ctx.r[9].s64 = ctx.r[10].s64 + 5272;
	// 826C07B8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C07BC: 390B11A8  addi r8, r11, 0x11a8
	ctx.r[8].s64 = ctx.r[11].s64 + 4520;
	// 826C07C0: 38E00007  li r7, 7
	ctx.r[7].s64 = 7;
	// 826C07C4: 388A508C  addi r4, r10, 0x508c
	ctx.r[4].s64 = ctx.r[10].s64 + 20620;
	// 826C07C8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C07CC: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C07D0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826C07D4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C07D8: 386A362C  addi r3, r10, 0x362c
	ctx.r[3].s64 = ctx.r[10].s64 + 13868;
	// 826C07DC: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826C07E0: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826C07E4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C07E8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C07EC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C07F0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C07F4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826C07F8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C07FC: 4BDA6625  bl 0x82466e20
	ctx.lr = 0x826C0800;
	sub_82466E20(ctx, base);
	// 826C0800: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C0804: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C0808: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C080C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C0810(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C0810 size=116
    let mut pc: u32 = 0x826C0810;
    'dispatch: loop {
        match pc {
            0x826C0810 => {
    //   block [0x826C0810..0x826C0884)
	// 826C0810: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C0814: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C0818: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C081C: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826C0820: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 826C0824: 390B1250  addi r8, r11, 0x1250
	ctx.r[8].s64 = ctx.r[11].s64 + 4688;
	// 826C0828: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C082C: 392A146C  addi r9, r10, 0x146c
	ctx.r[9].s64 = ctx.r[10].s64 + 5228;
	// 826C0830: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C0834: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
	// 826C0838: 38AA28AC  addi r5, r10, 0x28ac
	ctx.r[5].s64 = ctx.r[10].s64 + 10412;
	// 826C083C: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826C0840: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C0844: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C0848: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C084C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C0850: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C0854: 3D608298  lis r11, -0x7d68
	ctx.r[11].s64 = -2103967744;
	// 826C0858: 388A50AC  addi r4, r10, 0x50ac
	ctx.r[4].s64 = ctx.r[10].s64 + 20652;
	// 826C085C: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826C0860: 386B365C  addi r3, r11, 0x365c
	ctx.r[3].s64 = ctx.r[11].s64 + 13916;
	// 826C0864: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826C0868: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C086C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C0870: 4BDA65B1  bl 0x82466e20
	ctx.lr = 0x826C0874;
	sub_82466E20(ctx, base);
	// 826C0874: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C0878: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C087C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C0880: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C0888(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C0888 size=112
    let mut pc: u32 = 0x826C0888;
    'dispatch: loop {
        match pc {
            0x826C0888 => {
    //   block [0x826C0888..0x826C08F8)
	// 826C0888: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C088C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C0890: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C0894: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 826C0898: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826C089C: 392A14C4  addi r9, r10, 0x14c4
	ctx.r[9].s64 = ctx.r[10].s64 + 5316;
	// 826C08A0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C08A4: 390B1268  addi r8, r11, 0x1268
	ctx.r[8].s64 = ctx.r[11].s64 + 4712;
	// 826C08A8: 38E00004  li r7, 4
	ctx.r[7].s64 = 4;
	// 826C08AC: 388A50C4  addi r4, r10, 0x50c4
	ctx.r[4].s64 = ctx.r[10].s64 + 20676;
	// 826C08B0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C08B4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C08B8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826C08BC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C08C0: 386A368C  addi r3, r10, 0x368c
	ctx.r[3].s64 = ctx.r[10].s64 + 13964;
	// 826C08C4: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826C08C8: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826C08CC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C08D0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C08D4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C08D8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C08DC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826C08E0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C08E4: 4BDA653D  bl 0x82466e20
	ctx.lr = 0x826C08E8;
	sub_82466E20(ctx, base);
	// 826C08E8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C08EC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C08F0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C08F4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C08F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C08F8 size=112
    let mut pc: u32 = 0x826C08F8;
    'dispatch: loop {
        match pc {
            0x826C08F8 => {
    //   block [0x826C08F8..0x826C0968)
	// 826C08F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C08FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C0900: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C0904: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C0908: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826C090C: 38AA28AC  addi r5, r10, 0x28ac
	ctx.r[5].s64 = ctx.r[10].s64 + 10412;
	// 826C0910: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C0914: 390B12C8  addi r8, r11, 0x12c8
	ctx.r[8].s64 = ctx.r[11].s64 + 4808;
	// 826C0918: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826C091C: 388A50E0  addi r4, r10, 0x50e0
	ctx.r[4].s64 = ctx.r[10].s64 + 20704;
	// 826C0920: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C0924: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C0928: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826C092C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C0930: 386A36BC  addi r3, r10, 0x36bc
	ctx.r[3].s64 = ctx.r[10].s64 + 14012;
	// 826C0934: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826C0938: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C093C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C0940: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C0944: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C0948: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C094C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C0950: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C0954: 4BDA64CD  bl 0x82466e20
	ctx.lr = 0x826C0958;
	sub_82466E20(ctx, base);
	// 826C0958: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C095C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C0960: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C0964: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C0968(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C0968 size=112
    let mut pc: u32 = 0x826C0968;
    'dispatch: loop {
        match pc {
            0x826C0968 => {
    //   block [0x826C0968..0x826C09D8)
	// 826C0968: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C096C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C0970: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C0974: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C0978: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826C097C: 38AA27BC  addi r5, r10, 0x27bc
	ctx.r[5].s64 = ctx.r[10].s64 + 10172;
	// 826C0980: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C0984: 390B12E0  addi r8, r11, 0x12e0
	ctx.r[8].s64 = ctx.r[11].s64 + 4832;
	// 826C0988: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 826C098C: 388A50F4  addi r4, r10, 0x50f4
	ctx.r[4].s64 = ctx.r[10].s64 + 20724;
	// 826C0990: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C0994: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C0998: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826C099C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C09A0: 386A36EC  addi r3, r10, 0x36ec
	ctx.r[3].s64 = ctx.r[10].s64 + 14060;
	// 826C09A4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826C09A8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C09AC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C09B0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C09B4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C09B8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C09BC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C09C0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C09C4: 4BDA645D  bl 0x82466e20
	ctx.lr = 0x826C09C8;
	sub_82466E20(ctx, base);
	// 826C09C8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C09CC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C09D0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C09D4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C09D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C09D8 size=112
    let mut pc: u32 = 0x826C09D8;
    'dispatch: loop {
        match pc {
            0x826C09D8 => {
    //   block [0x826C09D8..0x826C0A48)
	// 826C09D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C09DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C09E0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C09E4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C09E8: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826C09EC: 38AA27BC  addi r5, r10, 0x27bc
	ctx.r[5].s64 = ctx.r[10].s64 + 10172;
	// 826C09F0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C09F4: 390B1328  addi r8, r11, 0x1328
	ctx.r[8].s64 = ctx.r[11].s64 + 4904;
	// 826C09F8: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 826C09FC: 388A510C  addi r4, r10, 0x510c
	ctx.r[4].s64 = ctx.r[10].s64 + 20748;
	// 826C0A00: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C0A04: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C0A08: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826C0A0C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C0A10: 386A371C  addi r3, r10, 0x371c
	ctx.r[3].s64 = ctx.r[10].s64 + 14108;
	// 826C0A14: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826C0A18: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C0A1C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C0A20: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C0A24: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C0A28: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C0A2C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C0A30: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C0A34: 4BDA63ED  bl 0x82466e20
	ctx.lr = 0x826C0A38;
	sub_82466E20(ctx, base);
	// 826C0A38: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C0A3C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C0A40: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C0A44: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C0A48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C0A48 size=112
    let mut pc: u32 = 0x826C0A48;
    'dispatch: loop {
        match pc {
            0x826C0A48 => {
    //   block [0x826C0A48..0x826C0AB8)
	// 826C0A48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C0A4C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C0A50: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C0A54: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C0A58: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826C0A5C: 38AA27EC  addi r5, r10, 0x27ec
	ctx.r[5].s64 = ctx.r[10].s64 + 10220;
	// 826C0A60: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C0A64: 390B1388  addi r8, r11, 0x1388
	ctx.r[8].s64 = ctx.r[11].s64 + 5000;
	// 826C0A68: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 826C0A6C: 388A511C  addi r4, r10, 0x511c
	ctx.r[4].s64 = ctx.r[10].s64 + 20764;
	// 826C0A70: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C0A74: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C0A78: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826C0A7C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C0A80: 386A374C  addi r3, r10, 0x374c
	ctx.r[3].s64 = ctx.r[10].s64 + 14156;
	// 826C0A84: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826C0A88: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C0A8C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C0A90: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C0A94: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C0A98: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C0A9C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C0AA0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C0AA4: 4BDA637D  bl 0x82466e20
	ctx.lr = 0x826C0AA8;
	sub_82466E20(ctx, base);
	// 826C0AA8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C0AAC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C0AB0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C0AB4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C0AB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C0AB8 size=112
    let mut pc: u32 = 0x826C0AB8;
    'dispatch: loop {
        match pc {
            0x826C0AB8 => {
    //   block [0x826C0AB8..0x826C0B28)
	// 826C0AB8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C0ABC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C0AC0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C0AC4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C0AC8: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826C0ACC: 38AA27EC  addi r5, r10, 0x27ec
	ctx.r[5].s64 = ctx.r[10].s64 + 10220;
	// 826C0AD0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C0AD4: 390B13E8  addi r8, r11, 0x13e8
	ctx.r[8].s64 = ctx.r[11].s64 + 5096;
	// 826C0AD8: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 826C0ADC: 388A512C  addi r4, r10, 0x512c
	ctx.r[4].s64 = ctx.r[10].s64 + 20780;
	// 826C0AE0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C0AE4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C0AE8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826C0AEC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C0AF0: 386A377C  addi r3, r10, 0x377c
	ctx.r[3].s64 = ctx.r[10].s64 + 14204;
	// 826C0AF4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826C0AF8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C0AFC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C0B00: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C0B04: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C0B08: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C0B0C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C0B10: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C0B14: 4BDA630D  bl 0x82466e20
	ctx.lr = 0x826C0B18;
	sub_82466E20(ctx, base);
	// 826C0B18: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C0B1C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C0B20: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C0B24: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C0B28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C0B28 size=112
    let mut pc: u32 = 0x826C0B28;
    'dispatch: loop {
        match pc {
            0x826C0B28 => {
    //   block [0x826C0B28..0x826C0B98)
	// 826C0B28: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C0B2C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C0B30: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C0B34: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C0B38: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826C0B3C: 38AA27BC  addi r5, r10, 0x27bc
	ctx.r[5].s64 = ctx.r[10].s64 + 10172;
	// 826C0B40: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C0B44: 390B1448  addi r8, r11, 0x1448
	ctx.r[8].s64 = ctx.r[11].s64 + 5192;
	// 826C0B48: 39200008  li r9, 8
	ctx.r[9].s64 = 8;
	// 826C0B4C: 388A5140  addi r4, r10, 0x5140
	ctx.r[4].s64 = ctx.r[10].s64 + 20800;
	// 826C0B50: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C0B54: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C0B58: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826C0B5C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C0B60: 386A37AC  addi r3, r10, 0x37ac
	ctx.r[3].s64 = ctx.r[10].s64 + 14252;
	// 826C0B64: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826C0B68: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C0B6C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C0B70: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C0B74: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C0B78: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C0B7C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C0B80: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C0B84: 4BDA629D  bl 0x82466e20
	ctx.lr = 0x826C0B88;
	sub_82466E20(ctx, base);
	// 826C0B88: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C0B8C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C0B90: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C0B94: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C0B98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C0B98 size=112
    let mut pc: u32 = 0x826C0B98;
    'dispatch: loop {
        match pc {
            0x826C0B98 => {
    //   block [0x826C0B98..0x826C0C08)
	// 826C0B98: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C0B9C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C0BA0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C0BA4: 3D40827E  lis r10, -0x7d82
	ctx.r[10].s64 = -2105671680;
	// 826C0BA8: 39000013  li r8, 0x13
	ctx.r[8].s64 = 19;
	// 826C0BAC: 38EA1508  addi r7, r10, 0x1508
	ctx.r[7].s64 = ctx.r[10].s64 + 5384;
	// 826C0BB0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C0BB4: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 826C0BB8: 388A5150  addi r4, r10, 0x5150
	ctx.r[4].s64 = ctx.r[10].s64 + 20816;
	// 826C0BBC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C0BC0: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826C0BC4: 396B14D8  addi r11, r11, 0x14d8
	ctx.r[11].s64 = ctx.r[11].s64 + 5336;
	// 826C0BC8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826C0BCC: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C0BD0: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C0BD4: 386A37DC  addi r3, r10, 0x37dc
	ctx.r[3].s64 = ctx.r[10].s64 + 14300;
	// 826C0BD8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C0BDC: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 826C0BE0: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C0BE4: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 826C0BE8: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C0BEC: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C0BF0: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826C0BF4: 4BDA622D  bl 0x82466e20
	ctx.lr = 0x826C0BF8;
	sub_82466E20(ctx, base);
	// 826C0BF8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C0BFC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C0C00: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C0C04: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C0C08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C0C08 size=112
    let mut pc: u32 = 0x826C0C08;
    'dispatch: loop {
        match pc {
            0x826C0C08 => {
    //   block [0x826C0C08..0x826C0C78)
	// 826C0C08: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C0C0C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C0C10: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C0C14: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C0C18: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826C0C1C: 38AA191C  addi r5, r10, 0x191c
	ctx.r[5].s64 = ctx.r[10].s64 + 6428;
	// 826C0C20: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C0C24: 390B16D0  addi r8, r11, 0x16d0
	ctx.r[8].s64 = ctx.r[11].s64 + 5840;
	// 826C0C28: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826C0C2C: 388A5168  addi r4, r10, 0x5168
	ctx.r[4].s64 = ctx.r[10].s64 + 20840;
	// 826C0C30: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C0C34: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C0C38: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826C0C3C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C0C40: 386A380C  addi r3, r10, 0x380c
	ctx.r[3].s64 = ctx.r[10].s64 + 14348;
	// 826C0C44: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826C0C48: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C0C4C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C0C50: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C0C54: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826C0C58: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C0C5C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C0C60: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C0C64: 4BDA61BD  bl 0x82466e20
	ctx.lr = 0x826C0C68;
	sub_82466E20(ctx, base);
	// 826C0C68: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C0C6C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C0C70: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C0C74: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C0C78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C0C78 size=112
    let mut pc: u32 = 0x826C0C78;
    'dispatch: loop {
        match pc {
            0x826C0C78 => {
    //   block [0x826C0C78..0x826C0CE8)
	// 826C0C78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C0C7C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C0C80: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C0C84: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C0C88: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826C0C8C: 38AA191C  addi r5, r10, 0x191c
	ctx.r[5].s64 = ctx.r[10].s64 + 6428;
	// 826C0C90: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C0C94: 390B16E8  addi r8, r11, 0x16e8
	ctx.r[8].s64 = ctx.r[11].s64 + 5864;
	// 826C0C98: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826C0C9C: 388A5184  addi r4, r10, 0x5184
	ctx.r[4].s64 = ctx.r[10].s64 + 20868;
	// 826C0CA0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C0CA4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C0CA8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826C0CAC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C0CB0: 386A383C  addi r3, r10, 0x383c
	ctx.r[3].s64 = ctx.r[10].s64 + 14396;
	// 826C0CB4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826C0CB8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C0CBC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C0CC0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C0CC4: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826C0CC8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C0CCC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C0CD0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C0CD4: 4BDA614D  bl 0x82466e20
	ctx.lr = 0x826C0CD8;
	sub_82466E20(ctx, base);
	// 826C0CD8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C0CDC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C0CE0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C0CE4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C0CE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C0CE8 size=112
    let mut pc: u32 = 0x826C0CE8;
    'dispatch: loop {
        match pc {
            0x826C0CE8 => {
    //   block [0x826C0CE8..0x826C0D58)
	// 826C0CE8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C0CEC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C0CF0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C0CF4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C0CF8: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826C0CFC: 38AA191C  addi r5, r10, 0x191c
	ctx.r[5].s64 = ctx.r[10].s64 + 6428;
	// 826C0D00: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C0D04: 390B1700  addi r8, r11, 0x1700
	ctx.r[8].s64 = ctx.r[11].s64 + 5888;
	// 826C0D08: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826C0D0C: 388A51A4  addi r4, r10, 0x51a4
	ctx.r[4].s64 = ctx.r[10].s64 + 20900;
	// 826C0D10: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C0D14: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C0D18: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826C0D1C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C0D20: 386A386C  addi r3, r10, 0x386c
	ctx.r[3].s64 = ctx.r[10].s64 + 14444;
	// 826C0D24: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826C0D28: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C0D2C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C0D30: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C0D34: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C0D38: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C0D3C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C0D40: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C0D44: 4BDA60DD  bl 0x82466e20
	ctx.lr = 0x826C0D48;
	sub_82466E20(ctx, base);
	// 826C0D48: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C0D4C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C0D50: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C0D54: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C0D58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C0D58 size=108
    let mut pc: u32 = 0x826C0D58;
    'dispatch: loop {
        match pc {
            0x826C0D58 => {
    //   block [0x826C0D58..0x826C0DC4)
	// 826C0D58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C0D5C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C0D60: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C0D64: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826C0D68: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C0D6C: 38EB1730  addi r7, r11, 0x1730
	ctx.r[7].s64 = ctx.r[11].s64 + 5936;
	// 826C0D70: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826C0D74: 388A51BC  addi r4, r10, 0x51bc
	ctx.r[4].s64 = ctx.r[10].s64 + 20924;
	// 826C0D78: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C0D7C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C0D80: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826C0D84: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C0D88: 386A389C  addi r3, r10, 0x389c
	ctx.r[3].s64 = ctx.r[10].s64 + 14492;
	// 826C0D8C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826C0D90: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C0D94: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C0D98: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C0D9C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C0DA0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C0DA4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C0DA8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C0DAC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826C0DB0: 4BDA6071  bl 0x82466e20
	ctx.lr = 0x826C0DB4;
	sub_82466E20(ctx, base);
	// 826C0DB4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C0DB8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C0DBC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C0DC0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C0DC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C0DC8 size=112
    let mut pc: u32 = 0x826C0DC8;
    'dispatch: loop {
        match pc {
            0x826C0DC8 => {
    //   block [0x826C0DC8..0x826C0E38)
	// 826C0DC8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C0DCC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C0DD0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C0DD4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C0DD8: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826C0DDC: 38AA191C  addi r5, r10, 0x191c
	ctx.r[5].s64 = ctx.r[10].s64 + 6428;
	// 826C0DE0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C0DE4: 390B1760  addi r8, r11, 0x1760
	ctx.r[8].s64 = ctx.r[11].s64 + 5984;
	// 826C0DE8: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826C0DEC: 388A51E4  addi r4, r10, 0x51e4
	ctx.r[4].s64 = ctx.r[10].s64 + 20964;
	// 826C0DF0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C0DF4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C0DF8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826C0DFC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C0E00: 386A38CC  addi r3, r10, 0x38cc
	ctx.r[3].s64 = ctx.r[10].s64 + 14540;
	// 826C0E04: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826C0E08: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C0E0C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C0E10: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C0E14: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826C0E18: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C0E1C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C0E20: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C0E24: 4BDA5FFD  bl 0x82466e20
	ctx.lr = 0x826C0E28;
	sub_82466E20(ctx, base);
	// 826C0E28: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C0E2C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C0E30: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C0E34: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C0E38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C0E38 size=108
    let mut pc: u32 = 0x826C0E38;
    'dispatch: loop {
        match pc {
            0x826C0E38 => {
    //   block [0x826C0E38..0x826C0EA4)
	// 826C0E38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C0E3C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C0E40: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C0E44: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826C0E48: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C0E4C: 38EB1778  addi r7, r11, 0x1778
	ctx.r[7].s64 = ctx.r[11].s64 + 6008;
	// 826C0E50: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826C0E54: 388A5200  addi r4, r10, 0x5200
	ctx.r[4].s64 = ctx.r[10].s64 + 20992;
	// 826C0E58: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C0E5C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C0E60: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826C0E64: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C0E68: 386A38FC  addi r3, r10, 0x38fc
	ctx.r[3].s64 = ctx.r[10].s64 + 14588;
	// 826C0E6C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826C0E70: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C0E74: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C0E78: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C0E7C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C0E80: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C0E84: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C0E88: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C0E8C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826C0E90: 4BDA5F91  bl 0x82466e20
	ctx.lr = 0x826C0E94;
	sub_82466E20(ctx, base);
	// 826C0E94: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C0E98: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C0E9C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C0EA0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C0EA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C0EA8 size=108
    let mut pc: u32 = 0x826C0EA8;
    'dispatch: loop {
        match pc {
            0x826C0EA8 => {
    //   block [0x826C0EA8..0x826C0F14)
	// 826C0EA8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C0EAC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C0EB0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C0EB4: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826C0EB8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C0EBC: 38EB17A8  addi r7, r11, 0x17a8
	ctx.r[7].s64 = ctx.r[11].s64 + 6056;
	// 826C0EC0: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 826C0EC4: 388A521C  addi r4, r10, 0x521c
	ctx.r[4].s64 = ctx.r[10].s64 + 21020;
	// 826C0EC8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C0ECC: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C0ED0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826C0ED4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C0ED8: 386A392C  addi r3, r10, 0x392c
	ctx.r[3].s64 = ctx.r[10].s64 + 14636;
	// 826C0EDC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826C0EE0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C0EE4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C0EE8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C0EEC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C0EF0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C0EF4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C0EF8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C0EFC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826C0F00: 4BDA5F21  bl 0x82466e20
	ctx.lr = 0x826C0F04;
	sub_82466E20(ctx, base);
	// 826C0F04: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C0F08: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C0F0C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C0F10: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C0F18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C0F18 size=112
    let mut pc: u32 = 0x826C0F18;
    'dispatch: loop {
        match pc {
            0x826C0F18 => {
    //   block [0x826C0F18..0x826C0F88)
	// 826C0F18: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C0F1C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C0F20: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C0F24: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C0F28: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826C0F2C: 38AA0DAC  addi r5, r10, 0xdac
	ctx.r[5].s64 = ctx.r[10].s64 + 3500;
	// 826C0F30: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C0F34: 390B17F0  addi r8, r11, 0x17f0
	ctx.r[8].s64 = ctx.r[11].s64 + 6128;
	// 826C0F38: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 826C0F3C: 388A523C  addi r4, r10, 0x523c
	ctx.r[4].s64 = ctx.r[10].s64 + 21052;
	// 826C0F40: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C0F44: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C0F48: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826C0F4C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C0F50: 386A395C  addi r3, r10, 0x395c
	ctx.r[3].s64 = ctx.r[10].s64 + 14684;
	// 826C0F54: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826C0F58: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C0F5C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C0F60: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C0F64: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C0F68: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C0F6C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C0F70: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C0F74: 4BDA5EAD  bl 0x82466e20
	ctx.lr = 0x826C0F78;
	sub_82466E20(ctx, base);
	// 826C0F78: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C0F7C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C0F80: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C0F84: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C0F88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C0F88 size=112
    let mut pc: u32 = 0x826C0F88;
    'dispatch: loop {
        match pc {
            0x826C0F88 => {
    //   block [0x826C0F88..0x826C0FF8)
	// 826C0F88: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C0F8C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C0F90: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C0F94: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C0F98: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826C0F9C: 38AA27EC  addi r5, r10, 0x27ec
	ctx.r[5].s64 = ctx.r[10].s64 + 10220;
	// 826C0FA0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C0FA4: 390B1838  addi r8, r11, 0x1838
	ctx.r[8].s64 = ctx.r[11].s64 + 6200;
	// 826C0FA8: 39200006  li r9, 6
	ctx.r[9].s64 = 6;
	// 826C0FAC: 388A5254  addi r4, r10, 0x5254
	ctx.r[4].s64 = ctx.r[10].s64 + 21076;
	// 826C0FB0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C0FB4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C0FB8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826C0FBC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C0FC0: 386A398C  addi r3, r10, 0x398c
	ctx.r[3].s64 = ctx.r[10].s64 + 14732;
	// 826C0FC4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826C0FC8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C0FCC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C0FD0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C0FD4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C0FD8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C0FDC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C0FE0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C0FE4: 4BDA5E3D  bl 0x82466e20
	ctx.lr = 0x826C0FE8;
	sub_82466E20(ctx, base);
	// 826C0FE8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C0FEC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C0FF0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C0FF4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C0FF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C0FF8 size=108
    let mut pc: u32 = 0x826C0FF8;
    'dispatch: loop {
        match pc {
            0x826C0FF8 => {
    //   block [0x826C0FF8..0x826C1064)
	// 826C0FF8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C0FFC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C1000: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C1004: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826C1008: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C100C: 38EB18C8  addi r7, r11, 0x18c8
	ctx.r[7].s64 = ctx.r[11].s64 + 6344;
	// 826C1010: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 826C1014: 388A5268  addi r4, r10, 0x5268
	ctx.r[4].s64 = ctx.r[10].s64 + 21096;
	// 826C1018: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C101C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C1020: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826C1024: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C1028: 386A39BC  addi r3, r10, 0x39bc
	ctx.r[3].s64 = ctx.r[10].s64 + 14780;
	// 826C102C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826C1030: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C1034: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C1038: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C103C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C1040: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C1044: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C1048: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C104C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826C1050: 4BDA5DD1  bl 0x82466e20
	ctx.lr = 0x826C1054;
	sub_82466E20(ctx, base);
	// 826C1054: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C1058: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C105C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C1060: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C1068(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C1068 size=108
    let mut pc: u32 = 0x826C1068;
    'dispatch: loop {
        match pc {
            0x826C1068 => {
    //   block [0x826C1068..0x826C10D4)
	// 826C1068: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C106C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C1070: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C1074: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826C1078: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C107C: 38EB1910  addi r7, r11, 0x1910
	ctx.r[7].s64 = ctx.r[11].s64 + 6416;
	// 826C1080: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826C1084: 388A5284  addi r4, r10, 0x5284
	ctx.r[4].s64 = ctx.r[10].s64 + 21124;
	// 826C1088: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C108C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C1090: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826C1094: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C1098: 386A39EC  addi r3, r10, 0x39ec
	ctx.r[3].s64 = ctx.r[10].s64 + 14828;
	// 826C109C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826C10A0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C10A4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C10A8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C10AC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C10B0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C10B4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C10B8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C10BC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826C10C0: 4BDA5D61  bl 0x82466e20
	ctx.lr = 0x826C10C4;
	sub_82466E20(ctx, base);
	// 826C10C4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C10C8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C10CC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C10D0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C10D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C10D8 size=108
    let mut pc: u32 = 0x826C10D8;
    'dispatch: loop {
        match pc {
            0x826C10D8 => {
    //   block [0x826C10D8..0x826C1144)
	// 826C10D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C10DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C10E0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C10E4: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826C10E8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C10EC: 38EB1940  addi r7, r11, 0x1940
	ctx.r[7].s64 = ctx.r[11].s64 + 6464;
	// 826C10F0: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826C10F4: 388A52A4  addi r4, r10, 0x52a4
	ctx.r[4].s64 = ctx.r[10].s64 + 21156;
	// 826C10F8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C10FC: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C1100: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826C1104: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C1108: 386A3A1C  addi r3, r10, 0x3a1c
	ctx.r[3].s64 = ctx.r[10].s64 + 14876;
	// 826C110C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826C1110: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C1114: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C1118: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C111C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C1120: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C1124: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C1128: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C112C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826C1130: 4BDA5CF1  bl 0x82466e20
	ctx.lr = 0x826C1134;
	sub_82466E20(ctx, base);
	// 826C1134: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C1138: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C113C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C1140: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C1148(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C1148 size=112
    let mut pc: u32 = 0x826C1148;
    'dispatch: loop {
        match pc {
            0x826C1148 => {
    //   block [0x826C1148..0x826C11B8)
	// 826C1148: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C114C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C1150: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C1154: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C1158: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826C115C: 38AA0DAC  addi r5, r10, 0xdac
	ctx.r[5].s64 = ctx.r[10].s64 + 3500;
	// 826C1160: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C1164: 390B1970  addi r8, r11, 0x1970
	ctx.r[8].s64 = ctx.r[11].s64 + 6512;
	// 826C1168: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826C116C: 388A52BC  addi r4, r10, 0x52bc
	ctx.r[4].s64 = ctx.r[10].s64 + 21180;
	// 826C1170: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C1174: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C1178: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826C117C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C1180: 386A3A4C  addi r3, r10, 0x3a4c
	ctx.r[3].s64 = ctx.r[10].s64 + 14924;
	// 826C1184: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826C1188: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C118C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C1190: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C1194: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C1198: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C119C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C11A0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C11A4: 4BDA5C7D  bl 0x82466e20
	ctx.lr = 0x826C11A8;
	sub_82466E20(ctx, base);
	// 826C11A8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C11AC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C11B0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C11B4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C11B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C11B8 size=112
    let mut pc: u32 = 0x826C11B8;
    'dispatch: loop {
        match pc {
            0x826C11B8 => {
    //   block [0x826C11B8..0x826C1228)
	// 826C11B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C11BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C11C0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C11C4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C11C8: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826C11CC: 38AA0DAC  addi r5, r10, 0xdac
	ctx.r[5].s64 = ctx.r[10].s64 + 3500;
	// 826C11D0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C11D4: 390B19A0  addi r8, r11, 0x19a0
	ctx.r[8].s64 = ctx.r[11].s64 + 6560;
	// 826C11D8: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826C11DC: 388A52CC  addi r4, r10, 0x52cc
	ctx.r[4].s64 = ctx.r[10].s64 + 21196;
	// 826C11E0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C11E4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C11E8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826C11EC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C11F0: 386A3A7C  addi r3, r10, 0x3a7c
	ctx.r[3].s64 = ctx.r[10].s64 + 14972;
	// 826C11F4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826C11F8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C11FC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C1200: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C1204: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C1208: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C120C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C1210: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C1214: 4BDA5C0D  bl 0x82466e20
	ctx.lr = 0x826C1218;
	sub_82466E20(ctx, base);
	// 826C1218: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C121C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C1220: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C1224: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C1228(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C1228 size=112
    let mut pc: u32 = 0x826C1228;
    'dispatch: loop {
        match pc {
            0x826C1228 => {
    //   block [0x826C1228..0x826C1298)
	// 826C1228: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C122C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C1230: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C1234: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C1238: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826C123C: 38AA0DAC  addi r5, r10, 0xdac
	ctx.r[5].s64 = ctx.r[10].s64 + 3500;
	// 826C1240: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C1244: 390B19B8  addi r8, r11, 0x19b8
	ctx.r[8].s64 = ctx.r[11].s64 + 6584;
	// 826C1248: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826C124C: 388A52E8  addi r4, r10, 0x52e8
	ctx.r[4].s64 = ctx.r[10].s64 + 21224;
	// 826C1250: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C1254: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C1258: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826C125C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C1260: 386A3AAC  addi r3, r10, 0x3aac
	ctx.r[3].s64 = ctx.r[10].s64 + 15020;
	// 826C1264: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826C1268: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C126C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C1270: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C1274: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C1278: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C127C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C1280: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C1284: 4BDA5B9D  bl 0x82466e20
	ctx.lr = 0x826C1288;
	sub_82466E20(ctx, base);
	// 826C1288: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C128C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C1290: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C1294: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C1298(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C1298 size=108
    let mut pc: u32 = 0x826C1298;
    'dispatch: loop {
        match pc {
            0x826C1298 => {
    //   block [0x826C1298..0x826C1304)
	// 826C1298: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C129C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C12A0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C12A4: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826C12A8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C12AC: 38EB19D0  addi r7, r11, 0x19d0
	ctx.r[7].s64 = ctx.r[11].s64 + 6608;
	// 826C12B0: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826C12B4: 388A5308  addi r4, r10, 0x5308
	ctx.r[4].s64 = ctx.r[10].s64 + 21256;
	// 826C12B8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C12BC: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C12C0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826C12C4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C12C8: 386A3ADC  addi r3, r10, 0x3adc
	ctx.r[3].s64 = ctx.r[10].s64 + 15068;
	// 826C12CC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826C12D0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C12D4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C12D8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C12DC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C12E0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C12E4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C12E8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C12EC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826C12F0: 4BDA5B31  bl 0x82466e20
	ctx.lr = 0x826C12F4;
	sub_82466E20(ctx, base);
	// 826C12F4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C12F8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C12FC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C1300: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C1308(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C1308 size=112
    let mut pc: u32 = 0x826C1308;
    'dispatch: loop {
        match pc {
            0x826C1308 => {
    //   block [0x826C1308..0x826C1378)
	// 826C1308: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C130C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C1310: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C1314: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C1318: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826C131C: 38AA0DAC  addi r5, r10, 0xdac
	ctx.r[5].s64 = ctx.r[10].s64 + 3500;
	// 826C1320: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C1324: 390B1A00  addi r8, r11, 0x1a00
	ctx.r[8].s64 = ctx.r[11].s64 + 6656;
	// 826C1328: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826C132C: 388A533C  addi r4, r10, 0x533c
	ctx.r[4].s64 = ctx.r[10].s64 + 21308;
	// 826C1330: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C1334: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C1338: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826C133C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C1340: 386A3B0C  addi r3, r10, 0x3b0c
	ctx.r[3].s64 = ctx.r[10].s64 + 15116;
	// 826C1344: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826C1348: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C134C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C1350: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C1354: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C1358: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C135C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C1360: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C1364: 4BDA5ABD  bl 0x82466e20
	ctx.lr = 0x826C1368;
	sub_82466E20(ctx, base);
	// 826C1368: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C136C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C1370: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C1374: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C1378(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C1378 size=108
    let mut pc: u32 = 0x826C1378;
    'dispatch: loop {
        match pc {
            0x826C1378 => {
    //   block [0x826C1378..0x826C13E4)
	// 826C1378: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C137C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C1380: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C1384: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826C1388: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C138C: 38EB1A18  addi r7, r11, 0x1a18
	ctx.r[7].s64 = ctx.r[11].s64 + 6680;
	// 826C1390: 3900000A  li r8, 0xa
	ctx.r[8].s64 = 10;
	// 826C1394: 388A535C  addi r4, r10, 0x535c
	ctx.r[4].s64 = ctx.r[10].s64 + 21340;
	// 826C1398: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C139C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C13A0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826C13A4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C13A8: 386A3B3C  addi r3, r10, 0x3b3c
	ctx.r[3].s64 = ctx.r[10].s64 + 15164;
	// 826C13AC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826C13B0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C13B4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C13B8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C13BC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C13C0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C13C4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C13C8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C13CC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826C13D0: 4BDA5A51  bl 0x82466e20
	ctx.lr = 0x826C13D4;
	sub_82466E20(ctx, base);
	// 826C13D4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C13D8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C13DC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C13E0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C13E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C13E8 size=112
    let mut pc: u32 = 0x826C13E8;
    'dispatch: loop {
        match pc {
            0x826C13E8 => {
    //   block [0x826C13E8..0x826C1458)
	// 826C13E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C13EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C13F0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C13F4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C13F8: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826C13FC: 38AA0DAC  addi r5, r10, 0xdac
	ctx.r[5].s64 = ctx.r[10].s64 + 3500;
	// 826C1400: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C1404: 390B1B08  addi r8, r11, 0x1b08
	ctx.r[8].s64 = ctx.r[11].s64 + 6920;
	// 826C1408: 39200012  li r9, 0x12
	ctx.r[9].s64 = 18;
	// 826C140C: 388A5380  addi r4, r10, 0x5380
	ctx.r[4].s64 = ctx.r[10].s64 + 21376;
	// 826C1410: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C1414: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C1418: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826C141C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C1420: 386A3B6C  addi r3, r10, 0x3b6c
	ctx.r[3].s64 = ctx.r[10].s64 + 15212;
	// 826C1424: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826C1428: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C142C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C1430: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C1434: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C1438: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C143C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C1440: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C1444: 4BDA59DD  bl 0x82466e20
	ctx.lr = 0x826C1448;
	sub_82466E20(ctx, base);
	// 826C1448: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C144C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C1450: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C1454: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C1458(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C1458 size=108
    let mut pc: u32 = 0x826C1458;
    'dispatch: loop {
        match pc {
            0x826C1458 => {
    //   block [0x826C1458..0x826C14C4)
	// 826C1458: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C145C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C1460: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C1464: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826C1468: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C146C: 38EB1CB8  addi r7, r11, 0x1cb8
	ctx.r[7].s64 = ctx.r[11].s64 + 7352;
	// 826C1470: 39000011  li r8, 0x11
	ctx.r[8].s64 = 17;
	// 826C1474: 388A5390  addi r4, r10, 0x5390
	ctx.r[4].s64 = ctx.r[10].s64 + 21392;
	// 826C1478: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C147C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C1480: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826C1484: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C1488: 386A3B9C  addi r3, r10, 0x3b9c
	ctx.r[3].s64 = ctx.r[10].s64 + 15260;
	// 826C148C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826C1490: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C1494: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C1498: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C149C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C14A0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C14A4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C14A8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C14AC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826C14B0: 4BDA5971  bl 0x82466e20
	ctx.lr = 0x826C14B4;
	sub_82466E20(ctx, base);
	// 826C14B4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C14B8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C14BC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C14C0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C14C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C14C8 size=112
    let mut pc: u32 = 0x826C14C8;
    'dispatch: loop {
        match pc {
            0x826C14C8 => {
    //   block [0x826C14C8..0x826C1538)
	// 826C14C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C14CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C14D0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C14D4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C14D8: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826C14DC: 38AA27EC  addi r5, r10, 0x27ec
	ctx.r[5].s64 = ctx.r[10].s64 + 10220;
	// 826C14E0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C14E4: 390B1E50  addi r8, r11, 0x1e50
	ctx.r[8].s64 = ctx.r[11].s64 + 7760;
	// 826C14E8: 39200019  li r9, 0x19
	ctx.r[9].s64 = 25;
	// 826C14EC: 388A53AC  addi r4, r10, 0x53ac
	ctx.r[4].s64 = ctx.r[10].s64 + 21420;
	// 826C14F0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C14F4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C14F8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826C14FC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C1500: 386A3BCC  addi r3, r10, 0x3bcc
	ctx.r[3].s64 = ctx.r[10].s64 + 15308;
	// 826C1504: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826C1508: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C150C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C1510: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C1514: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C1518: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C151C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C1520: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C1524: 4BDA58FD  bl 0x82466e20
	ctx.lr = 0x826C1528;
	sub_82466E20(ctx, base);
	// 826C1528: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C152C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C1530: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C1534: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C1538(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C1538 size=100
    let mut pc: u32 = 0x826C1538;
    'dispatch: loop {
        match pc {
            0x826C1538 => {
    //   block [0x826C1538..0x826C159C)
	// 826C1538: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C153C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C1540: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C1544: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C1548: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C154C: 38AA0DAC  addi r5, r10, 0xdac
	ctx.r[5].s64 = ctx.r[10].s64 + 3500;
	// 826C1550: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C1554: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C1558: 388A53C0  addi r4, r10, 0x53c0
	ctx.r[4].s64 = ctx.r[10].s64 + 21440;
	// 826C155C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C1560: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C1564: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C1568: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C156C: 386A3BFC  addi r3, r10, 0x3bfc
	ctx.r[3].s64 = ctx.r[10].s64 + 15356;
	// 826C1570: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C1574: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C1578: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826C157C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C1580: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826C1584: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C1588: 4BDA5899  bl 0x82466e20
	ctx.lr = 0x826C158C;
	sub_82466E20(ctx, base);
	// 826C158C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C1590: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C1594: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C1598: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C15A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C15A0 size=112
    let mut pc: u32 = 0x826C15A0;
    'dispatch: loop {
        match pc {
            0x826C15A0 => {
    //   block [0x826C15A0..0x826C1610)
	// 826C15A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C15A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C15A8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C15AC: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C15B0: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826C15B4: 38AA3BFC  addi r5, r10, 0x3bfc
	ctx.r[5].s64 = ctx.r[10].s64 + 15356;
	// 826C15B8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C15BC: 390B20A8  addi r8, r11, 0x20a8
	ctx.r[8].s64 = ctx.r[11].s64 + 8360;
	// 826C15C0: 39200005  li r9, 5
	ctx.r[9].s64 = 5;
	// 826C15C4: 388A53D8  addi r4, r10, 0x53d8
	ctx.r[4].s64 = ctx.r[10].s64 + 21464;
	// 826C15C8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C15CC: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C15D0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826C15D4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C15D8: 386A3C2C  addi r3, r10, 0x3c2c
	ctx.r[3].s64 = ctx.r[10].s64 + 15404;
	// 826C15DC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826C15E0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C15E4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C15E8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C15EC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C15F0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C15F4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C15F8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C15FC: 4BDA5825  bl 0x82466e20
	ctx.lr = 0x826C1600;
	sub_82466E20(ctx, base);
	// 826C1600: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C1604: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C1608: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C160C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C1610(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C1610 size=100
    let mut pc: u32 = 0x826C1610;
    'dispatch: loop {
        match pc {
            0x826C1610 => {
    //   block [0x826C1610..0x826C1674)
	// 826C1610: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C1614: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C1618: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C161C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C1620: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C1624: 38AA0DAC  addi r5, r10, 0xdac
	ctx.r[5].s64 = ctx.r[10].s64 + 3500;
	// 826C1628: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C162C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C1630: 388A53F8  addi r4, r10, 0x53f8
	ctx.r[4].s64 = ctx.r[10].s64 + 21496;
	// 826C1634: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C1638: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C163C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C1640: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C1644: 386A3C5C  addi r3, r10, 0x3c5c
	ctx.r[3].s64 = ctx.r[10].s64 + 15452;
	// 826C1648: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C164C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C1650: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826C1654: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C1658: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826C165C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C1660: 4BDA57C1  bl 0x82466e20
	ctx.lr = 0x826C1664;
	sub_82466E20(ctx, base);
	// 826C1664: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C1668: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C166C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C1670: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C1678(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C1678 size=108
    let mut pc: u32 = 0x826C1678;
    'dispatch: loop {
        match pc {
            0x826C1678 => {
    //   block [0x826C1678..0x826C16E4)
	// 826C1678: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C167C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C1680: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C1684: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826C1688: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C168C: 38EB2120  addi r7, r11, 0x2120
	ctx.r[7].s64 = ctx.r[11].s64 + 8480;
	// 826C1690: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 826C1694: 388A5408  addi r4, r10, 0x5408
	ctx.r[4].s64 = ctx.r[10].s64 + 21512;
	// 826C1698: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C169C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C16A0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826C16A4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C16A8: 386A3C8C  addi r3, r10, 0x3c8c
	ctx.r[3].s64 = ctx.r[10].s64 + 15500;
	// 826C16AC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826C16B0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C16B4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C16B8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C16BC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C16C0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C16C4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C16C8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C16CC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826C16D0: 4BDA5751  bl 0x82466e20
	ctx.lr = 0x826C16D4;
	sub_82466E20(ctx, base);
	// 826C16D4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C16D8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C16DC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C16E0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C16E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C16E8 size=112
    let mut pc: u32 = 0x826C16E8;
    'dispatch: loop {
        match pc {
            0x826C16E8 => {
    //   block [0x826C16E8..0x826C1758)
	// 826C16E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C16EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C16F0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C16F4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C16F8: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826C16FC: 38AA3C5C  addi r5, r10, 0x3c5c
	ctx.r[5].s64 = ctx.r[10].s64 + 15452;
	// 826C1700: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C1704: 390B2168  addi r8, r11, 0x2168
	ctx.r[8].s64 = ctx.r[11].s64 + 8552;
	// 826C1708: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826C170C: 388A5434  addi r4, r10, 0x5434
	ctx.r[4].s64 = ctx.r[10].s64 + 21556;
	// 826C1710: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C1714: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C1718: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826C171C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C1720: 386A3CBC  addi r3, r10, 0x3cbc
	ctx.r[3].s64 = ctx.r[10].s64 + 15548;
	// 826C1724: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826C1728: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C172C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C1730: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C1734: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C1738: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C173C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C1740: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C1744: 4BDA56DD  bl 0x82466e20
	ctx.lr = 0x826C1748;
	sub_82466E20(ctx, base);
	// 826C1748: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C174C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C1750: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C1754: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C1758(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C1758 size=100
    let mut pc: u32 = 0x826C1758;
    'dispatch: loop {
        match pc {
            0x826C1758 => {
    //   block [0x826C1758..0x826C17BC)
	// 826C1758: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C175C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C1760: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C1764: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C1768: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C176C: 38AA0DAC  addi r5, r10, 0xdac
	ctx.r[5].s64 = ctx.r[10].s64 + 3500;
	// 826C1770: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C1774: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C1778: 388A544C  addi r4, r10, 0x544c
	ctx.r[4].s64 = ctx.r[10].s64 + 21580;
	// 826C177C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C1780: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C1784: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C1788: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C178C: 386A3CEC  addi r3, r10, 0x3cec
	ctx.r[3].s64 = ctx.r[10].s64 + 15596;
	// 826C1790: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C1794: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C1798: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826C179C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C17A0: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826C17A4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C17A8: 4BDA5679  bl 0x82466e20
	ctx.lr = 0x826C17AC;
	sub_82466E20(ctx, base);
	// 826C17AC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C17B0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C17B4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C17B8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C17C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C17C0 size=100
    let mut pc: u32 = 0x826C17C0;
    'dispatch: loop {
        match pc {
            0x826C17C0 => {
    //   block [0x826C17C0..0x826C1824)
	// 826C17C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C17C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C17C8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C17CC: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C17D0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C17D4: 38AA0DAC  addi r5, r10, 0xdac
	ctx.r[5].s64 = ctx.r[10].s64 + 3500;
	// 826C17D8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C17DC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C17E0: 388A5468  addi r4, r10, 0x5468
	ctx.r[4].s64 = ctx.r[10].s64 + 21608;
	// 826C17E4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C17E8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C17EC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C17F0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C17F4: 386A3D1C  addi r3, r10, 0x3d1c
	ctx.r[3].s64 = ctx.r[10].s64 + 15644;
	// 826C17F8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C17FC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C1800: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826C1804: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C1808: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826C180C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C1810: 4BDA5611  bl 0x82466e20
	ctx.lr = 0x826C1814;
	sub_82466E20(ctx, base);
	// 826C1814: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C1818: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C181C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C1820: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C1828(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C1828 size=112
    let mut pc: u32 = 0x826C1828;
    'dispatch: loop {
        match pc {
            0x826C1828 => {
    //   block [0x826C1828..0x826C1898)
	// 826C1828: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C182C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C1830: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C1834: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C1838: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826C183C: 38AA3CEC  addi r5, r10, 0x3cec
	ctx.r[5].s64 = ctx.r[10].s64 + 15596;
	// 826C1840: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C1844: 390B2198  addi r8, r11, 0x2198
	ctx.r[8].s64 = ctx.r[11].s64 + 8600;
	// 826C1848: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 826C184C: 388A5480  addi r4, r10, 0x5480
	ctx.r[4].s64 = ctx.r[10].s64 + 21632;
	// 826C1850: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C1854: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C1858: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826C185C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C1860: 386A3D4C  addi r3, r10, 0x3d4c
	ctx.r[3].s64 = ctx.r[10].s64 + 15692;
	// 826C1864: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826C1868: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C186C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C1870: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C1874: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C1878: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C187C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C1880: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C1884: 4BDA559D  bl 0x82466e20
	ctx.lr = 0x826C1888;
	sub_82466E20(ctx, base);
	// 826C1888: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C188C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C1890: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C1894: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C1898(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C1898 size=112
    let mut pc: u32 = 0x826C1898;
    'dispatch: loop {
        match pc {
            0x826C1898 => {
    //   block [0x826C1898..0x826C1908)
	// 826C1898: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C189C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C18A0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C18A4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C18A8: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826C18AC: 38AA3D1C  addi r5, r10, 0x3d1c
	ctx.r[5].s64 = ctx.r[10].s64 + 15644;
	// 826C18B0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C18B4: 390B21F8  addi r8, r11, 0x21f8
	ctx.r[8].s64 = ctx.r[11].s64 + 8696;
	// 826C18B8: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 826C18BC: 388A54A4  addi r4, r10, 0x54a4
	ctx.r[4].s64 = ctx.r[10].s64 + 21668;
	// 826C18C0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C18C4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C18C8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826C18CC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C18D0: 386A3D7C  addi r3, r10, 0x3d7c
	ctx.r[3].s64 = ctx.r[10].s64 + 15740;
	// 826C18D4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826C18D8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C18DC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C18E0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C18E4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C18E8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C18EC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C18F0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C18F4: 4BDA552D  bl 0x82466e20
	ctx.lr = 0x826C18F8;
	sub_82466E20(ctx, base);
	// 826C18F8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C18FC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C1900: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C1904: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C1908(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C1908 size=100
    let mut pc: u32 = 0x826C1908;
    'dispatch: loop {
        match pc {
            0x826C1908 => {
    //   block [0x826C1908..0x826C196C)
	// 826C1908: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C190C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C1910: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C1914: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C1918: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C191C: 38AA0DAC  addi r5, r10, 0xdac
	ctx.r[5].s64 = ctx.r[10].s64 + 3500;
	// 826C1920: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C1924: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C1928: 388A54C8  addi r4, r10, 0x54c8
	ctx.r[4].s64 = ctx.r[10].s64 + 21704;
	// 826C192C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C1930: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C1934: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C1938: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C193C: 386A3DAC  addi r3, r10, 0x3dac
	ctx.r[3].s64 = ctx.r[10].s64 + 15788;
	// 826C1940: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C1944: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C1948: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826C194C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C1950: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826C1954: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C1958: 4BDA54C9  bl 0x82466e20
	ctx.lr = 0x826C195C;
	sub_82466E20(ctx, base);
	// 826C195C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C1960: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C1964: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C1968: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C1970(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C1970 size=112
    let mut pc: u32 = 0x826C1970;
    'dispatch: loop {
        match pc {
            0x826C1970 => {
    //   block [0x826C1970..0x826C19E0)
	// 826C1970: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C1974: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C1978: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C197C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C1980: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826C1984: 38AA3DAC  addi r5, r10, 0x3dac
	ctx.r[5].s64 = ctx.r[10].s64 + 15788;
	// 826C1988: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C198C: 390B2258  addi r8, r11, 0x2258
	ctx.r[8].s64 = ctx.r[11].s64 + 8792;
	// 826C1990: 3920000A  li r9, 0xa
	ctx.r[9].s64 = 10;
	// 826C1994: 388A54D8  addi r4, r10, 0x54d8
	ctx.r[4].s64 = ctx.r[10].s64 + 21720;
	// 826C1998: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C199C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C19A0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826C19A4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C19A8: 386A3DDC  addi r3, r10, 0x3ddc
	ctx.r[3].s64 = ctx.r[10].s64 + 15836;
	// 826C19AC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826C19B0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C19B4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C19B8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C19BC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C19C0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C19C4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C19C8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C19CC: 4BDA5455  bl 0x82466e20
	ctx.lr = 0x826C19D0;
	sub_82466E20(ctx, base);
	// 826C19D0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C19D4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C19D8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C19DC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C19E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C19E0 size=108
    let mut pc: u32 = 0x826C19E0;
    'dispatch: loop {
        match pc {
            0x826C19E0 => {
    //   block [0x826C19E0..0x826C1A4C)
	// 826C19E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C19E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C19E8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C19EC: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826C19F0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C19F4: 38EB2348  addi r7, r11, 0x2348
	ctx.r[7].s64 = ctx.r[11].s64 + 9032;
	// 826C19F8: 3900000A  li r8, 0xa
	ctx.r[8].s64 = 10;
	// 826C19FC: 388A54F0  addi r4, r10, 0x54f0
	ctx.r[4].s64 = ctx.r[10].s64 + 21744;
	// 826C1A00: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C1A04: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C1A08: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826C1A0C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C1A10: 386A3E0C  addi r3, r10, 0x3e0c
	ctx.r[3].s64 = ctx.r[10].s64 + 15884;
	// 826C1A14: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826C1A18: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C1A1C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C1A20: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C1A24: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C1A28: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C1A2C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C1A30: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C1A34: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826C1A38: 4BDA53E9  bl 0x82466e20
	ctx.lr = 0x826C1A3C;
	sub_82466E20(ctx, base);
	// 826C1A3C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C1A40: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C1A44: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C1A48: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C1A50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C1A50 size=108
    let mut pc: u32 = 0x826C1A50;
    'dispatch: loop {
        match pc {
            0x826C1A50 => {
    //   block [0x826C1A50..0x826C1ABC)
	// 826C1A50: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C1A54: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C1A58: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C1A5C: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826C1A60: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C1A64: 38EB2438  addi r7, r11, 0x2438
	ctx.r[7].s64 = ctx.r[11].s64 + 9272;
	// 826C1A68: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 826C1A6C: 388A551C  addi r4, r10, 0x551c
	ctx.r[4].s64 = ctx.r[10].s64 + 21788;
	// 826C1A70: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C1A74: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C1A78: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826C1A7C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C1A80: 386A3E3C  addi r3, r10, 0x3e3c
	ctx.r[3].s64 = ctx.r[10].s64 + 15932;
	// 826C1A84: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826C1A88: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C1A8C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C1A90: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C1A94: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C1A98: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C1A9C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C1AA0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C1AA4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826C1AA8: 4BDA5379  bl 0x82466e20
	ctx.lr = 0x826C1AAC;
	sub_82466E20(ctx, base);
	// 826C1AAC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C1AB0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C1AB4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C1AB8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C1AC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C1AC0 size=108
    let mut pc: u32 = 0x826C1AC0;
    'dispatch: loop {
        match pc {
            0x826C1AC0 => {
    //   block [0x826C1AC0..0x826C1B2C)
	// 826C1AC0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C1AC4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C1AC8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C1ACC: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826C1AD0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C1AD4: 38EB2480  addi r7, r11, 0x2480
	ctx.r[7].s64 = ctx.r[11].s64 + 9344;
	// 826C1AD8: 39000009  li r8, 9
	ctx.r[8].s64 = 9;
	// 826C1ADC: 388A553C  addi r4, r10, 0x553c
	ctx.r[4].s64 = ctx.r[10].s64 + 21820;
	// 826C1AE0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C1AE4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C1AE8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826C1AEC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C1AF0: 386A3E6C  addi r3, r10, 0x3e6c
	ctx.r[3].s64 = ctx.r[10].s64 + 15980;
	// 826C1AF4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826C1AF8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C1AFC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C1B00: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C1B04: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C1B08: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C1B0C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C1B10: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C1B14: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826C1B18: 4BDA5309  bl 0x82466e20
	ctx.lr = 0x826C1B1C;
	sub_82466E20(ctx, base);
	// 826C1B1C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C1B20: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C1B24: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C1B28: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C1B30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C1B30 size=108
    let mut pc: u32 = 0x826C1B30;
    'dispatch: loop {
        match pc {
            0x826C1B30 => {
    //   block [0x826C1B30..0x826C1B9C)
	// 826C1B30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C1B34: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C1B38: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C1B3C: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826C1B40: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C1B44: 38EB2558  addi r7, r11, 0x2558
	ctx.r[7].s64 = ctx.r[11].s64 + 9560;
	// 826C1B48: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826C1B4C: 388A5560  addi r4, r10, 0x5560
	ctx.r[4].s64 = ctx.r[10].s64 + 21856;
	// 826C1B50: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C1B54: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C1B58: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826C1B5C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C1B60: 386A3E9C  addi r3, r10, 0x3e9c
	ctx.r[3].s64 = ctx.r[10].s64 + 16028;
	// 826C1B64: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826C1B68: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C1B6C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C1B70: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C1B74: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C1B78: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C1B7C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C1B80: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C1B84: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826C1B88: 4BDA5299  bl 0x82466e20
	ctx.lr = 0x826C1B8C;
	sub_82466E20(ctx, base);
	// 826C1B8C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C1B90: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C1B94: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C1B98: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C1BA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C1BA0 size=100
    let mut pc: u32 = 0x826C1BA0;
    'dispatch: loop {
        match pc {
            0x826C1BA0 => {
    //   block [0x826C1BA0..0x826C1C04)
	// 826C1BA0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C1BA4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C1BA8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C1BAC: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C1BB0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C1BB4: 38AA0DAC  addi r5, r10, 0xdac
	ctx.r[5].s64 = ctx.r[10].s64 + 3500;
	// 826C1BB8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C1BBC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C1BC0: 388A5578  addi r4, r10, 0x5578
	ctx.r[4].s64 = ctx.r[10].s64 + 21880;
	// 826C1BC4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C1BC8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C1BCC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C1BD0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C1BD4: 386A3ECC  addi r3, r10, 0x3ecc
	ctx.r[3].s64 = ctx.r[10].s64 + 16076;
	// 826C1BD8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C1BDC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C1BE0: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826C1BE4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C1BE8: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826C1BEC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C1BF0: 4BDA5231  bl 0x82466e20
	ctx.lr = 0x826C1BF4;
	sub_82466E20(ctx, base);
	// 826C1BF4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C1BF8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C1BFC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C1C00: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C1C08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C1C08 size=112
    let mut pc: u32 = 0x826C1C08;
    'dispatch: loop {
        match pc {
            0x826C1C08 => {
    //   block [0x826C1C08..0x826C1C78)
	// 826C1C08: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C1C0C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C1C10: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C1C14: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C1C18: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826C1C1C: 38AA3ECC  addi r5, r10, 0x3ecc
	ctx.r[5].s64 = ctx.r[10].s64 + 16076;
	// 826C1C20: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C1C24: 390B2570  addi r8, r11, 0x2570
	ctx.r[8].s64 = ctx.r[11].s64 + 9584;
	// 826C1C28: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 826C1C2C: 388A558C  addi r4, r10, 0x558c
	ctx.r[4].s64 = ctx.r[10].s64 + 21900;
	// 826C1C30: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C1C34: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C1C38: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826C1C3C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C1C40: 386A3EFC  addi r3, r10, 0x3efc
	ctx.r[3].s64 = ctx.r[10].s64 + 16124;
	// 826C1C44: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826C1C48: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C1C4C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C1C50: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C1C54: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C1C58: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C1C5C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C1C60: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C1C64: 4BDA51BD  bl 0x82466e20
	ctx.lr = 0x826C1C68;
	sub_82466E20(ctx, base);
	// 826C1C68: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C1C6C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C1C70: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C1C74: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C1C78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C1C78 size=108
    let mut pc: u32 = 0x826C1C78;
    'dispatch: loop {
        match pc {
            0x826C1C78 => {
    //   block [0x826C1C78..0x826C1CE4)
	// 826C1C78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C1C7C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C1C80: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C1C84: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826C1C88: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C1C8C: 38EB25B8  addi r7, r11, 0x25b8
	ctx.r[7].s64 = ctx.r[11].s64 + 9656;
	// 826C1C90: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 826C1C94: 388A55A8  addi r4, r10, 0x55a8
	ctx.r[4].s64 = ctx.r[10].s64 + 21928;
	// 826C1C98: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C1C9C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C1CA0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826C1CA4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C1CA8: 386A3F2C  addi r3, r10, 0x3f2c
	ctx.r[3].s64 = ctx.r[10].s64 + 16172;
	// 826C1CAC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826C1CB0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C1CB4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C1CB8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C1CBC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C1CC0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C1CC4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C1CC8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C1CCC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826C1CD0: 4BDA5151  bl 0x82466e20
	ctx.lr = 0x826C1CD4;
	sub_82466E20(ctx, base);
	// 826C1CD4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C1CD8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C1CDC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C1CE0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C1CE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C1CE8 size=112
    let mut pc: u32 = 0x826C1CE8;
    'dispatch: loop {
        match pc {
            0x826C1CE8 => {
    //   block [0x826C1CE8..0x826C1D58)
	// 826C1CE8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C1CEC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C1CF0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C1CF4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C1CF8: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826C1CFC: 38AA0DAC  addi r5, r10, 0xdac
	ctx.r[5].s64 = ctx.r[10].s64 + 3500;
	// 826C1D00: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C1D04: 390B2600  addi r8, r11, 0x2600
	ctx.r[8].s64 = ctx.r[11].s64 + 9728;
	// 826C1D08: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826C1D0C: 388A55D8  addi r4, r10, 0x55d8
	ctx.r[4].s64 = ctx.r[10].s64 + 21976;
	// 826C1D10: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C1D14: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C1D18: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826C1D1C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C1D20: 386A3F5C  addi r3, r10, 0x3f5c
	ctx.r[3].s64 = ctx.r[10].s64 + 16220;
	// 826C1D24: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826C1D28: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C1D2C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C1D30: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C1D34: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C1D38: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C1D3C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C1D40: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C1D44: 4BDA50DD  bl 0x82466e20
	ctx.lr = 0x826C1D48;
	sub_82466E20(ctx, base);
	// 826C1D48: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C1D4C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C1D50: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C1D54: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C1D58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C1D58 size=108
    let mut pc: u32 = 0x826C1D58;
    'dispatch: loop {
        match pc {
            0x826C1D58 => {
    //   block [0x826C1D58..0x826C1DC4)
	// 826C1D58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C1D5C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C1D60: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C1D64: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826C1D68: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C1D6C: 38EB2618  addi r7, r11, 0x2618
	ctx.r[7].s64 = ctx.r[11].s64 + 9752;
	// 826C1D70: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 826C1D74: 388A55EC  addi r4, r10, 0x55ec
	ctx.r[4].s64 = ctx.r[10].s64 + 21996;
	// 826C1D78: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C1D7C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C1D80: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826C1D84: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C1D88: 386A3F8C  addi r3, r10, 0x3f8c
	ctx.r[3].s64 = ctx.r[10].s64 + 16268;
	// 826C1D8C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826C1D90: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C1D94: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C1D98: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C1D9C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C1DA0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C1DA4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C1DA8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C1DAC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826C1DB0: 4BDA5071  bl 0x82466e20
	ctx.lr = 0x826C1DB4;
	sub_82466E20(ctx, base);
	// 826C1DB4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C1DB8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C1DBC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C1DC0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C1DC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C1DC8 size=112
    let mut pc: u32 = 0x826C1DC8;
    'dispatch: loop {
        match pc {
            0x826C1DC8 => {
    //   block [0x826C1DC8..0x826C1E38)
	// 826C1DC8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C1DCC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C1DD0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C1DD4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C1DD8: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826C1DDC: 38AA3F5C  addi r5, r10, 0x3f5c
	ctx.r[5].s64 = ctx.r[10].s64 + 16220;
	// 826C1DE0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C1DE4: 390B2660  addi r8, r11, 0x2660
	ctx.r[8].s64 = ctx.r[11].s64 + 9824;
	// 826C1DE8: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826C1DEC: 388A5628  addi r4, r10, 0x5628
	ctx.r[4].s64 = ctx.r[10].s64 + 22056;
	// 826C1DF0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C1DF4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C1DF8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826C1DFC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C1E00: 386A3FBC  addi r3, r10, 0x3fbc
	ctx.r[3].s64 = ctx.r[10].s64 + 16316;
	// 826C1E04: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826C1E08: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C1E0C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C1E10: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C1E14: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C1E18: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C1E1C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C1E20: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C1E24: 4BDA4FFD  bl 0x82466e20
	ctx.lr = 0x826C1E28;
	sub_82466E20(ctx, base);
	// 826C1E28: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C1E2C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C1E30: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C1E34: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C1E38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C1E38 size=100
    let mut pc: u32 = 0x826C1E38;
    'dispatch: loop {
        match pc {
            0x826C1E38 => {
    //   block [0x826C1E38..0x826C1E9C)
	// 826C1E38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C1E3C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C1E40: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C1E44: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C1E48: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C1E4C: 38AA0DAC  addi r5, r10, 0xdac
	ctx.r[5].s64 = ctx.r[10].s64 + 3500;
	// 826C1E50: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C1E54: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C1E58: 388A5644  addi r4, r10, 0x5644
	ctx.r[4].s64 = ctx.r[10].s64 + 22084;
	// 826C1E5C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C1E60: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C1E64: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C1E68: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C1E6C: 386A3FEC  addi r3, r10, 0x3fec
	ctx.r[3].s64 = ctx.r[10].s64 + 16364;
	// 826C1E70: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C1E74: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C1E78: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826C1E7C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C1E80: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826C1E84: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C1E88: 4BDA4F99  bl 0x82466e20
	ctx.lr = 0x826C1E8C;
	sub_82466E20(ctx, base);
	// 826C1E8C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C1E90: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C1E94: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C1E98: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C1EA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C1EA0 size=112
    let mut pc: u32 = 0x826C1EA0;
    'dispatch: loop {
        match pc {
            0x826C1EA0 => {
    //   block [0x826C1EA0..0x826C1F10)
	// 826C1EA0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C1EA4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C1EA8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C1EAC: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C1EB0: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826C1EB4: 38AA3FEC  addi r5, r10, 0x3fec
	ctx.r[5].s64 = ctx.r[10].s64 + 16364;
	// 826C1EB8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C1EBC: 390B2678  addi r8, r11, 0x2678
	ctx.r[8].s64 = ctx.r[11].s64 + 9848;
	// 826C1EC0: 39200007  li r9, 7
	ctx.r[9].s64 = 7;
	// 826C1EC4: 388A565C  addi r4, r10, 0x565c
	ctx.r[4].s64 = ctx.r[10].s64 + 22108;
	// 826C1EC8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C1ECC: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C1ED0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826C1ED4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C1ED8: 386A401C  addi r3, r10, 0x401c
	ctx.r[3].s64 = ctx.r[10].s64 + 16412;
	// 826C1EDC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826C1EE0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C1EE4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C1EE8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C1EEC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C1EF0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C1EF4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C1EF8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C1EFC: 4BDA4F25  bl 0x82466e20
	ctx.lr = 0x826C1F00;
	sub_82466E20(ctx, base);
	// 826C1F00: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C1F04: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C1F08: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C1F0C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C1F10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C1F10 size=108
    let mut pc: u32 = 0x826C1F10;
    'dispatch: loop {
        match pc {
            0x826C1F10 => {
    //   block [0x826C1F10..0x826C1F7C)
	// 826C1F10: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C1F14: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C1F18: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C1F1C: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826C1F20: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C1F24: 38EB2720  addi r7, r11, 0x2720
	ctx.r[7].s64 = ctx.r[11].s64 + 10016;
	// 826C1F28: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826C1F2C: 388A567C  addi r4, r10, 0x567c
	ctx.r[4].s64 = ctx.r[10].s64 + 22140;
	// 826C1F30: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C1F34: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C1F38: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826C1F3C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C1F40: 386A404C  addi r3, r10, 0x404c
	ctx.r[3].s64 = ctx.r[10].s64 + 16460;
	// 826C1F44: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826C1F48: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C1F4C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C1F50: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C1F54: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C1F58: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C1F5C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C1F60: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C1F64: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826C1F68: 4BDA4EB9  bl 0x82466e20
	ctx.lr = 0x826C1F6C;
	sub_82466E20(ctx, base);
	// 826C1F6C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C1F70: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C1F74: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C1F78: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C1F80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C1F80 size=112
    let mut pc: u32 = 0x826C1F80;
    'dispatch: loop {
        match pc {
            0x826C1F80 => {
    //   block [0x826C1F80..0x826C1FF0)
	// 826C1F80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C1F84: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C1F88: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C1F8C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C1F90: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826C1F94: 38AA0DAC  addi r5, r10, 0xdac
	ctx.r[5].s64 = ctx.r[10].s64 + 3500;
	// 826C1F98: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C1F9C: 390B2750  addi r8, r11, 0x2750
	ctx.r[8].s64 = ctx.r[11].s64 + 10064;
	// 826C1FA0: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 826C1FA4: 388A568C  addi r4, r10, 0x568c
	ctx.r[4].s64 = ctx.r[10].s64 + 22156;
	// 826C1FA8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C1FAC: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C1FB0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826C1FB4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C1FB8: 386A407C  addi r3, r10, 0x407c
	ctx.r[3].s64 = ctx.r[10].s64 + 16508;
	// 826C1FBC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826C1FC0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C1FC4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C1FC8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C1FCC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C1FD0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C1FD4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C1FD8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C1FDC: 4BDA4E45  bl 0x82466e20
	ctx.lr = 0x826C1FE0;
	sub_82466E20(ctx, base);
	// 826C1FE0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C1FE4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C1FE8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C1FEC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C1FF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C1FF0 size=112
    let mut pc: u32 = 0x826C1FF0;
    'dispatch: loop {
        match pc {
            0x826C1FF0 => {
    //   block [0x826C1FF0..0x826C2060)
	// 826C1FF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C1FF4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C1FF8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C1FFC: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C2000: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826C2004: 38AA0DAC  addi r5, r10, 0xdac
	ctx.r[5].s64 = ctx.r[10].s64 + 3500;
	// 826C2008: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C200C: 390B2798  addi r8, r11, 0x2798
	ctx.r[8].s64 = ctx.r[11].s64 + 10136;
	// 826C2010: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 826C2014: 388A56A0  addi r4, r10, 0x56a0
	ctx.r[4].s64 = ctx.r[10].s64 + 22176;
	// 826C2018: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C201C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C2020: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826C2024: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C2028: 386A40AC  addi r3, r10, 0x40ac
	ctx.r[3].s64 = ctx.r[10].s64 + 16556;
	// 826C202C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826C2030: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C2034: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C2038: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C203C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C2040: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C2044: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C2048: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C204C: 4BDA4DD5  bl 0x82466e20
	ctx.lr = 0x826C2050;
	sub_82466E20(ctx, base);
	// 826C2050: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C2054: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C2058: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C205C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C2060(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C2060 size=100
    let mut pc: u32 = 0x826C2060;
    'dispatch: loop {
        match pc {
            0x826C2060 => {
    //   block [0x826C2060..0x826C20C4)
	// 826C2060: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C2064: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C2068: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C206C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C2070: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C2074: 38AA0DAC  addi r5, r10, 0xdac
	ctx.r[5].s64 = ctx.r[10].s64 + 3500;
	// 826C2078: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C207C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C2080: 388A56B0  addi r4, r10, 0x56b0
	ctx.r[4].s64 = ctx.r[10].s64 + 22192;
	// 826C2084: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C2088: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C208C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C2090: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C2094: 386A40DC  addi r3, r10, 0x40dc
	ctx.r[3].s64 = ctx.r[10].s64 + 16604;
	// 826C2098: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C209C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C20A0: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826C20A4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C20A8: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826C20AC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C20B0: 4BDA4D71  bl 0x82466e20
	ctx.lr = 0x826C20B4;
	sub_82466E20(ctx, base);
	// 826C20B4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C20B8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C20BC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C20C0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C20C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C20C8 size=112
    let mut pc: u32 = 0x826C20C8;
    'dispatch: loop {
        match pc {
            0x826C20C8 => {
    //   block [0x826C20C8..0x826C2138)
	// 826C20C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C20CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C20D0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C20D4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C20D8: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826C20DC: 38AA40DC  addi r5, r10, 0x40dc
	ctx.r[5].s64 = ctx.r[10].s64 + 16604;
	// 826C20E0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C20E4: 390B27E0  addi r8, r11, 0x27e0
	ctx.r[8].s64 = ctx.r[11].s64 + 10208;
	// 826C20E8: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 826C20EC: 388A56C8  addi r4, r10, 0x56c8
	ctx.r[4].s64 = ctx.r[10].s64 + 22216;
	// 826C20F0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C20F4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C20F8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826C20FC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C2100: 386A410C  addi r3, r10, 0x410c
	ctx.r[3].s64 = ctx.r[10].s64 + 16652;
	// 826C2104: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826C2108: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C210C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C2110: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C2114: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C2118: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C211C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C2120: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C2124: 4BDA4CFD  bl 0x82466e20
	ctx.lr = 0x826C2128;
	sub_82466E20(ctx, base);
	// 826C2128: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C212C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C2130: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C2134: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C2138(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C2138 size=112
    let mut pc: u32 = 0x826C2138;
    'dispatch: loop {
        match pc {
            0x826C2138 => {
    //   block [0x826C2138..0x826C21A8)
	// 826C2138: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C213C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C2140: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C2144: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C2148: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826C214C: 38AA0DAC  addi r5, r10, 0xdac
	ctx.r[5].s64 = ctx.r[10].s64 + 3500;
	// 826C2150: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C2154: 390B2828  addi r8, r11, 0x2828
	ctx.r[8].s64 = ctx.r[11].s64 + 10280;
	// 826C2158: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826C215C: 388A56E8  addi r4, r10, 0x56e8
	ctx.r[4].s64 = ctx.r[10].s64 + 22248;
	// 826C2160: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C2164: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C2168: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826C216C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C2170: 386A413C  addi r3, r10, 0x413c
	ctx.r[3].s64 = ctx.r[10].s64 + 16700;
	// 826C2174: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826C2178: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C217C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C2180: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C2184: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C2188: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C218C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C2190: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C2194: 4BDA4C8D  bl 0x82466e20
	ctx.lr = 0x826C2198;
	sub_82466E20(ctx, base);
	// 826C2198: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C219C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C21A0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C21A4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C21A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C21A8 size=112
    let mut pc: u32 = 0x826C21A8;
    'dispatch: loop {
        match pc {
            0x826C21A8 => {
    //   block [0x826C21A8..0x826C2218)
	// 826C21A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C21AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C21B0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C21B4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C21B8: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826C21BC: 38AA0DAC  addi r5, r10, 0xdac
	ctx.r[5].s64 = ctx.r[10].s64 + 3500;
	// 826C21C0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C21C4: 390B2840  addi r8, r11, 0x2840
	ctx.r[8].s64 = ctx.r[11].s64 + 10304;
	// 826C21C8: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826C21CC: 388A5700  addi r4, r10, 0x5700
	ctx.r[4].s64 = ctx.r[10].s64 + 22272;
	// 826C21D0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C21D4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C21D8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826C21DC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C21E0: 386A416C  addi r3, r10, 0x416c
	ctx.r[3].s64 = ctx.r[10].s64 + 16748;
	// 826C21E4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826C21E8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C21EC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C21F0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C21F4: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826C21F8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C21FC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C2200: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C2204: 4BDA4C1D  bl 0x82466e20
	ctx.lr = 0x826C2208;
	sub_82466E20(ctx, base);
	// 826C2208: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C220C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C2210: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C2214: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C2218(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C2218 size=112
    let mut pc: u32 = 0x826C2218;
    'dispatch: loop {
        match pc {
            0x826C2218 => {
    //   block [0x826C2218..0x826C2288)
	// 826C2218: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C221C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C2220: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C2224: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C2228: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826C222C: 38AA413C  addi r5, r10, 0x413c
	ctx.r[5].s64 = ctx.r[10].s64 + 16700;
	// 826C2230: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C2234: 390B2858  addi r8, r11, 0x2858
	ctx.r[8].s64 = ctx.r[11].s64 + 10328;
	// 826C2238: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 826C223C: 388A571C  addi r4, r10, 0x571c
	ctx.r[4].s64 = ctx.r[10].s64 + 22300;
	// 826C2240: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C2244: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C2248: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826C224C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C2250: 386A419C  addi r3, r10, 0x419c
	ctx.r[3].s64 = ctx.r[10].s64 + 16796;
	// 826C2254: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826C2258: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C225C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C2260: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C2264: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C2268: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C226C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C2270: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C2274: 4BDA4BAD  bl 0x82466e20
	ctx.lr = 0x826C2278;
	sub_82466E20(ctx, base);
	// 826C2278: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C227C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C2280: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C2284: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C2288(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C2288 size=72
    let mut pc: u32 = 0x826C2288;
    'dispatch: loop {
        match pc {
            0x826C2288 => {
    //   block [0x826C2288..0x826C22D0)
	// 826C2288: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C228C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C2290: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C2294: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 826C2298: 38A00004  li r5, 4
	ctx.r[5].s64 = 4;
	// 826C229C: 38CB02B8  addi r6, r11, 0x2b8
	ctx.r[6].s64 = ctx.r[11].s64 + 696;
	// 826C22A0: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 826C22A4: 388B1530  addi r4, r11, 0x1530
	ctx.r[4].s64 = ctx.r[11].s64 + 5424;
	// 826C22A8: 3D608298  lis r11, -0x7d68
	ctx.r[11].s64 = -2103967744;
	// 826C22AC: 386B41CC  addi r3, r11, 0x41cc
	ctx.r[3].s64 = ctx.r[11].s64 + 16844;
	// 826C22B0: 4BDB97D9  bl 0x8247ba88
	ctx.lr = 0x826C22B4;
	sub_8247BA88(ctx, base);
	// 826C22B4: 3D608271  lis r11, -0x7d8f
	ctx.r[11].s64 = -2106523648;
	// 826C22B8: 386BCED0  addi r3, r11, -0x3130
	ctx.r[3].s64 = ctx.r[11].s64 + -12592;
	// 826C22BC: 4BE7087D  bl 0x82532b38
	ctx.lr = 0x826C22C0;
	sub_82532B38(ctx, base);
	// 826C22C0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 826C22C4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C22C8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C22CC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C22D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C22D0 size=108
    let mut pc: u32 = 0x826C22D0;
    'dispatch: loop {
        match pc {
            0x826C22D0 => {
    //   block [0x826C22D0..0x826C233C)
	// 826C22D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C22D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C22D8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C22DC: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826C22E0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C22E4: 38EB37D0  addi r7, r11, 0x37d0
	ctx.r[7].s64 = ctx.r[11].s64 + 14288;
	// 826C22E8: 39000005  li r8, 5
	ctx.r[8].s64 = 5;
	// 826C22EC: 388A2B24  addi r4, r10, 0x2b24
	ctx.r[4].s64 = ctx.r[10].s64 + 11044;
	// 826C22F0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C22F4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C22F8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826C22FC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C2300: 386A41E4  addi r3, r10, 0x41e4
	ctx.r[3].s64 = ctx.r[10].s64 + 16868;
	// 826C2304: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826C2308: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C230C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C2310: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C2314: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C2318: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C231C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C2320: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C2324: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826C2328: 4BDA4AF9  bl 0x82466e20
	ctx.lr = 0x826C232C;
	sub_82466E20(ctx, base);
	// 826C232C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C2330: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C2334: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C2338: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C2340(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826C2340 size=24
    let mut pc: u32 = 0x826C2340;
    'dispatch: loop {
        match pc {
            0x826C2340 => {
    //   block [0x826C2340..0x826C2358)
	// 826C2340: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826C2344: 3D40827F  lis r10, -0x7d81
	ctx.r[10].s64 = -2105606144;
	// 826C2348: 394A9AD0  addi r10, r10, -0x6530
	ctx.r[10].s64 = ctx.r[10].s64 + -25904;
	// 826C234C: 816B3848  lwz r11, 0x3848(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(14408 as u32) ) } as u64;
	// 826C2350: 916A0038  stw r11, 0x38(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(56 as u32), ctx.r[11].u32 ) };
	// 826C2354: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C2358(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C2358 size=112
    let mut pc: u32 = 0x826C2358;
    'dispatch: loop {
        match pc {
            0x826C2358 => {
    //   block [0x826C2358..0x826C23C8)
	// 826C2358: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C235C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C2360: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C2364: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 826C2368: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C236C: 392B1AC4  addi r9, r11, 0x1ac4
	ctx.r[9].s64 = ctx.r[11].s64 + 6852;
	// 826C2370: 38C00003  li r6, 3
	ctx.r[6].s64 = 3;
	// 826C2374: 38E90014  addi r7, r9, 0x14
	ctx.r[7].s64 = ctx.r[9].s64 + 20;
	// 826C2378: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826C237C: 388A2B3C  addi r4, r10, 0x2b3c
	ctx.r[4].s64 = ctx.r[10].s64 + 11068;
	// 826C2380: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C2384: 396B9AD0  addi r11, r11, -0x6530
	ctx.r[11].s64 = ctx.r[11].s64 + -25904;
	// 826C2388: 90C1005C  stw r6, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[6].u32 ) };
	// 826C238C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C2390: 90E10064  stw r7, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[7].u32 ) };
	// 826C2394: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C2398: 386A4214  addi r3, r10, 0x4214
	ctx.r[3].s64 = ctx.r[10].s64 + 16916;
	// 826C239C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826C23A0: 91010074  stw r8, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[8].u32 ) };
	// 826C23A4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C23A8: 9101006C  stw r8, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[8].u32 ) };
	// 826C23AC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826C23B0: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826C23B4: 4BDA4A6D  bl 0x82466e20
	ctx.lr = 0x826C23B8;
	sub_82466E20(ctx, base);
	// 826C23B8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C23BC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C23C0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C23C4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C23C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C23C8 size=108
    let mut pc: u32 = 0x826C23C8;
    'dispatch: loop {
        match pc {
            0x826C23C8 => {
    //   block [0x826C23C8..0x826C2434)
	// 826C23C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C23CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C23D0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C23D4: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826C23D8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C23DC: 38EB384C  addi r7, r11, 0x384c
	ctx.r[7].s64 = ctx.r[11].s64 + 14412;
	// 826C23E0: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826C23E4: 388A2B50  addi r4, r10, 0x2b50
	ctx.r[4].s64 = ctx.r[10].s64 + 11088;
	// 826C23E8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C23EC: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C23F0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826C23F4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C23F8: 386A4244  addi r3, r10, 0x4244
	ctx.r[3].s64 = ctx.r[10].s64 + 16964;
	// 826C23FC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826C2400: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C2404: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C2408: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C240C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C2410: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C2414: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C2418: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C241C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826C2420: 4BDA4A01  bl 0x82466e20
	ctx.lr = 0x826C2424;
	sub_82466E20(ctx, base);
	// 826C2424: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C2428: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C242C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C2430: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C2438(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C2438 size=108
    let mut pc: u32 = 0x826C2438;
    'dispatch: loop {
        match pc {
            0x826C2438 => {
    //   block [0x826C2438..0x826C24A4)
	// 826C2438: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C243C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C2440: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C2444: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826C2448: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C244C: 38EB387C  addi r7, r11, 0x387c
	ctx.r[7].s64 = ctx.r[11].s64 + 14460;
	// 826C2450: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826C2454: 388A2B6C  addi r4, r10, 0x2b6c
	ctx.r[4].s64 = ctx.r[10].s64 + 11116;
	// 826C2458: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C245C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C2460: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826C2464: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C2468: 386A4274  addi r3, r10, 0x4274
	ctx.r[3].s64 = ctx.r[10].s64 + 17012;
	// 826C246C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826C2470: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C2474: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C2478: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C247C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C2480: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C2484: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C2488: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C248C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826C2490: 4BDA4991  bl 0x82466e20
	ctx.lr = 0x826C2494;
	sub_82466E20(ctx, base);
	// 826C2494: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C2498: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C249C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C24A0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C24A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C24A8 size=112
    let mut pc: u32 = 0x826C24A8;
    'dispatch: loop {
        match pc {
            0x826C24A8 => {
    //   block [0x826C24A8..0x826C2518)
	// 826C24A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C24AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C24B0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C24B4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C24B8: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826C24BC: 38AA4EA4  addi r5, r10, 0x4ea4
	ctx.r[5].s64 = ctx.r[10].s64 + 20132;
	// 826C24C0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C24C4: 390B38B0  addi r8, r11, 0x38b0
	ctx.r[8].s64 = ctx.r[11].s64 + 14512;
	// 826C24C8: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 826C24CC: 388A2BB4  addi r4, r10, 0x2bb4
	ctx.r[4].s64 = ctx.r[10].s64 + 11188;
	// 826C24D0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C24D4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C24D8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826C24DC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C24E0: 386A42A4  addi r3, r10, 0x42a4
	ctx.r[3].s64 = ctx.r[10].s64 + 17060;
	// 826C24E4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826C24E8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C24EC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C24F0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C24F4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C24F8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C24FC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C2500: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C2504: 4BDA491D  bl 0x82466e20
	ctx.lr = 0x826C2508;
	sub_82466E20(ctx, base);
	// 826C2508: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C250C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C2510: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C2514: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C2518(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C2518 size=108
    let mut pc: u32 = 0x826C2518;
    'dispatch: loop {
        match pc {
            0x826C2518 => {
    //   block [0x826C2518..0x826C2584)
	// 826C2518: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C251C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C2520: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C2524: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826C2528: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C252C: 38EB3910  addi r7, r11, 0x3910
	ctx.r[7].s64 = ctx.r[11].s64 + 14608;
	// 826C2530: 39000005  li r8, 5
	ctx.r[8].s64 = 5;
	// 826C2534: 388A2BC8  addi r4, r10, 0x2bc8
	ctx.r[4].s64 = ctx.r[10].s64 + 11208;
	// 826C2538: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C253C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C2540: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826C2544: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C2548: 386A42D4  addi r3, r10, 0x42d4
	ctx.r[3].s64 = ctx.r[10].s64 + 17108;
	// 826C254C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826C2550: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C2554: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C2558: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C255C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C2560: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C2564: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C2568: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C256C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826C2570: 4BDA48B1  bl 0x82466e20
	ctx.lr = 0x826C2574;
	sub_82466E20(ctx, base);
	// 826C2574: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C2578: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C257C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C2580: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C2588(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C2588 size=112
    let mut pc: u32 = 0x826C2588;
    'dispatch: loop {
        match pc {
            0x826C2588 => {
    //   block [0x826C2588..0x826C25F8)
	// 826C2588: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C258C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C2590: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C2594: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C2598: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826C259C: 38AA42A4  addi r5, r10, 0x42a4
	ctx.r[5].s64 = ctx.r[10].s64 + 17060;
	// 826C25A0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C25A4: 390B3988  addi r8, r11, 0x3988
	ctx.r[8].s64 = ctx.r[11].s64 + 14728;
	// 826C25A8: 39200007  li r9, 7
	ctx.r[9].s64 = 7;
	// 826C25AC: 388A2C00  addi r4, r10, 0x2c00
	ctx.r[4].s64 = ctx.r[10].s64 + 11264;
	// 826C25B0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C25B4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C25B8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826C25BC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C25C0: 386A4304  addi r3, r10, 0x4304
	ctx.r[3].s64 = ctx.r[10].s64 + 17156;
	// 826C25C4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826C25C8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C25CC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C25D0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C25D4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C25D8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C25DC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C25E0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C25E4: 4BDA483D  bl 0x82466e20
	ctx.lr = 0x826C25E8;
	sub_82466E20(ctx, base);
	// 826C25E8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C25EC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C25F0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C25F4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C25F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C25F8 size=112
    let mut pc: u32 = 0x826C25F8;
    'dispatch: loop {
        match pc {
            0x826C25F8 => {
    //   block [0x826C25F8..0x826C2668)
	// 826C25F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C25FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C2600: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C2604: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C2608: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826C260C: 38AA42A4  addi r5, r10, 0x42a4
	ctx.r[5].s64 = ctx.r[10].s64 + 17060;
	// 826C2610: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C2614: 390B3A30  addi r8, r11, 0x3a30
	ctx.r[8].s64 = ctx.r[11].s64 + 14896;
	// 826C2618: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826C261C: 388A2C24  addi r4, r10, 0x2c24
	ctx.r[4].s64 = ctx.r[10].s64 + 11300;
	// 826C2620: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C2624: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C2628: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826C262C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C2630: 386A4334  addi r3, r10, 0x4334
	ctx.r[3].s64 = ctx.r[10].s64 + 17204;
	// 826C2634: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826C2638: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C263C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C2640: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C2644: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C2648: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C264C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C2650: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C2654: 4BDA47CD  bl 0x82466e20
	ctx.lr = 0x826C2658;
	sub_82466E20(ctx, base);
	// 826C2658: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C265C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C2660: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C2664: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C2668(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C2668 size=108
    let mut pc: u32 = 0x826C2668;
    'dispatch: loop {
        match pc {
            0x826C2668 => {
    //   block [0x826C2668..0x826C26D4)
	// 826C2668: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C266C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C2670: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C2674: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826C2678: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C267C: 38EB3A48  addi r7, r11, 0x3a48
	ctx.r[7].s64 = ctx.r[11].s64 + 14920;
	// 826C2680: 39000005  li r8, 5
	ctx.r[8].s64 = 5;
	// 826C2684: 388A2C44  addi r4, r10, 0x2c44
	ctx.r[4].s64 = ctx.r[10].s64 + 11332;
	// 826C2688: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C268C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C2690: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826C2694: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C2698: 386A4364  addi r3, r10, 0x4364
	ctx.r[3].s64 = ctx.r[10].s64 + 17252;
	// 826C269C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826C26A0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C26A4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C26A8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C26AC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C26B0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C26B4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C26B8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C26BC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826C26C0: 4BDA4761  bl 0x82466e20
	ctx.lr = 0x826C26C4;
	sub_82466E20(ctx, base);
	// 826C26C4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C26C8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C26CC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C26D0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C26D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C26D8 size=112
    let mut pc: u32 = 0x826C26D8;
    'dispatch: loop {
        match pc {
            0x826C26D8 => {
    //   block [0x826C26D8..0x826C2748)
	// 826C26D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C26DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C26E0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C26E4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C26E8: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826C26EC: 38AA42A4  addi r5, r10, 0x42a4
	ctx.r[5].s64 = ctx.r[10].s64 + 17060;
	// 826C26F0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C26F4: 390B3AC0  addi r8, r11, 0x3ac0
	ctx.r[8].s64 = ctx.r[11].s64 + 15040;
	// 826C26F8: 39200007  li r9, 7
	ctx.r[9].s64 = 7;
	// 826C26FC: 388A2C74  addi r4, r10, 0x2c74
	ctx.r[4].s64 = ctx.r[10].s64 + 11380;
	// 826C2700: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C2704: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C2708: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826C270C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C2710: 386A4394  addi r3, r10, 0x4394
	ctx.r[3].s64 = ctx.r[10].s64 + 17300;
	// 826C2714: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826C2718: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C271C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C2720: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C2724: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C2728: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C272C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C2730: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C2734: 4BDA46ED  bl 0x82466e20
	ctx.lr = 0x826C2738;
	sub_82466E20(ctx, base);
	// 826C2738: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C273C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C2740: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C2744: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C2748(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C2748 size=108
    let mut pc: u32 = 0x826C2748;
    'dispatch: loop {
        match pc {
            0x826C2748 => {
    //   block [0x826C2748..0x826C27B4)
	// 826C2748: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C274C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C2750: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C2754: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826C2758: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C275C: 38EB3B68  addi r7, r11, 0x3b68
	ctx.r[7].s64 = ctx.r[11].s64 + 15208;
	// 826C2760: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826C2764: 388A2C90  addi r4, r10, 0x2c90
	ctx.r[4].s64 = ctx.r[10].s64 + 11408;
	// 826C2768: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C276C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C2770: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826C2774: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C2778: 386A43C4  addi r3, r10, 0x43c4
	ctx.r[3].s64 = ctx.r[10].s64 + 17348;
	// 826C277C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826C2780: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C2784: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C2788: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C278C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C2790: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C2794: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C2798: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C279C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826C27A0: 4BDA4681  bl 0x82466e20
	ctx.lr = 0x826C27A4;
	sub_82466E20(ctx, base);
	// 826C27A4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C27A8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C27AC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C27B0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C27B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C27B8 size=108
    let mut pc: u32 = 0x826C27B8;
    'dispatch: loop {
        match pc {
            0x826C27B8 => {
    //   block [0x826C27B8..0x826C2824)
	// 826C27B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C27BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C27C0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C27C4: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826C27C8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C27CC: 38EB3B80  addi r7, r11, 0x3b80
	ctx.r[7].s64 = ctx.r[11].s64 + 15232;
	// 826C27D0: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 826C27D4: 388A2CA8  addi r4, r10, 0x2ca8
	ctx.r[4].s64 = ctx.r[10].s64 + 11432;
	// 826C27D8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C27DC: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C27E0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826C27E4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C27E8: 386A43F4  addi r3, r10, 0x43f4
	ctx.r[3].s64 = ctx.r[10].s64 + 17396;
	// 826C27EC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826C27F0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C27F4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C27F8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C27FC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C2800: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C2804: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C2808: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C280C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826C2810: 4BDA4611  bl 0x82466e20
	ctx.lr = 0x826C2814;
	sub_82466E20(ctx, base);
	// 826C2814: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C2818: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C281C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C2820: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C2828(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C2828 size=116
    let mut pc: u32 = 0x826C2828;
    'dispatch: loop {
        match pc {
            0x826C2828 => {
    //   block [0x826C2828..0x826C289C)
	// 826C2828: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C282C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C2830: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C2834: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826C2838: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 826C283C: 390B3BE0  addi r8, r11, 0x3be0
	ctx.r[8].s64 = ctx.r[11].s64 + 15328;
	// 826C2840: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C2844: 392A1B00  addi r9, r10, 0x1b00
	ctx.r[9].s64 = ctx.r[10].s64 + 6912;
	// 826C2848: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C284C: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
	// 826C2850: 38AA4EA4  addi r5, r10, 0x4ea4
	ctx.r[5].s64 = ctx.r[10].s64 + 20132;
	// 826C2854: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826C2858: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C285C: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C2860: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C2864: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C2868: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C286C: 3D608298  lis r11, -0x7d68
	ctx.r[11].s64 = -2103967744;
	// 826C2870: 388A2CB8  addi r4, r10, 0x2cb8
	ctx.r[4].s64 = ctx.r[10].s64 + 11448;
	// 826C2874: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826C2878: 386B4424  addi r3, r11, 0x4424
	ctx.r[3].s64 = ctx.r[11].s64 + 17444;
	// 826C287C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826C2880: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C2884: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C2888: 4BDA4599  bl 0x82466e20
	ctx.lr = 0x826C288C;
	sub_82466E20(ctx, base);
	// 826C288C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C2890: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C2894: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C2898: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C28A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C28A0 size=108
    let mut pc: u32 = 0x826C28A0;
    'dispatch: loop {
        match pc {
            0x826C28A0 => {
    //   block [0x826C28A0..0x826C290C)
	// 826C28A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C28A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C28A8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C28AC: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826C28B0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C28B4: 38EB3BF8  addi r7, r11, 0x3bf8
	ctx.r[7].s64 = ctx.r[11].s64 + 15352;
	// 826C28B8: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 826C28BC: 388A2CCC  addi r4, r10, 0x2ccc
	ctx.r[4].s64 = ctx.r[10].s64 + 11468;
	// 826C28C0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C28C4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C28C8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826C28CC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C28D0: 386A4454  addi r3, r10, 0x4454
	ctx.r[3].s64 = ctx.r[10].s64 + 17492;
	// 826C28D4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826C28D8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C28DC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C28E0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C28E4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C28E8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C28EC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C28F0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C28F4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826C28F8: 4BDA4529  bl 0x82466e20
	ctx.lr = 0x826C28FC;
	sub_82466E20(ctx, base);
	// 826C28FC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C2900: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C2904: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C2908: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C2910(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C2910 size=108
    let mut pc: u32 = 0x826C2910;
    'dispatch: loop {
        match pc {
            0x826C2910 => {
    //   block [0x826C2910..0x826C297C)
	// 826C2910: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C2914: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C2918: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C291C: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826C2920: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C2924: 38EB3C40  addi r7, r11, 0x3c40
	ctx.r[7].s64 = ctx.r[11].s64 + 15424;
	// 826C2928: 39000006  li r8, 6
	ctx.r[8].s64 = 6;
	// 826C292C: 388A2CF0  addi r4, r10, 0x2cf0
	ctx.r[4].s64 = ctx.r[10].s64 + 11504;
	// 826C2930: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C2934: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C2938: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826C293C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C2940: 386A4484  addi r3, r10, 0x4484
	ctx.r[3].s64 = ctx.r[10].s64 + 17540;
	// 826C2944: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826C2948: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C294C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C2950: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C2954: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C2958: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C295C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C2960: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C2964: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826C2968: 4BDA44B9  bl 0x82466e20
	ctx.lr = 0x826C296C;
	sub_82466E20(ctx, base);
	// 826C296C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C2970: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C2974: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C2978: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C2980(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C2980 size=108
    let mut pc: u32 = 0x826C2980;
    'dispatch: loop {
        match pc {
            0x826C2980 => {
    //   block [0x826C2980..0x826C29EC)
	// 826C2980: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C2984: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C2988: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C298C: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826C2990: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C2994: 38EB3CD0  addi r7, r11, 0x3cd0
	ctx.r[7].s64 = ctx.r[11].s64 + 15568;
	// 826C2998: 39000006  li r8, 6
	ctx.r[8].s64 = 6;
	// 826C299C: 388A2D14  addi r4, r10, 0x2d14
	ctx.r[4].s64 = ctx.r[10].s64 + 11540;
	// 826C29A0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C29A4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C29A8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826C29AC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C29B0: 386A44B4  addi r3, r10, 0x44b4
	ctx.r[3].s64 = ctx.r[10].s64 + 17588;
	// 826C29B4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826C29B8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C29BC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C29C0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C29C4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C29C8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C29CC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C29D0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C29D4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826C29D8: 4BDA4449  bl 0x82466e20
	ctx.lr = 0x826C29DC;
	sub_82466E20(ctx, base);
	// 826C29DC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C29E0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C29E4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C29E8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C29F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C29F0 size=100
    let mut pc: u32 = 0x826C29F0;
    'dispatch: loop {
        match pc {
            0x826C29F0 => {
    //   block [0x826C29F0..0x826C2A54)
	// 826C29F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C29F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C29F8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C29FC: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C2A00: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C2A04: 38AA4EA4  addi r5, r10, 0x4ea4
	ctx.r[5].s64 = ctx.r[10].s64 + 20132;
	// 826C2A08: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C2A0C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C2A10: 388A2D2C  addi r4, r10, 0x2d2c
	ctx.r[4].s64 = ctx.r[10].s64 + 11564;
	// 826C2A14: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C2A18: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C2A1C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C2A20: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C2A24: 386A44E4  addi r3, r10, 0x44e4
	ctx.r[3].s64 = ctx.r[10].s64 + 17636;
	// 826C2A28: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C2A2C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C2A30: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826C2A34: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C2A38: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826C2A3C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C2A40: 4BDA43E1  bl 0x82466e20
	ctx.lr = 0x826C2A44;
	sub_82466E20(ctx, base);
	// 826C2A44: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C2A48: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C2A4C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C2A50: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C2A58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C2A58 size=112
    let mut pc: u32 = 0x826C2A58;
    'dispatch: loop {
        match pc {
            0x826C2A58 => {
    //   block [0x826C2A58..0x826C2AC8)
	// 826C2A58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C2A5C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C2A60: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C2A64: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C2A68: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826C2A6C: 38AA44E4  addi r5, r10, 0x44e4
	ctx.r[5].s64 = ctx.r[10].s64 + 17636;
	// 826C2A70: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C2A74: 390B3D60  addi r8, r11, 0x3d60
	ctx.r[8].s64 = ctx.r[11].s64 + 15712;
	// 826C2A78: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 826C2A7C: 388A2D48  addi r4, r10, 0x2d48
	ctx.r[4].s64 = ctx.r[10].s64 + 11592;
	// 826C2A80: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C2A84: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C2A88: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826C2A8C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C2A90: 386A4514  addi r3, r10, 0x4514
	ctx.r[3].s64 = ctx.r[10].s64 + 17684;
	// 826C2A94: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826C2A98: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C2A9C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C2AA0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C2AA4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C2AA8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C2AAC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C2AB0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C2AB4: 4BDA436D  bl 0x82466e20
	ctx.lr = 0x826C2AB8;
	sub_82466E20(ctx, base);
	// 826C2AB8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C2ABC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C2AC0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C2AC4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C2AC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C2AC8 size=108
    let mut pc: u32 = 0x826C2AC8;
    'dispatch: loop {
        match pc {
            0x826C2AC8 => {
    //   block [0x826C2AC8..0x826C2B34)
	// 826C2AC8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C2ACC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C2AD0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C2AD4: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826C2AD8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C2ADC: 38EB3DC0  addi r7, r11, 0x3dc0
	ctx.r[7].s64 = ctx.r[11].s64 + 15808;
	// 826C2AE0: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826C2AE4: 388A2D68  addi r4, r10, 0x2d68
	ctx.r[4].s64 = ctx.r[10].s64 + 11624;
	// 826C2AE8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C2AEC: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C2AF0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826C2AF4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C2AF8: 386A4544  addi r3, r10, 0x4544
	ctx.r[3].s64 = ctx.r[10].s64 + 17732;
	// 826C2AFC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826C2B00: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C2B04: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C2B08: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C2B0C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C2B10: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C2B14: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C2B18: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C2B1C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826C2B20: 4BDA4301  bl 0x82466e20
	ctx.lr = 0x826C2B24;
	sub_82466E20(ctx, base);
	// 826C2B24: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C2B28: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C2B2C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C2B30: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C2B38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C2B38 size=108
    let mut pc: u32 = 0x826C2B38;
    'dispatch: loop {
        match pc {
            0x826C2B38 => {
    //   block [0x826C2B38..0x826C2BA4)
	// 826C2B38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C2B3C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C2B40: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C2B44: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826C2B48: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C2B4C: 38EB3DF0  addi r7, r11, 0x3df0
	ctx.r[7].s64 = ctx.r[11].s64 + 15856;
	// 826C2B50: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 826C2B54: 388A2D70  addi r4, r10, 0x2d70
	ctx.r[4].s64 = ctx.r[10].s64 + 11632;
	// 826C2B58: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C2B5C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C2B60: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826C2B64: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C2B68: 386A4574  addi r3, r10, 0x4574
	ctx.r[3].s64 = ctx.r[10].s64 + 17780;
	// 826C2B6C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826C2B70: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C2B74: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C2B78: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C2B7C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C2B80: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C2B84: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C2B88: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C2B8C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826C2B90: 4BDA4291  bl 0x82466e20
	ctx.lr = 0x826C2B94;
	sub_82466E20(ctx, base);
	// 826C2B94: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C2B98: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C2B9C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C2BA0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C2BA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C2BA8 size=108
    let mut pc: u32 = 0x826C2BA8;
    'dispatch: loop {
        match pc {
            0x826C2BA8 => {
    //   block [0x826C2BA8..0x826C2C14)
	// 826C2BA8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C2BAC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C2BB0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C2BB4: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826C2BB8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C2BBC: 38EB3E50  addi r7, r11, 0x3e50
	ctx.r[7].s64 = ctx.r[11].s64 + 15952;
	// 826C2BC0: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 826C2BC4: 388A2D84  addi r4, r10, 0x2d84
	ctx.r[4].s64 = ctx.r[10].s64 + 11652;
	// 826C2BC8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C2BCC: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C2BD0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826C2BD4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C2BD8: 386A45A4  addi r3, r10, 0x45a4
	ctx.r[3].s64 = ctx.r[10].s64 + 17828;
	// 826C2BDC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826C2BE0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C2BE4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C2BE8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C2BEC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C2BF0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C2BF4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C2BF8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C2BFC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826C2C00: 4BDA4221  bl 0x82466e20
	ctx.lr = 0x826C2C04;
	sub_82466E20(ctx, base);
	// 826C2C04: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C2C08: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C2C0C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C2C10: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C2C18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C2C18 size=108
    let mut pc: u32 = 0x826C2C18;
    'dispatch: loop {
        match pc {
            0x826C2C18 => {
    //   block [0x826C2C18..0x826C2C84)
	// 826C2C18: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C2C1C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C2C20: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C2C24: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826C2C28: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C2C2C: 38EB3EB0  addi r7, r11, 0x3eb0
	ctx.r[7].s64 = ctx.r[11].s64 + 16048;
	// 826C2C30: 39000005  li r8, 5
	ctx.r[8].s64 = 5;
	// 826C2C34: 388A2DCC  addi r4, r10, 0x2dcc
	ctx.r[4].s64 = ctx.r[10].s64 + 11724;
	// 826C2C38: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C2C3C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C2C40: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826C2C44: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C2C48: 386A45D4  addi r3, r10, 0x45d4
	ctx.r[3].s64 = ctx.r[10].s64 + 17876;
	// 826C2C4C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826C2C50: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C2C54: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C2C58: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C2C5C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C2C60: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C2C64: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C2C68: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C2C6C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826C2C70: 4BDA41B1  bl 0x82466e20
	ctx.lr = 0x826C2C74;
	sub_82466E20(ctx, base);
	// 826C2C74: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C2C78: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C2C7C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C2C80: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C2C88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C2C88 size=112
    let mut pc: u32 = 0x826C2C88;
    'dispatch: loop {
        match pc {
            0x826C2C88 => {
    //   block [0x826C2C88..0x826C2CF8)
	// 826C2C88: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C2C8C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C2C90: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C2C94: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 826C2C98: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826C2C9C: 392A1B34  addi r9, r10, 0x1b34
	ctx.r[9].s64 = ctx.r[10].s64 + 6964;
	// 826C2CA0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C2CA4: 390B3F30  addi r8, r11, 0x3f30
	ctx.r[8].s64 = ctx.r[11].s64 + 16176;
	// 826C2CA8: 38E0000B  li r7, 0xb
	ctx.r[7].s64 = 11;
	// 826C2CAC: 388A2E00  addi r4, r10, 0x2e00
	ctx.r[4].s64 = ctx.r[10].s64 + 11776;
	// 826C2CB0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C2CB4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C2CB8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826C2CBC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C2CC0: 386A4604  addi r3, r10, 0x4604
	ctx.r[3].s64 = ctx.r[10].s64 + 17924;
	// 826C2CC4: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826C2CC8: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826C2CCC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C2CD0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C2CD4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C2CD8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C2CDC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826C2CE0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C2CE4: 4BDA413D  bl 0x82466e20
	ctx.lr = 0x826C2CE8;
	sub_82466E20(ctx, base);
	// 826C2CE8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C2CEC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C2CF0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C2CF4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C2CF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C2CF8 size=112
    let mut pc: u32 = 0x826C2CF8;
    'dispatch: loop {
        match pc {
            0x826C2CF8 => {
    //   block [0x826C2CF8..0x826C2D68)
	// 826C2CF8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C2CFC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C2D00: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C2D04: 3D40827E  lis r10, -0x7d82
	ctx.r[10].s64 = -2105671680;
	// 826C2D08: 3900000B  li r8, 0xb
	ctx.r[8].s64 = 11;
	// 826C2D0C: 38EA4038  addi r7, r10, 0x4038
	ctx.r[7].s64 = ctx.r[10].s64 + 16440;
	// 826C2D10: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C2D14: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 826C2D18: 388A2E44  addi r4, r10, 0x2e44
	ctx.r[4].s64 = ctx.r[10].s64 + 11844;
	// 826C2D1C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C2D20: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826C2D24: 396B1B48  addi r11, r11, 0x1b48
	ctx.r[11].s64 = ctx.r[11].s64 + 6984;
	// 826C2D28: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826C2D2C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C2D30: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C2D34: 386A4634  addi r3, r10, 0x4634
	ctx.r[3].s64 = ctx.r[10].s64 + 17972;
	// 826C2D38: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C2D3C: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 826C2D40: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C2D44: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 826C2D48: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C2D4C: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C2D50: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826C2D54: 4BDA40CD  bl 0x82466e20
	ctx.lr = 0x826C2D58;
	sub_82466E20(ctx, base);
	// 826C2D58: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C2D5C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C2D60: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C2D64: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C2D68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C2D68 size=112
    let mut pc: u32 = 0x826C2D68;
    'dispatch: loop {
        match pc {
            0x826C2D68 => {
    //   block [0x826C2D68..0x826C2DD8)
	// 826C2D68: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C2D6C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C2D70: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C2D74: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 826C2D78: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826C2D7C: 392A1B8C  addi r9, r10, 0x1b8c
	ctx.r[9].s64 = ctx.r[10].s64 + 7052;
	// 826C2D80: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C2D84: 390B4140  addi r8, r11, 0x4140
	ctx.r[8].s64 = ctx.r[11].s64 + 16704;
	// 826C2D88: 38E00002  li r7, 2
	ctx.r[7].s64 = 2;
	// 826C2D8C: 388A2E64  addi r4, r10, 0x2e64
	ctx.r[4].s64 = ctx.r[10].s64 + 11876;
	// 826C2D90: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C2D94: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C2D98: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826C2D9C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C2DA0: 386A4664  addi r3, r10, 0x4664
	ctx.r[3].s64 = ctx.r[10].s64 + 18020;
	// 826C2DA4: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826C2DA8: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826C2DAC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C2DB0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C2DB4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C2DB8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C2DBC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826C2DC0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C2DC4: 4BDA405D  bl 0x82466e20
	ctx.lr = 0x826C2DC8;
	sub_82466E20(ctx, base);
	// 826C2DC8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C2DCC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C2DD0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C2DD4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C2DD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C2DD8 size=100
    let mut pc: u32 = 0x826C2DD8;
    'dispatch: loop {
        match pc {
            0x826C2DD8 => {
    //   block [0x826C2DD8..0x826C2E3C)
	// 826C2DD8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C2DDC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C2DE0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C2DE4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C2DE8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C2DEC: 38AA4C34  addi r5, r10, 0x4c34
	ctx.r[5].s64 = ctx.r[10].s64 + 19508;
	// 826C2DF0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C2DF4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C2DF8: 388A2E70  addi r4, r10, 0x2e70
	ctx.r[4].s64 = ctx.r[10].s64 + 11888;
	// 826C2DFC: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C2E00: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C2E04: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C2E08: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C2E0C: 386A4694  addi r3, r10, 0x4694
	ctx.r[3].s64 = ctx.r[10].s64 + 18068;
	// 826C2E10: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C2E14: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C2E18: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826C2E1C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C2E20: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826C2E24: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C2E28: 4BDA3FF9  bl 0x82466e20
	ctx.lr = 0x826C2E2C;
	sub_82466E20(ctx, base);
	// 826C2E2C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C2E30: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C2E34: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C2E38: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C2E40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C2E40 size=116
    let mut pc: u32 = 0x826C2E40;
    'dispatch: loop {
        match pc {
            0x826C2E40 => {
    //   block [0x826C2E40..0x826C2EB4)
	// 826C2E40: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C2E44: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C2E48: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C2E4C: 3D40827E  lis r10, -0x7d82
	ctx.r[10].s64 = -2105671680;
	// 826C2E50: 38E00007  li r7, 7
	ctx.r[7].s64 = 7;
	// 826C2E54: 390A4170  addi r8, r10, 0x4170
	ctx.r[8].s64 = ctx.r[10].s64 + 16752;
	// 826C2E58: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C2E5C: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 826C2E60: 38AA4694  addi r5, r10, 0x4694
	ctx.r[5].s64 = ctx.r[10].s64 + 18068;
	// 826C2E64: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C2E68: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826C2E6C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C2E70: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826C2E74: 388A2EDC  addi r4, r10, 0x2edc
	ctx.r[4].s64 = ctx.r[10].s64 + 11996;
	// 826C2E78: 396B1BA0  addi r11, r11, 0x1ba0
	ctx.r[11].s64 = ctx.r[11].s64 + 7072;
	// 826C2E7C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C2E80: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C2E84: 386A46C4  addi r3, r10, 0x46c4
	ctx.r[3].s64 = ctx.r[10].s64 + 18116;
	// 826C2E88: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 826C2E8C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C2E90: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 826C2E94: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C2E98: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C2E9C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C2EA0: 4BDA3F81  bl 0x82466e20
	ctx.lr = 0x826C2EA4;
	sub_82466E20(ctx, base);
	// 826C2EA4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C2EA8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C2EAC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C2EB0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C2EB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C2EB8 size=100
    let mut pc: u32 = 0x826C2EB8;
    'dispatch: loop {
        match pc {
            0x826C2EB8 => {
    //   block [0x826C2EB8..0x826C2F1C)
	// 826C2EB8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C2EBC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C2EC0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C2EC4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C2EC8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C2ECC: 38AA46C4  addi r5, r10, 0x46c4
	ctx.r[5].s64 = ctx.r[10].s64 + 18116;
	// 826C2ED0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C2ED4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C2ED8: 388A2EF8  addi r4, r10, 0x2ef8
	ctx.r[4].s64 = ctx.r[10].s64 + 12024;
	// 826C2EDC: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C2EE0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C2EE4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C2EE8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C2EEC: 386A46F4  addi r3, r10, 0x46f4
	ctx.r[3].s64 = ctx.r[10].s64 + 18164;
	// 826C2EF0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C2EF4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C2EF8: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826C2EFC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C2F00: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826C2F04: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C2F08: 4BDA3F19  bl 0x82466e20
	ctx.lr = 0x826C2F0C;
	sub_82466E20(ctx, base);
	// 826C2F0C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C2F10: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C2F14: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C2F18: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C2F20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C2F20 size=112
    let mut pc: u32 = 0x826C2F20;
    'dispatch: loop {
        match pc {
            0x826C2F20 => {
    //   block [0x826C2F20..0x826C2F90)
	// 826C2F20: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C2F24: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C2F28: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C2F2C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C2F30: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826C2F34: 38AA4EA4  addi r5, r10, 0x4ea4
	ctx.r[5].s64 = ctx.r[10].s64 + 20132;
	// 826C2F38: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C2F3C: 390B4218  addi r8, r11, 0x4218
	ctx.r[8].s64 = ctx.r[11].s64 + 16920;
	// 826C2F40: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 826C2F44: 388A2F1C  addi r4, r10, 0x2f1c
	ctx.r[4].s64 = ctx.r[10].s64 + 12060;
	// 826C2F48: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C2F4C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C2F50: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826C2F54: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C2F58: 386A4724  addi r3, r10, 0x4724
	ctx.r[3].s64 = ctx.r[10].s64 + 18212;
	// 826C2F5C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826C2F60: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C2F64: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C2F68: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C2F6C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C2F70: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C2F74: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C2F78: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C2F7C: 4BDA3EA5  bl 0x82466e20
	ctx.lr = 0x826C2F80;
	sub_82466E20(ctx, base);
	// 826C2F80: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C2F84: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C2F88: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C2F8C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C2F90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C2F90 size=116
    let mut pc: u32 = 0x826C2F90;
    'dispatch: loop {
        match pc {
            0x826C2F90 => {
    //   block [0x826C2F90..0x826C3004)
	// 826C2F90: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C2F94: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C2F98: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C2F9C: 3D40827E  lis r10, -0x7d82
	ctx.r[10].s64 = -2105671680;
	// 826C2FA0: 38E00007  li r7, 7
	ctx.r[7].s64 = 7;
	// 826C2FA4: 390A4260  addi r8, r10, 0x4260
	ctx.r[8].s64 = ctx.r[10].s64 + 16992;
	// 826C2FA8: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C2FAC: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 826C2FB0: 38AA4694  addi r5, r10, 0x4694
	ctx.r[5].s64 = ctx.r[10].s64 + 18068;
	// 826C2FB4: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C2FB8: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826C2FBC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C2FC0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826C2FC4: 388A2F38  addi r4, r10, 0x2f38
	ctx.r[4].s64 = ctx.r[10].s64 + 12088;
	// 826C2FC8: 396B1BCC  addi r11, r11, 0x1bcc
	ctx.r[11].s64 = ctx.r[11].s64 + 7116;
	// 826C2FCC: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C2FD0: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C2FD4: 386A4754  addi r3, r10, 0x4754
	ctx.r[3].s64 = ctx.r[10].s64 + 18260;
	// 826C2FD8: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 826C2FDC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C2FE0: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 826C2FE4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C2FE8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C2FEC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C2FF0: 4BDA3E31  bl 0x82466e20
	ctx.lr = 0x826C2FF4;
	sub_82466E20(ctx, base);
	// 826C2FF4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C2FF8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C2FFC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C3000: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C3008(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C3008 size=108
    let mut pc: u32 = 0x826C3008;
    'dispatch: loop {
        match pc {
            0x826C3008 => {
    //   block [0x826C3008..0x826C3074)
	// 826C3008: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C300C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C3010: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C3014: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826C3018: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C301C: 38EB4308  addi r7, r11, 0x4308
	ctx.r[7].s64 = ctx.r[11].s64 + 17160;
	// 826C3020: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 826C3024: 388A2F4C  addi r4, r10, 0x2f4c
	ctx.r[4].s64 = ctx.r[10].s64 + 12108;
	// 826C3028: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C302C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C3030: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826C3034: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C3038: 386A4784  addi r3, r10, 0x4784
	ctx.r[3].s64 = ctx.r[10].s64 + 18308;
	// 826C303C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826C3040: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C3044: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C3048: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C304C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C3050: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C3054: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C3058: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C305C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826C3060: 4BDA3DC1  bl 0x82466e20
	ctx.lr = 0x826C3064;
	sub_82466E20(ctx, base);
	// 826C3064: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C3068: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C306C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C3070: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C3078(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826C3078 size=24
    let mut pc: u32 = 0x826C3078;
    'dispatch: loop {
        match pc {
            0x826C3078 => {
    //   block [0x826C3078..0x826C3090)
	// 826C3078: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826C307C: 3D40827F  lis r10, -0x7d81
	ctx.r[10].s64 = -2105606144;
	// 826C3080: 394A9B18  addi r10, r10, -0x64e8
	ctx.r[10].s64 = ctx.r[10].s64 + -25832;
	// 826C3084: 816B4350  lwz r11, 0x4350(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(17232 as u32) ) } as u64;
	// 826C3088: 916A0008  stw r11, 8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 826C308C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C3090(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C3090 size=116
    let mut pc: u32 = 0x826C3090;
    'dispatch: loop {
        match pc {
            0x826C3090 => {
    //   block [0x826C3090..0x826C3104)
	// 826C3090: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C3094: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C3098: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C309C: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 826C30A0: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C30A4: 392B1C14  addi r9, r11, 0x1c14
	ctx.r[9].s64 = ctx.r[11].s64 + 7188;
	// 826C30A8: 38AA4694  addi r5, r10, 0x4694
	ctx.r[5].s64 = ctx.r[10].s64 + 18068;
	// 826C30AC: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C30B0: 38E90014  addi r7, r9, 0x14
	ctx.r[7].s64 = ctx.r[9].s64 + 20;
	// 826C30B4: 38C0000E  li r6, 0xe
	ctx.r[6].s64 = 14;
	// 826C30B8: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826C30BC: 388A3040  addi r4, r10, 0x3040
	ctx.r[4].s64 = ctx.r[10].s64 + 12352;
	// 826C30C0: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C30C4: 396B9B18  addi r11, r11, -0x64e8
	ctx.r[11].s64 = ctx.r[11].s64 + -25832;
	// 826C30C8: 90E10064  stw r7, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[7].u32 ) };
	// 826C30CC: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C30D0: 90C1005C  stw r6, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[6].u32 ) };
	// 826C30D4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C30D8: 386A47B4  addi r3, r10, 0x47b4
	ctx.r[3].s64 = ctx.r[10].s64 + 18356;
	// 826C30DC: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826C30E0: 91010074  stw r8, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[8].u32 ) };
	// 826C30E4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C30E8: 9101006C  stw r8, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[8].u32 ) };
	// 826C30EC: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826C30F0: 4BDA3D31  bl 0x82466e20
	ctx.lr = 0x826C30F4;
	sub_82466E20(ctx, base);
	// 826C30F4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C30F8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C30FC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C3100: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C3108(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C3108 size=112
    let mut pc: u32 = 0x826C3108;
    'dispatch: loop {
        match pc {
            0x826C3108 => {
    //   block [0x826C3108..0x826C3178)
	// 826C3108: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C310C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C3110: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C3114: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C3118: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826C311C: 38AA4694  addi r5, r10, 0x4694
	ctx.r[5].s64 = ctx.r[10].s64 + 18068;
	// 826C3120: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C3124: 390B4354  addi r8, r11, 0x4354
	ctx.r[8].s64 = ctx.r[11].s64 + 17236;
	// 826C3128: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826C312C: 388A3054  addi r4, r10, 0x3054
	ctx.r[4].s64 = ctx.r[10].s64 + 12372;
	// 826C3130: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C3134: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C3138: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826C313C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C3140: 386A47E4  addi r3, r10, 0x47e4
	ctx.r[3].s64 = ctx.r[10].s64 + 18404;
	// 826C3144: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826C3148: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C314C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C3150: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C3154: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C3158: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C315C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C3160: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C3164: 4BDA3CBD  bl 0x82466e20
	ctx.lr = 0x826C3168;
	sub_82466E20(ctx, base);
	// 826C3168: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C316C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C3170: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C3174: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C3178(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C3178 size=112
    let mut pc: u32 = 0x826C3178;
    'dispatch: loop {
        match pc {
            0x826C3178 => {
    //   block [0x826C3178..0x826C31E8)
	// 826C3178: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C317C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C3180: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C3184: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C3188: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826C318C: 38AA4694  addi r5, r10, 0x4694
	ctx.r[5].s64 = ctx.r[10].s64 + 18068;
	// 826C3190: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C3194: 390B4384  addi r8, r11, 0x4384
	ctx.r[8].s64 = ctx.r[11].s64 + 17284;
	// 826C3198: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826C319C: 388A30D8  addi r4, r10, 0x30d8
	ctx.r[4].s64 = ctx.r[10].s64 + 12504;
	// 826C31A0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C31A4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C31A8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826C31AC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C31B0: 386A4814  addi r3, r10, 0x4814
	ctx.r[3].s64 = ctx.r[10].s64 + 18452;
	// 826C31B4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826C31B8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C31BC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C31C0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C31C4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C31C8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C31CC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C31D0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C31D4: 4BDA3C4D  bl 0x82466e20
	ctx.lr = 0x826C31D8;
	sub_82466E20(ctx, base);
	// 826C31D8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C31DC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C31E0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C31E4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C31E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C31E8 size=100
    let mut pc: u32 = 0x826C31E8;
    'dispatch: loop {
        match pc {
            0x826C31E8 => {
    //   block [0x826C31E8..0x826C324C)
	// 826C31E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C31EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C31F0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C31F4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C31F8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C31FC: 38AA4C34  addi r5, r10, 0x4c34
	ctx.r[5].s64 = ctx.r[10].s64 + 19508;
	// 826C3200: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C3204: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C3208: 388A30F4  addi r4, r10, 0x30f4
	ctx.r[4].s64 = ctx.r[10].s64 + 12532;
	// 826C320C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C3210: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C3214: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C3218: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C321C: 386A4844  addi r3, r10, 0x4844
	ctx.r[3].s64 = ctx.r[10].s64 + 18500;
	// 826C3220: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C3224: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C3228: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826C322C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C3230: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826C3234: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C3238: 4BDA3BE9  bl 0x82466e20
	ctx.lr = 0x826C323C;
	sub_82466E20(ctx, base);
	// 826C323C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C3240: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C3244: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C3248: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C3250(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C3250 size=112
    let mut pc: u32 = 0x826C3250;
    'dispatch: loop {
        match pc {
            0x826C3250 => {
    //   block [0x826C3250..0x826C32C0)
	// 826C3250: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C3254: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C3258: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C325C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C3260: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826C3264: 38AA4844  addi r5, r10, 0x4844
	ctx.r[5].s64 = ctx.r[10].s64 + 18500;
	// 826C3268: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C326C: 390B439C  addi r8, r11, 0x439c
	ctx.r[8].s64 = ctx.r[11].s64 + 17308;
	// 826C3270: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826C3274: 388A3164  addi r4, r10, 0x3164
	ctx.r[4].s64 = ctx.r[10].s64 + 12644;
	// 826C3278: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C327C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C3280: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826C3284: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C3288: 386A4874  addi r3, r10, 0x4874
	ctx.r[3].s64 = ctx.r[10].s64 + 18548;
	// 826C328C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826C3290: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C3294: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C3298: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C329C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C32A0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C32A4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C32A8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C32AC: 4BDA3B75  bl 0x82466e20
	ctx.lr = 0x826C32B0;
	sub_82466E20(ctx, base);
	// 826C32B0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C32B4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C32B8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C32BC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C32C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C32C0 size=108
    let mut pc: u32 = 0x826C32C0;
    'dispatch: loop {
        match pc {
            0x826C32C0 => {
    //   block [0x826C32C0..0x826C332C)
	// 826C32C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C32C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C32C8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C32CC: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826C32D0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C32D4: 38EB43D0  addi r7, r11, 0x43d0
	ctx.r[7].s64 = ctx.r[11].s64 + 17360;
	// 826C32D8: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 826C32DC: 388A317C  addi r4, r10, 0x317c
	ctx.r[4].s64 = ctx.r[10].s64 + 12668;
	// 826C32E0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C32E4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C32E8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826C32EC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C32F0: 386A48A4  addi r3, r10, 0x48a4
	ctx.r[3].s64 = ctx.r[10].s64 + 18596;
	// 826C32F4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826C32F8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C32FC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C3300: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C3304: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C3308: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C330C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C3310: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C3314: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826C3318: 4BDA3B09  bl 0x82466e20
	ctx.lr = 0x826C331C;
	sub_82466E20(ctx, base);
	// 826C331C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C3320: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C3324: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C3328: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C3330(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C3330 size=108
    let mut pc: u32 = 0x826C3330;
    'dispatch: loop {
        match pc {
            0x826C3330 => {
    //   block [0x826C3330..0x826C339C)
	// 826C3330: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C3334: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C3338: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C333C: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826C3340: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C3344: 38EB4430  addi r7, r11, 0x4430
	ctx.r[7].s64 = ctx.r[11].s64 + 17456;
	// 826C3348: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826C334C: 388A318C  addi r4, r10, 0x318c
	ctx.r[4].s64 = ctx.r[10].s64 + 12684;
	// 826C3350: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C3354: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C3358: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826C335C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C3360: 386A48D4  addi r3, r10, 0x48d4
	ctx.r[3].s64 = ctx.r[10].s64 + 18644;
	// 826C3364: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826C3368: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C336C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C3370: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C3374: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C3378: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C337C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C3380: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C3384: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826C3388: 4BDA3A99  bl 0x82466e20
	ctx.lr = 0x826C338C;
	sub_82466E20(ctx, base);
	// 826C338C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C3390: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C3394: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C3398: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C33A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C33A0 size=116
    let mut pc: u32 = 0x826C33A0;
    'dispatch: loop {
        match pc {
            0x826C33A0 => {
    //   block [0x826C33A0..0x826C3414)
	// 826C33A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C33A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C33A8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C33AC: 3D40827E  lis r10, -0x7d82
	ctx.r[10].s64 = -2105671680;
	// 826C33B0: 38E00010  li r7, 0x10
	ctx.r[7].s64 = 16;
	// 826C33B4: 390A4460  addi r8, r10, 0x4460
	ctx.r[8].s64 = ctx.r[10].s64 + 17504;
	// 826C33B8: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C33BC: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 826C33C0: 38AA4844  addi r5, r10, 0x4844
	ctx.r[5].s64 = ctx.r[10].s64 + 18500;
	// 826C33C4: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C33C8: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826C33CC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C33D0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826C33D4: 388A31E0  addi r4, r10, 0x31e0
	ctx.r[4].s64 = ctx.r[10].s64 + 12768;
	// 826C33D8: 396B1C70  addi r11, r11, 0x1c70
	ctx.r[11].s64 = ctx.r[11].s64 + 7280;
	// 826C33DC: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C33E0: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C33E4: 386A4904  addi r3, r10, 0x4904
	ctx.r[3].s64 = ctx.r[10].s64 + 18692;
	// 826C33E8: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 826C33EC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C33F0: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 826C33F4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C33F8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C33FC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C3400: 4BDA3A21  bl 0x82466e20
	ctx.lr = 0x826C3404;
	sub_82466E20(ctx, base);
	// 826C3404: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C3408: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C340C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C3410: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C3418(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C3418 size=112
    let mut pc: u32 = 0x826C3418;
    'dispatch: loop {
        match pc {
            0x826C3418 => {
    //   block [0x826C3418..0x826C3488)
	// 826C3418: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C341C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C3420: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C3424: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C3428: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826C342C: 38AA4844  addi r5, r10, 0x4844
	ctx.r[5].s64 = ctx.r[10].s64 + 18500;
	// 826C3430: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C3434: 390B45E0  addi r8, r11, 0x45e0
	ctx.r[8].s64 = ctx.r[11].s64 + 17888;
	// 826C3438: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826C343C: 388A31F4  addi r4, r10, 0x31f4
	ctx.r[4].s64 = ctx.r[10].s64 + 12788;
	// 826C3440: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C3444: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C3448: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826C344C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C3450: 386A4934  addi r3, r10, 0x4934
	ctx.r[3].s64 = ctx.r[10].s64 + 18740;
	// 826C3454: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826C3458: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C345C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C3460: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C3464: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C3468: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C346C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C3470: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C3474: 4BDA39AD  bl 0x82466e20
	ctx.lr = 0x826C3478;
	sub_82466E20(ctx, base);
	// 826C3478: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C347C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C3480: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C3484: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C3488(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C3488 size=116
    let mut pc: u32 = 0x826C3488;
    'dispatch: loop {
        match pc {
            0x826C3488 => {
    //   block [0x826C3488..0x826C34FC)
	// 826C3488: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C348C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C3490: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C3494: 3D40827E  lis r10, -0x7d82
	ctx.r[10].s64 = -2105671680;
	// 826C3498: 38E00004  li r7, 4
	ctx.r[7].s64 = 4;
	// 826C349C: 390A45F8  addi r8, r10, 0x45f8
	ctx.r[8].s64 = ctx.r[10].s64 + 17912;
	// 826C34A0: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C34A4: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 826C34A8: 38AA4844  addi r5, r10, 0x4844
	ctx.r[5].s64 = ctx.r[10].s64 + 18500;
	// 826C34AC: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C34B0: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826C34B4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C34B8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826C34BC: 388A3210  addi r4, r10, 0x3210
	ctx.r[4].s64 = ctx.r[10].s64 + 12816;
	// 826C34C0: 396B1CBC  addi r11, r11, 0x1cbc
	ctx.r[11].s64 = ctx.r[11].s64 + 7356;
	// 826C34C4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C34C8: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C34CC: 386A4964  addi r3, r10, 0x4964
	ctx.r[3].s64 = ctx.r[10].s64 + 18788;
	// 826C34D0: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 826C34D4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C34D8: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 826C34DC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C34E0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C34E4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C34E8: 4BDA3939  bl 0x82466e20
	ctx.lr = 0x826C34EC;
	sub_82466E20(ctx, base);
	// 826C34EC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C34F0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C34F4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C34F8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C3500(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C3500 size=112
    let mut pc: u32 = 0x826C3500;
    'dispatch: loop {
        match pc {
            0x826C3500 => {
    //   block [0x826C3500..0x826C3570)
	// 826C3500: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C3504: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C3508: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C350C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C3510: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826C3514: 38AA4844  addi r5, r10, 0x4844
	ctx.r[5].s64 = ctx.r[10].s64 + 18500;
	// 826C3518: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C351C: 390B4658  addi r8, r11, 0x4658
	ctx.r[8].s64 = ctx.r[11].s64 + 18008;
	// 826C3520: 39200009  li r9, 9
	ctx.r[9].s64 = 9;
	// 826C3524: 388A3260  addi r4, r10, 0x3260
	ctx.r[4].s64 = ctx.r[10].s64 + 12896;
	// 826C3528: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C352C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C3530: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826C3534: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C3538: 386A4994  addi r3, r10, 0x4994
	ctx.r[3].s64 = ctx.r[10].s64 + 18836;
	// 826C353C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826C3540: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C3544: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C3548: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C354C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C3550: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C3554: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C3558: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C355C: 4BDA38C5  bl 0x82466e20
	ctx.lr = 0x826C3560;
	sub_82466E20(ctx, base);
	// 826C3560: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C3564: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C3568: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C356C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C3570(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C3570 size=112
    let mut pc: u32 = 0x826C3570;
    'dispatch: loop {
        match pc {
            0x826C3570 => {
    //   block [0x826C3570..0x826C35E0)
	// 826C3570: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C3574: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C3578: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C357C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C3580: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826C3584: 38AA4844  addi r5, r10, 0x4844
	ctx.r[5].s64 = ctx.r[10].s64 + 18500;
	// 826C3588: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C358C: 390B4730  addi r8, r11, 0x4730
	ctx.r[8].s64 = ctx.r[11].s64 + 18224;
	// 826C3590: 3920000B  li r9, 0xb
	ctx.r[9].s64 = 11;
	// 826C3594: 388A3274  addi r4, r10, 0x3274
	ctx.r[4].s64 = ctx.r[10].s64 + 12916;
	// 826C3598: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C359C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C35A0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826C35A4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C35A8: 386A49C4  addi r3, r10, 0x49c4
	ctx.r[3].s64 = ctx.r[10].s64 + 18884;
	// 826C35AC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826C35B0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C35B4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C35B8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C35BC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C35C0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C35C4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C35C8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C35CC: 4BDA3855  bl 0x82466e20
	ctx.lr = 0x826C35D0;
	sub_82466E20(ctx, base);
	// 826C35D0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C35D4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C35D8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C35DC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C35E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C35E0 size=112
    let mut pc: u32 = 0x826C35E0;
    'dispatch: loop {
        match pc {
            0x826C35E0 => {
    //   block [0x826C35E0..0x826C3650)
	// 826C35E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C35E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C35E8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C35EC: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C35F0: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826C35F4: 38AA4844  addi r5, r10, 0x4844
	ctx.r[5].s64 = ctx.r[10].s64 + 18500;
	// 826C35F8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826C35FC: 390B4838  addi r8, r11, 0x4838
	ctx.r[8].s64 = ctx.r[11].s64 + 18488;
	// 826C3600: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826C3604: 388A43BC  addi r4, r10, 0x43bc
	ctx.r[4].s64 = ctx.r[10].s64 + 17340;
	// 826C3608: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C360C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C3610: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826C3614: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C3618: 386A49F4  addi r3, r10, 0x49f4
	ctx.r[3].s64 = ctx.r[10].s64 + 18932;
	// 826C361C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826C3620: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C3624: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C3628: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C362C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C3630: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C3634: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C3638: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C363C: 4BDA37E5  bl 0x82466e20
	ctx.lr = 0x826C3640;
	sub_82466E20(ctx, base);
	// 826C3640: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C3644: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C3648: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C364C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C3650(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826C3650 size=24
    let mut pc: u32 = 0x826C3650;
    'dispatch: loop {
        match pc {
            0x826C3650 => {
    //   block [0x826C3650..0x826C3668)
	// 826C3650: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826C3654: 3D40827F  lis r10, -0x7d81
	ctx.r[10].s64 = -2105606144;
	// 826C3658: 394A9C68  addi r10, r10, -0x6398
	ctx.r[10].s64 = ctx.r[10].s64 + -25496;
	// 826C365C: 816B43CC  lwz r11, 0x43cc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(17356 as u32) ) } as u64;
	// 826C3660: 916A0068  stw r11, 0x68(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(104 as u32), ctx.r[11].u32 ) };
	// 826C3664: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C3668(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C3668 size=116
    let mut pc: u32 = 0x826C3668;
    'dispatch: loop {
        match pc {
            0x826C3668 => {
    //   block [0x826C3668..0x826C36DC)
	// 826C3668: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C366C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C3670: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C3674: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 826C3678: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C367C: 392B1CEC  addi r9, r11, 0x1cec
	ctx.r[9].s64 = ctx.r[11].s64 + 7404;
	// 826C3680: 38AA4844  addi r5, r10, 0x4844
	ctx.r[5].s64 = ctx.r[10].s64 + 18500;
	// 826C3684: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826C3688: 38E90014  addi r7, r9, 0x14
	ctx.r[7].s64 = ctx.r[9].s64 + 20;
	// 826C368C: 38C00011  li r6, 0x11
	ctx.r[6].s64 = 17;
	// 826C3690: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826C3694: 388A4454  addi r4, r10, 0x4454
	ctx.r[4].s64 = ctx.r[10].s64 + 17492;
	// 826C3698: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C369C: 396B9C68  addi r11, r11, -0x6398
	ctx.r[11].s64 = ctx.r[11].s64 + -25496;
	// 826C36A0: 90E10064  stw r7, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[7].u32 ) };
	// 826C36A4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C36A8: 90C1005C  stw r6, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[6].u32 ) };
	// 826C36AC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C36B0: 386A4A24  addi r3, r10, 0x4a24
	ctx.r[3].s64 = ctx.r[10].s64 + 18980;
	// 826C36B4: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826C36B8: 91010074  stw r8, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[8].u32 ) };
	// 826C36BC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C36C0: 9101006C  stw r8, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[8].u32 ) };
	// 826C36C4: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826C36C8: 4BDA3759  bl 0x82466e20
	ctx.lr = 0x826C36CC;
	sub_82466E20(ctx, base);
	// 826C36CC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C36D0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C36D4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C36D8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C36E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C36E0 size=116
    let mut pc: u32 = 0x826C36E0;
    'dispatch: loop {
        match pc {
            0x826C36E0 => {
    //   block [0x826C36E0..0x826C3754)
	// 826C36E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C36E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C36E8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C36EC: 3D40827E  lis r10, -0x7d82
	ctx.r[10].s64 = -2105671680;
	// 826C36F0: 38E00002  li r7, 2
	ctx.r[7].s64 = 2;
	// 826C36F4: 390A4868  addi r8, r10, 0x4868
	ctx.r[8].s64 = ctx.r[10].s64 + 18536;
	// 826C36F8: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C36FC: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 826C3700: 38AA4EA4  addi r5, r10, 0x4ea4
	ctx.r[5].s64 = ctx.r[10].s64 + 20132;
	// 826C3704: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C3708: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826C370C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C3710: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826C3714: 388A32A4  addi r4, r10, 0x32a4
	ctx.r[4].s64 = ctx.r[10].s64 + 12964;
	// 826C3718: 396B1D5C  addi r11, r11, 0x1d5c
	ctx.r[11].s64 = ctx.r[11].s64 + 7516;
	// 826C371C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C3720: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C3724: 386A4A54  addi r3, r10, 0x4a54
	ctx.r[3].s64 = ctx.r[10].s64 + 19028;
	// 826C3728: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 826C372C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C3730: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 826C3734: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C3738: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C373C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C3740: 4BDA36E1  bl 0x82466e20
	ctx.lr = 0x826C3744;
	sub_82466E20(ctx, base);
	// 826C3744: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C3748: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C374C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C3750: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C3758(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C3758 size=108
    let mut pc: u32 = 0x826C3758;
    'dispatch: loop {
        match pc {
            0x826C3758 => {
    //   block [0x826C3758..0x826C37C4)
	// 826C3758: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C375C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C3760: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C3764: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826C3768: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C376C: 38EB4898  addi r7, r11, 0x4898
	ctx.r[7].s64 = ctx.r[11].s64 + 18584;
	// 826C3770: 39000006  li r8, 6
	ctx.r[8].s64 = 6;
	// 826C3774: 388A32B4  addi r4, r10, 0x32b4
	ctx.r[4].s64 = ctx.r[10].s64 + 12980;
	// 826C3778: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C377C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C3780: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826C3784: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C3788: 386A4A84  addi r3, r10, 0x4a84
	ctx.r[3].s64 = ctx.r[10].s64 + 19076;
	// 826C378C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826C3790: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C3794: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C3798: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C379C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C37A0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C37A4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C37A8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C37AC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826C37B0: 4BDA3671  bl 0x82466e20
	ctx.lr = 0x826C37B4;
	sub_82466E20(ctx, base);
	// 826C37B4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C37B8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C37BC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C37C0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C37C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826C37C8 size=24
    let mut pc: u32 = 0x826C37C8;
    'dispatch: loop {
        match pc {
            0x826C37C8 => {
    //   block [0x826C37C8..0x826C37E0)
	// 826C37C8: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826C37CC: 3D40827F  lis r10, -0x7d81
	ctx.r[10].s64 = -2105606144;
	// 826C37D0: 394A9E00  addi r10, r10, -0x6200
	ctx.r[10].s64 = ctx.r[10].s64 + -25088;
	// 826C37D4: 816B4928  lwz r11, 0x4928(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(18728 as u32) ) } as u64;
	// 826C37D8: 916A0050  stw r11, 0x50(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 826C37DC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C37E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C37E0 size=116
    let mut pc: u32 = 0x826C37E0;
    'dispatch: loop {
        match pc {
            0x826C37E0 => {
    //   block [0x826C37E0..0x826C3854)
	// 826C37E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C37E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C37E8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C37EC: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 826C37F0: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C37F4: 392B1D80  addi r9, r11, 0x1d80
	ctx.r[9].s64 = ctx.r[11].s64 + 7552;
	// 826C37F8: 38AA4844  addi r5, r10, 0x4844
	ctx.r[5].s64 = ctx.r[10].s64 + 18500;
	// 826C37FC: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C3800: 38E90014  addi r7, r9, 0x14
	ctx.r[7].s64 = ctx.r[9].s64 + 20;
	// 826C3804: 38C0000B  li r6, 0xb
	ctx.r[6].s64 = 11;
	// 826C3808: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826C380C: 388A3300  addi r4, r10, 0x3300
	ctx.r[4].s64 = ctx.r[10].s64 + 13056;
	// 826C3810: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C3814: 396B9E00  addi r11, r11, -0x6200
	ctx.r[11].s64 = ctx.r[11].s64 + -25088;
	// 826C3818: 90E10064  stw r7, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[7].u32 ) };
	// 826C381C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C3820: 90C1005C  stw r6, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[6].u32 ) };
	// 826C3824: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C3828: 386A4AB4  addi r3, r10, 0x4ab4
	ctx.r[3].s64 = ctx.r[10].s64 + 19124;
	// 826C382C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826C3830: 91010074  stw r8, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[8].u32 ) };
	// 826C3834: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C3838: 9101006C  stw r8, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[8].u32 ) };
	// 826C383C: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826C3840: 4BDA35E1  bl 0x82466e20
	ctx.lr = 0x826C3844;
	sub_82466E20(ctx, base);
	// 826C3844: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C3848: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C384C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C3850: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C3858(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C3858 size=112
    let mut pc: u32 = 0x826C3858;
    'dispatch: loop {
        match pc {
            0x826C3858 => {
    //   block [0x826C3858..0x826C38C8)
	// 826C3858: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C385C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C3860: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C3864: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C3868: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826C386C: 38AA4844  addi r5, r10, 0x4844
	ctx.r[5].s64 = ctx.r[10].s64 + 18500;
	// 826C3870: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C3874: 390B492C  addi r8, r11, 0x492c
	ctx.r[8].s64 = ctx.r[11].s64 + 18732;
	// 826C3878: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826C387C: 388A331C  addi r4, r10, 0x331c
	ctx.r[4].s64 = ctx.r[10].s64 + 13084;
	// 826C3880: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C3884: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C3888: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826C388C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C3890: 386A4AE4  addi r3, r10, 0x4ae4
	ctx.r[3].s64 = ctx.r[10].s64 + 19172;
	// 826C3894: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826C3898: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C389C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C38A0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C38A4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C38A8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C38AC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C38B0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C38B4: 4BDA356D  bl 0x82466e20
	ctx.lr = 0x826C38B8;
	sub_82466E20(ctx, base);
	// 826C38B8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C38BC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C38C0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C38C4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C38C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C38C8 size=116
    let mut pc: u32 = 0x826C38C8;
    'dispatch: loop {
        match pc {
            0x826C38C8 => {
    //   block [0x826C38C8..0x826C393C)
	// 826C38C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C38CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C38D0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C38D4: 3D40827E  lis r10, -0x7d82
	ctx.r[10].s64 = -2105671680;
	// 826C38D8: 38E00006  li r7, 6
	ctx.r[7].s64 = 6;
	// 826C38DC: 390A4960  addi r8, r10, 0x4960
	ctx.r[8].s64 = ctx.r[10].s64 + 18784;
	// 826C38E0: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C38E4: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 826C38E8: 38AA4844  addi r5, r10, 0x4844
	ctx.r[5].s64 = ctx.r[10].s64 + 18500;
	// 826C38EC: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C38F0: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826C38F4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C38F8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826C38FC: 388A3340  addi r4, r10, 0x3340
	ctx.r[4].s64 = ctx.r[10].s64 + 13120;
	// 826C3900: 396B1DC8  addi r11, r11, 0x1dc8
	ctx.r[11].s64 = ctx.r[11].s64 + 7624;
	// 826C3904: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C3908: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C390C: 386A4B14  addi r3, r10, 0x4b14
	ctx.r[3].s64 = ctx.r[10].s64 + 19220;
	// 826C3910: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 826C3914: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C3918: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 826C391C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C3920: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C3924: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C3928: 4BDA34F9  bl 0x82466e20
	ctx.lr = 0x826C392C;
	sub_82466E20(ctx, base);
	// 826C392C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C3930: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C3934: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C3938: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C3940(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C3940 size=108
    let mut pc: u32 = 0x826C3940;
    'dispatch: loop {
        match pc {
            0x826C3940 => {
    //   block [0x826C3940..0x826C39AC)
	// 826C3940: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C3944: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C3948: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C394C: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826C3950: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C3954: 38EB49F0  addi r7, r11, 0x49f0
	ctx.r[7].s64 = ctx.r[11].s64 + 18928;
	// 826C3958: 3900000E  li r8, 0xe
	ctx.r[8].s64 = 14;
	// 826C395C: 388A3374  addi r4, r10, 0x3374
	ctx.r[4].s64 = ctx.r[10].s64 + 13172;
	// 826C3960: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C3964: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C3968: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826C396C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C3970: 386A4B44  addi r3, r10, 0x4b44
	ctx.r[3].s64 = ctx.r[10].s64 + 19268;
	// 826C3974: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826C3978: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C397C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C3980: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C3984: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C3988: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C398C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C3990: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C3994: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826C3998: 4BDA3489  bl 0x82466e20
	ctx.lr = 0x826C399C;
	sub_82466E20(ctx, base);
	// 826C399C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C39A0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C39A4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C39A8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C39B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C39B0 size=116
    let mut pc: u32 = 0x826C39B0;
    'dispatch: loop {
        match pc {
            0x826C39B0 => {
    //   block [0x826C39B0..0x826C3A24)
	// 826C39B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C39B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C39B8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C39BC: 3D40827E  lis r10, -0x7d82
	ctx.r[10].s64 = -2105671680;
	// 826C39C0: 38E00008  li r7, 8
	ctx.r[7].s64 = 8;
	// 826C39C4: 390A4B40  addi r8, r10, 0x4b40
	ctx.r[8].s64 = ctx.r[10].s64 + 19264;
	// 826C39C8: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C39CC: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 826C39D0: 38AA4844  addi r5, r10, 0x4844
	ctx.r[5].s64 = ctx.r[10].s64 + 18500;
	// 826C39D4: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C39D8: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826C39DC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C39E0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826C39E4: 388A3394  addi r4, r10, 0x3394
	ctx.r[4].s64 = ctx.r[10].s64 + 13204;
	// 826C39E8: 396B1DEC  addi r11, r11, 0x1dec
	ctx.r[11].s64 = ctx.r[11].s64 + 7660;
	// 826C39EC: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C39F0: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C39F4: 386A4B74  addi r3, r10, 0x4b74
	ctx.r[3].s64 = ctx.r[10].s64 + 19316;
	// 826C39F8: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 826C39FC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C3A00: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 826C3A04: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C3A08: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C3A0C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C3A10: 4BDA3411  bl 0x82466e20
	ctx.lr = 0x826C3A14;
	sub_82466E20(ctx, base);
	// 826C3A14: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C3A18: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C3A1C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C3A20: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C3A28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C3A28 size=112
    let mut pc: u32 = 0x826C3A28;
    'dispatch: loop {
        match pc {
            0x826C3A28 => {
    //   block [0x826C3A28..0x826C3A98)
	// 826C3A28: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C3A2C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C3A30: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C3A34: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C3A38: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826C3A3C: 38AA4844  addi r5, r10, 0x4844
	ctx.r[5].s64 = ctx.r[10].s64 + 18500;
	// 826C3A40: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C3A44: 390B4C00  addi r8, r11, 0x4c00
	ctx.r[8].s64 = ctx.r[11].s64 + 19456;
	// 826C3A48: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826C3A4C: 388A33B0  addi r4, r10, 0x33b0
	ctx.r[4].s64 = ctx.r[10].s64 + 13232;
	// 826C3A50: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C3A54: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C3A58: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826C3A5C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C3A60: 386A4BA4  addi r3, r10, 0x4ba4
	ctx.r[3].s64 = ctx.r[10].s64 + 19364;
	// 826C3A64: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826C3A68: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C3A6C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C3A70: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C3A74: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C3A78: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C3A7C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C3A80: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C3A84: 4BDA339D  bl 0x82466e20
	ctx.lr = 0x826C3A88;
	sub_82466E20(ctx, base);
	// 826C3A88: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C3A8C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C3A90: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C3A94: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C3A98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C3A98 size=112
    let mut pc: u32 = 0x826C3A98;
    'dispatch: loop {
        match pc {
            0x826C3A98 => {
    //   block [0x826C3A98..0x826C3B08)
	// 826C3A98: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C3A9C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C3AA0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C3AA4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C3AA8: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826C3AAC: 38AA4994  addi r5, r10, 0x4994
	ctx.r[5].s64 = ctx.r[10].s64 + 18836;
	// 826C3AB0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C3AB4: 390B4C18  addi r8, r11, 0x4c18
	ctx.r[8].s64 = ctx.r[11].s64 + 19480;
	// 826C3AB8: 39200006  li r9, 6
	ctx.r[9].s64 = 6;
	// 826C3ABC: 388A33D4  addi r4, r10, 0x33d4
	ctx.r[4].s64 = ctx.r[10].s64 + 13268;
	// 826C3AC0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C3AC4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C3AC8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826C3ACC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C3AD0: 386A4BD4  addi r3, r10, 0x4bd4
	ctx.r[3].s64 = ctx.r[10].s64 + 19412;
	// 826C3AD4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826C3AD8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C3ADC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C3AE0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C3AE4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C3AE8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C3AEC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C3AF0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C3AF4: 4BDA332D  bl 0x82466e20
	ctx.lr = 0x826C3AF8;
	sub_82466E20(ctx, base);
	// 826C3AF8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C3AFC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C3B00: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C3B04: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C3B08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C3B08 size=112
    let mut pc: u32 = 0x826C3B08;
    'dispatch: loop {
        match pc {
            0x826C3B08 => {
    //   block [0x826C3B08..0x826C3B78)
	// 826C3B08: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C3B0C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C3B10: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C3B14: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C3B18: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826C3B1C: 38AA4844  addi r5, r10, 0x4844
	ctx.r[5].s64 = ctx.r[10].s64 + 18500;
	// 826C3B20: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C3B24: 390B4CA8  addi r8, r11, 0x4ca8
	ctx.r[8].s64 = ctx.r[11].s64 + 19624;
	// 826C3B28: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826C3B2C: 388A33E8  addi r4, r10, 0x33e8
	ctx.r[4].s64 = ctx.r[10].s64 + 13288;
	// 826C3B30: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C3B34: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C3B38: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826C3B3C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C3B40: 386A4C04  addi r3, r10, 0x4c04
	ctx.r[3].s64 = ctx.r[10].s64 + 19460;
	// 826C3B44: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826C3B48: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C3B4C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C3B50: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C3B54: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C3B58: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C3B5C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C3B60: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C3B64: 4BDA32BD  bl 0x82466e20
	ctx.lr = 0x826C3B68;
	sub_82466E20(ctx, base);
	// 826C3B68: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C3B6C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C3B70: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C3B74: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C3B78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C3B78 size=116
    let mut pc: u32 = 0x826C3B78;
    'dispatch: loop {
        match pc {
            0x826C3B78 => {
    //   block [0x826C3B78..0x826C3BEC)
	// 826C3B78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C3B7C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C3B80: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C3B84: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826C3B88: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 826C3B8C: 390B4CC0  addi r8, r11, 0x4cc0
	ctx.r[8].s64 = ctx.r[11].s64 + 19648;
	// 826C3B90: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C3B94: 392A1E34  addi r9, r10, 0x1e34
	ctx.r[9].s64 = ctx.r[10].s64 + 7732;
	// 826C3B98: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C3B9C: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
	// 826C3BA0: 38AA4EA4  addi r5, r10, 0x4ea4
	ctx.r[5].s64 = ctx.r[10].s64 + 20132;
	// 826C3BA4: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826C3BA8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C3BAC: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C3BB0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C3BB4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C3BB8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C3BBC: 3D608298  lis r11, -0x7d68
	ctx.r[11].s64 = -2103967744;
	// 826C3BC0: 388A33FC  addi r4, r10, 0x33fc
	ctx.r[4].s64 = ctx.r[10].s64 + 13308;
	// 826C3BC4: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826C3BC8: 386B4C34  addi r3, r11, 0x4c34
	ctx.r[3].s64 = ctx.r[11].s64 + 19508;
	// 826C3BCC: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826C3BD0: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C3BD4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C3BD8: 4BDA3249  bl 0x82466e20
	ctx.lr = 0x826C3BDC;
	sub_82466E20(ctx, base);
	// 826C3BDC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C3BE0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C3BE4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C3BE8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C3BF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C3BF0 size=100
    let mut pc: u32 = 0x826C3BF0;
    'dispatch: loop {
        match pc {
            0x826C3BF0 => {
    //   block [0x826C3BF0..0x826C3C54)
	// 826C3BF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C3BF4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C3BF8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C3BFC: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C3C00: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C3C04: 38AA4EA4  addi r5, r10, 0x4ea4
	ctx.r[5].s64 = ctx.r[10].s64 + 20132;
	// 826C3C08: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C3C0C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C3C10: 388A3404  addi r4, r10, 0x3404
	ctx.r[4].s64 = ctx.r[10].s64 + 13316;
	// 826C3C14: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C3C18: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C3C1C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C3C20: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C3C24: 386A4C64  addi r3, r10, 0x4c64
	ctx.r[3].s64 = ctx.r[10].s64 + 19556;
	// 826C3C28: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C3C2C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C3C30: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826C3C34: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C3C38: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826C3C3C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C3C40: 4BDA31E1  bl 0x82466e20
	ctx.lr = 0x826C3C44;
	sub_82466E20(ctx, base);
	// 826C3C44: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C3C48: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C3C4C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C3C50: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C3C58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C3C58 size=112
    let mut pc: u32 = 0x826C3C58;
    'dispatch: loop {
        match pc {
            0x826C3C58 => {
    //   block [0x826C3C58..0x826C3CC8)
	// 826C3C58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C3C5C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C3C60: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C3C64: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C3C68: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826C3C6C: 38AA4C64  addi r5, r10, 0x4c64
	ctx.r[5].s64 = ctx.r[10].s64 + 19556;
	// 826C3C70: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C3C74: 390B4CD8  addi r8, r11, 0x4cd8
	ctx.r[8].s64 = ctx.r[11].s64 + 19672;
	// 826C3C78: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826C3C7C: 388A3414  addi r4, r10, 0x3414
	ctx.r[4].s64 = ctx.r[10].s64 + 13332;
	// 826C3C80: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C3C84: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C3C88: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826C3C8C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C3C90: 386A4C94  addi r3, r10, 0x4c94
	ctx.r[3].s64 = ctx.r[10].s64 + 19604;
	// 826C3C94: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826C3C98: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C3C9C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C3CA0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C3CA4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C3CA8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C3CAC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C3CB0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C3CB4: 4BDA316D  bl 0x82466e20
	ctx.lr = 0x826C3CB8;
	sub_82466E20(ctx, base);
	// 826C3CB8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C3CBC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C3CC0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C3CC4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C3CC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C3CC8 size=108
    let mut pc: u32 = 0x826C3CC8;
    'dispatch: loop {
        match pc {
            0x826C3CC8 => {
    //   block [0x826C3CC8..0x826C3D34)
	// 826C3CC8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C3CCC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C3CD0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C3CD4: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826C3CD8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C3CDC: 38EB4CF0  addi r7, r11, 0x4cf0
	ctx.r[7].s64 = ctx.r[11].s64 + 19696;
	// 826C3CE0: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 826C3CE4: 388A35CC  addi r4, r10, 0x35cc
	ctx.r[4].s64 = ctx.r[10].s64 + 13772;
	// 826C3CE8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C3CEC: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C3CF0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826C3CF4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C3CF8: 386A4CC4  addi r3, r10, 0x4cc4
	ctx.r[3].s64 = ctx.r[10].s64 + 19652;
	// 826C3CFC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826C3D00: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C3D04: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C3D08: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C3D0C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C3D10: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C3D14: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C3D18: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C3D1C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826C3D20: 4BDA3101  bl 0x82466e20
	ctx.lr = 0x826C3D24;
	sub_82466E20(ctx, base);
	// 826C3D24: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C3D28: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C3D2C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C3D30: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C3D38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C3D38 size=108
    let mut pc: u32 = 0x826C3D38;
    'dispatch: loop {
        match pc {
            0x826C3D38 => {
    //   block [0x826C3D38..0x826C3DA4)
	// 826C3D38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C3D3C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C3D40: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C3D44: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826C3D48: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C3D4C: 38EB4D50  addi r7, r11, 0x4d50
	ctx.r[7].s64 = ctx.r[11].s64 + 19792;
	// 826C3D50: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 826C3D54: 388A3610  addi r4, r10, 0x3610
	ctx.r[4].s64 = ctx.r[10].s64 + 13840;
	// 826C3D58: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C3D5C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C3D60: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826C3D64: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C3D68: 386A4CF4  addi r3, r10, 0x4cf4
	ctx.r[3].s64 = ctx.r[10].s64 + 19700;
	// 826C3D6C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826C3D70: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C3D74: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C3D78: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C3D7C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C3D80: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C3D84: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C3D88: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C3D8C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826C3D90: 4BDA3091  bl 0x82466e20
	ctx.lr = 0x826C3D94;
	sub_82466E20(ctx, base);
	// 826C3D94: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C3D98: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C3D9C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C3DA0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C3DA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C3DA8 size=116
    let mut pc: u32 = 0x826C3DA8;
    'dispatch: loop {
        match pc {
            0x826C3DA8 => {
    //   block [0x826C3DA8..0x826C3E1C)
	// 826C3DA8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C3DAC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C3DB0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C3DB4: 3D40827E  lis r10, -0x7d82
	ctx.r[10].s64 = -2105671680;
	// 826C3DB8: 38E0000C  li r7, 0xc
	ctx.r[7].s64 = 12;
	// 826C3DBC: 390A4D98  addi r8, r10, 0x4d98
	ctx.r[8].s64 = ctx.r[10].s64 + 19864;
	// 826C3DC0: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C3DC4: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 826C3DC8: 38AA4694  addi r5, r10, 0x4694
	ctx.r[5].s64 = ctx.r[10].s64 + 18068;
	// 826C3DCC: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C3DD0: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826C3DD4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C3DD8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826C3DDC: 388A362C  addi r4, r10, 0x362c
	ctx.r[4].s64 = ctx.r[10].s64 + 13868;
	// 826C3DE0: 396B1E48  addi r11, r11, 0x1e48
	ctx.r[11].s64 = ctx.r[11].s64 + 7752;
	// 826C3DE4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C3DE8: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C3DEC: 386A4D24  addi r3, r10, 0x4d24
	ctx.r[3].s64 = ctx.r[10].s64 + 19748;
	// 826C3DF0: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 826C3DF4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C3DF8: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 826C3DFC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C3E00: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C3E04: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C3E08: 4BDA3019  bl 0x82466e20
	ctx.lr = 0x826C3E0C;
	sub_82466E20(ctx, base);
	// 826C3E0C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C3E10: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C3E14: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C3E18: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C3E20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C3E20 size=108
    let mut pc: u32 = 0x826C3E20;
    'dispatch: loop {
        match pc {
            0x826C3E20 => {
    //   block [0x826C3E20..0x826C3E8C)
	// 826C3E20: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C3E24: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C3E28: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C3E2C: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826C3E30: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826C3E34: 38EB4EB8  addi r7, r11, 0x4eb8
	ctx.r[7].s64 = ctx.r[11].s64 + 20152;
	// 826C3E38: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 826C3E3C: 388A251C  addi r4, r10, 0x251c
	ctx.r[4].s64 = ctx.r[10].s64 + 9500;
	// 826C3E40: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C3E44: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C3E48: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826C3E4C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C3E50: 386A4D54  addi r3, r10, 0x4d54
	ctx.r[3].s64 = ctx.r[10].s64 + 19796;
	// 826C3E54: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826C3E58: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C3E5C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C3E60: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C3E64: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C3E68: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C3E6C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C3E70: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C3E74: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826C3E78: 4BDA2FA9  bl 0x82466e20
	ctx.lr = 0x826C3E7C;
	sub_82466E20(ctx, base);
	// 826C3E7C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C3E80: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C3E84: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C3E88: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C3E90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C3E90 size=104
    let mut pc: u32 = 0x826C3E90;
    'dispatch: loop {
        match pc {
            0x826C3E90 => {
    //   block [0x826C3E90..0x826C3EF8)
	// 826C3E90: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C3E94: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C3E98: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C3E9C: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 826C3EA0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C3EA4: 392A1E9C  addi r9, r10, 0x1e9c
	ctx.r[9].s64 = ctx.r[10].s64 + 7836;
	// 826C3EA8: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C3EAC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C3EB0: 38AA4EA4  addi r5, r10, 0x4ea4
	ctx.r[5].s64 = ctx.r[10].s64 + 20132;
	// 826C3EB4: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C3EB8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C3EBC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C3EC0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C3EC4: 388A363C  addi r4, r10, 0x363c
	ctx.r[4].s64 = ctx.r[10].s64 + 13884;
	// 826C3EC8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C3ECC: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C3ED0: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826C3ED4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C3ED8: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826C3EDC: 386A4D84  addi r3, r10, 0x4d84
	ctx.r[3].s64 = ctx.r[10].s64 + 19844;
	// 826C3EE0: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826C3EE4: 4BDA2F3D  bl 0x82466e20
	ctx.lr = 0x826C3EE8;
	sub_82466E20(ctx, base);
	// 826C3EE8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C3EEC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C3EF0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C3EF4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C3EF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826C3EF8 size=24
    let mut pc: u32 = 0x826C3EF8;
    'dispatch: loop {
        match pc {
            0x826C3EF8 => {
    //   block [0x826C3EF8..0x826C3F10)
	// 826C3EF8: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826C3EFC: 3D40827F  lis r10, -0x7d81
	ctx.r[10].s64 = -2105606144;
	// 826C3F00: 394A9F08  addi r10, r10, -0x60f8
	ctx.r[10].s64 = ctx.r[10].s64 + -24824;
	// 826C3F04: 816B4F08  lwz r11, 0x4f08(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20232 as u32) ) } as u64;
	// 826C3F08: 916A0050  stw r11, 0x50(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 826C3F0C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C3F10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C3F10 size=116
    let mut pc: u32 = 0x826C3F10;
    'dispatch: loop {
        match pc {
            0x826C3F10 => {
    //   block [0x826C3F10..0x826C3F84)
	// 826C3F10: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C3F14: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C3F18: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C3F1C: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 826C3F20: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C3F24: 392B1EE0  addi r9, r11, 0x1ee0
	ctx.r[9].s64 = ctx.r[11].s64 + 7904;
	// 826C3F28: 38AA4D84  addi r5, r10, 0x4d84
	ctx.r[5].s64 = ctx.r[10].s64 + 19844;
	// 826C3F2C: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C3F30: 38E90028  addi r7, r9, 0x28
	ctx.r[7].s64 = ctx.r[9].s64 + 40;
	// 826C3F34: 38C00008  li r6, 8
	ctx.r[6].s64 = 8;
	// 826C3F38: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826C3F3C: 388A3690  addi r4, r10, 0x3690
	ctx.r[4].s64 = ctx.r[10].s64 + 13968;
	// 826C3F40: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C3F44: 396B9F08  addi r11, r11, -0x60f8
	ctx.r[11].s64 = ctx.r[11].s64 + -24824;
	// 826C3F48: 90E10064  stw r7, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[7].u32 ) };
	// 826C3F4C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C3F50: 90C1005C  stw r6, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[6].u32 ) };
	// 826C3F54: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C3F58: 386A4DB4  addi r3, r10, 0x4db4
	ctx.r[3].s64 = ctx.r[10].s64 + 19892;
	// 826C3F5C: 39400002  li r10, 2
	ctx.r[10].s64 = 2;
	// 826C3F60: 91010074  stw r8, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[8].u32 ) };
	// 826C3F64: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C3F68: 9101006C  stw r8, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[8].u32 ) };
	// 826C3F6C: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826C3F70: 4BDA2EB1  bl 0x82466e20
	ctx.lr = 0x826C3F74;
	sub_82466E20(ctx, base);
	// 826C3F74: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C3F78: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C3F7C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C3F80: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C3F88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C3F88 size=112
    let mut pc: u32 = 0x826C3F88;
    'dispatch: loop {
        match pc {
            0x826C3F88 => {
    //   block [0x826C3F88..0x826C3FF8)
	// 826C3F88: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C3F8C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C3F90: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C3F94: 3D40827E  lis r10, -0x7d82
	ctx.r[10].s64 = -2105671680;
	// 826C3F98: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 826C3F9C: 38EA4F10  addi r7, r10, 0x4f10
	ctx.r[7].s64 = ctx.r[10].s64 + 20240;
	// 826C3FA0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C3FA4: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 826C3FA8: 388A3770  addi r4, r10, 0x3770
	ctx.r[4].s64 = ctx.r[10].s64 + 14192;
	// 826C3FAC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C3FB0: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826C3FB4: 396B1F68  addi r11, r11, 0x1f68
	ctx.r[11].s64 = ctx.r[11].s64 + 8040;
	// 826C3FB8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826C3FBC: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C3FC0: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C3FC4: 386A4DE4  addi r3, r10, 0x4de4
	ctx.r[3].s64 = ctx.r[10].s64 + 19940;
	// 826C3FC8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C3FCC: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 826C3FD0: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C3FD4: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 826C3FD8: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C3FDC: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C3FE0: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826C3FE4: 4BDA2E3D  bl 0x82466e20
	ctx.lr = 0x826C3FE8;
	sub_82466E20(ctx, base);
	// 826C3FE8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C3FEC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C3FF0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C3FF4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C3FF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C3FF8 size=108
    let mut pc: u32 = 0x826C3FF8;
    'dispatch: loop {
        match pc {
            0x826C3FF8 => {
    //   block [0x826C3FF8..0x826C4064)
	// 826C3FF8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C3FFC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C4000: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C4004: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826C4008: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C400C: 38EB4F70  addi r7, r11, 0x4f70
	ctx.r[7].s64 = ctx.r[11].s64 + 20336;
	// 826C4010: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826C4014: 388A3788  addi r4, r10, 0x3788
	ctx.r[4].s64 = ctx.r[10].s64 + 14216;
	// 826C4018: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C401C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C4020: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826C4024: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C4028: 386A4E14  addi r3, r10, 0x4e14
	ctx.r[3].s64 = ctx.r[10].s64 + 19988;
	// 826C402C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826C4030: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C4034: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C4038: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C403C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C4040: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C4044: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C4048: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C404C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826C4050: 4BDA2DD1  bl 0x82466e20
	ctx.lr = 0x826C4054;
	sub_82466E20(ctx, base);
	// 826C4054: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C4058: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C405C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C4060: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C4068(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C4068 size=116
    let mut pc: u32 = 0x826C4068;
    'dispatch: loop {
        match pc {
            0x826C4068 => {
    //   block [0x826C4068..0x826C40DC)
	// 826C4068: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C406C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C4070: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C4074: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826C4078: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 826C407C: 390B4FA0  addi r8, r11, 0x4fa0
	ctx.r[8].s64 = ctx.r[11].s64 + 20384;
	// 826C4080: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C4084: 392A1F54  addi r9, r10, 0x1f54
	ctx.r[9].s64 = ctx.r[10].s64 + 8020;
	// 826C4088: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C408C: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
	// 826C4090: 38AA4EA4  addi r5, r10, 0x4ea4
	ctx.r[5].s64 = ctx.r[10].s64 + 20132;
	// 826C4094: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826C4098: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C409C: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C40A0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C40A4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C40A8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C40AC: 3D608298  lis r11, -0x7d68
	ctx.r[11].s64 = -2103967744;
	// 826C40B0: 388A37A0  addi r4, r10, 0x37a0
	ctx.r[4].s64 = ctx.r[10].s64 + 14240;
	// 826C40B4: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826C40B8: 386B4E44  addi r3, r11, 0x4e44
	ctx.r[3].s64 = ctx.r[11].s64 + 20036;
	// 826C40BC: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826C40C0: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C40C4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C40C8: 4BDA2D59  bl 0x82466e20
	ctx.lr = 0x826C40CC;
	sub_82466E20(ctx, base);
	// 826C40CC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C40D0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C40D4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C40D8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C40E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C40E0 size=96
    let mut pc: u32 = 0x826C40E0;
    'dispatch: loop {
        match pc {
            0x826C40E0 => {
    //   block [0x826C40E0..0x826C4140)
	// 826C40E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C40E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C40E8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C40EC: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 826C40F0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C40F4: 388A6DD4  addi r4, r10, 0x6dd4
	ctx.r[4].s64 = ctx.r[10].s64 + 28116;
	// 826C40F8: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C40FC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C4100: 386A4E74  addi r3, r10, 0x4e74
	ctx.r[3].s64 = ctx.r[10].s64 + 20084;
	// 826C4104: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C4108: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C410C: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826C4110: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C4114: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C4118: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C411C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C4120: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826C4124: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826C4128: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826C412C: 4BDA2CF5  bl 0x82466e20
	ctx.lr = 0x826C4130;
	sub_82466E20(ctx, base);
	// 826C4130: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C4134: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C4138: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C413C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C4140(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C4140 size=112
    let mut pc: u32 = 0x826C4140;
    'dispatch: loop {
        match pc {
            0x826C4140 => {
    //   block [0x826C4140..0x826C41B0)
	// 826C4140: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C4144: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C4148: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C414C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C4150: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826C4154: 38AA4E74  addi r5, r10, 0x4e74
	ctx.r[5].s64 = ctx.r[10].s64 + 20084;
	// 826C4158: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 826C415C: 390B4FB8  addi r8, r11, 0x4fb8
	ctx.r[8].s64 = ctx.r[11].s64 + 20408;
	// 826C4160: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826C4164: 388A7380  addi r4, r10, 0x7380
	ctx.r[4].s64 = ctx.r[10].s64 + 29568;
	// 826C4168: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C416C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C4170: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826C4174: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C4178: 386A4EA4  addi r3, r10, 0x4ea4
	ctx.r[3].s64 = ctx.r[10].s64 + 20132;
	// 826C417C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826C4180: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C4184: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C4188: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C418C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C4190: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C4194: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C4198: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C419C: 4BDA2C85  bl 0x82466e20
	ctx.lr = 0x826C41A0;
	sub_82466E20(ctx, base);
	// 826C41A0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C41A4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C41A8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C41AC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C41B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C41B0 size=112
    let mut pc: u32 = 0x826C41B0;
    'dispatch: loop {
        match pc {
            0x826C41B0 => {
    //   block [0x826C41B0..0x826C4220)
	// 826C41B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C41B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C41B8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C41BC: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 826C41C0: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826C41C4: 392A1F84  addi r9, r10, 0x1f84
	ctx.r[9].s64 = ctx.r[10].s64 + 8068;
	// 826C41C8: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 826C41CC: 390B4FF0  addi r8, r11, 0x4ff0
	ctx.r[8].s64 = ctx.r[11].s64 + 20464;
	// 826C41D0: 38E00007  li r7, 7
	ctx.r[7].s64 = 7;
	// 826C41D4: 388A7468  addi r4, r10, 0x7468
	ctx.r[4].s64 = ctx.r[10].s64 + 29800;
	// 826C41D8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C41DC: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C41E0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826C41E4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C41E8: 386A4ED4  addi r3, r10, 0x4ed4
	ctx.r[3].s64 = ctx.r[10].s64 + 20180;
	// 826C41EC: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826C41F0: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826C41F4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C41F8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C41FC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C4200: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C4204: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826C4208: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C420C: 4BDA2C15  bl 0x82466e20
	ctx.lr = 0x826C4210;
	sub_82466E20(ctx, base);
	// 826C4210: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C4214: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C4218: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C421C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C4220(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C4220 size=108
    let mut pc: u32 = 0x826C4220;
    'dispatch: loop {
        match pc {
            0x826C4220 => {
    //   block [0x826C4220..0x826C428C)
	// 826C4220: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C4224: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C4228: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C422C: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826C4230: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 826C4234: 38EB5098  addi r7, r11, 0x5098
	ctx.r[7].s64 = ctx.r[11].s64 + 20632;
	// 826C4238: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826C423C: 388A7888  addi r4, r10, 0x7888
	ctx.r[4].s64 = ctx.r[10].s64 + 30856;
	// 826C4240: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C4244: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C4248: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826C424C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C4250: 386A4F04  addi r3, r10, 0x4f04
	ctx.r[3].s64 = ctx.r[10].s64 + 20228;
	// 826C4254: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826C4258: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C425C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C4260: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C4264: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C4268: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C426C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C4270: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C4274: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826C4278: 4BDA2BA9  bl 0x82466e20
	ctx.lr = 0x826C427C;
	sub_82466E20(ctx, base);
	// 826C427C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C4280: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C4284: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C4288: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C4290(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C4290 size=108
    let mut pc: u32 = 0x826C4290;
    'dispatch: loop {
        match pc {
            0x826C4290 => {
    //   block [0x826C4290..0x826C42FC)
	// 826C4290: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C4294: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C4298: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C429C: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826C42A0: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 826C42A4: 38EB50C8  addi r7, r11, 0x50c8
	ctx.r[7].s64 = ctx.r[11].s64 + 20680;
	// 826C42A8: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826C42AC: 388A7898  addi r4, r10, 0x7898
	ctx.r[4].s64 = ctx.r[10].s64 + 30872;
	// 826C42B0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C42B4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C42B8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826C42BC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C42C0: 386A4F34  addi r3, r10, 0x4f34
	ctx.r[3].s64 = ctx.r[10].s64 + 20276;
	// 826C42C4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826C42C8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C42CC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C42D0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C42D4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C42D8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C42DC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C42E0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C42E4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826C42E8: 4BDA2B39  bl 0x82466e20
	ctx.lr = 0x826C42EC;
	sub_82466E20(ctx, base);
	// 826C42EC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C42F0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C42F4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C42F8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


