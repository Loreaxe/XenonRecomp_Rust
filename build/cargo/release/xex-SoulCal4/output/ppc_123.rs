pub fn sub_826CAA38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826CAA38 size=108
    let mut pc: u32 = 0x826CAA38;
    'dispatch: loop {
        match pc {
            0x826CAA38 => {
    //   block [0x826CAA38..0x826CAAA4)
	// 826CAA38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826CAA3C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826CAA40: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826CAA44: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826CAA48: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826CAA4C: 38EB8B70  addi r7, r11, -0x7490
	ctx.r[7].s64 = ctx.r[11].s64 + -29840;
	// 826CAA50: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826CAA54: 388A52A4  addi r4, r10, 0x52a4
	ctx.r[4].s64 = ctx.r[10].s64 + 21156;
	// 826CAA58: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826CAA5C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826CAA60: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826CAA64: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826CAA68: 386A7B14  addi r3, r10, 0x7b14
	ctx.r[3].s64 = ctx.r[10].s64 + 31508;
	// 826CAA6C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826CAA70: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826CAA74: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826CAA78: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826CAA7C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826CAA80: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826CAA84: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826CAA88: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826CAA8C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826CAA90: 4BD9C391  bl 0x82466e20
	ctx.lr = 0x826CAA94;
	sub_82466E20(ctx, base);
	// 826CAA94: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826CAA98: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826CAA9C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826CAAA0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826CAAA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826CAAA8 size=112
    let mut pc: u32 = 0x826CAAA8;
    'dispatch: loop {
        match pc {
            0x826CAAA8 => {
    //   block [0x826CAAA8..0x826CAB18)
	// 826CAAA8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826CAAAC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826CAAB0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826CAAB4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826CAAB8: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826CAABC: 38AA4EA4  addi r5, r10, 0x4ea4
	ctx.r[5].s64 = ctx.r[10].s64 + 20132;
	// 826CAAC0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826CAAC4: 390B8BA0  addi r8, r11, -0x7460
	ctx.r[8].s64 = ctx.r[11].s64 + -29792;
	// 826CAAC8: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826CAACC: 388A52BC  addi r4, r10, 0x52bc
	ctx.r[4].s64 = ctx.r[10].s64 + 21180;
	// 826CAAD0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826CAAD4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826CAAD8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826CAADC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826CAAE0: 386A7B44  addi r3, r10, 0x7b44
	ctx.r[3].s64 = ctx.r[10].s64 + 31556;
	// 826CAAE4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826CAAE8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826CAAEC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826CAAF0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826CAAF4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826CAAF8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826CAAFC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826CAB00: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826CAB04: 4BD9C31D  bl 0x82466e20
	ctx.lr = 0x826CAB08;
	sub_82466E20(ctx, base);
	// 826CAB08: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826CAB0C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826CAB10: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826CAB14: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826CAB18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826CAB18 size=112
    let mut pc: u32 = 0x826CAB18;
    'dispatch: loop {
        match pc {
            0x826CAB18 => {
    //   block [0x826CAB18..0x826CAB88)
	// 826CAB18: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826CAB1C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826CAB20: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826CAB24: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826CAB28: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826CAB2C: 38AA4EA4  addi r5, r10, 0x4ea4
	ctx.r[5].s64 = ctx.r[10].s64 + 20132;
	// 826CAB30: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826CAB34: 390B8BD0  addi r8, r11, -0x7430
	ctx.r[8].s64 = ctx.r[11].s64 + -29744;
	// 826CAB38: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826CAB3C: 388A52CC  addi r4, r10, 0x52cc
	ctx.r[4].s64 = ctx.r[10].s64 + 21196;
	// 826CAB40: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826CAB44: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826CAB48: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826CAB4C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826CAB50: 386A7B74  addi r3, r10, 0x7b74
	ctx.r[3].s64 = ctx.r[10].s64 + 31604;
	// 826CAB54: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826CAB58: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826CAB5C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826CAB60: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826CAB64: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826CAB68: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826CAB6C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826CAB70: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826CAB74: 4BD9C2AD  bl 0x82466e20
	ctx.lr = 0x826CAB78;
	sub_82466E20(ctx, base);
	// 826CAB78: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826CAB7C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826CAB80: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826CAB84: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826CAB88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826CAB88 size=112
    let mut pc: u32 = 0x826CAB88;
    'dispatch: loop {
        match pc {
            0x826CAB88 => {
    //   block [0x826CAB88..0x826CABF8)
	// 826CAB88: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826CAB8C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826CAB90: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826CAB94: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826CAB98: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826CAB9C: 38AA4EA4  addi r5, r10, 0x4ea4
	ctx.r[5].s64 = ctx.r[10].s64 + 20132;
	// 826CABA0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826CABA4: 390B8BE8  addi r8, r11, -0x7418
	ctx.r[8].s64 = ctx.r[11].s64 + -29720;
	// 826CABA8: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826CABAC: 388A52E8  addi r4, r10, 0x52e8
	ctx.r[4].s64 = ctx.r[10].s64 + 21224;
	// 826CABB0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826CABB4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826CABB8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826CABBC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826CABC0: 386A7BA4  addi r3, r10, 0x7ba4
	ctx.r[3].s64 = ctx.r[10].s64 + 31652;
	// 826CABC4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826CABC8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826CABCC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826CABD0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826CABD4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826CABD8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826CABDC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826CABE0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826CABE4: 4BD9C23D  bl 0x82466e20
	ctx.lr = 0x826CABE8;
	sub_82466E20(ctx, base);
	// 826CABE8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826CABEC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826CABF0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826CABF4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826CABF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826CABF8 size=108
    let mut pc: u32 = 0x826CABF8;
    'dispatch: loop {
        match pc {
            0x826CABF8 => {
    //   block [0x826CABF8..0x826CAC64)
	// 826CABF8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826CABFC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826CAC00: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826CAC04: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826CAC08: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826CAC0C: 38EB8C00  addi r7, r11, -0x7400
	ctx.r[7].s64 = ctx.r[11].s64 + -29696;
	// 826CAC10: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826CAC14: 388A5308  addi r4, r10, 0x5308
	ctx.r[4].s64 = ctx.r[10].s64 + 21256;
	// 826CAC18: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826CAC1C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826CAC20: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826CAC24: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826CAC28: 386A7BD4  addi r3, r10, 0x7bd4
	ctx.r[3].s64 = ctx.r[10].s64 + 31700;
	// 826CAC2C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826CAC30: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826CAC34: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826CAC38: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826CAC3C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826CAC40: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826CAC44: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826CAC48: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826CAC4C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826CAC50: 4BD9C1D1  bl 0x82466e20
	ctx.lr = 0x826CAC54;
	sub_82466E20(ctx, base);
	// 826CAC54: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826CAC58: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826CAC5C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826CAC60: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826CAC68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826CAC68 size=112
    let mut pc: u32 = 0x826CAC68;
    'dispatch: loop {
        match pc {
            0x826CAC68 => {
    //   block [0x826CAC68..0x826CACD8)
	// 826CAC68: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826CAC6C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826CAC70: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826CAC74: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826CAC78: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826CAC7C: 38AA4EA4  addi r5, r10, 0x4ea4
	ctx.r[5].s64 = ctx.r[10].s64 + 20132;
	// 826CAC80: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826CAC84: 390B8C30  addi r8, r11, -0x73d0
	ctx.r[8].s64 = ctx.r[11].s64 + -29648;
	// 826CAC88: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826CAC8C: 388A533C  addi r4, r10, 0x533c
	ctx.r[4].s64 = ctx.r[10].s64 + 21308;
	// 826CAC90: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826CAC94: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826CAC98: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826CAC9C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826CACA0: 386A7C04  addi r3, r10, 0x7c04
	ctx.r[3].s64 = ctx.r[10].s64 + 31748;
	// 826CACA4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826CACA8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826CACAC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826CACB0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826CACB4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826CACB8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826CACBC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826CACC0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826CACC4: 4BD9C15D  bl 0x82466e20
	ctx.lr = 0x826CACC8;
	sub_82466E20(ctx, base);
	// 826CACC8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826CACCC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826CACD0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826CACD4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826CACD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826CACD8 size=108
    let mut pc: u32 = 0x826CACD8;
    'dispatch: loop {
        match pc {
            0x826CACD8 => {
    //   block [0x826CACD8..0x826CAD44)
	// 826CACD8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826CACDC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826CACE0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826CACE4: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826CACE8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826CACEC: 38EB8C48  addi r7, r11, -0x73b8
	ctx.r[7].s64 = ctx.r[11].s64 + -29624;
	// 826CACF0: 3900000A  li r8, 0xa
	ctx.r[8].s64 = 10;
	// 826CACF4: 388A535C  addi r4, r10, 0x535c
	ctx.r[4].s64 = ctx.r[10].s64 + 21340;
	// 826CACF8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826CACFC: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826CAD00: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826CAD04: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826CAD08: 386A7C34  addi r3, r10, 0x7c34
	ctx.r[3].s64 = ctx.r[10].s64 + 31796;
	// 826CAD0C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826CAD10: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826CAD14: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826CAD18: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826CAD1C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826CAD20: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826CAD24: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826CAD28: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826CAD2C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826CAD30: 4BD9C0F1  bl 0x82466e20
	ctx.lr = 0x826CAD34;
	sub_82466E20(ctx, base);
	// 826CAD34: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826CAD38: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826CAD3C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826CAD40: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826CAD48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826CAD48 size=112
    let mut pc: u32 = 0x826CAD48;
    'dispatch: loop {
        match pc {
            0x826CAD48 => {
    //   block [0x826CAD48..0x826CADB8)
	// 826CAD48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826CAD4C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826CAD50: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826CAD54: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826CAD58: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826CAD5C: 38AA4EA4  addi r5, r10, 0x4ea4
	ctx.r[5].s64 = ctx.r[10].s64 + 20132;
	// 826CAD60: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826CAD64: 390B8D38  addi r8, r11, -0x72c8
	ctx.r[8].s64 = ctx.r[11].s64 + -29384;
	// 826CAD68: 39200012  li r9, 0x12
	ctx.r[9].s64 = 18;
	// 826CAD6C: 388A5380  addi r4, r10, 0x5380
	ctx.r[4].s64 = ctx.r[10].s64 + 21376;
	// 826CAD70: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826CAD74: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826CAD78: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826CAD7C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826CAD80: 386A7C64  addi r3, r10, 0x7c64
	ctx.r[3].s64 = ctx.r[10].s64 + 31844;
	// 826CAD84: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826CAD88: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826CAD8C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826CAD90: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826CAD94: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826CAD98: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826CAD9C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826CADA0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826CADA4: 4BD9C07D  bl 0x82466e20
	ctx.lr = 0x826CADA8;
	sub_82466E20(ctx, base);
	// 826CADA8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826CADAC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826CADB0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826CADB4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826CADB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826CADB8 size=108
    let mut pc: u32 = 0x826CADB8;
    'dispatch: loop {
        match pc {
            0x826CADB8 => {
    //   block [0x826CADB8..0x826CAE24)
	// 826CADB8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826CADBC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826CADC0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826CADC4: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826CADC8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826CADCC: 38EB8EE8  addi r7, r11, -0x7118
	ctx.r[7].s64 = ctx.r[11].s64 + -28952;
	// 826CADD0: 39000011  li r8, 0x11
	ctx.r[8].s64 = 17;
	// 826CADD4: 388A5390  addi r4, r10, 0x5390
	ctx.r[4].s64 = ctx.r[10].s64 + 21392;
	// 826CADD8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826CADDC: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826CADE0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826CADE4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826CADE8: 386A7C94  addi r3, r10, 0x7c94
	ctx.r[3].s64 = ctx.r[10].s64 + 31892;
	// 826CADEC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826CADF0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826CADF4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826CADF8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826CADFC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826CAE00: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826CAE04: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826CAE08: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826CAE0C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826CAE10: 4BD9C011  bl 0x82466e20
	ctx.lr = 0x826CAE14;
	sub_82466E20(ctx, base);
	// 826CAE14: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826CAE18: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826CAE1C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826CAE20: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826CAE28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826CAE28 size=112
    let mut pc: u32 = 0x826CAE28;
    'dispatch: loop {
        match pc {
            0x826CAE28 => {
    //   block [0x826CAE28..0x826CAE98)
	// 826CAE28: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826CAE2C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826CAE30: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826CAE34: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826CAE38: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826CAE3C: 38AA68E4  addi r5, r10, 0x68e4
	ctx.r[5].s64 = ctx.r[10].s64 + 26852;
	// 826CAE40: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826CAE44: 390B9080  addi r8, r11, -0x6f80
	ctx.r[8].s64 = ctx.r[11].s64 + -28544;
	// 826CAE48: 39200019  li r9, 0x19
	ctx.r[9].s64 = 25;
	// 826CAE4C: 388A53AC  addi r4, r10, 0x53ac
	ctx.r[4].s64 = ctx.r[10].s64 + 21420;
	// 826CAE50: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826CAE54: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826CAE58: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826CAE5C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826CAE60: 386A7CC4  addi r3, r10, 0x7cc4
	ctx.r[3].s64 = ctx.r[10].s64 + 31940;
	// 826CAE64: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826CAE68: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826CAE6C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826CAE70: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826CAE74: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826CAE78: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826CAE7C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826CAE80: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826CAE84: 4BD9BF9D  bl 0x82466e20
	ctx.lr = 0x826CAE88;
	sub_82466E20(ctx, base);
	// 826CAE88: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826CAE8C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826CAE90: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826CAE94: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826CAE98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826CAE98 size=100
    let mut pc: u32 = 0x826CAE98;
    'dispatch: loop {
        match pc {
            0x826CAE98 => {
    //   block [0x826CAE98..0x826CAEFC)
	// 826CAE98: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826CAE9C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826CAEA0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826CAEA4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826CAEA8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826CAEAC: 38AA4EA4  addi r5, r10, 0x4ea4
	ctx.r[5].s64 = ctx.r[10].s64 + 20132;
	// 826CAEB0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826CAEB4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826CAEB8: 388A53C0  addi r4, r10, 0x53c0
	ctx.r[4].s64 = ctx.r[10].s64 + 21440;
	// 826CAEBC: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826CAEC0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826CAEC4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826CAEC8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826CAECC: 386A7CF4  addi r3, r10, 0x7cf4
	ctx.r[3].s64 = ctx.r[10].s64 + 31988;
	// 826CAED0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826CAED4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826CAED8: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826CAEDC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826CAEE0: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826CAEE4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826CAEE8: 4BD9BF39  bl 0x82466e20
	ctx.lr = 0x826CAEEC;
	sub_82466E20(ctx, base);
	// 826CAEEC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826CAEF0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826CAEF4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826CAEF8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826CAF00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826CAF00 size=112
    let mut pc: u32 = 0x826CAF00;
    'dispatch: loop {
        match pc {
            0x826CAF00 => {
    //   block [0x826CAF00..0x826CAF70)
	// 826CAF00: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826CAF04: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826CAF08: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826CAF0C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826CAF10: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826CAF14: 38AA7CF4  addi r5, r10, 0x7cf4
	ctx.r[5].s64 = ctx.r[10].s64 + 31988;
	// 826CAF18: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826CAF1C: 390B92D8  addi r8, r11, -0x6d28
	ctx.r[8].s64 = ctx.r[11].s64 + -27944;
	// 826CAF20: 39200005  li r9, 5
	ctx.r[9].s64 = 5;
	// 826CAF24: 388A53D8  addi r4, r10, 0x53d8
	ctx.r[4].s64 = ctx.r[10].s64 + 21464;
	// 826CAF28: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826CAF2C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826CAF30: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826CAF34: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826CAF38: 386A7D24  addi r3, r10, 0x7d24
	ctx.r[3].s64 = ctx.r[10].s64 + 32036;
	// 826CAF3C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826CAF40: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826CAF44: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826CAF48: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826CAF4C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826CAF50: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826CAF54: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826CAF58: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826CAF5C: 4BD9BEC5  bl 0x82466e20
	ctx.lr = 0x826CAF60;
	sub_82466E20(ctx, base);
	// 826CAF60: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826CAF64: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826CAF68: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826CAF6C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826CAF70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826CAF70 size=100
    let mut pc: u32 = 0x826CAF70;
    'dispatch: loop {
        match pc {
            0x826CAF70 => {
    //   block [0x826CAF70..0x826CAFD4)
	// 826CAF70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826CAF74: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826CAF78: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826CAF7C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826CAF80: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826CAF84: 38AA4EA4  addi r5, r10, 0x4ea4
	ctx.r[5].s64 = ctx.r[10].s64 + 20132;
	// 826CAF88: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826CAF8C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826CAF90: 388A53F8  addi r4, r10, 0x53f8
	ctx.r[4].s64 = ctx.r[10].s64 + 21496;
	// 826CAF94: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826CAF98: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826CAF9C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826CAFA0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826CAFA4: 386A7D54  addi r3, r10, 0x7d54
	ctx.r[3].s64 = ctx.r[10].s64 + 32084;
	// 826CAFA8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826CAFAC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826CAFB0: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826CAFB4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826CAFB8: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826CAFBC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826CAFC0: 4BD9BE61  bl 0x82466e20
	ctx.lr = 0x826CAFC4;
	sub_82466E20(ctx, base);
	// 826CAFC4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826CAFC8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826CAFCC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826CAFD0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826CAFD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826CAFD8 size=108
    let mut pc: u32 = 0x826CAFD8;
    'dispatch: loop {
        match pc {
            0x826CAFD8 => {
    //   block [0x826CAFD8..0x826CB044)
	// 826CAFD8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826CAFDC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826CAFE0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826CAFE4: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826CAFE8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826CAFEC: 38EB9350  addi r7, r11, -0x6cb0
	ctx.r[7].s64 = ctx.r[11].s64 + -27824;
	// 826CAFF0: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 826CAFF4: 388A5408  addi r4, r10, 0x5408
	ctx.r[4].s64 = ctx.r[10].s64 + 21512;
	// 826CAFF8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826CAFFC: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826CB000: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826CB004: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826CB008: 386A7D84  addi r3, r10, 0x7d84
	ctx.r[3].s64 = ctx.r[10].s64 + 32132;
	// 826CB00C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826CB010: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826CB014: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826CB018: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826CB01C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826CB020: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826CB024: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826CB028: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826CB02C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826CB030: 4BD9BDF1  bl 0x82466e20
	ctx.lr = 0x826CB034;
	sub_82466E20(ctx, base);
	// 826CB034: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826CB038: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826CB03C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826CB040: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826CB048(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826CB048 size=112
    let mut pc: u32 = 0x826CB048;
    'dispatch: loop {
        match pc {
            0x826CB048 => {
    //   block [0x826CB048..0x826CB0B8)
	// 826CB048: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826CB04C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826CB050: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826CB054: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826CB058: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826CB05C: 38AA7D54  addi r5, r10, 0x7d54
	ctx.r[5].s64 = ctx.r[10].s64 + 32084;
	// 826CB060: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826CB064: 390B9398  addi r8, r11, -0x6c68
	ctx.r[8].s64 = ctx.r[11].s64 + -27752;
	// 826CB068: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826CB06C: 388A5434  addi r4, r10, 0x5434
	ctx.r[4].s64 = ctx.r[10].s64 + 21556;
	// 826CB070: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826CB074: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826CB078: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826CB07C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826CB080: 386A7DB4  addi r3, r10, 0x7db4
	ctx.r[3].s64 = ctx.r[10].s64 + 32180;
	// 826CB084: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826CB088: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826CB08C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826CB090: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826CB094: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826CB098: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826CB09C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826CB0A0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826CB0A4: 4BD9BD7D  bl 0x82466e20
	ctx.lr = 0x826CB0A8;
	sub_82466E20(ctx, base);
	// 826CB0A8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826CB0AC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826CB0B0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826CB0B4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826CB0B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826CB0B8 size=100
    let mut pc: u32 = 0x826CB0B8;
    'dispatch: loop {
        match pc {
            0x826CB0B8 => {
    //   block [0x826CB0B8..0x826CB11C)
	// 826CB0B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826CB0BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826CB0C0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826CB0C4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826CB0C8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826CB0CC: 38AA4EA4  addi r5, r10, 0x4ea4
	ctx.r[5].s64 = ctx.r[10].s64 + 20132;
	// 826CB0D0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826CB0D4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826CB0D8: 388A544C  addi r4, r10, 0x544c
	ctx.r[4].s64 = ctx.r[10].s64 + 21580;
	// 826CB0DC: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826CB0E0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826CB0E4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826CB0E8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826CB0EC: 386A7DE4  addi r3, r10, 0x7de4
	ctx.r[3].s64 = ctx.r[10].s64 + 32228;
	// 826CB0F0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826CB0F4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826CB0F8: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826CB0FC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826CB100: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826CB104: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826CB108: 4BD9BD19  bl 0x82466e20
	ctx.lr = 0x826CB10C;
	sub_82466E20(ctx, base);
	// 826CB10C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826CB110: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826CB114: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826CB118: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826CB120(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826CB120 size=100
    let mut pc: u32 = 0x826CB120;
    'dispatch: loop {
        match pc {
            0x826CB120 => {
    //   block [0x826CB120..0x826CB184)
	// 826CB120: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826CB124: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826CB128: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826CB12C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826CB130: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826CB134: 38AA4EA4  addi r5, r10, 0x4ea4
	ctx.r[5].s64 = ctx.r[10].s64 + 20132;
	// 826CB138: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826CB13C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826CB140: 388A5468  addi r4, r10, 0x5468
	ctx.r[4].s64 = ctx.r[10].s64 + 21608;
	// 826CB144: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826CB148: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826CB14C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826CB150: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826CB154: 386A7E14  addi r3, r10, 0x7e14
	ctx.r[3].s64 = ctx.r[10].s64 + 32276;
	// 826CB158: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826CB15C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826CB160: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826CB164: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826CB168: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826CB16C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826CB170: 4BD9BCB1  bl 0x82466e20
	ctx.lr = 0x826CB174;
	sub_82466E20(ctx, base);
	// 826CB174: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826CB178: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826CB17C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826CB180: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826CB188(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826CB188 size=112
    let mut pc: u32 = 0x826CB188;
    'dispatch: loop {
        match pc {
            0x826CB188 => {
    //   block [0x826CB188..0x826CB1F8)
	// 826CB188: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826CB18C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826CB190: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826CB194: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826CB198: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826CB19C: 38AA7DE4  addi r5, r10, 0x7de4
	ctx.r[5].s64 = ctx.r[10].s64 + 32228;
	// 826CB1A0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826CB1A4: 390B93C8  addi r8, r11, -0x6c38
	ctx.r[8].s64 = ctx.r[11].s64 + -27704;
	// 826CB1A8: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 826CB1AC: 388A5480  addi r4, r10, 0x5480
	ctx.r[4].s64 = ctx.r[10].s64 + 21632;
	// 826CB1B0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826CB1B4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826CB1B8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826CB1BC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826CB1C0: 386A7E44  addi r3, r10, 0x7e44
	ctx.r[3].s64 = ctx.r[10].s64 + 32324;
	// 826CB1C4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826CB1C8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826CB1CC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826CB1D0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826CB1D4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826CB1D8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826CB1DC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826CB1E0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826CB1E4: 4BD9BC3D  bl 0x82466e20
	ctx.lr = 0x826CB1E8;
	sub_82466E20(ctx, base);
	// 826CB1E8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826CB1EC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826CB1F0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826CB1F4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826CB1F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826CB1F8 size=112
    let mut pc: u32 = 0x826CB1F8;
    'dispatch: loop {
        match pc {
            0x826CB1F8 => {
    //   block [0x826CB1F8..0x826CB268)
	// 826CB1F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826CB1FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826CB200: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826CB204: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826CB208: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826CB20C: 38AA7E14  addi r5, r10, 0x7e14
	ctx.r[5].s64 = ctx.r[10].s64 + 32276;
	// 826CB210: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826CB214: 390B9428  addi r8, r11, -0x6bd8
	ctx.r[8].s64 = ctx.r[11].s64 + -27608;
	// 826CB218: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 826CB21C: 388A54A4  addi r4, r10, 0x54a4
	ctx.r[4].s64 = ctx.r[10].s64 + 21668;
	// 826CB220: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826CB224: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826CB228: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826CB22C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826CB230: 386A7E74  addi r3, r10, 0x7e74
	ctx.r[3].s64 = ctx.r[10].s64 + 32372;
	// 826CB234: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826CB238: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826CB23C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826CB240: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826CB244: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826CB248: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826CB24C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826CB250: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826CB254: 4BD9BBCD  bl 0x82466e20
	ctx.lr = 0x826CB258;
	sub_82466E20(ctx, base);
	// 826CB258: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826CB25C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826CB260: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826CB264: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826CB268(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826CB268 size=100
    let mut pc: u32 = 0x826CB268;
    'dispatch: loop {
        match pc {
            0x826CB268 => {
    //   block [0x826CB268..0x826CB2CC)
	// 826CB268: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826CB26C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826CB270: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826CB274: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826CB278: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826CB27C: 38AA4EA4  addi r5, r10, 0x4ea4
	ctx.r[5].s64 = ctx.r[10].s64 + 20132;
	// 826CB280: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826CB284: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826CB288: 388A54C8  addi r4, r10, 0x54c8
	ctx.r[4].s64 = ctx.r[10].s64 + 21704;
	// 826CB28C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826CB290: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826CB294: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826CB298: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826CB29C: 386A7EA4  addi r3, r10, 0x7ea4
	ctx.r[3].s64 = ctx.r[10].s64 + 32420;
	// 826CB2A0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826CB2A4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826CB2A8: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826CB2AC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826CB2B0: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826CB2B4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826CB2B8: 4BD9BB69  bl 0x82466e20
	ctx.lr = 0x826CB2BC;
	sub_82466E20(ctx, base);
	// 826CB2BC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826CB2C0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826CB2C4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826CB2C8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826CB2D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826CB2D0 size=112
    let mut pc: u32 = 0x826CB2D0;
    'dispatch: loop {
        match pc {
            0x826CB2D0 => {
    //   block [0x826CB2D0..0x826CB340)
	// 826CB2D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826CB2D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826CB2D8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826CB2DC: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826CB2E0: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826CB2E4: 38AA7EA4  addi r5, r10, 0x7ea4
	ctx.r[5].s64 = ctx.r[10].s64 + 32420;
	// 826CB2E8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826CB2EC: 390B9488  addi r8, r11, -0x6b78
	ctx.r[8].s64 = ctx.r[11].s64 + -27512;
	// 826CB2F0: 3920000A  li r9, 0xa
	ctx.r[9].s64 = 10;
	// 826CB2F4: 388A54D8  addi r4, r10, 0x54d8
	ctx.r[4].s64 = ctx.r[10].s64 + 21720;
	// 826CB2F8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826CB2FC: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826CB300: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826CB304: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826CB308: 386A7ED4  addi r3, r10, 0x7ed4
	ctx.r[3].s64 = ctx.r[10].s64 + 32468;
	// 826CB30C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826CB310: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826CB314: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826CB318: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826CB31C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826CB320: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826CB324: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826CB328: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826CB32C: 4BD9BAF5  bl 0x82466e20
	ctx.lr = 0x826CB330;
	sub_82466E20(ctx, base);
	// 826CB330: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826CB334: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826CB338: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826CB33C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826CB340(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826CB340 size=108
    let mut pc: u32 = 0x826CB340;
    'dispatch: loop {
        match pc {
            0x826CB340 => {
    //   block [0x826CB340..0x826CB3AC)
	// 826CB340: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826CB344: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826CB348: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826CB34C: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826CB350: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826CB354: 38EB9578  addi r7, r11, -0x6a88
	ctx.r[7].s64 = ctx.r[11].s64 + -27272;
	// 826CB358: 3900000A  li r8, 0xa
	ctx.r[8].s64 = 10;
	// 826CB35C: 388A54F0  addi r4, r10, 0x54f0
	ctx.r[4].s64 = ctx.r[10].s64 + 21744;
	// 826CB360: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826CB364: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826CB368: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826CB36C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826CB370: 386A7F04  addi r3, r10, 0x7f04
	ctx.r[3].s64 = ctx.r[10].s64 + 32516;
	// 826CB374: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826CB378: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826CB37C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826CB380: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826CB384: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826CB388: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826CB38C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826CB390: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826CB394: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826CB398: 4BD9BA89  bl 0x82466e20
	ctx.lr = 0x826CB39C;
	sub_82466E20(ctx, base);
	// 826CB39C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826CB3A0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826CB3A4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826CB3A8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826CB3B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826CB3B0 size=108
    let mut pc: u32 = 0x826CB3B0;
    'dispatch: loop {
        match pc {
            0x826CB3B0 => {
    //   block [0x826CB3B0..0x826CB41C)
	// 826CB3B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826CB3B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826CB3B8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826CB3BC: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826CB3C0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826CB3C4: 38EB9668  addi r7, r11, -0x6998
	ctx.r[7].s64 = ctx.r[11].s64 + -27032;
	// 826CB3C8: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 826CB3CC: 388A551C  addi r4, r10, 0x551c
	ctx.r[4].s64 = ctx.r[10].s64 + 21788;
	// 826CB3D0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826CB3D4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826CB3D8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826CB3DC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826CB3E0: 386A7F34  addi r3, r10, 0x7f34
	ctx.r[3].s64 = ctx.r[10].s64 + 32564;
	// 826CB3E4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826CB3E8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826CB3EC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826CB3F0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826CB3F4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826CB3F8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826CB3FC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826CB400: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826CB404: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826CB408: 4BD9BA19  bl 0x82466e20
	ctx.lr = 0x826CB40C;
	sub_82466E20(ctx, base);
	// 826CB40C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826CB410: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826CB414: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826CB418: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826CB420(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826CB420 size=108
    let mut pc: u32 = 0x826CB420;
    'dispatch: loop {
        match pc {
            0x826CB420 => {
    //   block [0x826CB420..0x826CB48C)
	// 826CB420: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826CB424: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826CB428: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826CB42C: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826CB430: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826CB434: 38EB96B0  addi r7, r11, -0x6950
	ctx.r[7].s64 = ctx.r[11].s64 + -26960;
	// 826CB438: 39000009  li r8, 9
	ctx.r[8].s64 = 9;
	// 826CB43C: 388A553C  addi r4, r10, 0x553c
	ctx.r[4].s64 = ctx.r[10].s64 + 21820;
	// 826CB440: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826CB444: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826CB448: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826CB44C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826CB450: 386A7F64  addi r3, r10, 0x7f64
	ctx.r[3].s64 = ctx.r[10].s64 + 32612;
	// 826CB454: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826CB458: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826CB45C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826CB460: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826CB464: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826CB468: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826CB46C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826CB470: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826CB474: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826CB478: 4BD9B9A9  bl 0x82466e20
	ctx.lr = 0x826CB47C;
	sub_82466E20(ctx, base);
	// 826CB47C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826CB480: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826CB484: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826CB488: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826CB490(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826CB490 size=108
    let mut pc: u32 = 0x826CB490;
    'dispatch: loop {
        match pc {
            0x826CB490 => {
    //   block [0x826CB490..0x826CB4FC)
	// 826CB490: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826CB494: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826CB498: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826CB49C: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826CB4A0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826CB4A4: 38EB9788  addi r7, r11, -0x6878
	ctx.r[7].s64 = ctx.r[11].s64 + -26744;
	// 826CB4A8: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826CB4AC: 388A5560  addi r4, r10, 0x5560
	ctx.r[4].s64 = ctx.r[10].s64 + 21856;
	// 826CB4B0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826CB4B4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826CB4B8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826CB4BC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826CB4C0: 386A7F94  addi r3, r10, 0x7f94
	ctx.r[3].s64 = ctx.r[10].s64 + 32660;
	// 826CB4C4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826CB4C8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826CB4CC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826CB4D0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826CB4D4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826CB4D8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826CB4DC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826CB4E0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826CB4E4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826CB4E8: 4BD9B939  bl 0x82466e20
	ctx.lr = 0x826CB4EC;
	sub_82466E20(ctx, base);
	// 826CB4EC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826CB4F0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826CB4F4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826CB4F8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826CB500(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826CB500 size=100
    let mut pc: u32 = 0x826CB500;
    'dispatch: loop {
        match pc {
            0x826CB500 => {
    //   block [0x826CB500..0x826CB564)
	// 826CB500: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826CB504: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826CB508: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826CB50C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826CB510: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826CB514: 38AA4EA4  addi r5, r10, 0x4ea4
	ctx.r[5].s64 = ctx.r[10].s64 + 20132;
	// 826CB518: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826CB51C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826CB520: 388A5578  addi r4, r10, 0x5578
	ctx.r[4].s64 = ctx.r[10].s64 + 21880;
	// 826CB524: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826CB528: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826CB52C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826CB530: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826CB534: 386A7FC4  addi r3, r10, 0x7fc4
	ctx.r[3].s64 = ctx.r[10].s64 + 32708;
	// 826CB538: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826CB53C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826CB540: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826CB544: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826CB548: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826CB54C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826CB550: 4BD9B8D1  bl 0x82466e20
	ctx.lr = 0x826CB554;
	sub_82466E20(ctx, base);
	// 826CB554: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826CB558: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826CB55C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826CB560: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826CB568(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826CB568 size=112
    let mut pc: u32 = 0x826CB568;
    'dispatch: loop {
        match pc {
            0x826CB568 => {
    //   block [0x826CB568..0x826CB5D8)
	// 826CB568: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826CB56C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826CB570: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826CB574: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826CB578: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826CB57C: 38AA7FC4  addi r5, r10, 0x7fc4
	ctx.r[5].s64 = ctx.r[10].s64 + 32708;
	// 826CB580: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826CB584: 390B97A0  addi r8, r11, -0x6860
	ctx.r[8].s64 = ctx.r[11].s64 + -26720;
	// 826CB588: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 826CB58C: 388A558C  addi r4, r10, 0x558c
	ctx.r[4].s64 = ctx.r[10].s64 + 21900;
	// 826CB590: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826CB594: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826CB598: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826CB59C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826CB5A0: 386A7FF4  addi r3, r10, 0x7ff4
	ctx.r[3].s64 = ctx.r[10].s64 + 32756;
	// 826CB5A4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826CB5A8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826CB5AC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826CB5B0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826CB5B4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826CB5B8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826CB5BC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826CB5C0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826CB5C4: 4BD9B85D  bl 0x82466e20
	ctx.lr = 0x826CB5C8;
	sub_82466E20(ctx, base);
	// 826CB5C8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826CB5CC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826CB5D0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826CB5D4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826CB5D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826CB5D8 size=108
    let mut pc: u32 = 0x826CB5D8;
    'dispatch: loop {
        match pc {
            0x826CB5D8 => {
    //   block [0x826CB5D8..0x826CB644)
	// 826CB5D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826CB5DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826CB5E0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826CB5E4: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826CB5E8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826CB5EC: 38EB97E8  addi r7, r11, -0x6818
	ctx.r[7].s64 = ctx.r[11].s64 + -26648;
	// 826CB5F0: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 826CB5F4: 388A55A8  addi r4, r10, 0x55a8
	ctx.r[4].s64 = ctx.r[10].s64 + 21928;
	// 826CB5F8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826CB5FC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826CB600: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826CB604: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826CB608: 386A8024  addi r3, r10, -0x7fdc
	ctx.r[3].s64 = ctx.r[10].s64 + -32732;
	// 826CB60C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826CB610: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826CB614: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826CB618: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826CB61C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826CB620: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826CB624: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826CB628: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826CB62C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826CB630: 4BD9B7F1  bl 0x82466e20
	ctx.lr = 0x826CB634;
	sub_82466E20(ctx, base);
	// 826CB634: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826CB638: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826CB63C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826CB640: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826CB648(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826CB648 size=112
    let mut pc: u32 = 0x826CB648;
    'dispatch: loop {
        match pc {
            0x826CB648 => {
    //   block [0x826CB648..0x826CB6B8)
	// 826CB648: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826CB64C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826CB650: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826CB654: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826CB658: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826CB65C: 38AA4EA4  addi r5, r10, 0x4ea4
	ctx.r[5].s64 = ctx.r[10].s64 + 20132;
	// 826CB660: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826CB664: 390B9830  addi r8, r11, -0x67d0
	ctx.r[8].s64 = ctx.r[11].s64 + -26576;
	// 826CB668: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826CB66C: 388A55D8  addi r4, r10, 0x55d8
	ctx.r[4].s64 = ctx.r[10].s64 + 21976;
	// 826CB670: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826CB674: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826CB678: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826CB67C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826CB680: 386A8054  addi r3, r10, -0x7fac
	ctx.r[3].s64 = ctx.r[10].s64 + -32684;
	// 826CB684: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826CB688: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826CB68C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826CB690: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826CB694: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826CB698: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826CB69C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826CB6A0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826CB6A4: 4BD9B77D  bl 0x82466e20
	ctx.lr = 0x826CB6A8;
	sub_82466E20(ctx, base);
	// 826CB6A8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826CB6AC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826CB6B0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826CB6B4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826CB6B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826CB6B8 size=108
    let mut pc: u32 = 0x826CB6B8;
    'dispatch: loop {
        match pc {
            0x826CB6B8 => {
    //   block [0x826CB6B8..0x826CB724)
	// 826CB6B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826CB6BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826CB6C0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826CB6C4: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826CB6C8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826CB6CC: 38EB9848  addi r7, r11, -0x67b8
	ctx.r[7].s64 = ctx.r[11].s64 + -26552;
	// 826CB6D0: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 826CB6D4: 388A55EC  addi r4, r10, 0x55ec
	ctx.r[4].s64 = ctx.r[10].s64 + 21996;
	// 826CB6D8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826CB6DC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826CB6E0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826CB6E4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826CB6E8: 386A8084  addi r3, r10, -0x7f7c
	ctx.r[3].s64 = ctx.r[10].s64 + -32636;
	// 826CB6EC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826CB6F0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826CB6F4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826CB6F8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826CB6FC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826CB700: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826CB704: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826CB708: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826CB70C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826CB710: 4BD9B711  bl 0x82466e20
	ctx.lr = 0x826CB714;
	sub_82466E20(ctx, base);
	// 826CB714: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826CB718: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826CB71C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826CB720: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826CB728(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826CB728 size=112
    let mut pc: u32 = 0x826CB728;
    'dispatch: loop {
        match pc {
            0x826CB728 => {
    //   block [0x826CB728..0x826CB798)
	// 826CB728: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826CB72C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826CB730: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826CB734: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826CB738: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826CB73C: 38AA8054  addi r5, r10, -0x7fac
	ctx.r[5].s64 = ctx.r[10].s64 + -32684;
	// 826CB740: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826CB744: 390B9890  addi r8, r11, -0x6770
	ctx.r[8].s64 = ctx.r[11].s64 + -26480;
	// 826CB748: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826CB74C: 388A5628  addi r4, r10, 0x5628
	ctx.r[4].s64 = ctx.r[10].s64 + 22056;
	// 826CB750: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826CB754: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826CB758: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826CB75C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826CB760: 386A80B4  addi r3, r10, -0x7f4c
	ctx.r[3].s64 = ctx.r[10].s64 + -32588;
	// 826CB764: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826CB768: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826CB76C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826CB770: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826CB774: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826CB778: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826CB77C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826CB780: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826CB784: 4BD9B69D  bl 0x82466e20
	ctx.lr = 0x826CB788;
	sub_82466E20(ctx, base);
	// 826CB788: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826CB78C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826CB790: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826CB794: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826CB798(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826CB798 size=100
    let mut pc: u32 = 0x826CB798;
    'dispatch: loop {
        match pc {
            0x826CB798 => {
    //   block [0x826CB798..0x826CB7FC)
	// 826CB798: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826CB79C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826CB7A0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826CB7A4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826CB7A8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826CB7AC: 38AA4EA4  addi r5, r10, 0x4ea4
	ctx.r[5].s64 = ctx.r[10].s64 + 20132;
	// 826CB7B0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826CB7B4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826CB7B8: 388A5644  addi r4, r10, 0x5644
	ctx.r[4].s64 = ctx.r[10].s64 + 22084;
	// 826CB7BC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826CB7C0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826CB7C4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826CB7C8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826CB7CC: 386A80E4  addi r3, r10, -0x7f1c
	ctx.r[3].s64 = ctx.r[10].s64 + -32540;
	// 826CB7D0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826CB7D4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826CB7D8: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826CB7DC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826CB7E0: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826CB7E4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826CB7E8: 4BD9B639  bl 0x82466e20
	ctx.lr = 0x826CB7EC;
	sub_82466E20(ctx, base);
	// 826CB7EC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826CB7F0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826CB7F4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826CB7F8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826CB800(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826CB800 size=112
    let mut pc: u32 = 0x826CB800;
    'dispatch: loop {
        match pc {
            0x826CB800 => {
    //   block [0x826CB800..0x826CB870)
	// 826CB800: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826CB804: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826CB808: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826CB80C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826CB810: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826CB814: 38AA80E4  addi r5, r10, -0x7f1c
	ctx.r[5].s64 = ctx.r[10].s64 + -32540;
	// 826CB818: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826CB81C: 390B98A8  addi r8, r11, -0x6758
	ctx.r[8].s64 = ctx.r[11].s64 + -26456;
	// 826CB820: 39200007  li r9, 7
	ctx.r[9].s64 = 7;
	// 826CB824: 388A565C  addi r4, r10, 0x565c
	ctx.r[4].s64 = ctx.r[10].s64 + 22108;
	// 826CB828: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826CB82C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826CB830: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826CB834: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826CB838: 386A8114  addi r3, r10, -0x7eec
	ctx.r[3].s64 = ctx.r[10].s64 + -32492;
	// 826CB83C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826CB840: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826CB844: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826CB848: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826CB84C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826CB850: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826CB854: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826CB858: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826CB85C: 4BD9B5C5  bl 0x82466e20
	ctx.lr = 0x826CB860;
	sub_82466E20(ctx, base);
	// 826CB860: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826CB864: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826CB868: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826CB86C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826CB870(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826CB870 size=108
    let mut pc: u32 = 0x826CB870;
    'dispatch: loop {
        match pc {
            0x826CB870 => {
    //   block [0x826CB870..0x826CB8DC)
	// 826CB870: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826CB874: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826CB878: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826CB87C: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826CB880: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826CB884: 38EB9950  addi r7, r11, -0x66b0
	ctx.r[7].s64 = ctx.r[11].s64 + -26288;
	// 826CB888: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826CB88C: 388A567C  addi r4, r10, 0x567c
	ctx.r[4].s64 = ctx.r[10].s64 + 22140;
	// 826CB890: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826CB894: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826CB898: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826CB89C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826CB8A0: 386A8144  addi r3, r10, -0x7ebc
	ctx.r[3].s64 = ctx.r[10].s64 + -32444;
	// 826CB8A4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826CB8A8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826CB8AC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826CB8B0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826CB8B4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826CB8B8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826CB8BC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826CB8C0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826CB8C4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826CB8C8: 4BD9B559  bl 0x82466e20
	ctx.lr = 0x826CB8CC;
	sub_82466E20(ctx, base);
	// 826CB8CC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826CB8D0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826CB8D4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826CB8D8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826CB8E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826CB8E0 size=112
    let mut pc: u32 = 0x826CB8E0;
    'dispatch: loop {
        match pc {
            0x826CB8E0 => {
    //   block [0x826CB8E0..0x826CB950)
	// 826CB8E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826CB8E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826CB8E8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826CB8EC: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826CB8F0: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826CB8F4: 38AA4EA4  addi r5, r10, 0x4ea4
	ctx.r[5].s64 = ctx.r[10].s64 + 20132;
	// 826CB8F8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826CB8FC: 390B9980  addi r8, r11, -0x6680
	ctx.r[8].s64 = ctx.r[11].s64 + -26240;
	// 826CB900: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 826CB904: 388A568C  addi r4, r10, 0x568c
	ctx.r[4].s64 = ctx.r[10].s64 + 22156;
	// 826CB908: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826CB90C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826CB910: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826CB914: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826CB918: 386A8174  addi r3, r10, -0x7e8c
	ctx.r[3].s64 = ctx.r[10].s64 + -32396;
	// 826CB91C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826CB920: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826CB924: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826CB928: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826CB92C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826CB930: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826CB934: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826CB938: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826CB93C: 4BD9B4E5  bl 0x82466e20
	ctx.lr = 0x826CB940;
	sub_82466E20(ctx, base);
	// 826CB940: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826CB944: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826CB948: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826CB94C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826CB950(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826CB950 size=112
    let mut pc: u32 = 0x826CB950;
    'dispatch: loop {
        match pc {
            0x826CB950 => {
    //   block [0x826CB950..0x826CB9C0)
	// 826CB950: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826CB954: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826CB958: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826CB95C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826CB960: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826CB964: 38AA4EA4  addi r5, r10, 0x4ea4
	ctx.r[5].s64 = ctx.r[10].s64 + 20132;
	// 826CB968: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826CB96C: 390B99C8  addi r8, r11, -0x6638
	ctx.r[8].s64 = ctx.r[11].s64 + -26168;
	// 826CB970: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 826CB974: 388A56A0  addi r4, r10, 0x56a0
	ctx.r[4].s64 = ctx.r[10].s64 + 22176;
	// 826CB978: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826CB97C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826CB980: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826CB984: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826CB988: 386A81A4  addi r3, r10, -0x7e5c
	ctx.r[3].s64 = ctx.r[10].s64 + -32348;
	// 826CB98C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826CB990: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826CB994: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826CB998: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826CB99C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826CB9A0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826CB9A4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826CB9A8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826CB9AC: 4BD9B475  bl 0x82466e20
	ctx.lr = 0x826CB9B0;
	sub_82466E20(ctx, base);
	// 826CB9B0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826CB9B4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826CB9B8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826CB9BC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826CB9C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826CB9C0 size=100
    let mut pc: u32 = 0x826CB9C0;
    'dispatch: loop {
        match pc {
            0x826CB9C0 => {
    //   block [0x826CB9C0..0x826CBA24)
	// 826CB9C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826CB9C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826CB9C8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826CB9CC: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826CB9D0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826CB9D4: 38AA4EA4  addi r5, r10, 0x4ea4
	ctx.r[5].s64 = ctx.r[10].s64 + 20132;
	// 826CB9D8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826CB9DC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826CB9E0: 388A56B0  addi r4, r10, 0x56b0
	ctx.r[4].s64 = ctx.r[10].s64 + 22192;
	// 826CB9E4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826CB9E8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826CB9EC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826CB9F0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826CB9F4: 386A81D4  addi r3, r10, -0x7e2c
	ctx.r[3].s64 = ctx.r[10].s64 + -32300;
	// 826CB9F8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826CB9FC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826CBA00: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826CBA04: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826CBA08: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826CBA0C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826CBA10: 4BD9B411  bl 0x82466e20
	ctx.lr = 0x826CBA14;
	sub_82466E20(ctx, base);
	// 826CBA14: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826CBA18: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826CBA1C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826CBA20: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826CBA28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826CBA28 size=112
    let mut pc: u32 = 0x826CBA28;
    'dispatch: loop {
        match pc {
            0x826CBA28 => {
    //   block [0x826CBA28..0x826CBA98)
	// 826CBA28: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826CBA2C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826CBA30: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826CBA34: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826CBA38: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826CBA3C: 38AA81D4  addi r5, r10, -0x7e2c
	ctx.r[5].s64 = ctx.r[10].s64 + -32300;
	// 826CBA40: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826CBA44: 390B9A10  addi r8, r11, -0x65f0
	ctx.r[8].s64 = ctx.r[11].s64 + -26096;
	// 826CBA48: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 826CBA4C: 388A56C8  addi r4, r10, 0x56c8
	ctx.r[4].s64 = ctx.r[10].s64 + 22216;
	// 826CBA50: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826CBA54: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826CBA58: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826CBA5C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826CBA60: 386A8204  addi r3, r10, -0x7dfc
	ctx.r[3].s64 = ctx.r[10].s64 + -32252;
	// 826CBA64: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826CBA68: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826CBA6C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826CBA70: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826CBA74: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826CBA78: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826CBA7C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826CBA80: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826CBA84: 4BD9B39D  bl 0x82466e20
	ctx.lr = 0x826CBA88;
	sub_82466E20(ctx, base);
	// 826CBA88: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826CBA8C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826CBA90: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826CBA94: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826CBA98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826CBA98 size=112
    let mut pc: u32 = 0x826CBA98;
    'dispatch: loop {
        match pc {
            0x826CBA98 => {
    //   block [0x826CBA98..0x826CBB08)
	// 826CBA98: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826CBA9C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826CBAA0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826CBAA4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826CBAA8: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826CBAAC: 38AA4EA4  addi r5, r10, 0x4ea4
	ctx.r[5].s64 = ctx.r[10].s64 + 20132;
	// 826CBAB0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826CBAB4: 390B9A58  addi r8, r11, -0x65a8
	ctx.r[8].s64 = ctx.r[11].s64 + -26024;
	// 826CBAB8: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826CBABC: 388A56E8  addi r4, r10, 0x56e8
	ctx.r[4].s64 = ctx.r[10].s64 + 22248;
	// 826CBAC0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826CBAC4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826CBAC8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826CBACC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826CBAD0: 386A8234  addi r3, r10, -0x7dcc
	ctx.r[3].s64 = ctx.r[10].s64 + -32204;
	// 826CBAD4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826CBAD8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826CBADC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826CBAE0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826CBAE4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826CBAE8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826CBAEC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826CBAF0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826CBAF4: 4BD9B32D  bl 0x82466e20
	ctx.lr = 0x826CBAF8;
	sub_82466E20(ctx, base);
	// 826CBAF8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826CBAFC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826CBB00: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826CBB04: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826CBB08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826CBB08 size=112
    let mut pc: u32 = 0x826CBB08;
    'dispatch: loop {
        match pc {
            0x826CBB08 => {
    //   block [0x826CBB08..0x826CBB78)
	// 826CBB08: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826CBB0C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826CBB10: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826CBB14: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826CBB18: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826CBB1C: 38AA4EA4  addi r5, r10, 0x4ea4
	ctx.r[5].s64 = ctx.r[10].s64 + 20132;
	// 826CBB20: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826CBB24: 390B9A70  addi r8, r11, -0x6590
	ctx.r[8].s64 = ctx.r[11].s64 + -26000;
	// 826CBB28: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826CBB2C: 388A5700  addi r4, r10, 0x5700
	ctx.r[4].s64 = ctx.r[10].s64 + 22272;
	// 826CBB30: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826CBB34: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826CBB38: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826CBB3C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826CBB40: 386A8264  addi r3, r10, -0x7d9c
	ctx.r[3].s64 = ctx.r[10].s64 + -32156;
	// 826CBB44: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826CBB48: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826CBB4C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826CBB50: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826CBB54: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826CBB58: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826CBB5C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826CBB60: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826CBB64: 4BD9B2BD  bl 0x82466e20
	ctx.lr = 0x826CBB68;
	sub_82466E20(ctx, base);
	// 826CBB68: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826CBB6C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826CBB70: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826CBB74: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826CBB78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826CBB78 size=112
    let mut pc: u32 = 0x826CBB78;
    'dispatch: loop {
        match pc {
            0x826CBB78 => {
    //   block [0x826CBB78..0x826CBBE8)
	// 826CBB78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826CBB7C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826CBB80: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826CBB84: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826CBB88: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826CBB8C: 38AA8234  addi r5, r10, -0x7dcc
	ctx.r[5].s64 = ctx.r[10].s64 + -32204;
	// 826CBB90: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826CBB94: 390B9A88  addi r8, r11, -0x6578
	ctx.r[8].s64 = ctx.r[11].s64 + -25976;
	// 826CBB98: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 826CBB9C: 388A571C  addi r4, r10, 0x571c
	ctx.r[4].s64 = ctx.r[10].s64 + 22300;
	// 826CBBA0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826CBBA4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826CBBA8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826CBBAC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826CBBB0: 386A8294  addi r3, r10, -0x7d6c
	ctx.r[3].s64 = ctx.r[10].s64 + -32108;
	// 826CBBB4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826CBBB8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826CBBBC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826CBBC0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826CBBC4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826CBBC8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826CBBCC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826CBBD0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826CBBD4: 4BD9B24D  bl 0x82466e20
	ctx.lr = 0x826CBBD8;
	sub_82466E20(ctx, base);
	// 826CBBD8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826CBBDC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826CBBE0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826CBBE4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826CBBE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826CBBE8 size=72
    let mut pc: u32 = 0x826CBBE8;
    'dispatch: loop {
        match pc {
            0x826CBBE8 => {
    //   block [0x826CBBE8..0x826CBC30)
	// 826CBBE8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826CBBEC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826CBBF0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826CBBF4: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 826CBBF8: 38A00004  li r5, 4
	ctx.r[5].s64 = 4;
	// 826CBBFC: 38CB1AA0  addi r6, r11, 0x1aa0
	ctx.r[6].s64 = ctx.r[11].s64 + 6816;
	// 826CBC00: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 826CBC04: 388B2CA0  addi r4, r11, 0x2ca0
	ctx.r[4].s64 = ctx.r[11].s64 + 11424;
	// 826CBC08: 3D608299  lis r11, -0x7d67
	ctx.r[11].s64 = -2103902208;
	// 826CBC0C: 386B82C4  addi r3, r11, -0x7d3c
	ctx.r[3].s64 = ctx.r[11].s64 + -32060;
	// 826CBC10: 4BDAFE79  bl 0x8247ba88
	ctx.lr = 0x826CBC14;
	sub_8247BA88(ctx, base);
	// 826CBC14: 3D608271  lis r11, -0x7d8f
	ctx.r[11].s64 = -2106523648;
	// 826CBC18: 386BCEE8  addi r3, r11, -0x3118
	ctx.r[3].s64 = ctx.r[11].s64 + -12568;
	// 826CBC1C: 4BE66F1D  bl 0x82532b38
	ctx.lr = 0x826CBC20;
	sub_82532B38(ctx, base);
	// 826CBC20: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 826CBC24: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826CBC28: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826CBC2C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826CBC30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826CBC30 size=108
    let mut pc: u32 = 0x826CBC30;
    'dispatch: loop {
        match pc {
            0x826CBC30 => {
    //   block [0x826CBC30..0x826CBC9C)
	// 826CBC30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826CBC34: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826CBC38: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826CBC3C: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826CBC40: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826CBC44: 38EBA988  addi r7, r11, -0x5678
	ctx.r[7].s64 = ctx.r[11].s64 + -22136;
	// 826CBC48: 39000005  li r8, 5
	ctx.r[8].s64 = 5;
	// 826CBC4C: 388A2B24  addi r4, r10, 0x2b24
	ctx.r[4].s64 = ctx.r[10].s64 + 11044;
	// 826CBC50: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826CBC54: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826CBC58: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826CBC5C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826CBC60: 386A82DC  addi r3, r10, -0x7d24
	ctx.r[3].s64 = ctx.r[10].s64 + -32036;
	// 826CBC64: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826CBC68: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826CBC6C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826CBC70: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826CBC74: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826CBC78: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826CBC7C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826CBC80: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826CBC84: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826CBC88: 4BD9B199  bl 0x82466e20
	ctx.lr = 0x826CBC8C;
	sub_82466E20(ctx, base);
	// 826CBC8C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826CBC90: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826CBC94: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826CBC98: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826CBCA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826CBCA0 size=24
    let mut pc: u32 = 0x826CBCA0;
    'dispatch: loop {
        match pc {
            0x826CBCA0 => {
    //   block [0x826CBCA0..0x826CBCB8)
	// 826CBCA0: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826CBCA4: 3D40827F  lis r10, -0x7d81
	ctx.r[10].s64 = -2105606144;
	// 826CBCA8: 394AF288  addi r10, r10, -0xd78
	ctx.r[10].s64 = ctx.r[10].s64 + -3448;
	// 826CBCAC: 816BAA00  lwz r11, -0x5600(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-22016 as u32) ) } as u64;
	// 826CBCB0: 916A0038  stw r11, 0x38(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(56 as u32), ctx.r[11].u32 ) };
	// 826CBCB4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826CBCB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826CBCB8 size=112
    let mut pc: u32 = 0x826CBCB8;
    'dispatch: loop {
        match pc {
            0x826CBCB8 => {
    //   block [0x826CBCB8..0x826CBD28)
	// 826CBCB8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826CBCBC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826CBCC0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826CBCC4: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 826CBCC8: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826CBCCC: 392A32D0  addi r9, r10, 0x32d0
	ctx.r[9].s64 = ctx.r[10].s64 + 13008;
	// 826CBCD0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826CBCD4: 390BF288  addi r8, r11, -0xd78
	ctx.r[8].s64 = ctx.r[11].s64 + -3448;
	// 826CBCD8: 38E00003  li r7, 3
	ctx.r[7].s64 = 3;
	// 826CBCDC: 388A2B3C  addi r4, r10, 0x2b3c
	ctx.r[4].s64 = ctx.r[10].s64 + 11068;
	// 826CBCE0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826CBCE4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826CBCE8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826CBCEC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826CBCF0: 386A830C  addi r3, r10, -0x7cf4
	ctx.r[3].s64 = ctx.r[10].s64 + -31988;
	// 826CBCF4: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826CBCF8: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826CBCFC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826CBD00: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826CBD04: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826CBD08: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826CBD0C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826CBD10: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826CBD14: 4BD9B10D  bl 0x82466e20
	ctx.lr = 0x826CBD18;
	sub_82466E20(ctx, base);
	// 826CBD18: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826CBD1C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826CBD20: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826CBD24: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826CBD28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826CBD28 size=108
    let mut pc: u32 = 0x826CBD28;
    'dispatch: loop {
        match pc {
            0x826CBD28 => {
    //   block [0x826CBD28..0x826CBD94)
	// 826CBD28: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826CBD2C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826CBD30: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826CBD34: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826CBD38: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826CBD3C: 38EBAA04  addi r7, r11, -0x55fc
	ctx.r[7].s64 = ctx.r[11].s64 + -22012;
	// 826CBD40: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826CBD44: 388A2B50  addi r4, r10, 0x2b50
	ctx.r[4].s64 = ctx.r[10].s64 + 11088;
	// 826CBD48: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826CBD4C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826CBD50: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826CBD54: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826CBD58: 386A833C  addi r3, r10, -0x7cc4
	ctx.r[3].s64 = ctx.r[10].s64 + -31940;
	// 826CBD5C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826CBD60: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826CBD64: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826CBD68: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826CBD6C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826CBD70: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826CBD74: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826CBD78: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826CBD7C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826CBD80: 4BD9B0A1  bl 0x82466e20
	ctx.lr = 0x826CBD84;
	sub_82466E20(ctx, base);
	// 826CBD84: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826CBD88: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826CBD8C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826CBD90: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826CBD98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826CBD98 size=108
    let mut pc: u32 = 0x826CBD98;
    'dispatch: loop {
        match pc {
            0x826CBD98 => {
    //   block [0x826CBD98..0x826CBE04)
	// 826CBD98: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826CBD9C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826CBDA0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826CBDA4: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826CBDA8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826CBDAC: 38EBAA34  addi r7, r11, -0x55cc
	ctx.r[7].s64 = ctx.r[11].s64 + -21964;
	// 826CBDB0: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826CBDB4: 388A2B6C  addi r4, r10, 0x2b6c
	ctx.r[4].s64 = ctx.r[10].s64 + 11116;
	// 826CBDB8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826CBDBC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826CBDC0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826CBDC4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826CBDC8: 386A836C  addi r3, r10, -0x7c94
	ctx.r[3].s64 = ctx.r[10].s64 + -31892;
	// 826CBDCC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826CBDD0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826CBDD4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826CBDD8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826CBDDC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826CBDE0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826CBDE4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826CBDE8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826CBDEC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826CBDF0: 4BD9B031  bl 0x82466e20
	ctx.lr = 0x826CBDF4;
	sub_82466E20(ctx, base);
	// 826CBDF4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826CBDF8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826CBDFC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826CBE00: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826CBE08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826CBE08 size=112
    let mut pc: u32 = 0x826CBE08;
    'dispatch: loop {
        match pc {
            0x826CBE08 => {
    //   block [0x826CBE08..0x826CBE78)
	// 826CBE08: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826CBE0C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826CBE10: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826CBE14: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826CBE18: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826CBE1C: 38AA86FC  addi r5, r10, -0x7904
	ctx.r[5].s64 = ctx.r[10].s64 + -30980;
	// 826CBE20: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826CBE24: 390BAA68  addi r8, r11, -0x5598
	ctx.r[8].s64 = ctx.r[11].s64 + -21912;
	// 826CBE28: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 826CBE2C: 388A2BB4  addi r4, r10, 0x2bb4
	ctx.r[4].s64 = ctx.r[10].s64 + 11188;
	// 826CBE30: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826CBE34: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826CBE38: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826CBE3C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826CBE40: 386A839C  addi r3, r10, -0x7c64
	ctx.r[3].s64 = ctx.r[10].s64 + -31844;
	// 826CBE44: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826CBE48: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826CBE4C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826CBE50: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826CBE54: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826CBE58: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826CBE5C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826CBE60: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826CBE64: 4BD9AFBD  bl 0x82466e20
	ctx.lr = 0x826CBE68;
	sub_82466E20(ctx, base);
	// 826CBE68: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826CBE6C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826CBE70: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826CBE74: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826CBE78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826CBE78 size=108
    let mut pc: u32 = 0x826CBE78;
    'dispatch: loop {
        match pc {
            0x826CBE78 => {
    //   block [0x826CBE78..0x826CBEE4)
	// 826CBE78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826CBE7C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826CBE80: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826CBE84: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826CBE88: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826CBE8C: 38EBAAC8  addi r7, r11, -0x5538
	ctx.r[7].s64 = ctx.r[11].s64 + -21816;
	// 826CBE90: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 826CBE94: 388A2BC8  addi r4, r10, 0x2bc8
	ctx.r[4].s64 = ctx.r[10].s64 + 11208;
	// 826CBE98: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826CBE9C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826CBEA0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826CBEA4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826CBEA8: 386A83CC  addi r3, r10, -0x7c34
	ctx.r[3].s64 = ctx.r[10].s64 + -31796;
	// 826CBEAC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826CBEB0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826CBEB4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826CBEB8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826CBEBC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826CBEC0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826CBEC4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826CBEC8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826CBECC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826CBED0: 4BD9AF51  bl 0x82466e20
	ctx.lr = 0x826CBED4;
	sub_82466E20(ctx, base);
	// 826CBED4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826CBED8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826CBEDC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826CBEE0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826CBEE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826CBEE8 size=112
    let mut pc: u32 = 0x826CBEE8;
    'dispatch: loop {
        match pc {
            0x826CBEE8 => {
    //   block [0x826CBEE8..0x826CBF58)
	// 826CBEE8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826CBEEC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826CBEF0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826CBEF4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826CBEF8: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826CBEFC: 38AA839C  addi r5, r10, -0x7c64
	ctx.r[5].s64 = ctx.r[10].s64 + -31844;
	// 826CBF00: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826CBF04: 390BAB28  addi r8, r11, -0x54d8
	ctx.r[8].s64 = ctx.r[11].s64 + -21720;
	// 826CBF08: 39200006  li r9, 6
	ctx.r[9].s64 = 6;
	// 826CBF0C: 388A2C00  addi r4, r10, 0x2c00
	ctx.r[4].s64 = ctx.r[10].s64 + 11264;
	// 826CBF10: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826CBF14: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826CBF18: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826CBF1C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826CBF20: 386A83FC  addi r3, r10, -0x7c04
	ctx.r[3].s64 = ctx.r[10].s64 + -31748;
	// 826CBF24: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826CBF28: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826CBF2C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826CBF30: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826CBF34: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826CBF38: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826CBF3C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826CBF40: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826CBF44: 4BD9AEDD  bl 0x82466e20
	ctx.lr = 0x826CBF48;
	sub_82466E20(ctx, base);
	// 826CBF48: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826CBF4C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826CBF50: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826CBF54: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826CBF58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826CBF58 size=112
    let mut pc: u32 = 0x826CBF58;
    'dispatch: loop {
        match pc {
            0x826CBF58 => {
    //   block [0x826CBF58..0x826CBFC8)
	// 826CBF58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826CBF5C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826CBF60: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826CBF64: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826CBF68: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826CBF6C: 38AA839C  addi r5, r10, -0x7c64
	ctx.r[5].s64 = ctx.r[10].s64 + -31844;
	// 826CBF70: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826CBF74: 390BABB8  addi r8, r11, -0x5448
	ctx.r[8].s64 = ctx.r[11].s64 + -21576;
	// 826CBF78: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826CBF7C: 388A2C24  addi r4, r10, 0x2c24
	ctx.r[4].s64 = ctx.r[10].s64 + 11300;
	// 826CBF80: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826CBF84: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826CBF88: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826CBF8C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826CBF90: 386A842C  addi r3, r10, -0x7bd4
	ctx.r[3].s64 = ctx.r[10].s64 + -31700;
	// 826CBF94: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826CBF98: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826CBF9C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826CBFA0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826CBFA4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826CBFA8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826CBFAC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826CBFB0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826CBFB4: 4BD9AE6D  bl 0x82466e20
	ctx.lr = 0x826CBFB8;
	sub_82466E20(ctx, base);
	// 826CBFB8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826CBFBC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826CBFC0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826CBFC4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826CBFC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826CBFC8 size=108
    let mut pc: u32 = 0x826CBFC8;
    'dispatch: loop {
        match pc {
            0x826CBFC8 => {
    //   block [0x826CBFC8..0x826CC034)
	// 826CBFC8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826CBFCC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826CBFD0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826CBFD4: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826CBFD8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826CBFDC: 38EBABD0  addi r7, r11, -0x5430
	ctx.r[7].s64 = ctx.r[11].s64 + -21552;
	// 826CBFE0: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 826CBFE4: 388A2C44  addi r4, r10, 0x2c44
	ctx.r[4].s64 = ctx.r[10].s64 + 11332;
	// 826CBFE8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826CBFEC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826CBFF0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826CBFF4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826CBFF8: 386A845C  addi r3, r10, -0x7ba4
	ctx.r[3].s64 = ctx.r[10].s64 + -31652;
	// 826CBFFC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826CC000: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826CC004: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826CC008: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826CC00C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826CC010: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826CC014: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826CC018: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826CC01C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826CC020: 4BD9AE01  bl 0x82466e20
	ctx.lr = 0x826CC024;
	sub_82466E20(ctx, base);
	// 826CC024: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826CC028: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826CC02C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826CC030: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826CC038(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826CC038 size=112
    let mut pc: u32 = 0x826CC038;
    'dispatch: loop {
        match pc {
            0x826CC038 => {
    //   block [0x826CC038..0x826CC0A8)
	// 826CC038: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826CC03C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826CC040: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826CC044: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826CC048: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826CC04C: 38AA839C  addi r5, r10, -0x7c64
	ctx.r[5].s64 = ctx.r[10].s64 + -31844;
	// 826CC050: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826CC054: 390BAC30  addi r8, r11, -0x53d0
	ctx.r[8].s64 = ctx.r[11].s64 + -21456;
	// 826CC058: 39200007  li r9, 7
	ctx.r[9].s64 = 7;
	// 826CC05C: 388A2C74  addi r4, r10, 0x2c74
	ctx.r[4].s64 = ctx.r[10].s64 + 11380;
	// 826CC060: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826CC064: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826CC068: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826CC06C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826CC070: 386A848C  addi r3, r10, -0x7b74
	ctx.r[3].s64 = ctx.r[10].s64 + -31604;
	// 826CC074: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826CC078: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826CC07C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826CC080: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826CC084: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826CC088: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826CC08C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826CC090: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826CC094: 4BD9AD8D  bl 0x82466e20
	ctx.lr = 0x826CC098;
	sub_82466E20(ctx, base);
	// 826CC098: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826CC09C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826CC0A0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826CC0A4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826CC0A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826CC0A8 size=108
    let mut pc: u32 = 0x826CC0A8;
    'dispatch: loop {
        match pc {
            0x826CC0A8 => {
    //   block [0x826CC0A8..0x826CC114)
	// 826CC0A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826CC0AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826CC0B0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826CC0B4: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826CC0B8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826CC0BC: 38EBACD8  addi r7, r11, -0x5328
	ctx.r[7].s64 = ctx.r[11].s64 + -21288;
	// 826CC0C0: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826CC0C4: 388A2C90  addi r4, r10, 0x2c90
	ctx.r[4].s64 = ctx.r[10].s64 + 11408;
	// 826CC0C8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826CC0CC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826CC0D0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826CC0D4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826CC0D8: 386A84BC  addi r3, r10, -0x7b44
	ctx.r[3].s64 = ctx.r[10].s64 + -31556;
	// 826CC0DC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826CC0E0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826CC0E4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826CC0E8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826CC0EC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826CC0F0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826CC0F4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826CC0F8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826CC0FC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826CC100: 4BD9AD21  bl 0x82466e20
	ctx.lr = 0x826CC104;
	sub_82466E20(ctx, base);
	// 826CC104: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826CC108: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826CC10C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826CC110: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826CC118(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826CC118 size=108
    let mut pc: u32 = 0x826CC118;
    'dispatch: loop {
        match pc {
            0x826CC118 => {
    //   block [0x826CC118..0x826CC184)
	// 826CC118: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826CC11C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826CC120: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826CC124: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826CC128: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826CC12C: 38EBACF0  addi r7, r11, -0x5310
	ctx.r[7].s64 = ctx.r[11].s64 + -21264;
	// 826CC130: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 826CC134: 388A2CA8  addi r4, r10, 0x2ca8
	ctx.r[4].s64 = ctx.r[10].s64 + 11432;
	// 826CC138: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826CC13C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826CC140: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826CC144: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826CC148: 386A84EC  addi r3, r10, -0x7b14
	ctx.r[3].s64 = ctx.r[10].s64 + -31508;
	// 826CC14C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826CC150: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826CC154: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826CC158: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826CC15C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826CC160: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826CC164: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826CC168: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826CC16C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826CC170: 4BD9ACB1  bl 0x82466e20
	ctx.lr = 0x826CC174;
	sub_82466E20(ctx, base);
	// 826CC174: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826CC178: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826CC17C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826CC180: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826CC188(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826CC188 size=112
    let mut pc: u32 = 0x826CC188;
    'dispatch: loop {
        match pc {
            0x826CC188 => {
    //   block [0x826CC188..0x826CC1F8)
	// 826CC188: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826CC18C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826CC190: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826CC194: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826CC198: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826CC19C: 38AA86FC  addi r5, r10, -0x7904
	ctx.r[5].s64 = ctx.r[10].s64 + -30980;
	// 826CC1A0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826CC1A4: 390BAD50  addi r8, r11, -0x52b0
	ctx.r[8].s64 = ctx.r[11].s64 + -21168;
	// 826CC1A8: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826CC1AC: 388A2CB8  addi r4, r10, 0x2cb8
	ctx.r[4].s64 = ctx.r[10].s64 + 11448;
	// 826CC1B0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826CC1B4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826CC1B8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826CC1BC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826CC1C0: 386A851C  addi r3, r10, -0x7ae4
	ctx.r[3].s64 = ctx.r[10].s64 + -31460;
	// 826CC1C4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826CC1C8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826CC1CC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826CC1D0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826CC1D4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826CC1D8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826CC1DC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826CC1E0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826CC1E4: 4BD9AC3D  bl 0x82466e20
	ctx.lr = 0x826CC1E8;
	sub_82466E20(ctx, base);
	// 826CC1E8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826CC1EC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826CC1F0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826CC1F4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826CC1F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826CC1F8 size=108
    let mut pc: u32 = 0x826CC1F8;
    'dispatch: loop {
        match pc {
            0x826CC1F8 => {
    //   block [0x826CC1F8..0x826CC264)
	// 826CC1F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826CC1FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826CC200: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826CC204: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826CC208: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826CC20C: 38EBAD68  addi r7, r11, -0x5298
	ctx.r[7].s64 = ctx.r[11].s64 + -21144;
	// 826CC210: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 826CC214: 388A2CCC  addi r4, r10, 0x2ccc
	ctx.r[4].s64 = ctx.r[10].s64 + 11468;
	// 826CC218: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826CC21C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826CC220: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826CC224: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826CC228: 386A854C  addi r3, r10, -0x7ab4
	ctx.r[3].s64 = ctx.r[10].s64 + -31412;
	// 826CC22C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826CC230: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826CC234: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826CC238: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826CC23C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826CC240: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826CC244: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826CC248: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826CC24C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826CC250: 4BD9ABD1  bl 0x82466e20
	ctx.lr = 0x826CC254;
	sub_82466E20(ctx, base);
	// 826CC254: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826CC258: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826CC25C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826CC260: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826CC268(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826CC268 size=108
    let mut pc: u32 = 0x826CC268;
    'dispatch: loop {
        match pc {
            0x826CC268 => {
    //   block [0x826CC268..0x826CC2D4)
	// 826CC268: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826CC26C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826CC270: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826CC274: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826CC278: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826CC27C: 38EBADB0  addi r7, r11, -0x5250
	ctx.r[7].s64 = ctx.r[11].s64 + -21072;
	// 826CC280: 39000006  li r8, 6
	ctx.r[8].s64 = 6;
	// 826CC284: 388A2CF0  addi r4, r10, 0x2cf0
	ctx.r[4].s64 = ctx.r[10].s64 + 11504;
	// 826CC288: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826CC28C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826CC290: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826CC294: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826CC298: 386A857C  addi r3, r10, -0x7a84
	ctx.r[3].s64 = ctx.r[10].s64 + -31364;
	// 826CC29C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826CC2A0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826CC2A4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826CC2A8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826CC2AC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826CC2B0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826CC2B4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826CC2B8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826CC2BC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826CC2C0: 4BD9AB61  bl 0x82466e20
	ctx.lr = 0x826CC2C4;
	sub_82466E20(ctx, base);
	// 826CC2C4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826CC2C8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826CC2CC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826CC2D0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826CC2D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826CC2D8 size=108
    let mut pc: u32 = 0x826CC2D8;
    'dispatch: loop {
        match pc {
            0x826CC2D8 => {
    //   block [0x826CC2D8..0x826CC344)
	// 826CC2D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826CC2DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826CC2E0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826CC2E4: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826CC2E8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826CC2EC: 38EBAE40  addi r7, r11, -0x51c0
	ctx.r[7].s64 = ctx.r[11].s64 + -20928;
	// 826CC2F0: 39000006  li r8, 6
	ctx.r[8].s64 = 6;
	// 826CC2F4: 388A2D14  addi r4, r10, 0x2d14
	ctx.r[4].s64 = ctx.r[10].s64 + 11540;
	// 826CC2F8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826CC2FC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826CC300: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826CC304: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826CC308: 386A85AC  addi r3, r10, -0x7a54
	ctx.r[3].s64 = ctx.r[10].s64 + -31316;
	// 826CC30C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826CC310: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826CC314: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826CC318: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826CC31C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826CC320: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826CC324: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826CC328: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826CC32C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826CC330: 4BD9AAF1  bl 0x82466e20
	ctx.lr = 0x826CC334;
	sub_82466E20(ctx, base);
	// 826CC334: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826CC338: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826CC33C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826CC340: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826CC348(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826CC348 size=100
    let mut pc: u32 = 0x826CC348;
    'dispatch: loop {
        match pc {
            0x826CC348 => {
    //   block [0x826CC348..0x826CC3AC)
	// 826CC348: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826CC34C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826CC350: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826CC354: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826CC358: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826CC35C: 38AA86FC  addi r5, r10, -0x7904
	ctx.r[5].s64 = ctx.r[10].s64 + -30980;
	// 826CC360: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826CC364: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826CC368: 388A2D2C  addi r4, r10, 0x2d2c
	ctx.r[4].s64 = ctx.r[10].s64 + 11564;
	// 826CC36C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826CC370: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826CC374: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826CC378: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826CC37C: 386A85DC  addi r3, r10, -0x7a24
	ctx.r[3].s64 = ctx.r[10].s64 + -31268;
	// 826CC380: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826CC384: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826CC388: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826CC38C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826CC390: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826CC394: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826CC398: 4BD9AA89  bl 0x82466e20
	ctx.lr = 0x826CC39C;
	sub_82466E20(ctx, base);
	// 826CC39C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826CC3A0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826CC3A4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826CC3A8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826CC3B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826CC3B0 size=112
    let mut pc: u32 = 0x826CC3B0;
    'dispatch: loop {
        match pc {
            0x826CC3B0 => {
    //   block [0x826CC3B0..0x826CC420)
	// 826CC3B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826CC3B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826CC3B8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826CC3BC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826CC3C0: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826CC3C4: 38AA85DC  addi r5, r10, -0x7a24
	ctx.r[5].s64 = ctx.r[10].s64 + -31268;
	// 826CC3C8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826CC3CC: 390BAED0  addi r8, r11, -0x5130
	ctx.r[8].s64 = ctx.r[11].s64 + -20784;
	// 826CC3D0: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 826CC3D4: 388A2D48  addi r4, r10, 0x2d48
	ctx.r[4].s64 = ctx.r[10].s64 + 11592;
	// 826CC3D8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826CC3DC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826CC3E0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826CC3E4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826CC3E8: 386A860C  addi r3, r10, -0x79f4
	ctx.r[3].s64 = ctx.r[10].s64 + -31220;
	// 826CC3EC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826CC3F0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826CC3F4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826CC3F8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826CC3FC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826CC400: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826CC404: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826CC408: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826CC40C: 4BD9AA15  bl 0x82466e20
	ctx.lr = 0x826CC410;
	sub_82466E20(ctx, base);
	// 826CC410: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826CC414: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826CC418: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826CC41C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826CC420(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826CC420 size=108
    let mut pc: u32 = 0x826CC420;
    'dispatch: loop {
        match pc {
            0x826CC420 => {
    //   block [0x826CC420..0x826CC48C)
	// 826CC420: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826CC424: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826CC428: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826CC42C: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826CC430: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826CC434: 38EBAF30  addi r7, r11, -0x50d0
	ctx.r[7].s64 = ctx.r[11].s64 + -20688;
	// 826CC438: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826CC43C: 388A2D68  addi r4, r10, 0x2d68
	ctx.r[4].s64 = ctx.r[10].s64 + 11624;
	// 826CC440: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826CC444: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826CC448: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826CC44C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826CC450: 386A863C  addi r3, r10, -0x79c4
	ctx.r[3].s64 = ctx.r[10].s64 + -31172;
	// 826CC454: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826CC458: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826CC45C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826CC460: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826CC464: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826CC468: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826CC46C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826CC470: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826CC474: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826CC478: 4BD9A9A9  bl 0x82466e20
	ctx.lr = 0x826CC47C;
	sub_82466E20(ctx, base);
	// 826CC47C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826CC480: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826CC484: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826CC488: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826CC490(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826CC490 size=108
    let mut pc: u32 = 0x826CC490;
    'dispatch: loop {
        match pc {
            0x826CC490 => {
    //   block [0x826CC490..0x826CC4FC)
	// 826CC490: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826CC494: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826CC498: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826CC49C: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826CC4A0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826CC4A4: 38EBAF60  addi r7, r11, -0x50a0
	ctx.r[7].s64 = ctx.r[11].s64 + -20640;
	// 826CC4A8: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 826CC4AC: 388A2D70  addi r4, r10, 0x2d70
	ctx.r[4].s64 = ctx.r[10].s64 + 11632;
	// 826CC4B0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826CC4B4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826CC4B8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826CC4BC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826CC4C0: 386A866C  addi r3, r10, -0x7994
	ctx.r[3].s64 = ctx.r[10].s64 + -31124;
	// 826CC4C4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826CC4C8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826CC4CC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826CC4D0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826CC4D4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826CC4D8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826CC4DC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826CC4E0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826CC4E4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826CC4E8: 4BD9A939  bl 0x82466e20
	ctx.lr = 0x826CC4EC;
	sub_82466E20(ctx, base);
	// 826CC4EC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826CC4F0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826CC4F4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826CC4F8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826CC500(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826CC500 size=108
    let mut pc: u32 = 0x826CC500;
    'dispatch: loop {
        match pc {
            0x826CC500 => {
    //   block [0x826CC500..0x826CC56C)
	// 826CC500: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826CC504: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826CC508: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826CC50C: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826CC510: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826CC514: 38EBAFA8  addi r7, r11, -0x5058
	ctx.r[7].s64 = ctx.r[11].s64 + -20568;
	// 826CC518: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 826CC51C: 388A2D84  addi r4, r10, 0x2d84
	ctx.r[4].s64 = ctx.r[10].s64 + 11652;
	// 826CC520: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826CC524: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826CC528: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826CC52C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826CC530: 386A869C  addi r3, r10, -0x7964
	ctx.r[3].s64 = ctx.r[10].s64 + -31076;
	// 826CC534: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826CC538: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826CC53C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826CC540: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826CC544: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826CC548: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826CC54C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826CC550: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826CC554: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826CC558: 4BD9A8C9  bl 0x82466e20
	ctx.lr = 0x826CC55C;
	sub_82466E20(ctx, base);
	// 826CC55C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826CC560: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826CC564: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826CC568: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826CC570(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826CC570 size=96
    let mut pc: u32 = 0x826CC570;
    'dispatch: loop {
        match pc {
            0x826CC570 => {
    //   block [0x826CC570..0x826CC5D0)
	// 826CC570: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826CC574: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826CC578: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826CC57C: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 826CC580: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826CC584: 388A6DD4  addi r4, r10, 0x6dd4
	ctx.r[4].s64 = ctx.r[10].s64 + 28116;
	// 826CC588: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826CC58C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826CC590: 386A86CC  addi r3, r10, -0x7934
	ctx.r[3].s64 = ctx.r[10].s64 + -31028;
	// 826CC594: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826CC598: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826CC59C: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826CC5A0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826CC5A4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826CC5A8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826CC5AC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826CC5B0: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826CC5B4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826CC5B8: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826CC5BC: 4BD9A865  bl 0x82466e20
	ctx.lr = 0x826CC5C0;
	sub_82466E20(ctx, base);
	// 826CC5C0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826CC5C4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826CC5C8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826CC5CC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826CC5D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826CC5D0 size=112
    let mut pc: u32 = 0x826CC5D0;
    'dispatch: loop {
        match pc {
            0x826CC5D0 => {
    //   block [0x826CC5D0..0x826CC640)
	// 826CC5D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826CC5D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826CC5D8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826CC5DC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826CC5E0: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826CC5E4: 38AA86CC  addi r5, r10, -0x7934
	ctx.r[5].s64 = ctx.r[10].s64 + -31028;
	// 826CC5E8: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 826CC5EC: 390BB008  addi r8, r11, -0x4ff8
	ctx.r[8].s64 = ctx.r[11].s64 + -20472;
	// 826CC5F0: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826CC5F4: 388A7380  addi r4, r10, 0x7380
	ctx.r[4].s64 = ctx.r[10].s64 + 29568;
	// 826CC5F8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826CC5FC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826CC600: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826CC604: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826CC608: 386A86FC  addi r3, r10, -0x7904
	ctx.r[3].s64 = ctx.r[10].s64 + -30980;
	// 826CC60C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826CC610: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826CC614: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826CC618: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826CC61C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826CC620: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826CC624: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826CC628: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826CC62C: 4BD9A7F5  bl 0x82466e20
	ctx.lr = 0x826CC630;
	sub_82466E20(ctx, base);
	// 826CC630: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826CC634: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826CC638: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826CC63C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826CC640(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826CC640 size=112
    let mut pc: u32 = 0x826CC640;
    'dispatch: loop {
        match pc {
            0x826CC640 => {
    //   block [0x826CC640..0x826CC6B0)
	// 826CC640: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826CC644: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826CC648: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826CC64C: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 826CC650: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826CC654: 392A32EC  addi r9, r10, 0x32ec
	ctx.r[9].s64 = ctx.r[10].s64 + 13036;
	// 826CC658: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 826CC65C: 390BB038  addi r8, r11, -0x4fc8
	ctx.r[8].s64 = ctx.r[11].s64 + -20424;
	// 826CC660: 38E00007  li r7, 7
	ctx.r[7].s64 = 7;
	// 826CC664: 388A7468  addi r4, r10, 0x7468
	ctx.r[4].s64 = ctx.r[10].s64 + 29800;
	// 826CC668: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826CC66C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826CC670: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826CC674: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826CC678: 386A872C  addi r3, r10, -0x78d4
	ctx.r[3].s64 = ctx.r[10].s64 + -30932;
	// 826CC67C: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826CC680: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826CC684: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826CC688: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826CC68C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826CC690: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826CC694: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826CC698: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826CC69C: 4BD9A785  bl 0x82466e20
	ctx.lr = 0x826CC6A0;
	sub_82466E20(ctx, base);
	// 826CC6A0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826CC6A4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826CC6A8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826CC6AC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826CC6B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826CC6B0 size=108
    let mut pc: u32 = 0x826CC6B0;
    'dispatch: loop {
        match pc {
            0x826CC6B0 => {
    //   block [0x826CC6B0..0x826CC71C)
	// 826CC6B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826CC6B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826CC6B8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826CC6BC: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826CC6C0: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 826CC6C4: 38EBB0E0  addi r7, r11, -0x4f20
	ctx.r[7].s64 = ctx.r[11].s64 + -20256;
	// 826CC6C8: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826CC6CC: 388A7888  addi r4, r10, 0x7888
	ctx.r[4].s64 = ctx.r[10].s64 + 30856;
	// 826CC6D0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826CC6D4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826CC6D8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826CC6DC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826CC6E0: 386A875C  addi r3, r10, -0x78a4
	ctx.r[3].s64 = ctx.r[10].s64 + -30884;
	// 826CC6E4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826CC6E8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826CC6EC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826CC6F0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826CC6F4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826CC6F8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826CC6FC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826CC700: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826CC704: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826CC708: 4BD9A719  bl 0x82466e20
	ctx.lr = 0x826CC70C;
	sub_82466E20(ctx, base);
	// 826CC70C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826CC710: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826CC714: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826CC718: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826CC720(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826CC720 size=108
    let mut pc: u32 = 0x826CC720;
    'dispatch: loop {
        match pc {
            0x826CC720 => {
    //   block [0x826CC720..0x826CC78C)
	// 826CC720: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826CC724: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826CC728: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826CC72C: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826CC730: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 826CC734: 38EBB110  addi r7, r11, -0x4ef0
	ctx.r[7].s64 = ctx.r[11].s64 + -20208;
	// 826CC738: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826CC73C: 388A7898  addi r4, r10, 0x7898
	ctx.r[4].s64 = ctx.r[10].s64 + 30872;
	// 826CC740: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826CC744: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826CC748: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826CC74C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826CC750: 386A878C  addi r3, r10, -0x7874
	ctx.r[3].s64 = ctx.r[10].s64 + -30836;
	// 826CC754: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826CC758: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826CC75C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826CC760: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826CC764: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826CC768: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826CC76C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826CC770: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826CC774: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826CC778: 4BD9A6A9  bl 0x82466e20
	ctx.lr = 0x826CC77C;
	sub_82466E20(ctx, base);
	// 826CC77C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826CC780: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826CC784: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826CC788: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826CC790(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826CC790 size=28
    let mut pc: u32 = 0x826CC790;
    'dispatch: loop {
        match pc {
            0x826CC790 => {
    //   block [0x826CC790..0x826CC7AC)
	// 826CC790: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826CC794: 3D40827F  lis r10, -0x7d81
	ctx.r[10].s64 = -2105606144;
	// 826CC798: 394AF2D0  addi r10, r10, -0xd30
	ctx.r[10].s64 = ctx.r[10].s64 + -3376;
	// 826CC79C: 816BB140  lwz r11, -0x4ec0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-20160 as u32) ) } as u64;
	// 826CC7A0: 916A0050  stw r11, 0x50(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 826CC7A4: 916A0068  stw r11, 0x68(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(104 as u32), ctx.r[11].u32 ) };
	// 826CC7A8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826CC7B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826CC7B0 size=112
    let mut pc: u32 = 0x826CC7B0;
    'dispatch: loop {
        match pc {
            0x826CC7B0 => {
    //   block [0x826CC7B0..0x826CC820)
	// 826CC7B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826CC7B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826CC7B8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826CC7BC: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 826CC7C0: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826CC7C4: 392A3458  addi r9, r10, 0x3458
	ctx.r[9].s64 = ctx.r[10].s64 + 13400;
	// 826CC7C8: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 826CC7CC: 390BF2D0  addi r8, r11, -0xd30
	ctx.r[8].s64 = ctx.r[11].s64 + -3376;
	// 826CC7D0: 38E00008  li r7, 8
	ctx.r[7].s64 = 8;
	// 826CC7D4: 388A72F4  addi r4, r10, 0x72f4
	ctx.r[4].s64 = ctx.r[10].s64 + 29428;
	// 826CC7D8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826CC7DC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826CC7E0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826CC7E4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826CC7E8: 386A87BC  addi r3, r10, -0x7844
	ctx.r[3].s64 = ctx.r[10].s64 + -30788;
	// 826CC7EC: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826CC7F0: 39400003  li r10, 3
	ctx.r[10].s64 = 3;
	// 826CC7F4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826CC7F8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826CC7FC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826CC800: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826CC804: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826CC808: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826CC80C: 4BD9A615  bl 0x82466e20
	ctx.lr = 0x826CC810;
	sub_82466E20(ctx, base);
	// 826CC810: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826CC814: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826CC818: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826CC81C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826CC820(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826CC820 size=108
    let mut pc: u32 = 0x826CC820;
    'dispatch: loop {
        match pc {
            0x826CC820 => {
    //   block [0x826CC820..0x826CC88C)
	// 826CC820: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826CC824: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826CC828: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826CC82C: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826CC830: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 826CC834: 38EBB14C  addi r7, r11, -0x4eb4
	ctx.r[7].s64 = ctx.r[11].s64 + -20148;
	// 826CC838: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826CC83C: 388A7788  addi r4, r10, 0x7788
	ctx.r[4].s64 = ctx.r[10].s64 + 30600;
	// 826CC840: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826CC844: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826CC848: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826CC84C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826CC850: 386A87EC  addi r3, r10, -0x7814
	ctx.r[3].s64 = ctx.r[10].s64 + -30740;
	// 826CC854: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826CC858: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826CC85C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826CC860: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826CC864: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826CC868: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826CC86C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826CC870: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826CC874: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826CC878: 4BD9A5A9  bl 0x82466e20
	ctx.lr = 0x826CC87C;
	sub_82466E20(ctx, base);
	// 826CC87C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826CC880: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826CC884: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826CC888: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826CC890(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826CC890 size=108
    let mut pc: u32 = 0x826CC890;
    'dispatch: loop {
        match pc {
            0x826CC890 => {
    //   block [0x826CC890..0x826CC8FC)
	// 826CC890: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826CC894: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826CC898: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826CC89C: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826CC8A0: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 826CC8A4: 38EBB17C  addi r7, r11, -0x4e84
	ctx.r[7].s64 = ctx.r[11].s64 + -20100;
	// 826CC8A8: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826CC8AC: 388A77AC  addi r4, r10, 0x77ac
	ctx.r[4].s64 = ctx.r[10].s64 + 30636;
	// 826CC8B0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826CC8B4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826CC8B8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826CC8BC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826CC8C0: 386A881C  addi r3, r10, -0x77e4
	ctx.r[3].s64 = ctx.r[10].s64 + -30692;
	// 826CC8C4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826CC8C8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826CC8CC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826CC8D0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826CC8D4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826CC8D8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826CC8DC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826CC8E0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826CC8E4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826CC8E8: 4BD9A539  bl 0x82466e20
	ctx.lr = 0x826CC8EC;
	sub_82466E20(ctx, base);
	// 826CC8EC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826CC8F0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826CC8F4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826CC8F8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826CC900(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826CC900 size=24
    let mut pc: u32 = 0x826CC900;
    'dispatch: loop {
        match pc {
            0x826CC900 => {
    //   block [0x826CC900..0x826CC918)
	// 826CC900: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826CC904: 3D40827F  lis r10, -0x7d81
	ctx.r[10].s64 = -2105606144;
	// 826CC908: 394AF390  addi r10, r10, -0xc70
	ctx.r[10].s64 = ctx.r[10].s64 + -3184;
	// 826CC90C: 816BB194  lwz r11, -0x4e6c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-20076 as u32) ) } as u64;
	// 826CC910: 916A0050  stw r11, 0x50(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 826CC914: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826CC918(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826CC918 size=112
    let mut pc: u32 = 0x826CC918;
    'dispatch: loop {
        match pc {
            0x826CC918 => {
    //   block [0x826CC918..0x826CC988)
	// 826CC918: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826CC91C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826CC920: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826CC924: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 826CC928: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826CC92C: 392A34AC  addi r9, r10, 0x34ac
	ctx.r[9].s64 = ctx.r[10].s64 + 13484;
	// 826CC930: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 826CC934: 390BF390  addi r8, r11, -0xc70
	ctx.r[8].s64 = ctx.r[11].s64 + -3184;
	// 826CC938: 38E00006  li r7, 6
	ctx.r[7].s64 = 6;
	// 826CC93C: 388A77C8  addi r4, r10, 0x77c8
	ctx.r[4].s64 = ctx.r[10].s64 + 30664;
	// 826CC940: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826CC944: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826CC948: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826CC94C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826CC950: 386A884C  addi r3, r10, -0x77b4
	ctx.r[3].s64 = ctx.r[10].s64 + -30644;
	// 826CC954: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826CC958: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826CC95C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826CC960: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826CC964: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826CC968: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826CC96C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826CC970: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826CC974: 4BD9A4AD  bl 0x82466e20
	ctx.lr = 0x826CC978;
	sub_82466E20(ctx, base);
	// 826CC978: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826CC97C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826CC980: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826CC984: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826CC988(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826CC988 size=108
    let mut pc: u32 = 0x826CC988;
    'dispatch: loop {
        match pc {
            0x826CC988 => {
    //   block [0x826CC988..0x826CC9F4)
	// 826CC988: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826CC98C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826CC990: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826CC994: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826CC998: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 826CC99C: 38EBB198  addi r7, r11, -0x4e68
	ctx.r[7].s64 = ctx.r[11].s64 + -20072;
	// 826CC9A0: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826CC9A4: 388A7BB0  addi r4, r10, 0x7bb0
	ctx.r[4].s64 = ctx.r[10].s64 + 31664;
	// 826CC9A8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826CC9AC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826CC9B0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826CC9B4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826CC9B8: 386A887C  addi r3, r10, -0x7784
	ctx.r[3].s64 = ctx.r[10].s64 + -30596;
	// 826CC9BC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826CC9C0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826CC9C4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826CC9C8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826CC9CC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826CC9D0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826CC9D4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826CC9D8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826CC9DC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826CC9E0: 4BD9A441  bl 0x82466e20
	ctx.lr = 0x826CC9E4;
	sub_82466E20(ctx, base);
	// 826CC9E4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826CC9E8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826CC9EC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826CC9F0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826CC9F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826CC9F8 size=108
    let mut pc: u32 = 0x826CC9F8;
    'dispatch: loop {
        match pc {
            0x826CC9F8 => {
    //   block [0x826CC9F8..0x826CCA64)
	// 826CC9F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826CC9FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826CCA00: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826CCA04: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826CCA08: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 826CCA0C: 38EBB1C8  addi r7, r11, -0x4e38
	ctx.r[7].s64 = ctx.r[11].s64 + -20024;
	// 826CCA10: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826CCA14: 388A7F88  addi r4, r10, 0x7f88
	ctx.r[4].s64 = ctx.r[10].s64 + 32648;
	// 826CCA18: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826CCA1C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826CCA20: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826CCA24: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826CCA28: 386A88AC  addi r3, r10, -0x7754
	ctx.r[3].s64 = ctx.r[10].s64 + -30548;
	// 826CCA2C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826CCA30: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826CCA34: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826CCA38: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826CCA3C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826CCA40: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826CCA44: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826CCA48: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826CCA4C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826CCA50: 4BD9A3D1  bl 0x82466e20
	ctx.lr = 0x826CCA54;
	sub_82466E20(ctx, base);
	// 826CCA54: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826CCA58: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826CCA5C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826CCA60: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826CCA68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826CCA68 size=112
    let mut pc: u32 = 0x826CCA68;
    'dispatch: loop {
        match pc {
            0x826CCA68 => {
    //   block [0x826CCA68..0x826CCAD8)
	// 826CCA68: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826CCA6C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826CCA70: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826CCA74: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 826CCA78: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826CCA7C: 392A34D0  addi r9, r10, 0x34d0
	ctx.r[9].s64 = ctx.r[10].s64 + 13520;
	// 826CCA80: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826CCA84: 390BB200  addi r8, r11, -0x4e00
	ctx.r[8].s64 = ctx.r[11].s64 + -19968;
	// 826CCA88: 38E00004  li r7, 4
	ctx.r[7].s64 = 4;
	// 826CCA8C: 388A82E8  addi r4, r10, -0x7d18
	ctx.r[4].s64 = ctx.r[10].s64 + -32024;
	// 826CCA90: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826CCA94: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826CCA98: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826CCA9C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826CCAA0: 386A88DC  addi r3, r10, -0x7724
	ctx.r[3].s64 = ctx.r[10].s64 + -30500;
	// 826CCAA4: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826CCAA8: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826CCAAC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826CCAB0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826CCAB4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826CCAB8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826CCABC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826CCAC0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826CCAC4: 4BD9A35D  bl 0x82466e20
	ctx.lr = 0x826CCAC8;
	sub_82466E20(ctx, base);
	// 826CCAC8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826CCACC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826CCAD0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826CCAD4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826CCAD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826CCAD8 size=108
    let mut pc: u32 = 0x826CCAD8;
    'dispatch: loop {
        match pc {
            0x826CCAD8 => {
    //   block [0x826CCAD8..0x826CCB44)
	// 826CCAD8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826CCADC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826CCAE0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826CCAE4: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826CCAE8: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 826CCAEC: 38EBB260  addi r7, r11, -0x4da0
	ctx.r[7].s64 = ctx.r[11].s64 + -19872;
	// 826CCAF0: 39000008  li r8, 8
	ctx.r[8].s64 = 8;
	// 826CCAF4: 388A7F28  addi r4, r10, 0x7f28
	ctx.r[4].s64 = ctx.r[10].s64 + 32552;
	// 826CCAF8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826CCAFC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826CCB00: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826CCB04: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826CCB08: 386A890C  addi r3, r10, -0x76f4
	ctx.r[3].s64 = ctx.r[10].s64 + -30452;
	// 826CCB0C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826CCB10: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826CCB14: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826CCB18: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826CCB1C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826CCB20: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826CCB24: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826CCB28: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826CCB2C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826CCB30: 4BD9A2F1  bl 0x82466e20
	ctx.lr = 0x826CCB34;
	sub_82466E20(ctx, base);
	// 826CCB34: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826CCB38: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826CCB3C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826CCB40: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826CCB48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826CCB48 size=108
    let mut pc: u32 = 0x826CCB48;
    'dispatch: loop {
        match pc {
            0x826CCB48 => {
    //   block [0x826CCB48..0x826CCBB4)
	// 826CCB48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826CCB4C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826CCB50: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826CCB54: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826CCB58: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 826CCB5C: 38EBB320  addi r7, r11, -0x4ce0
	ctx.r[7].s64 = ctx.r[11].s64 + -19680;
	// 826CCB60: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826CCB64: 388A78C0  addi r4, r10, 0x78c0
	ctx.r[4].s64 = ctx.r[10].s64 + 30912;
	// 826CCB68: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826CCB6C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826CCB70: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826CCB74: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826CCB78: 386A893C  addi r3, r10, -0x76c4
	ctx.r[3].s64 = ctx.r[10].s64 + -30404;
	// 826CCB7C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826CCB80: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826CCB84: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826CCB88: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826CCB8C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826CCB90: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826CCB94: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826CCB98: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826CCB9C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826CCBA0: 4BD9A281  bl 0x82466e20
	ctx.lr = 0x826CCBA4;
	sub_82466E20(ctx, base);
	// 826CCBA4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826CCBA8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826CCBAC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826CCBB0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826CCBB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826CCBB8 size=108
    let mut pc: u32 = 0x826CCBB8;
    'dispatch: loop {
        match pc {
            0x826CCBB8 => {
    //   block [0x826CCBB8..0x826CCC24)
	// 826CCBB8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826CCBBC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826CCBC0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826CCBC4: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826CCBC8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826CCBCC: 38EBB338  addi r7, r11, -0x4cc8
	ctx.r[7].s64 = ctx.r[11].s64 + -19656;
	// 826CCBD0: 39000005  li r8, 5
	ctx.r[8].s64 = 5;
	// 826CCBD4: 388A8060  addi r4, r10, -0x7fa0
	ctx.r[4].s64 = ctx.r[10].s64 + -32672;
	// 826CCBD8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826CCBDC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826CCBE0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826CCBE4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826CCBE8: 386A896C  addi r3, r10, -0x7694
	ctx.r[3].s64 = ctx.r[10].s64 + -30356;
	// 826CCBEC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826CCBF0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826CCBF4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826CCBF8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826CCBFC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826CCC00: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826CCC04: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826CCC08: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826CCC0C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826CCC10: 4BD9A211  bl 0x82466e20
	ctx.lr = 0x826CCC14;
	sub_82466E20(ctx, base);
	// 826CCC14: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826CCC18: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826CCC1C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826CCC20: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826CCC28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826CCC28 size=24
    let mut pc: u32 = 0x826CCC28;
    'dispatch: loop {
        match pc {
            0x826CCC28 => {
    //   block [0x826CCC28..0x826CCC40)
	// 826CCC28: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826CCC2C: 3D40827F  lis r10, -0x7d81
	ctx.r[10].s64 = -2105606144;
	// 826CCC30: 394AF420  addi r10, r10, -0xbe0
	ctx.r[10].s64 = ctx.r[10].s64 + -3040;
	// 826CCC34: 816BB1FC  lwz r11, -0x4e04(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-19972 as u32) ) } as u64;
	// 826CCC38: 916A0020  stw r11, 0x20(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 826CCC3C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826CCC40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826CCC40 size=108
    let mut pc: u32 = 0x826CCC40;
    'dispatch: loop {
        match pc {
            0x826CCC40 => {
    //   block [0x826CCC40..0x826CCCAC)
	// 826CCC40: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826CCC44: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826CCC48: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826CCC4C: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826CCC50: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826CCC54: 38EBF420  addi r7, r11, -0xbe0
	ctx.r[7].s64 = ctx.r[11].s64 + -3040;
	// 826CCC58: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826CCC5C: 388AB020  addi r4, r10, -0x4fe0
	ctx.r[4].s64 = ctx.r[10].s64 + -20448;
	// 826CCC60: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826CCC64: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826CCC68: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826CCC6C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826CCC70: 386A899C  addi r3, r10, -0x7664
	ctx.r[3].s64 = ctx.r[10].s64 + -30308;
	// 826CCC74: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826CCC78: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826CCC7C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826CCC80: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826CCC84: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826CCC88: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826CCC8C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826CCC90: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826CCC94: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826CCC98: 4BD9A189  bl 0x82466e20
	ctx.lr = 0x826CCC9C;
	sub_82466E20(ctx, base);
	// 826CCC9C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826CCCA0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826CCCA4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826CCCA8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826CCCB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826CCCB0 size=24
    let mut pc: u32 = 0x826CCCB0;
    'dispatch: loop {
        match pc {
            0x826CCCB0 => {
    //   block [0x826CCCB0..0x826CCCC8)
	// 826CCCB0: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826CCCB4: 3D40827F  lis r10, -0x7d81
	ctx.r[10].s64 = -2105606144;
	// 826CCCB8: 394AF450  addi r10, r10, -0xbb0
	ctx.r[10].s64 = ctx.r[10].s64 + -2992;
	// 826CCCBC: 816BB1FC  lwz r11, -0x4e04(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-19972 as u32) ) } as u64;
	// 826CCCC0: 916A0020  stw r11, 0x20(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 826CCCC4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826CCCC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826CCCC8 size=108
    let mut pc: u32 = 0x826CCCC8;
    'dispatch: loop {
        match pc {
            0x826CCCC8 => {
    //   block [0x826CCCC8..0x826CCD34)
	// 826CCCC8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826CCCCC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826CCCD0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826CCCD4: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826CCCD8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826CCCDC: 38EBF450  addi r7, r11, -0xbb0
	ctx.r[7].s64 = ctx.r[11].s64 + -2992;
	// 826CCCE0: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826CCCE4: 388A9C48  addi r4, r10, -0x63b8
	ctx.r[4].s64 = ctx.r[10].s64 + -25528;
	// 826CCCE8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826CCCEC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826CCCF0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826CCCF4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826CCCF8: 386A89CC  addi r3, r10, -0x7634
	ctx.r[3].s64 = ctx.r[10].s64 + -30260;
	// 826CCCFC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826CCD00: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826CCD04: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826CCD08: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826CCD0C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826CCD10: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826CCD14: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826CCD18: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826CCD1C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826CCD20: 4BD9A101  bl 0x82466e20
	ctx.lr = 0x826CCD24;
	sub_82466E20(ctx, base);
	// 826CCD24: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826CCD28: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826CCD2C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826CCD30: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826CCD38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826CCD38 size=108
    let mut pc: u32 = 0x826CCD38;
    'dispatch: loop {
        match pc {
            0x826CCD38 => {
    //   block [0x826CCD38..0x826CCDA4)
	// 826CCD38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826CCD3C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826CCD40: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826CCD44: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826CCD48: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826CCD4C: 38EBB3B0  addi r7, r11, -0x4c50
	ctx.r[7].s64 = ctx.r[11].s64 + -19536;
	// 826CCD50: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826CCD54: 388AA080  addi r4, r10, -0x5f80
	ctx.r[4].s64 = ctx.r[10].s64 + -24448;
	// 826CCD58: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826CCD5C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826CCD60: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826CCD64: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826CCD68: 386A89FC  addi r3, r10, -0x7604
	ctx.r[3].s64 = ctx.r[10].s64 + -30212;
	// 826CCD6C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826CCD70: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826CCD74: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826CCD78: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826CCD7C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826CCD80: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826CCD84: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826CCD88: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826CCD8C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826CCD90: 4BD9A091  bl 0x82466e20
	ctx.lr = 0x826CCD94;
	sub_82466E20(ctx, base);
	// 826CCD94: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826CCD98: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826CCD9C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826CCDA0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826CCDA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826CCDA8 size=24
    let mut pc: u32 = 0x826CCDA8;
    'dispatch: loop {
        match pc {
            0x826CCDA8 => {
    //   block [0x826CCDA8..0x826CCDC0)
	// 826CCDA8: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826CCDAC: 3D40827F  lis r10, -0x7d81
	ctx.r[10].s64 = -2105606144;
	// 826CCDB0: 394AF480  addi r10, r10, -0xb80
	ctx.r[10].s64 = ctx.r[10].s64 + -2944;
	// 826CCDB4: 816BB1FC  lwz r11, -0x4e04(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-19972 as u32) ) } as u64;
	// 826CCDB8: 916A0020  stw r11, 0x20(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 826CCDBC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826CCDC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826CCDC0 size=108
    let mut pc: u32 = 0x826CCDC0;
    'dispatch: loop {
        match pc {
            0x826CCDC0 => {
    //   block [0x826CCDC0..0x826CCE2C)
	// 826CCDC0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826CCDC4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826CCDC8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826CCDCC: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826CCDD0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826CCDD4: 38EBF480  addi r7, r11, -0xb80
	ctx.r[7].s64 = ctx.r[11].s64 + -2944;
	// 826CCDD8: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826CCDDC: 388A9BE8  addi r4, r10, -0x6418
	ctx.r[4].s64 = ctx.r[10].s64 + -25624;
	// 826CCDE0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826CCDE4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826CCDE8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826CCDEC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826CCDF0: 386A8A2C  addi r3, r10, -0x75d4
	ctx.r[3].s64 = ctx.r[10].s64 + -30164;
	// 826CCDF4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826CCDF8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826CCDFC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826CCE00: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826CCE04: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826CCE08: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826CCE0C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826CCE10: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826CCE14: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826CCE18: 4BD9A009  bl 0x82466e20
	ctx.lr = 0x826CCE1C;
	sub_82466E20(ctx, base);
	// 826CCE1C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826CCE20: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826CCE24: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826CCE28: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826CCE30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826CCE30 size=108
    let mut pc: u32 = 0x826CCE30;
    'dispatch: loop {
        match pc {
            0x826CCE30 => {
    //   block [0x826CCE30..0x826CCE9C)
	// 826CCE30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826CCE34: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826CCE38: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826CCE3C: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826CCE40: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826CCE44: 38EBB3C8  addi r7, r11, -0x4c38
	ctx.r[7].s64 = ctx.r[11].s64 + -19512;
	// 826CCE48: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826CCE4C: 388AA778  addi r4, r10, -0x5888
	ctx.r[4].s64 = ctx.r[10].s64 + -22664;
	// 826CCE50: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826CCE54: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826CCE58: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826CCE5C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826CCE60: 386A8A5C  addi r3, r10, -0x75a4
	ctx.r[3].s64 = ctx.r[10].s64 + -30116;
	// 826CCE64: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826CCE68: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826CCE6C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826CCE70: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826CCE74: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826CCE78: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826CCE7C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826CCE80: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826CCE84: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826CCE88: 4BD99F99  bl 0x82466e20
	ctx.lr = 0x826CCE8C;
	sub_82466E20(ctx, base);
	// 826CCE8C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826CCE90: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826CCE94: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826CCE98: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826CCEA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826CCEA0 size=108
    let mut pc: u32 = 0x826CCEA0;
    'dispatch: loop {
        match pc {
            0x826CCEA0 => {
    //   block [0x826CCEA0..0x826CCF0C)
	// 826CCEA0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826CCEA4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826CCEA8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826CCEAC: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826CCEB0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826CCEB4: 38EBB3F8  addi r7, r11, -0x4c08
	ctx.r[7].s64 = ctx.r[11].s64 + -19464;
	// 826CCEB8: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826CCEBC: 388AA350  addi r4, r10, -0x5cb0
	ctx.r[4].s64 = ctx.r[10].s64 + -23728;
	// 826CCEC0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826CCEC4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826CCEC8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826CCECC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826CCED0: 386A8A8C  addi r3, r10, -0x7574
	ctx.r[3].s64 = ctx.r[10].s64 + -30068;
	// 826CCED4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826CCED8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826CCEDC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826CCEE0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826CCEE4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826CCEE8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826CCEEC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826CCEF0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826CCEF4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826CCEF8: 4BD99F29  bl 0x82466e20
	ctx.lr = 0x826CCEFC;
	sub_82466E20(ctx, base);
	// 826CCEFC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826CCF00: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826CCF04: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826CCF08: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826CCF10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826CCF10 size=108
    let mut pc: u32 = 0x826CCF10;
    'dispatch: loop {
        match pc {
            0x826CCF10 => {
    //   block [0x826CCF10..0x826CCF7C)
	// 826CCF10: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826CCF14: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826CCF18: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826CCF1C: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826CCF20: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826CCF24: 38EBB428  addi r7, r11, -0x4bd8
	ctx.r[7].s64 = ctx.r[11].s64 + -19416;
	// 826CCF28: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826CCF2C: 388AAA44  addi r4, r10, -0x55bc
	ctx.r[4].s64 = ctx.r[10].s64 + -21948;
	// 826CCF30: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826CCF34: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826CCF38: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826CCF3C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826CCF40: 386A8ABC  addi r3, r10, -0x7544
	ctx.r[3].s64 = ctx.r[10].s64 + -30020;
	// 826CCF44: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826CCF48: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826CCF4C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826CCF50: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826CCF54: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826CCF58: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826CCF5C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826CCF60: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826CCF64: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826CCF68: 4BD99EB9  bl 0x82466e20
	ctx.lr = 0x826CCF6C;
	sub_82466E20(ctx, base);
	// 826CCF6C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826CCF70: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826CCF74: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826CCF78: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826CCF80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826CCF80 size=112
    let mut pc: u32 = 0x826CCF80;
    'dispatch: loop {
        match pc {
            0x826CCF80 => {
    //   block [0x826CCF80..0x826CCFF0)
	// 826CCF80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826CCF84: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826CCF88: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826CCF8C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826CCF90: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826CCF94: 38AA8B1C  addi r5, r10, -0x74e4
	ctx.r[5].s64 = ctx.r[10].s64 + -29924;
	// 826CCF98: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826CCF9C: 390BB458  addi r8, r11, -0x4ba8
	ctx.r[8].s64 = ctx.r[11].s64 + -19368;
	// 826CCFA0: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826CCFA4: 388AA9A0  addi r4, r10, -0x5660
	ctx.r[4].s64 = ctx.r[10].s64 + -22112;
	// 826CCFA8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826CCFAC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826CCFB0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826CCFB4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826CCFB8: 386A8AEC  addi r3, r10, -0x7514
	ctx.r[3].s64 = ctx.r[10].s64 + -29972;
	// 826CCFBC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826CCFC0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826CCFC4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826CCFC8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826CCFCC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826CCFD0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826CCFD4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826CCFD8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826CCFDC: 4BD99E45  bl 0x82466e20
	ctx.lr = 0x826CCFE0;
	sub_82466E20(ctx, base);
	// 826CCFE0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826CCFE4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826CCFE8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826CCFEC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826CCFF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826CCFF0 size=108
    let mut pc: u32 = 0x826CCFF0;
    'dispatch: loop {
        match pc {
            0x826CCFF0 => {
    //   block [0x826CCFF0..0x826CD05C)
	// 826CCFF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826CCFF4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826CCFF8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826CCFFC: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826CD000: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826CD004: 38EBB470  addi r7, r11, -0x4b90
	ctx.r[7].s64 = ctx.r[11].s64 + -19344;
	// 826CD008: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826CD00C: 388AA640  addi r4, r10, -0x59c0
	ctx.r[4].s64 = ctx.r[10].s64 + -22976;
	// 826CD010: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826CD014: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826CD018: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826CD01C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826CD020: 386A8B1C  addi r3, r10, -0x74e4
	ctx.r[3].s64 = ctx.r[10].s64 + -29924;
	// 826CD024: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826CD028: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826CD02C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826CD030: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826CD034: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826CD038: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826CD03C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826CD040: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826CD044: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826CD048: 4BD99DD9  bl 0x82466e20
	ctx.lr = 0x826CD04C;
	sub_82466E20(ctx, base);
	// 826CD04C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826CD050: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826CD054: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826CD058: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826CD060(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826CD060 size=108
    let mut pc: u32 = 0x826CD060;
    'dispatch: loop {
        match pc {
            0x826CD060 => {
    //   block [0x826CD060..0x826CD0CC)
	// 826CD060: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826CD064: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826CD068: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826CD06C: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826CD070: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826CD074: 38EBB4A0  addi r7, r11, -0x4b60
	ctx.r[7].s64 = ctx.r[11].s64 + -19296;
	// 826CD078: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826CD07C: 388AA2E0  addi r4, r10, -0x5d20
	ctx.r[4].s64 = ctx.r[10].s64 + -23840;
	// 826CD080: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826CD084: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826CD088: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826CD08C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826CD090: 386A8B4C  addi r3, r10, -0x74b4
	ctx.r[3].s64 = ctx.r[10].s64 + -29876;
	// 826CD094: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826CD098: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826CD09C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826CD0A0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826CD0A4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826CD0A8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826CD0AC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826CD0B0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826CD0B4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826CD0B8: 4BD99D69  bl 0x82466e20
	ctx.lr = 0x826CD0BC;
	sub_82466E20(ctx, base);
	// 826CD0BC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826CD0C0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826CD0C4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826CD0C8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826CD0D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826CD0D0 size=108
    let mut pc: u32 = 0x826CD0D0;
    'dispatch: loop {
        match pc {
            0x826CD0D0 => {
    //   block [0x826CD0D0..0x826CD13C)
	// 826CD0D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826CD0D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826CD0D8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826CD0DC: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826CD0E0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826CD0E4: 38EBB4B8  addi r7, r11, -0x4b48
	ctx.r[7].s64 = ctx.r[11].s64 + -19272;
	// 826CD0E8: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826CD0EC: 388AA304  addi r4, r10, -0x5cfc
	ctx.r[4].s64 = ctx.r[10].s64 + -23804;
	// 826CD0F0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826CD0F4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826CD0F8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826CD0FC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826CD100: 386A8B7C  addi r3, r10, -0x7484
	ctx.r[3].s64 = ctx.r[10].s64 + -29828;
	// 826CD104: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826CD108: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826CD10C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826CD110: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826CD114: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826CD118: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826CD11C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826CD120: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826CD124: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826CD128: 4BD99CF9  bl 0x82466e20
	ctx.lr = 0x826CD12C;
	sub_82466E20(ctx, base);
	// 826CD12C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826CD130: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826CD134: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826CD138: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826CD140(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826CD140 size=108
    let mut pc: u32 = 0x826CD140;
    'dispatch: loop {
        match pc {
            0x826CD140 => {
    //   block [0x826CD140..0x826CD1AC)
	// 826CD140: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826CD144: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826CD148: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826CD14C: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826CD150: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826CD154: 38EBB4E8  addi r7, r11, -0x4b18
	ctx.r[7].s64 = ctx.r[11].s64 + -19224;
	// 826CD158: 39000007  li r8, 7
	ctx.r[8].s64 = 7;
	// 826CD15C: 388AAED0  addi r4, r10, -0x5130
	ctx.r[4].s64 = ctx.r[10].s64 + -20784;
	// 826CD160: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826CD164: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826CD168: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826CD16C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826CD170: 386A8BAC  addi r3, r10, -0x7454
	ctx.r[3].s64 = ctx.r[10].s64 + -29780;
	// 826CD174: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826CD178: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826CD17C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826CD180: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826CD184: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826CD188: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826CD18C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826CD190: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826CD194: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826CD198: 4BD99C89  bl 0x82466e20
	ctx.lr = 0x826CD19C;
	sub_82466E20(ctx, base);
	// 826CD19C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826CD1A0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826CD1A4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826CD1A8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826CD1B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826CD1B0 size=108
    let mut pc: u32 = 0x826CD1B0;
    'dispatch: loop {
        match pc {
            0x826CD1B0 => {
    //   block [0x826CD1B0..0x826CD21C)
	// 826CD1B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826CD1B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826CD1B8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826CD1BC: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826CD1C0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826CD1C4: 38EBB590  addi r7, r11, -0x4a70
	ctx.r[7].s64 = ctx.r[11].s64 + -19056;
	// 826CD1C8: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826CD1CC: 388AB290  addi r4, r10, -0x4d70
	ctx.r[4].s64 = ctx.r[10].s64 + -19824;
	// 826CD1D0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826CD1D4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826CD1D8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826CD1DC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826CD1E0: 386A8BDC  addi r3, r10, -0x7424
	ctx.r[3].s64 = ctx.r[10].s64 + -29732;
	// 826CD1E4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826CD1E8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826CD1EC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826CD1F0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826CD1F4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826CD1F8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826CD1FC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826CD200: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826CD204: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826CD208: 4BD99C19  bl 0x82466e20
	ctx.lr = 0x826CD20C;
	sub_82466E20(ctx, base);
	// 826CD20C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826CD210: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826CD214: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826CD218: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826CD220(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826CD220 size=108
    let mut pc: u32 = 0x826CD220;
    'dispatch: loop {
        match pc {
            0x826CD220 => {
    //   block [0x826CD220..0x826CD28C)
	// 826CD220: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826CD224: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826CD228: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826CD22C: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826CD230: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826CD234: 38EBB5C0  addi r7, r11, -0x4a40
	ctx.r[7].s64 = ctx.r[11].s64 + -19008;
	// 826CD238: 39000008  li r8, 8
	ctx.r[8].s64 = 8;
	// 826CD23C: 388AB2A8  addi r4, r10, -0x4d58
	ctx.r[4].s64 = ctx.r[10].s64 + -19800;
	// 826CD240: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826CD244: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826CD248: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826CD24C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826CD250: 386A8C0C  addi r3, r10, -0x73f4
	ctx.r[3].s64 = ctx.r[10].s64 + -29684;
	// 826CD254: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826CD258: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826CD25C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826CD260: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826CD264: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826CD268: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826CD26C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826CD270: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826CD274: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826CD278: 4BD99BA9  bl 0x82466e20
	ctx.lr = 0x826CD27C;
	sub_82466E20(ctx, base);
	// 826CD27C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826CD280: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826CD284: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826CD288: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826CD290(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826CD290 size=24
    let mut pc: u32 = 0x826CD290;
    'dispatch: loop {
        match pc {
            0x826CD290 => {
    //   block [0x826CD290..0x826CD2A8)
	// 826CD290: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826CD294: 3D40827F  lis r10, -0x7d81
	ctx.r[10].s64 = -2105606144;
	// 826CD298: 394AF4B0  addi r10, r10, -0xb50
	ctx.r[10].s64 = ctx.r[10].s64 + -2896;
	// 826CD29C: 816BB680  lwz r11, -0x4980(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-18816 as u32) ) } as u64;
	// 826CD2A0: 916A0008  stw r11, 8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 826CD2A4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826CD2A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826CD2A8 size=112
    let mut pc: u32 = 0x826CD2A8;
    'dispatch: loop {
        match pc {
            0x826CD2A8 => {
    //   block [0x826CD2A8..0x826CD318)
	// 826CD2A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826CD2AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826CD2B0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826CD2B4: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 826CD2B8: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826CD2BC: 392A3540  addi r9, r10, 0x3540
	ctx.r[9].s64 = ctx.r[10].s64 + 13632;
	// 826CD2C0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826CD2C4: 390BF4B0  addi r8, r11, -0xb50
	ctx.r[8].s64 = ctx.r[11].s64 + -2896;
	// 826CD2C8: 38E00005  li r7, 5
	ctx.r[7].s64 = 5;
	// 826CD2CC: 388AB098  addi r4, r10, -0x4f68
	ctx.r[4].s64 = ctx.r[10].s64 + -20328;
	// 826CD2D0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826CD2D4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826CD2D8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826CD2DC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826CD2E0: 386A8C3C  addi r3, r10, -0x73c4
	ctx.r[3].s64 = ctx.r[10].s64 + -29636;
	// 826CD2E4: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826CD2E8: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826CD2EC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826CD2F0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826CD2F4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826CD2F8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826CD2FC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826CD300: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826CD304: 4BD99B1D  bl 0x82466e20
	ctx.lr = 0x826CD308;
	sub_82466E20(ctx, base);
	// 826CD308: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826CD30C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826CD310: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826CD314: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826CD318(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826CD318 size=108
    let mut pc: u32 = 0x826CD318;
    'dispatch: loop {
        match pc {
            0x826CD318 => {
    //   block [0x826CD318..0x826CD384)
	// 826CD318: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826CD31C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826CD320: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826CD324: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826CD328: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826CD32C: 38EBB688  addi r7, r11, -0x4978
	ctx.r[7].s64 = ctx.r[11].s64 + -18808;
	// 826CD330: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826CD334: 388AAC70  addi r4, r10, -0x5390
	ctx.r[4].s64 = ctx.r[10].s64 + -21392;
	// 826CD338: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826CD33C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826CD340: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826CD344: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826CD348: 386A8C6C  addi r3, r10, -0x7394
	ctx.r[3].s64 = ctx.r[10].s64 + -29588;
	// 826CD34C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826CD350: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826CD354: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826CD358: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826CD35C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826CD360: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826CD364: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826CD368: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826CD36C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826CD370: 4BD99AB1  bl 0x82466e20
	ctx.lr = 0x826CD374;
	sub_82466E20(ctx, base);
	// 826CD374: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826CD378: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826CD37C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826CD380: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826CD388(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826CD388 size=112
    let mut pc: u32 = 0x826CD388;
    'dispatch: loop {
        match pc {
            0x826CD388 => {
    //   block [0x826CD388..0x826CD3F8)
	// 826CD388: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826CD38C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826CD390: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826CD394: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 826CD398: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826CD39C: 392A3584  addi r9, r10, 0x3584
	ctx.r[9].s64 = ctx.r[10].s64 + 13700;
	// 826CD3A0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826CD3A4: 390BB6B8  addi r8, r11, -0x4948
	ctx.r[8].s64 = ctx.r[11].s64 + -18760;
	// 826CD3A8: 38E00008  li r7, 8
	ctx.r[7].s64 = 8;
	// 826CD3AC: 388AAC88  addi r4, r10, -0x5378
	ctx.r[4].s64 = ctx.r[10].s64 + -21368;
	// 826CD3B0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826CD3B4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826CD3B8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826CD3BC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826CD3C0: 386A8C9C  addi r3, r10, -0x7364
	ctx.r[3].s64 = ctx.r[10].s64 + -29540;
	// 826CD3C4: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826CD3C8: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826CD3CC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826CD3D0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826CD3D4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826CD3D8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826CD3DC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826CD3E0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826CD3E4: 4BD99A3D  bl 0x82466e20
	ctx.lr = 0x826CD3E8;
	sub_82466E20(ctx, base);
	// 826CD3E8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826CD3EC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826CD3F0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826CD3F4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826CD3F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826CD3F8 size=24
    let mut pc: u32 = 0x826CD3F8;
    'dispatch: loop {
        match pc {
            0x826CD3F8 => {
    //   block [0x826CD3F8..0x826CD410)
	// 826CD3F8: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826CD3FC: 3D40827F  lis r10, -0x7d81
	ctx.r[10].s64 = -2105606144;
	// 826CD400: 394AF528  addi r10, r10, -0xad8
	ctx.r[10].s64 = ctx.r[10].s64 + -2776;
	// 826CD404: 816BB778  lwz r11, -0x4888(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-18568 as u32) ) } as u64;
	// 826CD408: 916A0020  stw r11, 0x20(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 826CD40C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826CD410(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826CD410 size=112
    let mut pc: u32 = 0x826CD410;
    'dispatch: loop {
        match pc {
            0x826CD410 => {
    //   block [0x826CD410..0x826CD480)
	// 826CD410: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826CD414: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826CD418: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826CD41C: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 826CD420: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826CD424: 392A35C0  addi r9, r10, 0x35c0
	ctx.r[9].s64 = ctx.r[10].s64 + 13760;
	// 826CD428: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826CD42C: 390BF528  addi r8, r11, -0xad8
	ctx.r[8].s64 = ctx.r[11].s64 + -2776;
	// 826CD430: 38E00003  li r7, 3
	ctx.r[7].s64 = 3;
	// 826CD434: 388AB004  addi r4, r10, -0x4ffc
	ctx.r[4].s64 = ctx.r[10].s64 + -20476;
	// 826CD438: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826CD43C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826CD440: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826CD444: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826CD448: 386A8CCC  addi r3, r10, -0x7334
	ctx.r[3].s64 = ctx.r[10].s64 + -29492;
	// 826CD44C: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826CD450: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826CD454: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826CD458: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826CD45C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826CD460: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826CD464: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826CD468: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826CD46C: 4BD999B5  bl 0x82466e20
	ctx.lr = 0x826CD470;
	sub_82466E20(ctx, base);
	// 826CD470: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826CD474: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826CD478: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826CD47C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826CD480(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826CD480 size=108
    let mut pc: u32 = 0x826CD480;
    'dispatch: loop {
        match pc {
            0x826CD480 => {
    //   block [0x826CD480..0x826CD4EC)
	// 826CD480: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826CD484: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826CD488: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826CD48C: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826CD490: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826CD494: 38EBB77C  addi r7, r11, -0x4884
	ctx.r[7].s64 = ctx.r[11].s64 + -18564;
	// 826CD498: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826CD49C: 388AAF00  addi r4, r10, -0x5100
	ctx.r[4].s64 = ctx.r[10].s64 + -20736;
	// 826CD4A0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826CD4A4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826CD4A8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826CD4AC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826CD4B0: 386A8CFC  addi r3, r10, -0x7304
	ctx.r[3].s64 = ctx.r[10].s64 + -29444;
	// 826CD4B4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826CD4B8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826CD4BC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826CD4C0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826CD4C4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826CD4C8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826CD4CC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826CD4D0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826CD4D4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826CD4D8: 4BD99949  bl 0x82466e20
	ctx.lr = 0x826CD4DC;
	sub_82466E20(ctx, base);
	// 826CD4DC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826CD4E0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826CD4E4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826CD4E8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826CD4F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826CD4F0 size=108
    let mut pc: u32 = 0x826CD4F0;
    'dispatch: loop {
        match pc {
            0x826CD4F0 => {
    //   block [0x826CD4F0..0x826CD55C)
	// 826CD4F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826CD4F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826CD4F8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826CD4FC: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826CD500: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826CD504: 38EBB794  addi r7, r11, -0x486c
	ctx.r[7].s64 = ctx.r[11].s64 + -18540;
	// 826CD508: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826CD50C: 388A9D38  addi r4, r10, -0x62c8
	ctx.r[4].s64 = ctx.r[10].s64 + -25288;
	// 826CD510: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826CD514: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826CD518: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826CD51C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826CD520: 386A8D2C  addi r3, r10, -0x72d4
	ctx.r[3].s64 = ctx.r[10].s64 + -29396;
	// 826CD524: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826CD528: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826CD52C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826CD530: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826CD534: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826CD538: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826CD53C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826CD540: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826CD544: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826CD548: 4BD998D9  bl 0x82466e20
	ctx.lr = 0x826CD54C;
	sub_82466E20(ctx, base);
	// 826CD54C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826CD550: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826CD554: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826CD558: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826CD560(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826CD560 size=24
    let mut pc: u32 = 0x826CD560;
    'dispatch: loop {
        match pc {
            0x826CD560 => {
    //   block [0x826CD560..0x826CD578)
	// 826CD560: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826CD564: 3D40827F  lis r10, -0x7d81
	ctx.r[10].s64 = -2105606144;
	// 826CD568: 394AF570  addi r10, r10, -0xa90
	ctx.r[10].s64 = ctx.r[10].s64 + -2704;
	// 826CD56C: 816BB7C4  lwz r11, -0x483c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-18492 as u32) ) } as u64;
	// 826CD570: 916A0008  stw r11, 8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 826CD574: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826CD578(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826CD578 size=112
    let mut pc: u32 = 0x826CD578;
    'dispatch: loop {
        match pc {
            0x826CD578 => {
    //   block [0x826CD578..0x826CD5E8)
	// 826CD578: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826CD57C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826CD580: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826CD584: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 826CD588: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826CD58C: 392A35FC  addi r9, r10, 0x35fc
	ctx.r[9].s64 = ctx.r[10].s64 + 13820;
	// 826CD590: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826CD594: 390BF570  addi r8, r11, -0xa90
	ctx.r[8].s64 = ctx.r[11].s64 + -2704;
	// 826CD598: 38E00005  li r7, 5
	ctx.r[7].s64 = 5;
	// 826CD59C: 388AB450  addi r4, r10, -0x4bb0
	ctx.r[4].s64 = ctx.r[10].s64 + -19376;
	// 826CD5A0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826CD5A4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826CD5A8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826CD5AC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826CD5B0: 386A8D5C  addi r3, r10, -0x72a4
	ctx.r[3].s64 = ctx.r[10].s64 + -29348;
	// 826CD5B4: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826CD5B8: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826CD5BC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826CD5C0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826CD5C4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826CD5C8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826CD5CC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826CD5D0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826CD5D4: 4BD9984D  bl 0x82466e20
	ctx.lr = 0x826CD5D8;
	sub_82466E20(ctx, base);
	// 826CD5D8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826CD5DC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826CD5E0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826CD5E4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826CD5E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826CD5E8 size=108
    let mut pc: u32 = 0x826CD5E8;
    'dispatch: loop {
        match pc {
            0x826CD5E8 => {
    //   block [0x826CD5E8..0x826CD654)
	// 826CD5E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826CD5EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826CD5F0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826CD5F4: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826CD5F8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826CD5FC: 38EBB7C8  addi r7, r11, -0x4838
	ctx.r[7].s64 = ctx.r[11].s64 + -18488;
	// 826CD600: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826CD604: 388A9CF4  addi r4, r10, -0x630c
	ctx.r[4].s64 = ctx.r[10].s64 + -25356;
	// 826CD608: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826CD60C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826CD610: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826CD614: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826CD618: 386A8D8C  addi r3, r10, -0x7274
	ctx.r[3].s64 = ctx.r[10].s64 + -29300;
	// 826CD61C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826CD620: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826CD624: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826CD628: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826CD62C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826CD630: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826CD634: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826CD638: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826CD63C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826CD640: 4BD997E1  bl 0x82466e20
	ctx.lr = 0x826CD644;
	sub_82466E20(ctx, base);
	// 826CD644: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826CD648: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826CD64C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826CD650: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826CD658(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826CD658 size=108
    let mut pc: u32 = 0x826CD658;
    'dispatch: loop {
        match pc {
            0x826CD658 => {
    //   block [0x826CD658..0x826CD6C4)
	// 826CD658: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826CD65C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826CD660: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826CD664: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826CD668: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826CD66C: 38EBB7E0  addi r7, r11, -0x4820
	ctx.r[7].s64 = ctx.r[11].s64 + -18464;
	// 826CD670: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 826CD674: 388AB148  addi r4, r10, -0x4eb8
	ctx.r[4].s64 = ctx.r[10].s64 + -20152;
	// 826CD678: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826CD67C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826CD680: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826CD684: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826CD688: 386A8DBC  addi r3, r10, -0x7244
	ctx.r[3].s64 = ctx.r[10].s64 + -29252;
	// 826CD68C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826CD690: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826CD694: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826CD698: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826CD69C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826CD6A0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826CD6A4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826CD6A8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826CD6AC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826CD6B0: 4BD99771  bl 0x82466e20
	ctx.lr = 0x826CD6B4;
	sub_82466E20(ctx, base);
	// 826CD6B4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826CD6B8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826CD6BC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826CD6C0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826CD6C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826CD6C8 size=108
    let mut pc: u32 = 0x826CD6C8;
    'dispatch: loop {
        match pc {
            0x826CD6C8 => {
    //   block [0x826CD6C8..0x826CD734)
	// 826CD6C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826CD6CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826CD6D0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826CD6D4: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826CD6D8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826CD6DC: 38EBB828  addi r7, r11, -0x47d8
	ctx.r[7].s64 = ctx.r[11].s64 + -18392;
	// 826CD6E0: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826CD6E4: 388AA9FC  addi r4, r10, -0x5604
	ctx.r[4].s64 = ctx.r[10].s64 + -22020;
	// 826CD6E8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826CD6EC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826CD6F0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826CD6F4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826CD6F8: 386A8DEC  addi r3, r10, -0x7214
	ctx.r[3].s64 = ctx.r[10].s64 + -29204;
	// 826CD6FC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826CD700: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826CD704: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826CD708: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826CD70C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826CD710: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826CD714: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826CD718: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826CD71C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826CD720: 4BD99701  bl 0x82466e20
	ctx.lr = 0x826CD724;
	sub_82466E20(ctx, base);
	// 826CD724: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826CD728: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826CD72C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826CD730: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826CD738(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826CD738 size=108
    let mut pc: u32 = 0x826CD738;
    'dispatch: loop {
        match pc {
            0x826CD738 => {
    //   block [0x826CD738..0x826CD7A4)
	// 826CD738: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826CD73C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826CD740: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826CD744: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826CD748: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826CD74C: 38EBB858  addi r7, r11, -0x47a8
	ctx.r[7].s64 = ctx.r[11].s64 + -18344;
	// 826CD750: 3900000C  li r8, 0xc
	ctx.r[8].s64 = 12;
	// 826CD754: 388AA978  addi r4, r10, -0x5688
	ctx.r[4].s64 = ctx.r[10].s64 + -22152;
	// 826CD758: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826CD75C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826CD760: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826CD764: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826CD768: 386A8E1C  addi r3, r10, -0x71e4
	ctx.r[3].s64 = ctx.r[10].s64 + -29156;
	// 826CD76C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826CD770: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826CD774: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826CD778: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826CD77C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826CD780: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826CD784: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826CD788: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826CD78C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826CD790: 4BD99691  bl 0x82466e20
	ctx.lr = 0x826CD794;
	sub_82466E20(ctx, base);
	// 826CD794: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826CD798: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826CD79C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826CD7A0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826CD7A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826CD7A8 size=108
    let mut pc: u32 = 0x826CD7A8;
    'dispatch: loop {
        match pc {
            0x826CD7A8 => {
    //   block [0x826CD7A8..0x826CD814)
	// 826CD7A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826CD7AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826CD7B0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826CD7B4: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826CD7B8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826CD7BC: 38EBB978  addi r7, r11, -0x4688
	ctx.r[7].s64 = ctx.r[11].s64 + -18056;
	// 826CD7C0: 39000006  li r8, 6
	ctx.r[8].s64 = 6;
	// 826CD7C4: 388AB380  addi r4, r10, -0x4c80
	ctx.r[4].s64 = ctx.r[10].s64 + -19584;
	// 826CD7C8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826CD7CC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826CD7D0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826CD7D4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826CD7D8: 386A8E4C  addi r3, r10, -0x71b4
	ctx.r[3].s64 = ctx.r[10].s64 + -29108;
	// 826CD7DC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826CD7E0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826CD7E4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826CD7E8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826CD7EC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826CD7F0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826CD7F4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826CD7F8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826CD7FC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826CD800: 4BD99621  bl 0x82466e20
	ctx.lr = 0x826CD804;
	sub_82466E20(ctx, base);
	// 826CD804: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826CD808: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826CD80C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826CD810: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826CD818(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826CD818 size=108
    let mut pc: u32 = 0x826CD818;
    'dispatch: loop {
        match pc {
            0x826CD818 => {
    //   block [0x826CD818..0x826CD884)
	// 826CD818: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826CD81C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826CD820: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826CD824: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826CD828: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826CD82C: 38EBBA08  addi r7, r11, -0x45f8
	ctx.r[7].s64 = ctx.r[11].s64 + -17912;
	// 826CD830: 39000008  li r8, 8
	ctx.r[8].s64 = 8;
	// 826CD834: 388AA228  addi r4, r10, -0x5dd8
	ctx.r[4].s64 = ctx.r[10].s64 + -24024;
	// 826CD838: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826CD83C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826CD840: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826CD844: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826CD848: 386A8E7C  addi r3, r10, -0x7184
	ctx.r[3].s64 = ctx.r[10].s64 + -29060;
	// 826CD84C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826CD850: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826CD854: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826CD858: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826CD85C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826CD860: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826CD864: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826CD868: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826CD86C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826CD870: 4BD995B1  bl 0x82466e20
	ctx.lr = 0x826CD874;
	sub_82466E20(ctx, base);
	// 826CD874: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826CD878: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826CD87C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826CD880: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826CD888(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826CD888 size=108
    let mut pc: u32 = 0x826CD888;
    'dispatch: loop {
        match pc {
            0x826CD888 => {
    //   block [0x826CD888..0x826CD8F4)
	// 826CD888: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826CD88C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826CD890: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826CD894: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826CD898: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826CD89C: 38EBBAC8  addi r7, r11, -0x4538
	ctx.r[7].s64 = ctx.r[11].s64 + -17720;
	// 826CD8A0: 39000009  li r8, 9
	ctx.r[8].s64 = 9;
	// 826CD8A4: 388AA000  addi r4, r10, -0x6000
	ctx.r[4].s64 = ctx.r[10].s64 + -24576;
	// 826CD8A8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826CD8AC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826CD8B0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826CD8B4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826CD8B8: 386A8EAC  addi r3, r10, -0x7154
	ctx.r[3].s64 = ctx.r[10].s64 + -29012;
	// 826CD8BC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826CD8C0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826CD8C4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826CD8C8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826CD8CC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826CD8D0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826CD8D4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826CD8D8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826CD8DC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826CD8E0: 4BD99541  bl 0x82466e20
	ctx.lr = 0x826CD8E4;
	sub_82466E20(ctx, base);
	// 826CD8E4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826CD8E8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826CD8EC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826CD8F0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826CD8F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826CD8F8 size=108
    let mut pc: u32 = 0x826CD8F8;
    'dispatch: loop {
        match pc {
            0x826CD8F8 => {
    //   block [0x826CD8F8..0x826CD964)
	// 826CD8F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826CD8FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826CD900: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826CD904: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826CD908: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826CD90C: 38EBBBA0  addi r7, r11, -0x4460
	ctx.r[7].s64 = ctx.r[11].s64 + -17504;
	// 826CD910: 39000008  li r8, 8
	ctx.r[8].s64 = 8;
	// 826CD914: 388A9F00  addi r4, r10, -0x6100
	ctx.r[4].s64 = ctx.r[10].s64 + -24832;
	// 826CD918: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826CD91C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826CD920: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826CD924: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826CD928: 386A8EDC  addi r3, r10, -0x7124
	ctx.r[3].s64 = ctx.r[10].s64 + -28964;
	// 826CD92C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826CD930: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826CD934: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826CD938: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826CD93C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826CD940: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826CD944: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826CD948: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826CD94C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826CD950: 4BD994D1  bl 0x82466e20
	ctx.lr = 0x826CD954;
	sub_82466E20(ctx, base);
	// 826CD954: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826CD958: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826CD95C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826CD960: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826CD968(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826CD968 size=108
    let mut pc: u32 = 0x826CD968;
    'dispatch: loop {
        match pc {
            0x826CD968 => {
    //   block [0x826CD968..0x826CD9D4)
	// 826CD968: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826CD96C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826CD970: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826CD974: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826CD978: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826CD97C: 38EBBC60  addi r7, r11, -0x43a0
	ctx.r[7].s64 = ctx.r[11].s64 + -17312;
	// 826CD980: 39000007  li r8, 7
	ctx.r[8].s64 = 7;
	// 826CD984: 388AADE0  addi r4, r10, -0x5220
	ctx.r[4].s64 = ctx.r[10].s64 + -21024;
	// 826CD988: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826CD98C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826CD990: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826CD994: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826CD998: 386A8F0C  addi r3, r10, -0x70f4
	ctx.r[3].s64 = ctx.r[10].s64 + -28916;
	// 826CD99C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826CD9A0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826CD9A4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826CD9A8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826CD9AC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826CD9B0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826CD9B4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826CD9B8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826CD9BC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826CD9C0: 4BD99461  bl 0x82466e20
	ctx.lr = 0x826CD9C4;
	sub_82466E20(ctx, base);
	// 826CD9C4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826CD9C8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826CD9CC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826CD9D0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826CD9D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826CD9D8 size=112
    let mut pc: u32 = 0x826CD9D8;
    'dispatch: loop {
        match pc {
            0x826CD9D8 => {
    //   block [0x826CD9D8..0x826CDA48)
	// 826CD9D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826CD9DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826CD9E0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826CD9E4: 3D40827F  lis r10, -0x7d81
	ctx.r[10].s64 = -2105606144;
	// 826CD9E8: 3900000C  li r8, 0xc
	ctx.r[8].s64 = 12;
	// 826CD9EC: 38EABD08  addi r7, r10, -0x42f8
	ctx.r[7].s64 = ctx.r[10].s64 + -17144;
	// 826CD9F0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826CD9F4: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 826CD9F8: 388AA5FC  addi r4, r10, -0x5a04
	ctx.r[4].s64 = ctx.r[10].s64 + -23044;
	// 826CD9FC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826CDA00: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826CDA04: 396B3610  addi r11, r11, 0x3610
	ctx.r[11].s64 = ctx.r[11].s64 + 13840;
	// 826CDA08: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826CDA0C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826CDA10: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826CDA14: 386A8F3C  addi r3, r10, -0x70c4
	ctx.r[3].s64 = ctx.r[10].s64 + -28868;
	// 826CDA18: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826CDA1C: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 826CDA20: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826CDA24: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 826CDA28: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826CDA2C: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826CDA30: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826CDA34: 4BD993ED  bl 0x82466e20
	ctx.lr = 0x826CDA38;
	sub_82466E20(ctx, base);
	// 826CDA38: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826CDA3C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826CDA40: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826CDA44: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826CDA48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826CDA48 size=108
    let mut pc: u32 = 0x826CDA48;
    'dispatch: loop {
        match pc {
            0x826CDA48 => {
    //   block [0x826CDA48..0x826CDAB4)
	// 826CDA48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826CDA4C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826CDA50: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826CDA54: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826CDA58: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826CDA5C: 38EBBE28  addi r7, r11, -0x41d8
	ctx.r[7].s64 = ctx.r[11].s64 + -16856;
	// 826CDA60: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 826CDA64: 388AAD28  addi r4, r10, -0x52d8
	ctx.r[4].s64 = ctx.r[10].s64 + -21208;
	// 826CDA68: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826CDA6C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826CDA70: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826CDA74: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826CDA78: 386A8F6C  addi r3, r10, -0x7094
	ctx.r[3].s64 = ctx.r[10].s64 + -28820;
	// 826CDA7C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826CDA80: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826CDA84: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826CDA88: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826CDA8C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826CDA90: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826CDA94: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826CDA98: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826CDA9C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826CDAA0: 4BD99381  bl 0x82466e20
	ctx.lr = 0x826CDAA4;
	sub_82466E20(ctx, base);
	// 826CDAA4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826CDAA8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826CDAAC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826CDAB0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826CDAB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826CDAB8 size=108
    let mut pc: u32 = 0x826CDAB8;
    'dispatch: loop {
        match pc {
            0x826CDAB8 => {
    //   block [0x826CDAB8..0x826CDB24)
	// 826CDAB8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826CDABC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826CDAC0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826CDAC4: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826CDAC8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826CDACC: 38EBBE88  addi r7, r11, -0x4178
	ctx.r[7].s64 = ctx.r[11].s64 + -16760;
	// 826CDAD0: 3900000B  li r8, 0xb
	ctx.r[8].s64 = 11;
	// 826CDAD4: 388A9070  addi r4, r10, -0x6f90
	ctx.r[4].s64 = ctx.r[10].s64 + -28560;
	// 826CDAD8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826CDADC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826CDAE0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826CDAE4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826CDAE8: 386A8F9C  addi r3, r10, -0x7064
	ctx.r[3].s64 = ctx.r[10].s64 + -28772;
	// 826CDAEC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826CDAF0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826CDAF4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826CDAF8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826CDAFC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826CDB00: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826CDB04: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826CDB08: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826CDB0C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826CDB10: 4BD99311  bl 0x82466e20
	ctx.lr = 0x826CDB14;
	sub_82466E20(ctx, base);
	// 826CDB14: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826CDB18: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826CDB1C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826CDB20: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826CDB28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826CDB28 size=108
    let mut pc: u32 = 0x826CDB28;
    'dispatch: loop {
        match pc {
            0x826CDB28 => {
    //   block [0x826CDB28..0x826CDB94)
	// 826CDB28: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826CDB2C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826CDB30: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826CDB34: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826CDB38: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826CDB3C: 38EBBF90  addi r7, r11, -0x4070
	ctx.r[7].s64 = ctx.r[11].s64 + -16496;
	// 826CDB40: 39000009  li r8, 9
	ctx.r[8].s64 = 9;
	// 826CDB44: 388A8E98  addi r4, r10, -0x7168
	ctx.r[4].s64 = ctx.r[10].s64 + -29032;
	// 826CDB48: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826CDB4C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826CDB50: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826CDB54: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826CDB58: 386A8FCC  addi r3, r10, -0x7034
	ctx.r[3].s64 = ctx.r[10].s64 + -28724;
	// 826CDB5C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826CDB60: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826CDB64: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826CDB68: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826CDB6C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826CDB70: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826CDB74: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826CDB78: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826CDB7C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826CDB80: 4BD992A1  bl 0x82466e20
	ctx.lr = 0x826CDB84;
	sub_82466E20(ctx, base);
	// 826CDB84: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826CDB88: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826CDB8C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826CDB90: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826CDB98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826CDB98 size=108
    let mut pc: u32 = 0x826CDB98;
    'dispatch: loop {
        match pc {
            0x826CDB98 => {
    //   block [0x826CDB98..0x826CDC04)
	// 826CDB98: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826CDB9C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826CDBA0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826CDBA4: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826CDBA8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826CDBAC: 38EBC068  addi r7, r11, -0x3f98
	ctx.r[7].s64 = ctx.r[11].s64 + -16280;
	// 826CDBB0: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826CDBB4: 388A8CF0  addi r4, r10, -0x7310
	ctx.r[4].s64 = ctx.r[10].s64 + -29456;
	// 826CDBB8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826CDBBC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826CDBC0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826CDBC4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826CDBC8: 386A8FFC  addi r3, r10, -0x7004
	ctx.r[3].s64 = ctx.r[10].s64 + -28676;
	// 826CDBCC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826CDBD0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826CDBD4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826CDBD8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826CDBDC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826CDBE0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826CDBE4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826CDBE8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826CDBEC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826CDBF0: 4BD99231  bl 0x82466e20
	ctx.lr = 0x826CDBF4;
	sub_82466E20(ctx, base);
	// 826CDBF4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826CDBF8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826CDBFC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826CDC00: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826CDC08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826CDC08 size=108
    let mut pc: u32 = 0x826CDC08;
    'dispatch: loop {
        match pc {
            0x826CDC08 => {
    //   block [0x826CDC08..0x826CDC74)
	// 826CDC08: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826CDC0C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826CDC10: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826CDC14: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826CDC18: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826CDC1C: 38EBC098  addi r7, r11, -0x3f68
	ctx.r[7].s64 = ctx.r[11].s64 + -16232;
	// 826CDC20: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826CDC24: 388A8D14  addi r4, r10, -0x72ec
	ctx.r[4].s64 = ctx.r[10].s64 + -29420;
	// 826CDC28: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826CDC2C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826CDC30: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826CDC34: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826CDC38: 386A902C  addi r3, r10, -0x6fd4
	ctx.r[3].s64 = ctx.r[10].s64 + -28628;
	// 826CDC3C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826CDC40: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826CDC44: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826CDC48: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826CDC4C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826CDC50: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826CDC54: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826CDC58: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826CDC5C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826CDC60: 4BD991C1  bl 0x82466e20
	ctx.lr = 0x826CDC64;
	sub_82466E20(ctx, base);
	// 826CDC64: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826CDC68: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826CDC6C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826CDC70: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826CDC78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826CDC78 size=108
    let mut pc: u32 = 0x826CDC78;
    'dispatch: loop {
        match pc {
            0x826CDC78 => {
    //   block [0x826CDC78..0x826CDCE4)
	// 826CDC78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826CDC7C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826CDC80: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826CDC84: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826CDC88: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826CDC8C: 38EBC0B0  addi r7, r11, -0x3f50
	ctx.r[7].s64 = ctx.r[11].s64 + -16208;
	// 826CDC90: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 826CDC94: 388A37C4  addi r4, r10, 0x37c4
	ctx.r[4].s64 = ctx.r[10].s64 + 14276;
	// 826CDC98: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826CDC9C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826CDCA0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826CDCA4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826CDCA8: 386A905C  addi r3, r10, -0x6fa4
	ctx.r[3].s64 = ctx.r[10].s64 + -28580;
	// 826CDCAC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826CDCB0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826CDCB4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826CDCB8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826CDCBC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826CDCC0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826CDCC4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826CDCC8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826CDCCC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826CDCD0: 4BD99151  bl 0x82466e20
	ctx.lr = 0x826CDCD4;
	sub_82466E20(ctx, base);
	// 826CDCD4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826CDCD8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826CDCDC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826CDCE0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826CDCE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826CDCE8 size=108
    let mut pc: u32 = 0x826CDCE8;
    'dispatch: loop {
        match pc {
            0x826CDCE8 => {
    //   block [0x826CDCE8..0x826CDD54)
	// 826CDCE8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826CDCEC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826CDCF0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826CDCF4: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826CDCF8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826CDCFC: 38EBC0F8  addi r7, r11, -0x3f08
	ctx.r[7].s64 = ctx.r[11].s64 + -16136;
	// 826CDD00: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826CDD04: 388A37F4  addi r4, r10, 0x37f4
	ctx.r[4].s64 = ctx.r[10].s64 + 14324;
	// 826CDD08: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826CDD0C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826CDD10: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826CDD14: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826CDD18: 386A908C  addi r3, r10, -0x6f74
	ctx.r[3].s64 = ctx.r[10].s64 + -28532;
	// 826CDD1C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826CDD20: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826CDD24: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826CDD28: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826CDD2C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826CDD30: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826CDD34: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826CDD38: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826CDD3C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826CDD40: 4BD990E1  bl 0x82466e20
	ctx.lr = 0x826CDD44;
	sub_82466E20(ctx, base);
	// 826CDD44: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826CDD48: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826CDD4C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826CDD50: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826CDD58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826CDD58 size=112
    let mut pc: u32 = 0x826CDD58;
    'dispatch: loop {
        match pc {
            0x826CDD58 => {
    //   block [0x826CDD58..0x826CDDC8)
	// 826CDD58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826CDD5C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826CDD60: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826CDD64: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826CDD68: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826CDD6C: 38AA86FC  addi r5, r10, -0x7904
	ctx.r[5].s64 = ctx.r[10].s64 + -30980;
	// 826CDD70: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826CDD74: 390BC110  addi r8, r11, -0x3ef0
	ctx.r[8].s64 = ctx.r[11].s64 + -16112;
	// 826CDD78: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 826CDD7C: 388A3810  addi r4, r10, 0x3810
	ctx.r[4].s64 = ctx.r[10].s64 + 14352;
	// 826CDD80: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826CDD84: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826CDD88: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826CDD8C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826CDD90: 386A90BC  addi r3, r10, -0x6f44
	ctx.r[3].s64 = ctx.r[10].s64 + -28484;
	// 826CDD94: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826CDD98: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826CDD9C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826CDDA0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826CDDA4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826CDDA8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826CDDAC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826CDDB0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826CDDB4: 4BD9906D  bl 0x82466e20
	ctx.lr = 0x826CDDB8;
	sub_82466E20(ctx, base);
	// 826CDDB8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826CDDBC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826CDDC0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826CDDC4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826CDDC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826CDDC8 size=108
    let mut pc: u32 = 0x826CDDC8;
    'dispatch: loop {
        match pc {
            0x826CDDC8 => {
    //   block [0x826CDDC8..0x826CDE34)
	// 826CDDC8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826CDDCC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826CDDD0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826CDDD4: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826CDDD8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826CDDDC: 38EBC158  addi r7, r11, -0x3ea8
	ctx.r[7].s64 = ctx.r[11].s64 + -16040;
	// 826CDDE0: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 826CDDE4: 388A3A30  addi r4, r10, 0x3a30
	ctx.r[4].s64 = ctx.r[10].s64 + 14896;
	// 826CDDE8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826CDDEC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826CDDF0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826CDDF4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826CDDF8: 386A90EC  addi r3, r10, -0x6f14
	ctx.r[3].s64 = ctx.r[10].s64 + -28436;
	// 826CDDFC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826CDE00: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826CDE04: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826CDE08: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826CDE0C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826CDE10: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826CDE14: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826CDE18: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826CDE1C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826CDE20: 4BD99001  bl 0x82466e20
	ctx.lr = 0x826CDE24;
	sub_82466E20(ctx, base);
	// 826CDE24: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826CDE28: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826CDE2C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826CDE30: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826CDE38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826CDE38 size=112
    let mut pc: u32 = 0x826CDE38;
    'dispatch: loop {
        match pc {
            0x826CDE38 => {
    //   block [0x826CDE38..0x826CDEA8)
	// 826CDE38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826CDE3C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826CDE40: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826CDE44: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826CDE48: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826CDE4C: 38AA90EC  addi r5, r10, -0x6f14
	ctx.r[5].s64 = ctx.r[10].s64 + -28436;
	// 826CDE50: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826CDE54: 390BC1B8  addi r8, r11, -0x3e48
	ctx.r[8].s64 = ctx.r[11].s64 + -15944;
	// 826CDE58: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 826CDE5C: 388A3A3C  addi r4, r10, 0x3a3c
	ctx.r[4].s64 = ctx.r[10].s64 + 14908;
	// 826CDE60: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826CDE64: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826CDE68: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826CDE6C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826CDE70: 386A911C  addi r3, r10, -0x6ee4
	ctx.r[3].s64 = ctx.r[10].s64 + -28388;
	// 826CDE74: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826CDE78: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826CDE7C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826CDE80: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826CDE84: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826CDE88: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826CDE8C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826CDE90: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826CDE94: 4BD98F8D  bl 0x82466e20
	ctx.lr = 0x826CDE98;
	sub_82466E20(ctx, base);
	// 826CDE98: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826CDE9C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826CDEA0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826CDEA4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826CDEA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826CDEA8 size=96
    let mut pc: u32 = 0x826CDEA8;
    'dispatch: loop {
        match pc {
            0x826CDEA8 => {
    //   block [0x826CDEA8..0x826CDF08)
	// 826CDEA8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826CDEAC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826CDEB0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826CDEB4: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826CDEB8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826CDEBC: 388A3A4C  addi r4, r10, 0x3a4c
	ctx.r[4].s64 = ctx.r[10].s64 + 14924;
	// 826CDEC0: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826CDEC4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826CDEC8: 386A914C  addi r3, r10, -0x6eb4
	ctx.r[3].s64 = ctx.r[10].s64 + -28340;
	// 826CDECC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826CDED0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826CDED4: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826CDED8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826CDEDC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826CDEE0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826CDEE4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826CDEE8: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826CDEEC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826CDEF0: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826CDEF4: 4BD98F2D  bl 0x82466e20
	ctx.lr = 0x826CDEF8;
	sub_82466E20(ctx, base);
	// 826CDEF8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826CDEFC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826CDF00: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826CDF04: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826CDF08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826CDF08 size=112
    let mut pc: u32 = 0x826CDF08;
    'dispatch: loop {
        match pc {
            0x826CDF08 => {
    //   block [0x826CDF08..0x826CDF78)
	// 826CDF08: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826CDF0C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826CDF10: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826CDF14: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826CDF18: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826CDF1C: 38AAA76C  addi r5, r10, -0x5894
	ctx.r[5].s64 = ctx.r[10].s64 + -22676;
	// 826CDF20: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826CDF24: 390BC200  addi r8, r11, -0x3e00
	ctx.r[8].s64 = ctx.r[11].s64 + -15872;
	// 826CDF28: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 826CDF2C: 388A3A64  addi r4, r10, 0x3a64
	ctx.r[4].s64 = ctx.r[10].s64 + 14948;
	// 826CDF30: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826CDF34: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826CDF38: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826CDF3C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826CDF40: 386A917C  addi r3, r10, -0x6e84
	ctx.r[3].s64 = ctx.r[10].s64 + -28292;
	// 826CDF44: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826CDF48: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826CDF4C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826CDF50: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826CDF54: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826CDF58: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826CDF5C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826CDF60: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826CDF64: 4BD98EBD  bl 0x82466e20
	ctx.lr = 0x826CDF68;
	sub_82466E20(ctx, base);
	// 826CDF68: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826CDF6C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826CDF70: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826CDF74: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826CDF78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826CDF78 size=96
    let mut pc: u32 = 0x826CDF78;
    'dispatch: loop {
        match pc {
            0x826CDF78 => {
    //   block [0x826CDF78..0x826CDFD8)
	// 826CDF78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826CDF7C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826CDF80: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826CDF84: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826CDF88: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826CDF8C: 388A3A7C  addi r4, r10, 0x3a7c
	ctx.r[4].s64 = ctx.r[10].s64 + 14972;
	// 826CDF90: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826CDF94: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826CDF98: 386A91AC  addi r3, r10, -0x6e54
	ctx.r[3].s64 = ctx.r[10].s64 + -28244;
	// 826CDF9C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826CDFA0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826CDFA4: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826CDFA8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826CDFAC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826CDFB0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826CDFB4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826CDFB8: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826CDFBC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826CDFC0: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826CDFC4: 4BD98E5D  bl 0x82466e20
	ctx.lr = 0x826CDFC8;
	sub_82466E20(ctx, base);
	// 826CDFC8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826CDFCC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826CDFD0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826CDFD4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826CDFD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826CDFD8 size=100
    let mut pc: u32 = 0x826CDFD8;
    'dispatch: loop {
        match pc {
            0x826CDFD8 => {
    //   block [0x826CDFD8..0x826CE03C)
	// 826CDFD8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826CDFDC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826CDFE0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826CDFE4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826CDFE8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826CDFEC: 38AA86FC  addi r5, r10, -0x7904
	ctx.r[5].s64 = ctx.r[10].s64 + -30980;
	// 826CDFF0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826CDFF4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826CDFF8: 388A3A9C  addi r4, r10, 0x3a9c
	ctx.r[4].s64 = ctx.r[10].s64 + 15004;
	// 826CDFFC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826CE000: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826CE004: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 826CE008: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826CE00C: 386A91DC  addi r3, r10, -0x6e24
	ctx.r[3].s64 = ctx.r[10].s64 + -28196;
	// 826CE010: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826CE014: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826CE018: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826CE01C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826CE020: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826CE024: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826CE028: 4BD98DF9  bl 0x82466e20
	ctx.lr = 0x826CE02C;
	sub_82466E20(ctx, base);
	// 826CE02C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826CE030: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826CE034: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826CE038: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826CE040(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826CE040 size=96
    let mut pc: u32 = 0x826CE040;
    'dispatch: loop {
        match pc {
            0x826CE040 => {
    //   block [0x826CE040..0x826CE0A0)
	// 826CE040: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826CE044: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826CE048: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826CE04C: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826CE050: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826CE054: 388A3AC4  addi r4, r10, 0x3ac4
	ctx.r[4].s64 = ctx.r[10].s64 + 15044;
	// 826CE058: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826CE05C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826CE060: 386A920C  addi r3, r10, -0x6df4
	ctx.r[3].s64 = ctx.r[10].s64 + -28148;
	// 826CE064: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826CE068: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826CE06C: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826CE070: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826CE074: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826CE078: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826CE07C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826CE080: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826CE084: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826CE088: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826CE08C: 4BD98D95  bl 0x82466e20
	ctx.lr = 0x826CE090;
	sub_82466E20(ctx, base);
	// 826CE090: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826CE094: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826CE098: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826CE09C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826CE0A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826CE0A0 size=112
    let mut pc: u32 = 0x826CE0A0;
    'dispatch: loop {
        match pc {
            0x826CE0A0 => {
    //   block [0x826CE0A0..0x826CE110)
	// 826CE0A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826CE0A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826CE0A8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826CE0AC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826CE0B0: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826CE0B4: 38AA91DC  addi r5, r10, -0x6e24
	ctx.r[5].s64 = ctx.r[10].s64 + -28196;
	// 826CE0B8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826CE0BC: 390BC260  addi r8, r11, -0x3da0
	ctx.r[8].s64 = ctx.r[11].s64 + -15776;
	// 826CE0C0: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826CE0C4: 388A3AF8  addi r4, r10, 0x3af8
	ctx.r[4].s64 = ctx.r[10].s64 + 15096;
	// 826CE0C8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826CE0CC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826CE0D0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826CE0D4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826CE0D8: 386A923C  addi r3, r10, -0x6dc4
	ctx.r[3].s64 = ctx.r[10].s64 + -28100;
	// 826CE0DC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826CE0E0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826CE0E4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826CE0E8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826CE0EC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826CE0F0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826CE0F4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826CE0F8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826CE0FC: 4BD98D25  bl 0x82466e20
	ctx.lr = 0x826CE100;
	sub_82466E20(ctx, base);
	// 826CE100: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826CE104: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826CE108: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826CE10C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826CE110(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826CE110 size=112
    let mut pc: u32 = 0x826CE110;
    'dispatch: loop {
        match pc {
            0x826CE110 => {
    //   block [0x826CE110..0x826CE180)
	// 826CE110: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826CE114: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826CE118: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826CE11C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826CE120: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826CE124: 38AA91DC  addi r5, r10, -0x6e24
	ctx.r[5].s64 = ctx.r[10].s64 + -28196;
	// 826CE128: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826CE12C: 390BC290  addi r8, r11, -0x3d70
	ctx.r[8].s64 = ctx.r[11].s64 + -15728;
	// 826CE130: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826CE134: 388A3B08  addi r4, r10, 0x3b08
	ctx.r[4].s64 = ctx.r[10].s64 + 15112;
	// 826CE138: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826CE13C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826CE140: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826CE144: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826CE148: 386A926C  addi r3, r10, -0x6d94
	ctx.r[3].s64 = ctx.r[10].s64 + -28052;
	// 826CE14C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826CE150: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826CE154: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826CE158: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826CE15C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826CE160: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826CE164: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826CE168: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826CE16C: 4BD98CB5  bl 0x82466e20
	ctx.lr = 0x826CE170;
	sub_82466E20(ctx, base);
	// 826CE170: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826CE174: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826CE178: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826CE17C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826CE180(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826CE180 size=100
    let mut pc: u32 = 0x826CE180;
    'dispatch: loop {
        match pc {
            0x826CE180 => {
    //   block [0x826CE180..0x826CE1E4)
	// 826CE180: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826CE184: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826CE188: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826CE18C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826CE190: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826CE194: 38AA91DC  addi r5, r10, -0x6e24
	ctx.r[5].s64 = ctx.r[10].s64 + -28196;
	// 826CE198: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826CE19C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826CE1A0: 388A3B20  addi r4, r10, 0x3b20
	ctx.r[4].s64 = ctx.r[10].s64 + 15136;
	// 826CE1A4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826CE1A8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826CE1AC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826CE1B0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826CE1B4: 386A929C  addi r3, r10, -0x6d64
	ctx.r[3].s64 = ctx.r[10].s64 + -28004;
	// 826CE1B8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826CE1BC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826CE1C0: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826CE1C4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826CE1C8: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826CE1CC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826CE1D0: 4BD98C51  bl 0x82466e20
	ctx.lr = 0x826CE1D4;
	sub_82466E20(ctx, base);
	// 826CE1D4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826CE1D8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826CE1DC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826CE1E0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826CE1E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826CE1E8 size=96
    let mut pc: u32 = 0x826CE1E8;
    'dispatch: loop {
        match pc {
            0x826CE1E8 => {
    //   block [0x826CE1E8..0x826CE248)
	// 826CE1E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826CE1EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826CE1F0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826CE1F4: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826CE1F8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826CE1FC: 388A3B38  addi r4, r10, 0x3b38
	ctx.r[4].s64 = ctx.r[10].s64 + 15160;
	// 826CE200: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826CE204: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826CE208: 386A92CC  addi r3, r10, -0x6d34
	ctx.r[3].s64 = ctx.r[10].s64 + -27956;
	// 826CE20C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826CE210: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826CE214: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826CE218: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826CE21C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826CE220: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826CE224: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826CE228: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826CE22C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826CE230: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826CE234: 4BD98BED  bl 0x82466e20
	ctx.lr = 0x826CE238;
	sub_82466E20(ctx, base);
	// 826CE238: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826CE23C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826CE240: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826CE244: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826CE248(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826CE248 size=112
    let mut pc: u32 = 0x826CE248;
    'dispatch: loop {
        match pc {
            0x826CE248 => {
    //   block [0x826CE248..0x826CE2B8)
	// 826CE248: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826CE24C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826CE250: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826CE254: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826CE258: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826CE25C: 38AA86FC  addi r5, r10, -0x7904
	ctx.r[5].s64 = ctx.r[10].s64 + -30980;
	// 826CE260: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826CE264: 390BC2A8  addi r8, r11, -0x3d58
	ctx.r[8].s64 = ctx.r[11].s64 + -15704;
	// 826CE268: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826CE26C: 388A3B54  addi r4, r10, 0x3b54
	ctx.r[4].s64 = ctx.r[10].s64 + 15188;
	// 826CE270: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826CE274: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826CE278: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826CE27C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826CE280: 386A92FC  addi r3, r10, -0x6d04
	ctx.r[3].s64 = ctx.r[10].s64 + -27908;
	// 826CE284: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826CE288: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826CE28C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826CE290: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826CE294: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826CE298: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826CE29C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826CE2A0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826CE2A4: 4BD98B7D  bl 0x82466e20
	ctx.lr = 0x826CE2A8;
	sub_82466E20(ctx, base);
	// 826CE2A8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826CE2AC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826CE2B0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826CE2B4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826CE2B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826CE2B8 size=108
    let mut pc: u32 = 0x826CE2B8;
    'dispatch: loop {
        match pc {
            0x826CE2B8 => {
    //   block [0x826CE2B8..0x826CE324)
	// 826CE2B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826CE2BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826CE2C0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826CE2C4: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826CE2C8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826CE2CC: 38EBC2C0  addi r7, r11, -0x3d40
	ctx.r[7].s64 = ctx.r[11].s64 + -15680;
	// 826CE2D0: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 826CE2D4: 388A3B88  addi r4, r10, 0x3b88
	ctx.r[4].s64 = ctx.r[10].s64 + 15240;
	// 826CE2D8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826CE2DC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826CE2E0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826CE2E4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826CE2E8: 386A932C  addi r3, r10, -0x6cd4
	ctx.r[3].s64 = ctx.r[10].s64 + -27860;
	// 826CE2EC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826CE2F0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826CE2F4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826CE2F8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826CE2FC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826CE300: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826CE304: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826CE308: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826CE30C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826CE310: 4BD98B11  bl 0x82466e20
	ctx.lr = 0x826CE314;
	sub_82466E20(ctx, base);
	// 826CE314: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826CE318: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826CE31C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826CE320: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826CE328(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826CE328 size=112
    let mut pc: u32 = 0x826CE328;
    'dispatch: loop {
        match pc {
            0x826CE328 => {
    //   block [0x826CE328..0x826CE398)
	// 826CE328: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826CE32C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826CE330: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826CE334: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826CE338: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826CE33C: 38AA944C  addi r5, r10, -0x6bb4
	ctx.r[5].s64 = ctx.r[10].s64 + -27572;
	// 826CE340: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826CE344: 390BC320  addi r8, r11, -0x3ce0
	ctx.r[8].s64 = ctx.r[11].s64 + -15584;
	// 826CE348: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826CE34C: 388A3B9C  addi r4, r10, 0x3b9c
	ctx.r[4].s64 = ctx.r[10].s64 + 15260;
	// 826CE350: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826CE354: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826CE358: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826CE35C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826CE360: 386A935C  addi r3, r10, -0x6ca4
	ctx.r[3].s64 = ctx.r[10].s64 + -27812;
	// 826CE364: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826CE368: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826CE36C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826CE370: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826CE374: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826CE378: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826CE37C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826CE380: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826CE384: 4BD98A9D  bl 0x82466e20
	ctx.lr = 0x826CE388;
	sub_82466E20(ctx, base);
	// 826CE388: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826CE38C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826CE390: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826CE394: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826CE398(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826CE398 size=112
    let mut pc: u32 = 0x826CE398;
    'dispatch: loop {
        match pc {
            0x826CE398 => {
    //   block [0x826CE398..0x826CE408)
	// 826CE398: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826CE39C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826CE3A0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826CE3A4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826CE3A8: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826CE3AC: 38AA92FC  addi r5, r10, -0x6d04
	ctx.r[5].s64 = ctx.r[10].s64 + -27908;
	// 826CE3B0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826CE3B4: 390BC338  addi r8, r11, -0x3cc8
	ctx.r[8].s64 = ctx.r[11].s64 + -15560;
	// 826CE3B8: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826CE3BC: 388A3BA8  addi r4, r10, 0x3ba8
	ctx.r[4].s64 = ctx.r[10].s64 + 15272;
	// 826CE3C0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826CE3C4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826CE3C8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826CE3CC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826CE3D0: 386A938C  addi r3, r10, -0x6c74
	ctx.r[3].s64 = ctx.r[10].s64 + -27764;
	// 826CE3D4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826CE3D8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826CE3DC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826CE3E0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826CE3E4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826CE3E8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826CE3EC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826CE3F0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826CE3F4: 4BD98A2D  bl 0x82466e20
	ctx.lr = 0x826CE3F8;
	sub_82466E20(ctx, base);
	// 826CE3F8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826CE3FC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826CE400: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826CE404: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826CE408(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826CE408 size=112
    let mut pc: u32 = 0x826CE408;
    'dispatch: loop {
        match pc {
            0x826CE408 => {
    //   block [0x826CE408..0x826CE478)
	// 826CE408: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826CE40C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826CE410: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826CE414: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826CE418: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826CE41C: 38AA92FC  addi r5, r10, -0x6d04
	ctx.r[5].s64 = ctx.r[10].s64 + -27908;
	// 826CE420: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826CE424: 390BC368  addi r8, r11, -0x3c98
	ctx.r[8].s64 = ctx.r[11].s64 + -15512;
	// 826CE428: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826CE42C: 388A3BB4  addi r4, r10, 0x3bb4
	ctx.r[4].s64 = ctx.r[10].s64 + 15284;
	// 826CE430: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826CE434: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826CE438: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826CE43C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826CE440: 386A93BC  addi r3, r10, -0x6c44
	ctx.r[3].s64 = ctx.r[10].s64 + -27716;
	// 826CE444: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826CE448: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826CE44C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826CE450: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826CE454: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826CE458: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826CE45C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826CE460: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826CE464: 4BD989BD  bl 0x82466e20
	ctx.lr = 0x826CE468;
	sub_82466E20(ctx, base);
	// 826CE468: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826CE46C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826CE470: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826CE474: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826CE478(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826CE478 size=112
    let mut pc: u32 = 0x826CE478;
    'dispatch: loop {
        match pc {
            0x826CE478 => {
    //   block [0x826CE478..0x826CE4E8)
	// 826CE478: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826CE47C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826CE480: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826CE484: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826CE488: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826CE48C: 38AA944C  addi r5, r10, -0x6bb4
	ctx.r[5].s64 = ctx.r[10].s64 + -27572;
	// 826CE490: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826CE494: 390BC380  addi r8, r11, -0x3c80
	ctx.r[8].s64 = ctx.r[11].s64 + -15488;
	// 826CE498: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826CE49C: 388A3BC4  addi r4, r10, 0x3bc4
	ctx.r[4].s64 = ctx.r[10].s64 + 15300;
	// 826CE4A0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826CE4A4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826CE4A8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826CE4AC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826CE4B0: 386A93EC  addi r3, r10, -0x6c14
	ctx.r[3].s64 = ctx.r[10].s64 + -27668;
	// 826CE4B4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826CE4B8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826CE4BC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826CE4C0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826CE4C4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826CE4C8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826CE4CC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826CE4D0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826CE4D4: 4BD9894D  bl 0x82466e20
	ctx.lr = 0x826CE4D8;
	sub_82466E20(ctx, base);
	// 826CE4D8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826CE4DC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826CE4E0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826CE4E4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826CE4E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826CE4E8 size=112
    let mut pc: u32 = 0x826CE4E8;
    'dispatch: loop {
        match pc {
            0x826CE4E8 => {
    //   block [0x826CE4E8..0x826CE558)
	// 826CE4E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826CE4EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826CE4F0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826CE4F4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826CE4F8: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826CE4FC: 38AA92FC  addi r5, r10, -0x6d04
	ctx.r[5].s64 = ctx.r[10].s64 + -27908;
	// 826CE500: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826CE504: 390BC3B0  addi r8, r11, -0x3c50
	ctx.r[8].s64 = ctx.r[11].s64 + -15440;
	// 826CE508: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826CE50C: 388A3BD4  addi r4, r10, 0x3bd4
	ctx.r[4].s64 = ctx.r[10].s64 + 15316;
	// 826CE510: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826CE514: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826CE518: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826CE51C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826CE520: 386A941C  addi r3, r10, -0x6be4
	ctx.r[3].s64 = ctx.r[10].s64 + -27620;
	// 826CE524: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826CE528: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826CE52C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826CE530: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826CE534: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826CE538: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826CE53C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826CE540: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826CE544: 4BD988DD  bl 0x82466e20
	ctx.lr = 0x826CE548;
	sub_82466E20(ctx, base);
	// 826CE548: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826CE54C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826CE550: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826CE554: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826CE558(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826CE558 size=112
    let mut pc: u32 = 0x826CE558;
    'dispatch: loop {
        match pc {
            0x826CE558 => {
    //   block [0x826CE558..0x826CE5C8)
	// 826CE558: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826CE55C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826CE560: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826CE564: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826CE568: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826CE56C: 38AA98FC  addi r5, r10, -0x6704
	ctx.r[5].s64 = ctx.r[10].s64 + -26372;
	// 826CE570: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826CE574: 390BC3C8  addi r8, r11, -0x3c38
	ctx.r[8].s64 = ctx.r[11].s64 + -15416;
	// 826CE578: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826CE57C: 388A3BE8  addi r4, r10, 0x3be8
	ctx.r[4].s64 = ctx.r[10].s64 + 15336;
	// 826CE580: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826CE584: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826CE588: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826CE58C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826CE590: 386A944C  addi r3, r10, -0x6bb4
	ctx.r[3].s64 = ctx.r[10].s64 + -27572;
	// 826CE594: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826CE598: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826CE59C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826CE5A0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826CE5A4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826CE5A8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826CE5AC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826CE5B0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826CE5B4: 4BD9886D  bl 0x82466e20
	ctx.lr = 0x826CE5B8;
	sub_82466E20(ctx, base);
	// 826CE5B8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826CE5BC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826CE5C0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826CE5C4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826CE5C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826CE5C8 size=112
    let mut pc: u32 = 0x826CE5C8;
    'dispatch: loop {
        match pc {
            0x826CE5C8 => {
    //   block [0x826CE5C8..0x826CE638)
	// 826CE5C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826CE5CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826CE5D0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826CE5D4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826CE5D8: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826CE5DC: 38AA965C  addi r5, r10, -0x69a4
	ctx.r[5].s64 = ctx.r[10].s64 + -27044;
	// 826CE5E0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826CE5E4: 390BC3E0  addi r8, r11, -0x3c20
	ctx.r[8].s64 = ctx.r[11].s64 + -15392;
	// 826CE5E8: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826CE5EC: 388A3BF8  addi r4, r10, 0x3bf8
	ctx.r[4].s64 = ctx.r[10].s64 + 15352;
	// 826CE5F0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826CE5F4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826CE5F8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826CE5FC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826CE600: 386A947C  addi r3, r10, -0x6b84
	ctx.r[3].s64 = ctx.r[10].s64 + -27524;
	// 826CE604: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826CE608: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826CE60C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826CE610: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826CE614: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826CE618: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826CE61C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826CE620: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826CE624: 4BD987FD  bl 0x82466e20
	ctx.lr = 0x826CE628;
	sub_82466E20(ctx, base);
	// 826CE628: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826CE62C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826CE630: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826CE634: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826CE638(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826CE638 size=112
    let mut pc: u32 = 0x826CE638;
    'dispatch: loop {
        match pc {
            0x826CE638 => {
    //   block [0x826CE638..0x826CE6A8)
	// 826CE638: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826CE63C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826CE640: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826CE644: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826CE648: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826CE64C: 38AA941C  addi r5, r10, -0x6be4
	ctx.r[5].s64 = ctx.r[10].s64 + -27620;
	// 826CE650: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826CE654: 390BC3F8  addi r8, r11, -0x3c08
	ctx.r[8].s64 = ctx.r[11].s64 + -15368;
	// 826CE658: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 826CE65C: 388A3C0C  addi r4, r10, 0x3c0c
	ctx.r[4].s64 = ctx.r[10].s64 + 15372;
	// 826CE660: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826CE664: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826CE668: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826CE66C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826CE670: 386A94AC  addi r3, r10, -0x6b54
	ctx.r[3].s64 = ctx.r[10].s64 + -27476;
	// 826CE674: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826CE678: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826CE67C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826CE680: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826CE684: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826CE688: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826CE68C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826CE690: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826CE694: 4BD9878D  bl 0x82466e20
	ctx.lr = 0x826CE698;
	sub_82466E20(ctx, base);
	// 826CE698: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826CE69C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826CE6A0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826CE6A4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826CE6A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826CE6A8 size=112
    let mut pc: u32 = 0x826CE6A8;
    'dispatch: loop {
        match pc {
            0x826CE6A8 => {
    //   block [0x826CE6A8..0x826CE718)
	// 826CE6A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826CE6AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826CE6B0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826CE6B4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826CE6B8: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826CE6BC: 38AA944C  addi r5, r10, -0x6bb4
	ctx.r[5].s64 = ctx.r[10].s64 + -27572;
	// 826CE6C0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826CE6C4: 390BC440  addi r8, r11, -0x3bc0
	ctx.r[8].s64 = ctx.r[11].s64 + -15296;
	// 826CE6C8: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826CE6CC: 388A3C24  addi r4, r10, 0x3c24
	ctx.r[4].s64 = ctx.r[10].s64 + 15396;
	// 826CE6D0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826CE6D4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826CE6D8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826CE6DC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826CE6E0: 386A94DC  addi r3, r10, -0x6b24
	ctx.r[3].s64 = ctx.r[10].s64 + -27428;
	// 826CE6E4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826CE6E8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826CE6EC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826CE6F0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826CE6F4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826CE6F8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826CE6FC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826CE700: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826CE704: 4BD9871D  bl 0x82466e20
	ctx.lr = 0x826CE708;
	sub_82466E20(ctx, base);
	// 826CE708: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826CE70C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826CE710: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826CE714: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826CE718(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826CE718 size=112
    let mut pc: u32 = 0x826CE718;
    'dispatch: loop {
        match pc {
            0x826CE718 => {
    //   block [0x826CE718..0x826CE788)
	// 826CE718: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826CE71C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826CE720: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826CE724: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826CE728: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826CE72C: 38AA944C  addi r5, r10, -0x6bb4
	ctx.r[5].s64 = ctx.r[10].s64 + -27572;
	// 826CE730: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826CE734: 390BC470  addi r8, r11, -0x3b90
	ctx.r[8].s64 = ctx.r[11].s64 + -15248;
	// 826CE738: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826CE73C: 388A3C3C  addi r4, r10, 0x3c3c
	ctx.r[4].s64 = ctx.r[10].s64 + 15420;
	// 826CE740: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826CE744: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826CE748: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826CE74C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826CE750: 386A950C  addi r3, r10, -0x6af4
	ctx.r[3].s64 = ctx.r[10].s64 + -27380;
	// 826CE754: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826CE758: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826CE75C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826CE760: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826CE764: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826CE768: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826CE76C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826CE770: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826CE774: 4BD986AD  bl 0x82466e20
	ctx.lr = 0x826CE778;
	sub_82466E20(ctx, base);
	// 826CE778: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826CE77C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826CE780: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826CE784: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826CE788(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826CE788 size=108
    let mut pc: u32 = 0x826CE788;
    'dispatch: loop {
        match pc {
            0x826CE788 => {
    //   block [0x826CE788..0x826CE7F4)
	// 826CE788: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826CE78C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826CE790: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826CE794: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826CE798: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826CE79C: 38EBC4A0  addi r7, r11, -0x3b60
	ctx.r[7].s64 = ctx.r[11].s64 + -15200;
	// 826CE7A0: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 826CE7A4: 388A3C54  addi r4, r10, 0x3c54
	ctx.r[4].s64 = ctx.r[10].s64 + 15444;
	// 826CE7A8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826CE7AC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826CE7B0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826CE7B4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826CE7B8: 386A953C  addi r3, r10, -0x6ac4
	ctx.r[3].s64 = ctx.r[10].s64 + -27332;
	// 826CE7BC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826CE7C0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826CE7C4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826CE7C8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826CE7CC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826CE7D0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826CE7D4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826CE7D8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826CE7DC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826CE7E0: 4BD98641  bl 0x82466e20
	ctx.lr = 0x826CE7E4;
	sub_82466E20(ctx, base);
	// 826CE7E4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826CE7E8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826CE7EC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826CE7F0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826CE7F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826CE7F8 size=112
    let mut pc: u32 = 0x826CE7F8;
    'dispatch: loop {
        match pc {
            0x826CE7F8 => {
    //   block [0x826CE7F8..0x826CE868)
	// 826CE7F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826CE7FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826CE800: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826CE804: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826CE808: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826CE80C: 38AA944C  addi r5, r10, -0x6bb4
	ctx.r[5].s64 = ctx.r[10].s64 + -27572;
	// 826CE810: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826CE814: 390BC4E8  addi r8, r11, -0x3b18
	ctx.r[8].s64 = ctx.r[11].s64 + -15128;
	// 826CE818: 39200005  li r9, 5
	ctx.r[9].s64 = 5;
	// 826CE81C: 388A3C78  addi r4, r10, 0x3c78
	ctx.r[4].s64 = ctx.r[10].s64 + 15480;
	// 826CE820: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826CE824: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826CE828: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826CE82C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826CE830: 386A956C  addi r3, r10, -0x6a94
	ctx.r[3].s64 = ctx.r[10].s64 + -27284;
	// 826CE834: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826CE838: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826CE83C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826CE840: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826CE844: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826CE848: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826CE84C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826CE850: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826CE854: 4BD985CD  bl 0x82466e20
	ctx.lr = 0x826CE858;
	sub_82466E20(ctx, base);
	// 826CE858: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826CE85C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826CE860: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826CE864: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826CE868(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826CE868 size=116
    let mut pc: u32 = 0x826CE868;
    'dispatch: loop {
        match pc {
            0x826CE868 => {
    //   block [0x826CE868..0x826CE8DC)
	// 826CE868: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826CE86C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826CE870: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826CE874: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826CE878: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 826CE87C: 390BC568  addi r8, r11, -0x3a98
	ctx.r[8].s64 = ctx.r[11].s64 + -15000;
	// 826CE880: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826CE884: 392A3698  addi r9, r10, 0x3698
	ctx.r[9].s64 = ctx.r[10].s64 + 13976;
	// 826CE888: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826CE88C: 38E00005  li r7, 5
	ctx.r[7].s64 = 5;
	// 826CE890: 38AA944C  addi r5, r10, -0x6bb4
	ctx.r[5].s64 = ctx.r[10].s64 + -27572;
	// 826CE894: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826CE898: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826CE89C: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826CE8A0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826CE8A4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826CE8A8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826CE8AC: 3D608299  lis r11, -0x7d67
	ctx.r[11].s64 = -2103902208;
	// 826CE8B0: 388A3C90  addi r4, r10, 0x3c90
	ctx.r[4].s64 = ctx.r[10].s64 + 15504;
	// 826CE8B4: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826CE8B8: 386B959C  addi r3, r11, -0x6a64
	ctx.r[3].s64 = ctx.r[11].s64 + -27236;
	// 826CE8BC: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826CE8C0: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826CE8C4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826CE8C8: 4BD98559  bl 0x82466e20
	ctx.lr = 0x826CE8CC;
	sub_82466E20(ctx, base);
	// 826CE8CC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826CE8D0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826CE8D4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826CE8D8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826CE8E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826CE8E0 size=100
    let mut pc: u32 = 0x826CE8E0;
    'dispatch: loop {
        match pc {
            0x826CE8E0 => {
    //   block [0x826CE8E0..0x826CE944)
	// 826CE8E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826CE8E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826CE8E8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826CE8EC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826CE8F0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826CE8F4: 38AA96EC  addi r5, r10, -0x6914
	ctx.r[5].s64 = ctx.r[10].s64 + -26900;
	// 826CE8F8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826CE8FC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826CE900: 388A3DB4  addi r4, r10, 0x3db4
	ctx.r[4].s64 = ctx.r[10].s64 + 15796;
	// 826CE904: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826CE908: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826CE90C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826CE910: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826CE914: 386A95CC  addi r3, r10, -0x6a34
	ctx.r[3].s64 = ctx.r[10].s64 + -27188;
	// 826CE918: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826CE91C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826CE920: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826CE924: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826CE928: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826CE92C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826CE930: 4BD984F1  bl 0x82466e20
	ctx.lr = 0x826CE934;
	sub_82466E20(ctx, base);
	// 826CE934: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826CE938: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826CE93C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826CE940: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826CE948(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826CE948 size=100
    let mut pc: u32 = 0x826CE948;
    'dispatch: loop {
        match pc {
            0x826CE948 => {
    //   block [0x826CE948..0x826CE9AC)
	// 826CE948: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826CE94C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826CE950: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826CE954: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826CE958: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826CE95C: 38AA92FC  addi r5, r10, -0x6d04
	ctx.r[5].s64 = ctx.r[10].s64 + -27908;
	// 826CE960: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826CE964: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826CE968: 388A3DC4  addi r4, r10, 0x3dc4
	ctx.r[4].s64 = ctx.r[10].s64 + 15812;
	// 826CE96C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826CE970: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826CE974: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826CE978: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826CE97C: 386A95FC  addi r3, r10, -0x6a04
	ctx.r[3].s64 = ctx.r[10].s64 + -27140;
	// 826CE980: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826CE984: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826CE988: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826CE98C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826CE990: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826CE994: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826CE998: 4BD98489  bl 0x82466e20
	ctx.lr = 0x826CE99C;
	sub_82466E20(ctx, base);
	// 826CE99C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826CE9A0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826CE9A4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826CE9A8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826CE9B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826CE9B0 size=108
    let mut pc: u32 = 0x826CE9B0;
    'dispatch: loop {
        match pc {
            0x826CE9B0 => {
    //   block [0x826CE9B0..0x826CEA1C)
	// 826CE9B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826CE9B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826CE9B8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826CE9BC: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826CE9C0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826CE9C4: 38EBC5E0  addi r7, r11, -0x3a20
	ctx.r[7].s64 = ctx.r[11].s64 + -14880;
	// 826CE9C8: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826CE9CC: 388A3DD8  addi r4, r10, 0x3dd8
	ctx.r[4].s64 = ctx.r[10].s64 + 15832;
	// 826CE9D0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826CE9D4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826CE9D8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826CE9DC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826CE9E0: 386A962C  addi r3, r10, -0x69d4
	ctx.r[3].s64 = ctx.r[10].s64 + -27092;
	// 826CE9E4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826CE9E8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826CE9EC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826CE9F0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826CE9F4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826CE9F8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826CE9FC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826CEA00: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826CEA04: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826CEA08: 4BD98419  bl 0x82466e20
	ctx.lr = 0x826CEA0C;
	sub_82466E20(ctx, base);
	// 826CEA0C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826CEA10: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826CEA14: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826CEA18: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826CEA20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826CEA20 size=112
    let mut pc: u32 = 0x826CEA20;
    'dispatch: loop {
        match pc {
            0x826CEA20 => {
    //   block [0x826CEA20..0x826CEA90)
	// 826CEA20: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826CEA24: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826CEA28: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826CEA2C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826CEA30: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826CEA34: 38AA941C  addi r5, r10, -0x6be4
	ctx.r[5].s64 = ctx.r[10].s64 + -27620;
	// 826CEA38: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826CEA3C: 390BC610  addi r8, r11, -0x39f0
	ctx.r[8].s64 = ctx.r[11].s64 + -14832;
	// 826CEA40: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826CEA44: 388A3DF0  addi r4, r10, 0x3df0
	ctx.r[4].s64 = ctx.r[10].s64 + 15856;
	// 826CEA48: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826CEA4C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826CEA50: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826CEA54: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826CEA58: 386A965C  addi r3, r10, -0x69a4
	ctx.r[3].s64 = ctx.r[10].s64 + -27044;
	// 826CEA5C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826CEA60: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826CEA64: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826CEA68: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826CEA6C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826CEA70: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826CEA74: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826CEA78: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826CEA7C: 4BD983A5  bl 0x82466e20
	ctx.lr = 0x826CEA80;
	sub_82466E20(ctx, base);
	// 826CEA80: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826CEA84: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826CEA88: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826CEA8C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826CEA90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826CEA90 size=108
    let mut pc: u32 = 0x826CEA90;
    'dispatch: loop {
        match pc {
            0x826CEA90 => {
    //   block [0x826CEA90..0x826CEAFC)
	// 826CEA90: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826CEA94: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826CEA98: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826CEA9C: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826CEAA0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826CEAA4: 38EBC628  addi r7, r11, -0x39d8
	ctx.r[7].s64 = ctx.r[11].s64 + -14808;
	// 826CEAA8: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826CEAAC: 388A3DFC  addi r4, r10, 0x3dfc
	ctx.r[4].s64 = ctx.r[10].s64 + 15868;
	// 826CEAB0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826CEAB4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826CEAB8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826CEABC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826CEAC0: 386A968C  addi r3, r10, -0x6974
	ctx.r[3].s64 = ctx.r[10].s64 + -26996;
	// 826CEAC4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826CEAC8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826CEACC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826CEAD0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826CEAD4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826CEAD8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826CEADC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826CEAE0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826CEAE4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826CEAE8: 4BD98339  bl 0x82466e20
	ctx.lr = 0x826CEAEC;
	sub_82466E20(ctx, base);
	// 826CEAEC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826CEAF0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826CEAF4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826CEAF8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826CEB00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826CEB00 size=28
    let mut pc: u32 = 0x826CEB00;
    'dispatch: loop {
        match pc {
            0x826CEB00 => {
    //   block [0x826CEB00..0x826CEB1C)
	// 826CEB00: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826CEB04: 3D40827F  lis r10, -0x7d81
	ctx.r[10].s64 = -2105606144;
	// 826CEB08: 394AF5E8  addi r10, r10, -0xa18
	ctx.r[10].s64 = ctx.r[10].s64 + -2584;
	// 826CEB0C: 816BC564  lwz r11, -0x3a9c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-15004 as u32) ) } as u64;
	// 826CEB10: 916A0068  stw r11, 0x68(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(104 as u32), ctx.r[11].u32 ) };
	// 826CEB14: 916A0080  stw r11, 0x80(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(128 as u32), ctx.r[11].u32 ) };
	// 826CEB18: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826CEB20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826CEB20 size=108
    let mut pc: u32 = 0x826CEB20;
    'dispatch: loop {
        match pc {
            0x826CEB20 => {
    //   block [0x826CEB20..0x826CEB8C)
	// 826CEB20: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826CEB24: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826CEB28: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826CEB2C: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826CEB30: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826CEB34: 38EBF5E8  addi r7, r11, -0xa18
	ctx.r[7].s64 = ctx.r[11].s64 + -2584;
	// 826CEB38: 3900000D  li r8, 0xd
	ctx.r[8].s64 = 13;
	// 826CEB3C: 388A3E1C  addi r4, r10, 0x3e1c
	ctx.r[4].s64 = ctx.r[10].s64 + 15900;
	// 826CEB40: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826CEB44: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826CEB48: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826CEB4C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826CEB50: 386A96BC  addi r3, r10, -0x6944
	ctx.r[3].s64 = ctx.r[10].s64 + -26948;
	// 826CEB54: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826CEB58: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826CEB5C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826CEB60: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826CEB64: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826CEB68: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826CEB6C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826CEB70: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826CEB74: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826CEB78: 4BD982A9  bl 0x82466e20
	ctx.lr = 0x826CEB7C;
	sub_82466E20(ctx, base);
	// 826CEB7C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826CEB80: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826CEB84: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826CEB88: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826CEB90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826CEB90 size=116
    let mut pc: u32 = 0x826CEB90;
    'dispatch: loop {
        match pc {
            0x826CEB90 => {
    //   block [0x826CEB90..0x826CEC04)
	// 826CEB90: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826CEB94: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826CEB98: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826CEB9C: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826CEBA0: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 826CEBA4: 390BC648  addi r8, r11, -0x39b8
	ctx.r[8].s64 = ctx.r[11].s64 + -14776;
	// 826CEBA8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826CEBAC: 392A36EC  addi r9, r10, 0x36ec
	ctx.r[9].s64 = ctx.r[10].s64 + 14060;
	// 826CEBB0: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826CEBB4: 38E00004  li r7, 4
	ctx.r[7].s64 = 4;
	// 826CEBB8: 38AA941C  addi r5, r10, -0x6be4
	ctx.r[5].s64 = ctx.r[10].s64 + -27620;
	// 826CEBBC: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826CEBC0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826CEBC4: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826CEBC8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826CEBCC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826CEBD0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826CEBD4: 3D608299  lis r11, -0x7d67
	ctx.r[11].s64 = -2103902208;
	// 826CEBD8: 388A3E30  addi r4, r10, 0x3e30
	ctx.r[4].s64 = ctx.r[10].s64 + 15920;
	// 826CEBDC: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826CEBE0: 386B96EC  addi r3, r11, -0x6914
	ctx.r[3].s64 = ctx.r[11].s64 + -26900;
	// 826CEBE4: 39400002  li r10, 2
	ctx.r[10].s64 = 2;
	// 826CEBE8: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826CEBEC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826CEBF0: 4BD98231  bl 0x82466e20
	ctx.lr = 0x826CEBF4;
	sub_82466E20(ctx, base);
	// 826CEBF4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826CEBF8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826CEBFC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826CEC00: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826CEC08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826CEC08 size=112
    let mut pc: u32 = 0x826CEC08;
    'dispatch: loop {
        match pc {
            0x826CEC08 => {
    //   block [0x826CEC08..0x826CEC78)
	// 826CEC08: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826CEC0C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826CEC10: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826CEC14: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826CEC18: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826CEC1C: 38AA93BC  addi r5, r10, -0x6c44
	ctx.r[5].s64 = ctx.r[10].s64 + -27716;
	// 826CEC20: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826CEC24: 390BC6A8  addi r8, r11, -0x3958
	ctx.r[8].s64 = ctx.r[11].s64 + -14680;
	// 826CEC28: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826CEC2C: 388A3E3C  addi r4, r10, 0x3e3c
	ctx.r[4].s64 = ctx.r[10].s64 + 15932;
	// 826CEC30: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826CEC34: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826CEC38: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826CEC3C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826CEC40: 386A971C  addi r3, r10, -0x68e4
	ctx.r[3].s64 = ctx.r[10].s64 + -26852;
	// 826CEC44: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826CEC48: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826CEC4C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826CEC50: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826CEC54: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826CEC58: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826CEC5C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826CEC60: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826CEC64: 4BD981BD  bl 0x82466e20
	ctx.lr = 0x826CEC68;
	sub_82466E20(ctx, base);
	// 826CEC68: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826CEC6C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826CEC70: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826CEC74: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826CEC78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826CEC78 size=108
    let mut pc: u32 = 0x826CEC78;
    'dispatch: loop {
        match pc {
            0x826CEC78 => {
    //   block [0x826CEC78..0x826CECE4)
	// 826CEC78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826CEC7C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826CEC80: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826CEC84: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826CEC88: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826CEC8C: 38EBC6C0  addi r7, r11, -0x3940
	ctx.r[7].s64 = ctx.r[11].s64 + -14656;
	// 826CEC90: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826CEC94: 388A3E98  addi r4, r10, 0x3e98
	ctx.r[4].s64 = ctx.r[10].s64 + 16024;
	// 826CEC98: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826CEC9C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826CECA0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826CECA4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826CECA8: 386A974C  addi r3, r10, -0x68b4
	ctx.r[3].s64 = ctx.r[10].s64 + -26804;
	// 826CECAC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826CECB0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826CECB4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826CECB8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826CECBC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826CECC0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826CECC4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826CECC8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826CECCC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826CECD0: 4BD98151  bl 0x82466e20
	ctx.lr = 0x826CECD4;
	sub_82466E20(ctx, base);
	// 826CECD4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826CECD8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826CECDC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826CECE0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826CECE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826CECE8 size=112
    let mut pc: u32 = 0x826CECE8;
    'dispatch: loop {
        match pc {
            0x826CECE8 => {
    //   block [0x826CECE8..0x826CED58)
	// 826CECE8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826CECEC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826CECF0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826CECF4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826CECF8: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826CECFC: 38AA92FC  addi r5, r10, -0x6d04
	ctx.r[5].s64 = ctx.r[10].s64 + -27908;
	// 826CED00: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826CED04: 390BC6F0  addi r8, r11, -0x3910
	ctx.r[8].s64 = ctx.r[11].s64 + -14608;
	// 826CED08: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826CED0C: 388A3EAC  addi r4, r10, 0x3eac
	ctx.r[4].s64 = ctx.r[10].s64 + 16044;
	// 826CED10: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826CED14: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826CED18: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826CED1C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826CED20: 386A977C  addi r3, r10, -0x6884
	ctx.r[3].s64 = ctx.r[10].s64 + -26756;
	// 826CED24: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826CED28: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826CED2C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826CED30: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826CED34: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826CED38: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826CED3C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826CED40: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826CED44: 4BD980DD  bl 0x82466e20
	ctx.lr = 0x826CED48;
	sub_82466E20(ctx, base);
	// 826CED48: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826CED4C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826CED50: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826CED54: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826CED58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826CED58 size=116
    let mut pc: u32 = 0x826CED58;
    'dispatch: loop {
        match pc {
            0x826CED58 => {
    //   block [0x826CED58..0x826CEDCC)
	// 826CED58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826CED5C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826CED60: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826CED64: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826CED68: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 826CED6C: 390BC720  addi r8, r11, -0x38e0
	ctx.r[8].s64 = ctx.r[11].s64 + -14560;
	// 826CED70: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826CED74: 392A371C  addi r9, r10, 0x371c
	ctx.r[9].s64 = ctx.r[10].s64 + 14108;
	// 826CED78: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826CED7C: 38E00002  li r7, 2
	ctx.r[7].s64 = 2;
	// 826CED80: 38AA98FC  addi r5, r10, -0x6704
	ctx.r[5].s64 = ctx.r[10].s64 + -26372;
	// 826CED84: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826CED88: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826CED8C: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826CED90: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826CED94: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826CED98: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826CED9C: 3D608299  lis r11, -0x7d67
	ctx.r[11].s64 = -2103902208;
	// 826CEDA0: 388A3EBC  addi r4, r10, 0x3ebc
	ctx.r[4].s64 = ctx.r[10].s64 + 16060;
	// 826CEDA4: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826CEDA8: 386B97AC  addi r3, r11, -0x6854
	ctx.r[3].s64 = ctx.r[11].s64 + -26708;
	// 826CEDAC: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826CEDB0: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826CEDB4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826CEDB8: 4BD98069  bl 0x82466e20
	ctx.lr = 0x826CEDBC;
	sub_82466E20(ctx, base);
	// 826CEDBC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826CEDC0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826CEDC4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826CEDC8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826CEDD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826CEDD0 size=100
    let mut pc: u32 = 0x826CEDD0;
    'dispatch: loop {
        match pc {
            0x826CEDD0 => {
    //   block [0x826CEDD0..0x826CEE34)
	// 826CEDD0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826CEDD4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826CEDD8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826CEDDC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826CEDE0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826CEDE4: 38AA92FC  addi r5, r10, -0x6d04
	ctx.r[5].s64 = ctx.r[10].s64 + -27908;
	// 826CEDE8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826CEDEC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826CEDF0: 388A3F14  addi r4, r10, 0x3f14
	ctx.r[4].s64 = ctx.r[10].s64 + 16148;
	// 826CEDF4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826CEDF8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826CEDFC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826CEE00: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826CEE04: 386A97DC  addi r3, r10, -0x6824
	ctx.r[3].s64 = ctx.r[10].s64 + -26660;
	// 826CEE08: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826CEE0C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826CEE10: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826CEE14: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826CEE18: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826CEE1C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826CEE20: 4BD98001  bl 0x82466e20
	ctx.lr = 0x826CEE24;
	sub_82466E20(ctx, base);
	// 826CEE24: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826CEE28: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826CEE2C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826CEE30: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826CEE38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826CEE38 size=112
    let mut pc: u32 = 0x826CEE38;
    'dispatch: loop {
        match pc {
            0x826CEE38 => {
    //   block [0x826CEE38..0x826CEEA8)
	// 826CEE38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826CEE3C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826CEE40: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826CEE44: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826CEE48: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826CEE4C: 38AA95FC  addi r5, r10, -0x6a04
	ctx.r[5].s64 = ctx.r[10].s64 + -27140;
	// 826CEE50: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826CEE54: 390BC750  addi r8, r11, -0x38b0
	ctx.r[8].s64 = ctx.r[11].s64 + -14512;
	// 826CEE58: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 826CEE5C: 388A3F2C  addi r4, r10, 0x3f2c
	ctx.r[4].s64 = ctx.r[10].s64 + 16172;
	// 826CEE60: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826CEE64: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826CEE68: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826CEE6C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826CEE70: 386A980C  addi r3, r10, -0x67f4
	ctx.r[3].s64 = ctx.r[10].s64 + -26612;
	// 826CEE74: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826CEE78: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826CEE7C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826CEE80: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826CEE84: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826CEE88: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826CEE8C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826CEE90: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826CEE94: 4BD97F8D  bl 0x82466e20
	ctx.lr = 0x826CEE98;
	sub_82466E20(ctx, base);
	// 826CEE98: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826CEE9C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826CEEA0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826CEEA4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826CEEA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826CEEA8 size=112
    let mut pc: u32 = 0x826CEEA8;
    'dispatch: loop {
        match pc {
            0x826CEEA8 => {
    //   block [0x826CEEA8..0x826CEF18)
	// 826CEEA8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826CEEAC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826CEEB0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826CEEB4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826CEEB8: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826CEEBC: 38AA95FC  addi r5, r10, -0x6a04
	ctx.r[5].s64 = ctx.r[10].s64 + -27140;
	// 826CEEC0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826CEEC4: 390BC798  addi r8, r11, -0x3868
	ctx.r[8].s64 = ctx.r[11].s64 + -14440;
	// 826CEEC8: 39200007  li r9, 7
	ctx.r[9].s64 = 7;
	// 826CEECC: 388A3F3C  addi r4, r10, 0x3f3c
	ctx.r[4].s64 = ctx.r[10].s64 + 16188;
	// 826CEED0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826CEED4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826CEED8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826CEEDC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826CEEE0: 386A983C  addi r3, r10, -0x67c4
	ctx.r[3].s64 = ctx.r[10].s64 + -26564;
	// 826CEEE4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826CEEE8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826CEEEC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826CEEF0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826CEEF4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826CEEF8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826CEEFC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826CEF00: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826CEF04: 4BD97F1D  bl 0x82466e20
	ctx.lr = 0x826CEF08;
	sub_82466E20(ctx, base);
	// 826CEF08: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826CEF0C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826CEF10: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826CEF14: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826CEF18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826CEF18 size=108
    let mut pc: u32 = 0x826CEF18;
    'dispatch: loop {
        match pc {
            0x826CEF18 => {
    //   block [0x826CEF18..0x826CEF84)
	// 826CEF18: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826CEF1C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826CEF20: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826CEF24: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826CEF28: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826CEF2C: 38EBC840  addi r7, r11, -0x37c0
	ctx.r[7].s64 = ctx.r[11].s64 + -14272;
	// 826CEF30: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 826CEF34: 388A3F58  addi r4, r10, 0x3f58
	ctx.r[4].s64 = ctx.r[10].s64 + 16216;
	// 826CEF38: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826CEF3C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826CEF40: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826CEF44: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826CEF48: 386A986C  addi r3, r10, -0x6794
	ctx.r[3].s64 = ctx.r[10].s64 + -26516;
	// 826CEF4C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826CEF50: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826CEF54: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826CEF58: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826CEF5C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826CEF60: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826CEF64: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826CEF68: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826CEF6C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826CEF70: 4BD97EB1  bl 0x82466e20
	ctx.lr = 0x826CEF74;
	sub_82466E20(ctx, base);
	// 826CEF74: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826CEF78: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826CEF7C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826CEF80: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826CEF88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826CEF88 size=112
    let mut pc: u32 = 0x826CEF88;
    'dispatch: loop {
        match pc {
            0x826CEF88 => {
    //   block [0x826CEF88..0x826CEFF8)
	// 826CEF88: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826CEF8C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826CEF90: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826CEF94: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826CEF98: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826CEF9C: 38AA941C  addi r5, r10, -0x6be4
	ctx.r[5].s64 = ctx.r[10].s64 + -27620;
	// 826CEFA0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826CEFA4: 390BC888  addi r8, r11, -0x3778
	ctx.r[8].s64 = ctx.r[11].s64 + -14200;
	// 826CEFA8: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 826CEFAC: 388A3F74  addi r4, r10, 0x3f74
	ctx.r[4].s64 = ctx.r[10].s64 + 16244;
	// 826CEFB0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826CEFB4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826CEFB8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826CEFBC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826CEFC0: 386A989C  addi r3, r10, -0x6764
	ctx.r[3].s64 = ctx.r[10].s64 + -26468;
	// 826CEFC4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826CEFC8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826CEFCC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826CEFD0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826CEFD4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826CEFD8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826CEFDC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826CEFE0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826CEFE4: 4BD97E3D  bl 0x82466e20
	ctx.lr = 0x826CEFE8;
	sub_82466E20(ctx, base);
	// 826CEFE8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826CEFEC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826CEFF0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826CEFF4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826CEFF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826CEFF8 size=100
    let mut pc: u32 = 0x826CEFF8;
    'dispatch: loop {
        match pc {
            0x826CEFF8 => {
    //   block [0x826CEFF8..0x826CF05C)
	// 826CEFF8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826CEFFC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826CF000: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826CF004: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826CF008: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826CF00C: 38AA944C  addi r5, r10, -0x6bb4
	ctx.r[5].s64 = ctx.r[10].s64 + -27572;
	// 826CF010: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826CF014: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826CF018: 388A3F88  addi r4, r10, 0x3f88
	ctx.r[4].s64 = ctx.r[10].s64 + 16264;
	// 826CF01C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826CF020: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826CF024: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826CF028: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826CF02C: 386A98CC  addi r3, r10, -0x6734
	ctx.r[3].s64 = ctx.r[10].s64 + -26420;
	// 826CF030: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826CF034: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826CF038: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826CF03C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826CF040: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826CF044: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826CF048: 4BD97DD9  bl 0x82466e20
	ctx.lr = 0x826CF04C;
	sub_82466E20(ctx, base);
	// 826CF04C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826CF050: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826CF054: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826CF058: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826CF060(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826CF060 size=100
    let mut pc: u32 = 0x826CF060;
    'dispatch: loop {
        match pc {
            0x826CF060 => {
    //   block [0x826CF060..0x826CF0C4)
	// 826CF060: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826CF064: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826CF068: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826CF06C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826CF070: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826CF074: 38AA92FC  addi r5, r10, -0x6d04
	ctx.r[5].s64 = ctx.r[10].s64 + -27908;
	// 826CF078: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826CF07C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826CF080: 388A3F98  addi r4, r10, 0x3f98
	ctx.r[4].s64 = ctx.r[10].s64 + 16280;
	// 826CF084: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826CF088: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826CF08C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826CF090: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826CF094: 386A98FC  addi r3, r10, -0x6704
	ctx.r[3].s64 = ctx.r[10].s64 + -26372;
	// 826CF098: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826CF09C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826CF0A0: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826CF0A4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826CF0A8: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826CF0AC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826CF0B0: 4BD97D71  bl 0x82466e20
	ctx.lr = 0x826CF0B4;
	sub_82466E20(ctx, base);
	// 826CF0B4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826CF0B8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826CF0BC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826CF0C0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826CF0C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826CF0C8 size=108
    let mut pc: u32 = 0x826CF0C8;
    'dispatch: loop {
        match pc {
            0x826CF0C8 => {
    //   block [0x826CF0C8..0x826CF134)
	// 826CF0C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826CF0CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826CF0D0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826CF0D4: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826CF0D8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826CF0DC: 38EBC8E8  addi r7, r11, -0x3718
	ctx.r[7].s64 = ctx.r[11].s64 + -14104;
	// 826CF0E0: 39000006  li r8, 6
	ctx.r[8].s64 = 6;
	// 826CF0E4: 388A4028  addi r4, r10, 0x4028
	ctx.r[4].s64 = ctx.r[10].s64 + 16424;
	// 826CF0E8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826CF0EC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826CF0F0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826CF0F4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826CF0F8: 386A992C  addi r3, r10, -0x66d4
	ctx.r[3].s64 = ctx.r[10].s64 + -26324;
	// 826CF0FC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826CF100: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826CF104: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826CF108: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826CF10C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826CF110: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826CF114: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826CF118: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826CF11C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826CF120: 4BD97D01  bl 0x82466e20
	ctx.lr = 0x826CF124;
	sub_82466E20(ctx, base);
	// 826CF124: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826CF128: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826CF12C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826CF130: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826CF138(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826CF138 size=112
    let mut pc: u32 = 0x826CF138;
    'dispatch: loop {
        match pc {
            0x826CF138 => {
    //   block [0x826CF138..0x826CF1A8)
	// 826CF138: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826CF13C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826CF140: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826CF144: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826CF148: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826CF14C: 38AA96EC  addi r5, r10, -0x6914
	ctx.r[5].s64 = ctx.r[10].s64 + -26900;
	// 826CF150: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826CF154: 390BC978  addi r8, r11, -0x3688
	ctx.r[8].s64 = ctx.r[11].s64 + -13960;
	// 826CF158: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826CF15C: 388A404C  addi r4, r10, 0x404c
	ctx.r[4].s64 = ctx.r[10].s64 + 16460;
	// 826CF160: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826CF164: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826CF168: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826CF16C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826CF170: 386A995C  addi r3, r10, -0x66a4
	ctx.r[3].s64 = ctx.r[10].s64 + -26276;
	// 826CF174: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826CF178: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826CF17C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826CF180: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826CF184: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826CF188: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826CF18C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826CF190: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826CF194: 4BD97C8D  bl 0x82466e20
	ctx.lr = 0x826CF198;
	sub_82466E20(ctx, base);
	// 826CF198: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826CF19C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826CF1A0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826CF1A4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826CF1A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826CF1A8 size=112
    let mut pc: u32 = 0x826CF1A8;
    'dispatch: loop {
        match pc {
            0x826CF1A8 => {
    //   block [0x826CF1A8..0x826CF218)
	// 826CF1A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826CF1AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826CF1B0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826CF1B4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826CF1B8: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826CF1BC: 38AA983C  addi r5, r10, -0x67c4
	ctx.r[5].s64 = ctx.r[10].s64 + -26564;
	// 826CF1C0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826CF1C4: 390BC990  addi r8, r11, -0x3670
	ctx.r[8].s64 = ctx.r[11].s64 + -13936;
	// 826CF1C8: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826CF1CC: 388A4060  addi r4, r10, 0x4060
	ctx.r[4].s64 = ctx.r[10].s64 + 16480;
	// 826CF1D0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826CF1D4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826CF1D8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826CF1DC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826CF1E0: 386A998C  addi r3, r10, -0x6674
	ctx.r[3].s64 = ctx.r[10].s64 + -26228;
	// 826CF1E4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826CF1E8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826CF1EC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826CF1F0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826CF1F4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826CF1F8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826CF1FC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826CF200: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826CF204: 4BD97C1D  bl 0x82466e20
	ctx.lr = 0x826CF208;
	sub_82466E20(ctx, base);
	// 826CF208: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826CF20C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826CF210: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826CF214: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826CF218(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826CF218 size=112
    let mut pc: u32 = 0x826CF218;
    'dispatch: loop {
        match pc {
            0x826CF218 => {
    //   block [0x826CF218..0x826CF288)
	// 826CF218: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826CF21C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826CF220: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826CF224: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826CF228: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826CF22C: 38AA92FC  addi r5, r10, -0x6d04
	ctx.r[5].s64 = ctx.r[10].s64 + -27908;
	// 826CF230: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826CF234: 390BC9C0  addi r8, r11, -0x3640
	ctx.r[8].s64 = ctx.r[11].s64 + -13888;
	// 826CF238: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 826CF23C: 388A4084  addi r4, r10, 0x4084
	ctx.r[4].s64 = ctx.r[10].s64 + 16516;
	// 826CF240: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826CF244: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826CF248: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826CF24C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826CF250: 386A99BC  addi r3, r10, -0x6644
	ctx.r[3].s64 = ctx.r[10].s64 + -26180;
	// 826CF254: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826CF258: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826CF25C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826CF260: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826CF264: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826CF268: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826CF26C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826CF270: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826CF274: 4BD97BAD  bl 0x82466e20
	ctx.lr = 0x826CF278;
	sub_82466E20(ctx, base);
	// 826CF278: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826CF27C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826CF280: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826CF284: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826CF288(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826CF288 size=112
    let mut pc: u32 = 0x826CF288;
    'dispatch: loop {
        match pc {
            0x826CF288 => {
    //   block [0x826CF288..0x826CF2F8)
	// 826CF288: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826CF28C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826CF290: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826CF294: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826CF298: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826CF29C: 38AA944C  addi r5, r10, -0x6bb4
	ctx.r[5].s64 = ctx.r[10].s64 + -27572;
	// 826CF2A0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826CF2A4: 390BCA08  addi r8, r11, -0x35f8
	ctx.r[8].s64 = ctx.r[11].s64 + -13816;
	// 826CF2A8: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 826CF2AC: 388A40B8  addi r4, r10, 0x40b8
	ctx.r[4].s64 = ctx.r[10].s64 + 16568;
	// 826CF2B0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826CF2B4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826CF2B8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826CF2BC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826CF2C0: 386A99EC  addi r3, r10, -0x6614
	ctx.r[3].s64 = ctx.r[10].s64 + -26132;
	// 826CF2C4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826CF2C8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826CF2CC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826CF2D0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826CF2D4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826CF2D8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826CF2DC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826CF2E0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826CF2E4: 4BD97B3D  bl 0x82466e20
	ctx.lr = 0x826CF2E8;
	sub_82466E20(ctx, base);
	// 826CF2E8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826CF2EC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826CF2F0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826CF2F4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826CF2F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826CF2F8 size=108
    let mut pc: u32 = 0x826CF2F8;
    'dispatch: loop {
        match pc {
            0x826CF2F8 => {
    //   block [0x826CF2F8..0x826CF364)
	// 826CF2F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826CF2FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826CF300: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826CF304: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826CF308: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826CF30C: 38EBCA50  addi r7, r11, -0x35b0
	ctx.r[7].s64 = ctx.r[11].s64 + -13744;
	// 826CF310: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 826CF314: 388A21D8  addi r4, r10, 0x21d8
	ctx.r[4].s64 = ctx.r[10].s64 + 8664;
	// 826CF318: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826CF31C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826CF320: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826CF324: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826CF328: 386A9A1C  addi r3, r10, -0x65e4
	ctx.r[3].s64 = ctx.r[10].s64 + -26084;
	// 826CF32C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826CF330: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826CF334: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826CF338: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826CF33C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826CF340: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826CF344: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826CF348: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826CF34C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826CF350: 4BD97AD1  bl 0x82466e20
	ctx.lr = 0x826CF354;
	sub_82466E20(ctx, base);
	// 826CF354: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826CF358: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826CF35C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826CF360: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826CF368(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826CF368 size=112
    let mut pc: u32 = 0x826CF368;
    'dispatch: loop {
        match pc {
            0x826CF368 => {
    //   block [0x826CF368..0x826CF3D8)
	// 826CF368: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826CF36C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826CF370: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826CF374: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826CF378: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826CF37C: 38AA93BC  addi r5, r10, -0x6c44
	ctx.r[5].s64 = ctx.r[10].s64 + -27716;
	// 826CF380: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826CF384: 390BCA98  addi r8, r11, -0x3568
	ctx.r[8].s64 = ctx.r[11].s64 + -13672;
	// 826CF388: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826CF38C: 388A40C8  addi r4, r10, 0x40c8
	ctx.r[4].s64 = ctx.r[10].s64 + 16584;
	// 826CF390: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826CF394: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826CF398: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826CF39C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826CF3A0: 386A9A4C  addi r3, r10, -0x65b4
	ctx.r[3].s64 = ctx.r[10].s64 + -26036;
	// 826CF3A4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826CF3A8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826CF3AC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826CF3B0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826CF3B4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826CF3B8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826CF3BC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826CF3C0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826CF3C4: 4BD97A5D  bl 0x82466e20
	ctx.lr = 0x826CF3C8;
	sub_82466E20(ctx, base);
	// 826CF3C8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826CF3CC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826CF3D0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826CF3D4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826CF3D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826CF3D8 size=112
    let mut pc: u32 = 0x826CF3D8;
    'dispatch: loop {
        match pc {
            0x826CF3D8 => {
    //   block [0x826CF3D8..0x826CF448)
	// 826CF3D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826CF3DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826CF3E0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826CF3E4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826CF3E8: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826CF3EC: 38AA941C  addi r5, r10, -0x6be4
	ctx.r[5].s64 = ctx.r[10].s64 + -27620;
	// 826CF3F0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826CF3F4: 390BCAB0  addi r8, r11, -0x3550
	ctx.r[8].s64 = ctx.r[11].s64 + -13648;
	// 826CF3F8: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826CF3FC: 388A40EC  addi r4, r10, 0x40ec
	ctx.r[4].s64 = ctx.r[10].s64 + 16620;
	// 826CF400: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826CF404: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826CF408: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826CF40C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826CF410: 386A9A7C  addi r3, r10, -0x6584
	ctx.r[3].s64 = ctx.r[10].s64 + -25988;
	// 826CF414: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826CF418: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826CF41C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826CF420: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826CF424: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826CF428: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826CF42C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826CF430: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826CF434: 4BD979ED  bl 0x82466e20
	ctx.lr = 0x826CF438;
	sub_82466E20(ctx, base);
	// 826CF438: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826CF43C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826CF440: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826CF444: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826CF448(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826CF448 size=108
    let mut pc: u32 = 0x826CF448;
    'dispatch: loop {
        match pc {
            0x826CF448 => {
    //   block [0x826CF448..0x826CF4B4)
	// 826CF448: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826CF44C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826CF450: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826CF454: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826CF458: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826CF45C: 38EBCAE0  addi r7, r11, -0x3520
	ctx.r[7].s64 = ctx.r[11].s64 + -13600;
	// 826CF460: 3900000A  li r8, 0xa
	ctx.r[8].s64 = 10;
	// 826CF464: 388A54F0  addi r4, r10, 0x54f0
	ctx.r[4].s64 = ctx.r[10].s64 + 21744;
	// 826CF468: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826CF46C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826CF470: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826CF474: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826CF478: 386A9AAC  addi r3, r10, -0x6554
	ctx.r[3].s64 = ctx.r[10].s64 + -25940;
	// 826CF47C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826CF480: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826CF484: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826CF488: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826CF48C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826CF490: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826CF494: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826CF498: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826CF49C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826CF4A0: 4BD97981  bl 0x82466e20
	ctx.lr = 0x826CF4A4;
	sub_82466E20(ctx, base);
	// 826CF4A4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826CF4A8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826CF4AC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826CF4B0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826CF4B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826CF4B8 size=108
    let mut pc: u32 = 0x826CF4B8;
    'dispatch: loop {
        match pc {
            0x826CF4B8 => {
    //   block [0x826CF4B8..0x826CF524)
	// 826CF4B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826CF4BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826CF4C0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826CF4C4: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826CF4C8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826CF4CC: 38EBCBD0  addi r7, r11, -0x3430
	ctx.r[7].s64 = ctx.r[11].s64 + -13360;
	// 826CF4D0: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 826CF4D4: 388A551C  addi r4, r10, 0x551c
	ctx.r[4].s64 = ctx.r[10].s64 + 21788;
	// 826CF4D8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826CF4DC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826CF4E0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826CF4E4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826CF4E8: 386A9ADC  addi r3, r10, -0x6524
	ctx.r[3].s64 = ctx.r[10].s64 + -25892;
	// 826CF4EC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826CF4F0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826CF4F4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826CF4F8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826CF4FC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826CF500: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826CF504: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826CF508: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826CF50C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826CF510: 4BD97911  bl 0x82466e20
	ctx.lr = 0x826CF514;
	sub_82466E20(ctx, base);
	// 826CF514: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826CF518: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826CF51C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826CF520: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826CF528(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826CF528 size=108
    let mut pc: u32 = 0x826CF528;
    'dispatch: loop {
        match pc {
            0x826CF528 => {
    //   block [0x826CF528..0x826CF594)
	// 826CF528: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826CF52C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826CF530: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826CF534: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826CF538: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826CF53C: 38EBCC18  addi r7, r11, -0x33e8
	ctx.r[7].s64 = ctx.r[11].s64 + -13288;
	// 826CF540: 39000009  li r8, 9
	ctx.r[8].s64 = 9;
	// 826CF544: 388A553C  addi r4, r10, 0x553c
	ctx.r[4].s64 = ctx.r[10].s64 + 21820;
	// 826CF548: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826CF54C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826CF550: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826CF554: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826CF558: 386A9B0C  addi r3, r10, -0x64f4
	ctx.r[3].s64 = ctx.r[10].s64 + -25844;
	// 826CF55C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826CF560: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826CF564: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826CF568: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826CF56C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826CF570: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826CF574: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826CF578: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826CF57C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826CF580: 4BD978A1  bl 0x82466e20
	ctx.lr = 0x826CF584;
	sub_82466E20(ctx, base);
	// 826CF584: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826CF588: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826CF58C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826CF590: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826CF598(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826CF598 size=108
    let mut pc: u32 = 0x826CF598;
    'dispatch: loop {
        match pc {
            0x826CF598 => {
    //   block [0x826CF598..0x826CF604)
	// 826CF598: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826CF59C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826CF5A0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826CF5A4: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826CF5A8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826CF5AC: 38EBCCF0  addi r7, r11, -0x3310
	ctx.r[7].s64 = ctx.r[11].s64 + -13072;
	// 826CF5B0: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826CF5B4: 388A5560  addi r4, r10, 0x5560
	ctx.r[4].s64 = ctx.r[10].s64 + 21856;
	// 826CF5B8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826CF5BC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826CF5C0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826CF5C4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826CF5C8: 386A9B3C  addi r3, r10, -0x64c4
	ctx.r[3].s64 = ctx.r[10].s64 + -25796;
	// 826CF5CC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826CF5D0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826CF5D4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826CF5D8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826CF5DC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826CF5E0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826CF5E4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826CF5E8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826CF5EC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826CF5F0: 4BD97831  bl 0x82466e20
	ctx.lr = 0x826CF5F4;
	sub_82466E20(ctx, base);
	// 826CF5F4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826CF5F8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826CF5FC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826CF600: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826CF608(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826CF608 size=112
    let mut pc: u32 = 0x826CF608;
    'dispatch: loop {
        match pc {
            0x826CF608 => {
    //   block [0x826CF608..0x826CF678)
	// 826CF608: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826CF60C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826CF610: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826CF614: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826CF618: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826CF61C: 38AA86FC  addi r5, r10, -0x7904
	ctx.r[5].s64 = ctx.r[10].s64 + -30980;
	// 826CF620: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826CF624: 390BCD08  addi r8, r11, -0x32f8
	ctx.r[8].s64 = ctx.r[11].s64 + -13048;
	// 826CF628: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 826CF62C: 388A444C  addi r4, r10, 0x444c
	ctx.r[4].s64 = ctx.r[10].s64 + 17484;
	// 826CF630: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826CF634: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826CF638: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826CF63C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826CF640: 386A9B6C  addi r3, r10, -0x6494
	ctx.r[3].s64 = ctx.r[10].s64 + -25748;
	// 826CF644: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826CF648: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826CF64C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826CF650: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826CF654: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826CF658: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826CF65C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826CF660: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826CF664: 4BD977BD  bl 0x82466e20
	ctx.lr = 0x826CF668;
	sub_82466E20(ctx, base);
	// 826CF668: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826CF66C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826CF670: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826CF674: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826CF678(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826CF678 size=112
    let mut pc: u32 = 0x826CF678;
    'dispatch: loop {
        match pc {
            0x826CF678 => {
    //   block [0x826CF678..0x826CF6E8)
	// 826CF678: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826CF67C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826CF680: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826CF684: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826CF688: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826CF68C: 38AA9B6C  addi r5, r10, -0x6494
	ctx.r[5].s64 = ctx.r[10].s64 + -25748;
	// 826CF690: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826CF694: 390BCD68  addi r8, r11, -0x3298
	ctx.r[8].s64 = ctx.r[11].s64 + -12952;
	// 826CF698: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826CF69C: 388A4458  addi r4, r10, 0x4458
	ctx.r[4].s64 = ctx.r[10].s64 + 17496;
	// 826CF6A0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826CF6A4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826CF6A8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826CF6AC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826CF6B0: 386A9B9C  addi r3, r10, -0x6464
	ctx.r[3].s64 = ctx.r[10].s64 + -25700;
	// 826CF6B4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826CF6B8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826CF6BC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826CF6C0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826CF6C4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826CF6C8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826CF6CC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826CF6D0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826CF6D4: 4BD9774D  bl 0x82466e20
	ctx.lr = 0x826CF6D8;
	sub_82466E20(ctx, base);
	// 826CF6D8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826CF6DC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826CF6E0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826CF6E4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826CF6E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826CF6E8 size=112
    let mut pc: u32 = 0x826CF6E8;
    'dispatch: loop {
        match pc {
            0x826CF6E8 => {
    //   block [0x826CF6E8..0x826CF758)
	// 826CF6E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826CF6EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826CF6F0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826CF6F4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826CF6F8: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826CF6FC: 38AA9B6C  addi r5, r10, -0x6494
	ctx.r[5].s64 = ctx.r[10].s64 + -25748;
	// 826CF700: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826CF704: 390BCD80  addi r8, r11, -0x3280
	ctx.r[8].s64 = ctx.r[11].s64 + -12928;
	// 826CF708: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826CF70C: 388A4468  addi r4, r10, 0x4468
	ctx.r[4].s64 = ctx.r[10].s64 + 17512;
	// 826CF710: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826CF714: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826CF718: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826CF71C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826CF720: 386A9BCC  addi r3, r10, -0x6434
	ctx.r[3].s64 = ctx.r[10].s64 + -25652;
	// 826CF724: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826CF728: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826CF72C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826CF730: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826CF734: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826CF738: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826CF73C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826CF740: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826CF744: 4BD976DD  bl 0x82466e20
	ctx.lr = 0x826CF748;
	sub_82466E20(ctx, base);
	// 826CF748: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826CF74C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826CF750: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826CF754: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826CF758(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826CF758 size=112
    let mut pc: u32 = 0x826CF758;
    'dispatch: loop {
        match pc {
            0x826CF758 => {
    //   block [0x826CF758..0x826CF7C8)
	// 826CF758: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826CF75C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826CF760: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826CF764: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826CF768: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826CF76C: 38AA9B6C  addi r5, r10, -0x6494
	ctx.r[5].s64 = ctx.r[10].s64 + -25748;
	// 826CF770: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826CF774: 390BCDB0  addi r8, r11, -0x3250
	ctx.r[8].s64 = ctx.r[11].s64 + -12880;
	// 826CF778: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826CF77C: 388A4478  addi r4, r10, 0x4478
	ctx.r[4].s64 = ctx.r[10].s64 + 17528;
	// 826CF780: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826CF784: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826CF788: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826CF78C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826CF790: 386A9BFC  addi r3, r10, -0x6404
	ctx.r[3].s64 = ctx.r[10].s64 + -25604;
	// 826CF794: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826CF798: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826CF79C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826CF7A0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826CF7A4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826CF7A8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826CF7AC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826CF7B0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826CF7B4: 4BD9766D  bl 0x82466e20
	ctx.lr = 0x826CF7B8;
	sub_82466E20(ctx, base);
	// 826CF7B8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826CF7BC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826CF7C0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826CF7C4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826CF7C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826CF7C8 size=24
    let mut pc: u32 = 0x826CF7C8;
    'dispatch: loop {
        match pc {
            0x826CF7C8 => {
    //   block [0x826CF7C8..0x826CF7E0)
	// 826CF7C8: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826CF7CC: 3D40827F  lis r10, -0x7d81
	ctx.r[10].s64 = -2105606144;
	// 826CF7D0: 394AF720  addi r10, r10, -0x8e0
	ctx.r[10].s64 = ctx.r[10].s64 + -2272;
	// 826CF7D4: 816BCDC8  lwz r11, -0x3238(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-12856 as u32) ) } as u64;
	// 826CF7D8: 916A0008  stw r11, 8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 826CF7DC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826CF7E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826CF7E0 size=112
    let mut pc: u32 = 0x826CF7E0;
    'dispatch: loop {
        match pc {
            0x826CF7E0 => {
    //   block [0x826CF7E0..0x826CF850)
	// 826CF7E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826CF7E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826CF7E8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826CF7EC: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 826CF7F0: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826CF7F4: 392A3758  addi r9, r10, 0x3758
	ctx.r[9].s64 = ctx.r[10].s64 + 14168;
	// 826CF7F8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826CF7FC: 390BF720  addi r8, r11, -0x8e0
	ctx.r[8].s64 = ctx.r[11].s64 + -2272;
	// 826CF800: 38E00003  li r7, 3
	ctx.r[7].s64 = 3;
	// 826CF804: 388A4488  addi r4, r10, 0x4488
	ctx.r[4].s64 = ctx.r[10].s64 + 17544;
	// 826CF808: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826CF80C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826CF810: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826CF814: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826CF818: 386A9C2C  addi r3, r10, -0x63d4
	ctx.r[3].s64 = ctx.r[10].s64 + -25556;
	// 826CF81C: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826CF820: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826CF824: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826CF828: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826CF82C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826CF830: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826CF834: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826CF838: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826CF83C: 4BD975E5  bl 0x82466e20
	ctx.lr = 0x826CF840;
	sub_82466E20(ctx, base);
	// 826CF840: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826CF844: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826CF848: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826CF84C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826CF850(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826CF850 size=108
    let mut pc: u32 = 0x826CF850;
    'dispatch: loop {
        match pc {
            0x826CF850 => {
    //   block [0x826CF850..0x826CF8BC)
	// 826CF850: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826CF854: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826CF858: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826CF85C: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826CF860: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826CF864: 38EBCDCC  addi r7, r11, -0x3234
	ctx.r[7].s64 = ctx.r[11].s64 + -12852;
	// 826CF868: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826CF86C: 388A4494  addi r4, r10, 0x4494
	ctx.r[4].s64 = ctx.r[10].s64 + 17556;
	// 826CF870: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826CF874: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826CF878: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826CF87C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826CF880: 386A9C5C  addi r3, r10, -0x63a4
	ctx.r[3].s64 = ctx.r[10].s64 + -25508;
	// 826CF884: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826CF888: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826CF88C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826CF890: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826CF894: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826CF898: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826CF89C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826CF8A0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826CF8A4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826CF8A8: 4BD97579  bl 0x82466e20
	ctx.lr = 0x826CF8AC;
	sub_82466E20(ctx, base);
	// 826CF8AC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826CF8B0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826CF8B4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826CF8B8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826CF8C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826CF8C0 size=108
    let mut pc: u32 = 0x826CF8C0;
    'dispatch: loop {
        match pc {
            0x826CF8C0 => {
    //   block [0x826CF8C0..0x826CF92C)
	// 826CF8C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826CF8C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826CF8C8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826CF8CC: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826CF8D0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826CF8D4: 38EBCDE8  addi r7, r11, -0x3218
	ctx.r[7].s64 = ctx.r[11].s64 + -12824;
	// 826CF8D8: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 826CF8DC: 388A44A4  addi r4, r10, 0x44a4
	ctx.r[4].s64 = ctx.r[10].s64 + 17572;
	// 826CF8E0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826CF8E4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826CF8E8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826CF8EC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826CF8F0: 386A9C8C  addi r3, r10, -0x6374
	ctx.r[3].s64 = ctx.r[10].s64 + -25460;
	// 826CF8F4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826CF8F8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826CF8FC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826CF900: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826CF904: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826CF908: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826CF90C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826CF910: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826CF914: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826CF918: 4BD97509  bl 0x82466e20
	ctx.lr = 0x826CF91C;
	sub_82466E20(ctx, base);
	// 826CF91C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826CF920: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826CF924: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826CF928: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826CF930(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826CF930 size=112
    let mut pc: u32 = 0x826CF930;
    'dispatch: loop {
        match pc {
            0x826CF930 => {
    //   block [0x826CF930..0x826CF9A0)
	// 826CF930: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826CF934: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826CF938: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826CF93C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826CF940: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826CF944: 38AA86FC  addi r5, r10, -0x7904
	ctx.r[5].s64 = ctx.r[10].s64 + -30980;
	// 826CF948: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826CF94C: 390BCE30  addi r8, r11, -0x31d0
	ctx.r[8].s64 = ctx.r[11].s64 + -12752;
	// 826CF950: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826CF954: 388A44B0  addi r4, r10, 0x44b0
	ctx.r[4].s64 = ctx.r[10].s64 + 17584;
	// 826CF958: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826CF95C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826CF960: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826CF964: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826CF968: 386A9CBC  addi r3, r10, -0x6344
	ctx.r[3].s64 = ctx.r[10].s64 + -25412;
	// 826CF96C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826CF970: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826CF974: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826CF978: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826CF97C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826CF980: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826CF984: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826CF988: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826CF98C: 4BD97495  bl 0x82466e20
	ctx.lr = 0x826CF990;
	sub_82466E20(ctx, base);
	// 826CF990: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826CF994: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826CF998: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826CF99C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826CF9A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826CF9A0 size=108
    let mut pc: u32 = 0x826CF9A0;
    'dispatch: loop {
        match pc {
            0x826CF9A0 => {
    //   block [0x826CF9A0..0x826CFA0C)
	// 826CF9A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826CF9A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826CF9A8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826CF9AC: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826CF9B0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826CF9B4: 38EBCE48  addi r7, r11, -0x31b8
	ctx.r[7].s64 = ctx.r[11].s64 + -12728;
	// 826CF9B8: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 826CF9BC: 388A21EC  addi r4, r10, 0x21ec
	ctx.r[4].s64 = ctx.r[10].s64 + 8684;
	// 826CF9C0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826CF9C4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826CF9C8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826CF9CC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826CF9D0: 386A9CEC  addi r3, r10, -0x6314
	ctx.r[3].s64 = ctx.r[10].s64 + -25364;
	// 826CF9D4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826CF9D8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826CF9DC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826CF9E0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826CF9E4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826CF9E8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826CF9EC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826CF9F0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826CF9F4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826CF9F8: 4BD97429  bl 0x82466e20
	ctx.lr = 0x826CF9FC;
	sub_82466E20(ctx, base);
	// 826CF9FC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826CFA00: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826CFA04: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826CFA08: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826CFA10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826CFA10 size=24
    let mut pc: u32 = 0x826CFA10;
    'dispatch: loop {
        match pc {
            0x826CFA10 => {
    //   block [0x826CFA10..0x826CFA28)
	// 826CFA10: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826CFA14: 3D40827F  lis r10, -0x7d81
	ctx.r[10].s64 = -2105606144;
	// 826CFA18: 394AF768  addi r10, r10, -0x898
	ctx.r[10].s64 = ctx.r[10].s64 + -2200;
	// 826CFA1C: 816BCEA8  lwz r11, -0x3158(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-12632 as u32) ) } as u64;
	// 826CFA20: 916A0050  stw r11, 0x50(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 826CFA24: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826CFA28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826CFA28 size=116
    let mut pc: u32 = 0x826CFA28;
    'dispatch: loop {
        match pc {
            0x826CFA28 => {
    //   block [0x826CFA28..0x826CFA9C)
	// 826CFA28: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826CFA2C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826CFA30: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826CFA34: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826CFA38: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 826CFA3C: 390BF768  addi r8, r11, -0x898
	ctx.r[8].s64 = ctx.r[11].s64 + -2200;
	// 826CFA40: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826CFA44: 392A3820  addi r9, r10, 0x3820
	ctx.r[9].s64 = ctx.r[10].s64 + 14368;
	// 826CFA48: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826CFA4C: 38E00008  li r7, 8
	ctx.r[7].s64 = 8;
	// 826CFA50: 38AA86FC  addi r5, r10, -0x7904
	ctx.r[5].s64 = ctx.r[10].s64 + -30980;
	// 826CFA54: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826CFA58: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826CFA5C: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826CFA60: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826CFA64: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826CFA68: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826CFA6C: 3D608299  lis r11, -0x7d67
	ctx.r[11].s64 = -2103902208;
	// 826CFA70: 388A44DC  addi r4, r10, 0x44dc
	ctx.r[4].s64 = ctx.r[10].s64 + 17628;
	// 826CFA74: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826CFA78: 386B9D1C  addi r3, r11, -0x62e4
	ctx.r[3].s64 = ctx.r[11].s64 + -25316;
	// 826CFA7C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826CFA80: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826CFA84: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826CFA88: 4BD97399  bl 0x82466e20
	ctx.lr = 0x826CFA8C;
	sub_82466E20(ctx, base);
	// 826CFA8C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826CFA90: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826CFA94: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826CFA98: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826CFAA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826CFAA0 size=112
    let mut pc: u32 = 0x826CFAA0;
    'dispatch: loop {
        match pc {
            0x826CFAA0 => {
    //   block [0x826CFAA0..0x826CFB10)
	// 826CFAA0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826CFAA4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826CFAA8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826CFAAC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826CFAB0: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826CFAB4: 38AA9CBC  addi r5, r10, -0x6344
	ctx.r[5].s64 = ctx.r[10].s64 + -25412;
	// 826CFAB8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826CFABC: 390BCEAC  addi r8, r11, -0x3154
	ctx.r[8].s64 = ctx.r[11].s64 + -12628;
	// 826CFAC0: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826CFAC4: 388A4518  addi r4, r10, 0x4518
	ctx.r[4].s64 = ctx.r[10].s64 + 17688;
	// 826CFAC8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826CFACC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826CFAD0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826CFAD4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826CFAD8: 386A9D4C  addi r3, r10, -0x62b4
	ctx.r[3].s64 = ctx.r[10].s64 + -25268;
	// 826CFADC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826CFAE0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826CFAE4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826CFAE8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826CFAEC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826CFAF0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826CFAF4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826CFAF8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826CFAFC: 4BD97325  bl 0x82466e20
	ctx.lr = 0x826CFB00;
	sub_82466E20(ctx, base);
	// 826CFB00: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826CFB04: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826CFB08: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826CFB0C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826CFB10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826CFB10 size=108
    let mut pc: u32 = 0x826CFB10;
    'dispatch: loop {
        match pc {
            0x826CFB10 => {
    //   block [0x826CFB10..0x826CFB7C)
	// 826CFB10: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826CFB14: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826CFB18: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826CFB1C: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826CFB20: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826CFB24: 38EBCEE0  addi r7, r11, -0x3120
	ctx.r[7].s64 = ctx.r[11].s64 + -12576;
	// 826CFB28: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 826CFB2C: 388A2224  addi r4, r10, 0x2224
	ctx.r[4].s64 = ctx.r[10].s64 + 8740;
	// 826CFB30: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826CFB34: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826CFB38: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826CFB3C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826CFB40: 386A9D7C  addi r3, r10, -0x6284
	ctx.r[3].s64 = ctx.r[10].s64 + -25220;
	// 826CFB44: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826CFB48: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826CFB4C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826CFB50: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826CFB54: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826CFB58: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826CFB5C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826CFB60: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826CFB64: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826CFB68: 4BD972B9  bl 0x82466e20
	ctx.lr = 0x826CFB6C;
	sub_82466E20(ctx, base);
	// 826CFB6C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826CFB70: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826CFB74: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826CFB78: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826CFB80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826CFB80 size=108
    let mut pc: u32 = 0x826CFB80;
    'dispatch: loop {
        match pc {
            0x826CFB80 => {
    //   block [0x826CFB80..0x826CFBEC)
	// 826CFB80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826CFB84: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826CFB88: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826CFB8C: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826CFB90: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826CFB94: 38EBCF28  addi r7, r11, -0x30d8
	ctx.r[7].s64 = ctx.r[11].s64 + -12504;
	// 826CFB98: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826CFB9C: 388A224C  addi r4, r10, 0x224c
	ctx.r[4].s64 = ctx.r[10].s64 + 8780;
	// 826CFBA0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826CFBA4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826CFBA8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826CFBAC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826CFBB0: 386A9DAC  addi r3, r10, -0x6254
	ctx.r[3].s64 = ctx.r[10].s64 + -25172;
	// 826CFBB4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826CFBB8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826CFBBC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826CFBC0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826CFBC4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826CFBC8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826CFBCC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826CFBD0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826CFBD4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826CFBD8: 4BD97249  bl 0x82466e20
	ctx.lr = 0x826CFBDC;
	sub_82466E20(ctx, base);
	// 826CFBDC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826CFBE0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826CFBE4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826CFBE8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826CFBF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826CFBF0 size=112
    let mut pc: u32 = 0x826CFBF0;
    'dispatch: loop {
        match pc {
            0x826CFBF0 => {
    //   block [0x826CFBF0..0x826CFC60)
	// 826CFBF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826CFBF4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826CFBF8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826CFBFC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826CFC00: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826CFC04: 38AA9CBC  addi r5, r10, -0x6344
	ctx.r[5].s64 = ctx.r[10].s64 + -25412;
	// 826CFC08: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826CFC0C: 390BCF58  addi r8, r11, -0x30a8
	ctx.r[8].s64 = ctx.r[11].s64 + -12456;
	// 826CFC10: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826CFC14: 388A4554  addi r4, r10, 0x4554
	ctx.r[4].s64 = ctx.r[10].s64 + 17748;
	// 826CFC18: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826CFC1C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826CFC20: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826CFC24: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826CFC28: 386A9DDC  addi r3, r10, -0x6224
	ctx.r[3].s64 = ctx.r[10].s64 + -25124;
	// 826CFC2C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826CFC30: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826CFC34: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826CFC38: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826CFC3C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826CFC40: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826CFC44: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826CFC48: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826CFC4C: 4BD971D5  bl 0x82466e20
	ctx.lr = 0x826CFC50;
	sub_82466E20(ctx, base);
	// 826CFC50: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826CFC54: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826CFC58: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826CFC5C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826CFC60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826CFC60 size=108
    let mut pc: u32 = 0x826CFC60;
    'dispatch: loop {
        match pc {
            0x826CFC60 => {
    //   block [0x826CFC60..0x826CFCCC)
	// 826CFC60: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826CFC64: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826CFC68: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826CFC6C: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826CFC70: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826CFC74: 38EBCF88  addi r7, r11, -0x3078
	ctx.r[7].s64 = ctx.r[11].s64 + -12408;
	// 826CFC78: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 826CFC7C: 388A2274  addi r4, r10, 0x2274
	ctx.r[4].s64 = ctx.r[10].s64 + 8820;
	// 826CFC80: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826CFC84: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826CFC88: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826CFC8C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826CFC90: 386A9E0C  addi r3, r10, -0x61f4
	ctx.r[3].s64 = ctx.r[10].s64 + -25076;
	// 826CFC94: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826CFC98: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826CFC9C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826CFCA0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826CFCA4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826CFCA8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826CFCAC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826CFCB0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826CFCB4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826CFCB8: 4BD97169  bl 0x82466e20
	ctx.lr = 0x826CFCBC;
	sub_82466E20(ctx, base);
	// 826CFCBC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826CFCC0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826CFCC4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826CFCC8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826CFCD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826CFCD0 size=108
    let mut pc: u32 = 0x826CFCD0;
    'dispatch: loop {
        match pc {
            0x826CFCD0 => {
    //   block [0x826CFCD0..0x826CFD3C)
	// 826CFCD0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826CFCD4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826CFCD8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826CFCDC: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826CFCE0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826CFCE4: 38EBCFE8  addi r7, r11, -0x3018
	ctx.r[7].s64 = ctx.r[11].s64 + -12312;
	// 826CFCE8: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 826CFCEC: 388A22A4  addi r4, r10, 0x22a4
	ctx.r[4].s64 = ctx.r[10].s64 + 8868;
	// 826CFCF0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826CFCF4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826CFCF8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826CFCFC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826CFD00: 386A9E3C  addi r3, r10, -0x61c4
	ctx.r[3].s64 = ctx.r[10].s64 + -25028;
	// 826CFD04: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826CFD08: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826CFD0C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826CFD10: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826CFD14: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826CFD18: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826CFD1C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826CFD20: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826CFD24: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826CFD28: 4BD970F9  bl 0x82466e20
	ctx.lr = 0x826CFD2C;
	sub_82466E20(ctx, base);
	// 826CFD2C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826CFD30: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826CFD34: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826CFD38: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826CFD40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826CFD40 size=116
    let mut pc: u32 = 0x826CFD40;
    'dispatch: loop {
        match pc {
            0x826CFD40 => {
    //   block [0x826CFD40..0x826CFDB4)
	// 826CFD40: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826CFD44: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826CFD48: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826CFD4C: 3D40827F  lis r10, -0x7d81
	ctx.r[10].s64 = -2105606144;
	// 826CFD50: 38E00006  li r7, 6
	ctx.r[7].s64 = 6;
	// 826CFD54: 390AD030  addi r8, r10, -0x2fd0
	ctx.r[8].s64 = ctx.r[10].s64 + -12240;
	// 826CFD58: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826CFD5C: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 826CFD60: 38AA9CBC  addi r5, r10, -0x6344
	ctx.r[5].s64 = ctx.r[10].s64 + -25412;
	// 826CFD64: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826CFD68: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826CFD6C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826CFD70: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826CFD74: 388A4590  addi r4, r10, 0x4590
	ctx.r[4].s64 = ctx.r[10].s64 + 17808;
	// 826CFD78: 396B3834  addi r11, r11, 0x3834
	ctx.r[11].s64 = ctx.r[11].s64 + 14388;
	// 826CFD7C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826CFD80: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826CFD84: 386A9E6C  addi r3, r10, -0x6194
	ctx.r[3].s64 = ctx.r[10].s64 + -24980;
	// 826CFD88: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 826CFD8C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826CFD90: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 826CFD94: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826CFD98: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826CFD9C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826CFDA0: 4BD97081  bl 0x82466e20
	ctx.lr = 0x826CFDA4;
	sub_82466E20(ctx, base);
	// 826CFDA4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826CFDA8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826CFDAC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826CFDB0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826CFDB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826CFDB8 size=112
    let mut pc: u32 = 0x826CFDB8;
    'dispatch: loop {
        match pc {
            0x826CFDB8 => {
    //   block [0x826CFDB8..0x826CFE28)
	// 826CFDB8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826CFDBC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826CFDC0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826CFDC4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826CFDC8: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826CFDCC: 38AA9ECC  addi r5, r10, -0x6134
	ctx.r[5].s64 = ctx.r[10].s64 + -24884;
	// 826CFDD0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826CFDD4: 390BD0C0  addi r8, r11, -0x2f40
	ctx.r[8].s64 = ctx.r[11].s64 + -12096;
	// 826CFDD8: 39200005  li r9, 5
	ctx.r[9].s64 = 5;
	// 826CFDDC: 388A45B0  addi r4, r10, 0x45b0
	ctx.r[4].s64 = ctx.r[10].s64 + 17840;
	// 826CFDE0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826CFDE4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826CFDE8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826CFDEC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826CFDF0: 386A9E9C  addi r3, r10, -0x6164
	ctx.r[3].s64 = ctx.r[10].s64 + -24932;
	// 826CFDF4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826CFDF8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826CFDFC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826CFE00: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826CFE04: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826CFE08: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826CFE0C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826CFE10: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826CFE14: 4BD9700D  bl 0x82466e20
	ctx.lr = 0x826CFE18;
	sub_82466E20(ctx, base);
	// 826CFE18: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826CFE1C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826CFE20: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826CFE24: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826CFE28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826CFE28 size=100
    let mut pc: u32 = 0x826CFE28;
    'dispatch: loop {
        match pc {
            0x826CFE28 => {
    //   block [0x826CFE28..0x826CFE8C)
	// 826CFE28: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826CFE2C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826CFE30: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826CFE34: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826CFE38: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826CFE3C: 38AA86FC  addi r5, r10, -0x7904
	ctx.r[5].s64 = ctx.r[10].s64 + -30980;
	// 826CFE40: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826CFE44: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826CFE48: 388A45C8  addi r4, r10, 0x45c8
	ctx.r[4].s64 = ctx.r[10].s64 + 17864;
	// 826CFE4C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826CFE50: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826CFE54: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826CFE58: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826CFE5C: 386A9ECC  addi r3, r10, -0x6134
	ctx.r[3].s64 = ctx.r[10].s64 + -24884;
	// 826CFE60: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826CFE64: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826CFE68: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826CFE6C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826CFE70: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826CFE74: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826CFE78: 4BD96FA9  bl 0x82466e20
	ctx.lr = 0x826CFE7C;
	sub_82466E20(ctx, base);
	// 826CFE7C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826CFE80: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826CFE84: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826CFE88: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826CFE90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826CFE90 size=24
    let mut pc: u32 = 0x826CFE90;
    'dispatch: loop {
        match pc {
            0x826CFE90 => {
    //   block [0x826CFE90..0x826CFEA8)
	// 826CFE90: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826CFE94: 3D40827F  lis r10, -0x7d81
	ctx.r[10].s64 = -2105606144;
	// 826CFE98: 394AF828  addi r10, r10, -0x7d8
	ctx.r[10].s64 = ctx.r[10].s64 + -2008;
	// 826CFE9C: 816BCEDC  lwz r11, -0x3124(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-12580 as u32) ) } as u64;
	// 826CFEA0: 916A0038  stw r11, 0x38(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(56 as u32), ctx.r[11].u32 ) };
	// 826CFEA4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826CFEA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826CFEA8 size=116
    let mut pc: u32 = 0x826CFEA8;
    'dispatch: loop {
        match pc {
            0x826CFEA8 => {
    //   block [0x826CFEA8..0x826CFF1C)
	// 826CFEA8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826CFEAC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826CFEB0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826CFEB4: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826CFEB8: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 826CFEBC: 390BF828  addi r8, r11, -0x7d8
	ctx.r[8].s64 = ctx.r[11].s64 + -2008;
	// 826CFEC0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826CFEC4: 392A3878  addi r9, r10, 0x3878
	ctx.r[9].s64 = ctx.r[10].s64 + 14456;
	// 826CFEC8: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826CFECC: 38E00004  li r7, 4
	ctx.r[7].s64 = 4;
	// 826CFED0: 38AA9CBC  addi r5, r10, -0x6344
	ctx.r[5].s64 = ctx.r[10].s64 + -25412;
	// 826CFED4: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826CFED8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826CFEDC: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826CFEE0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826CFEE4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826CFEE8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826CFEEC: 3D608299  lis r11, -0x7d67
	ctx.r[11].s64 = -2103902208;
	// 826CFEF0: 388A45EC  addi r4, r10, 0x45ec
	ctx.r[4].s64 = ctx.r[10].s64 + 17900;
	// 826CFEF4: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826CFEF8: 386B9EFC  addi r3, r11, -0x6104
	ctx.r[3].s64 = ctx.r[11].s64 + -24836;
	// 826CFEFC: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826CFF00: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826CFF04: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826CFF08: 4BD96F19  bl 0x82466e20
	ctx.lr = 0x826CFF0C;
	sub_82466E20(ctx, base);
	// 826CFF0C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826CFF10: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826CFF14: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826CFF18: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826CFF20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826CFF20 size=112
    let mut pc: u32 = 0x826CFF20;
    'dispatch: loop {
        match pc {
            0x826CFF20 => {
    //   block [0x826CFF20..0x826CFF90)
	// 826CFF20: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826CFF24: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826CFF28: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826CFF2C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826CFF30: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826CFF34: 38AA9CBC  addi r5, r10, -0x6344
	ctx.r[5].s64 = ctx.r[10].s64 + -25412;
	// 826CFF38: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826CFF3C: 390BD138  addi r8, r11, -0x2ec8
	ctx.r[8].s64 = ctx.r[11].s64 + -11976;
	// 826CFF40: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 826CFF44: 388A462C  addi r4, r10, 0x462c
	ctx.r[4].s64 = ctx.r[10].s64 + 17964;
	// 826CFF48: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826CFF4C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826CFF50: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826CFF54: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826CFF58: 386A9F2C  addi r3, r10, -0x60d4
	ctx.r[3].s64 = ctx.r[10].s64 + -24788;
	// 826CFF5C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826CFF60: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826CFF64: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826CFF68: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826CFF6C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826CFF70: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826CFF74: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826CFF78: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826CFF7C: 4BD96EA5  bl 0x82466e20
	ctx.lr = 0x826CFF80;
	sub_82466E20(ctx, base);
	// 826CFF80: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826CFF84: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826CFF88: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826CFF8C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826CFF90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826CFF90 size=112
    let mut pc: u32 = 0x826CFF90;
    'dispatch: loop {
        match pc {
            0x826CFF90 => {
    //   block [0x826CFF90..0x826D0000)
	// 826CFF90: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826CFF94: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826CFF98: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826CFF9C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826CFFA0: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826CFFA4: 38AA9E6C  addi r5, r10, -0x6194
	ctx.r[5].s64 = ctx.r[10].s64 + -24980;
	// 826CFFA8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826CFFAC: 390BD180  addi r8, r11, -0x2e80
	ctx.r[8].s64 = ctx.r[11].s64 + -11904;
	// 826CFFB0: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 826CFFB4: 388A22D4  addi r4, r10, 0x22d4
	ctx.r[4].s64 = ctx.r[10].s64 + 8916;
	// 826CFFB8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826CFFBC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826CFFC0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826CFFC4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826CFFC8: 386A9F5C  addi r3, r10, -0x60a4
	ctx.r[3].s64 = ctx.r[10].s64 + -24740;
	// 826CFFCC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826CFFD0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826CFFD4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826CFFD8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826CFFDC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826CFFE0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826CFFE4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826CFFE8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826CFFEC: 4BD96E35  bl 0x82466e20
	ctx.lr = 0x826CFFF0;
	sub_82466E20(ctx, base);
	// 826CFFF0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826CFFF4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826CFFF8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826CFFFC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D0000(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D0000 size=108
    let mut pc: u32 = 0x826D0000;
    'dispatch: loop {
        match pc {
            0x826D0000 => {
    //   block [0x826D0000..0x826D006C)
	// 826D0000: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D0004: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D0008: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D000C: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D0010: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826D0014: 38EBD1E0  addi r7, r11, -0x2e20
	ctx.r[7].s64 = ctx.r[11].s64 + -11808;
	// 826D0018: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 826D001C: 388A22F4  addi r4, r10, 0x22f4
	ctx.r[4].s64 = ctx.r[10].s64 + 8948;
	// 826D0020: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D0024: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D0028: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826D002C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D0030: 386A9F8C  addi r3, r10, -0x6074
	ctx.r[3].s64 = ctx.r[10].s64 + -24692;
	// 826D0034: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826D0038: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D003C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D0040: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D0044: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D0048: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D004C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D0050: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D0054: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826D0058: 4BD96DC9  bl 0x82466e20
	ctx.lr = 0x826D005C;
	sub_82466E20(ctx, base);
	// 826D005C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D0060: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D0064: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D0068: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D0070(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D0070 size=108
    let mut pc: u32 = 0x826D0070;
    'dispatch: loop {
        match pc {
            0x826D0070 => {
    //   block [0x826D0070..0x826D00DC)
	// 826D0070: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D0074: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D0078: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D007C: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D0080: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826D0084: 38EBD228  addi r7, r11, -0x2dd8
	ctx.r[7].s64 = ctx.r[11].s64 + -11736;
	// 826D0088: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 826D008C: 388A2320  addi r4, r10, 0x2320
	ctx.r[4].s64 = ctx.r[10].s64 + 8992;
	// 826D0090: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D0094: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D0098: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826D009C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D00A0: 386A9FBC  addi r3, r10, -0x6044
	ctx.r[3].s64 = ctx.r[10].s64 + -24644;
	// 826D00A4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826D00A8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D00AC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D00B0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D00B4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D00B8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D00BC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D00C0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D00C4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826D00C8: 4BD96D59  bl 0x82466e20
	ctx.lr = 0x826D00CC;
	sub_82466E20(ctx, base);
	// 826D00CC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D00D0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D00D4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D00D8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D00E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D00E0 size=112
    let mut pc: u32 = 0x826D00E0;
    'dispatch: loop {
        match pc {
            0x826D00E0 => {
    //   block [0x826D00E0..0x826D0150)
	// 826D00E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D00E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D00E8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D00EC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D00F0: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D00F4: 38AA9CBC  addi r5, r10, -0x6344
	ctx.r[5].s64 = ctx.r[10].s64 + -25412;
	// 826D00F8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826D00FC: 390BD270  addi r8, r11, -0x2d90
	ctx.r[8].s64 = ctx.r[11].s64 + -11664;
	// 826D0100: 39200007  li r9, 7
	ctx.r[9].s64 = 7;
	// 826D0104: 388A466C  addi r4, r10, 0x466c
	ctx.r[4].s64 = ctx.r[10].s64 + 18028;
	// 826D0108: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D010C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D0110: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826D0114: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D0118: 386A9FEC  addi r3, r10, -0x6014
	ctx.r[3].s64 = ctx.r[10].s64 + -24596;
	// 826D011C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826D0120: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D0124: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D0128: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D012C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D0130: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D0134: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D0138: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D013C: 4BD96CE5  bl 0x82466e20
	ctx.lr = 0x826D0140;
	sub_82466E20(ctx, base);
	// 826D0140: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D0144: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D0148: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D014C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D0150(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D0150 size=112
    let mut pc: u32 = 0x826D0150;
    'dispatch: loop {
        match pc {
            0x826D0150 => {
    //   block [0x826D0150..0x826D01C0)
	// 826D0150: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D0154: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D0158: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D015C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D0160: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D0164: 38AA9CBC  addi r5, r10, -0x6344
	ctx.r[5].s64 = ctx.r[10].s64 + -25412;
	// 826D0168: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826D016C: 390BD318  addi r8, r11, -0x2ce8
	ctx.r[8].s64 = ctx.r[11].s64 + -11496;
	// 826D0170: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 826D0174: 388A46AC  addi r4, r10, 0x46ac
	ctx.r[4].s64 = ctx.r[10].s64 + 18092;
	// 826D0178: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D017C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D0180: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826D0184: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D0188: 386AA01C  addi r3, r10, -0x5fe4
	ctx.r[3].s64 = ctx.r[10].s64 + -24548;
	// 826D018C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826D0190: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D0194: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D0198: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D019C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D01A0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D01A4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D01A8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D01AC: 4BD96C75  bl 0x82466e20
	ctx.lr = 0x826D01B0;
	sub_82466E20(ctx, base);
	// 826D01B0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D01B4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D01B8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D01BC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D01C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D01C0 size=108
    let mut pc: u32 = 0x826D01C0;
    'dispatch: loop {
        match pc {
            0x826D01C0 => {
    //   block [0x826D01C0..0x826D022C)
	// 826D01C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D01C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D01C8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D01CC: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D01D0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826D01D4: 38EBD360  addi r7, r11, -0x2ca0
	ctx.r[7].s64 = ctx.r[11].s64 + -11424;
	// 826D01D8: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826D01DC: 388A234C  addi r4, r10, 0x234c
	ctx.r[4].s64 = ctx.r[10].s64 + 9036;
	// 826D01E0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D01E4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D01E8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826D01EC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D01F0: 386AA04C  addi r3, r10, -0x5fb4
	ctx.r[3].s64 = ctx.r[10].s64 + -24500;
	// 826D01F4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826D01F8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D01FC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D0200: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D0204: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D0208: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D020C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D0210: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D0214: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826D0218: 4BD96C09  bl 0x82466e20
	ctx.lr = 0x826D021C;
	sub_82466E20(ctx, base);
	// 826D021C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D0220: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D0224: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D0228: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D0230(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D0230 size=108
    let mut pc: u32 = 0x826D0230;
    'dispatch: loop {
        match pc {
            0x826D0230 => {
    //   block [0x826D0230..0x826D029C)
	// 826D0230: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D0234: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D0238: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D023C: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D0240: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826D0244: 38EBD390  addi r7, r11, -0x2c70
	ctx.r[7].s64 = ctx.r[11].s64 + -11376;
	// 826D0248: 39000006  li r8, 6
	ctx.r[8].s64 = 6;
	// 826D024C: 388A2374  addi r4, r10, 0x2374
	ctx.r[4].s64 = ctx.r[10].s64 + 9076;
	// 826D0250: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D0254: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D0258: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826D025C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D0260: 386AA07C  addi r3, r10, -0x5f84
	ctx.r[3].s64 = ctx.r[10].s64 + -24452;
	// 826D0264: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826D0268: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D026C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D0270: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D0274: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D0278: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D027C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D0280: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D0284: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826D0288: 4BD96B99  bl 0x82466e20
	ctx.lr = 0x826D028C;
	sub_82466E20(ctx, base);
	// 826D028C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D0290: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D0294: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D0298: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D02A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D02A0 size=112
    let mut pc: u32 = 0x826D02A0;
    'dispatch: loop {
        match pc {
            0x826D02A0 => {
    //   block [0x826D02A0..0x826D0310)
	// 826D02A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D02A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D02A8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D02AC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D02B0: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D02B4: 38AA9CBC  addi r5, r10, -0x6344
	ctx.r[5].s64 = ctx.r[10].s64 + -25412;
	// 826D02B8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826D02BC: 390BD420  addi r8, r11, -0x2be0
	ctx.r[8].s64 = ctx.r[11].s64 + -11232;
	// 826D02C0: 39200006  li r9, 6
	ctx.r[9].s64 = 6;
	// 826D02C4: 388A46E4  addi r4, r10, 0x46e4
	ctx.r[4].s64 = ctx.r[10].s64 + 18148;
	// 826D02C8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D02CC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D02D0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826D02D4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D02D8: 386AA0AC  addi r3, r10, -0x5f54
	ctx.r[3].s64 = ctx.r[10].s64 + -24404;
	// 826D02DC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826D02E0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D02E4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D02E8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D02EC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D02F0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D02F4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D02F8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D02FC: 4BD96B25  bl 0x82466e20
	ctx.lr = 0x826D0300;
	sub_82466E20(ctx, base);
	// 826D0300: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D0304: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D0308: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D030C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D0310(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D0310 size=112
    let mut pc: u32 = 0x826D0310;
    'dispatch: loop {
        match pc {
            0x826D0310 => {
    //   block [0x826D0310..0x826D0380)
	// 826D0310: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D0314: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D0318: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D031C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D0320: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D0324: 38AA9CBC  addi r5, r10, -0x6344
	ctx.r[5].s64 = ctx.r[10].s64 + -25412;
	// 826D0328: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826D032C: 390BD4B0  addi r8, r11, -0x2b50
	ctx.r[8].s64 = ctx.r[11].s64 + -11088;
	// 826D0330: 39200008  li r9, 8
	ctx.r[9].s64 = 8;
	// 826D0334: 388A46FC  addi r4, r10, 0x46fc
	ctx.r[4].s64 = ctx.r[10].s64 + 18172;
	// 826D0338: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D033C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D0340: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826D0344: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D0348: 386AA0DC  addi r3, r10, -0x5f24
	ctx.r[3].s64 = ctx.r[10].s64 + -24356;
	// 826D034C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826D0350: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D0354: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D0358: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D035C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D0360: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D0364: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D0368: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D036C: 4BD96AB5  bl 0x82466e20
	ctx.lr = 0x826D0370;
	sub_82466E20(ctx, base);
	// 826D0370: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D0374: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D0378: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D037C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D0380(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D0380 size=112
    let mut pc: u32 = 0x826D0380;
    'dispatch: loop {
        match pc {
            0x826D0380 => {
    //   block [0x826D0380..0x826D03F0)
	// 826D0380: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D0384: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D0388: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D038C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D0390: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D0394: 38AA9CBC  addi r5, r10, -0x6344
	ctx.r[5].s64 = ctx.r[10].s64 + -25412;
	// 826D0398: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826D039C: 390BD570  addi r8, r11, -0x2a90
	ctx.r[8].s64 = ctx.r[11].s64 + -10896;
	// 826D03A0: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826D03A4: 388A484C  addi r4, r10, 0x484c
	ctx.r[4].s64 = ctx.r[10].s64 + 18508;
	// 826D03A8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D03AC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D03B0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826D03B4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D03B8: 386AA10C  addi r3, r10, -0x5ef4
	ctx.r[3].s64 = ctx.r[10].s64 + -24308;
	// 826D03BC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826D03C0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D03C4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D03C8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D03CC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D03D0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D03D4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D03D8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D03DC: 4BD96A45  bl 0x82466e20
	ctx.lr = 0x826D03E0;
	sub_82466E20(ctx, base);
	// 826D03E0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D03E4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D03E8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D03EC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D03F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D03F0 size=108
    let mut pc: u32 = 0x826D03F0;
    'dispatch: loop {
        match pc {
            0x826D03F0 => {
    //   block [0x826D03F0..0x826D045C)
	// 826D03F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D03F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D03F8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D03FC: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D0400: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826D0404: 38EBD588  addi r7, r11, -0x2a78
	ctx.r[7].s64 = ctx.r[11].s64 + -10872;
	// 826D0408: 39000005  li r8, 5
	ctx.r[8].s64 = 5;
	// 826D040C: 388A4890  addi r4, r10, 0x4890
	ctx.r[4].s64 = ctx.r[10].s64 + 18576;
	// 826D0410: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D0414: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D0418: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826D041C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D0420: 386AA13C  addi r3, r10, -0x5ec4
	ctx.r[3].s64 = ctx.r[10].s64 + -24260;
	// 826D0424: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826D0428: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D042C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D0430: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D0434: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D0438: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D043C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D0440: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D0444: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826D0448: 4BD969D9  bl 0x82466e20
	ctx.lr = 0x826D044C;
	sub_82466E20(ctx, base);
	// 826D044C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D0450: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D0454: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D0458: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D0460(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D0460 size=112
    let mut pc: u32 = 0x826D0460;
    'dispatch: loop {
        match pc {
            0x826D0460 => {
    //   block [0x826D0460..0x826D04D0)
	// 826D0460: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D0464: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D0468: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D046C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D0470: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D0474: 38AA9CBC  addi r5, r10, -0x6344
	ctx.r[5].s64 = ctx.r[10].s64 + -25412;
	// 826D0478: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826D047C: 390BD600  addi r8, r11, -0x2a00
	ctx.r[8].s64 = ctx.r[11].s64 + -10752;
	// 826D0480: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 826D0484: 388A48B0  addi r4, r10, 0x48b0
	ctx.r[4].s64 = ctx.r[10].s64 + 18608;
	// 826D0488: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D048C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D0490: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826D0494: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D0498: 386AA16C  addi r3, r10, -0x5e94
	ctx.r[3].s64 = ctx.r[10].s64 + -24212;
	// 826D049C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826D04A0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D04A4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D04A8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D04AC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D04B0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D04B4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D04B8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D04BC: 4BD96965  bl 0x82466e20
	ctx.lr = 0x826D04C0;
	sub_82466E20(ctx, base);
	// 826D04C0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D04C4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D04C8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D04CC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D04D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D04D0 size=100
    let mut pc: u32 = 0x826D04D0;
    'dispatch: loop {
        match pc {
            0x826D04D0 => {
    //   block [0x826D04D0..0x826D0534)
	// 826D04D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D04D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D04D8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D04DC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D04E0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D04E4: 38AA86FC  addi r5, r10, -0x7904
	ctx.r[5].s64 = ctx.r[10].s64 + -30980;
	// 826D04E8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826D04EC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D04F0: 388A48CC  addi r4, r10, 0x48cc
	ctx.r[4].s64 = ctx.r[10].s64 + 18636;
	// 826D04F4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D04F8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D04FC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D0500: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D0504: 386AA19C  addi r3, r10, -0x5e64
	ctx.r[3].s64 = ctx.r[10].s64 + -24164;
	// 826D0508: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D050C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D0510: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826D0514: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D0518: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826D051C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D0520: 4BD96901  bl 0x82466e20
	ctx.lr = 0x826D0524;
	sub_82466E20(ctx, base);
	// 826D0524: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D0528: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D052C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D0530: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D0538(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D0538 size=112
    let mut pc: u32 = 0x826D0538;
    'dispatch: loop {
        match pc {
            0x826D0538 => {
    //   block [0x826D0538..0x826D05A8)
	// 826D0538: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D053C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D0540: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D0544: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D0548: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D054C: 38AAA19C  addi r5, r10, -0x5e64
	ctx.r[5].s64 = ctx.r[10].s64 + -24164;
	// 826D0550: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826D0554: 390BD648  addi r8, r11, -0x29b8
	ctx.r[8].s64 = ctx.r[11].s64 + -10680;
	// 826D0558: 39200005  li r9, 5
	ctx.r[9].s64 = 5;
	// 826D055C: 388A4928  addi r4, r10, 0x4928
	ctx.r[4].s64 = ctx.r[10].s64 + 18728;
	// 826D0560: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D0564: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D0568: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826D056C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D0570: 386AA1CC  addi r3, r10, -0x5e34
	ctx.r[3].s64 = ctx.r[10].s64 + -24116;
	// 826D0574: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826D0578: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D057C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D0580: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D0584: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D0588: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D058C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D0590: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D0594: 4BD9688D  bl 0x82466e20
	ctx.lr = 0x826D0598;
	sub_82466E20(ctx, base);
	// 826D0598: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D059C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D05A0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D05A4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D05A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D05A8 size=112
    let mut pc: u32 = 0x826D05A8;
    'dispatch: loop {
        match pc {
            0x826D05A8 => {
    //   block [0x826D05A8..0x826D0618)
	// 826D05A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D05AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D05B0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D05B4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D05B8: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D05BC: 38AAA19C  addi r5, r10, -0x5e64
	ctx.r[5].s64 = ctx.r[10].s64 + -24164;
	// 826D05C0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826D05C4: 390BD6C0  addi r8, r11, -0x2940
	ctx.r[8].s64 = ctx.r[11].s64 + -10560;
	// 826D05C8: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 826D05CC: 388A4944  addi r4, r10, 0x4944
	ctx.r[4].s64 = ctx.r[10].s64 + 18756;
	// 826D05D0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D05D4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D05D8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826D05DC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D05E0: 386AA1FC  addi r3, r10, -0x5e04
	ctx.r[3].s64 = ctx.r[10].s64 + -24068;
	// 826D05E4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826D05E8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D05EC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D05F0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D05F4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D05F8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D05FC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D0600: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D0604: 4BD9681D  bl 0x82466e20
	ctx.lr = 0x826D0608;
	sub_82466E20(ctx, base);
	// 826D0608: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D060C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D0610: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D0614: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D0618(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D0618 size=112
    let mut pc: u32 = 0x826D0618;
    'dispatch: loop {
        match pc {
            0x826D0618 => {
    //   block [0x826D0618..0x826D0688)
	// 826D0618: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D061C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D0620: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D0624: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D0628: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D062C: 38AAA19C  addi r5, r10, -0x5e64
	ctx.r[5].s64 = ctx.r[10].s64 + -24164;
	// 826D0630: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826D0634: 390BD720  addi r8, r11, -0x28e0
	ctx.r[8].s64 = ctx.r[11].s64 + -10464;
	// 826D0638: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 826D063C: 388A4964  addi r4, r10, 0x4964
	ctx.r[4].s64 = ctx.r[10].s64 + 18788;
	// 826D0640: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D0644: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D0648: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826D064C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D0650: 386AA22C  addi r3, r10, -0x5dd4
	ctx.r[3].s64 = ctx.r[10].s64 + -24020;
	// 826D0654: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826D0658: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D065C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D0660: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D0664: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D0668: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D066C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D0670: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D0674: 4BD967AD  bl 0x82466e20
	ctx.lr = 0x826D0678;
	sub_82466E20(ctx, base);
	// 826D0678: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D067C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D0680: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D0684: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D0688(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D0688 size=116
    let mut pc: u32 = 0x826D0688;
    'dispatch: loop {
        match pc {
            0x826D0688 => {
    //   block [0x826D0688..0x826D06FC)
	// 826D0688: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D068C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D0690: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D0694: 3D40827F  lis r10, -0x7d81
	ctx.r[10].s64 = -2105606144;
	// 826D0698: 38E00012  li r7, 0x12
	ctx.r[7].s64 = 18;
	// 826D069C: 390AD780  addi r8, r10, -0x2880
	ctx.r[8].s64 = ctx.r[10].s64 + -10368;
	// 826D06A0: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D06A4: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 826D06A8: 38AAA6DC  addi r5, r10, -0x5924
	ctx.r[5].s64 = ctx.r[10].s64 + -22820;
	// 826D06AC: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826D06B0: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826D06B4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D06B8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826D06BC: 388A49FC  addi r4, r10, 0x49fc
	ctx.r[4].s64 = ctx.r[10].s64 + 18940;
	// 826D06C0: 396B38A0  addi r11, r11, 0x38a0
	ctx.r[11].s64 = ctx.r[11].s64 + 14496;
	// 826D06C4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D06C8: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D06CC: 386AA25C  addi r3, r10, -0x5da4
	ctx.r[3].s64 = ctx.r[10].s64 + -23972;
	// 826D06D0: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 826D06D4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D06D8: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 826D06DC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D06E0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D06E4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D06E8: 4BD96739  bl 0x82466e20
	ctx.lr = 0x826D06EC;
	sub_82466E20(ctx, base);
	// 826D06EC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D06F0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D06F4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D06F8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D0700(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D0700 size=100
    let mut pc: u32 = 0x826D0700;
    'dispatch: loop {
        match pc {
            0x826D0700 => {
    //   block [0x826D0700..0x826D0764)
	// 826D0700: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D0704: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D0708: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D070C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D0710: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D0714: 38AA86FC  addi r5, r10, -0x7904
	ctx.r[5].s64 = ctx.r[10].s64 + -30980;
	// 826D0718: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826D071C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D0720: 388A4A08  addi r4, r10, 0x4a08
	ctx.r[4].s64 = ctx.r[10].s64 + 18952;
	// 826D0724: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D0728: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D072C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D0730: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D0734: 386AA28C  addi r3, r10, -0x5d74
	ctx.r[3].s64 = ctx.r[10].s64 + -23924;
	// 826D0738: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D073C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D0740: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826D0744: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D0748: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826D074C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D0750: 4BD966D1  bl 0x82466e20
	ctx.lr = 0x826D0754;
	sub_82466E20(ctx, base);
	// 826D0754: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D0758: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D075C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D0760: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D0768(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D0768 size=100
    let mut pc: u32 = 0x826D0768;
    'dispatch: loop {
        match pc {
            0x826D0768 => {
    //   block [0x826D0768..0x826D07CC)
	// 826D0768: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D076C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D0770: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D0774: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D0778: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D077C: 38AAA31C  addi r5, r10, -0x5ce4
	ctx.r[5].s64 = ctx.r[10].s64 + -23780;
	// 826D0780: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826D0784: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D0788: 388A4A1C  addi r4, r10, 0x4a1c
	ctx.r[4].s64 = ctx.r[10].s64 + 18972;
	// 826D078C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D0790: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D0794: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D0798: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D079C: 386AA2BC  addi r3, r10, -0x5d44
	ctx.r[3].s64 = ctx.r[10].s64 + -23876;
	// 826D07A0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D07A4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D07A8: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826D07AC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D07B0: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826D07B4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D07B8: 4BD96669  bl 0x82466e20
	ctx.lr = 0x826D07BC;
	sub_82466E20(ctx, base);
	// 826D07BC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D07C0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D07C4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D07C8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D07D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D07D0 size=100
    let mut pc: u32 = 0x826D07D0;
    'dispatch: loop {
        match pc {
            0x826D07D0 => {
    //   block [0x826D07D0..0x826D0834)
	// 826D07D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D07D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D07D8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D07DC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D07E0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D07E4: 38AAA25C  addi r5, r10, -0x5da4
	ctx.r[5].s64 = ctx.r[10].s64 + -23972;
	// 826D07E8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826D07EC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D07F0: 388A4A38  addi r4, r10, 0x4a38
	ctx.r[4].s64 = ctx.r[10].s64 + 19000;
	// 826D07F4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D07F8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D07FC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D0800: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D0804: 386AA2EC  addi r3, r10, -0x5d14
	ctx.r[3].s64 = ctx.r[10].s64 + -23828;
	// 826D0808: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D080C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D0810: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826D0814: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D0818: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826D081C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D0820: 4BD96601  bl 0x82466e20
	ctx.lr = 0x826D0824;
	sub_82466E20(ctx, base);
	// 826D0824: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D0828: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D082C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D0830: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D0838(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D0838 size=104
    let mut pc: u32 = 0x826D0838;
    'dispatch: loop {
        match pc {
            0x826D0838 => {
    //   block [0x826D0838..0x826D08A0)
	// 826D0838: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D083C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D0840: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D0844: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 826D0848: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D084C: 392A388C  addi r9, r10, 0x388c
	ctx.r[9].s64 = ctx.r[10].s64 + 14476;
	// 826D0850: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D0854: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D0858: 38AAA28C  addi r5, r10, -0x5d74
	ctx.r[5].s64 = ctx.r[10].s64 + -23924;
	// 826D085C: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826D0860: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D0864: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D0868: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D086C: 388A4A44  addi r4, r10, 0x4a44
	ctx.r[4].s64 = ctx.r[10].s64 + 19012;
	// 826D0870: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D0874: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D0878: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826D087C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D0880: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826D0884: 386AA31C  addi r3, r10, -0x5ce4
	ctx.r[3].s64 = ctx.r[10].s64 + -23780;
	// 826D0888: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826D088C: 4BD96595  bl 0x82466e20
	ctx.lr = 0x826D0890;
	sub_82466E20(ctx, base);
	// 826D0890: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D0894: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D0898: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D089C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D08A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D08A0 size=108
    let mut pc: u32 = 0x826D08A0;
    'dispatch: loop {
        match pc {
            0x826D08A0 => {
    //   block [0x826D08A0..0x826D090C)
	// 826D08A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D08A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D08A8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D08AC: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D08B0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826D08B4: 38EBD934  addi r7, r11, -0x26cc
	ctx.r[7].s64 = ctx.r[11].s64 + -9932;
	// 826D08B8: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826D08BC: 388A4A5C  addi r4, r10, 0x4a5c
	ctx.r[4].s64 = ctx.r[10].s64 + 19036;
	// 826D08C0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D08C4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D08C8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826D08CC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D08D0: 386AA34C  addi r3, r10, -0x5cb4
	ctx.r[3].s64 = ctx.r[10].s64 + -23732;
	// 826D08D4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826D08D8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D08DC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D08E0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D08E4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D08E8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D08EC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D08F0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D08F4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826D08F8: 4BD96529  bl 0x82466e20
	ctx.lr = 0x826D08FC;
	sub_82466E20(ctx, base);
	// 826D08FC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D0900: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D0904: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D0908: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D0910(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D0910 size=112
    let mut pc: u32 = 0x826D0910;
    'dispatch: loop {
        match pc {
            0x826D0910 => {
    //   block [0x826D0910..0x826D0980)
	// 826D0910: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D0914: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D0918: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D091C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D0920: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D0924: 38AAA31C  addi r5, r10, -0x5ce4
	ctx.r[5].s64 = ctx.r[10].s64 + -23780;
	// 826D0928: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826D092C: 390BD968  addi r8, r11, -0x2698
	ctx.r[8].s64 = ctx.r[11].s64 + -9880;
	// 826D0930: 39200007  li r9, 7
	ctx.r[9].s64 = 7;
	// 826D0934: 388A4A80  addi r4, r10, 0x4a80
	ctx.r[4].s64 = ctx.r[10].s64 + 19072;
	// 826D0938: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D093C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D0940: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826D0944: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D0948: 386AA37C  addi r3, r10, -0x5c84
	ctx.r[3].s64 = ctx.r[10].s64 + -23684;
	// 826D094C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826D0950: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D0954: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D0958: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D095C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D0960: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D0964: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D0968: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D096C: 4BD964B5  bl 0x82466e20
	ctx.lr = 0x826D0970;
	sub_82466E20(ctx, base);
	// 826D0970: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D0974: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D0978: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D097C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D0980(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D0980 size=116
    let mut pc: u32 = 0x826D0980;
    'dispatch: loop {
        match pc {
            0x826D0980 => {
    //   block [0x826D0980..0x826D09F4)
	// 826D0980: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D0984: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D0988: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D098C: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D0990: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 826D0994: 390BDA10  addi r8, r11, -0x25f0
	ctx.r[8].s64 = ctx.r[11].s64 + -9712;
	// 826D0998: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D099C: 392A390C  addi r9, r10, 0x390c
	ctx.r[9].s64 = ctx.r[10].s64 + 14604;
	// 826D09A0: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D09A4: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
	// 826D09A8: 38AA86FC  addi r5, r10, -0x7904
	ctx.r[5].s64 = ctx.r[10].s64 + -30980;
	// 826D09AC: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826D09B0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D09B4: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826D09B8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D09BC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D09C0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D09C4: 3D608299  lis r11, -0x7d67
	ctx.r[11].s64 = -2103902208;
	// 826D09C8: 388A4B0C  addi r4, r10, 0x4b0c
	ctx.r[4].s64 = ctx.r[10].s64 + 19212;
	// 826D09CC: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826D09D0: 386BA3AC  addi r3, r11, -0x5c54
	ctx.r[3].s64 = ctx.r[11].s64 + -23636;
	// 826D09D4: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826D09D8: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D09DC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D09E0: 4BD96441  bl 0x82466e20
	ctx.lr = 0x826D09E4;
	sub_82466E20(ctx, base);
	// 826D09E4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D09E8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D09EC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D09F0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D09F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D09F8 size=112
    let mut pc: u32 = 0x826D09F8;
    'dispatch: loop {
        match pc {
            0x826D09F8 => {
    //   block [0x826D09F8..0x826D0A68)
	// 826D09F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D09FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D0A00: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D0A04: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D0A08: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D0A0C: 38AAA46C  addi r5, r10, -0x5b94
	ctx.r[5].s64 = ctx.r[10].s64 + -23444;
	// 826D0A10: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826D0A14: 390BDA28  addi r8, r11, -0x25d8
	ctx.r[8].s64 = ctx.r[11].s64 + -9688;
	// 826D0A18: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826D0A1C: 388A4B18  addi r4, r10, 0x4b18
	ctx.r[4].s64 = ctx.r[10].s64 + 19224;
	// 826D0A20: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D0A24: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D0A28: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826D0A2C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D0A30: 386AA3DC  addi r3, r10, -0x5c24
	ctx.r[3].s64 = ctx.r[10].s64 + -23588;
	// 826D0A34: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826D0A38: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D0A3C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D0A40: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D0A44: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D0A48: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D0A4C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D0A50: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D0A54: 4BD963CD  bl 0x82466e20
	ctx.lr = 0x826D0A58;
	sub_82466E20(ctx, base);
	// 826D0A58: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D0A5C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D0A60: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D0A64: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D0A68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D0A68 size=100
    let mut pc: u32 = 0x826D0A68;
    'dispatch: loop {
        match pc {
            0x826D0A68 => {
    //   block [0x826D0A68..0x826D0ACC)
	// 826D0A68: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D0A6C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D0A70: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D0A74: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D0A78: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D0A7C: 38AAA43C  addi r5, r10, -0x5bc4
	ctx.r[5].s64 = ctx.r[10].s64 + -23492;
	// 826D0A80: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826D0A84: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D0A88: 388A4B24  addi r4, r10, 0x4b24
	ctx.r[4].s64 = ctx.r[10].s64 + 19236;
	// 826D0A8C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D0A90: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D0A94: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D0A98: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D0A9C: 386AA40C  addi r3, r10, -0x5bf4
	ctx.r[3].s64 = ctx.r[10].s64 + -23540;
	// 826D0AA0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D0AA4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D0AA8: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826D0AAC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D0AB0: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826D0AB4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D0AB8: 4BD96369  bl 0x82466e20
	ctx.lr = 0x826D0ABC;
	sub_82466E20(ctx, base);
	// 826D0ABC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D0AC0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D0AC4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D0AC8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D0AD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D0AD0 size=112
    let mut pc: u32 = 0x826D0AD0;
    'dispatch: loop {
        match pc {
            0x826D0AD0 => {
    //   block [0x826D0AD0..0x826D0B40)
	// 826D0AD0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D0AD4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D0AD8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D0ADC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D0AE0: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D0AE4: 38AAA46C  addi r5, r10, -0x5b94
	ctx.r[5].s64 = ctx.r[10].s64 + -23444;
	// 826D0AE8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826D0AEC: 390BDA40  addi r8, r11, -0x25c0
	ctx.r[8].s64 = ctx.r[11].s64 + -9664;
	// 826D0AF0: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826D0AF4: 388A4B38  addi r4, r10, 0x4b38
	ctx.r[4].s64 = ctx.r[10].s64 + 19256;
	// 826D0AF8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D0AFC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D0B00: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826D0B04: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D0B08: 386AA43C  addi r3, r10, -0x5bc4
	ctx.r[3].s64 = ctx.r[10].s64 + -23492;
	// 826D0B0C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826D0B10: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D0B14: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D0B18: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D0B1C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D0B20: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D0B24: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D0B28: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D0B2C: 4BD962F5  bl 0x82466e20
	ctx.lr = 0x826D0B30;
	sub_82466E20(ctx, base);
	// 826D0B30: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D0B34: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D0B38: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D0B3C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D0B40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D0B40 size=112
    let mut pc: u32 = 0x826D0B40;
    'dispatch: loop {
        match pc {
            0x826D0B40 => {
    //   block [0x826D0B40..0x826D0BB0)
	// 826D0B40: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D0B44: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D0B48: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D0B4C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D0B50: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D0B54: 38AAA3AC  addi r5, r10, -0x5c54
	ctx.r[5].s64 = ctx.r[10].s64 + -23636;
	// 826D0B58: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826D0B5C: 390BDA70  addi r8, r11, -0x2590
	ctx.r[8].s64 = ctx.r[11].s64 + -9616;
	// 826D0B60: 39200007  li r9, 7
	ctx.r[9].s64 = 7;
	// 826D0B64: 388A2424  addi r4, r10, 0x2424
	ctx.r[4].s64 = ctx.r[10].s64 + 9252;
	// 826D0B68: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D0B6C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D0B70: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826D0B74: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D0B78: 386AA46C  addi r3, r10, -0x5b94
	ctx.r[3].s64 = ctx.r[10].s64 + -23444;
	// 826D0B7C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826D0B80: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D0B84: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D0B88: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D0B8C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D0B90: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D0B94: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D0B98: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D0B9C: 4BD96285  bl 0x82466e20
	ctx.lr = 0x826D0BA0;
	sub_82466E20(ctx, base);
	// 826D0BA0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D0BA4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D0BA8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D0BAC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D0BB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D0BB0 size=100
    let mut pc: u32 = 0x826D0BB0;
    'dispatch: loop {
        match pc {
            0x826D0BB0 => {
    //   block [0x826D0BB0..0x826D0C14)
	// 826D0BB0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D0BB4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D0BB8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D0BBC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D0BC0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D0BC4: 38AAA46C  addi r5, r10, -0x5b94
	ctx.r[5].s64 = ctx.r[10].s64 + -23444;
	// 826D0BC8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826D0BCC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D0BD0: 388A4B60  addi r4, r10, 0x4b60
	ctx.r[4].s64 = ctx.r[10].s64 + 19296;
	// 826D0BD4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D0BD8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D0BDC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D0BE0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D0BE4: 386AA49C  addi r3, r10, -0x5b64
	ctx.r[3].s64 = ctx.r[10].s64 + -23396;
	// 826D0BE8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D0BEC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D0BF0: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826D0BF4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D0BF8: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826D0BFC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D0C00: 4BD96221  bl 0x82466e20
	ctx.lr = 0x826D0C04;
	sub_82466E20(ctx, base);
	// 826D0C04: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D0C08: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D0C0C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D0C10: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D0C18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D0C18 size=100
    let mut pc: u32 = 0x826D0C18;
    'dispatch: loop {
        match pc {
            0x826D0C18 => {
    //   block [0x826D0C18..0x826D0C7C)
	// 826D0C18: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D0C1C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D0C20: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D0C24: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D0C28: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D0C2C: 38AAA3DC  addi r5, r10, -0x5c24
	ctx.r[5].s64 = ctx.r[10].s64 + -23588;
	// 826D0C30: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826D0C34: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D0C38: 388A4B70  addi r4, r10, 0x4b70
	ctx.r[4].s64 = ctx.r[10].s64 + 19312;
	// 826D0C3C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D0C40: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D0C44: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D0C48: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D0C4C: 386AA4CC  addi r3, r10, -0x5b34
	ctx.r[3].s64 = ctx.r[10].s64 + -23348;
	// 826D0C50: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D0C54: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D0C58: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826D0C5C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D0C60: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826D0C64: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D0C68: 4BD961B9  bl 0x82466e20
	ctx.lr = 0x826D0C6C;
	sub_82466E20(ctx, base);
	// 826D0C6C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D0C70: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D0C74: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D0C78: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D0C80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D0C80 size=100
    let mut pc: u32 = 0x826D0C80;
    'dispatch: loop {
        match pc {
            0x826D0C80 => {
    //   block [0x826D0C80..0x826D0CE4)
	// 826D0C80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D0C84: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D0C88: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D0C8C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D0C90: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D0C94: 38AAA49C  addi r5, r10, -0x5b64
	ctx.r[5].s64 = ctx.r[10].s64 + -23396;
	// 826D0C98: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826D0C9C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D0CA0: 388A4B88  addi r4, r10, 0x4b88
	ctx.r[4].s64 = ctx.r[10].s64 + 19336;
	// 826D0CA4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D0CA8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D0CAC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D0CB0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D0CB4: 386AA4FC  addi r3, r10, -0x5b04
	ctx.r[3].s64 = ctx.r[10].s64 + -23300;
	// 826D0CB8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D0CBC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D0CC0: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826D0CC4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D0CC8: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826D0CCC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D0CD0: 4BD96151  bl 0x82466e20
	ctx.lr = 0x826D0CD4;
	sub_82466E20(ctx, base);
	// 826D0CD4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D0CD8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D0CDC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D0CE0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D0CE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D0CE8 size=100
    let mut pc: u32 = 0x826D0CE8;
    'dispatch: loop {
        match pc {
            0x826D0CE8 => {
    //   block [0x826D0CE8..0x826D0D4C)
	// 826D0CE8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D0CEC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D0CF0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D0CF4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D0CF8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D0CFC: 38AAA3DC  addi r5, r10, -0x5c24
	ctx.r[5].s64 = ctx.r[10].s64 + -23588;
	// 826D0D00: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826D0D04: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D0D08: 388A4BA4  addi r4, r10, 0x4ba4
	ctx.r[4].s64 = ctx.r[10].s64 + 19364;
	// 826D0D0C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D0D10: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D0D14: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D0D18: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D0D1C: 386AA52C  addi r3, r10, -0x5ad4
	ctx.r[3].s64 = ctx.r[10].s64 + -23252;
	// 826D0D20: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D0D24: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D0D28: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826D0D2C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D0D30: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826D0D34: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D0D38: 4BD960E9  bl 0x82466e20
	ctx.lr = 0x826D0D3C;
	sub_82466E20(ctx, base);
	// 826D0D3C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D0D40: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D0D44: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D0D48: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D0D50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D0D50 size=112
    let mut pc: u32 = 0x826D0D50;
    'dispatch: loop {
        match pc {
            0x826D0D50 => {
    //   block [0x826D0D50..0x826D0DC0)
	// 826D0D50: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D0D54: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D0D58: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D0D5C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D0D60: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D0D64: 38AAA5BC  addi r5, r10, -0x5a44
	ctx.r[5].s64 = ctx.r[10].s64 + -23108;
	// 826D0D68: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826D0D6C: 390BDB18  addi r8, r11, -0x24e8
	ctx.r[8].s64 = ctx.r[11].s64 + -9448;
	// 826D0D70: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826D0D74: 388A4BB4  addi r4, r10, 0x4bb4
	ctx.r[4].s64 = ctx.r[10].s64 + 19380;
	// 826D0D78: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D0D7C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D0D80: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826D0D84: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D0D88: 386AA55C  addi r3, r10, -0x5aa4
	ctx.r[3].s64 = ctx.r[10].s64 + -23204;
	// 826D0D8C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826D0D90: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D0D94: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D0D98: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D0D9C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D0DA0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D0DA4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D0DA8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D0DAC: 4BD96075  bl 0x82466e20
	ctx.lr = 0x826D0DB0;
	sub_82466E20(ctx, base);
	// 826D0DB0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D0DB4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D0DB8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D0DBC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D0DC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D0DC0 size=112
    let mut pc: u32 = 0x826D0DC0;
    'dispatch: loop {
        match pc {
            0x826D0DC0 => {
    //   block [0x826D0DC0..0x826D0E30)
	// 826D0DC0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D0DC4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D0DC8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D0DCC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D0DD0: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D0DD4: 38AAA5EC  addi r5, r10, -0x5a14
	ctx.r[5].s64 = ctx.r[10].s64 + -23060;
	// 826D0DD8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826D0DDC: 390BDB48  addi r8, r11, -0x24b8
	ctx.r[8].s64 = ctx.r[11].s64 + -9400;
	// 826D0DE0: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826D0DE4: 388A4BC4  addi r4, r10, 0x4bc4
	ctx.r[4].s64 = ctx.r[10].s64 + 19396;
	// 826D0DE8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D0DEC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D0DF0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826D0DF4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D0DF8: 386AA58C  addi r3, r10, -0x5a74
	ctx.r[3].s64 = ctx.r[10].s64 + -23156;
	// 826D0DFC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826D0E00: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D0E04: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D0E08: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D0E0C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D0E10: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D0E14: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D0E18: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D0E1C: 4BD96005  bl 0x82466e20
	ctx.lr = 0x826D0E20;
	sub_82466E20(ctx, base);
	// 826D0E20: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D0E24: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D0E28: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D0E2C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D0E30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D0E30 size=112
    let mut pc: u32 = 0x826D0E30;
    'dispatch: loop {
        match pc {
            0x826D0E30 => {
    //   block [0x826D0E30..0x826D0EA0)
	// 826D0E30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D0E34: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D0E38: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D0E3C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D0E40: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D0E44: 38AAA6DC  addi r5, r10, -0x5924
	ctx.r[5].s64 = ctx.r[10].s64 + -22820;
	// 826D0E48: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826D0E4C: 390BDB60  addi r8, r11, -0x24a0
	ctx.r[8].s64 = ctx.r[11].s64 + -9376;
	// 826D0E50: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826D0E54: 388A4BDC  addi r4, r10, 0x4bdc
	ctx.r[4].s64 = ctx.r[10].s64 + 19420;
	// 826D0E58: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D0E5C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D0E60: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826D0E64: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D0E68: 386AA5BC  addi r3, r10, -0x5a44
	ctx.r[3].s64 = ctx.r[10].s64 + -23108;
	// 826D0E6C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826D0E70: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D0E74: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D0E78: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D0E7C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D0E80: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D0E84: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D0E88: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D0E8C: 4BD95F95  bl 0x82466e20
	ctx.lr = 0x826D0E90;
	sub_82466E20(ctx, base);
	// 826D0E90: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D0E94: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D0E98: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D0E9C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D0EA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D0EA0 size=112
    let mut pc: u32 = 0x826D0EA0;
    'dispatch: loop {
        match pc {
            0x826D0EA0 => {
    //   block [0x826D0EA0..0x826D0F10)
	// 826D0EA0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D0EA4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D0EA8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D0EAC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D0EB0: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D0EB4: 38AAA5BC  addi r5, r10, -0x5a44
	ctx.r[5].s64 = ctx.r[10].s64 + -23108;
	// 826D0EB8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826D0EBC: 390BDB90  addi r8, r11, -0x2470
	ctx.r[8].s64 = ctx.r[11].s64 + -9328;
	// 826D0EC0: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826D0EC4: 388A4BE8  addi r4, r10, 0x4be8
	ctx.r[4].s64 = ctx.r[10].s64 + 19432;
	// 826D0EC8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D0ECC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D0ED0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826D0ED4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D0ED8: 386AA5EC  addi r3, r10, -0x5a14
	ctx.r[3].s64 = ctx.r[10].s64 + -23060;
	// 826D0EDC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826D0EE0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D0EE4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D0EE8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D0EEC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D0EF0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D0EF4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D0EF8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D0EFC: 4BD95F25  bl 0x82466e20
	ctx.lr = 0x826D0F00;
	sub_82466E20(ctx, base);
	// 826D0F00: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D0F04: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D0F08: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D0F0C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D0F10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D0F10 size=112
    let mut pc: u32 = 0x826D0F10;
    'dispatch: loop {
        match pc {
            0x826D0F10 => {
    //   block [0x826D0F10..0x826D0F80)
	// 826D0F10: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D0F14: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D0F18: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D0F1C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D0F20: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D0F24: 38AAA5EC  addi r5, r10, -0x5a14
	ctx.r[5].s64 = ctx.r[10].s64 + -23060;
	// 826D0F28: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826D0F2C: 390BDBA8  addi r8, r11, -0x2458
	ctx.r[8].s64 = ctx.r[11].s64 + -9304;
	// 826D0F30: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826D0F34: 388A4BF8  addi r4, r10, 0x4bf8
	ctx.r[4].s64 = ctx.r[10].s64 + 19448;
	// 826D0F38: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D0F3C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D0F40: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826D0F44: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D0F48: 386AA61C  addi r3, r10, -0x59e4
	ctx.r[3].s64 = ctx.r[10].s64 + -23012;
	// 826D0F4C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826D0F50: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D0F54: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D0F58: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D0F5C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D0F60: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D0F64: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D0F68: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D0F6C: 4BD95EB5  bl 0x82466e20
	ctx.lr = 0x826D0F70;
	sub_82466E20(ctx, base);
	// 826D0F70: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D0F74: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D0F78: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D0F7C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D0F80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D0F80 size=108
    let mut pc: u32 = 0x826D0F80;
    'dispatch: loop {
        match pc {
            0x826D0F80 => {
    //   block [0x826D0F80..0x826D0FEC)
	// 826D0F80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D0F84: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D0F88: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D0F8C: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D0F90: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826D0F94: 38EBDBC4  addi r7, r11, -0x243c
	ctx.r[7].s64 = ctx.r[11].s64 + -9276;
	// 826D0F98: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826D0F9C: 388A37B0  addi r4, r10, 0x37b0
	ctx.r[4].s64 = ctx.r[10].s64 + 14256;
	// 826D0FA0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D0FA4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D0FA8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826D0FAC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D0FB0: 386AA64C  addi r3, r10, -0x59b4
	ctx.r[3].s64 = ctx.r[10].s64 + -22964;
	// 826D0FB4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826D0FB8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D0FBC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D0FC0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D0FC4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D0FC8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D0FCC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D0FD0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D0FD4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826D0FD8: 4BD95E49  bl 0x82466e20
	ctx.lr = 0x826D0FDC;
	sub_82466E20(ctx, base);
	// 826D0FDC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D0FE0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D0FE4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D0FE8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D0FF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D0FF0 size=112
    let mut pc: u32 = 0x826D0FF0;
    'dispatch: loop {
        match pc {
            0x826D0FF0 => {
    //   block [0x826D0FF0..0x826D1060)
	// 826D0FF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D0FF4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D0FF8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D0FFC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D1000: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D1004: 38AA86FC  addi r5, r10, -0x7904
	ctx.r[5].s64 = ctx.r[10].s64 + -30980;
	// 826D1008: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826D100C: 390BDBF8  addi r8, r11, -0x2408
	ctx.r[8].s64 = ctx.r[11].s64 + -9224;
	// 826D1010: 39200006  li r9, 6
	ctx.r[9].s64 = 6;
	// 826D1014: 388A4C10  addi r4, r10, 0x4c10
	ctx.r[4].s64 = ctx.r[10].s64 + 19472;
	// 826D1018: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D101C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D1020: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826D1024: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D1028: 386AA67C  addi r3, r10, -0x5984
	ctx.r[3].s64 = ctx.r[10].s64 + -22916;
	// 826D102C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826D1030: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D1034: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D1038: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D103C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D1040: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D1044: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D1048: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D104C: 4BD95DD5  bl 0x82466e20
	ctx.lr = 0x826D1050;
	sub_82466E20(ctx, base);
	// 826D1050: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D1054: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D1058: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D105C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D1060(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826D1060 size=48
    let mut pc: u32 = 0x826D1060;
    'dispatch: loop {
        match pc {
            0x826D1060 => {
    //   block [0x826D1060..0x826D1090)
	// 826D1060: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D1064: 814BDC90  lwz r10, -0x2370(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-9072 as u32) ) } as u64;
	// 826D1068: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D106C: 396BF888  addi r11, r11, -0x778
	ctx.r[11].s64 = ctx.r[11].s64 + -1912;
	// 826D1070: 914B0050  stw r10, 0x50(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 826D1074: 3D40827F  lis r10, -0x7d81
	ctx.r[10].s64 = -2105606144;
	// 826D1078: 814ADC8C  lwz r10, -0x2374(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-9076 as u32) ) } as u64;
	// 826D107C: 914B0110  stw r10, 0x110(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(272 as u32), ctx.r[10].u32 ) };
	// 826D1080: 3D40827F  lis r10, -0x7d81
	ctx.r[10].s64 = -2105606144;
	// 826D1084: 814ADC88  lwz r10, -0x2378(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-9080 as u32) ) } as u64;
	// 826D1088: 914B0230  stw r10, 0x230(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(560 as u32), ctx.r[10].u32 ) };
	// 826D108C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D1090(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D1090 size=116
    let mut pc: u32 = 0x826D1090;
    'dispatch: loop {
        match pc {
            0x826D1090 => {
    //   block [0x826D1090..0x826D1104)
	// 826D1090: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D1094: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D1098: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D109C: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 826D10A0: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D10A4: 392B3A90  addi r9, r11, 0x3a90
	ctx.r[9].s64 = ctx.r[11].s64 + 14992;
	// 826D10A8: 38AA86FC  addi r5, r10, -0x7904
	ctx.r[5].s64 = ctx.r[10].s64 + -30980;
	// 826D10AC: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826D10B0: 38E90050  addi r7, r9, 0x50
	ctx.r[7].s64 = ctx.r[9].s64 + 80;
	// 826D10B4: 38C0001B  li r6, 0x1b
	ctx.r[6].s64 = 27;
	// 826D10B8: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D10BC: 388A4FC8  addi r4, r10, 0x4fc8
	ctx.r[4].s64 = ctx.r[10].s64 + 20424;
	// 826D10C0: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D10C4: 396BF888  addi r11, r11, -0x778
	ctx.r[11].s64 = ctx.r[11].s64 + -1912;
	// 826D10C8: 90E10064  stw r7, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[7].u32 ) };
	// 826D10CC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D10D0: 90C1005C  stw r6, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[6].u32 ) };
	// 826D10D4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D10D8: 386AA6AC  addi r3, r10, -0x5954
	ctx.r[3].s64 = ctx.r[10].s64 + -22868;
	// 826D10DC: 39400004  li r10, 4
	ctx.r[10].s64 = 4;
	// 826D10E0: 91010074  stw r8, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[8].u32 ) };
	// 826D10E4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D10E8: 9101006C  stw r8, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[8].u32 ) };
	// 826D10EC: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826D10F0: 4BD95D31  bl 0x82466e20
	ctx.lr = 0x826D10F4;
	sub_82466E20(ctx, base);
	// 826D10F4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D10F8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D10FC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D1100: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D1108(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D1108 size=116
    let mut pc: u32 = 0x826D1108;
    'dispatch: loop {
        match pc {
            0x826D1108 => {
    //   block [0x826D1108..0x826D117C)
	// 826D1108: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D110C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D1110: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D1114: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D1118: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 826D111C: 390BDC98  addi r8, r11, -0x2368
	ctx.r[8].s64 = ctx.r[11].s64 + -9064;
	// 826D1120: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D1124: 392A399C  addi r9, r10, 0x399c
	ctx.r[9].s64 = ctx.r[10].s64 + 14748;
	// 826D1128: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D112C: 38E00005  li r7, 5
	ctx.r[7].s64 = 5;
	// 826D1130: 38AA86FC  addi r5, r10, -0x7904
	ctx.r[5].s64 = ctx.r[10].s64 + -30980;
	// 826D1134: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826D1138: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D113C: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826D1140: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D1144: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D1148: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D114C: 3D608299  lis r11, -0x7d67
	ctx.r[11].s64 = -2103902208;
	// 826D1150: 388A4FD8  addi r4, r10, 0x4fd8
	ctx.r[4].s64 = ctx.r[10].s64 + 20440;
	// 826D1154: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826D1158: 386BA6DC  addi r3, r11, -0x5924
	ctx.r[3].s64 = ctx.r[11].s64 + -22820;
	// 826D115C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826D1160: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D1164: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D1168: 4BD95CB9  bl 0x82466e20
	ctx.lr = 0x826D116C;
	sub_82466E20(ctx, base);
	// 826D116C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D1170: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D1174: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D1178: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D1180(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D1180 size=112
    let mut pc: u32 = 0x826D1180;
    'dispatch: loop {
        match pc {
            0x826D1180 => {
    //   block [0x826D1180..0x826D11F0)
	// 826D1180: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D1184: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D1188: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D118C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D1190: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D1194: 38AA86FC  addi r5, r10, -0x7904
	ctx.r[5].s64 = ctx.r[10].s64 + -30980;
	// 826D1198: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826D119C: 390BDD10  addi r8, r11, -0x22f0
	ctx.r[8].s64 = ctx.r[11].s64 + -8944;
	// 826D11A0: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826D11A4: 388A4FE8  addi r4, r10, 0x4fe8
	ctx.r[4].s64 = ctx.r[10].s64 + 20456;
	// 826D11A8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D11AC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D11B0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826D11B4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D11B8: 386AA70C  addi r3, r10, -0x58f4
	ctx.r[3].s64 = ctx.r[10].s64 + -22772;
	// 826D11BC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826D11C0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D11C4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D11C8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D11CC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D11D0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D11D4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D11D8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D11DC: 4BD95C45  bl 0x82466e20
	ctx.lr = 0x826D11E0;
	sub_82466E20(ctx, base);
	// 826D11E0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D11E4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D11E8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D11EC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


