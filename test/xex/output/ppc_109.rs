pub fn sub_826C6AF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C6AF0 size=112
    let mut pc: u32 = 0x826C6AF0;
    'dispatch: loop {
        match pc {
            0x826C6AF0 => {
    //   block [0x826C6AF0..0x826C6B60)
	// 826C6AF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C6AF4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C6AF8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C6AFC: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C6B00: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826C6B04: 38AA6194  addi r5, r10, 0x6194
	ctx.r[5].s64 = ctx.r[10].s64 + 24980;
	// 826C6B08: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C6B0C: 390B6800  addi r8, r11, 0x6800
	ctx.r[8].s64 = ctx.r[11].s64 + 26624;
	// 826C6B10: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826C6B14: 388A3EBC  addi r4, r10, 0x3ebc
	ctx.r[4].s64 = ctx.r[10].s64 + 16060;
	// 826C6B18: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C6B1C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C6B20: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826C6B24: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C6B28: 386A6044  addi r3, r10, 0x6044
	ctx.r[3].s64 = ctx.r[10].s64 + 24644;
	// 826C6B2C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826C6B30: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C6B34: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C6B38: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C6B3C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C6B40: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C6B44: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C6B48: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C6B4C: 4BDA02D5  bl 0x82466e20
	ctx.lr = 0x826C6B50;
	sub_82466E20(ctx, base);
	// 826C6B50: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C6B54: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C6B58: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C6B5C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C6B60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C6B60 size=100
    let mut pc: u32 = 0x826C6B60;
    'dispatch: loop {
        match pc {
            0x826C6B60 => {
    //   block [0x826C6B60..0x826C6BC4)
	// 826C6B60: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C6B64: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C6B68: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C6B6C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C6B70: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C6B74: 38AA5B34  addi r5, r10, 0x5b34
	ctx.r[5].s64 = ctx.r[10].s64 + 23348;
	// 826C6B78: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C6B7C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C6B80: 388A3F14  addi r4, r10, 0x3f14
	ctx.r[4].s64 = ctx.r[10].s64 + 16148;
	// 826C6B84: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C6B88: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C6B8C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C6B90: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C6B94: 386A6074  addi r3, r10, 0x6074
	ctx.r[3].s64 = ctx.r[10].s64 + 24692;
	// 826C6B98: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C6B9C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C6BA0: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826C6BA4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C6BA8: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826C6BAC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C6BB0: 4BDA0271  bl 0x82466e20
	ctx.lr = 0x826C6BB4;
	sub_82466E20(ctx, base);
	// 826C6BB4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C6BB8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C6BBC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C6BC0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C6BC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C6BC8 size=112
    let mut pc: u32 = 0x826C6BC8;
    'dispatch: loop {
        match pc {
            0x826C6BC8 => {
    //   block [0x826C6BC8..0x826C6C38)
	// 826C6BC8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C6BCC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C6BD0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C6BD4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C6BD8: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826C6BDC: 38AA5E94  addi r5, r10, 0x5e94
	ctx.r[5].s64 = ctx.r[10].s64 + 24212;
	// 826C6BE0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C6BE4: 390B6830  addi r8, r11, 0x6830
	ctx.r[8].s64 = ctx.r[11].s64 + 26672;
	// 826C6BE8: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 826C6BEC: 388A3F2C  addi r4, r10, 0x3f2c
	ctx.r[4].s64 = ctx.r[10].s64 + 16172;
	// 826C6BF0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C6BF4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C6BF8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826C6BFC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C6C00: 386A60A4  addi r3, r10, 0x60a4
	ctx.r[3].s64 = ctx.r[10].s64 + 24740;
	// 826C6C04: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826C6C08: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C6C0C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C6C10: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C6C14: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C6C18: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C6C1C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C6C20: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C6C24: 4BDA01FD  bl 0x82466e20
	ctx.lr = 0x826C6C28;
	sub_82466E20(ctx, base);
	// 826C6C28: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C6C2C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C6C30: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C6C34: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C6C38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C6C38 size=112
    let mut pc: u32 = 0x826C6C38;
    'dispatch: loop {
        match pc {
            0x826C6C38 => {
    //   block [0x826C6C38..0x826C6CA8)
	// 826C6C38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C6C3C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C6C40: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C6C44: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C6C48: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826C6C4C: 38AA5E94  addi r5, r10, 0x5e94
	ctx.r[5].s64 = ctx.r[10].s64 + 24212;
	// 826C6C50: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C6C54: 390B6878  addi r8, r11, 0x6878
	ctx.r[8].s64 = ctx.r[11].s64 + 26744;
	// 826C6C58: 39200007  li r9, 7
	ctx.r[9].s64 = 7;
	// 826C6C5C: 388A3F3C  addi r4, r10, 0x3f3c
	ctx.r[4].s64 = ctx.r[10].s64 + 16188;
	// 826C6C60: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C6C64: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C6C68: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826C6C6C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C6C70: 386A60D4  addi r3, r10, 0x60d4
	ctx.r[3].s64 = ctx.r[10].s64 + 24788;
	// 826C6C74: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826C6C78: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C6C7C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C6C80: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C6C84: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C6C88: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C6C8C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C6C90: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C6C94: 4BDA018D  bl 0x82466e20
	ctx.lr = 0x826C6C98;
	sub_82466E20(ctx, base);
	// 826C6C98: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C6C9C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C6CA0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C6CA4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C6CA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C6CA8 size=108
    let mut pc: u32 = 0x826C6CA8;
    'dispatch: loop {
        match pc {
            0x826C6CA8 => {
    //   block [0x826C6CA8..0x826C6D14)
	// 826C6CA8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C6CAC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C6CB0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C6CB4: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826C6CB8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C6CBC: 38EB6920  addi r7, r11, 0x6920
	ctx.r[7].s64 = ctx.r[11].s64 + 26912;
	// 826C6CC0: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 826C6CC4: 388A3F58  addi r4, r10, 0x3f58
	ctx.r[4].s64 = ctx.r[10].s64 + 16216;
	// 826C6CC8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C6CCC: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C6CD0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826C6CD4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C6CD8: 386A6104  addi r3, r10, 0x6104
	ctx.r[3].s64 = ctx.r[10].s64 + 24836;
	// 826C6CDC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826C6CE0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C6CE4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C6CE8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C6CEC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C6CF0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C6CF4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C6CF8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C6CFC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826C6D00: 4BDA0121  bl 0x82466e20
	ctx.lr = 0x826C6D04;
	sub_82466E20(ctx, base);
	// 826C6D04: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C6D08: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C6D0C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C6D10: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C6D18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C6D18 size=112
    let mut pc: u32 = 0x826C6D18;
    'dispatch: loop {
        match pc {
            0x826C6D18 => {
    //   block [0x826C6D18..0x826C6D88)
	// 826C6D18: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C6D1C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C6D20: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C6D24: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C6D28: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826C6D2C: 38AA5CB4  addi r5, r10, 0x5cb4
	ctx.r[5].s64 = ctx.r[10].s64 + 23732;
	// 826C6D30: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C6D34: 390B6968  addi r8, r11, 0x6968
	ctx.r[8].s64 = ctx.r[11].s64 + 26984;
	// 826C6D38: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 826C6D3C: 388A3F74  addi r4, r10, 0x3f74
	ctx.r[4].s64 = ctx.r[10].s64 + 16244;
	// 826C6D40: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C6D44: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C6D48: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826C6D4C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C6D50: 386A6134  addi r3, r10, 0x6134
	ctx.r[3].s64 = ctx.r[10].s64 + 24884;
	// 826C6D54: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826C6D58: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C6D5C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C6D60: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C6D64: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C6D68: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C6D6C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C6D70: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C6D74: 4BDA00AD  bl 0x82466e20
	ctx.lr = 0x826C6D78;
	sub_82466E20(ctx, base);
	// 826C6D78: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C6D7C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C6D80: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C6D84: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C6D88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C6D88 size=100
    let mut pc: u32 = 0x826C6D88;
    'dispatch: loop {
        match pc {
            0x826C6D88 => {
    //   block [0x826C6D88..0x826C6DEC)
	// 826C6D88: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C6D8C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C6D90: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C6D94: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C6D98: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C6D9C: 38AA5CE4  addi r5, r10, 0x5ce4
	ctx.r[5].s64 = ctx.r[10].s64 + 23780;
	// 826C6DA0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C6DA4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C6DA8: 388A3F88  addi r4, r10, 0x3f88
	ctx.r[4].s64 = ctx.r[10].s64 + 16264;
	// 826C6DAC: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C6DB0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C6DB4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C6DB8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C6DBC: 386A6164  addi r3, r10, 0x6164
	ctx.r[3].s64 = ctx.r[10].s64 + 24932;
	// 826C6DC0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C6DC4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C6DC8: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826C6DCC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C6DD0: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826C6DD4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C6DD8: 4BDA0049  bl 0x82466e20
	ctx.lr = 0x826C6DDC;
	sub_82466E20(ctx, base);
	// 826C6DDC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C6DE0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C6DE4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C6DE8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C6DF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C6DF0 size=100
    let mut pc: u32 = 0x826C6DF0;
    'dispatch: loop {
        match pc {
            0x826C6DF0 => {
    //   block [0x826C6DF0..0x826C6E54)
	// 826C6DF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C6DF4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C6DF8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C6DFC: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C6E00: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C6E04: 38AA5B34  addi r5, r10, 0x5b34
	ctx.r[5].s64 = ctx.r[10].s64 + 23348;
	// 826C6E08: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C6E0C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C6E10: 388A3F98  addi r4, r10, 0x3f98
	ctx.r[4].s64 = ctx.r[10].s64 + 16280;
	// 826C6E14: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C6E18: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C6E1C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C6E20: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C6E24: 386A6194  addi r3, r10, 0x6194
	ctx.r[3].s64 = ctx.r[10].s64 + 24980;
	// 826C6E28: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C6E2C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C6E30: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826C6E34: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C6E38: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826C6E3C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C6E40: 4BD9FFE1  bl 0x82466e20
	ctx.lr = 0x826C6E44;
	sub_82466E20(ctx, base);
	// 826C6E44: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C6E48: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C6E4C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C6E50: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C6E58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C6E58 size=112
    let mut pc: u32 = 0x826C6E58;
    'dispatch: loop {
        match pc {
            0x826C6E58 => {
    //   block [0x826C6E58..0x826C6EC8)
	// 826C6E58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C6E5C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C6E60: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C6E64: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C6E68: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826C6E6C: 38AA4EA4  addi r5, r10, 0x4ea4
	ctx.r[5].s64 = ctx.r[10].s64 + 20132;
	// 826C6E70: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C6E74: 390B69C8  addi r8, r11, 0x69c8
	ctx.r[8].s64 = ctx.r[11].s64 + 27080;
	// 826C6E78: 39200006  li r9, 6
	ctx.r[9].s64 = 6;
	// 826C6E7C: 388A4028  addi r4, r10, 0x4028
	ctx.r[4].s64 = ctx.r[10].s64 + 16424;
	// 826C6E80: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C6E84: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C6E88: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826C6E8C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C6E90: 386A61C4  addi r3, r10, 0x61c4
	ctx.r[3].s64 = ctx.r[10].s64 + 25028;
	// 826C6E94: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826C6E98: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C6E9C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C6EA0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C6EA4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C6EA8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C6EAC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C6EB0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C6EB4: 4BD9FF6D  bl 0x82466e20
	ctx.lr = 0x826C6EB8;
	sub_82466E20(ctx, base);
	// 826C6EB8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C6EBC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C6EC0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C6EC4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C6EC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C6EC8 size=112
    let mut pc: u32 = 0x826C6EC8;
    'dispatch: loop {
        match pc {
            0x826C6EC8 => {
    //   block [0x826C6EC8..0x826C6F38)
	// 826C6EC8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C6ECC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C6ED0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C6ED4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C6ED8: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826C6EDC: 38AA5F84  addi r5, r10, 0x5f84
	ctx.r[5].s64 = ctx.r[10].s64 + 24452;
	// 826C6EE0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C6EE4: 390B6A58  addi r8, r11, 0x6a58
	ctx.r[8].s64 = ctx.r[11].s64 + 27224;
	// 826C6EE8: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826C6EEC: 388A404C  addi r4, r10, 0x404c
	ctx.r[4].s64 = ctx.r[10].s64 + 16460;
	// 826C6EF0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C6EF4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C6EF8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826C6EFC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C6F00: 386A61F4  addi r3, r10, 0x61f4
	ctx.r[3].s64 = ctx.r[10].s64 + 25076;
	// 826C6F04: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826C6F08: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C6F0C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C6F10: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C6F14: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C6F18: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C6F1C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C6F20: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C6F24: 4BD9FEFD  bl 0x82466e20
	ctx.lr = 0x826C6F28;
	sub_82466E20(ctx, base);
	// 826C6F28: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C6F2C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C6F30: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C6F34: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C6F38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C6F38 size=112
    let mut pc: u32 = 0x826C6F38;
    'dispatch: loop {
        match pc {
            0x826C6F38 => {
    //   block [0x826C6F38..0x826C6FA8)
	// 826C6F38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C6F3C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C6F40: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C6F44: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C6F48: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826C6F4C: 38AA60D4  addi r5, r10, 0x60d4
	ctx.r[5].s64 = ctx.r[10].s64 + 24788;
	// 826C6F50: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C6F54: 390B6A70  addi r8, r11, 0x6a70
	ctx.r[8].s64 = ctx.r[11].s64 + 27248;
	// 826C6F58: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826C6F5C: 388A4060  addi r4, r10, 0x4060
	ctx.r[4].s64 = ctx.r[10].s64 + 16480;
	// 826C6F60: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C6F64: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C6F68: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826C6F6C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C6F70: 386A6224  addi r3, r10, 0x6224
	ctx.r[3].s64 = ctx.r[10].s64 + 25124;
	// 826C6F74: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826C6F78: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C6F7C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C6F80: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C6F84: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C6F88: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C6F8C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C6F90: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C6F94: 4BD9FE8D  bl 0x82466e20
	ctx.lr = 0x826C6F98;
	sub_82466E20(ctx, base);
	// 826C6F98: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C6F9C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C6FA0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C6FA4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C6FA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C6FA8 size=112
    let mut pc: u32 = 0x826C6FA8;
    'dispatch: loop {
        match pc {
            0x826C6FA8 => {
    //   block [0x826C6FA8..0x826C7018)
	// 826C6FA8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C6FAC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C6FB0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C6FB4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C6FB8: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826C6FBC: 38AA5B34  addi r5, r10, 0x5b34
	ctx.r[5].s64 = ctx.r[10].s64 + 23348;
	// 826C6FC0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C6FC4: 390B6AA0  addi r8, r11, 0x6aa0
	ctx.r[8].s64 = ctx.r[11].s64 + 27296;
	// 826C6FC8: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 826C6FCC: 388A4084  addi r4, r10, 0x4084
	ctx.r[4].s64 = ctx.r[10].s64 + 16516;
	// 826C6FD0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C6FD4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C6FD8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826C6FDC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C6FE0: 386A6254  addi r3, r10, 0x6254
	ctx.r[3].s64 = ctx.r[10].s64 + 25172;
	// 826C6FE4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826C6FE8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C6FEC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C6FF0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C6FF4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C6FF8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C6FFC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C7000: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C7004: 4BD9FE1D  bl 0x82466e20
	ctx.lr = 0x826C7008;
	sub_82466E20(ctx, base);
	// 826C7008: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C700C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C7010: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C7014: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C7018(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C7018 size=112
    let mut pc: u32 = 0x826C7018;
    'dispatch: loop {
        match pc {
            0x826C7018 => {
    //   block [0x826C7018..0x826C7088)
	// 826C7018: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C701C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C7020: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C7024: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C7028: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826C702C: 38AA5CE4  addi r5, r10, 0x5ce4
	ctx.r[5].s64 = ctx.r[10].s64 + 23780;
	// 826C7030: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C7034: 390B6AE8  addi r8, r11, 0x6ae8
	ctx.r[8].s64 = ctx.r[11].s64 + 27368;
	// 826C7038: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 826C703C: 388A40B8  addi r4, r10, 0x40b8
	ctx.r[4].s64 = ctx.r[10].s64 + 16568;
	// 826C7040: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C7044: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C7048: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826C704C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C7050: 386A6284  addi r3, r10, 0x6284
	ctx.r[3].s64 = ctx.r[10].s64 + 25220;
	// 826C7054: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826C7058: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C705C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C7060: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C7064: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C7068: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C706C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C7070: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C7074: 4BD9FDAD  bl 0x82466e20
	ctx.lr = 0x826C7078;
	sub_82466E20(ctx, base);
	// 826C7078: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C707C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C7080: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C7084: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C7088(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C7088 size=112
    let mut pc: u32 = 0x826C7088;
    'dispatch: loop {
        match pc {
            0x826C7088 => {
    //   block [0x826C7088..0x826C70F8)
	// 826C7088: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C708C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C7090: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C7094: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C7098: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826C709C: 38AA5C54  addi r5, r10, 0x5c54
	ctx.r[5].s64 = ctx.r[10].s64 + 23636;
	// 826C70A0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C70A4: 390B6B30  addi r8, r11, 0x6b30
	ctx.r[8].s64 = ctx.r[11].s64 + 27440;
	// 826C70A8: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826C70AC: 388A40C8  addi r4, r10, 0x40c8
	ctx.r[4].s64 = ctx.r[10].s64 + 16584;
	// 826C70B0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C70B4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C70B8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826C70BC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C70C0: 386A62B4  addi r3, r10, 0x62b4
	ctx.r[3].s64 = ctx.r[10].s64 + 25268;
	// 826C70C4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826C70C8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C70CC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C70D0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C70D4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C70D8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C70DC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C70E0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C70E4: 4BD9FD3D  bl 0x82466e20
	ctx.lr = 0x826C70E8;
	sub_82466E20(ctx, base);
	// 826C70E8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C70EC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C70F0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C70F4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C70F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C70F8 size=112
    let mut pc: u32 = 0x826C70F8;
    'dispatch: loop {
        match pc {
            0x826C70F8 => {
    //   block [0x826C70F8..0x826C7168)
	// 826C70F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C70FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C7100: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C7104: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C7108: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826C710C: 38AA5CB4  addi r5, r10, 0x5cb4
	ctx.r[5].s64 = ctx.r[10].s64 + 23732;
	// 826C7110: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C7114: 390B6B48  addi r8, r11, 0x6b48
	ctx.r[8].s64 = ctx.r[11].s64 + 27464;
	// 826C7118: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826C711C: 388A40EC  addi r4, r10, 0x40ec
	ctx.r[4].s64 = ctx.r[10].s64 + 16620;
	// 826C7120: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C7124: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C7128: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826C712C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C7130: 386A62E4  addi r3, r10, 0x62e4
	ctx.r[3].s64 = ctx.r[10].s64 + 25316;
	// 826C7134: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826C7138: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C713C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C7140: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C7144: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C7148: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C714C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C7150: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C7154: 4BD9FCCD  bl 0x82466e20
	ctx.lr = 0x826C7158;
	sub_82466E20(ctx, base);
	// 826C7158: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C715C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C7160: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C7164: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C7168(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826C7168 size=24
    let mut pc: u32 = 0x826C7168;
    'dispatch: loop {
        match pc {
            0x826C7168 => {
    //   block [0x826C7168..0x826C7180)
	// 826C7168: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826C716C: 3D40827F  lis r10, -0x7d81
	ctx.r[10].s64 = -2105606144;
	// 826C7170: 394AA418  addi r10, r10, -0x5be8
	ctx.r[10].s64 = ctx.r[10].s64 + -23528;
	// 826C7174: 816B670C  lwz r11, 0x670c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(26380 as u32) ) } as u64;
	// 826C7178: 916A0008  stw r11, 8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 826C717C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C7180(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C7180 size=112
    let mut pc: u32 = 0x826C7180;
    'dispatch: loop {
        match pc {
            0x826C7180 => {
    //   block [0x826C7180..0x826C71F0)
	// 826C7180: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C7184: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C7188: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C718C: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 826C7190: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826C7194: 392A2548  addi r9, r10, 0x2548
	ctx.r[9].s64 = ctx.r[10].s64 + 9544;
	// 826C7198: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C719C: 390BA418  addi r8, r11, -0x5be8
	ctx.r[8].s64 = ctx.r[11].s64 + -23528;
	// 826C71A0: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
	// 826C71A4: 388A4124  addi r4, r10, 0x4124
	ctx.r[4].s64 = ctx.r[10].s64 + 16676;
	// 826C71A8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C71AC: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C71B0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826C71B4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C71B8: 386A6314  addi r3, r10, 0x6314
	ctx.r[3].s64 = ctx.r[10].s64 + 25364;
	// 826C71BC: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826C71C0: 39400002  li r10, 2
	ctx.r[10].s64 = 2;
	// 826C71C4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C71C8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C71CC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C71D0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C71D4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826C71D8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C71DC: 4BD9FC45  bl 0x82466e20
	ctx.lr = 0x826C71E0;
	sub_82466E20(ctx, base);
	// 826C71E0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C71E4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C71E8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C71EC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C71F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C71F0 size=112
    let mut pc: u32 = 0x826C71F0;
    'dispatch: loop {
        match pc {
            0x826C71F0 => {
    //   block [0x826C71F0..0x826C7260)
	// 826C71F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C71F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C71F8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C71FC: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C7200: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826C7204: 38AA6314  addi r5, r10, 0x6314
	ctx.r[5].s64 = ctx.r[10].s64 + 25364;
	// 826C7208: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C720C: 390B6B7C  addi r8, r11, 0x6b7c
	ctx.r[8].s64 = ctx.r[11].s64 + 27516;
	// 826C7210: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826C7214: 388A4138  addi r4, r10, 0x4138
	ctx.r[4].s64 = ctx.r[10].s64 + 16696;
	// 826C7218: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C721C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C7220: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826C7224: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C7228: 386A6344  addi r3, r10, 0x6344
	ctx.r[3].s64 = ctx.r[10].s64 + 25412;
	// 826C722C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826C7230: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C7234: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C7238: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C723C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C7240: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C7244: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C7248: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C724C: 4BD9FBD5  bl 0x82466e20
	ctx.lr = 0x826C7250;
	sub_82466E20(ctx, base);
	// 826C7250: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C7254: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C7258: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C725C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C7260(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C7260 size=108
    let mut pc: u32 = 0x826C7260;
    'dispatch: loop {
        match pc {
            0x826C7260 => {
    //   block [0x826C7260..0x826C72CC)
	// 826C7260: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C7264: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C7268: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C726C: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826C7270: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C7274: 38EB6BAC  addi r7, r11, 0x6bac
	ctx.r[7].s64 = ctx.r[11].s64 + 27564;
	// 826C7278: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826C727C: 388A4150  addi r4, r10, 0x4150
	ctx.r[4].s64 = ctx.r[10].s64 + 16720;
	// 826C7280: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C7284: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C7288: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826C728C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C7290: 386A6374  addi r3, r10, 0x6374
	ctx.r[3].s64 = ctx.r[10].s64 + 25460;
	// 826C7294: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826C7298: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C729C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C72A0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C72A4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C72A8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C72AC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C72B0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C72B4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826C72B8: 4BD9FB69  bl 0x82466e20
	ctx.lr = 0x826C72BC;
	sub_82466E20(ctx, base);
	// 826C72BC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C72C0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C72C4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C72C8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C72D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C72D0 size=100
    let mut pc: u32 = 0x826C72D0;
    'dispatch: loop {
        match pc {
            0x826C72D0 => {
    //   block [0x826C72D0..0x826C7334)
	// 826C72D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C72D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C72D8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C72DC: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C72E0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C72E4: 38AA6314  addi r5, r10, 0x6314
	ctx.r[5].s64 = ctx.r[10].s64 + 25364;
	// 826C72E8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C72EC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C72F0: 388A4160  addi r4, r10, 0x4160
	ctx.r[4].s64 = ctx.r[10].s64 + 16736;
	// 826C72F4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C72F8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C72FC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C7300: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C7304: 386A63A4  addi r3, r10, 0x63a4
	ctx.r[3].s64 = ctx.r[10].s64 + 25508;
	// 826C7308: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C730C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C7310: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826C7314: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C7318: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826C731C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C7320: 4BD9FB01  bl 0x82466e20
	ctx.lr = 0x826C7324;
	sub_82466E20(ctx, base);
	// 826C7324: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C7328: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C732C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C7330: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C7338(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C7338 size=112
    let mut pc: u32 = 0x826C7338;
    'dispatch: loop {
        match pc {
            0x826C7338 => {
    //   block [0x826C7338..0x826C73A8)
	// 826C7338: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C733C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C7340: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C7344: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C7348: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826C734C: 38AA6314  addi r5, r10, 0x6314
	ctx.r[5].s64 = ctx.r[10].s64 + 25364;
	// 826C7350: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C7354: 390B6BC4  addi r8, r11, 0x6bc4
	ctx.r[8].s64 = ctx.r[11].s64 + 27588;
	// 826C7358: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826C735C: 388A417C  addi r4, r10, 0x417c
	ctx.r[4].s64 = ctx.r[10].s64 + 16764;
	// 826C7360: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C7364: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C7368: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826C736C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C7370: 386A63D4  addi r3, r10, 0x63d4
	ctx.r[3].s64 = ctx.r[10].s64 + 25556;
	// 826C7374: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826C7378: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C737C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C7380: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C7384: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C7388: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C738C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C7390: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C7394: 4BD9FA8D  bl 0x82466e20
	ctx.lr = 0x826C7398;
	sub_82466E20(ctx, base);
	// 826C7398: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C739C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C73A0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C73A4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C73A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C73A8 size=112
    let mut pc: u32 = 0x826C73A8;
    'dispatch: loop {
        match pc {
            0x826C73A8 => {
    //   block [0x826C73A8..0x826C7418)
	// 826C73A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C73AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C73B0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C73B4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C73B8: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826C73BC: 38AA6314  addi r5, r10, 0x6314
	ctx.r[5].s64 = ctx.r[10].s64 + 25364;
	// 826C73C0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C73C4: 390B6BDC  addi r8, r11, 0x6bdc
	ctx.r[8].s64 = ctx.r[11].s64 + 27612;
	// 826C73C8: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826C73CC: 388A4198  addi r4, r10, 0x4198
	ctx.r[4].s64 = ctx.r[10].s64 + 16792;
	// 826C73D0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C73D4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C73D8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826C73DC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C73E0: 386A6404  addi r3, r10, 0x6404
	ctx.r[3].s64 = ctx.r[10].s64 + 25604;
	// 826C73E4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826C73E8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C73EC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C73F0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C73F4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C73F8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C73FC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C7400: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C7404: 4BD9FA1D  bl 0x82466e20
	ctx.lr = 0x826C7408;
	sub_82466E20(ctx, base);
	// 826C7408: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C740C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C7410: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C7414: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C7418(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C7418 size=112
    let mut pc: u32 = 0x826C7418;
    'dispatch: loop {
        match pc {
            0x826C7418 => {
    //   block [0x826C7418..0x826C7488)
	// 826C7418: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C741C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C7420: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C7424: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C7428: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826C742C: 38AA6314  addi r5, r10, 0x6314
	ctx.r[5].s64 = ctx.r[10].s64 + 25364;
	// 826C7430: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C7434: 390B6C0C  addi r8, r11, 0x6c0c
	ctx.r[8].s64 = ctx.r[11].s64 + 27660;
	// 826C7438: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826C743C: 388A41BC  addi r4, r10, 0x41bc
	ctx.r[4].s64 = ctx.r[10].s64 + 16828;
	// 826C7440: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C7444: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C7448: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826C744C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C7450: 386A6434  addi r3, r10, 0x6434
	ctx.r[3].s64 = ctx.r[10].s64 + 25652;
	// 826C7454: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826C7458: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C745C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C7460: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C7464: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C7468: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C746C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C7470: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C7474: 4BD9F9AD  bl 0x82466e20
	ctx.lr = 0x826C7478;
	sub_82466E20(ctx, base);
	// 826C7478: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C747C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C7480: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C7484: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C7488(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C7488 size=112
    let mut pc: u32 = 0x826C7488;
    'dispatch: loop {
        match pc {
            0x826C7488 => {
    //   block [0x826C7488..0x826C74F8)
	// 826C7488: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C748C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C7490: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C7494: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C7498: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826C749C: 38AA6314  addi r5, r10, 0x6314
	ctx.r[5].s64 = ctx.r[10].s64 + 25364;
	// 826C74A0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C74A4: 390B6C3C  addi r8, r11, 0x6c3c
	ctx.r[8].s64 = ctx.r[11].s64 + 27708;
	// 826C74A8: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826C74AC: 388A41E4  addi r4, r10, 0x41e4
	ctx.r[4].s64 = ctx.r[10].s64 + 16868;
	// 826C74B0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C74B4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C74B8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826C74BC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C74C0: 386A6464  addi r3, r10, 0x6464
	ctx.r[3].s64 = ctx.r[10].s64 + 25700;
	// 826C74C4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826C74C8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C74CC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C74D0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C74D4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C74D8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C74DC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C74E0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C74E4: 4BD9F93D  bl 0x82466e20
	ctx.lr = 0x826C74E8;
	sub_82466E20(ctx, base);
	// 826C74E8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C74EC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C74F0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C74F4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C74F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C74F8 size=112
    let mut pc: u32 = 0x826C74F8;
    'dispatch: loop {
        match pc {
            0x826C74F8 => {
    //   block [0x826C74F8..0x826C7568)
	// 826C74F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C74FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C7500: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C7504: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C7508: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826C750C: 38AA6314  addi r5, r10, 0x6314
	ctx.r[5].s64 = ctx.r[10].s64 + 25364;
	// 826C7510: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C7514: 390B6C6C  addi r8, r11, 0x6c6c
	ctx.r[8].s64 = ctx.r[11].s64 + 27756;
	// 826C7518: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826C751C: 388A4208  addi r4, r10, 0x4208
	ctx.r[4].s64 = ctx.r[10].s64 + 16904;
	// 826C7520: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C7524: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C7528: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826C752C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C7530: 386A6494  addi r3, r10, 0x6494
	ctx.r[3].s64 = ctx.r[10].s64 + 25748;
	// 826C7534: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826C7538: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C753C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C7540: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C7544: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C7548: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C754C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C7550: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C7554: 4BD9F8CD  bl 0x82466e20
	ctx.lr = 0x826C7558;
	sub_82466E20(ctx, base);
	// 826C7558: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C755C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C7560: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C7564: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C7568(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C7568 size=112
    let mut pc: u32 = 0x826C7568;
    'dispatch: loop {
        match pc {
            0x826C7568 => {
    //   block [0x826C7568..0x826C75D8)
	// 826C7568: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C756C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C7570: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C7574: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C7578: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826C757C: 38AA6314  addi r5, r10, 0x6314
	ctx.r[5].s64 = ctx.r[10].s64 + 25364;
	// 826C7580: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C7584: 390B6C84  addi r8, r11, 0x6c84
	ctx.r[8].s64 = ctx.r[11].s64 + 27780;
	// 826C7588: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826C758C: 388A4228  addi r4, r10, 0x4228
	ctx.r[4].s64 = ctx.r[10].s64 + 16936;
	// 826C7590: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C7594: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C7598: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826C759C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C75A0: 386A64C4  addi r3, r10, 0x64c4
	ctx.r[3].s64 = ctx.r[10].s64 + 25796;
	// 826C75A4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826C75A8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C75AC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C75B0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C75B4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C75B8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C75BC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C75C0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C75C4: 4BD9F85D  bl 0x82466e20
	ctx.lr = 0x826C75C8;
	sub_82466E20(ctx, base);
	// 826C75C8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C75CC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C75D0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C75D4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C75D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C75D8 size=112
    let mut pc: u32 = 0x826C75D8;
    'dispatch: loop {
        match pc {
            0x826C75D8 => {
    //   block [0x826C75D8..0x826C7648)
	// 826C75D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C75DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C75E0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C75E4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C75E8: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826C75EC: 38AA6314  addi r5, r10, 0x6314
	ctx.r[5].s64 = ctx.r[10].s64 + 25364;
	// 826C75F0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C75F4: 390B6CA0  addi r8, r11, 0x6ca0
	ctx.r[8].s64 = ctx.r[11].s64 + 27808;
	// 826C75F8: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 826C75FC: 388A423C  addi r4, r10, 0x423c
	ctx.r[4].s64 = ctx.r[10].s64 + 16956;
	// 826C7600: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C7604: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C7608: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826C760C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C7610: 386A64F4  addi r3, r10, 0x64f4
	ctx.r[3].s64 = ctx.r[10].s64 + 25844;
	// 826C7614: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826C7618: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C761C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C7620: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C7624: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C7628: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C762C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C7630: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C7634: 4BD9F7ED  bl 0x82466e20
	ctx.lr = 0x826C7638;
	sub_82466E20(ctx, base);
	// 826C7638: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C763C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C7640: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C7644: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C7648(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C7648 size=112
    let mut pc: u32 = 0x826C7648;
    'dispatch: loop {
        match pc {
            0x826C7648 => {
    //   block [0x826C7648..0x826C76B8)
	// 826C7648: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C764C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C7650: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C7654: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C7658: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826C765C: 38AA6314  addi r5, r10, 0x6314
	ctx.r[5].s64 = ctx.r[10].s64 + 25364;
	// 826C7660: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C7664: 390B6CE8  addi r8, r11, 0x6ce8
	ctx.r[8].s64 = ctx.r[11].s64 + 27880;
	// 826C7668: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 826C766C: 388A4254  addi r4, r10, 0x4254
	ctx.r[4].s64 = ctx.r[10].s64 + 16980;
	// 826C7670: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C7674: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C7678: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826C767C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C7680: 386A6524  addi r3, r10, 0x6524
	ctx.r[3].s64 = ctx.r[10].s64 + 25892;
	// 826C7684: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826C7688: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C768C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C7690: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C7694: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C7698: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C769C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C76A0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C76A4: 4BD9F77D  bl 0x82466e20
	ctx.lr = 0x826C76A8;
	sub_82466E20(ctx, base);
	// 826C76A8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C76AC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C76B0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C76B4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C76B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C76B8 size=112
    let mut pc: u32 = 0x826C76B8;
    'dispatch: loop {
        match pc {
            0x826C76B8 => {
    //   block [0x826C76B8..0x826C7728)
	// 826C76B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C76BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C76C0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C76C4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C76C8: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826C76CC: 38AA6314  addi r5, r10, 0x6314
	ctx.r[5].s64 = ctx.r[10].s64 + 25364;
	// 826C76D0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C76D4: 390B6D30  addi r8, r11, 0x6d30
	ctx.r[8].s64 = ctx.r[11].s64 + 27952;
	// 826C76D8: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826C76DC: 388A4270  addi r4, r10, 0x4270
	ctx.r[4].s64 = ctx.r[10].s64 + 17008;
	// 826C76E0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C76E4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C76E8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826C76EC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C76F0: 386A6554  addi r3, r10, 0x6554
	ctx.r[3].s64 = ctx.r[10].s64 + 25940;
	// 826C76F4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826C76F8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C76FC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C7700: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C7704: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C7708: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C770C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C7710: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C7714: 4BD9F70D  bl 0x82466e20
	ctx.lr = 0x826C7718;
	sub_82466E20(ctx, base);
	// 826C7718: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C771C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C7720: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C7724: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C7728(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C7728 size=112
    let mut pc: u32 = 0x826C7728;
    'dispatch: loop {
        match pc {
            0x826C7728 => {
    //   block [0x826C7728..0x826C7798)
	// 826C7728: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C772C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C7730: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C7734: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C7738: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826C773C: 38AA6314  addi r5, r10, 0x6314
	ctx.r[5].s64 = ctx.r[10].s64 + 25364;
	// 826C7740: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C7744: 390B6D48  addi r8, r11, 0x6d48
	ctx.r[8].s64 = ctx.r[11].s64 + 27976;
	// 826C7748: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826C774C: 388A4288  addi r4, r10, 0x4288
	ctx.r[4].s64 = ctx.r[10].s64 + 17032;
	// 826C7750: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C7754: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C7758: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826C775C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C7760: 386A6584  addi r3, r10, 0x6584
	ctx.r[3].s64 = ctx.r[10].s64 + 25988;
	// 826C7764: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826C7768: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C776C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C7770: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C7774: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C7778: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C777C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C7780: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C7784: 4BD9F69D  bl 0x82466e20
	ctx.lr = 0x826C7788;
	sub_82466E20(ctx, base);
	// 826C7788: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C778C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C7790: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C7794: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C7798(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C7798 size=116
    let mut pc: u32 = 0x826C7798;
    'dispatch: loop {
        match pc {
            0x826C7798 => {
    //   block [0x826C7798..0x826C780C)
	// 826C7798: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C779C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C77A0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C77A4: 3D40827E  lis r10, -0x7d82
	ctx.r[10].s64 = -2105671680;
	// 826C77A8: 38E00005  li r7, 5
	ctx.r[7].s64 = 5;
	// 826C77AC: 390A6D78  addi r8, r10, 0x6d78
	ctx.r[8].s64 = ctx.r[10].s64 + 28024;
	// 826C77B0: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C77B4: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 826C77B8: 38AA6314  addi r5, r10, 0x6314
	ctx.r[5].s64 = ctx.r[10].s64 + 25364;
	// 826C77BC: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C77C0: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826C77C4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C77C8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826C77CC: 388A429C  addi r4, r10, 0x429c
	ctx.r[4].s64 = ctx.r[10].s64 + 17052;
	// 826C77D0: 396B2570  addi r11, r11, 0x2570
	ctx.r[11].s64 = ctx.r[11].s64 + 9584;
	// 826C77D4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C77D8: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C77DC: 386A65B4  addi r3, r10, 0x65b4
	ctx.r[3].s64 = ctx.r[10].s64 + 26036;
	// 826C77E0: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 826C77E4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C77E8: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 826C77EC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C77F0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C77F4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C77F8: 4BD9F629  bl 0x82466e20
	ctx.lr = 0x826C77FC;
	sub_82466E20(ctx, base);
	// 826C77FC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C7800: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C7804: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C7808: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C7810(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C7810 size=116
    let mut pc: u32 = 0x826C7810;
    'dispatch: loop {
        match pc {
            0x826C7810 => {
    //   block [0x826C7810..0x826C7884)
	// 826C7810: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C7814: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C7818: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C781C: 3D40827E  lis r10, -0x7d82
	ctx.r[10].s64 = -2105671680;
	// 826C7820: 38E00006  li r7, 6
	ctx.r[7].s64 = 6;
	// 826C7824: 390A6DF0  addi r8, r10, 0x6df0
	ctx.r[8].s64 = ctx.r[10].s64 + 28144;
	// 826C7828: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C782C: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 826C7830: 38AA6314  addi r5, r10, 0x6314
	ctx.r[5].s64 = ctx.r[10].s64 + 25364;
	// 826C7834: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C7838: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826C783C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C7840: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826C7844: 388A42B8  addi r4, r10, 0x42b8
	ctx.r[4].s64 = ctx.r[10].s64 + 17080;
	// 826C7848: 396B2588  addi r11, r11, 0x2588
	ctx.r[11].s64 = ctx.r[11].s64 + 9608;
	// 826C784C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C7850: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C7854: 386A65E4  addi r3, r10, 0x65e4
	ctx.r[3].s64 = ctx.r[10].s64 + 26084;
	// 826C7858: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 826C785C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C7860: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 826C7864: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C7868: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C786C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C7870: 4BD9F5B1  bl 0x82466e20
	ctx.lr = 0x826C7874;
	sub_82466E20(ctx, base);
	// 826C7874: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C7878: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C787C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C7880: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C7888(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826C7888 size=24
    let mut pc: u32 = 0x826C7888;
    'dispatch: loop {
        match pc {
            0x826C7888 => {
    //   block [0x826C7888..0x826C78A0)
	// 826C7888: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826C788C: 3D40827F  lis r10, -0x7d81
	ctx.r[10].s64 = -2105606144;
	// 826C7890: 394AA430  addi r10, r10, -0x5bd0
	ctx.r[10].s64 = ctx.r[10].s64 + -23504;
	// 826C7894: 816B6C9C  lwz r11, 0x6c9c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(27804 as u32) ) } as u64;
	// 826C7898: 916A0050  stw r11, 0x50(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 826C789C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C78A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C78A0 size=116
    let mut pc: u32 = 0x826C78A0;
    'dispatch: loop {
        match pc {
            0x826C78A0 => {
    //   block [0x826C78A0..0x826C7914)
	// 826C78A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C78A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C78A8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C78AC: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 826C78B0: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C78B4: 392B25B4  addi r9, r11, 0x25b4
	ctx.r[9].s64 = ctx.r[11].s64 + 9652;
	// 826C78B8: 38AA6314  addi r5, r10, 0x6314
	ctx.r[5].s64 = ctx.r[10].s64 + 25364;
	// 826C78BC: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C78C0: 38E90014  addi r7, r9, 0x14
	ctx.r[7].s64 = ctx.r[9].s64 + 20;
	// 826C78C4: 38C00007  li r6, 7
	ctx.r[6].s64 = 7;
	// 826C78C8: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826C78CC: 388A42D4  addi r4, r10, 0x42d4
	ctx.r[4].s64 = ctx.r[10].s64 + 17108;
	// 826C78D0: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C78D4: 396BA430  addi r11, r11, -0x5bd0
	ctx.r[11].s64 = ctx.r[11].s64 + -23504;
	// 826C78D8: 90E10064  stw r7, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[7].u32 ) };
	// 826C78DC: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C78E0: 90C1005C  stw r6, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[6].u32 ) };
	// 826C78E4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C78E8: 386A6614  addi r3, r10, 0x6614
	ctx.r[3].s64 = ctx.r[10].s64 + 26132;
	// 826C78EC: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826C78F0: 91010074  stw r8, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[8].u32 ) };
	// 826C78F4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C78F8: 9101006C  stw r8, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[8].u32 ) };
	// 826C78FC: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826C7900: 4BD9F521  bl 0x82466e20
	ctx.lr = 0x826C7904;
	sub_82466E20(ctx, base);
	// 826C7904: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C7908: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C790C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C7910: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C7918(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C7918 size=112
    let mut pc: u32 = 0x826C7918;
    'dispatch: loop {
        match pc {
            0x826C7918 => {
    //   block [0x826C7918..0x826C7988)
	// 826C7918: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C791C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C7920: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C7924: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C7928: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826C792C: 38AA6314  addi r5, r10, 0x6314
	ctx.r[5].s64 = ctx.r[10].s64 + 25364;
	// 826C7930: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C7934: 390B6E80  addi r8, r11, 0x6e80
	ctx.r[8].s64 = ctx.r[11].s64 + 28288;
	// 826C7938: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 826C793C: 388A42F0  addi r4, r10, 0x42f0
	ctx.r[4].s64 = ctx.r[10].s64 + 17136;
	// 826C7940: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C7944: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C7948: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826C794C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C7950: 386A6644  addi r3, r10, 0x6644
	ctx.r[3].s64 = ctx.r[10].s64 + 26180;
	// 826C7954: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826C7958: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C795C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C7960: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C7964: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C7968: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C796C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C7970: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C7974: 4BD9F4AD  bl 0x82466e20
	ctx.lr = 0x826C7978;
	sub_82466E20(ctx, base);
	// 826C7978: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C797C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C7980: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C7984: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C7988(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C7988 size=112
    let mut pc: u32 = 0x826C7988;
    'dispatch: loop {
        match pc {
            0x826C7988 => {
    //   block [0x826C7988..0x826C79F8)
	// 826C7988: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C798C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C7990: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C7994: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C7998: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826C799C: 38AA6314  addi r5, r10, 0x6314
	ctx.r[5].s64 = ctx.r[10].s64 + 25364;
	// 826C79A0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C79A4: 390B6EE0  addi r8, r11, 0x6ee0
	ctx.r[8].s64 = ctx.r[11].s64 + 28384;
	// 826C79A8: 39200007  li r9, 7
	ctx.r[9].s64 = 7;
	// 826C79AC: 388A430C  addi r4, r10, 0x430c
	ctx.r[4].s64 = ctx.r[10].s64 + 17164;
	// 826C79B0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C79B4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C79B8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826C79BC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C79C0: 386A6674  addi r3, r10, 0x6674
	ctx.r[3].s64 = ctx.r[10].s64 + 26228;
	// 826C79C4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826C79C8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C79CC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C79D0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C79D4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C79D8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C79DC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C79E0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C79E4: 4BD9F43D  bl 0x82466e20
	ctx.lr = 0x826C79E8;
	sub_82466E20(ctx, base);
	// 826C79E8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C79EC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C79F0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C79F4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C79F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C79F8 size=112
    let mut pc: u32 = 0x826C79F8;
    'dispatch: loop {
        match pc {
            0x826C79F8 => {
    //   block [0x826C79F8..0x826C7A68)
	// 826C79F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C79FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C7A00: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C7A04: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C7A08: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826C7A0C: 38AA6314  addi r5, r10, 0x6314
	ctx.r[5].s64 = ctx.r[10].s64 + 25364;
	// 826C7A10: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C7A14: 390B6F88  addi r8, r11, 0x6f88
	ctx.r[8].s64 = ctx.r[11].s64 + 28552;
	// 826C7A18: 39200005  li r9, 5
	ctx.r[9].s64 = 5;
	// 826C7A1C: 388A4328  addi r4, r10, 0x4328
	ctx.r[4].s64 = ctx.r[10].s64 + 17192;
	// 826C7A20: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C7A24: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C7A28: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826C7A2C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C7A30: 386A66A4  addi r3, r10, 0x66a4
	ctx.r[3].s64 = ctx.r[10].s64 + 26276;
	// 826C7A34: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826C7A38: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C7A3C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C7A40: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C7A44: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C7A48: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C7A4C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C7A50: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C7A54: 4BD9F3CD  bl 0x82466e20
	ctx.lr = 0x826C7A58;
	sub_82466E20(ctx, base);
	// 826C7A58: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C7A5C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C7A60: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C7A64: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C7A68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C7A68 size=112
    let mut pc: u32 = 0x826C7A68;
    'dispatch: loop {
        match pc {
            0x826C7A68 => {
    //   block [0x826C7A68..0x826C7AD8)
	// 826C7A68: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C7A6C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C7A70: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C7A74: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C7A78: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826C7A7C: 38AA6314  addi r5, r10, 0x6314
	ctx.r[5].s64 = ctx.r[10].s64 + 25364;
	// 826C7A80: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C7A84: 390B7000  addi r8, r11, 0x7000
	ctx.r[8].s64 = ctx.r[11].s64 + 28672;
	// 826C7A88: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 826C7A8C: 388A4348  addi r4, r10, 0x4348
	ctx.r[4].s64 = ctx.r[10].s64 + 17224;
	// 826C7A90: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C7A94: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C7A98: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826C7A9C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C7AA0: 386A66D4  addi r3, r10, 0x66d4
	ctx.r[3].s64 = ctx.r[10].s64 + 26324;
	// 826C7AA4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826C7AA8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C7AAC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C7AB0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C7AB4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C7AB8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C7ABC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C7AC0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C7AC4: 4BD9F35D  bl 0x82466e20
	ctx.lr = 0x826C7AC8;
	sub_82466E20(ctx, base);
	// 826C7AC8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C7ACC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C7AD0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C7AD4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C7AD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C7AD8 size=112
    let mut pc: u32 = 0x826C7AD8;
    'dispatch: loop {
        match pc {
            0x826C7AD8 => {
    //   block [0x826C7AD8..0x826C7B48)
	// 826C7AD8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C7ADC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C7AE0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C7AE4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C7AE8: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826C7AEC: 38AA6314  addi r5, r10, 0x6314
	ctx.r[5].s64 = ctx.r[10].s64 + 25364;
	// 826C7AF0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C7AF4: 390B7048  addi r8, r11, 0x7048
	ctx.r[8].s64 = ctx.r[11].s64 + 28744;
	// 826C7AF8: 39200006  li r9, 6
	ctx.r[9].s64 = 6;
	// 826C7AFC: 388A4364  addi r4, r10, 0x4364
	ctx.r[4].s64 = ctx.r[10].s64 + 17252;
	// 826C7B00: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C7B04: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C7B08: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826C7B0C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C7B10: 386A6704  addi r3, r10, 0x6704
	ctx.r[3].s64 = ctx.r[10].s64 + 26372;
	// 826C7B14: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826C7B18: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C7B1C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C7B20: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C7B24: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C7B28: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C7B2C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C7B30: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C7B34: 4BD9F2ED  bl 0x82466e20
	ctx.lr = 0x826C7B38;
	sub_82466E20(ctx, base);
	// 826C7B38: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C7B3C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C7B40: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C7B44: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C7B48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C7B48 size=112
    let mut pc: u32 = 0x826C7B48;
    'dispatch: loop {
        match pc {
            0x826C7B48 => {
    //   block [0x826C7B48..0x826C7BB8)
	// 826C7B48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C7B4C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C7B50: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C7B54: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C7B58: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826C7B5C: 38AA6314  addi r5, r10, 0x6314
	ctx.r[5].s64 = ctx.r[10].s64 + 25364;
	// 826C7B60: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C7B64: 390B70D8  addi r8, r11, 0x70d8
	ctx.r[8].s64 = ctx.r[11].s64 + 28888;
	// 826C7B68: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 826C7B6C: 388A4380  addi r4, r10, 0x4380
	ctx.r[4].s64 = ctx.r[10].s64 + 17280;
	// 826C7B70: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C7B74: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C7B78: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826C7B7C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C7B80: 386A6734  addi r3, r10, 0x6734
	ctx.r[3].s64 = ctx.r[10].s64 + 26420;
	// 826C7B84: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826C7B88: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C7B8C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C7B90: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C7B94: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C7B98: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C7B9C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C7BA0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C7BA4: 4BD9F27D  bl 0x82466e20
	ctx.lr = 0x826C7BA8;
	sub_82466E20(ctx, base);
	// 826C7BA8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C7BAC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C7BB0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C7BB4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C7BB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C7BB8 size=112
    let mut pc: u32 = 0x826C7BB8;
    'dispatch: loop {
        match pc {
            0x826C7BB8 => {
    //   block [0x826C7BB8..0x826C7C28)
	// 826C7BB8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C7BBC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C7BC0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C7BC4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C7BC8: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826C7BCC: 38AA6314  addi r5, r10, 0x6314
	ctx.r[5].s64 = ctx.r[10].s64 + 25364;
	// 826C7BD0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C7BD4: 390B7138  addi r8, r11, 0x7138
	ctx.r[8].s64 = ctx.r[11].s64 + 28984;
	// 826C7BD8: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 826C7BDC: 388A4398  addi r4, r10, 0x4398
	ctx.r[4].s64 = ctx.r[10].s64 + 17304;
	// 826C7BE0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C7BE4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C7BE8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826C7BEC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C7BF0: 386A6764  addi r3, r10, 0x6764
	ctx.r[3].s64 = ctx.r[10].s64 + 26468;
	// 826C7BF4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826C7BF8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C7BFC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C7C00: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C7C04: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C7C08: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C7C0C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C7C10: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C7C14: 4BD9F20D  bl 0x82466e20
	ctx.lr = 0x826C7C18;
	sub_82466E20(ctx, base);
	// 826C7C18: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C7C1C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C7C20: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C7C24: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C7C28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C7C28 size=112
    let mut pc: u32 = 0x826C7C28;
    'dispatch: loop {
        match pc {
            0x826C7C28 => {
    //   block [0x826C7C28..0x826C7C98)
	// 826C7C28: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C7C2C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C7C30: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C7C34: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C7C38: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826C7C3C: 38AA6764  addi r5, r10, 0x6764
	ctx.r[5].s64 = ctx.r[10].s64 + 26468;
	// 826C7C40: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C7C44: 390B7180  addi r8, r11, 0x7180
	ctx.r[8].s64 = ctx.r[11].s64 + 29056;
	// 826C7C48: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826C7C4C: 388A43B4  addi r4, r10, 0x43b4
	ctx.r[4].s64 = ctx.r[10].s64 + 17332;
	// 826C7C50: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C7C54: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C7C58: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826C7C5C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C7C60: 386A6794  addi r3, r10, 0x6794
	ctx.r[3].s64 = ctx.r[10].s64 + 26516;
	// 826C7C64: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826C7C68: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C7C6C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C7C70: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C7C74: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C7C78: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C7C7C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C7C80: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C7C84: 4BD9F19D  bl 0x82466e20
	ctx.lr = 0x826C7C88;
	sub_82466E20(ctx, base);
	// 826C7C88: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C7C8C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C7C90: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C7C94: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C7C98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C7C98 size=112
    let mut pc: u32 = 0x826C7C98;
    'dispatch: loop {
        match pc {
            0x826C7C98 => {
    //   block [0x826C7C98..0x826C7D08)
	// 826C7C98: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C7C9C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C7CA0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C7CA4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C7CA8: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826C7CAC: 38AA6764  addi r5, r10, 0x6764
	ctx.r[5].s64 = ctx.r[10].s64 + 26468;
	// 826C7CB0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C7CB4: 390B71B0  addi r8, r11, 0x71b0
	ctx.r[8].s64 = ctx.r[11].s64 + 29104;
	// 826C7CB8: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826C7CBC: 388A43D8  addi r4, r10, 0x43d8
	ctx.r[4].s64 = ctx.r[10].s64 + 17368;
	// 826C7CC0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C7CC4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C7CC8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826C7CCC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C7CD0: 386A67C4  addi r3, r10, 0x67c4
	ctx.r[3].s64 = ctx.r[10].s64 + 26564;
	// 826C7CD4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826C7CD8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C7CDC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C7CE0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C7CE4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C7CE8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C7CEC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C7CF0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C7CF4: 4BD9F12D  bl 0x82466e20
	ctx.lr = 0x826C7CF8;
	sub_82466E20(ctx, base);
	// 826C7CF8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C7CFC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C7D00: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C7D04: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C7D08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C7D08 size=100
    let mut pc: u32 = 0x826C7D08;
    'dispatch: loop {
        match pc {
            0x826C7D08 => {
    //   block [0x826C7D08..0x826C7D6C)
	// 826C7D08: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C7D0C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C7D10: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C7D14: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C7D18: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C7D1C: 38AA6764  addi r5, r10, 0x6764
	ctx.r[5].s64 = ctx.r[10].s64 + 26468;
	// 826C7D20: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C7D24: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C7D28: 388A43FC  addi r4, r10, 0x43fc
	ctx.r[4].s64 = ctx.r[10].s64 + 17404;
	// 826C7D2C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C7D30: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C7D34: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C7D38: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C7D3C: 386A67F4  addi r3, r10, 0x67f4
	ctx.r[3].s64 = ctx.r[10].s64 + 26612;
	// 826C7D40: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C7D44: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C7D48: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826C7D4C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C7D50: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826C7D54: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C7D58: 4BD9F0C9  bl 0x82466e20
	ctx.lr = 0x826C7D5C;
	sub_82466E20(ctx, base);
	// 826C7D5C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C7D60: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C7D64: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C7D68: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C7D70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C7D70 size=112
    let mut pc: u32 = 0x826C7D70;
    'dispatch: loop {
        match pc {
            0x826C7D70 => {
    //   block [0x826C7D70..0x826C7DE0)
	// 826C7D70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C7D74: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C7D78: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C7D7C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C7D80: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826C7D84: 38AA6764  addi r5, r10, 0x6764
	ctx.r[5].s64 = ctx.r[10].s64 + 26468;
	// 826C7D88: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C7D8C: 390B71E0  addi r8, r11, 0x71e0
	ctx.r[8].s64 = ctx.r[11].s64 + 29152;
	// 826C7D90: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826C7D94: 388A4424  addi r4, r10, 0x4424
	ctx.r[4].s64 = ctx.r[10].s64 + 17444;
	// 826C7D98: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C7D9C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C7DA0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826C7DA4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C7DA8: 386A6824  addi r3, r10, 0x6824
	ctx.r[3].s64 = ctx.r[10].s64 + 26660;
	// 826C7DAC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826C7DB0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C7DB4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C7DB8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C7DBC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C7DC0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C7DC4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C7DC8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C7DCC: 4BD9F055  bl 0x82466e20
	ctx.lr = 0x826C7DD0;
	sub_82466E20(ctx, base);
	// 826C7DD0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C7DD4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C7DD8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C7DDC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C7DE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C7DE0 size=112
    let mut pc: u32 = 0x826C7DE0;
    'dispatch: loop {
        match pc {
            0x826C7DE0 => {
    //   block [0x826C7DE0..0x826C7E50)
	// 826C7DE0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C7DE4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C7DE8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C7DEC: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C7DF0: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826C7DF4: 38AA4EA4  addi r5, r10, 0x4ea4
	ctx.r[5].s64 = ctx.r[10].s64 + 20132;
	// 826C7DF8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C7DFC: 390B71F8  addi r8, r11, 0x71f8
	ctx.r[8].s64 = ctx.r[11].s64 + 29176;
	// 826C7E00: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 826C7E04: 388A444C  addi r4, r10, 0x444c
	ctx.r[4].s64 = ctx.r[10].s64 + 17484;
	// 826C7E08: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C7E0C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C7E10: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826C7E14: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C7E18: 386A6854  addi r3, r10, 0x6854
	ctx.r[3].s64 = ctx.r[10].s64 + 26708;
	// 826C7E1C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826C7E20: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C7E24: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C7E28: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C7E2C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C7E30: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C7E34: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C7E38: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C7E3C: 4BD9EFE5  bl 0x82466e20
	ctx.lr = 0x826C7E40;
	sub_82466E20(ctx, base);
	// 826C7E40: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C7E44: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C7E48: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C7E4C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C7E50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C7E50 size=112
    let mut pc: u32 = 0x826C7E50;
    'dispatch: loop {
        match pc {
            0x826C7E50 => {
    //   block [0x826C7E50..0x826C7EC0)
	// 826C7E50: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C7E54: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C7E58: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C7E5C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C7E60: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826C7E64: 38AA6854  addi r5, r10, 0x6854
	ctx.r[5].s64 = ctx.r[10].s64 + 26708;
	// 826C7E68: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C7E6C: 390B7258  addi r8, r11, 0x7258
	ctx.r[8].s64 = ctx.r[11].s64 + 29272;
	// 826C7E70: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826C7E74: 388A4458  addi r4, r10, 0x4458
	ctx.r[4].s64 = ctx.r[10].s64 + 17496;
	// 826C7E78: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C7E7C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C7E80: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826C7E84: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C7E88: 386A6884  addi r3, r10, 0x6884
	ctx.r[3].s64 = ctx.r[10].s64 + 26756;
	// 826C7E8C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826C7E90: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C7E94: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C7E98: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C7E9C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C7EA0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C7EA4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C7EA8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C7EAC: 4BD9EF75  bl 0x82466e20
	ctx.lr = 0x826C7EB0;
	sub_82466E20(ctx, base);
	// 826C7EB0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C7EB4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C7EB8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C7EBC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C7EC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C7EC0 size=112
    let mut pc: u32 = 0x826C7EC0;
    'dispatch: loop {
        match pc {
            0x826C7EC0 => {
    //   block [0x826C7EC0..0x826C7F30)
	// 826C7EC0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C7EC4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C7EC8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C7ECC: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C7ED0: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826C7ED4: 38AA6854  addi r5, r10, 0x6854
	ctx.r[5].s64 = ctx.r[10].s64 + 26708;
	// 826C7ED8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C7EDC: 390B7270  addi r8, r11, 0x7270
	ctx.r[8].s64 = ctx.r[11].s64 + 29296;
	// 826C7EE0: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826C7EE4: 388A4468  addi r4, r10, 0x4468
	ctx.r[4].s64 = ctx.r[10].s64 + 17512;
	// 826C7EE8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C7EEC: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C7EF0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826C7EF4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C7EF8: 386A68B4  addi r3, r10, 0x68b4
	ctx.r[3].s64 = ctx.r[10].s64 + 26804;
	// 826C7EFC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826C7F00: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C7F04: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C7F08: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C7F0C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C7F10: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C7F14: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C7F18: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C7F1C: 4BD9EF05  bl 0x82466e20
	ctx.lr = 0x826C7F20;
	sub_82466E20(ctx, base);
	// 826C7F20: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C7F24: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C7F28: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C7F2C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C7F30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C7F30 size=112
    let mut pc: u32 = 0x826C7F30;
    'dispatch: loop {
        match pc {
            0x826C7F30 => {
    //   block [0x826C7F30..0x826C7FA0)
	// 826C7F30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C7F34: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C7F38: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C7F3C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C7F40: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826C7F44: 38AA6854  addi r5, r10, 0x6854
	ctx.r[5].s64 = ctx.r[10].s64 + 26708;
	// 826C7F48: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C7F4C: 390B72A0  addi r8, r11, 0x72a0
	ctx.r[8].s64 = ctx.r[11].s64 + 29344;
	// 826C7F50: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826C7F54: 388A4478  addi r4, r10, 0x4478
	ctx.r[4].s64 = ctx.r[10].s64 + 17528;
	// 826C7F58: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C7F5C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C7F60: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826C7F64: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C7F68: 386A68E4  addi r3, r10, 0x68e4
	ctx.r[3].s64 = ctx.r[10].s64 + 26852;
	// 826C7F6C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826C7F70: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C7F74: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C7F78: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C7F7C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C7F80: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C7F84: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C7F88: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C7F8C: 4BD9EE95  bl 0x82466e20
	ctx.lr = 0x826C7F90;
	sub_82466E20(ctx, base);
	// 826C7F90: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C7F94: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C7F98: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C7F9C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C7FA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826C7FA0 size=24
    let mut pc: u32 = 0x826C7FA0;
    'dispatch: loop {
        match pc {
            0x826C7FA0 => {
    //   block [0x826C7FA0..0x826C7FB8)
	// 826C7FA0: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826C7FA4: 3D40827F  lis r10, -0x7d81
	ctx.r[10].s64 = -2105606144;
	// 826C7FA8: 394AA4D8  addi r10, r10, -0x5b28
	ctx.r[10].s64 = ctx.r[10].s64 + -23336;
	// 826C7FAC: 816B72B8  lwz r11, 0x72b8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(29368 as u32) ) } as u64;
	// 826C7FB0: 916A0008  stw r11, 8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 826C7FB4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C7FB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C7FB8 size=112
    let mut pc: u32 = 0x826C7FB8;
    'dispatch: loop {
        match pc {
            0x826C7FB8 => {
    //   block [0x826C7FB8..0x826C8028)
	// 826C7FB8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C7FBC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C7FC0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C7FC4: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 826C7FC8: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826C7FCC: 392A2610  addi r9, r10, 0x2610
	ctx.r[9].s64 = ctx.r[10].s64 + 9744;
	// 826C7FD0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C7FD4: 390BA4D8  addi r8, r11, -0x5b28
	ctx.r[8].s64 = ctx.r[11].s64 + -23336;
	// 826C7FD8: 38E00003  li r7, 3
	ctx.r[7].s64 = 3;
	// 826C7FDC: 388A4488  addi r4, r10, 0x4488
	ctx.r[4].s64 = ctx.r[10].s64 + 17544;
	// 826C7FE0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C7FE4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C7FE8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826C7FEC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C7FF0: 386A6914  addi r3, r10, 0x6914
	ctx.r[3].s64 = ctx.r[10].s64 + 26900;
	// 826C7FF4: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826C7FF8: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826C7FFC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C8000: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C8004: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C8008: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C800C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826C8010: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C8014: 4BD9EE0D  bl 0x82466e20
	ctx.lr = 0x826C8018;
	sub_82466E20(ctx, base);
	// 826C8018: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C801C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C8020: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C8024: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C8028(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C8028 size=108
    let mut pc: u32 = 0x826C8028;
    'dispatch: loop {
        match pc {
            0x826C8028 => {
    //   block [0x826C8028..0x826C8094)
	// 826C8028: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C802C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C8030: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C8034: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826C8038: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C803C: 38EB72BC  addi r7, r11, 0x72bc
	ctx.r[7].s64 = ctx.r[11].s64 + 29372;
	// 826C8040: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826C8044: 388A4494  addi r4, r10, 0x4494
	ctx.r[4].s64 = ctx.r[10].s64 + 17556;
	// 826C8048: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C804C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C8050: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826C8054: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C8058: 386A6944  addi r3, r10, 0x6944
	ctx.r[3].s64 = ctx.r[10].s64 + 26948;
	// 826C805C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826C8060: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C8064: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C8068: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C806C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C8070: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C8074: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C8078: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C807C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826C8080: 4BD9EDA1  bl 0x82466e20
	ctx.lr = 0x826C8084;
	sub_82466E20(ctx, base);
	// 826C8084: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C8088: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C808C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C8090: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C8098(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C8098 size=108
    let mut pc: u32 = 0x826C8098;
    'dispatch: loop {
        match pc {
            0x826C8098 => {
    //   block [0x826C8098..0x826C8104)
	// 826C8098: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C809C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C80A0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C80A4: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826C80A8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C80AC: 38EB72D8  addi r7, r11, 0x72d8
	ctx.r[7].s64 = ctx.r[11].s64 + 29400;
	// 826C80B0: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 826C80B4: 388A44A4  addi r4, r10, 0x44a4
	ctx.r[4].s64 = ctx.r[10].s64 + 17572;
	// 826C80B8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C80BC: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C80C0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826C80C4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C80C8: 386A6974  addi r3, r10, 0x6974
	ctx.r[3].s64 = ctx.r[10].s64 + 26996;
	// 826C80CC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826C80D0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C80D4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C80D8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C80DC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C80E0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C80E4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C80E8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C80EC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826C80F0: 4BD9ED31  bl 0x82466e20
	ctx.lr = 0x826C80F4;
	sub_82466E20(ctx, base);
	// 826C80F4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C80F8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C80FC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C8100: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C8108(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C8108 size=116
    let mut pc: u32 = 0x826C8108;
    'dispatch: loop {
        match pc {
            0x826C8108 => {
    //   block [0x826C8108..0x826C817C)
	// 826C8108: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C810C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C8110: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C8114: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826C8118: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 826C811C: 390B7320  addi r8, r11, 0x7320
	ctx.r[8].s64 = ctx.r[11].s64 + 29472;
	// 826C8120: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C8124: 392A26C8  addi r9, r10, 0x26c8
	ctx.r[9].s64 = ctx.r[10].s64 + 9928;
	// 826C8128: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C812C: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
	// 826C8130: 38AA4EA4  addi r5, r10, 0x4ea4
	ctx.r[5].s64 = ctx.r[10].s64 + 20132;
	// 826C8134: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826C8138: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C813C: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C8140: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C8144: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C8148: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C814C: 3D608298  lis r11, -0x7d68
	ctx.r[11].s64 = -2103967744;
	// 826C8150: 388A44B0  addi r4, r10, 0x44b0
	ctx.r[4].s64 = ctx.r[10].s64 + 17584;
	// 826C8154: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826C8158: 386B69A4  addi r3, r11, 0x69a4
	ctx.r[3].s64 = ctx.r[11].s64 + 27044;
	// 826C815C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826C8160: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C8164: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C8168: 4BD9ECB9  bl 0x82466e20
	ctx.lr = 0x826C816C;
	sub_82466E20(ctx, base);
	// 826C816C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C8170: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C8174: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C8178: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C8180(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C8180 size=108
    let mut pc: u32 = 0x826C8180;
    'dispatch: loop {
        match pc {
            0x826C8180 => {
    //   block [0x826C8180..0x826C81EC)
	// 826C8180: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C8184: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C8188: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C818C: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826C8190: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826C8194: 38EB7338  addi r7, r11, 0x7338
	ctx.r[7].s64 = ctx.r[11].s64 + 29496;
	// 826C8198: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 826C819C: 388A21EC  addi r4, r10, 0x21ec
	ctx.r[4].s64 = ctx.r[10].s64 + 8684;
	// 826C81A0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C81A4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C81A8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826C81AC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C81B0: 386A69D4  addi r3, r10, 0x69d4
	ctx.r[3].s64 = ctx.r[10].s64 + 27092;
	// 826C81B4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826C81B8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C81BC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C81C0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C81C4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C81C8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C81CC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C81D0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C81D4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826C81D8: 4BD9EC49  bl 0x82466e20
	ctx.lr = 0x826C81DC;
	sub_82466E20(ctx, base);
	// 826C81DC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C81E0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C81E4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C81E8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C81F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826C81F0 size=24
    let mut pc: u32 = 0x826C81F0;
    'dispatch: loop {
        match pc {
            0x826C81F0 => {
    //   block [0x826C81F0..0x826C8208)
	// 826C81F0: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826C81F4: 3D40827F  lis r10, -0x7d81
	ctx.r[10].s64 = -2105606144;
	// 826C81F8: 394AA520  addi r10, r10, -0x5ae0
	ctx.r[10].s64 = ctx.r[10].s64 + -23264;
	// 826C81FC: 816B7398  lwz r11, 0x7398(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(29592 as u32) ) } as u64;
	// 826C8200: 916A0068  stw r11, 0x68(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(104 as u32), ctx.r[11].u32 ) };
	// 826C8204: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C8208(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C8208 size=116
    let mut pc: u32 = 0x826C8208;
    'dispatch: loop {
        match pc {
            0x826C8208 => {
    //   block [0x826C8208..0x826C827C)
	// 826C8208: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C820C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C8210: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C8214: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826C8218: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 826C821C: 390BA520  addi r8, r11, -0x5ae0
	ctx.r[8].s64 = ctx.r[11].s64 + -23264;
	// 826C8220: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C8224: 392A2724  addi r9, r10, 0x2724
	ctx.r[9].s64 = ctx.r[10].s64 + 10020;
	// 826C8228: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C822C: 38E00009  li r7, 9
	ctx.r[7].s64 = 9;
	// 826C8230: 38AA4EA4  addi r5, r10, 0x4ea4
	ctx.r[5].s64 = ctx.r[10].s64 + 20132;
	// 826C8234: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826C8238: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C823C: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C8240: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C8244: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C8248: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C824C: 3D608298  lis r11, -0x7d68
	ctx.r[11].s64 = -2103967744;
	// 826C8250: 388A44DC  addi r4, r10, 0x44dc
	ctx.r[4].s64 = ctx.r[10].s64 + 17628;
	// 826C8254: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826C8258: 386B6A04  addi r3, r11, 0x6a04
	ctx.r[3].s64 = ctx.r[11].s64 + 27140;
	// 826C825C: 39400003  li r10, 3
	ctx.r[10].s64 = 3;
	// 826C8260: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C8264: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C8268: 4BD9EBB9  bl 0x82466e20
	ctx.lr = 0x826C826C;
	sub_82466E20(ctx, base);
	// 826C826C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C8270: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C8274: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C8278: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C8280(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C8280 size=108
    let mut pc: u32 = 0x826C8280;
    'dispatch: loop {
        match pc {
            0x826C8280 => {
    //   block [0x826C8280..0x826C82EC)
	// 826C8280: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C8284: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C8288: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C828C: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826C8290: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C8294: 38EB73A4  addi r7, r11, 0x73a4
	ctx.r[7].s64 = ctx.r[11].s64 + 29604;
	// 826C8298: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826C829C: 388A44F4  addi r4, r10, 0x44f4
	ctx.r[4].s64 = ctx.r[10].s64 + 17652;
	// 826C82A0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C82A4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C82A8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826C82AC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C82B0: 386A6A34  addi r3, r10, 0x6a34
	ctx.r[3].s64 = ctx.r[10].s64 + 27188;
	// 826C82B4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826C82B8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C82BC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C82C0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C82C4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C82C8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C82CC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C82D0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C82D4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826C82D8: 4BD9EB49  bl 0x82466e20
	ctx.lr = 0x826C82DC;
	sub_82466E20(ctx, base);
	// 826C82DC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C82E0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C82E4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C82E8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C82F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C82F0 size=112
    let mut pc: u32 = 0x826C82F0;
    'dispatch: loop {
        match pc {
            0x826C82F0 => {
    //   block [0x826C82F0..0x826C8360)
	// 826C82F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C82F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C82F8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C82FC: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C8300: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826C8304: 38AA69A4  addi r5, r10, 0x69a4
	ctx.r[5].s64 = ctx.r[10].s64 + 27044;
	// 826C8308: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C830C: 390B73D4  addi r8, r11, 0x73d4
	ctx.r[8].s64 = ctx.r[11].s64 + 29652;
	// 826C8310: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826C8314: 388A4518  addi r4, r10, 0x4518
	ctx.r[4].s64 = ctx.r[10].s64 + 17688;
	// 826C8318: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C831C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C8320: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826C8324: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C8328: 386A6A64  addi r3, r10, 0x6a64
	ctx.r[3].s64 = ctx.r[10].s64 + 27236;
	// 826C832C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826C8330: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C8334: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C8338: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C833C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C8340: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C8344: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C8348: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C834C: 4BD9EAD5  bl 0x82466e20
	ctx.lr = 0x826C8350;
	sub_82466E20(ctx, base);
	// 826C8350: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C8354: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C8358: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C835C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C8360(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C8360 size=112
    let mut pc: u32 = 0x826C8360;
    'dispatch: loop {
        match pc {
            0x826C8360 => {
    //   block [0x826C8360..0x826C83D0)
	// 826C8360: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C8364: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C8368: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C836C: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 826C8370: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826C8374: 392A2768  addi r9, r10, 0x2768
	ctx.r[9].s64 = ctx.r[10].s64 + 10088;
	// 826C8378: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C837C: 390B73F0  addi r8, r11, 0x73f0
	ctx.r[8].s64 = ctx.r[11].s64 + 29680;
	// 826C8380: 38E00003  li r7, 3
	ctx.r[7].s64 = 3;
	// 826C8384: 388A4538  addi r4, r10, 0x4538
	ctx.r[4].s64 = ctx.r[10].s64 + 17720;
	// 826C8388: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C838C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C8390: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826C8394: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C8398: 386A6A94  addi r3, r10, 0x6a94
	ctx.r[3].s64 = ctx.r[10].s64 + 27284;
	// 826C839C: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826C83A0: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826C83A4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C83A8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C83AC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C83B0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C83B4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826C83B8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C83BC: 4BD9EA65  bl 0x82466e20
	ctx.lr = 0x826C83C0;
	sub_82466E20(ctx, base);
	// 826C83C0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C83C4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C83C8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C83CC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C83D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C83D0 size=112
    let mut pc: u32 = 0x826C83D0;
    'dispatch: loop {
        match pc {
            0x826C83D0 => {
    //   block [0x826C83D0..0x826C8440)
	// 826C83D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C83D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C83D8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C83DC: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C83E0: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826C83E4: 38AA69A4  addi r5, r10, 0x69a4
	ctx.r[5].s64 = ctx.r[10].s64 + 27044;
	// 826C83E8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C83EC: 390B7438  addi r8, r11, 0x7438
	ctx.r[8].s64 = ctx.r[11].s64 + 29752;
	// 826C83F0: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826C83F4: 388A4554  addi r4, r10, 0x4554
	ctx.r[4].s64 = ctx.r[10].s64 + 17748;
	// 826C83F8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C83FC: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C8400: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826C8404: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C8408: 386A6AC4  addi r3, r10, 0x6ac4
	ctx.r[3].s64 = ctx.r[10].s64 + 27332;
	// 826C840C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826C8410: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C8414: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C8418: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C841C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C8420: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C8424: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C8428: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C842C: 4BD9E9F5  bl 0x82466e20
	ctx.lr = 0x826C8430;
	sub_82466E20(ctx, base);
	// 826C8430: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C8434: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C8438: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C843C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C8440(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C8440 size=112
    let mut pc: u32 = 0x826C8440;
    'dispatch: loop {
        match pc {
            0x826C8440 => {
    //   block [0x826C8440..0x826C84B0)
	// 826C8440: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C8444: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C8448: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C844C: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 826C8450: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826C8454: 392A2794  addi r9, r10, 0x2794
	ctx.r[9].s64 = ctx.r[10].s64 + 10132;
	// 826C8458: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C845C: 390B7458  addi r8, r11, 0x7458
	ctx.r[8].s64 = ctx.r[11].s64 + 29784;
	// 826C8460: 38E00006  li r7, 6
	ctx.r[7].s64 = 6;
	// 826C8464: 388A456C  addi r4, r10, 0x456c
	ctx.r[4].s64 = ctx.r[10].s64 + 17772;
	// 826C8468: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C846C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C8470: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826C8474: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C8478: 386A6AF4  addi r3, r10, 0x6af4
	ctx.r[3].s64 = ctx.r[10].s64 + 27380;
	// 826C847C: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826C8480: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826C8484: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C8488: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C848C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C8490: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C8494: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826C8498: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C849C: 4BD9E985  bl 0x82466e20
	ctx.lr = 0x826C84A0;
	sub_82466E20(ctx, base);
	// 826C84A0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C84A4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C84A8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C84AC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C84B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C84B0 size=112
    let mut pc: u32 = 0x826C84B0;
    'dispatch: loop {
        match pc {
            0x826C84B0 => {
    //   block [0x826C84B0..0x826C8520)
	// 826C84B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C84B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C84B8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C84BC: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C84C0: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826C84C4: 38AA69A4  addi r5, r10, 0x69a4
	ctx.r[5].s64 = ctx.r[10].s64 + 27044;
	// 826C84C8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C84CC: 390B74E8  addi r8, r11, 0x74e8
	ctx.r[8].s64 = ctx.r[11].s64 + 29928;
	// 826C84D0: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826C84D4: 388A4590  addi r4, r10, 0x4590
	ctx.r[4].s64 = ctx.r[10].s64 + 17808;
	// 826C84D8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C84DC: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C84E0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826C84E4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C84E8: 386A6B24  addi r3, r10, 0x6b24
	ctx.r[3].s64 = ctx.r[10].s64 + 27428;
	// 826C84EC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826C84F0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C84F4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C84F8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C84FC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C8500: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C8504: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C8508: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C850C: 4BD9E915  bl 0x82466e20
	ctx.lr = 0x826C8510;
	sub_82466E20(ctx, base);
	// 826C8510: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C8514: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C8518: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C851C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C8520(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C8520 size=112
    let mut pc: u32 = 0x826C8520;
    'dispatch: loop {
        match pc {
            0x826C8520 => {
    //   block [0x826C8520..0x826C8590)
	// 826C8520: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C8524: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C8528: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C852C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C8530: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826C8534: 38AA6B84  addi r5, r10, 0x6b84
	ctx.r[5].s64 = ctx.r[10].s64 + 27524;
	// 826C8538: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C853C: 390B7500  addi r8, r11, 0x7500
	ctx.r[8].s64 = ctx.r[11].s64 + 29952;
	// 826C8540: 39200005  li r9, 5
	ctx.r[9].s64 = 5;
	// 826C8544: 388A45B0  addi r4, r10, 0x45b0
	ctx.r[4].s64 = ctx.r[10].s64 + 17840;
	// 826C8548: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C854C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C8550: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826C8554: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C8558: 386A6B54  addi r3, r10, 0x6b54
	ctx.r[3].s64 = ctx.r[10].s64 + 27476;
	// 826C855C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826C8560: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C8564: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C8568: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C856C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C8570: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C8574: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C8578: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C857C: 4BD9E8A5  bl 0x82466e20
	ctx.lr = 0x826C8580;
	sub_82466E20(ctx, base);
	// 826C8580: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C8584: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C8588: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C858C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C8590(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C8590 size=100
    let mut pc: u32 = 0x826C8590;
    'dispatch: loop {
        match pc {
            0x826C8590 => {
    //   block [0x826C8590..0x826C85F4)
	// 826C8590: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C8594: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C8598: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C859C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C85A0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C85A4: 38AA4EA4  addi r5, r10, 0x4ea4
	ctx.r[5].s64 = ctx.r[10].s64 + 20132;
	// 826C85A8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C85AC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C85B0: 388A45C8  addi r4, r10, 0x45c8
	ctx.r[4].s64 = ctx.r[10].s64 + 17864;
	// 826C85B4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C85B8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C85BC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C85C0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C85C4: 386A6B84  addi r3, r10, 0x6b84
	ctx.r[3].s64 = ctx.r[10].s64 + 27524;
	// 826C85C8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C85CC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C85D0: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826C85D4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C85D8: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826C85DC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C85E0: 4BD9E841  bl 0x82466e20
	ctx.lr = 0x826C85E4;
	sub_82466E20(ctx, base);
	// 826C85E4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C85E8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C85EC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C85F0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C85F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826C85F8 size=24
    let mut pc: u32 = 0x826C85F8;
    'dispatch: loop {
        match pc {
            0x826C85F8 => {
    //   block [0x826C85F8..0x826C8610)
	// 826C85F8: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826C85FC: 3D40827F  lis r10, -0x7d81
	ctx.r[10].s64 = -2105606144;
	// 826C8600: 394AA5F8  addi r10, r10, -0x5a08
	ctx.r[10].s64 = ctx.r[10].s64 + -23048;
	// 826C8604: 816B7454  lwz r11, 0x7454(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(29780 as u32) ) } as u64;
	// 826C8608: 916A0050  stw r11, 0x50(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 826C860C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C8610(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C8610 size=116
    let mut pc: u32 = 0x826C8610;
    'dispatch: loop {
        match pc {
            0x826C8610 => {
    //   block [0x826C8610..0x826C8684)
	// 826C8610: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C8614: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C8618: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C861C: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826C8620: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 826C8624: 390BA5F8  addi r8, r11, -0x5a08
	ctx.r[8].s64 = ctx.r[11].s64 + -23048;
	// 826C8628: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C862C: 392A27D0  addi r9, r10, 0x27d0
	ctx.r[9].s64 = ctx.r[10].s64 + 10192;
	// 826C8630: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C8634: 38E00005  li r7, 5
	ctx.r[7].s64 = 5;
	// 826C8638: 38AA69A4  addi r5, r10, 0x69a4
	ctx.r[5].s64 = ctx.r[10].s64 + 27044;
	// 826C863C: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826C8640: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C8644: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C8648: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C864C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C8650: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C8654: 3D608298  lis r11, -0x7d68
	ctx.r[11].s64 = -2103967744;
	// 826C8658: 388A45EC  addi r4, r10, 0x45ec
	ctx.r[4].s64 = ctx.r[10].s64 + 17900;
	// 826C865C: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826C8660: 386B6BB4  addi r3, r11, 0x6bb4
	ctx.r[3].s64 = ctx.r[11].s64 + 27572;
	// 826C8664: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826C8668: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C866C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C8670: 4BD9E7B1  bl 0x82466e20
	ctx.lr = 0x826C8674;
	sub_82466E20(ctx, base);
	// 826C8674: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C8678: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C867C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C8680: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C8688(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C8688 size=108
    let mut pc: u32 = 0x826C8688;
    'dispatch: loop {
        match pc {
            0x826C8688 => {
    //   block [0x826C8688..0x826C86F4)
	// 826C8688: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C868C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C8690: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C8694: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826C8698: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C869C: 38EB7578  addi r7, r11, 0x7578
	ctx.r[7].s64 = ctx.r[11].s64 + 30072;
	// 826C86A0: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826C86A4: 388A4608  addi r4, r10, 0x4608
	ctx.r[4].s64 = ctx.r[10].s64 + 17928;
	// 826C86A8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C86AC: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C86B0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826C86B4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C86B8: 386A6BE4  addi r3, r10, 0x6be4
	ctx.r[3].s64 = ctx.r[10].s64 + 27620;
	// 826C86BC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826C86C0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C86C4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C86C8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C86CC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C86D0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C86D4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C86D8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C86DC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826C86E0: 4BD9E741  bl 0x82466e20
	ctx.lr = 0x826C86E4;
	sub_82466E20(ctx, base);
	// 826C86E4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C86E8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C86EC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C86F0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C86F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C86F8 size=112
    let mut pc: u32 = 0x826C86F8;
    'dispatch: loop {
        match pc {
            0x826C86F8 => {
    //   block [0x826C86F8..0x826C8768)
	// 826C86F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C86FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C8700: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C8704: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C8708: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826C870C: 38AA69A4  addi r5, r10, 0x69a4
	ctx.r[5].s64 = ctx.r[10].s64 + 27044;
	// 826C8710: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C8714: 390B75A8  addi r8, r11, 0x75a8
	ctx.r[8].s64 = ctx.r[11].s64 + 30120;
	// 826C8718: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826C871C: 388A462C  addi r4, r10, 0x462c
	ctx.r[4].s64 = ctx.r[10].s64 + 17964;
	// 826C8720: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C8724: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C8728: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826C872C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C8730: 386A6C14  addi r3, r10, 0x6c14
	ctx.r[3].s64 = ctx.r[10].s64 + 27668;
	// 826C8734: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826C8738: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C873C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C8740: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C8744: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C8748: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C874C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C8750: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C8754: 4BD9E6CD  bl 0x82466e20
	ctx.lr = 0x826C8758;
	sub_82466E20(ctx, base);
	// 826C8758: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C875C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C8760: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C8764: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C8768(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C8768 size=112
    let mut pc: u32 = 0x826C8768;
    'dispatch: loop {
        match pc {
            0x826C8768 => {
    //   block [0x826C8768..0x826C87D8)
	// 826C8768: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C876C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C8770: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C8774: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 826C8778: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826C877C: 392A27F4  addi r9, r10, 0x27f4
	ctx.r[9].s64 = ctx.r[10].s64 + 10228;
	// 826C8780: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C8784: 390B75C8  addi r8, r11, 0x75c8
	ctx.r[8].s64 = ctx.r[11].s64 + 30152;
	// 826C8788: 38E00007  li r7, 7
	ctx.r[7].s64 = 7;
	// 826C878C: 388A464C  addi r4, r10, 0x464c
	ctx.r[4].s64 = ctx.r[10].s64 + 17996;
	// 826C8790: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C8794: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C8798: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826C879C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C87A0: 386A6C44  addi r3, r10, 0x6c44
	ctx.r[3].s64 = ctx.r[10].s64 + 27716;
	// 826C87A4: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826C87A8: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826C87AC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C87B0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C87B4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C87B8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C87BC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826C87C0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C87C4: 4BD9E65D  bl 0x82466e20
	ctx.lr = 0x826C87C8;
	sub_82466E20(ctx, base);
	// 826C87C8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C87CC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C87D0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C87D4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C87D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C87D8 size=112
    let mut pc: u32 = 0x826C87D8;
    'dispatch: loop {
        match pc {
            0x826C87D8 => {
    //   block [0x826C87D8..0x826C8848)
	// 826C87D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C87DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C87E0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C87E4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C87E8: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826C87EC: 38AA69A4  addi r5, r10, 0x69a4
	ctx.r[5].s64 = ctx.r[10].s64 + 27044;
	// 826C87F0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C87F4: 390B7670  addi r8, r11, 0x7670
	ctx.r[8].s64 = ctx.r[11].s64 + 30320;
	// 826C87F8: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826C87FC: 388A466C  addi r4, r10, 0x466c
	ctx.r[4].s64 = ctx.r[10].s64 + 18028;
	// 826C8800: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C8804: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C8808: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826C880C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C8810: 386A6C74  addi r3, r10, 0x6c74
	ctx.r[3].s64 = ctx.r[10].s64 + 27764;
	// 826C8814: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826C8818: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C881C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C8820: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C8824: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C8828: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C882C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C8830: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C8834: 4BD9E5ED  bl 0x82466e20
	ctx.lr = 0x826C8838;
	sub_82466E20(ctx, base);
	// 826C8838: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C883C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C8840: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C8844: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C8848(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C8848 size=108
    let mut pc: u32 = 0x826C8848;
    'dispatch: loop {
        match pc {
            0x826C8848 => {
    //   block [0x826C8848..0x826C88B4)
	// 826C8848: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C884C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C8850: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C8854: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826C8858: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C885C: 38EB7688  addi r7, r11, 0x7688
	ctx.r[7].s64 = ctx.r[11].s64 + 30344;
	// 826C8860: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826C8864: 388A4688  addi r4, r10, 0x4688
	ctx.r[4].s64 = ctx.r[10].s64 + 18056;
	// 826C8868: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C886C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C8870: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826C8874: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C8878: 386A6CA4  addi r3, r10, 0x6ca4
	ctx.r[3].s64 = ctx.r[10].s64 + 27812;
	// 826C887C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826C8880: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C8884: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C8888: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C888C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C8890: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C8894: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C8898: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C889C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826C88A0: 4BD9E581  bl 0x82466e20
	ctx.lr = 0x826C88A4;
	sub_82466E20(ctx, base);
	// 826C88A4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C88A8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C88AC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C88B0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C88B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C88B8 size=112
    let mut pc: u32 = 0x826C88B8;
    'dispatch: loop {
        match pc {
            0x826C88B8 => {
    //   block [0x826C88B8..0x826C8928)
	// 826C88B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C88BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C88C0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C88C4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C88C8: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826C88CC: 38AA69A4  addi r5, r10, 0x69a4
	ctx.r[5].s64 = ctx.r[10].s64 + 27044;
	// 826C88D0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C88D4: 390B76B8  addi r8, r11, 0x76b8
	ctx.r[8].s64 = ctx.r[11].s64 + 30392;
	// 826C88D8: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826C88DC: 388A46AC  addi r4, r10, 0x46ac
	ctx.r[4].s64 = ctx.r[10].s64 + 18092;
	// 826C88E0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C88E4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C88E8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826C88EC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C88F0: 386A6CD4  addi r3, r10, 0x6cd4
	ctx.r[3].s64 = ctx.r[10].s64 + 27860;
	// 826C88F4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826C88F8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C88FC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C8900: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C8904: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C8908: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C890C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C8910: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C8914: 4BD9E50D  bl 0x82466e20
	ctx.lr = 0x826C8918;
	sub_82466E20(ctx, base);
	// 826C8918: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C891C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C8920: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C8924: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C8928(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C8928 size=112
    let mut pc: u32 = 0x826C8928;
    'dispatch: loop {
        match pc {
            0x826C8928 => {
    //   block [0x826C8928..0x826C8998)
	// 826C8928: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C892C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C8930: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C8934: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 826C8938: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826C893C: 392A2828  addi r9, r10, 0x2828
	ctx.r[9].s64 = ctx.r[10].s64 + 10280;
	// 826C8940: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C8944: 390B76D0  addi r8, r11, 0x76d0
	ctx.r[8].s64 = ctx.r[11].s64 + 30416;
	// 826C8948: 38E00007  li r7, 7
	ctx.r[7].s64 = 7;
	// 826C894C: 388A46C8  addi r4, r10, 0x46c8
	ctx.r[4].s64 = ctx.r[10].s64 + 18120;
	// 826C8950: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C8954: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C8958: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826C895C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C8960: 386A6D04  addi r3, r10, 0x6d04
	ctx.r[3].s64 = ctx.r[10].s64 + 27908;
	// 826C8964: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826C8968: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826C896C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C8970: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C8974: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C8978: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C897C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826C8980: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C8984: 4BD9E49D  bl 0x82466e20
	ctx.lr = 0x826C8988;
	sub_82466E20(ctx, base);
	// 826C8988: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C898C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C8990: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C8994: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C8998(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C8998 size=112
    let mut pc: u32 = 0x826C8998;
    'dispatch: loop {
        match pc {
            0x826C8998 => {
    //   block [0x826C8998..0x826C8A08)
	// 826C8998: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C899C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C89A0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C89A4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C89A8: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826C89AC: 38AA69A4  addi r5, r10, 0x69a4
	ctx.r[5].s64 = ctx.r[10].s64 + 27044;
	// 826C89B0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C89B4: 390B7778  addi r8, r11, 0x7778
	ctx.r[8].s64 = ctx.r[11].s64 + 30584;
	// 826C89B8: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 826C89BC: 388A46E4  addi r4, r10, 0x46e4
	ctx.r[4].s64 = ctx.r[10].s64 + 18148;
	// 826C89C0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C89C4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C89C8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826C89CC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C89D0: 386A6D34  addi r3, r10, 0x6d34
	ctx.r[3].s64 = ctx.r[10].s64 + 27956;
	// 826C89D4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826C89D8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C89DC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C89E0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C89E4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C89E8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C89EC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C89F0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C89F4: 4BD9E42D  bl 0x82466e20
	ctx.lr = 0x826C89F8;
	sub_82466E20(ctx, base);
	// 826C89F8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C89FC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C8A00: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C8A04: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C8A08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C8A08 size=112
    let mut pc: u32 = 0x826C8A08;
    'dispatch: loop {
        match pc {
            0x826C8A08 => {
    //   block [0x826C8A08..0x826C8A78)
	// 826C8A08: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C8A0C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C8A10: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C8A14: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C8A18: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826C8A1C: 38AA69A4  addi r5, r10, 0x69a4
	ctx.r[5].s64 = ctx.r[10].s64 + 27044;
	// 826C8A20: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C8A24: 390B77C0  addi r8, r11, 0x77c0
	ctx.r[8].s64 = ctx.r[11].s64 + 30656;
	// 826C8A28: 39200009  li r9, 9
	ctx.r[9].s64 = 9;
	// 826C8A2C: 388A46FC  addi r4, r10, 0x46fc
	ctx.r[4].s64 = ctx.r[10].s64 + 18172;
	// 826C8A30: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C8A34: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C8A38: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826C8A3C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C8A40: 386A6D64  addi r3, r10, 0x6d64
	ctx.r[3].s64 = ctx.r[10].s64 + 28004;
	// 826C8A44: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826C8A48: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C8A4C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C8A50: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C8A54: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C8A58: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C8A5C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C8A60: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C8A64: 4BD9E3BD  bl 0x82466e20
	ctx.lr = 0x826C8A68;
	sub_82466E20(ctx, base);
	// 826C8A68: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C8A6C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C8A70: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C8A74: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C8A78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C8A78 size=100
    let mut pc: u32 = 0x826C8A78;
    'dispatch: loop {
        match pc {
            0x826C8A78 => {
    //   block [0x826C8A78..0x826C8ADC)
	// 826C8A78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C8A7C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C8A80: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C8A84: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C8A88: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C8A8C: 38AA69A4  addi r5, r10, 0x69a4
	ctx.r[5].s64 = ctx.r[10].s64 + 27044;
	// 826C8A90: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C8A94: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C8A98: 388A4718  addi r4, r10, 0x4718
	ctx.r[4].s64 = ctx.r[10].s64 + 18200;
	// 826C8A9C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C8AA0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C8AA4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C8AA8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C8AAC: 386A6D94  addi r3, r10, 0x6d94
	ctx.r[3].s64 = ctx.r[10].s64 + 28052;
	// 826C8AB0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C8AB4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C8AB8: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826C8ABC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C8AC0: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826C8AC4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C8AC8: 4BD9E359  bl 0x82466e20
	ctx.lr = 0x826C8ACC;
	sub_82466E20(ctx, base);
	// 826C8ACC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C8AD0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C8AD4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C8AD8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C8AE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C8AE0 size=112
    let mut pc: u32 = 0x826C8AE0;
    'dispatch: loop {
        match pc {
            0x826C8AE0 => {
    //   block [0x826C8AE0..0x826C8B50)
	// 826C8AE0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C8AE4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C8AE8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C8AEC: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C8AF0: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826C8AF4: 38AA6A04  addi r5, r10, 0x6a04
	ctx.r[5].s64 = ctx.r[10].s64 + 27140;
	// 826C8AF8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C8AFC: 390B7898  addi r8, r11, 0x7898
	ctx.r[8].s64 = ctx.r[11].s64 + 30872;
	// 826C8B00: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826C8B04: 388A4730  addi r4, r10, 0x4730
	ctx.r[4].s64 = ctx.r[10].s64 + 18224;
	// 826C8B08: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C8B0C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C8B10: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826C8B14: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C8B18: 386A6DC4  addi r3, r10, 0x6dc4
	ctx.r[3].s64 = ctx.r[10].s64 + 28100;
	// 826C8B1C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826C8B20: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C8B24: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C8B28: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C8B2C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C8B30: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C8B34: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C8B38: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C8B3C: 4BD9E2E5  bl 0x82466e20
	ctx.lr = 0x826C8B40;
	sub_82466E20(ctx, base);
	// 826C8B40: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C8B44: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C8B48: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C8B4C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C8B50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C8B50 size=112
    let mut pc: u32 = 0x826C8B50;
    'dispatch: loop {
        match pc {
            0x826C8B50 => {
    //   block [0x826C8B50..0x826C8BC0)
	// 826C8B50: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C8B54: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C8B58: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C8B5C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C8B60: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826C8B64: 38AA6854  addi r5, r10, 0x6854
	ctx.r[5].s64 = ctx.r[10].s64 + 26708;
	// 826C8B68: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C8B6C: 390B78C8  addi r8, r11, 0x78c8
	ctx.r[8].s64 = ctx.r[11].s64 + 30920;
	// 826C8B70: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826C8B74: 388A474C  addi r4, r10, 0x474c
	ctx.r[4].s64 = ctx.r[10].s64 + 18252;
	// 826C8B78: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C8B7C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C8B80: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826C8B84: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C8B88: 386A6DF4  addi r3, r10, 0x6df4
	ctx.r[3].s64 = ctx.r[10].s64 + 28148;
	// 826C8B8C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826C8B90: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C8B94: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C8B98: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C8B9C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C8BA0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C8BA4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C8BA8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C8BAC: 4BD9E275  bl 0x82466e20
	ctx.lr = 0x826C8BB0;
	sub_82466E20(ctx, base);
	// 826C8BB0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C8BB4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C8BB8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C8BBC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C8BC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C8BC0 size=108
    let mut pc: u32 = 0x826C8BC0;
    'dispatch: loop {
        match pc {
            0x826C8BC0 => {
    //   block [0x826C8BC0..0x826C8C2C)
	// 826C8BC0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C8BC4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C8BC8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C8BCC: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826C8BD0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C8BD4: 38EB78E0  addi r7, r11, 0x78e0
	ctx.r[7].s64 = ctx.r[11].s64 + 30944;
	// 826C8BD8: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826C8BDC: 388A476C  addi r4, r10, 0x476c
	ctx.r[4].s64 = ctx.r[10].s64 + 18284;
	// 826C8BE0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C8BE4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C8BE8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826C8BEC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C8BF0: 386A6E24  addi r3, r10, 0x6e24
	ctx.r[3].s64 = ctx.r[10].s64 + 28196;
	// 826C8BF4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826C8BF8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C8BFC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C8C00: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C8C04: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C8C08: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C8C0C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C8C10: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C8C14: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826C8C18: 4BD9E209  bl 0x82466e20
	ctx.lr = 0x826C8C1C;
	sub_82466E20(ctx, base);
	// 826C8C1C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C8C20: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C8C24: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C8C28: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C8C30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C8C30 size=112
    let mut pc: u32 = 0x826C8C30;
    'dispatch: loop {
        match pc {
            0x826C8C30 => {
    //   block [0x826C8C30..0x826C8CA0)
	// 826C8C30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C8C34: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C8C38: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C8C3C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C8C40: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826C8C44: 38AA6D94  addi r5, r10, 0x6d94
	ctx.r[5].s64 = ctx.r[10].s64 + 28052;
	// 826C8C48: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C8C4C: 390B7910  addi r8, r11, 0x7910
	ctx.r[8].s64 = ctx.r[11].s64 + 30992;
	// 826C8C50: 39200006  li r9, 6
	ctx.r[9].s64 = 6;
	// 826C8C54: 388A4790  addi r4, r10, 0x4790
	ctx.r[4].s64 = ctx.r[10].s64 + 18320;
	// 826C8C58: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C8C5C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C8C60: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826C8C64: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C8C68: 386A6E54  addi r3, r10, 0x6e54
	ctx.r[3].s64 = ctx.r[10].s64 + 28244;
	// 826C8C6C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826C8C70: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C8C74: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C8C78: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C8C7C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C8C80: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C8C84: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C8C88: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C8C8C: 4BD9E195  bl 0x82466e20
	ctx.lr = 0x826C8C90;
	sub_82466E20(ctx, base);
	// 826C8C90: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C8C94: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C8C98: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C8C9C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C8CA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C8CA0 size=112
    let mut pc: u32 = 0x826C8CA0;
    'dispatch: loop {
        match pc {
            0x826C8CA0 => {
    //   block [0x826C8CA0..0x826C8D10)
	// 826C8CA0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C8CA4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C8CA8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C8CAC: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 826C8CB0: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826C8CB4: 392A2854  addi r9, r10, 0x2854
	ctx.r[9].s64 = ctx.r[10].s64 + 10324;
	// 826C8CB8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C8CBC: 390B79A8  addi r8, r11, 0x79a8
	ctx.r[8].s64 = ctx.r[11].s64 + 31144;
	// 826C8CC0: 38E00003  li r7, 3
	ctx.r[7].s64 = 3;
	// 826C8CC4: 388A47A8  addi r4, r10, 0x47a8
	ctx.r[4].s64 = ctx.r[10].s64 + 18344;
	// 826C8CC8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C8CCC: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C8CD0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826C8CD4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C8CD8: 386A6E84  addi r3, r10, 0x6e84
	ctx.r[3].s64 = ctx.r[10].s64 + 28292;
	// 826C8CDC: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826C8CE0: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826C8CE4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C8CE8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C8CEC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C8CF0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C8CF4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826C8CF8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C8CFC: 4BD9E125  bl 0x82466e20
	ctx.lr = 0x826C8D00;
	sub_82466E20(ctx, base);
	// 826C8D00: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C8D04: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C8D08: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C8D0C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C8D10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C8D10 size=112
    let mut pc: u32 = 0x826C8D10;
    'dispatch: loop {
        match pc {
            0x826C8D10 => {
    //   block [0x826C8D10..0x826C8D80)
	// 826C8D10: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C8D14: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C8D18: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C8D1C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C8D20: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826C8D24: 38AA69A4  addi r5, r10, 0x69a4
	ctx.r[5].s64 = ctx.r[10].s64 + 27044;
	// 826C8D28: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C8D2C: 390B79F0  addi r8, r11, 0x79f0
	ctx.r[8].s64 = ctx.r[11].s64 + 31216;
	// 826C8D30: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826C8D34: 388A47C0  addi r4, r10, 0x47c0
	ctx.r[4].s64 = ctx.r[10].s64 + 18368;
	// 826C8D38: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C8D3C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C8D40: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826C8D44: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C8D48: 386A6EB4  addi r3, r10, 0x6eb4
	ctx.r[3].s64 = ctx.r[10].s64 + 28340;
	// 826C8D4C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826C8D50: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C8D54: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C8D58: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C8D5C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C8D60: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C8D64: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C8D68: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C8D6C: 4BD9E0B5  bl 0x82466e20
	ctx.lr = 0x826C8D70;
	sub_82466E20(ctx, base);
	// 826C8D70: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C8D74: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C8D78: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C8D7C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C8D80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C8D80 size=108
    let mut pc: u32 = 0x826C8D80;
    'dispatch: loop {
        match pc {
            0x826C8D80 => {
    //   block [0x826C8D80..0x826C8DEC)
	// 826C8D80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C8D84: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C8D88: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C8D8C: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826C8D90: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C8D94: 38EB7A08  addi r7, r11, 0x7a08
	ctx.r[7].s64 = ctx.r[11].s64 + 31240;
	// 826C8D98: 39000006  li r8, 6
	ctx.r[8].s64 = 6;
	// 826C8D9C: 388A47D4  addi r4, r10, 0x47d4
	ctx.r[4].s64 = ctx.r[10].s64 + 18388;
	// 826C8DA0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C8DA4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C8DA8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826C8DAC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C8DB0: 386A6EE4  addi r3, r10, 0x6ee4
	ctx.r[3].s64 = ctx.r[10].s64 + 28388;
	// 826C8DB4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826C8DB8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C8DBC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C8DC0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C8DC4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C8DC8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C8DCC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C8DD0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C8DD4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826C8DD8: 4BD9E049  bl 0x82466e20
	ctx.lr = 0x826C8DDC;
	sub_82466E20(ctx, base);
	// 826C8DDC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C8DE0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C8DE4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C8DE8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C8DF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C8DF0 size=116
    let mut pc: u32 = 0x826C8DF0;
    'dispatch: loop {
        match pc {
            0x826C8DF0 => {
    //   block [0x826C8DF0..0x826C8E64)
	// 826C8DF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C8DF4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C8DF8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C8DFC: 3D40827E  lis r10, -0x7d82
	ctx.r[10].s64 = -2105671680;
	// 826C8E00: 38E00009  li r7, 9
	ctx.r[7].s64 = 9;
	// 826C8E04: 390A7A98  addi r8, r10, 0x7a98
	ctx.r[8].s64 = ctx.r[10].s64 + 31384;
	// 826C8E08: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C8E0C: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 826C8E10: 38AA6D94  addi r5, r10, 0x6d94
	ctx.r[5].s64 = ctx.r[10].s64 + 28052;
	// 826C8E14: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C8E18: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826C8E1C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C8E20: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826C8E24: 388A47F8  addi r4, r10, 0x47f8
	ctx.r[4].s64 = ctx.r[10].s64 + 18424;
	// 826C8E28: 396B2868  addi r11, r11, 0x2868
	ctx.r[11].s64 = ctx.r[11].s64 + 10344;
	// 826C8E2C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C8E30: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C8E34: 386A6F14  addi r3, r10, 0x6f14
	ctx.r[3].s64 = ctx.r[10].s64 + 28436;
	// 826C8E38: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 826C8E3C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C8E40: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 826C8E44: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C8E48: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C8E4C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C8E50: 4BD9DFD1  bl 0x82466e20
	ctx.lr = 0x826C8E54;
	sub_82466E20(ctx, base);
	// 826C8E54: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C8E58: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C8E5C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C8E60: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C8E68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C8E68 size=108
    let mut pc: u32 = 0x826C8E68;
    'dispatch: loop {
        match pc {
            0x826C8E68 => {
    //   block [0x826C8E68..0x826C8ED4)
	// 826C8E68: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C8E6C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C8E70: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C8E74: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826C8E78: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C8E7C: 38EB7B70  addi r7, r11, 0x7b70
	ctx.r[7].s64 = ctx.r[11].s64 + 31600;
	// 826C8E80: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 826C8E84: 388A480C  addi r4, r10, 0x480c
	ctx.r[4].s64 = ctx.r[10].s64 + 18444;
	// 826C8E88: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C8E8C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C8E90: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826C8E94: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C8E98: 386A6F44  addi r3, r10, 0x6f44
	ctx.r[3].s64 = ctx.r[10].s64 + 28484;
	// 826C8E9C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826C8EA0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C8EA4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C8EA8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C8EAC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C8EB0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C8EB4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C8EB8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C8EBC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826C8EC0: 4BD9DF61  bl 0x82466e20
	ctx.lr = 0x826C8EC4;
	sub_82466E20(ctx, base);
	// 826C8EC4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C8EC8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C8ECC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C8ED0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C8ED8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C8ED8 size=112
    let mut pc: u32 = 0x826C8ED8;
    'dispatch: loop {
        match pc {
            0x826C8ED8 => {
    //   block [0x826C8ED8..0x826C8F48)
	// 826C8ED8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C8EDC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C8EE0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C8EE4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C8EE8: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826C8EEC: 38AA6D94  addi r5, r10, 0x6d94
	ctx.r[5].s64 = ctx.r[10].s64 + 28052;
	// 826C8EF0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C8EF4: 390B7BB8  addi r8, r11, 0x7bb8
	ctx.r[8].s64 = ctx.r[11].s64 + 31672;
	// 826C8EF8: 39200005  li r9, 5
	ctx.r[9].s64 = 5;
	// 826C8EFC: 388A4834  addi r4, r10, 0x4834
	ctx.r[4].s64 = ctx.r[10].s64 + 18484;
	// 826C8F00: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C8F04: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C8F08: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826C8F0C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C8F10: 386A6F74  addi r3, r10, 0x6f74
	ctx.r[3].s64 = ctx.r[10].s64 + 28532;
	// 826C8F14: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826C8F18: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C8F1C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C8F20: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C8F24: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C8F28: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C8F2C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C8F30: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C8F34: 4BD9DEED  bl 0x82466e20
	ctx.lr = 0x826C8F38;
	sub_82466E20(ctx, base);
	// 826C8F38: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C8F3C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C8F40: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C8F44: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C8F48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C8F48 size=112
    let mut pc: u32 = 0x826C8F48;
    'dispatch: loop {
        match pc {
            0x826C8F48 => {
    //   block [0x826C8F48..0x826C8FB8)
	// 826C8F48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C8F4C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C8F50: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C8F54: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C8F58: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826C8F5C: 38AA69A4  addi r5, r10, 0x69a4
	ctx.r[5].s64 = ctx.r[10].s64 + 27044;
	// 826C8F60: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C8F64: 390B7C30  addi r8, r11, 0x7c30
	ctx.r[8].s64 = ctx.r[11].s64 + 31792;
	// 826C8F68: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826C8F6C: 388A484C  addi r4, r10, 0x484c
	ctx.r[4].s64 = ctx.r[10].s64 + 18508;
	// 826C8F70: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C8F74: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C8F78: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826C8F7C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C8F80: 386A6FA4  addi r3, r10, 0x6fa4
	ctx.r[3].s64 = ctx.r[10].s64 + 28580;
	// 826C8F84: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826C8F88: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C8F8C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C8F90: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C8F94: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C8F98: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C8F9C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C8FA0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C8FA4: 4BD9DE7D  bl 0x82466e20
	ctx.lr = 0x826C8FA8;
	sub_82466E20(ctx, base);
	// 826C8FA8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C8FAC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C8FB0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C8FB4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C8FB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C8FB8 size=108
    let mut pc: u32 = 0x826C8FB8;
    'dispatch: loop {
        match pc {
            0x826C8FB8 => {
    //   block [0x826C8FB8..0x826C9024)
	// 826C8FB8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C8FBC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C8FC0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C8FC4: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826C8FC8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C8FCC: 38EB7C60  addi r7, r11, 0x7c60
	ctx.r[7].s64 = ctx.r[11].s64 + 31840;
	// 826C8FD0: 39000005  li r8, 5
	ctx.r[8].s64 = 5;
	// 826C8FD4: 388A4890  addi r4, r10, 0x4890
	ctx.r[4].s64 = ctx.r[10].s64 + 18576;
	// 826C8FD8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C8FDC: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C8FE0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826C8FE4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C8FE8: 386A6FD4  addi r3, r10, 0x6fd4
	ctx.r[3].s64 = ctx.r[10].s64 + 28628;
	// 826C8FEC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826C8FF0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C8FF4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C8FF8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C8FFC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C9000: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C9004: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C9008: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C900C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826C9010: 4BD9DE11  bl 0x82466e20
	ctx.lr = 0x826C9014;
	sub_82466E20(ctx, base);
	// 826C9014: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C9018: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C901C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C9020: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C9028(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C9028 size=112
    let mut pc: u32 = 0x826C9028;
    'dispatch: loop {
        match pc {
            0x826C9028 => {
    //   block [0x826C9028..0x826C9098)
	// 826C9028: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C902C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C9030: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C9034: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C9038: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826C903C: 38AA69A4  addi r5, r10, 0x69a4
	ctx.r[5].s64 = ctx.r[10].s64 + 27044;
	// 826C9040: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C9044: 390B7CD8  addi r8, r11, 0x7cd8
	ctx.r[8].s64 = ctx.r[11].s64 + 31960;
	// 826C9048: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 826C904C: 388A48B0  addi r4, r10, 0x48b0
	ctx.r[4].s64 = ctx.r[10].s64 + 18608;
	// 826C9050: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C9054: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C9058: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826C905C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C9060: 386A7004  addi r3, r10, 0x7004
	ctx.r[3].s64 = ctx.r[10].s64 + 28676;
	// 826C9064: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826C9068: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C906C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C9070: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C9074: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C9078: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C907C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C9080: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C9084: 4BD9DD9D  bl 0x82466e20
	ctx.lr = 0x826C9088;
	sub_82466E20(ctx, base);
	// 826C9088: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C908C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C9090: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C9094: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C9098(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826C9098 size=24
    let mut pc: u32 = 0x826C9098;
    'dispatch: loop {
        match pc {
            0x826C9098 => {
    //   block [0x826C9098..0x826C90B0)
	// 826C9098: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826C909C: 3D40827F  lis r10, -0x7d81
	ctx.r[10].s64 = -2105606144;
	// 826C90A0: 394AA670  addi r10, r10, -0x5990
	ctx.r[10].s64 = ctx.r[10].s64 + -22928;
	// 826C90A4: 816B79A4  lwz r11, 0x79a4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(31140 as u32) ) } as u64;
	// 826C90A8: 916A0008  stw r11, 8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 826C90AC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C90B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C90B0 size=116
    let mut pc: u32 = 0x826C90B0;
    'dispatch: loop {
        match pc {
            0x826C90B0 => {
    //   block [0x826C90B0..0x826C9124)
	// 826C90B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C90B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C90B8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C90BC: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826C90C0: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 826C90C4: 390BA670  addi r8, r11, -0x5990
	ctx.r[8].s64 = ctx.r[11].s64 + -22928;
	// 826C90C8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C90CC: 392A28C4  addi r9, r10, 0x28c4
	ctx.r[9].s64 = ctx.r[10].s64 + 10436;
	// 826C90D0: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C90D4: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
	// 826C90D8: 38AA4EA4  addi r5, r10, 0x4ea4
	ctx.r[5].s64 = ctx.r[10].s64 + 20132;
	// 826C90DC: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826C90E0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C90E4: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C90E8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C90EC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C90F0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C90F4: 3D608298  lis r11, -0x7d68
	ctx.r[11].s64 = -2103967744;
	// 826C90F8: 388A48CC  addi r4, r10, 0x48cc
	ctx.r[4].s64 = ctx.r[10].s64 + 18636;
	// 826C90FC: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826C9100: 386B7034  addi r3, r11, 0x7034
	ctx.r[3].s64 = ctx.r[11].s64 + 28724;
	// 826C9104: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826C9108: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C910C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C9110: 4BD9DD11  bl 0x82466e20
	ctx.lr = 0x826C9114;
	sub_82466E20(ctx, base);
	// 826C9114: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C9118: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C911C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C9120: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C9128(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C9128 size=112
    let mut pc: u32 = 0x826C9128;
    'dispatch: loop {
        match pc {
            0x826C9128 => {
    //   block [0x826C9128..0x826C9198)
	// 826C9128: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C912C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C9130: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C9134: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C9138: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826C913C: 38AA7034  addi r5, r10, 0x7034
	ctx.r[5].s64 = ctx.r[10].s64 + 28724;
	// 826C9140: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C9144: 390B7D20  addi r8, r11, 0x7d20
	ctx.r[8].s64 = ctx.r[11].s64 + 32032;
	// 826C9148: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826C914C: 388A48E0  addi r4, r10, 0x48e0
	ctx.r[4].s64 = ctx.r[10].s64 + 18656;
	// 826C9150: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C9154: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C9158: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826C915C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C9160: 386A7064  addi r3, r10, 0x7064
	ctx.r[3].s64 = ctx.r[10].s64 + 28772;
	// 826C9164: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826C9168: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C916C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C9170: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C9174: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C9178: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C917C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C9180: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C9184: 4BD9DC9D  bl 0x82466e20
	ctx.lr = 0x826C9188;
	sub_82466E20(ctx, base);
	// 826C9188: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C918C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C9190: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C9194: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C9198(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C9198 size=112
    let mut pc: u32 = 0x826C9198;
    'dispatch: loop {
        match pc {
            0x826C9198 => {
    //   block [0x826C9198..0x826C9208)
	// 826C9198: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C919C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C91A0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C91A4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C91A8: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826C91AC: 38AA7064  addi r5, r10, 0x7064
	ctx.r[5].s64 = ctx.r[10].s64 + 28772;
	// 826C91B0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C91B4: 390B7D50  addi r8, r11, 0x7d50
	ctx.r[8].s64 = ctx.r[11].s64 + 32080;
	// 826C91B8: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 826C91BC: 388A4928  addi r4, r10, 0x4928
	ctx.r[4].s64 = ctx.r[10].s64 + 18728;
	// 826C91C0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C91C4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C91C8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826C91CC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C91D0: 386A7094  addi r3, r10, 0x7094
	ctx.r[3].s64 = ctx.r[10].s64 + 28820;
	// 826C91D4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826C91D8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C91DC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C91E0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C91E4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C91E8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C91EC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C91F0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C91F4: 4BD9DC2D  bl 0x82466e20
	ctx.lr = 0x826C91F8;
	sub_82466E20(ctx, base);
	// 826C91F8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C91FC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C9200: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C9204: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C9208(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C9208 size=112
    let mut pc: u32 = 0x826C9208;
    'dispatch: loop {
        match pc {
            0x826C9208 => {
    //   block [0x826C9208..0x826C9278)
	// 826C9208: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C920C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C9210: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C9214: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C9218: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826C921C: 38AA7064  addi r5, r10, 0x7064
	ctx.r[5].s64 = ctx.r[10].s64 + 28772;
	// 826C9220: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C9224: 390B7DB0  addi r8, r11, 0x7db0
	ctx.r[8].s64 = ctx.r[11].s64 + 32176;
	// 826C9228: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826C922C: 388A4944  addi r4, r10, 0x4944
	ctx.r[4].s64 = ctx.r[10].s64 + 18756;
	// 826C9230: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C9234: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C9238: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826C923C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C9240: 386A70C4  addi r3, r10, 0x70c4
	ctx.r[3].s64 = ctx.r[10].s64 + 28868;
	// 826C9244: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826C9248: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C924C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C9250: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C9254: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C9258: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C925C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C9260: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C9264: 4BD9DBBD  bl 0x82466e20
	ctx.lr = 0x826C9268;
	sub_82466E20(ctx, base);
	// 826C9268: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C926C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C9270: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C9274: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C9278(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C9278 size=112
    let mut pc: u32 = 0x826C9278;
    'dispatch: loop {
        match pc {
            0x826C9278 => {
    //   block [0x826C9278..0x826C92E8)
	// 826C9278: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C927C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C9280: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C9284: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C9288: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826C928C: 38AA7064  addi r5, r10, 0x7064
	ctx.r[5].s64 = ctx.r[10].s64 + 28772;
	// 826C9290: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C9294: 390B7DE0  addi r8, r11, 0x7de0
	ctx.r[8].s64 = ctx.r[11].s64 + 32224;
	// 826C9298: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 826C929C: 388A4964  addi r4, r10, 0x4964
	ctx.r[4].s64 = ctx.r[10].s64 + 18788;
	// 826C92A0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C92A4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C92A8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826C92AC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C92B0: 386A70F4  addi r3, r10, 0x70f4
	ctx.r[3].s64 = ctx.r[10].s64 + 28916;
	// 826C92B4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826C92B8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C92BC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C92C0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C92C4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C92C8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C92CC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C92D0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C92D4: 4BD9DB4D  bl 0x82466e20
	ctx.lr = 0x826C92D8;
	sub_82466E20(ctx, base);
	// 826C92D8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C92DC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C92E0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C92E4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C92E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C92E8 size=108
    let mut pc: u32 = 0x826C92E8;
    'dispatch: loop {
        match pc {
            0x826C92E8 => {
    //   block [0x826C92E8..0x826C9354)
	// 826C92E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C92EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C92F0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C92F4: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826C92F8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C92FC: 38EB7E28  addi r7, r11, 0x7e28
	ctx.r[7].s64 = ctx.r[11].s64 + 32296;
	// 826C9300: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826C9304: 388A4980  addi r4, r10, 0x4980
	ctx.r[4].s64 = ctx.r[10].s64 + 18816;
	// 826C9308: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C930C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C9310: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826C9314: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C9318: 386A7124  addi r3, r10, 0x7124
	ctx.r[3].s64 = ctx.r[10].s64 + 28964;
	// 826C931C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826C9320: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C9324: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C9328: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C932C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C9330: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C9334: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C9338: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C933C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826C9340: 4BD9DAE1  bl 0x82466e20
	ctx.lr = 0x826C9344;
	sub_82466E20(ctx, base);
	// 826C9344: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C9348: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C934C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C9350: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C9358(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C9358 size=112
    let mut pc: u32 = 0x826C9358;
    'dispatch: loop {
        match pc {
            0x826C9358 => {
    //   block [0x826C9358..0x826C93C8)
	// 826C9358: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C935C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C9360: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C9364: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C9368: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826C936C: 38AA69A4  addi r5, r10, 0x69a4
	ctx.r[5].s64 = ctx.r[10].s64 + 27044;
	// 826C9370: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C9374: 390B7E58  addi r8, r11, 0x7e58
	ctx.r[8].s64 = ctx.r[11].s64 + 32344;
	// 826C9378: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826C937C: 388A499C  addi r4, r10, 0x499c
	ctx.r[4].s64 = ctx.r[10].s64 + 18844;
	// 826C9380: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C9384: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C9388: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826C938C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C9390: 386A7154  addi r3, r10, 0x7154
	ctx.r[3].s64 = ctx.r[10].s64 + 29012;
	// 826C9394: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826C9398: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C939C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C93A0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C93A4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C93A8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C93AC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C93B0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C93B4: 4BD9DA6D  bl 0x82466e20
	ctx.lr = 0x826C93B8;
	sub_82466E20(ctx, base);
	// 826C93B8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C93BC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C93C0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C93C4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C93C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C93C8 size=116
    let mut pc: u32 = 0x826C93C8;
    'dispatch: loop {
        match pc {
            0x826C93C8 => {
    //   block [0x826C93C8..0x826C943C)
	// 826C93C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C93CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C93D0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C93D4: 3D40827E  lis r10, -0x7d82
	ctx.r[10].s64 = -2105671680;
	// 826C93D8: 38E00010  li r7, 0x10
	ctx.r[7].s64 = 16;
	// 826C93DC: 390A7E70  addi r8, r10, 0x7e70
	ctx.r[8].s64 = ctx.r[10].s64 + 32368;
	// 826C93E0: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C93E4: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 826C93E8: 38AA75D4  addi r5, r10, 0x75d4
	ctx.r[5].s64 = ctx.r[10].s64 + 30164;
	// 826C93EC: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C93F0: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826C93F4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C93F8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826C93FC: 388A49FC  addi r4, r10, 0x49fc
	ctx.r[4].s64 = ctx.r[10].s64 + 18940;
	// 826C9400: 396B28D8  addi r11, r11, 0x28d8
	ctx.r[11].s64 = ctx.r[11].s64 + 10456;
	// 826C9404: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C9408: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C940C: 386A7184  addi r3, r10, 0x7184
	ctx.r[3].s64 = ctx.r[10].s64 + 29060;
	// 826C9410: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 826C9414: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C9418: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 826C941C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C9420: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C9424: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C9428: 4BD9D9F9  bl 0x82466e20
	ctx.lr = 0x826C942C;
	sub_82466E20(ctx, base);
	// 826C942C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C9430: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C9434: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C9438: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C9440(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C9440 size=100
    let mut pc: u32 = 0x826C9440;
    'dispatch: loop {
        match pc {
            0x826C9440 => {
    //   block [0x826C9440..0x826C94A4)
	// 826C9440: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C9444: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C9448: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C944C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C9450: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C9454: 38AA4EA4  addi r5, r10, 0x4ea4
	ctx.r[5].s64 = ctx.r[10].s64 + 20132;
	// 826C9458: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C945C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C9460: 388A4A08  addi r4, r10, 0x4a08
	ctx.r[4].s64 = ctx.r[10].s64 + 18952;
	// 826C9464: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C9468: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C946C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C9470: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C9474: 386A71B4  addi r3, r10, 0x71b4
	ctx.r[3].s64 = ctx.r[10].s64 + 29108;
	// 826C9478: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C947C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C9480: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826C9484: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C9488: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826C948C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C9490: 4BD9D991  bl 0x82466e20
	ctx.lr = 0x826C9494;
	sub_82466E20(ctx, base);
	// 826C9494: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C9498: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C949C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C94A0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C94A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C94A8 size=100
    let mut pc: u32 = 0x826C94A8;
    'dispatch: loop {
        match pc {
            0x826C94A8 => {
    //   block [0x826C94A8..0x826C950C)
	// 826C94A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C94AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C94B0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C94B4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C94B8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C94BC: 38AA7244  addi r5, r10, 0x7244
	ctx.r[5].s64 = ctx.r[10].s64 + 29252;
	// 826C94C0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C94C4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C94C8: 388A4A1C  addi r4, r10, 0x4a1c
	ctx.r[4].s64 = ctx.r[10].s64 + 18972;
	// 826C94CC: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C94D0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C94D4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C94D8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C94DC: 386A71E4  addi r3, r10, 0x71e4
	ctx.r[3].s64 = ctx.r[10].s64 + 29156;
	// 826C94E0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C94E4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C94E8: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826C94EC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C94F0: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826C94F4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C94F8: 4BD9D929  bl 0x82466e20
	ctx.lr = 0x826C94FC;
	sub_82466E20(ctx, base);
	// 826C94FC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C9500: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C9504: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C9508: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C9510(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C9510 size=100
    let mut pc: u32 = 0x826C9510;
    'dispatch: loop {
        match pc {
            0x826C9510 => {
    //   block [0x826C9510..0x826C9574)
	// 826C9510: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C9514: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C9518: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C951C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C9520: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C9524: 38AA7184  addi r5, r10, 0x7184
	ctx.r[5].s64 = ctx.r[10].s64 + 29060;
	// 826C9528: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C952C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C9530: 388A4A38  addi r4, r10, 0x4a38
	ctx.r[4].s64 = ctx.r[10].s64 + 19000;
	// 826C9534: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C9538: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C953C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C9540: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C9544: 386A7214  addi r3, r10, 0x7214
	ctx.r[3].s64 = ctx.r[10].s64 + 29204;
	// 826C9548: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C954C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C9550: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826C9554: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C9558: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826C955C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C9560: 4BD9D8C1  bl 0x82466e20
	ctx.lr = 0x826C9564;
	sub_82466E20(ctx, base);
	// 826C9564: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C9568: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C956C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C9570: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C9578(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C9578 size=104
    let mut pc: u32 = 0x826C9578;
    'dispatch: loop {
        match pc {
            0x826C9578 => {
    //   block [0x826C9578..0x826C95E0)
	// 826C9578: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C957C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C9580: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C9584: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 826C9588: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C958C: 392A293C  addi r9, r10, 0x293c
	ctx.r[9].s64 = ctx.r[10].s64 + 10556;
	// 826C9590: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C9594: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C9598: 38AA71B4  addi r5, r10, 0x71b4
	ctx.r[5].s64 = ctx.r[10].s64 + 29108;
	// 826C959C: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C95A0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C95A4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C95A8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C95AC: 388A4A44  addi r4, r10, 0x4a44
	ctx.r[4].s64 = ctx.r[10].s64 + 19012;
	// 826C95B0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C95B4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C95B8: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826C95BC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C95C0: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826C95C4: 386A7244  addi r3, r10, 0x7244
	ctx.r[3].s64 = ctx.r[10].s64 + 29252;
	// 826C95C8: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826C95CC: 4BD9D855  bl 0x82466e20
	ctx.lr = 0x826C95D0;
	sub_82466E20(ctx, base);
	// 826C95D0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C95D4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C95D8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C95DC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C95E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C95E0 size=108
    let mut pc: u32 = 0x826C95E0;
    'dispatch: loop {
        match pc {
            0x826C95E0 => {
    //   block [0x826C95E0..0x826C964C)
	// 826C95E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C95E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C95E8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C95EC: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826C95F0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C95F4: 38EB7FF4  addi r7, r11, 0x7ff4
	ctx.r[7].s64 = ctx.r[11].s64 + 32756;
	// 826C95F8: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826C95FC: 388A4A5C  addi r4, r10, 0x4a5c
	ctx.r[4].s64 = ctx.r[10].s64 + 19036;
	// 826C9600: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C9604: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C9608: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826C960C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C9610: 386A7274  addi r3, r10, 0x7274
	ctx.r[3].s64 = ctx.r[10].s64 + 29300;
	// 826C9614: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826C9618: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C961C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C9620: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C9624: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C9628: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C962C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C9630: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C9634: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826C9638: 4BD9D7E9  bl 0x82466e20
	ctx.lr = 0x826C963C;
	sub_82466E20(ctx, base);
	// 826C963C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C9640: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C9644: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C9648: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C9650(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C9650 size=112
    let mut pc: u32 = 0x826C9650;
    'dispatch: loop {
        match pc {
            0x826C9650 => {
    //   block [0x826C9650..0x826C96C0)
	// 826C9650: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C9654: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C9658: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C965C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C9660: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826C9664: 38AA7244  addi r5, r10, 0x7244
	ctx.r[5].s64 = ctx.r[10].s64 + 29252;
	// 826C9668: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C966C: 390B8028  addi r8, r11, -0x7fd8
	ctx.r[8].s64 = ctx.r[11].s64 + -32728;
	// 826C9670: 39200007  li r9, 7
	ctx.r[9].s64 = 7;
	// 826C9674: 388A4A80  addi r4, r10, 0x4a80
	ctx.r[4].s64 = ctx.r[10].s64 + 19072;
	// 826C9678: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C967C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C9680: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826C9684: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C9688: 386A72A4  addi r3, r10, 0x72a4
	ctx.r[3].s64 = ctx.r[10].s64 + 29348;
	// 826C968C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826C9690: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C9694: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C9698: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C969C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C96A0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C96A4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C96A8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C96AC: 4BD9D775  bl 0x82466e20
	ctx.lr = 0x826C96B0;
	sub_82466E20(ctx, base);
	// 826C96B0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C96B4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C96B8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C96BC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C96C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826C96C0 size=24
    let mut pc: u32 = 0x826C96C0;
    'dispatch: loop {
        match pc {
            0x826C96C0 => {
    //   block [0x826C96C0..0x826C96D8)
	// 826C96C0: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826C96C4: 3D40827F  lis r10, -0x7d81
	ctx.r[10].s64 = -2105606144;
	// 826C96C8: 394AA688  addi r10, r10, -0x5978
	ctx.r[10].s64 = ctx.r[10].s64 + -22904;
	// 826C96CC: 816B8024  lwz r11, -0x7fdc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-32732 as u32) ) } as u64;
	// 826C96D0: 916A0008  stw r11, 8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 826C96D4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C96D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C96D8 size=116
    let mut pc: u32 = 0x826C96D8;
    'dispatch: loop {
        match pc {
            0x826C96D8 => {
    //   block [0x826C96D8..0x826C974C)
	// 826C96D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C96DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C96E0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C96E4: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826C96E8: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 826C96EC: 390BA688  addi r8, r11, -0x5978
	ctx.r[8].s64 = ctx.r[11].s64 + -22904;
	// 826C96F0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C96F4: 392A29A0  addi r9, r10, 0x29a0
	ctx.r[9].s64 = ctx.r[10].s64 + 10656;
	// 826C96F8: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C96FC: 38E00005  li r7, 5
	ctx.r[7].s64 = 5;
	// 826C9700: 38AA4EA4  addi r5, r10, 0x4ea4
	ctx.r[5].s64 = ctx.r[10].s64 + 20132;
	// 826C9704: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826C9708: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C970C: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C9710: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C9714: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C9718: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C971C: 3D608298  lis r11, -0x7d68
	ctx.r[11].s64 = -2103967744;
	// 826C9720: 388A4B0C  addi r4, r10, 0x4b0c
	ctx.r[4].s64 = ctx.r[10].s64 + 19212;
	// 826C9724: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826C9728: 386B72D4  addi r3, r11, 0x72d4
	ctx.r[3].s64 = ctx.r[11].s64 + 29396;
	// 826C972C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826C9730: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C9734: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C9738: 4BD9D6E9  bl 0x82466e20
	ctx.lr = 0x826C973C;
	sub_82466E20(ctx, base);
	// 826C973C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C9740: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C9744: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C9748: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C9750(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C9750 size=100
    let mut pc: u32 = 0x826C9750;
    'dispatch: loop {
        match pc {
            0x826C9750 => {
    //   block [0x826C9750..0x826C97B4)
	// 826C9750: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C9754: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C9758: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C975C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C9760: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C9764: 38AA72D4  addi r5, r10, 0x72d4
	ctx.r[5].s64 = ctx.r[10].s64 + 29396;
	// 826C9768: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C976C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C9770: 388A4B18  addi r4, r10, 0x4b18
	ctx.r[4].s64 = ctx.r[10].s64 + 19224;
	// 826C9774: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C9778: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C977C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C9780: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C9784: 386A7304  addi r3, r10, 0x7304
	ctx.r[3].s64 = ctx.r[10].s64 + 29444;
	// 826C9788: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C978C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C9790: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826C9794: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C9798: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826C979C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C97A0: 4BD9D681  bl 0x82466e20
	ctx.lr = 0x826C97A4;
	sub_82466E20(ctx, base);
	// 826C97A4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C97A8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C97AC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C97B0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C97B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C97B8 size=100
    let mut pc: u32 = 0x826C97B8;
    'dispatch: loop {
        match pc {
            0x826C97B8 => {
    //   block [0x826C97B8..0x826C981C)
	// 826C97B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C97BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C97C0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C97C4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C97C8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C97CC: 38AA7364  addi r5, r10, 0x7364
	ctx.r[5].s64 = ctx.r[10].s64 + 29540;
	// 826C97D0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C97D4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C97D8: 388A4B24  addi r4, r10, 0x4b24
	ctx.r[4].s64 = ctx.r[10].s64 + 19236;
	// 826C97DC: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C97E0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C97E4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C97E8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C97EC: 386A7334  addi r3, r10, 0x7334
	ctx.r[3].s64 = ctx.r[10].s64 + 29492;
	// 826C97F0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C97F4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C97F8: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826C97FC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C9800: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826C9804: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C9808: 4BD9D619  bl 0x82466e20
	ctx.lr = 0x826C980C;
	sub_82466E20(ctx, base);
	// 826C980C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C9810: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C9814: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C9818: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C9820(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C9820 size=112
    let mut pc: u32 = 0x826C9820;
    'dispatch: loop {
        match pc {
            0x826C9820 => {
    //   block [0x826C9820..0x826C9890)
	// 826C9820: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C9824: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C9828: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C982C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C9830: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826C9834: 38AA72D4  addi r5, r10, 0x72d4
	ctx.r[5].s64 = ctx.r[10].s64 + 29396;
	// 826C9838: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C983C: 390B80D0  addi r8, r11, -0x7f30
	ctx.r[8].s64 = ctx.r[11].s64 + -32560;
	// 826C9840: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826C9844: 388A4B38  addi r4, r10, 0x4b38
	ctx.r[4].s64 = ctx.r[10].s64 + 19256;
	// 826C9848: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C984C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C9850: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826C9854: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C9858: 386A7364  addi r3, r10, 0x7364
	ctx.r[3].s64 = ctx.r[10].s64 + 29540;
	// 826C985C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826C9860: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C9864: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C9868: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C986C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C9870: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C9874: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C9878: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C987C: 4BD9D5A5  bl 0x82466e20
	ctx.lr = 0x826C9880;
	sub_82466E20(ctx, base);
	// 826C9880: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C9884: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C9888: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C988C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C9890(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C9890 size=100
    let mut pc: u32 = 0x826C9890;
    'dispatch: loop {
        match pc {
            0x826C9890 => {
    //   block [0x826C9890..0x826C98F4)
	// 826C9890: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C9894: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C9898: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C989C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C98A0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C98A4: 38AA7364  addi r5, r10, 0x7364
	ctx.r[5].s64 = ctx.r[10].s64 + 29540;
	// 826C98A8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C98AC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C98B0: 388A4B50  addi r4, r10, 0x4b50
	ctx.r[4].s64 = ctx.r[10].s64 + 19280;
	// 826C98B4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C98B8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C98BC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C98C0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C98C4: 386A7394  addi r3, r10, 0x7394
	ctx.r[3].s64 = ctx.r[10].s64 + 29588;
	// 826C98C8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C98CC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C98D0: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826C98D4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C98D8: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826C98DC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C98E0: 4BD9D541  bl 0x82466e20
	ctx.lr = 0x826C98E4;
	sub_82466E20(ctx, base);
	// 826C98E4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C98E8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C98EC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C98F0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C98F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C98F8 size=100
    let mut pc: u32 = 0x826C98F8;
    'dispatch: loop {
        match pc {
            0x826C98F8 => {
    //   block [0x826C98F8..0x826C995C)
	// 826C98F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C98FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C9900: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C9904: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C9908: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C990C: 38AA72D4  addi r5, r10, 0x72d4
	ctx.r[5].s64 = ctx.r[10].s64 + 29396;
	// 826C9910: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C9914: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C9918: 388A4B60  addi r4, r10, 0x4b60
	ctx.r[4].s64 = ctx.r[10].s64 + 19296;
	// 826C991C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C9920: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C9924: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C9928: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C992C: 386A73C4  addi r3, r10, 0x73c4
	ctx.r[3].s64 = ctx.r[10].s64 + 29636;
	// 826C9930: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C9934: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C9938: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826C993C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C9940: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826C9944: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C9948: 4BD9D4D9  bl 0x82466e20
	ctx.lr = 0x826C994C;
	sub_82466E20(ctx, base);
	// 826C994C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C9950: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C9954: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C9958: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C9960(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C9960 size=100
    let mut pc: u32 = 0x826C9960;
    'dispatch: loop {
        match pc {
            0x826C9960 => {
    //   block [0x826C9960..0x826C99C4)
	// 826C9960: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C9964: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C9968: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C996C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C9970: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C9974: 38AA7304  addi r5, r10, 0x7304
	ctx.r[5].s64 = ctx.r[10].s64 + 29444;
	// 826C9978: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C997C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C9980: 388A4B70  addi r4, r10, 0x4b70
	ctx.r[4].s64 = ctx.r[10].s64 + 19312;
	// 826C9984: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C9988: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C998C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C9990: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C9994: 386A73F4  addi r3, r10, 0x73f4
	ctx.r[3].s64 = ctx.r[10].s64 + 29684;
	// 826C9998: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C999C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C99A0: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826C99A4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C99A8: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826C99AC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C99B0: 4BD9D471  bl 0x82466e20
	ctx.lr = 0x826C99B4;
	sub_82466E20(ctx, base);
	// 826C99B4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C99B8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C99BC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C99C0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C99C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C99C8 size=100
    let mut pc: u32 = 0x826C99C8;
    'dispatch: loop {
        match pc {
            0x826C99C8 => {
    //   block [0x826C99C8..0x826C9A2C)
	// 826C99C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C99CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C99D0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C99D4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C99D8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C99DC: 38AA73C4  addi r5, r10, 0x73c4
	ctx.r[5].s64 = ctx.r[10].s64 + 29636;
	// 826C99E0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C99E4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C99E8: 388A4B88  addi r4, r10, 0x4b88
	ctx.r[4].s64 = ctx.r[10].s64 + 19336;
	// 826C99EC: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C99F0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C99F4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C99F8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C99FC: 386A7424  addi r3, r10, 0x7424
	ctx.r[3].s64 = ctx.r[10].s64 + 29732;
	// 826C9A00: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C9A04: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C9A08: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826C9A0C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C9A10: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826C9A14: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C9A18: 4BD9D409  bl 0x82466e20
	ctx.lr = 0x826C9A1C;
	sub_82466E20(ctx, base);
	// 826C9A1C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C9A20: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C9A24: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C9A28: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C9A30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C9A30 size=100
    let mut pc: u32 = 0x826C9A30;
    'dispatch: loop {
        match pc {
            0x826C9A30 => {
    //   block [0x826C9A30..0x826C9A94)
	// 826C9A30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C9A34: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C9A38: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C9A3C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C9A40: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C9A44: 38AA7304  addi r5, r10, 0x7304
	ctx.r[5].s64 = ctx.r[10].s64 + 29444;
	// 826C9A48: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C9A4C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C9A50: 388A4BA4  addi r4, r10, 0x4ba4
	ctx.r[4].s64 = ctx.r[10].s64 + 19364;
	// 826C9A54: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C9A58: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C9A5C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C9A60: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C9A64: 386A7454  addi r3, r10, 0x7454
	ctx.r[3].s64 = ctx.r[10].s64 + 29780;
	// 826C9A68: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C9A6C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C9A70: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826C9A74: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C9A78: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826C9A7C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C9A80: 4BD9D3A1  bl 0x82466e20
	ctx.lr = 0x826C9A84;
	sub_82466E20(ctx, base);
	// 826C9A84: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C9A88: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C9A8C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C9A90: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C9A98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C9A98 size=112
    let mut pc: u32 = 0x826C9A98;
    'dispatch: loop {
        match pc {
            0x826C9A98 => {
    //   block [0x826C9A98..0x826C9B08)
	// 826C9A98: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C9A9C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C9AA0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C9AA4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C9AA8: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826C9AAC: 38AA74E4  addi r5, r10, 0x74e4
	ctx.r[5].s64 = ctx.r[10].s64 + 29924;
	// 826C9AB0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C9AB4: 390B8100  addi r8, r11, -0x7f00
	ctx.r[8].s64 = ctx.r[11].s64 + -32512;
	// 826C9AB8: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826C9ABC: 388A4BB4  addi r4, r10, 0x4bb4
	ctx.r[4].s64 = ctx.r[10].s64 + 19380;
	// 826C9AC0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C9AC4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C9AC8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826C9ACC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C9AD0: 386A7484  addi r3, r10, 0x7484
	ctx.r[3].s64 = ctx.r[10].s64 + 29828;
	// 826C9AD4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826C9AD8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C9ADC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C9AE0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C9AE4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C9AE8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C9AEC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C9AF0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C9AF4: 4BD9D32D  bl 0x82466e20
	ctx.lr = 0x826C9AF8;
	sub_82466E20(ctx, base);
	// 826C9AF8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C9AFC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C9B00: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C9B04: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C9B08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C9B08 size=112
    let mut pc: u32 = 0x826C9B08;
    'dispatch: loop {
        match pc {
            0x826C9B08 => {
    //   block [0x826C9B08..0x826C9B78)
	// 826C9B08: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C9B0C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C9B10: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C9B14: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C9B18: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826C9B1C: 38AA7514  addi r5, r10, 0x7514
	ctx.r[5].s64 = ctx.r[10].s64 + 29972;
	// 826C9B20: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C9B24: 390B8130  addi r8, r11, -0x7ed0
	ctx.r[8].s64 = ctx.r[11].s64 + -32464;
	// 826C9B28: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826C9B2C: 388A4BC4  addi r4, r10, 0x4bc4
	ctx.r[4].s64 = ctx.r[10].s64 + 19396;
	// 826C9B30: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C9B34: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C9B38: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826C9B3C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C9B40: 386A74B4  addi r3, r10, 0x74b4
	ctx.r[3].s64 = ctx.r[10].s64 + 29876;
	// 826C9B44: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826C9B48: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C9B4C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C9B50: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C9B54: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C9B58: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C9B5C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C9B60: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C9B64: 4BD9D2BD  bl 0x82466e20
	ctx.lr = 0x826C9B68;
	sub_82466E20(ctx, base);
	// 826C9B68: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C9B6C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C9B70: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C9B74: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C9B78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C9B78 size=112
    let mut pc: u32 = 0x826C9B78;
    'dispatch: loop {
        match pc {
            0x826C9B78 => {
    //   block [0x826C9B78..0x826C9BE8)
	// 826C9B78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C9B7C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C9B80: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C9B84: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C9B88: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826C9B8C: 38AA75D4  addi r5, r10, 0x75d4
	ctx.r[5].s64 = ctx.r[10].s64 + 30164;
	// 826C9B90: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C9B94: 390B8148  addi r8, r11, -0x7eb8
	ctx.r[8].s64 = ctx.r[11].s64 + -32440;
	// 826C9B98: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826C9B9C: 388A4BDC  addi r4, r10, 0x4bdc
	ctx.r[4].s64 = ctx.r[10].s64 + 19420;
	// 826C9BA0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C9BA4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C9BA8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826C9BAC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C9BB0: 386A74E4  addi r3, r10, 0x74e4
	ctx.r[3].s64 = ctx.r[10].s64 + 29924;
	// 826C9BB4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826C9BB8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C9BBC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C9BC0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C9BC4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C9BC8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C9BCC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C9BD0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C9BD4: 4BD9D24D  bl 0x82466e20
	ctx.lr = 0x826C9BD8;
	sub_82466E20(ctx, base);
	// 826C9BD8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C9BDC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C9BE0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C9BE4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C9BE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C9BE8 size=112
    let mut pc: u32 = 0x826C9BE8;
    'dispatch: loop {
        match pc {
            0x826C9BE8 => {
    //   block [0x826C9BE8..0x826C9C58)
	// 826C9BE8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C9BEC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C9BF0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C9BF4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C9BF8: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826C9BFC: 38AA74E4  addi r5, r10, 0x74e4
	ctx.r[5].s64 = ctx.r[10].s64 + 29924;
	// 826C9C00: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C9C04: 390B8178  addi r8, r11, -0x7e88
	ctx.r[8].s64 = ctx.r[11].s64 + -32392;
	// 826C9C08: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826C9C0C: 388A4BE8  addi r4, r10, 0x4be8
	ctx.r[4].s64 = ctx.r[10].s64 + 19432;
	// 826C9C10: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C9C14: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C9C18: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826C9C1C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C9C20: 386A7514  addi r3, r10, 0x7514
	ctx.r[3].s64 = ctx.r[10].s64 + 29972;
	// 826C9C24: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826C9C28: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C9C2C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C9C30: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C9C34: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C9C38: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C9C3C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C9C40: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C9C44: 4BD9D1DD  bl 0x82466e20
	ctx.lr = 0x826C9C48;
	sub_82466E20(ctx, base);
	// 826C9C48: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C9C4C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C9C50: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C9C54: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C9C58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C9C58 size=112
    let mut pc: u32 = 0x826C9C58;
    'dispatch: loop {
        match pc {
            0x826C9C58 => {
    //   block [0x826C9C58..0x826C9CC8)
	// 826C9C58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C9C5C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C9C60: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C9C64: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C9C68: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826C9C6C: 38AA7514  addi r5, r10, 0x7514
	ctx.r[5].s64 = ctx.r[10].s64 + 29972;
	// 826C9C70: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C9C74: 390B8190  addi r8, r11, -0x7e70
	ctx.r[8].s64 = ctx.r[11].s64 + -32368;
	// 826C9C78: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826C9C7C: 388A4BF8  addi r4, r10, 0x4bf8
	ctx.r[4].s64 = ctx.r[10].s64 + 19448;
	// 826C9C80: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C9C84: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C9C88: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826C9C8C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C9C90: 386A7544  addi r3, r10, 0x7544
	ctx.r[3].s64 = ctx.r[10].s64 + 30020;
	// 826C9C94: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826C9C98: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C9C9C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C9CA0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C9CA4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C9CA8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C9CAC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C9CB0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C9CB4: 4BD9D16D  bl 0x82466e20
	ctx.lr = 0x826C9CB8;
	sub_82466E20(ctx, base);
	// 826C9CB8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C9CBC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C9CC0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C9CC4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C9CC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C9CC8 size=116
    let mut pc: u32 = 0x826C9CC8;
    'dispatch: loop {
        match pc {
            0x826C9CC8 => {
    //   block [0x826C9CC8..0x826C9D3C)
	// 826C9CC8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C9CCC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C9CD0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C9CD4: 3D40827F  lis r10, -0x7d81
	ctx.r[10].s64 = -2105606144;
	// 826C9CD8: 38E00007  li r7, 7
	ctx.r[7].s64 = 7;
	// 826C9CDC: 390A81A8  addi r8, r10, -0x7e58
	ctx.r[8].s64 = ctx.r[10].s64 + -32344;
	// 826C9CE0: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C9CE4: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 826C9CE8: 38AA4EA4  addi r5, r10, 0x4ea4
	ctx.r[5].s64 = ctx.r[10].s64 + 20132;
	// 826C9CEC: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C9CF0: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826C9CF4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C9CF8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826C9CFC: 388A4C10  addi r4, r10, 0x4c10
	ctx.r[4].s64 = ctx.r[10].s64 + 19472;
	// 826C9D00: 396B29B4  addi r11, r11, 0x29b4
	ctx.r[11].s64 = ctx.r[11].s64 + 10676;
	// 826C9D04: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C9D08: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C9D0C: 386A7574  addi r3, r10, 0x7574
	ctx.r[3].s64 = ctx.r[10].s64 + 30068;
	// 826C9D10: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 826C9D14: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C9D18: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 826C9D1C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C9D20: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C9D24: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C9D28: 4BD9D0F9  bl 0x82466e20
	ctx.lr = 0x826C9D2C;
	sub_82466E20(ctx, base);
	// 826C9D2C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C9D30: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C9D34: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C9D38: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C9D40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826C9D40 size=48
    let mut pc: u32 = 0x826C9D40;
    'dispatch: loop {
        match pc {
            0x826C9D40 => {
    //   block [0x826C9D40..0x826C9D70)
	// 826C9D40: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826C9D44: 814B825C  lwz r10, -0x7da4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-32164 as u32) ) } as u64;
	// 826C9D48: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826C9D4C: 396BA700  addi r11, r11, -0x5900
	ctx.r[11].s64 = ctx.r[11].s64 + -22784;
	// 826C9D50: 914B0050  stw r10, 0x50(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 826C9D54: 3D40827F  lis r10, -0x7d81
	ctx.r[10].s64 = -2105606144;
	// 826C9D58: 814A8258  lwz r10, -0x7da8(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-32168 as u32) ) } as u64;
	// 826C9D5C: 914B0110  stw r10, 0x110(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(272 as u32), ctx.r[10].u32 ) };
	// 826C9D60: 3D40827F  lis r10, -0x7d81
	ctx.r[10].s64 = -2105606144;
	// 826C9D64: 814A8254  lwz r10, -0x7dac(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-32172 as u32) ) } as u64;
	// 826C9D68: 914B0230  stw r10, 0x230(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(560 as u32), ctx.r[10].u32 ) };
	// 826C9D6C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C9D70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C9D70 size=116
    let mut pc: u32 = 0x826C9D70;
    'dispatch: loop {
        match pc {
            0x826C9D70 => {
    //   block [0x826C9D70..0x826C9DE4)
	// 826C9D70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C9D74: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C9D78: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C9D7C: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 826C9D80: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C9D84: 392B2A88  addi r9, r11, 0x2a88
	ctx.r[9].s64 = ctx.r[11].s64 + 10888;
	// 826C9D88: 38AA4EA4  addi r5, r10, 0x4ea4
	ctx.r[5].s64 = ctx.r[10].s64 + 20132;
	// 826C9D8C: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C9D90: 38E90050  addi r7, r9, 0x50
	ctx.r[7].s64 = ctx.r[9].s64 + 80;
	// 826C9D94: 38C0001B  li r6, 0x1b
	ctx.r[6].s64 = 27;
	// 826C9D98: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826C9D9C: 388A4FC8  addi r4, r10, 0x4fc8
	ctx.r[4].s64 = ctx.r[10].s64 + 20424;
	// 826C9DA0: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C9DA4: 396BA700  addi r11, r11, -0x5900
	ctx.r[11].s64 = ctx.r[11].s64 + -22784;
	// 826C9DA8: 90E10064  stw r7, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[7].u32 ) };
	// 826C9DAC: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C9DB0: 90C1005C  stw r6, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[6].u32 ) };
	// 826C9DB4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C9DB8: 386A75A4  addi r3, r10, 0x75a4
	ctx.r[3].s64 = ctx.r[10].s64 + 30116;
	// 826C9DBC: 39400004  li r10, 4
	ctx.r[10].s64 = 4;
	// 826C9DC0: 91010074  stw r8, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[8].u32 ) };
	// 826C9DC4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C9DC8: 9101006C  stw r8, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[8].u32 ) };
	// 826C9DCC: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826C9DD0: 4BD9D051  bl 0x82466e20
	ctx.lr = 0x826C9DD4;
	sub_82466E20(ctx, base);
	// 826C9DD4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C9DD8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C9DDC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C9DE0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C9DE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C9DE8 size=116
    let mut pc: u32 = 0x826C9DE8;
    'dispatch: loop {
        match pc {
            0x826C9DE8 => {
    //   block [0x826C9DE8..0x826C9E5C)
	// 826C9DE8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C9DEC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C9DF0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C9DF4: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826C9DF8: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 826C9DFC: 390B8268  addi r8, r11, -0x7d98
	ctx.r[8].s64 = ctx.r[11].s64 + -32152;
	// 826C9E00: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C9E04: 392A2BB0  addi r9, r10, 0x2bb0
	ctx.r[9].s64 = ctx.r[10].s64 + 11184;
	// 826C9E08: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C9E0C: 38E00006  li r7, 6
	ctx.r[7].s64 = 6;
	// 826C9E10: 38AA4EA4  addi r5, r10, 0x4ea4
	ctx.r[5].s64 = ctx.r[10].s64 + 20132;
	// 826C9E14: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826C9E18: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C9E1C: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C9E20: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C9E24: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C9E28: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C9E2C: 3D608298  lis r11, -0x7d68
	ctx.r[11].s64 = -2103967744;
	// 826C9E30: 388A4FD8  addi r4, r10, 0x4fd8
	ctx.r[4].s64 = ctx.r[10].s64 + 20440;
	// 826C9E34: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826C9E38: 386B75D4  addi r3, r11, 0x75d4
	ctx.r[3].s64 = ctx.r[11].s64 + 30164;
	// 826C9E3C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826C9E40: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C9E44: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C9E48: 4BD9CFD9  bl 0x82466e20
	ctx.lr = 0x826C9E4C;
	sub_82466E20(ctx, base);
	// 826C9E4C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C9E50: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C9E54: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C9E58: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C9E60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C9E60 size=112
    let mut pc: u32 = 0x826C9E60;
    'dispatch: loop {
        match pc {
            0x826C9E60 => {
    //   block [0x826C9E60..0x826C9ED0)
	// 826C9E60: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C9E64: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C9E68: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C9E6C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C9E70: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826C9E74: 38AA4EA4  addi r5, r10, 0x4ea4
	ctx.r[5].s64 = ctx.r[10].s64 + 20132;
	// 826C9E78: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C9E7C: 390B82F8  addi r8, r11, -0x7d08
	ctx.r[8].s64 = ctx.r[11].s64 + -32008;
	// 826C9E80: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826C9E84: 388A4FE8  addi r4, r10, 0x4fe8
	ctx.r[4].s64 = ctx.r[10].s64 + 20456;
	// 826C9E88: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C9E8C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C9E90: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826C9E94: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C9E98: 386A7604  addi r3, r10, 0x7604
	ctx.r[3].s64 = ctx.r[10].s64 + 30212;
	// 826C9E9C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826C9EA0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C9EA4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C9EA8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C9EAC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C9EB0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C9EB4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C9EB8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C9EBC: 4BD9CF65  bl 0x82466e20
	ctx.lr = 0x826C9EC0;
	sub_82466E20(ctx, base);
	// 826C9EC0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C9EC4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C9EC8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C9ECC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C9ED0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C9ED0 size=112
    let mut pc: u32 = 0x826C9ED0;
    'dispatch: loop {
        match pc {
            0x826C9ED0 => {
    //   block [0x826C9ED0..0x826C9F40)
	// 826C9ED0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C9ED4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C9ED8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C9EDC: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C9EE0: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826C9EE4: 38AA5954  addi r5, r10, 0x5954
	ctx.r[5].s64 = ctx.r[10].s64 + 22868;
	// 826C9EE8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C9EEC: 390B8310  addi r8, r11, -0x7cf0
	ctx.r[8].s64 = ctx.r[11].s64 + -31984;
	// 826C9EF0: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826C9EF4: 388A5000  addi r4, r10, 0x5000
	ctx.r[4].s64 = ctx.r[10].s64 + 20480;
	// 826C9EF8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C9EFC: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C9F00: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826C9F04: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C9F08: 386A7634  addi r3, r10, 0x7634
	ctx.r[3].s64 = ctx.r[10].s64 + 30260;
	// 826C9F0C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826C9F10: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C9F14: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C9F18: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C9F1C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C9F20: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C9F24: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C9F28: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C9F2C: 4BD9CEF5  bl 0x82466e20
	ctx.lr = 0x826C9F30;
	sub_82466E20(ctx, base);
	// 826C9F30: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C9F34: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C9F38: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C9F3C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C9F40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C9F40 size=108
    let mut pc: u32 = 0x826C9F40;
    'dispatch: loop {
        match pc {
            0x826C9F40 => {
    //   block [0x826C9F40..0x826C9FAC)
	// 826C9F40: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C9F44: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C9F48: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C9F4C: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826C9F50: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C9F54: 38EB8328  addi r7, r11, -0x7cd8
	ctx.r[7].s64 = ctx.r[11].s64 + -31960;
	// 826C9F58: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826C9F5C: 388A5014  addi r4, r10, 0x5014
	ctx.r[4].s64 = ctx.r[10].s64 + 20500;
	// 826C9F60: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C9F64: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C9F68: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826C9F6C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C9F70: 386A7664  addi r3, r10, 0x7664
	ctx.r[3].s64 = ctx.r[10].s64 + 30308;
	// 826C9F74: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826C9F78: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C9F7C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826C9F80: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C9F84: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C9F88: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826C9F8C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826C9F90: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826C9F94: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826C9F98: 4BD9CE89  bl 0x82466e20
	ctx.lr = 0x826C9F9C;
	sub_82466E20(ctx, base);
	// 826C9F9C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826C9FA0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826C9FA4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826C9FA8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826C9FB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826C9FB0 size=112
    let mut pc: u32 = 0x826C9FB0;
    'dispatch: loop {
        match pc {
            0x826C9FB0 => {
    //   block [0x826C9FB0..0x826CA020)
	// 826C9FB0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826C9FB4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826C9FB8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826C9FBC: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C9FC0: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826C9FC4: 38AA4EA4  addi r5, r10, 0x4ea4
	ctx.r[5].s64 = ctx.r[10].s64 + 20132;
	// 826C9FC8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826C9FCC: 390B8340  addi r8, r11, -0x7cc0
	ctx.r[8].s64 = ctx.r[11].s64 + -31936;
	// 826C9FD0: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 826C9FD4: 388A5028  addi r4, r10, 0x5028
	ctx.r[4].s64 = ctx.r[10].s64 + 20520;
	// 826C9FD8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826C9FDC: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826C9FE0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826C9FE4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826C9FE8: 386A7694  addi r3, r10, 0x7694
	ctx.r[3].s64 = ctx.r[10].s64 + 30356;
	// 826C9FEC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826C9FF0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826C9FF4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826C9FF8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826C9FFC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826CA000: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826CA004: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826CA008: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826CA00C: 4BD9CE15  bl 0x82466e20
	ctx.lr = 0x826CA010;
	sub_82466E20(ctx, base);
	// 826CA010: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826CA014: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826CA018: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826CA01C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826CA020(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826CA020 size=108
    let mut pc: u32 = 0x826CA020;
    'dispatch: loop {
        match pc {
            0x826CA020 => {
    //   block [0x826CA020..0x826CA08C)
	// 826CA020: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826CA024: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826CA028: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826CA02C: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826CA030: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826CA034: 38EB8388  addi r7, r11, -0x7c78
	ctx.r[7].s64 = ctx.r[11].s64 + -31864;
	// 826CA038: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826CA03C: 388A505C  addi r4, r10, 0x505c
	ctx.r[4].s64 = ctx.r[10].s64 + 20572;
	// 826CA040: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826CA044: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826CA048: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826CA04C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826CA050: 386A76C4  addi r3, r10, 0x76c4
	ctx.r[3].s64 = ctx.r[10].s64 + 30404;
	// 826CA054: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826CA058: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826CA05C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826CA060: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826CA064: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826CA068: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826CA06C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826CA070: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826CA074: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826CA078: 4BD9CDA9  bl 0x82466e20
	ctx.lr = 0x826CA07C;
	sub_82466E20(ctx, base);
	// 826CA07C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826CA080: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826CA084: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826CA088: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826CA090(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826CA090 size=112
    let mut pc: u32 = 0x826CA090;
    'dispatch: loop {
        match pc {
            0x826CA090 => {
    //   block [0x826CA090..0x826CA100)
	// 826CA090: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826CA094: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826CA098: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826CA09C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826CA0A0: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826CA0A4: 38AA4EA4  addi r5, r10, 0x4ea4
	ctx.r[5].s64 = ctx.r[10].s64 + 20132;
	// 826CA0A8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826CA0AC: 390B83A0  addi r8, r11, -0x7c60
	ctx.r[8].s64 = ctx.r[11].s64 + -31840;
	// 826CA0B0: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826CA0B4: 388A5070  addi r4, r10, 0x5070
	ctx.r[4].s64 = ctx.r[10].s64 + 20592;
	// 826CA0B8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826CA0BC: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826CA0C0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826CA0C4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826CA0C8: 386A76F4  addi r3, r10, 0x76f4
	ctx.r[3].s64 = ctx.r[10].s64 + 30452;
	// 826CA0CC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826CA0D0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826CA0D4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826CA0D8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826CA0DC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826CA0E0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826CA0E4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826CA0E8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826CA0EC: 4BD9CD35  bl 0x82466e20
	ctx.lr = 0x826CA0F0;
	sub_82466E20(ctx, base);
	// 826CA0F0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826CA0F4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826CA0F8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826CA0FC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826CA100(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826CA100 size=112
    let mut pc: u32 = 0x826CA100;
    'dispatch: loop {
        match pc {
            0x826CA100 => {
    //   block [0x826CA100..0x826CA170)
	// 826CA100: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826CA104: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826CA108: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826CA10C: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 826CA110: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826CA114: 392A2C08  addi r9, r10, 0x2c08
	ctx.r[9].s64 = ctx.r[10].s64 + 11272;
	// 826CA118: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826CA11C: 390B83D8  addi r8, r11, -0x7c28
	ctx.r[8].s64 = ctx.r[11].s64 + -31784;
	// 826CA120: 38E00007  li r7, 7
	ctx.r[7].s64 = 7;
	// 826CA124: 388A508C  addi r4, r10, 0x508c
	ctx.r[4].s64 = ctx.r[10].s64 + 20620;
	// 826CA128: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826CA12C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826CA130: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826CA134: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826CA138: 386A7724  addi r3, r10, 0x7724
	ctx.r[3].s64 = ctx.r[10].s64 + 30500;
	// 826CA13C: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826CA140: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826CA144: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826CA148: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826CA14C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826CA150: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826CA154: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826CA158: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826CA15C: 4BD9CCC5  bl 0x82466e20
	ctx.lr = 0x826CA160;
	sub_82466E20(ctx, base);
	// 826CA160: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826CA164: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826CA168: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826CA16C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826CA170(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826CA170 size=116
    let mut pc: u32 = 0x826CA170;
    'dispatch: loop {
        match pc {
            0x826CA170 => {
    //   block [0x826CA170..0x826CA1E4)
	// 826CA170: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826CA174: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826CA178: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826CA17C: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826CA180: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 826CA184: 390B8480  addi r8, r11, -0x7b80
	ctx.r[8].s64 = ctx.r[11].s64 + -31616;
	// 826CA188: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826CA18C: 392A2BDC  addi r9, r10, 0x2bdc
	ctx.r[9].s64 = ctx.r[10].s64 + 11228;
	// 826CA190: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826CA194: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
	// 826CA198: 38AA69A4  addi r5, r10, 0x69a4
	ctx.r[5].s64 = ctx.r[10].s64 + 27044;
	// 826CA19C: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826CA1A0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826CA1A4: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826CA1A8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826CA1AC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826CA1B0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826CA1B4: 3D608298  lis r11, -0x7d68
	ctx.r[11].s64 = -2103967744;
	// 826CA1B8: 388A50AC  addi r4, r10, 0x50ac
	ctx.r[4].s64 = ctx.r[10].s64 + 20652;
	// 826CA1BC: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826CA1C0: 386B7754  addi r3, r11, 0x7754
	ctx.r[3].s64 = ctx.r[11].s64 + 30548;
	// 826CA1C4: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826CA1C8: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826CA1CC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826CA1D0: 4BD9CC51  bl 0x82466e20
	ctx.lr = 0x826CA1D4;
	sub_82466E20(ctx, base);
	// 826CA1D4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826CA1D8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826CA1DC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826CA1E0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826CA1E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826CA1E8 size=112
    let mut pc: u32 = 0x826CA1E8;
    'dispatch: loop {
        match pc {
            0x826CA1E8 => {
    //   block [0x826CA1E8..0x826CA258)
	// 826CA1E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826CA1EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826CA1F0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826CA1F4: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 826CA1F8: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826CA1FC: 392A2C34  addi r9, r10, 0x2c34
	ctx.r[9].s64 = ctx.r[10].s64 + 11316;
	// 826CA200: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826CA204: 390B8498  addi r8, r11, -0x7b68
	ctx.r[8].s64 = ctx.r[11].s64 + -31592;
	// 826CA208: 38E00004  li r7, 4
	ctx.r[7].s64 = 4;
	// 826CA20C: 388A50C4  addi r4, r10, 0x50c4
	ctx.r[4].s64 = ctx.r[10].s64 + 20676;
	// 826CA210: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826CA214: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826CA218: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826CA21C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826CA220: 386A7784  addi r3, r10, 0x7784
	ctx.r[3].s64 = ctx.r[10].s64 + 30596;
	// 826CA224: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826CA228: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826CA22C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826CA230: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826CA234: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826CA238: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826CA23C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826CA240: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826CA244: 4BD9CBDD  bl 0x82466e20
	ctx.lr = 0x826CA248;
	sub_82466E20(ctx, base);
	// 826CA248: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826CA24C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826CA250: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826CA254: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826CA258(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826CA258 size=112
    let mut pc: u32 = 0x826CA258;
    'dispatch: loop {
        match pc {
            0x826CA258 => {
    //   block [0x826CA258..0x826CA2C8)
	// 826CA258: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826CA25C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826CA260: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826CA264: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826CA268: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826CA26C: 38AA69A4  addi r5, r10, 0x69a4
	ctx.r[5].s64 = ctx.r[10].s64 + 27044;
	// 826CA270: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826CA274: 390B84F8  addi r8, r11, -0x7b08
	ctx.r[8].s64 = ctx.r[11].s64 + -31496;
	// 826CA278: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826CA27C: 388A50E0  addi r4, r10, 0x50e0
	ctx.r[4].s64 = ctx.r[10].s64 + 20704;
	// 826CA280: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826CA284: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826CA288: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826CA28C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826CA290: 386A77B4  addi r3, r10, 0x77b4
	ctx.r[3].s64 = ctx.r[10].s64 + 30644;
	// 826CA294: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826CA298: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826CA29C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826CA2A0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826CA2A4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826CA2A8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826CA2AC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826CA2B0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826CA2B4: 4BD9CB6D  bl 0x82466e20
	ctx.lr = 0x826CA2B8;
	sub_82466E20(ctx, base);
	// 826CA2B8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826CA2BC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826CA2C0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826CA2C4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826CA2C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826CA2C8 size=112
    let mut pc: u32 = 0x826CA2C8;
    'dispatch: loop {
        match pc {
            0x826CA2C8 => {
    //   block [0x826CA2C8..0x826CA338)
	// 826CA2C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826CA2CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826CA2D0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826CA2D4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826CA2D8: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826CA2DC: 38AA68B4  addi r5, r10, 0x68b4
	ctx.r[5].s64 = ctx.r[10].s64 + 26804;
	// 826CA2E0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826CA2E4: 390B8510  addi r8, r11, -0x7af0
	ctx.r[8].s64 = ctx.r[11].s64 + -31472;
	// 826CA2E8: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 826CA2EC: 388A50F4  addi r4, r10, 0x50f4
	ctx.r[4].s64 = ctx.r[10].s64 + 20724;
	// 826CA2F0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826CA2F4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826CA2F8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826CA2FC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826CA300: 386A77E4  addi r3, r10, 0x77e4
	ctx.r[3].s64 = ctx.r[10].s64 + 30692;
	// 826CA304: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826CA308: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826CA30C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826CA310: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826CA314: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826CA318: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826CA31C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826CA320: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826CA324: 4BD9CAFD  bl 0x82466e20
	ctx.lr = 0x826CA328;
	sub_82466E20(ctx, base);
	// 826CA328: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826CA32C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826CA330: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826CA334: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826CA338(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826CA338 size=112
    let mut pc: u32 = 0x826CA338;
    'dispatch: loop {
        match pc {
            0x826CA338 => {
    //   block [0x826CA338..0x826CA3A8)
	// 826CA338: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826CA33C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826CA340: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826CA344: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826CA348: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826CA34C: 38AA68B4  addi r5, r10, 0x68b4
	ctx.r[5].s64 = ctx.r[10].s64 + 26804;
	// 826CA350: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826CA354: 390B8558  addi r8, r11, -0x7aa8
	ctx.r[8].s64 = ctx.r[11].s64 + -31400;
	// 826CA358: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 826CA35C: 388A510C  addi r4, r10, 0x510c
	ctx.r[4].s64 = ctx.r[10].s64 + 20748;
	// 826CA360: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826CA364: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826CA368: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826CA36C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826CA370: 386A7814  addi r3, r10, 0x7814
	ctx.r[3].s64 = ctx.r[10].s64 + 30740;
	// 826CA374: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826CA378: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826CA37C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826CA380: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826CA384: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826CA388: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826CA38C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826CA390: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826CA394: 4BD9CA8D  bl 0x82466e20
	ctx.lr = 0x826CA398;
	sub_82466E20(ctx, base);
	// 826CA398: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826CA39C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826CA3A0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826CA3A4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826CA3A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826CA3A8 size=112
    let mut pc: u32 = 0x826CA3A8;
    'dispatch: loop {
        match pc {
            0x826CA3A8 => {
    //   block [0x826CA3A8..0x826CA418)
	// 826CA3A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826CA3AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826CA3B0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826CA3B4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826CA3B8: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826CA3BC: 38AA68E4  addi r5, r10, 0x68e4
	ctx.r[5].s64 = ctx.r[10].s64 + 26852;
	// 826CA3C0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826CA3C4: 390B85B8  addi r8, r11, -0x7a48
	ctx.r[8].s64 = ctx.r[11].s64 + -31304;
	// 826CA3C8: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 826CA3CC: 388A511C  addi r4, r10, 0x511c
	ctx.r[4].s64 = ctx.r[10].s64 + 20764;
	// 826CA3D0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826CA3D4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826CA3D8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826CA3DC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826CA3E0: 386A7844  addi r3, r10, 0x7844
	ctx.r[3].s64 = ctx.r[10].s64 + 30788;
	// 826CA3E4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826CA3E8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826CA3EC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826CA3F0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826CA3F4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826CA3F8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826CA3FC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826CA400: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826CA404: 4BD9CA1D  bl 0x82466e20
	ctx.lr = 0x826CA408;
	sub_82466E20(ctx, base);
	// 826CA408: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826CA40C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826CA410: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826CA414: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826CA418(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826CA418 size=112
    let mut pc: u32 = 0x826CA418;
    'dispatch: loop {
        match pc {
            0x826CA418 => {
    //   block [0x826CA418..0x826CA488)
	// 826CA418: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826CA41C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826CA420: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826CA424: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826CA428: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826CA42C: 38AA68E4  addi r5, r10, 0x68e4
	ctx.r[5].s64 = ctx.r[10].s64 + 26852;
	// 826CA430: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826CA434: 390B8618  addi r8, r11, -0x79e8
	ctx.r[8].s64 = ctx.r[11].s64 + -31208;
	// 826CA438: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 826CA43C: 388A512C  addi r4, r10, 0x512c
	ctx.r[4].s64 = ctx.r[10].s64 + 20780;
	// 826CA440: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826CA444: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826CA448: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826CA44C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826CA450: 386A7874  addi r3, r10, 0x7874
	ctx.r[3].s64 = ctx.r[10].s64 + 30836;
	// 826CA454: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826CA458: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826CA45C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826CA460: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826CA464: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826CA468: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826CA46C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826CA470: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826CA474: 4BD9C9AD  bl 0x82466e20
	ctx.lr = 0x826CA478;
	sub_82466E20(ctx, base);
	// 826CA478: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826CA47C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826CA480: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826CA484: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826CA488(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826CA488 size=112
    let mut pc: u32 = 0x826CA488;
    'dispatch: loop {
        match pc {
            0x826CA488 => {
    //   block [0x826CA488..0x826CA4F8)
	// 826CA488: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826CA48C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826CA490: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826CA494: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826CA498: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826CA49C: 38AA68B4  addi r5, r10, 0x68b4
	ctx.r[5].s64 = ctx.r[10].s64 + 26804;
	// 826CA4A0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826CA4A4: 390B8678  addi r8, r11, -0x7988
	ctx.r[8].s64 = ctx.r[11].s64 + -31112;
	// 826CA4A8: 39200008  li r9, 8
	ctx.r[9].s64 = 8;
	// 826CA4AC: 388A5140  addi r4, r10, 0x5140
	ctx.r[4].s64 = ctx.r[10].s64 + 20800;
	// 826CA4B0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826CA4B4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826CA4B8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826CA4BC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826CA4C0: 386A78A4  addi r3, r10, 0x78a4
	ctx.r[3].s64 = ctx.r[10].s64 + 30884;
	// 826CA4C4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826CA4C8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826CA4CC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826CA4D0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826CA4D4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826CA4D8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826CA4DC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826CA4E0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826CA4E4: 4BD9C93D  bl 0x82466e20
	ctx.lr = 0x826CA4E8;
	sub_82466E20(ctx, base);
	// 826CA4E8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826CA4EC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826CA4F0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826CA4F4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826CA4F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826CA4F8 size=112
    let mut pc: u32 = 0x826CA4F8;
    'dispatch: loop {
        match pc {
            0x826CA4F8 => {
    //   block [0x826CA4F8..0x826CA568)
	// 826CA4F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826CA4FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826CA500: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826CA504: 3D40827F  lis r10, -0x7d81
	ctx.r[10].s64 = -2105606144;
	// 826CA508: 39000013  li r8, 0x13
	ctx.r[8].s64 = 19;
	// 826CA50C: 38EA8738  addi r7, r10, -0x78c8
	ctx.r[7].s64 = ctx.r[10].s64 + -30920;
	// 826CA510: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826CA514: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 826CA518: 388A5150  addi r4, r10, 0x5150
	ctx.r[4].s64 = ctx.r[10].s64 + 20816;
	// 826CA51C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826CA520: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826CA524: 396B2C48  addi r11, r11, 0x2c48
	ctx.r[11].s64 = ctx.r[11].s64 + 11336;
	// 826CA528: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826CA52C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826CA530: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826CA534: 386A78D4  addi r3, r10, 0x78d4
	ctx.r[3].s64 = ctx.r[10].s64 + 30932;
	// 826CA538: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826CA53C: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 826CA540: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826CA544: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 826CA548: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826CA54C: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826CA550: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826CA554: 4BD9C8CD  bl 0x82466e20
	ctx.lr = 0x826CA558;
	sub_82466E20(ctx, base);
	// 826CA558: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826CA55C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826CA560: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826CA564: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826CA568(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826CA568 size=112
    let mut pc: u32 = 0x826CA568;
    'dispatch: loop {
        match pc {
            0x826CA568 => {
    //   block [0x826CA568..0x826CA5D8)
	// 826CA568: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826CA56C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826CA570: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826CA574: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826CA578: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826CA57C: 38AA5A14  addi r5, r10, 0x5a14
	ctx.r[5].s64 = ctx.r[10].s64 + 23060;
	// 826CA580: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826CA584: 390B8900  addi r8, r11, -0x7700
	ctx.r[8].s64 = ctx.r[11].s64 + -30464;
	// 826CA588: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826CA58C: 388A5168  addi r4, r10, 0x5168
	ctx.r[4].s64 = ctx.r[10].s64 + 20840;
	// 826CA590: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826CA594: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826CA598: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826CA59C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826CA5A0: 386A7904  addi r3, r10, 0x7904
	ctx.r[3].s64 = ctx.r[10].s64 + 30980;
	// 826CA5A4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826CA5A8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826CA5AC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826CA5B0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826CA5B4: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826CA5B8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826CA5BC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826CA5C0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826CA5C4: 4BD9C85D  bl 0x82466e20
	ctx.lr = 0x826CA5C8;
	sub_82466E20(ctx, base);
	// 826CA5C8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826CA5CC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826CA5D0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826CA5D4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826CA5D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826CA5D8 size=112
    let mut pc: u32 = 0x826CA5D8;
    'dispatch: loop {
        match pc {
            0x826CA5D8 => {
    //   block [0x826CA5D8..0x826CA648)
	// 826CA5D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826CA5DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826CA5E0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826CA5E4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826CA5E8: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826CA5EC: 38AA5A14  addi r5, r10, 0x5a14
	ctx.r[5].s64 = ctx.r[10].s64 + 23060;
	// 826CA5F0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826CA5F4: 390B8918  addi r8, r11, -0x76e8
	ctx.r[8].s64 = ctx.r[11].s64 + -30440;
	// 826CA5F8: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826CA5FC: 388A5184  addi r4, r10, 0x5184
	ctx.r[4].s64 = ctx.r[10].s64 + 20868;
	// 826CA600: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826CA604: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826CA608: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826CA60C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826CA610: 386A7934  addi r3, r10, 0x7934
	ctx.r[3].s64 = ctx.r[10].s64 + 31028;
	// 826CA614: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826CA618: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826CA61C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826CA620: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826CA624: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826CA628: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826CA62C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826CA630: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826CA634: 4BD9C7ED  bl 0x82466e20
	ctx.lr = 0x826CA638;
	sub_82466E20(ctx, base);
	// 826CA638: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826CA63C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826CA640: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826CA644: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826CA648(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826CA648 size=112
    let mut pc: u32 = 0x826CA648;
    'dispatch: loop {
        match pc {
            0x826CA648 => {
    //   block [0x826CA648..0x826CA6B8)
	// 826CA648: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826CA64C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826CA650: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826CA654: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826CA658: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826CA65C: 38AA5A14  addi r5, r10, 0x5a14
	ctx.r[5].s64 = ctx.r[10].s64 + 23060;
	// 826CA660: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826CA664: 390B8930  addi r8, r11, -0x76d0
	ctx.r[8].s64 = ctx.r[11].s64 + -30416;
	// 826CA668: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826CA66C: 388A51A4  addi r4, r10, 0x51a4
	ctx.r[4].s64 = ctx.r[10].s64 + 20900;
	// 826CA670: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826CA674: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826CA678: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826CA67C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826CA680: 386A7964  addi r3, r10, 0x7964
	ctx.r[3].s64 = ctx.r[10].s64 + 31076;
	// 826CA684: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826CA688: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826CA68C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826CA690: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826CA694: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826CA698: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826CA69C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826CA6A0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826CA6A4: 4BD9C77D  bl 0x82466e20
	ctx.lr = 0x826CA6A8;
	sub_82466E20(ctx, base);
	// 826CA6A8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826CA6AC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826CA6B0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826CA6B4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826CA6B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826CA6B8 size=108
    let mut pc: u32 = 0x826CA6B8;
    'dispatch: loop {
        match pc {
            0x826CA6B8 => {
    //   block [0x826CA6B8..0x826CA724)
	// 826CA6B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826CA6BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826CA6C0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826CA6C4: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826CA6C8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826CA6CC: 38EB8960  addi r7, r11, -0x76a0
	ctx.r[7].s64 = ctx.r[11].s64 + -30368;
	// 826CA6D0: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826CA6D4: 388A51BC  addi r4, r10, 0x51bc
	ctx.r[4].s64 = ctx.r[10].s64 + 20924;
	// 826CA6D8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826CA6DC: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826CA6E0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826CA6E4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826CA6E8: 386A7994  addi r3, r10, 0x7994
	ctx.r[3].s64 = ctx.r[10].s64 + 31124;
	// 826CA6EC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826CA6F0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826CA6F4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826CA6F8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826CA6FC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826CA700: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826CA704: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826CA708: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826CA70C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826CA710: 4BD9C711  bl 0x82466e20
	ctx.lr = 0x826CA714;
	sub_82466E20(ctx, base);
	// 826CA714: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826CA718: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826CA71C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826CA720: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826CA728(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826CA728 size=112
    let mut pc: u32 = 0x826CA728;
    'dispatch: loop {
        match pc {
            0x826CA728 => {
    //   block [0x826CA728..0x826CA798)
	// 826CA728: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826CA72C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826CA730: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826CA734: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826CA738: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826CA73C: 38AA5A14  addi r5, r10, 0x5a14
	ctx.r[5].s64 = ctx.r[10].s64 + 23060;
	// 826CA740: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826CA744: 390B8990  addi r8, r11, -0x7670
	ctx.r[8].s64 = ctx.r[11].s64 + -30320;
	// 826CA748: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826CA74C: 388A51E4  addi r4, r10, 0x51e4
	ctx.r[4].s64 = ctx.r[10].s64 + 20964;
	// 826CA750: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826CA754: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826CA758: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826CA75C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826CA760: 386A79C4  addi r3, r10, 0x79c4
	ctx.r[3].s64 = ctx.r[10].s64 + 31172;
	// 826CA764: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826CA768: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826CA76C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826CA770: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826CA774: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826CA778: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826CA77C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826CA780: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826CA784: 4BD9C69D  bl 0x82466e20
	ctx.lr = 0x826CA788;
	sub_82466E20(ctx, base);
	// 826CA788: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826CA78C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826CA790: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826CA794: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826CA798(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826CA798 size=108
    let mut pc: u32 = 0x826CA798;
    'dispatch: loop {
        match pc {
            0x826CA798 => {
    //   block [0x826CA798..0x826CA804)
	// 826CA798: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826CA79C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826CA7A0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826CA7A4: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826CA7A8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826CA7AC: 38EB89A8  addi r7, r11, -0x7658
	ctx.r[7].s64 = ctx.r[11].s64 + -30296;
	// 826CA7B0: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826CA7B4: 388A5200  addi r4, r10, 0x5200
	ctx.r[4].s64 = ctx.r[10].s64 + 20992;
	// 826CA7B8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826CA7BC: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826CA7C0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826CA7C4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826CA7C8: 386A79F4  addi r3, r10, 0x79f4
	ctx.r[3].s64 = ctx.r[10].s64 + 31220;
	// 826CA7CC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826CA7D0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826CA7D4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826CA7D8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826CA7DC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826CA7E0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826CA7E4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826CA7E8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826CA7EC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826CA7F0: 4BD9C631  bl 0x82466e20
	ctx.lr = 0x826CA7F4;
	sub_82466E20(ctx, base);
	// 826CA7F4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826CA7F8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826CA7FC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826CA800: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826CA808(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826CA808 size=108
    let mut pc: u32 = 0x826CA808;
    'dispatch: loop {
        match pc {
            0x826CA808 => {
    //   block [0x826CA808..0x826CA874)
	// 826CA808: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826CA80C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826CA810: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826CA814: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826CA818: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826CA81C: 38EB89D8  addi r7, r11, -0x7628
	ctx.r[7].s64 = ctx.r[11].s64 + -30248;
	// 826CA820: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 826CA824: 388A521C  addi r4, r10, 0x521c
	ctx.r[4].s64 = ctx.r[10].s64 + 21020;
	// 826CA828: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826CA82C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826CA830: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826CA834: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826CA838: 386A7A24  addi r3, r10, 0x7a24
	ctx.r[3].s64 = ctx.r[10].s64 + 31268;
	// 826CA83C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826CA840: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826CA844: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826CA848: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826CA84C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826CA850: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826CA854: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826CA858: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826CA85C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826CA860: 4BD9C5C1  bl 0x82466e20
	ctx.lr = 0x826CA864;
	sub_82466E20(ctx, base);
	// 826CA864: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826CA868: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826CA86C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826CA870: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826CA878(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826CA878 size=112
    let mut pc: u32 = 0x826CA878;
    'dispatch: loop {
        match pc {
            0x826CA878 => {
    //   block [0x826CA878..0x826CA8E8)
	// 826CA878: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826CA87C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826CA880: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826CA884: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826CA888: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826CA88C: 38AA4EA4  addi r5, r10, 0x4ea4
	ctx.r[5].s64 = ctx.r[10].s64 + 20132;
	// 826CA890: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826CA894: 390B8A20  addi r8, r11, -0x75e0
	ctx.r[8].s64 = ctx.r[11].s64 + -30176;
	// 826CA898: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 826CA89C: 388A523C  addi r4, r10, 0x523c
	ctx.r[4].s64 = ctx.r[10].s64 + 21052;
	// 826CA8A0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826CA8A4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826CA8A8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826CA8AC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826CA8B0: 386A7A54  addi r3, r10, 0x7a54
	ctx.r[3].s64 = ctx.r[10].s64 + 31316;
	// 826CA8B4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826CA8B8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826CA8BC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826CA8C0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826CA8C4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826CA8C8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826CA8CC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826CA8D0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826CA8D4: 4BD9C54D  bl 0x82466e20
	ctx.lr = 0x826CA8D8;
	sub_82466E20(ctx, base);
	// 826CA8D8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826CA8DC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826CA8E0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826CA8E4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826CA8E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826CA8E8 size=112
    let mut pc: u32 = 0x826CA8E8;
    'dispatch: loop {
        match pc {
            0x826CA8E8 => {
    //   block [0x826CA8E8..0x826CA958)
	// 826CA8E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826CA8EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826CA8F0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826CA8F4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826CA8F8: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826CA8FC: 38AA68E4  addi r5, r10, 0x68e4
	ctx.r[5].s64 = ctx.r[10].s64 + 26852;
	// 826CA900: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826CA904: 390B8A68  addi r8, r11, -0x7598
	ctx.r[8].s64 = ctx.r[11].s64 + -30104;
	// 826CA908: 39200006  li r9, 6
	ctx.r[9].s64 = 6;
	// 826CA90C: 388A5254  addi r4, r10, 0x5254
	ctx.r[4].s64 = ctx.r[10].s64 + 21076;
	// 826CA910: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826CA914: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826CA918: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826CA91C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826CA920: 386A7A84  addi r3, r10, 0x7a84
	ctx.r[3].s64 = ctx.r[10].s64 + 31364;
	// 826CA924: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826CA928: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826CA92C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826CA930: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826CA934: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826CA938: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826CA93C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826CA940: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826CA944: 4BD9C4DD  bl 0x82466e20
	ctx.lr = 0x826CA948;
	sub_82466E20(ctx, base);
	// 826CA948: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826CA94C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826CA950: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826CA954: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826CA958(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826CA958 size=108
    let mut pc: u32 = 0x826CA958;
    'dispatch: loop {
        match pc {
            0x826CA958 => {
    //   block [0x826CA958..0x826CA9C4)
	// 826CA958: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826CA95C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826CA960: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826CA964: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826CA968: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826CA96C: 38EB8AF8  addi r7, r11, -0x7508
	ctx.r[7].s64 = ctx.r[11].s64 + -29960;
	// 826CA970: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 826CA974: 388A5268  addi r4, r10, 0x5268
	ctx.r[4].s64 = ctx.r[10].s64 + 21096;
	// 826CA978: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826CA97C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826CA980: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826CA984: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826CA988: 386A7AB4  addi r3, r10, 0x7ab4
	ctx.r[3].s64 = ctx.r[10].s64 + 31412;
	// 826CA98C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826CA990: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826CA994: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826CA998: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826CA99C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826CA9A0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826CA9A4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826CA9A8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826CA9AC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826CA9B0: 4BD9C471  bl 0x82466e20
	ctx.lr = 0x826CA9B4;
	sub_82466E20(ctx, base);
	// 826CA9B4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826CA9B8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826CA9BC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826CA9C0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826CA9C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826CA9C8 size=108
    let mut pc: u32 = 0x826CA9C8;
    'dispatch: loop {
        match pc {
            0x826CA9C8 => {
    //   block [0x826CA9C8..0x826CAA34)
	// 826CA9C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826CA9CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826CA9D0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826CA9D4: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826CA9D8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826CA9DC: 38EB8B40  addi r7, r11, -0x74c0
	ctx.r[7].s64 = ctx.r[11].s64 + -29888;
	// 826CA9E0: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826CA9E4: 388A5284  addi r4, r10, 0x5284
	ctx.r[4].s64 = ctx.r[10].s64 + 21124;
	// 826CA9E8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826CA9EC: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826CA9F0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826CA9F4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826CA9F8: 386A7AE4  addi r3, r10, 0x7ae4
	ctx.r[3].s64 = ctx.r[10].s64 + 31460;
	// 826CA9FC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826CAA00: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826CAA04: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826CAA08: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826CAA0C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826CAA10: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826CAA14: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826CAA18: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826CAA1C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826CAA20: 4BD9C401  bl 0x82466e20
	ctx.lr = 0x826CAA24;
	sub_82466E20(ctx, base);
	// 826CAA24: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826CAA28: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826CAA2C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826CAA30: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


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


